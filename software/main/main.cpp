/*
 * CAN Lightswitch module
 */

#include <cstdio>
#include <cstdlib>
#include <cstring>

#include <esp_err.h>
#include <esp_log.h>
#include <freertos/FreeRTOS.h>
#include <freertos/semphr.h>
#include <freertos/task.h>

#include "Button.hpp"
#include "CAN.hpp"
#include "Console.hpp"
#include "ConsoleCommandDevice.hpp"
#include "Device.hpp"
#include "PinConfig.hpp"
#include "Relais.hpp"
#include "Selftest.hpp"
/* --------------------- Definitions and static variables ------------------ */

#define TAG "CANLIGHTSWITCH"

static SemaphoreHandle_t shutdown_sem;

/* --------------------------- Tasks and Functions -------------------------- */

static PinConfig pin_config;
static CAN can;
static Device device(can);
static Relais relais(can, pin_config.board_config.signal_config);
static Button hall0(can);
static Button hall1(can);
static Button hall2(can);
static Selftest selftest;
static Console console;
static ConsoleCommandDevice console_command_device(console, selftest, relais);

extern "C" void app_main() {
    // Create semaphores and tasks
    shutdown_sem = xSemaphoreCreateBinary();

    pin_config.init();
    can.init(pin_config.get_can_config(), true);
    device.init();
    relais.init();
    hall0.init(pin_config.board_config.hall_config.in[0], Button::button_id_t::HALL1);
    hall1.init(pin_config.board_config.hall_config.in[1], Button::button_id_t::HALL2);
    hall2.init(pin_config.board_config.hall_config.in[2], Button::button_id_t::HALL3);
    // init update at last; rollback will be disabled on init

    selftest.init();
    // console.init();

    xSemaphoreTake(shutdown_sem, portMAX_DELAY); // Wait for tasks to complete

    can.deinit();

    // Cleanup
    vSemaphoreDelete(shutdown_sem);
}
