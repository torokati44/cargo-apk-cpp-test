#include <cstdint>
#include <vector>

extern "C" {

int32_t cpp_sub(int32_t a, int32_t b) {
    std::vector<int32_t> v;
    v.push_back(a);
    v.push_back(b);
    return v[0] - v[1];
}

}