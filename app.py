from PyQt6.QtWidgets import QApplication, QMainWindow, QPushButton, QWidget, QVBoxLayout, QLabel

app = QApplication()
win = QMainWindow()
win.setWindowTitle("rython Qt6 Test")
win.setMinimumSize(400, 300)

central = QWidget()
layout = QVBoxLayout(central)
layout.setSpacing(10)

label = QLabel("Hello from rython!")
layout.addWidget(label, 0)

btn = QPushButton("Click me!")
layout.addWidget(btn, 0)

win.setCentralWidget(central)
win.show()
app.exec()
