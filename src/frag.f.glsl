precision mediump float;

varying vec2 uv;

void main() {
    float edge_thickness  = 0.10;
    float edge_smoothness = 0.06;

    // 1 at edges, 0 in the middle
    float edge = max(
            smoothstep(1.0 - edge_thickness, 1.0 - edge_thickness + edge_smoothness, uv.x), // right
            smoothstep(      edge_thickness,       edge_thickness - edge_smoothness, uv.x)  // left
        );

    gl_FragColor = vec4(0.0, 0.0, 0.0, max(0.1, edge));
}
