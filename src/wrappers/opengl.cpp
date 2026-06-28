#define _POSIX_C_SOURCE 199309L
#define GL_GLEXT_PROTOTYPES
#include <GL/gl.h>
#include <GL/glu.h>
#include <GL/glx.h>
#include <GL/glext.h>
#include <X11/X.h>
#include <X11/Xlib.h>
#include <time.h>
#include <cstring>

extern "C" {

static Display* gl_display = nullptr;
static Window gl_window = 0;
static GLXContext gl_ctx = nullptr;

int glfw_init() {
    gl_display = XOpenDisplay(nullptr);
    if (!gl_display) return -1;
    return 0;
}

void* glfw_create_window(int width, int height, const char* title) {
    if (!gl_display) return nullptr;

    int screen = DefaultScreen(gl_display);
    Window root = RootWindow(gl_display, screen);

    int attr[] = { GLX_RGBA, GLX_DEPTH_SIZE, 24, GLX_DOUBLEBUFFER, None };
    XVisualInfo* vi = glXChooseVisual(gl_display, screen, attr);
    if (!vi) return nullptr;

    XSetWindowAttributes swa;
    swa.colormap = XCreateColormap(gl_display, root, vi->visual, AllocNone);
    swa.event_mask = ExposureMask | KeyPressMask | ButtonPressMask | StructureNotifyMask;

    Window win = XCreateWindow(gl_display, root, 0, 0, width, height, 0,
                               vi->depth, InputOutput, vi->visual,
                               CWColormap | CWEventMask, &swa);

    XStoreName(gl_display, win, title);
    XMapWindow(gl_display, win);

    gl_ctx = glXCreateContext(gl_display, vi, nullptr, GL_TRUE);
    XFree(vi);

    if (!gl_ctx) return nullptr;

    gl_window = win;
    return (void*)(uintptr_t)win;
}

void glfw_make_context_current(void* window) {
    if (gl_display && gl_ctx) {
        glXMakeCurrent(gl_display, (Window)(uintptr_t)window, gl_ctx);
    }
}

void glfw_swap_buffers(void* window) {
    if (gl_display) {
        glXSwapBuffers(gl_display, (Window)(uintptr_t)window);
    }
}

void glfw_poll_events() {
    if (!gl_display) return;
    while (XPending(gl_display)) {
        XEvent ev;
        XNextEvent(gl_display, &ev);
    }
}

int glfw_window_should_close(void*) {
    return 0;
}

void glfw_destroy_window(void* window) {
    if (gl_ctx) {
        glXDestroyContext(gl_display, gl_ctx);
        gl_ctx = nullptr;
    }
    if (gl_window) {
        XDestroyWindow(gl_display, (Window)(uintptr_t)window);
        gl_window = 0;
    }
}

void glfw_terminate() {
    if (gl_display) {
        XCloseDisplay(gl_display);
        gl_display = nullptr;
    }
}

double glfw_get_time() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec + ts.tv_nsec / 1e9;
}

void gl_clear_color(float r, float g, float b, float a) {
    glClearColor(r, g, b, a);
}

void gl_clear(int mask) {
    glClear((GLbitfield)mask);
}

void gl_viewport(int x, int y, int w, int h) {
    glViewport(x, y, w, h);
}

void gl_begin(int mode) {
    glBegin((GLenum)mode);
}

void gl_end() {
    glEnd();
}

void gl_vertex2f(float x, float y) {
    glVertex2f(x, y);
}

void gl_vertex3f(float x, float y, float z) {
    glVertex3f(x, y, z);
}

void gl_color3f(float r, float g, float b) {
    glColor3f(r, g, b);
}

void gl_color4f(float r, float g, float b, float a) {
    glColor4f(r, g, b, a);
}

void gl_load_identity() {
    glLoadIdentity();
}

void gl_translatef(float x, float y, float z) {
    glTranslatef(x, y, z);
}

void gl_rotatef(float angle, float x, float y, float z) {
    glRotatef(angle, x, y, z);
}

void gl_ortho(float left, float right, float bottom, float top, float near_val, float far_val) {
    glOrtho(left, right, bottom, top, near_val, far_val);
}

void gl_matrix_mode(int mode) {
    glMatrixMode((GLenum)mode);
}

void gl_enable(int cap) {
    glEnable((GLenum)cap);
}

void gl_disable(int cap) {
    glDisable((GLenum)cap);
}

void gl_flush() {
    glFlush();
}

int gl_get_error() {
    return (int)glGetError();
}

unsigned int gl_create_shader(int type) {
    return glCreateShader((GLenum)type);
}

void gl_shader_source(unsigned int shader, const char* source) {
    glShaderSource(shader, 1, &source, nullptr);
}

void gl_compile_shader(unsigned int shader) {
    glCompileShader(shader);
}

unsigned int gl_create_program() {
    return glCreateProgram();
}

void gl_attach_shader(unsigned int program, unsigned int shader) {
    glAttachShader(program, shader);
}

void gl_link_program(unsigned int program) {
    glLinkProgram(program);
}

void gl_use_program(unsigned int program) {
    glUseProgram(program);
}

} // extern "C"
