#include <iostream>
#include <thread>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
    int acc = 0;

    vector<thread> threads;
    // create 10 threads
    for (int i = 0; i < 10; i++) {
        threads.push_back(thread([&]() {
            for (int j = 0; j < 100000; j++) {
                acc++;  // data race happens here; acc is not synchronized
            }
        }));
    }

    for_each(threads.begin(), threads.end(), [](thread &t) {
        t.join();
    });

    cout << acc << endl;

    return 0;
}
