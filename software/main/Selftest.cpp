#include <esp_err.h>
#include <esp_log.h>
#include "Selftest.hpp"

const char* Selftest::TAG = "Selftest";

void selftest_task(void *this_ptr)
{
    auto selftest = reinterpret_cast<Selftest*>(this_ptr);
    uint32_t channel = 0;
    
    while (selftest->running())
    {
        vTaskDelay(pdMS_TO_TICKS(500));
    }
    
    selftest->stop_sem();
    vTaskDelete(NULL);
}

Selftest::Selftest() : m_running(false)
{
    
}

void Selftest::init()
{
    m_stop_sem = xSemaphoreCreateBinary();
}

void Selftest::start()
{
    stop();
    m_running = true;
    xTaskCreate(selftest_task, "selftest_task",  configMINIMAL_STACK_SIZE*4, 
        this, 5, NULL);
}

void Selftest::stop()
{
    if (m_running)
    {
        m_running = false;
        xSemaphoreTake(m_stop_sem, portMAX_DELAY);
    }
}

bool Selftest::running()
{
    return m_running;
}

void Selftest::stop_sem()
{
    xSemaphoreGive(m_stop_sem);
}
