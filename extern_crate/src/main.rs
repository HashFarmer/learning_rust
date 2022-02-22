//用external crate的2个步骤：
//1、在Cargo.toml的dependencies中添加crate名
//2、use carate_name::
//不必extern crate crate_name;
use rand::Rng;
fn main() {
    let secret_number : u8 = rand::thread_rng().gen_range(1..10);
    println!("Hello, {}", secret_number);
}
