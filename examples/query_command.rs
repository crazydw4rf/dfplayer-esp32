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

    let playback_source = player.playback_source()?;
    println!("Current playback source: {playback_source:?}");

    let volume = player.volume()?;
    println!("Current volume level: {volume}/30");

    let status = player.playback_status()?;
    println!("Current playback status: {status:?}");

    let total_tracks = player.track_count_tf()?;
    println!("Total tracks on TF/microSD card: {total_tracks}");

    Ok(())
}
