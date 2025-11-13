use alaz::parser::AssemblyParser;
use alaz::semantic::SemanticInterpreter;

fn main() {
    println!("端到端测试：objdump 解析 → 语义解释\n");
    
    let test_instructions = vec![
        "ldadd      w1, w1, [x0]",
        "ldaddal    w1, w1, [x0]",
        "casal      x1, x2, [x1]",
        "swp        w0, w0, [x1]",
        "aese       v0.16b, v1.16b",
    ];
    
    for asm_str in test_instructions {
        println!("汇编指令: {}", asm_str);
        
        // 步骤 1: 解析
        let mut parser = AssemblyParser::new();
        match parser.parse(asm_str) {
            Ok(instructions) if !instructions.is_empty() => {
                let inst = &instructions[0];
                println!("  解析结果: {:?}", inst.instruction_type);
                
                // 步骤 2: 语义解释
                let interpretation = SemanticInterpreter::interpret(inst);
                println!("  语义解释: {}", interpretation);
            }
            Ok(_) => {
                println!("  ❌ 解析结果为空");
            }
            Err(e) => {
                println!("  ❌ 解析失败: {}", e);
            }
        }
        println!();
    }
}
