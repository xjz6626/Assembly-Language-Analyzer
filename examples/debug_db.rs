use alaz::instruction_db::InstructionDatabase;

fn main() {
    let db = InstructionDatabase::load_embedded().unwrap();
    let map = db.build_instruction_map();
    
    println!("Total instructions loaded: {}", map.len());
    println!("\nLooking for 'ldadd':");
    
    if let Some(inst) = map.get("ldadd") {
        println!("  ✅ Found: {} - {}", inst.name, inst.description);
    } else {
        println!("  ❌ Not found");
        println!("\nAll atomic-related instructions:");
        for (key, val) in map.iter() {
            if key.contains("ld") || key.contains("cas") || key.contains("swp") {
                println!("    {}: {}", key, val.name);
            }
        }
    }
}
