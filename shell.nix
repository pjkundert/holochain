{ pkgs ? import <nixpkgs> {}, ... } @ args:

let
  default = import (builtins.toString ./default.nix) args;
in

pkgs.stdenv.mkDerivation rec {
  name = "coredev-env";
  buildInputs = with pkgs; [
    rustup
    pkgconfig
    kcov
    lldb

    openssh_with_kerberos
    zsh
    jq
    shellcheck
    ripgrep
    bashInteractive

    openssl.dev
    zlib

    openshift
  ];

  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.zlib}/lib
  '';
}
