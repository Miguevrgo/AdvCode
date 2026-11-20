#include <chrono>
#include <iostream>

#define main solve
#include AOC_SOURCE
#undef main

int main(int argc, char** argv) {
    const auto start = std::chrono::steady_clock::now();
#ifdef AOC_DAY_01
    const int status = solve(argc, argv);
#else
    const int status = solve();
#endif
    const auto elapsed = std::chrono::steady_clock::now() - start;
    std::cerr << std::chrono::duration_cast<std::chrono::nanoseconds>(elapsed).count() << '\n';
    return status;
}
