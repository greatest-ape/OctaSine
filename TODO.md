# TODO

## split

* Actually add info about different licenses. We want MIT OR Apache 2.0, also
  for sleef simd crate
* TimeCounter should just be any type
* Envelope? probably won't be added to vst2_helpers because of complexity

## Prioritized

* clippy, rustfmt
* Envelopes: evaluate new curve. Consider if linear mixing is really
  necessary, and if the minimum envelope time could/should be adjusted
* Fix name of calculate_curveolume_output_in_range
* Use FMA again for precision, possibly enabling removing .fract() call
  in sound gen? Was bad for performance on my computer before, strangely


## TODO

* Target features instead of target-cpu

    `rustc --print cfg -C target-cpu=native -C opt-level=3`

* Nice online documentation
* More intelligent analysis of whether volume is off, with dependency analysis.
  Could start with getting operator and envelope volume of all operators. Then
  go through from operator 1 upwards. Check modulation targets and if additive is 0.
  Something like that.
* Default preset bank instead of default presets
* Integration tests for presets/preset parameters/processing parameters?
* integration tests in general?
* NUM_PARAMETERS constant?

* Add prefix to exported json like ---patch-data-below--- so exports from
  programs can be automatically imported as default patch bank. regex::bytes
  could probably be used.
* Why is live taking to long to load vsts? Check with time profiler?
* manual text input in parameters: DAW integration working anywhere?
* sample rate change: what needs to be done? (time reset?)
* Portable shell support in scripts (not only bash). Might be very easy

# Non-important improvements

* Optional callback in interpolation get_value

# Maybe

* Volume shown in dB
* Iterator for presets and preset parameters
* volume off by default for operator 3 and 4. Would need to change ::default to ::new and this would require a refactor
* proper beta scaling - double with doubling modulator frequency
* suspend mode and so on, maybe just reset time, note time, envelopes etc on resume
* Remove BPM fetch support