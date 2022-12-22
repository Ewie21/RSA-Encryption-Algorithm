use std::time::Instant;
use num::bigint::Sign::Plus;
use num::{FromPrimitive, BigUint, One, Zero};
use num_bigint::BigInt;
use ascii_converter;
use num::Integer;
use core::str;
use num_prime::RandPrime;
use std::str::{FromStr};
//TODO: LEARN HOW BigInt::new() really works

pub static DEBUG:bool = false;
fn main() {
    //modular_inverse(BigInt::from_u8(3).unwrap(), BigInt::from_u8(26).unwrap()).ok().unwrap();
    //test_generate_encrypt_decrypt();
    //test_generate_encrypt_break_decrypt();
    //test_string_big_int_conversion();
    test_generate_convert_encrypt_break_decrypt_convert();
    //assert_eq!(BigInt::new(Plus, vec![0012]), BigInt::new(Plus, vec![0012]));
    assert_eq!(BigInt::from(128), BigInt::new(Plus, vec![128]));
    assert_eq!(BigInt::new(Plus, vec![1,4,8,4]), BigInt::new(Plus, vec![1,4,8,4]));
    //println!("{:?}", big_pow(BigInt::new(Plus, vec![10]), BigInt::new(Plus, vec![10000000,0000000000,000000000000000000,00000000000000,000000000000,000000000000,000000000000000,000000000000])));
    //decrypt();
}

#[allow(unused)]
fn decrypt(){
    let decrypted:Vec<BigInt> = decrypt_vector(BigInt::new(Plus, vec![5,1,1,1,4,2,9,4,7,9,2,7,4,4,5,3,9,2,7,9,5,3,6,5,9,0,3,7,7,7,5,9,2,9,6,8,2,3,3,3,1,2,4,1,7,7,3,0,1,4,7,0,8,8,7,4,6,3,4,0,7,4,3,2,0,3,7,6,7,8,7,9,8,6,7,3,3]), BigInt::new(Plus, vec![3,5,4,0,4,7,6,3,3,1,3,4,2,3,8,6,5,4,9,4,6,4,1,9,2,8,3,5,0,6,0,7,5,9,7,3,2,5,4,4,6,7,7,9,2,4,8,2,2,5,5,6,1,5,5,0,5,8,7,6,5,8,4,0,4,0,5,9,0,9,3,9,4,4,2,3]), vec![BigInt::new(Plus, vec![4,3,5,2,3,4,8,0,5,6,8,9,5,7,6,4,3,5,5,6,0,5,6,5,9,8,0,7,3,5,8,8,3,2,6,0,4,5,6,3,2,3,5,8,7,3,9,8,2,7,0,7,5,1,7,6,4,3,3,0,1,4,7,1,0,4,2,3,2,0,8,0,7,2,3,6,6]),BigInt::new(Plus, vec![7,6,6,0,8,7,5,3,9,8,7,7,5,1,9,7,0,3,6,0,1,7,9,4,6,7,9,7,4,0,2,8,8,5,7,8,2,5,3,1,1,3,7,5,0,3,0,5,1,1,8,6,1,6,9,8,7,6,2,4,8,7,7,5,2,4,5,4,7,5,2,6,7,7,7]),BigInt::new(Plus, vec![1,7,8,0,6,2,4,9,6,1,4,6,1,3,2,4,0,7,0,9,0,1,2,4,5,4,1,1,6,2,5,9,8,5,1,5,6,5,2,7,9,7,4,7,7,4,5,0,8,6,6,3,2,7,9,8,8,8,0,3,9,1,8,4,7,5,4,4,8,1,2,8,7,3,9,1,0]),BigInt::new(Plus, vec![3,0,6,0,2,2,0,0,8,7,8,7,6,6,3,0,3,8,3,0,8,5,6,2,4,4,6,1,1,5,0,4,4,5,9,7,2,2,4,6,0,3,7,4,3,5,2,0,8,6,9,0,9,5,3,8,3,4,4,4,1,8,8,9,3,2,5,2,9,7,0,9,9,8,2,3]),BigInt::new(Plus, vec![1,0,2,7,6,0,9,1,0,5,3,0,2,3,6,5,4,6,6,3,3,8,2,0,8,1,8,6,7,7,3,4,4,7,1,2,1,8,3,8,6,8,7,8,3,0,8,1,8,9,8,3,7,9,8,7,8,3,3,6,3,7,3,8,3,5,1,9,6,5,4,2,2,5,4,0,1]),BigInt::new(Plus, vec![2,8,0,3,3,2,6,0,2,5,7,5,3,2,6,4,1,2,0,2,3,8,2,1,5,5,1,4,1,6,8,3,7,3,5,7,8,0,8,6,8,2,2,6,2,9,0,9,5,7,6,4,6,5,4,5,6,4,4,1,2,0,7,8,0,2,0,8,9,6,0,6,6,5,0,1])]).unwrap();
    let decrypted_message:String = message_from_big_int(decrypted.clone()).unwrap();
    println!("{:?} {:?}", decrypted, decrypted_message);
}

#[allow(unused)]
fn test_generate_convert_encrypt_break_decrypt_convert() {
    let keys:Vec<(u64, BigInt, BigInt)> = generate_keys(32).ok().unwrap();
    let n:&BigInt = &keys[0].1;
    let e:&BigInt = &keys[0].2;

    let message:String = String::from("life");
    let s:Vec<BigInt> = message_to_big_int(message.clone()).unwrap();
    if DEBUG {println!("Unencrypted: {:?}", s)}; //Constant

    let encrypted:Vec<BigInt> = encrypt_vector(n.clone(), e.clone(), s.clone()).unwrap();
    if DEBUG {println!("Encrypted: {:?}", encrypted)}; //Variable

    let d:BigInt = break_decrypt(n, e).unwrap();
    if DEBUG {println!("d: {:?}", d)}; //Variable

    let decrypted:Vec<BigInt> = decrypt_vector(n.clone(), d, encrypted.clone()).unwrap();
    if DEBUG {println!("Decrypted: {:?}", decrypted)}; //Variable(This is the problem)

    let decrypted_message:String = message_from_big_int(decrypted.clone()).unwrap();

    print!("Message: {:?} Original: {:?} Encrypted: {:?} Decrypted: {:?} Message: {:?}", message, s, encrypted, decrypted, decrypted_message)
}

#[allow(unused)]
fn test_generate_encrypt_break_decrypt(){
    let keys:Vec<(u64, BigInt, BigInt)> = generate_keys(32).ok().unwrap();
    let n:&BigInt = &keys[0].1;
    let e:&BigInt = &keys[0].2;

    let s:Vec<BigInt> = vec![BigInt::from_u128(1000).unwrap()];

    let encrypted:Vec<BigInt> = encrypt_vector(n.clone(), e.clone(), s.clone()).unwrap();

    let d:BigInt = break_decrypt(n, e).unwrap();

    let decrypted:Vec<BigInt> = decrypt_vector(n.clone(), d, encrypted.clone()).unwrap();

    print!("Original: {:?} Encrypted: {:?} Decrypted: {:?}", s, encrypted, decrypted)
}

#[allow(unused)]
fn test_generate_encrypt_decrypt(){
    let keys:Vec<(u64, BigInt, BigInt)> = generate_keys(128).ok().unwrap();
    let n:&BigInt = &keys[0].1;
    let d:BigInt = keys[1].2.clone();
    let e:&BigInt = &keys[0].2;

    let s:Vec<BigInt> = vec![BigInt::from_u32(1000).unwrap()];

    let encrypted:Vec<BigInt> = encrypt_vector(n.clone(), e.clone(), s.clone()).unwrap();

    let decrypted:Vec<BigInt> = decrypt_vector(n.clone(), d, encrypted.clone()).unwrap();

    print!("Original: {:?} Encrypted: {:?} Decrypted: {:?}", s, encrypted, decrypted)
}

#[allow(unused)]
fn test_string_big_int_conversion(){
    let message:String = String::from_str("").ok().unwrap();
    let m_vec:Vec<BigInt> = message_to_big_int(message.clone()).unwrap();
    let final_message:String = message_from_big_int(m_vec.clone()).unwrap();
    println!("Message: {:?} Message Vector: {:?} Final Message: {:?}", message, m_vec, final_message);
}

#[allow(unused_assignments)]
fn modular_inverse(a:BigInt,b:BigInt) -> Result<BigInt, &'static str>{
    let init = Instant::now();
    let one:BigInt = BigInt::from_u32(1).unwrap();
    let zero:BigInt = BigInt::from_u32(0).unwrap();
    if b == one{
        return Ok(one);
    }
    let mut r0:BigInt = b.clone();
    let mut x0:BigInt = zero.clone();
    
    let mut q:BigInt;

    let mut r1:BigInt = a;
    let mut x1:BigInt = one.clone();

    let mut r2:BigInt = BigInt::new(Plus, vec![]);
    let mut x2:BigInt = BigInt::new(Plus, vec![]);
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
    let mut x:BigInt = x2;
    if x < zero {
        x -= b;
    }
    let elapsed = init.elapsed();
    println!("Found Inverse {:?} in {:?}", x, elapsed);
    return Ok(x);

}

fn encrypt_vector(n:BigInt, e:BigInt, s:Vec<BigInt>) -> Option<Vec<BigInt>>{
    let mut m:Vec<BigInt> = vec![];
    for i in 0..s.len(){
        m.push(crypt(&n, &e, &s[i]).unwrap());
    }

    Some(m)
}

fn crypt(a:&BigInt, b:&BigInt, c:&BigInt) -> Option<BigInt>{
    let ret: BigInt = c.modpow(&b, &a);

    Some(ret)
}

fn decrypt_vector(n:BigInt, d:BigInt, m:Vec<BigInt>) -> Option<Vec<BigInt>>{
    let mut s:Vec<BigInt> = vec![];
    for i in 0..m.len(){
        s.push(crypt(&n, &d, &m[i]).unwrap())
    }

    Some(s)
}

fn generate_keys(key_size:usize) -> Result<Vec<(u64, BigInt, BigInt)>, &'static str>{
    let one:BigInt = BigInt::new(Plus, vec![1]);
    let zero:BigInt = BigInt::new(Plus, vec![0]);
    let mut rng = rand::thread_rng();
    let up:BigUint = rng.gen_prime(key_size/2, None);
    let mut p:BigInt = BigInt::from_biguint(Plus, up);

    let up:BigUint = rng.gen_prime(key_size/2, None);
    let mut q:BigInt = BigInt::from_biguint(Plus, up);

    let mut n:BigInt = &p * &q;
    while n.bits() != (key_size as u64).try_into().unwrap(){
        while n.bits() != p.bits() + q.bits() || p.bits() & p.bits() != (key_size/2 as usize).try_into().unwrap() || n.bits() != (key_size as usize).try_into().unwrap(){
            let up:BigUint = rng.gen_prime(key_size/2, None);
            p = BigInt::from_biguint(Plus,up);
    
            let up:BigUint = rng.gen_prime(key_size/2, None);
            q = BigInt::from_biguint(Plus,up);
    
            n = &p * &q;
        }
    }
    while n.bits() != p.bits() + q.bits() || p.bits() & p.bits() != (key_size/2 as usize).try_into().unwrap() || n.bits() != (key_size as usize).try_into().unwrap(){
        let up:BigUint = rng.gen_prime(key_size/2, None);
        p = BigInt::from_biguint(Plus,up);

        let up:BigUint = rng.gen_prime(key_size/2, None);
        q = BigInt::from_biguint(Plus,up);

        n = &p * &q;
    }
    let phi:BigInt = phi(&n, &p, &q).unwrap();

    let mut up:BigUint = rng.gen_prime(key_size/2, None);
    let mut e:BigInt = BigInt::from_biguint(Plus,up);

    assert!(one < e && e < phi);
    assert!(*&e.gcd(&&phi) == one);

    let mut d:BigInt = modular_inverse(e.clone(), phi.clone()).expect("ERROR");

    while e <= zero || d <= zero {
        up = rng.gen_prime(key_size/2, None);
        e = BigInt::from_biguint(Plus,up);

        assert!(one < e && e < phi);
        assert!(*&e.gcd(&phi) == one);

        d = modular_inverse(e.clone(), phi.clone()).expect("ERROR");
    }
    
    let bits:u64 = n.bits().to_owned();
    // let e1:BigInt = &(one.mod_floor(&phi))/&d;
    // assert!(e1 == e);
    assert!(e > zero && d > zero);
    let key_vector:Vec<(u64, BigInt, BigInt)> = vec![(bits, n.clone(), e.clone()),(bits, n, d)];
    println!("{:?}", key_vector);

    Ok(key_vector)
}

fn fermat(n:&BigInt) -> Option<(BigInt, BigInt)>{
    let init = Instant::now();
    let one:BigInt = BigInt::new(Plus, vec![1]);
    let sqrt_n:BigInt = n.sqrt();
    let mut x:BigInt = &sqrt_n - 1;
    let mut y:BigInt = BigInt::new(Plus, vec![0]);
    
    while (&x * &x) - (&y * &y) != *n && (&x-&y) != one {
        if (&x * &x) - (&y * &y) < *n{
            x += 1;
        }else {
            y += 1;
        }
    }

    let q:BigInt = x-y;
    let p:BigInt = n/&q;
    assert!(&p * &q == *n);
    let elapsed = init.elapsed();
    println!("Found factors {:?} and {:?}, of integer {:?}", p, q, n);
    println!("Broke Encryption with Fermat in {:?}", elapsed);

    Some((p,q))
}

fn break_decrypt(n:&BigInt, e:&BigInt) -> Option<BigInt>{
    let pq:(BigInt, BigInt) = fermat(&n).expect("Houston, we have a problem");
    //let pq:(BigInt, BigInt) = dixon(&n).expect("Houston we have a problem");
    let p:BigInt = pq.0;
    let q:BigInt = pq.1;
    assert!(&p * &q == *n);
    let phi:BigInt = phi(&n, &p, &q).unwrap();
    let d:BigInt = modular_inverse(e.to_owned(), phi).ok().unwrap();
    assert!(*e != d);

    Some(d)
}

fn message_to_big_int(message:String) -> Option<Vec<BigInt>> {
    let mut vector:Vec<Vec<u8>> = vec![];
    let message_vector:Vec<&str> = message.split(" ").collect();
    for i in 0..message_vector.len(){
        vector.push(ascii_converter::string_to_decimals(message_vector[i]).unwrap());
    }
    let mut m_vec:Vec<BigInt> = vec![];
    for i in 0..vector.len(){
        m_vec.push(BigInt::from_bytes_be(Plus, &vector[i]));
    }

    Some(m_vec)
}

fn message_from_big_int(m_vec:Vec<BigInt>) -> Option<String>{
    let mut bytes:Vec<Vec<u8>> = vec![];
    for i in m_vec {
        bytes.push(i.to_bytes_be().1);
    }
    let mut message:String = String::from_str("").unwrap();
    let mut count:usize = 0;
    println!("Bytes: {:?}", bytes);
    for i in &bytes {
        message.push_str(ascii_converter::decimals_to_string(&i).expect("Houston!").as_str());
        if count + 1 < bytes.len(){
            message.push_str(" ");
        }
        count += 1;

    }

    Some(message)
}

fn phi(n:&BigInt, p:&BigInt, q:&BigInt) -> Option<BigInt>{
    let one:BigInt = BigInt::new(Plus, vec![1]);
    assert!(n > &(p + q));

    Some(n-p-q+one)
}
#[allow(unused)]
fn quadratic_sieve(_n:&BigInt){
    
}

#[allow(unused)]
fn weird_slow_factor(n:&BigInt) -> Option<(BigInt, BigInt)>{
    let mut a:BigInt = BigInt::new(Plus, vec![2]);
    let mut p:BigInt;
    let one:BigInt = BigInt::one();
    let two:BigInt = BigInt::new(Plus, vec![2]);
    loop {
        while (&a * &a) % n == one.clone() {
            a += 1;
        }
        let b:BigInt = a.pow(((n-&one)/&two).try_into().expect("Houston, this is your fault, find a better way to deal with BigInt exponents")).mod_floor(&two);
        
        if b % n == one.clone() {
            p = (a^((n-&one)/two)-&one).gcd(n);
            break;
        }else {
            continue;
        }
    }
    let q = n/&p;

    Some((p, q))
}

// fn find_realively_primes(n:&BigInt) -> Vec<BigInt>{
//     let coprimes:Vec<BigInt> = vec![];
//     let two:BigInt = BigInt::new(Plus, vec![2]);
//     let one:BigInt = BigInt::one();

//     for i in &two..n {
//         if i.gcd(n) == one{
//             coprimes.push(i);
//         }
//     }

//     coprimes
// }

#[allow(unused)]
fn big_pow(base:BigInt, exponent:BigInt) -> Option<BigInt>{
    let mut ret:BigInt = BigInt::zero();
    let zero:BigInt = BigInt::zero();
    let one:BigInt = BigInt::one();
    let mut count:BigInt = BigInt::zero();
    if exponent == one {
        return Some(base);
    }
    if exponent == zero {
        return Some(one);
    }
    while count < exponent {
        if ret == zero {
            ret += &base;
        }else {
            ret *= &base;
        }
        count += &one;
    }

    Some(ret)
}

#[allow(unused)]
fn dixon(n:&BigInt) {
    let one:BigInt = BigInt::one();
    let zero:BigInt = BigInt::zero();
    let r:BigInt = n.sqrt() + one;
    let fbase:Vec<BigInt> = vec![BigInt::from(2), BigInt::from(3), BigInt::from(5), BigInt::from(7)];


}

#[allow(unused)]
fn fr(r:BigInt, n:BigInt) -> Option<BigInt>{

    Some((&r * &r) % n)
}
