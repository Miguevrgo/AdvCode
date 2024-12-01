#include <fstream>
#include <iostream>
#include <string>
#include <cmath>
#include <vector>
#include <algorithm>

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

long part1(vector<string> &galaxy) {
    for (int i = 0; i < galaxy.size(); ++i){
        bool emptyLine = true;
        for (const char &c : galaxy[i]){
            if (c == '#'){
                emptyLine = false;
                break;
            }
        }
        if (emptyLine){
            galaxy.insert(galaxy.begin() + i, string(galaxy[i].size(), '.'));
            ++i;
        }
    }
    //Same in columns
    for (int i = 0; i < galaxy[0].size(); ++i){
        bool emptyColumn = true;
        for (auto & j : galaxy){
            if (j[i] == '#'){
                emptyColumn = false;
                break;
            }
        }
        if (emptyColumn){
            for (auto & j : galaxy){
                j.insert(j.begin() + i, '.');
            }
            ++i;
        }
    }

    vector<pair<long, long>> asteroids;
    long distance = 0;
    for (long i = 0; i < galaxy.size(); ++i){
        for (long j = 0; j < galaxy[i].size(); ++j) {
            if (galaxy[i][j] == '#'){
                asteroids.emplace_back(i, j);
            }
        }
    }
    for (long i = 0; i < asteroids.size(); ++i){
        for (long j = i+1; j < asteroids.size(); ++j){
            distance += abs(asteroids[i].first - asteroids[j].first) + abs(asteroids[i].second - asteroids[j].second);
        }
    }
    return distance;
}

long part2(vector<string> &galaxy) {
    vector<int> emptyLines;
    vector<int> emptyColumns;
    for (int i = 0; i < galaxy.size(); ++i){
        bool emptyLine = true;
        for (const char &c : galaxy[i]){
            if (c == '#'){
                emptyLine = false;
                break;
            }
        }
        if (emptyLine){
            emptyLines.push_back(i);
        }
    }
    //Same in columns
    for (int i = 0; i < galaxy[0].size(); ++i){
        bool emptyColumn = true;
        for (auto & j : galaxy){
            if (j[i] == '#'){
                emptyColumn = false;
                break;
            }
        }
        if (emptyColumn){
            emptyColumns.push_back(i);
        }
    }

    vector<pair<long, long>> asteroids;
    long distance = 0;
    for (long i = 0; i < galaxy.size(); ++i){
        for (long j = 0; j < galaxy[i].size(); ++j) {
            if (galaxy[i][j] == '#'){
                asteroids.emplace_back(i, j);
            }
        }
    }
    for (long i = 0; i < asteroids.size(); ++i){
        for (long j = i+1; j < asteroids.size(); ++j){
            distance += abs(asteroids[i].first - asteroids[j].first)    // Rows    between the two asteroids
                        + abs(asteroids[i].second - asteroids[j].second); // Columns between the two asteroids
            // Know the number of empty lines between the two asteroids
            int emptyLinesBetween = 0;
            for (int emptyLine : emptyLines){
                if ((emptyLine > asteroids[i].first && emptyLine < asteroids[j].first) ||
                    (emptyLine > asteroids[j].first && emptyLine < asteroids[i].first)){
                    ++emptyLinesBetween;
                }
            }
            // Know the number of empty columns between the two asteroids
            int emptyColumnsBetween = 0;
            for (int emptyColumn : emptyColumns){
                if ((emptyColumn > asteroids[i].second && emptyColumn < asteroids[j].second) ||
                    (emptyColumn > asteroids[j].second && emptyColumn < asteroids[i].second)){
                    ++emptyColumnsBetween;
                }
            }
            distance += emptyLinesBetween*999999 + emptyColumnsBetween*999999;
        }
    }
    return distance;
}


int main(){
    string filename = "input.txt";
    vector<string> galaxy = processInput(filename);
    vector<string> galaxy2 = galaxy;

    cout << "Part 1: " << part1(galaxy) << endl;
    cout << "Part 2: " << part2(galaxy2) << endl;

    return 0;
}