#include <iostream>
#include <sstream>
#include <unordered_map>
#include "challenge.h"

class Direction {
    private:
        char c;
    public:
        Direction(char c);
        uint8_t index() const;
};

Direction::Direction(char c) {
    this->c = c;
}

uint8_t Direction::index() const {
    if (this->c == 'L') {
        return 0;
    } else if (this->c == 'R') {
        return 1;
    }

    throw (this->c);
}

class Node {
    private:
        std::string name;
        std::vector<std::string> neighbors;
    public:
        Node(){}
        Node(std::string name, std::vector<std::string> neighbors);
        friend std::ostream& operator<<(std::ostream& os, const Node& n) {
            os << n.name << ":" << n.neighbors[0] << " " << n.neighbors[1];
            return os;
        }
        std::string apply_direction(const Direction &direction) const;
};

Node::Node(std::string name, std::vector<std::string> neighbors) {
    this->name = name;
    this->neighbors = neighbors;
}

std::string Node::apply_direction(const Direction &direction) const {
    return this->neighbors[direction.index()];
}

class Day8: public Challenge {
    private:
        static std::vector<Direction> directions(std::string s) {
            char c;
            std::stringstream ss(s);
            std::vector<Direction> result;

            while (ss >> c) {
                result.push_back(Direction(c));
            }

            return result;
        }

        static std::unordered_map<std::string, Node> nodes(std::vector<std::string> lines) {
            std::unordered_map<std::string, Node> result;

            for (const auto &line : lines) {
                std::stringstream ss(line);
                std::string src, n1, n2;

                ss >> src;
                ss.ignore();
                ss.ignore();
                ss.ignore();
                ss.ignore();
                ss >> n1;
                ss >> n2;

                result.emplace(src, Node(std::string(src), std::vector<std::string>({n1.substr(0, 3), n2.substr(0, 3)})));
            }

            return result;
        }

    public:
        void part_one();
        void part_two();
};

void Day8::part_one() {
    std::vector<std::string> lines = Day8::read_input("src/day8/input");
    std::vector<Direction> directions = Day8::directions(lines[0]);
    std::unordered_map<std::string, Node> nodes = Day8::nodes(std::vector<std::string>(lines.begin() + 2, lines.end()));
    uint result(0);
    std::string current("AAA");

    while (current != "ZZZ") {
        const Direction d = directions[result % directions.size()];
        current = nodes[current].apply_direction(d);
        result++;
    }

    std::cout << result << std::endl;
}

void Day8::part_two() {
    std::vector<std::string> lines = Day8::read_input("src/day8/input");
    std::vector<Direction> directions = Day8::directions(lines[0]);
    std::unordered_map<std::string, Node> nodes = Day8::nodes(std::vector<std::string>(lines.begin() + 2, lines.end()));
    uint result(0);
    std::vector<std::string> currents;

    for (const auto &entry : nodes) {
        if (entry.first[entry.first.size() - 1] == 'A') {
            currents.push_back(entry.first);
        }
    }
 
    while (true) {
        bool done(true);

        for (const auto &current : currents) {
            if (current[current.size() - 1] != 'Z') {
                done = false;
                break;
            }
        }

        if (done) {
            break;
        }

        const Direction d = directions[result % directions.size()];

        for (uint i = 0; i < currents.size(); i++) {
            currents[i] = nodes[currents[i]].apply_direction(d);
        }

        result++;
    }

    std::cout << result << std::endl;
}


int main() {
    Day8 day;

    day.part_one();
    day.part_two();
}