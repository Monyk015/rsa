extern crate gmp;

use gmp::mpz::Mpz;
use std::convert::From;
use gmp::rand::RandState;

fn generate_big_prime(bits: u64) -> Mpz {
    let mut state = RandState::new();
    let random_number = state.urandom_2exp(bits);
    let big_prime = random_number.nextprime();
    big_prime
}

fn generate_key() -> (Mpz, Mpz, Mpz) {
    let first_prime = generate_big_prime(1024);
    let second_prime = generate_big_prime(1024);
    let n = &first_prime * &second_prime;
    let euler_function = (first_prime - 1) * (second_prime - 1);
    let mut e: Mpz = From::<u64>::from(65537);
    while e.gcd(&euler_function) != Mpz::one() {
        e = e + 2;
    }
    let d = e.invert(&euler_function).unwrap();
    (n, e, d)
}

#[test]
fn prime() {
    let mut nextprime = generate_big_prime(1024);
    assert_eq!(nextprime.millerrabin(100), 1);
}

#[test]
fn invert_test() {
    let mut state = RandState::new();
    let a = state.urandom_2exp(1000).nextprime();
    let modulo = state.urandom_2exp(1000).nextprime();
    let c = a.invert(&modulo).unwrap();
    assert_eq!((a * c).modulus(&modulo), Mpz::one());
}

#[test]
fn m_to_the_power_of_e_multiplied_by_d_modulo_n_should_be_m() {
    let (n, e, d) = generate_key();
    let mut state = RandState::new();
    let m = state.urandom_2exp(2000);
    let cipher = m.powm(&(&e*&d), &n);
    // let decrypted = cipher.powm(&d, &n);
    assert_eq!(m, cipher);
}

