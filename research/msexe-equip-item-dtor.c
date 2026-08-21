
//===========================================================
// FUN_1402fb5c0 @ 1402fb5c0   (99 bytes)
//===========================================================

longlong FUN_1402fb5c0(longlong param_1,ulonglong param_2)

{
  FUN_1401d5120(param_1 + 0x212);
  FUN_1401d5120(param_1 + 0x1d2);
  if (*(longlong *)(param_1 + 0x28) != 0) {
    thunk_FUN_140205820(*(longlong *)(param_1 + 0x28),0xc);
  }
  FUN_1401d5120(param_1);
  if ((param_2 & 1) != 0) {
    thunk_FUN_140205820(param_1,0x467);
  }
  return param_1;
}



//===========================================================
// FUN_1402cc180 @ 1402cc180   (229 bytes)
//===========================================================

longlong FUN_1402cc180(longlong param_1,int param_2)

{
  longlong lVar1;
  
  if (param_2 == 1) {
    lVar1 = FUN_14019b780(&DAT_143ad68a0,0x467);
    if (lVar1 == 0) {
      lVar1 = 0;
    }
    else {
      lVar1 = FUN_1402f7da0(lVar1);
    }
  }
  else if (param_2 == 2) {
    lVar1 = FUN_14019b780(&DAT_143ad68a0,0x7e);
    if (lVar1 == 0) {
      lVar1 = 0;
    }
    else {
      lVar1 = FUN_1402f7cd0(lVar1);
    }
  }
  else {
    if (param_2 != 3) {
      *(undefined8 *)(param_1 + 8) = 0;
      return param_1;
    }
    lVar1 = FUN_14019b780(&DAT_143ad68a0,0xce);
    if (lVar1 == 0) {
      lVar1 = 0;
    }
    else {
      lVar1 = FUN_1402f8b40(lVar1);
    }
  }
  *(longlong *)(param_1 + 8) = lVar1;
  if (lVar1 != 0) {
    if (0xfffff < *(ulonglong *)(lVar1 + 8)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar1 + 8) = *(longlong *)(lVar1 + 8) + 1;
    UNLOCK();
  }
  return param_1;
}



//===========================================================
// FUN_1402f7da0 @ 1402f7da0   (524 bytes)
//===========================================================

undefined8 * FUN_1402f7da0(undefined8 *param_1)

{
  byte bVar1;
  
  FUN_1402f7aa0();
  *param_1 = &PTR_FUN_14327e1d8;
  FUN_1402f7fb0((longlong)param_1 + 0x62);
  *(undefined8 *)((longlong)param_1 + 0x1ea) = 0;
  *(undefined8 *)((longlong)param_1 + 0x1da) = 0;
  *(undefined8 *)((longlong)param_1 + 0x1e2) = 0;
  *(undefined ***)((longlong)param_1 + 0x1d2) = &PTR_FUN_14327e1c8;
  *(undefined8 *)((longlong)param_1 + 0x1f2) = 0;
  *(undefined8 *)((longlong)param_1 + 0x206) = 0;
  *(undefined4 *)((longlong)param_1 + 0x20e) = 0;
  *(undefined8 *)((longlong)param_1 + 0x1fa) = DAT_14327dd80;
  *(undefined4 *)((longlong)param_1 + 0x202) = 0;
  *(undefined8 *)((longlong)param_1 + 0x22a) = 0;
  *(undefined8 *)((longlong)param_1 + 0x21a) = 0;
  *(undefined8 *)((longlong)param_1 + 0x222) = 0;
  *(undefined ***)((longlong)param_1 + 0x212) = &PTR_FUN_14327e1d0;
  *(undefined8 *)((longlong)param_1 + 0x232) = 0;
  *(undefined8 *)((longlong)param_1 + 0x23a) = 0;
  *(undefined8 *)((longlong)param_1 + 0x242) = 0;
  *(undefined8 *)((longlong)param_1 + 0x24a) = 0;
  *(undefined8 *)((longlong)param_1 + 0x252) = 0;
  *(undefined4 *)((longlong)param_1 + 0x25a) = 0;
  *(undefined1 *)((longlong)param_1 + 0x25e) = 0;
  *(undefined4 *)((longlong)param_1 + 0x25f) = 0;
  *(undefined8 *)((longlong)param_1 + 0x263) = DAT_14327dd80;
  *(undefined8 *)((longlong)param_1 + 0x242) = 0;
  *(undefined8 *)((longlong)param_1 + 0x24a) = 0;
  *(undefined1 *)((longlong)param_1 + 0x252) = 0;
  *(undefined8 *)((longlong)param_1 + 0x263) = DAT_14327dd80;
  FUN_1402f8a00((longlong)param_1 + 0x26b);
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x303) = bVar1;
  *(byte *)((longlong)param_1 + 0x304) = bVar1;
  *(uint *)((longlong)param_1 + 0x307) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x30b) = bVar1;
  *(byte *)((longlong)param_1 + 0x30c) = bVar1;
  *(uint *)((longlong)param_1 + 0x30f) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x303) = bVar1;
  *(byte *)((longlong)param_1 + 0x304) = bVar1;
  *(uint *)((longlong)param_1 + 0x307) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x30b) = bVar1;
  *(byte *)((longlong)param_1 + 0x30c) = bVar1;
  *(uint *)((longlong)param_1 + 0x30f) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  FUN_1402fb7e0((longlong)param_1 + 0x26b);
  FUN_1402f8a00((longlong)param_1 + 0x313);
  *(undefined4 *)((longlong)param_1 + 0x3ab) = 0;
  FUN_1402f8710((longlong)param_1 + 0x3af);
  *(undefined8 *)((longlong)param_1 + 0x4d) = 0;
  *(undefined1 *)((longlong)param_1 + 0x55) = 0;
  return param_1;
}


