//! Markdown 表格生成器
//! 
//! 生成汇编代码和 C 代码对应关系的 Markdown 表格

use crate::objdump::DumpEntry;
use crate::semantic::SemanticInterpreter;
use std::path::PathBuf;
use std::fs;
use std::io::Write;

/// 表格生成器
pub struct TableGenerator {
    /// C 代码列宽度
    c_code_width: usize,
}

impl TableGenerator {
    pub fn new() -> Self {
        Self {
            c_code_width: 80,  // 增加到 80，确保提示信息完整显示
        }
    }

    /// 生成单个优化级别的表格
    pub fn generate_table(&self, entries: &[DumpEntry]) -> String {
        let mut output = String::new();
        
        // 表头
        output.push_str("| C代码 | 汇编指令 | 语义解释 |\n");
        output.push_str("|-------|----------|----------|\n");
        
        // 按 C 代码分组
        let mut current_c_code = String::new();
        
        for entry in entries {
            // 如果汇编指令为空，说明这是一条提示信息（不截断）
            if entry.asm_instruction.is_empty() {
                output.push_str(&format!(
                    "| {} | | |\n",
                    &entry.c_code  // 提示信息不截断
                ));
                continue;
            }
            
            let c_code = if entry.c_code.is_empty() || entry.c_code == current_c_code {
                String::from("") // 相同的 C 代码不重复显示
            } else {
                current_c_code = entry.c_code.clone();
                self.format_c_code(&entry.c_code)
            };
            
            let asm_inst = &entry.asm_instruction;
            
            // 获取语义解释
            let semantic = if let Some(ref parsed) = entry.parsed_instruction {
                SemanticInterpreter::interpret(parsed)
            } else {
                // 如果无法解析，尝试提供基本解释
                Self::basic_interpret(asm_inst)
            };
            
            output.push_str(&format!(
                "| {} | {} | {} |\n",
                c_code, asm_inst, semantic
            ));
        }
        
        output
    }
    
    /// 为无法解析的指令提供基本解释
    fn basic_interpret(asm_inst: &str) -> String {
        let inst_lower = asm_inst.to_lowercase();
        
        // 尝试提取基本的操作数信息
        if inst_lower.starts_with("ldp") {
            Self::interpret_ldp_basic(asm_inst)
        } else if inst_lower.starts_with("stp") {
            Self::interpret_stp_basic(asm_inst)
        } else if inst_lower.starts_with("ldr") {
            Self::interpret_ldr_basic(asm_inst)
        } else if inst_lower.starts_with("str") {
            Self::interpret_str_basic(asm_inst)
        } else if inst_lower.starts_with("bl ") {
            String::from("调用函数")
        } else if inst_lower.starts_with("b.") {
            String::from("条件跳转")
        } else if inst_lower.starts_with("b ") {
            String::from("无条件跳转")
        } else if inst_lower.starts_with("ccmp") {
            String::from("条件比较")
        } else if inst_lower.starts_with("mov") {
            Self::interpret_mov_basic(asm_inst)
        } else if inst_lower.starts_with("add") {
            String::from("加法运算")
        } else if inst_lower.starts_with("sub") {
            String::from("减法运算")
        } else if inst_lower.starts_with("cmp") {
            String::from("比较运算")
        } else if inst_lower.starts_with("ret") {
            String::from("函数返回")
        } else if inst_lower.starts_with("nop") {
            String::from("空操作")
        } else {
            String::from("指令")
        }
    }
    
    fn interpret_ldr_basic(asm: &str) -> String {
        // 尝试提取目标寄存器
        if let Some(parts) = asm.split_whitespace().nth(1) {
            if let Some(reg) = parts.split(',').next() {
                return format!("从内存加载到 {}", reg.trim());
            }
        }
        String::from("从内存加载")
    }
    
    fn interpret_str_basic(asm: &str) -> String {
        // 尝试提取源寄存器
        if let Some(parts) = asm.split_whitespace().nth(1) {
            if let Some(reg) = parts.split(',').next() {
                return format!("将 {} 存储到内存", reg.trim());
            }
        }
        String::from("存储到内存")
    }
    
    fn interpret_ldp_basic(asm: &str) -> String {
        // 提取两个目标寄存器
        if let Some(operands) = asm.split_whitespace().nth(1) {
            let regs: Vec<&str> = operands.split(',').take(2).collect();
            if regs.len() == 2 {
                return format!("从内存加载 {} 和 {}", regs[0].trim(), regs[1].trim());
            }
        }
        String::from("从内存加载一对寄存器")
    }
    
    fn interpret_stp_basic(asm: &str) -> String {
        // 提取两个源寄存器
        if let Some(operands) = asm.split_whitespace().nth(1) {
            let regs: Vec<&str> = operands.split(',').take(2).collect();
            if regs.len() == 2 {
                return format!("将 {} 和 {} 存储到内存", regs[0].trim(), regs[1].trim());
            }
        }
        String::from("存储一对寄存器到内存")
    }
    
    fn interpret_mov_basic(asm: &str) -> String {
        if let Some(operands) = asm.split_whitespace().nth(1) {
            let parts: Vec<&str> = operands.split(',').take(2).collect();
            if parts.len() == 2 {
                return format!("{} = {}", parts[0].trim(), parts[1].trim());
            }
        }
        String::from("数据移动")
    }

    /// 生成多个优化级别的对比表格
    pub fn generate_comparison_table(
        &self,
        o0_entries: &[DumpEntry],
        o1_entries: &[DumpEntry],
        o2_entries: &[DumpEntry],
    ) -> String {
        let mut output = String::new();
        
        output.push_str("## 优化级别对比\n\n");
        
        // O0 表格
        output.push_str("### O0 (无优化)\n\n");
        output.push_str(&self.generate_table(o0_entries));
        output.push_str("\n");
        
        // O1 表格
        output.push_str("### O1 (基础优化)\n\n");
        output.push_str(&self.generate_table(o1_entries));
        output.push_str("\n");
        
        // O2 表格
        output.push_str("### O2 (高级优化)\n\n");
        output.push_str(&self.generate_table(o2_entries));
        output.push_str("\n");
        
        // 统计信息
        output.push_str("### 统计信息\n\n");
        output.push_str(&format!("- O0: {} 条指令\n", o0_entries.len()));
        output.push_str(&format!("- O1: {} 条指令\n", o1_entries.len()));
        output.push_str(&format!("- O2: {} 条指令\n", o2_entries.len()));
        output.push_str("\n");
        
        output
    }

    /// 格式化 C 代码（处理过长的代码）
    fn format_c_code(&self, code: &str) -> String {
        if code.is_empty() {
            return String::from("");
        }
        
        // 替换 <br> 为空格，但保留换行的语义
        let code = code.replace("<br>", " ");
        
        // 清理多余空格
        let code = code.split_whitespace().collect::<Vec<_>>().join(" ");
        
        // 如果太长，智能截断（在合适的位置）
        if code.len() > self.c_code_width {
            // 尝试在逗号、分号、括号等位置截断
            if let Some(pos) = code[..self.c_code_width].rfind(|c: char| c == ',' || c == ';' || c == ')' || c == ' ') {
                format!("{}...", &code[..pos + 1].trim())
            } else {
                format!("{}...", &code[..self.c_code_width - 3])
            }
        } else {
            code
        }
    }

    /// 保存到文件
    pub fn save_to_file(&self, content: &str, path: &PathBuf) -> std::io::Result<()> {
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    /// 从三个 dump 文件生成对比表格并保存
    pub fn generate_from_dumps(
        &self,
        function_name: &str,
        dump_prefix: &str,
        output_dir: Option<&PathBuf>,
    ) -> anyhow::Result<()> {
        use crate::objdump::ObjdumpParser;
        
        // 智能处理前缀：如果包含 .dump 后缀，先去掉
        let clean_prefix = dump_prefix
            .strip_suffix(".dump").unwrap_or(dump_prefix)
            .trim_end_matches("_O0")
            .trim_end_matches("_O1")
            .trim_end_matches("_O2");
        
        // 加载三个 dump 文件
        let o0_path = format!("{}_O0.dump", clean_prefix);
        let o1_path = format!("{}_O1.dump", clean_prefix);
        let o2_path = format!("{}_O2.dump", clean_prefix);
        
        println!("读取 {} ...", o0_path);
        let o0_parser = ObjdumpParser::from_file(&o0_path)?;
        let o0_entries = o0_parser.extract_function_data(function_name)?;
        
        println!("读取 {} ...", o1_path);
        let o1_parser = ObjdumpParser::from_file(&o1_path)?;
        let o1_entries = o1_parser.extract_function_data(function_name)?;
        
        println!("读取 {} ...", o2_path);
        let o2_parser = ObjdumpParser::from_file(&o2_path)?;
        let o2_entries = o2_parser.extract_function_data(function_name)?;
        
        // 生成表格
        println!("生成对比表格...");
        let table = self.generate_comparison_table(&o0_entries, &o1_entries, &o2_entries);
        
        // 保存到文件
        let output_path = if let Some(dir) = output_dir {
            dir.join(format!("{}_comparison.md", function_name))
        } else {
            PathBuf::from(format!("{}_comparison.md", function_name))
        };
        
        println!("保存到 {} ...", output_path.display());
        self.save_to_file(&table, &output_path)?;
        
        println!("完成！");
        Ok(())
    }

    /// 从单个 dump 文件生成函数分析表格
    pub fn generate_from_single_dump(
        &self,
        function_name: &str,
        dump_path: &str,
        output_dir: Option<&PathBuf>,
    ) -> anyhow::Result<()> {
        use crate::objdump::ObjdumpParser;
        
        println!("读取 {} ...", dump_path);
        let parser = ObjdumpParser::from_file(dump_path)?;
        let entries = parser.extract_function_data(function_name)?;
        
        // 生成表格
        println!("生成分析表格...");
        let table = self.generate_table(&entries);
        
        // 保存到文件
        let output_path = if let Some(dir) = output_dir {
            dir.join(format!("{}_analysis.md", function_name))
        } else {
            PathBuf::from(format!("{}_analysis.md", function_name))
        };
        
        println!("保存到 {} ...", output_path.display());
        self.save_to_file(&table, &output_path)?;
        
        println!("完成！");
        Ok(())
    }
}

impl Default for TableGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::{Instruction, InstructionType, Operand};
    use crate::register::Register;

    #[test]
    fn test_generate_table() {
        let generator = TableGenerator::new();
        
        let entries = vec![
            DumpEntry {
                c_line: Some(1),
                c_code: String::from("int a = 0;"),
                address: String::from("0x1000"),
                machine_code: String::from("d2800000"),
                asm_instruction: String::from("mov x0, #0"),
                parsed_instruction: Some(Instruction::new(
                    InstructionType::MOV,
                    vec![
                        Operand::Register(Register::X0),
                        Operand::Immediate(0),
                    ],
                    0x1000,
                )),
            },
        ];
        
        let table = generator.generate_table(&entries);
        assert!(table.contains("C代码"));
        assert!(table.contains("语义解释"));
        assert!(table.contains("mov x0, #0"));
    }
}
