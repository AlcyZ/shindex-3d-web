use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;

mod triangles;
mod rectangle;

#[allow(dead_code)]
pub fn draw_triangles(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    triangles::draw(&canvas)?;

    Ok(())
}

#[allow(dead_code)]
pub fn draw_rectangle(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    rectangle::draw(&canvas)?;

    Ok(())
}
