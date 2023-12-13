#include <vector>
#include <iostream>
#include <fstream>

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

vector<vector<string> > processBlocks(const vector<string> &lines){
    vector<vector<string> > blocks;
    vector<string> block;
    for (auto &line : lines){
        if (line.empty()){
            blocks.emplace_back(block);
            block.clear();
        }
        else {
            block.emplace_back(line);
        }
    }

    return blocks;
}

long part1(const vector<vector<string> > &blocks) {
    long sum = 0;
    for (auto &block : blocks) {
        long rowsLeft = 0;
        long colsAbove = 0;

        for ( int i = 1; i < block.size(); ++i) {
            if (block[i] == block[i-1]) { // Found simmetry row
                rowsLeft = block.size() - i;
                break;
            }
        }

        // Same for simmetry col
        for ( int i = 1; i < block[0].size(); ++i) {
            bool found = true;
            for (int j = 1; j < block.size(); ++j) {
                if (block[j][i] != block[j][i-1]) {
                    found = false;
                    break;
                }
            }
            if (found) {
                colsAbove = block[0].size() - i;
                break;
            }
        }

        sum += rowsLeft + colsAbove*100;
    }
    return sum;
}

int part2(const vector<vector<string> > &blocks) {
    return 0;
}


int main() {
    string filename = "input.txt";
    vector<string> lines = processInput(filename);
    vector<vector<string> > blocks = processBlocks(lines);

    cout << part1(blocks) << endl;
    cout << part2(blocks) << endl;
    return 0;
}