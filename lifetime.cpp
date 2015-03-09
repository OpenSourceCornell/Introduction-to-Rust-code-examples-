#include <string>
#include <iostream>

using namespace std;

int main() {
    string *ptr;
    {
        string s;  // s is allocated here
        s = "Hello world!";
        cout << s << endl;
        ptr = &s;
        // s is destroyed here
    }
    cout << *ptr << endl;
}
