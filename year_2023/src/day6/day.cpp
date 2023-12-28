#include <iostream>
#include <cmath>
#include <sstream>
#include "challenge.h"

typedef double Coefficient;
typedef double Solution;

class Quadratic {
    private:
        Coefficient a, b, c;
    public:
        Quadratic(Coefficient a, Coefficient b, Coefficient c);
        Coefficient get_b() const;
        std::vector<Solution> solve() const;
        uint count(std::vector<Solution> solutions, std::pair<int64_t, int64_t> interval) const;
        
};

Quadratic::Quadratic(Coefficient a, Coefficient b, Coefficient c) {
    this->a = a;
    this->b = b;
    this->c = c;
}

Coefficient Quadratic::get_b() const {
    return this->b;
}

std::vector<Solution> Quadratic::solve() const {
    double delta(std::pow(this->b, 2) - 4 * this->a * this->c);
    std::vector<Solution> result;

    if (delta < 0) {
        return result;
    }

    result.push_back(((-1) * this->b - std::sqrt(delta)) / (2 * this->a));

    if (delta == 0) {
        return result;
    }

    result.push_back(((-1) * this->b + std::sqrt(delta)) / (2 * this->a));

    return result;
}

uint Quadratic::count(std::vector<Solution> solutions, std::pair<int64_t, int64_t> interval) const {
    if (solutions.size() != 2) {
        return 0;
    }

    std::sort(solutions.begin(), solutions.end());

    if (solutions[1] < interval.first || interval.second < solutions[0]) {
        return 0;
    }

    int64_t left, right;

    left = (int64_t)fmax(std::ceill(solutions[0]), (double)interval.first);
    right = (int64_t)fmin(std::floor(solutions[1]), (double)interval.second);

    if (left == solutions[0]) {
        left++;
    }

    if (right == solutions[1]) {
        right--;
    }

    return right - left + 1;
}

class Day6: public Challenge {
    public:
        void part_one();
        void part_two();
        std::vector<Quadratic> parse(std::vector<std::string> lines);
};

std::vector<Coefficient> coefficients(std::string line) {
    std::stringstream ss(line);
    std::string part;
    Coefficient c;
    std::vector<Coefficient> result;

    std::getline(ss, part, ':');
    std::getline(ss, part);

    ss = std::stringstream(part);
    
    while (ss >> c) {
        result.push_back(c);
    }

    return result;
}

std::vector<Quadratic> Day6::parse(std::vector<std::string> lines) {
    std::pair<std::vector<Coefficient>, std::vector<Coefficient>> pairs(coefficients(lines[0]), coefficients(lines[1]));
    std::vector<Quadratic> result;

    for (uint i = 0; i < pairs.first.size(); i++) {
        result.push_back(Quadratic(-1, pairs.first[i], -pairs.second[i]));
    }

    return result;
}

void Day6::part_one() {
    std::vector<std::string> lines = Day6::read_input("src/day6/input");
    std::vector<Quadratic> equations = this->parse(lines);
    uint result(1);

    for (const auto &equation : equations) {
        std::vector<Solution> solutions = equation.solve();

        result *= equation.count(solutions, std::pair<int64_t, int64_t>(0, (int64_t)equation.get_b()));

        if (result == 0) {
            break;
        }
    }

    std::cout << result << std::endl;
}

void Day6::part_two() {
    std::vector<std::string> lines = Day6::read_input("src/day6/input");
    std::pair<std::vector<Coefficient>, std::vector<Coefficient>> pairs(coefficients(lines[0]), coefficients(lines[1]));
    std::string s1(""), s2("");
    
    for (uint i = 0; i < pairs.first.size(); i++) {
        s1 = s1 + std::to_string((int64_t)pairs.first[i]);
        s2 = s2 + std::to_string((int64_t)pairs.second[i]);
    }
    
    Quadratic equation(-1, std::stol(s1), -std::stol(s2));
    std::vector<Solution> solutions(equation.solve());

    std::cout << equation.count(solutions, std::pair<int64_t, int64_t>(0, (int64_t)equation.get_b())) << std::endl;
}

int main() {
    Day6 day;

    day.part_one();
    day.part_two();
}