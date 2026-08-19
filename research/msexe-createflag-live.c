
//===========================================================
// FUN_140c9ef80 @ 140c9ef80   (1 bytes)
//===========================================================

/* WARNING: Heritage AFTER dead removal. Example location: s0xffffffffffffff68 : 0x000140c9f3dd */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

void FUN_140c9ef80(void)

{
  longlong lVar1;
  undefined1 *puVar2;
  undefined1 *puVar3;
  byte bStack_b8;
  byte bStack_b7;
  byte abStack_b6 [6];
  int *piStack_b0;
  byte bStack_a8;
  undefined1 uStack_a7;
  undefined1 auStack_a6 [2];
  ushort auStack_a4 [2];
  undefined1 uStack_a0;
  undefined1 uStack_9f;
  undefined1 uStack_9e;
  byte abStack_98 [8];
  int iStack_90;
  int iStack_8c;
  int iStack_88;
  int iStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  byte *pbStack_70;
  undefined1 *puStack_68;
  byte *pbStack_60;
  byte *pbStack_58;
  longlong lStack_50;
  longlong lStack_48;
  longlong lStack_40;
  ulonglong uStack_38;
  undefined8 uStack_30;
  undefined1 *puStack_28;
  byte *pbStack_20;
  
  bStack_a8 = 0;
  uStack_80 = 0;
  uStack_78 = 1;
  uStack_7c = 0;
  auStack_a6[0] = DAT_143ac7e54;
  uStack_a7 = DAT_143ac7fe5;
  piStack_b0 = &DAT_143ac8168;
  abStack_b6[0] = *DAT_143ac8170;
  bStack_b8 = DAT_143ac8170[1];
  pbStack_58 = abStack_b6;
  pbStack_70 = DAT_143ac8170;
  auStack_a4[0] = 0x9a65;
  iStack_90 = 0;
  while (iStack_90 == 0) {
    if (bStack_b8 == 0) {
      bStack_b8 = 0x2a;
    }
    bStack_b8 = bStack_b8 + 0x2a + *DAT_143ac8170;
    auStack_a4[0] =
         auStack_a4[0] << 3 | (short)((int)(uint)auStack_a4[0] >> 0xd) + (ushort)bStack_b8;
    iStack_90 = 1;
  }
  FUN_140c78f50(0x171,0x21a3f1a4a8af17);
  if (((auStack_a4[0] != *(ushort *)(*(longlong *)(piStack_b0 + 2) + 4)) ||
      ((char)piStack_b0[1] != *(char *)(*(longlong *)(piStack_b0 + 2) + 2))) ||
     ((char)piStack_b0[4] != *(char *)(*(longlong *)(piStack_b0 + 2) + 3))) {
    lStack_50 = *(longlong *)(piStack_b0 + 2) + 3;
    uStack_a0 = (undefined1)piStack_b0[4];
    lStack_48 = *(longlong *)(piStack_b0 + 2) + 2;
    uStack_9f = (undefined1)piStack_b0[1];
    lStack_40 = *(longlong *)(piStack_b0 + 2) + 4;
    uStack_38 = (ulonglong)abStack_b6[0];
    uStack_74 = 2;
    FUN_140197dd0(&DAT_143271f04,0x17c,5,&uStack_74,&uStack_38,&bStack_b8,auStack_a4,lStack_40,
                  &uStack_9f,lStack_48,&uStack_a0,lStack_50);
  }
  iStack_88 = *piStack_b0 + 1;
  *piStack_b0 = iStack_88;
  if (iStack_88 % 0x37 == 0) {
    abStack_98[0] = abStack_b6[0];
    iStack_84 = *piStack_b0 + 1;
    *piStack_b0 = iStack_84;
    if (iStack_84 % 0x6f == 0) {
      puStack_68 = *(undefined1 **)(piStack_b0 + 2);
      uStack_30 = FUN_14019a150(6);
      *(undefined8 *)(piStack_b0 + 2) = uStack_30;
      puVar2 = puStack_68;
      puVar3 = *(undefined1 **)(piStack_b0 + 2);
      for (lVar1 = 6; lVar1 != 0; lVar1 = lVar1 + -1) {
        *puVar3 = *puVar2;
        puVar2 = puVar2 + 1;
        puVar3 = puVar3 + 1;
      }
      puStack_28 = puStack_68;
      thunk_FUN_140205820(puStack_68,6);
    }
    uStack_9e = FUN_142f04924();
    *(undefined1 *)(*(longlong *)(piStack_b0 + 2) + 1) = uStack_9e;
    bStack_b7 = *(byte *)(*(longlong *)(piStack_b0 + 2) + 1);
    pbStack_20 = abStack_98;
    pbStack_60 = *(byte **)(piStack_b0 + 2);
    *(undefined2 *)(*(longlong *)(piStack_b0 + 2) + 4) = 0x9a65;
    iStack_8c = 0;
    while (iStack_8c == 0) {
      if (bStack_b7 == 0) {
        bStack_b7 = 0x2a;
      }
      *pbStack_60 = abStack_98[0] ^ bStack_b7;
      bStack_b7 = bStack_b7 + 0x2a + *pbStack_60;
      *(ushort *)(*(longlong *)(piStack_b0 + 2) + 4) =
           *(short *)(*(longlong *)(piStack_b0 + 2) + 4) << 3 |
           (short)((int)(uint)*(ushort *)(*(longlong *)(piStack_b0 + 2) + 4) >> 0xd) +
           (ushort)bStack_b7;
      iStack_8c = 1;
    }
  }
  FUN_140c79130(0x182,piStack_b0 + 0xc0a84bdc4a);
  bStack_a8 = abStack_b6[0];
  uStack_80 = func_0x000140caf1b0(&DAT_143ac8138);
  func_0x000140ca2fc0(399,&uStack_78,&uStack_7c,auStack_a6,&uStack_a7,&bStack_a8,&uStack_80);
  return;
}



//===========================================================
// FUN_140c950a0 @ 140c950a0   (1 bytes)
//===========================================================

void FUN_140c950a0(undefined4 param_1)

{
  char cVar1;
  
  if ((iRam0000000143ac7d90 != 0) &&
     (cVar1 = FUN_1408fcaa0(iRam0000000143ac7d90,10000,param_1), cVar1 == '\0')) {
    return;
  }
  iRam0000000143ac7d90 = param_1;
  cVar1 = FUN_142e0ea90();
  if ((cVar1 != '\0') && (func_0x000140c9ec90(), cRam0000000143ac7d3f == '\0')) {
    cRam0000000143ac7d3f = '\x01';
    FUN_140c9ef80();
  }
  return;
}



//===========================================================
// FUN_14003fb80 @ 14003fb80   (643 bytes)
//===========================================================

/* WARNING: Heritage AFTER dead removal. Example location: s0xffffffffffffff90 : 0x00014003fd79 */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

void FUN_14003fb80(void)

{
  byte bVar1;
  longlong lVar2;
  byte *pbVar3;
  byte *pbVar4;
  byte bStack_78;
  byte abStack_70 [8];
  int iStack_68;
  int iStack_64;
  byte *pbStack_60;
  byte *pbStack_58;
  byte *pbStack_50;
  byte **ppbStack_48;
  byte **ppbStack_40;
  byte *pbStack_38;
  byte *pbStack_30;
  byte *pbStack_28;
  
  DAT_143ac8168 = 0;
  DAT_143ac8170 = (byte *)FUN_14019a150(6);
  ppbStack_48 = &DAT_143ac8170;
  pbStack_50 = DAT_143ac8170;
  _DAT_143ac816c = FUN_142f04924();
  _DAT_143ac816c = (int)ppbStack_48 + -0x4000 + _DAT_143ac816c;
  ppbStack_40 = &DAT_143ac8170;
  _DAT_143ac8178 = FUN_142f04924();
  _DAT_143ac8178 = (int)ppbStack_40 + -0x4000 + _DAT_143ac8178;
  DAT_143ac8170[2] = DAT_143ac816c;
  DAT_143ac8170[3] = DAT_143ac8178;
  abStack_70[0] = 0;
  DAT_143ac8168 = DAT_143ac8168 + 1;
  iStack_64 = DAT_143ac8168;
  if (DAT_143ac8168 % 0x6f == 0) {
    pbStack_60 = DAT_143ac8170;
    pbStack_38 = (byte *)FUN_14019a150(6);
    pbVar3 = pbStack_60;
    pbVar4 = pbStack_38;
    DAT_143ac8170 = pbStack_38;
    for (lVar2 = 6; lVar2 != 0; lVar2 = lVar2 + -1) {
      *pbVar4 = *pbVar3;
      pbVar3 = pbVar3 + 1;
      pbVar4 = pbVar4 + 1;
    }
    pbStack_30 = pbStack_60;
    thunk_FUN_140205820(pbStack_60,6);
  }
  bVar1 = FUN_142f04924();
  DAT_143ac8170[1] = bVar1;
  pbVar3 = DAT_143ac8170;
  bStack_78 = DAT_143ac8170[1];
  pbStack_28 = abStack_70;
  pbStack_58 = DAT_143ac8170;
  pbVar3[4] = 0x65;
  pbVar3[5] = 0x9a;
  iStack_68 = 0;
  while (iStack_68 == 0) {
    if (bStack_78 == 0) {
      bStack_78 = 0x2a;
    }
    *pbVar3 = abStack_70[0] ^ bStack_78;
    bStack_78 = bStack_78 + 0x2a + *pbVar3;
    *(ushort *)(DAT_143ac8170 + 4) =
         *(short *)(DAT_143ac8170 + 4) << 3 |
         (short)((int)(uint)*(ushort *)(DAT_143ac8170 + 4) >> 0xd) + (ushort)bStack_78;
    iStack_68 = 1;
  }
  atexit(FUN_14321dff0);
  return;
}


