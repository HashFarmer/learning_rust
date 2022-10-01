//用external crate的2个步骤：
//1、在Cargo.toml的dependencies中添加crate名
//2、use carate_name::
//不必extern crate crate_name;
// use是指定某个具体的功能对象，struct 或者 function ？
// extern crate 引入某一个包，只是一个名空间

// javascript:
// import {} from ... ， 指明要引入的item
// import * from ... ，把整个目标模块里的东西引入过来，把其中所有的item都对象化，使用的时候用  "."

// python :
// from ... import ... , 指明要引入的item
// import pandas , 引入整个模块，把其中的所有item对象化，本质和JavaScript一样

// rust
// 没有把模块里的item都对象化
// 仅仅相当于引入了一个名空间，当然也有 1、引入某个具体的item，用use， 2、引入整个名空间，用 extern crate ...



use rand::Rng;
fn main() {
    let secret_number : u8 = rand::thread_rng().gen_range(1..10);
    println!("Hello, {}", secret_number);
}
