# LocoDownloader

A CLI tool to download localization files from [Localise.biz](https://localise.biz) for iOS and Android projects.

## Installation

Download the latest release for your platform from the [Releases](../../releases) page.

### macOS (Apple Silicon)

```sh
curl -L https://github.com/<your-org>/LocoDownloader/releases/latest/download/loco-<VERSION>-macos-arm.tar.gz | tar xz
sudo sh install/install.sh
```

### Linux (x86-64)

```sh
curl -L https://github.com/<your-org>/LocoDownloader/releases/latest/download/loco-<VERSION>-linux-x86.tar.gz | tar xz
sudo sh install/install.sh
```

The installer copies the `loco` binary to `/usr/local/bin/loco`.

## Prerequisites

Set your Localise.biz API token as an environment variable:

```sh
export LOCO_TOKEN=your_token_here
```

## Usage

```sh
loco [--app <app>] [--lang <lang>] [--output-dir <dir>] [--help]
```

### Arguments

| Argument              | Default   | Accepted values        | Description                              |
|-----------------------|-----------|------------------------|------------------------------------------|
| `--app <app>`         | `iOS`     | `iOS`, `Android`       | Target platform                          |
| `--lang <lang>`       | `FR`      | `FR`, `ES`, `DE`, `BR` | Language to download                     |
| `--output-dir <dir>`  | `.`       | Any valid path         | Directory where the output file is saved |
| `--help`              | —         | —                      | Print usage and exit                     |

### Language codes

| Code | Language             |
|------|----------------------|
| `FR` | French               |
| `ES` | Spanish              |
| `DE` | German               |
| `BR` | Brazilian Portuguese |

### Output files

| Platform  | Output file           |
|-----------|-----------------------|
| `iOS`     | `Localizable.strings` |
| `Android` | `strings.xml`         |

## Examples

Download French localizations for iOS (defaults):

```sh
loco
```

Download Spanish localizations for Android into a specific directory:

```sh
loco --app Android --lang ES --output-dir ./res
```
