## TODO

* compute next state on tick (i.e. implement commands)
  - `E` east: Moves eastward, or bangs.
  - `G` generator(x y len): Writes operands with offset.
  - `H` hold: Holds southward operand.
  - `I` increment(step mod): Increments southward operand.
  - `J` jumper(val): Outputs northward operand.
  - `K` konkat(len): Reads multiple variables.
  - `L` lesser(a b): Outputs smallest of inputs.
  - `M` multiply(a b): Outputs product of inputs.
  - `N` north: Moves Northward, or bangs.
  - `O` read(x y read): Reads operand with offset.
  - `P` push(len key val): Writes eastward operand.
  - `Q` query(x y len): Reads operands with offset.
  - `R` random(min max): Outputs random value.
  - `S` south: Moves southward, or bangs.
  - `T` track(key len val): Reads eastward operand.
  - `U` uclid(step max): Bangs on Euclidean rhythm.
  - `V` variable(write read): Reads and writes variable.
  - `W` west: Moves westward, or bangs.
  - `X` write(x y val): Writes operand with offset.
  - `Y` yumper(val): Outputs westward operand.
  - `Z` lerp(rate target): Transitions operand to input.
  - `*` bang: Bangs neighboring operands.
  - `#` comment: Holds a line.
  - `$` self(cmd): Send a command to Orca, or load external file.
  - `:` midi(ch oct note velocity*): Send a midi note.
  - `!` midi cc(ch knob val): Send a midi control change.
  - `;` pitch(oct note): Send pitch byte out.
  - `/` byte(high low): Send a raw hexadecimal byte.
  - `=` play(ch oct note velocity*): Play note with built-in synth.
* scheduler
  - midi IO
  - OSC IO
  - UDP IO
* load/save file
* grid markers
* re-size
* re-grid
* config file
* center grid