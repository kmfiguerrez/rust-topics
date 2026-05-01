use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 1];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),

  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Running Code on Cleanup with the Drop Trait");

  println!(
  "The second trait important to the smart pointer pattern is {0}, which lets you customize what happens when a value is \
  about to go out of scope.\n\
  You can provide an implementation for the {0} trait on any type, and that code can be used to release resources like \
  files or network connections.\n\n\
  We're introducing {0} in the context of smart pointers because the functionality of the {0} trait is almost always used \
  when implementing a smart pointer.\n\
  For example, when a {1} is dropped, it will deallocate the space on the heap that the box points to.\n\n\
  In some languages, for some types, the programmer must call code to free memory or resources every time they finish using \
  an instance of those types.\n\
  Examples include file handles, sockets, and locks.\n\
  If the programmer forgets, the system might become overloaded and crash.\n\
  In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert \
  this code automatically.\n\
  As a result, you don't need to be careful about placing cleanup code everywhere in a program that an instance of a particular \
  type is finished with—you still won't leak resources!\n\n\
  You specify the code to run when a value goes out of scope by implementing the {0} trait.\n\
  The {0} trait requires you to implement one method named {2} that takes a mutable reference to {3}.\n\
  To see when Rust calls {2}, let's implement {2} with {4} statements for now.\n\n\
  Listing 15-14 shows a {5} struct whose only custom functionality is that it will print {6} \
  when the instance goes out of scope, to show when Rust runs the {2} method.\n\n\
  See Listing 15-14: {7}, for code sample and complete reading.
  ",
  "Drop".bright_yellow().bold(),
  "Box<T>".bright_yellow().bold(),
  "drop".bright_yellow().bold(),
  "self".bright_yellow().bold(),
  "println!".bright_yellow().bold(),
  "CustomSmartPointer".bright_yellow().bold(),
  "Dropping CustomSmartPointer!".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch15-03-drop.html#listing-15-14".bright_cyan()
  );

  println!(
  "Occasionally, however, you might want to clean up a value early.\n\
  One example is when using smart pointers that manage locks: You might want to force the {0} method that releases the lock so \
  that other code in the same scope can acquire the lock.\n\n\
  We can't disable the automatic insertion of {0} when a value goes out of scope, and we can't call the {0} method explicitly \
  because Rust would still automatically call {0} on the value at the end of {1}.\n\
  This would cause a double free error because Rust would be trying to clean up the same value twice.\n\
  So, if we need to force a value to be cleaned up early, we use the {2} function.
  ",
  "drop".bright_yellow().bold(),
  "main".bright_yellow().bold(),
  "std::mem::drop".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Variables are dropped in the reverse order of their creation, so later-created variables will be \
  dropped before earlier-created variables.\n\
  {solid_disc} Rust doesn't let you call the {1} trait's {5} method manually; instead, you have to call the {2} function \
  provided by the standard library if you want to force a value to be dropped before the end of its scope.\n\
  {solid_disc} if we need to force a value to be cleaned up early, we use the {5} function.\n\
  {solid_disc} The term {3} is the general programming term for a function that cleans up an instance. \
  A destructor is analogous to a {4}, which creates an instance. The {5} function in Rust is one particular destructor.\n\
  {solid_disc} The {2} function is different from the {5} method in the {1} trait and it is included in the prelude.\n\
  {solid_disc} With the {1} trait and Rust's ownership system, you don't have to remember to clean up, because Rust does \
  it automatically.\n\
  {solid_disc} You also don't have to worry about problems resulting from accidentally cleaning up values still in use: \
  The ownership system that makes sure references are always valid also ensures that {5} gets called only once when the value \
  is no longer being used.
  ",
  "REMEMBER".bright_white().bold(),
  "Drop".bright_yellow().bold(),
  "std::mem::drop".bright_yellow().bold(),
  "destructor".italic().bold(),
  "constructor".italic().bold(),
  "drop".bright_yellow().bold(),
  )
}