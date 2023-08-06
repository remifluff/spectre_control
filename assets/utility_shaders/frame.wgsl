// Vertex shader

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};


struct VertexOutput {
    [[location(0)]] tex_coords: vec2<f32>;
    [[builtin(position)]] clip_position: vec4<f32>;
    
};

[[stage(vertex)]]
fn main(model: VertexInput) -> VertexOutput {

    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}



[[group(0), binding(0)]]
var tex: texture_2d<f32>;
[[group(0), binding(1)]]
var tex_sampler: sampler;

struct FragmentOutput {
    [[location(0)]] f_color: vec4<f32>;
};



// [[group(0), binding(0)]]
// var tex: texture_2d<f32>;
// [[group(0), binding(1)]]
// var tex: texture_2d<f32>;
// [[group(0), binding(2)]]
// var tex_sampler: sampler;

[[stage(fragment)]]
fn main(in: VertexOutput) -> FragmentOutput {

    var inputOne = vec3<f32>(0.0, 0.0, 0.0);

    let out = textureSample(tex, tex_sampler, in.tex_coords);

    return FragmentOutput(out);
}
 

 

 