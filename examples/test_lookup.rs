use alaz::instruction_db::InstructionDatabase;

fn main() {
    let db = InstructionDatabase::load_embedded().unwrap();
    
    // 测试查找各种指令
    let test_mnemonics = ["ldadd", "ldaddal", "swp", "casal", "adrp", "fadd", "aese"];
    
    println!("测试指令查找功能\n");
    
    for mnemonic in test_mnemonics {
        print!("查找 '{}': ", mnemonic);
        
        if let Some(inst) = db.find_instruction(mnemonic) {
            println!("✅ 找到 - {} ({})", inst.name, inst.description);
        } else {
            println!("❌ 未找到");
        }
    }
    
    // 统计
    let map = db.build_instruction_map();
    println!("\n总计: {} 条指令", map.len());
}
