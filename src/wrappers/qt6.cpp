#include <QApplication>
#include <QMainWindow>
#include <QWidget>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QPushButton>
#include <QLineEdit>
#include <QLabel>
#include <QCheckBox>
#include <QListWidget>
#include <QListWidgetItem>
#include <QStatusBar>
#include <QFileDialog>
#include <QFont>
#include <QColor>
#include <QPalette>
#include <QThread>
#include <QMetaObject>
#include <QString>
#include <QObject>
#include <QStyleFactory>

#include <cstdint>
#include <unordered_map>
#include <mutex>

extern "C" {

// Callback types
typedef void (*qt_callback)();
typedef void (*qt_callback_int)(int);
typedef void (*qt_callback_str)(const char*);

// Global callback registry
static std::unordered_map<int, qt_callback> callbacks;
static std::mutex cb_mutex;
static int next_cb_id = 1;

int qt_register_callback(qt_callback cb) {
    std::lock_guard<std::mutex> lock(cb_mutex);
    int id = next_cb_id++;
    callbacks[id] = cb;
    return id;
}

void qt_unregister_callback(int id) {
    std::lock_guard<std::mutex> lock(cb_mutex);
    callbacks.erase(id);
}

void qt_invoke_callback(int id) {
    std::lock_guard<std::mutex> lock(cb_mutex);
    auto it = callbacks.find(id);
    if (it != callbacks.end()) {
        it->second();
    }
}

// Signal connection
void* qt_connect_clicked(void* btn, int cb_id) {
    QPushButton* b = static_cast<QPushButton*>(btn);
    QMetaObject::Connection* conn = new QMetaObject::Connection;
    *conn = QObject::connect(b, &QPushButton::clicked, [cb_id]() {
        qt_invoke_callback(cb_id);
    });
    return conn;
}

void* qt_connect_return_pressed(void* edit, int cb_id) {
    QLineEdit* e = static_cast<QLineEdit*>(edit);
    QMetaObject::Connection* conn = new QMetaObject::Connection;
    *conn = QObject::connect(e, &QLineEdit::returnPressed, [cb_id]() {
        qt_invoke_callback(cb_id);
    });
    return conn;
}

void* qt_connect_item_double_clicked(void* list, int cb_id) {
    QListWidget* l = static_cast<QListWidget*>(list);
    QMetaObject::Connection* conn = new QMetaObject::Connection;
    *conn = QObject::connect(l, &QListWidget::itemDoubleClicked, [cb_id](QListWidgetItem*) {
        qt_invoke_callback(cb_id);
    });
    return conn;
}

// QApplication
void* qt_app_create(int argc, char** argv) {
    static int ac = argc;
    static char** av = argv;
    return new QApplication(ac, av);
}

int qt_app_exec(void* app) {
    return static_cast<QApplication*>(app)->exec();
}

void qt_app_set_style(void* app, const char* style) {
    static_cast<QApplication*>(app)->setStyleSheet(style);
}

void qt_app_set_style_fusion(const char* name) {
    QApplication::setStyle(QString::fromUtf8(name));
}

// QMainWindow
void* qt_mainwindow_create() {
    return new QMainWindow();
}

void qt_mainwindow_set_title(void* win, const char* title) {
    static_cast<QMainWindow*>(win)->setWindowTitle(QString::fromUtf8(title));
}

void qt_mainwindow_set_min_size(void* win, int w, int h) {
    static_cast<QMainWindow*>(win)->setMinimumSize(w, h);
}

void qt_mainwindow_set_central(void* win, void* widget) {
    static_cast<QMainWindow*>(win)->setCentralWidget(static_cast<QWidget*>(widget));
}

void qt_widget_show(void* win) {
    static_cast<QMainWindow*>(win)->show();
}

void* qt_mainwindow_status_bar(void* win) {
    return static_cast<QMainWindow*>(win)->statusBar();
}

void qt_statusbar_message(void* sb, const char* msg) {
    static_cast<QStatusBar*>(sb)->showMessage(QString::fromUtf8(msg));
}

// QWidget
void* qt_widget_create() {
    return new QWidget();
}

void qt_widget_set_palette(void* widget, void* palette) {
    static_cast<QWidget*>(widget)->setPalette(*static_cast<QPalette*>(palette));
}

// Layouts
void* qt_vbox_create(void* parent) {
    return new QVBoxLayout(parent ? static_cast<QWidget*>(parent) : nullptr);
}

void* qt_hbox_create(void* parent) {
    return new QHBoxLayout(parent ? static_cast<QWidget*>(parent) : nullptr);
}

void qt_layout_set_spacing(void* layout, int spacing) {
    static_cast<QBoxLayout*>(layout)->setSpacing(spacing);
}

void qt_layout_add_widget(void* layout, void* widget, int stretch) {
    static_cast<QBoxLayout*>(layout)->addWidget(static_cast<QWidget*>(widget), stretch);
}

void qt_layout_add_layout(void* parent, void* child) {
    static_cast<QBoxLayout*>(parent)->addLayout(static_cast<QLayout*>(child));
}

void qt_layout_add_stretch(void* layout) {
    static_cast<QBoxLayout*>(layout)->addStretch();
}

void qt_widget_set_layout(void* widget, void* layout) {
    static_cast<QWidget*>(widget)->setLayout(static_cast<QLayout*>(layout));
}

// QPushButton
void* qt_button_create(const char* text) {
    return new QPushButton(QString::fromUtf8(text));
}

void qt_button_set_text(void* btn, const char* text) {
    static_cast<QPushButton*>(btn)->setText(QString::fromUtf8(text));
}

void qt_button_set_enabled(void* btn, int enabled) {
    static_cast<QPushButton*>(btn)->setEnabled(enabled);
}

void qt_button_set_min_height(void* btn, int h) {
    static_cast<QPushButton*>(btn)->setMinimumHeight(h);
}

// QLineEdit
void* qt_lineedit_create() {
    return new QLineEdit();
}

void qt_lineedit_set_placeholder(void* edit, const char* text) {
    static_cast<QLineEdit*>(edit)->setPlaceholderText(QString::fromUtf8(text));
}

void qt_lineedit_set_min_height(void* edit, int h) {
    static_cast<QLineEdit*>(edit)->setMinimumHeight(h);
}

void qt_lineedit_set_font(void* edit, void* font) {
    static_cast<QLineEdit*>(edit)->setFont(*static_cast<QFont*>(font));
}

void qt_lineedit_set_text(void* edit, const char* text) {
    static_cast<QLineEdit*>(edit)->setText(QString::fromUtf8(text));
}

const char* qt_lineedit_text(void* edit) {
    static std::string s;
    s = static_cast<QLineEdit*>(edit)->text().toUtf8().constData();
    return s.c_str();
}

// QLabel
void* qt_label_create(const char* text) {
    return new QLabel(QString::fromUtf8(text));
}

void qt_label_set_text(void* label, const char* text) {
    static_cast<QLabel*>(label)->setText(QString::fromUtf8(text));
}

// QCheckBox
void* qt_checkbox_create(const char* text) {
    return new QCheckBox(QString::fromUtf8(text));
}

void qt_checkbox_set_checked(void* cb, int checked) {
    static_cast<QCheckBox*>(cb)->setChecked(checked != 0);
}

int qt_checkbox_is_checked(void* cb) {
    return static_cast<QCheckBox*>(cb)->isChecked() ? 1 : 0;
}

// QListWidget
void* qt_listwidget_create() {
    return new QListWidget();
}

void qt_listwidget_clear(void* list) {
    static_cast<QListWidget*>(list)->clear();
}

void qt_listwidget_add_item(void* list, const char* text) {
    static_cast<QListWidget*>(list)->addItem(QString::fromUtf8(text));
}

int qt_listwidget_count(void* list) {
    return static_cast<QListWidget*>(list)->count();
}

const char* qt_listwidget_item_text(void* list, int row) {
    static std::string s;
    auto item = static_cast<QListWidget*>(list)->item(row);
    if (item) {
        s = item->text().toUtf8().constData();
    }
    return s.c_str();
}

// QFileDialog
const char* qt_filedialog_get_dir(const char* title, const char* dir) {
    static std::string s;
    s = QFileDialog::getExistingDirectory(
        nullptr,
        QString::fromUtf8(title),
        QString::fromUtf8(dir)
    ).toUtf8().constData();
    return s.c_str();
}

// QFont
void* qt_font_create(const char* family, int size) {
    return new QFont(QString::fromUtf8(family), size);
}

// QColor
void* qt_color_create(int r, int g, int b) {
    return new QColor(r, g, b);
}

// QPalette
void* qt_palette_create() {
    return new QPalette();
}

void qt_palette_set_color(void* pal, int role, void* color) {
    static_cast<QPalette*>(pal)->setColor(
        static_cast<QPalette::ColorRole>(role),
        *static_cast<QColor*>(color)
    );
}

// QThread
void* qt_thread_create() {
    return new QThread();
}

void qt_thread_start(void* t) {
    static_cast<QThread*>(t)->start();
}

int qt_thread_is_running(void* t) {
    return static_cast<QThread*>(t)->isRunning() ? 1 : 0;
}

void qt_thread_quit(void* t) {
    static_cast<QThread*>(t)->quit();
}

void qt_thread_wait(void* t) {
    static_cast<QThread*>(t)->wait();
}

// Utility
const char* qt_style_factory_keys() {
    static std::string s;
    s = QStyleFactory::keys().join(" ").toUtf8().constData();
    return s.c_str();
}

} // extern "C"
