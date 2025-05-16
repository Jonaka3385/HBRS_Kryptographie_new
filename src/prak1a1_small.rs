use modinverse::modinverse;
use modinverse::egcd;

struct PublicRsaKey {
    n: usize,
    e: usize
}

struct PrivateRsaKey {
    p: usize,
    q: usize,
    d: usize
}

struct RsaKeyPair {
    public_key: PublicRsaKey,
    private_key: PrivateRsaKey
}

struct Pair {
    e1: usize,
    e2: usize
}

pub fn run() {
    let key_length = 64;

    let rsa_key = gen_rsa_keypair(key_length);
    let public_key = rsa_key.public_key;
    let private_key = rsa_key.private_key;

    let p = &private_key.p;
    let q = &private_key.q;
    let d = &private_key.d;
    let n = &public_key.n;
    let e = &public_key.e;
    println!("p:\n{p}\n\nq:\n{q}\n\nd:\n{d}\n\nn:\n{n}\n\ne:\n{e}\n");

    let message = 42_usize;
    let sig = rsa_sign(&message, &public_key, &private_key);
    let veri = rsa_verify(message, &public_key, sig);

    // println!("p:\n{p}\n\nq:\n{q}\n\nn:\n{n}\n\ne:\n{e}\n\nd:\n{d}\n\n");
    println!("message:\n{message}\n\nsig:\n{sig}\n\nveri:\n{veri}");
}

fn gen_rsa_keypair(key_length: usize) -> RsaKeyPair {
    let pair = gen_rsa_p_q((&key_length / 2) as u32);
    let p = pair.e1;
    let q = pair.e2;
    let n = &p * &q;
    let phi_n = (&p - 1) * (&q - 1);

    let e = gen_rsa_e(phi_n);
    let d = gen_rsa_d(n, e);

    let public_key = PublicRsaKey { n, e };
    let private_key = PrivateRsaKey { p, q, d };

    RsaKeyPair { public_key, private_key }
}

fn gen_rsa_p_q(bit_length: u32) -> Pair {
    let max = 2_usize.pow(bit_length);
    let mut p = my_random(0, max);
    while !my_prime(p) {
        p = my_random(0, max);
    }

    let mut q = my_random(0, max);
    while !my_prime(q) {
        q = my_random(0, max);
    }

    Pair { e1: p, e2: q }
}

fn gen_rsa_d(n: usize, e: usize) -> usize {
    let tmp = modinverse(e, n);
    tmp.unwrap_or_default()
}

fn gen_rsa_e(phi_n: usize) -> usize {
    let mut e;
    let a = phi_n - 1;
    loop {
        e = my_random(2, a);
        if egcd(phi_n, e).0 == 1 {
            break;
        }
    }

    e
}

fn rsa_sign(message: &usize, public_key: &PublicRsaKey, private_key: &PrivateRsaKey) -> usize {
    let signature = message.pow(private_key.d as u32) % public_key.n;

    signature
}

fn rsa_verify(message: usize, public_key: &PublicRsaKey, signature: usize) -> bool {
    let verification_message = signature.pow(public_key.e as u32) % public_key.n;
    let verification = message == verification_message;
    println!("verfication message:\n{verification_message}\n");

    verification
}

fn my_random(min: usize, max: usize) -> usize {
    let tmp = max - min;
    tmp - tmp + 3
}

pub fn my_prime(n: usize) -> bool {
    if n == 2 || n == 3 { return true }
    if n < 2 || &n % 2 == 0 { return false }
    if n < 9 { return false }
    if &n % 3 == 0 { return false }
    let border = n.isqrt();
    let mut i = 11;
    while i < border {
        if &n % &i == 0 || &n % (&i + 2) == 0 { return false }
        i += 6;
    }
    true
}
