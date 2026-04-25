use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 7];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Following the Reference to the Value", ftrttv_content),
    chapter::SubHeader::new("Using Box<T> Like a Reference", ublar_content),
    chapter::SubHeader::new("Defining Our Own Smart Pointer", doosp_content),
    chapter::SubHeader::new("Implementing the Deref Trait", itdt_content),
    chapter::SubHeader::new("Using Deref Coercion in Functions and Methods", udcifam_content),
    chapter::SubHeader::new("Handling Deref Coercion with Mutable References", hdcwmr_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  menu::subheader_title("Section Introduction: Treating Smart Pointers Like Regular References");

  println!(
  "Implementing the {0} trait allows you to customize the behavior of the {1} {2} (not to be confused with the \
  multiplication or glob operator).\n\
  By implementing {0} in such a way that a smart pointer can be treated like a regular reference, you can write code that \
  operates on references and use that code with smart pointers too.\n\n\
  Let's first look at how the dereference operator works with regular references.\n\
  Then, we'll try to define a custom type that behaves like {3} and see why the dereference operator doesn't work like \
  a reference on our newly defined type.\n\
  We'll explore how implementing the {0} trait makes it possible for smart pointers to work in ways similar to references.\n\
  Then, we'll look at Rust's deref coercion feature and how it lets us work with either references or smart pointers.
  ",
  "Deref".bright_yellow().bold(),
  "dereference operator".italic().bold(),
  "*".bright_yellow().bold(),
  "Box<T>".bright_yellow().bold(),
  )
}

// Header: Following the Reference to the Value. Abbreviated as ftrttv.
fn ftrttv_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Following the Reference to the Value");

  println!(
  "A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else.\n\
  In Listing 15-6, we create a reference to an {0} value and then use the dereference operator to \
  follow the reference to the value.\n\n\
  See Listing 15-6:{1}, for code sample.\n\n\
  The variable {2} holds an {0} value {3}.\n\
  We set {4} equal to a reference to {2}.\n\
  We can assert that {2} is equal to {3}.\n\
  However, if we want to make an assertion about the value in {4}, we have to use {5} to follow the reference to the value it's \
  pointing to (hence, dereference) so that the compiler can compare the actual value.\n\
  Once we dereference {4}, we have access to the integer value {4} is pointing to that we can compare with {3}.\n\n\
  If we tried to write {6} instead, we would get compilation error:\n\n\
  See: {7}, for output error.\n\n\
  Comparing a number and a reference to a number isn't allowed because they're different types.\n\
  We must use the dereference operator to follow the reference to the value it's pointing to.
  ",
  "i32".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch15-02-deref.html#listing-15-6".bright_cyan(),
  "x".bright_yellow().bold(),
  "5".bright_yellow().bold(),
  "y".bright_yellow().bold(),
  "*y".bright_yellow().bold(),
  "assert_eq!(5, y);".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch15-02-deref.html#following-the-reference-to-the-value".bright_cyan(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} {1} means to follow the reference to the value it's pointing to.
  ",
  "REMEMBER".bright_white().bold(),
  "dereference".italic().bold()
  )
}

// Header: Using Box<T> Like a Reference. Abbreviated as ublar.
fn ublar_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Using Box<T> Like a Reference");

  println!(
  "We can rewrite the code in Listing 15-6 to use a {2} instead of a reference; the dereference operator used on the \
  {2} in Listing 15-7 functions in the same way as the dereference operator used on the reference in Listing 15-6.\n\n\
  Listing 15-6: {0}\n\
  Listing 15-7: {1}\n\n\
  The main difference between Listing 15-7 and Listing 15-6 is that here we set {3} to be an instance of a box pointing to a \
  copied value of {4} rather than a reference pointing to the value of {4}.\n\
  In the last assertion, we can use the dereference operator to follow the box's pointer in the same way that we did when {3} \
  was a reference.\n\
  Next, we'll explore what is special about {2} that enables us to use the dereference operator by defining our own box type.
  ",
  "https://doc.rust-lang.org/book/ch15-02-deref.html#listing-15-6".bright_cyan(),
  "https://doc.rust-lang.org/book/ch15-02-deref.html#listing-15-7".bright_cyan(),
  "Box<T>".bright_yellow().bold(),
  "y".bright_yellow().bold(),
  "x".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} We can use the dereference operator to follow a box's pointer the same way with regular reference.
  ",
  "REMEMBER".bright_white().bold(),
  )

}

// Header: Defining Our Own Smart Pointer. Abbreviated as doosp.
fn doosp_content() {
  menu::subheader_title("Defining Our Own Smart Pointer");

  println!(
  "Let's build a wrapper type similar to the {0} type provided by the standard library to experience how smart pointer types \
  behave differently from references by default.\n\
  Then, we'll look at how to add the ability to use the dereference operator.
  ",
  "Box<T>".bright_yellow().bold()
  );

  println!(
  "Note: There's one big difference between the {0} type we're about to build and the real {1}: \
  Our version will not store its data on the heap.\n\
  We are focusing this example on {2}, so where the data is actually stored is less important than the pointer-like behavior.
  ",
  "MyBox<T>".bright_yellow().bold(),
  "Box<T>".bright_yellow().bold(),
  "Deref".bright_yellow().bold(),
  );

  println!(
  "The {0} type is ultimately defined as a tuple struct with one element, \
  so Listing 15-8 defines a {1} type in the same way.\n\
  We'll also define a {2} function to match the {2} function defined on {0}.\n\n\
  See Listing 15-8: {3}, for code sample.\n\n\
  We define a struct named {4} and declare a generic parameter {5} because we want our type to hold values of any type.\n\
  The {4} type is a tuple struct with one element of type {5}.\n\
  The {6} function takes one parameter of type {5} and returns a {4} instance that holds the value passed in.\n\n\
  Let's try adding the {7} function in Listing 15-7 to Listing 15-8 and changing it to use the {1} type we've defined \
  instead of {0}.\n\
  The code in Listing 15-9 won't compile, because Rust doesn't know how to dereference {4}.\n\n\
  See Listing 15-9: {8}, for code sample and output error.\n\n\
  Our {1} type can't be dereferenced because we haven't implemented that ability on our type.\n\
  To enable dereferencing with the {9} operator, we implement the {10} trait.
  ",
  "Box<T>".bright_yellow().bold(),
  "MyBox<T>".bright_yellow().bold(),
  "new".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch15-02-deref.html#listing-15-8".bright_cyan().bold(),
  "MyBox".bright_yellow().bold(),
  "T".bright_yellow().bold(),
  "MyBox::new".bright_yellow().bold(),
  "main".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch15-02-deref.html#listing-15-9".bright_cyan().bold(),
  "*".bright_yellow().bold(),
  "Deref".bright_yellow().bold(),
  )
}

// Header: Implementing the Deref Trait. Abbreviated as itdt.
fn itdt_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Implementing the Deref Trait");

  println!(
  "As discussed in “Implementing a Trait on a Type”: {}, in Chapter 10, to implement a trait we need to provide implementations \
  for the trait's required methods.\n\
  The {1} trait, provided by the standard library, requires us to implement one method named {2} that borrows {3} and \
  returns a reference to the inner data.\n\
  Listing 15-10 contains an implementation of {1} to add to the definition of {4}.\n\n\
  See Listing 15-10: {5}, for code sample  and complete reading.
  ",
  "https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type".bright_cyan(),
  "Deref".bright_yellow().bold(),
  "deref".bright_yellow().bold(),
  "self".bright_yellow().bold(),
  "MyBox<T>".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch15-02-deref.html#listing-15-10".bright_cyan(),
  );

  println!(
  "{}\n\n\
  {solid_disc} The {1} method returns a reference to a value.
  {solid_disc} Note that the {2} operator is replaced with a call to the {1} method and then a call to the {2} operator \
  just once, each time we use a {2} in our code. Because the substitution of the {2} operator does not recurse infinitely.
  ",
  "REMEMBER".bright_white().bold(),
  "defef".bright_yellow().bold(),
  "*".bright_yellow().bold()
  )
}

// Header: Using Deref Coercion in Functions and Methods. Abbreviated as udcifam.
fn udcifam_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Using Deref Coercion in Functions and Methods");

  println!(
  "{0} converts a reference to a type that implements the Deref trait into a reference to another type.\n\
  For example, deref coercion can convert {1} to {2} because {3} implements the {4} trait such that it returns {2}.\n\
  Deref coercion is a convenience Rust performs on arguments to functions and methods, and it works only on types \
  that implement the {4} trait.\n\
  It happens automatically when we pass a reference to a particular type's value as an argument to a function or method \
  that doesn't match the parameter type in the function or method definition.\n\
  A sequence of calls to the {5} method converts the type we provided into the type the parameter needs.
  ",
  "Deref coercion".italic().bold(),
  "&String".bright_yellow().bold(),
  "&str".bright_yellow().bold(),
  "String".bright_yellow().bold(),
  "Deref".bright_yellow().bold(),
  "deref".bright_yellow().bold(),
  );

  println!(
  "Deref coercion was added to Rust so that programmers writing function and method calls don't need to add as many explicit \
  references and dereferences with {0} and {1}.\n\
  The deref coercion feature also lets us write more code that can work for either references or smart pointers.
  ",
  "&".bright_yellow().bold(),
  "*".bright_yellow().bold(),
  );

  println!(
  "To see deref coercion in action, let's use the {0} type we defined in Listing 15-8 as well as the implementation of {1} that \
  we added in Listing 15-10.\n\
  Listing 15-11 shows the definition of a function that has a string slice parameter.\n\n\
  See Listing 15-11: {2}, for code sample and complete reading.\n\n\
  When the {1} trait is defined for the types involved, Rust will analyze the types and use {3} as many times as \
  necessary to get a reference to match the parameter's type.\
  The number of times that {3} needs to be inserted is resolved at compile time, so there is no runtime \
  penalty for taking advantage of deref coercion!
  ",
  "MyBox<T>".bright_yellow().bold(),
  "Deref".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch15-02-deref.html#listing-15-11".bright_cyan(),
  "Deref::deref".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} {1} converts a reference to a type that implements the {2} trait into a reference to another type.\n\
  {solid_disc} {1} is a convenience Rust performs on arguments to functions and methods, \
  and it works only on types that implement the Deref trait.\n\
  {solid_disc} The {3} is used as many times as necessary to get a reference to match the parameter's type and resolved \
  at compile time, so there is no runtime penalty for taking advantage of deref coercion!.\n\
  {solid_disc} {1} was added to Rust so that programmers writing function and method calls don't need \
  to add as many explicit references and dereferences with {4} and {5}.\n\
  {solid_disc} The deref coercion feature also lets us write more code that can work for either references or smart pointers.

  ",
  "REMEMBER".bright_white().bold(),
  "Deref coercion".italic().bold(),
  "Deref".bright_yellow().bold(),
  "Deref::deref".bright_yellow().bold(),
  "&".bright_yellow().bold(),
  "*".bright_yellow().bold(),
  )
}

// Header: Handling Deref Coercion with Mutable References. Abbreviated as hdcwmr.
fn hdcwmr_content() {
  menu::subheader_title("Handling Deref Coercion with Mutable References");

  println!(
  "Similar to how you use the {0} trait to override the {1} operator on immutable references, you can use the {2} trait \
  to override the {1} operator on mutable references.\n\n\
  Rust does deref coercion when it finds types and trait implementations in three cases:\n\n\
  1. From {3} to {4} when {5}\n\
  2. From {6} to {7} when {8}\n\
  3. From {6} to {4} when {5}\n\n\
  The first two cases are the same except that the second implements mutability.\n\
  The first case states that if you have a {3}, and {9} implements {0} to some type {10}, you can get a {4} transparently.\n\
  The second case states that the same deref coercion happens for mutable references.\n\n\
  The third case is trickier: Rust will also coerce a mutable reference to an immutable one.\n\
  But the reverse is not possible: Immutable references will never coerce to mutable references.\n\
  Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that \
  data (otherwise, the program wouldn't compile).\n\
  Converting one mutable reference to one immutable reference will never break the borrowing rules.\n\
  Converting an immutable reference to a mutable reference would require that the initial immutable reference is the only \
  immutable reference to that data, but the borrowing rules don't guarantee that.\n\
  Therefore, Rust can't make the assumption that converting an immutable reference to a mutable reference is possible.
  ",
  "Deref".bright_yellow().bold(),
  "*".bright_yellow().bold(),
  "DerefMut".bright_yellow().bold(),
  "&T".bright_yellow().bold(),
  "&U".bright_yellow().bold(),
  "T: Deref<Target=U>".bright_yellow().bold(),
  "&mut T".bright_yellow().bold(),
  "&mut U".bright_yellow().bold(),
  "T: DerefMut<Target=U>".bright_yellow().bold(),
  "T".bright_yellow().bold(),
  "U".bright_yellow().bold(),
  )
}


