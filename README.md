# DFPlayer Mini ESP32

A Rust library for communicating with the DFPlayer Mini module.
Only supports ESP-IDF for now.

## Examples

### Playing specific track and set volume

```rust
use dfplayer_esp32::DfPlayer;
use esp_idf_hal::{
    gpio::AnyIOPin,
    peripherals::Peripherals,
    uart::{UartConfig, UartDriver},
};

fn main() -> anyhow::Result<()> {
    esp_idf_hal::sys::link_patches();

    let peripherals = Peripherals::take()?;

    let config = UartConfig::new().baudrate(9600.into());

    let uart = UartDriver::new(
        peripherals.uart2,
        peripherals.pins.gpio25, // ESP32 TX -> DFPlayer RX
        peripherals.pins.gpio26, // ESP32 RX -> DFPlayer TX
        Option::<AnyIOPin>::None,
        Option::<AnyIOPin>::None,
        &config,
    )?;

    let player = DfPlayer::new(&uart);

    // Set volume to a reasonable level (range: 0–30)
    player.set_volume(15)?;

    // Play track 1 from the root directory
    player.play_track(1)?;

    Ok(())
}
```

Take a look at [examples](./examples) directory for more examples.

References and datasheets:

- <https://github.com/PowerBroker2/DFPlayerMini_Fast/blob/master/extras/FN-M16P%2BEmbedded%2BMP3%2BAudio%2BModule%2BDatasheet.pdf>
- <https://docs.keyestudio.com/projects/ks0387/en/latest/docs/Keyestudio%20YX5200-24SS%20MP3%20Module.html>
- <https://cdn.robotshop.com/media/d/dfr/rb-dfr-562/pdf/dfplayer_mini_manual.pdf>

## TODO

- [ ] support `esp-hal` or `no_std`
- [ ] write proper documentation
- [ ] setup github actions to build, test, and publish the crate.
