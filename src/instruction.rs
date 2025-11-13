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
    SMULL,
    UMULL,
    NEG,
    ADC,
    SBC,
    
    // 数据处理 - 逻辑运算
    AND,
    ORR,
    EOR,
    BIC,
    ORN,
    EON,
    MVN,
    
    // 数据处理 - 移位
    LSL,
    LSR,
    ASR,
    ROR,
    
    // 位域操作
    UBFM,
    SBFM,
    BFM,
    BFI,
    BFXIL,
    UBFX,
    SBFX,
    
    // 反转和位操作
    REV,
    REV16,
    REV32,
    CLZ,
    CLS,
    RBIT,
    
    // 加载存储
    LDR,
    LDRB,
    LDRH,
    LDRSB,
    LDRSH,
    LDRSW,
    LDP,
    LDUR,
    LDXR,
    LDAR,
    STR,
    STRB,
    STRH,
    STP,
    STUR,
    STXR,
    STLR,
    
    // 原子操作
    LDADD,
    LDADDAL,
    LDCLR,
    LDEOR,
    LDSET,
    SWP,
    CAS,
    CASAL,
    
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
    
    // 系统指令
    NOP,
    SVC,
    HLT,
    BRK,
    DMB,
    DSB,
    ISB,
    WFE,
    WFI,
    YIELD,
    
    // 系统寄存器访问
    MRS,
    MSR,
    
    // 浮点运算
    FADD,
    FSUB,
    FMUL,
    FDIV,
    FMADD,
    FMSUB,
    FNEG,
    FABS,
    FSQRT,
    FCMP,
    FCMPE,
    FCVT,
    FCVTZS,
    FCVTZU,
    SCVTF,
    UCVTF,
    FMOV,
    
    // SIMD/NEON 向量
    ADDV,
    SMAXV,
    SMINV,
    UMAXV,
    EXT,
    ZIP1,
    ZIP2,
    UZP1,
    TRN1,
    TBL,
    TBX,
    LD1,
    ST1,
    LD2,
    ST2,
    
    // 加密扩展
    AESE,
    AESD,
    AESMC,
    AESIMC,
    SHA1C,
    SHA1H,
    SHA1M,
    SHA1P,
    SHA256H,
    SHA256H2,
    SHA256SU0,
    SHA256SU1,
    
    // CRC校验
    CRC32B,
    CRC32H,
    CRC32W,
    CRC32X,
    CRC32CB,
    
    // 指针认证
    PACIA,
    PACDA,
    AUTIA,
    AUTDA,
    
    // 内存标签
    IRG,
    GMI,
    LDG,
    STG,
    
    // 条件操作
    CSEL,
    CSINC,
    CSINV,
    CSNEG,
    CSET,
    CSETM,
    CINC,
    CINV,
    CNEG,
    CCMP,
    CCMN,
    
    // 位域操作
    UBFIZ,
    SBFIZ,
    EXTR,
    
    // 浮点高级指令
    FMLA,
    FMLS,
    FMIN,
    FMAX,
    FMINNM,
    FMAXNM,
    FCVTAS,
    FCVTAU,
    FCVTMS,
    FCVTMU,
    FCVTNS,
    FCVTNU,
    FCVTPS,
    FCVTPU,
    FRINTA,
    FRINTI,
    FRINTM,
    FRINTN,
    FRINTP,
    FRINTX,
    FRINTZ,
    
    // SIMD 数据处理
    UADDLV,
    SADDLV,
    UMINV,
    INS,
    DUP,
    UZP2,
    TRN2,
    CNT,
    SQADD,
    UQADD,
    SQSUB,
    UQSUB,
    SHL,
    SSHR,
    USHR,
    SXTL,
    UXTL,
    
    // 原子操作扩展
    LDADDH,
    LDADDB,
    LDADDLH,
    LDADDLB,
    CASA,
    CASB,
    CASH,
    CASP,
    STADD,
    STADDL,
    STADDB,
    STADDH,
    
    // 加载/存储独占扩展
    LDXRB,
    LDXRH,
    STXRB,
    STXRH,
    LDAXRB,
    LDAXRH,
    STLXRB,
    STLXRH,
    LDXP,
    STXP,
    
    // 异常处理
    ERET,
    DRPS,
    
    // PC相对地址
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
