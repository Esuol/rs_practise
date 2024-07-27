use std::ffi::{c_char, CString};

#[no_mangle]
pub extern "C" fn hello_rust() -> *mut c_char {
    CString::new("Hello Rust").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_hello(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        // CString::from_raw 函数会将一个原始指针转换为 CString，并接管该指针的所有权。
        // 为了正确释放内存，我们需要显式地丢弃这个 CString 对象。可以通过调用 drop 函数来实现，或者使用 let _ = 来忽略返回值
        drop(CString::from_raw(s));
    };
}
