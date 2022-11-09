/// hashmod is a function used by prometheus to hash scrape targets to a specific shard
pub fn hashmod(data: &[u8], modulus: u64) -> u64 {
    sum64(data) % modulus
}

/// sum64 is a hashing function used by hashmod
pub fn sum64(data: &[u8]) -> u64 {
    let data = md5::compute(data);
    let data = data[8..].try_into().unwrap();
    u64::from_be_bytes(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum64(b"test1");
        assert_eq!(result, 10916960992523345290);
    }
}
