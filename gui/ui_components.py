"""
UI组件工厂模块
提供创建各种UI组件的工厂函数
"""
import os
from PyQt5.QtCore import QSize
from PyQt5.QtWidgets import QComboBox, QCheckBox, QGridLayout
from PyQt5.QtGui import QIcon
from .config_manager import load_plugins_manifest, get_default_value


def get_folder_icon():
    """获取文件夹图标"""
    icon_path = os.path.join('gui', 'icons', 'folder.ico')
    return QIcon(icon_path) if os.path.exists(icon_path) else QIcon()


def get_icon(icon_name):
    """
    获取指定名称的图标
    
    Args:
        icon_name: 图标文件名（不含扩展名）
    """
    icon_path = os.path.join('gui', 'icons', f'{icon_name}.ico')
    return QIcon(icon_path) if os.path.exists(icon_path) else QIcon()


def create_encryption_combobox():
    """创建加密方式下拉框"""
    combo = QComboBox()
    combo.setIconSize(QSize(20, 20))
    
    enc_icon = get_icon('enc')
    manifest = load_plugins_manifest()
    
    for e in manifest['encryption']:
        combo.addItem(enc_icon, e.get('label', e['id']), e['id'])
    
    # 设置默认值
    default_enc = get_default_value('encryption')
    if default_enc:
        for i in range(combo.count()):
            if combo.itemData(i) == default_enc:
                combo.setCurrentIndex(i)
                break
    
    return combo


def create_mem_mode_combobox():
    """创建内存分配方式下拉框"""
    combo = QComboBox()
    mem_icon = get_icon('mem')
    
    manifest = load_plugins_manifest()
    mem_modes = manifest.get('alloc_mem_modes', [])
    
    for m in mem_modes:
        combo.addItem(mem_icon, m.get('label', m['id']), m['id'])
    
    # 设置默认值
    default_mem = get_default_value('alloc_mem_mode')
    if default_mem:
        for i in range(combo.count()):
            if combo.itemData(i) == default_mem:
                combo.setCurrentIndex(i)
                break
    
    return combo


def create_vm_checks_grid():
    """
    创建VM检测复选框网格
    
    Returns:
        tuple: (grid_layout, checkboxes_list)
    """
    manifest = load_plugins_manifest()
    vm_items = manifest.get('vm_checks', [])
    
    # 回退：如果配置为空，使用内置列表
    if not vm_items:
        vm_items = [
            {'id': t, 'label': t} for t in [
                'c_drive', 'desktop_files', 'tick', 'memory', 'api_flood',
                'mouse', 'common_software', 'uptime'
            ]
        ]
    
    grid = QGridLayout()
    checkboxes = []
    
    for i, item in enumerate(vm_items):
        text = item.get('label', item.get('id', ''))
        vm_id = item.get('id', text)
        
        cb = QCheckBox(text)
        cb.setProperty('vm_id', vm_id)
        checkboxes.append(cb)
        
        grid.addWidget(cb, i // 4, i % 4)
    
    return grid, checkboxes


def create_run_mode_combobox():
    """创建运行方式下拉框"""
    combo = QComboBox()
    combo.setIconSize(QSize(20, 20))
    
    run_icon = get_icon('run')
    manifest = load_plugins_manifest()
    
    for rm in manifest['run_modes']:
        combo.addItem(run_icon, rm.get('label', rm['id']), rm['id'])
    
    # 设置默认值
    default_rm = get_default_value('run_mode')
    if default_rm:
        for i in range(combo.count()):
            if combo.itemData(i) == default_rm:
                combo.setCurrentIndex(i)
                break
    
    return combo
