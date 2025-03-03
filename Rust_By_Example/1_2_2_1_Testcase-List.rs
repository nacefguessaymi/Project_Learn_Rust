use std::fmt; //imports formatting

//Define list struct
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //Extracts the value using tuples
        let vec = &self.0;

        write!(f,"[")?;

        //Iterate over the values of 'v' in vec
        for (count, v) in vec.iter().enumerate() {
            //Adds a comma to every element except the first
            if count != 0 { write!(f, ", ")?; }
            write!(f,"{}", v)?;
        }
    }
}

fn main() {
    let v = List(vec![1,2,3]);
    println!("{}", v);
}