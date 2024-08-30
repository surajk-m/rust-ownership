extern crate rust_ownership;

use rust_ownership::*;

#[test]
fn test_static_ownership() {
    ownership_example();
}

#[test]
fn test_cell() {
    cell_example();
    im_cell_example();
}

#[test]
fn test_refcell() {
    refcell_example();
    im_refcell_example();
}

#[test]
fn test_atomic() {
    atomic_example();
    im_atomic_example();
}

#[test]
fn test_mutex_rwlock() {
    mutex_example();
    rwlock_example();
}

#[test]
fn test_rc() {
    rc_example();
}

#[test]
fn test_rc_cell_refcell() {
    rc_cell_example();
    rc_refcell_example();
}

#[test]
fn test_arc() {
    arc_example();
}

#[test]
fn test_arc_atomic_mutex_rwlock() {
    arc_atomic_example();
    arc_mutex_example();
    arc_rwlock_example();
}

#[test]
fn test_producer_consumer() {
    producer_consumer_example();
}

#[test]
fn test_worker_pool() {
    worker_pool_example();
}
