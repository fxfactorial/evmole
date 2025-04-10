use alloy_primitives::{hex, Selector};
use serde_json;
use std::ffi::CStr;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn produce_selectors(message: *const libc::c_char) -> *const libc::c_char {
    let message_cstr = unsafe { CStr::from_ptr(message) };
    let message = message_cstr.to_str().unwrap();
    match hex::decode(message) {
        Err(_) => CString::from(c"").into_raw(),
        Ok(code) => {
            let info = crate::contract_info(
                crate::ContractInfoArgs::new(&code)
                    .with_selectors()
                    .with_arguments()
                    .with_state_mutability(),
            );

            let payload = info
                .functions
                .expect("functions extraction was requested")
                .into_iter()
                .map(|f| {
                    (
                        <[u8; 4] as Into<Selector>>::into(f.selector),
                        f.arguments
                            .expect("arguments extraction was requested")
                            .into_iter()
                            .map(|t| t.sol_type_name().to_string())
                            .collect::<Vec<String>>()
                            .join(","),
                        f.state_mutability
                            .expect("state_mutability extraction was requested")
                            .as_json_str(),
                    )
                })
                .collect::<Vec<(Selector, String, &str)>>();
            let payload = serde_json::to_string(&payload).unwrap_or("".to_string());

            CString::new(payload)
                .unwrap_or(CString::from(c""))
                .into_raw()
        }
    }
}
