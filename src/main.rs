use num::bigint::Sign::Plus;
use num::integer::sqrt;
use num::{FromPrimitive, ToPrimitive, BigUint};
use num_bigint::BigInt;
use ascii_converter;
use glass_pumpkin::prime;
use num::Integer;
use rand::rngs::ThreadRng;
use core::str;
use std::thread::Thread;
use num_prime::RandPrime;
use std::str::{FromStr, from_utf8};
use std::time::Instant;
pub const ERROR:&'static str =  "Houston, we have a problem!";
fn main() {
    println!("Hello, world!");
    let key_size = 64;
    //Generate a key_pair
    let keys = generate_keys(key_size).ok().unwrap();
    let message:&str = &"1000";
    assert_eq!(keys[0].1, keys[1].1);
    let n = &keys[0].1;
    let d = keys[1].2.clone();
    let e = &keys[0].2;

    let encrypted = encrypt(message, &n.to_string(), &d.to_string(), &e.to_string()).unwrap();
    let m_vec:Vec<BigInt> = encrypted.2.to_vec();

    //let n:BigInt = BigInt::new(Plus, vec![5111,4294792,744539279,536590,3777,592968233,1241773,014708874,634074,32037,787986733]);
    //let d:BigInt = BigInt::new(Plus, vec![3540476331,3423865,4946,4192,8350607,59732,544677,924822556,155,0587658404,0590939,4423]);
    //let m:String = String::from_str("43523480568957643556056598073588326045632358739827075176433014710423208072366,7660875398775197036017946797402885728253113750305118616987624877524547526777,17806249614613240709012454116259851565279747745086632798880391847544812873910,30602200878766303830856244611504459722460374352086909538344418893252797099823,10276091053023654663382081867734471218386878308189837987833637383519654225401,28033260257532641202382135514168373578086822629095764654564412078020896066501").ok().expect(ERROR);
    //let s:BigInt = decrypt_message(n, d, m).expect(ERROR);

    //let d:BigInt = break_decrypt().unwrap();
    let s:Vec<BigInt> = decrypt_message(BigInt::from_str(n.clone().to_string().as_str()).ok().unwrap(), d, m_vec).unwrap();
    println!("\n Decrypted {:?}", s);
    //let s:String = decrypt(m, BigInt::from_str(n).ok().unwrap(), d).expect(ERROR);
    //println!("{:?}", s);

}

fn encrypt(message: &str, n:&str, d:&str, e:&str) -> Result<((u64, BigInt, BigInt), (u64, BigInt, BigInt), Vec<BigInt>), String>{
    //let init_key:Vec<(u64, BigInt, BigInt)> = generate_keys(128);
    //let s:Vec<BigInt> = message_to_vec_big_int(String::from(message)).unwrap();
    let s:Vec<BigInt> = vec![BigInt::from_str(message).unwrap()];
    let public_key:(u64, BigInt, BigInt) = (255, BigInt::from_str(n).unwrap(), BigInt::from_str(e).unwrap());
    //let public_key:(u64, BigInt, BigInt) = init_key[0].clone();
    let private_key:(u64, BigInt, BigInt) = (255, BigInt::from_str(n).unwrap(), BigInt::from_str(d).unwrap());
    //let private_key:(u64, BigInt, BigInt) = init_key[1].clone();
    let encrypted:Vec<BigInt> = encrypt_vector(private_key.1.clone(), private_key.2.clone(), s.clone()).expect("Houston, we can't ecrypt");
    println!("Public Key {:?}", public_key);
    print!("Private Key {:?}", private_key);
    println!("");
    println!("Original Message {:?}", s);
    println!("Encrypted Message {:?}", encrypted);

    Ok((public_key, private_key, encrypted))
}

fn break_decrypt() -> Option<BigInt>{
    let one:BigInt = BigInt::new(Plus, vec![1]);
    let public_key:(u64, BigInt, BigInt) = (255, BigInt::new(Plus, vec![511142,94792744,53927,953659,03777592,96823331,241773,014708874,6340,7432037,67879,86733]), BigInt::from_u128(283885473145120582674843094238786272231).expect(ERROR));
    let m:BigInt = BigInt::new(Plus, vec![88079,25475,557073180,772056,9331,672906466,182201,378675,96140,74399,726068082,3256474]);
    let n:BigInt = public_key.1;
    let e:BigInt = public_key.2;
    let pq:(BigInt, BigInt) = fermat(&n).expect(ERROR);
    let p:BigInt = pq.0;
    let q:BigInt = pq.1;
    let phi:BigInt = phi(&n, &p, &q).unwrap();
    let d:BigInt = (one.mod_floor(&phi))/e;
    //let s:BigInt = decrypt_message(n, d, m).expect(ERROR);
    
    Some(d)
}

fn message_to_big_int(message:String) -> Option<BigInt> {
    /* 
    let vector:Vec<u8> = message.as_bytes().to_vec();
    let mut new_vector:Vec<u32> = vec![];
    let mut count = 0;
    for i in vector{
        new_vector.push(i as u32);
        count+=1;
    }
    let m:BigInt = BigInt::new(Plus, new_vector);
    m
    */

    let vector:Vec<u8> = ascii_converter::string_to_decimals(&message).unwrap();
    let mut m_vec:Vec<u32> = vec![];
    for i in vector{
        m_vec.push(i as u32);
    }
    let m = BigInt::new(Plus, m_vec);

    Some(m)
}

fn decrypt(s:String, n:BigInt, d:BigInt) -> Option<Vec<BigInt>>{
    //let mut s_u128:Vec<u128> = vec![];
    let s_vec:Vec<BigInt> = string_to_bigint_vec(s).unwrap();

    let decrypted:Vec<BigInt> = decrypt_message(n.clone(), d.clone(), s_vec).expect(ERROR);
    //Big Integer is too large
    //s_u128.push(decrypted.to_u128().unwrap());
    
    //let mut bytes_array:Vec<Vec<u8>> = vec![];
    // for i in 0..s_u128.len() {
    //     bytes_array.push(u8::to_ne_bytes(s_u128[i].try_into().expect(ERROR)).to_vec());
        
    // }
    // let new_bytes_array:Vec<u8> = bytes_array.concat();
    // let m:String = from_utf8(&new_bytes_array).ok().expect(ERROR).to_owned();

    // Some(m)

    Some(decrypted)
}

fn string_to_bigint_vec(str:String) -> Option<Vec<BigInt>>{

    let str_vec:Vec<String> = str.split(",").map(str::to_string).collect();
    let mut big_vec:Vec<BigInt> = vec![];
    for i in 0..str_vec.len(){
        big_vec.push(BigInt::from_str(&str_vec[i]).ok().unwrap());
    }

    Some(big_vec)
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

fn decrypt_message(n:BigInt, d:BigInt, m:Vec<BigInt>) -> Option<Vec<BigInt>>{
    let mut s:Vec<BigInt> = vec![];
    for i in 0..m.len(){
        s.push(crypt(&n, &d, &m[i]).unwrap())
    }

    Some(s)
}

//for generating keys
fn generate_keys(key_size:usize) -> Result<Vec<(u64, BigInt, BigInt)>, &'static str>{
    let mut rng = rand::thread_rng();
    let up:BigUint = rng.gen_prime(key_size/2, None);
    let mut p:BigInt = BigInt::from_biguint(Plus, up);
    let up:BigUint = rng.gen_prime(key_size/2, None);
    let mut q:BigInt = BigInt::from_biguint(Plus, up);
    let one:BigInt = BigInt::new(Plus, vec![1]);
    let mut n:BigInt = &p * &q;

    while n.bits() != p.bits() + q.bits() {
            let up:BigUint = rng.gen_prime(key_size/2, None);
            p = BigInt::from_biguint(Plus,up);
            let up:BigUint = rng.gen_prime(key_size/2, None);
            q = BigInt::from_biguint(Plus,up);
            n = &p * &q;
    }

    let phi:BigInt = phi(&n, &p, &q).unwrap();
    let up:BigUint = rng.gen_prime(key_size/2, None);
    let e:&BigInt = &BigInt::from_biguint(Plus,up);
    assert!(one < *e && e < &phi);
    assert!(*&e.gcd(&&phi) == one);
    let d:BigInt = modular_inverse(e.clone(), phi).expect(ERROR);
    
    let bits:u64 = n.bits().to_owned();
    
    let key_vector:Vec<(u64, BigInt, BigInt)> = vec![(bits, n.clone(), e.clone()),(bits, n, d)];
    
    Ok(key_vector)
}

//make borrow checker happy!

fn phi(n:&BigInt, p:&BigInt, q:&BigInt) -> Option<BigInt>{
    let one:BigInt = BigInt::new(Plus, vec![1]);
    assert!(n > &(p + q));

    Some(n-p-q+one)
}

#[warn(unused_mut)]
fn modular_inverse(a:BigInt,b:BigInt) -> Result<BigInt, &'static str>{
    let one:BigInt = BigInt::from_u32(1).unwrap();
    let zero:BigInt = BigInt::from_u32(0).unwrap();
    if b == one{
        return Ok(one);
    }
    let mut r0:BigInt = b.clone();
    let mut x0:BigInt = zero.clone();
    let mut q:BigInt = BigInt::new(Plus, vec![]);

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
    return Ok(x);

}

//for breaking RSA::TODO: FIX
fn find_key(n:&BigInt) -> Result<Option<(BigInt, BigInt)>, &'static str>{
    //find p and q that are prime factors of n
    //TODO: write dixon's or fermat's
    let zero:BigInt = BigInt::new(Plus, vec![0]);
    let two:BigInt = BigInt::new(Plus, vec![2]);
    let mut rng = rand::thread_rng();

    //assert!(prime::check_with(&n.to_biguint().expect("Houston, we have a problem!"), &mut rng));
    let up:BigUint = rng.gen_prime(n.bits() as usize, None);
    let x:BigInt = BigInt::from_biguint(Plus, up);
    let y:BigInt = x.modpow(&two, &n);
    let diff:BigInt = y-x;
    let k:BigInt = &diff/n;

    assert!(&diff.is_multiple_of(&n));
    let fact1:BigInt = &diff/k;
    let fact2:BigInt = diff/n;
    let mut p:BigInt = two;
    while &fact1 % &p != zero{
        p += 1;
    }
    let q:BigInt= fact1 / &p;

    Ok(Some((p, q)))
}

fn fermat(n:&BigInt) -> Option<(BigInt, BigInt)>{
    let init = Instant::now();
    let one:BigInt = BigInt::new(Plus, vec![1]);
    let sqrt_n:BigInt = n.sqrt();
    let mut x:BigInt = &sqrt_n - 1;
    let mut y:BigInt = BigInt::new(Plus, vec![0]);
    
    while (&x * &x) - (&y * &y) != *n && (&x-&y) != one{
        if (&x * &x) - (&y * &y) < *n{
            x += 1;
            print!("here");
        }else {
            y += 1;
        }
    }
    let q:BigInt = x-y;
    let p:BigInt = n/&q;
    let elapsed = init.elapsed();
    println!("Broke Encryption with Fermat in {:?} milliseconds", elapsed);

    Some((p,q))
}
