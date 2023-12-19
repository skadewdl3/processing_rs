#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define QUARTER_PI 0.785398163397448309615660845819875721

#define HALF_PI 1.57079632679489661923132169163975144

#define PI 3.14159265358979323846264338327950288

#define TWO_PI 6.28318530717958647692528676655900577

#define TAU TWO_PI

#define LN_2 0.693147180559945309417232121458176568

#define LN_10 2.30258509299404568401799145468436421

#define E 2.71828182845904523536028747135266250

typedef enum PEvent {
  PMousePressed,
  PMouseReleased,
  PMouseMoved,
  NoEvent,
} PEvent;

typedef enum PMouseButton {
  Left,
  Right,
  Middle,
  NoButton,
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

double mouseX(void);

double mouseY(void);

void rect(float x, float y, float width, float height);

void square(float x, float y, float side);

void point(void);

void triangle(void);

double map(double value,
           double input_range_start,
           double input_range_end,
           double output_range_start,
           double output_range_end);
