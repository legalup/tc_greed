let
  rust-overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [(import rust-overlay)];
  };
  toolchain = pkgs.rust-bin.fromRustupToolchainFile /home/luis/toolchain.toml;
in
  pkgs.mkShell {
    packages = [
      toolchain
    ];
  }
# shell.nix

  
  
  

  
  
