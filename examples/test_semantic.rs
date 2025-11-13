use alaz::instruction::{Instruction, InstructionType, Operand};
use alaz::semantic::SemanticInterpreter;
use alaz::register::Register;

fn main() {
    println!("测试完整的语义解释流程\n");
    
    // 创建测试指令
    let test_instructions = vec![
        ("LDADD", InstructionType::LDADD),
        ("LDADDAL", InstructionType::LDADDAL),
        ("CASAL", InstructionType::CASAL),
        ("SWP", InstructionType::SWP),
        ("AESE", InstructionType::AESE),
        ("ADD", InstructionType::ADD),  // 对比：旧指令
    ];
    
    for (name, inst_type) in test_instructions {
        let instruction = Instruction {
            address: 0x1000,
            instruction_type: inst_type,
            operands: vec![
                Operand::Register(Register::X0),
                Operand::Register(Register::X1),
                Operand::Register(Register::X2),
            ],
            encoding: None,
            condition: None,
        };
        
        let interpretation = SemanticInterpreter::interpret(&instruction);
        
        println!("{:12} => {}", name, interpretation);
    }
}
