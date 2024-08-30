use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub fn rc_cell_example() {
    let data = Rc::new(Cell::new(5));

    let data1 = Rc::clone(&data);
    data1.set(10);

    println!("Data in Rc<Cell>: {}", data.get());
}

pub fn rc_refcell_example() {
    let data = Rc::new(RefCell::new(vec![1, 2, 3]));

    let data1 = Rc::clone(&data);
    data1.borrow_mut().push(4);

    println!("Data in Rc<RefCell>: {:?}", data.borrow());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_cell() {
        rc_cell_example();
    }

    #[test]
    fn test_rc_refcell() {
        rc_refcell_example();
    }
}
