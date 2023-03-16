{ pkgs ? import <nixpkgs> {} }:

with pkgs;

let
  unstable = import <unstable> {};
  secrets = {};

  python-custom = unstable.python3.withPackages(pkgs: with pkgs; [
    lldb
  ]);

  mold-custom = pkgs.mold.overrideDerivation(attrs: rec {
    version = "1.10.1";
    name = "mold-${version}";
    src = pkgs.fetchFromGitHub {
      owner = "rui314";
      repo = "mold";
      rev = "v${version}";
      sha256 = "sha256-5zE5a+BYzQjgVb0Ti7bSQrGzTyysOTTR0NMOO5IKG68=";
    };
  });

  gdb-custom = pkgs.gdb.overrideDerivation(attrs: rec {
    version = "13.1";
    name = "gdb-${version}";
    src = pkgs.fetchurl {
      url = "https://ftp.gnu.org/gnu/gdb/gdb-${version}.tar.gz";
      sha256 = "sha256-TMPXFD1tVNKJ0iex5yidvA+ky9RhMauHE24eqDHPRtQ=";
    };
  });

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
    # rust + native deps
    lldb_15 gdb-custom pkg-config openssl unstable.clang_14
    portmidi
    rustup mold-custom python-custom
    debug-tests
  ];

  # RUST_BACKTRACE = "1";
  # CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER = "rust-lldb"; 

  shellHook = ''
  export PATH=$PATH:$(dirname $(rustup which rust-analyzer))
  '';
})