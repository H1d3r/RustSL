
def get_main_stylesheet():
    return """
    QWidget {
        background:         color:         font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
        font-size: 16px;
    }
    QGroupBox {
        border: 1px solid         border-radius: 8px;
        margin-top: 10px;
        background:         font-weight: bold;
        padding-top: 10px;
        font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
        font-size: 18px;
    }
    QGroupBox:title {
        subcontrol-origin: margin;
        left: 10px;
        padding: 0 3px 0 3px;
    }
    QLabel {
        color:         font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
    }
    QLineEdit, QComboBox, QTextEdit {
        background:         border: 1px solid         border-radius: 5px;
        color:         padding: 4px;
        font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
    }
    QPushButton {
        background: qlineargradient(x1:0, y1:0, x2:0, y2:1, stop:0         color:         border-radius: 6px;
        padding: 6px 18px;
        font-weight: bold;
        font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
    }
    QPushButton:hover {
        background:     }
    QProgressBar {
        border: 1px solid         border-radius: 6px;
        text-align: center;
        background:         height: 18px;
    }
    QProgressBar::chunk {
        background: qlineargradient(x1:0, y1:0, x2:1, y2:0, stop:0         border-radius: 6px;
    }
    QTextEdit {
        font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Consolas', 'Fira Mono', 'Microsoft YaHei';
        font-size: 15px;
        background:         color:     }
