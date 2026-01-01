use owo_colors::OwoColorize;

pub fn display_contents() {
  chapter_nine_title();
  uewp_content();
}

fn chapter_nine_title() {
    let title = "Error Handling";
    println!("{}", title.bright_blue().bold());
    println!("{} \n", "-".repeat(title.len()).bright_blue());
}
//  Introduction content below.
fn uewp_content() {
  let solid_disc = "\u{2022}";

  println!(
    "Errors are a fact of life in software, so Rust has a number of features for handling situations in which something goes wrong. \n\
    In many cases, Rust requires you to acknowledge the possibility of an error and take some action before your code will compile. \n\
    This requirement makes your program more robust by ensuring that you’ll discover errors and handle them appropriately before \
    deploying your code to production!
  ");
  
  println!(
    "Rust groups errors into two major categories: \n\
    {solid_disc} recoverable \n\
    {solid_disc} unrecoverable errors. \n\n\
    For a {0}, such as a {1}, we most likely just want to report the problem to the user and retry \
    the operation. \n\
    {2} are always symptoms of bugs, such as trying to access a location beyond \
    the end of an array, and so we want to immediately stop the program.
  ",
    "recoverable error".italic(),
    "file not found error".italic(),
    "Unrecoverable errors".italic() 
  );

  println!(
    "Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions. \n\
    Rust doesn’t have exceptions. Instead, it has the type {0} for recoverable errors and the {1} macro that stops execution \
    when the program encounters an unrecoverable error. \n\
    This chapter covers calling {1} first and then talks about returning {0} values. \n\
    Additionally, we’ll explore considerations when deciding whether to try to recover from an error or to stop execution.
  ",
    "Result<T, E>".bright_yellow().bold(),
    "panic!".bright_yellow().bold()
  );

}