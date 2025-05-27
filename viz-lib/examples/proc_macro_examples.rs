// filepath: c:\Projects\advent-of-code-rs\viz-lib\examples\proc_macro_examples.rs
// Runner for procedural macro examples

use viz_lib::examples::proc_macro::proc_macro_demo;

fn main() {
    println!("🤖 Procedural Macro Examples Demo");
    println!("==================================");
    
    // Run proc macro demo
    proc_macro_demo();
    
    println!();
    println!("🎉 Procedural macro examples completed!");
    println!("📁 Check the traces/ directory for generated visualization files");
    println!("🌐 View them at: http://localhost:3031 (start server with: cargo run --bin viz-server --features server)");
}
