use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let vec = &self.0; // since we want the items in the vactor, not the vector object itself, os we can iter all the values of the vector

        write!(f, "[")?; // why ? idk

        for (i, v) in vec.iter().enumerate() {

            if i != 0 { write!(f, ", ")?; } // the ? denotes we will print multiple of these out

            write!(f, "{}: {}", i, v)?

        }

        write!(f, "]")

    }
}

fn main() {

    let v = List(vec![1, 2, 3]); // for instantiating a vector, need to have the struct if applicable, then the vec! with the array inside
    println!("{}", v);

}