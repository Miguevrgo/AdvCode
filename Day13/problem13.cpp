#include <vector>
#include <iostream>
#include <fstream>
#include <string>
#include <algorithm>
#include <cmath>


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
    blocks.emplace_back(block);

    return blocks;
}

int find_mirror(const vector<string>& grid) {
    for (int r = 1; r < grid.size(); ++r) {
        vector<string> above(grid.begin(), grid.begin() + r);
        reverse(above.begin(), above.end());
        vector<string> below(grid.begin() + r, grid.end());

        above.resize(min(above.size(), below.size()));
        below.resize(min(above.size(), below.size()));

        if (above == below) {
            return r;
        }
    }
    return 0;
}

long part1(const vector<vector<string> > &blocks) {
    int total = 0;
    for (auto& block : blocks) {
        int row = find_mirror(block);
        total += row * 100;

        vector<string> transposed_block(block[0].size(), string(block.size(), ' '));
        for (int i = 0; i < block.size(); ++i) {
            for (int j = 0; j < block[i].size(); ++j) {
                transposed_block[j][i] = block[i][j];
            }
        }

        int col = find_mirror(transposed_block);
        total += col;
    }
    return total;
}

int find_mirror_part2(const vector<string>& grid) {
    for (int r = 1; r < grid.size(); ++r) {
        vector<string> above(grid.begin(), grid.begin() + r);
        reverse(above.begin(), above.end());
        vector<string> below(grid.begin() + r, grid.end());

        above.resize(min(above.size(), below.size()));
        below.resize(min(above.size(), below.size()));

        int diff_count = 0;
        for (int i = 0; i < min(above.size(), below.size()); ++i) {
            for (int j = 0; j < above[i].size(); ++j) {
                if (above[i][j] != below[i][j]) {
                    ++diff_count;
                }
            }
        }

        if (diff_count == 1) {
            return r;
        }
    }
    return 0;
}

long part2(const vector<vector<string> > &blocks) {
    int total = 0;
    for (auto& block : blocks) {
        int row = find_mirror_part2(block);
        total += row * 100;

        vector<string> transposed_block(block[0].size(), string(block.size(), ' '));
        for (int i = 0; i < block.size(); ++i) {
            for (int j = 0; j < block[i].size(); ++j) {
                transposed_block[j][i] = block[i][j];
            }
        }

        int col = find_mirror_part2(transposed_block);
        total += col;
    }
    return total;
}

int main() {
    string filename = "input.txt";
    vector<string> lines = processInput(filename);
    vector<vector<string> > blocks = processBlocks(lines);

    cout << part1(blocks) << endl;
    cout << part2(blocks) << endl;
    return 0;
}