#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv) {
  if (argc != 2) {
    printf("Usage: %s <input_file>, where input_file is a file containing a "
           "list of numbers, one per line.",
           argv[0]);
    return 1;
  }

  // open file for reading
  FILE *input_file = fopen(argv[1], "r");
  if (input_file == NULL) {
    printf("Could not open file %s for reading.", argv[1]);
    return 1;
  }

  // read file line by line
  // for each line, convert to int and add to sum
  char *line = NULL;
  size_t len = 0;
  ssize_t read;
  int count = 0;
  int top[3] = {0, 0, 0};

  while ((read = getline(&line, &len, input_file)) != -1) {
    if (line[0] == '\n') {
      for (int i = 2; i >= 0; i--) {
        if (top[i] <= count) {
          if (i != 0)
            for (int j = 0; j < i; j++)
              top[j] = top[j + 1];
          top[i] = count;
          break;
        }
      }
      count = 0;
    } else {
      line[read - 1] = '\0';
      int number = atoi(line);
      count += number;
    }
  }

  if (line)
    free(line);

  printf("The most carried by an elf is: %d\n", top[2]);
  printf("The top 3 elves are carying: %d, %d, %d\n", top[0], top[1], top[2]);
  printf("A total of: %d\n", top[0] + top[1] + top[2]);
}
