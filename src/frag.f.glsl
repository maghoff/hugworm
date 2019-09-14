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

    float middle =
            smoothstep(0.5 - (edge_thickness / 2.0), 0.5 - (edge_thickness / 2.0 - edge_smoothness), uv.x) *
            smoothstep(0.5 + (edge_thickness / 2.0 + edge_smoothness), 0.5 + (edge_thickness / 2.0), uv.x);

    float marking = middle * smoothstep(0.8, 0.9, abs(sin(uv.y * 20.0)));

    vec3 color = edge    * vec3(0.85, 0.89, 0.86) +
                 marking * vec3(0.92, 0.90, 0.60) +
                 (1.0 - marking - edge) * vec3(0.58, 0.62, 0.62);

    gl_FragColor = vec4(color, 1.0);
}
