fn main() {

    // formatted print examples
    // println!("what is this");
    // format!("yo"); this wont rly print anything since no output stream

    // eprintln!("error message stream i think");

    /*
    
        Now, we will be formatting strings with args
        ex:
        "{}, "1"" -> prints out 1 as a string since this auto stringifies the first var after the comma in the formatted string

        numbered args:
        "{0} likes {1}, "Bob", "Alice""
        "Bob likes Alice"

        named args:
        "{who} {what} {when}, who="", what="", when="""
        ""

     */

    // println!("{} the goat", "Lebron James");
    // println!("I like to eat {food}", food="pie");

    // you may be wondering
    /*
    
        So how exactly do we do number formatting? We use b, o, x, where b = binary, o = octal, x = he(x)adecimal

    
     */
    println!("{number:>5}", number=5);
    println!("{number:0<5}", number=5);

    // remember the following:
    // > = means there are ""> to the left of the string specified, e.g.: if number = 4, number:0>5 = 00004, vice versa
    // < = means there are ""< to the right of the string specified

    // for precision we will do the following:
    // println!("PI rounded to the hundreths is {pi:.2}", pi=3.141);
    // in this notation, the 2 represents the number of values to show to the right of the decimal, and rust auto rounds

    

}
