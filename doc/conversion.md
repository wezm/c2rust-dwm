# c2rust dwm conversion

Steps:

    bear make
    # Remove optimisation from compile_commands.json
    c2rust transpile --main dwm compile_commands.json

Then manually fix a couple of issues to get it to compile:

* <https://git.sr.ht/~wezm/dwm-rust/commit/54c2e33564dff00ce53b7d4b9f9ab2240cad53fe>
* <https://git.sr.ht/~wezm/dwm-rust/commit/2da35fbb128941676ce4ebb4779aa1f30af4e550>

Add libraries to build script:

* <https://git.sr.ht/~wezm/dwm-rust/commit/efb0eae1321a7b8e2c20bfff6b2c1ef490bb3179>

At this point it seems to run fine and this was the starting point for refactoring.
