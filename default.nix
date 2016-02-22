{ pkgs ? (import <nixpkgs> {}) }:

let
  env = with pkgs.rustStable; [
    rustc
    cargo
  ];
in

pkgs.stdenv.mkDerivation rec {
    name = "i3-rustbar";
    src = ./.;
    version = "0.0.0";

    buildInputs = [ env ];

}

