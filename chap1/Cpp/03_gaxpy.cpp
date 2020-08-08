#include <armadillo>
#include <cassert>

using namespace arma;

void gaxpy(const mat &A, const vec &x, vec &y) {
    auto m = A.n_rows;
    auto n = A.n_cols;
    assert(n == x.size() && m == y.size());
    
    for (int i=0; i<m; i++) {
        for (int j=0; j<n; j++) {
            y[i] += A(i,j) * x[j];
        }
    }
}

int main() {
    mat A(2, 2);
    A << 1 << 2 << endr
        << 3 << 4 << endr;
    vec x = {1, 2};
    vec y = {0, 0};
    gaxpy(A, x, y);
    y.print();
}
