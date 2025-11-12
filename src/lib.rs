//! AArch64 Objdump Analyzer
//! 
//! 解析 objdump 文件并生成 C 代码与汇编指令的对比表格，包含语义解释。
//! 
//! # 模块
//! 
//! - `instruction`: 指令定义和指令集
//! - `register`: 寄存器定义和管理
//! - `parser`: 汇编代码解析器
//! - `error`: 错误类型定义
//! - `objdump`: objdump 文件解析器
//! - `semantic`: 汇编指令语义解释器
//! - `table`: Markdown 表格生成器

pub mod instruction;
pub mod register;
pub mod parser;
pub mod error;
pub mod objdump;
pub mod semantic;
pub mod table;

// 重新导出常用类型
pub use instruction::{Instruction, InstructionType, Operand};
pub use register::Register;
pub use error::{Result, InterpreterError};
