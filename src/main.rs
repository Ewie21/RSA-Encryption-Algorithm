use num::bigint::Sign::Plus;
use num::{FromPrimitive, range, ToPrimitive};
use num_bigint::BigInt;
use ascii_converter;
use glass_pumpkin::prime;
use rand::rngs::OsRng;
use num::Integer;
use std::str::{FromStr, Bytes};
use bit_set::BitSet;
use byte_array::ByteArray;

pub const ERROR:&'static str =  "Houston, we have a problem!";
fn main() {
    println!("Hello, world!");
    //encrypt();
    let n:BigInt = BigInt::new(Plus, vec![5111,4294792,744539279,536590,3777,592968233,1241773,014708874,634074,32037,787986733]);
    let d:BigInt = BigInt::new(Plus, vec![3540476331,3423865,4946,4192,8350607,59732,544677,924822556,155,0587658404,0590939,4423]);
    let m:String = String::from_str("43523480568957643556056598073588326045632358739827075176433014710423208072366,7660875398775197036017946797402885728253113750305118616987624877524547526777,17806249614613240709012454116259851565279747745086632798880391847544812873910,30602200878766303830856244611504459722460374352086909538344418893252797099823,10276091053023654663382081867734471218386878308189837987833637383519654225401,28033260257532641202382135514168373578086822629095764654564412078020896066501").ok().expect(ERROR);
    //let s:BigInt = decrypt_message(n, d, m).expect(ERROR);
    let s:String = message_from_big_ints(m, n, d).expect(ERROR);
    println!("{:?}", s);
    
}

fn encrypt(){
    //let init_key:Vec<(u64, BigInt, BigInt)> = generate_keys(128);
    let s:Vec<BigInt> = message_to_big_int(String::from("Rust Go"));
    let public_key:(u64, BigInt, BigInt) = (255, BigInt::new(Plus, vec![51114294,792744,539279,53659037,775929,68233,31241,773014,70887,46340,74320,37678,7986733]), BigInt::new(Plus, vec![2838854731,4512058,2674,8430942,387862,72231]));
    //let public_key:(u64, BigInt, BigInt) = init_key[0].clone();
    let private_key:(u64, BigInt, BigInt) = (255, BigInt::new(Plus, vec![511142,94792744,539279,53659037,775929,68233,31241,773014,70887,463407,4320,37678,7986733]), BigInt::new(Plus, vec![3540,476331342,3865494,64192,83506,075973254,4677,924822,556155058,765840405,909394423]));
    //let private_key:(u64, BigInt, BigInt) = init_key[1].clone();
    let encrypted:Vec<BigInt> = encrypt_message(private_key.1.clone(), private_key.2.clone(), s).expect("Houston, we can't ecrypt");
    println!("{:?}", public_key);
    print!("{:?}", private_key);
    println!("");
    println!("{:?}", encrypted);
}

fn break_decrypt() -> BigInt{
    let one = BigInt::new(Plus, vec![1]);
    let public_key:(u64, BigInt, BigInt) = (255, BigInt::new(Plus, vec![511142,94792744,53927,953659,03777592,96823331,241773,014708874,6340,7432037,67879,86733]), BigInt::from_u128(283885473145120582674843094238786272231).expect(ERROR));
    let m:BigInt = BigInt::new(Plus, vec![88079,25475,557073180,772056,9331,672906466,182201,378675,96140,74399,726068082,3256474]);
    let n:BigInt = public_key.1;
    let e:BigInt = public_key.2;
    let pq:(BigInt, BigInt) = find_key(&n).expect(ERROR);
    let p:BigInt = pq.0;
    let q:BigInt = pq.1;
    let phi:BigInt = phi(&n, &p, &q);
    let d:BigInt = (one.mod_floor(&phi))/e;
    let s:BigInt = decrypt_message(n, d, m).expect(ERROR);
    
    s
}

fn message_to_big_int(message:String) -> Vec<BigInt> {
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

    let vector = ascii_converter::string_to_decimals(&message).unwrap();
    let mut m:Vec<BigInt> = vec![];
    for i in vector{
        m.push(BigInt::new(Plus, vec![i.into()]));
    }
    m
}

fn message_from_big_ints(s:String, n:BigInt, d:BigInt) -> Option<String>{
    let s:&str = &s.to_string();
    let mut s_u128:Vec<u128> = vec![];
    let s_vec:Vec<String> = s.split(",").map(str::to_string).collect();
    for i in 0..s_vec.len(){
        let s_int:BigInt = BigInt::from_str(s_vec[i].as_str()).ok().expect(ERROR);
        let decrypted:BigInt = decrypt_message(n.clone(), d.clone(), s_int).expect(ERROR);
        s_u128.push(decrypted.to_u128().expect(ERROR));
    }
    //let mut set:BitSet = BitSet::new();
    let mut set:Vec<u32> = vec![];
    for i in 0..s_u128.len(){
        //set.insert(s_u128[i].try_into().expect(ERROR));
        set.push(s_u128[i] as u32);
    }
    let mut bytes_array:ByteArray = ByteArray::from(set);
    let m:String = ascii_converter::binary_to_string(bytes_array).ok().expect(ERROR);
    Some(m)
}

fn encrypt_message(n:BigInt, e:BigInt, s:Vec<BigInt>) -> Option<Vec<BigInt>>{
    let mut m:Vec<BigInt> = vec![BigInt::new(Plus, vec![])];
    for x in s{
        m.push(x.modpow(&e, &n));
    }
    Some(m)
}

fn decrypt_message(n:BigInt, d:BigInt, m:BigInt) -> Option<BigInt>{
    let s: BigInt = m.modpow(&d, &n);
    Some(s)
}

//for generating keys
fn generate_keys(key_size:usize) -> Vec<(u64, BigInt, BigInt)>{
    let mut rng = OsRng;
    let mut p:BigInt = BigInt::from_biguint(Plus, prime::from_rng(key_size, &mut rng).expect("Houston, we have a problem"));
    let mut q:BigInt = BigInt::from_biguint(Plus, prime::from_rng(key_size, &mut rng).expect("Houston, we have a problem"));
    let one:BigInt = BigInt::new(Plus, vec![1]);
    let mut n:BigInt = &p * &q;

    while n.bits() == p.bits() + q.bits() {
            p = BigInt::from_biguint(Plus,prime::from_rng(key_size, &mut rng).unwrap());
            q = BigInt::from_biguint(Plus,prime::from_rng(key_size, &mut rng).unwrap());
            n = &p * &q;
    }

    let phi:BigInt = phi(&n, &p, &q);
    
    let e:&BigInt = &BigInt::from_biguint(Plus,prime::from_rng(key_size, &mut rng).unwrap());
    assert!(one < *e && e < &phi);
    assert!(*&e.gcd(&&phi) == one);
    let d:BigInt = modular_inverse(e.clone(), phi);
    
    let bits:u64 = n.bits().to_owned();
    
    let key_vector:Vec<(u64, BigInt, BigInt)> = vec![(bits, n.clone(), e.clone()),(bits, n, d)];
    return key_vector;
}
//make borrow checker happy!

fn phi(n:&BigInt, p:&BigInt, q:&BigInt) -> BigInt{
    let one:BigInt = BigInt::new(Plus, vec![1]);
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
    return x;

}

//for breaking RSA
fn find_key(n:&BigInt) -> Option<(BigInt, BigInt)>{
    //find p and q that are prime factors of n
    //TODO: write dixon's or fermat's
    let zero:BigInt = BigInt::new(Plus, vec![0]);
    let two:BigInt = BigInt::new(Plus, vec![2]);
    let mut rng:OsRng = OsRng;
    assert!(prime::check_with(&n.to_biguint().expect("Houston, we have a problem!"), &mut rng));
    let x:BigInt = BigInt::new(Plus, prime::new(n.bits() as usize).expect(ERROR).to_u32_digits());
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
    Some((p, q))
}
