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

