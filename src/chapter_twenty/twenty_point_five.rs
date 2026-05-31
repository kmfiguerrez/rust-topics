use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 7];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("The Difference Between Macros and Functions", tdbmaf_content),
    chapter::SubHeader::new("Declarative Macros for General Metaprogramming", dmfgm_content),
    chapter::SubHeader::new("Procedural Macros for Generating Code from Attributes", pmfgcfa_content),
    chapter::SubHeader::new("Custom derive Macros", cdm_content),
    chapter::SubHeader::new("Attribute-Like Macros", alm_content),
    chapter::SubHeader::new("Function-Like Macros", flm_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Macros");

  println!(
  "We've used macros like {0} throughout this book, but we haven't fully explored what a macro is and how it works.\n\
  The term {1} refers to a family of features in Rust—declarative macros with {2} and three kinds of procedural macros:\n\n\
  {solid_disc} Custom {3} macros that specify code added with the derive attribute used on structs and enums\n\
  {solid_disc} Attribute-like macros that define custom attributes usable on any item\n\
  {solid_disc} Function-like macros that look like function calls but operate on the tokens specified as their argument\n\n\
  We'll talk about each of these in turn, but first, let's look at why we even need macros when we already have functions.
  ",
  "println!".bright_yellow().bold(),
  "macro".italic().bold(),
  "macro_rules!".bright_yellow().bold(),
  "#[derive]".bright_yellow().bold(),
  )
}

// Header: The Difference Between Macros and Functions. Abbreviated as tdbmaf.
fn tdbmaf_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("The Difference Between Macros and Functions");

  println!(
  "Fundamentally, macros are a way of writing code that writes other code, which is known as {0}.\n\
  In Appendix C, we discuss the {1} attribute, which generates an implementation of various traits for you.\n\
  We've also used the {2} and {3} macros throughout the book.\n\
  All of these macros expand to produce more code than the code you've written manually.\n\n\
  Metaprogramming is useful for reducing the amount of code you have to write and maintain, which is also one of the roles of \
  functions.\n\
  However, macros have some additional powers that functions don't have.
  ",
  "metaprogramming".italic().bold(),
  "derive".bright_yellow().bold(),
  "println!".bright_yellow().bold(),
  "vec!".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  A function signature must declare the number and type of parameters the function has.\n\
  Macros, on the other hand, can take a variable number of parameters: We can call {1} with one argument or {2} with two \
  arguments.\n\
  Also, macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a \
  trait on a given type.\n\
  A function can't, because it gets called at runtime and a trait needs to be implemented at compile time.
  ",
  "Macros have over Functions".bright_magenta().bold(),
  "println!(\"hello\")".bright_yellow().bold(),
  "println!(\"hello {}\", name)".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  The downside to implementing a macro instead of a function is that macro definitions are more complex than function definitions \
  because you're writing Rust code that writes Rust code.\n\
  Due to this indirection, macro definitions are generally more difficult to read, understand, and maintain than function \
  definitions.\n\n\
  Another important difference between macros and functions is that you must define macros or bring them into scope before you call \
  them in a file, as opposed to functions you can define anywhere and call anywhere.
  ",
  "Cons of macros".bright_magenta().bold(),
  );  

  println!(
  "{0}\n\n\
  {solid_disc} Fundamentally, macros are a way of writing code that writes other code, which is known as {1}.\n\
  ",
  "REMEMBER".bright_white().bold(),
  "metaprogramming".italic().bold(),
  )
}

// Header: Declarative Macros for General Metaprogramming. Abbreviated as dmfgm.
fn dmfgm_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Declarative Macros for General Metaprogramming");

  println!(
  "The most widely used form of macros in Rust is the declarative macro.\n\
  These are also sometimes referred to as “macros by example,” “{0} macros,” or just plain “macros.”\n\
  At their core, declarative macros allow you to write something similar to a Rust match expression.\n\
  As discussed in Chapter 6, {1} expressions are control structures that take an expression, compare the resultant value of the \
  expression to patterns, and then run the code associated with the matching pattern.\n\
  Macros also compare a value to patterns that are associated with particular code: In this situation, the value is the literal \
  Rust source code passed to the macro; the patterns are compared with the structure of that source code; and the code associated \
  with each pattern, when matched, replaces the code passed to the macro.\n\
  This all happens during compilation.\n\n\
  See: {2}, for complete reading.
  ",
  "macro_rules!".bright_yellow().bold(),
  "match".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-05-macros.html#declarative-macros-for-general-metaprogramming".bright_cyan()
  );

  println!(
  "{0}\n\n\
  {solid_disc} The most widely used form of macros in Rust is the {1}, sometimes referred to as “macros by example,” \
  “{2} macros,” or just plain “macros.”.\n\
  {solid_disc} At their core, declarative macros allow you to write something similar to a Rust {3} expression. 
  ",
  "REMEMBER".bright_yellow().bold(),
  "declarative macro".italic().bold(),
  "macro_rules!".bright_yellow().bold(),
  "match".bright_yellow().bold(),
  )
}

// Header: Procedural Macros for Generating Code from Attributes. Abbreviated as pmfgcfa.
fn pmfgcfa_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Procedural Macros for Generating Code from Attributes");

  println!(
  "The second form of macros is the procedural macro, which acts more like a function (and is a type of procedure).\n\
  Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather than matching \
  against patterns and replacing the code with other code as declarative macros do.\n\
  The three kinds of procedural macros are:\n\n\
  {solid_disc} custom derive,\n\
  {solid_disc} attribute-like,\n\
  {solid_disc} and function-like,\n\n\
  and all work in a similar fashion.\n\n\
  When creating procedural macros, the definitions must reside in their own crate with a special crate type.\n\
  This is for complex technical reasons that we hope to eliminate in the future.\n\
  In Listing 20-36, we show how to define a procedural macro, where {0} is a placeholder for using a specific macro variety.\n\n\
  See: Listing 20-36:{1}, for code sample.\n\n\
  The function that defines a procedural macro takes a {2} as an input and produces a {2} as an output.\n\
  The {2} type is defined by the {3} crate that is included with Rust and represents a sequence of tokens.\n\
  This is the core of the macro: The source code that the macro is operating on makes up the input TokenStream, and the code \
  the macro produces is the output {2}.\n\
  The function also has an attribute attached to it that specifies which kind of procedural macro we're creating.\n\
  We can have multiple kinds of procedural macros in the same crate.
  ",
  "some_attribute".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-05-macros.html#listing-20-36".bright_cyan(),
  "TokenStream".bright_yellow().bold(),
  "proc_macro".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} The function that defines a procedural macro takes a {1} as an input and produces a {1} as an output.\n\
  {solid_disc} The function also has an attribute attached to it that specifies which kind of procedural macro we're creating.\n\
  {solid_disc} We can have multiple kinds of procedural macros in the same crate.
  ",
  "REMEMBER".bright_yellow().bold(),
  "TokenStream".bright_yellow().bold(),
  )
}

// Header: Custom derive Macros. Abbreviated as cdm.
fn cdm_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Custom derive Macros");

  println!(
  "See: {0}, for complete reading.\n\n\
  The purpose of custom derive macros is to spare users from having to write the implementation block for each type just to use \
  the default implementations of associated functions or methods.\n\
  Instead, we want them to just annotate their type with {1}
  ",
  "https://doc.rust-lang.org/book/ch20-05-macros.html#custom-derive-macros".bright_cyan(),
  "#[derive(Trait)]".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} At the time of this writing, procedural macros need to be in their own crate.\n\
  {solid_disc} The {1} crate is the compiler's API that allows us to read and manipulate Rust code from our code.\n\
  {solid_disc} The {2} crate parses Rust code from a string into a data structure that we can perform operations on.\n\
  {solid_disc} The {3} crate turns syn data structures back into Rust code.
  ",
  "REMEMBER".bright_white().bold(),
  "proc_macro".bright_yellow().bold(),
  "syn".bright_yellow().bold(),
  "quote".bright_yellow().bold(),
  )
}

// Header: Attribute-Like Macros. Abbreviated as alm.
fn alm_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Attribute-Like Macros");

  println!(
  "Attribute-like macros are similar to custom {0} macros, but instead of generating code for the {0} attribute, they allow you to \
  create new attributes.\n\
  They're also more flexible: derive only works for structs and enums; attributes can be applied to other items as well, such as \
  functions.\n\
  Here's an example of using an attribute-like macro.\n\
  Say you have an attribute named {2} that annotates functions when using a web application framework:\n\n\
  See: {1}, for code samples and complete reading.
  ",
  "derive".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-05-macros.html#attribute-like-macros".bright_cyan()
  "route".bright_yellow().bold(),
  )
}

// Header: Function-Like Macros. Abbreviated as flm.
fn flm_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Function-Like Macros");

  println!(
  "Function-like macros define macros that look like function calls.\n\
  Similarly to {0} macros, they're more flexible than functions; for example, they can take an unknown number of arguments.\n\
  However, {0} macros can only be defined using the match-like syntax we discussed in the \
  “Declarative Macros for General Metaprogramming” section earlier.\n\
  Function-like macros take a {1} parameter, and their definition manipulates that {1} using Rust code as the other two types \
  of procedural macros do.\n\
  An example of a function-like macro is an {2} macro that might be called like so:\n\n\
  See:{3}. for samples and complete reading.
  ",
  "macro_rules!".bright_yellow().bold(),
  "TokenStream".bright_yellow().bold(),
  "sql!".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-05-macros.html#function-like-macros".bright_cyan()

  )
}
