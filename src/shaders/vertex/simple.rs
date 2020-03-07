pub const SHADER: &str = r#"
attribute vec4 position;

void main() {
    gl_Position = position;
}
"#;