from time_speed import time_count
from OpenSSL.crypto import PKey, TYPE_RSA
from cryptography.hazmat.primitives.asymmetric.padding import MGF1, OAEP
from cryptography.hazmat.primitives.hashes import SHA256


COUNT_ITER = 1000


class WorkKeys:

    def generate_key_pair(self):
        key = PKey()
        key.generate_key(TYPE_RSA, 2048)
        self.__key_new = key
    
    @property
    def key_private(self) -> PKey:
        return self.__key_new.to_cryptography_key()
    
    @property
    def key_public(self):
        return self.__key_new.to_cryptography_key().public_key()
    
    @property
    def padding_shcema(self):
        return OAEP(
                mgf=MGF1(algorithm=SHA256()),
                algorithm=SHA256(),
                label=None
            )
            
    def encrypt_massage(self, massage: bytes):
        return self.key_public.encrypt(massage, self.padding_shcema)
        
    def decrypt_massage(self, cipher_massage: bytes):
        self.key_private.decrypt(
            cipher_massage,
            self.padding_shcema
        )


@time_count
def check_generate_key_pair(key_work_space: WorkKeys):
    for _ in range(COUNT_ITER):
        key_work_space.generate_key_pair()


@time_count
def check_encrypt_massage(key_work_space: WorkKeys):
    for _ in range(COUNT_ITER):
        key_work_space.encrypt_massage(b'Hello ones !')


@time_count
def decript(key_work_space: WorkKeys, cipher_massage: bytes):
    for _ in range(COUNT_ITER):
        key_work_space.decrypt_massage(cipher_massage)
    

if __name__ == "__main__":
    key_instance_work = WorkKeys()

    print('Genarate 1000 keys')
    check_generate_key_pair(key_instance_work)

    print('cipher 1000 massage')
    key_instance_work.generate_key_pair()
    check_encrypt_massage(key_instance_work)

    print('decipher 1000 massage')
    key_instance_work.generate_key_pair()
    cipher_text = key_instance_work.encrypt_massage(b'Hack me!')
    decript(key_instance_work, cipher_text)
