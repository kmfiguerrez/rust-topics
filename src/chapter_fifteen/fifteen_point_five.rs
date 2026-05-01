use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 5];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Using Interior Mutability", uim_content),
    chapter::SubHeader::new("Enforcing Borrowing Rules at Runtime", ebrar_content),
    chapter::SubHeader::new("Using Interior Mutability", uim_content),
    chapter::SubHeader::new("Allowing Multiple Owners of Mutable Data", amoomd_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: RefCell<T> and the Interior Mutability Pattern");

  println!(
  "{0} is a design pattern in Rust that allows you to mutate data even when there are immutable references to that \
  data; normally, this action is disallowed by the borrowing rules.\n\
  To mutate data, the pattern uses {1} code inside a data structure to bend Rust's usual rules that govern mutation \
  and borrowing.\n\
  Unsafe code indicates to the compiler that we're checking the rules manually instead of relying on the compiler to check them \
  for us; we will discuss unsafe code more in Chapter 20.\n\n\
  We can use types that use the interior mutability pattern only when we can ensure that the borrowing rules will be followed \
  at runtime, even though the compiler can't guarantee that.\n\
  The unsafe code involved is then wrapped in a safe API, and the outer type is still immutable.
  ",
  "Interior mutability".italic().bold(),
  "unsafe".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} {1} is a design pattern in Rust that allows you to mutate data even when there are immutable references to that \
  data; normally, this action is disallowed by the borrowing rules.\n\
  {solid_disc} We can use types that use the interior mutability pattern only when we can ensure that the borrowing rules will \
  be followed at runtime, even though the compiler can't guarantee that.
  ",
  "REMEMBER".bright_white().bold(),
  "Interior mutability".italic().bold(),
  )
}

// Header: Enforcing Borrowing Rules at Runtime. Abbreviated as ebrar.
fn ebrar_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Enforcing Borrowing Rules at Runtime");

  println!(
  "Unlike {0}, the {1} type represents single ownership over the data it holds.\n\
  So, what makes {1} different from a type like {2}? Recall the borrowing rules you learned in Chapter 4:\n\
  {solid_disc} At any given time (time here means reference scope), you can have either one mutable reference or any number \
  of immutable references (but not both).\n\
  {solid_disc} References must always be valid.\n\n\
  With references and {2}, the borrowing rules' invariants are enforced at compile time. With {1}, these invariants are \
  enforced at runtime.\n\
  With references, if you break these rules, you'll get a compiler error.\n\
  With {1}, if you break these rules, your program will panic and exit.
  ",
  "Rc<T>".bright_yellow().bold(),
  "RefCell<T>".bright_yellow().bold(),
  "Box<T>".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  The advantages of checking the borrowing rules at compile time are that errors will be caught sooner in the development process, \
  and there is no impact on runtime performance because all the analysis is completed beforehand.\n\
  For those reasons, checking the borrowing rules at compile time is the best choice in the majority of cases, which is why this \
  is Rust's default.
  ",
  "ADVANTAGES OF CHECKING THE BORROWING RULES AT COMPILE TIME".bright_magenta().bold(),
  );

  println!(
  "{0}\n\n\
  The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed, \
  where they would've been disallowed by the compile-time checks.\n\
  Static analysis, like the Rust compiler, is inherently conservative.\n\
  Some properties of code are impossible to detect by analyzing the code: The most famous example is the Halting Problem, \
  which is beyond the scope of this book but is an interesting topic to research.
  ",
  "ADVANTAGES OF CHECKING THE BORROWING RULES AT RUNTIME".bright_magenta().bold()
  );

  println!(
  "Because some analysis is impossible, if the Rust compiler can't be sure the code complies with the ownership rules, \
  it might reject a correct program; in this way, it's conservative.\n\
  If Rust accepted an incorrect program, users wouldn't be able to trust the guarantees Rust makes.\n\
  However, if Rust rejects a correct program, the programmer will be inconvenienced, but nothing catastrophic can occur.\n\
  The {0} type is useful when you're sure your code follows the borrowing rules but the compiler is unable to understand and \
  guarantee that.\n\n\
  Similar to {1}, {0} is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in \
  a multithreaded context.\n\
  We'll talk about how to get the functionality of {0} in a multithreaded program in Chapter 16.
  ",
  "RefCell<T>".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold()
  );

  println!(
  "Here is a recap of the reasons to choose {0}, {1}, or {2}:\n\n\
  {solid_disc} {1} enables multiple owners of the same data; {0} and {2} have single owners.\n\
  {solid_disc} {0} allows immutable or mutable borrows checked at compile time; {1} allows only immutable borrows checked at \
  compile time; {2} allows immutable or mutable borrows checked at runtime.\n\
  {solid_disc} Because {2} allows mutable borrows checked at runtime, you can mutate the value inside the {2} even when \
  the {2} is immutable.
  ",
  "Box<T>".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold(),
  "RefCell<T>".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} The {1} type is useful when you're sure your code follows the borrowing rules but the compiler is unable to understand and \
  guarantee that.\n\
  {solid_disc} Similar to {2}, {1} is only for use in single-threaded scenarios and will give you a compile-time error if you try \
  using it in a multithreaded context.\n\
  {solid_disc} Mutating the value inside an immutable value is the interior mutability pattern.
  ",
  "REMEMBER".bright_white().bold(),
  "RefCell<T>".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold()
  )
}

// Header: Using Interior Mutability. Abbreviated as uim.
fn uim_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Using Interior Mutability");

  println!(
  "See: {0}, for sample, output and complete reading.\n\n\
  However, there are situations in which it would be useful for a value to mutate itself in its methods but appear \
  immutable to other code.\n\
  Code outside the value's methods would not be able to mutate the value.\n\
  Using {1} is one way to get the ability to have interior mutability, but {1} doesn't get around the borrowing rules completely: \
  The borrow checker in the compiler allows this interior mutability, and the borrowing rules are checked at runtime instead.\n\
  If you violate the rules, you'll get a {2} instead of a compiler error.
  ",
  "https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#using-interior-mutability".bright_cyan(),
  "RefCell<T>".bright_yellow().bold(),
  "panic!".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  Sometimes during testing a programmer will use a type in place of another type, in order to observe particular behavior and \
  assert that it's implemented correctly.\n\
  This placeholder type is called a {1}.\n\
  Think of it in the sense of a stunt double in filmmaking, where a person steps in and substitutes for an actor to do a \
  particularly tricky scene.\n\
  Test doubles stand in for other types when we're running tests.\n\
  Mock objects are specific types of test doubles that record what happens during a test so that you can assert that the \
  correct actions took place.\n\n\
  Rust doesn't have objects in the same sense as other languages have objects, and Rust doesn't have mock object functionality \
  built into the standard library as some other languages do.\n\
  However, you can definitely create a struct that will serve the same purposes as a mock object.\n\n\
  See: {2}, for complete reading.\n\n\
  This content talks about using {3} to make an immutable value that can mutate itself, and then using that to create \
  a mock object to test that the code is calling the correct methods on a trait object.
  ",
  "Testing with Mock Objects".bright_magenta().bold(),
  "test double".italic().bold(),
  "https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#testing-with-mock-objects".bright_cyan(),
  "RefCell<T>".bright_yellow().bold()
  );

  println!(
  "{0}\n\n\
  When creating immutable and mutable references, we use the {1} and {2} syntax, respectively.\n\
  With {3}, we use the {4} and {5} methods, which are part of the safe API that belongs to {3}.\n\
  The {4} method returns the smart pointer type {6}, and {5} returns the smart pointer type {7}.\n\
  Both types implement {8}, so we can treat them like regular references.\n\n\
  The {3} keeps track of how many {6} and {7} smart pointers are currently active.\n\
  Every time we call borrow, the {3} increases its count of how many immutable borrows are active.\n\
  When a {6} value goes out of scope, the count of immutable borrows goes down by 1.\n\
  Just like the compile-time borrowing rules, {3} lets us have many immutable borrows or one mutable borrow at \n\
  any point in time.\n\n\
  If we try to violate these rules, rather than getting a compiler error as we would with references, the implementation \
  of {3} will panic at runtime.\n\
  Listing 15-23 shows a modification of the implementation of send in Listing 15-22.\n\
  We're deliberately trying to create two mutable borrows active for the same scope to illustrate that {3} prevents us from \
  doing this at runtime.\n\n\
  See Listing 15-23:{9}, for code sample, output and complete reading.\n\n\
  Choosing to catch borrowing errors at runtime rather than compile time, as we've done here, means you'd potentially be \
  finding mistakes in your code later in the development process: possibly not until your code was deployed to production.\n\
  Also, your code would incur a small runtime performance penalty as a result of keeping track of the borrows at runtime rather \
  than compile time.\n\
  However, using {3} makes it possible to write a mock object that can modify itself to keep track of the messages it has seen \
  while you're using it in a context where only immutable values are allowed.\n\
  You can use {3} despite its trade-offs to get more functionality than regular references provide.
  ",
  "Tracking Borrows at Runtime".bright_magenta().bold(),
  "&".bright_yellow().bold(),
  "&mut".bright_yellow().bold(),
  "RefCell<T>".bright_yellow().bold(),
  "borrow".bright_yellow().bold(),
  "borrow_mut".bright_yellow().bold(),
  "Ref<T>".bright_yellow().bold(),
  "RefMut<T>".bright_yellow().bold(),
  "Deref".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#listing-15-23".bright_cyan()
  );

  println!(
  "{0}\n\n\
  {solid_disc} There are situations in which it would be useful for a value to mutate itself in its methods but appear \
  immutable to other code. Code outside the value's methods would not be able to mutate the value.\n\
  {solid_disc} Using {1} is one way to get the ability to have interior mutability, but {1} doesn't get around \
  the borrowing rules completely.\n\
  {solid_disc} {2} is a placeholder type used during testing to observe particular behavior and assert that it's \
  implemented correctly.\n\
  {solid_disc} Mock objects are specific types of {2} that record what happens during a test so that \
  you can assert that the correct actions took place.\n\
  {solid_disc} The {3} keeps track of how many {4} and {5} smart pointers are currently active.\n\
  {solid_disc} Just like the compile-time borrowing rules, {3} lets us have many immutable borrows or one mutable \
  borrow at any point in time.
  ",
  "REMEMBER".bright_white().bold(),
  "RefCell<T>".bright_yellow().bold(),
  "test double".italic().bold(),
  "RefCell<T>".bright_yellow().bold(),
  "Ref<T>".bright_yellow().bold(),
  "RefMut<T>".bright_yellow().bold()
  )
}

// Header: Allowing Multiple Owners of Mutable Data. Abbreviated as amoomd.
fn amoomd_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Allowing Multiple Owners of Mutable Data");

  println!(
  "A common way to use {0} is in combination with {1}.\n\
  Recall that {1} lets you have multiple owners of some data, but it only gives immutable access to that data.\n\
  If you have an {1} that holds a {0}, you can get a value that can have multiple owners and that you can mutate!\n\n\
  For example, recall the cons list example in Listing 15-18 where we used {1} to allow multiple lists to share ownership of \
  another list.\n\
  Because {1} holds only immutable values, we can't change any of the values in the list once we've created them.\n\
  Let's add in {1} for its ability to change the values in the lists.\n\
  Listing 15-24 shows that by using a {0} in the {2} definition, we can modify the value stored in all the lists.\n\n\
  See Listing 15-24:{3}, for code sample, output and complete reading.\n\n\
  Note that Rust has a feature called {4} and {5}. Calling methods is one of the few places in Rust with this behavior.\n\
  When you call a method with {6}, Rust automatically adds in {7}, {8}, or {9} so that object matches the signature of the method.\n\
  See: {10}\n\n\
  For example, the line {11} in Listing 15-24, before anything else can happen, Rust evaluates the right side of the dot operator.\n\
  {solid_disc} {12} is of type {13}.\n\
  {solid_disc} The {1} (Reference Counted) smart pointer does not have a method called {14}.\n\
  {solid_disc} {15}: The compiler knows that {1} implements the {16} trait. It automatically follows the pointer inside the {1} to \
  get to the inner {17}.\n\
  {solid_disc} It then calls the {14} method on the {17} which returns a {18} smart pointer, and we use the dereference operator \
  on it and change the inner value.

  ",
  "RefCell<T>".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold(),
  "Cons".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#listing-15-24".bright_cyan(),
  "automatic referencing".italic().bold(),
  "dereferencing".italic().bold(),
  "object.something()".bright_yellow().bold(),
  "&".bright_yellow().bold(),
  "&mut".bright_yellow().bold(),
  "*".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator".bright_cyan(),
  "*value.borrow_mut() += 10;".bright_yellow().bold(),
  "value".bright_yellow().bold(),
  "Rc<RefCell<i32>>".bright_yellow().bold(),
  "borrow_mut()".bright_yellow().bold(),
  "Auto-deref kicks in".bright_white().bold(),
  "Deref".bright_yellow().bold(),
  "RefCell<i32>".bright_yellow().bold(),
  "RefMut<T>".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Note that Rust has a feature called {11} and {12}.\n\
  {solid_disc} {10} enforces borrowing rules at runtime.\n\
  {solid_disc} The smart pointer types {1} and {2} returned by the {3} and {4} methods repectively implement the {5} trait, \
  so we can treat them like a regular references.\n\
  {solid_disc} When we use the {6} dereference operator on types that implement the {7} trait, Rust run the code: {8}, \
  behind the scenes to dereference the value.\n\
  {solid_disc} Rust substitutes the {6} operator with a call to the {9} method and then a plain dereference.
  ",
  "REMEMBER".bright_white().bold(),
  "RefMut<T>".bright_yellow().bold(),
  "Ref<T>".bright_yellow().bold(),
  "borrow_mut".bright_yellow().bold(),
  "borrow".bright_yellow().bold(),
  "Deref".bright_yellow().bold(),
  "*".bright_yellow().bold(),
  "Deref".bright_yellow().bold(),
  "*(TypeInstance.deref())".bright_yellow().bold(),
  "deref".bright_yellow().bold(),
  "RefCell<T>".bright_yellow().bold(),
  "automatic referencing".italic().bold(),
  "dereferencing".italic().bold(),

  )
}




