#include <vector>
#include <fstream>
#include <iostream>
#include <unordered_map>
#include <numeric>
#include <string>
using namespace std;

vector<string> process_input(const string& filename) {
    ifstream file(filename);
    vector<string> lines;
    string line;

    while (getline(file, line)) {
        lines.emplace_back(line);
    }
    return lines;
}

int part1(const vector<string>& lines) {
    vector<char> instructions;
    unordered_map<string, string> map;

    // fill instructions vector
    instructions.reserve(lines[0].size());
    for (auto c : lines[0])
        instructions.emplace_back(c);

    // fill map
    for (int i = 2; i < lines.size(); ++i) {
        string key = lines[i].substr(0, 3);
        string value = lines[i].substr(6, 15);
        map[key] = value;
    }

    int steps = 0;
    const int inst_size = instructions.size();
    string curr = "AAA";
    const string goal = "ZZZ";
    for (int i = 0; curr != goal; ++i) {
        if (i == inst_size){
            i = 0;
        }
        if (curr == goal){ break;}
        string map_value = map[curr];
        curr = instructions[i] == 'R' ? map_value.substr(6, 3) : map_value.substr(1, 3);
        ++steps;
    }
    return steps;
}

long part2(const vector<string>& lines) {
    vector<char> instructions;
    vector<string> ghosts;
    vector<int> cycles;
    unordered_map<string, string> map;

    instructions.reserve(lines[0].size());
    for (auto c : lines[0]){
        instructions.emplace_back(c);
    }

    for (int i = 2; i < lines.size(); ++i) {
        if (lines[i][2] == 'A')
            ghosts.push_back(lines[i].substr(0, 3));
        map[lines[i].substr(0, 3)] = lines[i].substr(6, 15);
    }

    int inst_size = instructions.size();
    for (auto &elem : ghosts) {
        int step_count = 0;
        for (int i = 0; elem[2] != 'Z'; ++i) {
            step_count++;
            if (i == inst_size){
                i = 0;
            }
            if (instructions[i] == 'R'){
                elem = map[elem].substr(6, 3);
            }
            else{
                elem = map[elem].substr(1, 3);
            }
        }
        cycles.emplace_back(step_count);
    }

    long lcm = 1;
    for (auto &elem : cycles)
        lcm = elem / gcd(elem, lcm) * lcm;

    return lcm;
}

int main() {
    string filename = "input.txt";
    vector<string> lines = process_input(filename);
    cout << "Part 1: " << part1(lines) << endl;
    cout << "Part 2: " << part2(lines) << endl;
    return 0;
}
