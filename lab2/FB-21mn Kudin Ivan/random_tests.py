import secrets
import time
import os
import random


def random_method(x):

    start_time = time.time()
    random_number = []
    for _ in range(x):
        random_number.append(random.randrange(1, 20, 1))\
    end_time = time.time()
    print(end_time - start_time)


def urandom_method(y):
    start_time = time.time()
    random_number = []
    for _ in range(y):
        random_number.append(os.urandom(2))
    end_time = time.time()
    print(end_time - start_time)


random_method(10000)
urandom_method(10000)