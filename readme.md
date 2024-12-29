# Work in Progress!

## TODO

- start wifi as client
- start web server w/flash api
- OTA software update
- send debug info via mqtt
- receive data over serial and send over mqtt
- convert to library
- create 'heatpump' crate and lock in name
- start wifi as access point (then change to client)

## References

## Where I want to go...  
I'm undecided on a few points: communicate via MQTT? websocket? not sure how ESPHome talks.  
The main difference is I want to start over entirely in Rust. Oh, and I strenuously dislike yaml.
- https://github.com/Sammy1Am/mitsubishi-uart

## The seminal works to reverse engineer the Mitsubishi heat pump.
- https://github.com/SwiCago/HeatPump (c/c++)
- https://github.com/gysmo38/mitsubishi2MQTT (web/mqtt/c/c++)
- https://github.com/geoffdavis/esphome-mitsubishiheatpump (yaml/esphome/)
- https://github.com/echavet/MitsubishiCN105ESPHome (yaml/esphome/python)
- SwiCago's inital communications from 2017, communicating over CN105
- https://web.archive.org/web/20171007190023/https://nicegear.co.nz/blog/hacking-a-mitsubishi-heat-pump-air-conditioner/
- https://web.archive.org/web/20171126013431/http://www.esp8266.com/viewtopic.php?f=29&t=13207 (pin diagram for cn105)
- https://github.com/hadleyrich/MQMitsi (the other side of the original project, python, good protocol info)

## CN105 Hardware:
- https://github.com/SwiCago/HeatPump/issues/13#issuecomment-457897457
- https://gotductless.com/products/airzone-aidoo-cn105-splitter-for-mitsubishi (cn105 splitter)

## Rust-related
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