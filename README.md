## TODO

* compute next state on tick (i.e. implement commands)
  - `K` konkat(len): Reads multiple variables.
  - `P` push(len key val): Writes eastward operand.
  - `Q` query(x y len): Reads operands with offset.
  - `T` track(key len val): Reads eastward operand.
  - `U` uclid(step max): Bangs on Euclidean rhythm.
  - `V` variable(write read): Reads and writes variable.
  - `Z` lerp(rate target): Transitions operand to input.
  - `*` bang: Bangs neighboring operands.
  - `$` self(cmd): Send a command to Orca, or load external file.
  - `:` midi(ch oct note velocity*): Send a midi note.
  - `!` midi cc(ch knob val): Send a midi control change.
  - `;` pitch(oct note): Send pitch byte out.
  - `/` byte(high low): Send a raw hexadecimal byte.
  - `=` play(ch oct note velocity*): Play note with built-in synth.
* scheduler
* load/save file
* re-size
* re-grid
* config file
