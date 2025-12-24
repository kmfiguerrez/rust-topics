use owo_colors::OwoColorize;

pub fn display_contents() {
  // tst_content();
  // ss_content();
  // rrs_content();
  // slas_content();
  // os_content();
  summary_content();
}

// Subheader contents below.

// Subheader: The Slice Type. Abbreviated as tst.
fn tst_content() {
  // Subheader title.
  println!("{} \n", "The Slice Type".bright_blue().bold());

  println!(
    "{0} let you reference a contiguous sequence of elements in a collection. \n\
    A slice is a kind of reference, so it does not have ownership. \n\n\
    See: {1}, for code samples. \n\n\
  ",
    "Slices".italic(),
    "https://doc.rust-lang.org/book/ch04-03-slices.html#listing-4-7".cyan()
  );
}

// Subheader: String Slices. Abbreviated as ss.
fn ss_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";

  // Subheader title.
  println!("{0} \n", "String Slices".bright_blue().bold());

  println!(
    "A {0} is a reference to a contiguous sequence of the elements of a String. \n\n\
    See: {1}, for code samples and diagrams. \n\n\
    To create a string slice, we use the format: \n\
    {2} \n\n\
    Where: \n\
    {two_spaces}{solid_disc} {3} is the first position in the slice \n\
    {two_spaces}{solid_disc} {4} is one more than the last position in the slice \n\n\
    {9}, which corresponds to {4} minus {3}. \n\
    So, in the case of {5}, in the code sample, {6} would be a slice that contains a pointer \
    to the byte at index 6 of {7} with a length value of {8}.
  ",
    "string slice".italic(),
    "https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices".cyan(),
    "&<Some string value>[starting_index..ending_index]".bright_yellow().bold(),
    "starting_index".bright_yellow().bold(),
    "ending_index".bright_yellow().bold(),
    "let world = &s[6..11];".bright_yellow().bold(),
    "world".bright_yellow().bold(),
    "s".bright_yellow().bold(),
    "5".bright_yellow().bold(),
    "Internally, the slice data structure stores the starting position and the length of \
    the slice".bright_white().bold()
  );
}

// Subheader: Rust range syntax. Abbreviated as rrs.
// I extracted this part from the String Slices section.
fn rrs_content() {
  let solid_disc = "\u{2022}";
  // Subheader title.
  println!("{} \n", "Rust's .. range syntax".bright_blue().bold());

  println!(
    "With Rust's {0} range syntax, if you want to start at index 0, you can drop the value \
    before the two periods. In other words, these are equal: \n\n\
    {1} \n\n\
    By the same token, if your slice includes the last byte of the String, you can drop \
    the trailing number. That means these are equal: \n\n\
    {2} \n\n\
    You can also drop both values to take a slice of the entire string. \
    So, these are equal: \n\n\
    {3} \n\
    {4}: String slice range indices must occur at valid UTF-8 character boundaries. \
    If you attempt to create a string slice in the middle of a multibyte character, \
    your program will exit with an error. \n\n\
    The type that signifies “string slice” is written as {5}. \n\n\
    See: {6}, for code updates and error explanation under Figure 4-7.
  ",
    "..".bright_yellow().bold(),
    "let s = String::from(\"hello\"); \n\n\
    let slice = &s[0..2]; \n\
    let slice = &s[..2];
    ".bright_yellow().bold(),
    "let len = s.len(); \n\n\
    let slice = &s[3..len]; \n\
    let slice = &s[3..];
    ".bright_yellow().bold(),
    "let slice = &s[0..len]; \n\
    let slice = &s[..];
    ".bright_yellow().bold(),
    "Note".red(),
    "&str".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices".cyan()
  );

  println!(
    "{0} \n\
    {solid_disc} With rust range syntax, you can leave off the starting index, ending index \
    or both. \n\
    {solid_disc} The type that signifies “string slice” is written as {1}. \n\
    {solid_disc} A string slice is a reference to a contiguous sequence of the elements of a {2}.
  ",
    "REMEMBER".bright_white().bold(),
    "&str".bright_yellow().bold(),
    "String".bright_yellow().bold()
  )
}

// Subheader: String Literals as Slices. Abbreviated as slas.
fn slas_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";

  // Subheader title.
  println!("{} \n", "String Literal as Slices".bright_blue().bold());

  println!(
    "Recall that we talked about string literals being stored inside the binary. Now that \
    we know about slices, we can properly understand string literals: \n\n\
    {} \n\n\
    The type of {1} here is {2}: It’s a slice pointing to that specific point of the binary \
    (Read-Only section of the binary). \
    This is also why string literals are immutable; {2} is an immutable reference. \n\n\
    In code sample: {3}, the type of parameter of the {4} function has been updated \
    from {5} to {2}. So it's now using a string slice. \n\

  ",
    "let s = \"Hello, world!\";".bright_yellow().bold(),
    "s".bright_yellow().bold(),
    "&str".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch04-03-slices.html#listing-4-9".cyan(),
    "first_word".bright_yellow().bold(),
    "&String".bright_yellow().bold()
  );

  println!(
    "Defining a function to take a string slice ({2}) instead of a reference to a {} \
    ({1}) makes our API more general and useful without losing any functionality: \n\
    {two_spaces}{solid_disc} We can pass slices of {0}, whether partial or whole. \n\
    {two_spaces}{solid_disc} We can pass references to {0}, because the compiler \
    coerces {1} to {2} using {3}. You can also think of {1} as equivalent \
    to whole slices of {0}. \n\
    {two_spaces}{solid_disc} We can pass slices of string literals, whether partial or \
    whole. \n\
    {two_spaces}{solid_disc} We can pass string literals because they are string \
    slices already. \n\n\
  ",
    "String".bright_yellow().bold(),
    "&String".bright_yellow().bold(),
    "&str".bright_yellow().bold(),
    "Deref coercion".italic()
  );

  println!(
    "{} \n\n\
    {solid_disc} Slices are references, the pointers (memory address) they get depends on \
    the data type. \n\
    {solid_disc} Internally, the slice data structure stores the starting position (byte index) \
    and the length of the slice. \n\
    {solid_disc} {1} references and slices of {1}, reference data on the heap. \n\
    {solid_disc} String literals and slices of literals reference data in Read-Only \
    section of the binary (Mapped to Read-Only memory). \n\
    {solid_disc} Since slices are references, you can think of a {1} reference ({2}) \
    equivalent to a whole slice of {1} ({3}). Internally Rust uses {4}. \n\
    {solid_disc} So, you can think of String references, String slices, string literals \
    and slices of string literal of the same type. The difference is where the data they \
    point to lives.
  ",
    "REMEMBER".bright_white().bold(),
    "String".bright_yellow().bold(),
    "&String".bright_yellow().bold(),
    "&String[..]".bright_yellow().bold(),
    "Deref coercion".italic()
  );
}

// Subheader: Other Slices. Abbreviated as os.
fn os_content() {
  // Subheader title.
  println!("{} \n", "Other Slices".bright_blue().bold());

  println!(
    "String slices, as you might imagine, are specific to strings. But there’s a \
    more general slice type too. Consider this array: \n\n\
    {} \n\n\
    Just as we might want to refer to part of a string, we might want to refer to part \
    of an array. We’d do so like this: \n\n\
    {} \n\n\
    This slice has the type {2}. {3}. You’ll use this kind of slice for all \
    sorts of other collections.
  ",
    "let a = [1, 2, 3, 4, 5];".bright_yellow().bold(),
    "let a = [1, 2, 3, 4, 5]; \n\n\
    let slice = &a[1..3]; \n\n\
    assert_eq!(slice, &[2, 3]);".bright_yellow().bold(),
    "&[i32]".bright_yellow().bold(),
    "It works the same way as string slices do, by storing \
    a reference to the first element (byte index) and a length".bright_white().bold()
  );

}

// Subheader: Summary.
fn summary_content() {
  // Subheader title.
  println!("{} \n", "Summary".bright_blue().bold());

  println!(
    "The concepts of ownership, borrowing, and slices ensure memory safety in Rust \
    programs at compile time. \n\
    The Rust language gives you control over your memory usage in the same way as \
    other systems programming languages. \n\
    But having the owner of data automatically clean up that data when the owner goes \
    out of scope means you don’t have to write and debug extra code to get this control.
  ");
}


