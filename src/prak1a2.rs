use crate::biguint_functions::*;
use num_bigint::{BigUint, ToBigUint};

struct BigPair {
    p: BigUint,
    q: BigUint
}

pub fn run() {
    let p_length = 3072_usize;
    let q_length = 256_usize;
    let modifier = 7_usize; // everything smaller than 7 takes way too long
    // 7 -> 1 403 584 rounds fuer 3072 bit
    // 6 -> 23 548 233 345 728 rounds fuer 3072 bit
    let pair = gen_dsa_p_q(q_length, p_length);
    let p = pair.p;
    let q = pair.q;

    println!("p: {:?}, q: {:?}", p, q);
}

fn gen_dsa_p_q(p_length: usize, q_length: usize) -> BigPair {
    let big0 = 0.to_biguint().unwrap();
    let big1 = 1.to_biguint().unwrap();

    let mut p;
    let mut rounds = calculate_rounds(p_length, 7);
    loop {
        p = random_in_fix_length(p_length);
        if probably_prime(p.clone(), rounds.clone()) {
            break;
        }
    }

    let mut q;
    rounds = calculate_rounds(q_length, 7);
    loop {
        q = random_in_fix_length(q_length);
        if probably_prime(q.clone(), rounds.clone()) && (&p - &big1) % &q == big0 {
            break;
        }
    }

    BigPair { p, q }
}
