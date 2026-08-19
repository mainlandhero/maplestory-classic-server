
//===========================================================
// FUN_142c95c20 @ 142c95c20   (37 bytes)
//===========================================================

undefined8 * FUN_142c95c20(longlong param_1,undefined8 *param_2)

{
  *param_2 = 0;
  FUN_14019a260(param_2,param_1 + 0x18);
  return param_2;
}



//===========================================================
// FUN_142c95c50 @ 142c95c50   (37 bytes)
//===========================================================

undefined8 * FUN_142c95c50(longlong param_1,undefined8 *param_2)

{
  *param_2 = 0;
  FUN_14019a260(param_2,param_1 + 0x20);
  return param_2;
}



//===========================================================
// FUN_142c95db0 @ 142c95db0   (1 bytes)
//===========================================================

longlong * FUN_142c95db0(longlong param_1,longlong *param_2)

{
  int *piVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  undefined8 *puVar7;
  
  puVar7 = (undefined8 *)(param_1 + 0x60);
  if (0xf < *(ulonglong *)(param_1 + 0x78)) {
    puVar7 = (undefined8 *)*puVar7;
  }
  *param_2 = 0;
  if (puVar7 == (undefined8 *)0x0) {
    return param_2;
  }
  uVar6 = 0xffffffffffffffff;
  uVar5 = 0xffffffffffffffff;
  do {
    uVar5 = uVar5 + 1;
  } while (*(char *)((longlong)puVar7 + uVar5) != '\0');
  iVar3 = (int)uVar5;
  iVar4 = 0;
  if (0 < iVar3) {
    iVar4 = iVar3;
  }
  piVar2 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
  piVar2[1] = iVar4;
  piVar1 = piVar2 + 4;
  *piVar2 = -1;
  piVar2[2] = 0;
  *param_2 = (longlong)piVar1;
  *(undefined1 *)piVar1 = 0;
  FUN_142ef7ba0(piVar1,puVar7,(longlong)iVar3);
  if (*piVar2 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar3 == -1) || (iVar3 <= piVar2[1])) {
    *piVar2 = 1;
    if (iVar3 != -1) goto code_r0x000142c95e7a;
    if (piVar1 != (int *)0x0) {
      do {
        uVar6 = uVar6 + 1;
      } while (*(char *)((longlong)piVar1 + uVar6) != '\0');
      uVar5 = uVar6 & 0xffffffff;
      goto code_r0x000142c95e82;
    }
    uVar5 = 0;
code_r0x000142c95e86:
    iVar4 = (int)uVar5;
    if (iVar4 < piVar2[1] + 1) goto code_r0x000142c95e9f;
  }
  else {
    FUN_142e54290(0x90,piVar2[1],uVar5 & 0xffffffff);
    *piVar2 = 1;
code_r0x000142c95e7a:
    *(undefined1 *)((longlong)iVar3 + *param_2) = 0;
code_r0x000142c95e82:
    if (-1 < (int)uVar5) goto code_r0x000142c95e86;
  }
  iVar4 = (int)uVar5;
  FUN_142e54290(0x9c,uVar5 & 0xffffffff,piVar2[1]);
code_r0x000142c95e9f:
  piVar2[2] = iVar4;
  return param_2;
}



//===========================================================
// FUN_142c95dd9 @ 142c95dd9   (1 bytes)
//===========================================================

void FUN_142c95dd9(void)

{
  int *piVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  longlong unaff_R14;
  longlong *unaff_R15;
  
  uVar6 = 0xffffffffffffffff;
  uVar5 = 0xffffffffffffffff;
  do {
    uVar5 = uVar5 + 1;
  } while (*(char *)(unaff_R14 + uVar5) != '\0');
  iVar3 = (int)uVar5;
  iVar4 = 0;
  if (0 < iVar3) {
    iVar4 = iVar3;
  }
  piVar2 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
  piVar2[1] = iVar4;
  piVar1 = piVar2 + 4;
  *piVar2 = -1;
  piVar2[2] = 0;
  *unaff_R15 = (longlong)piVar1;
  *(undefined1 *)piVar1 = 0;
  FUN_142ef7ba0(piVar1);
  if (*piVar2 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar3 == -1) || (iVar3 <= piVar2[1])) {
    *piVar2 = 1;
    if (iVar3 != -1) goto code_r0x000142c95e7a;
    if (piVar1 != (int *)0x0) {
      do {
        uVar6 = uVar6 + 1;
      } while (*(char *)((longlong)piVar1 + uVar6) != '\0');
      uVar5 = uVar6 & 0xffffffff;
      goto code_r0x000142c95e82;
    }
    uVar5 = 0;
code_r0x000142c95e86:
    iVar4 = (int)uVar5;
    if (iVar4 < piVar2[1] + 1) goto code_r0x000142c95e9f;
  }
  else {
    FUN_142e54290(0x90,piVar2[1],uVar5 & 0xffffffff);
    *piVar2 = 1;
code_r0x000142c95e7a:
    *(undefined1 *)((longlong)iVar3 + *unaff_R15) = 0;
code_r0x000142c95e82:
    if (-1 < (int)uVar5) goto code_r0x000142c95e86;
  }
  iVar4 = (int)uVar5;
  FUN_142e54290(0x9c,uVar5 & 0xffffffff,piVar2[1]);
code_r0x000142c95e9f:
  piVar2[2] = iVar4;
  return;
}



//===========================================================
// FUN_142c95eb6 @ 142c95eb6   (1 bytes)
//===========================================================

void FUN_142c95eb6(void)

{
  return;
}



//===========================================================
// FUN_142c95ec2 @ 142c95ec2   (1 bytes)
//===========================================================

void FUN_142c95ec2(void)

{
  int unaff_EBX;
  longlong unaff_RSI;
  longlong unaff_RDI;
  longlong unaff_R12;
  longlong *unaff_R15;
  
  *(undefined4 *)(unaff_RDI + -0x10) = 1;
  if (unaff_EBX == (int)unaff_RSI) {
    if (unaff_RDI != 0) {
      do {
        unaff_RSI = unaff_RSI + 1;
      } while (*(char *)(unaff_RDI + unaff_RSI) != '\0');
      unaff_EBX = (int)unaff_RSI;
      goto code_r0x000142c95e82;
    }
    unaff_EBX = 0;
code_r0x000142c95e86:
    if (unaff_EBX < *(int *)(unaff_RDI + -0xc) + 1) goto code_r0x000142c95e9f;
  }
  else {
    *(undefined1 *)(unaff_R12 + *unaff_R15) = 0;
code_r0x000142c95e82:
    if (-1 < unaff_EBX) goto code_r0x000142c95e86;
  }
  FUN_142e54290(0x9c,unaff_EBX,*(undefined4 *)(unaff_RDI + -0xc));
code_r0x000142c95e9f:
  *(int *)(unaff_RDI + -8) = unaff_EBX;
  return;
}



//===========================================================
// FUN_142c95ef0 @ 142c95ef0   (40 bytes)
//===========================================================

undefined8 * FUN_142c95ef0(longlong param_1,undefined8 *param_2)

{
  *param_2 = 0;
  FUN_14019a260(param_2,param_1 + 0x80);
  return param_2;
}



//===========================================================
// FUN_142c95f20 @ 142c95f20   (74 bytes)
//===========================================================

undefined8 * FUN_142c95f20(longlong param_1,undefined8 *param_2,int param_3)

{
  if ((-1 < param_3) && ((ulonglong)(longlong)param_3 < 6)) {
    *param_2 = 0;
    FUN_14019a260(param_2,param_1 + 0x90 + (longlong)param_3 * 8);
    return param_2;
  }
  *param_2 = 0;
  return param_2;
}



//===========================================================
// FUN_142c95f70 @ 142c95f70   (1 bytes)
//===========================================================

undefined4 FUN_142c95f70(longlong param_1)

{
  return *(undefined4 *)(param_1 + 200);
}



//===========================================================
// FUN_142c95fe0 @ 142c95fe0   (549 bytes)
//===========================================================

void FUN_142c95fe0(void)

{
  byte *pbVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 *puVar6;
  byte bVar7;
  byte bVar8;
  int iVar9;
  undefined8 *puVar10;
  uint uVar11;
  byte *pbVar12;
  undefined4 local_res8;
  
  iVar9 = FUN_140739110(0x49542,0x6dd9a2);
  local_res8 = (*DAT_143262db0)();
  puVar6 = DAT_143ade408;
  local_res8 = local_res8 + iVar9;
  DAT_143ade400 = DAT_143ade400 + 1;
  if (DAT_143ade400 == (DAT_143ade400 / 0x6f) * 0x6f) {
    puVar10 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    DAT_143ade408 = puVar10;
    *puVar10 = *puVar6;
    *(undefined4 *)(puVar10 + 1) = *(undefined4 *)(puVar6 + 1);
    thunk_FUN_140205820(puVar6,0xc);
  }
  bVar7 = FUN_142f04924();
  uVar11 = 0;
  *(byte *)((longlong)DAT_143ade408 + 4) = bVar7;
  puVar6 = DAT_143ade408;
  lVar2 = -(longlong)DAT_143ade408;
  lVar3 = -(longlong)DAT_143ade408;
  lVar4 = -(longlong)DAT_143ade408;
  lVar5 = -(longlong)DAT_143ade408;
  *(undefined2 *)(DAT_143ade408 + 1) = 0x9a65;
  pbVar12 = (byte *)((longlong)puVar6 + 2);
  do {
    pbVar1 = pbVar12 + 4;
    if (bVar7 == 0) {
      bVar7 = 0x2a;
    }
    bVar8 = pbVar12[(longlong)(&stack0x00000006 + lVar2)];
    pbVar12[-2] = bVar7 ^ bVar8;
    bVar7 = bVar7 + (bVar7 ^ bVar8) + 0x2a;
    *(ushort *)(DAT_143ade408 + 1) =
         (*(ushort *)(DAT_143ade408 + 1) >> 0xd) + (ushort)bVar7 |
         *(ushort *)(DAT_143ade408 + 1) << 3;
    bVar8 = 0x2a;
    if (bVar7 != 0) {
      bVar8 = bVar7;
    }
    bVar7 = pbVar1[(longlong)(&stack0x00000003 + lVar3)];
    pbVar12[-1] = bVar8 ^ bVar7;
    bVar8 = (bVar8 ^ bVar7) + bVar8 + 0x2a;
    *(ushort *)(DAT_143ade408 + 1) =
         (*(ushort *)(DAT_143ade408 + 1) >> 0xd) + (ushort)bVar8 |
         *(ushort *)(DAT_143ade408 + 1) << 3;
    bVar7 = 0x2a;
    if (bVar8 != 0) {
      bVar7 = bVar8;
    }
    bVar8 = pbVar1[(longlong)(&stack0x00000004 + lVar4)];
    *pbVar12 = bVar7 ^ bVar8;
    bVar8 = (bVar7 ^ bVar8) + bVar7 + 0x2a;
    *(ushort *)(DAT_143ade408 + 1) =
         (*(ushort *)(DAT_143ade408 + 1) >> 0xd) + (ushort)bVar8 |
         *(ushort *)(DAT_143ade408 + 1) << 3;
    bVar7 = 0x2a;
    if (bVar8 != 0) {
      bVar7 = bVar8;
    }
    uVar11 = uVar11 + 4;
    bVar8 = pbVar1[(longlong)(&stack0x00000005 + lVar5)];
    pbVar12[1] = bVar7 ^ bVar8;
    bVar7 = (bVar7 ^ bVar8) + bVar7 + 0x2a;
    *(ushort *)(DAT_143ade408 + 1) =
         (*(ushort *)(DAT_143ade408 + 1) >> 0xd) + (ushort)bVar7 |
         *(ushort *)(DAT_143ade408 + 1) << 3;
    pbVar12 = pbVar1;
  } while (uVar11 < 4);
  return;
}



//===========================================================
// FUN_142c96210 @ 142c96210   (517 bytes)
//===========================================================

void FUN_142c96210(void)

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
  
  puVar6 = DAT_143ade408;
  uVar10 = 0;
  DAT_143ade400 = DAT_143ade400 + 1;
  local_res8[0] = 0;
  local_res8[1] = 0;
  local_res8[2] = 0;
  local_res8[3] = 0;
  if (DAT_143ade400 == (DAT_143ade400 / 0x6f) * 0x6f) {
    puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    DAT_143ade408 = puVar9;
    *puVar9 = *puVar6;
    *(undefined4 *)(puVar9 + 1) = *(undefined4 *)(puVar6 + 1);
    thunk_FUN_140205820(puVar6,0xc);
  }
  bVar7 = FUN_142f04924();
  *(byte *)((longlong)DAT_143ade408 + 4) = bVar7;
  puVar6 = DAT_143ade408;
  lVar2 = -(longlong)DAT_143ade408;
  lVar3 = -(longlong)DAT_143ade408;
  lVar4 = -(longlong)DAT_143ade408;
  lVar5 = -(longlong)DAT_143ade408;
  *(undefined2 *)(DAT_143ade408 + 1) = 0x9a65;
  pbVar11 = (byte *)((longlong)puVar6 + 2);
  do {
    pbVar1 = pbVar11 + 4;
    if (bVar7 == 0) {
      bVar7 = 0x2a;
    }
    bVar8 = pbVar11[(longlong)(&stack0x00000006 + lVar2)];
    pbVar11[-2] = bVar7 ^ bVar8;
    bVar7 = bVar7 + (bVar7 ^ bVar8) + 0x2a;
    *(ushort *)(DAT_143ade408 + 1) =
         (*(ushort *)(DAT_143ade408 + 1) >> 0xd) + (ushort)bVar7 |
         *(ushort *)(DAT_143ade408 + 1) << 3;
    bVar8 = 0x2a;
    if (bVar7 != 0) {
      bVar8 = bVar7;
    }
    bVar7 = pbVar1[(longlong)(&stack0x00000003 + lVar3)];
    pbVar11[-1] = bVar8 ^ bVar7;
    bVar8 = (bVar8 ^ bVar7) + bVar8 + 0x2a;
    *(ushort *)(DAT_143ade408 + 1) =
         (*(ushort *)(DAT_143ade408 + 1) >> 0xd) + (ushort)bVar8 |
         *(ushort *)(DAT_143ade408 + 1) << 3;
    bVar7 = 0x2a;
    if (bVar8 != 0) {
      bVar7 = bVar8;
    }
    bVar8 = pbVar1[(longlong)(&stack0x00000004 + lVar4)];
    *pbVar11 = bVar7 ^ bVar8;
    bVar8 = (bVar7 ^ bVar8) + bVar7 + 0x2a;
    *(ushort *)(DAT_143ade408 + 1) =
         (*(ushort *)(DAT_143ade408 + 1) >> 0xd) + (ushort)bVar8 |
         *(ushort *)(DAT_143ade408 + 1) << 3;
    bVar7 = 0x2a;
    if (bVar8 != 0) {
      bVar7 = bVar8;
    }
    uVar10 = uVar10 + 4;
    bVar8 = pbVar1[(longlong)(&stack0x00000005 + lVar5)];
    pbVar11[1] = bVar7 ^ bVar8;
    bVar7 = (bVar7 ^ bVar8) + bVar7 + 0x2a;
    *(ushort *)(DAT_143ade408 + 1) =
         (*(ushort *)(DAT_143ade408 + 1) >> 0xd) + (ushort)bVar7 |
         *(ushort *)(DAT_143ade408 + 1) << 3;
    pbVar11 = pbVar1;
  } while (uVar10 < 4);
  return;
}



//===========================================================
// FUN_142c95d20 @ 142c95d20   (1 bytes)
//===========================================================

undefined4 FUN_142c95d20(longlong param_1)

{
  return *(undefined4 *)(param_1 + 0x50);
}



//===========================================================
// FUN_142c95d30 @ 142c95d30   (1 bytes)
//===========================================================

undefined1 FUN_142c95d30(longlong param_1)

{
  return *(undefined1 *)(param_1 + 0x54);
}


