
// kalau pakai assoc type disini power for u32 cuma bisa 1
// sementara disini kita butuh untuk 2 input yaitu u32 dan u16
// makanya orang biasa pakai generic type
pub trait Power<N> {
    fn power(self, n: N) -> Self;
}

// implementasi untuk u32 ^ u32
impl Power<u32> for u32 {
    fn power(self, n: u32) -> Self {
        self.pow(n)
    }
}

// implementasi untuk u32 ^ u16
impl Power<u16> for u32 {
    fn power(self, n: u16) -> Self {
        self.pow(n as u32)
    }
}

// implementasi untuk u32 ^ &u32 (reference)
impl Power<&u32> for u32 {
    fn power(self, n: &u32) -> Self {
        self.pow(*n)
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
