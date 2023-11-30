#ifndef _GLIBCXX_NO_ASSERT
#include <cassert>
#endif
#include <cctype>
#include <cerrno>
#include <cfloat>
#include <ciso646>
#include <climits>
#include <clocale>
#include <cmath>
#include <csetjmp>
#include <csignal>
#include <cstdarg>
#include <cstddef>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>

#if __cplusplus >= 201103L
#include <ccomplex>
#include <cfenv>
#include <cinttypes>
#include <cstdalign>
#include <cstdbool>
#include <cstdint>
#include <ctgmath>
#include <cwchar>
#include <cwctype>
#endif

// C++
#include <algorithm>
#include <bitset>
#include <complex>
#include <deque>
#include <exception>
#include <fstream>
#include <functional>
#include <iomanip>
#include <ios>
#include <iosfwd>
#include <iostream>
#include <istream>
#include <iterator>
#include <limits>
#include <list>
#include <locale>
#include <map>
#include <memory>
#include <new>
#include <numeric>
#include <ostream>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <streambuf>
#include <string>
#include <typeinfo>
#include <utility>
#include <valarray>
#include <vector>

#if __cplusplus >= 201103L
#include <array>
#include <atomic>
#include <chrono>
#include <condition_variable>
#include <forward_list>
#include <future>
#include <initializer_list>
#include <mutex>
#include <random>
#include <ratio>
#include <regex>
#include <scoped_allocator>
#include <system_error>
#include <thread>
#include <tuple>
#include <typeindex>
#include <type_traits>
#include <unordered_map>
#include <unordered_set>
#endif
#include <C:/Users/Арсений/Desktop/Учеба 13 семестр/МРКМ/Lab 1 InfInt/InfInt/InfInt.h>
#include <chrono>
#include "windows.h"
#include <ios>
#include <fstream>
#include <string>
using namespace std::chrono;

using namespace std;



InfInt NthFibonacci(int n) {
    InfInt a(1), b(1), c;
    if (!n)
        return c;
    n--;
    while (n--) {
        c = a + b;
        b = a;
        a = c;
    }
    return b;
}

InfInt Factorial(int n) {
    InfInt f(1);
    for (int i = 2; i <= n; i++)
        f *= i;
    return f;
}



int main(void)
{
    /*int mas[10];
    for (int j = 0; j < 10; j++)
    {
        cout << "-------------------------Fibonacci"
            << "------------------------------\n";
        auto start = high_resolution_clock::now();
        for (int i = 0; i <= 100; i++) {
            InfInt Fib;
            Fib = NthFibonacci(i);
            cout << "Fibonacci " << i << " = " << Fib << '\n';
        }
        auto stop = high_resolution_clock::now();
        auto duration = duration_cast<microseconds>(stop - start);
        cout << "Execution time(microseconds): " << duration.count() << endl;
        mas[j] = duration.count();
    }

    cout << endl << endl;
    int sum = 0;
    for (int i = 0; i < 10; i++)
    {
        sum += mas[i];
    }
    cout << sum / 10;*/







    /*
    int mas[10];
    for (int j = 0; j < 10; j++)
    {
        cout << "-------------------------Factorial"
            << "------------------------------\n";
        auto start = high_resolution_clock::now();
        for (int i = 0; i <= 100; i++) {
            InfInt fact;
            fact = Factorial(i);
            cout << "Factorial of "
                << i << " = ";
            cout << fact << '\n';
        }
        auto stop = high_resolution_clock::now();
        auto duration = duration_cast<microseconds>(stop - start);
        cout << "Execution time(microseconds): " << duration.count() << endl;
        mas[j] = duration.count();
    }
    cout << endl << endl;
    int sum = 0;
    for (int i = 0; i < 10; i++)
    {
        sum += mas[i];
    }
    cout << sum / 10;
   */

    InfInt a = 2;
    for (int i = 1; i < 1024; i++)
    {
        a = a * 2;
        cout << i << endl;
    }
    cout << a;

    return 0;
}
