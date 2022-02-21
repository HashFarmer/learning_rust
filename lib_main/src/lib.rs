pub mod layer1_a{
    pub fn print_from_monsters(){
        println!("Printing from crate monsters!");
    }
}

pub mod layer1_b{
    pub fn indirect_print_from_monsters(){
        //lib_main::layer1_a::print_from_monsters();//use of undeclared crate or module `lib_main`
        crate::layer1_a::print_from_monsters();
    }
}

pub fn hello(){
    layer1_a::print_from_monsters();//layer1_a前面不需要super::，否则there are too many leading `super` keywords
}