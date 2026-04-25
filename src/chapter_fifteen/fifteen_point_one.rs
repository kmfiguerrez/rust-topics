use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 2];
  subheaders = [
    chapter::SubHeader::new("Chapter Introduction", ci_content),
    chapter::SubHeader::new("Section Introduction", si_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Chapter Introduction. Abbreviated as ci.
fn ci_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Chapter Introduction: Smart Pointers");

  println!(
  "A pointer is a general concept for a variable that contains an address in memory.\n\
  This address refers to, or “points at,” some other data.\n\
  The most common kind of pointer in Rust is a reference, which you learned about in Chapter 4.\n\
  References are indicated by the {0} symbol and borrow the value they point to.\n\
  They don't have any special capabilities other than referring to data, and they have no overhead.
  ",
  "&".bright_yellow().bold()
  );

  println!(
  "Smart pointers, on the other hand, are data structures that act like a pointer but also have additional \
  metadata and capabilities.\n\
  The concept of smart pointers isn't unique to Rust: Smart pointers originated in C++ and exist in other languages as well.\n\
  Rust has a variety of smart pointers defined in the standard library that provide functionality beyond that \
  provided by references.\n\
  To explore the general concept, we'll look at a couple of different examples of smart pointers, including a {0} \
  smart pointer type.\n\
  This pointer enables you to allow data to have multiple owners by keeping track of the number of owners and, when no \
  owners remain, cleaning up the data.
  ",
  "reference counting".italic().bold()
  );

  println!(
  "In Rust, with its concept of ownership and borrowing, there is an additional difference between references and smart pointers: \
  {0}.
  ",
  "While references only borrow data, in many cases smart pointers own the data they point to".bright_white().bold()
  );

  println!(
  "Smart pointers are usually implemented using structs.\n\
  Unlike an ordinary struct, smart pointers implement the {0} and {1} traits.\n\
  The {0} trait allows an instance of the smart pointer struct to behave like a reference so that you can write your code to \
  work with either references or smart pointers.\n\
  The {1} trait allows you to customize the code that's run when an instance of the smart pointer goes out of scope.\n\
  In this chapter, we'll discuss both of these traits and demonstrate why they're important to smart pointers.
  ",
  "Deref".bright_yellow().bold(),
  "Drop".bright_yellow().bold(),
  );

  println!(
  "Given that the smart pointer pattern is a general design pattern used frequently in Rust, this chapter won't cover every \
  existing smart pointer.\n\
  Many libraries have their own smart pointers, and you can even write your own.\n\
  We'll cover the most common smart pointers in the standard library:\n\
  {solid_disc} {0}, for allocating values on the heap\n\
  {solid_disc} {1}, a reference counting type that enables multiple ownership\n\
  {solid_disc} {2} and {3}, accessed through {4}, a type that enforces the borrowing rules at runtime \
  instead of compile time\n\n\
  In addition, we'll cover the {5} pattern where an immutable type exposes an API for mutating an interior value.\n\
  We'll also discuss reference cycles: how they can leak memory and how to prevent them.
  ",
  "Box<T>".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold(),
  "Ref<T>".bright_yellow().bold(),
  "RefMut<T>".bright_yellow().bold(),
  "RefCell<T>".bright_yellow().bold(),
  "interior mutability".italic().bold()
  );

  println!(
  "{0}\n\n\
  {solid_disc} A pointer is an address in memory.\n\
  {solid_disc} The most common kind of pointer in Rust is a reference.\n\
  {solid_disc} References are indicated by the {1} symbol and borrow the value they point to. \
  They don't have any special capabilities other than referring to data, and they have no overhead.\n\
  {solid_disc} You can think of a pointer like an arrow to a value stored somewhere else.\n\
  {solid_disc} {2}, on the other hand, are data structures that act like a pointer but also \
  have additional metadata and capabilities.\n\
  {solid_disc} {2} are usually implemented using structs that implement the {3} and {4} traits.
  ",
  "REMEMBER".bright_white().bold(),
  "&".bright_yellow().bold(),
  "Smart pointers".bright_yellow().bold(),
  "Deref".bright_yellow().bold(),
  "Drop".bright_yellow().bold(),
  )
}

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  menu::subheader_title("Section Introduction");
}

