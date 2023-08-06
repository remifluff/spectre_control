#import frag::utility

[[stage(fragment)]]
fn main(in: VertexOutput) -> FragmentOutput {

    let x = in.tex_coords.x % 0.2;
    let y = in.tex_coords.y % 0.2;


    let out = vec4<f32>(x, y, 0.0, 1.0);


    return FragmentOutput(out);
}
