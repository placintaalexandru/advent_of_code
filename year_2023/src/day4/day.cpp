#include <iostream>
#include <math.h>
#include <sstream>
#include <deque>
#include <unordered_set>
#include "challenge.h"

typedef std::unordered_set<uint8_t> Card;
typedef std::unordered_set<uint8_t> Choices;

class Interval {
    private:
        std::pair<uint, uint> margins;

    public:
        Interval(uint start, uint end);
        bool within_interval(uint point);
        int get_start();
        int get_end();
};

Interval::Interval(uint start, uint end) {
    this->margins = std::pair<uint, uint>(start, end);
}

bool Interval::within_interval(uint point) {
    return this->margins.first <= point && point <= this->margins.second;
}

int Interval::get_start() {
    return this->margins.first;
}

int Interval::get_end() {
    return this->margins.second;
}

class Game {
    private:
        Card card;
        Choices choices;
    public:
        Game(Card card, Choices choices);

        uint16_t score();
        uint8_t mathces();
};

Game::Game(Card card, Choices choices) {
    this->card = card;
    this->choices = choices;
}

uint8_t Game::mathces() {
    uint8_t count(0);

    for (const auto elem : this->choices) {
        count += (uint8_t)(this->card.find(elem) != this->card.end());
    }

    return count;
}

uint16_t Game::score() {
    uint8_t count(this->mathces());
    return count == 0 ? 0 : std::pow(2, count - 1);
}

class Day4: public Challenge {
    private:
        std::unordered_set<uint8_t> parse_numbers(std::string s);
    public:
        void part_one();
        void part_two();
};

std::unordered_set<uint8_t> Day4::parse_numbers(std::string s) {
    std::stringstream ss(s);
    std::unordered_set<uint8_t> result;
    uint16_t value;

    while (ss >> value) {
        result.insert(value);
    }
    
    return result;
}

void Day4::part_one() {
    std::vector<std::string> lines = Day4::read_input("src/day4/input");
    uint32_t result(0);

    for (const auto &line : lines) {
        std::stringstream ss(line);
        std::string card_part, choices_part, card_numbers_part;

        std::getline(ss, card_part, '|');
        std::getline(ss, choices_part);
        
        ss = std::stringstream(card_part);
        std::getline(ss, card_numbers_part, ':');
        std::getline(ss, card_numbers_part);
        
        Game game(Day4::parse_numbers(card_numbers_part), Day4::parse_numbers(choices_part));

        result += game.score();
    }

    std::cout << result << std::endl;
}

void Day4::part_two() {
    std::vector<std::string> lines = Day4::read_input("src/day4/input");
    uint32_t result(0);
    std::deque<std::pair<Interval, uint>> intervals;

    for (uint i = 0; i < lines.size(); i++) {
        uint cards(0);
        std::stringstream ss(lines[i]);
        std::string card_part, choices_part, card_numbers_part;

        std::getline(ss, card_part, '|');
        std::getline(ss, choices_part);

        ss = std::stringstream(card_part);
        std::getline(ss, card_numbers_part, ':');
        std::getline(ss, card_numbers_part);

        while (!intervals.empty() && intervals.front().first.get_end() < i) {
            intervals.pop_front();
        }

        cards++;

        for (auto &interval : intervals) {
            if (i < interval.first.get_start()) {
                break;
            }
            
            cards += interval.first.within_interval(i) * interval.second;
        }

        Game game(Day4::parse_numbers(card_numbers_part), Day4::parse_numbers(choices_part));
        uint16_t new_copies(game.mathces());

        intervals.push_back(std::pair<Interval, uint>(Interval(i + 1, i + new_copies), cards));
        result += cards;
    }

    std::cout << result << std::endl;
}

int main() {
    Day4 day;

    day.part_one();
    day.part_two();
}