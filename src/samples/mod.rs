use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;

mod rectangle;
mod rectangle_index_buffer;
mod triangles;

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

#[allow(dead_code)]
pub fn draw_rectangle_with_index_buffer(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    rectangle_index_buffer::draw(&canvas)?;

    Ok(())
}
