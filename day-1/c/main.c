#include "sys/types.h"
#include <errno.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern int errno;

void debug(FILE *file, char *buf);
void sample(FILE *file, bool isSample);
void part2(FILE *file, bool isSample);

struct Ret {
  bool equal;
  bool isErr;
};
struct Ret mystrcmp(char *line, char *word, size_t len);
/*
 * cases
 * 1: word is compared and everything is the same.
 *    the loop goes through the whole word, every check is true, returns a
 * struct where we say there was no error and the value is true.
 *
 * 2: word comes up as false but line still has characters left
 *    the loop stops the moment there is an unequal char, and returns value as
 * false with no error
 *
 * 3: we reach the end of the line before we finish the loop
 *    we return a false value but more importantly the err is true
 *
 * Optimizations:
 * 1: Reduce the number of comparisons: only check up to an upper boundary:
 *    either till the word boundary or a certain length defined as a parameter.
 *
 */
struct Ret mystrcmp(char *line, char *word, size_t len) {
  // printf("check %s %s %zu\n", line, word, len);
  // TODO: optimize this comparison.
  for (size_t i = 0; i < len || word[i] != '\0'; i++) {

    // NOTE: we reach the end of the line without exhausting the word length,
    // make it clear so that no other larger words are checked.
    if (line[i] == '\0' || line[i] == '\n') {
      // printf("SHOULD BE DONE %zu\n", i);
      return (struct Ret){
          .equal = false,
          .isErr = true,
      };
    } else if (line[i] != word[i] || len == i) {
      // printf("NOT Q\n");
      return (struct Ret){
          .equal = false,
          .isErr = false,
      };
    }
  }
  // printf("EQUAL!\n");
  return (struct Ret){
      .equal = true,
      .isErr = false,
  };
}

void debug(FILE *file, char *buf) {
  for (size_t i = 0; fgets(buf, sizeof(buf), file) != NULL; i++) {
    size_t l = 0;
    for (; buf[l] == '\n' || buf[l] != '\0'; l++) {
      printf("C: %c, H: %02x U: %u ", buf[l], buf[l], buf[l]);
    }
    printf("\n");
    printf("%zu %zu %s", i, l, buf);
  }
}
bool ctou(char c) {
  if (48 <= c && c <= 57) {
    return true;
  } else {
    return false;
  }
}

void sample(FILE *file, bool isSample) {
  char buf[256];
  uint sol = 0;
  for (size_t i = 0; fgets(buf, sizeof(buf), file) != NULL; i++) {
    uint first = 0;
    uint last = 0;
    for (size_t l = 0; buf[l] != '\n' && buf[l] != '\0'; l++) {
      // printf("SM: %zu %c\n", l, buf[l]);
      if (ctou(buf[l])) {
        uint tmp = buf[l] - 48;
        if (first == 0) {
          first = tmp;
          last = tmp;
        } else {
          last = tmp;
        }
      }
    }
    sol += first * 10 + last;
    // printf("%u %u %s", first, last, buf);
  }
  if (isSample) {
    printf("Sample: ");
  } else {
    printf("Part 1: ");
  }
  printf("%u\n", sol);
}
void part2(FILE *file, bool isSample) {
  // NOTE: A lot of this is based on rust code i wrote to avoid the use of
  // hashtable which made it easier to port to c.
  const char *STRLIST[9] = {
      "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
  };
  const uint THREE[3] = {1, 2, 6};
  const uint FOUR[3] = {4, 5, 9};
  const uint FIVE[3] = {3, 7, 8};
  // NOTE: add an offset of 3
  const uint *words[3] = {THREE, FOUR, FIVE};

  char buf[256];
  uint sol = 0;
  for (size_t i = 0; fgets(buf, sizeof(buf), file) != NULL; i++) {
    uint first = 0;
    uint last = 0;
    // for ()
    // iterate through all the chars
    for (size_t l = 0; buf[l] != '\n'; l++) {
      if (ctou(buf[l])) {
        uint tmp = buf[l] - 48;
        if (first == 0) {
          first = tmp;
          last = tmp;
        } else {
          last = tmp;
        }
        // check for word
      } else {
        bool skip = false;
        for (size_t w = 0; w < 3; w++) {
          if (skip) {
            break;
          }
          uint len = w + 3;
          for (size_t wt = 0; wt < 3; wt++) {
            if (skip) {
              break;
            }
            uint num = words[w][wt];
            const char *word = STRLIST[num - 1];
            // NOTE: ignore warning of const char * to char *, since that would
            // restrict the number of types that can be used.
            // TODO: maybe do explicit conversion to char * here?
            struct Ret ret = mystrcmp(&buf[l], word, len);
            if (ret.isErr) {
              skip = true;
            } else if (ret.equal) {
              uint tmp = num;
              if (first == 0) {
                first = tmp;
                last = tmp;
              } else {
                last = tmp;
              }
              // NOTE: optimize by looking past the characters in the happy
              // path.
              l += len - 2;
            }
          }
        }
      }
    }
    sol += first * 10 + last;
  }
  if (isSample) {
    printf("Sample: ");
  } else {
    printf("Part 2: ");
  }
  printf("%u\n", sol);
}

int main(int argc, char *argv[]) {
  FILE *fsample = fopen("../sample.txt", "r");
  if (fsample == NULL) {
    printf("FILE DOES NOT EXIST, EXITING");
    return EXIT_FAILURE;
  }
  sample(fsample, true);
  fclose(fsample);
  FILE *fpart1 = fopen("../input.txt", "r");
  if (fsample == NULL) {
    printf("FILE DOES NOT EXIST, EXITING");
    return EXIT_FAILURE;
  }
  sample(fpart1, false);
  FILE *fsample2 = fopen("../sample_pt2.txt", "r");
  part2(fsample2, true);

  fseek(fpart1, 0L, SEEK_SET);
  part2(fpart1, false);
  // while (fgets(buf, sizeof(buf), file) != NULL) {
  //   printf("%s", buf);
  // }
  fclose(fsample2);
  fclose(fpart1);

  return EXIT_SUCCESS;
}
