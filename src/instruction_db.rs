//! 指令数据库模块 - 从 JSON 加载指令定义
//! 
//! 这个模块负责从 aarch64_instructions.json 加载指令定义，
//! 实现了完全解耦的设计，添加新指令只需修改 JSON 文件

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::{Result, Context};

/// 指令定义（来自 JSON）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionDef {
    pub mnemonic: String,
    pub name: String,
    pub format: String,
    pub description: String,
    #[serde(default)]
    pub flags_affected: Vec<String>,
    pub example: String,
}

/// 指令集数据库
#[derive(Debug, Serialize, Deserialize)]
pub struct InstructionDatabase {
    pub instruction_set: String,
    #[serde(default)]
    pub categories: HashMap<String, serde_json::Value>,
    #[serde(flatten)]
    pub extra_categories: HashMap<String, serde_json::Value>,
}

impl InstructionDatabase {
    /// 从嵌入的 JSON 文件加载指令数据库
    pub fn load_embedded() -> Result<Self> {
        const JSON_DATA: &str = include_str!("../aarch64_instructions.json");
        let db: InstructionDatabase = serde_json::from_str(JSON_DATA)
            .context("Failed to parse aarch64_instructions.json")?;
        Ok(db)
    }

    /// 从文件加载指令数据库
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .context(format!("Failed to read {}", path))?;
        let db: InstructionDatabase = serde_json::from_str(&content)
            .context("Failed to parse instruction database")?;
        Ok(db)
    }

    /// 构建指令助记符到定义的映射表
    pub fn build_instruction_map(&self) -> HashMap<String, InstructionDef> {
        let mut map = HashMap::new();
        
        // 遍历 categories 中的所有类别
        for category_value in self.categories.values() {
            self.extract_instructions_recursive(category_value, &mut map);
        }
        
        // 遍历额外类别（atomic_operations, cryptographic 等）
        for category_value in self.extra_categories.values() {
            self.extract_instructions_recursive(category_value, &mut map);
        }
        
        map
    }

    /// 递归提取指令定义
    fn extract_instructions_recursive(
        &self,
        value: &serde_json::Value,
        map: &mut HashMap<String, InstructionDef>,
    ) {
        match value {
            serde_json::Value::Array(arr) => {
                for item in arr {
                    if let Ok(inst) = serde_json::from_value::<InstructionDef>(item.clone()) {
                        map.insert(inst.mnemonic.to_lowercase(), inst);
                    }
                }
            }
            serde_json::Value::Object(obj) => {
                for (_key, val) in obj {
                    self.extract_instructions_recursive(val, map);
                }
            }
            _ => {}
        }
    }

    /// 根据助记符查找指令定义
    pub fn find_instruction(&self, mnemonic: &str) -> Option<InstructionDef> {
        let map = self.build_instruction_map();
        map.get(&mnemonic.to_lowercase()).cloned()
    }

    /// 获取所有指令助记符列表
    pub fn get_all_mnemonics(&self) -> Vec<String> {
        let map = self.build_instruction_map();
        let mut mnemonics: Vec<String> = map.keys().cloned().collect();
        mnemonics.sort();
        mnemonics
    }

    /// 获取指令数量统计
    pub fn get_instruction_count(&self) -> usize {
        self.build_instruction_map().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_embedded_database() {
        let db = InstructionDatabase::load_embedded();
        assert!(db.is_ok());
        let db = db.unwrap();
        assert_eq!(db.instruction_set, "AArch64 (ARM 64-bit)");
    }

    #[test]
    fn test_build_instruction_map() {
        let db = InstructionDatabase::load_embedded().unwrap();
        let map = db.build_instruction_map();
        
        // 应该包含基本指令
        assert!(map.contains_key("add"));
        assert!(map.contains_key("sub"));
        assert!(map.contains_key("mov"));
    }

    #[test]
    fn test_find_instruction() {
        let db = InstructionDatabase::load_embedded().unwrap();
        
        // 测试查找基本指令（唯一的）
        let madd_inst = db.find_instruction("madd");
        assert!(madd_inst.is_some());
        let madd_inst = madd_inst.unwrap();
        assert_eq!(madd_inst.name, "Multiply-Add");
        
        // 测试不区分大小写
        let sdiv_upper = db.find_instruction("SDIV");
        assert!(sdiv_upper.is_some());
        assert_eq!(sdiv_upper.unwrap().name, "Signed Divide");
        
        // 测试浮点指令
        let fadd_inst = db.find_instruction("fadd");
        assert!(fadd_inst.is_some());
        assert_eq!(fadd_inst.unwrap().name, "Floating-point Add");
        
        // 测试 SIMD 指令
        let ld1_inst = db.find_instruction("ld1");
        assert!(ld1_inst.is_some());
        
        // 测试原子操作指令
        let ldadd_inst = db.find_instruction("ldadd");
        assert!(ldadd_inst.is_some(), "ldadd should be found in database");
        assert_eq!(ldadd_inst.unwrap().name, "Atomic Add");
    }

    #[test]
    fn test_instruction_count() {
        let db = InstructionDatabase::load_embedded().unwrap();
        let count = db.get_instruction_count();
        
        // 应该有很多指令（至少50+）
        assert!(count > 50, "Expected at least 50 instructions, got {}", count);
    }
}
