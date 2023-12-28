#include <iostream>
#include <sstream>
#include "challenge.h"

class Dataset {
    private:
        std::vector<int64_t> dataset;
    public:
        Dataset(std::vector<int64_t> dataset);
        int64_t simulate() const;
};

Dataset::Dataset(std::vector<int64_t> dataset) {
    this->dataset = dataset;
}

int64_t Dataset::simulate() const {
    std::vector<std::vector<int64_t>> steps({
        this->dataset
    });
    int64_t result(0);

    while (true) {
        const std::vector<int64_t> step = steps[steps.size() - 1];
        std::vector<int64_t> new_step;
        bool done(true);

        for (const auto e : step) {
            if (e != 0) {
                done = false;
                break;
            }
        }

        if (done) {
            break;
        }

        for (uint i = 0; i < step.size() - 1; i++) {
            new_step.push_back(step[i + 1] - step[i]);
        }

        steps.push_back(new_step);
    }

    for (const auto &step : steps) {
        result += step[step.size() - 1];
    }

    return result;
}

class Day9: public Challenge {
    public:
        void part_one();
        void part_two();
};

void Day9::part_one() {
    std::vector<std::string> lines = Day9::read_input("src/day9/input");
    int64_t result(0);

    for (const auto &line : lines) {
        std::stringstream ss(line);
        std::vector<int64_t> dataset;
        int64_t value;

        while (ss >> value) {
            dataset.push_back(value);
        }

        result += Dataset(dataset).simulate();
    }

    std::cout << result << std::endl;
}

void Day9::part_two() {
    std::vector<std::string> lines = Day9::read_input("src/day9/input");
    int64_t result(0);

    for (const auto &line : lines) {
        std::stringstream ss(line);
        std::vector<int64_t> dataset;
        int64_t value;

        while (ss >> value) {
            dataset.push_back(value);
        }

        std::reverse(dataset.begin(), dataset.end());
        result += Dataset(dataset).simulate();
    }

    std::cout << result << std::endl;
}

int main() {
    Day9 day;

    day.part_one();
    day.part_two();
}