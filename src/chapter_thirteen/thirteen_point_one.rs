use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 6];
  subheaders = [
    chapter::SubHeader::new("Chapter Introduction", ci_content),
    chapter::SubHeader::new("Section Introduction: Closures", si_content),
    chapter::SubHeader::new("Capturing the Environment", cte_content),
    chapter::SubHeader::new("Inferring and Annotating Closure Types", iact_content),
    chapter::SubHeader::new("Capturing References or Moving Ownership", cromo_content),
    chapter::SubHeader::new("Moving Captured Values Out of Closures", mcvoc_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Chapter Introduction. Abbreviated as ci.
fn ci_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Chapter Introduction");

  println!(
  "Rust's design has taken inspiration from many existing languages and techniques, and one significant influence is \
  {0}.\n\
  Programming in a functional style often includes using functions as values by passing them in arguments, returning them \
  from other functions, assigning them to variables for later execution, and so forth.\n\n\
  In this chapter, we won't debate the issue of what functional programming is or isn't but will instead discuss some features \
  of Rust that are similar to features in many languages often referred to as functional.\n\n\
  More specifically, we'll cover:\n\n\
  {solid_disc} {1}, a function-like construct you can store in a variable\n\
  {solid_disc} {2}, a way of processing a series of elements\n\
  {solid_disc} How to use closures and iterators to improve the I/O project in Chapter 12\n\
  {solid_disc} The performance of closures and iterators (spoiler alert: They're faster than you might think!)\n\n\
  We've already covered some other Rust features, such as pattern matching and enums, that are also influenced by the \
  functional style.\n\
  Because mastering closures and iterators is an important part of writing fast, idiomatic, Rust code, we'll devote this entire chapter to them.
  ",
  "functional programming".italic().bold(),
  "Closures".italic().bold(),
  "Iterators".italic().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Programming in a functional style often includes using functions as values by passing them in arguments, returning them \
  from other functions, assigning them to variables for later execution, and so forth.
  ",
  "REMEMBER".bright_white().bold()
  )
}

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  menu::subheader_title("Section Introduction");

  println!(
  "Rust's closures are anonymous functions you can save in a variable or pass as arguments to other functions.\n\
  You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context.\n\
  Unlike functions, closures can capture values from the scope in which they're defined.\n\
  We'll demonstrate how these closure features allow for code reuse and behavior customization.
  ")
}

// Header: Capturing the Environment. Abbreviated as cte.
fn cte_content() {
  menu::subheader_title("Capturing the Environment");

  println!(
  "See: {0}, for code sample and explanations.\n\n\
  One thing to keep in mind is that closures can capture values from the scope in which they're defined, \
  which is a powerful feature that allows for more flexible and reusable code.\n\
  This means that closures can access and manipulate variables that are defined outside of their own scope, \
  making them particularly useful for tasks like iterating over collections or creating custom behavior for functions.
  ",
  "https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-the-environment".cyan()
  )
}

// Header: Inferring and Annotating Closure Types. Abbreviated as iact.
fn iact_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Inferring and Annotating Closure Types");
  
  println!(
  "There are more differences between functions and closures.\n\
  Closures don't usually require you to annotate the types of the parameters or the return value like {0} functions do.\n\
  Type annotations are required on functions because the types are part of an explicit interface exposed to your users.\n\
  Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns.\n\
  Closures, on the other hand, aren't used in an exposed interface like this:\n\
  {solid_disc} They're stored in variables\n\
  {solid_disc} and they're used without naming them\n\
  {solid_disc} and exposing them to users of our library.\n\
  ",
  "fn".bright_yellow().bold()
  );

  println!(
  "Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario.\n\
  Within these limited contexts, the compiler can infer the types of the parameters and the return type, similar to \
  how it's able to infer the types of most variables ({0}).
  ",
  "there are rare cases where the compiler needs closure type annotations too".bright_white().bold()
  );

  println!(
  "As with variables, we can add type annotations if we want to increase explicitness and clarity at the cost of being \
  more verbose than is strictly necessary.\n\
  Annotating the types for a closure would look like the definition shown in Listing 13-2.\n\
  In this example, we're defining a closure and storing it in a variable rather than defining the closure in the spot we \
  pass it as an argument.\n\n\
  See Listing 13-2:{0}, for code sample, explanations and for complete reading.
  ",
  "https://doc.rust-lang.org/book/ch13-01-closures.html#listing-13-2".cyan()
  );

  println!(
  "{0}\n\n\
  {solid_disc} Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario.\n\
  {solid_disc} Within these limited contexts, the compiler can infer the types of the parameters and the return type, \
  similar to how it's able to infer the types of most variables.\n\
  {solid_disc} There are rare cases where the compiler needs closure type annotations too.\n\
  {solid_disc} We usually define closure in the spot we pass it as an argument.\n\
  {solid_disc} You can remove curly braces if the closure body has only one expression.\n\
  {solid_disc} For closure definitions, the compiler will infer one concrete type for each of their parameters and for \
  their return value. \n\
  {solid_disc} If a closure has no type annotations, we can call the closure with any type, and the compiler will infer \
  the types based on how the closure is first used. \n\
  ",
  "REMEMBER".bright_white().bold()
  )
}

// Header: Capturing References or Moving Ownership. Abbreviated as cromo.
fn cromo_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Capturing References or Moving Ownership");

  println!(
  "Closures can capture values from their environment in three ways, which directly map to the three ways a \
  function can take a parameter:\n\
  {solid_disc} borrowing immutably\n\
  {solid_disc} borrowing mutably,\n\
  {solid_disc} and taking ownership.\n\n\
  The closure will decide which of these to use based on what the body of the function (closure) does with the captured values.
  ");

  println!(
  "In Listing 13-4, we define a closure that captures an immutable reference to the vector named list because it only \
  needs an immutable reference to print the value.\n\n\
  See Listing 13-4:{0}, for code sample, explanations and for complete reading.
  ",
  "https://doc.rust-lang.org/book/ch13-01-closures.html#listing-13-4".cyan()
  );

  println!(
  "{}\n\n\
  {solid_disc} You can bind a variable to a closure definition, and we can later call the closure by using the variable name \
  and parentheses as if the variable name were a function name.\n\
  {solid_disc} Closures also follow the rules for references and borrowing that we covered in Chapter 4. \
  You can only have one mutable reference or any number of immutable references to a particular piece of \
  data in a particular scope, but not both at the same time.\n\
  {solid_disc} You can to force the closure to take ownership of the values it uses in the environment even though the body \
  of the closure doesn't strictly need ownership, you can use the {1} keyword before the parameter list.\n\
  {solid_disc} The technique of closures taking ownership of the values they use from the environment is \
  mostly useful when passing a closure to a new thread to move the data so that it's owned by the new thread.
  ",
  "REMEMBER".bright_white().bold(),
  "move".bright_yellow().bold()
  );

}

// Header: Moving Captured Values Out of Closures. Abbreviated as mcvoc.
fn mcvoc_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";


  menu::subheader_title("Moving Captured Values Out of Closures");

  println!(
  "Once a closure has captured a reference or captured ownership of a value from the environment where the closure is defined \
  (thus affecting what, if anything, is moved into the closure), the code in the body of the closure defines what happens to the \
  references or values when the closure is evaluated later (thus affecting what, if anything, is moved out of the closure).\n\n\
  A closure body can do any of the following:\n\
  {two_spaces}{solid_disc} Move a captured value out of the closure,\n\
  {two_spaces}{solid_disc} mutate the captured value,\n\
  {two_spaces}{solid_disc} neither move nor mutate the value,\n\
  {two_spaces}{solid_disc} or capture nothing from the environment to begin with.\n\
  ");

  println!(
  "The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are \
  how functions and structs can specify what kinds of closures they can use.\n\
  Closures will automatically implement one, two, or all three of these {0} traits, in an additive fashion, depending on how the \
  closure's body handles the values:\n\
  {two_spaces}{solid_disc} {1} applies to closures that can be called once. All closures implement at least this trait because \
  all closures can be called. A closure that moves captured values out of its body will only implement {1} and none of the \
  other {0} traits because it can only be called once.\n\
  {two_spaces}{solid_disc} {2} applies to closures that don't move captured values out of their body but might mutate the \
  captured values. These closures can be called more than once.\n\
  {two_spaces}{solid_disc} {0} applies to closures that don't move captured values out of their body and don't mutate captured \
  values, as well as closures that capture nothing from their environment. These closures can be called more than once without \
  mutating their environment, which is important in cases such as calling a closure multiple times concurrently.\n\n\
  See: {3}, for complete reading.
  ",
  "Fn".bright_yellow().bold(),
  "FnOnce".bright_yellow().bold(),
  "FnMut".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch13-01-closures.html#moving-captured-values-out-of-closures".cyan()
  );

  println!(
  "{0}\n\n\
  {solid_disc} The code in the body of the closure defines what happens to the captured references or captured ownership when \
  the closure is evaluated later.\n\
  {solid_disc} The way a closure captures and handles values from the environment affects which traits the closure implements, \
  and traits are how functions and structs can specify what kinds of closures they can use.\n\
  {solid_disc} The {1} traits are important when defining or using functions or types that make use of closures.
  ",
  "REMEMBER".bright_white().bold(),
  "Fn".bright_yellow().bold()
  )
}


