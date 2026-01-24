use owo_colors::OwoColorize;
use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 6];
  subheaders = [
    chapter::SubHeader::new("Recoverable Errors with Result", rewr_content),
    chapter::SubHeader::new("Matching on Different Errors", mode_content),
    chapter::SubHeader::new("Shortcuts for Panic on Error", sfpoe_content),
    chapter::SubHeader::new("Propagating Errors", pe_content),
    chapter::SubHeader::new("The ? Operator Shortcut", tqmos_content),
    chapter::SubHeader::new("Where to Use the ? Operator", wtutqmo_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Subheader: Recoverable Errors with Result. Abbreviated as rewr.
fn rewr_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";

// Subheader title.
  menu::subheader_title("Recoverable Errors with Result");

  println!(
    "Most errors aren’t serious enough to require the program to stop entirely. \n\
    Sometimes when a function fails, it’s for a reason that you can easily interpret \
    and respond to. \n\
    For example, if you try to open a file and that operation fails because the file \
    doesn’t exist, you might want to create the file instead of terminating the process.
  ");

  println!(
    "In these situations, Rust has the {0} enum that is defined in the standard library. \
    It has two variants, {2} and {3}, which are used to encode error handling information. \
    \n\n\
    {1}
    \n\
    The {2} variant indicates that the operation was successful and contains a value \
    that represents the success. \n\
    The {3} variant indicates that the operation failed and contains an error value \
    that represents the reason for the failure. \n\
  ",
    "Result".bright_yellow().bold(),
    "enum Result<T, E> {\n  Ok(T), \n  Err(E), \n}".bright_yellow().bold(),
    "Ok".bright_green().bold(),
    "Err".bright_red().bold(),
  );

  println!(
    "The {0} and {1} are generic type parameters: \n\
    {two_spaces}{solid_disc} {0} is the type of the value that will be returned in the success case within the {2} variant. \n\
    {two_spaces}{solid_disc} {1} represents the type of the error that will be returned in a failure case within the {3} variant. \n\n\
    Because {4} has these generic type parameters, we can use the {4} type and the functions defined on it in many different \
    situations where the success value and error value we want to return may differ.
  ",
    "T".bright_yellow().bold(),
    "E".bright_yellow().bold(),
    "Ok".bright_green().bold(),
    "Err".bright_red().bold(),
    "Result".bright_yellow().bold()
  );

  println!(
    "See: {}, for code samples and further explanation.
  ",
    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#listing-9-3".cyan()
  );

  println!(
    "{} \n\n\
    {solid_disc} Note that, like the {1} enum, the {2} enum and its variants have been brought into scope by the prelude, \
    so we don’t need to specify {3} before the {4} and {5} variants in the match arms.
  ",
    "REMEMBER".bright_white().bold(),
    "Option".bright_yellow().bold(),
    "Result".bright_yellow().bold(),
    "Result::".bright_yellow().bold(),
    "Ok".bright_green().bold(),
    "Err".bright_red().bold()
  );

}

// Subheader: Matching on Different Errors. Abbreviated as mode.
fn mode_content() {
  // Subheader title.
  menu::subheader_title("Matching on Different Errors");

  println!("\
    When you call a function or code that returns a {0} value, you’ll often want to take different \
    actions for different kinds of errors that might have occurred. \n\
    You can use a match expression to handle each error variant differently, as shown in \
    Listing 9-6. \n\n\
    See: {1}, for code samples and further explanation.
  ",
    "Result".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#matching-on-different-errors".cyan()
  )
}

// Subheader: Shortcuts for Panic on Error. Abbreviated as sfpoe.
fn sfpoe_content() {
  // Subheader title.
  menu::subheader_title("Shortcuts for Panic on Error");

  println!(
    "Using {0} works well enough, but it can be a bit verbose and doesn’t always communicate intent well. \n\
    The {1} type has many helper methods defined on it to do various, more specific tasks. \n\n\
    {10} \n\n\
    The {2} method is a shortcut method implemented just like the {0} expression. \n\
    If the {3} value is the {4} variant, {2} will return the value inside the {4}. If the {3} is the {5} variant, \
    {2} will call the {8} macro for us. \n\n\
    Here is an example of {2} in action: \n\n\
    {6} \n\n\
    If we run this code without a {7} file, we’ll see an error message from the {8} call that the {2} method makes: \n\
    {9}
  ",
    "match".bright_yellow().bold(),
    "Result<T, E>".bright_yellow().bold(),
    "unwrap".bright_yellow().bold(),
    "Result".bright_yellow().bold(),
    "Ok".bright_green().bold(),
    "Err".bright_red().bold(),
    "use std::fs::File; \n\
    fn main() { \n\
    \u{2003}\u{2003}let f = File::open(\"hello.txt\").unwrap(); \n\
    }".bright_yellow().bold(),
    "hello.txt".italic(),
    "panic!".bright_yellow().bold(),
    "thread 'main' panicked at src/main.rs:4:49:\n\
    called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: \"No such file or directory\"}
    ".bright_red(),
    "UNWRAP METHOD".bright_magenta().bold()
  );

  println!(
    "{} \n\n\
    The {1} method lets us also choose the {2} error message. \n\
    Using {1} instead of {3} and providing good error messages can \
    convey your intent and make tracking down the source of a panic easier. \n\n\
    The syntax of {1} looks like this: \n\n\
    {4} \n\n\
    We use {1} in the same way as {3}: to return the file handle or call the {2} macro. \n\
    The error message used by {1} in its call to {2} will be the parameter that we pass to {1}, \n\
    rather than the default {2} message that {3} uses. \n\
    Here’s what it looks like: \n\
    {5} \n\n\
    In production-quality code, most Rustaceans choose {1} rather than {3} and give more context about why the operation is \
    expected to always succeed. That way, if your assumptions are ever proven wrong, you have more information to use in debugging.
  ",  
    "EXPECT METHOD".bright_magenta().bold(),
    "expect".bright_yellow().bold(),
    "panic!".bright_yellow().bold(),
    "unwrap".bright_yellow().bold(),
    "use std::fs::File; \n\n\
    fn main() {\n\
    \u{2003}\u{2003}let greeting_file = File::open(\"hello.txt\")\n\
    \u{2003}\u{2003}\u{2003}\u{2003}.expect(\"hello.txt should be included in this project\");\n\
    }".bright_yellow().bold(),
    "thread 'main' panicked at src/main.rs:5:10:\n\
    hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }".bright_red()
  );

}

// Subheader: Propagating Errors. Abrreviated as pe.
fn pe_content() {
  // Subheader title.
  menu::subheader_title("Propagating Errors");

  println!(
    "When a function’s implementation calls something that might fail, instead of \
    handling the error within the function itself, you can return the error to the \
    calling code so that it can decide what to do. \n\
    This is known as {0} the error and gives more control to the calling code, \
    where there might be more information or logic that dictates how the error should \
    be handled than what you have available in the context of your code. \n\n\
    See: {1}, for code samples and further explanation.
  ",
    "propagating".italic(),
    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#listing-9-6".cyan()
  );


}

// Subheader: The ? Operator Shortcut. Abbreviated as tqmos.
fn tqmos_content() {
  let solid_disc = "\u{2022}";

  // Subheader title.
  menu::subheader_title("The ? Operator Shortcut");


  println!(
    "Using {0} to propagate errors is such a common pattern that Rust has special syntax \
    to make it easier: the {1} operator. \n\
    The {1} placed after a {2} value is defined to work in almost the same way as the \
    {0} expressions that we defined to handle the {2} values in Listing 9-6: {5} \n\n\
    Note that the {1} operator can only be used inside functions to propagate something. \n\n\
    When we use the {1} operator on a {2} value, the following happens: \n\
    If the value of the {2} is an {3}, the value inside the {3} will get returned \
    from this expression, and the program will continue. \n\
    If the value is an {4}, the {4} will be returned from the whole function as if we \
    had used the {6} keyword so that the error value gets propagated to the calling code. \n\n\
    See Listing 9-7: {7}, for code samples and further explanation.
  ",
    "match".bright_yellow().bold(),
    "?".bright_yellow().bold(),
    "Result".bright_yellow().bold(),
    "Ok".bright_green().bold(),
    "Err".bright_red().bold(),
    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#listing-9-6".cyan(),
    "return".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#listing-9-7".cyan()
  );

  println!(
    "{} \n\n\
    There is a difference between what the {1} expression from Listing 9-6 does and what the {2} \
    operator does: Error values that have the {2} operator called on them go through the {3} function, \
    defined in the {4} trait in the standard library, which is used to convert values from one type into another. \n\
    When the {2} operator calls the {3} function, the error type received is converted into the error \
    type defined in the return type of the current function. \n\
    This is useful when a function returns one error type to represent all the ways a function might fail, \
    even if parts might fail for many different reasons. \n\n\
    For example, we could change the {5} function in Listing 9-7 to return a custom \
    error type named {6} that we define. \n\
    If we also define {7} for {6} to construct an instance of {6} from an {8}, \
    then the {2} operator calls in the body of {5} will call {3} and convert the error types without \
    needing to add any more code to the function. \n\n\
    This conversion is necessary because the error type returned from the called function \
    might be different from the error type defined in the return type of the current function. \n\
    The {2} operator takes care of this conversion for us by calling the {3} function behind the scenes. \n\n\
    For the {2} operator to perform this conversion, the error type returned from the called function \
    must implement the {3} trait for the error type defined in the return type of the current function. \n\
    Fortunately, the standard library provides implementations of the {3} trait for many common error \
    types, so this conversion often happens automatically. \n\
    Most of the time, you won’t even notice that this conversion is happening; the {2} operator will just work. \n\n\
    See: {9}, for code samples and further explanation.
  ",
    "THE from FUNCTION THE ? OPERATOR USES".bright_magenta().bold(),
    "match".bright_yellow().bold(),
    "?".bright_yellow().bold(),
    "from".bright_yellow().bold(),
    "From".bright_yellow().bold(),
    "read_username_from_file".bright_yellow().bold(),
    "OurError".bright_yellow().bold(),
    "impl From<io::Error>".bright_yellow().bold(),
    "io::Error".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#listing-9-7".cyan()
  );

  println!(
    "{} \n\n\
    The {1} operator eliminates a lot of boilerplate and makes this function’s implementation simpler. We could even \
    shorten code further by chaining method calls immediately after the {1}, as shown in Listing 9-8: {2}.
  ",
    "METHOD CHAINING WITH THE ? OPERATOR".bright_magenta().bold(),
    "?".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#listing-9-8".cyan()
  );
  
  println!(
    "{} \n\n\
    Reading a file into a string is a fairly common operation, so the standard library provides the convenient \
    {} function that opens the file, creates a new {2}, reads the contents of the file, puts \
    the contents into that {2}, and returns it. \n\n\
    See: {3}, for code samples and further explanation.
  ",
    "THE fs::read_to_string FUNCTION".bright_magenta().bold(),
    "fs::read_to_string".bright_yellow().bold(),
    "String".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#listing-9-9".cyan()
  );

  println!(
    "{} \n\n\
    {solid_disc} The {1} operator can only be used in a function that returns \
    {2}, {3}, or another type that implements {4}. \n\
    {solid_disc} So you can also use the {1} operator on values that return {3} \
    and types the implement {4} in functions that return the same type respectively.
  ",
    "REMEMBER".bright_white().bold(),
    "?".bright_yellow().bold(),
    "Result".bright_yellow().bold(),
    "Option".bright_yellow().bold(),
    "FromResidual".bright_yellow().bold()
  );
}

// Subheader: Where to Use the ? Operator. Abbreviated as wtutqmo.
fn wtutqmo_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";

  // Subheader title.
  menu::subheader_title("Where to Use the ? Operator");

  println!(
    "The {0} operator can only be used in functions whose return type is compatible \
    with the value the {0} is used on. \n\
    This is because the {0} operator is defined to perform an early return of a value \
    out of the function, in the same manner as the {1} expression we defined in \
    Listing 9-6. \n\n\
    Listing 9-6: {2} \n\n\
    In Listing 9-6, the {1} was using a {3} value, and the early return arm \
    returned an {4} value. The return type of the function has to be a {3} so \
    that it's compatible with this return.
  ",
    "?".bright_yellow().bold(),
    "match".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#listing-9-6".cyan(),
    "Result".bright_yellow().bold(),
    "Err(e)".bright_red().bold()
  );

  println!(
    "{0} \n\n\
    In Listing 9-10, let's look at the error we'll get if we use the {1} operator in a \
    {2} function with a return type that is incompatible with the type of the \
    value we use {1} on. \n\n\
    See Listing 9-10: {3}, for code samples and further explanation. 
  ",
    "USING THE ? OPERATOR WITH AN INCOMPATIBLE RETURN TYPE".bright_magenta().bold(),
    "?".bright_yellow().bold(),
    "main".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#listing-9-10".cyan()
  );

  println!(
    "The error in listing 9-10 points out that we're only allowed to use the {0} operator \
    in a function that returns {1}, {2}, or another type that implements {3}. \n\n\
    To fix the error, you have two choices: \n\
    {two_spaces}{solid_disc} One choice is to change the return type of your function \
    to be compatible with the value you're using the {0} operator on as long as you have \
    no restrictions preventing that. \n\
    {two_spaces}{solid_disc} The other choice is to use a {4} or one of the \
    {5} methods to handle the {5} in whatever way is appropriate.
  ",
    "?".bright_yellow().bold(),
    "Result".bright_yellow().bold(),
    "Option".bright_yellow().bold(),
    "FromResidual".bright_yellow().bold(),
    "match".bright_yellow().bold(),
    "Result<T, E>".bright_yellow().bold()
  );

  println!(
    "The error message also mentioned that {0} can be used with {1} values as well. \n\
    As with using {0} on {2}, you can only use {0} on {3} in a function that \
    returns an {3}. \n\
    The behavior of the {0} operator when called on an {1} is similar to its \
    behavior when called on a {4}: If the value is {5}, the {5} will be \
    returned early from the function at that point. \n\
    If the value is {6}, the value inside the {6} is the resultant value of the \
    expression, and the function continues. \n\n\
    See listing 9-11: {7}, for code samples and further explanation.
  ",
    "?".bright_yellow().bold(),
    "Option<T>".bright_yellow().bold(),
    "Result".bright_yellow().bold(),
    "Option".bright_yellow().bold(),
    "Result<T, E>".bright_yellow().bold(),
    "None".bright_red().bold(),
    "Some".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#listing-9-11".cyan()
  );

  println!(
    "{} \n\n\
    Note that you can use the {1} operator on a {2} in a function that returns \
    {2}, and you can use the {1} operator on an {3} in a function that \
    returns {3}, but you can't mix and match. \n\
    The {1} operator won’t automatically convert a {2} to an {3} or vice versa; \
    in those cases, you can use methods like the {4} method on {2} or the \
    {5} method on {3} to do the conversion explicitly.
  ",
    "MIX N MATCHING ISN'T ALLOWED".bright_magenta().bold(),
    "?".bright_yellow().bold(),
    "Result".bright_yellow().bold(),
    "Option".bright_yellow().bold(),
    "ok".bright_yellow().bold(),
    "ok_or".bright_yellow().bold()
  );

  println!(
    "{} \n\n\
    So far, all the {1} functions we've used return {2}. \n\
    The {1} function is special because it's the entry point and exit point of an \
    executable program, and there are restrictions on what its return type can be \
    for the program to behave as expected. \n\n\
    Luckily, {1} can also return a {3}. \n\
    Listing 9-12 changed the return type of {1} to be {4} and added a return value \
    {5} to the end. \n\n\
    See listing 9-12: {6}, for code samples and further explanation.
  ",
    "USING ? IN MAIN FUNCTION".bright_magenta().bold(),
    "main".bright_yellow().bold(),
    "()".bright_yellow().bold(),
    "Result<(), E>".bright_yellow().bold(),
    "Result<(), Box<dyn Error>>".bright_yellow().bold(),
    "Ok(())".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#listing-9-12".cyan()
  );

  println!(
    "The {0} type is a trait object. \n\
    For now, you can read {0} to mean “any kind of error.” \n\
    Using {1} on a {2} value in a {3} function with the error type \
    {0} is allowed because it allows any {4} value to be returned early. \n\
    Even though the body of the in Listing 9-12 {3} function will only ever \
    return errors of type {5}, by specifying {0}, this signature will continue \
    to be correct even if more code that returns other errors is added to the body of \
    {3}.
  ",
    "Box<dyn Error>".bright_yellow().bold(),
    "?".bright_yellow().bold(),
    "Result".bright_yellow().bold(),
    "main".bright_yellow().bold(),
    "Err".bright_red().bold(),
    "std::io::Error".bright_yellow().bold()
  );

  println!(
    "When a {0} function returns a {1}, the executable will exit with a \
    value of 0 if {0} returns {2} and will exit with a nonzero value if {0} \
    returns an {3} value. \n\
    Executables written in C return integers when they exit: Programs that \
    exit successfully return the integer {4}, and programs that error return some \
    integer other than {4}. \n\
    Rust also returns integers from executables to be compatible with this convention.
  ",
    "main".bright_yellow().bold(),
    "Result<(), E>".bright_yellow().bold(),
    "Ok(())".bright_yellow().bold(),
    "Err".bright_red().bold(),
    "0".bright_yellow().bold()
  );

  println!(
    "The {0} function may return any types that implement the \
    {1} trait, which contains a function {2} that returns an {3}. \n\
    Consult the standard library documentation for more information on implementing \
    the {4} trait for your own types.
  ",
    "main".bright_yellow().bold(),
    "std::process::Termination".bright_yellow().bold(),
    "report".bright_yellow().bold(),
    "ExitCode".bright_yellow().bold(),
    "Termination".bright_yellow().bold()
  );
}
