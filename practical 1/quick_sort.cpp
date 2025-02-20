#include <iostream>
#include <vector>
using namespace std;

template <typename T>
unsigned int pivot(vector<T>& vec, unsigned int left, unsigned int right) {
    unsigned int pivot = left;
    for (unsigned int i = left; i < right; i++) {
        if (vec[i] < vec[pivot]) {
            swap(vec[i], vec[pivot + 1]);
            swap(vec[pivot], vec[pivot + 1]);
            pivot += 1;
        }
    }
    return pivot;
}

template <typename T>
ostream& operator<< (ostream& output, vector<T>& vec){
    output << "[";
    for (unsigned int i = 0; i < vec.size(); i++) {
        output << vec[i];
        if (i < vec.size() - 1) {
            output << ", ";
        }
    }
    output << ']';

    return output;
}

template <typename T>
void quick_sort(vector<T>& vec, unsigned int left, unsigned int right) {
    if (right <= left) {
        return;
    }
    unsigned int piv = pivot(vec, left, right);
    quick_sort(vec, left, piv);
    quick_sort(vec, piv + 1, right);
    cout << vec;
}

int main() {
    vector <int> vec = {5, 7, 1, 6, 8, 6, 11, 0};
    quick_sort(vec, 0, vec.size());
    cout << vec << endl;
}