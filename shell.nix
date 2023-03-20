{ pkgs ? import <nixpkgs> {} }:

with pkgs;

let
  unstable = import <unstable> {};
  secrets = {};

  debug-tests = pkgs.writeScriptBin "debug-tests" ''
  #!/usr/bin/env bash
  exe=$(
  cargo test --no-run --message-format=json \
    | jq -r 'select(.profile.test == true) 
             | select(.target.kind[] | contains("lib")) 
             | .executable'
  )
  reset
  rust-gdb --eval-command='b rust_panic' $exe
  '';

in  mkShell (lib.mergeAttrs secrets {
  buildInputs = [ 
    lldb_15 pkg-config openssl
    portmidi rustup orca-c
  ];

  # RUST_BACKTRACE = "1";
  # CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER = "rust-lldb"; 

  shellHook = ''
  export PATH=$PATH:$(dirname $(rustup which rust-analyzer))
  '';
})