pub mod module_topics {
    /// A small example function exposed by the library crate.
    pub fn module_content() {
        // println!("Hello from library module: module_topics::module_content()");
        println!("Making the module public doesn’t make its contents public.");
        println!("\
        The pub keyword on a module only lets code in its ancestor modules refer to it, \
        not access its inner code. \n\
        Because modules are containers, there’s not much we can do by only making the module public; \
        we need to go further and choose to make one or more of the items within the module public as well.
        ");
    }
}
