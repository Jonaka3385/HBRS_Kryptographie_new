use std::*;
use num_bigint::*;
use rand::*;

/*
struct PublicRsaKey {
    n: BigUint,
    e: BigUint
}

struct PrivateRsaKey {
    p: BigUint,
    q: BigUint,
    d: BigUint
}

struct FullRsaKey {
    public_rsa_key: PublicRsaKey,
    private_rsa_key: PrivateRsaKey
}
*/

struct BigPair {
    e1: BigUint,
    e2: BigUint
}

pub fn run(bit_length: usize, e: usize) {
    let pair = generate_p_q(bit_length / 2, e);
    let p = pair.e1;
    let q = pair.e2;
    let n = &p * &q;

    println!("p:\n{p}\n\nq:\n{q}\n\nn:\n{n}");
}

fn random_range_fix(bit: usize) -> BigUint {
    let mut rng = rng();
    let mut bytes = vec![0u8; bit / 8]; // bits / 8 = bytes
    rng.fill(&mut bytes[..]);

    bytes[0] |= 0x80; //first bit 1

    BigUint::from_bytes_be(&bytes)
}

/*
fn random_range(bit: usize) -> BigUint {
    let mut rng = rng();
    let mut bytes = vec![0u8; bit / 8]; // bits / 8 = bytes
    rng.fill(&mut bytes[..]);

    BigUint::from_bytes_be(&bytes)
}
*/

fn generate_p_q(l: usize, e: usize) -> BigPair {
    let mut p = random_range_fix(l);
    while !probably_prime(p.clone(), e) {
        p = random_range_fix(l);
    }

    let mut q = random_range_fix(l);
    while !probably_prime(q.clone(), e) {
        q = random_range_fix(l);
    }

    BigPair { e1: p, e2: q }
}

/*
fn is_prime(n: BigUint) -> bool {
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
*/

fn probably_prime(n: BigUint, e: usize) -> bool {
    let max = e.to_biguint().unwrap();
    let big0 = 0.to_biguint().unwrap();
    let big1 = 1.to_biguint().unwrap();
    let big2 = 2.to_biguint().unwrap();
    let big3 = 3.to_biguint().unwrap();
    let big6 = 6.to_biguint().unwrap();
    let big9 = 9.to_biguint().unwrap();

    if n == big2 || n == big3 { return true }
    if n < big2 || &n % &big2 == big0 { return false }
    if n < big9 { return false }
    if &n % &big3 == big0 { return false }
    let mut count = big1.clone();
    let mut i = 11.to_biguint().unwrap();
    while count <= max {
        if &n % &i == big0 || &n % (&i + &big2) == big0 { return false }
        i = i + &big6;
        count = count + &big1
    }
    true
}
