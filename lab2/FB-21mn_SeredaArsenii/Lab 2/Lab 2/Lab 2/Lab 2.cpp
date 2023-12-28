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
#include <C:/Users/Арсений/Desktop/Учеба 13 семестр/СТК что то там/Lab 1 InfInt/InfInt/InfInt.h>
#include <chrono>
#include "windows.h"
#include <ios>
#include <fstream>
#include <string>
#include "math.h"
using namespace std::chrono;
using namespace std;

const int N = 256; ///change 


InfInt GetRandom()
{
	cout << "GetRandom:";
	int mas[N];
	srand(time(0));
	int a = 0;
	for (int i = 0; i < N; i++)
	{
		a = rand();
		mas[i] = a % 2;
	}

	InfInt b = 0;
	InfInt c = 1;
	InfInt last_c = 1;
	int last_i = 0;

	for (int i = 0; i < N ; i++)
	{
		if (mas[i] == 1)
		{
			//cout << i << ":";
			c = last_c;
			for (int j = last_i; j < i; j++)
			{
				c = c * 2;
			}
			b = b + c;
			last_i = i;
			last_c = c;
		}
	}
	cout << "GetRandomEND:";
	return b;
}

bool IsPrime(InfInt n)
{
	cout << "IsPrime:";
	if (n == 0 || n == 1) {
		return false;
	}

	for (InfInt i = 2; i <= n / i; ++i) 
	{
		if (n % i == 0) {
			return false;
		}
	}

	return true;
}

//////////
InfInt modulo(InfInt base, InfInt e, InfInt mod) {
	cout << "Modulo:";
	InfInt a = 1;
	InfInt b = base;
	while (e > 0) {
		if (e % 2 == 1)
			a = (a * b) % mod;
		b = (b * b) % mod;
		e = e / 2;
	}
	return a % mod;
}


bool Fermat(InfInt m, int iterations) {
	cout << "Fermat:";
	if (m == 1) {
		return false;
	}
	for (int i = 0; i < iterations; i++) {
		InfInt x = GetRandom()%(m - 1) + 1;
		if (modulo(x, m - 1, m) != 1) {
			return false;
		}
	}
	return true;
}
////////////

InfInt GetPrime(InfInt n)
{
	cout << "GetPrime:";
	for (InfInt i = n; ; i++) {
		/*if (IsPrime(i)) {
			return i;
		}*/
		if (Fermat(i, 2)) {
			if (Fermat(i, 5))
			{
				return i;
			}
		}
	}
}

/*InfInt Maurer()
{
	cout << "Maurer:";
	InfInt prime_mas[100];
	int counter = 0;
	for (InfInt i = 0; ; i++)
	{
		if (IsPrime(i))
		{
			prime_mas[counter] = i;
			counter++;
		}
	}

	for (int i = 0; i < 100; i++)
	{
		cout << prime_mas[i] << " ";
	}
}
*/

int main(void)
{
	InfInt a = GetPrime(GetRandom());
	cout << a;
}

