
use gfx;
use gfx::DeviceHelper;
use graphics::{
    BackEnd,
};

use Texture;

static BUFFER_SIZE: uint = 1024;

static VERTEX_SHADER: gfx::ShaderSource = shaders!{
    GLSL_120: b"
#version 120
attribute vec2 pos;
attribute vec3 color;
varying vec4 v_Color;
void main() {
    v_Color = vec4(color, 1.0);
    gl_Position = vec4(pos, 0.0, 1.0);
}
"
    GLSL_150: b"
#version 150 core
in vec2 pos;
in vec3 color;
out vec4 v_Color;
void main() {
    v_Color = vec4(color, 1.0);
    gl_Position = vec4(pos, 0.0, 1.0);
}
"
};

static FRAGMENT_SHADER: gfx::ShaderSource = shaders!{
    GLSL_120: b"
#version 120
varying vec4 v_Color;
void main() {
    gl_FragColor = v_Color;
}
"
    GLSL_150: b"
#version 150 core
in vec4 v_Color;
out vec4 o_Color;
void main() {
    o_Color = v_Color;
}
"
};

static VERTEX_SHADER_UV: gfx::ShaderSource = shaders!{
    GLSL_120: b"
#version 120
attribute vec2 pos;
attribute vec3 color;
attribute vec2 uv;
uniform sampler2D s_texture;
varying vec4 v_Color;
varying vec2 v_UV;
void main() {
    v_UV = uv;
    v_Color = color;
    gl_Position = a_v4Position;
}
"
    GLSL_150: b"
#version 150 core
in vec2 pos;
in vec3 color;
in vec2 uv;
uniform sampler2D s_texture;
out vec4 v_Color;
out vec2 v_UV;
void main() {
    v_UV = uv;
    v_Color = color;
    gl_Position = a_v4Position;
}
"
};

static FRAGMENT_SHADER_UV: gfx::ShaderSource = shaders!{
    GLSL_120: b"
#version 120
uniform sampler2D s_texture;
varying vec2 v_UV;
varying vec4 v_Color;
void main()
{
    gl_FragColor = texture(s_texture, v_UV) * v_Color;
}
"
    GLSL_150: b"
#version 150 core
out vec4 o_Color;
uniform sampler2D s_texture;
in vec2 v_UV;
in vec4 v_Color;
void main()
{
    o_Color = texture(s_texture, v_UV) * v_Color;
}
"
};

#[vertex_format]
struct Vertex {
    pos: [f32, ..2],
    color: [f32, ..4],
}

impl Vertex {
    fn new(pos: [f32, ..2], color: [f32, ..4], uv: [f32, ..2]) -> Vertex {
        Vertex {
            pos: pos,
            color: color,
        }
    }
}

#[vertex_format]
struct VertexUV {
    pos: [f32, ..2],
    color: [f32, ..4],
    uv: [f32, ..2],
}

impl VertexUV {
    fn new(pos: [f32, ..2], color: [f32, ..4], uv: [f32, ..2]) -> VertexUV {
        VertexUV {
            pos: pos,
            color: color,
            uv: uv,
        }
    }
}

/// The graphics back-end.
pub struct Gfx2d {
    state: gfx::DrawState,
    program: gfx::shade::EmptyProgram,
    program_uv: gfx::shade::EmptyProgram,
    vertex_data: Vec<Vertex>,
    vertex_data_uv: Vec<VertexUV>,
}

impl Gfx2d {
    /// Creates a new Gfx2d object.
    pub fn new<D: gfx::Device>(device: &mut D) -> Gfx2d {
        Gfx2d {
            state: gfx::DrawState::new(),
            program: device.link_program(
                    VERTEX_SHADER.clone(),
                    FRAGMENT_SHADER.clone()
                ).unwrap(),
            program_uv: device.link_program(
                    VERTEX_SHADER_UV.clone(),
                    FRAGMENT_SHADER_UV.clone()
                ).unwrap(),
            vertex_data: Vec::with_capacity(BUFFER_SIZE),
            vertex_data_uv: Vec::with_capacity(BUFFER_SIZE)
        }
    }
}

impl BackEnd<Texture> for Gfx2d {

}
