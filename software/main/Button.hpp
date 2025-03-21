#pragma once

#include <cstdint>
#include "button.h"
#include "ICAN.hpp"
#include <esp_timer.h>

class Button
{
public:
    enum class button_id_t : uint8_t {
        HALL1 = 10,
        HALL2 = 11,
        HALL3 = 12,
    };
    Button(ICAN&);
    void init(gpio_num_t, button_id_t);
    void dispatch(button_state_t);
    void send_multi();
    static const char* TAG;
private:
    ICAN& can;
    button_t button;
    esp_timer_create_args_t timer_args;
    esp_timer_handle_t timer;
    struct button_data_t
    {
        uint8_t identifier;
        ICAN::BUTTON_EVENT_t state;
        uint16_t count;
    };
    #pragma pack(push,1)
    union
    {
        button_data_t button_data;
        uint8_t data[4];
    };
    #pragma pack(pop)
};
