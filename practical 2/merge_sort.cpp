#include <iostream>
#include <vector>
using namespace std;

template <typename T>
ostream& operator <<(ostream& output, vector<T>& vec){
    output << '[';
    for (size_t i = 0; i < vec.size(); i++) {
        output << vec[i];
        output << (i < vec.size() - 1 ? ", " : "]"); 
    }
    return output;
}

template <typename T>
vector<T> merge(vector<T> left, vector<T> right) {
    vector<T> new_vec;
    size_t i = 0, j = 0; 

    while(i < left.size() && j < right.size()) {
        if (left[i] < right[j]) new_vec.push_back(left[i++]);
        else new_vec.push_back(right[j++]);
    }

    for(; i < left.size(); i++)  new_vec.push_back( left[i]);
    for(; j < right.size(); j++) new_vec.push_back(right[j]);
    return new_vec;
}

template <typename T>
vector<T> merge_sort(vector<T>& vec) {
    if (vec.size() <= 1) return vec;

    auto mid = vec.size()/2;
    auto left = vector<T>(vec.begin(), vec.begin() + mid);
    auto right = vector<T>(vec.begin() + mid,  vec.end());

    return merge(merge_sort(left), merge_sort(right));
}

int main() {
    vector<float> unsorted_vec = {3, 2, 1, 4, 2, 3 ,6, 9, 88};
    vector<float> sorted_vec = merge_sort(unsorted_vec);
    
    cout << "Before sort : " << unsorted_vec << endl 
        <<  "After sort :  " << sorted_vec << endl;
}