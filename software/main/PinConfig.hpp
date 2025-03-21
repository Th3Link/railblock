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
        gpio_num_t in[3];
    };
 
	struct signal_config_t
    {
        gpio_num_t out[7];
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
    static const char* TAG;
    board_config_t board_config;
private:
};
