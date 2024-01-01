struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal : vec3<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_normal : vec3<f32>,
    @location(2) world_position : vec3<f32>
}

struct ModelInformation {
    matrix : mat4x4<f32>,
};

@group(2) @binding(0)
var<uniform> model_information : ModelInformation;

struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_pos: vec4<f32>
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct LightUniform {
    position: vec3<f32>,
    colour: vec3<f32>
};
@group(1) @binding(0)
var<uniform> light : LightUniform;

@vertex
fn vs_main(
    model : VertexInput
) -> VertexOutput   
{
    var out : VertexOutput;
    out.tex_coords = model.tex_coords;
    out.world_normal = normalize(model_information.matrix * vec4<f32>(model.normal, 0.0)).xyz;
    var world_position : vec4<f32> = model_information.matrix * vec4<f32>(model.position, 1.0);
    out.world_position = world_position.xyz;
    out.clip_position = camera.view_proj * world_position;
    return out;
}


// @group(1) @binding(0)
// var t_diffuse: texture_2d<f32>;
// @group(1) @binding(1)
// var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>
{
    // let object_col: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);

    let object_col: vec4<f32> = vec4<f32>(in.world_normal, 1.0);

    let ambient_strength = 0.1;
    let ambient_colour = light.colour * ambient_strength;

    let light_dir = normalize(light.position - in.world_position);

    let diffuse_strength = max(dot(in.world_normal, light_dir), 0.0);
    let diffuse_colour = light.colour * diffuse_strength;

    let view_dir = normalize(camera.view_pos.xyz - in.world_position);
    let half_dir = normalize(view_dir + light_dir);
    let reflect_dir = reflect(-light_dir, in.world_normal);

    let specular_strength = pow(max(dot(in.world_normal, half_dir), 0.0), 32.0);
    let specular_colour = light.colour * specular_strength;

    let result = (ambient_colour + diffuse_colour + specular_colour) * object_col.xyz; 

    return vec4<f32>(result, object_col.a);
}