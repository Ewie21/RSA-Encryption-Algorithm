use std::cmp::Ordering;
use std::time::Instant;
use num::bigint::Sign::Plus;
use num::{FromPrimitive, BigUint, One, Zero, abs, ToPrimitive};
use num_bigint::{BigInt, RandBigInt};
use ascii_converter;
use num::Integer;
use core::str;
use num_prime::{RandPrime};
use std::str::{FromStr};
use std::io::{self, Read};
use rand::{thread_rng, Rng};

pub static DEBUG:bool = true;

#[derive(Debug)]
struct PubKey {
    b: u64,
    n: BigInt,
    e: BigInt
}
#[derive(Debug)]
struct PriKey {
    b: u64,
    n: BigInt,
    d: BigInt
}
#[derive(Debug)]
struct FullKey {
    pri_key: PriKey,
    pub_key: PubKey
}

impl FullKey {
    fn generate_keys(key_size:usize) -> Result<FullKey, &'static str>{
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
        let keys:FullKey = FullKey{ pri_key: PriKey{ b: bits, n: n.clone(), d: d.clone()} , pub_key: PubKey{ b: bits, n: n, e: e.clone() } } ;
        println!("Public {:?};{:?};{:?} \nPrivate: {:?};{:?};{:?}", keys.pub_key.b, keys.pub_key.n, keys.pub_key.e, keys.pri_key.b, keys.pri_key.n, keys.pri_key.d);
    
        Ok(keys)
    }
}
//TODO: TRY TO FIGURE OUT HOW TO ENCRYPT AND DECRYPT BASED ON HOW THEY ENCRYPTED
fn main() {
    //modular_inverse(BigInt::from_u8(3).unwrap(), BigInt::from_u8(26).unwrap()).ok().unwrap();
    //test_generate_encrypt_decrypt();
    //test_generate_encrypt_break_decrypt();
    //test_string_big_int_conversion();
    //test_generate_convert_encrypt_break_decrypt_convert();
    //assert_eq!(BigInt::new(Plus, vec![0012]), BigInt::new(Plus, vec![0012]));
    assert_eq!(BigInt::from(128), BigInt::new(Plus, vec![128]));
    assert_eq!(BigInt::new(Plus, vec![1,4,8,4]), BigInt::new(Plus, vec![1,4,8,4]));
    //println!("{:?}", big_pow(BigInt::new(Plus, vec![10]), BigInt::new(Plus, vec![10000000,0000000000,000000000000000000,00000000000000,000000000000,000000000000,000000000000000,000000000000])));
    //println!("{:?}", break_decrypt(&BigInt::new(Plus, vec![3,8,3,7,7,6,2,3,9,6,8,0,7,9,4,3,7,3,6,8,5,9,4,9]), &BigInt::new(Plus, vec![6,4,7,3,1,5,74,5,0,5,9])));
    //break_and_decrypt(BigInt::new(Plus, vec![]), BigInt::new(Plus, vec![]), )
    program();
    //message_to_big_int(String::from("Hello, World!"));
    //test_break_decrypt();
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


#[allow(unused)]
fn fermat(n:&BigInt) -> Option<(BigInt, BigInt)>{
    let init = Instant::now();
    let one:BigInt = BigInt::new(Plus, vec![1]);
    let sqrt_n:BigInt = n.sqrt();
    let mut x:BigInt = &sqrt_n - 1;
    let mut y:BigInt = BigInt::new(Plus, vec![0]);
    
    while (&x * &x) - (&y * &y) != *n && (&x-&y) != one {
        if (&x * &x) - (&y * &y) < *n {
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
    let init = Instant::now();
    if DEBUG {println!("made it to breaking")}
    let pq:(BigInt, BigInt) = pr(n.clone()).expect("Houston, we have a problem");
    let elapsed = init.elapsed();
    println!("Broke Encryption with pr in: {:?}", elapsed);
    let p:BigInt = pq.0;
    let q:BigInt = pq.1;
    assert!(&p * &q == *n);
    let phi:BigInt = phi(&n, &p, &q).unwrap();
    let d:BigInt = modular_inverse(e.to_owned(), phi).ok().unwrap();
    assert!(*e != d);

    Some(d)
}

fn message_to_big_int(message:String) -> Option<Vec<BigInt>> {
    let bytes:Vec<u8> = ascii_converter::string_to_decimals(&message).unwrap();
    println!("{:?}", bytes);
    //Up to here works
    //not functional
    let mut i = 0;
    let mut int: u64;
    let mut int_vec:Vec<u64> = vec![];
    while i < bytes.len() {
        int = 0;
        for j in 0..=7 {
            if i + j >= bytes.len(){
                break;
            }
            //int = (int << 8) | (bytes[i + j]) as u64;
            int |=  ((bytes[i + j]) as u64) << (8 * j);
        }
        println!("{:?}", int);
        i += 8;
        int_vec.push(int);
    }

    let mut m_vec:Vec<BigInt> = vec![];
    for i in 0..int_vec.len(){
        m_vec.push(BigInt::from_u64(int_vec[i]).unwrap());
    }

    Some(m_vec)
}

fn message_from_big_int(m_vec:Vec<BigInt>) -> Option<String>{
    if DEBUG {println!("Message From Big Int Start:")};
    let mut bytes:Vec<Vec<u8>> = vec![];
    let mut int_vec:Vec<u64> = vec![];
    for i in m_vec {
        if DEBUG {println!("{:?}", i)};
        int_vec.push(i.to_u64().unwrap());
    }
    if DEBUG {println!("Integer Array: {:?}", int_vec)}

    for i in int_vec {
        bytes.push(i.to_le_bytes().to_vec());
    }
    let mut byte = bytes.concat();
    byte.retain(|x| *x != 0);
    println!("{:?}", byte);
    let message:String = String::from(ascii_converter::decimals_to_string(&byte).unwrap().as_str());    
    if DEBUG {println!("message: {:?}", message)}

    

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
    let mut ret:BigInt = base.clone();
    let zero:BigInt = BigInt::zero();
    let one:BigInt = BigInt::one();
    let mut count:BigInt = BigInt::zero();

    match &exponent.cmp(&zero) {
        Ordering::Greater => {if exponent == one {Some(base.clone())} else {
            while count < exponent {
                ret *= &base;
                count += &one;
            }
        
            Some(ret)
        }},
        
        Ordering::Equal => Some(one.clone()),

        Ordering::Less => None
    }
}


//Not Written Dixon's
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

fn pr(n: BigInt) -> Option<(BigInt, BigInt)> {
    let mut rng = rand::thread_rng();
    // let mut x:BigInt = rng.gen_bigint(n.bits());
    // let c:BigInt = rng.gen_bigint(n.bits());
    let mut x:BigInt = BigInt::from(2);
    let c:BigInt = BigInt::from(2);
    let mut common:BigInt = BigInt::one();
    let mut y:BigInt = x.clone()/2;

    while common == BigInt::one() {
        x = f(&x, &c, &n);
        y = f(&f(&y, &c, &n), &c, &n);

        common = abs(&x - &y).gcd(&n);
    }
    if &common == &n {
        return None;
    }else {
        return Some((common.clone(), n/common));
    }
}

fn f(x:&BigInt, c:&BigInt, n:&BigInt) -> BigInt{
    return ((x * x) + c) % n
}


fn program(){
    loop {
        let mut action = String::new();
        println!("What action to do you want to perform?: ");
        let _b = std::io::stdin().read_line(&mut action).unwrap();
        let _pop = action.pop();
        //Decryption
        if action.to_lowercase() == String::from("decrypt") {
            decrypt_program();
            break;
        //Encryption
        }else if action.to_lowercase() == String::from("encryptG") {
            encrypt_program();
            break;
        //Breaking Decryption
        }else if action.to_lowercase() == String::from("break") {
            break_decrypt_program();
            break;
        }else if action.to_lowercase() == String::from("encrypt"){
            encrypt_key_program();
        }else {
            println!("You need to input a valid action");
            println!("Decrypt, Encrypt, or Break");
            println!("\n");
            continue;
        }

    }   
    
}

#[allow(unused)]
fn decrypt(n:BigInt, d:BigInt, m:Vec<BigInt>) {
    let decrypted:Vec<BigInt> = decrypt_vector(n, d, m).unwrap();
    let decrypted_message:String = message_from_big_int(decrypted.clone()).unwrap();
    println!("{:?} {:?}", decrypted, decrypted_message);
}

fn decrypt_program(){
    //Key Handling
    let mut init_pri_key = String::new();
    println!("What is the private key?");
    let _b_pub = std::io::stdin().read_line(&mut init_pri_key).unwrap();
    let mut pri_key_ints:Vec<BigInt> = vec![];
    let pri_key_str = init_pri_key.split(";").collect::<Vec<&str>>();
    for i in 0..pri_key_str.len(){
        let mut value = String::from(pri_key_str[i]);
        value.pop();
        println!("{:?}", value);
        value.pop();
        pri_key_ints.push(BigInt::from_str(value.as_str()).unwrap())
    }
    //Message Handling
    let mut message_str = String::new();
    println!("What is the message you want to decrypt");
    let _b_message = std::io::stdin().read_line(&mut message_str).unwrap();
    let message_str_vec:Vec<&str> = message_str.split(",").collect::<Vec<&str>>();
    let mut message_int:Vec<BigInt> = vec![];
    for i in 0..message_str_vec.len(){
        let mut value = String::from(message_str_vec[i]);
        let _pop = value.pop();
        message_int.push(BigInt::from_str(value.as_str()).unwrap());
    }
    
    decrypt(pri_key_ints[1].clone(), pri_key_ints[2].clone(), message_int);
}

fn encrypt_program(){
    let mut message_str = String::new();
    println!("What is the message you want to encrypt");
    let _b_message = std::io::stdin().read_line(&mut message_str).unwrap();
    let _pop = message_str.pop();
    let big_int_vec:Vec<BigInt> = message_to_big_int(message_str).unwrap();
    let keys:FullKey= FullKey::generate_keys(64).ok().unwrap();
    let n:BigInt = keys.pub_key.n;
    let e:BigInt = keys.pub_key.e;
    println!("{:?}", encrypt_vector(n, e, big_int_vec).unwrap());
    
}

fn encrypt_key_program(){
    let mut message_str:String = String::new();
    println!("What is the message you want to encrypt");
    let _b_message = std::io::stdin().read_line(&mut message_str).unwrap();
    let _pop:char = message_str.pop().unwrap();
    let big_int_vec:Vec<BigInt> = message_to_big_int(message_str).unwrap();
    
    let mut keys_str:String = String::new();
    println!("What is the public key you want to encrypt with");
    let _keys_message = std::io::stdin().read_line(&mut keys_str).unwrap();
    let _pop = keys_str.pop();
    let keys_vec = keys_str.split(";").collect::<Vec<&str>>();
    let mut keys_int_vec = vec![];
    for i in 0..keys_vec.len(){
        let mut value = String::from(keys_vec[i]);
        let _pop = value.pop();
        keys_int_vec.push(BigInt::from_str(value.as_str()).unwrap());
    }
    let  b = keys_int_vec[0].bits();
    let keys:PubKey = PubKey { 
        b,
        n: keys_int_vec[0].clone(),
        e: keys_int_vec[1].clone()
    };
    println!("{:?}", encrypt_vector(keys.n, keys.e, big_int_vec));
}

fn break_decrypt_program() {
    let mut init_pub_key:String = String::new();
    println!("What is the public key?");
    let _b_pub = std::io::stdin().read_line(&mut init_pub_key).unwrap();
    let mut pub_key_ints:Vec<BigInt> = vec![];
    let pub_key_str:Vec<&str> = init_pub_key.split(";").collect::<Vec<&str>>();
    for i in 0..pub_key_str.len(){
        let mut value = String::from(pub_key_str[i]);
        let _pop = value.pop();
        pub_key_ints.push(BigInt::from_str(value.as_str()).unwrap());
    }
    let d = break_decrypt(&pub_key_ints[1], &pub_key_ints[2]).unwrap();
    println!("d {:?}", d);
    println!("private key: {:?};{:?};{:?}", pub_key_ints[1].bits(), pub_key_ints[1], d);
    let mut message_str = String::new();
    println!("What is the message you want to decrypt");
    let _b_message = std::io::stdin().read_line(&mut message_str).unwrap();
    let message_str_vec:Vec<&str> = message_str.split(" ").collect::<Vec<&str>>();
    let mut message_int:Vec<BigInt> = vec![];
    for i in 0..message_str_vec.len(){
        println!("{:?}", message_str_vec[i]);
        let mut message_str_neo = message_str_vec[i].to_string();
        message_str_neo.pop();
        message_int.push(BigInt::from_str(message_str_neo.trim()).unwrap());
    }
    let d:BigInt = break_decrypt(&pub_key_ints[0], &pub_key_ints[1]).unwrap();
    decrypt(pub_key_ints[1].clone(), d, message_int);
}

#[allow(unused)]
fn test_generate_convert_encrypt_break_decrypt_convert() {
    let keys:FullKey= FullKey::generate_keys(100).ok().unwrap();
    let n:&BigInt = &&keys.pub_key.n;
    let e:&BigInt = &&keys.pub_key.e;

    let message:String = String::from("Elo is a silly rust user please deoxidize");
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
    let keys:FullKey = FullKey::generate_keys(32).ok().unwrap();
    let n:&BigInt = &&keys.pub_key.n;
    let e:&BigInt = &&keys.pub_key.e;

    let s:Vec<BigInt> = vec![BigInt::from_u128(1000).unwrap()];

    let encrypted:Vec<BigInt> = encrypt_vector(n.clone(), e.clone(), s.clone()).unwrap();

    let d:BigInt = break_decrypt(n, e).unwrap();

    let decrypted:Vec<BigInt> = decrypt_vector(n.clone(), d, encrypted.clone()).unwrap();

    print!("Original: {:?} Encrypted: {:?} Decrypted: {:?}", s, encrypted, decrypted)
}

#[allow(unused)]
fn test_generate_encrypt_decrypt(){
    let keys:FullKey= FullKey::generate_keys(128).ok().unwrap();
    let n:&BigInt = &&keys.pri_key.n;
    let d:BigInt = keys.pri_key.d;
    let e:&BigInt = &keys.pub_key.e;

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

fn test_break_decrypt() {
    let message:String = String::from_str("427082699202961660081635548589,257308990900421338058942854537,574998263658055632316677820304,306521457487953767562670597140,282112942623160744412936572918,212292727044276609518234945856").ok().unwrap(); //message goes here
    let pub_key_str:String = String::from_str("100;705697450299153461734661545213;997654445494429").ok().unwrap(); // key goes here
    let pub_key_split:Vec<&str> = pub_key_str.split(";").collect::<Vec<&str>>();
    let mut pub_key_ints:Vec<BigInt> = vec![];
    for i in pub_key_split {
        pub_key_ints.push(BigInt::from_str(i).unwrap())
    }
    let pub_key:PubKey = PubKey { b: pub_key_ints[0].to_u64().unwrap(), n: pub_key_ints[1].clone(), e: pub_key_ints[2].clone() };
    let m_vec_str = message.split(",").collect::<Vec<&str>>();
    let mut m_vec = vec![];
    for i in m_vec_str {
        m_vec.push(BigInt::from_str(i).unwrap());
    }
    let d = break_decrypt(&pub_key.n, &pub_key.e).unwrap();
    if DEBUG {println!("D: {:?}", d)};
    let decrypted_ints = decrypt_vector(pub_key.n, d, m_vec.clone()).unwrap();
    println!("Decrypted: {:?}", decrypted_ints);
    let str_message = message_from_big_int(m_vec).unwrap();
    println!("Final message: {:?}", str_message);
}