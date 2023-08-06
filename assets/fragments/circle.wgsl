#import frag::utility




[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
        let line_amount = PARAMATER0(in.tex_coords);

        let line_angle = PARAMATER1(in.tex_coords);

    let xy = in.tex_coords;
    let xy = xy - vec2<f32>(mouseuniform.x, mouseuniform.y);
    let lines = pow(xy.x, 2.0) + pow(xy.y, 2.0);

    return vec4<f32>(lines, lines, lines, 1.0);
    // return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}