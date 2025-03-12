#pragma once

#include <freertos/FreeRTOS.h>
#include <freertos/semphr.h>

class Selftest
{
public:
    Selftest();
    void init();
    void start();
    void stop();
    void stop_sem();
    bool running();
    static const char* TAG;
private:
    bool m_running;
    SemaphoreHandle_t m_stop_sem;
};
