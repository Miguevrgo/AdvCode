#include <iostream>
#include <fstream>
#include <vector>
#include <set>
#include <sstream>
#include <algorithm>
#include <numeric>
#include <cmath>


using namespace std;

vector<int> splitStringToNumbers(const string& str, char delimiter) {
    vector<int> numbers;
    stringstream ss(str);
    string temp;

    while (getline(ss, temp, delimiter)) {
        if (temp != ""){
            numbers.push_back(stoi(temp));
        }
    }

    return numbers;
}

int part1(vector<string> lines) {
    int total = 0;
    for (string line : lines) {
        unsigned long splitPos = line.find(':');
        unsigned long splitPos2 = line.find('|', splitPos+1);
        vector<int> x = splitStringToNumbers(string(line.begin() + splitPos + 2, line.begin() + splitPos2), ' ');
        vector<int> y = splitStringToNumbers(string(line.begin() + splitPos2 + 1, line.end()), ' ');

        set<int> xSet(x.begin(), x.end()); // set of numbers before | (winning numbers)
        set<int> ySet(y.begin(), y.end()); // set of numbers after | (our numbers)

        // Find the number of matches between the two sets
        set<int> intersect;
        set_intersection(xSet.begin(), xSet.end(), ySet.begin(), ySet.end(), inserter(intersect,intersect.begin()));
        int matches = intersect.size();
        total += matches ? pow(2, matches - 1) : 0;
    }
    return total;
}

long part2(vector<string> lines) {
    vector<int> cards(lines.size(), 1);
    for (int i = 0; i < lines.size(); ++i) {
        unsigned long splitPos = lines[i].find(':');
        unsigned long splitPos2 = lines[i].find('|', splitPos+1);
        vector<int> x = splitStringToNumbers(string(lines[i].begin() + splitPos + 2, lines[i].begin() + splitPos2), ' ');
        vector<int> y = splitStringToNumbers(string(lines[i].begin() + splitPos2 + 1, lines[i].end()), ' ');

        set<int> xSet(x.begin(), x.end()); // set of numbers before | (winning numbers)
        set<int> ySet(y.begin(), y.end()); // set of numbers after | (our numbers)

        // Find the number of matches between the two sets
        set<int> intersect;
        set_intersection(xSet.begin(), xSet.end(), ySet.begin(), ySet.end(), inserter(intersect,intersect.begin()));
        int matches = intersect.size();

        for (int j = 0; j < matches; ++j) {
            cards[i + j +1 ] += cards[i];
        }
    }
    return accumulate(cards.begin(), cards.end(), 0);
}

int main() {
    ifstream file("input.txt");
    vector<string> lines;
    string line;
    while (getline(file, line)) {
        lines.push_back(line);
    }
    cout << "Part 1: " << part1(lines) << endl
         << "Part 2: " << part2(lines) << endl;
    return 0;
}