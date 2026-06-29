from PyQt6.QtWidgets import QListWidget, QCheckBox
from PyQt6.QtGui import QFont, QColor, QPalette

def build_list():
    win = QListWidget()
    win.addItem("Red")
    win.addItem("Green")
    win.addItem("Blue")
    return win

def build_checkboxes(parent):
    cb1 = QCheckBox("Option A")
    cb2 = QCheckBox("Option B")
    cb2.setChecked(True)
    parent.addWidget(cb1)
    parent.addWidget(cb2)

def apply_style(widget):
    pal = QPalette()
    color = QColor(200, 220, 255)
    pal.setColor(10, color)
    widget.setPalette(pal)
