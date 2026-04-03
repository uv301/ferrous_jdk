pub mod common;
pub mod gc;
pub mod serial;
pub mod parallel;
pub mod zgc;
pub mod shenandoah;
pub mod g1gc;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
