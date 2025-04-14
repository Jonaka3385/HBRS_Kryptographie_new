use num_traits::*;
use std::*;
use num_bigint::*;
use rand::*;

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

struct BigPair{
    e1: BigUint,
    e2: BigUint
}

pub fn run(bit_length: usize) {
    let pair = generate_p_q(bit_length / 2);
    let p = pair.e1;
    let q = pair.e2;

    println!("p:\n{p}\n\nq:\n{q}");
}

fn random_range_fix(bit: usize) -> BigUint {
    let mut rng = rng();
    let mut bytes = vec![0u8; (bit / 8)]; // bits / 8 = bytes
    rng.fill(&mut bytes[..]);

    bytes[0] |= 0x80; //first bit 1

    BigUint::from_bytes_be(&bytes)
}

fn random_range(bit: usize) -> BigUint {
    let mut rng = rng();
    let mut bytes = vec![0u8; (bit / 8)]; // bits / 8 = bytes
    rng.fill(&mut bytes[..]);

    BigUint::from_bytes_be(&bytes)
}

fn generate_p_q(l: usize) -> BigPair {
    let mut p = random_range(l);
    while !is_prime(p.clone()) {
        p = random_range(l);
    }

    let mut q = random_range(l);
    while !is_prime(q.clone()) {
        q = random_range(l);
    }

    BigPair { e1: p, e2: q }
}

fn is_prime(n: BigUint) -> bool {
    true
}
