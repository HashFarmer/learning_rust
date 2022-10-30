
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
