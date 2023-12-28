#include <iostream>
#include <cctype>
#include "challenge.h"

class Day1: public Challenge {
    public:
        void part_one();
        void part_two();

    private:
        uint8_t from_string_to_digit(std::string s);

};

void Day1::part_one() {
    uint64_t result = 0;
    std::vector<std::string> lines = Day1::read_input("src/day1/input");

    for (const auto &line : lines) {
        uint16_t offset;
        uint64_t number = 0;

        for (auto it = line.begin(); it != line.end(); it++) {
            if (isdigit(*it)) {
                number = *it - '0';
                offset = std::distance(line.begin(), it);
                break;
            }
        }

        for (auto it = line.rbegin(); it != line.rend(); ++it) {
            if (isdigit(*it)) {
                if (std::distance(line.rend(), it) != offset) {
                    number = number * 10 + (*it - '0');
                }
                
                break;
            }
        }

        result += number;
    }

    std::cout << result << std::endl;
}

uint8_t Day1::from_string_to_digit(std::string s) {
    if (s == "one" || s == "1") {
        return 1;
    } else if (s == "two" || s == "2") {
        return 2;
    } else if (s == "three" || s == "3") {
        return 3;
    }  else if (s == "four" || s == "4") {
        return 4;
    } else if (s == "five" || s == "5") {
        return 5;
    } else if (s == "six" || s == "6") {
        return 6;
    } else if (s == "seven" || s == "7") {
        return 7;
    } else if (s == "eight" || s == "8") {
        return 8;
    } else if (s == "nine" || s == "9") {
        return 9;
    }

    std::cout << "invalid number: " << s << std::endl;
    exit(1);
 }

void Day1::part_two() {
    std::vector<std::string> numbers = { 
        "one", "1", 
        "two", "2", 
        "three", "3", 
        "four", "4", 
        "five", "5", 
        "six", "6", 
        "seven", "7", 
        "eight", "8", 
        "nine", "9"
    };
    uint64_t result = 0;
    std::vector<std::string> lines = Day1::read_input("src/day1/input");

    for (const auto &line : lines) {
        uint16_t offset;
        std::string left, right;
        size_t left_pos(line.length()), right_pos(0);

        for (const auto number : numbers) {
            size_t pos = line.find(number);

            if (pos != std::string::npos && pos <= left_pos) {
                left_pos = pos;
                left = number;
            }

            for (int i = line.length() - 1; i >= 0; i--) {
                pos = line.find(number, i);

                if (pos != std::string::npos && pos >= right_pos) {
                    right_pos = pos;
                    right = number;
                }
            }
        }

        result += from_string_to_digit(left) * 10 + from_string_to_digit(right);
    }

    std::cout << result << std::endl;
}

int main() {
    Day1 day;

    day.part_one();
    day.part_two();
}
