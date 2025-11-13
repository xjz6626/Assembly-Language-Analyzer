//! 展示指令数据库功能
//! 
//! 这个示例程序演示了如何使用基于 JSON 的指令数据库

use alaz::instruction_db::InstructionDatabase;

fn main() {
    println!("=== ALAZ 指令数据库演示 ===\n");
    
    // 加载指令数据库
    let db = InstructionDatabase::load_embedded()
        .expect("Failed to load instruction database");
    
    println!("指令集: {}", db.instruction_set);
    println!("总指令数: {}\n", db.get_instruction_count());
    
    // 展示一些指令
    println!("=== 基本指令示例 ===");
    show_instruction(&db, "add");
    show_instruction(&db, "sub");
    show_instruction(&db, "mul");
    
    println!("\n=== 浮点指令示例 ===");
    show_instruction(&db, "fadd");
    show_instruction(&db, "fmul");
    show_instruction(&db, "fsqrt");
    
    println!("\n=== SIMD指令示例 ===");
    show_instruction(&db, "ld1");
    show_instruction(&db, "st1");
    
    println!("\n=== 系统指令示例 ===");
    show_instruction(&db, "dmb");
    show_instruction(&db, "wfi");
    
    // 显示所有指令助记符
    println!("\n=== 所有支持的指令助记符 ===");
    let mnemonics = db.get_all_mnemonics();
    println!("共 {} 个指令:", mnemonics.len());
    for (i, mnemonic) in mnemonics.iter().enumerate() {
        print!("{:8}", mnemonic);
        if (i + 1) % 8 == 0 {
            println!();
        }
    }
    println!();
}

fn show_instruction(db: &InstructionDatabase, mnemonic: &str) {
    if let Some(inst) = db.find_instruction(mnemonic) {
        println!("\n[{}] {}", inst.mnemonic.to_uppercase(), inst.name);
        println!("  格式: {}", inst.format);
        println!("  描述: {}", inst.description);
        println!("  示例: {}", inst.example);
        if !inst.flags_affected.is_empty() {
            println!("  影响标志: {:?}", inst.flags_affected);
        }
    } else {
        println!("\n[{}] 未找到指令定义", mnemonic.to_uppercase());
    }
}
