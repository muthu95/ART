
use std;

pub trait KeyInterface {
    fn bytes(&self) -> &[u8];
}

impl KeyInterface for u32 {
    fn bytes(&self) -> &[u8] {
        let ptr = self as *const u32 as *const u8;
        unsafe { std::slice::from_raw_parts(ptr, 4) }
    }
}

impl KeyInterface for std::string::String {
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}
