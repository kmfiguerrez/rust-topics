pub mod module_topics {
    /// A small example function exposed by the library crate.
    pub fn module_content() {
        // println!("Hello from library module: module_topics::module_content()");
        println!("\
        7.2 \n\
        The module tree might remind you of the filesystem’s directory tree on your computer. \n\
        Just like directories in a filesystem, you use modules to organize your code. \n\
        And just like files in a directory, we need a way to find our modules.
        ");

        println!("\
        7.3 Paths for Referring to an item in the Module Tree \n\
        A path can take two forms: \n\
        - An absolute path is the full path starting from a crate root; \n\
        for code from an external crate, the absolute path begins with the crate name, \n\
        and for code from the current crate, it starts with the literal crate. \n\
        - A relative path starts from the current module and uses self, super, \
        or an identifier in the current module. \n\n\
        Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).
        ");

        println!("\
        7.3 \n\
        If items are defined in the same crate, we can use the crate keyword \
        to start an absolute path. \n\
        We then include each of the successive modules until we make our way to \
        to the desired item. \n\
        using the crate name to start from the crate root is like using / to start from the filesystem root in your shell. \n\
        Starting with a module name means that the path is relative.
        ");

        println!("\
        7.3 \n\
        Choosing whether to use a relative or absolute path is a decision you’ll make based on your project, \n\
        and it depends on whether you’re more likely to move item definition code separately from or together \
        with the code that uses the item. \n\
        Our preference in general is to specify absolute paths because it’s more likely \
        we’ll want to move code definitions and item calls independently of each other.
        ");

        println!("\
        7.3 \n\
        In Rust, all items (functions, methods, structs, enums, modules, and constants) \
        are private to parent modules by default. \n\
        If you want to make an item like a function or struct private, you put it in a module.
        ");

        println!("\
        7.3 \n\
        Items in a parent module can’t use the private items inside child modules, but items in \
        child modules can use the items in their ancestor modules. \n\
        This is because child modules wrap and hide their implementation details, \
        but the child modules can see the context in which they’re defined. \n\
        However, Rust does give you the option to expose inner parts of child modules’ \
        code to outer ancestor modules by using the pub keyword to make an item public.
        ");

        println!("\
        7.3 Exposing Paths with the pub keyword \n\
        Making the module public doesn’t make its contents public. \n\
        The pub keyword on a module only lets code in its ancestor modules refer to it, \
        not access its inner code. \n\
        Because modules are containers, there’s not much we can do by only making the module public; \
        we need to go further and choose to make one or more of the items within the module public as well.
        ");

        println!("\
        7.3 Best Practices for Packages with a Binary and a Library \n\
        A package can contain both a src/main.rs binary crate root as well as a \
        src/lib.rs library crate root, and both crates will have the package name by default. \n\
        Typically, packages with this pattern of containing both a library and a binary crate will \n\
        have just enough code in the binary crate to start an executable that calls code defined in \
        the library crate. \n\
        This lets other projects benefit from the most functionality that the package provides \
        because the library crate’s code can be shared.
        ");

        println!("\
        7.3 \n\
        The module tree should be defined in src/lib.rs. \n\
        Then, any public items can be used in the binary crate by starting paths with the \
        name of the package. \n\
        The binary crate becomes a user of the library crate just like a completely external \
        crate would use the library crate: it can only use the public API. \n\
        This helps you design a good API; not only are you the author, you’re also a client!
        ");

        println!("\
        7.3 Starting Relative Paths with super \n\
        We can construct relative paths that begin in the parent module, rather than the current \
        module or the crate root, by using super at the start of the path. \n\
        This is like starting a filesystem path with the .. syntax that means to go to the parent directory. \n\
        Using super allows us to reference an item that we know is in the parent module, \n\
        which can make rearranging the module tree easier when the module is closely related to \n\
        the parent but the parent might be moved elsewhere in the module tree someday.
        ");               
    }
}
