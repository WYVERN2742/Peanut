# Peanut - Pinenut-01S BL602 MotherBoard

The Peanut is a motherboard for the [PINE64 Pinenut Model-01S](https://pine64.com/product/pinenut-model01s-wifi-ble5-module/) BL602 Wifi/BLE5 Module.

It aims to offer a very large suite of sensors, including Motion, Humidity, Temperature, Ambient Light, and Pressure. While also providing some LEDs and buttons for interaction, with audio output and buzzers for the classic electronic bleeps. It's entirely powered by a single LiFePO4 battery, allowing for easy charging and a compact size.

A prime design point is to also provide capability for extensions; such as solar charging, motor powering, or literally anything else you can imagine.


>[!warning]
>This is a hobby project and our first exploration into creating a custom PCB from scratch. Mistakes are Guarenteed!

## Roadmap

- [ ] Electronic Schematics
- [ ] Breadboard + Devkit prototyping
- [ ] Rev 1.0 Board ordering
- [ ] Refinements and Debugging
	- Continual Refinements until design works!
- [ ] Case and Extensions

## Components

> [!info]
> Component list is subject to change; especially before Rev 1.0


| Type                 | Component                                                                                                       | Price | Comments           | Connection     | In Stock for [JLCPCB](jlcpcb.com/parts)? |
| -------------------- | --------------------------------------------------------------------------------------------------------------- | ----- | ------------------ | -------------- | ---------------------------------------- |
| Motion Sensor        | [BNO085](https://www.digikey.co.uk/en/products/detail/ceva-technologies-inc/BNO085/9445940)                     | £12   | Used in slime VR's | I2C            | N                                        |
| Piezo                |                                                                                                                 |       |                    | Analog (PWM)   |                                          |
| Humidity Sensor      | [SHT40-BD1B-R2](https://www.mouser.co.uk/ProductDetail/?qs=ulEaXIWI0c%252BjgdkzNQkrGw%3D%3D)                    | £2    |                    | I2C            |                                          |
| Temperature Sensor   | [TMP1075DSGR](https://www.digikey.co.uk/en/products/detail/texas-instruments/TMP1075DSGR/10715322)              | £0.52 |                    | I2C            |                                          |
| 4-Channel DAC        | [MCP4728T-E/UN](https://www.digikey.co.uk/en/products/detail/microchip-technology/MCP4728T-E-UN/2126093)        | £1.98 |                    | I2C -> Analog  |                                          |
| Light Sensor         | [VEML7700](https://www.digikey.co.uk/en/products/detail/vishay-semiconductor-opto-division/VEML7700-TR/5820243) | £1.5  |                    | I2C            |                                          |
| 8-Bit I2C -> Digital | [PCF8574ADWR](https://www.digikey.co.uk/en/products/detail/texas-instruments/PCF8574ADWR/484754)                | £1    |                    | I2C -> Digital |                                          |
| 12Bit I2C ADC        | [ADC121C021CIMM](https://www.digikey.co.uk/en/products/detail/texas-instruments/ADC121C021CIMM-NOPB/2075626)    | £1.95 |                    | Analog -> I2C  |                                          |
| Pressure Sensor      | LWP040-PSGLC-S                                                                                                  | £0.30 |                    | Analog         |                                          |
| Push Button          |                                                                                                                 |       |                    | Digital        |                                          |
| Side Switch          |                                                                                                                 |       |                    | Digital        |                                          |


## Electronic Schematics:

- [x] [[Motion Sensor]]
- [ ] Humidity Sensor
- [ ] Temperature Sensor
- [ ] Ambient Light Sensor
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
