import time
from Crypto.PublicKey import RSA
from Crypto.Cipher import PKCS1_OAEP
from memory_profiler import profile

def generate_rsa_key_pair(key_size=2048):
    key = RSA.generate(key_size)
    return key

def rsa_encrypt(plaintext, public_key):
    cipher = PKCS1_OAEP.new(public_key)
    ciphertext = cipher.encrypt(plaintext.encode('utf-8'))
    return ciphertext

def rsa_decrypt(ciphertext, private_key):
    cipher = PKCS1_OAEP.new(private_key)
    plaintext = cipher.decrypt(ciphertext)
    return plaintext.decode('utf-8')

@profile
def main():
    plaintext = "This is a secret message."

    generation_time = 0
    encryption_time = 0
    decryption_time = 0

    for i in range(1000):
        start_time = time.time()
        rsa_key = generate_rsa_key_pair()
        end_time = time.time()
        execution_time = end_time - start_time
        generation_time = generation_time + (execution_time - generation_time) / (i + 1)

        start_time_encryption = time.time()
        ciphertext = rsa_encrypt(plaintext, rsa_key.publickey())
        end_time_encryption = time.time()
        execution_time = end_time_encryption - start_time_encryption
        encryption_time = encryption_time + (execution_time - encryption_time) / (i + 1)

        start_time_decryption = time.time()
        decrypted_text = rsa_decrypt(ciphertext, rsa_key)
        end_time_decryption = time.time()
        execution_time = end_time_decryption - start_time_decryption
        decryption_time = decryption_time + (execution_time - decryption_time) / (i + 1)

    print(f"RSA Key Generation Time: {generation_time} seconds")
    print(f"RSA Encryption Time: {encryption_time} seconds")
    print(f"RSA Decryption Time: {decryption_time} seconds")
    print(f"Decrypted Text: {decrypted_text}")

if __name__ == "__main__":
    main()