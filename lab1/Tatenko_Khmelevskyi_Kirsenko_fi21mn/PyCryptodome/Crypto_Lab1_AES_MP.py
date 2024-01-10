import time
from Crypto.Cipher import AES
from Crypto.Random import get_random_bytes
from memory_profiler import profile

def encrypt_data(plaintext, key):
    cipher = AES.new(key, AES.MODE_EAX)
    plaintext_bytes = plaintext.encode('utf-8')
    ciphertext, tag = cipher.encrypt_and_digest(plaintext_bytes)
    return cipher, ciphertext, tag

def decrypt_data(cipher, ciphertext, tag, key):
    decrypt_cipher = AES.new(key, AES.MODE_EAX, nonce=cipher.nonce)
    decrypted_text_bytes = decrypt_cipher.decrypt_and_verify(ciphertext, tag)
    decrypted_text = decrypted_text_bytes.decode('utf-8')
    return decrypted_text

@profile
def main():
    with open('input.txt', 'r', encoding='utf-8') as file:
        plaintext = file.read()

    encryption_time = 0
    decryption_time = 0

    for i in range(1000):
        key = get_random_bytes(32)

        start_time = time.time()
        cipher, ciphertext, tag = encrypt_data(plaintext, key)
        end_time = time.time()

        execution_time = end_time - start_time
        encryption_time = encryption_time + (execution_time - encryption_time) / (i + 1)

        start_time = time.time()
        decrypted_text = decrypt_data(cipher, ciphertext, tag, key)
        end_time = time.time()

        execution_time = end_time - start_time
        decryption_time = decryption_time + (execution_time - decryption_time) / (i + 1)

    print(f"Encryption Time: {encryption_time} seconds")
    print(f"Decryption Time: {decryption_time} seconds")

if __name__ == "__main__":
    main()