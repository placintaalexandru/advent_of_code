#include <iostream>
#include <sstream>
#include <array>
#include <unordered_map>
#include "challenge.h"

enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAkind,
    FiveOfAkind
};

typedef char Card;
typedef std::array<Card, 5> Cards;

std::unordered_map<Card, uint8_t> CARD_POWERS = {
    {'2', 1},
    {'3', 2},
    {'4', 3},
    {'5', 4},
    {'6', 5},
    {'7', 6},
    {'8', 7},
    {'9', 8},
    {'T', 9},
    {'J', 10},
    {'Q', 11},
    {'K', 12},
    {'A', 13},
};

std::unordered_map<Card, uint8_t> CARD_POWERS2 = {
    {'J', 0},
    {'2', 1},
    {'3', 2},
    {'4', 3},
    {'5', 4},
    {'6', 5},
    {'7', 6},
    {'8', 7},
    {'9', 8},
    {'T', 9},
    {'Q', 11},
    {'K', 12},
    {'A', 13},
};

class Hand {
    private:
        Cards cards;
    public:
        Hand(std::array<Card, 5> cards);

        bool operator<(const Hand& other) const;

        bool operator>(const Hand& other) const;

        Type type() const;

        Type type2() const;

        Card first_non_joker() const;

        Card max_card2() const;
};

Hand::Hand(Cards cards) {
    this->cards = cards;
}

bool Hand::operator<(const Hand& other) const {
    const Type t1(this->type()), t2(other.type());
    
    if (t1 < t2) {
        return true;
    }

    if (t1 > t2) {
        return false;
    }

    for (uint8_t i = 0; i < this->cards.size(); i++) {
        if (CARD_POWERS[this->cards[i]] > CARD_POWERS[other.cards[i]]) {
            return false;
        }

        if (CARD_POWERS[this->cards[i]] < CARD_POWERS[other.cards[i]]) {
            return true;
        }
    }

    return false;
}

bool Hand::operator>(const Hand& other) const {
    const Type t1(this->type2()), t2(other.type2());

    if (t1 > t2) {
        return true;
    }

    if (t1 < t2) {
        return false;
    }

    for (uint8_t i = 0; i < this->cards.size(); i++) {
        if (CARD_POWERS2[this->cards[i]] > CARD_POWERS2[other.cards[i]]) {
            return true;
        }

        if (CARD_POWERS2[this->cards[i]] < CARD_POWERS2[other.cards[i]]) {
            return false;
        }
    }

    return false;
}

Type Hand::type() const {
    std::unordered_map<Card, uint8_t> counts;

    for (uint8_t i = 0; i < this->cards.size(); i++) {
        Card card(this->cards[i]);

        if (counts.find(card) == counts.end()) {
            counts[card] = 1;
            continue;
        }

        counts[card]++;
    }

    if (counts.size() == 1) {
        return FiveOfAkind;
    } else if (counts.size() == 2) {
        for (const auto &entry : counts) {
            if (entry.second == 4) {
                return FourOfAkind;
            }
        }

        return FullHouse;
    } else if (counts.size() == 3) {
        for (const auto &entry : counts) {
            if (entry.second == 3) {
                return ThreeOfAKind;
            }
        }

        return TwoPair;
    } else if (counts.size() == 4) {
        return OnePair;
    }

    return HighCard;
}

Card Hand::max_card2() const {
    Card result('J');

    for (const auto c : this->cards) {
        if (CARD_POWERS2[c] > CARD_POWERS2[result]) {
            result = c;
        }
    }

    return result;
}

Type Hand::type2() const {
    std::unordered_map<Card, uint8_t> counts;
    uint jokers(0);

    for (uint8_t i = 0; i < this->cards.size(); i++) {
        Card card(this->cards[i]);

        if (card == 'J') {
            jokers++;
            continue;
        }

        if (counts.find(card) == counts.end()) {
            counts[card] = 1;
            continue;
        }

        counts[card]++;
    }

    if (jokers > 0) {
        Card joker;

        if (counts.size() == 0) {
            joker = 'A';
        } else if (counts.size() == 1) {
            joker = this->first_non_joker();
        } else if (counts.size() == 2 || counts.size() == 3) {
            std::pair<Card, uint8_t> p('J', 0);

            for (const auto entry : counts) {
                if (p.second < entry.second) {
                    p.first = entry.first;
                    p.second = entry.second;
                } else if (p.second == entry.second && CARD_POWERS2[p.first] < CARD_POWERS2[p.first]) {
                    p.first = entry.first;
                }
            }
            joker = p.first;
        } else if (counts.size() == 4) {
            joker = this->max_card2();
        }

        if (counts.find(joker) == counts.end()) {
            counts[joker] = jokers;
        } else {
            counts[joker] += jokers;
        }
    }
    
    if (counts.size() == 1) {
        return FiveOfAkind;
    } else if (counts.size() == 2) {
        for (const auto &entry : counts) {
            if (entry.second == 4) {
                return FourOfAkind;
            }
        }

        return FullHouse;
    } else if (counts.size() == 3) {
        for (const auto &entry : counts) {
            if (entry.second == 3) {
                return ThreeOfAKind;
            }
        }

        return TwoPair;
    } else if (counts.size() == 4) {
        return OnePair;
    }

    return HighCard;
}

Card Hand::first_non_joker() const {
    for (const auto c : this->cards) {
        if (c != 'J') {
            return c;
        }
    }

    exit(1);
}

class Day7: public Challenge {
    public:
        void part_one();
        void part_two();
};

void Day7::part_one() {
    std::vector<std::string> lines = Day7::read_input("src/day7/input");
    std::vector<std::pair<Hand, uint>> rounds;
    uint result(0);

    for (const auto &line : lines) {
        Cards cards;
        uint bid;
        std::stringstream ss(line);

        ss >> cards.data() >> bid;
        rounds.push_back(std::make_pair(Hand(cards), bid));
    }

    std::sort(rounds.begin(), rounds.end(), [](std::pair<Hand, uint> &p1, std::pair<Hand, uint> &p2) {
        return p1.first < p2.first;
    });

    for (uint i = 0; i < rounds.size(); i++) {
        result += rounds[i].second * (i + 1);
    }

    std::cout << result << std::endl;
}

void Day7::part_two() {
    std::vector<std::string> lines = Day7::read_input("src/day7/input");
    std::vector<std::pair<Hand, uint>> rounds;
    uint result(0);

    for (const auto &line : lines) {
        Cards cards;
        uint bid;
        std::stringstream ss(line);

        ss >> cards.data() >> bid;
        rounds.push_back(std::make_pair(Hand(cards), bid));
    }

    std::sort(rounds.begin(), rounds.end(), [](std::pair<Hand, uint> &p1, std::pair<Hand, uint> &p2) {
        return p1.first > p2.first;
    });

    for (uint i = 0; i < rounds.size(); i++) {
        result += rounds[i].second * (rounds.size() - i);
    }

    std::cout << result << std::endl;
}

int main() {
    Day7 day;

    day.part_one();
    day.part_two();
}