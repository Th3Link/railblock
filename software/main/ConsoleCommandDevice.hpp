#pragma once

#include "Console.hpp"
#include "cmd_device.h"
#include "Selftest.hpp"

class ConsoleCommandDevice
{
public:
    ConsoleCommandDevice(Console&, Selftest&);
    static ConsoleCommandDevice* console_command;
    static void selftest_start();
    static void selftest_stop();
    Selftest& selftest();
private:
    Selftest& m_selftest;
};
