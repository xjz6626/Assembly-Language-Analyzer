//! objdump 文件解析器
//! 
//! 解析 objdump 格式的汇编文件，提取函数、C代码和汇编指令

use crate::instruction::Instruction;
use crate::error::{Result, InterpreterError};
use std::collections::HashMap;
use regex::Regex;

/// objdump 文件中的一条记录
#[derive(Debug, Clone)]
pub struct DumpEntry {
    /// C 源代码行号
    pub c_line: Option<usize>,
    /// C 源代码
    pub c_code: String,
    /// 汇编指令地址
    pub address: String,
    /// 机器码
    pub machine_code: String,
    /// 汇编指令
    pub asm_instruction: String,
    /// 解析后的指令结构
    pub parsed_instruction: Option<Instruction>,
}

/// objdump 文件解析器
pub struct ObjdumpParser {
    /// 行数据
    lines: Vec<String>,
}

impl ObjdumpParser {
    /// 创建新的解析器
    pub fn new(content: String) -> Self {
        let lines = content.lines().map(|s| s.to_string()).collect();
        Self { lines }
    }

    /// 从文件加载
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(Self::new(content))
    }

    /// 查找函数的起始和结束行
    pub fn find_function(&self, func_name: &str) -> Option<(usize, usize)> {
        let func_pattern = Regex::new(&format!(r"^[0-9a-f]+\s+<{}>:", regex::escape(func_name)))
            .ok()?;

        let mut start_line = None;

        // 查找函数开始
        for (i, line) in self.lines.iter().enumerate() {
            if func_pattern.is_match(line) {
                start_line = Some(i);
                break;
            }
        }

        let start_line = start_line?;

        // 查找函数结束
        let next_func_pattern = Regex::new(r"^[0-9a-f]+\s+<\w+>:").ok()?;
        let section_pattern = Regex::new(r"^Disassembly of section").ok()?;

        for i in (start_line + 1)..self.lines.len() {
            if next_func_pattern.is_match(&self.lines[i]) 
                || section_pattern.is_match(&self.lines[i]) {
                return Some((start_line, i - 1));
            }
        }

        Some((start_line, self.lines.len() - 1))
    }

    /// 列出所有函数名称
    pub fn list_functions(&self) -> Result<Vec<String>> {
        let func_pattern = Regex::new(r"^[0-9a-f]+\s+<([^>]+)>:")
            .map_err(|e| InterpreterError::ParseError(format!("正则表达式错误: {}", e)))?;
        
        let mut functions = Vec::new();
        
        for line in &self.lines {
            if let Some(caps) = func_pattern.captures(line) {
                let func_name = caps.get(1).unwrap().as_str().to_string();
                functions.push(func_name);
            }
        }
        
        Ok(functions)
    }

    /// 提取函数的汇编数据
    pub fn extract_function_data(&self, func_name: &str) -> Result<Vec<DumpEntry>> {
        let (start, end) = self.find_function(func_name)
            .ok_or_else(|| InterpreterError::ParseError(
                format!("未找到函数: {}", func_name)
            ))?;

        let asm_pattern = Regex::new(r"^\s*([0-9a-f]+):\s+([0-9a-f]+)\s+(.+)$")
            .map_err(|e| InterpreterError::ParseError(format!("正则表达式错误: {}", e)))?;
        
        // 检测是否有内联函数调用
        let inline_pattern = Regex::new(r"<([^>]+\.part\.\d+)>")
            .map_err(|e| InterpreterError::ParseError(format!("正则表达式错误: {}", e)))?;
        
        let mut has_inline = None;
        for i in (start + 1)..=end {
            if let Some(caps) = inline_pattern.captures(&self.lines[i]) {
                has_inline = Some(caps.get(1).unwrap().as_str().to_string());
                break;
            }
        }
        
        let source_pattern = Regex::new(r"^/.*:\d+")
            .map_err(|e| InterpreterError::ParseError(format!("正则表达式错误: {}", e)))?;

        // 第一步：收集所有 C 代码行
        let mut c_code_map: HashMap<usize, String> = HashMap::new();
        let mut first_asm_line = None;

        for i in (start + 1)..=end {
            let line = &self.lines[i];

            if asm_pattern.is_match(line) {
                if first_asm_line.is_none() {
                    first_asm_line = Some(i);
                }
                continue;
            }

            let cleaned = line.trim();
            if cleaned.is_empty() 
                || cleaned.starts_with("Disassembly") 
                || cleaned.starts_with("objdump")
                || cleaned.starts_with("file format") 
                || source_pattern.is_match(cleaned) {
                continue;
            }

            // 过滤掉单独的括号和预处理指令
            if cleaned == "{" || cleaned == "}" 
                || cleaned.starts_with("#endif")
                || cleaned.starts_with("#ifdef")
                || cleaned.starts_with("#else")
                || cleaned.starts_with("ERROR:") {
                continue;
            }

            c_code_map.insert(i, cleaned.to_string());
        }

        // 合并函数签名
        let mut c_code_list = Vec::new();
        if let Some(first_asm) = first_asm_line {
            let mut prologue = Vec::new();
            let mut prologue_idx = 0;

            for i in (start + 1)..first_asm {
                if let Some(c_code) = c_code_map.get(&i) {
                    prologue.push(c_code.clone());
                    prologue_idx = i;
                }
            }

            if !prologue.is_empty() {
                let combined = prologue.join(" <br> ");
                c_code_list.push((prologue_idx, combined));
            }

            // 添加其他 C 代码
            for i in first_asm..=end {
                if let Some(c_code) = c_code_map.get(&i) {
                    c_code_list.push((i, c_code.clone()));
                }
            }
        }

        // 第二步：提取汇编指令并关联 C 代码
        let mut entries = Vec::new();
        let mut current_c_code = String::new();
        let mut current_c_line = None;

        for i in (start + 1)..=end {
            let line = &self.lines[i];

            // 更新当前 C 代码
            for (c_idx, c_code) in c_code_list.iter() {
                if *c_idx == i {
                    current_c_code = c_code.clone();
                    current_c_line = Some(*c_idx);
                    break;
                }
            }

            if let Some(caps) = asm_pattern.captures(line) {
                let address = caps.get(1).unwrap().as_str().to_string();
                let machine_code = caps.get(2).unwrap().as_str().to_string();
                let asm_instruction = caps.get(3).unwrap().as_str().trim().to_string();

                // 尝试解析汇编指令
                let parsed_instruction = Self::parse_instruction(&asm_instruction);

                entries.push(DumpEntry {
                    c_line: current_c_line,
                    c_code: current_c_code.clone(),
                    address,
                    machine_code,
                    asm_instruction,
                    parsed_instruction,
                });
            }
        }
        
        // 如果检测到内联函数，添加提示信息
        if let Some(inline_func) = has_inline {
            if !entries.is_empty() {
                entries.push(DumpEntry {
                    c_line: None,
                    c_code: format!("⚠️ 注意：主要逻辑已被编译器优化，实际代码在编译器生成的内部函数 <{}> 中执行", inline_func),
                    address: String::new(),
                    machine_code: String::new(),
                    asm_instruction: String::new(),
                    parsed_instruction: None,
                });
            }
        }

        Ok(entries)
    }

    /// 解析单条汇编指令
    fn parse_instruction(asm_str: &str) -> Option<Instruction> {
        use crate::parser::AssemblyParser;
        
        // 尝试解析指令
        let mut parser = AssemblyParser::new();
        match parser.parse(asm_str) {
            Ok(instructions) if !instructions.is_empty() => Some(instructions[0].clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_objdump_parser() {
        let content = r#"
0000000000000000 <test_func>:
   0:   d100c3ff    sub sp, sp, #0x30
   4:   f90007e0    str x0, [sp, #8]
"#;
        let parser = ObjdumpParser::new(content.to_string());
        let result = parser.find_function("test_func");
        assert!(result.is_some());
    }
}
