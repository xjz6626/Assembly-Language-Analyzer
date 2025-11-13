use alaz::instruction::InstructionType;

fn main() {
    let test_types = vec![
        InstructionType::LDADD,
        InstructionType::LDADDAL,
        InstructionType::CASAL,
        InstructionType::SWP,
        InstructionType::AESE,
    ];
    
    println!("测试 InstructionType 的 Debug 格式化\n");
    
    for inst_type in test_types {
        let debug_str = format!("{:?}", inst_type);
        let lowercase = debug_str.to_lowercase();
        println!("InstructionType::{:?}", inst_type);
        println!("  Debug 格式: '{}'", debug_str);
        println!("  小写格式: '{}'", lowercase);
        println!();
    }
}
