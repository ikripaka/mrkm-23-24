use std::fmt::{Display, Formatter};
use std::process;

use num_bigint::{BigInt, BigUint, ToBigInt, ToBigUint};
use num_traits::{Num, One, ToPrimitive, Zero};
use rand::{thread_rng, RngCore};

use crate::{biguint_mod2, calc_jacobi, fast_square_root, gcd, inverse, DefaultGenerator, PRNG, tonelli_shanks_square_root};

#[derive(Default, Debug, Clone)]
pub struct PubDataEncrypt {
    pub c: BigUint,
    pub b_1: bool,
    pub b_2: bool,
}

#[derive(Default, Debug, Clone)]
pub struct PubDataSign {
    pub m: BigUint,
    pub s: BigUint,
}

#[derive(Debug, Clone)]
pub struct RabinPubKey {
    pub n: BigUint,
    pub b: BigUint,
}

// Structure to handle creation of private keys for RSA
#[derive(Debug, Clone)]
pub struct RabinPrivateKey {
    pub pubkey_components: RabinPubKey,
    pub primes: Vec<BigUint>,
}

#[derive(Debug)]
pub struct RabinEncryptor {
    private_key: RabinPrivateKey,
}

#[derive(Debug)]
pub struct RabinSigner {
    private_key: RabinPrivateKey,
}

pub struct User {
    signer: RabinSigner,
    encryptor: RabinEncryptor,
}

const MESSAGE_OFFSET: u64 = 80; //10 bytes

impl RabinPubKey {
    pub fn from_numbers(n: &str, b: &str) -> Self {
        RabinPubKey {
            n: BigUint::from_str_radix(n, 16).unwrap(),
            b: BigUint::from_str_radix(b, 16).unwrap(),
        }
    }

    // formatting message in format: "00 || FF || m || r"
    fn format_message(&self, m: &BigUint) -> Result<BigUint, String> {
        let g = self.n.bits();
        let l = (self.n.bits() as f64 / 8.).ceil() as u64 * 8;
        let n_bit = self.n.bits();
        if m.bits() < l - MESSAGE_OFFSET {
            let r: u64 = thread_rng().next_u64();

            let mut x = BigUint::from_str_radix("00FF", 16).unwrap();
            x = (((&x << (l - MESSAGE_OFFSET)) | m) << (8 * 8)) | BigUint::from(r); // BigUint::from_str_radix("f461cbd6e1a57036", 16).unwrap();
            Ok(x)
        } else {
            Err(format!(
                "incorrect message m_len:{} bit, l-8*10:{}",
                m.bits(),
                l - MESSAGE_OFFSET
            ))
        }
    }

    fn unformat_message(&self, m: &BigUint) -> BigUint {
        let l = (self.n.bits() as f64 / 8.).ceil() as u64 * 8;
        let mut m = m >> (8 * 8);
        m = &m % (BigUint::one() << (l - MESSAGE_OFFSET));
        m
    }

    fn is_message_ok(&self, m: &BigUint) -> bool {
        let eight_bits = BigUint::from(u8::MAX);
        let l = (self.n.bits() as f64 / 8.).ceil() as u64 * 8;

        if (m & (&eight_bits << l - 3 * 8)) >> (l - 1 * 8) == BigUint::zero()
            && (m & (&eight_bits << (l - 2 * 8))) >> (l - 2 * 8) == BigUint::from(u8::MAX)
            && m.bits() == (l - 8)
        {
            return true;
        }
        false
    }
}

impl PubDataSign {
    pub fn from_numbers(m: &str, s: &str) -> Self {
        PubDataSign {
            m: BigUint::from_str_radix(m, 16).unwrap(),
            s: BigUint::from_str_radix(s, 16).unwrap(),
        }
    }
}

impl PubDataEncrypt {
    pub fn from_numbers(c: &str, b_1: bool, b_2: bool) -> Self {
        PubDataEncrypt {
            c: BigUint::from_str_radix(c, 16).unwrap(),
            b_1,
            b_2,
        }
    }
}

impl RabinPrivateKey {
    pub fn from_numbers(n: &str, b: &str, primes: Vec<&str>) -> Self {
        RabinPrivateKey {
            pubkey_components: RabinPubKey {
                n: BigUint::from_str_radix(n, 16).unwrap(),
                b: BigUint::from_str_radix(b, 16).unwrap(),
            },
            primes: primes
                .into_iter()
                .map(|x| -> BigUint { BigUint::from_str_radix(x, 16).unwrap() })
                .collect(),
        }
    }

    // Creating new private key for Rabin
    pub fn new(prime_byte_len: usize) -> (Self, RabinPubKey) {
        let mut gen = DefaultGenerator::new();
        let mut primes = Vec::new();
        let prime_byte_len = prime_byte_len / 2;
        primes.push(gen.gen_blum_prime(prime_byte_len));
        primes.push(gen.gen_blum_prime(prime_byte_len));

        let n = &*primes.get(0).unwrap() * &*primes.get(1).unwrap();
        let pubkey_components = RabinPubKey {
            n: n.clone(),
            b: DefaultGenerator::new().gen_num_less_than(&n, n.bits() as usize),
        };
        (
            RabinPrivateKey {
                pubkey_components: pubkey_components.clone(),
                primes,
            },
            pubkey_components,
        )
    }
}

impl User {
    pub fn from_private_key(pk: RabinPrivateKey) -> Self {
        User {
            signer: RabinSigner {
                private_key: pk.clone(),
            },
            encryptor: RabinEncryptor { private_key: pk },
        }
    }

    pub fn new(prime_byte_len: usize) -> Self {
        let (private_key, public_key) = RabinPrivateKey::new(prime_byte_len);

        User {
            encryptor: RabinEncryptor {
                private_key: private_key.clone(),
            },
            signer: RabinSigner { private_key },
        }
    }

    // Generates key in range 0 < k < n
    pub fn gen_key(&self) -> BigUint {
        let mut gen = DefaultGenerator::new();
        loop {
            let k = gen.gen_prime_num(self.signer.private_key.pubkey_components.n.bits() as usize);
            if k != BigUint::zero() && k != self.signer.private_key.pubkey_components.n {
                return k;
            }
        }
    }

    pub fn encrypt(&self, m: &BigUint, pub_key: &RabinPubKey) -> PubDataEncrypt {
        self.encryptor.encrypt(m, pub_key)
    }

    pub fn decrypt(&self, c: &PubDataEncrypt) -> BigUint {
        self.encryptor.decrypt(c).unwrap_or_else(
            (|err| {
                eprintln!("problem with extracting message: {}", err);
                process::exit(0);
            }),
        )
    }

    pub fn sign(&self, m: &BigUint) -> PubDataSign {
        self.signer.sign(m)
    }

    pub fn verify(&self, pub_data: &PubDataSign, pub_key: &RabinPubKey) -> bool {
        self.signer.check_sign(pub_data, pub_key)
    }

    pub fn get_pub_key(&self) -> &RabinPubKey {
        &self.encryptor.private_key.pubkey_components
    }

    pub fn factor_n(&self, z: &BigUint, t: &BigUint, n: &BigUint) -> Option<Vec<BigUint>> {
        let result_1 = gcd(&(t + z), n);
        let result_2 = gcd(&(t + (n - z)), n);
        println!("gcd(t + z, n):{:#X}", result_1);
        println!("gcd(t - z, n):{:#X}", result_2);
        if result_1 != BigUint::one() {
            return Some(vec![n / &result_1, result_1]);
        } else if result_2 != BigUint::one() {
            return Some(vec![n / &result_2, result_2]);
        }
        None
    }
}

impl RabinEncryptor {
    fn from_private_key(private_key: RabinPrivateKey) -> Self {
        RabinEncryptor { private_key }
    }

    fn new(prime_byte_len: usize) -> (Self, RabinPubKey) {
        let (private_key, public_key) = RabinPrivateKey::new(prime_byte_len);

        (
            RabinEncryptor {
                private_key: private_key,
            },
            public_key,
        )
    }

    fn decrypt(&self, ciphertext: &PubDataEncrypt) -> Result<BigUint, String> {
        let n = &self.private_key.pubkey_components.n;
        let two_inverse = inverse(&BigInt::from(2_u32), &BigInt::from(n.clone())).unwrap();
        let four_inverse = inverse(&BigInt::from(4_u32), &BigInt::from(n.clone())).unwrap();
        println!("n: {}, p: {}, q: {}", n, &self.private_key.primes[0], &self.private_key.primes[1] );
        println!(
            "c:{:#X}, b_1:{}, b_2:{}, n:{:#X}",
            ciphertext.c, &ciphertext.b_1, &ciphertext.b_2, n
        );

        let roots = tonelli_shanks_square_root(
            &(&ciphertext.c
                + &self
                    .private_key
                    .pubkey_components
                    .b
                    .modpow(&BigUint::from(2_u32), n)
                    * &four_inverse),
            &self.private_key.primes[0],
            &self.private_key.primes[1],
            n,
        );

        println!("m_1( mod2:{}, (x/n):{} )\nm_2( mod2:{}, (x/n):{} )\nm_3( mod2:{}, (x/n):{} )\nm_4( mod2:{}, (x/n):{} )\n",
                 &(&roots[0] % n) % BigUint::from(2_u32), calc_jacobi(&roots[0], n),
                 &(&roots[1] % n) % BigUint::from(2_u32), calc_jacobi(&roots[1], n),
                 &(&roots[2] % n) % BigUint::from(2_u32), calc_jacobi(&roots[2], n),
                 &(&roots[3] % n) % BigUint::from(2_u32), calc_jacobi(&roots[3], n));

        let root_1_result = (biguint_mod2(&roots[0]), calc_jacobi(&roots[0], n));
        let root_2_result = (biguint_mod2(&roots[1]), calc_jacobi(&roots[1], n));
        let root_3_result = (biguint_mod2(&roots[2]), calc_jacobi(&roots[2], n));
        let root_4_result = (biguint_mod2(&roots[3]), calc_jacobi(&roots[3], n));

        let roots: Vec<BigUint> = roots
            .into_iter()
            .map(|x| -> BigUint {
                let subtractor =
                    (n - ((&self.private_key.pubkey_components.b * &two_inverse) % n)) % n;
                (x + subtractor) % n
            })
            .collect();

        println!("{:X}, {:X}, {:X}, {:X}", &roots[0], &roots[1], &roots[2], &roots[3]);

        //choose m1, m4
        if ciphertext.b_2 as i8 == root_1_result.1 {
            if ciphertext.b_1 == root_1_result.0 {
                let m: &BigUint = &roots[0];
                if self.get_pub_key().is_message_ok(m) {
                    return Ok(self.get_pub_key().unformat_message(m));
                }
                return Err(format!("incorrect message formatting, message: {:#X}", m));
            } else if ciphertext.b_1 == root_4_result.0 {
                let m: &BigUint = &roots[3];
                if self.get_pub_key().is_message_ok(m) {
                    return Ok(self.get_pub_key().unformat_message(m));
                }
                return Err(format!("incorrect message formatting, message: {:#X}", m));
            }

        } else {
            //choose m2, m3
            if ciphertext.b_1 == root_2_result.0 {
                let m: &BigUint = &roots[1];
                if self.get_pub_key().is_message_ok(m) {
                    return Ok(self.get_pub_key().unformat_message(m));
                }
                return Err(format!("incorrect message formatting, message: {:#X}", m));
            } else if ciphertext.b_1 == root_3_result.0{
                let m: &BigUint = &roots[2];
                if self.get_pub_key().is_message_ok(m) {
                    return Ok(self.get_pub_key().unformat_message(m));
                }
                return Err(format!("incorrect message formatting, message: {:#X}", m));
            }
        }
        Err(format!("can't find correct answer"))
    }

    fn encrypt(&self, m: &BigUint, pub_key: &RabinPubKey) -> PubDataEncrypt {
        let n = &pub_key.n;
        let two_inverse = inverse(&BigInt::from(2_u32), &BigInt::from(n.clone())).unwrap();

        let x = pub_key.format_message(m).unwrap();
        println!("formatted message: {:#X}", x);
        let y = (&x * (&x + &pub_key.b)) % n;
        println!("y: {:#X}", y);
        println!("b: {:#X}", &pub_key.b);


        let x = &((&x + &pub_key.b * &two_inverse) % n);
        let mod2 = biguint_mod2(x);
        let jacobi = match calc_jacobi(x, n) {
            1 => true,
            -1 => false,
            _ => false,
        };
        // println!(
        //     "m:{:#X}, b:{:#X}, n:{:#X}, parity:{}, jacobi:{}",
        //     m, &pub_key.b, &pub_key.n, mod2, jacobi
        // );
        PubDataEncrypt {
            c: y,
            b_1: mod2,
            b_2: jacobi,
        }
    }

    fn get_pub_key(&self) -> RabinPubKey {
        self.private_key.pubkey_components.clone()
    }

    fn get_priv_key(&self) -> &RabinPrivateKey {
        &self.private_key
    }
}

impl RabinSigner {
    fn from_private_key(private_key: RabinPrivateKey) -> Self {
        RabinSigner { private_key }
    }

    fn new(prime_byte_len: usize) -> (Self, RabinPubKey) {
        let (private_key, public_key) = RabinPrivateKey::new(prime_byte_len);

        (RabinSigner { private_key }, public_key)
    }

    fn sign(&self, m: &BigUint) -> PubDataSign {
        let p: &BigUint = self.private_key.primes.get(0).unwrap();
        let q: &BigUint = self.private_key.primes.get(1).unwrap();
        let mut gen = DefaultGenerator::new();

        loop {
            let m_formatted = self
                .private_key
                .pubkey_components
                .format_message(m)
                .unwrap();
            if calc_jacobi(&m_formatted, p) == 1 && calc_jacobi(&m_formatted, q) == 1 {
                let roots =
                    fast_square_root(&m_formatted, p, q, &self.private_key.pubkey_components.n);
                let random_index = gen.take(2).to_usize().unwrap();
                return PubDataSign {
                    m: m.clone(),
                    s: (*roots.get(random_index % 4).unwrap()).clone(),
                };
            }
        }
    }

    fn check_sign(&self, pub_data: &PubDataSign, pub_key: &RabinPubKey) -> bool {
        // println!(
        //     "s:{:#X}, b:{:#X}, m:{:#X}, n:{:#X}",
        //     &pub_data.s, &pub_key.b, &pub_data.m, &pub_key.n
        // );
        let formatted_message = &pub_data.s.modpow(&BigUint::from(2_u32), &pub_key.n);
        // println!("formatted message: {:#X}", formatted_message);
        if pub_key.unformat_message(formatted_message) == pub_data.m
            && pub_key.is_message_ok(formatted_message)
        {
            return true;
        }
        false
    }

    fn get_pub_key(&self) -> RabinPubKey {
        self.private_key.pubkey_components.clone()
    }

    fn get_priv_key(&self) -> &RabinPrivateKey {
        &self.private_key
    }
}

impl Display for RabinPrivateKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "private key: \n   b:{:#X},\n   n:{:#X},\n   n_factor:[\n    {:#X},\n    {:#X}\n     ]\n", self.pubkey_components.b, self.pubkey_components.n, self.primes[0], self.primes[1])
    }
}

impl Display for RabinPubKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "public key: \n   n:{:#X},\n   b:{:#X},\n",
            self.n, self.b
        )
    }
}

impl Display for PubDataSign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "transmitted: \n   s:{:#X},\n   m:{:#X},\n",
            self.s, self.m
        )
    }
}

impl Display for PubDataEncrypt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "transmitted: \n   ciphertext:{:#X}\n, parity:{}, jacobi:{}",
            self.c, self.b_1, self.b_2
        )
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User has there private key:\n   pk:{}\n",
            self.encryptor.private_key
        )
    }
}

#[cfg(test)]
mod default_gen_tests {
    use num_bigint::BigUint;
    use num_traits::{Num, One};

    use crate::default_gen::DefaultGenerator;
    use crate::rabin_wrap::RabinEncryptor;
    use crate::{is_prime, RabinPrivateKey, DEFAULT_PRIMALITY_TEST_ITER, PRNG};

    #[test]
    fn making_encryptor() {
        let (encryptor, pub_key) = RabinEncryptor::new(256_usize);
        println!("{}, {}", encryptor.private_key, pub_key);
        let ciphertext = encryptor.encrypt(&BigUint::from(0x2_u32), &pub_key);
        println!("{}", ciphertext);
        println!("decrypted text{:?}", encryptor.decrypt(&ciphertext))
    }

    #[test]
    fn testing_formatting() {
        let private_key = RabinPrivateKey::new(256);
        let private_key = RabinPrivateKey::from_numbers(
            "83AF54972F20E3002DB6E4E71DF18BC6",
            "7D1C17CBA5897E11609EDAA71158DCB94B578659CF882B02D268A40859E24557",
            [
                "861c1daff983487b757f2b981ad15e77",
                "39f5bcf976338203d5faceb5971dc91b",
            ]
            .to_vec(),
        );
        println!("{}", private_key.pubkey_components.n.bits());
        let formatted_message = private_key
            .pubkey_components
            .format_message(&BigUint::from_str_radix("8A26", 16).unwrap())
            .unwrap();

        println!(
            "{:#X}\n{:#X}, is_ok:{}",
            &formatted_message,
            private_key
                .pubkey_components
                .unformat_message(&formatted_message),
            private_key
                .pubkey_components
                .is_message_ok(&formatted_message)
        );
    }

    #[test]
    fn testing_formatting_2() {
        let private_key = RabinPrivateKey::from_numbers(
            "DCBA2907FBFE5461A771084E226ABF6D",
            "878F0F72BA2CBC534",
            [
                "861c1daff983487b",
                "39f5bcf976338203",
            ]
                .to_vec(),
        );
        // let (private_key, pub_key) = RabinPrivateKey::new(90);
        println!("{}", private_key.pubkey_components.n.bits());
        let formatted_message = private_key
            .pubkey_components
            .format_message(&BigUint::from_str_radix("ABC", 16).unwrap())
            .unwrap();

        println!(
            "{:#X}\n{:#X}, is_ok:{}",
            &formatted_message,
            private_key
                .pubkey_components
                .unformat_message(&formatted_message),
            private_key
                .pubkey_components
                .is_message_ok(&formatted_message)
        );
    }
}
