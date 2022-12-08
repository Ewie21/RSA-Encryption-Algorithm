use num::bigint::Sign::Plus;
use num::FromPrimitive;
use num_bigint::BigInt;
use ascii_converter;
use glass_pumpkin::prime;
use rand::rngs::ThreadRng;
use num::Integer;
use rand::thread_rng;
fn main() {
    println!("Hello, world!");
    let init_key:Vec<(u64, BigInt, BigInt)> = generate_keys(128);
    let m:BigInt = message_to_big_uint(String::from("Rust>Go"));
    println!("{:?}", init_key);
    println!("{:?}", m);
}

fn message_to_big_uint(message:String) -> BigInt {
    let vector = ascii_converter::string_to_decimals(&message).unwrap();
    let mut new_vector:Vec<u32> = vec![];
    let mut count = 0;
    for i in vector{
        new_vector[count] = i as u32;
        count+=1;
    }
    let m:BigInt = BigInt::new(Plus, new_vector);
    m
}

fn encrypt_message(n:BigInt, e:BigInt, s:BigInt) -> Option<BigInt>{
    let m:BigInt =  s.modpow(&e, &n);
    Some(m)
}

fn decrypt_message(n:BigInt, d:BigInt, m:BigInt) -> Option<BigInt>{
    let s: BigInt = m.modpow(&d, &n);
    Some(s)
}

//for generating keys
fn generate_keys(key_size:usize) -> Vec<(u64, BigInt, BigInt)>{

    let mut p:BigInt = BigInt::from_biguint(Plus, prime::new(key_size).expect("Houston we have a problem"));
    let mut q:BigInt = BigInt::from_biguint(Plus,prime::new(key_size).expect("Houston we have a problem"));
    let one:BigInt = BigInt::from_u32(1).unwrap();

    let mut n:BigInt = &p * &q;

    while n.bits() == p.bits() + q.bits() {
            p = BigInt::from_biguint(Plus,prime::new(key_size).unwrap());
            q = BigInt::from_biguint(Plus,prime::new(key_size).unwrap());
            n = &p * &q;
    }

    let phi:BigInt = phi(&n, &p, &q, &one);
    
    let e:&BigInt = &BigInt::from_biguint(Plus,prime::new(key_size).unwrap());
    assert!(one < *e && e < &phi);
    assert!(*&e.gcd(&&phi) == one);
    let d:BigInt = modular_inverse(e.clone(), phi);
    
    let bits:u64 = n.bits().to_owned();
    
    let key_vector:Vec<(u64, BigInt, BigInt)> = vec![(bits, n, e.clone()),(bits, d, e.clone())];
    return key_vector;
}
//make borrow checker happy!

fn phi(n:&BigInt, p:&BigInt, q:&BigInt, one:&BigInt) -> BigInt{
    assert!(n > &(p + q));
    return n-p-q+one;
}

#[warn(unused_mut)]
fn modular_inverse(a:BigInt,b:BigInt) -> BigInt{
    let one:BigInt = BigInt::from_u32(1).expect("Houston this isn't one");
    let zero:BigInt = BigInt::from_u32(0).expect("Houston this isn't zero");
    if b == one{
        return one;
    }
    let one:BigInt = BigInt::from_u32(1).unwrap();
    let mut r0:BigInt = b.clone();
    let mut x0:BigInt = BigInt::new(Plus, vec![]);
    let mut q:BigInt = BigInt::new(Plus, vec![]);

    let mut r1:BigInt = a;
    let mut x1:BigInt = BigInt::new(Plus, vec![]);

    let mut r2:BigInt = BigInt::new(Plus, vec![]);
    let mut x2:BigInt = BigInt::new(Plus, vec![]);
    while r2 != one {
        let x1:BigInt = x2.to_owned().try_into().unwrap();
        let r1:BigInt = r2.to_owned().try_into().unwrap();

        let r0:BigInt = r1.to_owned().try_into().unwrap();
        let x0:BigInt = x1.to_owned().try_into().unwrap();

        q = &r0 / &r1;
        r2 = r0 - (&q * r1);
        x2 = x0 -(q * x1);

    }
    let mut x:BigInt = x2;
    if x < zero {
        x -= b;
    }
    return x;

}

//for breaking RSA
fn factor(n:BigInt) -> Option<BigInt>{
    //find p and q that are prime factors of n
    //TODO: write dixon's or fermat's
    None
}
