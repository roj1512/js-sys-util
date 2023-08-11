use js_sys::Array;
use js_sys::JsString;
use js_sys::Object;
use wasm_bindgen::JsValue;

pub fn create_null() -> JsValue {
    JsValue::NULL
}

pub fn create_boolean(b: bool) -> JsValue {
    if b {
        JsValue::TRUE
    } else {
        JsValue::FALSE
    }
}

pub fn create_number(number: f64) -> js_sys::Number {
    js_sys::Number::from(number)
}

pub fn create_string(string: &str) -> JsString {
    JsString::from(string.to_string())
}

pub fn create_array(entries_: &[&JsValue]) -> Array {
    let array = Array::new();
    for v in entries_ {
        array.push(v);
    }
    array
}

pub fn create_entry(key: &str, value: &JsValue) -> Array {
    create_array(&[&create_string(key), value])
}

pub fn create_object(entries: &[&JsValue]) -> Object {
    let entries = create_array(entries);
    Object::from_entries(&entries).unwrap()
}
