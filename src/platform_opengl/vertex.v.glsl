#version 140

in vec4 position;
out vec2 uv;

void main() {
    gl_Position = vec4(position.xy, 0.0, 1.0);
    uv = position.zw;
}
