let
  nixpkgs = import <nixpkgs> { };
in
  with nixpkgs;

pkgs.mkShell {
    buildInputs = with pkgs; [
      pkg-config
      openssl
      zlib
      nodejs
    ];

    RUST_LOG = "info";
 }
