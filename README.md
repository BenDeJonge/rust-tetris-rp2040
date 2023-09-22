# rust-tetris-rp2040

An embedded Rust implementation of the classic tetris game, running on a small TFT display, steered by an RP2040 microcontroller. This project was developed with the goal to familiarize myself further with embedded development, while having some fun along the way. The game aims to follow the (to my surprise existing) [official Tetris guidelines](https://tetris.fandom.com/wiki/Tetris_Guideline) and the so-called [Super Rotation System](https://tetris.fandom.com/wiki/SRS).

## Software

- Rust v1.72.0
- cargo v1.72.0


## Hardware

### Bill of materials

| Item | Number | Documentation | Vendor |
|------|--------|---------------|--------|
| Raspberry Pi RP2040 | 2 | [Link](https://www.raspberrypi.com/documentation/microcontrollers/rp2040.html) | [Link](https://www.amazon.com.be/dp/B09M41JJTQ?ref_=pe_43847721_714291821_302_E_DDE_dt_1&th=1) |
| ST7735 TFT display | 1 | [Link](https://docs.rs/embedded-graphics/0.8.1/embedded_graphics/) | [Link](https://www.az-delivery.de/en/products/1-8-zoll-spi-tft-display?_pos=1&_sid=3976a5e65&_ss=r&variant=4982039773211) |
| KY-023 Joystick | 2 | [Link](https://cdn.shopify.com/s/files/1/1509/1638/files/Joystick_Modul_Datenblatt.pdf?17237179332079383541) | [Link](https://www.az-delivery.de/en/products/joystick-modul) |
| Perfboard | 1	| [Link](https://morepcb.com/perfboard-an-essential-tool-for-diy-electronics-enthusiasts/) | [Link](https://www.amazon.com.be/gp/product/B08YXKRJY1/ref=ppx_yo_dt_b_asin_title_o02_s00?ie=UTF8&psc=1) |
| Resistor $1 \mathrm{k}\Omega$ | 4 | / | [Link](https://www.amazon.com.be/-/en/AUKENIEN-4W-Resistors-Assortment-Resistor/dp/B0967TG6XR/ref=sr_1_2?crid=2ATCMYVLQ3L06&keywords=aukenien+resistor&qid=1695394845&s=industrial&sprefix=aukenien+resistor%2Cindustrial%2C80&sr=1-2) |
| Resistor $110 \Omega$ | 5 | / | [Link](https://www.amazon.com.be/-/en/AUKENIEN-4W-Resistors-Assortment-Resistor/dp/B0967TG6XR/ref=sr_1_2?crid=2ATCMYVLQ3L06&keywords=aukenien+resistor&qid=1695394845&s=industrial&sprefix=aukenien+resistor%2Cindustrial%2C80&sr=1-2) |
| Capacitor $110 \mathrm{nF}$ | 4 | / | [Link](https://www.amazon.com.be/-/en/AUKENIEN-Ceramic-Capacitor-Assortment-Capacitors/dp/B09NLZBC7R/ref=pd_bxgy_sccl_1/258-7074883-2651129?pd_rd_w=cLzGS&content-id=amzn1.sym.48cad8ef-3402-4c15-886d-cf5371c3d62a&pf_rd_p=48cad8ef-3402-4c15-886d-cf5371c3d62a&pf_rd_r=5N5NMCZWMYKVH76QDD59&pd_rd_wg=VsAC4&pd_rd_r=b1dd503b-5604-4f81-85d2-e9234d88fea6&pd_rd_i=B09NLZBC7R&th=1) |

### Circuit diagram

![Circuit diagram](circuit.svg)

## Demo

TBD

## Links

- [RP2040 pinout](https://mischianti.org/2022/09/19/waveshare-rp2040-zero-high-resolution-pinout-and-specs/#SPI_Pins)
- [ST7735 pinout 1](https://thesolaruniverse.wordpress.com/2020/12/26/the-tiny-tft-that-delivers-the-0-96-inch-80160-tft-display-with-st7735-driver-for-arduino/)
- [ST7735 pinout 2](https://cdn.shopify.com/s/files/1/1509/1638/files/1_77_Zoll_SPI_TFT_Display_Datenblatt_AZ-Delivery_Vertriebs_GmbH_0eab71a3-f0c9-42af-8089-d8e6f689e9dc.pdf?v=1606166813)
- [SPI basics](https://www.analog.com/en/analog-dialogue/articles/introduction-to-spi-interface.html)
- [Chip select](https://en.wikipedia.org/wiki/Chip_select)
- [Rust HAL](https://github.com/rp-rs/rp-hal)