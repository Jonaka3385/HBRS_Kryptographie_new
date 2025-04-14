extern crate num_bigint;
use num_bigint::*;

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

pub fn run(bit_length: u32) {
    let base: BigUint = BigUint::from(2_u32);
    let max: BigUint = base.pow(bit_length);
    let min: BigUint = BigUint::from(0_u32);
    println!("Minimum: \n{min}\n\nMaximum: \n{max}");
}

fn rsa_key(len: u32) -> FullRsaKey {
    let n: BigUint = BigUint::from(2_u32);
    let p: BigUint = BigUint::from(2_u32);
    let q: BigUint = BigUint::from(2_u32);
    let e: BigUint = BigUint::from(2_u32);
    let d: BigUint = BigUint::from(2_u32);
    let public_rsa_key: PublicRsaKey = PublicRsaKey { n, e };
    let private_rsa_key = PrivateRsaKey { p, q, d };
    let full_rsa_key = FullRsaKey { public_rsa_key, private_rsa_key };
    full_rsa_key
}

fn rsa_sign() {
    println!("not implemented");
}
