use owo_colors::OwoColorize;
use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 1];
  subheaders = [
    chapter::SubHeader::new("Seperating Modules into Different Files", smidf_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheader contents below.

// Subheader: Seperating Modules into Different Files. Abbreaviated as smidf.
fn smidf_content() {
    menu::subheader_title("Seperating Modules into Different Files");

    println!("\
        When modules get large, you might want to move their definitions to a separate file \
        to make the code easier to navigate. \n\
        Rust wants to extract modules into files instead of having all the modules defined \
        in the crate root file. \n\
        Remember that the crate roor files are {0} and {1}.\n\n\
        See: {2}, for code sample.
    ",
    "src/lib.rs".italic(),
    "src/main.rs".italic(),
    "https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html#listing-7-21".cyan()
    );

    println!("\
        To define a module in another file, you use the {} keyword in the parent module \
        followed by a semicolon instead of a block containing the module contents. \n\
        For example, to define a module named sound in its own file, you would write \
        {1} in the parent module. \n\
    ",
    "mod".green().bold(),
    "mod sound;".green().bold()
    );

    println!("\
        Note that you only need to load a file using a {0} declaration once in your module tree. \n\
        Once the compiler knows the file is part of the project (and knows where in the module tree \n\
        the code resides because of where you’ve put the {0} statement), other files in your project \n\
        should refer to the loaded file’s code using a path to where it was declared, as covered in  \n\
        the “Paths for Referring to an Item in the Module Tree” section. \n\
        In other words, {0} is not an “include” operation that you may have seen in other programming languages.
    ",
    "mod".green().bold()
    );

    println!("\
        In the code sample in Listing 7-21, a module {0} is declared in the {4}. \n\
        which is the crate root file for a library crate that has an implicit module named {1}. \n\
        The {0} module has a child module named hosting which in turn has a function named {2}. \n\n\
        The {0} module is extracted in its own file named {3}. \n\
        Next, rust wants to extract {5} module to its own file. \n\
        The process is a bit different because {5} is a child module of {0}, \
        not of the root module. \n\
        We’ll place the file for {5} in a new directory that will be named \
        for its ancestors in the module tree, in this case {6}. \n\
        To start moving {5}, we change {3} to contain only the declaration of the \
        {5} module: {7} \n\
        Then we create a {6} directory and a {8} file to \
        contain the definitions made in the {5} module: {9}
    ",
    "front_of_house".green().bold(),
    "crate".green().bold(),
    "add_to_waitlist".green().bold(),
    "src/front_of_house.rs".italic(),
    "src/lib.rs".italic(),
    "hosting".green().bold(),
    "src/front_of_house".italic(),
    "pub mod hosting;".green().bold(),
    "hosting.rs".italic(),
    "pub fn add_to_waitlist() {}".green().bold()
    );

    println!("\
        If we instead put {0} in the src directory, the compiler would expect the \
        {0} code to be in a {1} module declared in the crate root, \n\
        and not declared as a child of the {2} module. \n\
        The compiler’s rules for which files to check for which modules’ code mean the \
        directories and files more closely match the module tree.
    ",
    "hosting.rs".italic(),
    "hosting".green().bold(),
    "front_of_house".green().bold()
    );

    println!("\
        We’ve moved each module’s code to a separate file, and the module tree remains the same. \n\
        The function calls in {} will work without any modification, even though the \
        definitions live in different files.\n\
        This technique lets you move modules to new files as they grow in size.\n\n\
        Note that the {1} statement in {2} also \n\
        hasn’t changed, nor does {3} have any impact on what files are compiled as part of the crate. \n\
        The {4} keyword declares modules, and Rust looks in a file with the same name as the module for \
        the code that goes into that module.
    ",
    "eat_at_restaurant();".green().bold(),
    "pub use crate::front_of_house::hosting".green().bold(),
    "src/lib.rs".italic(),
    "use".green().bold(),
    "mod" .green().bold()
    );

    println!("\
        {}\n\n\
        Rust lets you split a package into multiple crates and a crate into modules so you \
        can refer to items defined in one module from another module. \n\
        You can do this by specifying absolute or relative paths. \n\
        These paths can be brought into scope with a {} statement so you can use a \
        shorter path for multiple uses of the item in that scope. \n\
        Module code is private by default, but you can make definitions public by adding the {} keyword.
    ",
    "Summary".blue().bold(),
    "use".green().bold(),
    "pub".green().bold()
    );
}
















