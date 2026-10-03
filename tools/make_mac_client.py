"""Build the Mac client: MapleCW.app around the same payload the Windows client ships.

    python tools/make_mac_client.py [--payload out/MapleCW] [--out out/MapleCW-setup-mac.zip]
    python tools/make_mac_client.py --self-test

The payload is the staging directory `tools/make-installer.ps1 -ClientOnly` leaves behind:
`maplecw-launcher.exe`, `grap64.dll` and `client/`. The Mac client runs those exact files in
Wine (`docs/mac-client.md`), so there is nothing to build for the Mac - only an app bundle to
wrap them in. `make-installer.ps1 -ClientOnly` calls this itself; running it by hand is for
re-wrapping a payload that is already staged.

**Why Python and not the installer's own PowerShell.** A `.app` is useless unless its
`Contents/MacOS/MapleCW` is executable, and a zip written on Windows carries no Unix mode
bits: macOS's Archive Utility would unpack a script that cannot run, and the app would
bounce in the Dock and say nothing. `zipfile` lets each entry say "Unix, 0755". The same
reason the script's line endings are normalised here: a `#!/bin/bash\r` is "bad interpreter".

The zip holds **only the app** - the owner, 2026-10-02: *"setup-mac which is just the .app file
inside"*:

    MapleCW.app/Contents/Info.plist
    MapleCW.app/Contents/MacOS/MapleCW          tools/mac/MapleCW, mode 0755, LF
    MapleCW.app/Contents/Resources/README-mac.txt
    MapleCW.app/Contents/Resources/payload/...  the -ClientOnly payload, unchanged
    MapleCW.app/Contents/Resources/payload/.payload-stamp

The player-facing instructions (the Gatekeeper step above all) therefore travel with the app,
not beside it: whoever hands out the zip has to pass them on. `docs/mac-client.md`.

`.payload-stamp` is what the app compares to decide whether to (re)install the payload into
`~/Library/Application Support/MapleCW/MapleCW` - the digests of the launcher, the stub and the
client executable, so a new zip installs and an unchanged one does not.

Standard library only.
"""

import argparse
import hashlib
import os
import plistlib
import sys
import tarfile
import tempfile
import time
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(HERE)
MAC = os.path.join(HERE, "mac")

APP = "MapleCW.app"
EXEC_MODE = 0o755
FILE_MODE = 0o644
DIR_MODE = 0o755
# Formats that are already compressed. NOT .wz: the first real build stored them and the Mac
# zip came out 836 MB against the Windows zip's 563 MB from the same payload.
STORE_EXT = {".jpg", ".png", ".mp3", ".ogg", ".zip"}

# **The Wine inside the app** - the owner, 2026-10-02: *"make it so that the MapleCW.app doesn't
# need the original MapleStory Launcher.app"*, with an open-source build, always used. Gcenx's
# macOS build of WineHQ stable, unmodified except that Wine Mono is left out (236 MB of .NET that
# neither the launcher nor the client uses). It is downloaded once onto the packaging machine -
# never committed - and pinned by digest, so a different file is refused rather than shipped.
WINE_URL = ("https://github.com/Gcenx/macOS_Wine_builds/releases/download/11.0_1/"
            "wine-stable-11.0_1-osx64.tar.xz")
WINE_ARCHIVE = os.path.join(REPO, "out", "wine", "wine-stable-11.0_1-osx64.tar.xz")
WINE_SHA256 = "b50dc50ec7f41d58b115a6b685d4d1315ba3c797bd3aa0f49213f2703cb82388"
WINE_ROOT = "Wine Stable.app/Contents/Resources/wine/"
WINE_SKIP = ("share/wine/mono/",)
LINK_MODE = 0o120777

# **Direct3D 11 for the client: DXVK-macOS**, because Wine's own wined3d cannot give the client
# the one feature level it asks for (11_0, Shader Model 5.0 throughout) on a Mac - measured on the
# owner's Mac, 2026-10-02, docs/mac-client.md. Gcenx's macOS fork of DXVK 1.10.3, the "repack"
# that pairs with Wine's builtin dxgi. Only the 64-bit d3d11 and d3d10core go in; the client is
# 64-bit. Pinned like the Wine build.
DXVK_URL = ("https://github.com/Gcenx/DXVK-macOS/releases/download/v1.10.3-20230507-repack/"
            "dxvk-macOS-async-v1.10.3-20230507-repack.tar.gz")
DXVK_ARCHIVE = os.path.join(REPO, "out", "wine", "dxvk-macOS-async-v1.10.3-20230507-repack.tar.gz")
DXVK_SHA256 = "acd1520ad105d8ef124a09c8e11a259a5dc8bdc565ad18e0e52693f9807b2477"
DXVK_FILES = ("x64/d3d11.dll", "x64/d3d10core.dll")

REQUIRED = ["maplecw-launcher.exe", "grap64.dll"]

# `launcher::selfupdate::MAC_CAPABLE`. A launcher without it is one built before the Mac client
# worked: its window cannot open under Wine, and a Mac launcher that has it refuses to update to
# one that does not. So a Mac zip with an unmarked launcher is refused here, not on a player's
# Mac. This check is what caught the first release build of the guard missing its own mark.
MAC_MARK = b"maplecw-launcher/mac-capable/v1"

# **The app's icon is the game's own, read from Nexon's Mac app at package time.** The owner,
# 2026-10-02: *"include an icon for the MapleCW.app similar to that originally exists."* Nexon's
# outer `MapleStory Launcher.app` wears the Nexon logo; the app a Mac player actually sees in the
# Dock while playing is the inner `MapleStory Classic World.app`, and its icon is the Orange
# Mushroom. That is the one taken. It is Nexon's art, so - like the client - it is never
# committed: `client-mac/` is gitignored and this reads it there (as a real `.app` directory, or
# as the zip-stored file the copy on the dev box is). No copy, no icon: the app still works and
# macOS draws its generic one, and the build says so.
NEXON_APP = os.path.join(REPO, "client-mac", "MapleStory Launcher.app")
GAME_ICON = ("Contents/SharedSupport/maplestoryna/MapleStory Classic World.app/"
             "Contents/Resources/CrossOverOEMHelper.icns")
ICON_NAME = "MapleCW.icns"


def find_icon(explicit=None):
    """(bytes, where-from) for the app icon, or (None, why-not)."""
    if explicit:
        data = open(explicit, "rb").read()
        src = explicit
    elif os.path.isdir(NEXON_APP):
        p = os.path.join(NEXON_APP, *GAME_ICON.split("/"))
        if not os.path.isfile(p):
            return None, f"{p} is missing"
        data, src = open(p, "rb").read(), p
    elif os.path.isfile(NEXON_APP) and zipfile.is_zipfile(NEXON_APP):
        member = "MapleStory Launcher.app/" + GAME_ICON
        with zipfile.ZipFile(NEXON_APP) as z:
            if member not in z.namelist():
                return None, f"{NEXON_APP} has no {GAME_ICON}"
            data = z.read(member)
        src = f"{NEXON_APP} (zip) -> {GAME_ICON}"
    else:
        return None, f"no {NEXON_APP} on this machine"
    if data[:4] != b"icns":
        return None, f"{src} is not an .icns file"
    return data, src
CLIENT_EXE = os.path.join("client", "MapleStory.exe")


def workspace_version():
    with open(os.path.join(REPO, "Cargo.toml"), encoding="utf-8") as f:
        in_ws = False
        for line in f:
            s = line.strip()
            if s.startswith("["):
                in_ws = s == "[workspace.package]"
            elif in_ws and s.startswith("version"):
                return s.split("=", 1)[1].strip().strip('"')
    return "0.0.0"


def sha256_16(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()[:16]


def stamp_for(payload):
    """One line per file whose change should reinstall the payload on a Mac."""
    lines = []
    for rel in REQUIRED + [CLIENT_EXE]:
        p = os.path.join(payload, rel)
        if os.path.isfile(p):
            lines.append(f"{sha256_16(p)}  {rel.replace(os.sep, '/')}")
    return "\n".join(lines) + "\n"


def lf(data):
    return data.replace(b"\r\n", b"\n")


def add_bytes(z, name, data, mode):
    info = zipfile.ZipInfo(name, date_time=time.localtime(time.time())[:6])
    info.create_system = 3  # Unix, so external_attr's high half is read as a mode
    info.external_attr = (0o100000 | mode) << 16
    info.compress_type = zipfile.ZIP_DEFLATED
    z.writestr(info, data)


def add_dir(z, name):
    info = zipfile.ZipInfo(name.rstrip("/") + "/", date_time=time.localtime(time.time())[:6])
    info.create_system = 3
    info.external_attr = ((0o040000 | DIR_MODE) << 16) | 0x10
    z.writestr(info, b"")


def add_file(z, name, path):
    info = zipfile.ZipInfo.from_file(path, name)
    info.create_system = 3
    info.external_attr = (0o100000 | FILE_MODE) << 16
    ext = os.path.splitext(path)[1].lower()
    info.compress_type = zipfile.ZIP_STORED if ext in STORE_EXT else zipfile.ZIP_DEFLATED
    with open(path, "rb") as src, z.open(info, "w") as dst:
        for chunk in iter(lambda: src.read(1 << 20), b""):
            dst.write(chunk)


def add_link(z, name, target):
    """A symbolic link, the way macOS's unzip and Archive Utility restore one."""
    info = zipfile.ZipInfo(name, date_time=time.localtime(time.time())[:6])
    info.create_system = 3
    info.external_attr = LINK_MODE << 16
    info.compress_type = zipfile.ZIP_STORED
    z.writestr(info, target.encode("utf-8"))


def file_sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def add_wine(z, archive, expect_sha256):
    """Stream the Wine build into Contents/Resources/wine. Returns (files, links, bytes)."""
    if not os.path.isfile(archive):
        sys.exit(f"no Wine build at {archive}. Download it once (it is not committed):\n"
                 f"  {WINE_URL}\n  into {os.path.dirname(archive)}  (sha256 {WINE_SHA256})")
    if expect_sha256 and file_sha256(archive) != expect_sha256:
        sys.exit(f"{archive} is not the pinned Wine build (sha256 should be {expect_sha256})")
    base = f"{APP}/Contents/Resources/wine"
    files = links = size = 0
    with tarfile.open(archive, "r:xz") as t:
        for m in t:
            if not m.name.startswith(WINE_ROOT):
                continue
            rel = m.name[len(WINE_ROOT):]
            if not rel or any((rel + "/").startswith(s) for s in WINE_SKIP):
                continue
            name = f"{base}/{rel}"
            if m.isdir():
                add_dir(z, name)
            elif m.issym():
                add_link(z, name, m.linkname)
                links += 1
            elif m.isfile():
                info = zipfile.ZipInfo(name, date_time=time.localtime(max(m.mtime, 315619200))[:6])  # zip has no time before 1980
                info.create_system = 3
                info.external_attr = (0o100000 | (EXEC_MODE if m.mode & 0o111 else FILE_MODE)) << 16
                info.compress_type = zipfile.ZIP_DEFLATED
                with t.extractfile(m) as src, z.open(info, "w", force_zip64=True) as dst:
                    for chunk in iter(lambda: src.read(1 << 20), b""):
                        dst.write(chunk)
                files += 1
                size += m.size
            else:
                sys.exit(f"{archive}: {m.name} is neither a file, a directory nor a symlink")
    notice = lf(open(os.path.join(MAC, "README-WINE.txt"), "rb").read())
    add_bytes(z, f"{base}/README-WINE.txt", notice, FILE_MODE)
    if files == 0:
        sys.exit(f"{archive} has nothing under {WINE_ROOT} - not the expected Wine build")
    return files, links, size


def add_dxvk(z, archive, expect_sha256):
    """The DXVK DLLs into Contents/Resources/dxvk, plus the VERSION the app compares."""
    if not os.path.isfile(archive):
        sys.exit(f"no DXVK build at {archive}. Download it once (it is not committed):\n"
                 f"  {DXVK_URL}\n  into {os.path.dirname(archive)}  (sha256 {DXVK_SHA256})")
    if expect_sha256 and file_sha256(archive) != expect_sha256:
        sys.exit(f"{archive} is not the pinned DXVK build (sha256 should be {expect_sha256})")
    base = f"{APP}/Contents/Resources/dxvk"
    add_dir(z, base)
    found = {}
    with tarfile.open(archive, "r:gz") as t:
        for m in t:
            rel = m.name.split("/", 1)[1] if "/" in m.name else m.name
            if m.isfile() and rel in DXVK_FILES:
                found[rel] = t.extractfile(m).read()
    missing = [f for f in DXVK_FILES if f not in found]
    if missing:
        sys.exit(f"{archive} lacks {missing} - not the expected DXVK build")
    for rel, data in found.items():
        add_bytes(z, f"{base}/{rel.split('/')[-1]}", data, FILE_MODE)
    version = os.path.basename(archive).replace(".tar.gz", "")
    digest = hashlib.sha256(b"".join(found[f] for f in DXVK_FILES)).hexdigest()[:16]
    add_bytes(z, f"{base}/VERSION", f"{version} {digest}\n".encode(), FILE_MODE)
    return version


def build(payload, out, require_client=True, icon=None, use_nexon_icon=True,
          wine=WINE_ARCHIVE, wine_sha256=WINE_SHA256, dxvk=DXVK_ARCHIVE, dxvk_sha256=DXVK_SHA256):
    for rel in REQUIRED:
        if not os.path.isfile(os.path.join(payload, rel)):
            sys.exit(f"{payload} has no {rel} - stage it with tools/make-installer.ps1 -ClientOnly first")
    with open(os.path.join(payload, "maplecw-launcher.exe"), "rb") as f:
        if MAC_MARK not in f.read():
            sys.exit(f"{payload}{os.sep}maplecw-launcher.exe has no Mac mark ({MAC_MARK.decode()}) - it cannot "
                     "run on a Mac or predates the guard. Rebuild it (make-installer.ps1 without -SkipBuild).")
    if require_client and not os.path.isfile(os.path.join(payload, CLIENT_EXE)):
        sys.exit(f"{payload} has no {CLIENT_EXE} - the client is the point of the payload")
    # A Windows-only payload has server binaries in bin\; a Mac player has no use for them and
    # they cannot run there. Refuse rather than ship 30 MB of the wrong thing.
    if os.path.isdir(os.path.join(payload, "bin")):
        sys.exit(f"{payload} has a bin\\ - that is the full server payload. Use the -ClientOnly one.")

    version = workspace_version()
    build_id = time.strftime("%Y%m%d%H%M%S", time.gmtime())
    plist = open(os.path.join(MAC, "Info.plist"), "rb").read().decode("utf-8")
    plist = plist.replace("@VERSION@", version).replace("@BUILD@", build_id)
    plistlib.loads(plist.encode("utf-8"))  # refuse to ship a plist macOS cannot read
    script = lf(open(os.path.join(MAC, "MapleCW"), "rb").read())
    if not script.startswith(b"#!/bin/bash\n"):
        sys.exit("tools/mac/MapleCW must start with #!/bin/bash")
    readme = lf(open(os.path.join(MAC, "README-mac.txt"), "rb").read())
    icon_data, icon_src = find_icon(icon) if (icon or use_nexon_icon) else (None, "not asked for")
    stamp = stamp_for(payload)

    os.makedirs(os.path.dirname(os.path.abspath(out)), exist_ok=True)
    tmp = out + ".partial"
    files = 0
    with zipfile.ZipFile(tmp, "w", allowZip64=True) as z:
        for d in (APP, f"{APP}/Contents", f"{APP}/Contents/MacOS", f"{APP}/Contents/Resources",
                  f"{APP}/Contents/Resources/payload"):
            add_dir(z, d)
        add_bytes(z, f"{APP}/Contents/Info.plist", plist.encode("utf-8"), FILE_MODE)
        add_bytes(z, f"{APP}/Contents/PkgInfo", b"APPL????", FILE_MODE)
        if icon_data:
            add_bytes(z, f"{APP}/Contents/Resources/{ICON_NAME}", icon_data, FILE_MODE)
        add_bytes(z, f"{APP}/Contents/MacOS/MapleCW", script, EXEC_MODE)
        add_bytes(z, f"{APP}/Contents/Resources/payload/.payload-stamp", stamp.encode(), FILE_MODE)
        add_bytes(z, f"{APP}/Contents/Resources/README-mac.txt", readme, FILE_MODE)
        for root, dirs, names in os.walk(payload):
            dirs.sort()
            rel_root = os.path.relpath(root, payload).replace(os.sep, "/")
            base = f"{APP}/Contents/Resources/payload" + ("" if rel_root == "." else "/" + rel_root)
            if rel_root != ".":
                add_dir(z, base)
            for n in sorted(names):
                if n == ".payload-stamp":
                    continue
                add_file(z, f"{base}/{n}", os.path.join(root, n))
                files += 1
        add_dir(z, f"{APP}/Contents/Resources/wine")
        wine_files, wine_links, wine_bytes = add_wine(z, wine, wine_sha256)
        dxvk_version = add_dxvk(z, dxvk, dxvk_sha256)
    os.replace(tmp, out)
    print(f"wrote {out}  ({os.path.getsize(out) / (1 << 20):,.0f} MB, {files} payload files)")
    print(f"  MapleCW.app {version} build {build_id}")
    print(f"  Wine: {os.path.basename(wine)} - {wine_files} files, {wine_links} links, "
          f"{wine_bytes / (1 << 20):,.0f} MB unpacked (Wine Mono left out)")
    print(f"  DXVK: {dxvk_version} (d3d11 + d3d10core, x64) for MapleStory.exe")
    if icon_data:
        print(f"  icon: {icon_src}")
    else:
        print(f"  NO icon ({icon_src}) - macOS will draw its generic app icon")
    for line in stamp.strip().splitlines():
        print(f"  {line}")
    return out


def check(out):
    """What a Mac will see: the modes, the plist, the script. Raises on any mistake."""
    with zipfile.ZipFile(out) as z:
        names = set(z.namelist())
        exe = z.getinfo(f"{APP}/Contents/MacOS/MapleCW")
        assert exe.create_system == 3, "the script entry is not marked Unix"
        assert (exe.external_attr >> 16) & 0o777 == EXEC_MODE, oct(exe.external_attr >> 16)
        assert b"\r" not in z.read(exe), "CRLF in the launch script"
        plist = plistlib.loads(z.read(f"{APP}/Contents/Info.plist"))
        assert plist["CFBundleExecutable"] == "MapleCW", plist
        icon = f"{APP}/Contents/Resources/{ICON_NAME}"
        if icon in names:
            assert plist.get("CFBundleIconFile") == ICON_NAME.rsplit(".", 1)[0], plist
            assert z.read(icon)[:4] == b"icns", "the icon is not an .icns file"
        assert f"{APP}/Contents/Resources/payload/maplecw-launcher.exe" in names
        assert f"{APP}/Contents/Resources/payload/grap64.dll" in names
        assert f"{APP}/Contents/Resources/payload/.payload-stamp" in names
        assert f"{APP}/Contents/Resources/README-mac.txt" in names
        wine = z.getinfo(f"{APP}/Contents/Resources/wine/bin/wine")
        assert (wine.external_attr >> 16) & 0o777 == EXEC_MODE, "bundled bin/wine is not executable"
        assert f"{APP}/Contents/Resources/wine/README-WINE.txt" in names, "Wine's licence notice is missing"
        assert not any("/share/wine/mono/" in n for n in names), "Wine Mono was meant to be left out"
        for f in ("d3d11.dll", "d3d10core.dll", "VERSION"):
            assert f"{APP}/Contents/Resources/dxvk/{f}" in names, f"DXVK's {f} is missing"
        assert z.read(f"{APP}/Contents/Resources/dxvk/d3d11.dll")[:2] == b"MZ", "DXVK d3d11.dll is not a PE"
        for info in z.infolist():
            if (info.external_attr >> 16) & 0o170000 == 0o120000:
                target = z.read(info).decode()
                assert target and not target.startswith("/"), f"{info.filename} -> {target!r}"
        stray = [n for n in names if not n.startswith(f"{APP}/")]
        assert not stray, f"the zip must hold only {APP}: {stray}"
        for info in z.infolist():
            if info.filename.endswith("/"):
                assert (info.external_attr >> 16) & 0o777 == DIR_MODE, info.filename
    return True


def self_test():
    """Build from a fake payload and check it - the packager's own positive control."""
    with tempfile.TemporaryDirectory() as t:
        payload = os.path.join(t, "MapleCW")
        os.makedirs(os.path.join(payload, "client", "Data"))
        for rel, data in [("maplecw-launcher.exe", b"MZlauncher " + MAC_MARK), ("grap64.dll", b"MZstub"),
                          (CLIENT_EXE, b"MZclient"), (os.path.join("client", "Data", "Base.wz"), b"wz")]:
            with open(os.path.join(payload, rel), "wb") as f:
                f.write(data)
        # A tiny stand-in for the Wine build: an executable, a library, a symlink to it, and a
        # Mono file that must be left out.
        fake_wine = os.path.join(t, "wine.tar.xz")
        with tarfile.open(fake_wine, "w:xz") as tw:
            def put(name, data=b"", mode=0o644, link=None):
                ti = tarfile.TarInfo(WINE_ROOT + name)
                if link:
                    ti.type, ti.linkname = tarfile.SYMTYPE, link
                else:
                    ti.size, ti.mode = len(data), mode
                tw.addfile(ti, None if link else __import__("io").BytesIO(data))
            put("bin/wine", b"\xcf\xfa\xed\xfe loader", 0o755)
            put("lib/libz.1.dylib", b"dylib", 0o755)
            put("lib/libz.dylib", link="libz.1.dylib")
            put("share/wine/mono/wine-mono.msi", b"mono")
        fake_dxvk = os.path.join(t, "dxvk-test.tar.gz")
        with tarfile.open(fake_dxvk, "w:gz") as td:
            for name in ("dxvk-test/x64/d3d11.dll", "dxvk-test/x64/d3d10core.dll", "dxvk-test/x32/d3d11.dll"):
                ti = tarfile.TarInfo(name)
                ti.size = 4
                td.addfile(ti, __import__("io").BytesIO(b"MZ\x90\x00"))
        fake_icon = os.path.join(t, "fake.icns")
        with open(fake_icon, "wb") as f:
            f.write(b"icns\x00\x00\x00\x08")
        out = build(payload, os.path.join(t, "MapleCW-setup-mac.zip"), icon=fake_icon,
                    wine=fake_wine, wine_sha256=None, dxvk=fake_dxvk, dxvk_sha256=None)
        check(out)
        with zipfile.ZipFile(out) as z:
            stamp = z.read(f"{APP}/Contents/Resources/payload/.payload-stamp").decode()
            assert "client/MapleStory.exe" in stamp, stamp
            assert f"{APP}/Contents/Resources/payload/client/Data/Base.wz" in z.namelist()
            assert z.read(f"{APP}/Contents/Resources/{ICON_NAME}")[:4] == b"icns"
            link = z.getinfo(f"{APP}/Contents/Resources/wine/lib/libz.dylib")
            assert (link.external_attr >> 16) == LINK_MODE, oct(link.external_attr >> 16)
            assert z.read(link) == b"libz.1.dylib"
            assert f"{APP}/Contents/Resources/dxvk/d3d11.dll" in z.namelist()
            assert not any("x32" in n for n in z.namelist()), "only the 64-bit DXVK belongs in the app"
            lib = z.getinfo(f"{APP}/Contents/Resources/wine/lib/libz.1.dylib")
            assert (lib.external_attr >> 16) & 0o777 == EXEC_MODE
        # A launcher without the Mac mark must be refused, not wrapped.
        with open(os.path.join(payload, "maplecw-launcher.exe"), "wb") as f:
            f.write(b"MZ an old launcher")
        try:
            build(payload, os.path.join(t, "unmarked.zip"), wine=fake_wine, wine_sha256=None,
                  dxvk=fake_dxvk, dxvk_sha256=None)
        except SystemExit as e:
            assert "Mac mark" in str(e), e
        else:
            raise AssertionError("a launcher without the Mac mark was wrapped")
        with open(os.path.join(payload, "maplecw-launcher.exe"), "wb") as f:
            f.write(b"MZlauncher " + MAC_MARK)
        # A server payload must be refused, not wrapped.
        os.makedirs(os.path.join(payload, "bin"))
        try:
            build(payload, os.path.join(t, "refused.zip"), wine=fake_wine, wine_sha256=None,
                  dxvk=fake_dxvk, dxvk_sha256=None)
        except SystemExit as e:
            assert "bin" in str(e), e
        else:
            raise AssertionError("a payload with bin\\ was wrapped")
    print("self-test passed")


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--payload", default=os.path.join(REPO, "out", "MapleCW"))
    ap.add_argument("--out", default=os.path.join(REPO, "out", "MapleCW-setup-mac.zip"))
    ap.add_argument("--icon", help="an .icns to use instead of the one in client-mac/")
    ap.add_argument("--no-icon", action="store_true", help="build without an icon")
    ap.add_argument("--no-client", action="store_true",
                    help="allow a payload without client/ (the installer's -NoClient dry run)")
    ap.add_argument("--wine", default=WINE_ARCHIVE, help="the pinned Wine build (.tar.xz)")
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    if a.self_test:
        self_test()
        return
    out = build(a.payload, a.out, require_client=not a.no_client, icon=a.icon,
                use_nexon_icon=not a.no_icon, wine=a.wine)
    check(out)
    print("checked: the app's executable and bundled Wine are Unix 0755, LF endings, symlinks relative, "
          "plist parses, no Wine Mono")


if __name__ == "__main__":
    main()
