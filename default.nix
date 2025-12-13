{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage rec {
  pname = "rp-fancontrol";
  version = "v0.1.0";

  src = ./.;

  cargoHash = "sha256-zgxP4Che6yPgQY/Lp657RjPYG4jTqijkIzkVzALuUkU=";

  meta = with lib; {
    description = "A controller for raspberry fans connected to the PWM GPIO pins";
    homepage = "https://github.com/jonboh/raspi-fancontrol";
    license = licenses.mit;
    maintainers = ["jonboh"];
  };
}
