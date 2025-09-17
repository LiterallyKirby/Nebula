
// Modules/AutoClicker.h
#ifndef NEBULA_AUTOCLICKER_H
#define NEBULA_AUTOCLICKER_H

#include "Module.h"
#include <iostream>

class AutoClicker : public Module {
public:
    void run(Nebula* nebula) override {
        if (!enabled) return;
        // do stuff here, for example:
        std::cout << "AutoClicker tick!" << std::endl;
    }
};

#endif
