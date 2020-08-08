#include <vector>
#include <cassert>
#include <iostream>

using namespace std;

template<typename T>
T dot(const vector<T> &x, const vector<T> &y) {
    assert(x.size() == y.size());
    T c = 0;
    for (int i=0; i<x.size(); i++) {
        c = c + x[i] * y[i];
    }
    return c;
}

int main() {
    vector<int> x = {1, 2, 3};
    vector<int> y = {4, 5, 6};
    int c = dot(x, y);
    cout << c << endl;
}
