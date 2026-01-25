use crate::{chapter, menu};
use owo_colors::OwoColorize;


pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 1];
  subheaders = [
    chapter::SubHeader::new("Packages and Crates", pac_content),

  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Subheader: Packages and Crates. Abbreviated as pac.
fn pac_content() {
    let solid_disc = "\u{2022}";
    let two_spaces = "\u{2003}\u{2003}";

    menu::subheader_title("Packages and Crates");

    println!(
      "A {0} is the smallest amount of code that the Rust compiler considers at a time. \n\
      Even if you run {1} rather than {2} and pass a single source code file, the compiler considers that file to be a crate. \n\
      Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.
    ",
      "crate".italic(),
      "rustc".bright_yellow().bold(),
      "cargo".bright_yellow().bold(),
    );

    println!(
      "A crate can come in one of two forms: \n\
      {two_spaces}{solid_disc} a binary crate \n\
      {two_spaces}{solid_disc} or a library crate. \n\n\
      {0} are programs you can compile to an executable that you can run, such as a command line program or a server. \n\
      Each must have a function called {1} that defines what happens when the executable runs. \n\n\
      {2} don't have a {1} function, and they don't compile to an executable.\n\
      Instead, they define functionality intended to be shared with multiple projects. \n\
      Most of the time when Rustaceans say “crate,” they mean library crate, and they use “crate” interchangeably \
      with the general programming concept of a “library.”
    ",
      "Binary crates".italic(),
      "main".bright_yellow().bold(),
      "Library crates".italic(),
    );

    println!(
      "The {0} root is a source file that the Rust compiler starts from and makes up the root module of your crate
    ",
      "crate".italic()
    );

    println!(
      "A {0} is a bundle of one or more crates that provides a set of functionality. \n\
      A package contains a {1} file that describes how to build those crates. \n\
      Cargo is actually a package that contains the binary crate for the command line tool you've been using to build your code. \n\
      The Cargo package also contains a library crate that the binary crate depends on. \n\
      Other projects can depend on the Cargo library crate to use the same logic the Cargo command line tool uses. \n\n\
      A package can contain as many binary crates as you like, but at most only one library crate. \n\
      A package must contain at least one crate, whether that’s a library or binary crate.
    ",
      "package".italic(),
      "Cargo.toml".italic(),
    );

    println!(
      "Cargo follows a convention that {0} is the crate root of a binary crate with the same name as the package. \n\
      Likewise, Cargo knows that if the package directory contains {1}, the package contains a library crate \
      with the same name as the package, and {1} is its crate root. \n\
      Cargo passes the crate root files to {2} to build the library or binary. \n\n\
      A package can have multiple binary crates by placing files in the {3} directory: Each file will be a separate binary crate.
    ",
      "src/main.rs".italic(),
      "src/lib.rs".italic(),
      "rustc".bright_yellow().bold(),
      "src/bin".italic(),
    );

    println!(
      "{}\n\n\
      {solid_disc} {1} is the crate root of a binary crate with the same name as the package \n\
      {solid_disc} {2} is the crate root of a library crate with the same name as the package \n\
      {solid_disc} In Rust, a file is considered a crate.
    ",
      "REMEMBER".bright_white().bold(),
      "src/main.rs".italic(),
      "src/lib.rs".italic(),
    )
}