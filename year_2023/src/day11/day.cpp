#include <iostream>
#include "../challenge.h"

typedef uint Axis;
typedef std::pair<Axis, Axis> Position;

class Universe {
    private:
        std::vector<std::string> lines;

    public:
        Universe(std::vector<std::string> lines);
        std::vector<Position> galaxies() const;
        std::vector<Axis> empty_rows() const;
        std::vector<Axis> empty_cols() const;
};

Universe::Universe(std::vector<std::string> lines) {
    this->lines = lines;
}

std::vector<Position> Universe::galaxies() const {
    std::vector<Position> result;

    for (uint i = 0; i < this->lines.size(); i++) {
        for (uint j = 0; j <this->lines[0].size(); j++) {
            if (this->lines[i][j] == '#') {
                result.push_back(std::make_pair(i, j));
            }
        }
    }

    return result;
}

std::vector<Axis> Universe::empty_rows() const {
    std::vector<Axis> result;

    for (uint i = 0; i < this->lines.size(); i++) {
        bool empty(true);

        for (uint j = 0; j <this->lines[0].size(); j++) {
            if (this->lines[i][j] == '#') {
                empty = false;
                break;
            }
        }

        if (empty) {
            result.push_back(i);
        }
    }

    return result;
}

std::vector<Axis> Universe::empty_cols() const {
    std::vector<Axis> result;

    for (uint j = 0; j < this->lines[0].size(); j++) {
        bool empty(true);

        for (uint i = 0; i <this->lines.size(); i++) {
            if (this->lines[i][j] == '#') {
                empty = false;
                break;
            }
        }

        if (empty) {
            result.push_back(j);
        }
    }

    return result;
}

class Day11: public Challenge {
    public:
        uint count(const std::vector<Axis>& elems, Axis low, Axis high) const;
        uint64_t part(uint expansion_factor) const;
        void part_one();
        void part_two();
};

uint Day11::count(const std::vector<Axis>& elems, Axis low, Axis high) const {
    uint result(0);

    for (const auto &e : elems) {
        if (e <= low) {
            continue;
        }

        if (e >= high) {
            break;
        }

        result++;
    }

    return result;
}

uint64_t Day11::part(uint expansion_factor) const {
    std::vector<std::string> lines = Day11::read_input("src/day11/input");
    Universe u(lines);
    std::vector<Position> galaxies = u.galaxies();
    std::vector<Axis> empty_rows(u.empty_rows()), empty_cols(u.empty_cols());
    uint64_t result(0);

    if (expansion_factor > 1) {
        expansion_factor--;
    } else {
        expansion_factor = 0;
    }

    for (uint i = 0; i < galaxies.size() - 1; i++) {
        for (uint j = i + 1; j < galaxies.size(); j++) {
            uint left_col, right_col, up_row, down_row;

            left_col = std::min(galaxies[i].second, galaxies[j].second);
            right_col = std::max(galaxies[i].second, galaxies[j].second);
            up_row = std::min(galaxies[i].first, galaxies[j].first);
            down_row = std::max(galaxies[i].first, galaxies[j].first);
            
            result += this->count(empty_cols, left_col, right_col) * expansion_factor;
            result += this->count(empty_rows, up_row, down_row) * expansion_factor;
            
            result += (right_col - left_col) + (down_row - up_row);
        }
    }

    return result;
}

void Day11::part_one() {
    std::cout << this->part(2) << std::endl;
}

void Day11::part_two() {
    std::cout << this->part(1000000) << std::endl;
}

int main() {
    Day11 day;

    day.part_one();
    day.part_two();
}