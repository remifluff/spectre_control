[[group(0), binding(0)]]
var tex0: texture_2d<f32>;
[[group(0), binding(1)]]
var tex_sampler0: sampler;
[[group(1), binding(0)]]
var tex1: texture_2d<f32>;
[[group(1), binding(1)]]
var tex_sampler1: sampler;
[[group(2), binding(0)]]
var tex2: texture_2d<f32>;
[[group(2), binding(1)]]
var tex_sampler2: sampler;
[[group(3), binding(0)]]
var tex3: texture_2d<f32>;
[[group(3), binding(1)]]
var tex_sampler3: sampler;
[[block]]
struct ParameterSet { 
    input0: f32;
    input1: f32;
    input2: f32;
    input3: f32;
    input4: f32;
    input5: f32;
    input6: f32;
    input7: f32;
}; 
[[group(4), binding(0)]] 
 var<uniform> parameterset: ParameterSet; 
[[block]]
struct MouseUniform { 
    x: f32;
    y: f32;
}; 
[[group(5), binding(0)]] 
 var<uniform> mouseuniform: MouseUniform; 
fn PARAMATER0(location: vec2<f32>) -> vec4<f32> {
    var param = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    var sample0 = textureSample(tex0, tex_sampler0, location);
    sample0 = sample0 * parameterset.input0;
    param = param + sample0;
    var sample1 = textureSample(tex1, tex_sampler1, location);
    sample1 = sample1 * parameterset.input1;
    param = param + sample1;
    var sample2 = textureSample(tex2, tex_sampler2, location);
    sample2 = sample2 * parameterset.input2;
    param = param + sample2;
    var sample3 = textureSample(tex3, tex_sampler3, location);
    sample3 = sample3 * parameterset.input3;
    param = param + sample3;
} 
struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};


[[stage(vertex)]]
fn main(model: VertexInput) -> VertexOutput {



    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}




struct FragmentOutput {
    [[location(0)]] f_color: vec4<f32>;
};


// fn PARAMATER(index: i32, location: vec2<f32>) -> vec4<f32> {
//     var num: f32 = f32(index);
//     return vec4<f32>(num, num, num, num);
// }





[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let xy = in.tex_coords;
    let xy = xy - vec2<f32>(mouseuniform.x, mouseuniform.y);
    let lines = pow(xy.x, 2.0) + pow(xy.y, 2.0);

    return vec4<f32>(lines, lines, lines, 1.0);
    // return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
 