#define _POSIX_C_SOURCE 200809L

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

const size_t BUFFER_SIZE = 1024;
const int64_t MAX_INSTRUCTION_LEN = 100;
const int64_t DIAL_START = 50;

typedef struct {
  // current value of dial
  int64_t dial;
  // number of times we hit zero
  uint64_t zero_clicks;
} dial_t;

void print_dial(dial_t *dial) {
  printf("Dial { dial = %ld, zero_clicks = %lu }\n", dial->dial,
         dial->zero_clicks);
}

void rotate(dial_t *dial, const char *instruction) {
  if (strnlen(instruction, MAX_INSTRUCTION_LEN) > MAX_INSTRUCTION_LEN) {
    fprintf(stderr, "bad instruction length: %s\n", instruction);
    exit(EXIT_FAILURE);
  }

  // parse instruction
  char direction = instruction[0];
  int64_t magnitude = atoi(&instruction[1]);
  printf("direction: %c | magnitude: %lu\n", direction, magnitude);

  switch (direction) {
  case 'L': {
    // if we start at zero, we already counted the click
    bool starts_at_zero = dial->dial == 0;
    int64_t difference = dial->dial - magnitude;

    // handle rotations
    if (difference == 0) {
      // ending at zero
      dial->dial = difference;
      dial->zero_clicks += 1;
    } else if (difference > 0) {
      // no rotations
      dial->dial = difference;
    } else if (difference < 0) {
      // at least one rotation
      uint16_t rotations = (labs(difference) + 100) / 100;
      uint16_t remainder = (labs(difference) + 100) % 100;
      dial->zero_clicks += rotations;
      dial->dial = (100 - remainder);
    }

    // remove already counted clicks
    if (starts_at_zero) {
      dial->zero_clicks -= 1;
    }

    break;
  }
  case 'R': {
    int64_t sum = dial->dial + magnitude;
    if (sum > 99) {
      uint16_t rotations = sum / 100;
      uint16_t remainder = sum % 100;
      dial->zero_clicks += rotations;
      dial->dial = remainder;
    } else {
      dial->dial = sum;
    }

    break;
  }
  default:
    fprintf(stderr, "bad instruction direction: %c\n", direction);
    exit(EXIT_FAILURE);
  }

  print_dial(dial);
}

int main(int argc, char **argv) {
  if (argc != 2) {
    fprintf(stderr, "Usage: %s <input_file>\n", argv[0]);
    exit(EXIT_FAILURE);
  }

  // open input
  FILE *fp = fopen(argv[1], "rb");
  if (fp == NULL) {
    fprintf(stderr, "file %s not found\n", argv[1]);
    exit(EXIT_FAILURE);
  }

  // check size of input
  struct stat st;
  if (fstat(fileno(fp), &st) != 0) {
    fprintf(stderr, "fstat failed");
    exit(EXIT_FAILURE);
  }
  size_t file_size = (size_t)st.st_size;
  if (file_size > BUFFER_SIZE) {
    fprintf(stderr, "file too big: %zu", file_size);
    exit(EXIT_FAILURE);
  }

  // read input
  char buffer[BUFFER_SIZE];
  size_t ret = fread(buffer, 1, file_size, fp);
  if (ret != file_size) {
    fprintf(stderr, "fread() failed: %zu\n", ret);
    exit(EXIT_FAILURE);
  }
  buffer[ret] = '\0';

  printf("%s\n", buffer);

  dial_t dial = {DIAL_START, 0};

  print_dial(&dial);
  rotate(&dial, "L68");
  rotate(&dial, "R98");
  rotate(&dial, "L30");

  fclose(fp);

  return EXIT_SUCCESS;
}
