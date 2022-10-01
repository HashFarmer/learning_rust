//用external crate的2个步骤：
//1、在Cargo.toml的dependencies中添加crate名
//2、use carate_name::
//不必extern crate crate_name;
// use是指定某个具体的功能对象，struct 或者 function ？
// extern crate 引入某一个包，只是一个名空间
use rand::Rng;
fn main() {
    let secret_number : u8 = rand::thread_rng().gen_range(1..10);
    println!("Hello, {}", secret_number);
}
