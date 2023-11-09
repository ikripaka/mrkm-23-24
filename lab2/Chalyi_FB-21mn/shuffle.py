from Crypto.Random.random import shuffle

sequence = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] 
shuffled_sequence = list(sequence) 
shuffle(shuffled_sequence) 

print(f'Shuffled sequence: {shuffled_sequence}')
