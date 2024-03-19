# cpp-relative-includes

Rewrites `#include`s to be relative rather than absolute, relative to the
project root.

Some build tools may require relative paths, which can be an issue when porting
code that uses absolute paths.

This is probably pretty niche. I needed this when porting the
[Lyra](https://x157.github.io/UE5/LyraStarterGame/) Unreal Engine project
`Source/` directory into a Game Feature Plugin.

## Usage

TBD
