from Crypto.PublicKey import RSA
from Crypto.Random import get_random_bytes
from Crypto.Cipher import AES, PKCS1_OAEP
import rsa
import time

def pycrypto():
    
    start_gen=time.time()
    for i in range(0,100):
        key = RSA.generate(2048)
        encrypted_key = key.export_key()
        print(encrypted_key)

    end_gen=time.time()
    print("Keygen RSA 2048bit 100 iteration: ",end_gen - start_gen)

    
    start_enc=time.time()

    for a in range(0,1000):
        out=open("out.txt","wb")
        dataf=open("in.txt","r")
        data=str(dataf.read()).encode("utf-8")
        recipient_key = RSA.import_key(open("private.pem").read())
        session_key = get_random_bytes(16)
        cipher_rsa = PKCS1_OAEP.new(recipient_key)
        enc_session_key = cipher_rsa.encrypt(session_key)
        cipher_aes = AES.new(session_key, AES.MODE_EAX)
        ciphertext, tag = cipher_aes.encrypt_and_digest(data)
        [ out.write(x) for x in (enc_session_key, cipher_aes.nonce, tag, ciphertext) ]
        out.close()
        dataf.close()
    end_enc=time.time()
    print("Encrypt RSA 2048bit 1000x128byte: ",end_enc - start_enc)
    
    start_dec=time.time()
    for b in range(0,1000):
        file_in = out=open("out.txt","rb")
        private_key = RSA.import_key(open("private.pem").read())
        enc_session_key, nonce, tag, ciphertext = \
           [ file_in.read(x) for x in (private_key.size_in_bytes(), 16, 16, -1) ]
        file_in.close()
        cipher_rsa = PKCS1_OAEP.new(private_key)
        session_key = cipher_rsa.decrypt(enc_session_key)
        cipher_aes = AES.new(session_key, AES.MODE_EAX, nonce)
        data = cipher_aes.decrypt_and_verify(ciphertext, tag)
        open("dec.txt","w").write(data.decode("utf-8"))
        file_in.close()

    end_dec=time.time()
    print("Decrypt RSA 2048bit 1000x128byte: ",end_dec - start_dec)

def pure_rsa():

    start_gen=time.time()
    for i in range(0,100):
        (public,private)=rsa.newkeys(2048)
        print(rsa.PrivateKey.save_pkcs1(private))

    end_gen=time.time()
    print("Keygen RSA 2048bit 100 iteration: ",end_gen - start_gen)

    start_enc=time.time()

    for a in range(0,1000):
        pem=open("public_py.pem","rb").read()
        pubkey = rsa.PublicKey.load_pkcs1(pem)
        
        dataf=open("in.txt","r")
        data=dataf.read().encode("utf-8")
        enc_data=rsa.encrypt(data,pubkey)
        open("out.txt","wb").write(enc_data)
    end_enc=time.time()
    print("Encrypt RSA 2048bit 1000x128byte: ",end_enc - start_enc)
    
    start_dec=time.time()
    for b in range(0,1000):
        pem=open("private_py.pem","rb").read()
        privkey = rsa.PrivateKey.load_pkcs1(pem)
        
        dataf=open("out.txt","rb")
        data=dataf.read()
        enc_data=rsa.decrypt(data,privkey).decode("utf-8")
        open("dec.txt","w").write(enc_data)
        
    end_dec=time.time()
    print("Decrypt RSA 2048bit 1000x128byte: ",end_dec - start_dec)

pycrypto()
pure_rsa()


