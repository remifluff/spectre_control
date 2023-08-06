#import frag::utility




[[stage(fragment)]]
fn main(in: VertexOutput) -> FragmentOutput {
    let dsdsadsad = PARAMATER0(in.tex_coords);
    let line_test_name = PARAMATER1(in.tex_coords);



    let out = dsdsadsad;

    var x = 1;
    x = x + 1;


    let xy = in.tex_coords % vec2<f32>(mouseuniform.x, mouseuniform.y) ;
    let lines = xy.x % out.x ;


    // let out = vec4<f32>(lines, lines, lines, 1.0);

    return FragmentOutput(out);
}

 