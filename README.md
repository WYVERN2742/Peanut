# Peanut - Pinenut-01S BL602 MotherBoard

The Peanut is a motherboard for the [PINE64 Pinenut Model-01S](https://pine64.com/product/pinenut-model01s-wifi-ble5-module/) BL602 Wifi/BLE5 Module.

It aims to offer a very large suite of sensors, including Motion, Humidity, Temperature, Ambient Light, and Pressure. While also providing some LEDs and buttons for interaction, with audio output and buzzers for the classic electronic bleeps. It's entirely powered by a single LiFePO4 battery, allowing for easy charging and a compact size.

A prime design point is to also provide capability for extensions; such as solar charging, motor powering, or literally anything else you can imagine.


> [!WARNING]
> This is a hobby project and our first exploration into creating a custom PCB from scratch. Mistakes are Guarenteed!

## Roadmap

- [ ] Electronic Schematics
- [ ] Breadboard + Devkit prototyping
- [ ] Rev 1.0 Board ordering
- [ ] Refinements and Debugging
	- Continual Refinements until design works!
- [ ] Case and Extensions

## Components

> [!NOTE]
> Component list is subject to change; especially before Rev 1.0


| Type                 | Component                                                                                                                     | Price | Comments           | Connection     | In Stock for [JLCPCB](jlcpcb.com/parts)? |
| -------------------- | ----------------------------------------------------------------------------------------------------------------------------- | ----- | ------------------ | -------------- | ---------------------------------------- |
| Motion Sensor        | [BNO085](https://www.digikey.co.uk/en/products/detail/ceva-technologies-inc/BNO085/9445940)                                   | £12   | Used in slime VR's | I2C            | N                                        |
| Piezo                |                                                                                                                               |       |                    | Analog (PWM)   |                                          |
| Humidity Sensor      | [HDC2010YPAR](https://www.lcsc.com/product-detail/Temperature-and-Humidity-Sensor_Texas-Instruments-HDC2010YPAR_C477922.html) | £1    |                    | I2C            | Y                                        |
| 4-Channel DAC        | [MCP4728T-E/UN](https://www.digikey.co.uk/en/products/detail/microchip-technology/MCP4728T-E-UN/2126093)                      | £1.98 |                    | I2C -> Analog  |                                          |
| Light Sensor         | [LTR-303ALS-01](https://www.lcsc.com/product-detail/Ambient-Light-Sensors_Lite-On-LTR-308ALS-01_C492382.html)                 | £0.44 |                    | I2C            | Y                                        |
| 8-Bit I2C -> Digital | [PCF8574ADWR](https://www.digikey.co.uk/en/products/detail/texas-instruments/PCF8574ADWR/484754)                              | £1    |                    | I2C -> Digital |                                          |
| 12Bit I2C ADC        | [ADC121C021CIMM](https://www.digikey.co.uk/en/products/detail/texas-instruments/ADC121C021CIMM-NOPB/2075626)                  | £1.95 |                    | Analog -> I2C  |                                          |
| Pressure Sensor      | LWP040-PSGLC-S                                                                                                                | £0.30 |                    | Analog         |                                          |
| Push Button          |                                                                                                                               |       |                    | Digital        |                                          |
| Side Switch          |                                                                                                                               |       |                    | Digital        |                                          |


## Electronic Schematics:

- [x] Motion Sensor
- [x] Humidity Sensor
- [x] Temperature Sensor
- [x] Ambient Light Sensor
- [x] Digital Muxer
	- [x] Push Buttons
	- [x] Switch
	- [x] LEDs
- [ ] DAC
	- [ ] Audio Jack
	- [ ] Piezo
- [ ] ADC
	- [ ] Pressure Sensor
	- [ ] Battery Monitor

## Design Goals

*User goals and considerations; along with some misc ideas/wants*
- Include Battery + Interfaces for Extension (data & power)
- Small in size;- strap to wrist.
- As many general sensors as possible
- Buttons on battery-side of board if space is constrained
- Buttons should be ergonomic (if mounted length-ways, buttons facing away) 
![](doc/img/quick-sketch-buttons.png)
