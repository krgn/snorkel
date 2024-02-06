{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        lldb = pkgs.lldb_15;

        lldbEnv = pkgs.buildEnv {
          name = "lldb-full-${lldb.version}";
          paths = [ lldb lldb.lib ];
          pathsToLink = [ "/bin" "/lib" "/share" "/lib/python3.10/site-packages/lldb" ];
          buildInputs = [ pkgs.makeWrapper ];
          postBuild = ''
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
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [ 
            bashInteractive
            pkg-config openssl portmidi rustup orca-c

            lldbEnv debug-tests
          ];

          PYTHONPATH = "$PYTHONPATH:${lldbEnv}/lib/python3.10/site-packages";
        };
      });
}
