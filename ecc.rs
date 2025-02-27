use num_bigint::{BigInt, Sign, ToBigInt};
use num_traits::{Zero, One};

#[derive(Clone, Debug)]
struct Point {
    x: BigInt,
    y: BigInt,
}

impl Point {
    fn new(x: BigInt, y: BigInt) -> Self {
        Point { x, y }
    }
}

fn mod_inv(a: &BigInt, p: &BigInt) -> BigInt {
	let d = ((a % p) +p)%p;
    let mut mn = (p.clone(), d.clone());
    let mut xy = (BigInt::zero(), BigInt::one());

    while mn.1 != BigInt::zero() {
        let quotient = &mn.0 / &mn.1;
        mn = (mn.1.clone(), &mn.0 - &quotient * &mn.1);
        xy = (xy.1.clone(), &xy.0 - &quotient * &xy.1);
    }

    if mn.0 != BigInt::one() {
        panic!("No modular inverse exists");
    }

    (xy.0 + p) % p
}

fn point_add(p1: &Point, p2: &Point, p: &BigInt) -> Point {
	if p1.x == BigInt::from(0) && p1.y == BigInt::from(0) {
		return p2.clone();
	}
	
	if p2.x == BigInt::from(0) && p2.y == BigInt::from(0) {
		return p1.clone();
	}
	
    if p1.x == p2.x && p1.y == p2.y {
        return point_double(p1, p);
    }
	
	let d = (((&p2.x - &p1.x) % p) + p)% p;
	
    let lambda = ((&p2.y - &p1.y) * mod_inv(&d, p)) % p;
    let x3 = (&lambda * &lambda - &p1.x - &p2.x) % p;
    let y3 = (&lambda * (&p1.x - &x3) - &p1.y) % p;

    Point::new(((x3%p) + p)%p, ((y3%p) + p)%p)
}

fn point_double(p: &Point, prime: &BigInt) -> Point {
    let lambda = ((BigInt::from(3) * &p.x * &p.x) * mod_inv(&(BigInt::from(2) * &p.y), prime)) % prime;
    let x3 = (&lambda * &lambda - &p.x * BigInt::from(2)) % prime;
    let y3 = (&lambda * (&p.x - &x3) - &p.y) % prime;

    Point::new(((x3%prime) + prime)%prime, ((y3%prime) + prime)%prime)
}

fn scalar_mult(k: &BigInt, point: &Point, p: &BigInt) -> Point {
    let mut result = Point::new(BigInt::zero(), BigInt::zero());
    let mut base = point.clone();
    let mut scalar = k.clone();

    while scalar > BigInt::zero() {
		
        if &scalar % BigInt::from(2) == BigInt::one() {
            result = point_add(&result, &base, p);
        }
        base = point_double(&base, p);
        scalar /= BigInt::from(2);
    }
    result
}

fn main() {
    let p: BigInt = BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).unwrap();
    let gx: BigInt = BigInt::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap();
    let gy: BigInt = BigInt::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap();
    let n: BigInt = BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).unwrap();

    let g = Point::new(gx, gy);
    let private_key: BigInt = BigInt::parse_bytes(b"1E99423A4ED2761A2F37F94F95CB19EF0F1A12C7D48F253FDF81F3B3B1BFD49D", 16).unwrap();

    let public_key = scalar_mult(&private_key, &g, &p);
    println!("Public Key: ({}, {})", public_key.x, public_key.y);
}
