// cli.rs の内容
pub fn some_function() {
    println!("This is a function in the cli module");
}
// src/cli.rs または src/cli/mod.rs
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // 関数の実装
    println!("Running the CLI");
    Ok(())
}


