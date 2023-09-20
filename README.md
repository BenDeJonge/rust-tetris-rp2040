# rust-tetris-rp2040

An embedded Rust implementation of the classic tetris game, running on a small TFT display, steered by an RP2040 microcontroller. This project was developed with the goal to familiarize myself further with embedded development, while having some fun along the way. The game aims to follow the (to my surprise existing) [official Tetris guidelines](https://tetris.fandom.com/wiki/Tetris_Guideline) and the so-called [Super Rotation System](https://tetris.fandom.com/wiki/SRS).

## Software

- Rust v1.72.0
- cargo v1.72.0
- [Rust HAL](https://github.com/rp-rs/rp-hal)

## Hardware

### Bill of materials

| Item                	| Documentation                                                                                             	| Vendor                                                                                                                 	|
|---------------------	|-----------------------------------------------------------------------------------------------------------	|------------------------------------------------------------------------------------------------------------------------	|
| Raspberry Pi RP2040 	| [1](https://www.raspberrypi.com/documentation/microcontrollers/rp2040.html)                               	| [1](https://www.amazon.com.be/dp/B09M41JJTQ?ref_=pe_43847721_714291821_302_E_DDE_dt_1&th=1)                            	|
| ST7735 TFT display  	| [2](https://docs.rs/embedded-graphics/0.8.1/embedded_graphics/)                                           	| [2](https://www.az-delivery.de/en/products/1-8-zoll-spi-tft-display?_pos=1&_sid=3976a5e65&_ss=r&variant=4982039773211) 	|
| KY-023 Joystick     	| [3](https://cdn.shopify.com/s/files/1/1509/1638/files/Joystick_Modul_Datenblatt.pdf?17237179332079383541) 	| [3](https://www.az-delivery.de/en/products/joystick-modul)                                                             	|
| Perfboard           	| [4](https://morepcb.com/perfboard-an-essential-tool-for-diy-electronics-enthusiasts/)                     	| [4](https://www.amazon.com.be/gp/product/B08YXKRJY1/ref=ppx_yo_dt_b_asin_title_o02_s00?ie=UTF8&psc=1)                  	|

### Circuit diagram

TBD

## Demo

TBD