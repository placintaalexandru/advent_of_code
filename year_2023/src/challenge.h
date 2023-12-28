#include <vector>
#include <fstream>

class Challenge {
    public:
        virtual void part_one() = 0;
        virtual void part_two() = 0;
        static std::vector<std::string> read_input(std::string file_name);
};

std::vector<std::string> Challenge::read_input(std::string file_name) {
    std::ifstream input(file_name);
    std::vector<std::string> lines;

    for (std::string line; getline(input, line); ) {
        lines.push_back(line);
    }

    return lines;
}