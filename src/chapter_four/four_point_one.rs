use owo_colors::OwoColorize;

pub fn display_contents() {
  chapter_four_title();
  subheader_4_1_content();
}

fn chapter_four_title() {
  let title = "Understanding Ownership";
  println!("{}", title.bright_blue().bold());
  println!("{} \n", "-".repeat(title.len()).bright_blue());
}

// Subheaders content below.

// Subheader 4.1: What is Ownership?. Abbre
pub fn subheader_4_1_content() {
  let subheader_title = "4.1 What is Ownership?";
  println!("{} \n", subheader_title.bright_blue());

  println!("\
    Ownership is Rust's most unique feature and it enables Rust to make memory safety guarantees without \
    needing a garbage collector. \n\
    Understanding ownership is key to becoming proficient in Rust programming. {}.
  ",
  "So it’s important to understand how ownership works".bold()
  );

  // Here are the basic rules of \
  //   ownership:
  //   1. Each value in Rust has a variable that’s called its owner.
  //   2. There can only be one owner at a time.
  //   3. When the owner goes out of scope, the value will be dropped.

  println!(
    "Ownership is a set of rules that governs how a Rust program manages memory. \n\
    All programs have to manage the way they use a computer’s memory while running. \n\
    Some languages have a garbage collector that automatically looks for unused memory and frees it, \
    while other languages require the programmer to explicitly allocate and free memory. \n\
    Rust uses a third approach: memory is managed through a system of ownership with a set of rules that \
    the compiler checks at compile time. \n\
    If any of the rules are violated, the program won’t compile. \n\
    None of the features of ownership will slow down your program while it’s running.
  ");

}