#include <vector>
#include <unordered_map>
#include <limits>
#include <iostream>
#include <fstream>
#include <sstream>
#include <list>

using namespace std;

std::vector<std::vector<std::string>> createBlocks(const std::string& filename) {
    std::ifstream file(filename);
    std::vector<std::vector<std::string>> blocks;
    std::vector<std::string> block;
    std::string line;

    while (std::getline(file, line)) {
        if (line.empty()) {
            blocks.push_back(block);
            block.clear();
        } else {
            block.push_back(line);
        }
    }

    if (!block.empty()) {
        blocks.push_back(block);
    }

    return blocks;
}

std::vector<long> splitStringToNumbers(const std::string& str, char delimiter) {
    std::vector<long> numbers;
    std::stringstream ss(str);
    std::string temp;
    while (std::getline(ss, temp, delimiter)) {
        numbers.push_back(std::stol(temp));
    }
    return numbers;
}

long part1(std::vector<std::string> lines){
    auto pos = lines[0].find(':');
    lines[0].erase(0, pos + 2);

    vector<long> v_seeds = splitStringToNumbers(lines[0], ' ');
    vector<pair<long,pair<long,bool>> > seeds;

    for (auto &seed : v_seeds){
        seeds.emplace_back(seed, make_pair(seed, false));
    }

    lines.erase(lines.begin());
    lines.erase(lines.begin());

    for (auto &line : lines){
        if (!isdigit(line[0])){
            for (auto &seed : seeds){
                seed.second.second = false;
            }
            continue;
        }

        // Leer tres numeros separados por ' '
        std::vector<long> numbers = splitStringToNumbers(line, ' ');
        long dest = numbers[0];
        long start = numbers[1];
        long length = numbers[2];
        numbers.clear();

        for (auto &seed : seeds){
            if ((seed.second.first >= start) && (seed.second.first < start + length) && (!seed.second.second)){
                seed.second.first = dest + (seed.second.first - start);
                seed.second.second = true;
            }
        }
    }

    long min_element = numeric_limits<long>::max();
    for (auto &seed : seeds){
        if (seed.second.first < min_element){
            min_element = seed.second.first;
        }
    }
    return min_element;
}

long part2(vector<vector<string> > blocks){
    vector<long> inputs = splitStringToNumbers(blocks[0][0].substr(blocks[0][0].find(':') + 2), ' ');
    blocks.erase(blocks.begin());

    list<pair<long, long> > seeds;
    for (size_t i = 0; i < inputs.size(); i += 2) {
        seeds.emplace_back(inputs[i], inputs[i] + inputs[i + 1]);
    }

    for (vector<string> &block : blocks) {
        block.erase(block.begin());

        vector<vector<long> > ranges;
        for (const string &line : block) {
            ranges.push_back(splitStringToNumbers(line, ' '));
        }

        list<pair<long, long> > new_seeds;
        while (!seeds.empty()) {
            auto [s, e] = seeds.back();
            seeds.pop_back();

            long os;
            long oe;

            for (vector<long> &range : ranges) {
                long a = range[0];
                long b = range[1];
                long c = range[2];

                os = max(s, b);
                oe = min(e, b + c);

                if (os < oe) {
                    new_seeds.emplace_back(os - b + a, oe - b + a);
                    if (os > s && !seeds.empty()) {
                        seeds.emplace_front(s, os);
                    }
                    if (e > oe && !seeds.empty()) {
                        seeds.emplace_front(oe, e);
                    }
                    break;
                }
            }
            if (os >= oe && !seeds.empty()) {
                new_seeds.emplace_back(s, e);
            }
        }
        seeds = new_seeds;
    }

    long min_element = numeric_limits<long>::max();
    for (const auto &[s, e] : seeds){
        min_element = min(min_element, s);
    }
    return min_element;
}

int main() {
    ifstream file("input.txt");
    vector<string> lines;
    string line;
    while (getline(file, line)) {
        lines.push_back(line);
    }
    cout << "Part 1: " << part1(lines) << endl
         << "Part 2: " << part2(createBlocks("input.txt")) << endl;
    return 0;
}