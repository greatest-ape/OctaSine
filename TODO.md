# TODO

## High priority

* Fuzz Log10Table (cargo-fuzz?)

## Normal priority

* Target features instead of target-cpu

    `rustc --print cfg -C target-cpu=native -C opt-level=3`

* Nice online documentation
* More intelligent analysis of whether volume is off, with dependency analysis.
  Could start with getting operator and envelope volume of all operators. Then
  go through from operator 1 upwards. Check modulation targets and if additive
  is 0. Something like that.
* Default preset bank instead of default presets
* NUM_PARAMETERS constant?
* Add prefix to exported json like ---patch-data-below--- so exports from
  programs can be automatically imported as default patch bank. regex::bytes
  could probably be used.
* manual text input in parameters: DAW integration working anywhere?
* sample rate change: what needs to be done? (time reset?)
* Fix clippy errors
* rustfmt

## Maybe do

* Volume shown in dB
* Iterator for presets and preset parameters
* volume off by default for operator 3 and 4. Would need to change ::default to ::new and this would require a refactor
* proper beta scaling - double with doubling modulator frequency
* suspend mode and so on, maybe just reset time, note time, envelopes etc on resume
* Remove BPM fetch support
* Use FMA again for precision, possibly enabling removing .fract() call
  in sound gen? Was bad for performance on my computer before, strangely