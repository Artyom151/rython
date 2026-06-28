#include <lvgl.h>
#include <cstring>
#include <cstdint>
#include <vector>

static lv_disp_draw_buf_t draw_buf;
static lv_disp_drv_t disp_drv;

static void lvgl_flush_cb(lv_disp_drv_t* drv, const lv_area_t* area, lv_color_t* color_p) {
    lv_disp_flush_ready(drv);
}

extern "C" {

int lvgl_init() {
    lv_init();
    return 0;
}

void* lvgl_create_display(int width, int height, void* buf1, void* buf2) {
    lv_disp_draw_buf_init(&draw_buf, buf1, buf2, width * height);
    lv_disp_drv_init(&disp_drv);
    disp_drv.hor_res = width;
    disp_drv.ver_res = height;
    disp_drv.flush_cb = lvgl_flush_cb;
    disp_drv.draw_buf = &draw_buf;
    return lv_disp_drv_register(&disp_drv);
}

void lvgl_tick_inc(int ms) {
    lv_tick_inc(ms);
}

void lvgl_task_handler() {
    lv_timer_handler();
}

void* lvgl_create_obj(void* parent) {
    return lv_obj_create((lv_obj_t*)parent);
}

void* lvgl_create_btn(void* parent) {
    return lv_btn_create((lv_obj_t*)parent);
}

void* lvgl_create_label(void* parent) {
    return lv_label_create((lv_obj_t*)parent);
}

void* lvgl_create_slider(void* parent) {
    return lv_slider_create((lv_obj_t*)parent);
}

void* lvgl_create_arc(void* parent) {
    return lv_arc_create((lv_obj_t*)parent);
}

void* lvgl_create_bar(void* parent) {
    return lv_bar_create((lv_obj_t*)parent);
}

void* lvgl_create_dropdown(void* parent) {
    return lv_dropdown_create((lv_obj_t*)parent);
}

void* lvgl_create_textarea(void* parent) {
    return lv_textarea_create((lv_obj_t*)parent);
}

void* lvgl_create_checkbox(void* parent) {
    return lv_checkbox_create((lv_obj_t*)parent);
}

void* lvgl_create_switch(void* parent) {
    return lv_switch_create((lv_obj_t*)parent);
}

void* lvgl_create_chart(void* parent) {
    return lv_chart_create((lv_obj_t*)parent);
}

void* lvgl_create_image(void* parent) {
    return lv_img_create((lv_obj_t*)parent);
}

void lvgl_obj_set_pos(void* obj, int x, int y) {
    lv_obj_set_pos((lv_obj_t*)obj, x, y);
}

void lvgl_obj_set_size(void* obj, int w, int h) {
    lv_obj_set_size((lv_obj_t*)obj, w, h);
}

void lvgl_obj_set_align(void* obj, int align) {
    lv_obj_set_align((lv_obj_t*)obj, (lv_align_t)align);
}

void lvgl_obj_center(void* obj) {
    lv_obj_center((lv_obj_t*)obj);
}

void lvgl_obj_add_flag(void* obj, int flag) {
    lv_obj_add_flag((lv_obj_t*)obj, (lv_obj_flag_t)flag);
}

void lvgl_obj_clear_flag(void* obj, int flag) {
    lv_obj_clear_flag((lv_obj_t*)obj, (lv_obj_flag_t)flag);
}

void lvgl_label_set_text(void* obj, const char* text) {
    lv_label_set_text((lv_obj_t*)obj, text);
}

void lvgl_btn_set_text(void* obj, const char* text) {
    lv_obj_t* btn = (lv_obj_t*)obj;
    lv_obj_t* label = lv_obj_get_child(btn, 0);
    if (!label) {
        label = lv_label_create(btn);
    }
    lv_label_set_text(label, text);
}

void lvgl_slider_set_value(void* obj, int value, int anim) {
    lv_slider_set_value((lv_obj_t*)obj, value, (lv_anim_enable_t)anim);
}

int lvgl_slider_get_value(void* obj) {
    return lv_slider_get_value((lv_obj_t*)obj);
}

void lvgl_textarea_set_text(void* obj, const char* text) {
    lv_textarea_set_text((lv_obj_t*)obj, text);
}

const char* lvgl_textarea_get_text(void* obj) {
    return lv_textarea_get_text((lv_obj_t*)obj);
}

void lvgl_dropdown_set_options(void* obj, const char* options) {
    lv_dropdown_set_options((lv_obj_t*)obj, options);
}

int lvgl_dropdown_get_selected(void* obj) {
    return lv_dropdown_get_selected((lv_obj_t*)obj);
}

void lvgl_arc_set_value(void* obj, int value) {
    lv_arc_set_value((lv_obj_t*)obj, value);
}

void lvgl_arc_set_range(void* obj, int min, int max) {
    lv_arc_set_range((lv_obj_t*)obj, min, max);
}

void lvgl_bar_set_value(void* obj, int value, int anim) {
    lv_bar_set_value((lv_obj_t*)obj, value, (lv_anim_enable_t)anim);
}

void lvgl_bar_set_range(void* obj, int min, int max) {
    lv_bar_set_range((lv_obj_t*)obj, min, max);
}

void lvgl_set_style_bg_color(void* obj, int r, int g, int b, int sel) {
    lv_obj_set_style_bg_color((lv_obj_t*)obj, lv_color_make(r, g, b), sel);
}

void lvgl_set_style_border_color(void* obj, int r, int g, int b, int sel) {
    lv_obj_set_style_border_color((lv_obj_t*)obj, lv_color_make(r, g, b), sel);
}

void lvgl_set_style_text_color(void* obj, int r, int g, int b, int sel) {
    lv_obj_set_style_text_color((lv_obj_t*)obj, lv_color_make(r, g, b), sel);
}

void lvgl_set_style_radius(void* obj, int r, int sel) {
    lv_obj_set_style_radius((lv_obj_t*)obj, r, sel);
}

void lvgl_set_style_pad(void* obj, int pad, int sel) {
    lv_obj_set_style_pad_all((lv_obj_t*)obj, pad, sel);
}

typedef void (*lv_callback)(void* obj);
static std::vector<lv_callback> callbacks;

static void lvgl_event_cb(lv_event_t* e) {
    lv_obj_t* obj = lv_event_get_target(e);
    int id = (int)(intptr_t)lv_event_get_user_data(e);
    if (id >= 0 && id < (int)callbacks.size() && callbacks[id]) {
        callbacks[id](obj);
    }
}

int lv_register_callback(lv_callback cb) {
    callbacks.push_back(cb);
    return (int)(callbacks.size() - 1);
}

void lvgl_obj_add_event_cb(void* obj, int cb_id, int event_code) {
    lv_obj_add_event_cb((lv_obj_t*)obj, lvgl_event_cb, (lv_event_code_t)event_code, (void*)(intptr_t)cb_id);
}

void* lvgl_scr_act() {
    return lv_scr_act();
}

void* lvgl_disp_get_default() {
    return lv_disp_get_default();
}

}
