use owo_colors::OwoColorize;

pub fn display_contents() {
  uewp_content();
}


// Subheaders content below.

// Subheader: Unrecoverable Errors with panic!. Abbreviated as uewp.
fn uewp_content() {
  let solid_disc = "\u{2022}";

  // Subheader title.
  println!("{} \n", "Unrecoverable Errors with panic!".bright_blue().bold());

  // Subheader content.
  println!(
    "Sometimes bad things happen in your code, and there’s nothing you can do about it. \
    In these cases, Rust has the {0} macro. \n\n\
    There are two ways to cause a panic in practice: \n\
    {solid_disc} By taking an action that causes our code to panic \
    (such as accessing an array past the end). \n\
    {solid_disc} By explicitly calling the {0} macro. \n\n\
    In both cases, we cause a panic in our program. \n\
    By default, these panics will print a failure message, unwind, clean up the stack, \
    and quit. \n\
    Via an environment variable, you can also have Rust display the call stack when \
    a panic occurs to make it easier to track down the source of the panic. \n\
  ",
    "panic!" .bright_yellow().bold(),
  );

  println!(
    "See {}, about unwinding the stack or aborting in response to a panic.
    ",
    "https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic".cyan()
  );

  println!(
    "This how you use {0} in your code: \n\n\
    {1} \n\n\
    See: {2}, for the error message.
  ",
    "panic!".bright_yellow().bold(),
    "fn main() {\n   panic!(\"crash and burn\"); \n}".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unrecoverable-errors-with-panic".cyan()
  );

  println!(
    "In the output above, the first line shows the panic message we specified: \
    {0}, and the place in our code where the panic occurred: {1} \
    indicates that it’s the second line, fifth character of our {2} file \
    or (line 2, column 5). \n\
  ",
    "crash and burn".bright_red().bold(),
    "src/main.rs:2:5".italic(),
    "src/main.rs".italic()
  );

  println!(
    "In this case, the line indicated is part of our code, and if we go to that line, \
    we see the {0} macro call. \n\
    In other cases, the {0} call might be in code that our code calls, and the \
    filename and line number reported by the error message will be someone else’s code \
    where the {0} macro is called, not the line of our code that eventually led to \
    the {0} call.
  ",
    "panic!".bright_yellow().bold()
  );

  println!(
    "{} \n\n\
    We can use the backtrace of the functions the {1} call came from to figure out \
    the part of our code that is causing the problem. \n\
    To understand how to use a {1} backtrace, let’s look at another example and \
    see what it’s like when a {1} call comes from a library because of a bug in \
    our code instead of from our code calling the macro directly. \n\n\
    See: {2}, for code sample and more information about backtraces. \n\n\
    A {3} is a list of all the functions that have been called to get to this point. \n\
    Backtraces in Rust work as they do in other languages: {4}. \n\
    That’s the spot where the problem originated. \n\
    The lines above that spot are code that your code has called; the lines below are \
    code that called your code. \n\
    These before-and-after lines might include core Rust code, standard library code, \
    or crates that you’re using.
  ",
    "BACKTRACE".bright_magenta().bold(),
    "panic!" .bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#listing-9-1".cyan(),
    "backtrace".italic(),
    "The key to reading the backtrace is to start from the top and read until you see \
    files you wrote".bright_white().bold()
  );
}







