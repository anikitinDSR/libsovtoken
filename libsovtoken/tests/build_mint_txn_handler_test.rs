
#[allow(unused_variables)]
#[allow(dead_code)]

extern crate libc;

extern crate sovtoken;
extern crate indy;                      // lib-sdk project

use libc::c_char;
use std::ptr;
use std::ffi::CString;

use indy::api::ErrorCode;
// ***** HELPER METHODS *****
extern "C" fn empty_create_payment_callback(command_handle_: i32, err: ErrorCode, payment_address: *const c_char) { }

// ***** HELPER TEST DATA  *****

const COMMAND_HANDLE:i32 = 10;
static INVALID_OUTPUT_JSON: &'static str = r#"{"totally" : "Not a Number", "bobby" : "DROP ALL TABLES"}"#;
const cb : Option<extern fn(command_handle_: i32, err: ErrorCode, payment_address: *const c_char)> = Some(empty_create_payment_callback);

// ***** UNIT TESTS ****

// the build_mint_txn_handler requires a callback and this test ensures that we
// receive an error when no callback is provided
#[test]
fn errors_with_no_call_back() {
    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, ptr::null(), None);
    assert_eq!(return_error, ErrorCode::CommonInvalidParam3, "Expecting Callback for 'build_mint_txn_handler'"); 
}

// the build mint txn handler method requires an outputs_json parameter and this test ensures that 
// a error is returned when no config is provided
#[test]
fn errors_with_no_outputs_json() {
    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, ptr::null(), cb);
    assert_eq!(return_error, ErrorCode::CommonInvalidParam2, "Expecting outputs_json for 'build_mint_txn_handler'");
}

// // the mint txn handler method requires a valid JSON format (format is described
// in build_mint_fees_handler description).  Expecting error when invalid json is inputted
#[test]
fn errors_with_invalid_output_json() {
    let output_str = CString::new(INVALID_OUTPUT_JSON).unwrap();
    let output_str_ptr = output_str.as_ptr();
    let return_error = sovtoken::api::build_mint_txn_handler(COMMAND_HANDLE, output_str_ptr, cb);
    assert_eq!(return_error, ErrorCode::CommonInvalidStructure, "Expecting Valid JSON for 'build_mint_txn_handler'");
}