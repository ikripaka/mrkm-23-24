from time import time

def time_count(func):
    def wrapper(*args, **kwargs):
        start_time = time()
        result = func(*args, **kwargs)
        end_time = time()

        print(f'result_time {end_time - start_time}')
        return result
    return wrapper
