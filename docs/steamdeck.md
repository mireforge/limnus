# Development on `SteamDeck` itself

## Allow to write to the file system (disable readonly)

```sh
sudo steamos-readonly disable
```

## Ensure that pacman works (needs keys)

```sh
sudo pacman-key --init
sudo pacman-key --populate archlinux
sudo pacman-key --populate holo
```

## Install development dependencies

```sh
sudo pacman -S base-devel -y
sudo pacman -S libudev -y
sudo pacman -S systemd-libs
```
