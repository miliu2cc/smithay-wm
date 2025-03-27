# Chessboard Window Manager

基于Smithay的棋盘式窗口管理器，无工作区概念，第一个窗口居中显示，其余窗口以棋盘方式排列。

## 特性

- 无工作区概念，所有窗口在同一个空间中管理
- 第一个窗口居中显示
- 其余窗口以棋盘方式围绕第一个窗口排列
- 使用Lua作为配置语言，支持自定义键绑定
- 基于Smithay Wayland合成器库构建

## 安装

### 依赖项

- `libudev`
- `libinput`
- `libgbm`
- `libseat`
- `xwayland` (可选，用于支持X11应用程序)

### 编译

```bash
cargo build --release
```

## 运行

```bash
cargo run -- --tty-udev  # 在TTY中运行
# 或
cargo run -- --x11       # 在X11会话中运行
# 或
cargo run -- --winit     # 在Wayland或X11会话中作为客户端运行
```

## 配置

配置文件使用Lua语言编写，默认位置为`config.lua`。

### 键绑定配置

```lua
-- 配置表
config = {
    -- 键绑定配置
    keybindings = {
        -- 启动终端 (Logo+Return)
        {
            modifiers = {"Logo"},
            key = "Return",
            action = "spawn",
            args = { command = "alacritty" }
        },
        
        -- 退出合成器 (Logo+q)
        {
            modifiers = {"Logo"},
            key = "q",
            action = "quit"
        },
        
        -- 关闭当前窗口 (Logo+c)
        {
            modifiers = {"Logo"},
            key = "c",
            action = "close_window"
        },
        
        -- 启动应用启动器 (Logo+d)
        {
            modifiers = {"Logo"},
            key = "d",
            action = "spawn",
            args = { command = "rofi -show drun" }
        },
    }
}
```

### 支持的修饰键

- `Shift`
- `Ctrl` 或 `Control`
- `Alt`
- `Logo` 或 `Super` 或 `Win`

### 支持的动作

- `quit`: 退出窗口管理器
- `spawn`: 启动程序，需要在`args`中指定`command`
- `close_window`: 关闭当前焦点窗口
- `toggle_fullscreen`: 切换当前窗口的全屏状态

## 窗口布局

窗口布局采用棋盘式排列：

- 第一个窗口居中显示
- 后续窗口按照螺旋方式围绕中心窗口排列
- 窗口位置自动计算，无需手动调整

## 开发

### 项目结构

- `src/shell/mod.rs`: 包含窗口布局逻辑
- `src/config.rs`: 配置文件解析
- `src/input_handler.rs`: 键盘输入处理

### 扩展

如需添加新功能，可以：

1. 在`config.rs`中添加新的配置选项
2. 在`input_handler.rs`中添加新的键绑定动作
3. 在`shell/mod.rs`中修改窗口布局算法

## 许可证

MIT