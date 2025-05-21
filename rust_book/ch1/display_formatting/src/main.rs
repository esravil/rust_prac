use std::fmt;

// notice how we dont use the derive debug macro here
struct Structure(i32);

impl fmt::Display for Structure {


    // if you want prettier formatting, you will need to write out the fmt signature within the implementation of
    // the display for the structure explicitly
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{}", self.0) // self refers to the 

    }

}

fn main() {
    


}
