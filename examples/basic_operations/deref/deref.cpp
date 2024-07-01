#include <iostream>

int main() {
    int t = 10;
    int* a = &t;
    int** b = &a;

    std::cout << **b << std::endl;

    return 0;
}