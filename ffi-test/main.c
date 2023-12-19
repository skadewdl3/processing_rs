#include <stdio.h>
#include "bindings.h"

int x = 400;
int inc = 10;

void setup () {
    create_window(1600, 800);
    background(color_hex("#ffff00"));
    set_frame_rate(60);
}

void draw () {
    rect(x, 400, 100, 50);
    if (x < 0 || x + 100 >= width()) {
        inc *= -1;
    }
    x += inc;
}

int main() {
    p_init(setup, draw);
    p_run();
    return 0;
}
