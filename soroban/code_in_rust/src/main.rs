fn main() {
    

    let x: i32 = 5;
    println!("The value of x is: {}", x);

    let y: String = String::from("hello, Soroban!");
    let z: &str = "Hello, Stellar";
    println!("The value of y is: {y}" );
    println!("The value of z is: {z}" );

    let p = Point { x: 5, y: 10 };
    println!("The value of x is: {}", p.x );
    println!("The value of y is: {}", p.y );

}

struct Point {
    x: i32,
    y: i32,
}