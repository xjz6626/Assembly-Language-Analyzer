//! 汇编指令语义解释器
//! 
//! 将汇编指令转换为人类可读的语义描述

use crate::instruction::{Instruction, InstructionType, Operand};

/// 指令语义解释器
pub struct SemanticInterpreter;

impl SemanticInterpreter {
    /// 解释单条指令
    pub fn interpret(instruction: &Instruction) -> String {
        match instruction.instruction_type {
            InstructionType::ADD => Self::interpret_add(instruction),
            InstructionType::SUB => Self::interpret_sub(instruction),
            InstructionType::MUL => Self::interpret_mul(instruction),
            InstructionType::AND => Self::interpret_and(instruction),
            InstructionType::ORR => Self::interpret_orr(instruction),
            InstructionType::EOR => Self::interpret_eor(instruction),
            InstructionType::LSL => Self::interpret_lsl(instruction),
            InstructionType::LSR => Self::interpret_lsr(instruction),
            InstructionType::ASR => Self::interpret_asr(instruction),
            InstructionType::LDR => Self::interpret_ldr(instruction),
            InstructionType::LDRB => Self::interpret_ldrb(instruction),
            InstructionType::LDRH => Self::interpret_ldrh(instruction),
            InstructionType::LDP => Self::interpret_ldp(instruction),
            InstructionType::STR => Self::interpret_str(instruction),
            InstructionType::STRB => Self::interpret_strb(instruction),
            InstructionType::STRH => Self::interpret_strh(instruction),
            InstructionType::STP => Self::interpret_stp(instruction),
            InstructionType::MOV => Self::interpret_mov(instruction),
            InstructionType::MOVZ => Self::interpret_movz(instruction),
            InstructionType::MOVK => Self::interpret_movk(instruction),
            InstructionType::CMP => Self::interpret_cmp(instruction),
            InstructionType::B => Self::interpret_b(instruction),
            InstructionType::BL => Self::interpret_bl(instruction),
            InstructionType::BR => Self::interpret_br(instruction),
            InstructionType::RET => String::from("从子程序返回"),
            InstructionType::BEQ => String::from("如果相等则跳转 (Z=1)"),
            InstructionType::BNE => String::from("如果不相等则跳转 (Z=0)"),
            InstructionType::BHI => String::from("如果无符号大于则跳转 (C=1且Z=0)"),
            InstructionType::BLS => String::from("如果无符号小于等于则跳转 (C=0或Z=1)"),
            InstructionType::BCC => String::from("如果无进位则跳转 (C=0)"),
            InstructionType::BGE => String::from("如果有符号大于等于则跳转 (N=V)"),
            InstructionType::BLT => String::from("如果有符号小于则跳转 (N≠V)"),
            InstructionType::BGT => String::from("如果有符号大于则跳转 (Z=0且N=V)"),
            InstructionType::BLE => String::from("如果有符号小于等于则跳转 (Z=1或N≠V)"),
            InstructionType::CBZ => Self::interpret_cbz(instruction),
            InstructionType::CBNZ => Self::interpret_cbnz(instruction),
            InstructionType::NOP => String::from("空操作"),
            _ => format!("{:?} 指令", instruction.instruction_type),
        }
    }

    // 各指令的解释函数
    fn interpret_add(inst: &Instruction) -> String {
        if inst.operands.len() >= 3 {
            let dest = Self::operand_name(&inst.operands[0]);
            let src1 = Self::operand_name(&inst.operands[1]);
            let src2 = Self::operand_name(&inst.operands[2]);
            format!("{} = {} + {}", dest, src1, src2)
        } else {
            String::from("加法运算")
        }
    }

    fn interpret_sub(inst: &Instruction) -> String {
        if inst.operands.len() >= 3 {
            let dest = Self::operand_name(&inst.operands[0]);
            let src1 = Self::operand_name(&inst.operands[1]);
            let src2 = Self::operand_name(&inst.operands[2]);
            format!("{} = {} - {}", dest, src1, src2)
        } else {
            String::from("减法运算")
        }
    }

    fn interpret_mul(inst: &Instruction) -> String {
        if inst.operands.len() >= 3 {
            let dest = Self::operand_name(&inst.operands[0]);
            let src1 = Self::operand_name(&inst.operands[1]);
            let src2 = Self::operand_name(&inst.operands[2]);
            format!("{} = {} × {}", dest, src1, src2)
        } else {
            String::from("乘法运算")
        }
    }

    fn interpret_and(inst: &Instruction) -> String {
        if inst.operands.len() >= 3 {
            let dest = Self::operand_name(&inst.operands[0]);
            let src1 = Self::operand_name(&inst.operands[1]);
            let src2 = Self::operand_name(&inst.operands[2]);
            format!("{} = {} & {}", dest, src1, src2)
        } else {
            String::from("按位与")
        }
    }

    fn interpret_orr(inst: &Instruction) -> String {
        if inst.operands.len() >= 3 {
            let dest = Self::operand_name(&inst.operands[0]);
            let src1 = Self::operand_name(&inst.operands[1]);
            let src2 = Self::operand_name(&inst.operands[2]);
            format!("{} = {} | {}", dest, src1, src2)
        } else {
            String::from("按位或")
        }
    }

    fn interpret_eor(inst: &Instruction) -> String {
        if inst.operands.len() >= 3 {
            let dest = Self::operand_name(&inst.operands[0]);
            let src1 = Self::operand_name(&inst.operands[1]);
            let src2 = Self::operand_name(&inst.operands[2]);
            format!("{} = {} ^ {}", dest, src1, src2)
        } else {
            String::from("按位异或")
        }
    }

    fn interpret_lsl(inst: &Instruction) -> String {
        if inst.operands.len() >= 3 {
            let dest = Self::operand_name(&inst.operands[0]);
            let src = Self::operand_name(&inst.operands[1]);
            let shift = Self::operand_name(&inst.operands[2]);
            format!("{} = {} << {}", dest, src, shift)
        } else {
            String::from("逻辑左移")
        }
    }

    fn interpret_lsr(inst: &Instruction) -> String {
        if inst.operands.len() >= 3 {
            let dest = Self::operand_name(&inst.operands[0]);
            let src = Self::operand_name(&inst.operands[1]);
            let shift = Self::operand_name(&inst.operands[2]);
            format!("{} = {} >> {}", dest, src, shift)
        } else {
            String::from("逻辑右移")
        }
    }

    fn interpret_asr(inst: &Instruction) -> String {
        if inst.operands.len() >= 3 {
            let dest = Self::operand_name(&inst.operands[0]);
            let src = Self::operand_name(&inst.operands[1]);
            let shift = Self::operand_name(&inst.operands[2]);
            format!("{} = {} >> {} (算术)", dest, src, shift)
        } else {
            String::from("算术右移")
        }
    }

    fn interpret_ldr(inst: &Instruction) -> String {
        if inst.operands.len() >= 2 {
            let dest = Self::operand_name(&inst.operands[0]);
            let mem = Self::memory_operand_desc(&inst.operands[1]);
            format!("从 {} 加载到 {}", mem, dest)
        } else {
            String::from("从内存加载")
        }
    }

    fn interpret_ldrb(inst: &Instruction) -> String {
        if inst.operands.len() >= 2 {
            let dest = Self::operand_name(&inst.operands[0]);
            let mem = Self::memory_operand_desc(&inst.operands[1]);
            format!("从 {} 加载字节到 {}", mem, dest)
        } else {
            String::from("从内存加载字节")
        }
    }

    fn interpret_ldrh(inst: &Instruction) -> String {
        if inst.operands.len() >= 2 {
            let dest = Self::operand_name(&inst.operands[0]);
            let mem = Self::memory_operand_desc(&inst.operands[1]);
            format!("从 {} 加载半字到 {}", mem, dest)
        } else {
            String::from("从内存加载半字")
        }
    }

    fn interpret_ldp(inst: &Instruction) -> String {
        if inst.operands.len() >= 3 {
            let dest1 = Self::operand_name(&inst.operands[0]);
            let dest2 = Self::operand_name(&inst.operands[1]);
            let mem = Self::memory_operand_desc(&inst.operands[2]);
            format!("从 {} 加载 {} 和 {}", mem, dest1, dest2)
        } else {
            String::from("从内存加载一对寄存器")
        }
    }

    fn interpret_str(inst: &Instruction) -> String {
        if inst.operands.len() >= 2 {
            let src = Self::operand_name(&inst.operands[0]);
            let mem = Self::memory_operand_desc(&inst.operands[1]);
            format!("将 {} 存储到 {}", src, mem)
        } else {
            String::from("存储到内存")
        }
    }

    fn interpret_strb(inst: &Instruction) -> String {
        if inst.operands.len() >= 2 {
            let src = Self::operand_name(&inst.operands[0]);
            let mem = Self::memory_operand_desc(&inst.operands[1]);
            format!("将 {} (字节) 存储到 {}", src, mem)
        } else {
            String::from("存储字节到内存")
        }
    }

    fn interpret_strh(inst: &Instruction) -> String {
        if inst.operands.len() >= 2 {
            let src = Self::operand_name(&inst.operands[0]);
            let mem = Self::memory_operand_desc(&inst.operands[1]);
            format!("将 {} (半字) 存储到 {}", src, mem)
        } else {
            String::from("存储半字到内存")
        }
    }

    fn interpret_stp(inst: &Instruction) -> String {
        if inst.operands.len() >= 3 {
            let src1 = Self::operand_name(&inst.operands[0]);
            let src2 = Self::operand_name(&inst.operands[1]);
            let mem = Self::memory_operand_desc(&inst.operands[2]);
            format!("将 {} 和 {} 存储到 {}", src1, src2, mem)
        } else {
            String::from("存储一对寄存器到内存")
        }
    }

    fn interpret_mov(inst: &Instruction) -> String {
        if inst.operands.len() >= 2 {
            let dest = Self::operand_name(&inst.operands[0]);
            let src = Self::operand_name(&inst.operands[1]);
            format!("{} = {}", dest, src)
        } else {
            String::from("数据移动")
        }
    }

    fn interpret_movz(inst: &Instruction) -> String {
        if inst.operands.len() >= 2 {
            let dest = Self::operand_name(&inst.operands[0]);
            let src = Self::operand_name(&inst.operands[1]);
            format!("{} = {} (其他位清零)", dest, src)
        } else {
            String::from("移动立即数并清零")
        }
    }

    fn interpret_movk(inst: &Instruction) -> String {
        if inst.operands.len() >= 2 {
            let dest = Self::operand_name(&inst.operands[0]);
            let src = Self::operand_name(&inst.operands[1]);
            format!("{} 的部分位 = {} (保持其他位)", dest, src)
        } else {
            String::from("移动立即数并保持")
        }
    }

    fn interpret_cmp(inst: &Instruction) -> String {
        if inst.operands.len() >= 2 {
            let src1 = Self::operand_name(&inst.operands[0]);
            let src2 = Self::operand_name(&inst.operands[1]);
            format!("比较 {} 和 {} (设置标志位)", src1, src2)
        } else {
            String::from("比较")
        }
    }

    fn interpret_b(inst: &Instruction) -> String {
        if !inst.operands.is_empty() {
            let target = Self::operand_name(&inst.operands[0]);
            format!("无条件跳转到 {}", target)
        } else {
            String::from("无条件跳转")
        }
    }

    fn interpret_bl(inst: &Instruction) -> String {
        if !inst.operands.is_empty() {
            let target = Self::operand_name(&inst.operands[0]);
            format!("调用函数 {} (保存返回地址)", target)
        } else {
            String::from("调用函数")
        }
    }

    fn interpret_br(inst: &Instruction) -> String {
        if !inst.operands.is_empty() {
            let target = Self::operand_name(&inst.operands[0]);
            format!("跳转到寄存器 {} 中的地址", target)
        } else {
            String::from("跳转到寄存器地址")
        }
    }

    fn interpret_cbz(inst: &Instruction) -> String {
        if inst.operands.len() >= 2 {
            let reg = Self::operand_name(&inst.operands[0]);
            let target = Self::operand_name(&inst.operands[1]);
            format!("如果 {} == 0 则跳转到 {}", reg, target)
        } else {
            String::from("比较为零则跳转")
        }
    }

    fn interpret_cbnz(inst: &Instruction) -> String {
        if inst.operands.len() >= 2 {
            let reg = Self::operand_name(&inst.operands[0]);
            let target = Self::operand_name(&inst.operands[1]);
            format!("如果 {} ≠ 0 则跳转到 {}", reg, target)
        } else {
            String::from("比较非零则跳转")
        }
    }

    // 辅助函数

    fn operand_name(operand: &Operand) -> String {
        match operand {
            Operand::Register(reg) => format!("{:?}", reg),
            Operand::Immediate(imm) => {
                if *imm < 0 {
                    format!("{}", imm)
                } else {
                    format!("0x{:x}", imm)
                }
            }
            Operand::Label(label) => label.clone(),
            Operand::Memory { base, offset, .. } => {
                if let Some(off) = offset {
                    if *off >= 0 {
                        format!("[{:?}+0x{:x}]", base, off)
                    } else {
                        format!("[{:?}-0x{:x}]", base, -off)
                    }
                } else {
                    format!("[{:?}]", base)
                }
            }
        }
    }

    fn memory_operand_desc(operand: &Operand) -> String {
        match operand {
            Operand::Memory { base, offset, index, .. } => {
                let mut desc = format!("({:?}", base);
                if let Some(off) = offset {
                    if *off >= 0 {
                        desc.push_str(&format!(" + 0x{:x}", off));
                    } else {
                        desc.push_str(&format!(" - 0x{:x}", -off));
                    }
                }
                if let Some(idx) = index {
                    desc.push_str(&format!(" + {:?}", idx));
                }
                desc.push(')');
                desc
            }
            _ => Self::operand_name(operand),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::Register;

    #[test]
    fn test_interpret_add() {
        let inst = Instruction::new(
            InstructionType::ADD,
            vec![
                Operand::Register(Register::X0),
                Operand::Register(Register::X1),
                Operand::Register(Register::X2),
            ],
            0,
        );
        let interpretation = SemanticInterpreter::interpret(&inst);
        assert_eq!(interpretation, "X0 = X1 + X2");
    }

    #[test]
    fn test_interpret_ldr() {
        let inst = Instruction::new(
            InstructionType::LDR,
            vec![
                Operand::Register(Register::X0),
                Operand::Memory {
                    base: Register::SP,
                    offset: Some(8),
                    index: None,
                    pre_indexed: false,
                    post_indexed: false,
                },
            ],
            0,
        );
        let interpretation = SemanticInterpreter::interpret(&inst);
        assert!(interpretation.contains("X0"));
        assert!(interpretation.contains("SP"));
    }
}
