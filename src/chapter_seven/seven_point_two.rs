use owo_colors::OwoColorize;
use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 2];
  subheaders = [
    chapter::SubHeader::new("Cheat Sheet", cs_content),
    chapter::SubHeader::new("Grouping Related Code in Modules", grcim_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content
// Subheader: Cheat Sheet. Abbreviated as cs
fn cs_content() {
    let solid_disc = "\u{2022}";
    let open_disc = "\u{25CB}";
    let two_spaces = "\u{2003}\u{2003}";

    // Subheader title.
    menu::subheader_title("Modules Cheat Sheet");
    
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
        {two_spaces}The compiler will look for the module’s code in these places: \n\
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

    println!("\
        {solid_disc} {0} Once a module is part of your crate, you can refer to code in that module \
        from anywhere else in that same crate, \n\
        {two_spaces}as long as the privacy rules allow, using the \
        path to the code. \n\
        {two_spaces}For example, an {1} type in the garden vegetables module would be found at {2}.
        
    ",
    "Paths to code in modules:".bold(),
    "Asparagus".green().bold(),
    "crate::garden::vegetables::Asparagus".green().bold(),
    );    

    println!("\
        {solid_disc} {0} Code within a module is private from its parent modules by default. \n\
        {two_spaces}To make a module public, declare it with {1} instead of {2}. \n\
        {two_spaces}To make items within a public module public as well, use {3} before their declarations.        
    ",
    "Private vs. public:".bold(),
    "pub mod".green().bold(),
    "mod".green().bold(),
    "pub".green().bold(),
    );

    println!("\
        {solid_disc} {0} Within a scope, the {1} keyword creates shortcuts to items to reduce repetition of long paths. \n\
        {two_spaces}In any scope that can refer to {2}, you can create a shortcut with \n\
        {two_spaces}{2}; and from then on you only need to write {3} to make use of that \
        type in the scope.
    ",
    "The use keyword:".bold(),
    "use".green().bold(),
    "crate::garden::vegetables::Asparagus".green().bold(),
    "Asparagus".green().bold(),
    );     

}

// Subheader: Grouping Related Code in Modules. Abbreviated as grcim
fn grcim_content() {
    menu::subheader_title("Grouping Related Code in Modules");

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






















