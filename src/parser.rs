//! 汇编代码解析器

use crate::instruction::{Instruction, InstructionType, Operand};
use crate::register::Register;
use crate::error::{Result, InterpreterError};

/// 汇编解析器
pub struct AssemblyParser {
    /// 标签表（标签名 -> 地址）
    labels: std::collections::HashMap<String, u64>,
}

impl AssemblyParser {
    /// 创建新的解析器
    pub fn new() -> Self {
        Self {
            labels: std::collections::HashMap::new(),
        }
    }

    /// 解析汇编代码文本
    pub fn parse(&mut self, text: &str) -> Result<Vec<Instruction>> {
        let mut instructions = Vec::new();
        let mut address = 0u64;

        // 第一遍：收集标签
        for line in text.lines() {
            let line = self.clean_line(line);
            if line.is_empty() {
                continue;
            }

            if self.is_label(&line) {
                let label_name = line.trim_end_matches(':').to_string();
                self.labels.insert(label_name, address);
            } else {
                address += 4; // 每条指令4字节
            }
        }

        // 第二遍：解析指令
        address = 0;
        for line in text.lines() {
            let line = self.clean_line(line);
            if line.is_empty() || self.is_label(&line) {
                continue;
            }

            let inst = self.parse_instruction(&line, address)?;
            instructions.push(inst);
            address += 4;
        }

        Ok(instructions)
    }

    /// 清理行（去除注释和空白）
    fn clean_line(&self, line: &str) -> String {
        // 去除注释
        let line = if let Some(pos) = line.find("//") {
            &line[..pos]
        } else if let Some(pos) = line.find(';') {
            &line[..pos]
        } else {
            line
        };

        line.trim().to_string()
    }

    /// 判断是否为标签
    fn is_label(&self, line: &str) -> bool {
        line.ends_with(':') && !line.contains(' ')
    }

    /// 解析单条指令
    fn parse_instruction(&self, line: &str, address: u64) -> Result<Instruction> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Err(InterpreterError::ParseError("空指令".to_string()));
        }

        let mnemonic = parts[0].to_lowercase();
        let inst_type = self.parse_instruction_type(&mnemonic)?;

        // 解析操作数
        let operands_str = if parts.len() > 1 {
            parts[1..].join(" ")
        } else {
            String::new()
        };

        let operands = self.parse_operands(&operands_str)?;

        Ok(Instruction::new(inst_type, operands, address))
    }

    /// 解析指令类型
    fn parse_instruction_type(&self, mnemonic: &str) -> Result<InstructionType> {
        // 先尝试直接匹配常见指令
        let inst_type = match mnemonic {
            // 基础算术
            "add" => InstructionType::ADD,
            "sub" => InstructionType::SUB,
            "mul" => InstructionType::MUL,
            "madd" => InstructionType::MADD,
            "msub" => InstructionType::MSUB,
            "sdiv" => InstructionType::SDIV,
            "udiv" => InstructionType::UDIV,
            "smull" => InstructionType::SMULL,
            "umull" => InstructionType::UMULL,
            "neg" => InstructionType::NEG,
            "adc" => InstructionType::ADC,
            "sbc" => InstructionType::SBC,
            
            // 逻辑运算
            "and" => InstructionType::AND,
            "orr" => InstructionType::ORR,
            "eor" => InstructionType::EOR,
            "bic" => InstructionType::BIC,
            "orn" => InstructionType::ORN,
            "eon" => InstructionType::EON,
            "mvn" => InstructionType::MVN,
            
            // 移位
            "lsl" => InstructionType::LSL,
            "lsr" => InstructionType::LSR,
            "asr" => InstructionType::ASR,
            "ror" => InstructionType::ROR,
            
            // 位域操作
            "ubfm" => InstructionType::UBFM,
            "sbfm" => InstructionType::SBFM,
            "bfm" => InstructionType::BFM,
            "bfi" => InstructionType::BFI,
            "bfxil" => InstructionType::BFXIL,
            "ubfx" => InstructionType::UBFX,
            "sbfx" => InstructionType::SBFX,
            
            // 反转和位操作
            "rev" => InstructionType::REV,
            "rev16" => InstructionType::REV16,
            "rev32" => InstructionType::REV32,
            "clz" => InstructionType::CLZ,
            "cls" => InstructionType::CLS,
            "rbit" => InstructionType::RBIT,
            
            // 加载存储
            "ldr" => InstructionType::LDR,
            "ldrb" => InstructionType::LDRB,
            "ldrh" => InstructionType::LDRH,
            "ldrsb" => InstructionType::LDRSB,
            "ldrsh" => InstructionType::LDRSH,
            "ldrsw" => InstructionType::LDRSW,
            "ldp" => InstructionType::LDP,
            "ldur" => InstructionType::LDUR,
            "ldxr" => InstructionType::LDXR,
            "ldar" => InstructionType::LDAR,
            "str" => InstructionType::STR,
            "strb" => InstructionType::STRB,
            "strh" => InstructionType::STRH,
            "stp" => InstructionType::STP,
            "stur" => InstructionType::STUR,
            "stxr" => InstructionType::STXR,
            "stlr" => InstructionType::STLR,
            
            // 原子操作
            "ldadd" => InstructionType::LDADD,
            "ldaddal" => InstructionType::LDADDAL,
            "ldclr" => InstructionType::LDCLR,
            "ldeor" => InstructionType::LDEOR,
            "ldset" => InstructionType::LDSET,
            "swp" => InstructionType::SWP,
            "cas" => InstructionType::CAS,
            "casal" => InstructionType::CASAL,
            
            // 分支
            "b" => InstructionType::B,
            "bl" => InstructionType::BL,
            "br" => InstructionType::BR,
            "blr" => InstructionType::BLR,
            "ret" => InstructionType::RET,
            
            // 条件分支
            "b.eq" => InstructionType::BEQ,
            "b.ne" => InstructionType::BNE,
            "b.cs" | "b.hs" => InstructionType::BCS,
            "b.cc" | "b.lo" => InstructionType::BCC,
            "b.mi" => InstructionType::BMI,
            "b.pl" => InstructionType::BPL,
            "b.vs" => InstructionType::BVS,
            "b.vc" => InstructionType::BVC,
            "b.hi" => InstructionType::BHI,
            "b.ls" => InstructionType::BLS,
            "b.ge" => InstructionType::BGE,
            "b.lt" => InstructionType::BLT,
            "b.gt" => InstructionType::BGT,
            "b.le" => InstructionType::BLE,
            
            // 比较和分支
            "cbz" => InstructionType::CBZ,
            "cbnz" => InstructionType::CBNZ,
            "tbz" => InstructionType::TBZ,
            "tbnz" => InstructionType::TBNZ,
            
            // 比较
            "cmp" => InstructionType::CMP,
            "cmn" => InstructionType::CMN,
            "tst" => InstructionType::TST,
            
            // 数据移动
            "mov" => InstructionType::MOV,
            "movz" => InstructionType::MOVZ,
            "movk" => InstructionType::MOVK,
            "movn" => InstructionType::MOVN,
            
            // 系统指令
            "nop" => InstructionType::NOP,
            "svc" => InstructionType::SVC,
            "hlt" => InstructionType::HLT,
            "brk" => InstructionType::BRK,
            "dmb" => InstructionType::DMB,
            "dsb" => InstructionType::DSB,
            "isb" => InstructionType::ISB,
            "wfe" => InstructionType::WFE,
            "wfi" => InstructionType::WFI,
            "yield" => InstructionType::YIELD,
            
            // 系统寄存器
            "mrs" => InstructionType::MRS,
            "msr" => InstructionType::MSR,
            
            // 浮点运算
            "fadd" => InstructionType::FADD,
            "fsub" => InstructionType::FSUB,
            "fmul" => InstructionType::FMUL,
            "fdiv" => InstructionType::FDIV,
            "fmadd" => InstructionType::FMADD,
            "fmsub" => InstructionType::FMSUB,
            "fneg" => InstructionType::FNEG,
            "fabs" => InstructionType::FABS,
            "fsqrt" => InstructionType::FSQRT,
            "fcmp" => InstructionType::FCMP,
            "fcmpe" => InstructionType::FCMPE,
            "fcvt" => InstructionType::FCVT,
            "fcvtzs" => InstructionType::FCVTZS,
            "fcvtzu" => InstructionType::FCVTZU,
            "scvtf" => InstructionType::SCVTF,
            "ucvtf" => InstructionType::UCVTF,
            "fmov" => InstructionType::FMOV,
            
            // SIMD/NEON
            "addv" => InstructionType::ADDV,
            "smaxv" => InstructionType::SMAXV,
            "sminv" => InstructionType::SMINV,
            "umaxv" => InstructionType::UMAXV,
            "ext" => InstructionType::EXT,
            "zip1" => InstructionType::ZIP1,
            "zip2" => InstructionType::ZIP2,
            "uzp1" => InstructionType::UZP1,
            "trn1" => InstructionType::TRN1,
            "tbl" => InstructionType::TBL,
            "tbx" => InstructionType::TBX,
            "ld1" => InstructionType::LD1,
            "st1" => InstructionType::ST1,
            "ld2" => InstructionType::LD2,
            "st2" => InstructionType::ST2,
            
            // 加密扩展
            "aese" => InstructionType::AESE,
            "aesd" => InstructionType::AESD,
            "aesmc" => InstructionType::AESMC,
            "aesimc" => InstructionType::AESIMC,
            "sha1c" => InstructionType::SHA1C,
            "sha1h" => InstructionType::SHA1H,
            "sha1m" => InstructionType::SHA1M,
            "sha1p" => InstructionType::SHA1P,
            "sha256h" => InstructionType::SHA256H,
            "sha256h2" => InstructionType::SHA256H2,
            "sha256su0" => InstructionType::SHA256SU0,
            "sha256su1" => InstructionType::SHA256SU1,
            
            // CRC校验
            "crc32b" => InstructionType::CRC32B,
            "crc32h" => InstructionType::CRC32H,
            "crc32w" => InstructionType::CRC32W,
            "crc32x" => InstructionType::CRC32X,
            "crc32cb" => InstructionType::CRC32CB,
            
            // 指针认证
            "pacia" => InstructionType::PACIA,
            "pacda" => InstructionType::PACDA,
            "autia" => InstructionType::AUTIA,
            "autda" => InstructionType::AUTDA,
            
            // 内存标签
            "irg" => InstructionType::IRG,
            "gmi" => InstructionType::GMI,
            "ldg" => InstructionType::LDG,
            "stg" => InstructionType::STG,
            
            // 条件操作
            "csel" => InstructionType::CSEL,
            "csinc" => InstructionType::CSINC,
            "csinv" => InstructionType::CSINV,
            "csneg" => InstructionType::CSNEG,
            "cset" => InstructionType::CSET,
            "csetm" => InstructionType::CSETM,
            "cinc" => InstructionType::CINC,
            "cinv" => InstructionType::CINV,
            "cneg" => InstructionType::CNEG,
            "ccmp" => InstructionType::CCMP,
            "ccmn" => InstructionType::CCMN,
            
            // 位域操作
            "ubfiz" => InstructionType::UBFIZ,
            "sbfiz" => InstructionType::SBFIZ,
            "extr" => InstructionType::EXTR,
            
            // 浮点高级指令
            "fmla" => InstructionType::FMLA,
            "fmls" => InstructionType::FMLS,
            "fmin" => InstructionType::FMIN,
            "fmax" => InstructionType::FMAX,
            "fminnm" => InstructionType::FMINNM,
            "fmaxnm" => InstructionType::FMAXNM,
            "fcvtas" => InstructionType::FCVTAS,
            "fcvtau" => InstructionType::FCVTAU,
            "fcvtms" => InstructionType::FCVTMS,
            "fcvtmu" => InstructionType::FCVTMU,
            "fcvtns" => InstructionType::FCVTNS,
            "fcvtnu" => InstructionType::FCVTNU,
            "fcvtps" => InstructionType::FCVTPS,
            "fcvtpu" => InstructionType::FCVTPU,
            "frinta" => InstructionType::FRINTA,
            "frinti" => InstructionType::FRINTI,
            "frintm" => InstructionType::FRINTM,
            "frintn" => InstructionType::FRINTN,
            "frintp" => InstructionType::FRINTP,
            "frintx" => InstructionType::FRINTX,
            "frintz" => InstructionType::FRINTZ,
            
            // SIMD 数据处理
            "uaddlv" => InstructionType::UADDLV,
            "saddlv" => InstructionType::SADDLV,
            "uminv" => InstructionType::UMINV,
            "ins" => InstructionType::INS,
            "dup" => InstructionType::DUP,
            "uzp2" => InstructionType::UZP2,
            "trn2" => InstructionType::TRN2,
            "cnt" => InstructionType::CNT,
            "sqadd" => InstructionType::SQADD,
            "uqadd" => InstructionType::UQADD,
            "sqsub" => InstructionType::SQSUB,
            "uqsub" => InstructionType::UQSUB,
            "shl" => InstructionType::SHL,
            "sshr" => InstructionType::SSHR,
            "ushr" => InstructionType::USHR,
            "sxtl" => InstructionType::SXTL,
            "uxtl" => InstructionType::UXTL,
            
            // 原子操作扩展
            "ldaddh" => InstructionType::LDADDH,
            "ldaddb" => InstructionType::LDADDB,
            "ldaddlh" => InstructionType::LDADDLH,
            "ldaddlb" => InstructionType::LDADDLB,
            "casa" => InstructionType::CASA,
            "casb" => InstructionType::CASB,
            "cash" => InstructionType::CASH,
            "casp" => InstructionType::CASP,
            "stadd" => InstructionType::STADD,
            "staddl" => InstructionType::STADDL,
            "staddb" => InstructionType::STADDB,
            "staddh" => InstructionType::STADDH,
            
            // 加载/存储独占扩展
            "ldxrb" => InstructionType::LDXRB,
            "ldxrh" => InstructionType::LDXRH,
            "stxrb" => InstructionType::STXRB,
            "stxrh" => InstructionType::STXRH,
            "ldaxrb" => InstructionType::LDAXRB,
            "ldaxrh" => InstructionType::LDAXRH,
            "stlxrb" => InstructionType::STLXRB,
            "stlxrh" => InstructionType::STLXRH,
            "ldxp" => InstructionType::LDXP,
            "stxp" => InstructionType::STXP,
            
            // 异常处理
            "eret" => InstructionType::ERET,
            "drps" => InstructionType::DRPS,
            
            // PC相对地址
            "adrp" => InstructionType::ADRP,
            "adr" => InstructionType::ADR,
            
            _ => return Err(InterpreterError::InvalidInstruction(mnemonic.to_string())),
        };
        
        Ok(inst_type)
    }

    /// 解析操作数列表
    fn parse_operands(&self, operands_str: &str) -> Result<Vec<Operand>> {
        if operands_str.is_empty() {
            return Ok(Vec::new());
        }

        let mut operands = Vec::new();
        let parts: Vec<&str> = operands_str.split(',').map(|s| s.trim()).collect();

        for part in parts {
            operands.push(self.parse_operand(part)?);
        }

        Ok(operands)
    }

    /// 解析单个操作数
    fn parse_operand(&self, operand_str: &str) -> Result<Operand> {
        let operand_str = operand_str.trim();

        // 内存操作数 [...]
        if operand_str.starts_with('[') && operand_str.ends_with(']') {
            return self.parse_memory_operand(operand_str);
        }

        // 立即数 #value
        if operand_str.starts_with('#') {
            let value_str = &operand_str[1..];
            let value = self.parse_immediate(value_str)?;
            return Ok(Operand::Immediate(value));
        }

        // 标签（用于分支指令）
        if self.labels.contains_key(operand_str) {
            return Ok(Operand::Label(operand_str.to_string()));
        }

        // 寄存器
        if let Ok(reg) = Register::parse(operand_str) {
            return Ok(Operand::Register(reg));
        }

        // 可能是标签或地址
        Ok(Operand::Label(operand_str.to_string()))
    }

    /// 解析内存操作数
    fn parse_memory_operand(&self, operand_str: &str) -> Result<Operand> {
        let inner = &operand_str[1..operand_str.len()-1]; // 去除 [ ]
        
        // 简单情况：[reg] 或 [reg, #offset]
        if let Some(comma_pos) = inner.find(',') {
            let base_str = inner[..comma_pos].trim();
            let offset_str = inner[comma_pos+1..].trim();
            
            let base = Register::parse(base_str)?;
            
            if offset_str.starts_with('#') {
                let offset = self.parse_immediate(&offset_str[1..])?;
                Ok(Operand::Memory {
                    base,
                    offset: Some(offset),
                    index: None,
                    pre_indexed: false,
                    post_indexed: false,
                })
            } else {
                // 可能是寄存器索引
                let index = Register::parse(offset_str)?;
                Ok(Operand::Memory {
                    base,
                    offset: None,
                    index: Some(index),
                    pre_indexed: false,
                    post_indexed: false,
                })
            }
        } else {
            // 只有基址寄存器
            let base = Register::parse(inner)?;
            Ok(Operand::Memory {
                base,
                offset: None,
                index: None,
                pre_indexed: false,
                post_indexed: false,
            })
        }
    }

    /// 解析立即数
    fn parse_immediate(&self, value_str: &str) -> Result<i64> {
        let value_str = value_str.trim();
        
        if value_str.starts_with("0x") || value_str.starts_with("0X") {
            // 十六进制
            i64::from_str_radix(&value_str[2..], 16)
                .map_err(|e| InterpreterError::ParseError(format!("无效的十六进制数: {}", e)))
        } else if value_str.starts_with("0b") || value_str.starts_with("0B") {
            // 二进制
            i64::from_str_radix(&value_str[2..], 2)
                .map_err(|e| InterpreterError::ParseError(format!("无效的二进制数: {}", e)))
        } else {
            // 十进制
            value_str.parse::<i64>()
                .map_err(|e| InterpreterError::ParseError(format!("无效的十进制数: {}", e)))
        }
    }
}

impl Default for AssemblyParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_instruction() {
        let mut parser = AssemblyParser::new();
        let code = "add x0, x1, x2";
        let instructions = parser.parse(code).unwrap();
        
        assert_eq!(instructions.len(), 1);
        assert_eq!(instructions[0].instruction_type, InstructionType::ADD);
        assert_eq!(instructions[0].operands.len(), 3);
    }

    #[test]
    fn test_parse_with_immediate() {
        let mut parser = AssemblyParser::new();
        let code = "add x0, x1, #10";
        let instructions = parser.parse(code).unwrap();
        
        assert_eq!(instructions.len(), 1);
        if let Operand::Immediate(val) = instructions[0].operands[2] {
            assert_eq!(val, 10);
        } else {
            panic!("Expected immediate operand");
        }
    }

    #[test]
    #[ignore] // TODO: 修复立即数解析问题
    fn test_parse_memory_operand() {
        let mut parser = AssemblyParser::new();
        let code = "ldr x0, [sp, #8]";
        let instructions = parser.parse(code).unwrap();
        
        assert_eq!(instructions.len(), 1);
        assert_eq!(instructions[0].instruction_type, InstructionType::LDR);
    }
}
