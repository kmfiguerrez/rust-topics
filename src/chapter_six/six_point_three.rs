use owo_colors::OwoColorize;
use crate::{chapter, menu};

pub fn content(section_title: &str) {
  let subheaders: [chapter::SubHeader; 3];
  subheaders = [
    chapter::SubHeader::new("Concise Control flow with if let and let else", ccfwilale_content),
    chapter::SubHeader::new("Staying on the \"Happy Path\" with let...else", le_content),
    chapter::SubHeader::new("The Option<T> match Pattern", summary_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title);
}

// Subheader contents below.

// Subheader: Concise Control flow with if let and let else. Abbreviated as ccfwilale.
fn ccfwilale_content() {
  // Subheader title.
  menu::subheader_title("Concise Control flow with if let and let else");

  println!(
    "The {0} syntax lets you combine {1} and {2} into a less verbose way to handle \
    values that match one pattern while ignoring the rest. \n\
    Consider the program in Listing 6-6: {3}, that matches on an {4} value in \
    the {5} variable but only wants to execute code if the value is the {6} \
    variant.
  ",
    "if let".bright_yellow().bold(),
    "if".bright_yellow().bold(),
    "let".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch06-03-if-let.html#listing-6-6".cyan(),
    "Option<i32>".bright_yellow().bold(),
    "value".bright_yellow().bold(),
    "Some".bright_yellow().bold()
  );

  println!(
    "If the value is {0}, we print out the value in the {0} variant by binding the \
    value to the variable {1} in the pattern. \n\
    We don’t want to do anything with the {2} value. \n\
    To satisfy the {3} expression, we have to add {4} after processing just one \
    variant, which is annoying boilerplate code to add. \n\n\
    Instead, we could write this in a shorter way using {5}. \n\
    The following code behaves the same as the {3} in Listing 6-6: \n\n\
    {6} \n\
    The syntax {5} takes a pattern and an expression separated by an equal sign. \n\
    It works the same way as a {3}, where the expression is given to the {3} \
    and the pattern is its first arm. \n\
    In this case, the pattern is {7}, and the {1} binds to the value inside \
    the {0}. \n\
    We can then use {1} in the body of the {5} block in the same way we used {1} in \
    the corresponding {3} arm. \n\
    The code in the {5} block only runs if the value matches the pattern.
  ",
    "Some".bright_yellow().bold(),
    "max".bright_yellow().bold(),
    "None".bright_yellow().bold(),
    "match".bright_yellow().bold(),
    "_ => ()".bright_yellow().bold(),
    "if let".bright_yellow().bold(),
    "let config_max = Some(3u8); \n\
    if let Some(max) = value {\n    println!(\"The maximum is {max}\");\n}
    ".bright_yellow().bold(),
    "Some(max)".bright_yellow().bold()
  );

  println!(
    "{} \n\n\
    Using {1} means less typing, less indentation, and less boilerplate code. \n\
    However, you lose the exhaustive checking {2} enforces that ensures that you \
    aren’t forgetting to handle any cases. \n\
    Choosing between {2} and {1} depends on what you’re doing in your particular \
    situation and whether gaining conciseness is an appropriate trade-off for losing \
    exhaustive checking. \n\n\
    In other words, you can think of {1} as syntax sugar for a {2} that runs code \
    when the value matches one pattern and then ignores all other values.
  ",
    "MATCH VS IF LET".bright_magenta().bold(),
    "if let".bright_yellow().bold(),
    "match".bright_yellow().bold()
  );

  println!(
    "{} \n\n\
    We can include an {1} with an {2}. \n\
    The block of code that goes with the {1} is the same as the block of code that \
    would go with the {3} case in the {4} expression that is equivalent to the \
    {2} and {1}. \n\
    Here is an example: \n\n\
    {5} \n\n\
    See: {6} for other examples and details. \n\n\
    The common pattern is to perform some computation when a value is present and return a default value otherwise.
  ",
    "ELSE WITH IF LET".bright_magenta().bold(),
    "else".bright_yellow().bold(),
    "if let".bright_yellow().bold(),
    "_".bright_yellow().bold(),
    "match".bright_yellow().bold(),
    "let config_max = Some(3u8); \n\
    if let Some(max) = config_max {\n   println!(\"The maximum is configured to be {max}\");\n\
    } else {\n  println!(\"The maximum is not configured.\");\n\
    }".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch06-03-if-let.html#else-with-if-let".cyan()
  );
}

// Subheader: Staying on the "Happy Path" with let...else. Abbreviated as le.
fn le_content() {
  let solid_disc = "\u{2022}";

  // Subheader title.
  menu::subheader_title("Staying on the \"Happy Path\" with let...else");

  println!(
    "The {0} syntax provides a way to handle unexpected values early and keep \
    the rest of the code at the main (top) indentation level, which is sometimes called \
    the \"happy path.\" \n\
    So happy path means avoiding nested code and keeping the main flow of the code \
    unindented as much as possible. \n\n\
    Its primary purpose is to {1} if the pattern doesn't match. \n\
    This approach can make your code cleaner and easier to read by reducing \
    nesting and highlighting the primary flow of execution.    
  ",
    "let...else".bright_yellow().bold(),
    "unwrap a value from a pattern or exit the current scope (or other divergence) \
    early".bright_white().bold()
  );

  println!(
    "{0} \n\n\
    {1} \n\n\
    The {2} syntax takes a pattern on the left side and an expression on \
    the right, very similar to {3}, but it does not have an {4} branch, only an \
    {5} branch.
  ",
    "SYNTAX".bright_magenta().bold(),
    "let PATTERN = EXPRESSION else {\n   DIVERGING_BLOCK // e.g., return, break, continue, panic!()\n\
    };".bright_yellow().bold(),
    "let...else".bright_yellow().bold(),
    "if let".bright_yellow().bold(),
    "if" .bright_yellow().bold(),
    "else".bright_yellow().bold()
  );

  println!(
    "{} \n\n\
    1. {1}: Rust tries to match the EXPRESSION against the PATTERN. \n\
    2. {2}: if EXPRESSION matches PATTERN, the variables defined in the pattern are bound to the \
    {3} (not inside a block), and the code continues. \n\
    3. {4}: if the PATTERN does not match, the {5} block is executed. This block {6} \
    (i.e., it must return, break, continue, or panic!).
  ",
    "BEHAVIOR".bright_magenta().bold(),
    "Attempt to match" .bright_white().bold(),
    "Successful match" .bright_white().bold(),
    "current scope" .bright_white().bold(),
    "Failed match" .bright_white().bold(),
    "else".bright_yellow().bold(),
    "must diverge".bright_white().bold()
  );

  println!(
    "See code samples in rust book: {0} for other examples and details.
  ",
    "https://doc.rust-lang.org/book/ch06-03-if-let.html#staying-on-the-happy-path-with-letelse".cyan()
  );

  println!(
    "{} \n\n\
    {solid_disc} {1} You want to validate input and exit immediately (or other divergence) if it is invalid. \n\
    {solid_disc} {2} You are dealing with {3} or {4} types and only care about the success value for the \
    rest of the function. \n\
    {solid_disc} {5} You want to keep your code \"flat\" rather than nesting logic deep inside {6} blocks. \n\
    {solid_disc} {7} The compiler will enforce that the code inside the {8} block never finishes normally. \
    You cannot just print a message and let execution flow out of the {8} block; you must leave the \
    current flow (e.g via divergence). The else block must diverge (i.e., it must return, break, continue, or panic!).
  ",
    "WHEN TO USE LET...ELSE".bright_magenta().bold(),
    "Validation:".bright_white().bold(),
    "Unwrapping:".bright_white().bold(),
    "Option".bright_yellow().bold(),
    "Result".bright_yellow().bold(),
    "Reduced Nesting:".bright_white().bold(),
    "if let".bright_yellow().bold(),
    "Note on Divergence:".bright_white().bold(),
    "else".bright_yellow().bold()
  );

  println!(
    "{} \n\n\
    {solid_disc} The {} syntax doesn't want multiple control flow branches. It only has an {} branch for conciseness. \n\
  ",
    "REMEMBER".bright_white().bold(),
    "let...else".bright_yellow().bold(),
    "else".bright_yellow().bold()
  );

}

// Subheader: Summary.
fn summary_content() {
  let solid_disc = "\u{2022}";
  menu::subheader_title("Summary");

  println!(
    "{solid_disc} We use enums to create custom types that can be one of a set of enumerated values. \n\
    {solid_disc} We’ve shown how the standard library’s {0} type helps you use the type system to prevent errors. \n\
    {solid_disc} When enum values have data inside them, you can use {1} or {2} to extract and use those values, \
    depending on how many cases you need to handle. \n\
    {solid_disc} The {2} syntax is a concise way to handle values that match one pattern while ignoring the rest. \n\
    {solid_disc} We can use {3} with {2} to perform some computation when a particular value is present, \
    and return a default value or perform default action otherwise. \n\
    {solid_disc} The {4} syntax provides a way to handle unexpected values early and binds the value that a variant holds \
    in the outer scope (not inside a block) and keep the rest of the code at \
    the main (top) indentation level, which is sometimes called the \"happy path.\"
  ",
    "Option<T>".bright_yellow().bold(),
    "match".bright_yellow().bold(),
    "if let".bright_yellow().bold(),
    "else".bright_yellow().bold(),
    "let...else".bright_yellow().bold()
  );
}