// Lab1 created by S.O.

#include <iostream>

#include <chrono>
#include <gmp.h>
#include <random>

/* 1 - Show detailed information. 0 - Show short form. */
#define DEF_DETAILED 1 

std::mt19937_64 glob_mtGenerator(std::chrono::steady_clock::now().time_since_epoch().count());

void vfCheckIntArithmetic();


int main()
{
    std::cout << "Laboratory #1" << std::endl;
    
    vfCheckIntArithmetic();
    
    return 0;
}

// Test 'add', 'sub', 'mul', 'div'.
void vfCheckIntArithmetic()
{
    const int ct_iArrSize = 5;
    
    double dArrTime[ct_iArrSize];
    double dTimeSum = 0.0;

    mpz_t mTmp;
    mpz_init(mTmp);
    unsigned long int uliNum1Tmp = 0, uliNum2Tmp = 0;
    std::chrono::steady_clock::time_point tp_TimeBegin = std::chrono::steady_clock::now(), 
    tp_TimeEnd = tp_TimeBegin;
    
    // ADD block.
    std::cout << "Checking 'add' functions..." << std::endl;
    for (int iIter = 0; iIter < ct_iArrSize; iIter++)
    {
        uliNum1Tmp = glob_mtGenerator();
        uliNum2Tmp = glob_mtGenerator();

        tp_TimeBegin = std::chrono::steady_clock::now();
        mpz_set_ui(mTmp, uliNum1Tmp);
        mpz_add_ui(mTmp, mTmp, uliNum2Tmp);
        tp_TimeEnd = std::chrono::steady_clock::now();

        dArrTime[iIter] = (std::chrono::duration_cast<std::chrono::duration<double>>(tp_TimeEnd - tp_TimeBegin)).count();
        dTimeSum += dArrTime[iIter];
        
        if (DEF_DETAILED)
        {
            std::cout << "\t[+]: " << uliNum1Tmp  << " + " << uliNum2Tmp << " = ";
            mpz_out_str(stdout, 10, mTmp);
            std::cout << " (time: " << dArrTime[iIter] << " s.)" << std::endl;
        }
    }

    dTimeSum = (dTimeSum / (double)ct_iArrSize);
    std::cout << "[AVG time]: " << dTimeSum << " s. ";
    for (int iIter = 0; iIter < ct_iArrSize; iIter++)
    {
        if (iIter == 0)
            std::cout << "(";
        std::cout << dArrTime[iIter];

        if (iIter != ct_iArrSize - 1)
            std::cout << ", ";
        else
            std::cout << ")" << std::endl << std::endl;
    }

    // SUB block.
    std::cout << "Checking 'sub' functions..." << std::endl;
    for (int iIter = 0; iIter < ct_iArrSize; iIter++)
    {
        uliNum1Tmp = glob_mtGenerator();
        uliNum2Tmp = glob_mtGenerator();

        tp_TimeBegin = std::chrono::steady_clock::now();
        mpz_set_ui(mTmp, uliNum1Tmp);
        mpz_sub_ui(mTmp, mTmp, uliNum2Tmp);
        tp_TimeEnd = std::chrono::steady_clock::now();

        dArrTime[iIter] = (std::chrono::duration_cast<std::chrono::duration<double>>(tp_TimeEnd - tp_TimeBegin)).count();
        dTimeSum += dArrTime[iIter];
        
        if (DEF_DETAILED)
        {
            std::cout << "\t[+]: " << uliNum1Tmp  << " - " << uliNum2Tmp << " = ";
            mpz_out_str(stdout, 10, mTmp);
            std::cout << " (time: " << dArrTime[iIter] << " s.)" << std::endl;
        }
    }

    dTimeSum = (dTimeSum / (double)ct_iArrSize);
    std::cout << "[AVG time]: " << dTimeSum << " s. ";
    for (int iIter = 0; iIter < ct_iArrSize; iIter++)
    {
        if (iIter == 0)
            std::cout << "(";
        std::cout << dArrTime[iIter];

        if (iIter != ct_iArrSize - 1)
            std::cout << ", ";
        else
            std::cout << ")" << std::endl << std::endl;
    }

    // MUL block.
    std::cout << "Checking 'mul' functions..." << std::endl;
    for (int iIter = 0; iIter < ct_iArrSize; iIter++)
    {
        uliNum1Tmp = glob_mtGenerator();
        uliNum2Tmp = glob_mtGenerator();

        tp_TimeBegin = std::chrono::steady_clock::now();
        mpz_set_ui(mTmp, uliNum1Tmp);
        mpz_mul_ui(mTmp, mTmp, uliNum2Tmp);
        tp_TimeEnd = std::chrono::steady_clock::now();

        dArrTime[iIter] = (std::chrono::duration_cast<std::chrono::duration<double>>(tp_TimeEnd - tp_TimeBegin)).count();
        dTimeSum += dArrTime[iIter];
        
        if (DEF_DETAILED)
        {
            std::cout << "\t[+]: " << uliNum1Tmp  << " * " << uliNum2Tmp << " = ";
            mpz_out_str(stdout, 10, mTmp);
            std::cout << " (time: " << dArrTime[iIter] << " s.)" << std::endl;
        }
    }

    dTimeSum = (dTimeSum / (double)ct_iArrSize);
    std::cout << "[AVG time]: " << dTimeSum << " s. ";
    for (int iIter = 0; iIter < ct_iArrSize; iIter++)
    {
        if (iIter == 0)
            std::cout << "(";
        std::cout << dArrTime[iIter];

        if (iIter != ct_iArrSize - 1)
            std::cout << ", ";
        else
            std::cout << ")" << std::endl << std::endl;
    }

    // DIV block.
    mpf_t mpfTmp1;
    mpf_init(mpfTmp1);

    std::cout << "Checking 'div' functions..." << std::endl;
    for (int iIter = 0; iIter < ct_iArrSize; iIter++)
    {
        uliNum1Tmp = glob_mtGenerator();
        uliNum2Tmp = glob_mtGenerator();

        tp_TimeBegin = std::chrono::steady_clock::now();
        
        mpf_set_ui(mpfTmp1, uliNum1Tmp);
        mpf_div_ui(mpfTmp1, mpfTmp1, uliNum2Tmp);

        tp_TimeEnd = std::chrono::steady_clock::now();

        dArrTime[iIter] = (std::chrono::duration_cast<std::chrono::duration<double>>(tp_TimeEnd - tp_TimeBegin)).count();
        dTimeSum += dArrTime[iIter];
        
        if (DEF_DETAILED)
        {
            std::cout << "\t[+]: " << uliNum1Tmp  << " / " << uliNum2Tmp << " = ";
            mpf_out_str(stdout, 10, 10, mpfTmp1);
            std::cout << " (time: " << dArrTime[iIter] << " s.)" << std::endl;
        }
    }

    dTimeSum = (dTimeSum / (double)ct_iArrSize);
    std::cout << "[AVG time]: " << dTimeSum << " s. ";
    for (int iIter = 0; iIter < ct_iArrSize; iIter++)
    {
        if (iIter == 0)
            std::cout << "(";
        std::cout << dArrTime[iIter];

        if (iIter != ct_iArrSize - 1)
            std::cout << ", ";
        else
            std::cout << ")" << std::endl << std::endl;
    }
    mpf_clear(mpfTmp1);


    mpz_clear(mTmp);
}