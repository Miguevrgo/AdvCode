#include <vector>
#include <unordered_map>
#include <limits>
#include <iostream>
#include <fstream>
#include <sstream>
#include <list>

using namespace std;


std::vector<long> splitStringToNumbers(const std::string& str) {
    std::vector<long> numbers;
    // Read numbers from string separated by spaces and store them in a vector
    std::istringstream iss(str);
    string number;
    while (iss >> number) {
        if (isdigit(number[0])){
            numbers.push_back(stol(number));
        }
    }
    return numbers;
}

long part1(vector<std::string> lines){
    auto pos = lines[0].find(':');
    lines[0].erase(0, pos + 9);

    long sum = 1;
    vector<long> v_time = splitStringToNumbers(lines[0]);
    lines.erase(lines.begin());
    lines[0].erase(0, pos + 8);
    vector<long> v_distance = splitStringToNumbers(lines[0]);
    vector<long> wins;

    for (int j = 0; j < v_time.size(); ++j){
        long counter = 0;
        for (int i = 0; i < v_time[j]; ++i){
            i*(v_time[j]-i) > v_distance[j] ? counter++ : counter = counter;
        }
        wins.push_back(counter);

    }
    for (auto win : wins){
        sum *= win;
    }
    return sum;

}

long part2(vector<std::string> lines){
    auto pos = lines[0].find(':');
    lines[0].erase(0, pos + 9);
    string aux;
    vector<long> v_time = splitStringToNumbers(lines[0]);
    lines.erase(lines.begin());
    lines[0].erase(0, pos + 8);
    vector<long> v_distance = splitStringToNumbers(lines[0]);
    vector<long> wins;

    for (auto val : v_distance){
        // Write in aux each digit together
        aux += to_string(val);
    }
    long distance_to_beat = stol(aux);
    // Same with time
    aux = "";
    for (auto val : v_time){
        aux += to_string(val);
    }
    long time_to_beat = stol(aux);
    long counter = 0;
    for (int i = 0; i < time_to_beat; ++i){
            long distance = i*(time_to_beat-i);
            distance > distance_to_beat ? counter++ : counter = counter;
    }
    return counter;

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