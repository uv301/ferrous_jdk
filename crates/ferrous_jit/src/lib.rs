pub mod jit;
pub mod common;
pub mod optimizations;
pub mod cranelift_backend;
pub mod llvm_backend;
pub mod aot;

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
