#include <stdio.h>
#include "bindings.h"

int x = 400;
int inc = 1;

void setup () {
    create_window(800, 800);
}

void draw () {
    rect(x, 400, 100, 50);
    if (x < 0 || x + 400 >= width()) {
        inc *= -1;
    }
    x += inc;
}

int main() {
    p_init(setup, draw);
    p_run();
    return 0;
}
