use owo_colors::OwoColorize;

const TITLE: &str = "Defining Modules to Control Scope and Privacy";

pub fn display_title() {
    println!("{}", TITLE.bold().bright_blue());
    println!("{}", "-".repeat(TITLE.len()).bright_blue());
}

pub fn display_content() {

    display_title();


    mcs_content();

    // Subheader.
    println!("{}", "Grouping Related Code in Modules \n".blue().bold());

    println!("\
        {} let us organize code within a crate for readability and easy reuse. \n\
        Modules also allow us to control the privacy of items because code within a module is private by default. \n\
        Private items are internal implementation details not available for outside use.
    ", "Modules".italic());

    println!("\
        We define a module with the {} keyword followed by the name of the module. \n\
        The body of the module then goes inside curly brackets. Inside modules, we can place other modules \n\
        Modules can also hold definitions for other items, such as structs, enums, constants, traits, and functions.
    ", "mod".green().bold());

    println!("\
        By using modules, we can group related definitions together and name why they’re related. \n\
        Programmers using this code can navigate the code based on the groups rather than having to \n\
        read through all the definitions, making it easier to find the definitions relevant to them. \n\
        Programmers adding new functionality to this code would know where to place the code to keep the \
        program organized.
    ");

    println!("\
        Earlier, we mentioned that {0} and {1} are called crate roots. \n\
        The reason for their name is that the contents of either of these two files form a module named \n\
        {2} at the root of the crate’s module structure, known as the {3}.
    ", 
    "src/main.rs".italic(),
    "src/lib.rs".italic(),
    "crate".green().bold(),
    "module tree".italic()
    );

    println!("\
        Modules that nest inside other modules are called {0} or {1}. \n\
        Modules are {2} if they're defined in the same module. \n\
        A module tree is rooted under the implicit module named {3}.
    ",
    "submodules".italic(),
    "child modules".italic(),
    "siblings".green().bold(),
    "crate".green().bold()
    );

}

// Subheaders content
// Modules Cheat Sheet. Abbreviated as mcs
fn mcs_content() {
    let solid_disc = "\u{2022}";
    let open_disc = "\u{25CB}";
    let two_spaces = "\u{2003}\u{2003}";

    // Subheader title.
    println!("{}", "Modules Cheat Sheet \n".blue().bold());
    
    println!("\
        {solid_disc} {0} When compiling a crate, the compiler first looks in the crate root file \n\
        (usually {1} for a library crate or {2} for a binary crate) for code to compile. \n\
        ",
        "Start from the crate root:".bold(),
        "src/lib.rs".italic(),
        "src/main.rs".italic()
    );

    println!("\
        {solid_disc} {0} In the crate root file, you can declare new modules; say you declare a \
        “garden” module with {1}. \n\
        \t{open_disc} Inline, within curly brackets that replace the semicolon following {1} \n\
        \t{open_disc} In the file {2} \n\
        \t{open_disc} In the file {3} (older style, not preferred)
    ",
    "Declaring modules:".bold(),
    "mod garden;".green().bold(),
    "src/garden.rs".italic(),
    "In the file src/garden/mod.rs".italic()
    );

    println!("\
        {solid_disc} {0} In any file other than the crate root, you can declare submodules. \n\
        {two_spaces}For example, you might declare {1} in {2}. \n\
        {two_spaces}The compiler will look for the submodule’s code within the directory named for the parent \
        module in these places: \n\
        \t{open_disc} Inline, directly following {3}, within curly brackets instead of the semicolon \n\
        \t{open_disc} In the file {4} \n\
        \t{open_disc} In the file {5} (older style, not preferred)
    ",
    "Declaring submodules:".bold(),
    "mod vegetables;".green().bold(),
    "src/garden.rs".italic(),
    "mod vegetables".green().bold(),
    "src/garden/vegetables.rs".italic(),
    "src/garden/vegetables/mod.rs".italic(),
    );    
}