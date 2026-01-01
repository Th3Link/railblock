#pragma once

#include "ICAN.hpp"
#include "button.h"
#include <cstdint>
#include <esp_timer.h>

class Button {
  public:
    enum class button_id_t : uint8_t {
        HALL1 = 0,
        HALL2 = 1,
        HALL3 = 2,
    };
    Button(ICAN &);
    void init(gpio_num_t, button_id_t);
    void dispatch(button_state_t);
    static const char *TAG;

  private:
    ICAN &can;
    button_t button;
    struct button_data_t {
        uint8_t identifier;
        uint8_t type;
        ICAN::BUTTON_EVENT_t state;
    };
#pragma pack(push, 1)
    union {
        button_data_t button_data;
        uint8_t data[4];
    };
#pragma pack(pop)
};
