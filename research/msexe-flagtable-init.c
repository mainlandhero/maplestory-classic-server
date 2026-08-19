
//===========================================================
// FUN_1400239d0 @ 1400239d0   (200 bytes)
//===========================================================

void FUN_1400239d0(void)

{
  undefined4 *puVar1;
  
  puVar1 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x13);
  puVar1[1] = 2;
  DAT_143abfcf8 = puVar1 + 4;
  *puVar1 = 0xffffffff;
  puVar1[2] = 0;
  *(undefined1 *)DAT_143abfcf8 = 0;
  *(undefined2 *)DAT_143abfcf8 = DAT_14327cd28;
  puVar1 = DAT_143abfcf8;
  if (DAT_143abfcf8[-4] != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((int)puVar1[-3] < 2) {
    FUN_142e54290(0x90,puVar1[-3],2);
  }
  puVar1[-4] = 1;
  *(undefined1 *)((longlong)DAT_143abfcf8 + 2) = 0;
  if (puVar1[-3] + 1 < 3) {
    FUN_142e54290(0x9c,2);
  }
  puVar1[-2] = 2;
  atexit((_func_5014 *)&LAB_143217ef0);
  return;
}



//===========================================================
// FUN_140023290 @ 140023290   (30 bytes)
//===========================================================

void FUN_140023290(void)

{
  FUN_140302c70(&DAT_143abee20,0);
  DAT_143abee20 = 1;
  return;
}



//===========================================================
// FUN_140023250 @ 140023250   (30 bytes)
//===========================================================

void FUN_140023250(void)

{
  FUN_140302c70(&DAT_143abf600,0);
  DAT_143abf611 = 1;
  return;
}



//===========================================================
// FUN_14019b600 @ 14019b600   (375 bytes)
//===========================================================

void FUN_14019b600(longlong param_1,ulonglong param_2)

{
  longlong *plVar1;
  int *piVar2;
  void *pvVar3;
  longlong lVar4;
  undefined8 *puVar5;
  int iVar6;
  undefined8 uVar7;
  longlong lVar8;
  int *piVar9;
  uint uVar10;
  longlong lVar11;
  
  pvVar3 = Self;
  uVar7 = 0x98;
  if (param_2 < 0x39) {
    uVar10 = (uint)(0x28 < param_2);
LAB_14019b65e:
    if ((int)uVar10 < 0) {
      FUN_14019d350(param_2);
      return;
    }
    if (uVar10 == 0) {
      iVar6 = 0x40;
      uVar7 = 0x28;
      goto LAB_14019b6a6;
    }
    if (uVar10 == 1) {
      iVar6 = 0x20;
      uVar7 = 0x38;
      goto LAB_14019b6a6;
    }
    if (uVar10 != 2) {
      if (uVar10 == 3) {
        iVar6 = 8;
      }
      else {
        iVar6 = 0;
        uVar7 = 0;
      }
      goto LAB_14019b6a6;
    }
  }
  else {
    if (0x58 < param_2) {
      uVar10 = 0xffffffff;
      if (param_2 < 0x99) {
        uVar10 = 3;
      }
      goto LAB_14019b65e;
    }
    uVar10 = 2;
  }
  iVar6 = 0x10;
  uVar7 = 0x58;
LAB_14019b6a6:
  lVar11 = (longlong)(int)uVar10;
  lVar8 = lVar11 * 0x10 + param_1;
  plVar1 = (longlong *)(lVar8 + 0x28);
  LOCK();
  lVar4 = *plVar1;
  if (lVar4 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  if (lVar4 == 0) {
LAB_14019b709:
    *(undefined4 *)(lVar8 + 0x30) = 1;
  }
  else if ((void *)*plVar1 == pvVar3) {
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  else {
    while( true ) {
      pvVar3 = Self;
      LOCK();
      lVar4 = *plVar1;
      if (lVar4 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar4 == 0) goto LAB_14019b709;
      if ((void *)*plVar1 == pvVar3) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  piVar9 = (int *)(lVar8 + 0x30);
  puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  if (puVar5 == (undefined8 *)0x0) {
    lVar4 = FUN_14019d3c0(uVar7,iVar6);
    *(undefined8 *)(lVar4 + -0x10) = *(undefined8 *)(param_1 + 0x88 + lVar11 * 8);
    *(longlong *)(param_1 + 0x88 + lVar11 * 8) = lVar4;
    *(longlong *)(param_1 + 0x68 + lVar11 * 8) = lVar4;
    piVar2 = (int *)(param_1 + 4 + lVar11 * 4);
    *piVar2 = *piVar2 + iVar6;
    puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  }
  piVar2 = (int *)(param_1 + 0x14 + lVar11 * 4);
  *piVar2 = *piVar2 + 1;
  *(undefined8 *)(param_1 + 0x68 + lVar11 * 8) = *puVar5;
  *piVar9 = *piVar9 + -1;
  if (*piVar9 == 0) {
    *plVar1 = 0;
  }
  return;
}



//===========================================================
// atexit @ 142ef3df8   (23 bytes)
//===========================================================

/* Library Function - Single Match
    atexit
   
   Library: Visual Studio 2019 Release */

int __cdecl atexit(_func_5014 *param_1)

{
  _onexit_t p_Var1;
  
  p_Var1 = _onexit((_onexit_t)param_1);
  return (p_Var1 != (_onexit_t)0x0) - 1;
}



//===========================================================
// FUN_142e54290 @ 142e54290   (185 bytes)
//===========================================================

void FUN_142e54290(undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  char cVar1;
  undefined4 local_res8 [2];
  undefined4 local_res10 [2];
  undefined4 local_res18 [2];
  undefined4 local_res20 [2];
  longlong local_18 [3];
  
  local_res8[0] = param_1;
  local_res10[0] = param_2;
  local_res18[0] = param_3;
  cVar1 = FUN_142e559e0();
  if (cVar1 != '\0') {
    FUN_140194c60(local_18);
    local_res20[0] = FUN_14091a3e0(local_18);
    FUN_142e5c990("LogCallStack5",&DAT_1434997dc,local_res20,&DAT_1434997f8,local_res8,"Info1",
                  local_res10,"info2",local_res18,local_18);
    if (local_18[0] != 0) {
      FUN_14019f2c0(local_18[0] + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_142e52dd0 @ 142e52dd0   (245 bytes)
//===========================================================

void FUN_142e52dd0(undefined4 param_1,undefined4 param_2)

{
  undefined8 uVar1;
  char cVar2;
  undefined4 local_res8 [2];
  undefined4 local_res10 [2];
  undefined4 local_res18 [2];
  longlong local_res20;
  longlong local_18;
  longlong local_10 [2];
  
  local_res8[0] = param_1;
  local_res10[0] = param_2;
  cVar2 = FUN_142e559e0();
  if (cVar2 != '\0') {
    FUN_140194c60(&local_res20);
    local_res18[0] = FUN_14091a3e0(&local_res20);
    uVar1 = FUN_142a1d8a0(local_10);
    uVar1 = FUN_142e5d800(&local_18,uVar1,"LogCallStack2",&DAT_1434997dc,local_res18,&DAT_1434997f8,
                          local_res8,"Info1",local_res10,&local_res20);
    FUN_142a1ec10(uVar1);
    if (local_18 != 0) {
      FUN_14019f2c0(local_18 + -0x10);
    }
    if (local_10[0] != 0) {
      FUN_14019f2c0(local_10[0] + -0x10);
    }
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_140302c70 @ 140302c70   (337 bytes)
//===========================================================

longlong FUN_140302c70(longlong param_1,undefined1 param_2)

{
  undefined1 *puVar1;
  uint uVar2;
  
  uVar2 = 0;
  puVar1 = (undefined1 *)(param_1 + 2);
  do {
    puVar1[-2] = param_2;
    puVar1[-1] = param_2;
    *puVar1 = param_2;
    puVar1[1] = param_2;
    puVar1[2] = param_2;
    puVar1[3] = param_2;
    puVar1[4] = param_2;
    puVar1[5] = param_2;
    puVar1[6] = param_2;
    puVar1[7] = param_2;
    puVar1[8] = param_2;
    puVar1[9] = param_2;
    puVar1[10] = param_2;
    puVar1[0xb] = param_2;
    puVar1[0xc] = param_2;
    puVar1[0xd] = param_2;
    puVar1[0xe] = param_2;
    puVar1[0xf] = param_2;
    puVar1[0x10] = param_2;
    puVar1[0x11] = param_2;
    puVar1[0x12] = param_2;
    puVar1[0x13] = param_2;
    puVar1[0x14] = param_2;
    puVar1[0x15] = param_2;
    puVar1[0x16] = param_2;
    puVar1[0x17] = param_2;
    puVar1[0x18] = param_2;
    puVar1[0x19] = param_2;
    puVar1[0x1a] = param_2;
    puVar1[0x1b] = param_2;
    puVar1[0x1c] = param_2;
    puVar1[0x1d] = param_2;
    puVar1[0x1e] = param_2;
    puVar1[0x1f] = param_2;
    puVar1[0x20] = param_2;
    puVar1[0x21] = param_2;
    puVar1[0x22] = param_2;
    puVar1[0x23] = param_2;
    puVar1[0x24] = param_2;
    puVar1[0x25] = param_2;
    puVar1[0x26] = param_2;
    puVar1[0x27] = param_2;
    puVar1[0x28] = param_2;
    puVar1[0x29] = param_2;
    puVar1[0x2a] = param_2;
    puVar1[0x2b] = param_2;
    puVar1[0x2c] = param_2;
    puVar1[0x2d] = param_2;
    puVar1[0x2e] = param_2;
    puVar1[0x2f] = param_2;
    puVar1[0x30] = param_2;
    puVar1[0x31] = param_2;
    puVar1[0x32] = param_2;
    puVar1[0x33] = param_2;
    puVar1[0x34] = param_2;
    puVar1[0x35] = param_2;
    puVar1[0x36] = param_2;
    puVar1[0x37] = param_2;
    puVar1[0x38] = param_2;
    puVar1[0x39] = param_2;
    puVar1[0x3a] = param_2;
    puVar1[0x3b] = param_2;
    puVar1[0x3c] = param_2;
    puVar1[0x3d] = param_2;
    puVar1[0x3e] = param_2;
    puVar1[0x3f] = param_2;
    puVar1[0x40] = param_2;
    puVar1[0x41] = param_2;
    puVar1[0x42] = param_2;
    puVar1[0x43] = param_2;
    puVar1[0x44] = param_2;
    puVar1[0x45] = param_2;
    puVar1[0x46] = param_2;
    puVar1[0x47] = param_2;
    puVar1[0x48] = param_2;
    puVar1[0x49] = param_2;
    puVar1[0x4a] = param_2;
    puVar1[0x4b] = param_2;
    puVar1[0x4c] = param_2;
    puVar1[0x4d] = param_2;
    puVar1[0x4e] = param_2;
    puVar1[0x4f] = param_2;
    uVar2 = uVar2 + 100;
    puVar1[0x50] = param_2;
    puVar1[0x51] = param_2;
    puVar1[0x52] = param_2;
    puVar1[0x53] = param_2;
    puVar1[0x54] = param_2;
    puVar1[0x55] = param_2;
    puVar1[0x56] = param_2;
    puVar1[0x57] = param_2;
    puVar1[0x58] = param_2;
    puVar1[0x59] = param_2;
    puVar1[0x5a] = param_2;
    puVar1[0x5b] = param_2;
    puVar1[0x5c] = param_2;
    puVar1[0x5d] = param_2;
    puVar1[0x5e] = param_2;
    puVar1[0x5f] = param_2;
    puVar1[0x60] = param_2;
    puVar1[0x61] = param_2;
    puVar1 = puVar1 + 100;
  } while (uVar2 < 100);
  return param_1;
}



//===========================================================
// FUN_14019d350 @ 14019d350   (105 bytes)
//===========================================================

longlong * FUN_14019d350(longlong param_1,ulonglong param_2)

{
  code *pcVar1;
  undefined8 uVar2;
  longlong *plVar3;
  
  if (0xc7fffff < param_2) {
    FUN_142e541f0(0x3a);
  }
  pcVar1 = DAT_143ad5528;
  uVar2 = (*DAT_143ad5538)();
  plVar3 = (longlong *)(*pcVar1)(uVar2,0,param_1 + 8);
  if (plVar3 != (longlong *)0x0) {
    *plVar3 = param_1;
    return plVar3 + 1;
  }
  return (longlong *)0x0;
}



//===========================================================
// FUN_14019d3c0 @ 14019d3c0   (144 bytes)
//===========================================================

undefined8 * FUN_14019d3c0(longlong param_1,longlong param_2)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  
  puVar3 = (undefined8 *)FUN_14019d350((param_1 + 8) * param_2 + 8,param_1 * param_2);
  *puVar3 = 0;
  puVar1 = puVar3 + 2;
  puVar3[1] = param_1;
  param_2 = param_2 + -1;
  puVar3 = puVar1;
  if (param_2 == 0) {
    *puVar1 = 0;
    return puVar1;
  }
  do {
    puVar2 = (undefined8 *)((longlong)puVar3 + param_1 + 8);
    *puVar3 = puVar2;
    puVar2[-1] = param_1;
    param_2 = param_2 + -1;
    puVar3 = puVar2;
  } while (param_2 != 0);
  *puVar2 = 0;
  return puVar1;
}



//===========================================================
// _onexit @ 142ef3dbc   (58 bytes)
//===========================================================

/* Library Function - Single Match
    _onexit
   
   Library: Visual Studio 2019 Release */

_onexit_t __cdecl _onexit(_onexit_t _Func)

{
  int iVar1;
  _onexit_t p_Var2;
  
  if (DAT_143ae2618 == -1) {
    iVar1 = FUN_142f2f4e0();
  }
  else {
    iVar1 = _register_onexit_function(&DAT_143ae2618);
  }
  p_Var2 = (_onexit_t)0x0;
  if (iVar1 == 0) {
    p_Var2 = _Func;
  }
  return p_Var2;
}



//===========================================================
// FUN_140194c60 @ 140194c60   (6267 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 * FUN_140194c60(undefined8 *param_1)

{
  code *pcVar1;
  undefined8 uVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  int iVar6;
  longlong *plVar7;
  int *piVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  ulonglong *puVar11;
  undefined4 *puVar12;
  longlong lVar13;
  longlong lVar14;
  char *pcVar15;
  undefined8 uVar16;
  int iVar17;
  int *piVar18;
  int *piVar19;
  int *piVar20;
  ulonglong uVar21;
  undefined8 *puVar22;
  int iVar23;
  int *piVar24;
  undefined1 auStack_aa8 [32];
  int **local_a88;
  int **local_a80;
  undefined8 local_a78;
  undefined8 local_a70;
  undefined8 local_a68;
  int *local_a58;
  longlong local_a50;
  undefined8 *local_a48;
  ulonglong local_a40;
  int local_a38;
  ulonglong local_a30;
  longlong local_a28;
  undefined8 *local_a20;
  undefined4 local_a18 [2];
  undefined8 local_a10;
  undefined8 uStack_a08;
  undefined8 local_a00;
  undefined8 uStack_9f8;
  undefined8 local_9e8;
  undefined1 local_9e0 [4];
  undefined4 local_9dc;
  longlong local_9c8;
  undefined4 local_9bc;
  undefined8 local_9b8;
  undefined4 local_9ac;
  undefined4 local_8d8;
  undefined8 local_8d4;
  undefined8 uStack_8cc;
  undefined8 local_8c4;
  undefined8 uStack_8bc;
  undefined8 local_8b4;
  undefined8 uStack_8ac;
  undefined8 local_8a4;
  undefined8 uStack_89c;
  undefined8 local_894;
  undefined8 uStack_88c;
  undefined8 local_884;
  undefined8 uStack_87c;
  undefined8 local_874;
  undefined8 uStack_86c;
  undefined8 local_864;
  undefined8 uStack_85c;
  undefined8 local_854;
  undefined8 uStack_84c;
  undefined1 local_838 [48];
  undefined4 local_808;
  undefined8 local_7a0;
  longlong local_798;
  undefined8 local_740;
  int *local_368 [34];
  undefined4 local_258 [6];
  undefined4 local_240;
  undefined1 local_23c [516];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_aa8;
  piVar24 = (int *)0x0;
  iVar3 = 0;
  local_a58 = (int *)0x0;
  local_a48 = param_1;
  local_a20 = param_1;
  if (DAT_143aa8288 == '\0') {
    local_a50 = 0;
    plVar7 = (longlong *)FUN_14019ba10(&local_a50,"Not Init\r\n");
    lVar14 = *plVar7;
    piVar18 = piVar24;
    if (lVar14 == 0) goto LAB_140194d86;
    iVar4 = *(int *)(lVar14 + -8);
    piVar20 = (int *)(longlong)iVar4;
    param_1 = local_a20;
    if (iVar4 == 0) goto LAB_140194d86;
    if (0 < iVar4) {
      iVar3 = iVar4;
    }
    piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
    piVar8[1] = iVar3;
    *piVar8 = -1;
    piVar18 = piVar8 + 4;
    piVar8[2] = 0;
    *(char *)piVar18 = '\0';
    local_a58 = piVar18;
    FUN_142ef7ba0(piVar18,lVar14,piVar20);
    if (*piVar8 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar4 == -1) || (iVar4 <= piVar8[1])) {
      *piVar8 = 1;
      if (iVar4 != -1) goto LAB_140194d5f;
      piVar20 = (int *)0xffffffffffffffff;
      if (piVar18 != (int *)0x0) {
        do {
          piVar24 = (int *)((longlong)piVar20 + 1);
          piVar20 = piVar24;
        } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,piVar8[1],iVar4);
      *piVar8 = 1;
LAB_140194d5f:
      *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
      piVar24 = piVar20;
    }
    iVar3 = (int)piVar24;
    if ((iVar3 < 0) || (piVar8[1] + 1 <= iVar3)) {
      FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
    }
    piVar8[2] = iVar3;
    param_1 = local_a20;
LAB_140194d86:
    if (local_a50 != 0) {
      FUN_14019f2c0(local_a50 + -0x10);
    }
    *param_1 = piVar18;
    return param_1;
  }
  FUN_142ef8250(local_838,0,0x4d0);
  local_808 = 0x10001f;
  (*DAT_143262840)(local_838);
  local_a50 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a50,&DAT_143271d00);
  lVar14 = *plVar7;
  piVar18 = piVar24;
  if (lVar14 != 0) {
    iVar4 = *(int *)(lVar14 + -8);
    piVar20 = (int *)(longlong)iVar4;
    piVar18 = (int *)0x0;
    if (iVar4 != 0) {
      iVar6 = 0;
      if (0 < iVar4) {
        iVar6 = iVar4;
      }
      piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
      piVar8[1] = iVar6;
      *piVar8 = -1;
      piVar18 = piVar8 + 4;
      piVar8[2] = 0;
      *(char *)piVar18 = '\0';
      local_a58 = piVar18;
      FUN_142ef7ba0(piVar18,lVar14,piVar20);
      if (*piVar8 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar4 == -1) || (iVar4 <= piVar8[1])) {
        *piVar8 = 1;
        if (iVar4 != -1) goto LAB_140194ea0;
        piVar20 = piVar24;
        if (piVar18 != (int *)0x0) {
          piVar20 = (int *)0xffffffffffffffff;
          do {
            piVar20 = (int *)((longlong)piVar20 + 1);
          } while (*(char *)((longlong)piVar18 + (longlong)piVar20) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar8[1],iVar4);
        *piVar8 = 1;
LAB_140194ea0:
        *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
      }
      iVar4 = (int)piVar20;
      if ((iVar4 < 0) || (piVar8[1] + 1 <= iVar4)) {
        FUN_142e54290(0x9c,(ulonglong)piVar20 & 0xffffffff);
      }
      piVar8[2] = iVar4;
    }
  }
  if (local_a50 != 0) {
    FUN_14019f2c0(local_a50 + -0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Major:%d Minor:%d\r\n",1);
  lVar14 = *plVar7;
  piVar20 = piVar18;
  local_a50 = lVar14;
  if (lVar14 != 0) {
    iVar4 = *(int *)(lVar14 + -8);
    piVar8 = (int *)(longlong)iVar4;
    if (iVar4 != 0) {
      piVar19 = piVar24;
      if (piVar18 == (int *)0x0) goto LAB_14019509b;
      if ((char)*piVar18 != '\0') {
        iVar6 = piVar18[-2];
        for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
        }
        piVar24 = piVar18 + -4;
        if (piVar24 == (int *)0x0) {
LAB_140194f9f:
          if (iVar3 < iVar17) {
            iVar3 = iVar17;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
          puVar12[1] = iVar3;
          *puVar12 = 0xffffffff;
          piVar20 = puVar12 + 4;
          local_a58 = piVar20;
          if (piVar24 == (int *)0x0) {
            puVar12[2] = 0;
            *(char *)piVar20 = '\0';
            lVar14 = local_a50;
          }
          else {
            iVar17 = piVar18[-2] + 1;
            iVar23 = iVar3 + 1;
            if (iVar23 < iVar17) {
              FUN_142e54290(0x5c,iVar17,iVar23);
              iVar17 = iVar23;
            }
            FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
            puVar12[2] = piVar18[-2];
            *(char *)((longlong)iVar3 + (longlong)piVar20) = '\0';
            FUN_14019f2c0(piVar24);
            lVar14 = local_a50;
          }
        }
        else {
          if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
            iVar3 = piVar18[-2];
            goto LAB_140194f9f;
          }
          if (*piVar24 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar24 = -1;
        }
        if (piVar20 == (int *)0x0) {
          iVar3 = 0;
        }
        else {
          iVar3 = piVar20[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar20),lVar14,piVar8);
        FUN_14019c870(&local_a58,iVar6 + iVar4);
        goto LAB_140195140;
      }
      if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_14019509b:
        if (iVar3 < iVar4) {
          iVar3 = iVar4;
        }
        puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
        puVar12[1] = iVar3;
        *puVar12 = 0xffffffff;
        piVar20 = puVar12 + 4;
        puVar12[2] = 0;
        *(char *)piVar20 = '\0';
        local_a58 = piVar20;
        if (piVar19 != (int *)0x0) {
          FUN_14019f2c0(piVar19);
        }
      }
      else {
        if ((1 < *piVar19) || (piVar18[-3] < iVar4)) {
          iVar3 = piVar18[-2];
          goto LAB_14019509b;
        }
        if (*piVar19 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar19 = -1;
      }
      FUN_142ef7ba0(piVar20,lVar14,piVar8);
      if (piVar20[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar4 == -1) || (iVar4 <= piVar20[-3])) {
        piVar20[-4] = 1;
        if (iVar4 != -1) goto LAB_14019511d;
        if (piVar20 != (int *)0x0) {
          piVar24 = (int *)0xffffffffffffffff;
          do {
            piVar24 = (int *)((longlong)piVar24 + 1);
          } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar20[-3],iVar4);
        piVar20[-4] = 1;
LAB_14019511d:
        *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
        piVar24 = piVar8;
      }
      iVar3 = (int)piVar24;
      if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
        FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
      }
      piVar20[-2] = iVar3;
    }
  }
LAB_140195140:
  piVar24 = (int *)0x0;
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Call stack:\r\n");
  lVar14 = *plVar7;
  piVar18 = piVar20;
  local_a50 = lVar14;
  if (lVar14 != 0) {
    iVar3 = *(int *)(lVar14 + -8);
    piVar8 = (int *)(longlong)iVar3;
    if (iVar3 != 0) {
      iVar4 = 0;
      piVar19 = piVar24;
      if (piVar20 == (int *)0x0) goto LAB_14019530e;
      if ((char)*piVar20 != '\0') {
        iVar6 = piVar20[-2];
        for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
        }
        piVar24 = piVar20 + -4;
        if (piVar24 == (int *)0x0) {
LAB_140195212:
          if (iVar4 < iVar17) {
            iVar4 = iVar17;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
          puVar12[1] = iVar4;
          *puVar12 = 0xffffffff;
          piVar18 = puVar12 + 4;
          local_a58 = piVar18;
          if (piVar24 == (int *)0x0) {
            puVar12[2] = 0;
            *(char *)piVar18 = '\0';
            lVar14 = local_a50;
          }
          else {
            iVar17 = piVar20[-2] + 1;
            iVar23 = iVar4 + 1;
            if (iVar23 < iVar17) {
              FUN_142e54290(0x5c,iVar17,iVar23);
              iVar17 = iVar23;
            }
            FUN_142ef7ba0(piVar18,piVar20,(longlong)iVar17);
            puVar12[2] = piVar20[-2];
            *(char *)((longlong)iVar4 + (longlong)piVar18) = '\0';
            FUN_14019f2c0(piVar24);
            lVar14 = local_a50;
          }
        }
        else {
          if ((1 < *piVar24) || (piVar20[-3] < iVar17)) {
            iVar4 = piVar20[-2];
            goto LAB_140195212;
          }
          if (*piVar24 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar24 = -1;
        }
        if (piVar18 == (int *)0x0) {
          iVar4 = 0;
        }
        else {
          iVar4 = piVar18[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar18),lVar14,piVar8);
        FUN_14019c870(&local_a58,iVar6 + iVar3);
        goto LAB_1401953b3;
      }
      if ((piVar20 == (int *)0x0) || (piVar19 = piVar20 + -4, piVar19 == (int *)0x0)) {
LAB_14019530e:
        if (iVar4 < iVar3) {
          iVar4 = iVar3;
        }
        puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
        puVar12[1] = iVar4;
        *puVar12 = 0xffffffff;
        piVar18 = puVar12 + 4;
        puVar12[2] = 0;
        *(char *)piVar18 = '\0';
        local_a58 = piVar18;
        if (piVar19 != (int *)0x0) {
          FUN_14019f2c0(piVar19);
        }
      }
      else {
        if ((1 < *piVar19) || (piVar20[-3] < iVar3)) {
          iVar4 = piVar20[-2];
          goto LAB_14019530e;
        }
        if (*piVar19 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar19 = -1;
      }
      FUN_142ef7ba0(piVar18,lVar14,piVar8);
      if (piVar18[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar3 == -1) || (iVar3 <= piVar18[-3])) {
        piVar18[-4] = 1;
        if (iVar3 != -1) goto LAB_140195390;
        if (piVar18 != (int *)0x0) {
          piVar24 = (int *)0xffffffffffffffff;
          do {
            piVar24 = (int *)((longlong)piVar24 + 1);
          } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar18[-3],iVar3);
        piVar18[-4] = 1;
LAB_140195390:
        *(char *)((longlong)piVar8 + (longlong)piVar18) = '\0';
        piVar24 = piVar8;
      }
      iVar3 = (int)piVar24;
      if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
        FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
      }
      piVar18[-2] = iVar3;
    }
  }
LAB_1401953b3:
  piVar24 = (int *)0x0;
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Address   Frame\r\n");
  lVar14 = *plVar7;
  piVar20 = piVar18;
  local_a50 = lVar14;
  if (lVar14 == 0) goto LAB_140195630;
  iVar3 = *(int *)(lVar14 + -8);
  piVar8 = (int *)(longlong)iVar3;
  if (iVar3 == 0) goto LAB_140195630;
  iVar4 = 0;
  piVar19 = piVar24;
  if (piVar18 == (int *)0x0) goto LAB_14019558b;
  if ((char)*piVar18 != '\0') {
    iVar6 = piVar18[-2];
    for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
    }
    piVar24 = piVar18 + -4;
    if (piVar24 == (int *)0x0) {
LAB_14019548f:
      if (iVar4 < iVar17) {
        iVar4 = iVar17;
      }
      puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
      puVar12[1] = iVar4;
      *puVar12 = 0xffffffff;
      piVar20 = puVar12 + 4;
      local_a58 = piVar20;
      if (piVar24 == (int *)0x0) {
        puVar12[2] = 0;
        *(char *)piVar20 = '\0';
        lVar14 = local_a50;
      }
      else {
        iVar17 = piVar18[-2] + 1;
        iVar23 = iVar4 + 1;
        if (iVar23 < iVar17) {
          FUN_142e54290(0x5c,iVar17,iVar23);
          iVar17 = iVar23;
        }
        FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
        puVar12[2] = piVar18[-2];
        *(char *)((longlong)iVar4 + (longlong)piVar20) = '\0';
        FUN_14019f2c0(piVar24);
        lVar14 = local_a50;
      }
    }
    else {
      if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
        iVar4 = piVar18[-2];
        goto LAB_14019548f;
      }
      if (*piVar24 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar24 = -1;
    }
    if (piVar20 == (int *)0x0) {
      iVar4 = 0;
    }
    else {
      iVar4 = piVar20[-2];
    }
    FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar20),lVar14,piVar8);
    FUN_14019c870(&local_a58,iVar6 + iVar3);
    goto LAB_140195630;
  }
  if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_14019558b:
    if (iVar4 < iVar3) {
      iVar4 = iVar3;
    }
    puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
    puVar12[1] = iVar4;
    *puVar12 = 0xffffffff;
    piVar20 = puVar12 + 4;
    puVar12[2] = 0;
    *(char *)piVar20 = '\0';
    local_a58 = piVar20;
    if (piVar19 != (int *)0x0) {
      FUN_14019f2c0(piVar19);
    }
  }
  else {
    if ((1 < *piVar19) || (piVar18[-3] < iVar3)) {
      iVar4 = piVar18[-2];
      goto LAB_14019558b;
    }
    if (*piVar19 != 1) {
      FUN_142e52dd0(0x74);
    }
    *piVar19 = -1;
  }
  FUN_142ef7ba0(piVar20,lVar14,piVar8);
  if (piVar20[-4] != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar3 == -1) || (iVar3 <= piVar20[-3])) {
    piVar20[-4] = 1;
    if (iVar3 != -1) goto LAB_14019560d;
    if (piVar20 != (int *)0x0) {
      piVar24 = (int *)0xffffffffffffffff;
      do {
        piVar24 = (int *)((longlong)piVar24 + 1);
      } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
    }
  }
  else {
    FUN_142e54290(0x90,piVar20[-3],iVar3);
    piVar20[-4] = 1;
LAB_14019560d:
    *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
    piVar24 = piVar8;
  }
  iVar3 = (int)piVar24;
  if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
    FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
  }
  piVar20[-2] = iVar3;
LAB_140195630:
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  FUN_142ef8250(local_9e0,0,0x100);
  uVar2 = DAT_143aa8260;
  uVar16 = DAT_143aa8258;
  pcVar1 = DAT_143aa8250;
  local_9e8 = local_740;
  local_9dc = 3;
  local_9b8 = local_7a0;
  local_9ac = 3;
  local_9c8 = local_798;
  local_9bc = 3;
  uVar9 = (*DAT_143ad5440)();
  uVar10 = (*DAT_143ad5408)();
  local_a68 = 0;
  local_a70 = uVar2;
  local_a78 = uVar16;
  local_a80 = (int **)0x0;
  local_a88 = (int **)local_838;
  iVar3 = (*pcVar1)(0x8664,uVar10,uVar9,&local_9e8);
  do {
    if ((iVar3 == 0) || (piVar24 = (int *)0x0, local_9c8 == 0)) {
      *local_a20 = piVar20;
      return local_a20;
    }
    local_a30 = 0;
    puVar11 = (ulonglong *)FUN_14019ba10(&local_a30,"%016X  %016X  ",local_9e8);
    uVar21 = *puVar11;
    piVar18 = piVar20;
    local_a40 = uVar21;
    if (uVar21 != 0) {
      iVar3 = *(int *)(uVar21 - 8);
      piVar8 = (int *)(longlong)iVar3;
      if (iVar3 != 0) {
        iVar4 = 0;
        piVar19 = piVar24;
        if (piVar20 == (int *)0x0) goto LAB_1401958b6;
        if ((char)*piVar20 != '\0') {
          iVar6 = piVar20[-2];
          for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
          }
          piVar24 = piVar20 + -4;
          if (piVar24 == (int *)0x0) {
LAB_1401957c2:
            if (iVar4 < iVar17) {
              iVar4 = iVar17;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
            puVar12[1] = iVar4;
            *puVar12 = 0xffffffff;
            piVar18 = puVar12 + 4;
            local_a58 = piVar18;
            if (piVar24 == (int *)0x0) {
              puVar12[2] = 0;
              *(char *)piVar18 = '\0';
              uVar21 = local_a40;
            }
            else {
              iVar17 = piVar20[-2] + 1;
              iVar23 = iVar4 + 1;
              if (iVar23 < iVar17) {
                FUN_142e54290(0x5c,iVar17,iVar23);
                iVar17 = iVar23;
              }
              FUN_142ef7ba0(piVar18,piVar20,(longlong)iVar17);
              puVar12[2] = piVar20[-2];
              *(char *)((longlong)iVar4 + (longlong)piVar18) = '\0';
              FUN_14019f2c0(piVar24);
              uVar21 = local_a40;
            }
          }
          else {
            if ((1 < *piVar24) || (piVar20[-3] < iVar17)) {
              iVar4 = piVar20[-2];
              goto LAB_1401957c2;
            }
            if (*piVar24 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar24 = -1;
          }
          if (piVar18 == (int *)0x0) {
            iVar4 = 0;
          }
          else {
            iVar4 = piVar18[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar18),uVar21,piVar8);
          FUN_14019c870(&local_a58,iVar6 + iVar3);
          goto LAB_14019595b;
        }
        if ((piVar20 == (int *)0x0) || (piVar19 = piVar20 + -4, piVar19 == (int *)0x0)) {
LAB_1401958b6:
          if (iVar4 < iVar3) {
            iVar4 = iVar3;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
          puVar12[1] = iVar4;
          *puVar12 = 0xffffffff;
          piVar18 = puVar12 + 4;
          puVar12[2] = 0;
          *(char *)piVar18 = '\0';
          local_a58 = piVar18;
          if (piVar19 != (int *)0x0) {
            FUN_14019f2c0(piVar19);
          }
        }
        else {
          if ((1 < *piVar19) || (piVar20[-3] < iVar3)) {
            iVar4 = piVar20[-2];
            goto LAB_1401958b6;
          }
          if (*piVar19 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar19 = -1;
        }
        FUN_142ef7ba0(piVar18,uVar21,piVar8);
        if (piVar18[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar3 == -1) || (iVar3 <= piVar18[-3])) {
          piVar18[-4] = 1;
          if (iVar3 != -1) goto LAB_140195938;
          if (piVar18 != (int *)0x0) {
            piVar24 = (int *)0xffffffffffffffff;
            do {
              piVar24 = (int *)((longlong)piVar24 + 1);
            } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar18[-3],iVar3);
          piVar18[-4] = 1;
LAB_140195938:
          *(char *)((longlong)piVar8 + (longlong)piVar18) = '\0';
          piVar24 = piVar8;
        }
        iVar3 = (int)piVar24;
        if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
          FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
        }
        piVar18[-2] = iVar3;
      }
    }
LAB_14019595b:
    piVar24 = (int *)0x0;
    if (local_a30 != 0) {
      FUN_14019f2c0(local_a30 - 0x10);
    }
    local_258[0] = 0x20;
    local_240 = 0x200;
    local_a50 = 0;
    FUN_142ef8250(local_368,0,0x104);
    iVar3 = 0;
    local_a40 = local_a40 & 0xffffffff00000000;
    local_a30 = local_a30 & 0xffffffff00000000;
    local_a18[0] = 0x28;
    local_a10 = 0;
    local_a00 = 0;
    uStack_9f8 = 0;
    uStack_a08 = 0xffffffff;
    if (DAT_143aa8280 == 0) {
      local_8d8 = 0x94;
      local_8d4 = 0;
      uStack_8cc = 0;
      local_8c4 = 0;
      uStack_8bc = 0;
      local_8b4 = 0;
      uStack_8ac = 0;
      local_8a4 = 0;
      uStack_89c = 0;
      local_894 = 0;
      uStack_88c = 0;
      local_884 = 0;
      uStack_87c = 0;
      local_874 = 0;
      uStack_86c = 0;
      local_864 = 0;
      uStack_85c = 0;
      local_854 = 0;
      uStack_84c = 0;
      (*DAT_143262820)(&local_8d8);
      if (uStack_8cc._4_4_ == 2) {
        DAT_143aa8280 = (*DAT_143ad5408)();
      }
      else {
        iVar4 = (*DAT_143ad5410)();
        DAT_143aa8280 = (longlong)iVar4;
      }
      if (DAT_143aa8280 == 0) {
        local_a28 = 0;
        plVar7 = (longlong *)FUN_14019ba10(&local_a28,"m_hProcess is Null\r\n");
        puVar22 = (undefined8 *)*plVar7;
        piVar20 = piVar18;
        local_a48 = puVar22;
        if (puVar22 == (undefined8 *)0x0) goto LAB_140196462;
        iVar4 = *(int *)(puVar22 + -1);
        piVar8 = (int *)(longlong)iVar4;
        if (iVar4 == 0) goto LAB_140196462;
        piVar19 = piVar24;
        if (piVar18 == (int *)0x0) goto LAB_1401963be;
        if ((char)*piVar18 != '\0') {
          iVar6 = piVar18[-2];
          for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
          }
          piVar24 = piVar18 + -4;
          if (piVar24 == (int *)0x0) {
LAB_1401962c5:
            if (iVar3 < iVar17) {
              iVar3 = iVar17;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
            puVar12[1] = iVar3;
            *puVar12 = 0xffffffff;
            piVar20 = puVar12 + 4;
            local_a58 = piVar20;
            if (piVar24 == (int *)0x0) {
              puVar12[2] = 0;
              *(char *)piVar20 = '\0';
              puVar22 = local_a48;
            }
            else {
              iVar17 = piVar18[-2] + 1;
              iVar23 = iVar3 + 1;
              if (iVar23 < iVar17) {
                FUN_142e54290(0x5c,iVar17,iVar23);
                iVar17 = iVar23;
              }
              FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
              puVar12[2] = piVar18[-2];
              *(char *)((longlong)iVar3 + (longlong)piVar20) = '\0';
              FUN_14019f2c0(piVar24);
              puVar22 = local_a48;
            }
          }
          else {
            if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
              iVar3 = piVar18[-2];
              goto LAB_1401962c5;
            }
            if (*piVar24 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar24 = -1;
          }
          iVar3 = 0;
          if (piVar20 != (int *)0x0) {
            iVar3 = piVar20[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar20),puVar22,piVar8);
          FUN_14019c870(&local_a58,iVar6 + iVar4);
          goto LAB_140196462;
        }
        if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_1401963be:
          if (iVar3 < iVar4) {
            iVar3 = iVar4;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
          puVar12[1] = iVar3;
          *puVar12 = 0xffffffff;
          piVar20 = puVar12 + 4;
          puVar12[2] = 0;
          *(char *)piVar20 = '\0';
          local_a58 = piVar20;
          if (piVar19 != (int *)0x0) {
            FUN_14019f2c0(piVar19);
          }
        }
        else {
          if ((1 < *piVar19) || (piVar18[-3] < iVar4)) {
            iVar3 = piVar18[-2];
            goto LAB_1401963be;
          }
          if (*piVar19 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar19 = -1;
        }
        FUN_142ef7ba0(piVar20,puVar22,piVar8);
        if (piVar20[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar4 == -1) || (iVar4 <= piVar20[-3])) {
          piVar20[-4] = 1;
          if (iVar4 == -1) {
            piVar18 = (int *)0xffffffffffffffff;
            if (piVar20 != (int *)0x0) {
              do {
                piVar24 = (int *)((longlong)piVar18 + 1);
                piVar18 = piVar24;
              } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
            }
LAB_140196443:
            iVar3 = (int)piVar24;
            if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
              FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
            }
            piVar20[-2] = iVar3;
LAB_140196462:
            if (local_a28 != 0) {
              FUN_14019f2c0(local_a28 + -0x10);
            }
            *local_a20 = piVar20;
            return local_a20;
          }
        }
        else {
          FUN_142e54290(0x90,piVar20[-3],iVar4);
          piVar20[-4] = 1;
        }
        *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
        piVar24 = piVar8;
        goto LAB_140196443;
      }
    }
    iVar4 = (*DAT_143aa8268)(DAT_143aa8280,local_9e8,&local_a50,local_258);
    local_a38 = iVar4;
    if (iVar4 == 0) {
      uVar5 = (*DAT_143262838)();
      local_a48 = (undefined8 *)0x0;
      plVar7 = (longlong *)FUN_14019ba10(&local_a48,"_SymGetLineFromAddr Error : %x ",uVar5);
      lVar14 = *plVar7;
      local_a28 = lVar14;
      if (lVar14 != 0) {
        iVar6 = *(int *)(lVar14 + -8);
        piVar20 = (int *)(longlong)iVar6;
        iVar4 = local_a38;
        if (iVar6 != 0) {
          piVar8 = piVar24;
          if (piVar18 == (int *)0x0) goto LAB_140195c6d;
          if ((char)*piVar18 != '\0') {
            iVar4 = piVar18[-2];
            for (iVar17 = piVar18[-3]; iVar17 < iVar4 + iVar6; iVar17 = iVar17 * 2) {
            }
            piVar24 = piVar18 + -4;
            if (piVar24 == (int *)0x0) {
LAB_140195b6f:
              if (iVar3 < iVar17) {
                iVar3 = iVar17;
              }
              puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
              puVar12[1] = iVar3;
              *puVar12 = 0xffffffff;
              piVar8 = puVar12 + 4;
              local_a58 = piVar8;
              if (piVar24 == (int *)0x0) {
                puVar12[2] = 0;
                *(char *)piVar8 = '\0';
                lVar14 = local_a28;
              }
              else {
                iVar17 = piVar18[-2] + 1;
                iVar23 = iVar3 + 1;
                if (iVar23 < iVar17) {
                  FUN_142e54290(0x5c,iVar17,iVar23);
                  iVar17 = iVar23;
                }
                FUN_142ef7ba0(piVar8,piVar18,(longlong)iVar17);
                puVar12[2] = piVar18[-2];
                *(char *)((longlong)iVar3 + (longlong)piVar8) = '\0';
                FUN_14019f2c0(piVar24);
                lVar14 = local_a28;
              }
            }
            else {
              if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
                iVar3 = piVar18[-2];
                goto LAB_140195b6f;
              }
              if (*piVar24 != 1) {
                FUN_142e52dd0(0x74);
              }
              *piVar24 = -1;
              piVar8 = piVar18;
            }
            if (piVar8 == (int *)0x0) {
              iVar3 = 0;
            }
            else {
              iVar3 = piVar8[-2];
            }
            FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar8),lVar14,piVar20);
            FUN_14019c870(&local_a58,iVar4 + iVar6);
            iVar4 = local_a38;
            goto LAB_140195d1d;
          }
          if ((piVar18 == (int *)0x0) || (piVar8 = piVar18 + -4, piVar8 == (int *)0x0)) {
LAB_140195c6d:
            if (iVar3 < iVar6) {
              iVar3 = iVar6;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
            puVar12[1] = iVar3;
            *puVar12 = 0xffffffff;
            piVar18 = puVar12 + 4;
            puVar12[2] = 0;
            *(char *)piVar18 = '\0';
            local_a58 = piVar18;
            if (piVar8 != (int *)0x0) {
              FUN_14019f2c0(piVar8);
            }
          }
          else {
            if ((1 < *piVar8) || (piVar18[-3] < iVar6)) {
              iVar3 = piVar18[-2];
              goto LAB_140195c6d;
            }
            if (*piVar8 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar8 = -1;
          }
          FUN_142ef7ba0(piVar18,lVar14,piVar20);
          if (piVar18[-4] != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar6 == -1) || (iVar6 <= piVar18[-3])) {
            piVar18[-4] = 1;
            if (iVar6 != -1) goto LAB_140195cf6;
            if (piVar18 != (int *)0x0) {
              piVar24 = (int *)0xffffffffffffffff;
              do {
                piVar24 = (int *)((longlong)piVar24 + 1);
              } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar18[-3],iVar6);
            piVar18[-4] = 1;
LAB_140195cf6:
            *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
            piVar24 = piVar20;
          }
          iVar3 = (int)piVar24;
          if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
            FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
          }
          piVar18[-2] = iVar3;
          iVar4 = local_a38;
        }
      }
LAB_140195d1d:
      if (local_a48 != (undefined8 *)0x0) {
        FUN_14019f2c0(local_a48 + -2);
      }
    }
    else if (DAT_143aa8270 != (code *)0x0) {
      (*DAT_143aa8270)(DAT_143aa8280,local_9e8,&local_a50,local_a18);
    }
    local_a80 = &local_a58;
    local_a88 = (int **)&local_a30;
    iVar6 = FUN_140194a90(local_9e8,local_368,0x104,&local_a40);
    local_a48 = (undefined8 *)0x0;
    iVar3 = 0;
    if ((int)uStack_a08 == -1) {
      if (iVar4 == 0) {
        local_a88 = local_368;
        plVar7 = (longlong *)
                 FUN_14019ba10(&local_a48,"%04X:%08X [%s]",local_a40 & 0xffffffff,
                               local_a30 & 0xffffffff);
        lVar14 = *plVar7;
        piVar20 = local_a58;
        if (lVar14 != 0) {
          iVar4 = *(int *)(lVar14 + -8);
          if (iVar4 != 0) {
            if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
              uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
              FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar4);
              piVar20 = local_a58;
            }
            else {
              iVar17 = local_a58[-2];
              for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
              }
              lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
              piVar20 = local_a58;
              iVar23 = iVar3;
              if (local_a58 != (int *)0x0) {
                iVar23 = local_a58[-2];
              }
              FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar17 + iVar4);
            }
          }
        }
      }
      else {
        local_a88 = local_368;
        plVar7 = (longlong *)FUN_14019ba10(&local_a48,"%hs()+%X [%s]",local_23c,local_a50);
        lVar14 = *plVar7;
        piVar20 = local_a58;
        if (lVar14 != 0) {
          iVar4 = *(int *)(lVar14 + -8);
          if (iVar4 != 0) {
            if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
              uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
              FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar4);
              piVar20 = local_a58;
            }
            else {
              iVar17 = local_a58[-2];
              for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
              }
              lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
              piVar20 = local_a58;
              iVar23 = iVar3;
              if (local_a58 != (int *)0x0) {
                iVar23 = local_a58[-2];
              }
              FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar17 + iVar4);
            }
          }
        }
      }
    }
    else {
      local_a80 = local_368;
      local_a88 = (int **)CONCAT44(local_a88._4_4_,(int)uStack_a08);
      plVar7 = (longlong *)FUN_14019ba10(&local_a48,"%hs() %hs(%lu) [%s]",local_23c,local_a00);
      lVar14 = *plVar7;
      piVar20 = local_a58;
      if (lVar14 != 0) {
        iVar4 = *(int *)(lVar14 + -8);
        if (iVar4 != 0) {
          if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
            uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
            FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
            FUN_14019c870(&local_a58,iVar4);
            piVar20 = local_a58;
          }
          else {
            iVar17 = local_a58[-2];
            for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
            }
            lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
            piVar20 = local_a58;
            iVar23 = iVar3;
            if (local_a58 != (int *)0x0) {
              iVar23 = local_a58[-2];
            }
            FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
            FUN_14019c870(&local_a58,iVar17 + iVar4);
          }
        }
      }
    }
    if (local_a48 != (undefined8 *)0x0) {
      FUN_14019f2c0(local_a48 + -2);
    }
    if (iVar6 == 0) {
      if ((piVar20 == (int *)0x0) || ((char)*piVar20 == '\0')) {
        pcVar15 = (char *)FUN_14019bd40(&local_a58,0xc);
        *(undefined8 *)pcVar15 = s_ErrorOccered_143272138._0_8_;
        *(undefined4 *)(pcVar15 + 8) = s_ErrorOccered_143272138._8_4_;
        FUN_14019c870(&local_a58,0xc);
        piVar20 = local_a58;
      }
      else {
        iVar4 = piVar20[-2];
        for (iVar6 = piVar20[-3]; iVar6 < iVar4 + 0xc; iVar6 = iVar6 * 2) {
        }
        lVar14 = FUN_14019bd40(&local_a58,iVar6,1);
        piVar20 = local_a58;
        iVar6 = iVar3;
        if (local_a58 != (int *)0x0) {
          iVar6 = local_a58[-2];
        }
        *(undefined8 *)(iVar6 + lVar14) = s_ErrorOccered_143272138._0_8_;
        *(undefined4 *)((longlong)iVar6 + 8 + lVar14) = s_ErrorOccered_143272138._8_4_;
        FUN_14019c870(&local_a58,iVar4 + 0xc);
      }
    }
    local_a48 = (undefined8 *)0x0;
    plVar7 = (longlong *)FUN_14019ba10(&local_a48,&DAT_143271d00);
    lVar14 = *plVar7;
    if (lVar14 != 0) {
      iVar4 = *(int *)(lVar14 + -8);
      if (iVar4 != 0) {
        if ((piVar20 == (int *)0x0) || ((char)*piVar20 == '\0')) {
          uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
          FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
          FUN_14019c870(&local_a58,iVar4);
          piVar20 = local_a58;
        }
        else {
          iVar6 = piVar20[-2];
          for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
          }
          lVar13 = FUN_14019bd40(&local_a58,iVar17,1);
          piVar20 = local_a58;
          if (local_a58 != (int *)0x0) {
            iVar3 = local_a58[-2];
          }
          FUN_142ef7ba0(iVar3 + lVar13,lVar14,(longlong)iVar4);
          FUN_14019c870(&local_a58,iVar6 + iVar4);
        }
      }
    }
    if (local_a48 != (undefined8 *)0x0) {
      FUN_14019f2c0(local_a48 + -2);
    }
    uVar2 = DAT_143aa8260;
    uVar16 = DAT_143aa8258;
    pcVar1 = DAT_143aa8250;
    uVar9 = (*DAT_143ad5440)();
    uVar10 = (*DAT_143ad5408)();
    local_a68 = 0;
    local_a70 = uVar2;
    local_a78 = uVar16;
    local_a80 = (int **)0x0;
    local_a88 = (int **)local_838;
    iVar3 = (*pcVar1)(0x8664,uVar10,uVar9,&local_9e8);
  } while( true );
}



//===========================================================
// FUN_142e559e0 @ 142e559e0   (127 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_142e559e0(void)

{
  char cVar1;
  int iVar2;
  undefined4 uVar3;
  
  iVar2 = FUN_14090d160(0x116,1);
  if (iVar2 != 0) {
    cVar1 = FUN_14090d340(0x11a);
    if (cVar1 != '\0') {
      _DAT_00000000 = 1;
    }
    if (DAT_143ae1514 < 3) {
      DAT_143ae1514 = DAT_143ae1514 + 1;
      return 1;
    }
    uVar3 = (*DAT_143262db0)();
    cVar1 = FUN_1408fcaa0(DAT_143ae1548,1800000,uVar3);
    if (cVar1 != '\0') {
      DAT_143ae1548 = uVar3;
      return 1;
    }
  }
  return 0;
}



//===========================================================
// FUN_14091a3e0 @ 14091a3e0   (62 bytes)
//===========================================================

uint FUN_14091a3e0(undefined8 *param_1)

{
  byte bVar1;
  uint uVar2;
  byte *pbVar3;
  ulonglong uVar4;
  
  pbVar3 = (byte *)*param_1;
  if (pbVar3 != (byte *)0x0) {
    uVar2 = 0x811c9dc5;
    if (*(uint *)(pbVar3 + -8) != 0) {
      uVar4 = (ulonglong)*(uint *)(pbVar3 + -8);
      do {
        bVar1 = *pbVar3;
        pbVar3 = pbVar3 + 1;
        uVar2 = (bVar1 ^ uVar2) * 0x1000193;
        uVar4 = uVar4 - 1;
      } while (uVar4 != 0);
    }
    return uVar2;
  }
  return 0x811c9dc5;
}



//===========================================================
// FUN_14019f2c0 @ 14019f2c0   (345 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14019f2c0(int *param_1)

{
  int iVar1;
  code *UNRECOVERED_JUMPTABLE;
  void *pvVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  int *piVar5;
  uint uVar6;
  longlong lVar7;
  bool bVar8;
  
  if (0x100000 < *param_1 + 1U) {
    FUN_142e52dd0(0x23);
  }
  LOCK();
  iVar1 = *param_1;
  *param_1 = *param_1 + -1;
  UNLOCK();
  if (1 < iVar1) {
    return;
  }
  if (*param_1 != 0) {
    FUN_142e52dd0(0x32);
  }
  pvVar2 = Self;
  UNRECOVERED_JUMPTABLE = DAT_143ad5530;
  uVar3 = *(ulonglong *)(param_1 + -2);
  if ((longlong)uVar3 < 0) {
    uVar3 = ~uVar3;
  }
  if (uVar3 < 0x39) {
    uVar6 = (uint)(0x28 < uVar3);
  }
  else {
    if (uVar3 < 0x59) {
      uVar6 = 2;
      goto LAB_14019f332;
    }
    uVar6 = 0xffffffff;
    if (uVar3 < 0x99) {
      uVar6 = 3;
    }
  }
  if ((int)uVar6 < 0) {
    uVar4 = (*DAT_143ad5538)();
                    /* WARNING: Could not recover jumptable at 0x00014019f3a6. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(uVar4,0,param_1 + -2);
    return;
  }
LAB_14019f332:
  uVar3 = (ulonglong)uVar6;
  lVar7 = (ulonglong)uVar6 * 0x10;
  LOCK();
  bVar8 = *(longlong *)(&DAT_143ad6a58 + lVar7) == 0;
  if (bVar8) {
    *(void **)(&DAT_143ad6a58 + lVar7) = Self;
  }
  UNLOCK();
  if (bVar8) {
LAB_14019f3d5:
    *(undefined4 *)(&DAT_143ad6a60 + lVar7) = 1;
  }
  else {
    if (*(void **)(&DAT_143ad6a58 + lVar7) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar8 = *(longlong *)(&DAT_143ad6a58 + lVar7) == 0;
        if (bVar8) {
          *(void **)(&DAT_143ad6a58 + lVar7) = Self;
        }
        UNLOCK();
        if (bVar8) goto LAB_14019f3d5;
        if (*(void **)(&DAT_143ad6a58 + lVar7) == pvVar2) break;
        (*DAT_143262828)(0);
      }
    }
    *(int *)(&DAT_143ad6a60 + lVar7) = *(int *)(&DAT_143ad6a60 + lVar7) + 1;
  }
  piVar5 = (int *)(&DAT_143ad6a60 + lVar7);
  *(undefined8 *)param_1 = *(undefined8 *)(&DAT_143ad6a98 + uVar3 * 8);
  *(int **)(&DAT_143ad6a98 + uVar3 * 8) = param_1;
  _DAT_143ad6ad8 = *(undefined8 *)param_1;
  *(int *)(&DAT_143ad6a44 + uVar3 * 4) = *(int *)(&DAT_143ad6a44 + uVar3 * 4) + -1;
  *piVar5 = *piVar5 + -1;
  if (*piVar5 == 0) {
    *(undefined8 *)(&DAT_143ad6a58 + lVar7) = 0;
  }
  return;
}



//===========================================================
// FUN_142e5c990 @ 142e5c990   (923 bytes)
//===========================================================

void FUN_142e5c990(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                  undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
                  undefined8 param_9,undefined8 param_10)

{
  undefined8 uVar1;
  undefined1 local_a4;
  longlong local_a0;
  longlong local_98;
  longlong local_90;
  longlong local_88;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  longlong local_50;
  longlong local_48;
  longlong local_40;
  longlong local_38;
  longlong local_30;
  longlong local_28 [2];
  
  uVar1 = FUN_142a1d8a0(local_28);
  local_a0 = 0;
  FUN_140198700(&local_a0,uVar1,local_a4);
  local_70 = 0;
  uVar1 = FUN_14019ba10(&local_70,&DAT_143272338,param_1);
  local_98 = 0;
  FUN_14019a260(&local_98,uVar1);
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  FUN_1401a1c50(&local_a0,&local_98);
  if (local_98 != 0) {
    FUN_14019f2c0(local_98 + -0x10);
  }
  local_68 = 0;
  uVar1 = FUN_14019ba10(&local_68,&DAT_143272338,param_2);
  local_90 = 0;
  FUN_14019a260(&local_90,uVar1);
  if (local_68 != 0) {
    FUN_14019f2c0(local_68 + -0x10);
  }
  FUN_1401a1c50(&local_a0,&local_90);
  if (local_90 != 0) {
    FUN_14019f2c0(local_90 + -0x10);
  }
  FUN_1408bc140(&local_60,param_3);
  FUN_1401a1c50(&local_a0,&local_60);
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  local_58 = 0;
  uVar1 = FUN_14019ba10(&local_58,&DAT_143272338,param_4);
  local_88 = 0;
  FUN_14019a260(&local_88,uVar1);
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  FUN_1401a1c50(&local_a0,&local_88);
  if (local_88 != 0) {
    FUN_14019f2c0(local_88 + -0x10);
  }
  FUN_1408bc020(&local_50,param_5);
  FUN_1401a1c50(&local_a0,&local_50);
  if (local_50 != 0) {
    FUN_14019f2c0(local_50 + -0x10);
  }
  local_48 = 0;
  uVar1 = FUN_14019ba10(&local_48,&DAT_143272338,param_6);
  local_80 = 0;
  FUN_14019a260(&local_80,uVar1);
  if (local_48 != 0) {
    FUN_14019f2c0(local_48 + -0x10);
  }
  FUN_1401a1c50(&local_a0,&local_80);
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  FUN_1408bc020(&local_40,param_7);
  FUN_1401a1c50(&local_a0,&local_40);
  if (local_40 != 0) {
    FUN_14019f2c0(local_40 + -0x10);
  }
  local_38 = 0;
  uVar1 = FUN_14019ba10(&local_38,&DAT_143272338,param_8);
  local_78 = 0;
  FUN_14019a260(&local_78,uVar1);
  if (local_38 != 0) {
    FUN_14019f2c0(local_38 + -0x10);
  }
  FUN_1401a1c50(&local_a0,&local_78);
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  FUN_1408bc020(&local_30,param_9);
  FUN_1401a1c50(&local_a0,&local_30);
  if (local_30 != 0) {
    FUN_14019f2c0(local_30 + -0x10);
  }
  FUN_140198700(&local_a0,param_10,local_a4);
  FUN_142a1ec10(&local_a0);
  if (local_a0 != 0) {
    FUN_14019f2c0(local_a0 + -0x10);
  }
  if (local_28[0] != 0) {
    FUN_14019f2c0(local_28[0] + -0x10);
  }
  return;
}



//===========================================================
// FUN_142a1ec10 @ 142a1ec10   (61 bytes)
//===========================================================

void FUN_142a1ec10(undefined8 param_1)

{
  undefined8 uVar1;
  undefined8 local_res10;
  undefined8 *local_res18;
  
  local_res18 = &local_res10;
  local_res10 = 0;
  FUN_14019a260(&local_res10,param_1);
  uVar1 = FUN_142e56bc0();
  FUN_142a23830(uVar1,&local_res10);
  return;
}



//===========================================================
// FUN_142e5d800 @ 142e5d800   (782 bytes)
//===========================================================

longlong *
FUN_142e5d800(longlong *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
             undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
             undefined8 param_9,undefined8 param_10)

{
  char *pcVar1;
  longlong lVar2;
  longlong lVar3;
  undefined4 *puVar4;
  int iVar5;
  int iVar6;
  int *piVar7;
  longlong lVar8;
  int iVar9;
  int *piVar10;
  int iVar11;
  longlong local_30;
  
  piVar7 = (int *)0x0;
  iVar5 = 0;
  *param_1 = 0;
  FUN_1408bc980(&local_30);
  lVar2 = local_30;
  iVar6 = 0;
  if (local_30 != 0) {
    iVar9 = *(int *)(local_30 + -8);
    lVar8 = (longlong)iVar9;
    if (iVar9 != 0) {
      pcVar1 = (char *)*param_1;
      piVar10 = piVar7;
      iVar11 = iVar6;
      if (pcVar1 == (char *)0x0) goto LAB_142e5d8ee;
      if (*pcVar1 == '\0') {
        piVar10 = (int *)(pcVar1 + -0x10);
        if (piVar10 == (int *)0x0) {
LAB_142e5d8ee:
          if (iVar11 < iVar9) {
            iVar11 = iVar9;
          }
          puVar4 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
          puVar4[1] = iVar11;
          *puVar4 = 0xffffffff;
          *param_1 = (longlong)(puVar4 + 4);
          puVar4[2] = 0;
          *(undefined1 *)*param_1 = 0;
          if (piVar10 != (int *)0x0) {
            FUN_14019f2c0(piVar10);
          }
        }
        else {
          if ((1 < *piVar10) || (*(int *)(pcVar1 + -0xc) < iVar9)) {
            iVar11 = *(int *)(pcVar1 + -8);
            goto LAB_142e5d8ee;
          }
          if (*piVar10 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar10 = -1;
        }
        FUN_142ef7ba0(*param_1,lVar2,lVar8);
      }
      else {
        iVar9 = *(int *)(pcVar1 + -8) + iVar9;
        for (iVar11 = *(int *)(pcVar1 + -0xc); iVar11 < iVar9; iVar11 = iVar11 * 2) {
        }
        lVar3 = FUN_14019bd40(param_1,iVar11,1);
        iVar11 = iVar5;
        if (*param_1 != 0) {
          iVar11 = *(int *)(*param_1 + -8);
        }
        FUN_142ef7ba0(iVar11 + lVar3,lVar2,lVar8);
      }
      FUN_14019c870(param_1,iVar9);
    }
  }
  if (local_30 != 0) {
    FUN_14019f2c0(local_30 + -0x10);
  }
  FUN_1408b6250(param_1,param_3,(ulonglong)param_1 & 0xff);
  FUN_1401ab0c0(param_1,param_4,(ulonglong)param_1 & 0xff);
  FUN_1408bc140(&local_30,param_5);
  lVar2 = local_30;
  if (local_30 == 0) goto LAB_142e5da9b;
  iVar9 = *(int *)(local_30 + -8);
  lVar8 = (longlong)iVar9;
  if (iVar9 == 0) goto LAB_142e5da9b;
  pcVar1 = (char *)*param_1;
  if (pcVar1 == (char *)0x0) goto LAB_142e5da3c;
  if (*pcVar1 == '\0') {
    piVar7 = (int *)(pcVar1 + -0x10);
    if (piVar7 == (int *)0x0) {
LAB_142e5da3c:
      if (iVar6 < iVar9) {
        iVar6 = iVar9;
      }
      puVar4 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
      puVar4[1] = iVar6;
      *puVar4 = 0xffffffff;
      *param_1 = (longlong)(puVar4 + 4);
      puVar4[2] = 0;
      *(undefined1 *)*param_1 = 0;
      if (piVar7 != (int *)0x0) {
        FUN_14019f2c0(piVar7);
      }
    }
    else {
      if ((1 < *piVar7) || (*(int *)(pcVar1 + -0xc) < iVar9)) {
        iVar6 = *(int *)(pcVar1 + -8);
        goto LAB_142e5da3c;
      }
      if (*piVar7 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar7 = -1;
    }
    FUN_142ef7ba0(*param_1,lVar2,lVar8);
  }
  else {
    iVar9 = *(int *)(pcVar1 + -8) + iVar9;
    for (iVar6 = *(int *)(pcVar1 + -0xc); iVar6 < iVar9; iVar6 = iVar6 * 2) {
    }
    lVar3 = FUN_14019bd40(param_1,iVar6,1);
    if (*param_1 != 0) {
      iVar5 = *(int *)(*param_1 + -8);
    }
    FUN_142ef7ba0(iVar5 + lVar3,lVar2,lVar8);
  }
  FUN_14019c870(param_1,iVar9);
LAB_142e5da9b:
  if (local_30 != 0) {
    FUN_14019f2c0(local_30 + -0x10);
  }
  FUN_1401ab0c0(param_1,param_6,(ulonglong)param_1 & 0xff);
  FUN_142e5e540(param_1,param_7,param_8,param_9,param_10);
  return param_1;
}



//===========================================================
// FUN_142a1d8a0 @ 142a1d8a0   (4667 bytes)
//===========================================================

ulonglong FUN_142a1d8a0(ulonglong param_1)

{
  longlong *plVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  int *piVar4;
  undefined4 *puVar5;
  longlong lVar6;
  int iVar7;
  longlong lVar8;
  int iVar9;
  int *piVar10;
  int iVar11;
  int *piVar12;
  ulonglong uVar13;
  int *piVar14;
  longlong lVar15;
  int iVar16;
  int *piVar17;
  longlong local_res10;
  undefined4 local_res18 [2];
  undefined4 local_res20;
  undefined4 uStackX_24;
  int *local_c0;
  longlong local_b8;
  undefined4 local_b0;
  undefined4 local_ac;
  undefined4 local_a8;
  undefined4 local_a4;
  longlong local_a0;
  longlong local_98;
  longlong local_90;
  longlong local_88;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  undefined8 local_68;
  longlong local_60;
  undefined8 local_58 [3];
  
  piVar17 = (int *)0x0;
  iVar11 = 0;
  uVar2 = FUN_142c4a030(DAT_143ac1898);
  local_res10 = CONCAT44(local_res10._4_4_,uVar2);
  local_68 = FUN_1408f6690();
  local_res18[0] = (*DAT_143262db0)();
  local_res20 = FUN_142c50c50();
  local_b0 = FUN_141892a90();
  uVar3 = FUN_1408fadb0(&local_60,&local_68);
  local_ac = 100;
  FUN_142a24230(param_1,"VERSION",&local_ac,"DATETIME",uVar3,&DAT_143487774,&local_b0,"LastUseName",
                &DAT_143adc708,"State",&local_res20,"Time1",local_res18,"Time2",&local_res10);
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  plVar1 = DAT_143aa84a0;
  if (DAT_143aa84a0 == (longlong *)0x0) {
    uVar3 = FUN_142a245b0(&local_res20,param_1,&DAT_143273ad4,&DAT_143271f04,"Channel",
                          &DAT_143271f04,&DAT_143273ac0,&DAT_143271f04,&DAT_143487790,&DAT_143271f04
                         );
    FUN_140319ad0(param_1,uVar3);
    if (CONCAT44(uStackX_24,local_res20) != 0) {
      FUN_14019f2c0(CONCAT44(uStackX_24,local_res20) + -0x10);
    }
    goto LAB_142a1ea24;
  }
  local_a4 = (**(code **)(*DAT_143aa84a0 + 0xa8))(DAT_143aa84a0);
  local_58[0] = FUN_142cb9610(plVar1);
  local_a8 = FUN_142cb9260(plVar1);
  local_res20 = FUN_142cb9230(plVar1);
  local_c0 = (int *)0x0;
  FUN_1408bc980(&local_88,param_1);
  lVar8 = local_88;
  piVar10 = piVar17;
  if (local_88 != 0) {
    iVar16 = *(int *)(local_88 + -8);
    piVar12 = (int *)(longlong)iVar16;
    if (iVar16 != 0) {
      iVar7 = 0;
      if (0 < iVar16) {
        iVar7 = iVar16;
      }
      piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar7 + 0x11));
      piVar4[1] = iVar7;
      *piVar4 = -1;
      piVar10 = piVar4 + 4;
      piVar4[2] = 0;
      *(char *)piVar10 = '\0';
      local_c0 = piVar10;
      FUN_142ef7ba0(piVar10,lVar8,piVar12);
      if (*piVar4 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar16 == -1) || (iVar16 <= piVar4[1])) {
        *piVar4 = 1;
        if (iVar16 != -1) goto LAB_142a1dab0;
        piVar12 = piVar17;
        if (piVar10 != (int *)0x0) {
          piVar12 = (int *)0xffffffffffffffff;
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
          } while (*(char *)((longlong)piVar10 + (longlong)piVar12) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar4[1],iVar16);
        *piVar4 = 1;
LAB_142a1dab0:
        *(char *)((longlong)piVar12 + (longlong)piVar10) = '\0';
      }
      iVar16 = (int)piVar12;
      if ((iVar16 < 0) || (piVar4[1] + 1 <= iVar16)) {
        FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
      }
      piVar4[2] = iVar16;
    }
  }
  if (local_88 != 0) {
    FUN_14019f2c0(local_88 + -0x10);
  }
  local_80 = 0;
  uVar3 = FUN_14019ba10(&local_80,&DAT_143272338,&DAT_143273ad4);
  local_b8 = 0;
  FUN_14019a260(&local_b8,uVar3);
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  lVar8 = local_b8;
  piVar12 = piVar10;
  if (local_b8 != 0) {
    iVar16 = *(int *)(local_b8 + -8);
    piVar4 = (int *)(longlong)iVar16;
    if (iVar16 != 0) {
      piVar14 = piVar17;
      if (piVar10 == (int *)0x0) goto LAB_142a1dcdf;
      if ((char)*piVar10 != '\0') {
        iVar7 = piVar10[-2];
        for (iVar9 = piVar10[-3]; iVar9 < iVar7 + iVar16; iVar9 = iVar9 * 2) {
        }
        piVar17 = piVar10 + -4;
        if (piVar17 == (int *)0x0) {
LAB_142a1dbdf:
          if (iVar11 < iVar9) {
            iVar11 = iVar9;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
          puVar5[1] = iVar11;
          *puVar5 = 0xffffffff;
          piVar12 = puVar5 + 4;
          local_c0 = piVar12;
          if (piVar17 == (int *)0x0) {
            puVar5[2] = 0;
            *(char *)piVar12 = '\0';
          }
          else {
            iVar9 = piVar10[-2] + 1;
            if (iVar11 + 1 < iVar9) {
              FUN_142e54290(0x5c,iVar9,iVar11 + 1);
              iVar9 = iVar11 + 1;
            }
            FUN_142ef7ba0(piVar12,piVar10,(longlong)iVar9);
            puVar5[2] = piVar10[-2];
            *(char *)((longlong)iVar11 + (longlong)piVar12) = '\0';
            FUN_14019f2c0(piVar17);
          }
        }
        else {
          if ((1 < *piVar17) || (piVar10[-3] < iVar9)) {
            iVar11 = piVar10[-2];
            goto LAB_142a1dbdf;
          }
          if (*piVar17 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar17 = -1;
        }
        iVar11 = 0;
        if (piVar12 != (int *)0x0) {
          iVar11 = piVar12[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar11 + (longlong)piVar12),lVar8,piVar4);
        FUN_14019c870(&local_c0,iVar7 + iVar16);
        goto LAB_142a1dd80;
      }
      if ((piVar10 == (int *)0x0) || (piVar14 = piVar10 + -4, piVar14 == (int *)0x0)) {
LAB_142a1dcdf:
        if (iVar11 < iVar16) {
          iVar11 = iVar16;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
        puVar5[1] = iVar11;
        *puVar5 = 0xffffffff;
        piVar12 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar12 = '\0';
        local_c0 = piVar12;
        if (piVar14 != (int *)0x0) {
          FUN_14019f2c0(piVar14);
        }
      }
      else {
        if ((1 < *piVar14) || (piVar10[-3] < iVar16)) {
          iVar11 = piVar10[-2];
          goto LAB_142a1dcdf;
        }
        if (*piVar14 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar14 = -1;
      }
      FUN_142ef7ba0(piVar12,lVar8,piVar4);
      if (piVar12[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar16 == -1) || (iVar16 <= piVar12[-3])) {
        piVar12[-4] = 1;
        if (iVar16 != -1) goto LAB_142a1dd5d;
        if (piVar12 != (int *)0x0) {
          piVar17 = (int *)0xffffffffffffffff;
          do {
            piVar17 = (int *)((longlong)piVar17 + 1);
          } while (*(char *)((longlong)piVar12 + (longlong)piVar17) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar12[-3],iVar16);
        piVar12[-4] = 1;
LAB_142a1dd5d:
        *(char *)((longlong)piVar4 + (longlong)piVar12) = '\0';
        piVar17 = piVar4;
      }
      iVar11 = (int)piVar17;
      if ((iVar11 < 0) || (piVar12[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
      }
      piVar12[-2] = iVar11;
    }
  }
LAB_142a1dd80:
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  FUN_1408bc080(&local_a0,&local_res20);
  lVar8 = local_a0;
  if (local_a0 != 0) {
    iVar11 = *(int *)(local_a0 + -8);
    uVar13 = (ulonglong)iVar11;
    if (iVar11 != 0) {
      if (piVar12 == (int *)0x0) {
LAB_142a1df3d:
        piVar17 = (int *)0x0;
LAB_142a1df3f:
        iVar16 = 0;
LAB_142a1df41:
        if (iVar16 < iVar11) {
          iVar16 = iVar11;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
        puVar5[1] = iVar16;
        *puVar5 = 0xffffffff;
        piVar12 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar12 = '\0';
        local_c0 = piVar12;
        if (piVar17 != (int *)0x0) {
          FUN_14019f2c0(piVar17);
        }
      }
      else {
        if ((char)*piVar12 != '\0') {
          iVar16 = piVar12[-2];
          for (iVar7 = piVar12[-3]; iVar7 < iVar16 + iVar11; iVar7 = iVar7 * 2) {
          }
          piVar17 = piVar12 + -4;
          if (piVar17 == (int *)0x0) {
            iVar9 = 0;
LAB_142a1de4f:
            if (iVar9 < iVar7) {
              iVar9 = iVar7;
            }
            puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
            puVar5[1] = iVar9;
            *puVar5 = 0xffffffff;
            piVar10 = puVar5 + 4;
            local_c0 = piVar10;
            if (piVar17 == (int *)0x0) {
              puVar5[2] = 0;
              *(char *)piVar10 = '\0';
            }
            else {
              iVar7 = piVar12[-2] + 1;
              if (iVar9 + 1 < iVar7) {
                FUN_142e54290(0x5c,iVar7,iVar9 + 1);
                iVar7 = iVar9 + 1;
              }
              FUN_142ef7ba0(piVar10,piVar12,(longlong)iVar7);
              puVar5[2] = piVar12[-2];
              *(char *)((longlong)iVar9 + (longlong)piVar10) = '\0';
              FUN_14019f2c0(piVar17);
            }
          }
          else {
            if ((1 < *piVar17) || (piVar12[-3] < iVar7)) {
              iVar9 = piVar12[-2];
              goto LAB_142a1de4f;
            }
            if (*piVar17 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar17 = -1;
            piVar10 = piVar12;
          }
          if (piVar10 == (int *)0x0) {
            iVar7 = 0;
          }
          else {
            iVar7 = piVar10[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar7 + (longlong)piVar10),lVar8,uVar13);
          FUN_14019c870(&local_c0,iVar16 + iVar11);
          piVar12 = piVar10;
          goto LAB_142a1dfe9;
        }
        if (piVar12 == (int *)0x0) goto LAB_142a1df3d;
        piVar17 = piVar12 + -4;
        if (piVar17 == (int *)0x0) goto LAB_142a1df3f;
        if ((1 < *piVar17) || (piVar12[-3] < iVar11)) {
          iVar16 = piVar12[-2];
          goto LAB_142a1df41;
        }
        if (*piVar17 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar17 = -1;
      }
      FUN_142ef7ba0(piVar12,lVar8,uVar13);
      if (piVar12[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar11 == -1) || (iVar11 <= piVar12[-3])) {
        piVar12[-4] = 1;
        if (iVar11 != -1) goto LAB_142a1dfc2;
        if (piVar12 == (int *)0x0) {
          uVar13 = 0;
        }
        else {
          uVar13 = 0xffffffffffffffff;
          do {
            uVar13 = uVar13 + 1;
          } while (*(char *)((longlong)piVar12 + uVar13) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar12[-3],iVar11);
        piVar12[-4] = 1;
LAB_142a1dfc2:
        *(char *)(uVar13 + (longlong)piVar12) = '\0';
      }
      iVar11 = (int)uVar13;
      if ((iVar11 < 0) || (piVar12[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,uVar13 & 0xffffffff);
      }
      piVar12[-2] = iVar11;
    }
  }
LAB_142a1dfe9:
  if (local_a0 != 0) {
    FUN_14019f2c0(local_a0 + -0x10);
  }
  iVar16 = 0;
  piVar17 = (int *)0x0;
  iVar11 = 0;
  local_78 = 0;
  uVar3 = FUN_14019ba10(&local_78,&DAT_143272338,"Channel");
  local_b8 = 0;
  FUN_14019a260(&local_b8,uVar3);
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  lVar8 = local_b8;
  piVar10 = piVar12;
  if (local_b8 != 0) {
    iVar7 = *(int *)(local_b8 + -8);
    piVar4 = (int *)(longlong)iVar7;
    if (iVar7 != 0) {
      piVar14 = piVar17;
      if (piVar12 == (int *)0x0) goto LAB_142a1e1de;
      if ((char)*piVar12 != '\0') {
        iVar16 = piVar12[-2];
        for (iVar9 = piVar12[-3]; iVar9 < iVar16 + iVar7; iVar9 = iVar9 * 2) {
        }
        piVar17 = piVar12 + -4;
        if (piVar17 == (int *)0x0) {
LAB_142a1e0e9:
          if (iVar11 < iVar9) {
            iVar11 = iVar9;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
          puVar5[1] = iVar11;
          *puVar5 = 0xffffffff;
          piVar10 = puVar5 + 4;
          local_c0 = piVar10;
          if (piVar17 == (int *)0x0) {
            puVar5[2] = 0;
            *(char *)piVar10 = '\0';
          }
          else {
            iVar9 = piVar12[-2] + 1;
            if (iVar11 + 1 < iVar9) {
              FUN_142e54290(0x5c,iVar9,iVar11 + 1);
              iVar9 = iVar11 + 1;
            }
            FUN_142ef7ba0(piVar10,piVar12,(longlong)iVar9);
            puVar5[2] = piVar12[-2];
            *(char *)((longlong)iVar11 + (longlong)piVar10) = '\0';
            FUN_14019f2c0(piVar17);
          }
        }
        else {
          if ((1 < *piVar17) || (piVar12[-3] < iVar9)) {
            iVar11 = piVar12[-2];
            goto LAB_142a1e0e9;
          }
          if (*piVar17 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar17 = -1;
        }
        if (piVar10 == (int *)0x0) {
          iVar11 = 0;
        }
        else {
          iVar11 = piVar10[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar11 + (longlong)piVar10),lVar8,piVar4);
        FUN_14019c870(&local_c0,iVar16 + iVar7);
        goto LAB_142a1e282;
      }
      if ((piVar12 == (int *)0x0) || (piVar14 = piVar12 + -4, piVar14 == (int *)0x0)) {
LAB_142a1e1de:
        if (iVar16 < iVar7) {
          iVar16 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
        puVar5[1] = iVar16;
        *puVar5 = 0xffffffff;
        piVar10 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar10 = '\0';
        local_c0 = piVar10;
        if (piVar14 != (int *)0x0) {
          FUN_14019f2c0(piVar14);
        }
      }
      else {
        if ((1 < *piVar14) || (piVar12[-3] < iVar7)) {
          iVar16 = piVar12[-2];
          goto LAB_142a1e1de;
        }
        if (*piVar14 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar14 = -1;
      }
      FUN_142ef7ba0(piVar10,lVar8,piVar4);
      if (piVar10[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar7 == -1) || (iVar7 <= piVar10[-3])) {
        piVar10[-4] = 1;
        if (iVar7 != -1) goto LAB_142a1e25f;
        if (piVar10 != (int *)0x0) {
          piVar17 = (int *)0xffffffffffffffff;
          do {
            piVar17 = (int *)((longlong)piVar17 + 1);
          } while (*(char *)((longlong)piVar10 + (longlong)piVar17) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar10[-3],iVar7);
        piVar10[-4] = 1;
LAB_142a1e25f:
        *(char *)((longlong)piVar4 + (longlong)piVar10) = '\0';
        piVar17 = piVar4;
      }
      iVar11 = (int)piVar17;
      if ((iVar11 < 0) || (piVar10[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
      }
      piVar10[-2] = iVar11;
    }
  }
LAB_142a1e282:
  piVar17 = (int *)0x0;
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  FUN_1408bc080(&local_98,&local_a8);
  lVar8 = local_98;
  if (local_98 != 0) {
    iVar11 = *(int *)(local_98 + -8);
    piVar12 = (int *)(longlong)iVar11;
    if (iVar11 != 0) {
      iVar16 = 0;
      piVar4 = piVar17;
      if (piVar10 == (int *)0x0) goto LAB_142a1e443;
      if ((char)*piVar10 == '\0') {
        if ((piVar10 == (int *)0x0) || (piVar4 = piVar10 + -4, piVar4 == (int *)0x0)) {
LAB_142a1e443:
          if (iVar16 < iVar11) {
            iVar16 = iVar11;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
          puVar5[1] = iVar16;
          *puVar5 = 0xffffffff;
          piVar10 = puVar5 + 4;
          puVar5[2] = 0;
          *(char *)piVar10 = '\0';
          local_c0 = piVar10;
          if (piVar4 != (int *)0x0) {
            FUN_14019f2c0(piVar4);
          }
        }
        else {
          if ((1 < *piVar4) || (piVar10[-3] < iVar11)) {
            iVar16 = piVar10[-2];
            goto LAB_142a1e443;
          }
          if (*piVar4 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar4 = -1;
        }
        FUN_142ef7ba0(piVar10,lVar8,piVar12);
        if (piVar10[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar11 == -1) || (iVar11 <= piVar10[-3])) {
          piVar10[-4] = 1;
          if (iVar11 != -1) goto LAB_142a1e4cb;
          if (piVar10 != (int *)0x0) {
            piVar17 = (int *)0xffffffffffffffff;
            do {
              piVar17 = (int *)((longlong)piVar17 + 1);
            } while (*(char *)((longlong)piVar10 + (longlong)piVar17) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar10[-3],iVar11);
          piVar10[-4] = 1;
LAB_142a1e4cb:
          *(char *)((longlong)piVar12 + (longlong)piVar10) = '\0';
          piVar17 = piVar12;
        }
        iVar11 = (int)piVar17;
        if ((iVar11 < 0) || (piVar10[-3] + 1 <= iVar11)) {
          FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
        }
        piVar10[-2] = iVar11;
        goto LAB_142a1e4f2;
      }
      iVar16 = piVar10[-2];
      for (iVar7 = piVar10[-3]; iVar7 < iVar16 + iVar11; iVar7 = iVar7 * 2) {
      }
      piVar17 = piVar10 + -4;
      if (piVar17 == (int *)0x0) {
        iVar9 = 0;
LAB_142a1e34f:
        if (iVar9 < iVar7) {
          iVar9 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
        puVar5[1] = iVar9;
        *puVar5 = 0xffffffff;
        piVar4 = puVar5 + 4;
        local_c0 = piVar4;
        if (piVar17 == (int *)0x0) {
          puVar5[2] = 0;
          *(char *)piVar4 = '\0';
        }
        else {
          iVar7 = piVar10[-2] + 1;
          if (iVar9 + 1 < iVar7) {
            FUN_142e54290(0x5c,iVar7,iVar9 + 1);
            iVar7 = iVar9 + 1;
          }
          FUN_142ef7ba0(piVar4,piVar10,(longlong)iVar7);
          puVar5[2] = piVar10[-2];
          *(char *)((longlong)iVar9 + (longlong)piVar4) = '\0';
          FUN_14019f2c0(piVar17);
        }
      }
      else {
        if ((1 < *piVar17) || (piVar10[-3] < iVar7)) {
          iVar9 = piVar10[-2];
          goto LAB_142a1e34f;
        }
        if (*piVar17 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar17 = -1;
        piVar4 = piVar10;
      }
      if (piVar4 == (int *)0x0) {
        iVar7 = 0;
      }
      else {
        iVar7 = piVar4[-2];
      }
      FUN_142ef7ba0((char *)((longlong)iVar7 + (longlong)piVar4),lVar8,piVar12);
      FUN_14019c870(&local_c0,iVar16 + iVar11);
      piVar10 = piVar4;
    }
  }
LAB_142a1e4f2:
  if (local_98 != 0) {
    FUN_14019f2c0(local_98 + -0x10);
  }
  piVar17 = (int *)0x0;
  local_70 = 0;
  uVar3 = FUN_14019ba10(&local_70,&DAT_143272338,&DAT_143273ac0);
  local_b8 = 0;
  FUN_14019a260(&local_b8,uVar3);
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  lVar8 = local_b8;
  piVar12 = piVar10;
  if (local_b8 != 0) {
    iVar11 = *(int *)(local_b8 + -8);
    piVar4 = (int *)(longlong)iVar11;
    if (iVar11 != 0) {
      iVar16 = 0;
      piVar14 = piVar17;
      if (piVar10 == (int *)0x0) goto LAB_142a1e6fa;
      if ((char)*piVar10 != '\0') {
        iVar7 = piVar10[-2];
        for (iVar9 = piVar10[-3]; iVar9 < iVar7 + iVar11; iVar9 = iVar9 * 2) {
        }
        piVar17 = piVar10 + -4;
        if (piVar17 == (int *)0x0) {
LAB_142a1e600:
          if (iVar16 < iVar9) {
            iVar16 = iVar9;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
          puVar5[1] = iVar16;
          *puVar5 = 0xffffffff;
          piVar12 = puVar5 + 4;
          local_c0 = piVar12;
          if (piVar17 == (int *)0x0) {
            puVar5[2] = 0;
            *(char *)piVar12 = '\0';
          }
          else {
            iVar9 = piVar10[-2] + 1;
            if (iVar16 + 1 < iVar9) {
              FUN_142e54290(0x5c,iVar9,iVar16 + 1);
              iVar9 = iVar16 + 1;
            }
            FUN_142ef7ba0(piVar12,piVar10,(longlong)iVar9);
            puVar5[2] = piVar10[-2];
            *(char *)((longlong)iVar16 + (longlong)piVar12) = '\0';
            FUN_14019f2c0(piVar17);
          }
        }
        else {
          if ((1 < *piVar17) || (piVar10[-3] < iVar9)) {
            iVar16 = piVar10[-2];
            goto LAB_142a1e600;
          }
          if (*piVar17 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar17 = -1;
        }
        if (piVar12 == (int *)0x0) {
          iVar16 = 0;
        }
        else {
          iVar16 = piVar12[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar16 + (longlong)piVar12),lVar8,piVar4);
        FUN_14019c870(&local_c0,iVar7 + iVar11);
        goto LAB_142a1e7a5;
      }
      if ((piVar10 == (int *)0x0) || (piVar14 = piVar10 + -4, piVar14 == (int *)0x0)) {
LAB_142a1e6fa:
        if (iVar16 < iVar11) {
          iVar16 = iVar11;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
        puVar5[1] = iVar16;
        *puVar5 = 0xffffffff;
        piVar12 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar12 = '\0';
        local_c0 = piVar12;
        if (piVar14 != (int *)0x0) {
          FUN_14019f2c0(piVar14);
        }
      }
      else {
        if ((1 < *piVar14) || (piVar10[-3] < iVar11)) {
          iVar16 = piVar10[-2];
          goto LAB_142a1e6fa;
        }
        if (*piVar14 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar14 = -1;
      }
      FUN_142ef7ba0(piVar12,lVar8,piVar4);
      if (piVar12[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar11 == -1) || (iVar11 <= piVar12[-3])) {
        piVar12[-4] = 1;
        if (iVar11 != -1) goto LAB_142a1e782;
        piVar10 = (int *)0xffffffffffffffff;
        if (piVar12 != (int *)0x0) {
          do {
            piVar17 = (int *)((longlong)piVar10 + 1);
            piVar10 = piVar17;
          } while (*(char *)((longlong)piVar12 + (longlong)piVar17) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar12[-3],iVar11);
        piVar12[-4] = 1;
LAB_142a1e782:
        *(char *)((longlong)piVar4 + (longlong)piVar12) = '\0';
        piVar17 = piVar4;
      }
      iVar11 = (int)piVar17;
      if ((iVar11 < 0) || (piVar12[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
      }
      piVar12[-2] = iVar11;
    }
  }
LAB_142a1e7a5:
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  FUN_1408bc4c0(&local_90,local_58);
  lVar8 = local_90;
  iVar11 = 0;
  if (local_90 != 0) {
    iVar16 = *(int *)(local_90 + -8);
    lVar15 = (longlong)iVar16;
    if (iVar16 != 0) {
      if ((piVar12 == (int *)0x0) || ((char)*piVar12 == '\0')) {
        uVar3 = FUN_14019bd40(&local_c0,iVar16,0);
        FUN_142ef7ba0(uVar3,lVar8,lVar15);
      }
      else {
        iVar16 = piVar12[-2] + iVar16;
        for (iVar7 = piVar12[-3]; iVar7 < iVar16; iVar7 = iVar7 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_c0,iVar7,1);
        iVar7 = iVar11;
        if (local_c0 != (int *)0x0) {
          iVar7 = local_c0[-2];
        }
        FUN_142ef7ba0(iVar7 + lVar6,lVar8,lVar15);
      }
      FUN_14019c870(&local_c0,iVar16);
    }
  }
  if (local_90 != 0) {
    FUN_14019f2c0(local_90 + -0x10);
  }
  FUN_1401d7f50(&local_c0,&DAT_143487790,param_1 & 0xff);
  FUN_1408bc080(&local_b8,&local_a4);
  lVar8 = local_b8;
  if (local_b8 != 0) {
    iVar16 = *(int *)(local_b8 + -8);
    lVar15 = (longlong)iVar16;
    if (iVar16 != 0) {
      if ((local_c0 == (int *)0x0) || ((char)*local_c0 == '\0')) {
        uVar3 = FUN_14019bd40(&local_c0,iVar16,0);
        FUN_142ef7ba0(uVar3,lVar8,lVar15);
      }
      else {
        iVar16 = local_c0[-2] + iVar16;
        for (iVar7 = local_c0[-3]; iVar7 < iVar16; iVar7 = iVar7 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_c0,iVar7,1);
        if (local_c0 != (int *)0x0) {
          iVar11 = local_c0[-2];
        }
        FUN_142ef7ba0(iVar11 + lVar6,lVar8,lVar15);
      }
      FUN_14019c870(&local_c0,iVar16);
    }
  }
  if (local_b8 != 0) {
    FUN_14019f2c0(local_b8 + -0x10);
  }
  FUN_140319ad0(param_1,&local_c0);
  if (local_c0 != (int *)0x0) {
    FUN_14019f2c0(local_c0 + -4);
  }
LAB_142a1ea24:
  if (DAT_143ac18a0 == 0) {
    uVar3 = FUN_142a247e0(&local_res20,param_1,"Socket",&DAT_143271f04,&DAT_143271f04,&DAT_143271f04
                         );
    FUN_140319ad0(param_1,uVar3);
    lVar8 = CONCAT44(uStackX_24,local_res20);
  }
  else {
    local_res10 = 0;
    FUN_142a24da0(&local_res10,param_1,"Socket",DAT_143ac18a0 + 0x50,DAT_143ac18a0 + 0xc,
                  DAT_143ac18a0 + 0x154);
    FUN_140319ad0(param_1,&local_res10);
    lVar8 = local_res10;
  }
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  return param_1;
}



//===========================================================
// FUN_142e541f0 @ 142e541f0   (146 bytes)
//===========================================================

void FUN_142e541f0(undefined4 param_1,undefined8 param_2)

{
  char cVar1;
  undefined4 local_res8 [2];
  undefined8 local_res10;
  undefined4 local_res18 [2];
  longlong local_res20;
  
  local_res8[0] = param_1;
  local_res10 = param_2;
  cVar1 = FUN_142e559e0();
  if (cVar1 != '\0') {
    FUN_140194c60(&local_res20);
    local_res18[0] = FUN_14091a3e0(&local_res20);
    FUN_142e5cd30("LogCallStack4",&DAT_1434997dc,local_res18,&DAT_1434997f8,local_res8,"Info1",
                  &local_res10,&local_res20);
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
  }
  return;
}



//===========================================================
// _register_onexit_function @ 142f2f55c   (72 bytes)
//===========================================================

/* Library Function - Single Match
    _register_onexit_function
   
   Library: Visual Studio 2019 Release */

void _register_onexit_function(undefined8 param_1,undefined8 param_2)

{
  undefined8 local_res8;
  undefined8 local_res10;
  undefined1 local_res18 [8];
  undefined4 local_res20 [2];
  undefined4 local_28 [2];
  undefined8 *local_20;
  undefined8 *local_18;
  
  local_20 = &local_res8;
  local_18 = &local_res10;
  local_res20[0] = 2;
  local_28[0] = 2;
  local_res8 = param_1;
  local_res10 = param_2;
  operator()<>(local_res18,local_28,&local_20,local_res20);
  return;
}



//===========================================================
// FUN_142f2f4e0 @ 142f2f4e0   (15 bytes)
//===========================================================

void FUN_142f2f4e0(undefined8 param_1)

{
  _register_onexit_function(&DAT_143ae3278,param_1);
  return;
}


