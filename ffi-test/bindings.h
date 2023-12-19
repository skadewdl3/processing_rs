#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum PEvent {
  PMousePressed,
  None,
} PEvent;

typedef enum PMouseButton {
  Left,
  Right,
  Middle,
  None,
} PMouseButton;

typedef void (*Callback)(void);

typedef struct PEventData {
  enum PEvent event_type;
  enum PMouseButton mouse_button;
  float mouse_x;
  float mouse_y;
} PEventData;

typedef void (*PEventCallback)(struct PEventData);

typedef struct Color {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;
} Color;

void p_init(Callback setup, Callback draw);

void p_run(void);

void p_on(enum PEvent event, PEventCallback callback);

void create_window(uint32_t width, uint32_t height);

void background(struct Color color);

uint32_t width(void);

uint32_t height(void);

void set_frame_rate(uint64_t rate);

struct Color color_rgb(uint8_t r, uint8_t g, uint8_t b);

struct Color color_rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a);

struct Color color_hex(const char *code);

void stroke(struct Color color);

void fill(struct Color color);

void rect(float x, float y, float width, float height);

void point(void);

void triangle(void);
