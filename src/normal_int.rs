
use num::{FromPrimitive};
use num_primes::Generator;
use num_u128::u128;
use ascii_converter;
use glass_pumpkin::prime;
use rand::rngs::OsRng;
use num::Integer;
fn main() {
    println!("Hello, world!");
    let init_key:Vec<(u64, u128, u128)> = generate_keys(128);
    let s:u128 = message_to_big_uint(String::from("Rust>Go"));
    let public_key:(u64, u128, u128) = init_key[0].clone();
    let private_key:(u64, u128, u128) = init_key[1].clone();
    let encrypted:u128 = encrypt_message(private_key.1.clone(), private_key.2.clone(), s).expect("Houston, we can't ecrypt");
    println!("{:?}", public_key);
    print!("{:?}", private_key);
    println!("");
    println!("{:?}", encrypted);
}

fn message_to_big_uint(message:String) -> u128 {
    let vector = ascii_converter::string_to_decimals(&message).unwrap();
    let mut new_vector:Vec<u32> = vec![];
    let mut count = 0;
    for i in vector{
        new_vector.push(i as u32);
        count+=1;
    }
    let m:u128 = new_vector.concat();
    m
}

fn encrypt_message(n:u128, e:u128, s:u128) -> Option<u128>{
    let m:u128 =  s.modpow(&e, &n);
    Some(m)
}

fn decrypt_message(n:u128, d:u128, m:u128) -> Option<u128>{
    let s: u128 = m.modpow(&d, &n);
    Some(s)
}

//for generating keys
fn generate_keys(key_size:usize) -> Vec<(u64, u128, u128)>{
    let mut rng = OsRng;
    let mut p:u128 = prime::from_rng(key_size, &mut rng).expect("Houston, we have a problem");
    let mut q:u128 = prime::from_rng(key_size, &mut rng).expect("Houston, we have a problem");
    let one:u128 = u128::from_u32(1).unwrap();

    let mut n:u128 = &p * &q;

    while n.bits() == p.bits() + q.bits() {
            p = prime::from_rng(key_size, &mut rng).unwrap();
            q = prime::from_rng(key_size, &mut rng).unwrap();
            n = &p * &q;
    }

    let phi:u128 = phi(&n, &p, &q, &one);
    
    let e:&u128 = prime::from_rng(key_size, &mut rng).unwrap();
    assert!(one < *e && e < &phi);
    assert!(*&e.gcd(&&phi) == one);
    let d:u128 = modular_inverse(e.clone(), phi);
    
    let bits:u64 = n.bits().to_owned();
    
    let key_vector:Vec<(u64, u128, u128)> = vec![(bits, n.clone(), e.clone()),(bits, n, d)];
    return key_vector;
}
//make borrow checker happy!

fn phi(n:&u128, p:&u128, q:&u128, one:&u128) -> u128{
    assert!(n > &(p + q));
    return n-p-q+one;
}

#[warn(unused_mut)]
fn modular_inverse(a:u128,b:u128) -> u128{
    let one:u128 = u128::from_u32(1).expect("Houston this isn't one");
    let zero:u128 = u128::from_u32(0).expect("Houston this isn't zero");
    if b == one{
        return one;
    }
    let mut r0:u128 = b.clone();
    let mut x0:u128 = zero.clone();
    let mut q:u128 = None;

    let mut r1:u128 = a;
    let mut x1:u128 = one.clone();

    let mut r2:u128 = None;
    let mut x2:u128 = None;
    while r2 != one {
    
        assert!(r1 != zero);
        q = &r0 / &r1;
        r2 = r0 - (&q * &r1);
        x2 = x0 -(q * &x1);

        r0 = r1.to_owned().try_into().unwrap();
        x0 = x1.to_owned().try_into().unwrap();

        x1 = x2.to_owned().try_into().unwrap();
        r1 = r2.to_owned().try_into().unwrap();
        
    }
    let mut x:u128 = x2;
    if x < zero {
        x -= b;
    }
    return x;

}

//for breaking RSA
fn factor(n:u128) -> Option<u128>{
    //find p and q that are prime factors of n
    //TODO: write dixon's or fermat's
    None
}

fn concat(vec: &[u128]) -> u128 {
    vec.iter().fold(0, |acc, elem| acc * 10 + elem)
}