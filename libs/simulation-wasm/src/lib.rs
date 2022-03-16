use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn whos_that_dog() -> String {
    "Hello who's dog is that?".into()
}