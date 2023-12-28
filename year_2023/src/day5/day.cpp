#include <iostream>
#include <sstream>
#include "challenge.h"

class Interval {
    private:
        std::pair<uint, uint> margins;

    public:
        Interval(uint start, uint end);
        bool contains(uint point) const;
        uint get_start() const;
        uint get_end() const;
};

Interval::Interval(uint start, uint end) {
    this->margins = std::pair<uint, uint>(start, end);
}

uint Interval::get_start() const {
    return this->margins.first;
}

uint Interval::get_end() const {
    return this->margins.second;
}

bool Interval::contains(uint point) const {
    return this->margins.first <= point && point <= this->margins.second;
}

typedef std::pair<Interval, Interval> Mapping;

class Recipee {
    private:
        std::vector<std::pair<Interval, Interval>> mappings;
    public:
        Recipee(std::vector<Mapping> mappings);
        uint map(uint value) const;
};

Recipee::Recipee(std::vector<Mapping> mappings) {
    this->mappings = mappings;
}

uint Recipee::map(uint value) const {
    if (this->mappings.empty() || 
        value < this->mappings[0].first.get_start() ||
        value > this->mappings[this->mappings.size() - 1].first.get_end()
    ) {
        return value;
    }

    for (const auto &mapping : this->mappings) {
        if (mapping.first.contains(value)) {
            return mapping.second.get_start() + value - mapping.first.get_start();
        }
    }

    return value;
}

class Almanac {
    private:
        std::vector<Recipee> recipees;
 
    public:
        Almanac(std::vector<Recipee>);
        uint map(uint value) const;
};

Almanac::Almanac(std::vector<Recipee> recipees) {
    this->recipees = recipees;
}

uint Almanac::map(uint value) const {
    for (const auto &recipee : this->recipees) {
        value = recipee.map(value);
    }

    return value;
}

class Day5: public Challenge {

    public:
        void part_one();
        void part_two();
        std::pair<Almanac, std::vector<uint>> parse(std::vector<std::string> lines) const;
};

std::pair<Almanac, std::vector<uint>> Day5::parse(std::vector<std::string> lines) const {
    std::vector<uint> seeds;
    uint seed;
    std::vector<Recipee> recipees;
    std::stringstream ss_seeds(lines[0].substr(6, lines[0].size() - 6));
    std::vector<Mapping> mappings;

    while (ss_seeds >> seed) {
        seeds.push_back(seed);
    }
    
    for (uint i = 2; i < lines.size(); i++) {
        if (!lines[i].size()) {
            std::sort(mappings.begin(), mappings.end(), [](Mapping a, Mapping b) {
                return a.first.get_start() < b.first.get_start();
            });
            recipees.push_back(Recipee(mappings));
            mappings = std::vector<Mapping>();
            continue;
        }

        if (lines[i].substr(lines[i].size() - 4, 4) == "map:") {
            continue;
        }

        std::stringstream ss_line(lines[i]);
        uint src, dst, length;

        ss_line >> dst >> src >> length;
        mappings.push_back(Mapping(Interval(src, src + length - 1), Interval(dst, dst + length - 1)));
    }

    std::sort(mappings.begin(), mappings.end(), [](Mapping a, Mapping b) {
        return a.first.get_start() < b.first.get_start();
    });
    recipees.push_back(Recipee(mappings));
    return std::pair<Almanac, std::vector<uint>>(Almanac(recipees), seeds);
}

void Day5::part_one() {
    std::vector<std::string> lines = Day5::read_input("src/day5/input");
    std::pair<Almanac, std::vector<uint>> input = this->parse(lines);
    uint result(UINT_MAX);

    for (const auto &seed :input.second) {
        result = std::min(result, input.first.map(seed));
    }

    std::cout << result << std::endl;
}

void Day5::part_two() {
    std::vector<std::string> lines = Day5::read_input("src/day5/input");
    std::pair<Almanac, std::vector<uint>> input = this->parse(lines);
    uint result(UINT_MAX);

    for (uint i = 1; i < input.second.size(); i += 2) {
        for (uint seed = input.second[i - 1]; seed < input.second[i - 1] + input.second[i]; seed++) {
            result = std::min(result, input.first.map(seed));
        }
    }

    std::cout << result << std::endl;
}

int main() {
    Day5 day;

    day.part_one();
    day.part_two();
}