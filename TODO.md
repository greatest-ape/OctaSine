# TODO

## High priority

* Option to disable LFOs / set no target
* In mod target widget, use space element instead of horizontal centering
* Envelope dragger "dragging" color
* Raise default volume (and maybe hard limit limit)
* Display patch index independently of patch name, use - or similar as default patch name: "001: -"
* Consider easing fine tuning of mod out / changing steps. More room where
  sounds are the most interesting.
* Include license info on all dependencies in releases
* Release v0.7.0 eventually

## Medium priority

* Consider built-in patch browsing / saving functionality
* Consider creating PR in cargo-deny-action
* Mode to lock together envelopes so changes affect all
* bench_process
  * Is it a cause for concern that not keeping wave type fixed has different
    effect depending on SIMD width?
* GUI
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
* Consider time-based instead of sample-based interpolation for processing
  parameters and LFOs
* Build for Apple silicon
  * ADVSIMD (NEON) acceleration should be supported, at least by enabling the
    target feature. I'm not sure about how that is done when cross-compiling.

## Very low priority

* Process benchmark output not same on Windows as on macOS/Linux
* Record video of workflow, upload to YouTube
* Consider updating envelope and lfo values in process benchmark too. This
  would further improve usefulness of output hashing.
* GUI
  * Operator audio output indicator, either binary or volume
  * Master audio output indicator
  * Zoom towards center of envelope duration instead of viewport if
    envelope doesn't cover viewport? (Or maybe always)
  * Nicer knob marks
    * Operator 2-4 middle marker
  * Do I need to run update_host_display?
    * Should it be run on knob drag release?
* sample rate change: what needs to be done? (time reset?)
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
