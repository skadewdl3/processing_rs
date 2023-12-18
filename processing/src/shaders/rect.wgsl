struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) color: vec4<f32>
}

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) color: vec4<f32>
}

@vertex
fn vs_main (vertex_data: VertexInput) -> VertexOutput {

  var output: VertexOutput;
  output.position = vec4<f32>(vertex_data.position, 1.0);
  output.color = vertex_data.color;

  return output;
}


@fragment
fn fs_main (fragment_data: VertexOutput) -> @location(0) vec4<f32> {
  return fragment_data.color;
}