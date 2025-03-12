#pragma once

#include <cstdint>
#include <driver/gpio.h>
#include <driver/i2c.h>

class PinConfig
{
public:   
    struct can_config_t
    {
        gpio_num_t tx;
        gpio_num_t rx;
        gpio_num_t standby;
    };

	struct hall_config_t
    {
        gpio_num_t in_0;
        gpio_num_t in_1;
        gpio_num_t in_2;
    };
 
	struct signal_config_t
    {
        gpio_num_t out_0;
        gpio_num_t out_1;
        gpio_num_t out_2;
        gpio_num_t out_3;
        gpio_num_t out_4;
        gpio_num_t out_5;
        gpio_num_t out_6;
    };   
	
	struct board_config_t
    {
        can_config_t can_config;
		hall_config_t hall_config;
		signal_config_t signal_config;
    };

    PinConfig();
    void init();
    can_config_t get_can_config();
    hall_config_t get_hall_config();
    signal_config_t get_signal_config();
    static const char* TAG;
private:
    board_config_t m_board_config;
};
