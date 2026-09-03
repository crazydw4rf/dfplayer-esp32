/// A collection of DFPlayer Mini commands.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Command {
    /// Plays the next track.
    Next = 0x01,

    /// Plays the previous track.
    Previous = 0x02,

    /// Plays a specific track number (1–2999).
    PlayTrack = 0x03,

    /// Increases the volume by one step.
    VolumeUp = 0x04,

    /// Decreases the volume by one step.
    VolumeDown = 0x05,

    /// Sets the volume level (0–30).
    SetVolume = 0x06,

    /// Sets the equalizer preset (Normal, Pop, Rock, Jazz, Classic, Bass).
    SetEqualizerPreset = 0x07,

    /// Sets the playback mode (Repeat, Folder repeat, Single repeat, Random).
    SetPlaybackMode = 0x08,

    /// Sets the playback source (USB, TF Card, AUX, Sleep, Flash).
    SetPlaybackSource = 0x09,

    /// Enters low-power standby mode.
    EnterStandby = 0x0A,

    /// Puts the module in normal working mode (exits standby).
    ExitStandby = 0x0B,

    /// Resets the DFPlayer Mini module.
    ResetModule = 0x0C,

    /// Resumes playback.
    PlaybackResume = 0x0D,

    /// Pauses playback.
    PlaybackPause = 0x0E,

    /// Plays a specific track from a specified folder (Folder 1–99, File 1–255).
    SetPlaybackFolder = 0x0F,

    /// Sets the volume gain (gain 0–31).
    SetVolumeGain = 0x10,

    /// Enables (1) or stops (0) repeat playback.
    RepeatPlay = 0x11,
}
