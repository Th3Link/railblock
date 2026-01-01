#pragma once

#include "ICAN.hpp"
#include "PinConfig.hpp"
#include <array>
#include <driver/gpio.h>
#include <message/Message.hpp>
#include <message/Queue.hpp>
#include <message/Receiver.hpp>

class Relais : public ICANDispatcher {
  public:
    Relais(ICAN &, PinConfig::signal_config_t);
    void init();
    void state(uint8_t p_num, bool p_state);
    bool state(uint8_t p_num);
    bool dispatch(uint32_t identifier, uint8_t *data, unsigned int data_len, bool request) override;
    static const char *TAG;
    static constexpr uint8_t MAX_RELAIS_COUNT = 1;

  private:
    ICAN &m_can;
    PinConfig::signal_config_t m_signals;
    uint16_t m_state;
};
