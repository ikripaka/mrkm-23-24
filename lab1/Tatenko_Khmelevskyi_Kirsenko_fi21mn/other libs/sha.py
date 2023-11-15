import codecs
import time
from hashlib import sha256


@profile
def main_sha():
    fileObj = codecs.open( "input.txt", "r", "utf_8_sig" )
    text = fileObj.read() # или читайте по строке
    fileObj.close()
    
    start = time.time()
    res = sha256(text.encode('utf-8')).hexdigest()
    end = time.time()
    hash_time = end - start
    
    
    print(f"Hash: {res}")
    print(f"Hash time: {hash_time} seconds")
    
if __name__ == '__main__':
    main_sha()