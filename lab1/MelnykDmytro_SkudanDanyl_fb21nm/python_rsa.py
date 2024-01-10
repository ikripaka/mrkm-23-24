import rsa
from time_speed import time_count


COUNT_ITER = 1000


class WorkKeys:

    def generate_key_pair(self):
        self.__public_key, self.__private_key = rsa.newkeys(2048)
    
    @property
    def key_private(self):
        return self.__private_key
    
    @property
    def key_public(self):
        return self.__public_key
            
    def encrypt_massage(self, massage: bytes):
        return rsa.encrypt(massage, self.key_public)
        
    def decrypt_massage(self, cipher_massage: bytes):
        rsa.decrypt(cipher_massage, self.key_private).decode("utf-8")


@time_count
def check_generate_key_pair(key_work_space: WorkKeys):
    for _ in range(1):
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

    print('Genarate 1 keys')
    check_generate_key_pair(key_instance_work)

    print('cipher 1000 massage')
    key_instance_work.generate_key_pair()
    check_encrypt_massage(key_instance_work)

    print('decipher 1000 massage')
    key_instance_work.generate_key_pair()
    cipher_text = key_instance_work.encrypt_massage(b'Hack me!')
    decript(key_instance_work, cipher_text)
