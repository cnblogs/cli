with import <nixpkgs> {};

let
  packages = with pkgs; [
    openssl_3
    pkg-config
  ];
in pkgs.mkShell {
  buildInputs = packages;

  shellHook = ''
    export LD_LIBRARY_PATH=${
      pkgs.lib.makeLibraryPath packages
    }:$LD_LIBRARY_PATH
  '';
}
