
//===========================================================
// FUN_140302e30 @ 140302e30   (1712 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_140302e30(undefined4 *param_1,undefined8 param_2,int param_3)

{
  byte *pbVar1;
  ushort uVar2;
  undefined8 *puVar3;
  byte *pbVar4;
  undefined1 uVar5;
  byte bVar6;
  undefined2 uVar7;
  short sVar8;
  undefined4 uVar9;
  uint uVar10;
  uint uVar11;
  undefined8 uVar12;
  undefined8 *puVar13;
  int iVar14;
  ulonglong uVar15;
  undefined8 *puVar16;
  undefined8 *puVar17;
  byte bVar18;
  byte *pbVar19;
  undefined1 auStack_88 [32];
  undefined4 local_68;
  undefined4 local_60;
  undefined4 local_5c;
  undefined1 local_58 [8];
  undefined1 local_50 [16];
  ulonglong local_40;
  
  local_40 = DAT_143a8b908 ^ (ulonglong)auStack_88;
  if (param_3 == 0) {
    uVar9 = FUN_1406e8c20(param_2);
    *param_1 = uVar9;
    uVar9 = FUN_1406e8c20(param_2);
    param_1[1] = uVar9;
    uVar9 = FUN_1406e8c20(param_2);
    param_1[2] = uVar9;
    FUN_1406e9170(param_2,param_1 + 3,0xd);
    uVar5 = FUN_1406e8ae0(param_2);
    *(undefined1 *)((longlong)param_1 + 0x19) = uVar5;
    uVar5 = FUN_1406e8ae0(param_2);
    uVar9 = FUN_1406e8c20(param_2);
    *(undefined1 *)((longlong)param_1 + 0x1a) = uVar5;
    *(undefined4 *)((longlong)param_1 + 0x1b) = uVar9;
    uVar9 = FUN_1406e8c20(param_2);
    *(undefined4 *)((longlong)param_1 + 0x1f) = uVar9;
    uVar9 = FUN_1406e8c20(param_2);
    *(undefined4 *)((longlong)param_1 + 0x23) = uVar9;
  }
  else {
    FUN_1406e8c20(param_2);
    uVar9 = FUN_1406e8c20(param_2);
    param_1[1] = uVar9;
    uVar9 = FUN_1406e8c20(param_2);
    param_1[2] = uVar9;
    FUN_1406e9170(param_2,local_50,0xd);
    FUN_1406e8ae0(param_2);
    FUN_1406e8ae0(param_2);
    FUN_1406e8c20(param_2);
    FUN_1406e8c20(param_2);
    FUN_1406e8c20(param_2);
  }
  uVar10 = FUN_1406e8c20(param_2);
  uVar11 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0x27) = uVar11;
  uVar10 = (uVar11 ^ uVar10) >> 5 | (uVar11 ^ uVar10) << 0x1b;
  *(uint *)((longlong)param_1 + 0x2b) = uVar10;
  *(uint *)((longlong)param_1 + 0x2f) =
       ((uVar11 ^ 0xbaadf00d) >> 5 | (uVar11 ^ 0xbaadf00d) << 0x1b) + uVar10;
  uVar7 = FUN_1406e8b80(param_2);
  uVar9 = FUN_1402f7010(uVar7,(longlong)param_1 + 0x33);
  *(undefined4 *)((longlong)param_1 + 0x37) = uVar9;
  uVar7 = FUN_1406e8b80(param_2);
  uVar9 = FUN_1402f7010(uVar7,(longlong)param_1 + 0x3b);
  *(undefined4 *)((longlong)param_1 + 0x3f) = uVar9;
  uVar7 = FUN_1406e8b80(param_2);
  uVar9 = FUN_1402f7010(uVar7,(longlong)param_1 + 0x43);
  *(undefined4 *)((longlong)param_1 + 0x47) = uVar9;
  uVar7 = FUN_1406e8b80(param_2);
  uVar9 = FUN_1402f7010(uVar7,(longlong)param_1 + 0x4b);
  *(undefined4 *)((longlong)param_1 + 0x4f) = uVar9;
  uVar7 = FUN_1406e8b80(param_2);
  uVar9 = FUN_1402f7010(uVar7,(longlong)param_1 + 0x53);
  *(undefined4 *)((longlong)param_1 + 0x57) = uVar9;
  uVar10 = FUN_1406e8c20(param_2);
  uVar11 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0x5b) = uVar11;
  uVar10 = (uVar11 ^ uVar10) >> 5 | (uVar11 ^ uVar10) << 0x1b;
  *(uint *)((longlong)param_1 + 0x5f) = uVar10;
  *(uint *)((longlong)param_1 + 99) =
       ((uVar11 ^ 0xbaadf00d) >> 5 | (uVar11 ^ 0xbaadf00d) << 0x1b) + uVar10;
  uVar10 = FUN_1406e8c20(param_2);
  uVar11 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0x67) = uVar11;
  uVar10 = (uVar11 ^ uVar10) >> 5 | (uVar11 ^ uVar10) << 0x1b;
  *(uint *)((longlong)param_1 + 0x6b) = uVar10;
  *(uint *)((longlong)param_1 + 0x6f) =
       ((uVar11 ^ 0xbaadf00d) >> 5 | (uVar11 ^ 0xbaadf00d) << 0x1b) + uVar10;
  uVar10 = FUN_1406e8c20(param_2);
  uVar11 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0x73) = uVar11;
  uVar10 = (uVar11 ^ uVar10) >> 5 | (uVar11 ^ uVar10) << 0x1b;
  *(uint *)((longlong)param_1 + 0x77) = uVar10;
  *(uint *)((longlong)param_1 + 0x7b) =
       ((uVar11 ^ 0xbaadf00d) >> 5 | (uVar11 ^ 0xbaadf00d) << 0x1b) + uVar10;
  uVar10 = FUN_1406e8c20(param_2);
  uVar11 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0x7f) = uVar11;
  uVar10 = (uVar11 ^ uVar10) >> 5 | (uVar11 ^ uVar10) << 0x1b;
  *(uint *)((longlong)param_1 + 0x83) = uVar10;
  *(uint *)((longlong)param_1 + 0x87) =
       ((uVar11 ^ 0xbaadf00d) >> 5 | (uVar11 ^ 0xbaadf00d) << 0x1b) + uVar10;
  uVar7 = FUN_1406e8b80(param_2);
  uVar9 = FUN_1402f7010(uVar7,(longlong)param_1 + 0x8b);
  *(undefined4 *)((longlong)param_1 + 0x8f) = uVar9;
  sVar8 = FUN_1401ab420((longlong)param_1 + 0x33,*(undefined4 *)((longlong)param_1 + 0x37));
  iVar14 = (int)sVar8;
  puVar17 = (undefined8 *)0x0;
  if (((((sVar8 == 0) ||
        ((iVar14 - 100U < 0x21 &&
         ((0x1c0701c01U >> ((longlong)(int)(iVar14 - 100U) & 0x3fU) & 1) != 0)))) ||
       ((iVar14 - 200U < 0x21 &&
        ((0x1c0701c01U >> ((longlong)(int)(iVar14 - 200U) & 0x3fU) & 1) != 0)))) ||
      ((((iVar14 - 300U < 0x17 && ((0x701c01U >> (iVar14 - 300U & 0x1f) & 1) != 0)) ||
        ((iVar14 - 400U < 0x17 && ((0x701c01U >> (iVar14 - 400U & 0x1f) & 1) != 0)))) ||
       (iVar14 - 0x1aeU < 10)))) ||
     ((iVar14 - 500U < 0x17 && ((0x701c01U >> (iVar14 - 500U & 0x1f) & 1) != 0)))) {
    uVar9 = FUN_1402f7010(0,(longlong)param_1 + 0x93);
    *(undefined4 *)((longlong)param_1 + 0x97) = uVar9;
    FUN_1402cb0d0((longlong)param_1 + 0xd7,param_2);
  }
  else {
    uVar7 = FUN_1406e8b80(param_2);
    uVar9 = FUN_1402f7010(uVar7,(longlong)param_1 + 0x93);
    *(undefined4 *)((longlong)param_1 + 0x97) = uVar9;
    puVar3 = *(undefined8 **)((longlong)param_1 + 0xdf);
    while (puVar3 != (undefined8 *)0x0) {
      uVar15 = puVar3[-4];
      if ((uVar15 != 0) && (uVar15 < 0x10001)) {
        FUN_142e52ed0(0x33e);
        uVar15 = puVar3[-4];
      }
      puVar13 = puVar17;
      if (uVar15 != 0) {
        puVar13 = (undefined8 *)(uVar15 + 0x28);
      }
      puVar16 = puVar17;
      if (puVar3 != (undefined8 *)0x0) {
        puVar16 = puVar3 + -5;
      }
      puVar3 = puVar13;
      if (puVar16 != (undefined8 *)0x0) {
        (**(code **)*puVar16)(puVar16,1);
      }
    }
    *(undefined8 *)((longlong)param_1 + 0xe7) = 0;
    *(undefined8 *)((longlong)param_1 + 0xdf) = 0;
    *(undefined4 *)((longlong)param_1 + 0xdb) = 0;
    *(undefined4 *)((longlong)param_1 + 0xef) = 0;
  }
  uVar12 = FUN_1406e8f10(param_2);
  uVar9 = FUN_1402f7170(uVar12,(longlong)param_1 + 0x9b);
  *(undefined4 *)((longlong)param_1 + 0xab) = uVar9;
  uVar10 = FUN_1406e8c20(param_2);
  uVar11 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0xb3) = uVar11;
  uVar10 = (uVar11 ^ uVar10) >> 5 | (uVar11 ^ uVar10) << 0x1b;
  *(uint *)((longlong)param_1 + 0xb7) = uVar10;
  *(uint *)((longlong)param_1 + 0xbb) =
       ((uVar11 ^ 0xbaadf00d) >> 5 | (uVar11 ^ 0xbaadf00d) << 0x1b) + uVar10;
  local_68 = FUN_1406e8c20(param_2);
  iVar14 = *(int *)((longlong)param_1 + 0xf3) + 1;
  *(int *)((longlong)param_1 + 0xf3) = iVar14;
  if (iVar14 == (iVar14 / 0x6f) * 0x6f) {
    puVar3 = *(undefined8 **)((longlong)param_1 + 0xfb);
    puVar13 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    *(undefined8 **)((longlong)param_1 + 0xfb) = puVar13;
    *puVar13 = *puVar3;
    *(undefined4 *)(puVar13 + 1) = *(undefined4 *)(puVar3 + 1);
    thunk_FUN_140205820(puVar3,0xc);
  }
  uVar5 = FUN_142f04924();
  *(undefined1 *)(*(longlong *)((longlong)param_1 + 0xfb) + 4) = uVar5;
  pbVar4 = *(byte **)((longlong)param_1 + 0xfb);
  bVar18 = pbVar4[4];
  pbVar4[8] = 0x65;
  pbVar4[9] = 0x9a;
  pbVar19 = pbVar4;
  do {
    pbVar1 = pbVar19 + 4;
    if (bVar18 == 0) {
      bVar18 = 0x2a;
    }
    bVar6 = pbVar1[(longlong)(auStack_88 + (0x1c - (longlong)pbVar4))];
    *pbVar19 = bVar18 ^ bVar6;
    bVar18 = bVar18 + (bVar18 ^ bVar6) + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)((longlong)param_1 + 0xfb) + 8);
    *(ushort *)(*(longlong *)((longlong)param_1 + 0xfb) + 8) =
         (uVar2 >> 0xd) + (ushort)bVar18 | uVar2 << 3;
    bVar6 = 0x2a;
    if (bVar18 != 0) {
      bVar6 = bVar18;
    }
    bVar18 = pbVar1[(longlong)(auStack_88 + (0x1d - (longlong)pbVar4))];
    pbVar19[1] = bVar6 ^ bVar18;
    bVar6 = (bVar6 ^ bVar18) + bVar6 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)((longlong)param_1 + 0xfb) + 8);
    *(ushort *)(*(longlong *)((longlong)param_1 + 0xfb) + 8) =
         (uVar2 >> 0xd) + (ushort)bVar6 | uVar2 << 3;
    bVar18 = 0x2a;
    if (bVar6 != 0) {
      bVar18 = bVar6;
    }
    bVar6 = pbVar1[(longlong)(auStack_88 + (0x1e - (longlong)pbVar4))];
    pbVar19[2] = bVar18 ^ bVar6;
    bVar6 = (bVar18 ^ bVar6) + bVar18 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)((longlong)param_1 + 0xfb) + 8);
    *(ushort *)(*(longlong *)((longlong)param_1 + 0xfb) + 8) =
         (uVar2 >> 0xd) + (ushort)bVar6 | uVar2 << 3;
    bVar18 = 0x2a;
    if (bVar6 != 0) {
      bVar18 = bVar6;
    }
    uVar10 = (int)puVar17 + 4;
    puVar17 = (undefined8 *)(ulonglong)uVar10;
    bVar6 = pbVar1[(longlong)(auStack_88 + (0x1f - (longlong)pbVar4))];
    pbVar19[3] = bVar18 ^ bVar6;
    bVar18 = (bVar18 ^ bVar6) + bVar18 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)((longlong)param_1 + 0xfb) + 8);
    *(ushort *)(*(longlong *)((longlong)param_1 + 0xfb) + 8) =
         (uVar2 >> 0xd) + (ushort)bVar18 | uVar2 << 3;
    pbVar19 = pbVar1;
  } while (uVar10 < 4);
  uVar5 = FUN_1406e8ae0(param_2);
  *(undefined1 *)((longlong)param_1 + 0x10b) = uVar5;
  uVar7 = FUN_1406e8b80(param_2);
  *(undefined2 *)(param_1 + 0x43) = uVar7;
  bVar18 = FUN_1406e8ae0(param_2);
  *(uint *)((longlong)param_1 + 0x10e) = (uint)bVar18;
  FUN_1406e9170(param_2,local_58,8);
  (*DAT_1432625b0)(local_58,(longlong)param_1 + 0x112);
  local_5c = FUN_1406e8c20(param_2);
  local_60 = FUN_1406e8c20(param_2);
  (*DAT_1432625b0)(&local_60,(longlong)param_1 + 0x126);
  return;
}


