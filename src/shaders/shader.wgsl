struct VertexInput
{
    @location(0) position : vec3<f32>,
    @location(1) colour : vec2<f32>
};

struct VertexOutput
{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) colour : vec3<f32>
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(
    model : VertexInput
) -> VertexOutput   
{
    var out : VertexOutput;
    out.colour = vec3<f32>(model.colour, 1.0);
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>
{
    return vec4<f32>(in.colour, 1.0);
}