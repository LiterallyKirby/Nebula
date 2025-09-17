
// Modules/Module.h
#ifndef NEBULA_MODULE_H
#define NEBULA_MODULE_H

class Nebula; // forward declaration

class Module {
public:
    Module() = default;
    virtual ~Module() = default;

    // called every tick
    virtual void run(Nebula* nebula) = 0;

    // optional: reset state when disabled
    virtual void reset(Nebula* nebula) {}
    
    bool enabled = false;
};

#endif
