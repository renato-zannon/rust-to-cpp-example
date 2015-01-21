#include <iostream>
#include <stdint.h>

extern "C" {
  int print_it(int32_t num) {
    std::cout << num << std::endl;
  }
}
