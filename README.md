# RailBlock

[![Firmware License:
GPLv3](https://img.shields.io/badge/Firmware-GPLv3-blue.svg)]()
[![Hardware License: CERN-OHL-S
v2](https://img.shields.io/badge/Hardware-CERN--OHL--S%20v2-orange.svg)]()

**RailBlock** is an open hardware CAN‑bus based block detector and
signal controller for model railways.

It is designed to integrate with a **Raspberry Pi running Rocrail**,
providing reliable **track occupancy detection** and **signal control**
using a distributed CAN bus network.

The project consists of:

-   **Hardware** -- KiCad PCB design
-   **Firmware** -- written in **Embedded Rust** using **Embassy** and
    **heapless**

------------------------------------------------------------------------

# Overview

RailBlock modules are connected using a **50 kbit/s CAN bus** carried
over **RJ45 cables**.\
Each board contains:

-   an **ESP32 microcontroller**
-   a **CAN transceiver**
-   inputs for **Hall sensors**
-   outputs for **signal lights**

Multiple boards can be connected in a **daisy-chain topology**,
distributing both **power (12V)** and **CAN communication** over a
single cable.

Typical setup:

    Raspberry Pi (Rocrail)
            │
       CAN Interface
            │
      ───── CAN Bus ─────
         │       │       │
     RailBlock RailBlock RailBlock
         │       │       │
     Sensors   Signals  Sensors

------------------------------------------------------------------------

# Features

-   CAN bus communication (**50 kbit/s**)
-   RJ45 bus connectors for **daisy chaining**
-   **Power + data on one cable**
-   Up to **3 Hall sensors** per board
-   Up to **7 signal outputs**
-   **ULN2003A sinking driver**
-   **ESP32 microcontroller**
-   **Embedded Rust firmware**
-   **JTAG debugging support**
-   Designed for **Rocrail integration**

------------------------------------------------------------------------

# Hardware

The hardware design is provided as a **KiCad project**.

Each RailBlock node contains:

-   ESP32 microcontroller
-   CAN transceiver
-   ULN2003A output driver
-   connectors for Hall sensors
-   connectors for signal lights
-   two RJ45 bus connectors

The RJ45 connectors allow the bus to **pass through each board**,
simplifying wiring.

Both **power and CAN signals** are carried through the same cable.

------------------------------------------------------------------------

# RJ45 Bus Pinout

RailBlock uses standard **RJ45 connectors** to distribute **CAN bus and
12V power**.

Multiple pins are paralleled to increase current capacity.

    RJ45 Pinout (RailBlock Bus)

       +-----------------------+
       | 1  2  3  4  5  6  7  8 |
       +-----------------------+

    Pin 1  ── GND
    Pin 2  ── GND
    Pin 3  ── GND

    Pin 4  ── CANH
    Pin 5  ── CANL

    Pin 6  ── +12V
    Pin 7  ── +12V
    Pin 8  ── +12V

Notes:

-   Pins **4 and 5 form a twisted pair** in standard Ethernet cables
    (blue / blue‑white).
-   Pins **1--3 and 6--8 are bundled** to increase current carrying
    capacity.
-   This allows moderate **power distribution along the bus**.

------------------------------------------------------------------------

# Bus Topology

RailBlock nodes must be connected in a **linear CAN topology**.

    12V + CAN Bus

    [Terminator] ── RailBlock ── RailBlock ── RailBlock ── [Terminator]

Important:

-   The **first and last node must be terminated**.
-   Termination uses the standard **120 Ω CAN resistor**.

------------------------------------------------------------------------

# Inputs -- Hall Sensors

Each RailBlock board supports **up to 3 Hall sensors** for track
occupancy detection.

Recommended sensor:

-   **TLE4905L**
-   or compatible digital Hall sensors

Typical setup:

-   a small magnet is mounted under a locomotive
-   the Hall sensor detects the magnet
-   the RailBlock node sends an occupancy event

Events are transmitted over CAN using **S88 event messages**.

------------------------------------------------------------------------

# Outputs -- Signals

Each RailBlock board supports **up to 7 signal outputs**.

Outputs are implemented using a **ULN2003A Darlington driver**.

The LED anode connects to **+12V or +5V via a resistor**, while the
RailBlock output sinks current. Some signals have internal resistors.

------------------------------------------------------------------------

# Firmware

The firmware is written in **Embedded Rust**.

Technologies used:

-   **Embassy** async runtime
-   **heapless** containers (no dynamic allocation)
-   **esp-hal**

Design goals:

-   deterministic behaviour
-   low memory usage
-   robust CAN communication

------------------------------------------------------------------------

# CAN Protocol

RailBlock communicates over CAN using two protocol types.

## Märklin CS2 Accessory Protocol

Used for:

-   signal control
-   accessory commands

## S88 Event Messages

Used to report:

-   Hall sensor triggers
-   block occupancy events

------------------------------------------------------------------------

# Programming and Debugging

The ESP32 firmware can be uploaded using:

### Programming Adapter

A dedicated **programming adapter** (separate project).

### JTAG

The board also supports **JTAG debugging**, allowing:

-   breakpoints
-   single stepping
-   real runtime debugging

------------------------------------------------------------------------

# Integration with Rocrail

RailBlock is designed to work with **Rocrail** running on a **Raspberry
Pi**.

Typical architecture:

    Raspberry Pi
       (Rocrail)
           │
       CAN Interface
           │
      ───── CAN Bus ─────
         │       │       │
     RailBlock RailBlock RailBlock
         │       │       │
     Sensors   Signals  Sensors

Rocrail can:

-   receive **occupancy events**
-   control **signals and accessories**

------------------------------------------------------------------------

# Repository Structure

    railblock/
    │
    ├─ firmware/        Embedded Rust firmware
    │
    ├─ hardware/        KiCad PCB design
    │
    ├─ docs/            Documentation
    │
    └─ README.md

------------------------------------------------------------------------

# Licensing

This repository contains both **hardware designs** and **software**.

## Firmware

Licensed under:

**GNU General Public License v3 (GPLv3)**

See:

    firmware/LICENSE.txt

## Hardware

Licensed under:

**CERN Open Hardware Licence Version 2 -- Strongly Reciprocal
(CERN-OHL-S-2.0)**

See:

    hardware/LICENSE.txt

------------------------------------------------------------------------

# Status

Early development.

Hardware and firmware are evolving together.

