attribute vec4 position;
varying vec4 v_color;

void main() {
    gl_Position = position;

    v_color = gl_Position * 0.66 + 0.33;
}