{ src ? builtins.fetchGit {
    url = "git@ghe.anduril.dev:anduril/anduril-nixpkgs.git";
    ref = "master";
    rev = "fe9eee948e52baca3650d00460190048b4f6a065";
  }
, cross ? false
, cross-system ? "aarch64-multiplatform"
, local ? false
, test ? false # extra test dependencies
}:

let

  pkgs-src = if local then <nixpkgs> else src;
  pkgs-native = import pkgs-src {};
  pkgs = if cross then pkgs-native.pkgsCross."${cross-system}" else pkgs-native;
in

with pkgs;

mkShell {
  LZ4_LIB_PATH = "${lz4.out}/lib";
  ZSTD_LIB_PATH = "${zstd.out}/lib";

  inputsFrom = with anduril; [
    (rust.addSrcToCrateToolchain rust.argos-rs)
  ];

  nativeBuildInputs = [
    anduril.rust.makeCLionToolchain
  ];

}

