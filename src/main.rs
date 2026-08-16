use clap::Parser;

#[derive(Parser)]
#[command(name = "TaiwanFRP Agent")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "TaiwanFRP 連接器", long_about = None)]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
    println!("Hello, world!");
}
