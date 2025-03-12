#include "PinConfig.hpp"
#include "ICAN.hpp"
#include <nvs_flash.h>
#include <esp_log.h>

const char* PinConfig::TAG = "PinConfig";
PinConfig::PinConfig()
{
    m_board_config.can_config.tx = GPIO_NUM_13;
    m_board_config.can_config.rx = GPIO_NUM_14;
    m_board_config.can_config.standby = GPIO_NUM_15;
	m_board_config.hall_config.in_0 = GPIO_NUM_33;
	m_board_config.hall_config.in_1 = GPIO_NUM_25;
	m_board_config.hall_config.in_2 = GPIO_NUM_26;
	m_board_config.signal_config.out_0 = GPIO_NUM_22;
	m_board_config.signal_config.out_1 = GPIO_NUM_21;
	m_board_config.signal_config.out_2 = GPIO_NUM_19;
	m_board_config.signal_config.out_3 = GPIO_NUM_18;
	m_board_config.signal_config.out_4 = GPIO_NUM_17;
	m_board_config.signal_config.out_5 = GPIO_NUM_16;
	m_board_config.signal_config.out_6 = GPIO_NUM_4;
}

void PinConfig::init()
{

}

PinConfig::can_config_t PinConfig::get_can_config()
{
    return m_board_config.can_config;
}

PinConfig::hall_config_t PinConfig::get_hall_config()
{
    return m_board_config.hall_config;
}

PinConfig::signal_config_t PinConfig::get_signal_config()
{
    return m_board_config.signal_config;
}
