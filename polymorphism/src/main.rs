//在Java中，多态指的是子类的对象使用父类的方法
//多态与继承关联，多态中并没有泛型参数<T>的声明
//enum+match, 也算是多态吗？
//多态是method层面的，函数泛型是function层面的


//example #1


//
struct Circle{radius: f64}
struct Square{side: f64}

//
trait Shape {
  fn area(&self) -> f64;

}


//
impl Shape for Circle{
  fn area(&self) -> f64{
 self.radius * self.radius * std::f64::consts::PI
  }

}

impl Shape for Square{
  fn area(&self) -> f64{
 self.side * self.side
  }

}


//
fn main() {
//Shape trait可以当类型名使用？
//trait object的概念，dyn的概念
//dynamic dispatch的概念
//加不加dyn的区别
//dyn Shape这种写法有点类似impl Shape这种，可以用来做类型名，实际上是一个类型集
//这种写法使异质的数据元素，可以像同质元素
//array必须是同质元素
  let shapes:[&dyn Shape; 4] = [
 &Circle{radius: 1.0},
 &Square{side: 3.0},
 &Circle{radius: 2.0},
 &Square{side: 4.0}
  ];

  for(i, shape) in shapes.iter().enumerate(){
 println!("shape #{} has area {}", i, shape.area());
  }

}
