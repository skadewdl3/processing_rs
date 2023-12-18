#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void (*Callback)(void);

void p_init(Callback setup, Callback draw);

void create_window(uint32_t width, uint32_t height);

void p_run(void);

void rect(float x, float y, float width, float height);

void point(void);

void triangle(void);
