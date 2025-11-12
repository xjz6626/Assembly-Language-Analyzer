//! 指令定义

use crate::register::{Register, Condition};
use serde::{Deserialize, Serialize};

/// 指令操作数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operand {
    /// 寄存器操作数
    Register(Register),
    /// 立即数操作数
    Immediate(i64),
    /// 标签操作数（用于分支）
    Label(String),
    /// 内存操作数
    Memory {
        base: Register,
        offset: Option<i64>,
        index: Option<Register>,
        pre_indexed: bool,
        post_indexed: bool,
    },
}

/// 指令类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstructionType {
    // 数据处理 - 算术运算
    ADD,
    SUB,
    MUL,
    MADD,
    MSUB,
    UDIV,
    SDIV,
    
    // 数据处理 - 逻辑运算
    AND,
    ORR,
    EOR,
    BIC,
    
    // 数据处理 - 移位
    LSL,
    LSR,
    ASR,
    ROR,
    
    // 加载存储
    LDR,
    LDRB,
    LDRH,
    LDRSB,
    LDRSH,
    LDRSW,
    LDP,
    STR,
    STRB,
    STRH,
    STP,
    
    // 分支
    B,
    BL,
    BR,
    BLR,
    RET,
    
    // 条件分支
    BEQ,
    BNE,
    BCS,
    BCC,
    BMI,
    BPL,
    BVS,
    BVC,
    BHI,
    BLS,
    BGE,
    BLT,
    BGT,
    BLE,
    
    // 比较和分支
    CBZ,
    CBNZ,
    TBZ,
    TBNZ,
    
    // 比较指令
    CMP,
    CMN,
    TST,
    
    // 数据移动
    MOV,
    MOVZ,
    MOVK,
    MOVN,
    MVN,
    
    // 系统指令
    NOP,
    SVC,
    HLT,
    BRK,
    
    // 系统寄存器访问
    MRS,
    MSR,
    
    // 其他
    ADRP,
    ADR,
}

/// 指令结构
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    /// 指令类型
    pub instruction_type: InstructionType,
    /// 操作数列表
    pub operands: Vec<Operand>,
    /// 指令地址
    pub address: u64,
    /// 机器码编码（可选）
    pub encoding: Option<u32>,
    /// 条件码（用于条件指令）
    pub condition: Option<Condition>,
}

impl Instruction {
    /// 创建新指令
    pub fn new(
        instruction_type: InstructionType,
        operands: Vec<Operand>,
        address: u64,
    ) -> Self {
        Self {
            instruction_type,
            operands,
            address,
            encoding: None,
            condition: None,
        }
    }

    /// 创建带条件的新指令
    pub fn new_with_condition(
        instruction_type: InstructionType,
        operands: Vec<Operand>,
        address: u64,
        condition: Condition,
    ) -> Self {
        Self {
            instruction_type,
            operands,
            address,
            encoding: None,
            condition: Some(condition),
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.instruction_type)?;
        for (i, operand) in self.operands.iter().enumerate() {
            if i == 0 {
                write!(f, " ")?;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", operand)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_creation() {
        let inst = Instruction::new(
            InstructionType::ADD,
            vec![
                Operand::Register(Register::X0),
                Operand::Register(Register::X1),
                Operand::Immediate(10),
            ],
            0x1000,
        );
        
        assert_eq!(inst.instruction_type, InstructionType::ADD);
        assert_eq!(inst.operands.len(), 3);
        assert_eq!(inst.address, 0x1000);
    }
}
