{ pkgs ? import <nixpkgs> {} }:

with pkgs;

let
  unstable = import <unstable> {};
  secrets = {};

  lldb = pkgs.lldb_15;

  lldbEnv = buildEnv {
    name = "lldb-full-${lldb.version}";
    paths = [ lldb lldb.lib ];
    pathsToLink = [ "/bin" "/lib" "/share" "/lib/python3.10/site-packages/lldb" ];
    buildInputs = [ pkgs.makeWrapper ];
    postBuild = ''
    rm $out/lib/python3.10/site-packages/lldb/lldb-argdumper
    rm $out/lib/python3.10/site-packages/lldb/_lldb.cpython-310-x86_64-linux-gnu.so
    ln -s $out/bin/lldb-argdumper $out/lib/python3.10/site-packages/lldb/lldb-argdumper
    ln -s $out/lib/liblldb.so     $out/lib/python3.10/site-packages/lldb/_lldb.cpython-310-x86_64-linux-gnu.so
    '';
  };

  debug-tests = pkgs.writeScriptBin "debug-tests" ''
  #!/usr/bin/env bash
  exe=$(
  cargo test --no-run --message-format=json \
    | jq -r 'select(.profile.test == true) 
             | select(.target.kind[] | contains("lib")) 
             | .executable'
  )
  rust-lldb $exe
  '';


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