#import frag::utility

[[stage(fragment)]]
fn main(in: VertexOutput) -> FragmentOutput {
       let line_amount = PARAMATER0(in.tex_coords);

        let line_angle = PARAMATER1(in.tex_coords);
    var inputOne = vec3<f32>(0.0, 0.0, 0.0);

    let one = textureSample(tex0, tex_sampler0, in.tex_coords);
    let two = textureSample(tex1, tex_sampler1, in.tex_coords);
    let out = one + two / 2.0;
    return FragmentOutput(out);
}

