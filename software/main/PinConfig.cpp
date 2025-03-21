#include "PinConfig.hpp"
#include "ICAN.hpp"
#include <nvs_flash.h>
#include <esp_log.h>

const char* PinConfig::TAG = "PinConfig";
PinConfig::PinConfig()
{
    board_config.can_config.tx = GPIO_NUM_13;
    board_config.can_config.rx = GPIO_NUM_14;
    board_config.can_config.standby = GPIO_NUM_15;
	board_config.hall_config.in[0] = GPIO_NUM_33;
	board_config.hall_config.in[1] = GPIO_NUM_25;
	board_config.hall_config.in[2] = GPIO_NUM_26;
	board_config.signal_config.out[0] = GPIO_NUM_22;
	board_config.signal_config.out[1] = GPIO_NUM_21;
	board_config.signal_config.out[2] = GPIO_NUM_19;
	board_config.signal_config.out[3] = GPIO_NUM_18;
	board_config.signal_config.out[4] = GPIO_NUM_17;
	board_config.signal_config.out[5] = GPIO_NUM_16;
	board_config.signal_config.out[6] = GPIO_NUM_4;
}

void PinConfig::init()
{

}

PinConfig::can_config_t PinConfig::get_can_config()
{
    return board_config.can_config;
}
