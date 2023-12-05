use crate::rabin_wrap::{RabinPrivateKey, RabinPubKey};
use default_gen::*;
use lazy_static::lazy_static;
use num_bigint::{BigInt, BigUint};
use num_traits::{one, One, ToPrimitive, Zero};
use std::cmp::min;
use std::io;

pub mod default_gen;
pub mod rabin_wrap;

const DEFAULT_TRIAL_DIVISION_ITER: u128 = 100;
const DEFAULT_PRIMALITY_TEST_ITER: u128 = 50;

lazy_static! {
    //first 100 numbers checked for primeness
    static ref PRIME_NUMBERS: Vec<BigUint> = {
        let mut vec = Vec::new();
        for i in 2..DEFAULT_TRIAL_DIVISION_ITER{
            let pseudoprime = BigUint::from(i);
            if is_prime(&pseudoprime, &DEFAULT_PRIMALITY_TEST_ITER).unwrap(){
                vec.push(pseudoprime);
            }
        }
        vec
    };
}

static GEN_RANDOM_VALUE: fn(u64) -> BigUint = |bits_to_gen: u64| -> BigUint {
    let mut gen = DefaultGenerator::new();
    gen.take(bits_to_gen as usize)
};

pub static FORMAT_KEYS: fn(&RabinPrivateKey, &RabinPubKey) -> (String, String) =
    |private_key: &RabinPrivateKey, public_key: &RabinPubKey| -> (String, String) {
        (format!("RabinPrivateKey::from_numbers( \n\"{}\", \n\"{}\" )", public_key.n.to_str_radix(16),
                 public_key.b.to_str_radix(16)),
         format!("RabinPrivateKey::from_numbers(\n    \"{}\",\n    \"{}\",\n    [\"{}\",\n    \"{}\"]\n    .to_vec())",
                 public_key.n.to_str_radix(16),
                 public_key.b.to_str_radix(16),
                 private_key.primes[0].to_str_radix(16),
                 private_key.primes[1].to_str_radix(16)
         ))
    };

// x = a (mod n)
#[derive(Clone, Debug)]
pub struct ModuleEquation {
    a: BigUint,
    n: BigUint,
}

pub trait PRNG {
    fn next(&mut self) -> u8;

    fn next_byte(&mut self) -> u8;

    fn take(&mut self, n: usize) -> BigUint;

    fn take_byte_vectorized(&mut self, n: usize) -> Vec<u8>;
}

/// Number is testing for primality with Miller-Rabin test
/// n - number that would be tested
/// k - iterations of random generating bases
pub fn is_prime(p: &BigUint, k: &u128) -> Result<bool, io::Error> {
    if *p == BigUint::one() {
        return Ok(true);
    }
    let two = BigUint::from(2_u32);

    let mut s: u128 = 0;
    let mut d = p - &BigUint::one();
    while &d % &two == BigUint::zero() && d != BigUint::zero() {
        d /= &two;
        s += 1;
    }

    for _ in 1..*k {
        let mut counter: BigUint = BigUint::zero();
        let mut x = GEN_RANDOM_VALUE((p - &BigUint::one()).bits());
        if gcd(&x, &p) > BigUint::one() {
            return Ok(false);
        }

        x = BigUint::modpow(&x, &d, p);
        if x == BigUint::one() || x == p - BigUint::one() {
            continue;
        }

        if s < 1 {
            return Ok(false);
        }

        for _ in 1..s {
            x = (&x * &x) % p;
            if x == p - BigUint::one() {
                break;
            }
            counter += BigUint::one();
        }

        if counter == (&s - BigUint::one()) && x != p - BigUint::one() && x != BigUint::one() {
            return Ok(false);
        }
    }
    return Ok(true);
}

/// Using euclid algorithm to calculate GCD
fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    if a == &BigUint::zero() || b == &BigUint::zero() {
        return BigUint::one();
    }
    let mut a = a.clone();
    let mut b = b.clone();
    let mut d = BigUint::one();

    while (&a & BigUint::one()) == BigUint::zero() && (&b & BigUint::one()) == BigUint::zero() {
        a >>= 1;
        b >>= 1;
        d <<= 1;
    }
    while (&a & BigUint::one()) == BigUint::zero() {
        a >>= 1;
    }
    while b != BigUint::zero() {
        while (&b & BigUint::one()) == BigUint::zero() {
            b >>= 1;
        }

        if &a > &b {
            let a_old = a.clone();
            let b_old = b.clone();

            a = min(a.clone(), b.clone());
            b = &a_old - &b_old;
        } else {
            let a_old = a.clone();
            let b_old = b.clone();

            a = min(a.clone(), b.clone());
            b = &b_old - &a_old;
        }
    }

    return d * a;
}

// Algorithm to find inverse by module using Extended Euclides algorithm
pub fn inverse(a: &BigInt, n: &BigInt) -> Result<BigUint, &'static str> {
    let mut a_mut = a.clone();
    if a >= n {
        a_mut %= n;
    }

    let mut t = BigInt::zero();
    let mut r = n.clone();
    let mut new_t = BigInt::one();
    let mut new_r = a_mut.clone();
    while new_r != BigInt::zero() {
        let quotient = &r / &new_r;
        let new_t_aux = t;
        t = new_t.clone();
        new_t = new_t_aux - &quotient * &new_t;
        let new_r_aux = r; //auxiliary
        r = new_r.clone();
        new_r = new_r_aux - &quotient * &new_r;
    }
    if r > BigInt::one() {
        return Err("number is not invertible");
    }
    if t < BigInt::zero() {
        t += n;
    }
    Ok(t.to_biguint().unwrap())
}

//calculates jacobi symbol
fn calc_jacobi(a: &BigUint, n: &BigUint) -> i8 {
    let one = BigUint::one();
    let two = BigUint::from(2_u32);
    let three = BigUint::from(3_u32);
    let four = BigUint::from(4_u32);
    let five = BigUint::from(5_u32);
    let eight = BigUint::from(8_u32);

    let mut a = a % n;
    let mut n = n.clone();
    let mut t = 1;
    while a != BigUint::zero() {
        while &a % &two == BigUint::zero() {
            a = &a >> 1;
            let r = &n % &eight;
            if r == three || r == five {
                t = -t;
            }
        }

        // (a,p) = (p,a)
        let a_old = a.clone();
        let p_old = n.clone();
        a = p_old;
        n = a_old;

        if &a % &four == three && &n % &four == three {
            t = -t;
        }
        a = &a % &n;
    }
    if n == one {
        return t;
    }
    return 0;
}

fn tonelli_shanks_square_root(y: &BigUint, p: &BigUint, q: &BigUint, n: &BigUint) -> Vec<BigUint> {
    let root_p = tonelli_shanks(y, p);
    let root_q = tonelli_shanks(y, q);

    let equations1 = vec![
        ModuleEquation {
            a: root_p.clone(),
            n: p.clone(),
        },
        ModuleEquation {
            a: root_q.clone(),
            n: q.clone(),
        },
    ];
    let result_1 = solve_equations(&equations1, n).unwrap();

    let equations2 = vec![
        ModuleEquation {
            a: root_p.clone(),
            n: p.clone(),
        },
        ModuleEquation {
            a: q - root_q.clone(),
            n: q.clone(),
        },
    ];
    let result_2 = solve_equations(&equations2, n).unwrap();

    let equations3 = vec![
        ModuleEquation {
            a: p - root_p.clone(),
            n: p.clone(),
        },
        ModuleEquation {
            a: root_q.clone(),
            n: q.clone(),
        },
    ];
    let result_3 = solve_equations(&equations3, n).unwrap();

    let equations4 = vec![
        ModuleEquation {
            a: p - root_p.clone(),
            n: p.clone(),
        },
        ModuleEquation {
            a: q - root_q.clone(),
            n: q.clone(),
        },
    ];
    let result_4 = solve_equations(&equations4, n).unwrap();

    println!("root_p: {}, root_q:{}, res1:{}, res2:{}, res3:{}, res4:{}", root_p, root_q, result_1, result_2, result_3, result_4);

    vec![result_1, result_2, result_3, result_4]
}

// Calculates with quick finding quadratic residue where primes are blum
fn fast_square_root(y: &BigUint, p: &BigUint, q: &BigUint, n: &BigUint) -> Vec<BigUint> {
    let four = BigUint::from(4_u32);

    let s1 = y.modpow(&((p + one::<BigUint>()) / &four), p);
    let s2 = y.modpow(&((q + one::<BigUint>()) / &four), q);

    let p_bigint = BigInt::from(p.clone());
    let q_bigint = BigInt::from(q.clone());

    let u = inverse(&p_bigint, &q_bigint).unwrap();
    let v = inverse(&q_bigint, &p_bigint).unwrap();

    let vqs_1 = (&v * q * &s1) % n;
    let ups_2 = (&u * p * &s2) % n;
    println!("u: {}, v:{}, s1:{}, s2:{}, vqs_1:{}, ups_2:{}", u, v, s1, s2, vqs_1, ups_2);
    println!(
        "s1: {}, s2: {}, roots: {:?}",
        s1, s2,
        vec![
            BigUint::from((&vqs_1 + &ups_2) % n),
            BigUint::from((&vqs_1 + (n - &ups_2)) % n),
            BigUint::from(((n - &vqs_1) + &ups_2) % n),
            BigUint::from(((n - &vqs_1) + (n - &ups_2)) % n),
        ]
    );

    vec![
        BigUint::from((&vqs_1 + &ups_2) % n),
        BigUint::from((&vqs_1 + (n - &ups_2)) % n),
        BigUint::from(((n - &vqs_1) + &ups_2) % n),
        BigUint::from(((n - &vqs_1) + (n - &ups_2)) % n),
    ]
}

// solving module equations by using (Generalized Chinese Remainder Theorem)
fn solve_equations(
    equations_vec: &Vec<ModuleEquation>,
    n: &BigUint,
) -> Result<BigUint, &'static str> {
    let mut m_i = vec![BigUint::zero(); equations_vec.len()];
    let mut m = BigUint::one();

    for equation in equations_vec.iter() {
        m *= &equation.n;
    }
    for i in 0..equations_vec.len() {
        m_i[i] = &m / &equations_vec[i].n;
    }

    let mut n_i = Vec::new();
    for i in 0..equations_vec.len() {
        n_i.push(
            inverse(
                &BigInt::from(m_i[i].clone()),
                &BigInt::from(equations_vec[i].n.clone()),
            )
                .unwrap(),
        )
    }

    let mut x = BigUint::zero();
    for i in 0..equations_vec.len() {
        x += (&equations_vec[i].a * &m_i[i] * &n_i[i]) % n;
    }

    return Ok(x % n);
}

pub fn tonelli_shanks(n: &BigUint, p: &BigUint) -> BigUint {
    let n = &(n % p);

    if gcd(n, p) != BigUint::one() {
        return BigUint::zero();
    } else if calc_jacobi(n, p) == -1 {
        return BigUint::zero();
    }

    let two = BigUint::from(2_u32);

    // p-1 = q*2^s
    let mut S: u128 = 0;
    let mut Q = p - &BigUint::one();
    while &Q % &two == BigUint::zero() && Q != BigUint::zero() {
        Q /= &two;
        S += 1;
    }

    // find non-residue
    let z = get_residue(p, false);
    let z = BigUint::from(3_u32);

    // init variables
    let mut M = BigUint::from(S);
    let mut c = z.modpow(&Q, p);
    let mut t = n.modpow(&Q, p);
    let mut R = n.modpow(&((&Q + BigUint::one()) >> 1), p);

    loop {
        if t == BigUint::zero() {
            return BigUint::zero();
        } else if t == BigUint::one() {
            return R;
        } else {
            let i = ord_2_exp(&t, p);

            let exp = BigUint::from(2_u32)
                .modpow(&(&M - i.clone() - BigUint::one()), &(p - BigUint::one()));
            let b = c.modpow(&exp, p);

            M = i;
            c = b.modpow(&BigUint::from(2_u32), p);
            t = (t * &c) % p;
            R = (R * b) % p;
        }
    }
}

fn ord_2_exp(t: &BigUint, p: &BigUint) -> BigUint {
    let mut i: u128 = 1;
    let mut exp = 1_u128 << i;
    loop {
        if t.modpow(&BigUint::from(exp), p) == BigUint::one() {
            return BigUint::from(i);
        }
        exp = exp << 1;
        i += 1;
    }
}

pub fn get_residue(n: &BigUint, is_quadratic_resiude: bool) -> BigUint {
    let mut condition = 0;
    match is_quadratic_resiude {
        true => {
            condition = 1;
        }
        false => {
            condition = -1;
        }
    }

    let mut gen = DefaultGenerator::new();
    loop {
        let t = gen.gen_num_less_than(n, n.bits() as usize);
        if calc_jacobi(&t, n) == condition {
            return t;
        }
    }
}

pub fn biguint_mod2(num: &BigUint) -> bool {
    let lsb = (num % BigUint::from(2_u32)).to_u8().unwrap();
    if lsb == 1 {
        return true;
    }
    false
}

/// Performs trial division algorithm
/// Number is tested for division on elements that < sqrt(n)
pub fn factor_trial_division(n: &BigUint) -> Option<&BigUint> {
    for divider in PRIME_NUMBERS.iter() {
        if n % divider == BigUint::zero() {
            return Some(divider);
        }
    }
    None
}

#[cfg(test)]
mod internals_test {
    use crate::{fast_square_root, is_prime, tonelli_shanks, tonelli_shanks_square_root};
    use num_bigint::BigUint;
    use num_traits::Num;
    use std::str::FromStr;

    #[test]
    fn solve_equations() {
        // let roots = solve_square_root(&BigUint::from_str_radix("", 16).unwrap(), &BigUint::from_str_radix("", 16).unwrap(), &BigUint::from_str_radix("", 16).unwrap(), &BigUint::from_str_radix("", 16).unwrap());
        // let roots = fast_square_root(
        //     &BigUint::from(7036_u32),
        //     &BigUint::from(383_u32),
        //     &BigUint::from(59_u32),
        //     &BigUint::from(22597_u32),
        // );
        // println!("{:?}", roots); //P1 = 10944, P2 = 10178, P3 = 12419, P4 = 11653
        let roots = tonelli_shanks_square_root(
            &BigUint::from(16_u32),
            &BigUint::from(3_u32),
            &BigUint::from(7_u32),
            &BigUint::from(21_u32),
        );
        println!("{:?}", roots); //P1 = 10944, P2 = 10178, P3 = 12419, P4 = 11653
    }

    #[test]
    fn is_prime_test() {
        assert_eq!(is_prime(&BigUint::from(8051_u128), &10).unwrap(), false);
        assert_eq!(
            is_prime(&BigUint::from(6118256737_u128), &10).unwrap(),
            true
        );
        assert_eq!(
            is_prime(&BigUint::from(9140096117_u128), &10).unwrap(),
            true
        );
        assert_eq!(
            is_prime(&BigUint::from(2951843171_u128), &10).unwrap(),
            true
        );
        assert_eq!(
            is_prime(&BigUint::from(2418116797_u128), &10).unwrap(),
            true
        );
        assert_eq!(
            is_prime(&BigUint::from(7382223733_u128), &10).unwrap(),
            true
        );
        assert_eq!(
            is_prime(
                &BigUint::from_str_radix("233812522175903330302841688516027262801", 10).unwrap(),
                &10,
            )
                .unwrap(),
            false
        );
        assert_eq!(
            is_prime(
                &BigUint::from_str_radix("260570066804195541625042755730066784431", 10).unwrap(),
                &10,
            )
                .unwrap(),
            false
        );
        assert_eq!(
            is_prime(
                &BigUint::from_str_radix("229406976110886911509376627892567244639", 10).unwrap(),
                &10,
            )
                .unwrap(),
            false
        );
    }
}
