use std::ops::Div;
use std::ops::Rem;
use num::BigUint;
use num::FromPrimitive;
use num::bigint;
use num::rational::{Ratio, BigRational};
use rand;
use num_prime::nt_func::{is_prime, factorize, factor};
fn main() {
    println!("Hello, world!");
    //let a:UBig = ubig!(100000000000000000000000000000000);
    let prime:bigint::BigUint = find_prime();
    println!("{}" , prime);
    let a:u32 = 32;
    //let b = a.clone().to_be_bytes();
}

fn message_to_key(m: String) -> Vec<BigUint>{

}

//for generating keys 

fn generate_keys(key_size: int){
    let p:BigUint;
    let q:BigUint;

    let n:BigUint = p*q;

    let phi:UBigint = phi(&n, &p, &q);
}

fn phi(n:&BigUint, p:&BigUint, q:&BigUint){
    return(n-p-q+1);
}

fn modular_inverse(a:&UBigint,b:&UBigInt){
    let i = 0;
    let one:BigUint = BigUint::from_u32(1).unwrap();
    let zero:BigUint = BigUint::from_u32(0).unwrap();
    let mut r1:BigUint;
    let mut r2:BigUint;
    let mut r3:BigUint;
    let mut x1:BigUint;
    let mut x2:BigUint;
    r1 = a;
    r2 = b;
    while r2 != one{
        
        let mut q:&BigUInt = &(r1/r2);
        //TODO:write modular inverse

    }

}

//for breaking RSA
fn factor(n:BigUint) -> Option<UBig>{
    //find p and q that are prime factors of n
    //TODO: write dixon's or fermat's
    None
}