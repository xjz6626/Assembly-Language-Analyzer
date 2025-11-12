//! 寄存器定义和管理

use crate::error::{Result, InterpreterError};
use serde::{Deserialize, Serialize};

/// 寄存器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Register {
    // 64位通用寄存器
    X0, X1, X2, X3, X4, X5, X6, X7, X8, X9,
    X10, X11, X12, X13, X14, X15, X16, X17, X18, X19,
    X20, X21, X22, X23, X24, X25, X26, X27, X28, X29, X30,
    
    // 32位通用寄存器（对应X寄存器的低32位）
    W0, W1, W2, W3, W4, W5, W6, W7, W8, W9,
    W10, W11, W12, W13, W14, W15, W16, W17, W18, W19,
    W20, W21, W22, W23, W24, W25, W26, W27, W28, W29, W30,
    
    // 特殊寄存器
    SP,   // 栈指针
    PC,   // 程序计数器
    XZR,  // 零寄存器 64位
    WZR,  // 零寄存器 32位
    
    // 帧指针和链接寄存器（别名）
    FP,   // 帧指针，相当于 X29
    LR,   // 链接寄存器，相当于 X30
}

/// 条件标志位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionFlags {
    pub n: bool,  // Negative
    pub z: bool,  // Zero
    pub c: bool,  // Carry
    pub v: bool,  // Overflow
}

impl Default for ConditionFlags {
    fn default() -> Self {
        Self {
            n: false,
            z: false,
            c: false,
            v: false,
        }
    }
}

impl ConditionFlags {
    /// 创建新的条件标志位
    pub fn new() -> Self {
        Self::default()
    }

    /// 根据结果设置 N 和 Z 标志
    pub fn set_nz(&mut self, value: u64, is_64bit: bool) {
        if is_64bit {
            self.n = (value as i64) < 0;
            self.z = value == 0;
        } else {
            self.n = ((value as u32) as i32) < 0;
            self.z = (value as u32) == 0;
        }
    }
}

/// 条件码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Condition {
    EQ,  // Equal (Z == 1)
    NE,  // Not Equal (Z == 0)
    CS,  // Carry Set (C == 1)
    CC,  // Carry Clear (C == 0)
    MI,  // Minus (N == 1)
    PL,  // Plus (N == 0)
    VS,  // Overflow Set (V == 1)
    VC,  // Overflow Clear (V == 0)
    HI,  // Higher (C == 1 && Z == 0)
    LS,  // Lower or Same (C == 0 || Z == 1)
    GE,  // Greater or Equal (N == V)
    LT,  // Less Than (N != V)
    GT,  // Greater Than (Z == 0 && N == V)
    LE,  // Less or Equal (Z == 1 || N != V)
    AL,  // Always (无条件)
}

impl Condition {
    /// 评估条件是否满足
    pub fn evaluate(&self, flags: &ConditionFlags) -> bool {
        match self {
            Condition::EQ => flags.z,
            Condition::NE => !flags.z,
            Condition::CS => flags.c,
            Condition::CC => !flags.c,
            Condition::MI => flags.n,
            Condition::PL => !flags.n,
            Condition::VS => flags.v,
            Condition::VC => !flags.v,
            Condition::HI => flags.c && !flags.z,
            Condition::LS => !flags.c || flags.z,
            Condition::GE => flags.n == flags.v,
            Condition::LT => flags.n != flags.v,
            Condition::GT => !flags.z && (flags.n == flags.v),
            Condition::LE => flags.z || (flags.n != flags.v),
            Condition::AL => true,
        }
    }
}

impl Register {
    /// 解析寄存器名称
    pub fn parse(name: &str) -> Result<Self> {
        let name_lower = name.to_lowercase();
        match name_lower.as_str() {
            "x0" => Ok(Register::X0),
            "x1" => Ok(Register::X1),
            "x2" => Ok(Register::X2),
            "x3" => Ok(Register::X3),
            "x4" => Ok(Register::X4),
            "x5" => Ok(Register::X5),
            "x6" => Ok(Register::X6),
            "x7" => Ok(Register::X7),
            "x8" => Ok(Register::X8),
            "x9" => Ok(Register::X9),
            "x10" => Ok(Register::X10),
            "x11" => Ok(Register::X11),
            "x12" => Ok(Register::X12),
            "x13" => Ok(Register::X13),
            "x14" => Ok(Register::X14),
            "x15" => Ok(Register::X15),
            "x16" => Ok(Register::X16),
            "x17" => Ok(Register::X17),
            "x18" => Ok(Register::X18),
            "x19" => Ok(Register::X19),
            "x20" => Ok(Register::X20),
            "x21" => Ok(Register::X21),
            "x22" => Ok(Register::X22),
            "x23" => Ok(Register::X23),
            "x24" => Ok(Register::X24),
            "x25" => Ok(Register::X25),
            "x26" => Ok(Register::X26),
            "x27" => Ok(Register::X27),
            "x28" => Ok(Register::X28),
            "x29" | "fp" => Ok(Register::X29),
            "x30" | "lr" => Ok(Register::X30),
            
            "w0" => Ok(Register::W0),
            "w1" => Ok(Register::W1),
            "w2" => Ok(Register::W2),
            "w3" => Ok(Register::W3),
            "w4" => Ok(Register::W4),
            "w5" => Ok(Register::W5),
            "w6" => Ok(Register::W6),
            "w7" => Ok(Register::W7),
            "w8" => Ok(Register::W8),
            "w9" => Ok(Register::W9),
            "w10" => Ok(Register::W10),
            "w11" => Ok(Register::W11),
            "w12" => Ok(Register::W12),
            "w13" => Ok(Register::W13),
            "w14" => Ok(Register::W14),
            "w15" => Ok(Register::W15),
            "w16" => Ok(Register::W16),
            "w17" => Ok(Register::W17),
            "w18" => Ok(Register::W18),
            "w19" => Ok(Register::W19),
            "w20" => Ok(Register::W20),
            "w21" => Ok(Register::W21),
            "w22" => Ok(Register::W22),
            "w23" => Ok(Register::W23),
            "w24" => Ok(Register::W24),
            "w25" => Ok(Register::W25),
            "w26" => Ok(Register::W26),
            "w27" => Ok(Register::W27),
            "w28" => Ok(Register::W28),
            "w29" => Ok(Register::W29),
            "w30" => Ok(Register::W30),
            
            "sp" => Ok(Register::SP),
            "pc" => Ok(Register::PC),
            "xzr" => Ok(Register::XZR),
            "wzr" => Ok(Register::WZR),
            
            _ => Err(InterpreterError::InvalidRegister(name.to_string())),
        }
    }

    /// 判断寄存器是否为64位
    pub fn is_64bit(&self) -> bool {
        matches!(self, 
            Register::X0 | Register::X1 | Register::X2 | Register::X3 | Register::X4 |
            Register::X5 | Register::X6 | Register::X7 | Register::X8 | Register::X9 |
            Register::X10 | Register::X11 | Register::X12 | Register::X13 | Register::X14 |
            Register::X15 | Register::X16 | Register::X17 | Register::X18 | Register::X19 |
            Register::X20 | Register::X21 | Register::X22 | Register::X23 | Register::X24 |
            Register::X25 | Register::X26 | Register::X27 | Register::X28 | Register::X29 |
            Register::X30 | Register::SP | Register::PC | Register::XZR | Register::FP | Register::LR
        )
    }

    /// 获取寄存器索引（用于访问寄存器数组）
    pub fn index(&self) -> Option<usize> {
        match self {
            Register::X0 | Register::W0 => Some(0),
            Register::X1 | Register::W1 => Some(1),
            Register::X2 | Register::W2 => Some(2),
            Register::X3 | Register::W3 => Some(3),
            Register::X4 | Register::W4 => Some(4),
            Register::X5 | Register::W5 => Some(5),
            Register::X6 | Register::W6 => Some(6),
            Register::X7 | Register::W7 => Some(7),
            Register::X8 | Register::W8 => Some(8),
            Register::X9 | Register::W9 => Some(9),
            Register::X10 | Register::W10 => Some(10),
            Register::X11 | Register::W11 => Some(11),
            Register::X12 | Register::W12 => Some(12),
            Register::X13 | Register::W13 => Some(13),
            Register::X14 | Register::W14 => Some(14),
            Register::X15 | Register::W15 => Some(15),
            Register::X16 | Register::W16 => Some(16),
            Register::X17 | Register::W17 => Some(17),
            Register::X18 | Register::W18 => Some(18),
            Register::X19 | Register::W19 => Some(19),
            Register::X20 | Register::W20 => Some(20),
            Register::X21 | Register::W21 => Some(21),
            Register::X22 | Register::W22 => Some(22),
            Register::X23 | Register::W23 => Some(23),
            Register::X24 | Register::W24 => Some(24),
            Register::X25 | Register::W25 => Some(25),
            Register::X26 | Register::W26 => Some(26),
            Register::X27 | Register::W27 => Some(27),
            Register::X28 | Register::W28 => Some(28),
            Register::X29 | Register::W29 | Register::FP => Some(29),
            Register::X30 | Register::W30 | Register::LR => Some(30),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_register_parsing() {
        assert_eq!(Register::parse("x0").unwrap(), Register::X0);
        assert_eq!(Register::parse("w15").unwrap(), Register::W15);
        assert_eq!(Register::parse("sp").unwrap(), Register::SP);
        assert_eq!(Register::parse("lr").unwrap(), Register::X30);
        assert!(Register::parse("invalid").is_err());
    }
    
    #[test]
    fn test_is_64bit() {
        assert!(Register::X0.is_64bit());
        assert!(!Register::W0.is_64bit());
        assert!(Register::SP.is_64bit());
    }

    #[test]
    fn test_condition_evaluation() {
        let mut flags = ConditionFlags::new();
        
        // Test EQ
        flags.z = true;
        assert!(Condition::EQ.evaluate(&flags));
        assert!(!Condition::NE.evaluate(&flags));
        
        // Test GT
        flags.z = false;
        flags.n = false;
        flags.v = false;
        assert!(Condition::GT.evaluate(&flags));
    }
}
