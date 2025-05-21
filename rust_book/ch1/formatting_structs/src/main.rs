use std::fmt;

// need to specify derive debug macro in order to access debug std fmt
#[derive(Debug)]
// this is a parentheses struct
struct MinMax(i64, i64); // in this example, we are not using a struct specifying the variables for the struct itself

// lets specify display std fmt

// implement display format for the struct
impl fmt::Display for MinMax {

    // function fmt with self, changing format, and the formatter itself
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{}, {}", self.0, self.1)

    }

}

// this is a BRACED FIELD STRUCt
#[derive(Debug)]
struct Points {
    x: f64,
    y: f64,
}

impl fmt::Display for Points {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{}, {}", self.x, self.y) // do not use a semicolon bc this becomes a statement then, and write macro already returns val as needed

    }

}

// Creating a struct called complex

#[derive(Debug)]
struct Complex {

    real: f64,
    imag: f64,

}

// define the implementation of the std::fmt::Display
impl fmt::Display for Complex {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{} + {}i", self.real, self.imag)

    }

}

// let us define a struct for a vector for the display
struct List(Vec<i32>);

impl fmt::Display for List {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let vec = &self.0;

        write!(f, "[");

        for (i, v) in vec.iter().enumerate() { // count is the index, v is the value of the value at index count

            if i != 0 {

                write!(f, ", ")?; // the question allows for multi args

            }
            write!(f, "{}: {}", i, v)?; // v is the value itself

        }

        write!(f, "]")

    }

}


// above, we did implementations for the std::fmt::Display as well as the std::fmt::Debug
// below we will write the main function out to do cool prints

fn main() {

    // think of the structs as classes, and these vars are the instantiations of them
    let mm = MinMax(3, 5);

    println!("Display {}", mm);
    println!("Display {:?}", mm);

    let pts = Points{
            x: 3.5, y: 99.9
            };
    println!("Display {}", pts);
    println!("Display {:?}", pts);

    let cmplx = Complex {

        real: 3.3,
        imag: 7.2,

    };

    println!("Display: {}", cmplx);
    println!("Display: {:?}", cmplx);
    
    let ml = List(vec![1, 2, 3]);
    println!("{}", ml);

}

