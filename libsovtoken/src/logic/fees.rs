#![allow(dead_code)]

use indy::IndyHandle;
use serde_json;
use indy::ErrorCode;
use logic::address;
use logic::input::Input;
use logic::output::Output;
use indy::crypto::Crypto;
use utils::general::base58::serialize_bytes;

pub type Inputs = Vec<Input>;
pub type Outputs = Vec<Output>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Fees {
    outputs: Outputs,
    inputs: Inputs,
}

impl InputSigner for Fees {}
impl Fees {
    pub fn new(inputs: Inputs, outputs: Outputs) -> Self
    {
        return Fees { inputs, outputs };
    }
}

pub trait InputSigner:  {

    fn sign_inputs(wallet_handle: IndyHandle, inputs: &Inputs, outputs: &Outputs)
        -> Result<Inputs, ErrorCode>
    {
        let signed_inputs: Result<Inputs, ErrorCode> = inputs.iter()
            .map(|input| Self::sign_input(wallet_handle, input, outputs))
            .collect();

        return signed_inputs;
    }

    fn sign_input(wallet_handle: IndyHandle, input: &Input, outputs: &Outputs) -> Result<Input, ErrorCode>
    {
        let verkey = address::verkey_from_address(input.payment_address.clone())?;
        debug!("Received verkey for payment address >>> {:?}", verkey);

        let message_json_value = json!([[input.payment_address, input.sequence_number], outputs]);
        debug!("Message to sign >>> {:?}", message_json_value);

        let message = serde_json::to_string(&message_json_value)
            .map_err(|_| ErrorCode::CommonInvalidStructure)?
            .to_string();

        return Self::indy_crypto_sign(wallet_handle, verkey, message)
            .map(|signed_string| {
                debug!("Received encoded signature >>> {:?}", signed_string);
                input.clone().sign_with(signed_string)
            });
    }

    fn indy_crypto_sign (
        wallet_handle: IndyHandle,
        verkey: String,
        message: String,
    ) -> Result<String, ErrorCode>
    {
         return Crypto::sign(wallet_handle, &verkey, message.as_bytes())
             .map(|vec| serialize_bytes(&vec));
    }

}

#[cfg(test)]
mod test_fees {
    use super::*;

    struct MockedFees {}

    impl InputSigner for MockedFees {
        fn indy_crypto_sign(
            _wallet_handle: IndyHandle,
            verkey: String,
            _message: String
        ) -> Result<String, ErrorCode> {
            return Ok(verkey + "signed");
        }
    }

    fn inputs_outputs_valid() -> (Inputs, Outputs) {
        let outputs = vec![
            Output::new(String::from("pay:sov:Va8VcAE9CDnDEXSDQlbluWBRO5hFpTEqbSzK1UgnpbUabg9Q"), 10, None),
            Output::new(String::from("pay:sov:FekbDoBkdsj3nH2a2nNhhedoPju2UmyKrr1ZzMZGT0KENbvp"), 22, None),
        ];

        let inputs = vec![
            Input::new(String::from("pay:sov:SBD8oNfQNm1aEGE6KkYI1khYEGqG5zmEqrEw7maqKitIs121"), 1, None),
            Input::new(String::from("pay:sov:hhX4LejW7N23hPwC2yLKdor1ppXy3RhJ38TeXCZLgoBMSGfg"), 1, None),
        ]; 

        return (inputs, outputs);
    }

   #[test]
   fn sign_input_invalid_sequence_number() {
       unimplemented!();
   }

   #[test]
   fn sign_input_invalid_address_output() {
       /*
           Neither sign_input or sign_inputs is expecting multiple addresses.
       */
       let wallet_handle = 1;
       let (inputs, mut outputs) = inputs_outputs_valid();
       String::remove(&mut outputs[0].payment_address, 5);

       let signed_input = MockedFees::sign_input(wallet_handle, &inputs[0], &outputs).unwrap_err();
       assert_eq!(ErrorCode::CommonInvalidStructure, signed_input);
   }

    #[test]
    fn sign_input_invalid_address_input() {
        let wallet_handle = 1;
        let (mut inputs, outputs) = inputs_outputs_valid();

        String::remove(&mut inputs[0].payment_address, 5);

        let signed_input = MockedFees::sign_input(wallet_handle, &inputs[0], &outputs).unwrap_err();
        assert_eq!(ErrorCode::CommonInvalidStructure, signed_input);
    }

    #[test]
    fn sign_input() {
        let (inputs, outputs) = inputs_outputs_valid();

        let wallet_handle = 1;

        let signed_input = MockedFees::sign_input( wallet_handle, &inputs[0], &outputs).unwrap();
        let expected = Input::new(String::from("pay:sov:SBD8oNfQNm1aEGE6KkYI1khYEGqG5zmEqrEw7maqKitIs121"), 1, Some(String::from("SBD8oNfQNm1aEGE6KkYI1khYEGqG5zmEqrEw7maqKitIsigned")));
        assert_eq!(expected, signed_input);
    }


    #[test]
    fn sign_multi_input_invalid_input_address() {
        let wallet_handle = 1;
        let (mut inputs, outputs) = inputs_outputs_valid();
        String::remove(&mut inputs[0].payment_address, 5);

        let signed_inputs = MockedFees::sign_inputs(wallet_handle, &inputs, &outputs).unwrap_err();

        assert_eq!(ErrorCode::CommonInvalidStructure, signed_inputs);
    }

    #[test]
    fn sign_multi_input() {
        let wallet_handle = 1;
        let (inputs, outputs) = inputs_outputs_valid();
        
        let expected_signed_inputs = vec![
            Input::new(String::from("pay:sov:SBD8oNfQNm1aEGE6KkYI1khYEGqG5zmEqrEw7maqKitIs121"), 1, Some(String::from("SBD8oNfQNm1aEGE6KkYI1khYEGqG5zmEqrEw7maqKitIsigned"))),
            Input::new(String::from("pay:sov:hhX4LejW7N23hPwC2yLKdor1ppXy3RhJ38TeXCZLgoBMSGfg"), 1, Some(String::from("hhX4LejW7N23hPwC2yLKdor1ppXy3RhJ38TeXCZLgoBMsigned"))),
        ];
        
        let signed_inputs = MockedFees::sign_inputs(wallet_handle, &inputs, &outputs).unwrap();
        assert_eq!(expected_signed_inputs, signed_inputs);
    }
}
