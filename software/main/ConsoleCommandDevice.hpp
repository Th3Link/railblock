#pragma once

#include "Console.hpp"
#include "Relais.hpp"
#include "Selftest.hpp"
#include "cmd_device.h"

class ConsoleCommandDevice {
  public:
    ConsoleCommandDevice(Console &, Selftest &, Relais &);
    static ConsoleCommandDevice *console_command;
    static void selftest_start();
    static void selftest_stop();
    static void relais_state(uint8_t channel, bool value);
    Selftest &selftest();
    Relais &relais();

  private:
    Selftest &m_selftest;
    Relais &m_relais;
};
