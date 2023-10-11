// default PRNG is based on ChaCha algorithm
// default generator --- https://docs.rs/rand/0.8.5/rand/rngs/struct.ThreadRng.html
// it uses https://docs.rs/rand/0.8.5/rand/rngs/struct.StdRng.html
// and finally https://docs.rs/rand_chacha/latest/rand_chacha/ (open ChaCha**Rng)

use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

use crate::{DEFAULT_PRIMALITY_TEST_ITER, factor_trial_division, is_prime, PRNG};

pub struct DefaultGenerator {
    pub rng: ThreadRng,
}

impl DefaultGenerator {
    pub fn new() -> Self {
        DefaultGenerator { rng: thread_rng() }
    }

    pub fn gen_prime_num(&mut self, bit_len: usize) -> BigUint {
        let mut tmp = 0;
        loop {
            let gen_value = self.take(bit_len);
            if (gen_value != BigUint::one() || gen_value != BigUint::zero())
                && factor_trial_division(&gen_value).is_none()
                && is_prime(&gen_value, &DEFAULT_PRIMALITY_TEST_ITER).unwrap()
            {
                return gen_value;
            }
            tmp += 1;
            continue;
        }
    }

    // Generates blum primes (p = 4k + 3)
    pub fn gen_blum_prime(&mut self, bit_len: usize) -> BigUint {
        let mut tmp = 0;
        let bit_len = bit_len - 2;
        loop {
            let gen_value = (self.take(bit_len) << 2) | BigUint::from(3_u8);
            if (gen_value != BigUint::one() || gen_value != BigUint::zero())
                && factor_trial_division(&gen_value).is_none()
                && is_prime(&gen_value, &DEFAULT_PRIMALITY_TEST_ITER).unwrap()
            {
                return gen_value;
            }
            tmp += 1;
            continue;
        }
    }

    pub fn gen_num_less_than(&mut self, n: &BigUint, bit_len: usize) -> BigUint {
        let mut gen_value = self.take(bit_len);
        loop {
            if gen_value < *n {
                return gen_value;
            }
            gen_value = self.take(bit_len);
        }
    }
}

impl PRNG for DefaultGenerator {
    fn next(&mut self) -> u8 {
        if self.rng.gen_bool(0.5) {
            return 1_u8;
        }
        return 0_u8;
    }

    fn next_byte(&mut self) -> u8 {
        self.rng.gen()
    }

    fn take(&mut self, n: usize) -> BigUint {
        let mut result = BigUint::from(self.next());
        for _ in 1..n {
            result = (result << 1) | BigUint::from(self.next());
        }
        result
    }

    fn take_byte_vectorized(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| -> u8 { self.rng.gen() }).collect()
    }
}

#[cfg(test)]
mod default_gen_tests {
    use num_bigint::BigUint;
    use num_traits::{Num, One};

    use crate::{calc_jacobi, DEFAULT_PRIMALITY_TEST_ITER, fast_square_root, is_prime, PRNG, tonelli_shanks, tonelli_shanks_square_root};
    use crate::default_gen::DefaultGenerator;

    #[test]
    fn making_gen() {
        let mut gen = DefaultGenerator::new();
        // println!("{:?}", gen.take_byte_vectorized(100))
        // println!("128");
        // gen.gen_prime_num(128);

        // println!("256");
        // gen.gen_prime_num(256);
        // println!("512");
        // gen.gen_prime_num(512);
        // println!("1024");
        // gen.gen_prime_num(1024);
        // println!("2048");
        // gen.gen_prime_num(2048);
        let tmp = gen.gen_blum_prime(256);
        let tmp2 = gen.gen_blum_prime(128);
        // let n = &tmp * &tmp2;
        // println!("{}, {}, {}", tmp.bits(), tmp2.bits(), n.bits());

        // println!("blum prime:{:#}, {}", &tmp, (&tmp - BigUint::from(3_u32)) / BigUint::from(4_u32));
        // let tmp = gen.gen_blum_prime(512);
        // println!("blum prime:{:#}, {}", &tmp, (&tmp - BigUint::from(3_u32)) / BigUint::from(4_u32));
        // let tmp = gen.gen_blum_prime(1024);
        // println!("blum prime:{:#X}, {}", &tmp, (&tmp - BigUint::from(3_u32)) / BigUint::from(4_u32));
        // let tmp = gen.gen_blum_prime(2048);
        // println!("blum prime:{:#X}, {}", &tmp, (&tmp - BigUint::from(3_u32)) / BigUint::from(4_u32))
    }

    #[test]
    fn get_random_number() {
        let mut gen = DefaultGenerator::new();
        // println!("{:#X}", gen.gen_prime_num(10_usize));
        // println!("{:#X}", gen.gen_prime_num(256_usize));
        // println!("{:#X}", gen.gen_prime_num(2048_usize));
        // let tmp = gen.gen_rsa_prime(2048_usize);
        let tmp = gen.gen_blum_prime(1024_usize);

        println!(
            "{:#X}, is_prime:{:?}",
            tmp,
            is_prime(&tmp, &DEFAULT_PRIMALITY_TEST_ITER)
        );
        // println!(
        //     "{:#X}, is_prime:{:?}",
        //     &tmp >> 1,
        //     is_prime(&(&tmp >> 1), &DEFAULT_PRIMALITY_TEST_ITER)
        // );
        // println!("{:#X}", gen.gen_rsa_prime(2048_usize));
    }

    #[test]
    fn tonelli_shanks_test() {
        let mut gen = DefaultGenerator::new();
        let n1 = gen.gen_blum_prime(25_usize);
        println!("n: {}, {:#b}, {:#b}, {:#b}", &n1, &n1 , &n1 - BigUint::from(3_u32), (&n1 - BigUint::from(3_u32))/BigUint::from(4_u32) );
        let n1 = BigUint::from(19143851_u32);

        let x = gen.gen_num_less_than(&n1, n1.bits() as usize);
        println!("n1:{} x:{}, x^2 mod n:{}", &n1, &x, &x.modpow(&BigUint::from(2_u32), &n1));
        let y = &x.modpow(&BigUint::from(2_u32), &n1);
        let y = &BigUint::from(8_u32);
        println!("n1:{}, y mod n:{}", &n1, &y);

        // let roots = solve_square_root(&BigUint::from_str_radix("", 16).unwrap(), &BigUint::from_str_radix("", 16).unwrap(), &BigUint::from_str_radix("", 16).unwrap(), &BigUint::from_str_radix("", 16).unwrap());
        let roots = tonelli_shanks(y, &n1);
        let roots: Vec<BigUint> = vec![roots.clone(), &n1 - roots];
        println!("{:?}", roots);
        println!("m_1( mod2:{}, (x/n):{} )\nm_2( mod2:{}, , (x/n):{} )\n",
                 &roots[0] % BigUint::from(2_u32), calc_jacobi(&roots[0], &n1),
                 &roots[1] % BigUint::from(2_u32), calc_jacobi(&roots[1], &n1), );
    }

    #[test]
    fn solve_equation() {
        let mut gen = DefaultGenerator::new();
        let n1 = gen.gen_blum_prime(25_usize);
        let n1 = BigUint::from(2143459_u32);

        let n2 = gen.gen_blum_prime(25_usize);
        let n2 = BigUint::from(15674279_u32);
        let n = (&n1 * &n2);

        let x = gen.gen_num_less_than(&n, n.bits() as usize);

        // println!("n1:{:#X}, n2:{:#X}, x:{:#X}, n:{:#X}, x^2 mod n:{:#X}", &n1, &n2, &x, &n, &x.modpow(&BigUint::from(2_u32), &n));
        println!("n1:{}, n2:{}, x:{}, n:{}, x^2 mod n:{}", &n1, &n2, &x, &n, &x.modpow(&BigUint::from(2_u32), &n));

        let y = &x.modpow(&BigUint::from(2_u32), &n);
        let y = BigUint::from(12399099110404_u128);

        let roots = fast_square_root(&y, &n1, &n2, &n);
        println!("roots: {:?}\n m_1( mod2:{}, (x/n):{} )\nm_2( mod2:{}, (x/n):{} )\nm_3( mod2:{}, (x/n):{} )\nm_4( mod2:{}, (x/n):{} )\n",
                 roots,
                 &roots[0] % BigUint::from(2_u32), calc_jacobi(&roots[0], &n),
                 &roots[1] % BigUint::from(2_u32), calc_jacobi(&roots[1], &n),
                 &roots[2] % BigUint::from(2_u32), calc_jacobi(&roots[2], &n),
                 &roots[3] % BigUint::from(2_u32), calc_jacobi(&roots[3], &n));

        let roots = tonelli_shanks_square_root(&y, &n1, &n2, &n);
        println!("roots: {:?}\n m_1( mod2:{}, (x/n):{} )\nm_2( mod2:{}, (x/n):{} )\nm_3( mod2:{}, (x/n):{} )\nm_4( mod2:{}, (x/n):{} )\n",
                 roots,
                 &roots[0] % BigUint::from(2_u32), calc_jacobi(&roots[0], &n),
                 &roots[1] % BigUint::from(2_u32), calc_jacobi(&roots[1], &n),
                 &roots[2] % BigUint::from(2_u32), calc_jacobi(&roots[2], &n),
                 &roots[3] % BigUint::from(2_u32), calc_jacobi(&roots[3], &n));

        // println!("{}", (&roots[0] * &roots[0]) % &n);
        // let x = BigUint::from_str_radix("", 16).unwrap();
        // let n1 = BigUint::from_str_radix("", 16).unwrap();
        // let n2 = BigUint::from_str_radix("", 16).unwrap();
        // let n = BigUint::from_str_radix("", 16).unwrap();
    }
}
