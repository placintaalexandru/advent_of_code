#include <iostream>
#include <unordered_map>
#include "challenge.h"

class Interval {
    private:
        int start;
        int end;

    public:
        Interval(int start, int end);
        bool within_interval(int point);
        int get_start();
        int get_end();
};

Interval::Interval(int start, int end) {
    this->start = start;
    this->end = end;
}

bool Interval::within_interval(int point) {
    return this->start <= point && point <= end;
}

int Interval::get_start() {
    return this->start;
}

int Interval::get_end() {
    return this->end;
}

class Engine {
    private:
        std::vector<std::string> rows;
        uint build_number(int row, int col);
    public:
        Engine(std::vector<std::string> rows);
        bool is_part_number(int row, Interval i);
        std::vector<std::pair<int, int>> gear_candidates(int row, Interval i);
};

Engine::Engine(std::vector<std::string> rows) {
    this->rows = rows;
}

bool Engine::is_part_number(int row, Interval i) {
    for (int col = i.get_start() - 1; col <= i.get_end() + 1; col++) {
        if (col < 0 || col >= this->rows[0].size()) {
            continue;
        }

        for (int row_num = row - 1; row_num <= row + 1; row_num++) {
            if (row_num < 0 || row_num >= this->rows.size()) {
                continue;
            }

            if (this->rows[row_num][col] != '.' && !isdigit(this->rows[row_num][col])) {
                return true;
            }
        }
    }

    return false;
}

std::vector<std::pair<int, int>> Engine::gear_candidates(int row, Interval i) {
    std::vector<std::pair<int, int>> result;

    for (int col = i.get_start() - 1; col <= i.get_end() + 1; col++) {
        if (col < 0 || col >= this->rows[0].size()) {
            continue;
        }

        for (int row_num = row - 1; row_num <= row + 1; row_num++) {
            if (row_num < 0 || row_num >= this->rows.size()) {
                continue;
            }

            if (this->rows[row_num][col] == '*') {
                result.push_back(std::pair<int, int>(row_num, col));
            }
        }
    }

    return result;
}

class Day3: public Challenge {
    public:
        void part_one();
        void part_two();
};

void Day3::part_one() {
    std::vector<std::string> lines = Day3::read_input("src/day3/input");
    Engine engine(lines);
    uint result(0);

    for (uint i = 0; i < lines.size(); i++) {
        std::string line = lines[i];
        uint number(0);
        int start(-1), end(-1);

        for (uint j = 0; j < line.length(); j++) {
            if (isdigit(line[j])) {
                number = number * 10 + (line[j] - '0');

                if (start == -1) {
                    start = end = j;
                } else {
                    end = j;
                }
            } else {
                if (start != -1) {
                    Interval interval(start, end);
                    start = end = -1;

                    if (engine.is_part_number(i, interval)) {
                        result += number;
                    }

                    number = 0;
                }
            }
        }

        if (start != -1) {
            Interval interval(start, line.size() - 1);

            if (engine.is_part_number(i, interval)) {
                result += number;
            }
        }
    }

    std::cout << result << std::endl;
}

struct PairHash {
    std::size_t operator()(std::pair<int, int> p) const {
        return static_cast<std::size_t>(p.first + p.second);
    }
};

void Day3::part_two() {
    std::vector<std::string> lines = Day3::read_input("src/day3/input");
    Engine engine(lines);
    std::unordered_map<std::pair<int, int>, std::vector<uint>, PairHash> gear_candidates;
    uint result(0);

    for (uint i = 0; i < lines.size(); i++) {
        std::string line = lines[i];
        uint number(0);
        int start(-1), end(-1);

        for (uint j = 0; j < line.length(); j++) {
            if (isdigit(line[j])) {
                number = number * 10 + (line[j] - '0');

                if (start == -1) {
                    start = end = j;
                } else {
                    end = j;
                }
            } else {
                if (start != -1) {
                    Interval interval(start, end);
                    start = end = -1;

                    if (engine.is_part_number(i, interval)) {
                        std::vector<std::pair<int, int>> candidates = engine.gear_candidates(i, interval);

                        for (const auto &candidate : candidates) {
                            if (gear_candidates.find(candidate) == gear_candidates.end()) {
                                gear_candidates[candidate] = std::vector<uint>();
                            }

                            gear_candidates[candidate].push_back(number);
                        }
                    }

                    number = 0;
                }
            }
        }

        if (start != -1) {
            Interval interval(start, line.size() - 1);

            if (engine.is_part_number(i, interval)) {
                std::vector<std::pair<int, int>> candidates = engine.gear_candidates(i, interval);
                
                for (const auto &candidate : candidates) {
                    if (gear_candidates.find(candidate) == gear_candidates.end()) {
                        gear_candidates[candidate] = std::vector<uint>();
                    }

                    gear_candidates[candidate].push_back(number);
                }
            }
        }
    }

    for (const auto &gear_entry : gear_candidates) {
        if (gear_entry.second.size() == 2) {
            result += gear_entry.second[0] * gear_entry.second[1];
        }
    }

    std::cout << result << std::endl;
}

int main() {
    Day3 day;
    day.part_two();
}