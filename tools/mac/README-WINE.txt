Wine, as bundled in MapleCW.app
================================

MapleCW.app runs its Windows programs with Wine, which is inside this folder. MapleCW did not
write Wine and did not modify it.

What it is
  An unmodified binary build of WineHQ's stable branch, Wine 11.0, for macOS (x86_64; runs under
  Rosetta 2 on Apple Silicon), made by Gcenx:
      https://github.com/Gcenx/macOS_Wine_builds/releases/tag/11.0_1
      file   wine-stable-11.0_1-osx64.tar.xz
      sha256 b50dc50ec7f41d58b115a6b685d4d1315ba3c797bd3aa0f49213f2703cb82388
  One thing was left out to save space: share/wine/mono (Wine Mono, the .NET runtime). Nothing
  else was added, removed or changed.

Licences
  Wine is free software under the GNU Lesser General Public License, version 2.1 or later:
      https://www.gnu.org/licenses/old-licenses/lgpl-2.1.html
  Its source code is published by the Wine project, and the exact build scripts by Gcenx:
      https://gitlab.winehq.org/wine/wine          (tag wine-11.0)
      https://github.com/Gcenx/macOS_Wine_builds   (how this binary was built)
  The libraries in lib/ that Wine uses (MoltenVK, GnuTLS, FreeType, SDL2, ICU, libpng, zlib and
  others) are each under their own open-source licences, published with their sources by their
  projects; Gcenx's repository lists what each build contains.

DXVK, as bundled in MapleCW.app (Contents/Resources/dxvk)
  The game draws with Direct3D 11 at feature level 11_0, which Wine's own Direct3D cannot give
  on a Mac, so MapleStory.exe (and only it) uses DXVK instead: Gcenx's macOS build of DXVK
  1.10.3, unmodified, which translates Direct3D 11 to Vulkan on MoltenVK.
      https://github.com/Gcenx/DXVK-macOS/releases/tag/v1.10.3-20230507-repack
      file   dxvk-macOS-async-v1.10.3-20230507-repack.tar.gz  (x64/d3d11.dll, x64/d3d10core.dll)
      sha256 acd1520ad105d8ef124a09c8e11a259a5dc8bdc565ad18e0e52693f9807b2477
  DXVK is under the zlib/libpng licence; source: https://github.com/doitsujin/dxvk and the
  macOS changes at https://github.com/Gcenx/DXVK-macOS.

  You may replace this Wine with another build of the same or a later version: put it at
  MapleCW.app/Contents/Resources/wine, or point MapleCW at one with
      launchctl setenv MAPLECW_WINE /path/to/bin/wine
