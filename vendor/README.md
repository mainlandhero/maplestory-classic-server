# vendor/ - patched copies of third-party crates

Each crate here is an unmodified crates.io release except for the lines marked `MAPLECW PATCH`,
and is wired in through `[patch.crates-io]` in the workspace `Cargo.toml`. Remove a copy (and its
`[patch]` line) as soon as upstream makes the patch unnecessary.

## `glutin/` - glutin 0.32.3 (MIT OR Apache-2.0, see its `LICENSE`)

**Why:** the Mac client runs `maplecw-launcher.exe` in Wine (`docs/mac-client.md`). The first
launch on a Mac, 2026-10-02, died before the window existed:

```text
The launcher window could not be created.
glutin error: extension to create ES context with wgl is not present
```

That message is eframe's **second** attempt. Its first asks glutin for its default context -
core profile 3.3 with no flags - and `winemac.drv` refuses every 3.2+ context that is not
**forward-compatible**. The refusal is in the runtime itself: `lib/wine/x86_64-unix/winemac.so`
inside Nexon's `MapleStory Launcher.app` carries the strings *"OS X only supports
forward-compatible 3.2+ contexts"* and *"Compatibility profiles for GL version >= 3.2 not
supported"*. eframe then falls back to an OpenGL ES context, which Wine does not offer at all -
that is the error on screen.

**The patch** (`src/api/wgl/context.rs`, `create_context_arb`): when the context is core 3.2+
**and the process is running under Wine** (`ntdll!wine_get_version` exists), also set
`WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB`. A forward-compatible core context removes only what a
core context already lacks, and egui_glow uses none of it. The Wine gate means a Windows launcher
asks for byte-for-byte the same context as before the patch.

**Measured:** the strings above, read from the shipped `winemac.so`; that the patch builds and
the launcher's suite passes on Windows. **Not yet measured:** the window opening on a Mac - the
next Mac launch is the test, and the same dialog again would mean a second refusal behind this
one.
