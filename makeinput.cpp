#include <fstream>
#include <iostream>
#include <cstdlib>
#include <ctime>

#ifndef NUM_LINES
#define NUM_LINES 100
#endif

int main() {
	std::srand(std::time(0));
	std::ofstream file;
	file.open("input");
	file << NUM_LINES << '\n';
	for(long i = 0; i < NUM_LINES; i++)
	{
		for(int j = 0; j < 7; j++)
			file << (char)(std::rand()%93+33);
		file << '\n';
	}
	file.close();
}
