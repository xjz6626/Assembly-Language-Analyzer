//! 错误类型定义

use thiserror::Error;

/// 解释器错误类型
#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("解析错误: {0}")]
    ParseError(String),

    #[error("无效的指令: {0}")]
    InvalidInstruction(String),

    #[error("无效的寄存器: {0}")]
    InvalidRegister(String),

    #[error("无效的操作数: {0}")]
    InvalidOperand(String),

    #[error("内存访问错误: {0}")]
    MemoryError(String),

    #[error("执行错误: {0}")]
    ExecutionError(String),

    #[error("未实现的功能: {0}")]
    Unimplemented(String),

    #[error("除零错误")]
    DivisionByZero,

    #[error("栈溢出")]
    StackOverflow,

    #[error("栈下溢")]
    StackUnderflow,

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON 错误: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, InterpreterError>;
