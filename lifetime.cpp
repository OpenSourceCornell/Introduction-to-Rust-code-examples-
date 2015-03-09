#include <string>
#include <iostream>

using namespace std;

int main() {
    string *ptr;
    {
        string s;
        s = "Hello world!";
        cout << s << endl;
        ptr = &s;
    }
    cout << *ptr << endl;
}
