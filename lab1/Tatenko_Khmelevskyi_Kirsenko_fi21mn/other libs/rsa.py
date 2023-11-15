import codecs
import time
from rsa_python import rsa


@profile
def main_rsa():
    fileObj = codecs.open( "input_rsa.txt", "r", "utf_8_sig" )
    text_rsa = fileObj.read() # или читайте по строке
    fileObj.close()
    
    key_pair = rsa.generate_key_pair(1024)
    
    start = time.time()
    cipher = rsa.encrypt(text_rsa, key_pair["public"], key_pair["modulus"])
    end = time.time()
    encryption_time = end - start
    
    start = time.time()
    decrypted_message = rsa.decrypt(cipher, key_pair["private"], key_pair["modulus"])
    end = time.time()
    decryption_time = end - start
    
    print(f"Encryption time: {encryption_time} seconds")
    print(f"Decryption time: {decryption_time} seconds")
    
if __name__ == '__main__':
    main_rsa()