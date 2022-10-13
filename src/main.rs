use std::fmt::Debug;

fn main() {
    let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    // let square_string = Square::<String>::new("6");

    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());
    // println!("square_string area is {}", square_string.area());

    // let triangle = Triangle::new(14.9, 20.1);
    // let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    // let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    // println!("pyramid_square volume is {}", pyramid_square.volume());
    // println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}


struct Square<T: Copy>
{
    side: f64,
    test: T,
}

impl<T: TryInto<f64> + Copy> Square<T>
{
    fn new( t: T) -> Self 
    where <T as TryInto<f64>>::Error: Debug {
        
        Square { side : t.try_into().unwrap(), test : t}
    }
}


trait Area {
    fn area(&self) -> f64;
}

impl<T: Copy> Area for Square<T>

{
    fn area(&self) -> f64 {
        let edge = self.side;
        return edge * edge;
    }

}
