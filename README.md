## TODO

* compute next state on tick (i.e. implement commands)
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
