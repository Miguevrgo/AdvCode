#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
#include <map>
#include <algorithm>

using namespace std;

vector<pair<string, int>> process_input(const string &filename) {
    ifstream file(filename);
    vector<pair<string, int>> cards;
    string line;
    while (getline(file, line)) {
        istringstream iss(line);
        string token;
        int value;
        iss >> token >> value;
        cards.emplace_back(token, value);
    }
    return cards;
}

string hand_type(map<string, int>& cards_in_hand, bool isPart2) {
    int jokers = cards_in_hand["J"];
    if (isPart2) {
        if (jokers >= 1 && jokers <= 4) {
            cards_in_hand.erase("J");
        }
    }

    vector<pair<string, int>> sorted_hand(cards_in_hand.begin(), cards_in_hand.end());

    sort(sorted_hand.begin(), sorted_hand.end(), [](const pair<string, int>& a, const pair<string, int>& b) {
        return a.second > b.second;
    });

    if (isPart2) {
        if (jokers >= 1 && jokers <= 4) {
            sorted_hand[0].second += jokers;
        }
    }

    int most_cards = sorted_hand[0].second;
    int next_most = sorted_hand[1].second;
    string type;
    switch (most_cards) {
        case 5:
            type = "5K";
            break;
        case 4:
            type = "4K";
            break;
        case 3:
            if (next_most == 2) {
                type = "4H";
            } else {
                type = "3K";
            }
            break;
        case 2:
            if (next_most == 2) {
                type = "2P";
            } else {
                type = "1P";
            }
            break;
        default:
            type = "0H";
    }
    return type;
}

string hand_key(string hand, const string &type, bool isPart2) {
    replace(hand.begin(), hand.end(), 'A', 'Z');
    replace(hand.begin(), hand.end(), 'K', 'Y');
    replace(hand.begin(), hand.end(), 'Q', 'X');
    replace(hand.begin(), hand.end(), 'J', isPart2 ? '0' : 'W');
    return type + " " + hand;
}

long score(const vector<pair<string, pair<string, int>>>& ranked) {
    long winnings = 0;
    int nbr_hands = ranked.size();
    for (int h = 0; h < nbr_hands; h++) {
        int score = (nbr_hands - h) * ranked[h].second.second;
        winnings += score;
    }
    return winnings;
}

vector<pair<string, pair<string, int>>> rank_cards(vector<pair<string, int>>& hands, bool isPart2) {
    vector<pair<string, pair<string, int>>> ranked;
    for (auto& hand : hands) {
        map<string, int> cards_in_hand;
        for (char& ch : hand.first) {
            cards_in_hand[string(1, ch)]++;
        }
        string type = hand_type(cards_in_hand, isPart2);
        string key = hand_key(hand.first, type, isPart2);
        ranked.emplace_back(key, hand);
    }
    sort(ranked.begin(), ranked.end(), greater<>());
    return ranked;
}

long part1(vector<pair<string, int>> hands) {
    vector<pair<string, pair<string, int>>> ranked = rank_cards(hands, false);
    return score(ranked);
}

long part2(vector<pair<string, int>> hands) {
    vector<pair<string, pair<string, int>>> ranked = rank_cards(hands, true);
    return score(ranked);
}

int main() {
    string filename = "input.txt";
    vector<pair<string, int>> hands = process_input(filename);
    cout << "Part 1: " << part1(hands) << endl;
    cout << "Part 2: " << part2(hands) << endl;
    return 0;
}