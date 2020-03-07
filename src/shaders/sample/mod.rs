pub const VERTEX_SHADER: &str = r#"
attribute vec4 position;
attribute vec4 color;

varying vec4 out_color;

void main() {
    out_color = color;

    gl_Position = position;
}
"#;
pub const FRAGMENT_SHADER: &str = r#"
precision mediump float;

varying vec4 out_color;

void main() {
    gl_FragColor = out_color;
}
"#;
