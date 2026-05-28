use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 4];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Defining a Trait for Common Behavior", dtfcb_content),
    chapter::SubHeader::new("Implementing the Trait", itt_content),
    chapter::SubHeader::new("Performing Dynamic Dispatch", pdp_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Using Trait Objects to Abstract over Shared Behavior");

  println!(
  "See: {0}, for complete reading.
  ",
  "https://doc.rust-lang.org/book/ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior".bright_cyan()
  )
}

// Header: Defining a Trait for Common Behavior. Abbreviated as dtfcb.
fn dtfcb_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Defining a Trait for Common Behavior");

  println!(
  "See: {0}, for complete reading.\n\n\
  {1}\n\n\
  A trait object points to both an instance of a type implementing our specified trait and a table used to look up trait methods \
  on that type at runtime.\n\
  We create a trait object by specifying some sort of pointer, such as a reference or a {2} smart pointer, then the {3} keyword, \
  and then specifying the relevant trait.\n\
  (We'll talk about the reason trait objects must use a pointer in “Dynamically Sized Types and the Sized Trait” in \
  Chapter 20.):{4}\n\n\
  We can use trait objects in place of a generic or concrete type.\n\
  Wherever we use a trait object, Rust's type system will ensure at compile time that any value used in that context will implement \
  the trait object's trait.\n\
  Consequently, we don't need to know all the possible types at compile time.\n\n\
  i.e. {5}, which is a trait object; it's a stand-in for any type inside a {6} that implements the {7} trait.
  ",
  "https://doc.rust-lang.org/book/ch18-02-trait-objects.html#defining-a-trait-for-common-behavior".bright_cyan(),
  "Defining trait object".bright_magenta().bold(),
  "Box<T>".bright_yellow().bold(),
  "dyn".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-03-advanced-types.html#dynamically-sized-types-and-the-sized-trait".bright_cyan(),
  "Box<dyn Draw>".bright_yellow().bold(),
  "Box".bright_yellow().bold(),
  "Draw".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  We've mentioned that, in Rust, we refrain from calling structs and enums “objects” to distinguish them from other languages' \
  objects.\n\
  In a struct or enum, the data in the struct fields and the behavior in impl blocks are separated, whereas in other languages, \
  the data and behavior combined into one concept is often labeled an object.\n\
  Trait objects differ from objects in other languages in that we can't add data to a trait object.\n\
  Trait objects aren't as generally useful as objects in other languages: Their specific purpose is to allow abstraction \
  across common behavior.\n\
  Listing 18-3 shows how to define a trait named {1} with one method named {2}.\n\n\
  See Listing 18-3: {3}, for code sample and complete reading.
  ",
  "Rust doesn't call structs and enums as objects".bright_magenta().bold(),
  "Draw".bright_yellow().bold(),
  "draw".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch18-02-trait-objects.html#listing-18-3".bright_cyan()
  );

  println!(
  "{0}\n\n\
  {solid_disc} A {1} points to both an instance of a type implementing our specified trait and a table used to look up trait \
  methods on that type at runtime.\n\
  {solid_disc} We create a trait object by specifying some sort of pointer, such as a reference or a {2} smart pointer, \
  then the {3} keyword, and then specifying the relevant trait.\n\
  {solid_disc} We can use trait objects in place of a generic or concrete type.\n\
  {solid_disc} A generic type parameter can be substituted with only one concrete type at a time, whereas trait objects allow for \
  multiple concrete types to fill in for the trait object at runtime.

  ",
  "REMEMBER".bright_white().bold(),
  "trait object".italic().bold(),
  "Box<T>".bright_yellow().bold(),
  "dyn".bright_yellow().bold(),
  )
}

// Header: Implementing the Trait. Abbreviated as itt.
fn itt_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Implementing the Trait");

  println!(
  "See: {0}, for code sample and complete reading.
  The advantage of using trait objects and Rust's type system to write code similar to code using duck typing is that we never have \
  to check whether a value implements a particular method at runtime or worry about getting errors if a value doesn't implement a \
  method but we call it anyway.\n\
  Rust won't compile our code if the values don't implement the traits that the trait objects need.
  ",
  "https://doc.rust-lang.org/book/ch18-02-trait-objects.html#implementing-the-trait".bright_cyan()
  )
}

// Header: Performing Dynamic Dispatch. Abbreviated as pdp.
fn pdp_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Performing Dynamic Dispatch");

  println!(
  "Recall in “Performance of Code Using Generics” in Chapter 10 our discussion on the monomorphization process performed on \
  generics by the compiler: The compiler generates nongeneric implementations of functions and methods for each concrete type \
  that we use in place of a generic type parameter.\n\
  The code that results from monomorphization is doing {0}, which is when the compiler knows what method you're \
  calling at compile time.\n\
  This is opposed to {1}, which is when the compiler can't tell at compile time which method you're calling.\n\
  In dynamic dispatch cases, the compiler emits code that at runtime will know which method to call.\n\n\
  When we use trait objects, Rust must use dynamic dispatch.\n\
  The compiler doesn't know all the types that might be used with the code that's using trait objects, so it doesn't know which \
  method implemented on which type to call.\n\
  Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call.\n\
  This lookup incurs a runtime cost that doesn't occur with static dispatch.\n\
  Dynamic dispatch also prevents the compiler from choosing to inline a method's code, which in turn prevents some optimizations, \
  and Rust has some rules about where you can and cannot use dynamic dispatch, called dyn compatibility.\n\
  Those rules are beyond the scope of this discussion, but you can read more about them in the reference.\n\
  However, we did get extra flexibility in the code that we wrote in Listing 18-5 and were able to support in Listing 18-9, \
  so it's a trade-off to consider.
  ",
  "static dispatch".italic().bold(),
  "dynamic dispatch".italic().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} When we use trait objects, Rust must use dynamic dispatch.\n\
  {solid_disc} Rust uses the pointers inside the trait object to know which method to call. \
  This lookup incurs a runtime cost that doesn't occur with static dispatch.

  ",
  "REMEMBER".bright_yellow().bold(),

  )
}













