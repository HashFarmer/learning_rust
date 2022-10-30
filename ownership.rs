
1、

when a = b , "copy" or "move" 

    /*
    let mut a = 1;
    let mut _a = a;  // copy
    println!("{}", a);
    */

    /*
    let mut b = "abc";
    let mut _b = b; // copy
    println!("{}", b);
    */
    
    // copy within function    
    /*
    let a: i32 = 1;
    fn func(x: i32) -> i32 { x }
    func(a); // copy
    println!("{}", a); // ok
    */
    
    
    

    /*
    let mut c = vec![1, 2, 3, 4];
    let mut _c = c; // move
    //println!("{:?}", c);
    */

    /*
    let mut d = String::from("abc");
    let mut _d = d;  // move
    //println!("{:?}", d);
    */
    
    
    // move within function
    /*
    let s = String::from("hello");
    fn func(s1: String) -> String {s1};
    func(s); // move, s被转移给了一个临时变量s1，函数调用完毕后s1销毁了，s也没有了
    println!("{}", s); // error
    */
    
    


2、

when a = &b, "borrow"

    /*
    let mut a = 1;
    let a_ref = &a;
    println!("{}", a);
    */

    /*
    let mut c = vec![1, 2, 3, 4];
    let c_ref = &c;
    println!("{:?}", c);
    */

    /*
    let s = String::from("hello"); // s -> "hello"
    let s1 = &s;// s1 -> s -> heap区的"hello" ，对引用型变量的引用，java中貌似没有这个概念？
    */
    
    // borrow 的发生1、&a，2、函数中，borrow之间的冲突（“可变借用”与“不可变借用”的冲突）
