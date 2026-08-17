
//===========================================================
// FUN_142c50300 @ 142c50300   (7 bytes)
//===========================================================

void FUN_142c50300(longlong param_1,undefined1 param_2)

{
  *(undefined1 *)(param_1 + 0x1a0) = param_2;
  return;
}



//===========================================================
// FUN_142c50b90 @ 142c50b90   (128 bytes)
//===========================================================

void FUN_142c50b90(int param_1)

{
  int iVar1;
  undefined7 uVar2;
  
  iVar1 = DAT_143adde2c;
  DAT_143adde2c = param_1;
  if (param_1 != iVar1) {
    uVar2 = (undefined7)
            ((ulonglong)
             (IMAGE_DOS_HEADER_140000000.e_magic +
             (&switchD_142c50bb9::switchdataD_142c50c10)[param_1]) >> 8);
    switch(param_1) {
    case 0:
      FUN_142e0eec0(CONCAT71(uVar2,1));
      return;
    case 1:
      FUN_142e0eec0(CONCAT71(uVar2,2));
      return;
    case 2:
      FUN_142e0eec0(CONCAT71(uVar2,3));
      return;
    case 3:
      FUN_142e0eec0(CONCAT71(uVar2,4));
      return;
    case 4:
      FUN_142e0eec0(CONCAT71(uVar2,5));
      return;
    case 5:
      FUN_142e0eec0(CONCAT71(uVar2,6));
      return;
    case 6:
      FUN_142e0eec0(CONCAT71(uVar2,7));
      return;
    case 7:
      FUN_142e0eec0(CONCAT71(uVar2,8));
      return;
    case 8:
      FUN_142e0eec0(CONCAT71(uVar2,9));
      return;
    case 9:
      FUN_142e0eec0(CONCAT71(uVar2,10));
      return;
    case 10:
      FUN_142e0eec0(CONCAT71(uVar2,0xb));
      return;
    case 0xb:
      FUN_142e0eec0(CONCAT71(uVar2,0xc));
      return;
    }
  }
  return;
}



//===========================================================
// FUN_141b10b20 @ 141b10b20   (517 bytes)
//===========================================================

void FUN_141b10b20(void)

{
  byte *pbVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 *puVar6;
  byte bVar7;
  byte bVar8;
  undefined8 *puVar9;
  uint uVar10;
  byte *pbVar11;
  byte local_res8 [8];
  
  puVar6 = DAT_143ad1fe8;
  uVar10 = 0;
  DAT_143ad1fe0 = DAT_143ad1fe0 + 1;
  local_res8[0] = 0;
  local_res8[1] = 0;
  local_res8[2] = 0;
  local_res8[3] = 0;
  if (DAT_143ad1fe0 == (DAT_143ad1fe0 / 0x6f) * 0x6f) {
    puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    DAT_143ad1fe8 = puVar9;
    *puVar9 = *puVar6;
    *(undefined4 *)(puVar9 + 1) = *(undefined4 *)(puVar6 + 1);
    thunk_FUN_140205820(puVar6,0xc);
  }
  bVar7 = FUN_142f04924();
  *(byte *)((longlong)DAT_143ad1fe8 + 4) = bVar7;
  puVar6 = DAT_143ad1fe8;
  lVar2 = -(longlong)DAT_143ad1fe8;
  lVar3 = -(longlong)DAT_143ad1fe8;
  lVar4 = -(longlong)DAT_143ad1fe8;
  lVar5 = -(longlong)DAT_143ad1fe8;
  *(undefined2 *)(DAT_143ad1fe8 + 1) = 0x9a65;
  pbVar11 = (byte *)((longlong)puVar6 + 2);
  do {
    pbVar1 = pbVar11 + 4;
    if (bVar7 == 0) {
      bVar7 = 0x2a;
    }
    bVar8 = pbVar11[(longlong)(&stack0x00000006 + lVar2)];
    pbVar11[-2] = bVar7 ^ bVar8;
    bVar7 = bVar7 + (bVar7 ^ bVar8) + 0x2a;
    *(ushort *)(DAT_143ad1fe8 + 1) =
         (*(ushort *)(DAT_143ad1fe8 + 1) >> 0xd) + (ushort)bVar7 |
         *(ushort *)(DAT_143ad1fe8 + 1) << 3;
    bVar8 = 0x2a;
    if (bVar7 != 0) {
      bVar8 = bVar7;
    }
    bVar7 = pbVar1[(longlong)(&stack0x00000003 + lVar3)];
    pbVar11[-1] = bVar8 ^ bVar7;
    bVar8 = (bVar8 ^ bVar7) + bVar8 + 0x2a;
    *(ushort *)(DAT_143ad1fe8 + 1) =
         (*(ushort *)(DAT_143ad1fe8 + 1) >> 0xd) + (ushort)bVar8 |
         *(ushort *)(DAT_143ad1fe8 + 1) << 3;
    bVar7 = 0x2a;
    if (bVar8 != 0) {
      bVar7 = bVar8;
    }
    bVar8 = pbVar1[(longlong)(&stack0x00000004 + lVar4)];
    *pbVar11 = bVar7 ^ bVar8;
    bVar8 = (bVar7 ^ bVar8) + bVar7 + 0x2a;
    *(ushort *)(DAT_143ad1fe8 + 1) =
         (*(ushort *)(DAT_143ad1fe8 + 1) >> 0xd) + (ushort)bVar8 |
         *(ushort *)(DAT_143ad1fe8 + 1) << 3;
    bVar7 = 0x2a;
    if (bVar8 != 0) {
      bVar7 = bVar8;
    }
    uVar10 = uVar10 + 4;
    bVar8 = pbVar1[(longlong)(&stack0x00000005 + lVar5)];
    pbVar11[1] = bVar7 ^ bVar8;
    bVar7 = (bVar7 ^ bVar8) + bVar7 + 0x2a;
    *(ushort *)(DAT_143ad1fe8 + 1) =
         (*(ushort *)(DAT_143ad1fe8 + 1) >> 0xd) + (ushort)bVar7 |
         *(ushort *)(DAT_143ad1fe8 + 1) << 3;
    pbVar11 = pbVar1;
  } while (uVar10 < 4);
  return;
}



//===========================================================
// FUN_1415d01c0 @ 1415d01c0   (171 bytes)
//===========================================================

void FUN_1415d01c0(longlong param_1)

{
  char cVar1;
  undefined8 uVar2;
  longlong lVar3;
  
  uVar2 = FUN_140caa510();
  FUN_142d11940(uVar2,*(undefined4 *)(param_1 + 0x434));
  cVar1 = FUN_1415e5a20(*(undefined4 *)(param_1 + 0x434));
  if (cVar1 == '\0') {
    cVar1 = FUN_1415e59d0(*(undefined4 *)(param_1 + 0x434));
    if (cVar1 == '\0') goto LAB_1415d0245;
  }
  uVar2 = FUN_140caa510();
  FUN_142d191b0(uVar2,*(undefined4 *)(param_1 + 0x434));
LAB_1415d0245:
  lVar3 = FUN_140caa4d0();
  if (lVar3 != 0) {
    FUN_1415d3990(lVar3,param_1);
  }
  return;
}


