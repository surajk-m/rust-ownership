pub fn ownership_example() {
    let x = String::from("hello");
    let y = x; // Ownership moves to y
               // println!("{}", x); // This line would cause a compile-time error because x no longer owns the data
    println!("{}", y); // y now owns the data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership() {
        ownership_example();
    }
}
