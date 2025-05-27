// Visualization server binary
// This binary starts the web server for the visualization interface

use viz_lib::server;

#[tokio::main]
async fn main() {
    println!("🚀 Starting Visualization Server...");
    println!("📊 Access the visualization interface at: http://localhost:3031");
    println!("💡 Run examples to generate trace files, then view them in the browser");
    println!();
    
    server::serve().await;
}
