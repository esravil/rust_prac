#[derive(Debug)]
    struct custom_print(i32);

    // now we will be placing custom_print struct within a new struct that we will be creating, and will also make it printable
#[derive(Debug)]
struct wrapper(custom_print);

fn main() {

    // why do we have multiple ways to format strings?
    // the debate btwn std::fmt::Display vs std::fmt::Debug
    // 
    // Display = end user, polished product
    // debug = for devs, helps us find all output in strings and an extremely feasible way to
    // print debug stmnts for the derive(Debug) for even custom struct

    println!("{:?} is today's date", "May 21st 2025");
    println!("{:?}", custom_print(4));
    println!("{:?}", wrapper(custom_print(4)));

    // we can also do pretty printing via fmt debug, via :#?

}