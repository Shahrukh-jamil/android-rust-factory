use std::ffi::c_void;
use std::ptr::null_mut;
use log::error;

pub type EGLDisplay = *mut c_void;
pub type EGLConfig = *mut c_void;
pub type EGLSurface = *mut c_void;
pub type EGLContext = *mut c_void;
pub type EGLNativeWindowType = *mut c_void;
pub type EGLint = i32;

const EGL_DEFAULT_DISPLAY: *mut c_void = null_mut();
const EGL_NO_CONTEXT: EGLContext = null_mut();
const EGL_NO_DISPLAY: EGLDisplay = null_mut();
const EGL_NO_SURFACE: EGLSurface = null_mut();
const EGL_CONTEXT_CLIENT_VERSION: EGLint = 0x3098;
const EGL_OPENGL_ES2_BIT: EGLint = 4;
const EGL_SURFACE_TYPE: EGLint = 0x3033;
const EGL_WINDOW_BIT: EGLint = 4;
const EGL_RED_SIZE: EGLint = 0x3024;
const EGL_GREEN_SIZE: EGLint = 0x3023;
const EGL_BLUE_SIZE: EGLint = 0x3022;
const EGL_ALPHA_SIZE: EGLint = 0x3021;
const EGL_DEPTH_SIZE: EGLint = 0x3025;
const EGL_NONE: EGLint = 0x3038;
const EGL_WIDTH: EGLint = 0x3057;
const EGL_HEIGHT: EGLint = 0x3056;

#[link(name = "EGL")]
extern "C" {
    fn eglGetDisplay(display_id: *mut c_void) -> EGLDisplay;
    fn eglInitialize(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> u32;
    fn eglChooseConfig(dpy: EGLDisplay, attrib_list: *const EGLint, configs: *mut EGLConfig, config_size: EGLint, num_config: *mut EGLint) -> u32;
    fn eglCreateWindowSurface(dpy: EGLDisplay, config: EGLConfig, win: EGLNativeWindowType, attrib_list: *const EGLint) -> EGLSurface;
    fn eglCreateContext(dpy: EGLDisplay, config: EGLConfig, share_context: EGLContext, attrib_list: *const EGLint) -> EGLContext;
    fn eglMakeCurrent(dpy: EGLDisplay, draw: EGLSurface, read: EGLSurface, ctx: EGLContext) -> u32;
    fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> u32;
    fn eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> u32;
    fn eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> u32;
    fn eglTerminate(dpy: EGLDisplay) -> u32;
    fn eglQuerySurface(dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: *mut EGLint) -> u32;
}

pub struct Engine {
    display: EGLDisplay,
    surface: EGLSurface,
    context: EGLContext,
    config: EGLConfig,
    pub width: i32,
    pub height: i32,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            display: EGL_NO_DISPLAY,
            surface: EGL_NO_SURFACE,
            context: EGL_NO_CONTEXT,
            config: null_mut(),
            width: 0,
            height: 0,
        }
    }

    pub fn init_surface(&mut self, window: &android_activity::NativeWindow) {
        unsafe {
            if self.display == EGL_NO_DISPLAY {
                self.display = eglGetDisplay(EGL_DEFAULT_DISPLAY);
                eglInitialize(self.display, null_mut(), null_mut());

                let attribs = [
                    EGL_SURFACE_TYPE, EGL_WINDOW_BIT,
                    EGL_RENDERABLE_TYPE, EGL_OPENGL_ES2_BIT,
                    EGL_RED_SIZE, 8,
                    EGL_GREEN_SIZE, 8,
                    EGL_BLUE_SIZE, 8,
                    EGL_ALPHA_SIZE, 8,
                    EGL_DEPTH_SIZE, 16,
                    EGL_NONE,
                ];

                let mut num_config = 0;
                eglChooseConfig(self.display, attribs.as_ptr(), &mut self.config, 1, &mut num_config);

                let context_attribs = [EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE];
                self.context = eglCreateContext(self.display, self.config, EGL_NO_CONTEXT, context_attribs.as_ptr());
            }

            self.surface = eglCreateWindowSurface(self.display, self.config, window.ptr().as_ptr() as _, null_mut());
            eglMakeCurrent(self.display, self.surface, self.surface, self.context);

            let mut w = 0;
            let mut h = 0;
            eglQuerySurface(self.display, self.surface, EGL_WIDTH, &mut w);
            eglQuerySurface(self.display, self.surface, EGL_HEIGHT, &mut h);
            self.width = w;
            self.height = h;
        }
    }

    pub fn destroy_surface(&mut self) {
        unsafe {
            if self.surface != EGL_NO_SURFACE {
                eglMakeCurrent(self.display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
                eglDestroySurface(self.display, self.surface);
                self.surface = EGL_NO_SURFACE;
            }
        }
    }

    pub fn has_surface(&self) -> bool {
        self.surface != EGL_NO_SURFACE
    }

    pub fn swap_buffers(&self) {
        unsafe {
            if self.surface != EGL_NO_SURFACE {
                eglSwapBuffers(self.display, self.surface);
            }
        }
    }
}
