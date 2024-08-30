use std::cell::RefCell;

pub fn refcell_example() {
    let x = RefCell::new(vec![1, 2, 3]);
    x.borrow_mut().push(4); // We can mutate the value inside RefCell
    println!("The value inside RefCell is: {:?}", x.borrow());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refcell() {
        refcell_example();
    }
}
