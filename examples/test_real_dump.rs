use alaz::objdump::ObjdumpParser;
use alaz::semantic::SemanticInterpreter;

fn main() {
    println!("测试实际 dump 文件的完整解析流程\n");
    
    let parser = ObjdumpParser::from_file("test_dumps/advanced_test_O0.dump")
        .expect("无法读取文件");
    
    let entries = parser.extract_function_data("atomic_operations")
        .expect("无法提取函数数据");
    
    println!("成功提取 {} 条记录\n", entries.len());
    
    for entry in &entries {
        if entry.asm_instruction.is_empty() {
            continue;
        }
        
        println!("C代码: {}", entry.c_code);
        println!("汇编: {}", entry.asm_instruction);
        
        if let Some(ref inst) = entry.parsed_instruction {
            println!("  ✅ 解析成功: {:?}", inst.instruction_type);
            let semantic = SemanticInterpreter::interpret(inst);
            println!("  📝 语义解释: {}", semantic);
        } else {
            println!("  ❌ 解析失败 (None)");
        }
        println!();
    }
}
