#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // these above four are methods since they start with self as a first parameter
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn print_area(&self) {
        println!("The area is {}", self.area());
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    tuples();
    ownership();
    implementing_struct();
}
// tuples
fn tuples() {
    let things_tuple = ("apple", "banana", 2i32, 21i32);
    //  println!("my things in the tuple {}",things_tuple);
    // cannot format the tuple
    println!("my things in the tuple {:?}", things_tuple);
    println!("m");
    // i wanna print one by one, so destructure
    let (fruit_1, _fruit_2, _count_1, _count_2) = things_tuple;
    let nums = (things_tuple.2, things_tuple.3);
    println!("fruit {}", fruit_1);
    println!("the nums array {:?}", reverse(nums));
    fn reverse(pair: (i32, i32)) -> (i32, i32) {
        let (x, y) = pair;
        // return expression without semicolon
        (y, x)
    }
}

fn ownership() {
    let n = 32;
    pass_num(n);
    println!("the first number {n}");
    let the_name = String::from("Kara");
    say_hello(&the_name);
    println!("the original name {the_name}");
    fn pass_num(x: i32) {
        println!("it is received in the function {}", x);
    }
    fn say_hello(name: &String) {
        println!("Hello {}", name);
    }
}
fn implementing_struct() {
    let rect1 = Rectangle {
        width: 30,
        height: 20,
    };
    let rect2 = Rectangle {
        width: 50,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 25,
        height: 15,
    };
    //how we call associated functions
    let sq = Rectangle::square(25);
    //    dbg!(&rect1);
    println!(
        "The rect1 {rect1:?} and it's greater than 0 {}",
        rect1.width()
    );

    println!("The square's area");
    sq.print_area();
    rect1.print_area();
    println!("The rect1 can hold rect2 {}", rect1.can_hold(&rect2));
    println!("The rect1 can hold rect2 {}", rect1.can_hold(&rect3));
}
