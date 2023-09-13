# Cnblogs' command line tool

[![Build / Release](https://github.com/cnblogs/cli/actions/workflows/build-release.yml/badge.svg)](https://github.com/cnblogs/cli/actions/workflows/build-release.yml)
[![Build / Development](https://github.com/cnblogs/cli/actions/workflows/build-dev.yml/badge.svg)](https://github.com/cnblogs/cli/actions/workflows/build-dev.yml)

Access cnblogs form CLI.

## Usage

To use `cnb` directly, register it to your environment variables is required.

### Login

You need to get your PAT from [https://account.cnblogs.com/settings/tokens](https://account.cnblogs.com/settings/tokens) to use this tool.

Then run `cnb user --login 'YOUR_PAT_HERE'`. This will save your PAT to `~/.cnbrc`.

If you want to logout, run `cnb user --logout` or just remove `~/.cnbrc`.

### Examples

After login, it's time to enjoy cnblogs.

Here are some simple examples:

```shell
# Check your post list
cnb post --list
# Check your post 
cnb --id 114514 post --show
# Create and publish post 
cnb post create --title 'Hello' --body 'world!' --publish
# Change your post body
cnb --id 114514 post update --body 'niconiconiconi'

# Show ing list
cnb ing list
# Publish ing 
cnb ing --publish 'Hello world!'
# Comment to ing 
cnb --id 114514 ing --comment 'Awesome!'

# Check your user infomation
cnb user --info
```

For more information, try `cnb --help`.

## Installation

### Build

This tool needs nightly toolchains(1.74.0+) to build.

```shell
git clone --depth 1 https://github.com/cnblogs/cli.git
cd cli
cargo build -r
```

Or get binaries from [CI](https://github.com/cnblogs/cli/actions) artifacts.

### License

[MIT](https://raw.githubusercontent.com/cnblogs/cli/main/LICENSE)

### Feedback

We’d love to hear your thoughts on this project. Feel free to drop us a note!

[Issues](https://github.com/cnblogs/cli/issues)
