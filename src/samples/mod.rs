use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;

mod triangles;
mod rectangle;

pub fn draw_triangles(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    triangles::draw(&canvas)?;

    Ok(())
}

pub fn _draw_rectangle(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    rectangle::_draw(&canvas)?;

    Ok(())
}
