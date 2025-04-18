use crate::biguint_functions::*;
use num_bigint::{BigUint, ToBigUint};

struct PublicRsaKey {
    n: BigUint,
    e: BigUint
}

struct PrivateRsaKey {
    p: BigUint,
    q: BigUint,
    d: BigUint
}

struct RsaKeyPair {
    public_key: PublicRsaKey,
    private_key: PrivateRsaKey
}

struct BigPair {
    e1: BigUint,
    e2: BigUint
}

pub fn run() {
    let key_length = 3072;

    let rsa_key = gen_rsa_keypair(key_length);
    let public_key = rsa_key.public_key;
    let private_key = rsa_key.private_key;

    let p = &private_key.p;
    let q = &private_key.q;
    let d = &private_key.d;
    let n = &public_key.n;
    let e = &public_key.e;
    println!("p:\n{p},\n\nq:{q},\n\nd:\n{d},\n\nn:\n{n},\n\ne:\n{e}\n");

    let message = random_in_fix_length(1024);
    let sig = rsa_sign(&message, &public_key, &private_key);
    let veri = rsa_verify(&message, &public_key, &sig);

    // println!("p:\n{p}\n\nq:\n{q}\n\nn:\n{n}\n\ne:\n{e}\n\nd:\n{d}\n\n");
    println!("message:\n{message}\n\nsig:\n{sig}\n\nveri:\n{veri}");
}

fn gen_rsa_keypair(key_length: usize) -> RsaKeyPair {
    let modifier = 7_usize; // everything smaller than 7 takes way too long
    // 7 -> 1 403 584 rounds fuer 3072 bit
    // 6 -> 23 548 233 345 728 rounds fuer 3072 bit
    let rounds = calculate_rounds(key_length, modifier);
    let pair = gen_rsa_p_q(&key_length / 2, rounds.clone());
    let p = pair.e1;
    let q = pair.e2;
    let n = &p * &q;
    let big1 = 1.to_biguint().unwrap();
    let phi_n = (&p - &big1) * (&q - &big1);

    let e = gen_rsa_e(phi_n.clone());
    let d = gen_rsa_d(n.clone(), e.clone());

    let public_key = PublicRsaKey { n, e };
    let private_key = PrivateRsaKey { p, q, d };

    RsaKeyPair { public_key, private_key }
}

fn gen_rsa_p_q(bit_length: usize, rounds: BigUint) -> BigPair {
    let mut p = random_in_fix_length(bit_length);
    while !probably_prime(p.clone(), rounds.clone()) {
        p = random_in_fix_length(bit_length);
    }

    let mut q = random_in_fix_length(bit_length);
    while !probably_prime(q.clone(), rounds.clone()) {
        q = random_in_fix_length(bit_length);
    }

    BigPair { e1: p, e2: q }
}

fn gen_rsa_d(n: BigUint, e: BigUint) -> BigUint {
    e.modinv(&n).unwrap()
}

fn gen_rsa_e(phi_n: BigUint) -> BigUint {
    let big1 = 1.to_biguint().unwrap();
    let big2 = 2.to_biguint().unwrap();
    let mut e;
    let a = &phi_n - &big1;
    loop {
        e = random_in_range(big2.clone(), a.clone());
        if ggt(phi_n.clone(), e.clone()) == big1 {
            break;
        }
    }

    e
}

fn rsa_sign(message: &BigUint, public_key: &PublicRsaKey, private_key: &PrivateRsaKey) -> BigUint {
    let signature = message.modpow(&private_key.d, &public_key.n);

    signature
}

fn rsa_verify(message: &BigUint, public_key: &PublicRsaKey, signature: &BigUint) -> bool {
    let verification_message = signature.modpow(&public_key.e, &public_key.n);
    let verification = message == &verification_message;
    println!("verfication message:\n{verification_message}\n");

    verification
}
