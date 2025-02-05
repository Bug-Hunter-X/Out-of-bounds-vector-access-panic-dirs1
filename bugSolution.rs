fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    match vec.get(1) {
        Some(val) => println!("The value is: {}", val),
        None => println!("Index out of bounds"),
    }

    //Alternative using if let
    if let Some(val) = vec.get(1) {
        println!("The value is: {}", val);
    } else {
        println!("Index out of bounds");
    }
} 