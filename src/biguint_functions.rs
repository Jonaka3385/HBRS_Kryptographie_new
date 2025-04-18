use num_bigint::{BigUint, ToBigUint};
use rand::{rng, Rng};

pub fn max_biguint(bit_length: usize) -> BigUint {
    let count = bit_length / 8;
    let mut bytes = vec![0u8; count]; // bits / 8 = bytes

    let mut i = 0;
    while i < count {
        bytes[i] |= 0x80;
        i += 1;
    }

    BigUint::from_bytes_be(&bytes)
}

pub fn is_prime(n: BigUint) -> bool {
    let big0 = 0.to_biguint().unwrap();
    let big2 = 2.to_biguint().unwrap();
    let big3 = 3.to_biguint().unwrap();
    let big6 = 6.to_biguint().unwrap();
    let big9 = 9.to_biguint().unwrap();

    if n == big2 || n == big3 { return true }
    if n < big2 || &n % &big2 == big0 { return false }
    if n < big9 { return false }
    if &n % &big3 == big0 { return false }
    let border = n.sqrt();
    let mut i = 11.to_biguint().unwrap();
    while i <= border {
        if &n % &i == big0 || &n % (&i + &big2) == big0 { return false }
        i = i + &big6;
    }
    true
}

pub fn calculate_rounds(key_length: usize, probability_modifier: usize) -> BigUint {
    let mut length_to_calc = key_length;
    if probability_modifier == 1 {
        length_to_calc /= 2;
    }
    if probability_modifier != 0 {
        let v = 2_usize.pow(probability_modifier as u32);
        if v >= key_length {
            return 1.to_biguint().unwrap();
        }
        length_to_calc /= v;
    }
    let tmp = max_biguint(length_to_calc);
    let rounds = &tmp / 6.to_biguint().unwrap();

    rounds
}

pub fn probably_prime(n: BigUint, rounds: BigUint) -> bool {
    let big0 = 0.to_biguint().unwrap();
    let big1 = 1.to_biguint().unwrap();
    let big2 = 2.to_biguint().unwrap();
    let big6 = 6.to_biguint().unwrap();

    if n == big2 || n == 3.to_biguint().unwrap() { return true }
    if n < big2 || &n % &big2 == big0 { return false }
    if n < 9.to_biguint().unwrap() { return false }
    if &n % 3.to_biguint().unwrap() == big0 { return false }
    let mut count = 1.to_biguint().unwrap();
    let border = n.sqrt();
    let mut i = 11.to_biguint().unwrap();
    while count <= rounds && i < border {
        if &n % &i == big0 || &n % (&i + &big2) == big0 { return false }
        i = i + &big6;
        count = count + &big1
    }
    true
}

pub fn random_in_range(min: BigUint, max: BigUint) -> BigUint {
    let bit_length = max.to_bytes_be().len() * 8;
    let mut rng = rng();

    let mut result;
    loop {
        let mut bytes = vec![0u8; bit_length / 8]; // bits / 8 = bytes
        rng.fill(&mut bytes[..]);
        result = BigUint::from_bytes_be(&bytes);
        if result > min {
            break;
        }
    }

    result
}

pub fn random_in_fix_length(bit_length: usize) -> BigUint {
    let mut rng = rng();
    let mut bytes = vec![0u8; bit_length / 8]; // bits / 8 = bytes
    rng.fill(&mut bytes[..]);

    bytes[0] |= 0x80; //first bit 1

    BigUint::from_bytes_be(&bytes)
}

pub fn ggt(n1: BigUint, n2: BigUint) -> BigUint {
    let rest = n1 % &n2;
    if rest == 0.to_biguint().unwrap() {
        return n2;
    }
    ggt(n2, rest)
}
