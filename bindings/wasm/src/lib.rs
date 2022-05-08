mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn tokenize(input: &str) -> Vec<JsValue> {
    cribrum::tokenize(&input)
        .into_iter()
        .map(|t| t.raw.into())
        .collect()
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("No global window exists?");
    let doc = window.document().expect("Should have a document in window");
    let body = doc.body().expect("document should have a body");

    let val = doc.create_element("h2")?;
    val.set_text_content(Some(&format!("{:?}", tokenize("From within rust"))));

    body.append_child(&val)?;

    Ok(())
}
