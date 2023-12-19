#include <stdio.h>
#include "bindings.h"

double mouse_x = 0;
double mouse_y = 0;


void setup () {
    create_window(1600, 800);
    background(color_hex("#ffff00"));
    set_frame_rate(60);
}

void draw () {
    mouse_x = mouseX();
    mouse_y = mouseY();
    rect(mouse_x - 50, mouse_y - 50, 100, 100);
}

int main() {
    p_init(setup, draw);
    p_run();
    return 0;
}
