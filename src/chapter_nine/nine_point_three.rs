use owo_colors::OwoColorize;
use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 3];
  subheaders = [
    chapter::SubHeader::new("To panic! or Not to panic!", tpontp_content),
    chapter::SubHeader::new("Guidelines for Error Handling", gfeh_content),
    chapter::SubHeader::new("Custom Types for Validation", ctfv_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Subheader: To panic! or Not to panic!. Abbreviated as tpontp.
fn tpontp_content() {
  // Subheader title.
  menu::subheader_title("To panic! or Not to panic!");

  println!(
    "So, how do you decide when you should call {0} and when you should return {1}? \n\
    When code panics, there's no way to recover. \n\
    You could call {0} for any error situation, whether there's a possible way to \
    recover or not, but then you're making the decision that a situation is \
    unrecoverable on behalf of the calling code. \n\
    When you choose to return a {1} value, you give the calling code options. \n\
    The calling code could choose to attempt to recover in a way that's appropriate \
    for its situation, or it could decide that an {2} value in this case is \
    unrecoverable, so it can call {0} and turn your recoverable error into an \
    unrecoverable one. \n\
    Therefore, returning {1} is a good default choice when you're defining a \
    function that might fail. \n\n\
    In situations such as examples, prototype code, and tests, it’s more appropriate \
    to write code that panics instead of returning a {1}.
  ",
    "panic!".bright_yellow().bold(),
    "Result".bright_yellow().bold(),
    "Err".bright_red().bold()
  );
}

// Subheader: Guidelines for Error Handling. Abbreviated as gfeh.
fn gfeh_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";

  // Subheader title.
  menu::subheader_title("Guidelines for Error Handling");

  println!(
    "It’s advisable to have your code panic when it's possible that your code could \
    end up in a bad state. \n\
    In this context, a bad state is when some assumption, guarantee, contract, \
    or invariant has been broken, such as when invalid values, contradictory values, \
    or missing values are passed to your code—plus one or more of the following: \n\
    {two_spaces}{solid_disc} The bad state is something that is unexpected, as opposed \
    to something that will likely happen occasionally, like a user entering data in \
    the wrong format. \n\
    {two_spaces}{solid_disc} Your code after this point needs to rely on not being \
    in this bad state, rather than checking for the problem at every step. \n\
    {two_spaces}{solid_disc} There’s not a good way to encode this information \
    in the types you use.
  ");

  println!(
    "If someone calls your code and passes in values that don't make sense, it's best \
    to return an error ({0}) if you can so that the user of the library can decide what \
    they want to do in that case. \n\
    However, in cases where continuing could be insecure or harmful, the best choice \
    might be to call {1} and alert the person using your library to the bug in \
    their code so that they can fix it during development. \n\
    Similarly, {1} is often appropriate if you’re calling external code that is \
    out of your control and returns an invalid state that you have no way of fixing. \n\n\
    See: {2}, for more information.
  ",
    "Result".bright_yellow().bold(),
    "panic!".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling".cyan()
  );
}

// Subheader: Custom Types for Validation. Abbreviated as ctfv.
fn ctfv_content() {
  // Subheader title.
  menu::subheader_title("Custom Types for Validation");

  println!(
    "See: {}, for code samples and further information.
  ",
    "https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#custom-types-for-validation".cyan()
  );
}