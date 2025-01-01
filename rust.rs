/// Provides a &'static str from a str for sending &str to threads or tasks.
/// Cheap to clone and move around.
/// Panics if the &str source lifetime is shorter than UnsafeStr.
#[derive(Clone, Copy)]
struct UnsafeStr{
    value: &'static str
}
impl UnsafeStr {
    fn new(string: &str) ->Self{
        unsafe {
            Self {
                value: core::str::from_utf8_unchecked(std::slice::from_raw_parts(string.as_ptr(), string.len()))
            }
        }
    }
}