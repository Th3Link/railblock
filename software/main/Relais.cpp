#include "Relais.hpp"
#include <freertos/FreeRTOS.h>
#include <freertos/task.h>
#include <freertos/semphr.h>
#include <cstring>
#include <esp_err.h>
#include <esp_log.h>
#include <nvs_flash.h>

const char* Relais::TAG = "Relais";

Relais::Relais(ICAN& ic, PinConfig::signal_config_t signals) : 
      m_can(ic), m_signals(signals)
{
    m_can.add_dispatcher(this);
}

void Relais::init()
{
	for (int i = 0; i < 7; i++) {
        gpio_config_t io_conf = {};
        io_conf.pin_bit_mask = (1ULL << m_signals.out[i]);
        io_conf.mode = GPIO_MODE_OUTPUT;
        io_conf.pull_up_en = GPIO_PULLUP_DISABLE;
        io_conf.pull_down_en = GPIO_PULLDOWN_DISABLE;
        io_conf.intr_type = GPIO_INTR_DISABLE;
        gpio_config(&io_conf);
    }
}

void Relais::state(uint8_t p_bank, uint8_t p_num, bool p_state)
{
    if (p_num < 7) {
        gpio_set_level(m_signals.out[p_num], p_state ? 1 : 0);
    }
}

bool Relais::state(uint8_t p_bank, uint8_t p_num)
{
    return (m_state[p_bank] & (1 << p_num));
}

bool Relais::dispatch(uint32_t identifier, uint8_t* data, unsigned int data_len, bool request)
{
	if (static_cast<ICAN::MSG_ID_t>(identifier & 0xFF) != ICAN::MSG_ID_t::RELAIS)
	{
		return false;
	}

    union {
        ICAN::RELAIS_MSG_t relais;
        uint8_t data8[sizeof(ICAN::RELAIS_MSG_t)];
    };
    
    for(auto& d : data8)
    {
        d = 0;
    }
    for (unsigned int i = 0; i < std::min(data_len, sizeof(ICAN::RELAIS_MSG_t)); i++)
    {
        data8[i] = data[i];
    }
    
            // hack to get some data of the message queue, especially for set only commands
            if (request)
            {
                
            }
            else
            {
				state(relais.bank, relais.number, relais.state);
            }
            return true;
}
