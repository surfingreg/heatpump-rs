# Work in Progress!

The goal is to control my Mitsubishi heat pumps in pure Rust. Others have gone before
in C++ and Python. I fully intend to stand on their shoulders. The project most 
similar to my own in mission is [this](https://github.com/Sammy1Am/mitsubishi-uart). However,
I'm not ready to commit to the ESPHome project. Primarily because I really, really do not like
working in yaml, which is excessively required by ESPHome. I do want to connect my stuff to 
Home Assistant. I'm not sure the best methodology just yet (ie MQTT, websocket, Kafka, ESPHome, plain old REST).
Regardless it'll be in Rust. 

## Seminal works to reverse engineer the Mitsubishi heat pump
- https://github.com/SwiCago/HeatPump (c/c++)
- https://github.com/gysmo38/mitsubishi2MQTT (web/mqtt/c/c++)
- https://github.com/geoffdavis/esphome-mitsubishiheatpump (yaml/esphome/)
- https://github.com/echavet/MitsubishiCN105ESPHome (yaml/esphome/python)
- SwiCago's inital communications from 2017, communicating over CN105
- https://web.archive.org/web/20171007190023/https://nicegear.co.nz/blog/hacking-a-mitsubishi-heat-pump-air-conditioner/
- https://web.archive.org/web/20171126013431/http://www.esp8266.com/viewtopic.php?f=29&t=13207 (pin diagram for cn105)
- https://github.com/hadleyrich/MQMitsi (the other side of the original project, python, good protocol info)

## Rust-Related
- https://github.com/rust-embedded/awesome-embedded-rust
- Rust on ESP Book: https://docs.esp-rs.org/book/writing-your-own-application/generate-project/index.html
- OTA: https://lib.rs/crates/esp-ota (rust, similar functionality hidden in esp-idf-svc, so not necessary, but documented)
- https://github.com/faern/esp-ota (use inside a web handler to copy new .bin to an OTA partition)
- https://embassy.dev/ ("framework for embedded", Rust, async, hardware timers)
- https://github.com/orgs/esp-rs/repositories (espressif, rust, esp-idf-svc, espflash, )
- https://github.com/actix/actix-web/issues/199 (TODO: PR for actix to allow use on ESP32)
- CO2 sensor using embassy, home assistant, todo: add esp32s3 support?
- https://github.com/matoushybl/air-force-one
- https://github.com/obabec/rust-mqtt (crate, tokio, embedded-io)
- https://blog.drogue.io/firmware-updates-part-1/ (embassy-boot)
- https://dev.to/theembeddedrustacean/embedded-rust-embassy-uart-serial-communication-4fd3 (esp32s3 example?)

## ESP32
- https://github.com/esp-rs/esp-idf-template (starter template, Rust over C++)
- reset esp32 https://support.arduino.cc/hc/en-us/articles/9810414060188-Reset-the-Arduino-bootloader-on-the-Nano-ESP32  
- esp32 pinout
- https://www.techrm.com/wp-content/uploads/2024/02/Arduino_Nano_ESP32_pinout.jpg
- https://www.techrm.com/independent-control-of-two-leds-with-arduino-nano-esp32-a-practical-introduction/ (platformio/c++)
- setting up embassy on esp32 https://pg3.dev/post/13
- various sample projects on esp32 https://github.com/PGIII/rust-esp32

## CN105 Hardware
- https://github.com/SwiCago/HeatPump/issues/13#issuecomment-457897457
- https://gotductless.com/products/airzone-aidoo-cn105-splitter-for-mitsubishi (cn105 splitter)


## TODO
- [] start wifi as client (axum, https://github.com/olegccc/esp32-axum-ws)
- [] start web server w/flash api
- [] OTA software update (https://github.com/faern/esp-ota)
- [] send debug info via mqtt
- [] receive data over serial and send over mqtt
- [] convert to library
- [] create crate
- [] start wifi as access point (then change to client)
