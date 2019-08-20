pub mod another_module {
    pub fn is_red() {
        println!("red");
    }
}

pub fn something() -> String {
    another_module::is_red(); // relative
    crate::module::another_module::is_red(); //absolute
    String::from("something")
}