#include <iostream>
#include <chrono>
#include <iomanip>
#include <io.h>
#include <fcntl.h>



class Timer
{
private:
	using clock_t = std::chrono::high_resolution_clock;
	using second_t = std::chrono::duration<double, std::ratio<1> >;

	std::chrono::time_point<clock_t> m_beg;

public:
	Timer() : m_beg(clock_t::now())
	{
	}

	void reset()
	{
		m_beg = clock_t::now();
	}

	double elapsed() const
	{
		return std::chrono::duration_cast<second_t>(clock_t::now() - m_beg).count();
	}
};


int main()
{

	
	//OpenSSL KeyGen RSA speed test
	Timer openssl_keygen_timer;
	for (int i = 0; i < 100; i++) {
		system("openssl.exe genrsa 2048");
	}
	std::cout<<"Keygen RSA 2048bit 100 iteration: " << std::setprecision(20) <<openssl_keygen_timer.elapsed()<<std::endl;



	//Encryption RSA speed test
	system("openssl.exe genrsa -out private.pem 2048");
	system("openssl.exe rsa -in private.pem -pubout -out public.pem");
	Timer openssl_enc_timer;
	for (int i = 0; i < 1000; i++) {
		system("openssl.exe pkeyutl -encrypt -inkey private.pem -in in.txt -out out.txt");
	}
	std::cout << "Encrypt RSA 2048bit 1000x128byte: " << std::setprecision(20) << openssl_enc_timer.elapsed() << std::endl;
	//Decryption RSA speed test
	Timer openssl_dec_timer;
	for (int i = 0; i < 1000; i++) {
		system("openssl.exe pkeyutl -decrypt -inkey private.pem -in out.txt -out dec.txt");
	}
	std::cout << "Decrypt RSA 2048bit 1000x128byte: " << std::setprecision(20) << openssl_dec_timer.elapsed() << std::endl;
	

	

}

