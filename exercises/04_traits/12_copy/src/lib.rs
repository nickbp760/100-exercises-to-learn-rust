// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

// secara aturan Rust, kamu tidak bisa Copy tanpa Cloneala
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

// yang pakai penambahan adalah struct WrappingU32 jadi butuh impl add
// karena tipe buat for u32 pasti input u32 jadi cukup pakai assoc type
impl std::ops::Add for WrappingU32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value.wrapping_add(rhs.value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        // + y kedua jadi butuh copy
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
