use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 4];
  subheaders = [
    chapter::SubHeader::new("Chapter Introduction", ci_content),
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Storing Data on the Heap", sdoth_content),
    chapter::SubHeader::new("Enabling Recursive Types with Boxes", ertwb_content),
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
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Using Box<T> to Point to Data on the Heap");

  println!(
  "The most straightforward smart pointer is a box, whose type is written {0}.\n\
  {1} allow you to store data on the heap rather than the stack.\n\
  What remains on the stack is the pointer to the heap data.\n\n\
  Boxes don't have performance overhead, other than storing their data on the heap instead of on the stack.\n\
  But they don't have many extra capabilities either.\n\
  You'll use them most often in these situations:\n\n\
  {solid_disc} When you have a type whose size can't be known at compile time, and you want to use a value of that type in a \
  context that requires an exact size\n\
  {solid_disc} When you have a large amount of data, and you want to transfer ownership but ensure that the data won't be \
  copied when you do so\n\
  {solid_disc} When you want to own a value, and you care only that it's a type that implements a particular trait rather \
  than being of a specific type\n\n\
  We'll demonstrate the first situation in “Enabling Recursive Types with Boxes”.\n\
  In the second case, transferring ownership of a large amount of data can take a long time because the data is copied around \
  on the stack.\n\
  To improve performance in this situation, we can store the large amount of data on the heap in a box.\n\
  Then, only the small amount of pointer data is copied around on the stack, while the data it references stays in one place on \
  the heap.\n\
  The third case is known as a trait object, and “Using Trait Objects to Abstract over Shared Behavior” in Chapter 18 is \n\
  devoted to that topic.\n\
  So, what you learn here you'll apply again in that section!
  ",
  "Box<T>".bright_yellow().bold(),
  "Boxes".italic().bold()
  );
}

// Header: Storing Data on the Heap. Abbreviated as sdoth.
fn sdoth_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Storing Data on the Heap");

  println!(
  "Before we discuss the heap storage use case for {0}, we'll cover the syntax and how to interact with values stored \
  within a {0}.\n\n\
  Listing 15-1 shows how to use a box to store an {1} value on the heap.\n\
  See Listing 15-1 {2}\n\n\
  We define the variable {4} to have the value of a {6} that points to the value {7}, which is allocated on the heap.\n\
  This program will print {3} in this case, we can access the data in the box similarly to how we would if this data were on \
  the stack.\n\
  Just like any owned value, when a box goes out of scope, as {4} does at the end of {5}, it will be deallocated.\n\
  The deallocation happens both for the box (stored on the stack) and the data it points to (stored on the heap).\n\n\
  Putting a single value on the heap isn't very useful, so you won't use boxes by themselves in this way very often.\n\
  Having values like a single {8} on the stack, where they're stored by default, is more appropriate in the majority \
  of situations.\n\
  Let's look at a case where boxes allow us to define types that we wouldn't be allowed to define if we didn't have boxes.
  ",
  "Box<T>".bright_yellow().bold(),
  "i32".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch15-01-box.html#listing-15-1".bright_cyan().bold(),
  "b = 5;".bright_yellow().bold(),
  "b".bright_yellow().bold(),
  "main".bright_yellow().bold(),
  "Box".bright_yellow().bold(),
  "5".bright_yellow().bold(),
  "i32".bright_yellow().bold(),
  );

  println!(
  "{}\n\n\
  {solid_disc} {1} is a pointer, fixed size stored on the stack.
  ",
  "REMEMBER".bright_white().bold(),
  "Box<T>".bright_yellow().bold(),
  )
}

// Header: Enabling Recursive Types with Boxes. Abbreviated as ertwb.
fn ertwb_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Enabling Recursive Types with Boxes");

  println!(
  "A value of a {0} can have another value of the same type as part of itself.\n\
  Recursive types pose an issue because Rust needs to know at compile time how much space a type takes up.\n\
  However, the nesting of values of recursive types could theoretically continue infinitely, so Rust can't know how much \
  space the value needs.\n\
  Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.\n\n\
  As an example of a recursive type, let's explore the cons list.\n\
  This is a data type commonly found in functional programming languages.\n\
  The cons list type we'll define is straightforward except for the recursion; therefore, the concepts in the example we'll \
  work with will be useful anytime you get into more complex situations involving recursive types.\n\n\
  {1}\n\n\
  See: {2}, for complete reading.\n\n\
  {3}\n\n\
  See: {4}, for complete reading
  ",
  "recursive type".italic().bold(),
  "Understanding the Cons List".bright_magenta().bold(),
  "https://doc.rust-lang.org/book/ch15-01-box.html#understanding-the-cons-list".bright_cyan().bold(),
  "Computing the Size of a Non-Recursive Type".bright_magenta().bold(),
  "https://doc.rust-lang.org/book/ch15-01-box.html#computing-the-size-of-a-non-recursive-type".bright_cyan().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} To determine how much memory a custom enum type needs, Rust goes through each of the \
  variants to see which variant needs the most space.\n\
  {solid_disc} Because only one variant will be used, the most space an enum value will need is the space it would take to \
  store the largest of its variants.\n\
  {solid_disc} You can make a custom enum type with an infinite size when you define one of it's variant recursive: \
  it holds another value of itself directly. As a result, Rust can't figure out how much \
  space it needs to store an enum value (enum variant).\n\
  {solid_disc} Boxes have a known size at compile time and provide only the indirection and heap allocation; they don't have \
  any other special capabilities.\n\
  {solid_disc} {2} means that instead of storing a value directly, we should change the data structure to store the \
  value indirectly by storing a pointer to the value instead.\n\
  {solid_disc} {1} is a pointer, Rust always knows how much space a {1} needs: \
  A pointer's size doesn't change based on the amount of data it's pointing to.\n\
  {solid_disc} The {1} type is a smart pointer because it implements the {3} trait, which allows {1} values \
  to be treated like references.\n\
  {solid_disc} When a {1} value goes out of scope, the heap data that the box is pointing to is cleaned up as well because \
  of the {4} trait implementation.
  ",
  "REMEMBER".bright_white().bold(),
  "Box<T>".bright_yellow().bold(),
  "Indirection".italic().bold(),
  "Deref".bright_yellow().bold(),
  "Drop".bright_yellow().bold(),
  )
}

