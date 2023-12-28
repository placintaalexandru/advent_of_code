#include <iostream>
#include <sstream>
#include <math.h>
#include <unordered_map>
#include "challenge.h"

enum Color {
    Red = 0,
    Green = 1,
    Blue = 2
};

struct ColorHash {
    std::size_t operator()(Color c) const {
        return static_cast<std::size_t>(c);
    }
};

Color string_to_color(std::string s) {
    if (s == "red") {
        return Red;
    } else if (s == "blue") {
        return Blue;
    } else if (s == "green") {
        return Green;
    }

    exit(1);
}

typedef std::unordered_map<Color, uint, ColorHash> Round;
typedef Round Inventory;

class Game {
    public:
        Game(std::string input);

        std::vector<Round> get_rounds() {
            return this->rounds;
        }

        uint get_id() {
            return this->id;
        }

        Inventory minimal_requirements();
    private:
        uint id;
        std::vector<Round> rounds;
};

Game::Game(std::string input) {
    std::stringstream ss(input), ss_rounds, ss_game;
    std::string game_part, rounds_part, round_token, game_token;
    std::vector<Round> rounds;
    uint game_id;

    std::getline(ss, game_part, ':');
    std::getline(ss, rounds_part, ':');
    
    ss_game = std::stringstream(game_part);
    ss_rounds = std::stringstream(rounds_part);

    ss_game >> game_token >> game_id;

    while (std::getline(ss_rounds, round_token, ';')) {
        Round round = {
            {Red, 0},
            {Green, 0},
            {Blue, 0}
        };
        std::string balls_token;
        std::stringstream ss_round(round_token);

        while (std::getline(ss_round, balls_token, ',')) {
            std::stringstream ss_balls(balls_token);
            std::string color;
            uint count;

            ss_balls >> count >> color;

            round[string_to_color(color)] = count;
        }

        rounds.push_back(round);
    }

    this->id = game_id;
    this->rounds = rounds;
}

Inventory Game::minimal_requirements() {
    Inventory inventory = {
        {Red, 0},
        {Green, 0},
        {Blue, 0}
    };

    for (auto &round : this->rounds) {
        for (auto i : inventory) {
            inventory[i.first] = std::max(i.second, round[i.first]);
        }
    }

    return inventory;
}

class Day2: public Challenge {
    public:
        Day2(Inventory inventory);
        void part_one();
        void part_two();
    private:
        Inventory inventory;
        bool can_play(Round &round);
        bool can_play(Game &game);
};

Day2::Day2(Inventory inventory) {
    this->inventory = inventory;
}

bool Day2::can_play(Round &round) {
    return this->inventory[Red] >= round[Red] && 
    this->inventory[Green] >= round[Green] && 
    this->inventory[Blue] >= round[Blue];
}

bool Day2::can_play(Game &game) {
    for (auto &round : game.get_rounds()) {
        if (!this->can_play(round)) {
            return false;
        }
    }

    return true;
}

void Day2::part_one() {
    std::vector<std::string> lines = Day2::read_input("src/day2/input");
    uint result(0);

    for (const auto &line : lines) {
        Game game(line);

        if (this->can_play(game)) {
            result += game.get_id();
        }
    }

    std::cout << result << std::endl;
}

void Day2::part_two() {
    std::vector<std::string> lines = Day2::read_input("src/day2/input");
    uint result(0);

    for (const auto &line : lines) {
        Game game(line);
        Inventory requirements = game.minimal_requirements();
        uint power(1);

        for (auto i : requirements) {
            power *= i.second;
        }
        
        result += power;
    }

    std::cout << result << std::endl;
}

int main() {
    Day2 day({
        {Red, 12},
        {Green, 13},
        {Blue, 14}
    });

    day.part_one();
    day.part_two();
}
