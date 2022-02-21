//use crate::print_from_monsters;//no `print_from_monsters` in the root。crate::不行
//原因解析：main.rs或lib.rs都是crate root，在main.rs中使用crate::，系统不知道指的是哪个文件，所以crate只能在同文件中使用
//在main.rs中使用具体的crate名，则系统自动去lib.rs中寻找资源，使用crate::则在同文件中去查找资源
//但是在lib.rs中使用具体的crate_name，而不用crate::，为什么又不行了呢？
//大致可以得出结论：crate::是同文件查找的，crate_name::是跨文件的
/*
#1，可行
mod lib;//把lib.rs文件视为一个普通的文件，模块
use lib::print_from_monsters;
*/
// #2 #3
//extern crate lib_main;//有没有这句话都行
//use的目的就是缩短资源引用路径

use lib_main::layer1_a::print_from_monsters;
fn main(){
    print_from_monsters();
}



//在main.rs中用crate::的示例
/*
pub mod layer1_c{
    pub fn print_from_monsters(){
        println!("Printing from crate monsters!");
    }
}
use crate::layer1_c::print_from_monsters;
fn main(){
    print_from_monsters();
}
*/