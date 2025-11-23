use owo_colors::OwoColorize;

fn main() {
    let title = "Modules";
    // bold, bright effect using style chaining
    println!("{}", title.bold().bright_blue());
    println!("{}", "-".repeat(title.len()).bright_blue());
    println!("This is some body text.\n");

    // call the library crate's module function (crate name from Cargo.toml is `rust-topics`, which maps to `rust_topics`)
    rust_topics::module_topics::module_content();
}