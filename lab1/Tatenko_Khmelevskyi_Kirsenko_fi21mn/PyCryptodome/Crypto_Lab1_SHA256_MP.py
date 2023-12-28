import time
from Crypto.Hash import SHA256
from memory_profiler import profile

def calculate_sha256_hash(data):
    sha256 = SHA256.new()
    sha256.update(data.encode('utf-8'))
    hash_result = sha256.hexdigest()
    return hash_result

@profile
def main():
    with open('input.txt', 'r', encoding='utf-8') as file:
        plaintext = file.read()

    hash_time = 0

    for i in range(10000):
        start_time = time.time()
        hash_result = calculate_sha256_hash(plaintext)
        end_time = time.time()

        execution_time = end_time - start_time
        hash_time = hash_time + (execution_time - hash_time) / (i + 1)

    print(f"Time: {hash_time} seconds")
    print(f"SHA-256 Hash: {hash_result}")

if __name__ == "__main__":
    main()