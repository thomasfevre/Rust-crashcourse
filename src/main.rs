trait Shape {
    fn area(&self) -> f64;
}

trait NShape<T> {
    fn new(value:T) -> Self;
}

trait AsNShape<T> {
    fn new(value:T,bivalue:T) -> Self;
}

struct Square<T> {
area_value:f64,side: T
}

impl <S> Shape for Square<S>{
    fn area(&self) -> f64 {self.area_value}
}

impl NShape<u32> for Square<u32> {
    fn new(value:u32) -> Square<u32> {Square::<u32>{area_value:value.pow(2) as f64,side:value}}
}

impl NShape<f64> for Square<f64> {
    fn new(value:f64) -> Square<f64> {Square::<f64>{area_value:value.powi(2),side:value}}
}

impl NShape<String> for Square<String> {
    fn new(value:String) -> Square<String> {Square::<String>{area_value:value.parse::<i32>().unwrap().pow(2) as f64,side:value}}
}

struct Triangle<T=f64> {
area_value:f64,base: T
}

impl AsNShape<f64> for Triangle<f64> {
    fn new(value:f64,value2:f64) -> Triangle {Triangle{area_value: value*value2/2.0,base:value}}
}

impl Shape for Triangle<f64>{
    fn area(&self) -> f64 {self.area_value}
}

trait TriShape<T,Y> {
    fn volume(&self) -> Y;
    fn new(base:T,height:Y) -> Self;
}

struct Pyramid<T,Y> {
base: T,height: Y
}

impl <S> TriShape<S,f64> for Pyramid<S,f64> where S: Shape{
    fn new(value: S,value2: f64) -> Pyramid<S,f64> {Pyramid::<S,f64>{base:value,height:value2}}
    fn volume(&self) -> f64 {let b = self.base.area().powi(1);b*self.height/3.0}
}

fn main() {
    let square=Square::<u32>::new(5);
    let square_float=Square::<f64>::new(5.4);
    let square_string=Square::<String>::new("6".to_string());

    println!("square area is {}",square.area());
    println!("square_float area is {}",square_float.area());
    println!("square_string area is {}",square_string.area());

    let triangle=Triangle::new(14.9,20.1);
    let pyramid_square=Pyramid::<Square<u32>,f64>::new(square,24.3);
    let pyramid_triangle=Pyramid::<Triangle<f64>,f64>::new(triangle,24.3);
    
    println!("pyramid_square volume is {}",pyramid_square.volume());
    println!("pyramid_triangle volume is {}",pyramid_triangle.volume());
}