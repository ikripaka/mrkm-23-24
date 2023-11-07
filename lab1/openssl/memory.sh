#!/bin/bash

# Запуск скрипта с использованием valgrind и massif
valgrind --tool=massif --massif-out-file=massif.out ./openssl.sh

# Анализ результатов
ms_print massif.out
