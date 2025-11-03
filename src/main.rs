// use std::{fmt::{Debug, Display}, mem::swap};

mod search;
mod fibonacci;
mod heap;


fn main() {
    fibonacci::test_fibonacci();
    search::test_search();
    heap::test_heap();
}
