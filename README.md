# DFPlayer Mini ESP32

A Rust library for communicating with the DFPlayer Mini module.
Only supports ESP-IDF for now.

References and datasheets:

- <https://github.com/PowerBroker2/DFPlayerMini_Fast/blob/master/extras/FN-M16P%2BEmbedded%2BMP3%2BAudio%2BModule%2BDatasheet.pdf>
- <https://docs.keyestudio.com/projects/ks0387/en/latest/docs/Keyestudio%20YX5200-24SS%20MP3%20Module.html>
- <https://cdn.robotshop.com/media/d/dfr/rb-dfr-562/pdf/dfplayer_mini_manual.pdf>

## TODO

- [ ] implement query commands
- [ ] support `esp-hal` or `no_std`
- [ ] write proper documentation
- [ ] add code examples
- [ ] setup github actions to build, test, and publish the crate.
