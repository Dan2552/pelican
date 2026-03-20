use std::ffi::{CStr, CString};

pub fn set_string(text: &str) {
    let c_str = CString::new(text).expect("text contained null byte");
    let result = unsafe { sdl2::sys::SDL_SetClipboardText(c_str.as_ptr()) };
    if result != 0 {
        panic!("SDL_SetClipboardText failed: {}", result);
    }
}

pub fn get_string() -> Option<String> {
    let buf = unsafe { sdl2::sys::SDL_GetClipboardText() };
    if buf.is_null() {
        None
    } else {
        let text = unsafe { CStr::from_ptr(buf).to_str().expect("clipboard text was not valid UTF-8").to_owned() };
        unsafe { sdl2::sys::SDL_free(buf as *mut sdl2::libc::c_void) };
        Some(text)
    }
}

pub fn contains_text() -> bool {
    unsafe { sdl2::sys::SDL_HasClipboardText() == sdl2::sys::SDL_bool::SDL_TRUE }
}
