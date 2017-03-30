#![allow(dead_code)]

extern crate gmp;
extern crate libc;

use gmp::mpz::Mpz;
use std::convert::From;
use gmp::rand::RandState;

use libc::*;
use std::ffi::CString;

use std::mem::forget;


fn generate_big_prime(state: &mut RandState, bits: u64) -> Mpz {
    let random_number = state.urandom_2exp(bits);
    let big_prime = random_number.nextprime();
    big_prime
}

#[repr(C)]
pub struct Key {
    n: *mut c_schar,
    e: *mut c_schar,
    d: *mut c_schar,
}

#[no_mangle]
pub extern "C" fn return2() -> int32_t {
    2
}

#[no_mangle]
pub extern "C" fn generate_key() -> Key {
    let mut state = RandState::new();
    let first_prime = generate_big_prime(&mut state, 1024);
    let second_prime = generate_big_prime(&mut state, 1024);
    let n = &first_prime * &second_prime;
    let euler_function = (first_prime - 1) * (second_prime - 1);
    let mut e: Mpz = From::<u64>::from(65537);
    while e.gcd(&euler_function) != Mpz::one() {
        e = e + 2;
    }
    let d = e.invert(&euler_function).unwrap();
    Key {
        n: CString::new(n.to_str_radix(16)).unwrap().into_raw(),
        e: CString::new(e.to_str_radix(16)).unwrap().into_raw(),
        d: CString::new(d.to_str_radix(16)).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn encode(m_str: *mut c_schar, e_str: *mut c_schar, n_str: *mut c_schar) -> *mut c_schar {
    let m = unsafe{Mpz::from_str_radix(&CString::from_raw(m_str).into_string().unwrap(), 16).unwrap()};
    let e = unsafe{Mpz::from_str_radix(&CString::from_raw(e_str).into_string().unwrap(), 16).unwrap()};
    let n = unsafe{Mpz::from_str_radix(&CString::from_raw(n_str).into_string().unwrap(), 16).unwrap()};
    let res = m.powm(&e, &n);
    forget(m);
    forget(m_str);
    forget(e);
    forget(e_str);
    forget(n);
    forget(n_str);
    CString::new(res.to_str_radix(16)).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn decode(c_str: *mut c_schar, d_str: *mut c_schar, n_str: *mut c_schar) -> *mut c_schar {
    let c = unsafe{Mpz::from_str_radix(&CString::from_raw(c_str).into_string().unwrap(), 16).unwrap()};
    let d = unsafe{Mpz::from_str_radix(&CString::from_raw(d_str).into_string().unwrap(), 16).unwrap()};
    let n = unsafe{Mpz::from_str_radix(&CString::from_raw(n_str).into_string().unwrap(), 16).unwrap()};
    let res = c.powm(&d, &n);
    forget(c);
    forget(c_str);
    forget(d);
    forget(d_str);
    forget(n);
    forget(n_str);
    CString::new(res.to_str_radix(16)).unwrap().into_raw()
}

#[test]
fn m_to_the_power_of_e_multiplied_by_d_modulo_n_should_be_m() {
    let (n, e, d) = generate_key();
    let mut state = RandState::new();
    let m = state.urandom_2exp(2000);
    let cipher = encode(&m, &e, &n);
    let decrypted = decode(&cipher, &d, &n);
    assert_eq!(m, decrypted);
}