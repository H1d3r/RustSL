"""
GUI模块
RSL加载器的图形用户界面

模块结构:
- main_window: 主窗口界面
- widgets: 自定义控件（BinComboBox, IcoComboBox）
- sign: 签名相关控件
- worker: 后台工作线程
- config_manager: 配置管理
- styles: UI样式表
- ui_components: UI组件工厂
"""

from .main_window import LoaderGUI

__all__ = ['LoaderGUI']