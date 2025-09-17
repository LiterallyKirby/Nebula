#include "Nebula.h"
#include <thread>
#include <atomic>
#include <unistd.h>

std::atomic<bool> g_running{true};
__attribute__((constructor)) 
int init() {
    Nebula nebula;          // create your Nebula object
    std::thread clientThread([&]() {
        nebula.runClient(); // start the client in a separate thread if you want
    });

    // optional: main thread can handle input or other logic
    while (g_running.load()) {
        usleep(1000); // 1ms tick
    }

    nebula.setRunning(false); // stop the Nebula loop
    clientThread.join();      // wait for thread to finish

    return 0;
}
