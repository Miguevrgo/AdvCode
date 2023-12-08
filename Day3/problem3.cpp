#include <iostream>
#include <fstream>
#include <string>
#include <vector>

using namespace std;


vector<vector<char>> fillMatrix(ifstream& file) {
    string line;
    vector<vector<char>> matrix;
    while (getline(file, line)) {
        vector<char> row(line.begin(), line.end());
        matrix.emplace_back(row);
    }
    return matrix;
}

long calculateSum1(vector <vector<char>> matrix) {
    long sum = 0;
    string line;

    for (int symb = 1, upper = 0, lower = 2; symb < matrix.size()-1; ++symb, ++upper, ++lower) {
        for (int col = 0; col < matrix[symb].size(); ++col) {
            if (matrix[symb][col] != '.' && !isdigit(matrix[symb][col])) {
                for (int r = upper; r <= lower; ++r) {
                    for (int c = col - 1; c <= col + 1; ++c) {
                        if (isdigit(matrix[r][c])) {
                            int number = 0;
                            int startPos = c;
                            int endPos = c;
                            while (startPos >= 0 && isdigit(matrix[r][startPos])) {
                                startPos--;
                            }
                            while (endPos < matrix[r].size() && isdigit(matrix[r][endPos])) {
                                endPos++;
                            }
                            for (int p = startPos + 1; p < endPos; ++p) {
                                number = number * 10 + (matrix[r][p]-'0');
                                matrix[r][p] = '.';
                            }
                            sum += number;
                        }
                    }
                }
            }
        }
    }
    return sum;
}

long calculateSum2(vector <vector<char>> matrix) {
    long sum = 0;
    int aux = 0;
    string line;

    for (int symb = 1, upper = 0, lower = 2; symb < matrix.size()-1; ++symb, ++upper, ++lower) {
        for (int col = 0; col < matrix[symb].size(); ++col) {
            if (matrix[symb][col] == '*' && !isdigit(matrix[symb][col])) {
                bool first = true;
                for (int r = upper; r <= lower; ++r) {
                    for (int c = col - 1; c <= col + 1; ++c) {
                        if (isdigit(matrix[r][c])) {
                            int number = 0;
                            int startPos = c;
                            int endPos = c;
                            while (startPos >= 0 && isdigit(matrix[r][startPos])) {
                                startPos--;
                            }
                            while (endPos < matrix[r].size() && isdigit(matrix[r][endPos])) {
                                endPos++;
                            }
                            for (int p = startPos + 1; p < endPos; ++p) {
                                number = number * 10 + (matrix[r][p]-'0');
                                matrix[r][p] = '.';
                            }
                            first ? aux = number : sum += (number*aux);
                            first = false;
                        }
                    }
                }
            }
        }
    }
    return sum;
}

int main(int argc, char* argv[]) {
    ifstream file1(argv[1], ios::in);
    vector<vector<char>> matrix = fillMatrix(file1);

    cout << "La suma 1 es " << calculateSum1(matrix) << endl;
    cout << "La suma 2 es " << calculateSum2(matrix) << endl;

    return 0;
}