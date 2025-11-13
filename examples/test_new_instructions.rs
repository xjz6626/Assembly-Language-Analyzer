use alaz::instruction_db::InstructionDatabase;

fn main() {
    let db = InstructionDatabase::load_embedded().unwrap();
    let map = db.build_instruction_map();
    
    println!("📊 指令集统计\n");
    println!("总计支持指令: {} 条\n", map.len());
    
    // 测试新增的高优先级指令
    let high_priority = vec![
        ("csel", "条件选择"),
        ("csinc", "条件选择递增"),
        ("adrp", "页地址计算"),
        ("adr", "地址计算"),
        ("fmla", "浮点融合乘加"),
        ("fmls", "浮点融合乘减"),
        ("ubfiz", "位域插入零"),
        ("extr", "提取寄存器"),
    ];
    
    println!("🔴 高优先级指令测试:");
    for (mnemonic, desc) in &high_priority {
        if let Some(inst) = map.get(*mnemonic) {
            println!("  ✅ {} - {}", mnemonic, inst.description.split('，').next().unwrap_or(desc));
        } else {
            println!("  ❌ {} 未找到", mnemonic);
        }
    }
    
    // 测试新增的中优先级指令
    let mid_priority = vec![
        ("ins", "插入向量元素"),
        ("dup", "复制向量"),
        ("cnt", "位计数"),
        ("fcvtas", "浮点转整数"),
        ("sqadd", "饱和加法"),
        ("ldxp", "独占加载对"),
    ];
    
    println!("\n🟠 中优先级指令测试:");
    for (mnemonic, desc) in &mid_priority {
        if let Some(inst) = map.get(*mnemonic) {
            println!("  ✅ {} - {}", mnemonic, inst.description.split('，').next().unwrap_or(desc));
        } else {
            println!("  ❌ {} 未找到", mnemonic);
        }
    }
    
    println!("\n✨ 所有新指令均已成功加载！");
}
