---
name: loco
version: 1.0.0
description: "Loco: Download localization files from Localise.biz."
metadata:
  openclaw:
    category: "localization"
    requires:
      bins: ["loco"]
    cliHelp: "loco --help"
---

# loco

Download localization files from [Localise.biz](https://localise.biz) for iOS or Android.

## Prerequisite

Set the `LOCO_TOKEN` environment variable before running any command:

```sh
export LOCO_TOKEN=your_token_here
```

## Usage

```sh
loco [--app <app>] [--lang <lang>] [--output-dir <dir>] [--help]
```

## Environment Variables

| Variable     | Required | Description                          |
|--------------|----------|--------------------------------------|
| `LOCO_TOKEN` | Yes      | Your Localise.biz API token          |

## Arguments

| Argument            | Default      | Accepted Values           | Description                                    |
|---------------------|--------------|---------------------------|------------------------------------------------|
| `--app <app>`       | `iOS`        | `iOS`, `Android`          | Target app platform                            |
| `--lang <lang>`     | `FR`         | `FR`, `ES`, `DE`, `BR`    | Language to download                           |
| `--output-dir <dir>`| `.` (current)| Any valid directory path  | Directory where the output file will be saved  |
| `--help`            | —            | —                         | Print usage information and exit               |

### Language codes

| Code | Language               |
|------|------------------------|
| `FR` | French                 |
| `ES` | Spanish                |
| `DE` | German                 |
| `BR` | Brazilian Portuguese   |

## Output Files

| App       | Output file            |
|-----------|------------------------|
| `iOS`     | `Localizable.strings`  |
| `Android` | `strings.xml`          |

## Examples

Download French localizations for iOS (default):

```sh
LOCO_TOKEN=abc123 loco
```

Download Spanish localizations for Android into a specific directory:

```sh
LOCO_TOKEN=abc123 loco --app Android --lang ES --output-dir ./res
```
