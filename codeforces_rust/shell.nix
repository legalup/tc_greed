{ src ? builtins.fetchGit {
    url = "git@ghe.anduril.dev:anduril/anduril-nixpkgs.git";
    ref = "tff/add-hop";
    rev = "7b215caa5c6f569cc7edce2f7c654dc1637c943d";
  }
, cross ? false
, cross-system ? "aarch64-multiplatform"
, local ? false
}:
let
  pkgs-native = import (if local then <nixpkgs> else src) {};
  pkgs = if cross then pkgs-native.pkgsCross."${cross-system}" else pkgs-native;
in
with pkgs;
mkShell {
  inputsFrom = with anduril; [
    (rust.addSrcToCrateToolchain rust.hop)
  ];
  nativeBuildInputs = [
    anduril.rust.makeCLionToolchain
    cmake
  ];
}