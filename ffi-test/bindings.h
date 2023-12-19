#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void (*Callback)(void);

typedef struct Color {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;
} Color;

void p_init(Callback setup, Callback draw);

void p_run(void);

void create_window(uint32_t width, uint32_t height);

void background(struct Color color);

uint32_t width(void);

uint32_t height(void);

void set_frame_rate(uint64_t rate);

struct Color color_rgb(uint8_t r, uint8_t g, uint8_t b);

struct Color color_rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a);

struct Color color_hex(const char *code);

void rect(float x, float y, float width, float height);

void point(void);

void triangle(void);
