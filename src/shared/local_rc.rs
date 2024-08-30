use std::rc::Rc;

pub fn rc_example() {
    let data = Rc::new(vec![1, 2, 3]);

    let data1 = Rc::clone(&data);
    let _data2 = Rc::clone(&data);

    println!("Rc count after cloning: {}", Rc::strong_count(&data));
    println!("Data: {:?}", data1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc() {
        rc_example();
    }
}
