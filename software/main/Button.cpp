#include "Button.hpp"
#include "button.h"
#include <esp_err.h>
#include <esp_log.h>

const char *Button::TAG = "Button";

static void button_dispatch(button_t *b, button_state_t s) {
    reinterpret_cast<Button *>(b->ctx)->dispatch(s);
}

void Button::dispatch(button_state_t s) {
    ESP_LOGI(Button::TAG, "Dispatch Button %d state %d\n", button_data.identifier, s);

    switch (s) {
    case BUTTON_RELEASED:
        button_data.state = ICAN::BUTTON_EVENT_t::RELEASED;
        can.send(ICAN::MSG_ID_t::BUTTON_EVENT, data, 3, false);
        break;
    case BUTTON_PRESSED:
        button_data.state = ICAN::BUTTON_EVENT_t::PRESSED;
        can.send(ICAN::MSG_ID_t::BUTTON_EVENT, data, 3, false);
        break;
    case BUTTON_PRESSED_LONG:
        button_data.state = ICAN::BUTTON_EVENT_t::PRESSED;
        can.send(ICAN::MSG_ID_t::BUTTON_EVENT, data, 3, false);
        break;
    default:
        break;
    }
}

Button::Button(ICAN &ic) : can(ic) {}

void Button::init(gpio_num_t gpio, button_id_t id) {
    if (gpio == GPIO_NUM_NC) {
        return;
    }

    button.gpio = gpio;
    button.pressed_level = 0;
    button.internal_pull = true;
    button.autorepeat = true;
    button.callback = button_dispatch;
    button.ctx = this;
    button_data.identifier = static_cast<uint8_t>(id);
    button_data.state = ICAN::BUTTON_EVENT_t::RELEASED;

    ESP_ERROR_CHECK(button_init(&button));
}
