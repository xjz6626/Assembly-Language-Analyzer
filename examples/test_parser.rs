use alaz::parser::AssemblyParser;

fn main() {
    println!("测试 Parser 解析新指令\n");
    
    let test_instructions = vec![
        "ldadd      w1, w1, [x0]",
        "ldaddal    w1, w1, [x0]",
        "ldclr      w1, w1, [x0]",
        "ldeor      w1, w1, [x0]",
        "ldset      w1, w1, [x0]",
        "swp        w0, w0, [x1]",
        "casal      x1, x2, [x1]",
    ];
    
    for asm_str in test_instructions {
        println!("原始: {}", asm_str);
        
        let mut parser = AssemblyParser::new();
        match parser.parse(asm_str) {
            Ok(instructions) if !instructions.is_empty() => {
                let inst = &instructions[0];
                println!("  ✅ 解析成功:");
                println!("    指令类型: {:?}", inst.instruction_type);
                println!("    操作数: {:?}", inst.operands);
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
