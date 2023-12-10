#include <fstream>
#include <iostream>
#include <string>
#include <map>
#include <vector>

using namespace std;

const pair<int, int> dirUp{-1, 0};
const pair<int, int> dirDown{1, 0};
const pair<int, int> dirRight{0, 1};
const pair<int, int> dirLeft{0, -1};

const vector<pair<int, int>> dirEdges{dirUp, dirRight, dirDown, dirLeft};
const map<char, pair<pair<int, int>, pair<int, int>>> dirMap{
        {'|', {dirUp, dirDown}},
        {'L', {dirUp, dirRight}},
        {'J', {dirUp, dirLeft}},
        {'-', {dirRight, dirLeft}},
        {'F', {dirRight, dirDown}},
        {'7', {dirDown, dirLeft}}
};

vector<string> process_input(const string& filename) {
    ifstream file(filename);
    vector<string> lines;
    string line;

    while (getline(file, line)) {
        lines.emplace_back(line);
    }
    return lines;
}

void parseGraph(vector<string> &graph, const pair<int, int> &startPos) {
    vector<pair<int, int>> edges;
    for (const pair<int, int> &edge : dirEdges) {
        char pipe = graph[startPos.first + edge.first][startPos.second + edge.second];

        pair<int, int> requiredEdge{-edge.first, -edge.second};
        if (dirMap.at(pipe).first == requiredEdge || dirMap.at(pipe).second == requiredEdge) {
            edges.emplace_back(edge);
        }
    }

    pair<pair<int, int>, pair<int, int>> startNodeEdges{edges[0], edges[1]};
    for (const auto &[pipeChar, pipeEdges] : dirMap) {
        if (pipeEdges == startNodeEdges) {
            graph[startPos.first][startPos.second] = pipeChar;
        }
    }
}

int part1(const vector<string> &graph, const pair<int, int> &startPos) {
    int length = 0;
    pair<int, int> currPos = startPos;
    pair<int, int> prevPos = {-1, -1};

    while (true) {
        char pipe = graph[currPos.first][currPos.second];
        pair<pair<int, int>, pair<int, int>> edges = dirMap.at(pipe);

        if (prevPos != make_pair(currPos.first + edges.first.first,currPos.second + edges.first.second)) {
            prevPos = currPos;
            currPos.first += edges.first.first;
            currPos.second += edges.first.second;
        } else {
            prevPos = currPos;
            currPos.first += edges.second.first;
            currPos.second += edges.second.second;
        }

        ++length;

        if (currPos == startPos) {
            break;
        }
    }

    return length/2;
}

int main(){
    string filename = "input.txt";
    vector<string> graph = process_input(filename);
    pair<int, int> start;

    for (size_t i = 0; i < graph.size(); ++i){
        if (graph[i].find('S') != string::npos) {
            start = {i, graph[i].find('S')};
            break;
        }
    }

    parseGraph(graph, start);
    cout << "Part 1: " << part1(graph, start);
    return 0;
}