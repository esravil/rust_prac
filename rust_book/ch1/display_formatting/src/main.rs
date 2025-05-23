use std::fmt;

#[derive(Debug)]
struct Complex {

    real: f64,
    imag: f64,

}

impl fmt::Display for Complex {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{} + {}i", self.real, self.imag)

    }


}

fn main() {

    let pt = Complex { real: 3.3, imag: 7.2 }; // when instantiating a braced struct, need to include exact param names

    println!("Display: {}", pt); // this is the formatting for display pretty for end users (can be used for custom structs as well)
    println!("Debug: {:?}", pt); // this is the formatting for debugging and devs, can be used for most things


}