#include <iostream>
#include <vector>
#include <string>

using namespace std;

int main() {
    vector<string> v;  // the first owner
    v.push_back("Hello");
    string& x = v[0];  // the second owner
    v.push_back("world");  // the first owner frees the underlying memory
    cout << x;  // the second owner uses the freed (invalid) memory
}
