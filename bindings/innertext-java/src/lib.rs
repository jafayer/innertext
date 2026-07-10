use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use innertext_core::Document;

/// Extract innerText from HTML
/// 
/// @param html HTML string
/// @return rendered text content
#[no_mangle]
pub extern "system" fn Java_com_innertext_InnerText_innerText<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    html: JString<'a>,
) -> jstring {
    let html_str: String = match env.get_string(&html) {
        Ok(s) => s.into(),
        Err(_) => {
            return env
                .new_string("Invalid HTML string")
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut());
        }
    };

    match Document::parse(&html_str) {
        Ok(doc) => {
            let result = doc.inner_text();
            env.new_string(result)
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut())
        }
        Err(_) => {
            env.new_string("Failed to parse HTML")
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut())
        }
    }
}

/// Extract outerText from HTML
/// 
/// @param html HTML string
/// @return outer text content
#[no_mangle]
pub extern "system" fn Java_com_innertext_InnerText_outerText<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    html: JString<'a>,
) -> jstring {
    let html_str: String = match env.get_string(&html) {
        Ok(s) => s.into(),
        Err(_) => {
            return env
                .new_string("Invalid HTML string")
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut());
        }
    };

    match Document::parse(&html_str) {
        Ok(doc) => {
            let result = doc.outer_text();
            env.new_string(result)
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut())
        }
        Err(_) => {
            env.new_string("Failed to parse HTML")
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut())
        }
    }
}

/// Extract textContent from HTML
/// 
/// @param html HTML string
/// @return structural text content
#[no_mangle]
pub extern "system" fn Java_com_innertext_InnerText_textContent<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    html: JString<'a>,
) -> jstring {
    let html_str: String = match env.get_string(&html) {
        Ok(s) => s.into(),
        Err(_) => {
            return env
                .new_string("Invalid HTML string")
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut());
        }
    };

    match Document::parse(&html_str) {
        Ok(doc) => {
            let result = doc.text_content();
            env.new_string(result)
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut())
        }
        Err(_) => {
            env.new_string("Failed to parse HTML")
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut())
        }
    }
}
