// AArch64 指令集定义 - 用于汇编解释器
// 本文件定义了 AArch64 (ARM 64位) 架构的指令集结构

use std::collections::HashMap;

/// 寄存器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConditionFlags {
    pub n: bool,  // Negative
    pub z: bool,  // Zero
    pub c: bool,  // Carry
    pub v: bool,  // Overflow
}

/// 条件码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// 指令操作数
#[derive(Debug, Clone)]
pub enum Operand {
    Register(Register),
    Immediate(i64),
    Label(String),
    Memory {
        base: Register,
        offset: Option<i64>,
        index: Option<Register>,
        pre_indexed: bool,
        post_indexed: bool,
    },
}

/// 指令类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionType {
    // 数据处理 - 算术运算
    ADD,
    SUB,
    MUL,
    MADD,
    MSUB,
    
    // 数据处理 - 逻辑运算
    AND,
    ORR,
    EOR,
    
    // 数据处理 - 移位
    LSL,
    LSR,
    ASR,
    
    // 加载存储
    LDR,
    LDRB,
    LDRH,
    LDP,
    STR,
    STRB,
    STRH,
    STP,
    
    // 分支
    B,
    BL,
    BR,
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
    
    // 系统寄存器访问
    MRS,
    MSR,
}

/// 指令结构
#[derive(Debug, Clone)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub operands: Vec<Operand>,
    pub address: u64,
    pub encoding: u32,
}

/// 指令信息（用于查找和文档）
#[derive(Debug, Clone)]
pub struct InstructionInfo {
    pub mnemonic: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub format: &'static str,
    pub category: &'static str,
}

/// 构建指令信息表
pub fn build_instruction_table() -> HashMap<InstructionType, InstructionInfo> {
    let mut table = HashMap::new();
    
    // 算术运算
    table.insert(InstructionType::ADD, InstructionInfo {
        mnemonic: "add",
        name: "Add",
        description: "加法运算，将两个寄存器的值相加",
        format: "ADD <Xd|Wd>, <Xn|Wn>, <Xm|Wm|#imm>",
        category: "arithmetic",
    });
    
    table.insert(InstructionType::SUB, InstructionInfo {
        mnemonic: "sub",
        name: "Subtract",
        description: "减法运算，从第一个操作数减去第二个操作数",
        format: "SUB <Xd|Wd>, <Xn|Wn>, <Xm|Wm|#imm>",
        category: "arithmetic",
    });
    
    table.insert(InstructionType::MUL, InstructionInfo {
        mnemonic: "mul",
        name: "Multiply",
        description: "乘法运算，两个寄存器值相乘",
        format: "MUL <Xd|Wd>, <Xn|Wn>, <Xm|Wm>",
        category: "arithmetic",
    });
    
    // 加载存储
    table.insert(InstructionType::LDR, InstructionInfo {
        mnemonic: "ldr",
        name: "Load Register",
        description: "从内存加载数据到寄存器",
        format: "LDR <Xt|Wt>, [<Xn|SP>{, #<imm>}]",
        category: "load_store",
    });
    
    table.insert(InstructionType::STR, InstructionInfo {
        mnemonic: "str",
        name: "Store Register",
        description: "将寄存器数据存储到内存",
        format: "STR <Xt|Wt>, [<Xn|SP>{, #<imm>}]",
        category: "load_store",
    });
    
    table.insert(InstructionType::LDP, InstructionInfo {
        mnemonic: "ldp",
        name: "Load Pair of Registers",
        description: "从内存加载两个寄存器",
        format: "LDP <Xt1>, <Xt2>, [<Xn|SP>{, #<imm>}]",
        category: "load_store",
    });
    
    table.insert(InstructionType::STP, InstructionInfo {
        mnemonic: "stp",
        name: "Store Pair of Registers",
        description: "将两个寄存器存储到内存",
        format: "STP <Xt1>, <Xt2>, [<Xn|SP>{, #<imm>}]",
        category: "load_store",
    });
    
    // 分支指令
    table.insert(InstructionType::B, InstructionInfo {
        mnemonic: "b",
        name: "Branch",
        description: "无条件跳转到指定标签",
        format: "B <label>",
        category: "branch",
    });
    
    table.insert(InstructionType::BL, InstructionInfo {
        mnemonic: "bl",
        name: "Branch with Link",
        description: "跳转并保存返回地址到X30（链接寄存器）",
        format: "BL <label>",
        category: "branch",
    });
    
    table.insert(InstructionType::RET, InstructionInfo {
        mnemonic: "ret",
        name: "Return",
        description: "从子程序返回，默认使用X30",
        format: "RET {<Xn>}",
        category: "branch",
    });
    
    table.insert(InstructionType::BEQ, InstructionInfo {
        mnemonic: "b.eq",
        name: "Branch if Equal",
        description: "如果相等则跳转（Z=1）",
        format: "B.EQ <label>",
        category: "branch_conditional",
    });
    
    table.insert(InstructionType::BNE, InstructionInfo {
        mnemonic: "b.ne",
        name: "Branch if Not Equal",
        description: "如果不相等则跳转（Z=0）",
        format: "B.NE <label>",
        category: "branch_conditional",
    });
    
    table.insert(InstructionType::BHI, InstructionInfo {
        mnemonic: "b.hi",
        name: "Branch if Higher",
        description: "无符号大于时跳转（C=1 且 Z=0）",
        format: "B.HI <label>",
        category: "branch_conditional",
    });
    
    table.insert(InstructionType::BLS, InstructionInfo {
        mnemonic: "b.ls",
        name: "Branch if Lower or Same",
        description: "无符号小于等于时跳转（C=0 或 Z=1）",
        format: "B.LS <label>",
        category: "branch_conditional",
    });
    
    table.insert(InstructionType::BCC, InstructionInfo {
        mnemonic: "b.cc",
        name: "Branch if Carry Clear",
        description: "如果进位标志清零则跳转（C=0）",
        format: "B.CC <label>",
        category: "branch_conditional",
    });
    
    // 比较指令
    table.insert(InstructionType::CMP, InstructionInfo {
        mnemonic: "cmp",
        name: "Compare",
        description: "比较两个值，设置条件标志（相当于 SUB 但不保存结果）",
        format: "CMP <Xn|Wn>, <Xm|Wm|#imm>",
        category: "comparison",
    });
    
    // 数据移动
    table.insert(InstructionType::MOV, InstructionInfo {
        mnemonic: "mov",
        name: "Move",
        description: "移动数据到寄存器",
        format: "MOV <Xd|Wd>, <Xm|Wm|#imm>",
        category: "move",
    });
    
    table.insert(InstructionType::MOVZ, InstructionInfo {
        mnemonic: "movz",
        name: "Move with Zero",
        description: "移动立即数并将其他位清零",
        format: "MOVZ <Xd|Wd>, #<imm>{, LSL #<shift>}",
        category: "move",
    });
    
    // 逻辑运算
    table.insert(InstructionType::LSL, InstructionInfo {
        mnemonic: "lsl",
        name: "Logical Shift Left",
        description: "逻辑左移",
        format: "LSL <Xd|Wd>, <Xn|Wn>, #<shift>",
        category: "shift",
    });
    
    table.insert(InstructionType::AND, InstructionInfo {
        mnemonic: "and",
        name: "Bitwise AND",
        description: "按位与运算",
        format: "AND <Xd|Wd>, <Xn|Wn>, <Xm|Wm>",
        category: "logical",
    });
    
    table.insert(InstructionType::ORR, InstructionInfo {
        mnemonic: "orr",
        name: "Bitwise OR",
        description: "按位或运算",
        format: "ORR <Xd|Wd>, <Xn|Wn>, <Xm|Wm>",
        category: "logical",
    });
    
    // 系统指令
    table.insert(InstructionType::NOP, InstructionInfo {
        mnemonic: "nop",
        name: "No Operation",
        description: "空操作，不执行任何动作",
        format: "NOP",
        category: "system",
    });
    
    table
}

/// 寄存器名称解析
pub fn parse_register(name: &str) -> Option<Register> {
    match name.to_lowercase().as_str() {
        "x0" => Some(Register::X0),
        "x1" => Some(Register::X1),
        "x2" => Some(Register::X2),
        "x3" => Some(Register::X3),
        "x4" => Some(Register::X4),
        "x5" => Some(Register::X5),
        "x6" => Some(Register::X6),
        "x7" => Some(Register::X7),
        "x8" => Some(Register::X8),
        "x9" => Some(Register::X9),
        "x10" => Some(Register::X10),
        "x11" => Some(Register::X11),
        "x12" => Some(Register::X12),
        "x13" => Some(Register::X13),
        "x14" => Some(Register::X14),
        "x15" => Some(Register::X15),
        "x16" => Some(Register::X16),
        "x17" => Some(Register::X17),
        "x18" => Some(Register::X18),
        "x19" => Some(Register::X19),
        "x20" => Some(Register::X20),
        "x21" => Some(Register::X21),
        "x22" => Some(Register::X22),
        "x23" => Some(Register::X23),
        "x24" => Some(Register::X24),
        "x25" => Some(Register::X25),
        "x26" => Some(Register::X26),
        "x27" => Some(Register::X27),
        "x28" => Some(Register::X28),
        "x29" | "fp" => Some(Register::X29),
        "x30" | "lr" => Some(Register::X30),
        
        "w0" => Some(Register::W0),
        "w1" => Some(Register::W1),
        "w2" => Some(Register::W2),
        "w3" => Some(Register::W3),
        "w4" => Some(Register::W4),
        "w5" => Some(Register::W5),
        "w6" => Some(Register::W6),
        "w7" => Some(Register::W7),
        "w8" => Some(Register::W8),
        "w9" => Some(Register::W9),
        "w10" => Some(Register::W10),
        "w11" => Some(Register::W11),
        "w12" => Some(Register::W12),
        "w13" => Some(Register::W13),
        "w14" => Some(Register::W14),
        "w15" => Some(Register::W15),
        "w16" => Some(Register::W16),
        "w17" => Some(Register::W17),
        "w18" => Some(Register::W18),
        "w19" => Some(Register::W19),
        "w20" => Some(Register::W20),
        "w21" => Some(Register::W21),
        "w22" => Some(Register::W22),
        "w23" => Some(Register::W23),
        "w24" => Some(Register::W24),
        "w25" => Some(Register::W25),
        "w26" => Some(Register::W26),
        "w27" => Some(Register::W27),
        "w28" => Some(Register::W28),
        "w29" => Some(Register::W29),
        "w30" => Some(Register::W30),
        
        "sp" => Some(Register::SP),
        "pc" => Some(Register::PC),
        "xzr" => Some(Register::XZR),
        "wzr" => Some(Register::WZR),
        
        _ => None,
    }
}

/// 判断寄存器是否为64位
pub fn is_64bit_register(reg: Register) -> bool {
    matches!(reg, 
        Register::X0 | Register::X1 | Register::X2 | Register::X3 | Register::X4 |
        Register::X5 | Register::X6 | Register::X7 | Register::X8 | Register::X9 |
        Register::X10 | Register::X11 | Register::X12 | Register::X13 | Register::X14 |
        Register::X15 | Register::X16 | Register::X17 | Register::X18 | Register::X19 |
        Register::X20 | Register::X21 | Register::X22 | Register::X23 | Register::X24 |
        Register::X25 | Register::X26 | Register::X27 | Register::X28 | Register::X29 |
        Register::X30 | Register::SP | Register::PC | Register::XZR | Register::FP | Register::LR
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_register_parsing() {
        assert_eq!(parse_register("x0"), Some(Register::X0));
        assert_eq!(parse_register("w15"), Some(Register::W15));
        assert_eq!(parse_register("sp"), Some(Register::SP));
        assert_eq!(parse_register("lr"), Some(Register::X30));
        assert_eq!(parse_register("invalid"), None);
    }
    
    #[test]
    fn test_is_64bit() {
        assert!(is_64bit_register(Register::X0));
        assert!(!is_64bit_register(Register::W0));
        assert!(is_64bit_register(Register::SP));
    }
    
    #[test]
    fn test_instruction_table() {
        let table = build_instruction_table();
        assert!(table.contains_key(&InstructionType::ADD));
        assert!(table.contains_key(&InstructionType::LDR));
        assert!(table.contains_key(&InstructionType::BEQ));
    }
}
