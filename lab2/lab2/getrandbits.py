from Crypto.Random import random

k = 32 
random_bits = random.getrandbits(k)
print(f'{k}-bit random number: {random_bits}')
