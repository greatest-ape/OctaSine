# TODO

## High priority

* Fix failing CI test
* Performance
  * Were envelope / LFO changes to use f32 100% OK?
* bench_process
  * try generating delta_frames with rng
  * try generating key velocity with rng
* Audio
  * Should operator volume affect feedback?
  * Envelope curve takeover is now 100ms. It could probably be shorter,
    like 50ms
  * Should modulation index now compensate for higher frequencies?
  * Should there be phase additions when modulating? See
    https://en.wikipedia.org/wiki/Frequency_modulation_synthesis#Spectral_analysis
* GUI
  * Envelopes
    * Zoom by dragging background up/down
    * Fit by double clicking?
    * Two envelope lock groups
* Other crates
  * create iced_audio 0.8.0 release, use it
  * ask for new baseview release, then create iced_baseview release, use them
* Add semver-compatible version info in patch/patch bank exports?
* Probably don't do / no longer relevant:
  * Envelope clicks when using DAW and looping notes without space between? Or
    just normal attack? But only when modulating. Interpolate for
    attack_duration.min(0.03) or similar? Or use VST MIDI noteOffset or similar
    to avoid restarting envelopes in some cases?
  * Use sleef for fract calculations etc?
  * Use fastmath for log table?
  * Operator freq ratio values: are really both PI, sqrt(2) and sqrt(3) values
    necessary? Yeah, why not, people might use three different inharmonic ones
    I suppose?
  * Parameter step mapping is expensive when combined with LFOs
    * For modulation index/feedback, maybe exp2 could be used. Same with frequency
      parameters.
  * GUI: in operator freq ratio values, display number too? E.g. 2pi: 6.28
  * Add small marks to operator ratio knobs indicating factors of 2?
* Release v0.7.0 eventually

## Medium priority

* Consider built-in patch browsing / saving / clearing functionality
  * Use crate https://github.com/PolyMeilex/rfd
  * Maybe use buttons like "C" for clear, "S" for save, "L" for load, "R" for
    rename. They could have tooltips.
* Parameter value text input
  * Maybe use https://github.com/jdm/tinyfiledialogs-rs
* Mode to lock together envelopes so changes affect all
* bench_process
  * Is it a cause for concern that not keeping wave type fixed has different
    effect depending on SIMD width?
* GUI
  * Consider adding widget for LFOs and operators showing cumulative
    frequency multiplier
  * Scrolling in dropdowns
    * iced 0.4: https://github.com/hecrj/iced/pull/872
    * Does scrolling (including touch) need to be added to baseview
      macOS code? What about other platforms?
* Documentation
  * Double-click to reset knobs
  * Shift-drag knobs for fine tuning

## Low priority

* GUI
  * Mouse drag movements in pick list transfer through to envelope editor
* Consider adding saw, square and triangle waves. Maybe look at
  TX81Z waveforms. https://www.reddit.com/r/synthesizers/comments/rkyk6j/comment/hpgcu6r/?utm_source=share&utm_medium=web2x&context=3
* Build for Apple silicon
  * ADVSIMD (NEON) acceleration should be supported, at least by enabling the
    target feature. I'm not sure about how that is done when cross-compiling.

## Very low priority

* Process benchmark output not same on Windows as on macOS/Linux
* Record video of workflow, upload to YouTube
* Consider updating envelope and lfo values in process benchmark too. This
  would further improve usefulness of output hashing.
* GUI
  * Zoom towards center of envelope duration instead of viewport if
    envelope doesn't cover viewport? (Or maybe always)
  * Do I need to run update_host_display?
    * Should it be run on knob drag release?
* clippy
* Test that number of sync and processing parameters is equal
* suspend mode and so on, maybe just reset time, note time, envelopes etc on resume
* Fuzz Log10Table (cargo-fuzz?)
* Is it necessary to look at time signatures etc for bpm sync?
  https://rustaudio.github.io/vst-rs/vst/api/struct.TimeInfo.html
* Preset parameter from text
  * Implement simple parsing etc for all
  * DAW integration working anywhere?
* Nice online documentation

## Don't do

* Free LFO mode. Difficult to sensibly combine with per-voice LFOs
* Cache sync value in interpolatable parameters too? Don't do this, it seems
  to hurt performance.
* proper beta scaling - double with doubling modulator frequency: too late now
* Add phase knobs. This isn't compatible with the fact that the voices have
  independent phases and FM is done by incrementing the phase. It probably
  wouldn't contribute a lot to audio generation flexibility to change this
  just to add possibility of setting operator phase in addition to frequency.
