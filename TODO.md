# TODO

* check why multiple rand versions are compiled

## Presets

* Add prefix to exported json like ---patch-data-below--- so exports from
  programs can be automatically imported as default patch bank. regex::bytes
  could probably be used.
* Default preset bank instead of default presets

## Code quality / safety

* NUM_PARAMETERS constant?
* Fix clippy errors
* rustfmt

## Other

* manual text input in parameters: DAW integration working anywhere?
* sample rate change: what needs to be done? (time reset?)
* Nice online documentation

## Maybe do

* Volume shown in dB
* Iterator for presets and preset parameters
* volume off by default for operator 3 and 4. Would need to change ::default to ::new and this would require a refactor
* proper beta scaling - double with doubling modulator frequency
* suspend mode and so on, maybe just reset time, note time, envelopes etc on resume
* Use FMA again for precision, possibly enabling removing .fract() call
  in fallback sound gen?
* Fuzz Log10Table (cargo-fuzz?)