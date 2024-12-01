#include <iostream>
#include <fstream>
#include <vector>
#include <map>
#include <sstream>
#include <algorithm>

using namespace std;

map<tuple<int, int, int>, long> memoizationMap;


vector<string> processInput(const string& filename) {
    ifstream file(filename);
    vector<string> lines;
    string line;

    while (getline(file, line)) {
        lines.emplace_back(line);
    }
    return lines;
}


long calculateScore(string pattern, vector<int> blocks, int patternIndex, int blockIndex, int currentBlockLength) {
    tuple<int, int, int> key = make_tuple(patternIndex, blockIndex, currentBlockLength);
    if (memoizationMap.find(key) != memoizationMap.end()) {
        return memoizationMap[key]; // Optimization to avoid recalculating the same thing
    }
    if (patternIndex == pattern.size()) {
        if (blockIndex == blocks.size() && currentBlockLength == 0) {
            return 1;
        }
        else if (blockIndex == blocks.size() - 1 && blocks[blockIndex] == currentBlockLength) {
            return 1;
        }
        else {
            return 0;
        }
    }
    long result = 0;
    for (char c : {'.', '#'}) {
        if (pattern[patternIndex] == c || pattern[patternIndex] == '?') {
            if (c == '.' && currentBlockLength == 0) {
                result += calculateScore(pattern, blocks, patternIndex + 1, blockIndex, 0);
            }
            else if (c == '.' && currentBlockLength > 0 && blockIndex < blocks.size() && blocks[blockIndex] == currentBlockLength) {
                result += calculateScore(pattern, blocks, patternIndex + 1, blockIndex + 1, 0);
            }
            else if (c == '#') {
                result += calculateScore(pattern, blocks, patternIndex + 1, blockIndex, currentBlockLength + 1);
            }
        }
    }
    memoizationMap[key] = result;
    return result;
}

long part2(const vector<string> &lines) {
    long totalScore = 0;
    for (const string& line : lines) {
        string pattern, blocksString;
        stringstream ss(line);
        getline(ss, pattern, ' ');
        getline(ss, blocksString, ' ');

        pattern +=  "?" + pattern + "?" + pattern + "?" + pattern + "?" + pattern;
        blocksString += + "," + blocksString + "," + blocksString + "," + blocksString + "," + blocksString;
        vector<int> blocks;
        stringstream ss2(blocksString);
        string block;

        while (getline(ss2, block, ',')) {
            blocks.emplace_back(stoi(block));
        }

        memoizationMap.clear();
        long score = calculateScore(pattern, blocks, 0, 0, 0);
        totalScore += score;
    }
    return totalScore;
}

int part1(const vector<string> &lines) {

    int totalScore = 0;
    for (const string& line : lines) {
        string record; // Line of record
        string blocksString; // Sequencing of blocks
        stringstream ss(line);

        getline(ss, record, ' ');
        getline(ss, blocksString, ' ');

        vector<int> blocks; //Sequence of blocks in the record in integer form
        stringstream ss2(blocksString);
        string block;

        while (getline(ss2, block, ',')) {
            blocks.emplace_back(stoi(block));
        }

        memoizationMap.clear();
        int score = calculateScore(record, blocks, 0, 0, 0);
        totalScore += score;
    }
    return totalScore;
}


int main() {
    string filename = "input.txt";
    vector<string> lines = processInput(filename);

    cout << part1(lines) << endl;
    cout << part2(lines) << endl;
    return 0;
}