#include <iostream>
#include <string>
#include <fstream>
#include <vector>  
#include <map>

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


int sol1(const vector<string> &lines, const map<string, int>& color_limits){
    int suma = 0;
    for(auto line : lines){
        string word;

        int number = 0;
        map<string, int> color_counter;

        int id = 0;
        int pos = line.find("Game");
        if (pos != string::npos) {
            line = line.substr(pos + 5);
        }

        //Read the number of the game
        while (isdigit(line[0])) {
            id = id * 10 + (line[0] - '0');
            line = line.substr(1);
        }

        line = line.substr(2);
        line += ";";
        bool possible = true;

        for(int i = 0; i < line.length() && possible; ++i) {
            color_counter.clear();
            if (isdigit(line[i])) {
                while (isdigit(line[i])) {
                    number = number * 10 + (line[i] - '0');
                    i++;
                }
            } else if (line[i] == ',' || line[i] == ' ' || line[i] == ';') {
                color_counter[word] += number;
                if (word != ""){
                    if (color_counter[word] > color_limits.at(word)) {
                        possible = false;
                    }
                }
                word = "";
                number = 0;
            } else {
                word += line[i];
            }
        }
        if (possible) {
            suma += id;
        }
    }
    return suma;
}


int sol2(const vector<string> &lines){

    int suma = 0;
    for(auto line : lines){
        string word;

        int number = 0;
        map<string,int> colour_counter;

        // Find the number of the game
        int pos = line.find(':');
        if (pos != string::npos) {
            line = line.substr(pos + 1);
        }
        int i = 0;
        if (isdigit(line[i])) {
            while (isdigit(line[i])) {
                number = number * 10 + (line[i] - '0');
                i++;
            }
        }
        else if (line[i] == ',' || line[i] == ' ' || line[i] == ';') {
            colour_counter[word] = max(colour_counter[word], number);
            word = "";
            number = 0;
        }
        else {
            word += line[i];
        }
        line += ";";

        for(int i = 0; i < line.length(); ++i) {
        }
        suma += colour_counter["red"] * colour_counter["green"] * colour_counter["blue"];
    }
    return suma;
}

int main(){
    map<string, int> color_limits = {{"red", 12}, {"green", 13}, {"blue", 14}};
    vector<string> lines = processInput("input.txt");



    cout << "Solución Parte 1: " << sol1(lines, color_limits) << endl;
    cout << "Solución Parte 2: " << sol2(lines) << endl;

    return 0;
}