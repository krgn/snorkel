{ pkgs ? import <nixpkgs> {} }:

with pkgs;

let
  unstable = import <unstable> {};
  secrets = {};

in  mkShell (lib.mergeAttrs secrets {
  buildInputs = [ 
    debug-tests lldbEnv pkg-config openssl
    portmidi rustup orca-c
  ];

  # RUST_BACKTRACE = "1";
  # CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER = "rust-lldb"; 

  PYTHONPATH = "$PYTHONPATH:${lldbEnv}/lib/python3.10/site-packages";

  shellHook = ''
  export PATH=$PATH:$(dirname $(rustup which rust-analyzer))
  '';
})
