#include <iostream>
#include <chrono>

using namespace std;

unsigned long long get_degree(unsigned long long num , unsigned long long divisor, int degree)
{
    if (num % divisor == 0)
    {
        num /= divisor;
        num = get_degree(num, divisor, degree + 1);
    }
    else cout << " ^" << degree << endl;
    
    return num;
}

int get_divisors(unsigned long long num)
{
    if (num < 4)
    {
        cout << num << " ^1" << endl;
        return 0;
    }
    if (num % 2 == 0)
    {
        num /= 2;
        cout << "2";
        num = get_degree(num, 2, 1);
    }
    if (num % 3 == 0)
    {
        num /= 3;
        cout << "3";
        num = get_degree(num, 3, 1);
    }

    unsigned long long divisor = 5;
    while (divisor * divisor < num + 1)
    {
        if (num % divisor == 0)
        {
            num /= divisor;
            cout << divisor;
            num = get_degree(num, divisor, 1);
        }
        if (num % (divisor + 2) == 0)
        {
            num /= divisor + 2;
            cout << divisor + 2;
            num = get_degree(num, divisor + 2, 1);
        }
        divisor += 6;
    }

    if (num != 1) cout << num << " ^1" << endl;

    return 0;
}

int main()
{
    cout << "This program returns divisors of a number.\nEnter the number:" << endl;

    unsigned long long num;
    cin >> num;

    cout << "----------" << endl;
    auto start = chrono::high_resolution_clock::now();
    
    get_divisors(num);

    auto end = chrono::high_resolution_clock::now();
    chrono::duration<double, milli> duration = end - start;
    cout << "----------\nFinished: "
         << duration.count() << "ms" << endl;
    
    return 0;
}
