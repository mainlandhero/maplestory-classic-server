
//===========================================================
// FUN_140c9e8a0 @ 140c9e8a0   (443 bytes)
//===========================================================

/* WARNING: Heritage AFTER dead removal. Example location: s0xffffffffffffffb0 : 0x000140c9e9e0 */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

void FUN_140c9e8a0(void)

{
  bool bVar1;
  byte *pbVar2;
  byte bVar3;
  longlong lVar4;
  byte *pbVar5;
  byte *pbVar6;
  byte local_58;
  
  pbVar2 = DAT_143ac8170;
  DAT_143ac7fe5 = 0;
  DAT_143ac8168 = DAT_143ac8168 + 1;
  if (DAT_143ac8168 % 0x6f == 0) {
    DAT_143ac8170 = (byte *)FUN_14019a150(6);
    pbVar5 = pbVar2;
    pbVar6 = DAT_143ac8170;
    for (lVar4 = 6; lVar4 != 0; lVar4 = lVar4 + -1) {
      *pbVar6 = *pbVar5;
      pbVar5 = pbVar5 + 1;
      pbVar6 = pbVar6 + 1;
    }
    thunk_FUN_140205820(pbVar2,6);
  }
  bVar3 = FUN_142f04924();
  DAT_143ac8170[1] = bVar3;
  pbVar2 = DAT_143ac8170;
  local_58 = DAT_143ac8170[1];
  pbVar2[4] = 0x65;
  pbVar2[5] = 0x9a;
  bVar1 = false;
  while (!bVar1) {
    if (local_58 == 0) {
      local_58 = 0x2a;
    }
    *pbVar2 = local_58;
    local_58 = local_58 + 0x2a + *pbVar2;
    *(ushort *)(DAT_143ac8170 + 4) =
         *(short *)(DAT_143ac8170 + 4) << 3 |
         (short)((int)(uint)*(ushort *)(DAT_143ac8170 + 4) >> 0xd) + (ushort)local_58;
    bVar1 = true;
  }
  return;
}



//===========================================================
// FUN_140c9e230 @ 140c9e230   (440 bytes)
//===========================================================

/* WARNING: Heritage AFTER dead removal. Example location: s0xffffffffffffffb0 : 0x000140c9e36d */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

void FUN_140c9e230(void)

{
  bool bVar1;
  byte *pbVar2;
  byte bVar3;
  longlong lVar4;
  byte *pbVar5;
  byte *pbVar6;
  byte local_58;
  
  pbVar2 = DAT_143ac8170;
  DAT_143ac8168 = DAT_143ac8168 + 1;
  if (DAT_143ac8168 % 0x6f == 0) {
    DAT_143ac8170 = (byte *)FUN_14019a150(6);
    pbVar5 = pbVar2;
    pbVar6 = DAT_143ac8170;
    for (lVar4 = 6; lVar4 != 0; lVar4 = lVar4 + -1) {
      *pbVar6 = *pbVar5;
      pbVar5 = pbVar5 + 1;
      pbVar6 = pbVar6 + 1;
    }
    thunk_FUN_140205820(pbVar2,6);
  }
  bVar3 = FUN_142f04924();
  DAT_143ac8170[1] = bVar3;
  pbVar2 = DAT_143ac8170;
  local_58 = DAT_143ac8170[1];
  pbVar2[4] = 0x65;
  pbVar2[5] = 0x9a;
  bVar1 = false;
  while (!bVar1) {
    if (local_58 == 0) {
      local_58 = 0x2a;
    }
    *pbVar2 = local_58 ^ 1;
    local_58 = local_58 + 0x2a + *pbVar2;
    *(ushort *)(DAT_143ac8170 + 4) =
         *(short *)(DAT_143ac8170 + 4) << 3 |
         (short)((int)(uint)*(ushort *)(DAT_143ac8170 + 4) >> 0xd) + (ushort)local_58;
    bVar1 = true;
  }
  return;
}


