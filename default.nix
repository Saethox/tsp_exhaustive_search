with import (builtins.fetchGit {
  name = "nixpkgs-2020-11-11";
  url = "https://github.com/NixOS/nixpkgs/";
  rev = "1c203a232544018680b757443083262a1bff8f20";
}) { config = { allowUnfree = true; }; };


mkShell {
  name = "beste-shell";
  venvDir = "./_venv";
  nativeBuildInputs = ([ rustc cargo gcc ]);
  postShellHook = ''
    unset SOURCE_DATE_EPOCH
    export LD_LIBRARY_PATH=${stdenv.cc.cc.lib}/lib:$LD_LIBRARY_PATH
    NIX_ENFORCE_PURITY=0
  '';
}
