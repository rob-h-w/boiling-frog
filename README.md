## Building

Boiling Frog is a simple Gnome UI in GTK4 to display the maximum temperature and fan speed for a
laptop/desktop. It relies
on [Alex Murray's Indicator Sensors](https://github.com/alexmurray/indicator-sensors) to retrieve
the data it displays.

The app is built purely using Rust code & the Rust toolchain. Installation and other utillities
are handled by a `Makefile`.

### Debug Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

Alternatively,

```bash
make build
```

can be used to generate both debug and release builds.

### Un/Installation

The app can be installed once built via `make`. `make` must be run as a superuser to ensure that
files get copied to the correct locations.

```bash
make install
```

To uninstall, again, as root, run:

```bash
make uninstall
```

---
A note on provenance: This was the product of the author suffering from Covid 19 for 2 weeks, &
resolving to learn something about Rust and GTK4 and DBus when illness permitted. Support for
this app is likely to be nonexistent.

The reason for creating this is that it's more convenient on a multi-screen setup to run games
on the main display and to have any performance metrics on another display. Hardware Sensors
Indicator resides in the taskbar, so is always hidden when full-screen gaming.
