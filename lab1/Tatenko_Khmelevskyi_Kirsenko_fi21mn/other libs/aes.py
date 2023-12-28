import codecs
import time
import os
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes

@profile
def main_aes():
    fileObj = codecs.open( "input.txt", "r", "utf_8_sig" )
    text = fileObj.read() # или читайте по строке
    fileObj.close()

    new_text = bytes(text + " " * 9,'UTF-8')
    key = os.urandom(32)
    iv = os.urandom(16)
    cipher = Cipher(algorithms.AES(key), modes.CBC(iv))

    start = time.time()
    encryptor = cipher.encryptor()
    ct = encryptor.update(new_text) + encryptor.finalize()
    end = time.time()
    encryption_time = end - start

    start = time.time()
    decryptor = cipher.decryptor()
    dec = decryptor.update(ct) + decryptor.finalize()
    text2 = dec.decode('utf-8')
    end = time.time()
    decryption_time = end - start

    print(f"Encryption time: {encryption_time} seconds")
    print(f"Decryption time: {decryption_time} seconds")

if __name__ == '__main__':
    main_aes()