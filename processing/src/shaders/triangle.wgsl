struct Uniforms {
  x1: f32,
  x2: f32,
  x3: f32,
  y1: f32,
  y2: f32,
  y3: f32,
};

struct VOutput {
  @builtin(position) position: vec4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main (@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
  var pos = array<vec3<f32>, 3> (
    vec3<f32>(uniforms.x1, uniforms.y1, 0.0),
    vec3<f32>(uniforms.x2, uniforms.y2, 0.0),
    vec3<f32>(uniforms.x3, uniforms.y3, 0.0),
  );
  return vec4<f32>(pos[in_vertex_index], 1.0);
}


@fragment
fn fs_main (in: VOutput) -> @location(0) vec4<f32> {
  return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}