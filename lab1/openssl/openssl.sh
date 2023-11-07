#!/bin/bash
start_time=$(date +%s.%N)  # Запуск таймера
# Генерация закрытого ключа RSA (2048 бит)
openssl genpkey -algorithm RSA -out private_key.pem
echo "Закрытый ключ RSA сгенерирован."

# Извлечение публичного ключа из закрытого ключа
openssl rsa -pubout -in private_key.pem -out public_key.pem
echo "Публичный ключ RSA извлечен."

# Генерация случайного симметричного ключа AES
openssl rand -out symmetric_key.bin 32
echo "Симметричный ключ AES сгенерирован."

# Шифрование данных с использованием симметричного ключа (AES)
openssl enc -aes-256-cfb -in plaintext.txt -out encrypted_data.enc -pass file:symmetric_key.bin -pbkdf2
echo "Данные успешно зашифрованы с использованием симметричного ключа AES."

# Шифрование симметричного ключа с использованием публичного ключа RSA
openssl pkeyutl -encrypt -pubin -inkey public_key.pem -in symmetric_key.bin -out encrypted_symmetric_key.bin
echo "Симметричный ключ успешно зашифрован с использованием публичного ключа RSA."

# Расшифрование симметричного ключа с использованием закрытого ключа RSA
openssl pkeyutl -decrypt -inkey private_key.pem -in encrypted_symmetric_key.bin -out symmetric_key.bin
echo "Симметричный ключ успешно расшифрован с использованием закрытого ключа RSA."

# Расшифрование данных симметричным ключом (AES)
openssl enc -d -aes-256-cfb -in encrypted_data.enc -out decrypted_data.txt -pass file:symmetric_key.bin -pbkdf2
echo "Данные успешно расшифрованы с использованием симметричного ключа AES."

# Вывод результатов
echo "Зашифрованный ключ RSA:"
cat encrypted_symmetric_key.bin
echo "Расшифрованный ключ AES:"
cat symmetric_key.bin
echo "Зашифрованный текст:"
cat encrypted_data.enc
echo "Расшифрованный текст:"
cat decrypted_data.txt
end_time=$(date +%s.%N)    # Остановка таймера
execution_time=$(echo "$end_time - $start_time" | bc)

echo "Время выполнения скрипта: $execution_time секунд"
