use dfplayer_esp32::DfPlayer;
use esp_idf_hal::{
    gpio::AnyIOPin,
    peripherals::Peripherals,
    uart::{UartConfig, UartDriver},
};

fn main() -> anyhow::Result<()> {
    esp_idf_hal::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

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
