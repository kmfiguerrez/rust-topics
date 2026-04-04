use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 5];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("The Iterator Trait and the next Method", titatnm_content),
    chapter::SubHeader::new("Methods That Consume the Iterator", mtcti_content),
    chapter::SubHeader::new("Methods That Produce Other Iterators", mtpoi_content),
    chapter::SubHeader::new("Closures That Capture Their Environment", ctcte_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Processing a Series of Items with Iterators");

  println!(
  "The iterator pattern allows you to perform some task on a sequence of items in turn.\n\
  An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished.\n\
  When you use iterators, you don't have to reimplement that logic yourself.\n\n\
  In Rust, iterators are {0}, meaning they have no effect until you call methods that consume the iterator to use it up.\n\
  For example, the code in Listing 13-10 creates an iterator over the items in the vector {1} by calling the iter method \
  defined on {2}.\n\
  This code by itself doesn't do anything useful.\n\n\
  See Listing 13-10:{3}, for code sample\n\n\
  The iterator is stored in the {4} variable.\n\
  Once we've created an iterator, we can use it in a variety of ways.\n\
  In Listing 3-5:{5}, we iterated over an array using a {6} loop to execute some code on each of its items.\n\
  Under the hood, this implicitly created and then consumed an iterator, but we glossed over how exactly that works until now.\n\n\
  In the example in Listing 13-11, we separate the creation of the iterator from the use of the iterator in the {6} loop.\n\
  When the {6} loop is called using the iterator in {4}, each element in the iterator is used in one iteration of the loop, \
  which prints out each value.\n\n\
  See Listing 13-11:{7}, for code sample\n\n\
  In languages that don't have iterators provided by their standard libraries, you would likely write this same functionality \
  by starting a variable at index 0, using that variable to index into the vector to get a value, and incrementing the variable \
  value in a loop until it reached the total number of items in the vector.\n\n\
  Iterators handle all of that logic for you, cutting down on repetitive code you could potentially mess up.\n\
  Iterators give you more flexibility to use the same logic with many different kinds of sequences, not just data structures you \
  can index into, like vectors.\n\
  Let's examine how iterators do that.
  ",
  "lazy".italic().bold(),
  "v1".bright_yellow().bold(),
  "Vec<T>".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch13-02-iterators.html#listing-13-10".cyan(),
  "v1_iter".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch03-05-control-flow.html#listing-3-5".cyan(),
  "for".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch13-02-iterators.html#listing-13-11".cyan(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} The iterator pattern allows you to perform some task on a sequence of items in turn.\n\
  {solid_disc} An iterator is responsible for the logic of iterating over each item and determining \
  when the sequence has finished.\n\
  {solid_disc} Under the hood, when the {1} loop iterates over collections, it implicitly creates and consumes an iterator.\n\
  {solid_disc} Iterators give you more flexibility to use the same logic with many different kinds of sequences, not just data \
  structures you can index into, like vectors.
  ",
  "REMEMBER".bright_white().bold(),
  "for".bright_yellow().bold(),
  )  
}

// Header: The Iterator Trait and the next Method. Abbreviated as titatnm.
fn titatnm_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("The Iterator Trait and the next Method");

  println!(
  "All iterators implement a trait named {3} that is defined in the standard library.\n\n\
  See: {0}, for code sample.\n\n\
  Notice that this definition uses some new syntax: {1} and {2}, which are defining an associated type with this trait.\n\
  We'll talk about associated types in depth in Chapter 20.\n\
  For now, all you need to know is that this code says implementing the {3} trait requires that you also define an {4} type, \
  and this {4} type is used in the return type of the {5} method.\n\
  In other words, the {4} type will be the type returned from the iterator.\n\n\
  The {3} trait only requires implementors to define one method: the {5} method, which returns one item of the iterator at a time, \
  wrapped in {6}, and, when iteration is over, returns {7}.\n\n\
  We can call the {5} method on iterators directly; Listing 13-12 demonstrates what values are returned from repeated calls to \
  {5} on the iterator created from the vector.\n\n\
  See Listing 13-12:{8}, for code sample.\n\n\
  Note that we needed to make {9} mutable: Calling the {5} method on an iterator changes internal state that the iterator uses \
  to keep track of where it is in the sequence.\n\
  In other words, this code consumes, or uses up, the iterator.\n\
  Each call to {5} eats up an item from the iterator.\n\
  We didn't need to make {9} mutable when we used a {10} loop, because the loop took ownership of {9} and made it mutable \
  behind the scenes.\n\n\
  Also note that the values we get from the calls to next are immutable references to the values in the vector.\n\
  The {11} method produces an iterator over immutable references.\n\
  If we want to create an iterator that takes ownership of {12} and returns owned values, we can call {13} instead of {11}.\n\
  Similarly, if we want to iterate over mutable references, we can call {14} instead of {11}.
  ",
  "https://doc.rust-lang.org/book/ch13-02-iterators.html#the-iterator-trait-and-the-next-method".cyan(),
  "type Item".bright_yellow().bold(),
  "Self::Item".bright_yellow().bold(),
  "Iterator".bright_yellow().bold(),
  "Item".bright_yellow().bold(),
  "next".bright_yellow().bold(),
  "Some".bright_yellow().bold(),
  "None".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch13-02-iterators.html#listing-13-12".cyan(),
  "v1_iter".bright_yellow().bold(),
  "for".bright_yellow().bold(),
  "iter".bright_yellow().bold(),
  "v1".bright_yellow().bold(),
  "into_iter".bright_yellow().bold(),
  "iter_mut".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} The {1} trait only requires implementors to define one method: the {3} method, which returns one item of the \
  iterator at a time, wrapped in {6}, and, when iteration is over, returns {7}.\n\
  {solid_disc} The {2} method produces an iterator over immutable references. \
  Calling the {3} method on an iterator changes internal state that the iterator uses \
  to keep track of where it is in the sequence. So the iterator produces by {2} method needs to be mutable. \
  Also note that the values we get from the calls to {3} are immutable references to the values in the collection.\n\
  {solid_disc} If we want to create an iterator that takes ownership of a collection and returns owned values, \
  we can call {4} instead of {2}.\n\
  {solid_disc} Similarly, if we want to iterate over mutable references, we can call {5} instead of {2}.

  ",
  "REMEMBER".bright_white().bold(),
  "Iterator".bright_yellow().bold(),
  "iter".bright_yellow().bold(),
  "next".bright_yellow().bold(),
  "into_iter".bright_yellow().bold(),
  "iter_mut".bright_yellow().bold(),
  "Some".bright_yellow().bold(),
  "None".bright_yellow().bold(),
  )

}

// Header: Methods That Consume the Iterator. Abbreviated as mtcti.
fn mtcti_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Methods That Consume the Iterator");

  println!(
  "The {0} trait has a number of different methods with default implementations provided by the standard library; \
  you can find out about these methods by looking in the standard library API documentation for the {0} trait.\n\
  Some of these methods call the {1} method in their definition, which is why you're required to implement the {1} method \
  when implementing the {0} trait.\n\n\
  Methods that call {1} are called {2} because calling them uses up the iterator.\n\
  One example is the {3} method, which takes ownership of the iterator and iterates through the items by repeatedly calling {1}, \
  thus consuming the iterator.\n\
  As it iterates through, it adds each item to a running total and returns the total when iteration is complete.\n\
  Listing 13-13 has a test illustrating a use of the sum method.\n\n\
  See Listing 13-13:{4}, for code sample.\n\n\
  We aren't allowed to use {5} after the call to {3}, because {3} takes ownership of the iterator we call it on.
  ",
  "Iterator".bright_yellow().bold(),
  "next".bright_yellow().bold(),
  "consuming adapters".italic(),
  "sum".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch13-02-iterators.html#listing-13-13".cyan(),
  "v1_iter".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Methods that call next are called {1} because calling them uses up the iterator - taking ownership of iterators.\n\
  {solid_disc} {1} consume iterators to prevent you from accidentally using an iterator after calling a method that consumes it, \
  which would be a bug because the iterator is no longer what it was after calling the {2} method.\n\
  {solid_disc} Because these {1} call the {2} method, which modifies the internal state of the iterator that keeps track of where \
  the iterator is in the sequence, and that state does not reset after the method is done, which would be a bug if you used the \
  iterator again after calling a consuming method, the compiler prevents you from doing so by enforcing ownership rules - \
  the consumed iterator is dropped when the method is done.\n\
  {solid_disc} An iterator is a one-way street. Once you've walked to the end of the street to count the houses (summing the values), \
  you are already at the end. The iterator is now \"empty\" (calling {2} will return {3}).
  ",
  "REMEMBER".bright_white().bold(),
  "consuming adapters".italic(),
  "next".bright_yellow().bold(),
  "None".bright_yellow().bold()
  )
}

// Header: Methods That Produce Other Iterators. Abbreviated as mtpoi.
fn mtpoi_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Methods That Produce Other Iterators");

  println!(
  "{0} are methods defined on the {1} trait that don't consume the iterator.\n\
  Instead, they produce different iterators by changing some aspect of the original iterator.\n\n\
  Listing 13-14 shows an example of calling the iterator adapter method {2}, which takes a closure to call on each item as the \
  items are iterated through.\n\
  The {2} method returns a new iterator that produces the modified items.\n\
  The closure here creates a new iterator in which each item from the vector will be incremented by 1.\n\n\
  See Listing 13-14:{3}, for code sample and the code ouput.\n\n\
  The code in Listing 13-14 doesn't do anything; {4}.\n\
  The warning reminds us why: Iterator adapters are lazy, and we need to consume the iterator here.\n\
  To fix this warning and consume the iterator, we'll use the collect method, which we used with {5} in Listing 12-1.\n\
  This method consumes the iterator and collects the resultant values into a collection data type.\n\n\
  In Listing 13-15, we collect the results of iterating over the iterator that's returned from the call to {2} into a vector.\n\
  This vector will end up containing each item from the original vector, incremented by 1.\n\n\
  See Listing 13-15:{6}, for code sampl.\n\n\
  Because {2} takes a closure, we can specify any operation we want to perform on each item.\n\
  This is a great example of how closures let you customize some behavior while reusing the iteration behavior that the {1} \
  trait provides.\n\
  You can chain multiple calls to iterator adapters to perform complex actions in a readable way.\n\
  But because all iterators are lazy, you have to call one of the consuming adapter methods to get results from calls to \
  iterator adapters.
  ",
  "Iterator adapters".italic(),
  "Iterator".bright_yellow().bold(),
  "map".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch13-02-iterators.html#listing-13-14".cyan(),
  "the closure we've specified never gets called".bright_white().bold(),
  "env::args".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch13-02-iterators.html#listing-13-15".cyan(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} {1} as well as an iterator itself are lazy (doesn't do anything), and need to be consumed to actually do anything \
  (to actually run iterator adapters). i.e. calling consumer adapters on iterator adapters.\n\
  {solid_disc} For example calling the {2} or {3} method on {4} method is the only time the {4} method's closure gets called.\n\
  {solid_disc} Rust's iterator system is considered \"zero-cost abstraction\"—it's incredibly efficient because it doesn't do \
  work until it absolutely has to.
  ",
  "REMEMBER".bright_white().bold(),
  "Iterator adapters".italic(),
  "collect".bright_yellow().bold(),
  "sum".bright_yellow().bold(),
  "map".bright_yellow().bold(),
  )

}

// Header: Closures That Capture Their Environment. Abbreviated as ctcte.
fn ctcte_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Closures That Capture Their Environment");

  println!(
  "Many iterator adapters take closures as arguments, and commonly the closures we'll specify as arguments to iterator adapters \
  will be closures that capture their environment.\n\n\
  For this example, we'll use the {0} method that takes a closure.\n\
  The closure gets an item from the iterator and returns a {1}.\n\
  If the closure returns {2}, the value will be included in the iteration produced by filter.\n\
  If the closure returns {3}, the value won't be included.\n\n\
  In Listing 13-16, we use {0} with a closure that captures the {4} variable from its environment to iterate over \
  a collection of {5} struct instances.\n\
  It will return only shoes that are the specified size.\n\n\
  See Listing 13-16:{6}, for code sample.\n\n\
  The {7} function takes ownership of a vector of shoes and a shoe size as parameters.\n\
  It returns a vector containing only shoes of the specified size.\n\n\
  In the body of {7}, we call {8} to create an iterator that takes ownership of the vector.\n\
  Then, we call {0} to adapt that iterator into a new iterator that only contains elements for which the closure returns {2}.\n\n\
  The closure captures the {4} parameter from the environment and compares the value with each shoe's size, keeping only shoes \
  of the size specified.\n\
  Finally, calling {9} gathers the values returned by the adapted iterator into a vector that's returned by the function.\n\n\
  The test shows that when we call {7}, we get back only shoes that have the same size as the value we specified.
  ",
  "filter".bright_yellow().bold(),
  "bool".bright_yellow().bold(),
  "true".bright_yellow().bold(),
  "false".bright_yellow().bold(),
  "shoe_size".bright_yellow().bold(),
  "Shoe".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch13-02-iterators.html#listing-13-16".cyan(),
  "shoes_in_size".bright_yellow().bold(),
  "into_iter".bright_yellow().bold(),
  "collect".bright_yellow().bold(),
  )
}


 