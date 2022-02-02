
// https://www.youtube.com/watch?v=ReBmm0eJg6g
// dyn some_trait  is similar to impl some_trait used in trait bound

pub trait Draw{
    fn draw(&self);
}

//#1
// trait object 

pub struct Screen{
    //用Box是因为这个Vec的元素大小是不确定的
    //但唯一确定的是它们implement了Draw这个trait
    //Vec的元素是不是必须同质？Box<dyn >使异质元素可以像同质元素那样
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen{
    pub fn run(&self){
        for component/*:&Box<dyn Draw>*/ in self.components.iter(){
            component.draw();
        }
    }
}

pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label: String,

}

impl Draw for Button{
    fn draw(&self){
        println!("Button drawing.")
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self){
        println!("SelectBox drawing.")
    }
}


fn main(){
    let screen = Screen{
        components: vec![
            Box::new( 
                SelectBox{
                    width: 100,
                    height: 100,
                    options: vec![
                    ]
                }
            ),
            Box::new(
                Button{
                    width: 100,
                    height: 100,
                    label: String::from("ok"),
                }
            ),
            //Box::new(String::from("test")),
            //从上面语句的错误提示看，dyn Draw,貌似也是一种trait bound
        ],
    };

    screen.run();

}

//summary
//static dispatch = generic + trait bound , compile time
//dynamic dispathc = doesn't know the concrete method you are calling at compile time, trait object=dynamic dispathc 




//#2
//generic + trait bound
/*
//              triat bound
pub struct Screen<T: Draw>{
    pub components: Vec<T>,
}

impl<T> Screen<T> where T:Draw, {
    pub fn run(&self){
        for component/*: &T*/ in self.components.iter(){
            component.draw();
        }
    }

}
*/
