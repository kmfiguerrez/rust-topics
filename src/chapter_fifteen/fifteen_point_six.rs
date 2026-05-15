use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 4];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Creating a Reference Cycle", carc_content),
    chapter::SubHeader::new("Preventing Reference Cycles Using Weak<T>", prcuw_content),
    chapter::SubHeader::new("Summary", s_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  menu::subheader_title("Section Introduction: Reference Cycles Can Leak Memory");

  println!(
  "Rust's memory safety guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up \
  (known as a memory leak).\n\
  Preventing memory leaks entirely is not one of Rust's guarantees, meaning memory leaks are memory safe in Rust.\n\
  We can see that Rust allows memory leaks by using {0} and {1}: \
  It's possible to create references where items refer to each other in a cycle.\n\
  This creates memory leaks because the reference count of each item in the cycle will never reach 0, \
  and the values will never be dropped.
  ",
  "Rc<T>".bright_yellow().bold(),
  "RefCell<T>".bright_yellow().bold()
  )
}

// Header: Creating a Reference Cycle. Abbreviated as carc.
fn carc_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Creating a Reference Cycle");

  println!(
  "Let's look at how a reference cycle might happen and how to prevent it, starting with the definition of the {0} enum and \
  a {1} method in Listing 15-25.\n\n\
  See Listing 15-25: {2}, for code sample and complete reading.\n\n\
  ",
  "List".bright_yellow().bold(),
  "tail".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#listing-15-25".bright_cyan()
  );

  println!(
  "Creating reference cycles is not easily done, but it's not impossible either.\n\
  If you have {0} values that contain {1} values or similar nested combinations of types with interior mutability and reference \
  counting, you must ensure that you don't create cycles; you can't rely on Rust to catch them.\n\
  Creating a reference cycle would be a logic bug in your program that you should use automated tests, code reviews, and \
  other software development practices to minimize.
  ",
  "RefCell<T>".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold()
  );

  println!(
  "Another solution for avoiding reference cycles is reorganizing your data structures so that some references express ownership \
  and some references don't.\n\
  As a result, you can have cycles made up of some ownership relationships and some non-ownership relationships, and only the \
  ownership relationships affect whether or not a value can be dropped.\n\
  In Listing 15-25, we always want {0} variants to own their list, so reorganizing the data structure isn't possible.
  ",
  "Cons".bright_yellow().bold()
  );

  println!(
  "{0}\n\n\
  {solid_disc} You can't rely on Rust to catch reference cycles.\n\
  {solid_disc} Creating a reference cycle would be a logic bug in your program that you should use automated tests, code reviews, and \
  other software development practices to minimize.\n\
  ",
  "REMEMBER".bright_white().bold(),
  )
}

// Header: Preventing Reference Cycles Using Weak<T>. Abbreviated as prcuw.
fn prcuw_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Preventing Reference Cycles Using Weak<T>");

  println!(
  "So far, we've demonstrated that calling {0} increases the {1} of an {2} instance, and an {2} instance is only cleaned up if \
  its {1} is 0.\n\
  You can also create a weak reference to the value within an {2} instance by calling {3} and passing a reference to the {2}.\n\
  {4} are how you can share ownership of an {2} instance.\n\
  {5} don't express an ownership relationship, and their count doesn't affect when an {2} instance is cleaned up.\n\
  They won't cause a reference cycle, because any cycle involving some weak references will be broken once the strong \
  reference count of values involved is 0.\n\n\
  When you call {3}, you get a smart pointer of type {6}.\n\
  Instead of increasing the {1} in the {2} instance by 1, calling {3} increases the {7} by 1.\n\
  The {2} type uses {7} to keep track of how many {6} references exist, similar to {1}.\n\
  The difference is the {7} doesn't need to be 0 for the {2} instance to be cleaned up.\n\n\
  Because the value that {6} references might have been dropped, to do anything with the value that a {6} is pointing to you \
  must make sure the value still exists.\n\
  Do this by calling the {11} method on a {6} instance, which will return an {8}.\n\
  You'll get a result of {9} if the {2} value has not been dropped yet and a result of {10} if the {2} value has been dropped.\n\
  Because {11} returns an {12}, Rust will ensure that the {9} case and the {10} case are handled, and there won't be an \
  invalid pointer.\n\n\
  As an example, rather than using a list whose items know only about the next item, we'll create a tree whose items know about \
  their child items and their parent items.
  ",
  "Rc::clone".bright_yellow().bold(),
  "strong_count".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold(),
  "Rc::downgrade".bright_yellow().bold(),
  "Strong references".italic().bold(),
  "Weak references".italic().bold(),
  "Weak<T>".bright_yellow().bold(),
  "weak_count".bright_yellow().bold(),
  "Option<Rc<T>>".bright_yellow().bold(),
  "Some".bright_yellow().bold(),
  "None".bright_yellow().bold(),
  "upgrade".bright_yellow().bold(),
  "Option<Rc<T>>".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  To start, we'll build a tree with nodes that know about their child nodes.\n\
  We'll create a struct named {1} that holds its own {2} value as well as references to its child {1} values.\n\n\
  See: {3}, for complete reading.
  ",
  "Creating a Tree Data Structure".bright_magenta().bold(),
  "Node".bright_yellow().bold(),
  "i32".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#creating-a-tree-data-structure".bright_cyan()
  );

  println!(
  "{0}\n\n\
  See: {1}, for complete reading.
  ",
  "Adding a Reference from a Child to Its Parent".bright_magenta().bold(),
  "https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#adding-a-reference-from-a-child-to-its-parent".bright_cyan()
  );

  println!(
  "{0}\n\n\
  See: {1}, for complete reading.\n\n\
  All of the logic that manages the counts and value dropping is built into {2} and {3} and their implementations of \
  the {4} trait.\n\
  By specifying that the relationship from a child to its parent should be a {3} reference in the definition of Node, you're \
  able to have parent nodes point to child nodes and vice versa without creating a reference cycle and memory leaks.
  ",
  "Visualizing Changes to strong_count and weak_count".bright_magenta().bold(),
  "https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#visualizing-changes-to-strong_count-and-weak_count".bright_cyan(),
  "Rc<T>".bright_yellow().bold(),
  "Weak<T>".bright_yellow().bold(),
  "Drop".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} {1} are how you can share ownership of an {3} instance.\n\
  {solid_disc} {2} don't express an ownership relationship, and their count doesn't affect when an {3} instance is cleaned up. \
  They won't cause a reference cycle, because any cycle involving some weak references will be broken once the strong \
  reference count of values involved is 0.\n\
  {solid_disc} You can create a weak reference to the value within an {3} instance by calling {4} and passing a \
  reference to the {3}.\n\
  {solid_disc} When you call {4}, you get a smart pointer of type {5}.\n\
  {solid_disc} {4} increases {6} not {7}.\n\
  {solid_disc} The {3} type uses {6} to keep track of how many {5} references exist, similar to {7}.\n\
  {solid_disc} The difference is the {6} doesn't need to be 0 for the {3} instance to be cleaned up.\n\
  ",
  "REMEMBER".bright_white().bold(),
  "Strong references".italic().bold(),
  "Weak references".italic().bold(),
  "Rc<T>".bright_yellow().bold(),
  "Rc::downgrade".bright_yellow().bold(),
  "Weak<T>".bright_yellow().bold(),
  "weak_count".bright_yellow().bold(),
  "strong_count".bright_yellow().bold(),
  )
}

// Header: Summary. Abbreviated as s.
fn s_content() {
  menu::subheader_title("Summary");

  println!(
  "This chapter covered how to use smart pointers to make different guarantees and trade-offs from those Rust makes by default \
  with regular references.\n\
  The {0} type has a known size and points to data allocated on the heap.\n\
  The {1} type keeps track of the number of references to data on the heap so that the data can have multiple owners.\n\
  The {2} type with its interior mutability gives us a type that we can use when we need an immutable type but need to change \
  an inner value of that type; it also enforces the borrowing rules at runtime instead of at compile time.\n\n\
  Also discussed were the {3} and {4} traits, which enable a lot of the functionality of smart pointers.\n\
  We explored reference cycles that can cause memory leaks and how to prevent them using {5}.
  ",
  "Box<T>".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold(),
  "RefCell<T>".bright_yellow().bold(),
  "Deref".bright_yellow().bold(),
  "Drop".bright_yellow().bold(),
  "Weak<T>".bright_yellow().bold(),
  )
}







