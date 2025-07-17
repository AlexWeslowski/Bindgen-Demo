#include <cstddef>
#include <complex>
#include <iostream>
#include <fstream>
#include <sstream>
#include <iomanip>
#include <string>
#include <thread>
#include <add.h>

using namespace std;

int main(int argc, char * argv[]) {
	long i = std::stol(argv[1]);
	long j = std::stol(argv[2]);
	std::cout << i << " + " << j << " = " << add_long(i, j) << std::endl;
	return 0;
}
