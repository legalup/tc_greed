#include <cmath>
#include <functional>
#include <future>
#include <iostream>
#include <thread>

// unique function to avoid disambiguating the std::pow overload set
int f(int x, int y) { return std::pow(x, y); }


std::vector<std::future<int>> futs;

int threadFunc(int a, int b) {
  std::cout << "Thread Function :: Start" << std::endl;
  std::cout << "Thread Function :: End" << std::endl;

  std::this_thread::sleep_for(std::chrono::seconds(5));

  // Return value from thread
  return a + b;
};

struct arg_struct {
  int arg1;
  int arg2;
};

void task_lambda() {
  std::packaged_task<int(int, int)> task(
      [](int a, int b) { return std::pow(a, b); });
  std::future<int> result = task.get_future();

  task(2, 9);

  std::cout << "task_lambda:\t" << result.get() << '\n';
}

void task_bind() {
  std::packaged_task<int()> task(std::bind(f, 2, 11));
  std::future<int> result = task.get_future();

  task();

  std::cout << "task_bind:\t" << result.get() << '\n';
}

void task_thread() {
  std::packaged_task<int(int, int)> task(threadFunc);
  std::future<int> result = task.get_future();

  std::thread task_td(std::move(task), 2, 10);

  task_td.detach();
  
  //    std::cout << "task_thread:\t" << result.get() << '\n';
  futs.emplace_back(std::move(result));
}

int runner()
{
  return 0;
}

int main(int argc, char **argv) {
  task_lambda();
  task_bind();

  task_thread();
  std::cout << "just got the future" << std::endl;
  std::future_status status;
  std::chrono::seconds ones(1);
  do {
    switch (status = futs[0].wait_for(ones); status) {
    case std::future_status::deferred:
      std::cout << "deferred\n";
      break;
    case std::future_status::timeout:
      std::cout << "timeout\n";
      break;
    case std::future_status::ready:
      std::cout << "ready!\n";
      break;
    }
  } while (status != std::future_status::ready);

  std::cout << "here is my answer: " << futs[0].get() << std::endl;
  futs.clear();
}
