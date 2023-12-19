#include <stdio.h>
#include "bindings.h"

int x = 400;

void setup () {
    create_window(800, 800);
}

void draw () {
    rect(x, 400, 100, 50);
    x += 10;
}

int main() {
    p_init(setup, draw);
    p_run();
    return 0;
}
