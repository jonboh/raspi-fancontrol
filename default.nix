{rustPlatform}:
rustPlatform.buildRustPackage rec {
  pname = "rp-fancontrol";
  version = "v0.1.0";

  src = ./.;

  cargoHash = "sha256-0RbO+vT94jK4oiZnDJaBwCyfTg60iaTkLxMsLv+wbQo=";

  # meta = with lib; {
  #   description = "A shell AI assistant";
  #   homepage = "https://github.com/jonboh/shai";
  #   license = licenses.mit;
  #   maintainers = ["jonboh"];
  # };
}
