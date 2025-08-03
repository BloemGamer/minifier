use std::ffi::{CStr, CString};
use std::os::raw::c_char;
mod minify;

#[unsafe(no_mangle)]
pub extern "C" fn minify(input: *const c_char) -> *mut c_char
{
    let c_str = unsafe { CStr::from_ptr(input) };
    let input_str = c_str.to_string_lossy(); // Handles UTF-8 safely
    
    // Your minification logic here
    let minified = minify::minify_file(input_str.to_string());
    
    // Convert back to C string (UTF-8)
    let c_string = CString::new(minified).unwrap();
    c_string.into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn free_string(s: *mut c_char)
{
    unsafe
    {
        if s.is_null() { return }
        let _ = CString::from_raw(s);
    };
}
