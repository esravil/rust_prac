use std::fmt;

// define a new struct, for vectors named list
struct List(Vec<i32>);
// dont presume that debug print will work on the list, but will need to further check in tests

// implement the fmt::Display here

impl fmt::Display for List {

    // nested fmt fxn
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // instantiate a struct here
        let vec = &self.0; // since we are grabbing the param from the vector items itself,
        // need to add a portion where we indicate that we are accessing the class var, or vec vars
        
        // write opening bracket
        write!(f, "[")?;

        for (i, v) in vec.iter().enumerate() {

            if i != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", i, v)?;

        }

        write!(f, "]") // doesnt need closing semi or the ? param, indicates end of all args to be passed through
    }

}

// main fn
fn main() {

    let vector = List(vec![1, 2, 3]);
    println!("{}", vector);

}