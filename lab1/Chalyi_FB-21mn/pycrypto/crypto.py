#!/usr/bin/env python

from Crypto.Cipher import AES, PKCS1_OAEP
from Crypto.PublicKey import RSA
from Crypto.Random import get_random_bytes
from Crypto.Util.Padding import pad, unpad

# Генерация закрытого ключа RSA (2048 бит)
private_key = RSA.generate(2048)

# Сохранение закрытого ключа в файл
private_key_file = open("private_key.pem", "wb")
private_key_file.write(private_key.export_key())
private_key_file.close()

print("Закрытый ключ RSA сгенерирован.")

# Получение публичного ключа из закрытого ключа
public_key = private_key.publickey()

# Сохранение публичного ключа в файл
public_key_file = open("public_key.pem", "wb")
public_key_file.write(public_key.export_key())
public_key_file.close()

print("Публичный ключ RSA извлечен.")

# Генерация случайного симметричного ключа AES
symmetric_key = get_random_bytes(16)  # 16 байт для AES-128

# Сохранение симметричного ключа в файл
symmetric_key_file = open("symmetric_key.bin", "wb")
symmetric_key_file.write(symmetric_key)
symmetric_key_file.close()

print("Симметричный ключ AES сгенерирован.")

# Загрузка симметричного ключа из файла
symmetric_key_file_in = open("symmetric_key.bin", "rb")
symmetric_key = symmetric_key_file_in.read()
symmetric_key_file_in.close()

# Шифрование данных с использованием симметричного ключа (AES)
plaintext = "Oleksii"
iv = get_random_bytes(16)  # Генерация случайного вектора инициализации
cipher = AES.new(symmetric_key, AES.MODE_CBC, iv)

# Добавляем дополнение PKCS7
padded_plaintext = pad(plaintext.encode('utf-8'), AES.block_size)
ciphertext = iv + cipher.encrypt(padded_plaintext)

# Шифрование симметричного ключа с использованием публичного ключа RSA
rsa_cipher = PKCS1_OAEP.new(public_key)
encrypted_symmetric_key = rsa_cipher.encrypt(symmetric_key)

encrypted_symmetric_key_file = open("encrypted_symmetric_key.bin", "wb")
encrypted_symmetric_key_file.write(encrypted_symmetric_key)
encrypted_symmetric_key_file.close()
print("Симметричный ключ успешно зашифрован с использованием публичного ключа RSA.")

# Расшифрование симметричного ключа с использованием закрытого ключа RSA
rsa_decipher = PKCS1_OAEP.new(private_key)
decrypted_symmetric_key = rsa_decipher.decrypt(encrypted_symmetric_key)

decrypted_symmetric_key_file = open("decrypted_symmetric_key.bin", "wb")
decrypted_symmetric_key_file.write(decrypted_symmetric_key)
decrypted_symmetric_key_file.close()
print("Симметричный ключ успешно расшифрован с использованием закрытого ключа RSA.")

# Расшифрование данных симметричным ключом (AES)
iv = ciphertext[:16]
cipher = AES.new(decrypted_symmetric_key, AES.MODE_CBC, iv)
padded_plaintext = cipher.decrypt(ciphertext[16:])
plaintext = unpad(padded_plaintext, AES.block_size).decode('utf-8')

# Вывод зашифрованного ключа RSA
print("Зашифрованный ключ RSA:", encrypted_symmetric_key.hex())

# Вывод расшифрованного ключа AES
print("Расшифрованный ключ AES:", decrypted_symmetric_key.hex())

# Вывод зашифрованного текста
print("Зашифрованный текст:", ciphertext.hex())
print("Данные успешно зашифрованы с использованием симметричного ключа AES.")

print("Данные успешно расшифрованы с использованием симметричного ключа AES.")
print("Расшифрованный текст:", plaintext)
