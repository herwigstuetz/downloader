#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Downloads `url` to the directory `tmp` and returns the path to the
 * downloaded file.
 */
char *dl_download(const char *url, const char *tmp);

/**
 * Frees `char` pointers returned by `dl_download`.
 */
void dl_free(char *s);
