#include <fstream>
#include <iostream>
#include <cstdlib>
#include <ctime>

int main() {
	std::srand(std::time(0));
	std::ofstream file;
	file.open("input");
	for(long i = 0; i < 100000000; i++)
	{
		for(int j = 0; j < 7; j++)
			file << (char)(std::rand()%93+33);
		file << '\n';
	}
	file.close();
}
