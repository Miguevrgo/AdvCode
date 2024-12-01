#include <vector>
#include <iostream>
#include <fstream>
#include <string>

using namespace std;

vector<string> processInput(const string& filename) {
    ifstream file(filename);
    vector<string> lines;
    string line;

    while (getline(file, line)) {
        lines.emplace_back(line);
    }
    return lines;
}

long part1(const vector<string>& lines) {
    long sum = 0;
    for (const string& line : lines) {
        for (int i = 0; i < line.size(); ++i) {
            if (line[i] == 'O') {
                sum += lines.size() - i;
            }
        }
    }
    return sum;
}

int part2(const vector<string>& lines) {
    return 0;
}


int main() {
    string filename = "input.txt";
    vector<string> lines = processInput(filename);

    cout << part1(lines) << endl;
    cout << part2(lines) << endl;
    return 0;
}