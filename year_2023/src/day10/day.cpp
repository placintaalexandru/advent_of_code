#include <iostream>
#include <queue>
#include <unordered_map>
#include <unordered_set>
#include "challenge.h"

typedef std::pair<uint, uint> Position;

enum Pixel {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    StartingPoint
};

enum Direction {
    Up,
    Down,
    Left,
    Right
};

Pixel pixel(char c) {
    if (c == 'S') {
        return StartingPoint;
    } else if (c == '|') {
        return Vertical;
    } else if (c == '-') {
        return Horizontal;
    } else if (c == 'J') {
        return NorthWest;
    } else if (c == 'L') {
        return NorthEast;
    } else if (c == '7') {
        return SouthWest;
    } else if (c == 'F') {
        return SouthEast;
    } else if (c == '.') {
        return Ground;
    }

    throw (c);
}

class Labyrinth {
    private:
        Position starting_point;
        std::vector<std::vector<Pixel>> grid;

        std::vector<std::pair<Direction, Position>> neighbors(
            const Position &positon,
            const Direction &d
        ) const;
    public:
        Labyrinth(std::vector<std::string> lines);
        std::pair<std::unordered_map<Position, Direction>, std::unordered_map<Position, uint>> bfs() const;
        Pixel get_pixel(const Position &p);
};

Labyrinth::Labyrinth(std::vector<std::string> lines) {
    std::vector<std::vector<Pixel>> grid;
    Position starting_point;

    for (uint i = 0; i < lines.size(); i++) {
        const std::string &line = lines[i];
        std::vector<Pixel> pixe_line;

        for (uint j = 0; j < line.size(); j++) {
            const Pixel p(pixel(line[j]));
            pixe_line.push_back(p);

            if (p == StartingPoint) {
                starting_point = Position(i, j);
            }
        }

        grid.push_back(pixe_line);
    }

    this->starting_point = starting_point;
    this->grid= grid;
}

std::vector<std::pair<Direction, Position>> Labyrinth::neighbors(
    const Position &positon,
    const Direction &d
) const {
    std::vector<std::pair<Direction, Position>> result;
    Pixel p(this->grid[positon.first][positon.second]);

    if (p == Vertical) {
        if (positon.first + 1 < this->grid.size() && d == Down) {
            result.push_back(
                std::pair<Direction, Position>(
                    d,
                    Position(positon.first + 1, positon.second)
                )
            );
        }

        if (positon.first > 0 && d == Up) {
            result.push_back(
                std::pair<Direction, Position>(
                    d,
                    Position(positon.first - 1, positon.second)
                )
            );
        }
    } else if (p == Horizontal) {
        if (positon.second + 1 < this->grid[0].size() && d == Right) {
            result.push_back(
                std::pair<Direction, Position>(
                    d,
                    Position(positon.first, positon.second + 1)
                )
            );
        }

        if (positon.second > 0 && d == Left) {
            result.push_back(
                std::pair<Direction, Position>(
                    d,
                    Position(positon.first, positon.second - 1)
                )
            );
        }
    } else if (p == NorthEast) {
        if (positon.second + 1 < this->grid[0].size() && d == Down) {
            result.push_back(
                std::pair<Direction, Position>(
                    Right, 
                    Position(positon.first, positon.second + 1)
                )
            );
        }

        if (positon.first > 0 && d == Left) {
            result.push_back(
                std::pair<Direction, Position>(
                    Up, 
                    Position(positon.first - 1, positon.second)
                )
            );
        }
    } else if (p == NorthWest) {
        if (positon.second > 0 && d == Down) {
            result.push_back(
                std::pair<Direction, Position>(
                    Left, 
                    Position(positon.first, positon.second - 1)
                )
            );
        }

        if (positon.first > 0 && d == Right) {
            result.push_back(
                std::pair<Direction, Position>(
                    Up, 
                    Position(positon.first - 1, positon.second)
                )
            );
        }
    } else if (p == SouthWest) {
        if (positon.second > 0 && d == Up) {
            result.push_back(
                std::pair<Direction, Position>(
                    Left, 
                    Position(positon.first, positon.second - 1)
                )
            );
        }

        if (positon.first + 1 < this->grid.size() && d == Right) {
            result.push_back(
                std::pair<Direction, Position>(
                    Down, 
                    Position(positon.first + 1, positon.second)
                )
            );
        }
    } else if(p == SouthEast) {
        if (positon.first + 1 < this->grid.size() && d == Left) {
            result.push_back(
                std::pair<Direction, Position>(
                    Down, 
                    Position(positon.first + 1, positon.second)
                )
            );
        }

        if (positon.second + 1 < this->grid[0].size() && d == Up) {
            result.push_back(
                std::pair<Direction, Position>(
                    Right, 
                    Position(positon.first, positon.second + 1)
                )
            );
        }
    } else if (p == StartingPoint) {
        if (positon.first > 0 && d == Up) {
            Pixel neighbor_pixel(this->grid[positon.first - 1][positon.second]);

            if (neighbor_pixel == Vertical || neighbor_pixel == SouthEast || SouthWest) {
                result.push_back(
                    std::pair<Direction, Position>(
                        d, 
                        Position(positon.first - 1, positon.second)
                    )
                );
            }
        }

        if (positon.first + 1 < this->grid.size() && d == Down) {
            Pixel neighbor_pixel(this->grid[positon.first + 1][positon.second]);

            if (neighbor_pixel == Vertical || neighbor_pixel == NorthEast || neighbor_pixel == NorthWest) {
                result.push_back(
                    std::pair<Direction, Position>(
                        d, 
                        Position(positon.first + 1, positon.second)
                    )
                );
            }
        }

        if (positon.second > 0 && d == Left) {
            Pixel neighbor_pixel(this->grid[positon.first][positon.second - 1]);

            if (neighbor_pixel == Horizontal || neighbor_pixel == NorthEast || neighbor_pixel == SouthEast) {
                result.push_back(
                    std::pair<Direction, Position>(
                        d, 
                        Position(positon.first, positon.second - 1)
                    )
                );
            }
        }

        if (positon.second + 1 < this->grid[0].size() && d == Right) {
            Pixel neighbor_pixel(this->grid[positon.first][positon.second + 1]);

            if (neighbor_pixel == Horizontal || neighbor_pixel == SouthWest || neighbor_pixel == NorthWest) {
                result.push_back(
                    std::pair<Direction, Position>(
                        d, 
                        Position(positon.first, positon.second + 1)
                    )
                );
            }
        }
    }

    return result;
}

template<>
struct std::hash<Position> {
    std::size_t operator()(const Position& p) const noexcept {
        return p.first * 1000000 + p.second;
    }
};

std::pair<std::unordered_map<Position, Direction>, std::unordered_map<Position, uint>> Labyrinth::bfs() const {
    std::queue<std::tuple<uint, Direction, Position>> q({
        {0, Up, this->starting_point},
        {0, Down, this->starting_point},
        {0, Left, this->starting_point},
        {0, Right, this->starting_point},
    });
    std::unordered_map<Position, uint> distances;
    std::unordered_map<Position, Direction> directions;
    uint result(0);

    while (!q.empty()) {
        auto [distance, direction, position] = q.front();
        std::vector<std::pair<Direction, Position>> neighbors;
        q.pop();        

        if (distances.find(position) == distances.end() || distance < distances[position]) {
            distances[position] = distance;
            directions[position] = direction;
        }

        neighbors = this->neighbors(position, direction);

        for (const auto &[neighbor_direction, neighbor_position] : neighbors) {
            if (distances.find(neighbor_position) != distances.end() && 
            distance + 1 >= distances[neighbor_position]) {
                continue;
            }

            q.push({distance + 1, neighbor_direction, neighbor_position});
        }
    }

    return std::pair<std::unordered_map<Position, Direction>, std::unordered_map<Position, uint>>(directions, distances);
}

Pixel Labyrinth::get_pixel(const Position &p) {
    return this->grid[p.first][p.second];
}

class Day10: public Challenge {
    public:
        void part_one();
        void part_two() {};
};

void Day10::part_one() {
    std::vector<std::string> lines = Day10::read_input("./src/day10/input");
    std::cout << lines.size() << std::endl;
    Labyrinth l(lines);
    auto [directions, distances] = l.bfs();
    uint result(0);

    for (const auto &entry : distances) {
        result = std::max(result, entry.second);
    }

    std::cout << result << std::endl;
}

int main() {
    Day10 day;

    day.part_one();
    day.part_two();
}