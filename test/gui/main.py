from PyQt6.QtWidgets import QApplication, QMainWindow, QWidget, QVBoxLayout, QHBoxLayout, QPushButton, QLabel, QLineEdit
from screen import build_list, build_checkboxes, apply_style

app = QApplication()
win = QMainWindow()
win.setWindowTitle("rython GUI Test")
win.setMinimumSize(500, 400)

central = QWidget()
layout = QVBoxLayout(central)
layout.setSpacing(8)

title = QLabel("rython — Python to Rust transpiler")
layout.addWidget(title, 0)

name_row = QWidget()
name_layout = QHBoxLayout(name_row)
name_layout.addWidget(QLabel("Name:"), 0)
name_edit = QLineEdit()
name_edit.setPlaceholderText("Enter your name")
name_layout.addWidget(name_edit, 0)
layout.addWidget(name_row, 0)

items = build_list()
layout.addWidget(items, 0)

check_panel = QWidget()
check_layout = QVBoxLayout(check_panel)
build_checkboxes(check_layout)
layout.addWidget(check_panel, 0)

btn_row = QWidget()
btn_layout = QHBoxLayout(btn_row)
action_btn = QPushButton("Action")
btn_layout.addWidget(action_btn, 0)
quit_btn = QPushButton("Quit")
btn_layout.addWidget(quit_btn, 0)
layout.addWidget(btn_row, 0)

apply_style(central)

sb = win.statusBar()
sb.showMessage("Ready")

win.setCentralWidget(central)
win.show()
app.exec()
