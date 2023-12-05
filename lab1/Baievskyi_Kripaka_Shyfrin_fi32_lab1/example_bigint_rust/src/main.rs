use num_bigint::BigUint;
use num_traits::Num;
use lab3::default_gen::DefaultGenerator;
use lab3::FORMAT_KEYS;
use lab3::rabin_wrap::{PubDataEncrypt, PubDataSign, RabinPrivateKey, RabinPubKey, User};

fn main() {
    let prime_byte_len = 1024;
    println!("{}", generate_private_keys(prime_byte_len));

    // Encrypt message with user public key
    // let A = User::new(prime_byte_len);
    // let pub_key = RabinPubKey::from_numbers("DCBA2907FBFE5461A771084E226ABF6D", "944FE429E2DDC8E701F8323AB2A63C3B");
    // encrypt_with_pub_key(A, pub_key, "A");
    // -----end

    // Decrypt ciphertext with generated user private key
    // let A = User::from_private_key(  RabinPrivateKey::from_numbers(
    //     "526f915a37c990e0f55d38faa3974915",
    //     "24ced1fa4f76345efa5cfeb3e58eed82",
    //     ["ab28dfd8935e2e63",
    //         "7b4c2c5f3ee26827"]
    //         .to_vec())
    // );
    // let message = PubDataEncrypt {
    //     c: BigUint::from_str_radix("410C619D2C8DB7C961DA366C92DD0BFE", 16).unwrap(),
    //     b_1: false,
    //     b_2: false,
    // };
    // decrypt_message(A, message);
    // -----end


    // Sign text with generated user private key
    let A = User::from_private_key( RabinPrivateKey::from_numbers(
        "68e756c651724cc242bbee7c5cb7a48f1ad6dd97009eac17f173acc22323a7871fc2558db2c099f1df37f7514138245b5cfc8cde2cb14d1b0520c56df53d8a56a976be469e048de67d57d0c60d9e023181359fb9bbbadc1d9d2f2cdacd0ae275fadcabf78240be56bb8f8b4181e527eec550d462d916e7ffe5ab04a4c7ecaca5",
        "c2606c82e863b0282ba9de88e21cf44fcc67d17a4b79c4feb4d2fab78ae959d1067693a2c9c81bb244adae83845db8f0708de73f7c5aeee24020cce9a7fe659699335a14c2f4c9a3639032a3c6b2de6d685139c03179bd915e987cb126f231a0a9cb6ac718558fc33e779ccf205e99881c845eff7047be0568ac7d8070e8b50",
        ["7f9eb1c75ef3a5e76753885611007d1c16594f05eee26e8c64828b0a7b44e95f2d1850fe935599920ce5b302bbfc80293c460c7ef37f3fc197a9c5fa2642846b",
            "d26ea5fef46d4c1e3a6189e014658730152b10ecc7531d65f71255b780622e86c3971423d9b84a2437ed3412057145503bf53a2bfeb352d3696aae8cd1a2572f"]
            .to_vec())
    );
    sign_message(A, "123456");
    // -----end

    // Verify sign with users public key
    // let A = User::new(prime_byte_len);
    // let pub_key = RabinPubKey::from_numbers("BCBC9C3B2933667805BE04823614309887A23E0CD9E427F5F97E9903AC0626ACD44CC75A53828A8E429041D71AA873BEC91E43F1463BE5A185B5DBB0026BEAD0209442281DAC403BED54D78CF26AAC5C7013AA6441D406214330699B7C1F01AAC6BCE8402CF30055EC4E3DB4AB7055778C84DB0DBECA4FAE372B3A827DE5C7A9", "2D08997394A36FE07936B9802D23AB1F3BA3791EA620E27519E10EE559032575B0AFE01FB0E77AE01F9372A5FBCC3969794450600A91558F9B3C9CC009289A4C0FEA36D2F4D0A4A558DFE342F4DF65488F816C2CFDF8A19CD2B2EA6774B3BCA5BB331653B0F415776E0E3FE4F9C6FE59040344916126EEB6BC8B261C710371A6");
    // let signed_message = PubDataSign::from_numbers("ABCDEF123456789", "3ECBB5489EACE2EAE6A0C3D20748C6B867D067F3D88D10D4F31BDC02124C32977BF3A1612491DDB64DD57CA8B617188AF95EF2EC9E0D0674ED70BE3E405E113A193B1386E65C5DCF0B14D281DB6CA3D3682A4950C37B8AA76DE7D1938186712BFA9A4AA6F69E3AFE3572CF71F840F32A7B65A9A7050F2D805686A14383D6CCC0");
    // check_sign(A, pub_key, signed_message);
    // -----end

    // Generate quadratic residue
    let A = User::new(prime_byte_len);
    let n = BigUint::from_str_radix("816AA001205FD796E89FBCA07837E99D0216E646C17F5548E1B5A13EFC838BC30BDBF6CBE17D2E52CBD716AD075331B6B262B073CB2E235CA731B5212FBB2E009C4F1E167D22A81A11AD2C4E8EE82592452DBEED256AB5285DFA08789448F1553931680E31FF799F86A8E2DB2E7744211A7CB69975065BCBA7EC22E1A062A1FC960ED9DAC48A00562583C4489BEE4CECA5269CD29CC5ECB4680D9C67D14B8AAEF11A7450802FC34CA8F7044AA5D5926A3625B31517EF60EBEAF374307DDBE80AFA37FD1BC04D4A5F04A77A1553053F1FFE000CC65AD2E528E1792998B6D56220F782EAD60BB6F82663C65BC6E82375447DC627877834040A89E04657C74F69FD", 16).unwrap();
    let (t, y) = generate_quadratic_residue(&n);
    let result = factor_n("466CBB253F6B9329D4A0863B19C7B31919589E88BCE410A7B373033EE8139640B733778468B1E814649F40852DF501B5A9A3C371919BF1A9816B1674CD449A4348826C2777A083A55E5A76407013E96CAE640AE96C24E77D8BF62EDEE7DBD682C3E9F794E6414424215017C8FFD1298C965C6C46AD7FA2CCE9DA6BBD0622C41991D0A2B25B9B0C129CED1DC3F868EDF6B925C97BECDEAF8924FA845C878545E037102C9D1FFD5A1B15C541F06EFFB29288A10A74C388443595BDAAADD3BFDF771D9D562542EAF1E7D816DAD05515925F40D9ABF90F4B2E40588F3A066DDD6B398F72093936A190CE839B776EAFD644662982AD133E8047D5D374F99E1F3915BD",
                          "33BD08A3504CC1A9F164E2AD535C4DC85DC6F21D423D7C454072FC7067BB4457E88C7C117F53B076135854A1F1397841E578C5CC886080EF45A185CD80142F56B9679BA85EF0BBDF411744935E37ECB3E23578A58F6C8B075637BE6D75B8071E060C83AE9B54FAEFE4E1E37E91A0ACD89E0896BED65A2FCFD6AA5B1C254B2AED9B674B5AB94A96A1E496EA75F7DF88E86FCFF627482D2D285836A0EE8D0A1F166F6DC9ECC2E5BD64F145A8A239B8B315CDEB7E4FED83224641432C5F8B3C85D63D73D16591550AD49701347F87C2BE6903511C09250B4722A29ABA93F7006103479A91782ED56FE6676F418D28E7CE2FD8509225D0A5303DA597408D4928F926",
                          &A, &n);
    let n_check = BigUint::from_str_radix("BA1ACAD6EB9DD3EFCC7D81B5E46A7261B93797E2E8CE79451C47067420933EFCF613BEF671AFB9D148210479CA891B8AA01602028E2943CF27CA0872FFFE7F432F29523FA398C3E03605D9F5676C829B533EE20F92438DFDFE80ED02C8200820AA959EB92995820AE61928F97304D79BFDB54A5529B5F83C7FB50BA45BA7AADF", 16).unwrap() *  BigUint::from_str_radix("B2057F29DE0F59EA6E9D2C37B36038703A41A0C4318AB44B3D2E06D210F1CB26FBB3077CC3D3EEB4C19973C5625A533BAD963911D2B2081F209663E45AF73D893B4ABD3B09D6F7C7C9C9E13707BE672A4BDBE340DB3642459834123FC11083341C2E42CF9D09C0B5AB3EBA513E63FDD7B8A848E2CBA9ECC72A64A10A50A422A3", 16).unwrap();
    println!("is ok:{}",n == n_check)


    // 128
    // RabinPrivateKey::from_numbers(
    //     "48bf70604a35fce6ed64e11d3ccef615",
    //     "3fb784fbe514a36d1887ca9f0df37897",
    //     ["5e3ee9fe6905b93f",
    //         "c59ae56defb487ab"]
    //         .to_vec())

    // 256
    // RabinPrivateKey::from_numbers(
    //     "f78d9def02eee977a73323a5d31b2a235630e8590941d1e99141fcb0db17821",
    //     "7f77f4e70f4bec24371a850cfeac947764550ba73012c6ee820997408610acf",
    //     ["174d93765b4a4d6c77d8b475e36b85cb",
    //         "a9f8b3d8807e14c683e45b981a59dc43"]
    //         .to_vec())

    // -----end
}


fn encrypt_with_pub_key(A: User, pub_key: RabinPubKey, message: &str) {
    // let x: String = "123456789".to_string().chars().rev().collect::<String>().as_str();
    let message = BigUint::from_str_radix(message, 16).unwrap();
    let encrypted_message = A.encrypt(&message, &pub_key);
    println!("encrypted message: {}", encrypted_message)
}

fn decrypt_message(A: User, ciphertext: PubDataEncrypt) {
    let decrypted_message = A.decrypt(&ciphertext);
    println!("decrypted message: {:#X}", decrypted_message)
}

fn sign_message(A: User, message: &str) {
    let message = BigUint::from_str_radix(message, 16).unwrap();
    let signed_message = A.sign(&message);
    let is_ok = A.verify(&signed_message, A.get_pub_key());
    println!("signed message: {}, is ok: {}", signed_message, is_ok)
}

fn check_sign(A: User, pub_key: RabinPubKey, signed_message: PubDataSign) {
    let is_ok = A.verify(&signed_message, &pub_key);
    println!("signed message: {}, is ok: {}", signed_message, is_ok)
}

fn generate_private_keys(prime_byte_len: usize) -> String {
    let (private_key, public_key) = RabinPrivateKey::new(prime_byte_len);
    FORMAT_KEYS(&private_key, &public_key).1
}

fn generate_quadratic_residue(n: &BigUint) -> (BigUint, BigUint) {
    let mut gen = DefaultGenerator::new();
    let t = gen.gen_num_less_than(&n, n.bits() as usize);
    println!("y = t^2:{:#X}, \nt:{:#X}", t.modpow(&BigUint::from(2_u32), &n), &t);
    (t.clone(), t.modpow(&BigUint::from(2_u32), &n))
}

fn factor_n(t: &str, z: &str, A: &User, n: &BigUint) -> Vec<BigUint> {
    let t = BigUint::from_str_radix(t, 16).unwrap();
    let z = BigUint::from_str_radix(z, 16).unwrap();

    let result: Vec<BigUint> = match A.factor_n(&z, &t, &n) {
        None => {
            println!("generate new one t");
            vec![]
        }
        Some(vec) => {
            println!("p:{:#X}, q:{:#X}", vec.get(0).unwrap(), vec.get(1).unwrap());
            vec
        }
    };
    println!("CHECK n % result[0]:{:#X}, n % result[1]:{:#X}", n % &result[0], n % &result[1]);
    result
}