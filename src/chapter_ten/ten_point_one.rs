use owo_colors::OwoColorize;
use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 2];
  subheaders = [
    chapter::SubHeader::new("Introduction", introduction_content),
    chapter::SubHeader::new("Generics in Function Definitions", gifd_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Introduction. Abbreviated as i.
fn introduction_content() {
  let solid_disc = "\u{2022}";

  // header title.
  menu::subheader_title("Introduction");


  // header content.
  println!(
    "Every programming language has tools for effectively handling the duplication of concepts. \
    In Rust, one such tool is {}: abstract stand-ins for concrete types or other properties. \n\
    We can express the behavior of generics or how they relate to other generics without \
    knowing what will be in their place when compiling and running the code.
  ",
    "generics".italic()
  );

  println!(
    "Functions can take parameters of some generic type, instead of a concrete type like \
    {0} or {1}, in the same way they take parameters with unknown values to run the same \
    code on multiple concrete values.
  ",
    "i32".bright_yellow().bold(),
    "String".bright_yellow().bold(),
  );

  println!(
    "You can combine traits with generic types to constrain a generic type to accept only \
    those types that have a particular behavior, as opposed to just any type. \n\n\
    We'll also use generics with {}: a variety of generics that give the compiler \
    information about how references relate to each other. \n\
    Lifetimes allow us to give the compiler enough information about borrowed values so \
    that it can ensure that references will be valid in more situations than it could without \
    our help.
  ",
    "lifetimes".italic()
  );

  println!(
    "{}\n\n\
    {solid_disc} Generics allow us to replace specific types with a placeholder that represents \
    multiple types to remove code duplication.
  ",
    "REMEMBER".bright_white().bold()
  )
}

// Header: Generics in Function Definitions. Abbreviated as gifd.
fn gifd_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Generics in Function Definitions");

  println!(
    "When defining a function that uses generics, we place the generics in the signature \
    of the function where we would usually specify the data types of the parameters and \
    return value. \n\
    Doing so makes our code more flexible and provides more functionality to callers of \
    our function while preventing code duplication.\n\n\
    See Listing 10-4: {}, for code samples
  ",
    "https://doc.rust-lang.org/book/ch10-01-syntax.html#listing-10-4".cyan()
  );

  println!(
    "In Listing 10-4, the {0} and {1} function have the same code, \
    so let's eliminate the duplication by introducing a {2} in a \
    single function.
  ",
    "largest_i32".bright_yellow().bold(),
    "largest_char".bright_yellow().bold(),
    "generic type parameter".italic()
  );

  println!(
    "To parameterize the types in a new single function, we need to name the {0}, \
    just as we do for the {1} to a function. \n\
    You can use any identifier as a type parameter name. \n\
    But we'll use {2} because, by convention, type parameter names in Rust are short, often just \
    one letter, and Rust's type-naming convention is UpperCamelCase. Short for {2}, {3} is the \
    default choice of most Rust programmers.
  ",
    "type parameter".italic(),
    "value parameters".italic(),
    "T".bright_yellow().bold(),
    "type".italic()
  );

  println!(
    "When we use a parameter in the body of the function, we have to declare the parameter \
    name in the signature so that the compiler knows what that name means.\n\
    Similarly, when we use a type parameter name in a function signature, we have to declare \
    the type parameter name before we use it.\n\
    To define the generic {0} function, we place type name declarations inside angle \
    brackets, {1}, between the name of the function and the parameter list, like this:\n\n\
    {2}\n\n\
    We read this definition as “The function {0} is generic over some type {3}.”\n\
    This function has one parameter named {4}, which is a slice of values of type {3}.\n\
    The {0} function will return a reference to a value of the same type {3}.
  ",
    "largest".bright_yellow().bold(),
    "<>".bright_yellow().bold(),
    "fn largest<T>(list: &[T]) -> &T {".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "list".bright_yellow().bold(),
  );

  println!(
    "Listing 10-5: {}, shows the combined {1} function definition using the generic data \
    type in its signature.\n\
    The listing also shows how we can call the function with either a slice of {2} values \
    or {3} values.\n\
    Note that this code won't compile yet. The error below listing 10-5 states that the body \
    of the {1} function won't work for all possible types that {4} could be.\n\
    Because we want to compare values of type {4} in the body, we can only use types whose \
    values can be ordered.\n\
    To enable comparisons, the standard library has the {5} trait that you \
    can implement on types.\n\
    To fix Listing 10-5, we can follow the help text's suggestion and restrict the types \
    valid for {4} to only those that implement {5}.
  ",
    "https://doc.rust-lang.org/book/ch10-01-syntax.html#listing-10-5".cyan(),
    "largest".bright_yellow().bold(),
    "i32".bright_yellow().bold(),
    "char".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "std::cmp::PartialOrd".bright_yellow().bold(),
  );

  println!(
    "{}\n\n\
    {solid_disc} By convention, type parameter names in Rust are short, often just one letter. \
    Short for type, {1} is the default choice of most Rust programmers.\n\
    {solid_disc} Rust’s type-naming convention is UpperCamelCase.\n\
    {solid_disc} To use a generic type parameter name in a function signature, we have to \
    declare the type parameter name inside angle brackets, {2}, between the name of the function and the parameter list.

  ",
    "REMEMBER".bright_white().bold(),
    "T".bright_yellow().bold(),
    "<>".bright_yellow().bold()
  )
}





