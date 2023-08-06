#import frag::utility






[[stage(fragment)]]
fn main(in: VertexOutput) -> FragmentOutput {
    // dsadasd   ;

    // ad
    // let x = Input(0.0, 0.0);

    let xy = in.tex_coords;
    let lines = xy.x % 0.2;


    let out = vec4<f32>(lines, lines, lines, 1.0);

    return FragmentOutput(out);
}
