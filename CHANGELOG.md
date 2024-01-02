# Changelog

## 0.7.0

### Added

- `serde` feature

## 0.6.0

### Added

- Many missing keys
- Check if ScrollLock is toggled
- `bind_all`
- `MousewheelUp` and `MousewheelDown`
- `KeybdKey::is_bound` and `MouseButton::is_bound`

## Changed

- `get_keybd_key` is now public

### Fixed

- Mouse cursor move on Linux
- Use Wayland for mouse press and release on Linux
- Support checks for whether is pressed on Linux with Wayland
- Synchronize device after use
- Improved examples
- Handling of Alt keys
- Updated dependencies
- Improved comments
- Reduced unsafe usage
- Various refactorings

## 0.5.0

### Added

- Blockable binds (optionally hide events from system)
- Block binds (hide events from system)
- Function to get current mouse cursor position
- License file

### Changed

- [Mouse cursor move functions behave correctly now](https://github.com/obv-mikhail/InputBot/pull/22)
- Mouse cursor methods changed to not take self
- Rust 2018 edition

### Removed

- Lazystatic


## 0.4.0

### Added

- Support for sending key sequences

### Changed

- Winapi 3.0