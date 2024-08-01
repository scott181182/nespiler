


pub trait TrimInPlace {
    fn trim_in_place(&mut self) -> ();
}

impl TrimInPlace for String {
    fn trim_in_place(&mut self) -> () {
        let (start, len): (*const u8, usize) = {
            let self_trimmed: &str = self.trim();
            (self_trimmed.as_ptr(), self_trimmed.len())
        };
        unsafe {
            core::ptr::copy(
                start,
                self.as_bytes_mut().as_mut_ptr(), // no str::as_mut_ptr() in std ...
                len,
            );
        }
        // This is bugged on non-ASCII since it does not behave as `set_len`.
        self.truncate(len);
    }
}