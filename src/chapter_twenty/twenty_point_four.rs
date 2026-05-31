use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 2];
  subheaders = [
    chapter::SubHeader::new("Function Pointers", si_content),
    chapter::SubHeader::new("Returning Closures", rc_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Function Pointers. Abbreviated as si.
fn si_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Function Pointers");

  println!(
  "We've talked about how to pass closures to functions; you can also pass regular functions to functions!\n\
  This technique is useful when you want to pass a function you've already defined rather than defining a new closure.\n\
  Functions coerce to the type {0} (with a lowercase {1}), not to be confused with the {2} closure trait.\n\
  The {0} type is called a function pointer.\n\
  Passing functions with function pointers will allow you to use functions as arguments to other functions.\n\n\
  The syntax for specifying that a parameter is a function pointer is similar to that of closures, as shown in Listing 20-28,\n\n\
  See Listing 20-28:{3}, for code samples and complete reading.\n\n\
  Unlike closures, {0} is a type rather than a trait, so we specify {0} as the parameter type directly rather than declaring a \
  generic type parameter with one of the Fn traits as a trait bound.\n\n\
  Function pointers implement all three of the closure traits ({2}, {4}, and {5}), meaning you can always pass a function \
  pointer as an argument for a function that expects a closure.\n\
  It's best to write functions using a generic type and one of the closure traits so that your functions can accept either \
  functions or closures.\n\n\
  That said, one example of where you would want to only accept {0} and not closures is when interfacing with external code that \
  doesn't have closures: C functions can accept functions as arguments, but C doesn't have closures.
  ",
  "fn".bright_yellow().bold(),
  "f".italic().bold(),
  "Fn".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html#listing-20-28".bright_cyan(),
  "FnMut".bright_yellow().bold(),
  "FnOnce".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} The {1} type is called a {2}. Functions coerce to the type {1}.\n\
  {solid_disc} Unlike closures, {1} is a type rather than a trait.\n\
  {solid_disc} Function pointers implement all three of the closure traits ({3}, {4}, and {5}).\n\
  {solid_disc} One example of where you would want to only accept {1} and not closures is when interfacing with external code that \
  doesn't have closures: C functions can accept functions as arguments, but C doesn't have closures.
  ",
  "REMEMBER".bright_white().bold(),
  "fn".bright_yellow().bold(),
  "function pointer".italic().bold(),
  "Fn".bright_yellow().bold(),
  "FnMut".bright_yellow().bold(),
  "FnOnce".bright_yellow().bold(),
  )
}

// Header: Returning Closures. Abbreviated as rc.
fn rc_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Returning Closures");

  println!(
  "Closures are represented by traits, which means you can't return closures directly.\n\
  In most cases where you might want to return a trait, you can instead use the concrete type that implements the trait as the \
  return value of the function.\n\
  However, you can't usually do that with closures because they don't have a concrete type that is returnable; you're not allowed \
  to use the function pointer {0} as a return type if the closure captures any values from its scope, for example.\n\n\
  Instead, you will normally use the impl Trait syntax we learned about in Chapter 10.\n\
  You can return any function type, using {1}, {2}, and {3}.\n\
  For example, the code in Listing 20-32 will compile just fine.\n\n\
  See Listing 20-33:{4}, for code samples.
  ",
  "fn".bright_yellow().bold(),
  "Fn".bright_yellow().bold(),
  "FnOnce".bright_yellow().bold(),
  "FnMut".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html#listing-20-32".bright_cyan()
  );

  println!(
  "{0}\n\n\
  However, as we noted in the “Inferring and Annotating Closure Types” section in Chapter 13:{}, each closure is also its own \
  distinct type.\n\
  If you need to work with multiple functions that have the same signature but different implementations, you will need to use \
  a trait object for them.\n\
  Consider what happens if you write code like that shown in Listing 20-33.\n\n\
  See Listing 20-33:{1}, for code sample, outputs and complete reading.
  ",
  "Closure types".bright_magenta().bold(),
  "https://doc.rust-lang.org/book/ch13-01-closures.html#inferring-and-annotating-closure-types".bright_cyan(),
  );

  println!(
  "{0}\n\n\
  The error message in Listing 20-33 tells us that whenever we return an {1}, Rust creates a unique opaque type, a type where we \
  cannot see into the details of what Rust constructs for us, nor can we guess the type Rust will generate to write ourselves.\n\
  So, even though these functions return closures that implement the same trait, {2}, the opaque types Rust generates for each are \
  distinct.\n\
  (This is similar to how Rust produces different concrete types for distinct async blocks even when they have the same output type, \
  as we saw in “The Pin Type and the Unpin Trait” in Chapter 17:{3}.)\n\
  We have seen a solution to this problem a few times now: We can use a trait object, as in Listing 20-34.\n\n\
  See Listing 20-34:{4}, for code sample\n\n\
  The code will compile just fine.\n\
  For more about trait objects, refer to the section “Using Trait Objects To Abstract over Shared Behavior” in Chapter 18:{5}.
  ",
  "Opaque type".bright_magenta().bold(),
  "impl Trait".bright_yellow().bold(),
  "Fn(i32) -> i32".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch17-05-traits-for-async.html#the-pin-type-and-the-unpin-trait".bright_cyan(),
  "https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html#listing-20-34".bright_cyan(),
  "https://doc.rust-lang.org/book/ch18-02-trait-objects.html".bright_cyan(),
  );  

  println!(
  "{0}\n\n\
  {solid_disc} Closures are represented by traits, which means you can't return closures directly.\n\
  {solid_disc} Each closure is also its own distinct type.\n\
  ",
  "REMEMBER".bright_white().bold(),
  )  
}











