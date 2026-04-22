![sacrifice](./docs/images/level-preview-sacrifice.gif)

# [Project Cube Collection](https://wiryls.github.io/cube-collection/)

[English](./README.md) | [中文](./README_zh-CN.md)

一个基于 [Bevy 引擎](https://github.com/bevyengine/bevy) 的极简益智游戏，移动方块以**覆盖所有目标点**，如下图所示。

![a-moth-to-flame](./docs/images/level-preview-a-moth-to-flame.gif)

在 [GitHub Pages](https://wiryls.github.io/cube-collection/) 上试玩**在线版本**！

## 游戏玩法

### 操作方式

- 移动：`方向键` 或 `W`/`A`/`S`/`D`。
- 重新开始：`R`。
- 跳过当前关卡：`N`。
- 返回上一关卡：`L`。
- 重置游戏：`ESC`。
- 重新加载游戏：`Shift` + `ESC`。

### 游戏规则

- 你移动所有的绿色方块。
- 让方块覆盖所有目标点即可进入下一关。
- 方块之间会相互吸收：
  - 红 + 绿 -> **红**
  - 绿 + 蓝 -> **绿**
  - 蓝 + 红 -> **蓝**
  - 红 + 绿 + 蓝 -> 不发生任何变化
- 同类方块（白色除外）碰撞时会合并。

## 关于本仓库

### 运行

1. 确保已安装 [Rust](https://www.rust-lang.org/tools/install)（edition 2021+）。
2. 克隆本仓库：`git clone https://github.com/wiryls/cube-collection.git`
3. 编译并运行：`cargo run --release -p cube-collection`

### 添加自定义关卡

关卡以 TOML 文件表示。例如：

```toml
[map]
raw = '''
                
                
  GGGGGGGGGGGGG  
  G   GG GG   G  
  G           G  
  G   R   R   G  
  G           G  
  G           G  
  G     x     G  
  G           G  
  W------------  
                '''

[info]
author = "w"
title = "Haircut"
```

- `map.raw` 是一个包含以下字符的 ASCII 图：
  - 方块（在此处放置方块）：
    - `W`：白色方块。
    - `R`：红色方块。
    - `G`：绿色方块。
    - `B`：蓝色方块。
  - 链接（在此处放置方块并链接到）：
    - `|`：上方方块。
    - `-`：左方方块。
    - `/`：上方和左方方块。
  - 其他：
    - ` `：此处无内容。
    - `x`：目标点。
- `info` 包含一些元数据。

> 注意：如果任何关卡文件无效，游戏将停止加载并记录错误。

如果你想添加自定义关卡：

1. 创建如上所示的 TOML 文件。
2. 将自定义关卡文件放入 `./cube-collection/assets/level/` 目录。
3. 将文件名添加到 `./cube-collection/assets/level/index.toml` 的 `name_list` 中。

## 许可证

本仓库使用双重许可：

- `./cube-core` 采用 **LGPL 3.0** 许可证，以及
- `./cube-collection` 采用 **MIT** 许可证。

## 已知问题

- 颜色方案可能对某些色觉障碍用户不够友好。
