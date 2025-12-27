use owo_colors::OwoColorize;

pub fn display_contents() {
  // tmcfc_content();
  // ptbtv_content();
  // tomp_content();
  // mae_content();
  capaup_content();
}

// Subheader contents below.

// Subheader The match Control Flow Construct. Abbreviated tmcfc.
fn tmcfc_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";  
  // Subheader title.
  println!("{} \n", "The match Control Flow Construct".bright_blue().bold());

  println!(
    "Rust has an extremely powerful control flow construct called {0} that allows you \
    to compare a value against a series of patterns and then execute code based on which \
    pattern matches. \n\
    Patterns can be made up of literal values, variable names, wildcards, and many other \
    things; \n\
    The power of {0} comes from the expressiveness of the patterns and the fact that \
    the compiler confirms that all possible cases are handled. \n\
    Values go through each pattern in a match, and at the first pattern the value “fits,” \
    the value falls into the associated code block to be used during execution. \n\n\
    See: {1}, for code samples.
  ",
    "match".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch06-02-match.html#listing-6-3".cyan()
  );

  println!(
    "The {0} expression is very similar to an {1} expression. The big difference is the \
    expression (single value) that follows the {0} and {1} keywords. \n\
    With {1}, the condition needs to evaluate to a Boolean value, but in the {0} construct \
    it can be any type.
  ",
    "match".bright_yellow().bold(),
    "if".bright_yellow().bold(),
  );

  println!(
    "{} \n\n\
    The {1} construct takes the form of the {1} keyword followed by an expression (single value), \
    then the {1} arms inside curly brackets. \n\
    An arms has two parts: \n\
    {two_spaces}{solid_disc} A pattern \n\
    {two_spaces}{solid_disc} Some code \n\n\
    The {2} seperates the pattern and the code to run. \n\
    Each arm is separated from the next with a comma. 
  ",
    "THE MATCH EXPRESSION STRUCTURE".bright_magenta().bold(),
    "match".bright_yellow().bold(),
    "=>".bright_yellow().bold(),
  );

  println!(
    "When the {0} expression executes, it compares the resultant value against the \
    pattern of each arm, in order. \n\
    If a pattern matches the value, the code associated with that pattern is executed. \n\
    If that pattern doesn’t match the value, execution continues to the next arm. \n\n\
    The code associated with each arm is an expression, and the resultant value of the \
    expression in the matching arm is the value that gets returned for the entire \
    {0} expression. \n\n\
    We don’t typically use curly brackets if the match arm code is short (if an arm just \
    returns a value). \n\
    If you want to run multiple lines of code in a match arm, you must use curly brackets, \
    and the comma following the arm is then optional.
  ",
    "match".bright_yellow().bold(),
  );
}

// Subheader: Patterns That Bind to Values. Abbreviated as ptbtv.
fn ptbtv_content() {
  // Subheader title.
  println!("{} \n", "Patterns That Bind to Values".bright_blue().bold());

  println!(
    "Another useful feature of match arms is that they can bind to the parts of the values \
    that match the pattern. \n\
    This is how we can extract values out of enum variants. \n\n\
    See: {}, for code samples.
  ",
    "https://doc.rust-lang.org/book/ch06-02-match.html#listing-6-4".cyan()
  );
}

// Subheader: The Option<T> match Pattern. Abbreviated as tomp.
fn tomp_content() {
  // Subheader title.
  println!("{} \n", "The Option<T> match Pattern".bright_blue().bold());

  println!(
    "We can also get the inner {0} value out of the {1} case (variant) when using \
    {2} using match. \n\n\
    See: {3}, for code samples.
  ",
    "T".bright_yellow().bold(),
    "Some".bright_yellow().bold(),
    "Option<T>".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch06-02-match.html#listing-6-5".cyan()
  );

  println!(
    "Combining {} and enums is useful in many situations. \n\
    You’ll see this pattern a lot in Rust code: {0} against an enum, bind a variable to \
    the data inside, and then execute code based on it. \n\
    It’s a bit tricky at first, but once you get used to it, you’ll wish you had it in all \
    languages. \n\
    It’s consistently a user favorite.
  ",
    "match".bright_yellow().bold()
  );
}

// Subheader: Matches Are Exhaustive. Abbreviated as mae.
fn mae_content() {
  // Subheader title.
  println!("{} \n", "Matches Are Exhaustive".bright_blue().bold());

  println!(
    "There’s one other aspect of match we need to discuss: The arms’ patterns must cover \
    all possibilities. \n\n\
    See: {}, for code samples.
  ",
    "https://doc.rust-lang.org/book/ch06-02-match.html#matches-are-exhaustive".cyan()
  );

  println!(
    "Rust knows if we don't cover every possible case and even knows which pattern we \
    forgot! \n\
    Matches in Rust are {0}: We must exhaust every last possibility in order for \
    the code to be valid. \n\
    Especially in the case of {1}, when Rust prevents us from forgetting to \
    explicitly handle the {2} case, it protects us from assuming that we have a value \
    when we might have null, thus making the billion-dollar mistake discussed earlier \
    impossible.
  ",
    "exhaustive".italic(),
    "Option<T>".bright_yellow().bold(),
    "None".bright_yellow().bold(),
  );
}

// Subheader: Catch-All Patterns and the _ Placeholder. Abbreviated capaup.
fn capaup_content() {
  let solid_disc = "\u{2022}";

  // Subheader title.
  println!("{} \n", "Catch-All Patterns and the _ Placeholder".bright_blue().bold());

  println!(
    "Using enums, we can also take special actions for a few particular values, but \
    for all other values take one default action. \n\n\
    See: {}, for code samples. \n\n\
    You can name a variable for all other values to take one default action.
  ",
    "https://doc.rust-lang.org/book/ch06-02-match.html#catch-all-patterns-and-the-_-placeholder".cyan()
  );

  println!(
    "Even though you don't list all the possible values of the expression being matched, \
    last pattern will match all values not specifically listed. \n\
    This catch-all pattern meets the requirement that {} must be exhaustive. \n\
    {} \n\
    {}, so Rust will \
    warn us if we add arms after a catch-all!
  ",
    "match".bright_yellow().bold(),
    "Note that we have to put the catch-all arm last because the patterns are evaluated \
    in order.".bright_white().bold(),
    "If we had put the catch-all arm earlier, the other arms would never run".red()
  );

  println!(
    "{} \n\n\
    Rust also has a pattern we can use when we want a catch-all but don’t want to use the \
    value in the catch-all pattern: {} is a special pattern that matches any value and \
    does not bind to that value. \n\
    This tells Rust we aren’t going to use the value, so Rust won’t warn us about an \
    unused variable. \n\
    This also meets the exhaustiveness requirement because we’re explicitly ignoring all \
    other values in the last arm; we haven’t forgotten anything.
  ",
    "THE _ SPECIAL PATTERN".bright_magenta().bold(),
    "_".bright_yellow().bold()
  );

  println!(
    "{} \n\n\
    We use the unit value (the empty tuple type), if we don't want to run some code in \
    the catch-all arm. Paired with the _ pattern. \n\
    Like this: {}
  ",
    "Do nothing in the catch-all arm".bright_magenta().bold(),
    "_ => ()".bright_yellow().bold()
  );

  println!(
    "{} \n\n\
    {solid_disc} Put the catch-all arm last.
  ",
    "REMEMBER".bright_white().bold()
  );
}


