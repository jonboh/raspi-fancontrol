# RP Fan Control

A simple temperature-based fan control utility specifically tailored for Raspberry Pi that regulates the fan speed using software Pulse-Width Modulation (PWM).

## Features
- Reads CPU temperature at configurable intervals.
- Adjusts fan PWM based on a specified temperature-to-PWM mapping curve.

## Requirements

- Rust and Cargo (Rust's package manager) should be installed on your Raspberry Pi.
- Any additional dependencies are managed with Cargo and specified in the `Cargo.toml` file.

## Usage

Compile and run the program with the following commands:

```sh
cargo build --release
./target/release/rp_fancontrol [OPTIONS]
```

### Command Line Options

Below are the available command line options for customizing the behavior of the fan control:

- `--temp_file` - Path to the file from which to read the temperature (default: `/sys/class/thermal/thermal_zone0/temp`).
- `-i`, `--interval` - Interval, in milliseconds, for re-evaluating fan power (default: `5000`).
- `-t`, `--temp` - Temperature value in the temp-to-PWM curve, use multiple times for different points (no default, must be specified).
- `-p`, `--pwm` - PWM value corresponding to the `--temp` values in the temp-to-PWM curve, use multiple times for different points (no default, must be specified).
- `--pwm_channel` - PWM channel to which the fan is connected (default: `2` (for`PWM2 == GPIO18`); other options are `0`, `1`, `3`).

## Configuration

Use `--temp` and `--pwm` to define the points in the temperature to PWM conversion curve. The number of `--temp` and `--pwm` points must agree and temperatures should be in increasing order, whereas PWM values should be non-decreasing.

Example command using configuration options:

```sh
./target/release/rp_fancontrol --interval 5000 --temp 40.0 --temp 60.0 --pwm 0.0 --pwm 1.0 --pwm_channel 2
```

This command sets the program to check temperature every 5000 milliseconds and to control the fan using PWM based on the defined temp-to-PWM curve.

## As a systemd unit in NixOS
I use this in a systemd unit in NixOS:
```nix
{
  systemd.services.rp-fancontrol = {
    enable = true;
    description = "RPi GPIO fan control service";
    after = ["multi-user.target"];
    wantedBy = ["multi-user.target"];
    serviceConfig = {
      ExecStart = "${pkgs.rp-fancontrol}/bin/rp-fancontrol --temp 40 --pwm 0 --temp 50 --pwm 0.5 --temp 60 --pwm 0.7 --temp 70 --pwm 1";
      Type = "simple";
      Restart = "always";
      RestartSec = "5";
    };
  };
}
```
To consume the package from the flake use:
```nix
{
  inputs = {
    # ... your other inputs
   raspberry-pi-nix.url = "github:nix-community/raspberry-pi-nix";
   raspi-fancontrol = {
      url = "github:jonboh/raspi-fancontrol";
    };
  };
  outputs = {self, ...}@inputs: {
      nixosConfigurations = {
        "brick" = inputs.nixpkgs.lib.nixosSystem {
          system = "aarch64-linux";
          specialArgs = {inherit self;};
          pkgs = import inputs.nixpkgs {
            system = "aarch64-linux";
            overlays = [
              (final: prev: {
                # here you add raspi-fancontrol to your pkgs
                raspi-fancontrol = self.inputs.raspi-fancontrol.packages.aarch64-linux.default;
              })
            ];
          };
          modules = [
            inputs.raspberry-pi-nix.nixosModules.raspberry-pi
            inputs.raspberry-pi-nix.nixosModules.sd-image
            ./configuration.nix
          ];
        };
      }
    };
}
```

In addition in your `configuration.nix` you need to configure the PWM pins:
```nix
{
  hardware.raspberry-pi.config = {
    # see: https://github.com/raspberrypi/firmware/blob/master/boot/overlays/README#L3880
    pi5.dt-overlays = {
      "pwm,pin=12,func=4" = {
        enable = true;
        params = {};
      };
      "pwm,pin=18,func=2" = {
        enable = true;
        params = {};
      };
    };
  };
}
```


