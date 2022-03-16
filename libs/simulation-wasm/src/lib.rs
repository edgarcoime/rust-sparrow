use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn whos_that_dog() -> String {
    "That's not a dog that's a turtle".into()
}