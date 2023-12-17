#include <stdio.h>
#include "bindings.h"

void setup () {
    create_window(800, 800);
}

void draw () {

}

int main() {
    p_init(setup, draw);
    p_run();
    return 0;
}
