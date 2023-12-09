#include <vector>
#include <fstream>
#include <sstream>
#include <iostream>

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

vector<long> splitStringToNumbers(const string& str) {
    vector<long> numbers;
    // Read numbers from string separated by spaces and store them in a vector
    istringstream iss(str);
    string number;
    while (iss >> number) {
        if (isdigit(number[0]) || (number[0] == '-' && isdigit(number[1]))){
            numbers.push_back(stol(number));
        }
    }
    return numbers;
}

// Extrapolate the sequences of numbers to find the next number in the sequence 1 for backwards, 2 for forwards
long ExtrapolateSequences( const vector<string> &lines, int part){
    long sum = 0;
    for (const string& line : lines) {
        vector<long> numbers = splitStringToNumbers(line);
        vector<vector<long>> sequences; // Vector of vectors of distances between numbers
        vector<long> distance; // Vector of distances between numbers
        bool complete = false; // Whether the sequence is complete (all distances are 0)

        sequences.emplace_back(numbers); // Add the first sequence to the vector
        int row = 0; // Row of the sequence we are looking at
        while (!complete){
            complete = true; // Assume complete until proven otherwise
            long dist; // Distance between numbers

            for (int i= 1; i < sequences[row].size(); ++i){
                dist = sequences[row][i]-sequences[row][i-1];
                distance.emplace_back(dist);
                if (dist != 0){
                    complete = false;
                }
            }

            sequences.emplace_back(distance);
            distance.clear();
            row++;
        }
        for (size_t i = sequences.size()-1; i > 0; --i){
            if (part == 1){
                sequences[i-1].emplace_back(sequences[i].back()+sequences[i-1].back());
            }
            else{
                sequences[i-1].insert(sequences[i-1].begin(), sequences[i-1].front()-sequences[i].front());
            }
        }
        if (part == 1){
            sum += sequences[0].back();
        }
        else{
            sum += sequences[0].front();
        }
        sequences.clear();
    }
    return sum;
}

long part1(const vector<string>& lines) {
    return ExtrapolateSequences(lines, 1); // Backwards
}

long part2(const vector<string>& lines) {
    return ExtrapolateSequences(lines, 2); // Forwards
}

int main() {
    string filename = "input.txt";
    vector<string> lines = process_input(filename);
    cout << "Part 1: " << part1(lines) << endl;
    cout << "Part 2: " << part2(lines) << endl;
    return 0;
}
