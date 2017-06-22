#include <iostream>
#include <thread>
#include <vector>

int main() {
  std::vector<int> logger;

  std::vector<std::thread> handlers;
  for(int i = 0; i<10; i++) {
    handlers.push_back(std::thread([i, &logger] {
      // XXX its invalid!
      logger.push_back(i);
    }));
  }

  for(auto& h : handlers) {
    h.join();
  }

  for(auto log : logger) {
    std::cout << log << ", ";
  }
  std::cout << std::endl;

  return 0;
}
