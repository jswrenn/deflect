/// A static value (e.g., enum discriminant).
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum Data {
    /// A byte of data.
    u8(u8),
    /// Two bytes of data.
    u16(u16),
    /// Four bytes of data.
    u32(u32),
    /// Eight bytes of data.
    u64(u64),
}
