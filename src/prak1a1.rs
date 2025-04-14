use num_traits::*;
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

pub fn run(bit_length: usize, bit_count: usize) {
    let min: BigUint = 0.to_biguint().unwrap();
    let base: BigUint = 2.to_biguint().unwrap();
    let max: BigUint = base.pow(bit_length);
    let rando = gen_random_3072bit();

    println!("Minimum: \n{min}\n\nMaximum: \n{max}\n\nRandom Number: \n{rando}\n\n");
}

fn gen_random_3072bit() -> BigUint {
    let mut rng = rand::rng();
    let mut bytes = vec![0u8; 384]; // 3072 bits = 384 bytes
    rng.fill(&mut bytes[..]);

    // Ensure the number is exactly 3072 bits by setting the highest bit
    bytes[0] |= 0x80;

    BigUint::from_bytes_be(&bytes)
}

/*
fn generate_p_q(p_l: u32, p_n: u32) -> BigPair {
    let l: BigUint = BigUint::from(p_l);
    let n: BigUint = BigUint::from(p_n);
    let g: BigUint = n;
    let n2: BigUint = l - 1;
    let b: BigUint = BigUint::modpow(&l-1, 1, g);
    while True:
        // generate q
        while True:
            let s: BigUint = randrange(1, 2 ** g);
            let a: BigUint = sha256(to_binary(s)).hexdigest();
            let zz: BigUint = (s + 1) % (2 ** g);
            let z: BigUint = sha256(to_binary(zz)).hexdigest();
            let u: BigUint = int(a, 16) ^ int(z, 16);
            let mask: BigUint = 2 ** (p_n - 1) + 1;  // nn-1 und niedrigste Bit auf 1 setzen rest 0
            let q: BigUint = u | mask;  // u OR mask
            if is_prime(q, 20):
                break;
        // generate p
        let i: BigUint = BigUint::from(0);  // counter
        let j: BigUint = BigUint::from(2);  // offset
        while i < 4096:
            v = [];
            for counter in range(n2 + 1):
                let arg: BigUint = (s + j + counter) % (2 ** g);
                let zzv: BigUint = sha256(to_binary(arg)).hexdigest();
                v.append(int(zzv, 16));
            let w: BigUint = 0;
            for counter2 in range(0, n2):
                let w: BigUint += v[counter2] * 2 ** (160 * counter2);
            let w: BigUint += (v[n2] % 2 ** b) * 2 ** (160 * n2);
            let xx: BigUint = w + 2 ** (p_l - 1);
            let c: BigUint = xx % (2 * q);
            let p: BigUint = xx - c + 1  # p = x - (c-1);
            if p >= 2 ** (p_l - 1):
                if is_prime(p, 10):
                    let pair: BigPair = {p, q};
                    return pair;
            i += 1;
            j += n2 + 1;
}

fn is_prime(n: BigUint) -> bool {
    true
}
*/