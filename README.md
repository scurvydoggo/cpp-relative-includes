# lyra-plugin-tool

Using [Lyra](https://x157.github.io/UE5/LyraStarterGame/) I'd like to depend on
`Source/` from my plugin, however this isn't possible since it isn't
pluginified.

This tool will migrate content of `Source/` into two plugins: `LyraGame` and
`LyraEditor`.

This was necessary because the way that `#include` directives are defined isn't
compatible and so a rewrite is required (from absolute to relative paths).

## Usage

TBD
