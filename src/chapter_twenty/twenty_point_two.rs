use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 5];
  subheaders = [
    chapter::SubHeader::new("Defining Traits with Associated Types", dtwat_content),
    chapter::SubHeader::new("Using Default Generic Parameters and Operator Overloading", udgpaoo_content),
    chapter::SubHeader::new("Disambiguating Between Identically Named Methods", dbinm_content),
    chapter::SubHeader::new("Using Supertraits", us_content),
    chapter::SubHeader::new("Implementing External Traits with the Newtype Pattern", ietwtnp_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Defining Traits with Associated Types. Abbreviated as dtwat.
fn dtwat_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Defining Traits with Associated Types");

  println!(
  "See: {0}, for complete reading.\n\n\
  Associate type is just arguing that if a trait definition uses generic type parameters, then it could have multilple \
  implementations for the same type.\n\
  When you use a method in the trait, you have to explicitly annotate the type of the item returned by the method to indicate \
  which implementation of the trait you want to use.\n\n\
  With associated types, we don't need to annotate types, because we can't implement a trait on a type multiple times.\n\
  As a result it prevents you from accidentally using the wrong implementation of a trait and getting an error that can \
  be difficult to understand.\n\n\
  Associated types also become part of the trait's contract: Implementors of the trait must provide a type to stand in for the \
  associated type placeholder.\n\
  Associated types often have a name that describes how the type will be used, and documenting the associated type in the API \
  documentation is a good practice.
  ",
  "https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#defining-traits-with-associated-types".bright_cyan()
  );

  println!(
  "{0}\n\n\
  {solid_disc} Traits with associated types can only have one implementation per type, so the compiler can infer the type \
  of the item returned by a method that uses an associated type.\n\
  {solid_disc} Traits with generic type parameters can have multiple implementations per type, so you must annotate the \
  return type of a method that uses a generic type parameter to specify which implementation you want to use.
  ",
  "REMEMBER".bright_yellow().bold(),
  )
}

// Header: Using Default Generic Parameters and Operator Overloading. Abbreviated as udgpaoo.
fn udgpaoo_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Using Default Generic Parameters and Operator Overloading");

  println!(
  "When we use generic type parameters, we can specify a default concrete type for the generic type.\n\
  This eliminates the need for implementors of the trait to specify a concrete type if the default type works.\n\
  You specify a default type when declaring a generic type with the {0} syntax.\n\n\
  A great example of a situation where this technique is useful is with {1}, in which you customize the behavior of an \
  operator (such as {2}) in particular situations.\n\n\
  Rust doesn't allow you to create your own operators or overload arbitrary operators.\n\
  But you can overload the operations and corresponding traits listed in {3} by implementing the traits associated \
  with the operator.\n\
  For example, in Listing 20-15, we overload the {2} operator to add two {4} instances together.\n\
  We do this by implementing the {5} trait on a {4} struct.\n\n\
  See Listing 20-15:{6}, for code sample and complete reading.
  ",
  "<PlaceholderType=ConcreteType>".bright_yellow().bold(),
  "operator overloading".italic().bold(),
  "+".bright_yellow().bold(),
  "std::ops".bright_yellow().bold(),
  "Point".bright_yellow().bold(),
  "Add".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#listing-20-15".bright_cyan()
  );

  println!(
  "{0}\n\n\
  {solid_disc} {1}, is when you customize the behavior of an operator (such as {2}) in particular situations.\n\
  {solid_disc} Rust doesn't allow you to create your own operators or overload arbitrary operators.\n\
  {solid_disc} But you can overload the operations and corresponding traits listed in {3} by implementing the traits associated \
  with the operator.\n\
  ",
  "REMEMBER".bright_yellow().bold(),
  "operator overloading".italic().bold(),
  "+".bright_yellow().bold(),
  "std::ops".bright_yellow().bold(),
  )
}

// Header: Disambiguating Between Identically Named Methods. Abbreviated as dbinm.
fn dbinm_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Disambiguating Between Identically Named Methods");

  println!(
  "Nothing in Rust prevents a trait from having a method with the same name as another trait's method, nor does Rust prevent you \
  from implementing both traits on one type.\n\
  It's also possible to implement a method directly on the type with the same name as methods from traits.\n\n\
  When calling methods with the same name, you'll need to tell Rust which one you want to use.\n\
  Consider the code in Listing 20-17 where we've defined two traits, {0} and {1}, that both have a method called {2}.\n\
  We then implement both traits on a type {3} that already has a method named {2} implemented on it.\n\
  Each {2} method does something different.\n\n\
  See Listing 20-17:{4}, for code sample and complete reading.
  ",
  "Pilot".bright_yellow().bold(),
  "Wizard".bright_yellow().bold(),
  "fly".bright_yellow().bold(),
  "Human".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#listing-20-17".bright_cyan()
  );

  println!(
  "{0}\n\n\
  See: Listing 20-19:{3}, for code sample and complete reading.\n\n\
  Because the {1} method takes a {2} parameter, if we had two types that both implement one trait, Rust could figure out which \
  implementation of a trait to use based on the type of {2}.
  ",
  "Calling methods with the same name from traits".bright_magenta().bold(),
  "fly".bright_yellow().bold(),
  "self".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#listing-20-19".bright_cyan()
  );

  println!(
  "{0}\n\n\
  However, associated functions that are not methods don't have a {1} parameter.\n\
  When there are multiple types or traits that define non-method functions with the same function name, Rust doesn't always know \
  which type you mean unless you use fully qualified syntax.\n\n\
  See Listing 20-20:{2}, for code sample and complete reading.\n\n\
  In general, fully qualified syntax is defined as follows:\n\
  {3}\n\n\
  For associated functions that aren't methods, there would not be a receiver: There would only be the list of other arguments.\n\
  You could use fully qualified syntax everywhere that you call functions or methods.\n\
  However, you're allowed to omit any part of this syntax that Rust can figure out from other information in the program.\n\
  You only need to use this more verbose syntax in cases where there are multiple implementations that use the same name and Rust \
  needs help to identify which implementation you want to call.
  ",
  "Using Fully Qualified Syntax".bright_magenta().bold(),
  "self".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#listing-20-20".bright_cyan(),
  "<Type as Trait>::function(receiver_if_method, next_arg, ...);".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} When calling methods with the same name, you'll need to tell Rust which one you want to use.\n\
  {solid_disc} The compiler defaults to calling the method that is directly implemented on the type, not from a trait.\n\
  {solid_disc} Specifying the trait name before the method name clarifies to Rust which implementation of the method we want \
  to call.\n\
  {solid_disc} We could also write {1}, which is equivalent to the {2} that we used in Listing 20-19, but this is a bit longer \
  to write if we don't need to disambiguate.
  ",
  "REMEMBER".bright_white().bold(),
  "StructName::MethodName(&StructInstance)".bright_yellow().bold(),
  "StructInstance.MethodName()".bright_yellow().bold()
  )
}

// Header: Using Supertraits. Abbreviated as us.
fn us_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Using Supertraits");

  println!(
  "Sometimes you might write a trait definition that depends on another trait: For a type to implement the first trait, you want \
  to require that type to also implement the second trait.\n\
  You would do this so that your trait definition can make use of the associated items of the second trait.\n\
  The trait your trait definition is relying on is called a {0} of your trait.\n\n\
  See: {1}, for code sample and complete reading.
  ",
  "supertrait".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#using-supertraits".bright_cyan()
  );

  println!(
  "{0}\n\n\
  {solid_disc} Using supertraits allows you to use the features of the supertrait in the subtrait's default method implementations.\n\
  ",
  "REMEMBER".bright_white().bold(),
  )
}

// Header: Implementing External Traits with the Newtype Pattern. Abbreviated as ietwtnp.
fn ietwtnp_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Implementing External Traits with the Newtype Pattern");

  println!(
  "In the “Implementing a Trait on a Type” section in Chapter 10:{0}, we mentioned the orphan rule that states we're only allowed \
  to implement a trait on a type if either the trait or the type, or both, are local to our crate.\n\
  It's possible to get around this restriction using the newtype pattern, which involves creating a new type in a tuple struct.\n\
  (We covered tuple structs in the “Creating Different Types with Tuple Structs” section in Chapter 5:{1}.)\n\
  The tuple struct will have one field and be a thin wrapper around the type for which we want to implement a trait.\n\
  Then, the wrapper type is local to our crate, and we can implement the trait on the wrapper.\n\
  Newtype is a term that originates from the Haskell programming language.\n\
  There is no runtime performance penalty for using this pattern, and the wrapper type is elided at compile time.\n\n\
  See Listing 20-24:{2}, for code sample and complete reading.
  ",
  "https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type".bright_cyan(),
  "https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-different-types-with-tuple-structs".bright_cyan(),
  "https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#listing-20-24".bright_cyan(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} The newtype pattern involves creating a new type in a tuple struct that has one field and is a thin wrapper \
  around the type for which we want to implement a trait.\n\
  {solid_disc} There is no runtime performance penalty for using this pattern, and the wrapper type is elided at compile time.\n\
  {solid_disc} The downside of using this technique is that wrapper is a new type, so it doesn't have the methods of the value \
  it's holding.\n\
  {solid_disc} This newtype pattern is also useful even when traits are not involved.
  ",
  "REMEMBER".bright_white().bold(),
  )
}









