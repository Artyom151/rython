import sys
from PyQt6.QtWidgets import (
    QApplication, QMainWindow, QWidget, QVBoxLayout, 
    QGridLayout, QPushButton, QLineEdit
)
from PyQt6.QtCore import Qt
from PyQt6.QtGui import QFont


class Calculator(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Калькулятор")
        self.setFixedSize(300, 400)
        
        # Переменные для вычислений
        self.current_number = ""
        self.previous_number = ""
        self.operation = None
        self.result_shown = False
        
        # Создаём центральный виджет
        central_widget = QWidget()
        self.setCentralWidget(central_widget)
        
        # Создаём основной вертикальный layout
        main_layout = QVBoxLayout()
        central_widget.setLayout(main_layout)
        
        # Поле для отображения
        self.display = QLineEdit()
        self.display.setReadOnly(True)
        self.display.setAlignment(Qt.AlignmentFlag.AlignRight)
        self.display.setFont(QFont("Arial", 20))
        self.display.setText("0")
        self.display.setStyleSheet("""
            QLineEdit {
                padding: 10px;
                border: 2px solid #ccc;
                border-radius: 5px;
                background: white;
                min-height: 50px;
            }
        """)
        main_layout.addWidget(self.display)
        
        # Создаём сетку для кнопок
        grid_layout = QGridLayout()
        main_layout.addLayout(grid_layout)
        
        # Определяем кнопки
        buttons = [
            ('7', 0, 0), ('8', 0, 1), ('9', 0, 2), ('/', 0, 3),
            ('4', 1, 0), ('5', 1, 1), ('6', 1, 2), ('*', 1, 3),
            ('1', 2, 0), ('2', 2, 1), ('3', 2, 2), ('-', 2, 3),
            ('0', 3, 0), ('.', 3, 1), ('=', 3, 2), ('+', 3, 3),
            ('C', 4, 0, 1, 2), ('⌫', 4, 2, 1, 2)
        ]
        
        # Создаём и добавляем кнопки
        for button_data in buttons:
            if len(button_data) == 3:
                text, row, col = button_data
                rowspan, colspan = 1, 1
            else:
                text, row, col, rowspan, colspan = button_data
            
            button = QPushButton(text)
            button.setFont(QFont("Arial", 14))
            
            # Устанавливаем стиль для разных типов кнопок
            if text in ['=', 'C', '⌫']:
                button.setStyleSheet("""
                    QPushButton {
                        background-color: #4CAF50;
                        color: white;
                        border: none;
                        border-radius: 5px;
                        padding: 10px;
                    }
                    QPushButton:hover {
                        background-color: #45a049;
                    }
                    QPushButton:pressed {
                        background-color: #3d8b40;
                    }
                """)
            elif text in ['+', '-', '*', '/']:
                button.setStyleSheet("""
                    QPushButton {
                        background-color: #f0ad4e;
                        color: white;
                        border: none;
                        border-radius: 5px;
                        padding: 10px;
                    }
                    QPushButton:hover {
                        background-color: #ec971f;
                    }
                    QPushButton:pressed {
                        background-color: #d58512;
                    }
                """)
            else:
                button.setStyleSheet("""
                    QPushButton {
                        background-color: #f8f9fa;
                        border: 1px solid #ccc;
                        border-radius: 5px;
                        padding: 10px;
                    }
                    QPushButton:hover {
                        background-color: #e9ecef;
                    }
                    QPushButton:pressed {
                        background-color: #dee2e6;
                    }
                """)
            
            button.clicked.connect(self.on_button_click)
            grid_layout.addWidget(button, row, col, rowspan, colspan)
        
        # Настраиваем растяжение кнопок
        for i in range(5):
            grid_layout.setRowStretch(i, 1)
        for i in range(4):
            grid_layout.setColumnStretch(i, 1)
    
    def on_button_click(self):
        """Обработчик нажатия кнопок"""
        button = self.sender()
        text = button.text()
        
        if text.isdigit() or text == '.':
            self.number_pressed(text)
        elif text in ['+', '-', '*', '/']:
            self.operation_pressed(text)
        elif text == '=':
            self.calculate()
        elif text == 'C':
            self.clear()
        elif text == '⌫':
            self.backspace()
    
    def number_pressed(self, text):
        """Обработка нажатия цифр и точки"""
        if self.result_shown:
            self.current_number = ""
            self.result_shown = False
        
        # Ограничиваем длину числа
        if len(self.current_number) >= 15:
            return
        
        # Запрещаем ввод нескольких точек
        if text == '.' and '.' in self.current_number:
            return
        
        self.current_number += text
        self.display.setText(self.current_number)
    
    def operation_pressed(self, text):
        """Обработка нажатия операций"""
        if self.current_number:
            if self.previous_number and self.operation and not self.result_shown:
                self.calculate()
            else:
                self.previous_number = self.current_number
                self.current_number = ""
        
        self.operation = text
        self.display.setText(self.previous_number + " " + text + " ")
        self.result_shown = False
    
    def calculate(self):
        """Выполнение вычислений"""
        try:
            if not self.current_number or not self.previous_number or not self.operation:
                return
            
            num1 = float(self.previous_number)
            num2 = float(self.current_number)
            result = None
            
            if self.operation == '+':
                result = num1 + num2
            elif self.operation == '-':
                result = num1 - num2
            elif self.operation == '*':
                result = num1 * num2
            elif self.operation == '/':
                if num2 == 0:
                    self.display.setText("Ошибка: деление на 0")
                    self.clear()
                    return
                result = num1 / num2
            
            # Форматируем результат
            if result is not None:
                if result.is_integer():
                    result = int(result)
                else:
                    result = round(result, 10)
                
                self.display.setText(str(result))
                self.current_number = str(result)
                self.previous_number = ""
                self.operation = None
                self.result_shown = True
        
        except ValueError:
            self.display.setText("Ошибка")
            self.clear()
        except Exception as e:
            self.display.setText("Ошибка")
            self.clear()
    
    def clear(self):
        """Очистка всех полей"""
        self.current_number = ""
        self.previous_number = ""
        self.operation = None
        self.result_shown = False
        self.display.setText("0")
    
    def backspace(self):
        """Удаление последнего символа"""
        if self.result_shown:
            self.clear()
            return
        
        if self.current_number:
            self.current_number = self.current_number[:-1]
            self.display.setText(self.current_number if self.current_number else "0")


def main():
    app = QApplication(sys.argv)
    
    # Устанавливаем стиль приложения
    app.setStyle('Fusion')
    
    # Дополнительная настройка стилей для всего приложения
    app.setStyleSheet("""
        QMainWindow {
            background-color: #f0f0f0;
        }
    """)
    
    calculator = Calculator()
    calculator.show()
    
    sys.exit(app.exec())


if __name__ == "__main__":
    main()
