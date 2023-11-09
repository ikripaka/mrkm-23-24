from Crypto.Random.random import sample

# Пример использования
population = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]  
k = 4 
random_sample = sample(population, k)

print(f'{k} random unique elements: {random_sample}')
