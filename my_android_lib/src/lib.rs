extern crate jni;

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jstring;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn Java_com_example_rustintegrationapp_MyRustLib_hello_1from_1rust(env: JNIEnv, _: JClass) -> jstring {
    let output = CString::new("This message was build from Rust!").unwrap();
    let jni_string = env.new_string(output.to_str().unwrap()).unwrap();
    jni_string.into_raw()
}
