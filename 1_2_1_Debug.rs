/*// this is not supposed to be printable
struct UnPrintable(i32);

//We can use the derive to get it to print
#[derive(Debug)];
struct DebugPrintable(i32); */

#[derive(Debug)]
struct Structure(i32);

//lets put a that struct within this one
#[derive(Debug)]
struct Deep(Structure);

// Let's play around with the "Pretty Print"
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Christian Slater";
    let age = 56;
    let actor = Person {name, age};
    //we can use '{:?}' to print these structs
    println!("{:#?} months in a year", 12);
    println!("{0:#?} is the {stri:?} name.",
            actor,
            stri = "actor's");
    // now the structure is printable
    println!("Now {:#?} will print!", Structure(3));

    // Now the struct within the struct will print
    // Let's see how ugly it looks
    println!("Now {:#?} will print!", Deep(Structure(7)));
    let name = "Nacef";
    let age = 29;
    let nacef = Person {name, age};
    println!("{:#?}", nacef)
}