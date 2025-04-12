#include <iostream>
#include <vector>
using namespace std;

bool binary_search(vector<int> vec, int target) {
    size_t lower_bound = 0;
    size_t upper_bound = vec.size();
    size_t mid = upper_bound / 2;
    while (lower_bound < upper_bound) {
        if (target == vec[mid]) {
            return true;
        } else if (target < vec[mid]) {
            upper_bound = mid;
        } else {
            lower_bound = mid;
        }
        mid = (upper_bound + lower_bound) / 2;
    }
    return false;
};
int main() {
    vector<int> oreo = {1,2,3,4,5,6,7,8,9,10};
    cout << binary_search(oreo, -1);
}