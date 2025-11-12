# ALAZ - Assembly Language Analyzer

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)

**ALAZ (Assembly Language Analyzer)** 是一个强大的 AArch64 汇编语言分析工具，用于解析 objdump 输出文件并生成美观的 Markdown 分析报告。

## ✨ 功能特性

- 🔍 **智能解析**: 自动解析 objdump 输出，提取汇编指令和源代码对应关系
- 📊 **多级对比**: 支持 O0/O1/O2 三个优化级别的汇编代码对比分析
- 🎯 **语义解释**: 自动生成每条指令的中文语义解释
- 🎨 **美观输出**: 生成格式化的 Markdown 表格，便于阅读和分享
- 🖥️ **交互模式**: 提供友好的交互式菜单，快速浏览和选择函数
- 📝 **单文件分析**: 支持对单个 dump 文件进行详细分析
- ⚡ **高性能**: Rust 编写，解析速度快，资源占用低

## 📦 安装

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/xjz6626/Assembly-Language-Analyzer.git
cd Assembly-Language-Analyzer

# 编译并安装
cargo install --path .

# 或者只编译
cargo build --release
# 可执行文件位于 target/release/alaz
```

### 使用 Cargo 安装

```bash
cargo install alaz
```

## 🚀 快速开始

### 1. 准备 dump 文件

首先，使用 `objdump` 生成反汇编文件：

```bash
# 编译不同优化级别的代码
gcc -O0 -g -c matrix.c -o matrix_O0.o
gcc -O1 -g -c matrix.c -o matrix_O1.o
gcc -O2 -g -c matrix.c -o matrix_O2.o

# 生成 dump 文件
objdump -d -S matrix_O0.o > matrix_O0.dump
objdump -d -S matrix_O1.o > matrix_O1.dump
objdump -d -S matrix_O2.o > matrix_O2.dump
```

### 2. 交互式分析

```bash
# 多文件模式 - 分析三个优化级别的共同函数
alaz interactive matrix

# 单文件模式 - 只分析一个 dump 文件
alaz interactive -s matrix_O2.dump
```

### 3. 直接分析指定函数

```bash
# 分析 Matrix_add 函数，生成对比报告
alaz analyze Matrix_add matrix

# 指定输出目录
alaz analyze Matrix_mul matrix -o ./reports
```

## 📖 使用指南

### 命令概览

```bash
alaz --help              # 查看帮助信息
alaz interactive <FILE>  # 交互式模式
alaz analyze <FUNC> <PREFIX>  # 直接分析
alaz completions <SHELL>  # 生成补全脚本
```

### 交互式模式

#### 多文件模式（默认）

分析三个优化级别的共同函数，生成对比报告：

```bash
# 使用文件前缀
alaz interactive matrix

# 或显式指定多文件模式
alaz interactive -m matrix

# 指定输出目录
alaz interactive matrix -o ./output
```

**特点**：
- ✅ 自动读取 `matrix_O0.dump`, `matrix_O1.dump`, `matrix_O2.dump`
- ✅ 只显示在三个文件中都存在的函数
- ✅ 生成 `<函数名>_comparison.md` 对比报告
- ✅ 包含三个优化级别的完整对比

#### 单文件模式

只分析一个 dump 文件：

```bash
# 分析 O2 优化级别的文件
alaz interactive -s matrix_O2.dump

# 分析 O0 优化级别的文件
alaz interactive -s matrix_O0.dump
```

**特点**：
- ✅ 显示该文件中的所有函数
- ✅ 可以选择函数进行详细分析
- ✅ 生成 `<函数名>_analysis.md` 分析报告
- ⚠️ 只有该文件的汇编和语义解释，无优化级别对比

### 直接分析模式

不使用交互菜单，直接分析指定函数：

```bash
# 基本用法
alaz analyze Matrix_add matrix

# 指定输出目录
alaz analyze Matrix_mul matrix -o ./reports

# 启用详细日志
alaz analyze -v Matrix_inv matrix
```

### Shell 补全

生成并安装 shell 补全脚本：

```bash
# Bash
alaz completions bash > ~/.local/share/bash-completion/completions/alaz

# Fish
alaz completions fish > ~/.config/fish/completions/alaz.fish

# Zsh
alaz completions zsh > ~/.zsh/completion/_alaz

# PowerShell
alaz completions powershell > alaz.ps1
```

## 📊 输出示例

### 多文件对比报告示例

生成的 `Matrix_add_comparison.md` 文件格式：

```markdown
# Matrix_add 函数对比分析

## O0 优化级别
| 地址 | 机器码 | 汇编指令 | 语义解释 | C代码 |
|------|--------|----------|----------|-------|
| 0x1000 | 910003fd | sub sp, sp, #0x10 | 分配16字节栈空间 | { |
| 0x1004 | f9000fe0 | str x0, [sp, #24] | 保存参数x0到栈 | |

## O1 优化级别
| 地址 | 机器码 | 汇编指令 | 语义解释 | C代码 |
|------|--------|----------|----------|-------|
| ... | ... | ... | ... | ... |

## O2 优化级别
| 地址 | 机器码 | 汇编指令 | 语义解释 | C代码 |
|------|--------|----------|----------|-------|
| ... | ... | ... | ... | ... |
```

## 🏗️ 项目结构
## 🏗️ 项目结构

```
alaz/
├── src/
│   ├── main.rs           # 命令行入口和交互模式
│   ├── lib.rs            # 库入口
│   ├── instruction.rs    # 指令定义和解释
│   ├── register.rs       # 寄存器定义
│   ├── parser.rs         # 汇编代码解析器
│   ├── semantic.rs       # 语义分析器
│   ├── objdump.rs        # objdump 文件解析
│   ├── table.rs          # 表格生成器
│   └── error.rs          # 错误类型定义
├── aarch64_instructions.json  # 指令集定义
├── Cargo.toml            # 项目配置
└── README.md             # 项目文档
```

## 🔧 技术细节

### 支持的指令类型

#### 1. 数据处理指令
- **算术运算**: ADD, SUB, MUL, MADD, MSUB, SDIV, UDIV
- **逻辑运算**: AND, ORR, EOR, BIC
- **移位操作**: LSL, LSR, ASR, ROR

#### 2. 加载存储指令
- **加载**: LDR, LDRB, LDRH, LDRSB, LDRSH, LDP
- **存储**: STR, STRB, STRH, STP

#### 3. 分支指令
- **无条件分支**: B, BL, BR, BLR, RET
- **条件分支**: B.cond (EQ, NE, CS, CC, MI, PL, VS, VC, HI, LS, GE, LT, GT, LE, AL)
- **比较分支**: CBZ, CBNZ, TBZ, TBNZ

#### 4. 比较指令
- CMP, CMN, TST, TEQ

#### 5. 移动指令
- MOV, MOVZ, MOVK, MOVN, MVN

#### 6. 其他指令
- NOP, ADRP, ADR, SXTW, UXTW

### 寄存器说明

#### 通用寄存器
- **64位**: X0-X30 (31个寄存器)
- **32位**: W0-W30 (X寄存器的低32位)

#### 特殊寄存器
- **SP**: 栈指针 (Stack Pointer)
- **X29/FP**: 帧指针 (Frame Pointer)
- **X30/LR**: 链接寄存器 (Link Register)
- **PC**: 程序计数器 (Program Counter)
- **XZR/WZR**: 零寄存器 (读取返回0，写入被忽略)

### 寻址模式

```assembly
[Xn]                 # 基址寻址
[Xn, #imm]          # 基址+偏移
[Xn, #imm]!         # 前索引 (先更新基址，再访问)
[Xn], #imm          # 后索引 (先访问，再更新基址)
[Xn, Xm]            # 寄存器偏移
[Xn, Wm, uxtw #3]   # 扩展寄存器偏移
```

## 🎨 语义解释示例

ALAZ 会为每条指令生成详细的中文语义解释：

| 指令 | 语义解释 |
|------|---------|
| `sub sp, sp, #0x30` | 分配48字节栈空间 |
| `stp x29, x30, [sp, #-64]!` | 将x29和x30压栈（前索引，先sp-=64再存储） |
| `ldr x0, [sp, #24]` | 从sp+24地址加载8字节到x0 |
| `add w0, w1, w2` | 将w1和w2相加，结果存入w0 |
| `cmp x0, #0` | 比较x0与0，更新条件标志 |
| `b.eq 1234` | 如果相等(Z=1)则跳转到0x1234 |
| `bl function` | 调用函数（保存返回地址到LR） |
| `ret` | 返回（跳转到LR保存的地址） |

## 📚 使用场景

### 1. 编译优化分析
比较不同优化级别下的代码生成差异：
```bash
alaz interactive mycode
# 查看 O0/O1/O2 的优化差异
```

### 2. 学习汇编语言
通过语义解释快速理解汇编指令：
```bash
alaz interactive -s example.dump
# 浏览并学习每条指令的含义
```

### 3. 性能调优
分析热点函数的汇编实现：
```bash
alaz analyze hot_function mycode -o ./analysis
```

### 4. 代码审查
生成可读性强的汇编报告用于代码审查：
```bash
alaz analyze critical_function security_lib
```

## 🤝 贡献

欢迎贡献！以下是一些可以改进的方向：

- [ ] 添加更多 AArch64 指令支持
- [ ] 支持浮点和 SIMD 指令
- [ ] 添加性能统计功能
- [ ] 支持其他架构（x86-64, RISC-V等）
- [ ] 添加图形化界面
- [ ] 支持更多输出格式（HTML, PDF等）

### 提交流程

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- ARM Architecture Reference Manual
- Rust 社区
- Clap 命令行解析库
- Colored 终端颜色库

## 📞 联系方式

- 作者: xjz
- Issues: [GitHub Issues](https://github.com/xjz6626/Assembly-Language-Analyzer/issues)

---

**Happy Analyzing! 🎉**