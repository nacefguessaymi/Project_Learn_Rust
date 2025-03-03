// ok so to use {} we have to use the standard format
use std::fmt;

// lets define a structure
struct Structure(i32);

// we have to implement it
impl fmt::Display for Structure{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}

// this has to be done for every struct? seems a lil excessive

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}


#[derive(Debug)]
struct Complex {
    real: f64,
    im: f64
}

impl fmt::Display for Complex{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} + {}i", self.real, self.im)
    }
}

fn main() {
    let minmax = MinMax(0,14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_raneg = MinMax(-3, 3);

    println!("The big range is {big} and the small range is {small}", 
            small = small_raneg,
            big = big_range);

    let point = Point2D { x: 3.3, y: 7.2};
    println!("Compare points:");
    println!("Display: {}", point);
    println!{"Debug: {:?}", point};

    let com = Complex {real: 3.3, im: 7.2};
    println!("Display: {}", com);
    println!("Debug: {:?}", com);

}