pub mod advanced;
pub mod borrowing;
pub mod shared;
pub mod unique;

pub use advanced::concurrency_patterns::{producer_consumer_example, worker_pool_example};
pub use advanced::interior_mutability::{
    atomic_example as im_atomic_example, cell_example as im_cell_example,
    refcell_example as im_refcell_example,
};
pub use borrowing::local_cell::cell_example;
pub use borrowing::local_refcell::refcell_example;
pub use borrowing::threaded_atomic::atomic_example;
pub use borrowing::threaded_mutex_rwlock::{mutex_example, rwlock_example};
pub use shared::local_rc::rc_example;
pub use shared::local_rc_cell_refcell::{rc_cell_example, rc_refcell_example};
pub use shared::threaded_arc::arc_example;
pub use shared::threaded_arc_atomic_mutex_rwlock::{
    arc_atomic_example, arc_mutex_example, arc_rwlock_example,
};
pub use unique::static_example::ownership_example;
