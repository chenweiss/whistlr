fn main() {
    if std::env::args().any(|a| a == "--version") {
        println!("whistlr {}", env!("CARGO_PKG_VERSION"));
    } else {
        println!("usage: whistlr [--version]");
    }
}
