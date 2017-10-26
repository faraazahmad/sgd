#include <stdlib.h>

int main()
{

  system("mv ../sgd ~/");
  system("cargo build");
  system("mv target/debug/s /usr/local/bin/");

  return 0;
}
