use std::ffi::c_void;
use std::time::Instant;

#[link(name = "GLESv2")]
extern "C" {
    fn glViewport(x: i32, y: i32, width: i32, height: i32);
    fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32);
    fn glClear(mask: u32);
    fn glCreateShader(shader_type: u32) -> u32;
    fn glShaderSource(shader: u32, count: i32, string: *const *const u8, length: *const i32);
    fn glCompileShader(shader: u32);
    fn glCreateProgram() -> u32;
    fn glAttachShader(program: u32, shader: u32);
    fn glLinkProgram(program: u32);
    fn glUseProgram(program: u32);
    fn glGetAttribLocation(program: u32, name: *const u8) -> i32;
    fn glGetUniformLocation(program: u32, name: *const u8) -> i32;
    fn glEnableVertexAttribArray(index: u32);
    fn glVertexAttribPointer(index: u32, size: i32, type_: u32, normalized: u8, stride: i32, pointer: *const c_void);
    fn glUniform1f(location: i32, v0: f32);
    fn glUniform2f(location: i32, v0: f32, v1: f32);
    fn glDrawArrays(mode: u32, first: i32, count: i32);
}

const GL_COLOR_BUFFER_BIT: u32 = 0x4000;
const GL_TRIANGLE_STRIP: u32 = 0x0005;
const GL_FLOAT: u32 = 0x1406;
const GL_FRAGMENT_SHADER: u32 = 0x8B30;
const GL_VERTEX_SHADER: u32 = 0x8B31;

const VERT_SRC: &[u8] = b"attribute vec2 position;\nvoid main() { gl_Position = vec4(position, 0.0, 1.0); }\0";

const FRAG_SRC: &[u8] = b"precision mediump float;\n\
uniform vec2 u_resolution;\n\
uniform float u_time;\n\
uniform vec2 u_touch;\n\
void main() {\n\
    vec2 uv = gl_FragCoord.xy / u_resolution.xy;\n\
    vec2 p = uv * 2.0 - 1.0;\n\
    p.x *= u_resolution.x / u_resolution.y;\n\
    float d = length(p - (u_touch * 2.0 - 1.0));\n\
    float c = sin(d * 18.0 - u_time * 4.0) * 0.5 + 0.5;\n\
    vec3 col = vec3(0.1 + 0.9 * c, 0.4 * uv.y, 0.8 * uv.x);\n\
    gl_FragColor = vec4(col, 1.0);\n\
}\0";

static QUAD: [f32; 8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];

pub struct AppState {
    program: u32,
    u_resolution: i32,
    u_time: i32,
    u_touch: i32,
    touch_x: f32,
    touch_y: f32,
    start_time: Option<Instant>,
    initialized: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            program: 0,
            u_resolution: -1,
            u_time: -1,
            u_touch: -1,
            touch_x: 0.5,
            touch_y: 0.5,
            start_time: None,
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        if self.initialized { return; }
        unsafe {
            let vs = glCreateShader(GL_VERTEX_SHADER);
            let vs_ptr = VERT_SRC.as_ptr();
            glShaderSource(vs, 1, &vs_ptr, &(VERT_SRC.len() as i32));
            glCompileShader(vs);

            let fs = glCreateShader(GL_FRAGMENT_SHADER);
            let fs_ptr = FRAG_SRC.as_ptr();
            glShaderSource(fs, 1, &fs_ptr, &(FRAG_SRC.len() as i32));
            glCompileShader(fs);

            let prog = glCreateProgram();
            glAttachShader(prog, vs);
            glAttachShader(prog, fs);
            glLinkProgram(prog);

            self.program = prog;
            self.u_resolution = glGetUniformLocation(prog, b"u_resolution\0".as_ptr());
            self.u_time = glGetUniformLocation(prog, b"u_time\0".as_ptr());
            self.u_touch = glGetUniformLocation(prog, b"u_touch\0".as_ptr());
            self.start_time = Some(Instant::now());
            self.initialized = true;
        }
    }

    pub fn on_touch(&mut self, _action: i32, x: f32, y: f32, w: i32, h: i32) {
        if w > 0 && h > 0 {
            self.touch_x = x / w as f32;
            self.touch_y = 1.0 - (y / h as f32);
        }
    }

    pub fn render(&mut self, width: i32, height: i32) {
        if !self.initialized || width <= 0 || height <= 0 { return; }
        unsafe {
            glViewport(0, 0, width, height);
            glClearColor(0.0, 0.0, 0.0, 1.0);
            glClear(GL_COLOR_BUFFER_BIT);

            glUseProgram(self.program);

            let elapsed = self.start_time.map_or(0.0, |t| t.elapsed().as_secs_f32());
            if self.u_time >= 0 { glUniform1f(self.u_time, elapsed); }
            if self.u_resolution >= 0 { glUniform2f(self.u_resolution, width as f32, height as f32); }
            if self.u_touch >= 0 { glUniform2f(self.u_touch, self.touch_x, self.touch_y); }

            let pos_loc = glGetAttribLocation(self.program, b"position\0".as_ptr()) as u32;
            glEnableVertexAttribArray(pos_loc);
            glVertexAttribPointer(pos_loc, 2, GL_FLOAT, 0, 0, QUAD.as_ptr() as _);

            glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);
        }
    }
}
