use owo_colors::OwoColorize;
use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 4];
  subheaders = [
    chapter::SubHeader::new("Paths for Referring to an Item in the Module Tree", primt_content),
    chapter::SubHeader::new("Exposing Paths with the pub Keyword", epwpk_content),
    chapter::SubHeader::new("Best Practices for Packages with a Binary and a Library", bpfpwbal_content),
    chapter::SubHeader::new("Starting Relative Paths with super", srpws_content)
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Subheader: Paths for Referring to an Item in the Module Tree. Abbreviated as primt.
fn primt_content() {
    let solid_disc = "\u{2022}";
    let two_spaces = "\u{2003}\u{2003}";

    menu::subheader_title("Paths for Referring to an Item in the Module Tree");

    println!("\
    A path can take two forms: \n\
    {two_spaces}{solid_disc} An {0} is the full path starting from a crate root; \n\
    {two_spaces}{two_spaces}for code from an external crate, the absolute path begins with the crate name, \n\
    {two_spaces}{two_spaces}and for code from the current crate, it starts with the literal {1}. \n\
    {two_spaces}{solid_disc} A {2} starts from the current module and uses {3}, \
    or an identifier in the current module. \n\n\
    Both absolute and relative paths are followed by one or more identifiers separated by double colons {4}.
    ",
    "absolute path".italic(),
    "crate".green().bold(),
    "relative path".italic(),
    "self, super".green().bold(),
    "(::)".green().bold()
    );

    println!("\
    If items are defined in the same crate, we can use the {0} keyword \
    to start an absolute path. \n\
    We then include each of the successive modules until we make our way to \
    to the desired item. \n\
    Using the {0} name to start from the crate root is like using / to start from the filesystem root in your shell. \n\
    Starting with a module name means that the path is relative.
    ",
    "crate".green().bold(),
    );

    println!("\
    Choosing whether to use a relative or absolute path is a decision you’ll make based on your project, \n\
    and it depends on whether you’re more likely to move item definition code separately from or together \
    with the code that uses the item. \n\
    Our preference in general is to specify absolute paths because it’s more likely \
    we’ll want to move code definitions and item calls independently of each other.
    ");

    println!("\
    In Rust, all items (functions, methods, structs, enums, modules, and constants) \
    are private to parent modules by default. \n\
    If you want to make an item like a function or struct private, you put it in a module.
    ");

    println!("\
    Items in a parent module can’t use the private items inside child modules, but items in \
    child modules can use the items in their ancestor modules. \n\
    This is because child modules wrap and hide their implementation details, \
    but the child modules can see the context in which they’re defined. \n\
    However, Rust does give you the option to expose inner parts of child modules’ \
    code to outer ancestor modules by using the pub keyword to make an item public.
    ");
}

// Subheader: Exposing Paths with the pub Keyword. Abbreviated as epwpk.
fn epwpk_content() {
    menu::subheader_title("Exposing Paths with the pub Keyword");

    println!("\
    Making the module public doesn’t make its contents public. \n\
    The {} keyword on a module only lets code in its ancestor modules refer to it, \
    not access its inner code. \n\
    Because modules are containers, there’s not much we can do by only making the module public; \n\
    we need to go further and choose to make one or more of the items within the module public as well. \n\
    The privacy rules apply to structs, enums, functions, and methods as well as modules.
    ",
    "pub".green().bold()
    );       
}

// Subheader: Best Practices for Packages with a Binary and a Library. Abbreviated as bpfpwbal.
fn bpfpwbal_content() {
    menu::subheader_title("Best Practices for Packages with a Binary and a Library");

    println!("\
    A package can contain both a {0} binary crate root as well as a \
    {1} library crate root, and both crates will have the package name by default. \n\
    Typically, packages with this pattern of containing both a library and a binary crate will \n\
    have just enough code in the binary crate to start an executable that calls code defined in \
    the library crate. \n\
    This lets other projects benefit from the most functionality that the package provides \
    because the library crate's code can be shared.
    ",
    "src/main.rs".italic(),
    "src/lib.rs".italic()
    );

    println!("\
    The module tree should be defined in {}. \n\
    Then, any public items can be used in the binary crate by starting paths with the \
    name of the package. \n\
    The binary crate becomes a user of the library crate just like a completely external \
    crate would use the library crate: it can only use the public API. \n\
    This helps you design a good API; not only are you the author, you're also a client!
    ",
    "src/lib.rs".italic()
    );
}

// Subheader: Starting Relative Paths with super. Abbreviated as srpws.
fn srpws_content() {
    menu::subheader_title("Starting Relative Paths with super");

    println!("\
    We can construct relative paths that begin in the parent module, rather than the current \
    module or the crate root, \n\
    by using {0} at the start of the path. \n\
    This is like starting a filesystem path with the {1} syntax that means to go to the parent directory. \n\
    Using {0} allows us to reference an item that we know is in the parent module, \n\
    which can make rearranging the module tree easier when the module is closely related to \n\
    the parent but the parent might be moved elsewhere in the module tree someday.
    ",
    "super".green().bold(),
    "..".green().bold(),
    );   
}
