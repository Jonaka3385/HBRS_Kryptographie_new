use num_bigint::{BigUint, ToBigUint};
use rand::{rng, Rng};

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

pub fn run(key_length: usize) {
    let rsa_key = gen_rsa_keypair(key_length);
    let public_key = rsa_key.public_key;
    let private_key = rsa_key.private_key;

    let p = &private_key.p;
    let q = &private_key.q;
    let d = &private_key.d;
    let n = &public_key.n;
    let e = &public_key.e;
    println!("p:\n{p},\n\nq:{q},\n\nd:\n{d},\n\nn:\n{n},\n\ne:\n{e}\n\n");

    let m = random_in_fix_length(32);
    let sig = rsa_sign(&m, &private_key, &public_key);
    let veri = rsa_verify(&m, &public_key, &sig);

    // println!("p:\n{p}\n\nq:\n{q}\n\nn:\n{n}\n\ne:\n{e}\n\nd:\n{d}\n\n");
    println!("m:\n{m}\n\nsig:\n{sig}\n\nveri:\n{veri}");
}

fn random_in_fix_length(bit: usize) -> BigUint {
    let mut rng = rng();
    let mut bytes = vec![0u8; bit / 8]; // bits / 8 = bytes
    rng.fill(&mut bytes[..]);

    bytes[0] |= 0x80; //first bit 1

    BigUint::from_bytes_be(&bytes)
}

fn max_biguint(bit_length: usize) -> BigUint {
    let count = bit_length / 8;
    let mut bytes = vec![0u8; count]; // bits / 8 = bytes

    let mut i = 0;
    while i < count {
        bytes[i] |= 0x80;
        i += 1;
    }

    BigUint::from_bytes_be(&bytes)
}

/*
fn random_in_range(bit: usize) -> BigUint {
    let mut rng = rng();
    let mut bytes = vec![0u8; bit / 8]; // bits / 8 = bytes
    rng.fill(&mut bytes[..]);

    BigUint::from_bytes_be(&bytes)
}
*/

fn gen_rsa_keypair(key_length: usize) -> RsaKeyPair {
    let modifier = 7_usize; // everything smaller than 7 takes way too long
    // 7 -> 1 403 584 rounds
    // 6 -> 23 548 233 345 728 rounds
    let rounds = calculate_rounds(key_length, modifier);
    let pair = gen_rsa_p_q(&key_length / 2, rounds.clone());
    let p = pair.e1;
    let q = pair.e2;
    let n = &p * &q;

    let e = gen_rsa_e();
    let d = gen_rsa_d();

    let public_key = PublicRsaKey { n, e };
    let private_key = PrivateRsaKey { p, q, d };

    RsaKeyPair { public_key, private_key }
}

fn calculate_rounds(key_length: usize, probability_modifier: usize) -> BigUint {
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

fn gen_rsa_p_q(l: usize, rounds: BigUint) -> BigPair {
    let mut p = random_in_fix_length(l);
    while !probably_prime(p.clone(), rounds.clone()) {
        p = random_in_fix_length(l);
    }

    let mut q = random_in_fix_length(l);
    while !probably_prime(q.clone(), rounds.clone()) {
        q = random_in_fix_length(l);
    }

    BigPair { e1: p, e2: q }
}

fn gen_rsa_d() -> BigUint {
    let tmp = 0.to_biguint().unwrap();

    tmp
}

fn gen_rsa_e() -> BigUint {
    let tmp = 1.to_biguint().unwrap();

    tmp
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

fn probably_prime(n: BigUint, rounds: BigUint) -> bool {
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

fn rsa_sign(message: &BigUint, private_key: &PrivateRsaKey, public_key: &PublicRsaKey) -> BigUint {
    let signature = message.modpow(&private_key.d, &public_key.n);

    signature
} // priv&pub -> full???; rsa keys public (keine Übegabe an methoden)???

fn rsa_verify(message: &BigUint, public_key: &PublicRsaKey, signature: &BigUint) -> bool {
    let verification_message = signature.modpow(&public_key.e, &public_key.n);
    let verification: bool = message == &verification_message;
    println!("message:\n{message}\n\nverfication message:\n{verification_message}\n");

    verification
}
