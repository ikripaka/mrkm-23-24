#include <iostream>
#include <fstream>
#include <cryptopp/aes.h>
#include <cryptopp/rsa.h>
#include <cryptopp/osrng.h>
#include <cryptopp/files.h>
#include <cryptopp/modes.h>

int main() {
    CryptoPP::AutoSeededRandomPool rng;

    // Генерация закрытого ключа RSA (2048 бит)
    CryptoPP::RSA::PrivateKey privateKey;
    privateKey.GenerateRandomWithKeySize(rng, 2048);

    // Сохранение закрытого ключа в файл
    CryptoPP::FileSink privateKeyFile("private_key.pem");
    privateKey.Save(privateKeyFile);

    std::cout << "Закрытый ключ RSA сгенерирован." << std::endl;

    // Получение публичного ключа из закрытого ключа
    CryptoPP::RSA::PublicKey publicKey;
    publicKey = privateKey;

    // Сохранение публичного ключа в файл
    CryptoPP::FileSink publicKeyFile("public_key.pem");
    publicKey.Save(publicKeyFile);

    std::cout << "Публичный ключ RSA извлечен." << std::endl;

    // Генерация случайного симметричного ключа AES
    CryptoPP::SecByteBlock symmetricKey(32); // Выберите длину ключа AES
    rng.GenerateBlock(symmetricKey, symmetricKey.size());

    // Сохранение симметричного ключа в файл
    std::ofstream symmetricKeyFile("symmetric_key.bin", std::ios::binary);
    symmetricKeyFile.write(reinterpret_cast<char*>(symmetricKey.BytePtr()), symmetricKey.size());
    symmetricKeyFile.close();

    std::cout << "Симметричный ключ AES сгенерирован." << std::endl;

    // Загрузка симметричного ключа из файла
    std::ifstream symmetricKeyFileIn("symmetric_key.bin", std::ios::binary);
    symmetricKeyFileIn.read(reinterpret_cast<char*>(symmetricKey.BytePtr()), symmetricKey.size());
    symmetricKeyFileIn.close();

    // Шифрование данных с использованием симметричного ключа (AES)
    std::string plaintext = "Oleksii";
    std::string ciphertext;
    
    CryptoPP::byte iv[CryptoPP::AES::BLOCKSIZE];
    rng.GenerateBlock(iv, sizeof(iv));


    CryptoPP::AES::Encryption aesEncryption(symmetricKey.BytePtr(), symmetricKey.size());
    CryptoPP::CBC_Mode_ExternalCipher::Encryption cbcEncryption(aesEncryption, iv);

    CryptoPP::StreamTransformationFilter encryptor(cbcEncryption, new CryptoPP::StringSink(ciphertext));
    encryptor.Put(reinterpret_cast<const unsigned char*>(plaintext.data()), plaintext.size());
    encryptor.MessageEnd();

    std::cout << "Данные успешно зашифрованы с использованием симметричного ключа AES." << std::endl;

    // Шифрование симметричного ключа с использованием публичного ключа RSA
    CryptoPP::RSAES_OAEP_SHA_Encryptor rsaEncryptor(publicKey);
    std::string encryptedSymmetricKey;

    CryptoPP::StringSource s(symmetricKey.BytePtr(), symmetricKey.size(), true, new CryptoPP::PK_EncryptorFilter(rng, rsaEncryptor, new CryptoPP::StringSink(encryptedSymmetricKey)));

    std::cout << "Симметричный ключ успешно зашифрован с использованием публичного ключа RSA." << std::endl;

    // Расшифрование симметричного ключа с использованием закрытого ключа RSA
    CryptoPP::RSAES_OAEP_SHA_Decryptor rsaDecryptor(privateKey);
    std::string decryptedSymmetricKey;

    CryptoPP::StringSource ss(encryptedSymmetricKey, true, new CryptoPP::PK_DecryptorFilter(rng, rsaDecryptor, new CryptoPP::StringSink(decryptedSymmetricKey)));

    std::cout << "Симметричный ключ успешно расшифрован с использованием закрытого ключа RSA." << std::endl;

    // Расшифрование данных симметричным ключом (AES)
    std::string recoveredPlaintext;

    CryptoPP::AES::Decryption aesDecryption(symmetricKey.BytePtr(), symmetricKey.size());
    CryptoPP::CBC_Mode_ExternalCipher::Decryption cbcDecryption(aesDecryption, iv);

    CryptoPP::StreamTransformationFilter decryptor(cbcDecryption, new CryptoPP::StringSink(recoveredPlaintext));
    decryptor.Put(reinterpret_cast<const unsigned char*>(ciphertext.data()), ciphertext.size());
    decryptor.MessageEnd();

    std::cout << "Данные успешно расшифрованы с использованием симметричного ключа AES." << std::endl;

    // Вывод результатов
    std::cout << "Зашифрованный ключ RSA: " << encryptedSymmetricKey << std::endl;
    std::cout << "Расшифрованный ключ AES: " << decryptedSymmetricKey << std::endl;
    std::cout << "Зашифрованный текст: " << ciphertext << std::endl;
    std::cout << "Расшифрованный текст: " << recoveredPlaintext << std::endl;

    return 0;
}
