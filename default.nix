{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage rec {
  pname = "rp-fancontrol";
  version = "v0.1.0";

  src = ./.;

  cargoHash = "sha256-0RbO+vT94jK4oiZnDJaBwCyfTg60iaTkLxMsLv+wbQo=";

  meta = with lib; {
    description = "A controller for raspberry fans connected to the PWM GPIO pins";
    homepage = "https://github.com/jonboh/raspi-fancontrol";
    license = licenses.mit;
    maintainers = ["jonboh"];
  };
}
