use std::cell::Cell;

pub fn cell_example() {
    let x = Cell::new(5);
    x.set(10); // We can mutate the value inside the Cell, even though x is not mutable
    println!("The value inside Cell is: {}", x.get());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell() {
        cell_example();
    }
}
