/*!
 * A set of test helpers dealing with the wallet.
 */

extern crate env_logger;
extern crate rust_indy_sdk as indy;
extern crate sovtoken;

use self::indy::ErrorCode;
use self::indy::wallet::Wallet as IndyWallet;
use self::sovtoken::utils::random::rand_string;

static USEFUL_CREDENTIALS : &'static str = r#"
   {
       "key": "12345678901234567890123456789012"
   }
"#;

/**
A test wallet that deletees itself when it leaves scope.

Use by calling `let wallet = Wallet::new();` and pass the `wallet.handle`.

```
use utils::wallet::Wallet;
// The wallet is opened and created.
let wallet_1 = Wallet::new();
{
    let wallet_2 = Wallet::new();
    // we have the wallet and wallet handle.
    assert!(wallet.handle > 0);
}
// Now wallet_2 is out of scope, it closes and deletes itself.
assert!(wallet.handle > 0);
```

*/
pub struct Wallet {
    name: String,
    pub handle: i32,
}

impl Wallet {

    pub fn new() -> Wallet {
        let wallet_name : String = rand_string(20);
        let config = Wallet::create_wallet_config(&wallet_name);
        let mut wallet = Wallet { name : wallet_name , handle: -1 };
        wallet.create(&config).unwrap();
        wallet.open().unwrap();

        return wallet;
    }

    pub fn from_name(name: &str) -> Wallet {
        let config = Wallet::create_wallet_config(name);
        let mut wallet = Wallet { name: name.to_string(), handle: -1 };
        wallet.create(&config).unwrap();
        wallet.open().unwrap();

        return wallet;
    }

    fn open(&mut self) -> Result<i32, ErrorCode> {
        let config : String = Wallet::create_wallet_config(&self.name);
        let handle = IndyWallet::open(&config, USEFUL_CREDENTIALS)?;
        self.handle = handle;
        return Ok(handle);
    }

    fn create_wallet_config(wallet_name: &str) -> String {
        let config = json!({ "id" : wallet_name.to_string() }).to_string();
        return config.to_string();
    }

    fn create(&self, config: &str) -> Result<(), ErrorCode> {
        IndyWallet::create(&config, USEFUL_CREDENTIALS)
    }

    fn close(&self) -> Result<(), ErrorCode> {
        IndyWallet::close(self.handle)
    }

    fn delete(&self) -> Result<(), ErrorCode> {
        let config : String = Wallet::create_wallet_config(&self.name);
        IndyWallet::delete(&config, USEFUL_CREDENTIALS)
    }
}

impl Drop for Wallet {
    fn drop(&mut self) {
        self.close().unwrap();
        self.delete().unwrap();
    }
}
