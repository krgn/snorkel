{ pkgs ? import <nixpkgs> {} }:

with pkgs;

let
  unstable = import <unstable> {};
  secrets = {};

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

in  mkShell (lib.mergeAttrs secrets {
  buildInputs = [ 
    # rust + native deps
    lldb pkg-config openssl unstable.clang_14
    portmidi
    rustup mold-custom
  ];

  shellHook = ''
  export PATH=$PATH:$(dirname $(rustup which rust-analyzer))
  '';
})