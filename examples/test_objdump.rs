use alaz::objdump::ObjdumpParser;

fn main() {
    println!("测试 ObjdumpParser 解析 dump 文件\n");
    
    let test_content = r#"
advanced_test_O0.o:     file format elf64-littleaarch64

Disassembly of section .text:

0000000000000000 <atomic_operations>:
// 原子操作测试
   0:   d503201f        nop
   4:   f9400000        ldr     x0, [x0]
   8:   b8210c01        ldadd   w1, w1, [x0]
   c:   b8a10c01        ldaddal w1, w1, [x0]
  10:   b8211c01        ldclr   w1, w1, [x0]
  14:   b8212c01        ldeor   w1, w1, [x0]
  18:   b8213c01        ldset   w1, w1, [x0]
  1c:   b8208020        swp     w0, w0, [x1]
  20:   c8a17c22        casal   x1, x2, [x1]
  24:   d65f03c0        ret
"#;
    
    let parser = ObjdumpParser::new(test_content.to_string());
    
    match parser.extract_function_data("atomic_operations") {
        Ok(entries) => {
            println!("成功提取 {} 条记录\n", entries.len());
            
            for entry in &entries {
                println!("汇编指令: {}", entry.asm_instruction);
                if let Some(ref inst) = entry.parsed_instruction {
                    println!("  解析结果: {:?}", inst.instruction_type);
                } else {
                    println!("  ❌ 解析失败 (parsed_instruction 是 None)");
                }
                println!();
            }
        }
        Err(e) => {
            println!("❌ 提取失败: {}", e);
        }
    }
}
