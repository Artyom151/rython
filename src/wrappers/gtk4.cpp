#include <gtk/gtk.h>
#include <cstdint>
#include <unordered_map>
#include <mutex>
#include <string>

extern "C" {

typedef void (*gtk4_callback)();

static std::unordered_map<int, gtk4_callback> callbacks;
static std::mutex cb_mutex;
static int next_cb_id = 1;

int gtk4_register_callback(gtk4_callback cb) {
    std::lock_guard<std::mutex> lock(cb_mutex);
    int id = next_cb_id++;
    callbacks[id] = cb;
    return id;
}

void gtk4_unregister_callback(int id) {
    std::lock_guard<std::mutex> lock(cb_mutex);
    callbacks.erase(id);
}

void gtk4_invoke_callback(int id) {
    std::lock_guard<std::mutex> lock(cb_mutex);
    auto it = callbacks.find(id);
    if (it != callbacks.end()) {
        it->second();
    }
}

int gtk4_signal_connect(void* instance, const char* signal_name, int cb_id) {
    return g_signal_connect_data(
        instance, signal_name,
        G_CALLBACK(+[](void*, gpointer user_data) -> void {
            intptr_t id = (intptr_t)user_data;
            gtk4_invoke_callback((int)id);
        }),
        (gpointer)(intptr_t)cb_id,
        nullptr, GConnectFlags(0)
    );
}

void* gtk4_app_create() {
    return gtk_application_new("com.rython.app", G_APPLICATION_DEFAULT_FLAGS);
}

int gtk4_app_run(void* app, int argc, char** argv) {
    return g_application_run(G_APPLICATION(app), argc, argv);
}

void gtk4_app_quit(void* app) {
    g_application_quit(G_APPLICATION(app));
}

void* gtk4_window_new(void* app) {
    return gtk_application_window_new(GTK_APPLICATION(app));
}

void gtk4_window_set_title(void* win, const char* title) {
    gtk_window_set_title(GTK_WINDOW(win), title);
}

void gtk4_window_set_default_size(void* win, int w, int h) {
    gtk_window_set_default_size(GTK_WINDOW(win), w, h);
}

void gtk4_window_set_child(void* win, void* child) {
    gtk_window_set_child(GTK_WINDOW(win), GTK_WIDGET(child));
}

void gtk4_window_set_titlebar(void* win, void* bar) {
    gtk_window_set_titlebar(GTK_WINDOW(win), GTK_WIDGET(bar));
}

void gtk4_widget_show(void* widget) {
    gtk_widget_show(GTK_WIDGET(widget));
}

void gtk4_widget_set_visible(void* widget, int visible) {
    gtk_widget_set_visible(GTK_WIDGET(widget), visible != 0);
}

void* gtk4_button_new_with_label(const char* label) {
    return gtk_button_new_with_label(label);
}

void gtk4_button_set_label(void* btn, const char* label) {
    gtk_button_set_label(GTK_BUTTON(btn), label);
}

void* gtk4_label_new(const char* text) {
    return gtk_label_new(text);
}

void gtk4_label_set_text(void* label, const char* text) {
    gtk_label_set_text(GTK_LABEL(label), text);
}

void* gtk4_entry_new() {
    return gtk_entry_new();
}

void gtk4_entry_set_placeholder_text(void* entry, const char* text) {
    gtk_entry_set_placeholder_text(GTK_ENTRY(entry), text);
}

const char* gtk4_entry_get_text(void* entry) {
    return gtk_editable_get_text(GTK_EDITABLE(entry));
}

void gtk4_entry_set_text(void* entry, const char* text) {
    gtk_editable_set_text(GTK_EDITABLE(entry), text);
}

void* gtk4_box_new(int orientation, int spacing) {
    return gtk_box_new(orientation == 0 ? GTK_ORIENTATION_HORIZONTAL : GTK_ORIENTATION_VERTICAL, spacing);
}

void gtk4_box_append(void* box, void* child) {
    gtk_box_append(GTK_BOX(box), GTK_WIDGET(child));
}

void* gtk4_scrolled_window_new() {
    return gtk_scrolled_window_new();
}

void* gtk4_text_view_new() {
    return gtk_text_view_new();
}

void* gtk4_text_view_get_buffer(void* view) {
    return gtk_text_view_get_buffer(GTK_TEXT_VIEW(view));
}

const char* gtk4_text_buffer_get_text(void* buf, int start, int end) {
    GtkTextIter start_iter, end_iter;
    gtk_text_buffer_get_iter_at_offset(GTK_TEXT_BUFFER(buf), &start_iter, start);
    gtk_text_buffer_get_iter_at_offset(GTK_TEXT_BUFFER(buf), &end_iter, end);
    char* text = gtk_text_buffer_get_text(GTK_TEXT_BUFFER(buf), &start_iter, &end_iter, false);
    static std::string s;
    s = text ? text : "";
    g_free(text);
    return s.c_str();
}

void gtk4_text_buffer_set_text(void* buf, const char* text) {
    gtk_text_buffer_set_text(GTK_TEXT_BUFFER(buf), text, -1);
}

void* gtk4_header_bar_new() {
    return gtk_header_bar_new();
}

// gtk_header_bar_set_title was removed in GTK4; use gtk_header_bar_set_title_widget or gtk_window_set_title

void gtk4_set_title(void* widget, const char* title) {
    if (GTK_IS_WINDOW(widget)) {
        gtk_window_set_title(GTK_WINDOW(widget), title);
    }
}

void gtk4_set_text(void* widget, const char* text) {
    if (GTK_IS_LABEL(widget)) {
        gtk_label_set_text(GTK_LABEL(widget), text);
    } else if (GTK_IS_EDITABLE(widget)) {
        gtk_editable_set_text(GTK_EDITABLE(widget), text);
    } else if (GTK_IS_TEXT_BUFFER(widget)) {
        gtk_text_buffer_set_text(GTK_TEXT_BUFFER(widget), text, -1);
    }
}

const char* gtk4_get_text(void* widget) {
    if (GTK_IS_EDITABLE(widget)) {
        return gtk_editable_get_text(GTK_EDITABLE(widget));
    }
    return "";
}

} // extern "C"
