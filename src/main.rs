use num::bigint::Sign::Plus;
use num::{FromPrimitive};
use num_bigint::BigInt;
use ascii_converter;
use glass_pumpkin::prime;
use rand::rngs::OsRng;
use num::Integer;

pub const ERROR:&'static str =  "Houston, we have a problem!";
fn main() {
    println!("Hello, world!");
    encrypt();
    
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

fn decrypt() -> BigInt{
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

fn message_from_big_int(s:BigInt) -> Option<String>{
    //let message:String = ;
    None
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
