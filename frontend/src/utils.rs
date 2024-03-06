use wasm_bindgen::UnwrapThrowExt;

pub fn is_https() -> bool {
    let window = web_sys::window().unwrap_throw();
    let location = window.location();
    location.protocol().unwrap_throw().starts_with("https")
}

pub fn port() -> u32 {
    let window = web_sys::window().unwrap_throw();
    let location = window.location();
    location.port().unwrap_throw().parse().unwrap_throw()
}

pub fn host() -> String {
    let window = web_sys::window().unwrap_throw();
    let location = window.location();
    location.hostname().unwrap_throw()
}