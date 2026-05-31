use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 4];
  subheaders = [
    chapter::SubHeader::new("Chapter Introduction", ci_content),
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Performing Unsafe Superpowers", pus_content),
    chapter::SubHeader::new("Dereferencing a Raw Pointer", darp_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Chapter Introduction. Abbreviated as ci.
fn ci_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Chapter Introduction: Advanced Features");

  println!(
  "We'll look at a few aspects of the language you might run into every once in a while but may not use every day.\n\
  You can use this chapter as a reference for when you encounter any unknowns.\n\
  The features covered here are useful in very specific situations.\n\
  Although you might not reach for them often, we want to make sure you have a grasp of all the features Rust has to offer.\n\n\
  In this chapter, we'll cover:\n\n\
  {solid_disc} Unsafe Rust: How to opt out of some of Rust's guarantees and take responsibility for manually upholding those \
  guarantees\n\
  {solid_disc} Advanced traits: Associated types, default type parameters, fully qualified syntax, supertraits, and the newtype \
  pattern in relation to traits\n\
  {solid_disc} Advanced types: More about the newtype pattern, type aliases, the never type, and dynamically sized types\n\
  {solid_disc} Advanced functions and closures: Function pointers and returning closures\n\
  {solid_disc} Macros: Ways to define code that defines more code at compile time\n\
  ")
}

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Unsafe Rust");

  println!(
  "All the code we've discussed so far has had Rust's memory safety guarantees enforced at compile time.\n\
  However, Rust has a second language hidden inside it that doesn't enforce these memory safety guarantees: It's called unsafe \
  Rust and works just like regular Rust but gives us extra superpowers.\n\n\
  Unsafe Rust exists because, by nature, static analysis is conservative.\n\
  When the compiler tries to determine whether or not code upholds the guarantees, it's better for it to reject some valid \
  programs than to accept some invalid programs.\n\
  Although the code might be okay, if the Rust compiler doesn't have enough information to be confident, it will reject the code.\n\
  In these cases, you can use unsafe code to tell the compiler, “Trust me, I know what I'm doing.”\n\
  Be warned, however, that you use unsafe Rust at your own risk: If you use unsafe code incorrectly, problems can occur due to \
  memory unsafety, such as null pointer dereferencing.\n\n\
  Another reason Rust has an unsafe alter ego is that the underlying computer hardware is inherently unsafe.\n\
  If Rust didn't let you do unsafe operations, you couldn't do certain tasks.\n\
  Rust needs to allow you to do low-level systems programming, such as directly interacting with the operating system or even \
  writing your own operating system.\n\
  Working with low-level systems programming is one of the goals of the language.\n\
  Let's explore what we can do with unsafe Rust and how to do it.
  ");

  println!(
  "{0}\n\n\
  {solid_disc} if the Rust compiler doesn't have enough information to be confident, it will reject the code. \
  In these cases, you can use unsafe code to tell the compiler, “Trust me, I know what I'm doing.”
  ",
  "REMEMBER".bright_yellow().bold(),
  )
}

// Header: Performing Unsafe Superpowers. Abbreviated as pus.
fn pus_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Performing Unsafe Superpowers");

  println!(
  "To switch to unsafe Rust, use the {0} keyword and then start a new block that holds the unsafe code.\n\
  You can take five actions in unsafe Rust that you can't in safe Rust, which we call unsafe superpowers.\n\
  Those superpowers include the ability to:\n\n\
  1. Dereference a raw pointer.\n\
  2. Call an unsafe function or method.\n\
  3. Access or modify a mutable static variable.\n\
  4. Implement an unsafe trait.\n\
  5. Access fields of unions.\n\n\
  It's important to understand that {0} doesn't turn off the borrow checker or disable any of Rust's other safety checks: \
  If you use a reference in unsafe code, it will still be checked.\n\
  The {0} keyword only gives you access to these five features that are then not checked by the compiler for memory safety.\n\
  You'll still get some degree of safety inside an unsafe block.\n\
  In addition, {0} does not mean the code inside the block is necessarily dangerous or that it will definitely have memory safety \
  problems: The intent is that as the programmer, you'll ensure that the code inside an unsafe block will access memory in a \
  valid way.\n\n\
  People are fallible and mistakes will happen, but by requiring these five unsafe operations to be inside blocks annotated with \
  {0}, you'll know that any errors related to memory safety must be within an unsafe block.\n\
  Keep {0} blocks small; you'll be thankful later when you investigate memory bugs.\n\n\
  To isolate unsafe code as much as possible, it's best to enclose such code within a safe abstraction and provide a safe API, \
  which we'll discuss later in the chapter when we examine unsafe functions and methods.\n\
  Parts of the standard library are implemented as safe abstractions over unsafe code that has been audited.\n
  Wrapping unsafe code in a safe abstraction prevents uses of {0} from leaking out into all the places that you or your users \
  might want to use the functionality implemented with {0} code, because using a safe abstraction is safe.\n\n\
  Let's look at each of the five unsafe superpowers in turn.\n\
  We'll also look at some abstractions that provide a safe interface to unsafe code.
  ",
  "unsafe".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} To switch to unsafe Rust, use the {1} keyword and then start a new block that holds the unsafe code.\n\
  {solid_disc} It's important to understand that {1} doesn't turn off the borrow checker or disable any of Rust's other \
  safety checks.\n\
  {solid_disc} In addition, {1} does not mean the code inside the block is necessarily dangerous or that it will definitely \
  have memory safety problems.\n\
  {solid_disc} Keep {1} blocks small; you'll be thankful later when you investigate memory bugs.\n\
  {solid_disc} To isolate unsafe code as much as possible, it's best to enclose such code within a safe abstraction and provide a \
  safe API

  ",
  "REMEMBER".bright_white().bold(),
  "unsafe".bright_yellow().bold(),
  )
}

// Header: Dereferencing a Raw Pointer. Abbreviated as darp.
fn darp_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Dereferencing a Raw Pointer");

  println!(
  "In Chapter 4, in the “Dangling References” section:{0}, we mentioned that the compiler ensures that references are always valid.\n\
  Unsafe Rust has two new types called {1} that are similar to references.\n\
  As with references, raw pointers can be immutable or mutable and are written as {2} and {3}, respectively.\n\
  The asterisk isn't the dereference operator; it's part of the type name.\n\
  In the context of raw pointers, immutable means that the pointer can't be directly assigned to after being dereferenced.\n\n\
  Different from references and smart pointers, raw pointers:\n\n\
  {solid_disc} Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers \
  to the same location\n\
  {solid_disc} Aren't guaranteed to point to valid memory\n\
  {solid_disc} Are allowed to be null\n\
  {solid_disc} Don't implement any automatic cleanup\n\n\
  By opting out of having Rust enforce these guarantees, you can give up guaranteed safety in exchange for greater performance or \
  the ability to interface with another language or hardware where Rust's guarantees don't apply.\n\n\
  Listing 20-1 shows how to create an immutable and a mutable raw pointer.\n\n\
  See Listing 21-1:{4}, for code sample.\n\n\
  In Listing 20-1, notice that we don;t include the {5} keyword in this code. We can create raw pointers in safe code; we just \
  can't dereference raw pointers outside an unsafe block, as you'll see in a bit.\n\n\
  {6}\n\n\
  We've created raw pointers by using the raw borrow operators: {7} creates a {8} immutable raw pointer, \
  and {9} creates a {10} mutable raw pointer.\n\
  Because we created them directly from a local variable, we know these particular raw pointers are valid, but we can’t make \
  that assumption about just any raw pointer.
  ",
  "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references".bright_cyan(),
  "raw pointers".italic().bold(),
  "*const T".bright_yellow().bold(),
  "*mut T".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#listing-20-1".bright_cyan(),
  "unsafe".bright_yellow().bold(),
  "The raw borrow operators".bright_magenta().bold(),
  "&raw const num".bright_yellow().bold(),
  "*const i32".bright_yellow().bold(),
  "&raw mut num".bright_yellow().bold(),
  "*mut i32".bright_yellow().bold()
  );

  println!(
  "{0}\n\n\
  Next we'll create a raw pointer whose validity we can't be so certain of, using the keyword {1} to cast a value instead of using \
  the raw borrow operator.\n\
  Listing 20-2 shows how to create a raw pointer to an arbitrary location in memory.\n\
  Trying to use arbitrary memory is undefined: There might be data at that address or there might not, the compiler might optimize \
  the code so that there is no memory access, or the program might terminate with a segmentation fault.\n\
  Usually, there is no good reason to write code like this, especially in cases where you can use a raw borrow operator instead, \
  but it is possible.\n\n\
  See Listing 20-2:{2}, for code sample.\n\n\
  Recall that we can create raw pointers in safe code, but we can't dereference raw pointers and read the data being pointed to.\n\
  In Listing 20-3, we use the dereference operator {3} on a raw pointer that requires an {4} block.\n\n\
  See Listing 20-3:{5}, for code sample.\n\n\
  Creating a pointer does no harm; it's only when we try to access the value that it points at that we might end up dealing with \
  an invalid value.
  ",
  "Arbitrary memory location".bright_magenta().bold(),
  "as".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#listing-20-2".bright_cyan(),
  "*".bright_yellow().bold(),
  "unsafe".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#listing-20-3".bright_cyan()
  );

  println!(
  "{0}\n\n\
  Note also that in Listings 20-1 and 20-3, we created {1} and {2} raw pointers that both pointed to the same memory location, \
  where {3} is stored.\n\
  If we instead tried to create an immutable and a mutable reference to {3}, the code would not have compiled because Rust's \
  ownership rules don't allow a mutable reference at the same time as any immutable references.\n\
  With raw pointers, we can create a mutable pointer and an immutable pointer to the same location and change data through the \
  mutable pointer, potentially creating a data race. Be careful!\n\n\
  {4}\n\n\
  With all of these dangers, why would you ever use raw pointers?\n\
  One major use case is when interfacing with C code, as you'll see in the next section.\n\
  Another case is when building up safe abstractions that the borrow checker doesn't understand.\n\
  We'll introduce unsafe functions and then look at an example of a safe abstraction that uses unsafe code.
  ",
  "Ignoring borrowing rules".bright_magenta().bold(),
  "*const i32".bright_yellow().bold(),
  "*mut i32".bright_yellow().bold(),
  "num".bright_yellow().bold(),
  "Why use raw pointers".bright_magenta().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Unsafe Rust has two new types called {1} that are similar to references.\n\
  {solid_disc} As with references, raw pointers can be immutable or mutable and are written as {2} and {3}, respectively. \
  The asterisk isn't the dereference operator; it's part of the type name.\n\
  {solid_disc} In the context of raw pointers, {4} means that the pointer can't be directly assigned to after being \
  dereferenced.\n\
  {solid_disc} We can create raw pointers in safe code; we just can't dereference raw pointers outside an unsafe block.\n\
  {solid_disc} Creating a pointer does no harm; it's only when we try to access the value that it points at that we might \
  end up dealing with an invalid value.
  ",
  "REMEMBER".bright_white().bold(),
  "raw pointers".italic().bold(),
  "*const T".bright_yellow().bold(),
  "*mut T".bright_yellow().bold(),
  "immutable".italic().bold(),
  )
}








