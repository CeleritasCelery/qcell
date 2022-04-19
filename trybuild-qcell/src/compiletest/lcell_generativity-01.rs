extern crate qcell;

#[allow(warnings)]
fn main() {
    use qcell::{LCell, LCellOwner, generativity::make_guard};
    use std::rc::Rc;
    make_guard!(guard1);
    let mut owner1 = LCellOwner::new(guard1);
    let owner2 = owner1.clone(); // Compile fail
}
