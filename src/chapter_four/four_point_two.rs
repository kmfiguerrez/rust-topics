use owo_colors::OwoColorize;

pub fn content(title: &str) {
  // rab_content();
  // mr_content();
  // cbtcs_content();
  // imr_content();
  dr_content();
}

// Subheader contents below.

// Subheader: References and borrowing. Abbreviated as rab.
fn rab_content() {
  let solid_disc = "\u{2022}";

  // Subheader title.
  println!("{} \n", "References and Borrowing".bright_blue().bold());

  println!(
    "The issue with returning ownership from functions is that we have to return the \
    borrowed data to the calling function. \n\
    Also it can lead to multiple owners of the same data, which Rust's ownership model \
    prohibits to ensure memory safety. \n\n\
    To solve this, Rust uses a system of references and borrowing. \n\
    A {} is like a pointer in that it’s an address we can follow to access the data \
    stored at that address; that data is owned by some other variable. \n\
    {}\n\
    References allow us to refer to some value without taking ownership of it. \n\
    {3}. \n\
    When we borrow a value, we can use it without taking ownership, and the original \
    owner retains control over the value's lifetime. \n\n\
    See: {2}, for code samples.
  ",
    "reference".italic(),
    "Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.".yellow(),
    "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing".cyan(),
    "Borrowing is the act of creating a reference to a value".bright_white().bold()
  );

  println!(
    "To create a reference, we use the ampersand symbol {0} before a variable name. \n\
    This creates a reference and they allow you to refer to some value without taking \
    ownership of it. \n\n\
    {1}: The opposite of referencing by using {0} is dereferencing, which is accomplished \
    with the dereference operator, {2}.
  ",
    "&".yellow().bold(),
    "Note".red(),
    "*".yellow().bold(),
  );

  println!(
    "In function definition, if you want a function to borrow a value rather than take \
    ownership of it, you specify an {0} before the parameter's type in the function \
    signature. \n\
    That also indicate that the type of the parameter is a reference. \n\
    When the reference stops being used, or the reference goes out of scope, the value it \
    points to will not be dropped because the reference does not have ownership of the value.
  ",
    "&".yellow().bold(),
  );

  println!(
    "{} \n\n\
    {solid_disc} Functions can also have parameters as references, allowing them to borrow values without taking ownership. \n\
  ",
    "REMEMBER!",
  );
}

// Subheader: Mutable References: Abbreviated as mr.
fn mr_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";
  
  // Subheader title.
  println!("{} \n", "Mutable References".bright_blue().bold());

  println!(
    "{0}. We’re not allowed \
    to modify something we have a reference to. \n\
    If we want to modify the value we’re borrowing, we need to create a mutable reference \
    instead of an immutable one. \n\
    To create a mutable reference, we use the {1} syntax. \n\
    This indicates that the reference is mutable, and we can use it to change the value \
    it points to. \n\n\
    See: {2}, for code samples.
  ",
    "Just as variables are immutable by default, so are references".bright_white().bold(),
    "&mut".yellow().bold(),
    "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references".cyan()
  );

  println!(
    "{0}: If you have a mutable reference to a value, \
    you can have no other references to that value within that mutable reference's scope. \n\
    {10}. \n\n\
    Look at this example: \n\n\
    {1} \n\n\
    This code is invalid because we cannot borrow {2} as mutable more than once at a time. \n\
    The first mutable borrow is in {3} and must last until it’s used in the {4}, but \
    between the creation of that mutable reference and its usage, we tried to create \
    another mutable reference in {5} that borrows the same data as {3}. \n\n\
    The restriction preventing multiple mutable references to the same data at the \
    same time allows for mutation but in a very controlled fashion. \n\
    {6} \n\
    The benefit of having this restriction is that Rust can prevent data races at compile time, \
    ensuring memory safety. \n\
    A {7} is similar to a race condition and happens when these three behaviors occur: \n\
    {two_spaces}{solid_disc} Two or more pointers access the same data at the same time. \n\
    {two_spaces}{solid_disc} At least one of the pointers is being used to write to the data. \n\
    {two_spaces}{solid_disc} There’s no mechanism being used to synchronize access to the data. \n\n\
    Data races cause undefined behavior and can be difficult to diagnose and fix when \
    you’re trying to track them down at runtime; {8} \n\n\
    Read more? \n\
    See: {9}
  ",
    "Mutable references have one big restriction".red(),
    "let mut s = String::from(\"hello\");\n\n\
    let r1 = &mut s; // no problem\n\
    let r2 = &mut s; // BIG PROBLEM\n\n\
    println!(\"{r1}\");".bright_yellow(),
    "s".yellow().bold(),
    "r1".yellow().bold(),
    "println!".yellow().bold(),
    "r2".yellow().bold(),
    "It’s something that new Rustaceans struggle with because most languages let you mutate \
    whenever you’d like.".red(),
    "data race".italic(),
    "Rust prevents this problem by refusing \
    to compile code with data races!".bright_green(),
    "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references".cyan(),
    "Note that a reference’s scope starts from where it is introduced and continues through \
    the last time that reference is used".bright_white().bold()
  );

  println!(
    "{} \n\n\
    {solid_disc} References are immutable by default, meaning you cannot modify the value they point to. \n\
    {solid_disc} To allow a function to modify the value it borrows, you can use mutable references. \n\
    {solid_disc} Mutable references are created by using the {} syntax. \n\
    {solid_disc} The rules of no other references (mutable or immutable) while having a mutable reference to the same value prevents a data race.
  ",
    "REMEMBER!",
    "&mut".yellow().bold()
  );
}

// Subheader: Curly Brackets to create scopes. Abbreviated as cbtcs.
// I extracted this section out of the Mutable References section.
fn cbtcs_content() {
  println!(
    "{0} \n\n\
    As always, we can use curly brackets to create a new scope, allowing for multiple \
    mutable references, just not simultaneous ones: \n\n\
    {1} \n\n\
    Read more? \n\
    See: {2}
  ",
    "Curly brackets to create scopes".bright_blue().bold(),
    "let mut s = String::from(\"hello\");\n\n\
    {\n\
    \tlet r1 = &mut s; \n\
    } // r1 goes out of scope here, so we can make a new reference with no problems. \n\n\
    let r2 = &mut s;".bright_yellow(),
    "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references".cyan()
  );
}

// Subheader: Immutable and Mutable references. Abbreviated as imr.
// I extracted this section out of the Mutable References section.
fn imr_content() {
  let solid_disc = "\u{2022}";

  // Subheader title.
  println!("{} \n", "Immutable and Mutable references".bright_blue().bold());

  println!(
    "Rust enforces a similar rule for combining mutable and immutable references. \n\
    This code results in an error: \n\n\
    {} \n\
    We also cannot have a mutable reference while we have an immutable one to the same value. \n\
    Users of an immutable reference don’t expect the value to suddenly change out from under them! \n\
    However, multiple immutable references are allowed because no one who is just reading \
    the data has the ability to affect anyone else’s reading of the data.
  ",
    "let mut s = String::from(\"hello\");\n\n\
    let r1 = &s; // no problem\n\
    let r2 = &s; // no problem\n\
    let r3 = &mut s; // BIG PROBLEM \n\n\
    println!(\"{r1}, {r2}\");
    ".bright_yellow()
  );

  println!(
    "{0}. \n\
    For instance, this code will compile because the last \
    usage of the immutable references is in the {1}, before the mutable reference is introduced: \n\n\
    {2}
  ",
    "Note that a reference’s scope starts from where it is introduced and continues through \
    the last time that reference is used".bright_white().bold(),
    "println!".bright_yellow().bold(),
    "let mut s = String::from(\"hello\");\n\n\
    let r1 = &s; // no problem\n\
    let r2 = &s; // no problem\n\
    println!(\"{r1}, {r2}\");\n\
    // Variables r1 and r2 are not be used after this point. \n\n\
    let r3 = &mut s; // no problem" .bright_yellow()
  );

  println!(
    "The scopes of the immutable references {0} and {1} end after the {2} where they \
    are last used, which is before the mutable reference {3} is created. \n\
    These scopes don’t overlap, so this code is allowed: The compiler can tell that the \
    reference is no longer being used at a point before the end of the scope. \n\n\
    Even though borrowing errors may be frustrating at times, remember that it’s the Rust \
    compiler pointing out a potential bug early (at compile time rather than at runtime) \
    and showing you exactly where the problem is. Then, you don’t have to track down why \
    your data isn’t what you thought it was. \n\n\
    Read more? \n\
    See: {4}
  ",
    "r1".bright_yellow().bold(),
    "r2".bright_yellow().bold(),
    "println!".bright_yellow().bold(),
    "r3".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references".cyan()
  );

  println!(
    "{}! \n\n\
    {solid_disc} Just like you cannot have other references while having a mutable reference to the same value \
    in that mutable reference's scope. \
    We also cannot have a mutable reference while we have an immutable one to the same value \
    within the immutable reference's scope. \n\
    {solid_disc} Rust implements these rules to prevent data races at compile time.
  ",
    "REMEMBER".bright_white().bold()
  );
}

// Subheader: Dangling References. Abbreviated as dr.
fn dr_content() {
  let solid_disc = "\u{2022}";

  // Subheader title.
  println!("{} \n", "Dangling References".bright_blue().bold());

  println!(
    "In languages with pointers, it’s easy to erroneously create a {0}—a pointer \
    that references a location in memory that may have been given \
    to someone else—by freeing some memory while preserving a pointer to that memory. \n\
    In Rust, by contrast, the compiler guarantees that references will never be dangling \
    references: If you have a reference to some data, the compiler will ensure that the \
    data will not go out of scope before the reference to the data does. \n\n\
    See: {1}, for code examples.
  ",
    "dangling pointer".italic(),
    "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references".cyan()
  );

  println!(
    "{} \n\n\
    Let’s recap what we’ve discussed about references: \n\n\
    {solid_disc} At any given time, you can have either one mutable reference or any number of immutable references. \n\
    {solid_disc} Refences must always be valid. \n\
    {solid_disc} A dangling pointer is a pointer that references a memory location that's \
    already been freed and might have been given to someone else.

  ",
    "The Rules of References".bright_white().bold()
  )
}

