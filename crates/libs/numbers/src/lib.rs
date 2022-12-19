pub struct ConverterU64U32(u64);

impl ConverterU64U32 {
    pub fn new_by_u64(number: u64) -> Self {
        Self(number)
    }

    pub fn new_by_u32(high: u32, low: u32) -> Self {
        let higher = (high as u64) << 32;
        let total = higher + low as u64;
        Self(total)
    }

    pub fn total(&self) -> u64 {
        self.0
    }

    pub fn high(&self) -> u32 {
        (self.0 >> 32) as u32
    }

    pub fn low(&self) -> u32 {
        self.0 as u32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bigger_than_u32() {
        let converter = ConverterU64U32((u32::MAX as u64) + 100);
        let result = ConverterU64U32::new_by_u32(converter.high(), converter.low());
        assert_eq!(converter.total(), result.total());
    }

    #[test]
    fn smaller_than_u32() {
        let converter = ConverterU64U32((u32::MAX as u64) - 100);
        let result = ConverterU64U32::new_by_u32(converter.high(), converter.low());
        assert_eq!(converter.total(), result.total());
    }

    #[test]
    fn zero() {
        let converter = ConverterU64U32(0);
        let result = ConverterU64U32::new_by_u32(converter.high(), converter.low());
        assert_eq!(converter.total(), result.total());
    }

    #[test]
    fn u64_max() {
        let converter = ConverterU64U32(u64::MAX);
        let result = ConverterU64U32::new_by_u32(converter.high(), converter.low());
        assert_eq!(converter.total(), result.total());
    }
}
