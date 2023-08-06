// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.vert_pos = out.clip_position.xyz;

    return out;
}

 

 // Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let x = in.clip_position[0] / 1000.0;
    let y = in.clip_position[1] / 1000.0;
    let z = in.clip_position[2] / 1000.0;

    let x2 = in.vert_pos[0] + 0.5;
    let y2 = in.vert_pos[1] + 0.5;
    // let z2 = in.vert_pos[2] ;


    // return vec4<f32>(x, y, z, 1.0);
    return vec4<f32>(x2, y2, 0.4, 1.0);
}


 