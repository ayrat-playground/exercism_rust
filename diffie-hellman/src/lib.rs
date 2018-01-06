pub fn private_key(p: u64) -> u64 {
    // this value should be random (I don't want to use external dependency)
    // Rust does not have random module in std library
    p - 1
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.pow(a as u32) % p
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.pow(a as u32) % p
}
