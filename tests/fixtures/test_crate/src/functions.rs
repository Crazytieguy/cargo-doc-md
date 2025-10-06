pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply<T: std::ops::Mul<Output = T> + Copy>(a: T, b: T) -> T {
    a * b
}

pub fn process_slice(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}

pub fn process_mut_slice(data: &mut [u8]) {
    for byte in data {
        *byte = byte.wrapping_add(1);
    }
}

pub async fn async_function(url: &str) -> Result<String, String> {
    Ok(format!("Fetched: {}", url))
}

pub fn higher_order_function<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(42)
}

pub unsafe fn unsafe_function(ptr: *const u8) -> u8 {
    *ptr
}

pub const fn const_function(x: i32) -> i32 {
    x * 2
}
