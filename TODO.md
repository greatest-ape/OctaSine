# TODO

## GUI

* Don't load parameter changes while dragging in `iced_audio` widgets.
  Also maybe don't load them in processing either during this time.
  Maybe add callbacks like `started_dragging` and `finished_dragging`

## Code quality / safety

* NUM_PARAMETERS constant?
* rustfmt

## Other

* manual text input in parameters: DAW integration working anywhere?
* sample rate change: what needs to be done? (time reset?)
* Nice online documentation
* Consider logging when preset can't be loaded (see `load_bank_data`)

## Maybe do

* Volume shown in dB
* Iterator for presets and preset parameters
* proper beta scaling - double with doubling modulator frequency
* suspend mode and so on, maybe just reset time, note time, envelopes etc on resume
* Use FMA again for precision, possibly enabling removing .fract() call
  in fallback sound gen?
* Fuzz Log10Table (cargo-fuzz?)
