// declarative macros looks like function
// `println!` or `vec!` are examples or declarative macros

// They're declared with the keyword `macro_rules!`
macro_rules! simple_vec {
    // Basically, a decl macro is a `match` statement
    // inputs are matched against each arm
    ( $( $x:expr ),* ) => { // Here I match 0 or more expressions, comma separated
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )* // Here the $()* indicate that this code will repeat as many times as there are inputs
            temp_vec // We return the vector
        }
    };
    // Add another arm here so that we can initialize vec3 in main()
    ($a:expr;$b:expr) => { // Here I match 0 or more expressions, comma separated
        {
            let mut temp_vec = Vec::with_capacity($b);
            for _ in 0..$b {
                temp_vec.push($a);
            }
            temp_vec // We return the vector
        }
    };
}

fn main() {
    let vec1 = simple_vec!('a', 'b', 'c');
    let vec2 = simple_vec!(1, 2, 3);
    let vec3 = simple_vec!("coucou";5);

    println!("{:?}\n{:?}", vec1, vec2);
    println!("{:?}", vec3);
}