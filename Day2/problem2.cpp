#include <iostream>
#include <string>
#include <fstream>
#include <map>

using namespace std;


int sol1(const string &file, const map<string, int>& color_limits){
    ifstream input(file, ios::in);

    int suma = 0;
    string line;
    while(getline(input, line)){
        string word;

        int number = 0;
        map<string, int> color_counter;

        int id = 0;
        int pos = line.find("Game");
        if (pos != string::npos) {
            line.erase(0, pos + 5);
        }

        //Read the number of the game
        while (isdigit(line[0])) {
            id = id * 10 + (line[0] - '0');
            line.erase(0, 1);
        }

        line.erase(0, 2);
        line.append(";");
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


int sol2(const string &file){
    //Open the file
    ifstream input(file, ios::in);

    int suma = 0;
    while(!input.eof()){
        string line;

        getline(input, line);
        string word;

        int number = 0;
        map<string,int> colour_counter;

        // Find the number of the game
        int pos = line.find(':');
        if (pos != string::npos) {
            line.erase(0, pos + 1);
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
        line.append(";");

        for(int i = 0; i < line.length(); ++i) {
        }
        suma += colour_counter["red"] * colour_counter["green"] * colour_counter["blue"];
    }
    return suma;
}

int main(int argc, char *argv[]){
    map<string, int> color_limits = {{"red", 12}, {"green", 13}, {"blue", 14}};
    int value1 = sol1(argv[1], color_limits);
    int value2 = sol2(argv[1]);

    cout << "Solución Parte 1: " << value1 << endl;
    cout << "Solución Parte 2: " << value2 << endl;

    return 0;
}