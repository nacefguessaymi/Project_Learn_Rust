fn main() {
    // so the {} is like printf
    println!("{} days", 31);
    // this is a weird way to do this but fine
    println!("{0}, this is {1}. {1}, this is {0}", "Nacef", "Alex");
    // ok now were getting into vars
    println!("{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jump over");
    // ard lesgooooo were cooking now
    println!("Base 10:                  {}", 69420);
    println!("Base 2 (binary):        {:b}", 69420);
    println!("Base 8:                 {:o}", 69420);
    println!("Base 16:                {:x}", 69420);
    // why would they structure the print statements above and THEN
    // teach you how to right justify
    println!("{number:>5}", number = 1);

    //now we learning floating point numbers
    println!("{number:0>5}", number = 1);
    // this flips it
    println!("{number:0<5}", number = 1);

    // I dont get this one
    println!("{number:0>width$}", number = 1, width = 5);
    //nvm i got it just passing along a var

    // fixing this
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // ok really getting lost now
    #[allow(dead_code)] //this is a really nice function here
    struct Structure(i32);

    // this is supposed to be broken here
    /*fmt::Display.
    println!("This struct `{}` won't print...", Structure(3));*/

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    //implementing pi
    let pi = 3.141592654;
    let dec = 3;
    println!("Pi is roughly {pi:.dec$}");
}