from time_speed import time_count
from Crypto.Cipher import AES
from Crypto.Random import get_random_bytes
from Crypto.Util.Padding import pad, unpad


COUNT_ITER = 1000


class WorkKeys:

    def generate_key_pair(self):
        self.__key_new = get_random_bytes(32)
        self.__cipher = AES.new(self.__key_new, AES.MODE_CBC)
        self.__decipher = AES.new(self.__key_new, AES.MODE_CBC, iv=self.__cipher.iv)
    
    @property
    def cipher(self) -> AES:
        return self.__cipher
    
    @property
    def decipher(self) -> AES:
        return self.__decipher
            
    def encrypt_massage(self, massage: bytes):
        return self.cipher.encrypt(pad(massage, AES.block_size))
        
    def decrypt_massage(self, cipher_massage: str):
        self.__decipher = AES.new(self.__key_new, AES.MODE_CBC, iv=self.__cipher.iv)
        unpad(self.decipher.decrypt(cipher_massage), AES.block_size)


@time_count
def check_generate_key_pair(key_work_space: WorkKeys):
    for _ in range(COUNT_ITER):
        key_work_space.generate_key_pair()


@time_count
def check_encrypt_massage(key_work_space: WorkKeys):
    for _ in range(COUNT_ITER):
        key_work_space.encrypt_massage('Hello ones !'.encode('utf-8'))


@time_count
def decript(key_work_space: WorkKeys, cipher_massage: str):
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
    cipher_text = key_instance_work.encrypt_massage('Hack me!'.encode('utf-8'))
    decript(key_instance_work, cipher_text)
