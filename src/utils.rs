use crate::F;

pub fn floats_equal(one: F, two: F) -> bool {
    (one - two).abs() < 0.00001
}
