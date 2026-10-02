
//===========================================================
// FUN_1411f4f70 @ 1411f4f70   (115 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1411f4f70(void)

{
  undefined1 auStack_498 [32];
  undefined4 local_478 [4];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  FUN_1406ed520(local_468,0x2eb);
  local_478[0] = 2;
  FUN_1406ede20(local_468,local_478,4);
  FUN_1415d01c0(local_468);
  FUN_1406ed610(local_468);
  return;
}



//===========================================================
// FUN_141601390 @ 141601390   (172 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141601390(longlong param_1)

{
  undefined1 auStack_4a8 [32];
  int local_488 [2];
  undefined8 local_480;
  undefined8 uStack_478;
  undefined8 local_470;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  if (*(longlong *)(param_1 + 8) != 0) {
    FUN_1406ed520(local_468,0x2eb);
    local_488[0] = FUN_1411f4f60(param_1);
    if ((local_488[0] == 0) || (local_488[0] == 1)) {
      FUN_1406ede20(local_468,local_488,4);
      local_480 = 0;
      uStack_478 = 0;
      local_470 = 0;
      FUN_1416011d0(local_468,param_1);
      FUN_1415d01c0(local_468);
    }
    FUN_1406ed610(local_468);
  }
  return;
}



//===========================================================
// FUN_141601440 @ 141601440   (1106 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141601440(longlong *param_1,int param_2)

{
  ulonglong uVar1;
  char cVar2;
  int iVar3;
  undefined4 uVar4;
  bool bVar5;
  longlong *plVar6;
  int *piVar7;
  longlong *plVar8;
  undefined8 *puVar9;
  longlong *plVar10;
  undefined8 *puVar11;
  longlong *plVar12;
  undefined8 *puVar13;
  undefined8 *puVar14;
  undefined8 *puVar15;
  int *piVar16;
  int *piVar17;
  undefined1 auStack_518 [32];
  longlong *local_4f8;
  ulonglong local_4f0;
  int local_4e8 [2];
  undefined8 local_4e0;
  undefined8 local_4d8;
  longlong *local_4c8;
  uint uStack_4c0;
  int *local_4a0;
  int *local_498;
  int *local_490;
  undefined1 local_488 [1104];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_518;
  if (param_1[1] != 0) {
    FUN_1406ed520(local_488,0x2eb);
    local_4e8[0] = FUN_1411f4f60(param_1);
    if ((local_4e8[0] == 0) || (local_4e8[0] == 1)) {
      FUN_1406ede20(local_488,local_4e8,4);
      piVar7 = (int *)FUN_140197eb0(4);
      piVar17 = piVar7 + 1;
      *piVar7 = param_2;
      local_4f8 = (longlong *)0x0;
      local_4f0 = 0;
      local_4a0 = piVar7;
      local_498 = piVar17;
      local_490 = piVar17;
      plVar8 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x28);
      *plVar8 = (longlong)plVar8;
      plVar8[1] = (longlong)plVar8;
      plVar8[2] = (longlong)plVar8;
      *(undefined2 *)(plVar8 + 3) = 0x101;
      local_4f8 = plVar8;
      for (piVar16 = piVar7; piVar16 != piVar17; piVar16 = piVar16 + 1) {
        puVar11 = (undefined8 *)*param_1;
        puVar15 = (undefined8 *)puVar11[1];
        cVar2 = *(char *)((longlong)puVar15 + 0x19);
        puVar9 = puVar11;
        if (cVar2 == '\0') {
          puVar13 = puVar15;
          do {
            if (*(int *)((longlong)puVar13 + 0x1c) < *piVar16) {
              puVar14 = (undefined8 *)puVar13[2];
            }
            else {
              puVar14 = (undefined8 *)*puVar13;
              puVar9 = puVar13;
            }
            puVar13 = puVar14;
          } while (*(char *)((longlong)puVar14 + 0x19) == '\0');
        }
        if (((*(char *)((longlong)puVar9 + 0x19) == '\0') &&
            (iVar3 = *piVar16, *(int *)((longlong)puVar9 + 0x1c) <= iVar3)) && (puVar9 != puVar11))
        {
          while (cVar2 == '\0') {
            if (*(int *)((longlong)puVar15 + 0x1c) < iVar3) {
              puVar9 = (undefined8 *)puVar15[2];
              puVar15 = puVar11;
            }
            else {
              puVar9 = (undefined8 *)*puVar15;
            }
            puVar11 = puVar15;
            puVar15 = puVar9;
            cVar2 = *(char *)((longlong)puVar9 + 0x19);
          }
          if ((*(char *)((longlong)puVar11 + 0x19) != '\0') ||
             (iVar3 < *(int *)((longlong)puVar11 + 0x1c))) {
                    /* WARNING: Subroutine does not return */
            FUN_142ed308c("invalid map<K, T> key");
          }
          uVar4 = *(undefined4 *)(puVar11 + 4);
          plVar10 = (longlong *)plVar8[1];
          uStack_4c0 = 0;
          cVar2 = *(char *)((longlong)plVar10 + 0x19);
          local_4c8 = plVar10;
          plVar12 = plVar8;
          while (plVar6 = plVar10, cVar2 == '\0') {
            bVar5 = iVar3 <= *(int *)((longlong)plVar6 + 0x1c);
            if (bVar5) {
              plVar10 = (longlong *)*plVar6;
              plVar12 = plVar6;
            }
            else {
              plVar10 = (longlong *)plVar6[2];
            }
            uStack_4c0 = (uint)bVar5;
            cVar2 = *(char *)((longlong)plVar10 + 0x19);
            local_4c8 = plVar6;
          }
          if ((*(char *)((longlong)plVar12 + 0x19) != '\0') ||
             (iVar3 < *(int *)((longlong)plVar12 + 0x1c))) {
            if (local_4f0 == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
              FUN_14019f9d0();
            }
            local_4e0 = &local_4f8;
            local_4d8 = 0;
            puVar11 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x28);
            *(int *)((longlong)puVar11 + 0x1c) = *piVar16;
            *(undefined4 *)(puVar11 + 4) = 0;
            *puVar11 = plVar8;
            puVar11[1] = plVar8;
            puVar11[2] = plVar8;
            *(undefined2 *)(puVar11 + 3) = 0;
            local_4d8 = 0;
            plVar12 = (longlong *)FUN_141606550(&local_4f8,&local_4c8,puVar11);
          }
          *(undefined4 *)(plVar12 + 4) = uVar4;
          plVar8 = local_4f8;
        }
      }
      if (local_4f0 == 0) {
        cVar2 = *(char *)(plVar8[1] + 0x19);
        plVar10 = (longlong *)plVar8[1];
        while (cVar2 == '\0') {
          FUN_141602610(&local_4f8,&local_4f8,plVar10[2]);
          plVar8 = (longlong *)*plVar10;
          thunk_FUN_140205820(plVar10,0x28);
          cVar2 = *(char *)((longlong)plVar8 + 0x19);
          plVar10 = plVar8;
          plVar8 = local_4f8;
        }
        thunk_FUN_140205820(plVar8,0x28);
      }
      else {
        if (local_4f0 < 0xff) {
          FUN_1406ed840(local_488,local_4f0 & 0xff);
          plVar10 = (longlong *)*local_4f8;
          cVar2 = *(char *)((longlong)plVar10 + 0x19);
          plVar8 = local_4f8;
          while (local_4f8 = plVar8, cVar2 == '\0') {
            local_4e0 = *(longlong ***)((longlong)plVar10 + 0x1c);
            FUN_1406ede20(local_488,&local_4e0,4);
            FUN_1406ed9d0(local_488,local_4e0._4_4_);
            plVar8 = (longlong *)plVar10[2];
            if (*(char *)((longlong)plVar8 + 0x19) == '\0') {
              cVar2 = *(char *)(*plVar8 + 0x19);
              plVar10 = plVar8;
              plVar8 = (longlong *)*plVar8;
              while (cVar2 == '\0') {
                cVar2 = *(char *)(*plVar8 + 0x19);
                plVar10 = plVar8;
                plVar8 = (longlong *)*plVar8;
              }
            }
            else {
              cVar2 = *(char *)(plVar10[1] + 0x19);
              plVar12 = (longlong *)plVar10[1];
              plVar8 = plVar10;
              while ((plVar10 = plVar12, cVar2 == '\0' && (plVar8 == (longlong *)plVar10[2]))) {
                cVar2 = *(char *)(plVar10[1] + 0x19);
                plVar12 = (longlong *)plVar10[1];
                plVar8 = plVar10;
              }
            }
            plVar8 = local_4f8;
            cVar2 = *(char *)((longlong)plVar10 + 0x19);
          }
        }
        cVar2 = *(char *)(plVar8[1] + 0x19);
        plVar10 = (longlong *)plVar8[1];
        while (cVar2 == '\0') {
          FUN_141602610(&local_4f8,&local_4f8,plVar10[2]);
          plVar8 = (longlong *)*plVar10;
          thunk_FUN_140205820(plVar10,0x28);
          cVar2 = *(char *)((longlong)plVar8 + 0x19);
          plVar10 = plVar8;
          plVar8 = local_4f8;
        }
        thunk_FUN_140205820(plVar8,0x28);
        FUN_1415d01c0(local_488);
      }
      if (piVar7 != (int *)0x0) {
        uVar1 = ((longlong)piVar17 - (longlong)piVar7 >> 2) * 4;
        piVar17 = piVar7;
        if (0xfff < uVar1) {
          piVar17 = *(int **)(piVar7 + -2);
          if (0x1f < (ulonglong)((longlong)piVar7 + (-8 - (longlong)piVar17))) {
                    /* WARNING: Subroutine does not return */
            FUN_142f04804(piVar17,uVar1 + 0x27);
          }
        }
        thunk_FUN_140205820(piVar17);
      }
    }
    FUN_1406ed610(local_488);
  }
  return;
}



//===========================================================
// FUN_1416018a0 @ 1416018a0   (1122 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1416018a0(longlong *param_1,undefined4 param_2,undefined4 param_3)

{
  ulonglong uVar1;
  char cVar2;
  int iVar3;
  undefined4 uVar4;
  bool bVar5;
  longlong *plVar6;
  int *piVar7;
  longlong *plVar8;
  undefined8 *puVar9;
  longlong *plVar10;
  undefined8 *puVar11;
  longlong *plVar12;
  undefined8 *puVar13;
  undefined8 *puVar14;
  undefined8 *puVar15;
  int *piVar16;
  int *piVar17;
  undefined1 auStack_518 [32];
  longlong *local_4f8;
  ulonglong local_4f0;
  int local_4e8 [2];
  undefined4 local_4e0;
  undefined4 uStack_4dc;
  undefined8 local_4d8;
  undefined8 local_4d0;
  longlong *local_4c8;
  uint uStack_4c0;
  int *local_4a8;
  int *local_4a0;
  int *local_498;
  undefined1 local_488 [1104];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_518;
  if (param_1[1] != 0) {
    FUN_1406ed520(local_488,0x2eb);
    local_4e8[0] = FUN_1411f4f60(param_1);
    if ((local_4e8[0] == 0) || (local_4e8[0] == 1)) {
      FUN_1406ede20(local_488,local_4e8,4);
      local_4e0 = param_2;
      uStack_4dc = param_3;
      piVar7 = (int *)FUN_140197eb0(8);
      piVar17 = piVar7 + 2;
      *(ulonglong *)piVar7 = CONCAT44(uStack_4dc,local_4e0);
      local_4f8 = (longlong *)0x0;
      local_4f0 = 0;
      local_4a8 = piVar7;
      local_4a0 = piVar17;
      local_498 = piVar17;
      plVar8 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x28);
      *plVar8 = (longlong)plVar8;
      plVar8[1] = (longlong)plVar8;
      plVar8[2] = (longlong)plVar8;
      *(undefined2 *)(plVar8 + 3) = 0x101;
      local_4f8 = plVar8;
      for (piVar16 = piVar7; piVar16 != piVar17; piVar16 = piVar16 + 1) {
        puVar11 = (undefined8 *)*param_1;
        puVar15 = (undefined8 *)puVar11[1];
        cVar2 = *(char *)((longlong)puVar15 + 0x19);
        puVar9 = puVar11;
        if (cVar2 == '\0') {
          puVar13 = puVar15;
          do {
            if (*(int *)((longlong)puVar13 + 0x1c) < *piVar16) {
              puVar14 = (undefined8 *)puVar13[2];
            }
            else {
              puVar14 = (undefined8 *)*puVar13;
              puVar9 = puVar13;
            }
            puVar13 = puVar14;
          } while (*(char *)((longlong)puVar14 + 0x19) == '\0');
        }
        if (((*(char *)((longlong)puVar9 + 0x19) == '\0') &&
            (iVar3 = *piVar16, *(int *)((longlong)puVar9 + 0x1c) <= iVar3)) && (puVar9 != puVar11))
        {
          while (cVar2 == '\0') {
            if (*(int *)((longlong)puVar15 + 0x1c) < iVar3) {
              puVar9 = (undefined8 *)puVar15[2];
              puVar15 = puVar11;
            }
            else {
              puVar9 = (undefined8 *)*puVar15;
            }
            puVar11 = puVar15;
            puVar15 = puVar9;
            cVar2 = *(char *)((longlong)puVar9 + 0x19);
          }
          if ((*(char *)((longlong)puVar11 + 0x19) != '\0') ||
             (iVar3 < *(int *)((longlong)puVar11 + 0x1c))) {
                    /* WARNING: Subroutine does not return */
            FUN_142ed308c("invalid map<K, T> key");
          }
          uVar4 = *(undefined4 *)(puVar11 + 4);
          plVar10 = (longlong *)plVar8[1];
          uStack_4c0 = 0;
          cVar2 = *(char *)((longlong)plVar10 + 0x19);
          local_4c8 = plVar10;
          plVar12 = plVar8;
          while (plVar6 = plVar10, cVar2 == '\0') {
            bVar5 = iVar3 <= *(int *)((longlong)plVar6 + 0x1c);
            if (bVar5) {
              plVar10 = (longlong *)*plVar6;
              plVar12 = plVar6;
            }
            else {
              plVar10 = (longlong *)plVar6[2];
            }
            uStack_4c0 = (uint)bVar5;
            cVar2 = *(char *)((longlong)plVar10 + 0x19);
            local_4c8 = plVar6;
          }
          if ((*(char *)((longlong)plVar12 + 0x19) != '\0') ||
             (iVar3 < *(int *)((longlong)plVar12 + 0x1c))) {
            if (local_4f0 == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
              FUN_14019f9d0();
            }
            local_4d8 = &local_4f8;
            local_4d0 = 0;
            puVar11 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x28);
            *(int *)((longlong)puVar11 + 0x1c) = *piVar16;
            *(undefined4 *)(puVar11 + 4) = 0;
            *puVar11 = plVar8;
            puVar11[1] = plVar8;
            puVar11[2] = plVar8;
            *(undefined2 *)(puVar11 + 3) = 0;
            local_4d0 = 0;
            plVar12 = (longlong *)FUN_141606550(&local_4f8,&local_4c8,puVar11);
          }
          *(undefined4 *)(plVar12 + 4) = uVar4;
          plVar8 = local_4f8;
        }
      }
      if (local_4f0 == 0) {
        cVar2 = *(char *)(plVar8[1] + 0x19);
        plVar10 = (longlong *)plVar8[1];
        while (cVar2 == '\0') {
          FUN_141602610(&local_4f8,&local_4f8,plVar10[2]);
          plVar8 = (longlong *)*plVar10;
          thunk_FUN_140205820(plVar10,0x28);
          cVar2 = *(char *)((longlong)plVar8 + 0x19);
          plVar10 = plVar8;
          plVar8 = local_4f8;
        }
        thunk_FUN_140205820(plVar8,0x28);
      }
      else {
        if (local_4f0 < 0xff) {
          FUN_1406ed840(local_488,local_4f0 & 0xff);
          plVar10 = (longlong *)*local_4f8;
          cVar2 = *(char *)((longlong)plVar10 + 0x19);
          plVar8 = local_4f8;
          while (local_4f8 = plVar8, cVar2 == '\0') {
            local_4d8 = *(longlong ***)((longlong)plVar10 + 0x1c);
            FUN_1406ede20(local_488,&local_4d8,4);
            FUN_1406ed9d0(local_488,local_4d8._4_4_);
            plVar8 = (longlong *)plVar10[2];
            if (*(char *)((longlong)plVar8 + 0x19) == '\0') {
              cVar2 = *(char *)(*plVar8 + 0x19);
              plVar10 = plVar8;
              plVar8 = (longlong *)*plVar8;
              while (cVar2 == '\0') {
                cVar2 = *(char *)(*plVar8 + 0x19);
                plVar10 = plVar8;
                plVar8 = (longlong *)*plVar8;
              }
            }
            else {
              cVar2 = *(char *)(plVar10[1] + 0x19);
              plVar12 = (longlong *)plVar10[1];
              plVar8 = plVar10;
              while ((plVar10 = plVar12, cVar2 == '\0' && (plVar8 == (longlong *)plVar10[2]))) {
                cVar2 = *(char *)(plVar10[1] + 0x19);
                plVar12 = (longlong *)plVar10[1];
                plVar8 = plVar10;
              }
            }
            plVar8 = local_4f8;
            cVar2 = *(char *)((longlong)plVar10 + 0x19);
          }
        }
        cVar2 = *(char *)(plVar8[1] + 0x19);
        plVar10 = (longlong *)plVar8[1];
        while (cVar2 == '\0') {
          FUN_141602610(&local_4f8,&local_4f8,plVar10[2]);
          plVar8 = (longlong *)*plVar10;
          thunk_FUN_140205820(plVar10,0x28);
          cVar2 = *(char *)((longlong)plVar8 + 0x19);
          plVar10 = plVar8;
          plVar8 = local_4f8;
        }
        thunk_FUN_140205820(plVar8,0x28);
        FUN_1415d01c0(local_488);
      }
      if (piVar7 != (int *)0x0) {
        uVar1 = ((longlong)piVar17 - (longlong)piVar7 >> 2) * 4;
        piVar17 = piVar7;
        if (0xfff < uVar1) {
          piVar17 = *(int **)(piVar7 + -2);
          if (0x1f < (ulonglong)((longlong)piVar7 + (-8 - (longlong)piVar17))) {
                    /* WARNING: Subroutine does not return */
            FUN_142f04804(piVar17,uVar1 + 0x27);
          }
        }
        thunk_FUN_140205820(piVar17);
      }
    }
    FUN_1406ed610(local_488);
  }
  return;
}



//===========================================================
// FUN_141601d10 @ 141601d10   (172 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141601d10(longlong param_1)

{
  undefined1 auStack_4a8 [32];
  int local_488 [2];
  undefined8 local_480;
  undefined8 uStack_478;
  undefined8 local_470;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  if (*(longlong *)(param_1 + 8) != 0) {
    FUN_1406ed520(local_468,0x2eb);
    local_488[0] = FUN_1411f4f50(param_1);
    if ((local_488[0] == 0) || (local_488[0] == 1)) {
      FUN_1406ede20(local_468,local_488,4);
      local_480 = 0;
      uStack_478 = 0;
      local_470 = 0;
      FUN_1416012b0(local_468,param_1);
      FUN_1415d01c0(local_468);
    }
    FUN_1406ed610(local_468);
  }
  return;
}



//===========================================================
// FUN_141601dc0 @ 141601dc0   (1106 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141601dc0(longlong *param_1,int param_2)

{
  ulonglong uVar1;
  char cVar2;
  int iVar3;
  undefined4 uVar4;
  bool bVar5;
  longlong *plVar6;
  int *piVar7;
  longlong *plVar8;
  undefined8 *puVar9;
  longlong *plVar10;
  undefined8 *puVar11;
  longlong *plVar12;
  undefined8 *puVar13;
  undefined8 *puVar14;
  undefined8 *puVar15;
  int *piVar16;
  int *piVar17;
  undefined1 auStack_518 [32];
  longlong *local_4f8;
  ulonglong local_4f0;
  int local_4e8 [2];
  undefined8 local_4e0;
  undefined8 local_4d8;
  longlong *local_4c8;
  uint uStack_4c0;
  int *local_4a0;
  int *local_498;
  int *local_490;
  undefined1 local_488 [1104];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_518;
  if (param_1[1] != 0) {
    FUN_1406ed520(local_488,0x2eb);
    local_4e8[0] = FUN_1411f4f50(param_1);
    if ((local_4e8[0] == 0) || (local_4e8[0] == 1)) {
      FUN_1406ede20(local_488,local_4e8,4);
      piVar7 = (int *)FUN_140197eb0(4);
      piVar17 = piVar7 + 1;
      *piVar7 = param_2;
      local_4f8 = (longlong *)0x0;
      local_4f0 = 0;
      local_4a0 = piVar7;
      local_498 = piVar17;
      local_490 = piVar17;
      plVar8 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x28);
      *plVar8 = (longlong)plVar8;
      plVar8[1] = (longlong)plVar8;
      plVar8[2] = (longlong)plVar8;
      *(undefined2 *)(plVar8 + 3) = 0x101;
      local_4f8 = plVar8;
      for (piVar16 = piVar7; piVar16 != piVar17; piVar16 = piVar16 + 1) {
        puVar11 = (undefined8 *)*param_1;
        puVar15 = (undefined8 *)puVar11[1];
        cVar2 = *(char *)((longlong)puVar15 + 0x19);
        puVar9 = puVar11;
        if (cVar2 == '\0') {
          puVar13 = puVar15;
          do {
            if (*(int *)((longlong)puVar13 + 0x1c) < *piVar16) {
              puVar14 = (undefined8 *)puVar13[2];
            }
            else {
              puVar14 = (undefined8 *)*puVar13;
              puVar9 = puVar13;
            }
            puVar13 = puVar14;
          } while (*(char *)((longlong)puVar14 + 0x19) == '\0');
        }
        if (((*(char *)((longlong)puVar9 + 0x19) == '\0') &&
            (iVar3 = *piVar16, *(int *)((longlong)puVar9 + 0x1c) <= iVar3)) && (puVar9 != puVar11))
        {
          while (cVar2 == '\0') {
            if (*(int *)((longlong)puVar15 + 0x1c) < iVar3) {
              puVar9 = (undefined8 *)puVar15[2];
              puVar15 = puVar11;
            }
            else {
              puVar9 = (undefined8 *)*puVar15;
            }
            puVar11 = puVar15;
            puVar15 = puVar9;
            cVar2 = *(char *)((longlong)puVar9 + 0x19);
          }
          if ((*(char *)((longlong)puVar11 + 0x19) != '\0') ||
             (iVar3 < *(int *)((longlong)puVar11 + 0x1c))) {
                    /* WARNING: Subroutine does not return */
            FUN_142ed308c("invalid map<K, T> key");
          }
          uVar4 = *(undefined4 *)(puVar11 + 4);
          plVar10 = (longlong *)plVar8[1];
          uStack_4c0 = 0;
          cVar2 = *(char *)((longlong)plVar10 + 0x19);
          local_4c8 = plVar10;
          plVar12 = plVar8;
          while (plVar6 = plVar10, cVar2 == '\0') {
            bVar5 = iVar3 <= *(int *)((longlong)plVar6 + 0x1c);
            if (bVar5) {
              plVar10 = (longlong *)*plVar6;
              plVar12 = plVar6;
            }
            else {
              plVar10 = (longlong *)plVar6[2];
            }
            uStack_4c0 = (uint)bVar5;
            cVar2 = *(char *)((longlong)plVar10 + 0x19);
            local_4c8 = plVar6;
          }
          if ((*(char *)((longlong)plVar12 + 0x19) != '\0') ||
             (iVar3 < *(int *)((longlong)plVar12 + 0x1c))) {
            if (local_4f0 == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
              FUN_14019f9d0();
            }
            local_4e0 = &local_4f8;
            local_4d8 = 0;
            puVar11 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x28);
            *(int *)((longlong)puVar11 + 0x1c) = *piVar16;
            *(undefined4 *)(puVar11 + 4) = 0;
            *puVar11 = plVar8;
            puVar11[1] = plVar8;
            puVar11[2] = plVar8;
            *(undefined2 *)(puVar11 + 3) = 0;
            local_4d8 = 0;
            plVar12 = (longlong *)FUN_1416067d0(&local_4f8,&local_4c8,puVar11);
          }
          *(undefined4 *)(plVar12 + 4) = uVar4;
          plVar8 = local_4f8;
        }
      }
      if (local_4f0 == 0) {
        cVar2 = *(char *)(plVar8[1] + 0x19);
        plVar10 = (longlong *)plVar8[1];
        while (cVar2 == '\0') {
          FUN_141602670(&local_4f8,&local_4f8,plVar10[2]);
          plVar8 = (longlong *)*plVar10;
          thunk_FUN_140205820(plVar10,0x28);
          cVar2 = *(char *)((longlong)plVar8 + 0x19);
          plVar10 = plVar8;
          plVar8 = local_4f8;
        }
        thunk_FUN_140205820(plVar8,0x28);
      }
      else {
        if (local_4f0 < 0xff) {
          FUN_1406ed840(local_488,local_4f0 & 0xff);
          plVar10 = (longlong *)*local_4f8;
          cVar2 = *(char *)((longlong)plVar10 + 0x19);
          plVar8 = local_4f8;
          while (local_4f8 = plVar8, cVar2 == '\0') {
            local_4e0 = *(longlong ***)((longlong)plVar10 + 0x1c);
            FUN_1406ede20(local_488,&local_4e0,4);
            FUN_1406ed9d0(local_488,local_4e0._4_4_);
            plVar8 = (longlong *)plVar10[2];
            if (*(char *)((longlong)plVar8 + 0x19) == '\0') {
              cVar2 = *(char *)(*plVar8 + 0x19);
              plVar10 = plVar8;
              plVar8 = (longlong *)*plVar8;
              while (cVar2 == '\0') {
                cVar2 = *(char *)(*plVar8 + 0x19);
                plVar10 = plVar8;
                plVar8 = (longlong *)*plVar8;
              }
            }
            else {
              cVar2 = *(char *)(plVar10[1] + 0x19);
              plVar12 = (longlong *)plVar10[1];
              plVar8 = plVar10;
              while ((plVar10 = plVar12, cVar2 == '\0' && (plVar8 == (longlong *)plVar10[2]))) {
                cVar2 = *(char *)(plVar10[1] + 0x19);
                plVar12 = (longlong *)plVar10[1];
                plVar8 = plVar10;
              }
            }
            plVar8 = local_4f8;
            cVar2 = *(char *)((longlong)plVar10 + 0x19);
          }
        }
        cVar2 = *(char *)(plVar8[1] + 0x19);
        plVar10 = (longlong *)plVar8[1];
        while (cVar2 == '\0') {
          FUN_141602670(&local_4f8,&local_4f8,plVar10[2]);
          plVar8 = (longlong *)*plVar10;
          thunk_FUN_140205820(plVar10,0x28);
          cVar2 = *(char *)((longlong)plVar8 + 0x19);
          plVar10 = plVar8;
          plVar8 = local_4f8;
        }
        thunk_FUN_140205820(plVar8,0x28);
        FUN_1415d01c0(local_488);
      }
      if (piVar7 != (int *)0x0) {
        uVar1 = ((longlong)piVar17 - (longlong)piVar7 >> 2) * 4;
        piVar17 = piVar7;
        if (0xfff < uVar1) {
          piVar17 = *(int **)(piVar7 + -2);
          if (0x1f < (ulonglong)((longlong)piVar7 + (-8 - (longlong)piVar17))) {
                    /* WARNING: Subroutine does not return */
            FUN_142f04804(piVar17,uVar1 + 0x27);
          }
        }
        thunk_FUN_140205820(piVar17);
      }
    }
    FUN_1406ed610(local_488);
  }
  return;
}



//===========================================================
// FUN_1411f4f60 @ 1411f4f60   (6 bytes)
//===========================================================

undefined8 FUN_1411f4f60(void)

{
  return 1;
}



//===========================================================
// FUN_1415f6660 @ 1415f6660   (2771 bytes)
//===========================================================

void FUN_1415f6660(longlong param_1)

{
  longlong lVar1;
  char cVar2;
  int iVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  int iVar6;
  int *piVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  uint local_res8 [2];
  
  lVar1 = param_1 + 0x438;
  FUN_141607520(lVar1);
  iVar3 = *(int *)(param_1 + 0x84);
  local_res8[0] = 4;
  iVar6 = FUN_140426d40(1,4);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if ((((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
         ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) || (puVar5 == puVar4)) ||
       (*(int *)(puVar5 + 4) != iVar3)) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x88);
  local_res8[0] = 5;
  iVar6 = FUN_140426d40(1,5);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if (((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
        ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) ||
       ((puVar5 == puVar4 || (*(int *)(puVar5 + 4) != iVar3)))) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0xfc);
  local_res8[0] = 0x10;
  iVar6 = FUN_140426d40(1,0x10);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if (((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
        ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) ||
       ((puVar5 == puVar4 || (*(int *)(puVar5 + 4) != iVar3)))) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x100);
  local_res8[0] = 0x11;
  iVar6 = FUN_140426d40(1,0x11);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if ((((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
         ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) || (puVar5 == puVar4)) ||
       (*(int *)(puVar5 + 4) != iVar3)) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x11c);
  local_res8[0] = 0xf;
  iVar6 = FUN_140426d40(1,0xf);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if (((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
        ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) ||
       ((puVar5 == puVar4 || (*(int *)(puVar5 + 4) != iVar3)))) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x10c);
  local_res8[0] = 6;
  iVar6 = FUN_140426d40(1,6);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if (((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
        ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) ||
       ((puVar5 == puVar4 || (*(int *)(puVar5 + 4) != iVar3)))) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x110);
  local_res8[0] = 3;
  iVar6 = FUN_140426d40(1,3);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if ((((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
         ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) || (puVar5 == puVar4)) ||
       (*(int *)(puVar5 + 4) != iVar3)) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x134);
  local_res8[0] = 0x13;
  iVar6 = FUN_140426d40(1,0x13);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if (((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
        ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) ||
       ((puVar5 == puVar4 || (*(int *)(puVar5 + 4) != iVar3)))) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x138);
  local_res8[0] = 0x14;
  iVar6 = FUN_140426d40(1,0x14);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if (((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
        ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) ||
       ((puVar5 == puVar4 || (*(int *)(puVar5 + 4) != iVar3)))) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x13c);
  local_res8[0] = 0;
  iVar6 = FUN_140426d40(1);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if ((((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
         ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) || (puVar5 == puVar4)) ||
       (*(int *)(puVar5 + 4) != iVar3)) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x140);
  local_res8[0] = 2;
  iVar6 = FUN_140426d40(1,2);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if (((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
        ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) ||
       ((puVar5 == puVar4 || (*(int *)(puVar5 + 4) != iVar3)))) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x144);
  local_res8[0] = 1;
  iVar6 = FUN_140426d40(1,1);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if (((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
        ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) ||
       ((puVar5 == puVar4 || (*(int *)(puVar5 + 4) != iVar3)))) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x148);
  local_res8[0] = 10;
  iVar6 = FUN_140426d40(1,10);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if ((((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
         ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) || (puVar5 == puVar4)) ||
       (*(int *)(puVar5 + 4) != iVar3)) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x14c);
  local_res8[0] = 0xb;
  iVar6 = FUN_140426d40(1,0xb);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if (((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
        ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) ||
       ((puVar5 == puVar4 || (*(int *)(puVar5 + 4) != iVar3)))) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x154);
  local_res8[0] = 7;
  iVar6 = FUN_140426d40(1,7);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if (((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
        ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) ||
       ((puVar5 == puVar4 || (*(int *)(puVar5 + 4) != iVar3)))) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x150);
  local_res8[0] = 0x12;
  iVar6 = FUN_140426d40(1,0x12);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if ((((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
         ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) || (puVar5 == puVar4)) ||
       (*(int *)(puVar5 + 4) != iVar3)) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x158);
  local_res8[0] = 8;
  iVar6 = FUN_140426d40(1,8);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if (((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
        ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) ||
       ((puVar5 == puVar4 || (*(int *)(puVar5 + 4) != iVar3)))) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x15c);
  local_res8[0] = 9;
  iVar6 = FUN_140426d40(1,9);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if (((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
        ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) ||
       ((puVar5 == puVar4 || (*(int *)(puVar5 + 4) != iVar3)))) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x160);
  local_res8[0] = 0xc;
  iVar6 = FUN_140426d40(1,0xc);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if ((((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
         ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) || (puVar5 == puVar4)) ||
       (*(int *)(puVar5 + 4) != iVar3)) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x164);
  local_res8[0] = 0xd;
  iVar6 = FUN_140426d40(1,0xd);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if (((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
        ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) ||
       ((puVar5 == puVar4 || (*(int *)(puVar5 + 4) != iVar3)))) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  iVar3 = *(int *)(param_1 + 0x31c);
  local_res8[0] = 0xe;
  iVar6 = FUN_140426d40(1,0xe);
  if (iVar6 != 0) {
    puVar4 = *(undefined8 **)(param_1 + 0x418);
    cVar2 = *(char *)((longlong)puVar4[1] + 0x19);
    puVar5 = puVar4;
    puVar9 = (undefined8 *)puVar4[1];
    while (cVar2 == '\0') {
      if (*(int *)((longlong)puVar9 + 0x1c) < (int)local_res8[0]) {
        puVar8 = (undefined8 *)puVar9[2];
        puVar9 = puVar5;
      }
      else {
        puVar8 = (undefined8 *)*puVar9;
      }
      puVar5 = puVar9;
      puVar9 = puVar8;
      cVar2 = *(char *)((longlong)puVar8 + 0x19);
    }
    if (((*(char *)((longlong)puVar5 + 0x19) != '\0') ||
        ((int)local_res8[0] < *(int *)((longlong)puVar5 + 0x1c))) ||
       ((puVar5 == puVar4 || (*(int *)(puVar5 + 4) != iVar3)))) {
      piVar7 = (int *)FUN_141603a40(param_1 + 0x418,local_res8);
      *piVar7 = iVar3;
      piVar7 = (int *)FUN_141603a40(lVar1,local_res8);
      *piVar7 = iVar3;
    }
  }
  if (*(longlong *)(param_1 + 0x440) != 0) {
    FUN_141601390(lVar1);
  }
  return;
}



//===========================================================
// FUN_1415f7140 @ 1415f7140   (1987 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x0001415f7306) */
/* WARNING: Removing unreachable block (ram,0x0001415f782e) */

void FUN_1415f7140(longlong param_1)

{
  longlong lVar1;
  int *piVar2;
  char cVar3;
  int iVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  int iVar7;
  int *piVar8;
  int *piVar9;
  int iVar10;
  ulonglong uVar11;
  longlong lVar12;
  ulonglong uVar13;
  int *local_res10;
  int *local_res18;
  int *local_res20;
  longlong local_78;
  longlong local_70;
  longlong local_68;
  longlong local_60;
  int *local_58;
  longlong local_50;
  undefined8 local_48;
  longlong local_40;
  
  lVar12 = DAT_143aa8518;
  lVar1 = DAT_143aa84a0;
  local_78 = DAT_143aa8518;
  if (DAT_143aa8518 == 0) {
    return;
  }
  if (DAT_143aa84a0 == 0) {
    return;
  }
  iVar4 = FUN_14276df20(DAT_143aa8518);
  if (iVar4 == *(int *)(param_1 + 0x298)) {
    return;
  }
  uVar5 = FUN_142cb9230(lVar1);
  local_res10 = (int *)CONCAT44(local_res10._4_4_,uVar5);
  *(int *)(param_1 + 0x298) = iVar4;
  FUN_141607520(param_1 + 0x418);
  FUN_141607520(param_1 + 0x438);
  if (*(longlong *)(param_1 + 0x18) != 0) {
    (*DAT_143ad5a48)();
    *(undefined8 *)(param_1 + 0x18) = 0;
  }
  local_res20 = (int *)0x0;
  FUN_14019ba10(&local_res20,PTR_s__s_g_d__d__08X_143a44fd8,
                PTR_s_SOFTWARE_Wizet_MapleStoryClassic_143a450b8,0,uVar5,iVar4);
  piVar9 = local_res20;
  local_res18 = (int *)0x0;
  uVar13 = 0xffffffffffffffff;
  if ((local_res20 != (int *)0x0) &&
     (piVar2 = local_res20 + -4, lVar12 = local_78, piVar2 != (int *)0x0)) {
    if (*piVar2 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar11 = 0xffffffffffffffff;
      do {
        uVar11 = uVar11 + 1;
      } while (*(char *)((longlong)piVar9 + uVar11) != '\0');
      iVar10 = (int)uVar11;
      iVar7 = 0;
      if (0 < iVar10) {
        iVar7 = iVar10;
      }
      piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar7 + 0x11));
      piVar8[1] = iVar7;
      *piVar8 = -1;
      piVar2 = piVar8 + 4;
      piVar8[2] = 0;
      *(undefined1 *)piVar2 = 0;
      local_68 = (longlong)iVar10;
      local_58 = piVar2;
      FUN_142ef7ba0(piVar2,piVar9,local_68);
      if (*piVar8 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= piVar8[1])) {
        *piVar8 = 1;
        if (iVar10 != -1) goto LAB_1415f72c2;
        if (piVar2 == (int *)0x0) {
          uVar11 = 0;
        }
        else {
          uVar11 = 0xffffffffffffffff;
          do {
            uVar11 = uVar11 + 1;
          } while (*(char *)((longlong)piVar2 + uVar11) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar8[1],uVar11 & 0xffffffff);
        *piVar8 = 1;
LAB_1415f72c2:
        *(undefined1 *)((longlong)piVar2 + local_68) = 0;
      }
      iVar7 = (int)uVar11;
      if ((iVar7 < 0) || (piVar8[1] + 1 <= iVar7)) {
        FUN_142e54290(0x9c,uVar11 & 0xffffffff);
      }
      piVar8[2] = iVar7;
      if (local_res18 != (int *)0x0) {
        FUN_14019f2c0();
      }
      lVar12 = local_78;
      uVar5 = local_res10._0_4_;
      local_res18 = piVar2;
    }
    else {
      if (*piVar2 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar2 = *piVar2 + 1;
      UNLOCK();
      if (local_res18 != (int *)0x0) {
        FUN_14019f2c0(local_res18 + -4);
      }
      local_res18 = piVar9;
      piVar9 = local_res20;
      lVar12 = local_78;
    }
  }
  cVar3 = FUN_1415ef160(param_1,&local_res18,param_1 + 0x18);
  if (cVar3 == '\0') goto LAB_1415f78d9;
  local_48 = FUN_142889070(lVar12);
  local_60 = lVar1;
  local_40 = lVar1;
  local_68 = param_1;
  local_50 = param_1;
  uVar6 = FUN_141604b10(&local_50,4,0x18b1d,0x3ef,1);
  *(undefined4 *)(param_1 + 0x84) = uVar6;
  uVar6 = FUN_141604b10(&local_50,5,0x18b1d,0x3f1,1);
  *(undefined4 *)(param_1 + 0x88) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_goSoulMP_143a45548,6,1,0);
  *(undefined4 *)(param_1 + 0x10c) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_goExchange_143a45540,3,1,0);
  *(undefined4 *)(param_1 + 0x110) = uVar6;
  if (DAT_143aa84a0 != 0) {
    local_70 = DAT_143aa84a0;
    local_78 = param_1;
    uVar6 = FUN_141604660(&local_78,PTR_s_soHPFlash_143a455a8,0x10,0x28,0,10,0,0x13);
    *(undefined4 *)(param_1 + 0xfc) = uVar6;
    uVar6 = FUN_141604660(&local_78,PTR_s_soMPFlash_143a455b0,0x11,0x29,0,10,0,0x13);
    *(undefined4 *)(param_1 + 0x100) = uVar6;
    iVar7 = FUN_141604660(&local_78,PTR_s_soAutoConsumePetHP_143a455b8,0xf,0x26,0,0,0,1);
    *(uint *)(param_1 + 0x11c) = (uint)(iVar7 != 0);
  }
  uVar6 = FUN_141604450(&local_68,PTR_s_goBuffAlign_143a45590,0x13,0,0);
  *(undefined4 *)(param_1 + 0x134) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_goBuffMin_143a45598,0x14,0,0);
  *(undefined4 *)(param_1 + 0x138) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_goWhisper_143a45528,0,1,0);
  *(undefined4 *)(param_1 + 0x13c) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_goMessenger_143a45538,2,1,0);
  *(undefined4 *)(param_1 + 0x140) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_goFriend_143a45530,1,1,0);
  *(undefined4 *)(param_1 + 0x144) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_goGuildTALK_143a45568,10,1,0);
  *(undefined4 *)(param_1 + 0x148) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_goAllianceTALK_143a45570,0xb,1,0);
  *(undefined4 *)(param_1 + 0x14c) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_goFollowRequest_143a455a0,0x12,1,0);
  *(undefined4 *)(param_1 + 0x150) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_goParty_143a45550,7,1,0);
  *(undefined4 *)(param_1 + 0x154) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_goGuildINVITE_143a45558,8,1,0);
  *(undefined4 *)(param_1 + 0x158) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_goAllianceINVITE_143a45560,9,1,0);
  *(undefined4 *)(param_1 + 0x15c) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_goFriendInvite_143a45578,0xc,1,0);
  *(undefined4 *)(param_1 + 0x160) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_FriendOnlineNotice_143a45580,0xd,1,0);
  *(undefined4 *)(param_1 + 0x164) = uVar6;
  uVar6 = FUN_141604450(&local_68,PTR_s_solErdaMiniUIOnOff_143a45588,0xe,0,0);
  *(undefined4 *)(param_1 + 0x31c) = uVar6;
  if (*(longlong *)(param_1 + 0x20) != 0) {
    (*DAT_143ad5a48)();
    *(undefined8 *)(param_1 + 0x20) = 0;
  }
  FUN_14019ba10(&local_res20,PTR_s__s_g2_d__d__08X_143a44fe0,
                PTR_s_SOFTWARE_Wizet_MapleStoryClassic_143a450b8,0,uVar5,iVar4);
  piVar9 = local_res20;
  local_res10 = (int *)0x0;
  piVar2 = local_res10;
  if ((local_res20 != (int *)0x0) && (piVar8 = local_res20 + -4, piVar8 != (int *)0x0)) {
    if (*piVar8 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar11 = 0xffffffffffffffff;
      do {
        uVar11 = uVar11 + 1;
      } while (*(char *)((longlong)piVar9 + uVar11) != '\0');
      iVar7 = (int)uVar11;
      iVar4 = 0;
      if (0 < iVar7) {
        iVar4 = iVar7;
      }
      piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
      piVar8[1] = iVar4;
      *piVar8 = -1;
      piVar2 = piVar8 + 4;
      piVar8[2] = 0;
      *(undefined1 *)piVar2 = 0;
      local_58 = piVar2;
      FUN_142ef7ba0(piVar2,piVar9,(longlong)iVar7);
      if (*piVar8 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar7 == -1) || (iVar7 <= piVar8[1])) {
        *piVar8 = 1;
        if (iVar7 != -1) goto LAB_1415f77ed;
        if (piVar2 == (int *)0x0) {
          uVar11 = 0;
        }
        else {
          do {
            uVar13 = uVar13 + 1;
          } while (*(char *)((longlong)piVar2 + uVar13) != '\0');
          uVar11 = uVar13 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar8[1],uVar11 & 0xffffffff);
        *piVar8 = 1;
LAB_1415f77ed:
        *(undefined1 *)((longlong)piVar2 + (longlong)iVar7) = 0;
      }
      iVar4 = (int)uVar11;
      if ((iVar4 < 0) || (piVar8[1] + 1 <= iVar4)) {
        FUN_142e54290(0x9c,uVar11 & 0xffffffff);
      }
      piVar8[2] = iVar4;
      if (local_res10 != (int *)0x0) {
        FUN_14019f2c0();
      }
    }
    else {
      if (*piVar8 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar8 = *piVar8 + 1;
      UNLOCK();
      if (local_res10 != (int *)0x0) {
        FUN_14019f2c0(local_res10 + -4);
      }
      local_res10 = piVar9;
      piVar9 = local_res20;
      piVar2 = local_res10;
    }
  }
  local_res10 = piVar2;
  cVar3 = FUN_1415ef160(param_1,&local_res10,param_1 + 0x20);
  lVar1 = DAT_143aa84a0;
  if (cVar3 != '\0') {
    if (DAT_143aa84a0 != 0) {
      *(undefined4 *)(DAT_143aa84a0 + 0x2f88) = *(undefined4 *)(param_1 + 0x134);
      *(undefined4 *)(lVar1 + 0x2f8c) = *(undefined4 *)(param_1 + 0x138);
    }
    FUN_141601390(param_1 + 0x438);
  }
LAB_1415f78d9:
  if (piVar9 != (int *)0x0) {
    FUN_14019f2c0(piVar9 + -4);
  }
  return;
}



//===========================================================
// FUN_1415f1930 @ 1415f1930   (178 bytes)
//===========================================================

void FUN_1415f1930(longlong param_1,int param_2)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  int iVar4;
  undefined8 *puVar5;
  int *piVar6;
  undefined8 *puVar7;
  int local_res10 [6];
  
  *(int *)(param_1 + 0x154) = param_2;
  local_res10[0] = 7;
  iVar4 = FUN_140426d40(1,7);
  if (iVar4 != 0) {
    puVar2 = *(undefined8 **)(param_1 + 0x418);
    cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
    puVar3 = puVar2;
    puVar7 = (undefined8 *)puVar2[1];
    while (cVar1 == '\0') {
      if (*(int *)((longlong)puVar7 + 0x1c) < local_res10[0]) {
        puVar5 = (undefined8 *)puVar7[2];
        puVar7 = puVar3;
      }
      else {
        puVar5 = (undefined8 *)*puVar7;
      }
      puVar3 = puVar7;
      puVar7 = puVar5;
      cVar1 = *(char *)((longlong)puVar5 + 0x19);
    }
    if ((((*(char *)((longlong)puVar3 + 0x19) != '\0') ||
         (local_res10[0] < *(int *)((longlong)puVar3 + 0x1c))) || (puVar3 == puVar2)) ||
       (*(int *)(puVar3 + 4) != param_2)) {
      piVar6 = (int *)FUN_141603a40(param_1 + 0x418,local_res10);
      *piVar6 = param_2;
      piVar6 = (int *)FUN_141603a40(param_1 + 0x438,local_res10);
      *piVar6 = param_2;
    }
  }
  FUN_141601440(param_1 + 0x418,7);
  return;
}



//===========================================================
// FUN_1415ff280 @ 1415ff280   (178 bytes)
//===========================================================

void FUN_1415ff280(longlong param_1,int param_2)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  int iVar4;
  undefined8 *puVar5;
  int *piVar6;
  undefined8 *puVar7;
  int local_res10 [6];
  
  *(int *)(param_1 + 0x31c) = param_2;
  local_res10[0] = 0xe;
  iVar4 = FUN_140426d40(1,0xe);
  if (iVar4 != 0) {
    puVar2 = *(undefined8 **)(param_1 + 0x418);
    cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
    puVar3 = puVar2;
    puVar7 = (undefined8 *)puVar2[1];
    while (cVar1 == '\0') {
      if (*(int *)((longlong)puVar7 + 0x1c) < local_res10[0]) {
        puVar5 = (undefined8 *)puVar7[2];
        puVar7 = puVar3;
      }
      else {
        puVar5 = (undefined8 *)*puVar7;
      }
      puVar3 = puVar7;
      puVar7 = puVar5;
      cVar1 = *(char *)((longlong)puVar5 + 0x19);
    }
    if ((((*(char *)((longlong)puVar3 + 0x19) != '\0') ||
         (local_res10[0] < *(int *)((longlong)puVar3 + 0x1c))) || (puVar3 == puVar2)) ||
       (*(int *)(puVar3 + 4) != param_2)) {
      piVar6 = (int *)FUN_141603a40(param_1 + 0x418,local_res10);
      *piVar6 = param_2;
      piVar6 = (int *)FUN_141603a40(param_1 + 0x438,local_res10);
      *piVar6 = param_2;
    }
  }
  FUN_141601440(param_1 + 0x418,0xe);
  return;
}



//===========================================================
// FUN_1415f1640 @ 1415f1640   (332 bytes)
//===========================================================

void FUN_1415f1640(longlong param_1,int param_2,int param_3)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  int iVar4;
  int *piVar5;
  undefined8 *puVar6;
  undefined8 *puVar7;
  uint local_res10 [2];
  
  *(int *)(param_1 + 0xfc) = param_2;
  *(int *)(param_1 + 0x100) = param_3;
  local_res10[0] = 0x10;
  iVar4 = FUN_140426d40(1,0x10);
  if (iVar4 != 0) {
    puVar2 = *(undefined8 **)(param_1 + 0x418);
    cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
    puVar3 = puVar2;
    puVar7 = (undefined8 *)puVar2[1];
    while (cVar1 == '\0') {
      if (*(int *)((longlong)puVar7 + 0x1c) < (int)local_res10[0]) {
        puVar6 = (undefined8 *)puVar7[2];
        puVar7 = puVar3;
      }
      else {
        puVar6 = (undefined8 *)*puVar7;
      }
      puVar3 = puVar7;
      puVar7 = puVar6;
      cVar1 = *(char *)((longlong)puVar6 + 0x19);
    }
    if ((((*(char *)((longlong)puVar3 + 0x19) != '\0') ||
         ((int)local_res10[0] < *(int *)((longlong)puVar3 + 0x1c))) || (puVar3 == puVar2)) ||
       (*(int *)(puVar3 + 4) != param_2)) {
      piVar5 = (int *)FUN_141603a40(param_1 + 0x418,local_res10);
      *piVar5 = param_2;
      piVar5 = (int *)FUN_141603a40(param_1 + 0x438,local_res10);
      *piVar5 = param_2;
    }
  }
  local_res10[0] = 0x11;
  iVar4 = FUN_140426d40(1,0x11);
  if (iVar4 != 0) {
    puVar2 = *(undefined8 **)(param_1 + 0x418);
    cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
    puVar3 = puVar2;
    puVar7 = (undefined8 *)puVar2[1];
    while (cVar1 == '\0') {
      if (*(int *)((longlong)puVar7 + 0x1c) < (int)local_res10[0]) {
        puVar6 = (undefined8 *)puVar7[2];
        puVar7 = puVar3;
      }
      else {
        puVar6 = (undefined8 *)*puVar7;
      }
      puVar3 = puVar7;
      puVar7 = puVar6;
      cVar1 = *(char *)((longlong)puVar6 + 0x19);
    }
    if (((*(char *)((longlong)puVar3 + 0x19) != '\0') ||
        ((int)local_res10[0] < *(int *)((longlong)puVar3 + 0x1c))) ||
       ((puVar3 == puVar2 || (*(int *)(puVar3 + 4) != param_3)))) {
      piVar5 = (int *)FUN_141603a40(param_1 + 0x418,local_res10);
      *piVar5 = param_3;
      piVar5 = (int *)FUN_141603a40(param_1 + 0x438,local_res10);
      *piVar5 = param_3;
    }
  }
  FUN_1416018a0(param_1 + 0x418,0x10,0x11);
  return;
}



//===========================================================
// FUN_1415f2d50 @ 1415f2d50   (4115 bytes)
//===========================================================

void FUN_1415f2d50(longlong param_1)

{
  longlong lVar1;
  char cVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  IUnknown *pIVar5;
  int iVar6;
  undefined4 uVar7;
  int iVar8;
  uint *puVar9;
  int *piVar10;
  undefined8 *puVar11;
  undefined8 *puVar12;
  uint uVar13;
  uint local_res8 [2];
  longlong local_res10;
  
  if (*(longlong *)(param_1 + 8) != 0) {
    lVar1 = param_1 + 0x428;
    FUN_141607590(lVar1);
    FUN_1415ef6e0(param_1,0,PTR_DAT_143a44f88,*(undefined8 *)(param_1 + 0x210));
    local_res8[0] = 0;
    iVar6 = (*DAT_143ad5a68)(*(undefined8 *)(param_1 + 8),PTR_s_scrFirstRun_143a44f98,0,4,local_res8
                             ,4);
    if (iVar6 != 0) {
      local_res10 = 0;
      FUN_1416051f0(&local_res10,iVar6);
      if (local_res10 != 0) {
        FUN_1401bebb0(local_res10 + -0x10);
      }
    }
    iVar6 = FUN_1410a3dc0(2);
    pIVar5 = DAT_143add050;
    if (*(int *)(param_1 + 0x44) == iVar6) {
      if (DAT_143add050 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      local_res8[0] = 0;
      iVar6 = (**(code **)(*(longlong *)DAT_143add050 + 0xf8))(DAT_143add050,local_res8);
      if (iVar6 < 0) {
        _com_issue_errorex(iVar6,pIVar5,(_GUID *)&DAT_14327fcd0);
      }
      if (local_res8[0] == 0) {
        uVar7 = FUN_1410a3dc0(0);
        *(undefined4 *)(param_1 + 0x44) = uVar7;
      }
    }
    local_res8[0] = *(uint *)(param_1 + 0x44);
    iVar6 = (*DAT_143ad5a68)(*(undefined8 *)(param_1 + 8),PTR_s_soScreenMode_143a44fd0,0,4,
                             local_res8,4);
    if (iVar6 != 0) {
      local_res10 = 0;
      FUN_1416051f0(&local_res10,iVar6);
      if (local_res10 != 0) {
        FUN_1401bebb0(local_res10 + -0x10);
      }
    }
    FUN_1415f3d70(param_1);
    iVar6 = *(int *)(param_1 + 0x4c);
    local_res8[0] = 0x15;
    iVar8 = FUN_140426d40(0,0x15);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      uVar13 = (uint)(iVar6 != 0);
      if ((((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
           ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) || (puVar4 == puVar3)) ||
         (*(uint *)(puVar4 + 4) != uVar13)) {
        puVar9 = (uint *)FUN_141603b40(param_1 + 0x408,local_res8);
        *puVar9 = uVar13;
        puVar9 = (uint *)FUN_141603b40(lVar1,local_res8);
        *puVar9 = uVar13;
      }
    }
    iVar6 = *(int *)(param_1 + 0x50);
    local_res8[0] = 0x1b;
    iVar8 = FUN_140426d40(0,0x1b);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if (((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
          ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) ||
         ((puVar4 == puVar3 || (*(int *)(puVar4 + 4) != iVar6)))) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    iVar6 = *(int *)(param_1 + 0x54);
    local_res8[0] = 0x1c;
    iVar8 = FUN_140426d40(0,0x1c);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if (((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
          ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) ||
         ((puVar4 == puVar3 || (*(int *)(puVar4 + 4) != iVar6)))) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    iVar6 = *(int *)(param_1 + 0x68);
    local_res8[0] = 0x14;
    iVar8 = FUN_140426d40(0,0x14);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if ((((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
           ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) || (puVar4 == puVar3)) ||
         (*(int *)(puVar4 + 4) != iVar6)) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    iVar6 = *(int *)(param_1 + 0x6c);
    local_res8[0] = 0x1e;
    iVar8 = FUN_140426d40(0,0x1e);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if (((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
          ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) ||
         ((puVar4 == puVar3 || (*(int *)(puVar4 + 4) != iVar6)))) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    iVar6 = *(int *)(param_1 + 0x70);
    local_res8[0] = 0x20;
    iVar8 = FUN_140426d40(0,0x20);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if (((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
          ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) ||
         ((puVar4 == puVar3 || (*(int *)(puVar4 + 4) != iVar6)))) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    iVar6 = *(int *)(param_1 + 0x48);
    local_res8[0] = 0x2a;
    iVar8 = FUN_140426d40(0,0x2a);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if ((((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
           ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) || (puVar4 == puVar3)) ||
         (*(int *)(puVar4 + 4) != iVar6)) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    local_res8[0] = (uint)(*(int *)(param_1 + 0x80) != 0);
    iVar6 = (*DAT_143ad5a68)(*(undefined8 *)(param_1 + 8),
                             PTR_s_soNotShowRemoteLargeScaleEffect_143a450f8,0,4,local_res8,4);
    if (iVar6 != 0) {
      local_res10 = 0;
      FUN_1416051f0(&local_res10,iVar6);
      if (local_res10 != 0) {
        FUN_1401bebb0(local_res10 + -0x10);
      }
    }
    iVar6 = *(int *)(param_1 + 0x58);
    local_res8[0] = 0x3c;
    iVar8 = FUN_140426d40(0,0x3c);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if (((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
          ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) ||
         ((puVar4 == puVar3 || (*(int *)(puVar4 + 4) != iVar6)))) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    iVar6 = *(int *)(param_1 + 0x5c);
    local_res8[0] = 0x3d;
    iVar8 = FUN_140426d40(0,0x3d);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if (((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
          ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) ||
         ((puVar4 == puVar3 || (*(int *)(puVar4 + 4) != iVar6)))) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    iVar6 = *(int *)(param_1 + 0x60);
    local_res8[0] = 0x3e;
    iVar8 = FUN_140426d40(0,0x3e);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if ((((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
           ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) || (puVar4 == puVar3)) ||
         (*(int *)(puVar4 + 4) != iVar6)) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    iVar6 = *(int *)(param_1 + 100);
    local_res8[0] = 0x3f;
    iVar8 = FUN_140426d40(0,0x3f);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if (((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
          ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) ||
         ((puVar4 == puVar3 || (*(int *)(puVar4 + 4) != iVar6)))) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    FUN_1415f4c80(param_1);
    FUN_1415f5110(param_1);
    iVar6 = *(int *)(param_1 + 0xbc);
    local_res8[0] = 0;
    iVar8 = FUN_140426d40(0,0);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if (((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
          ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) ||
         ((puVar4 == puVar3 || (*(int *)(puVar4 + 4) != iVar6)))) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    local_res8[0] = *(uint *)(param_1 + 0xbc);
    iVar6 = (*DAT_143ad5a68)(*(undefined8 *)(param_1 + 8),PTR_s_soBGMVol_143a44fb0,0,4,local_res8,4)
    ;
    if (iVar6 != 0) {
      local_res10 = 0;
      FUN_1416051f0(&local_res10,iVar6);
      if (local_res10 != 0) {
        FUN_1401bebb0(local_res10 + -0x10);
      }
    }
    iVar6 = *(int *)(param_1 + 0xc4);
    local_res8[0] = 2;
    iVar8 = FUN_140426d40(0,2);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if ((((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
           ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) || (puVar4 == puVar3)) ||
         (*(int *)(puVar4 + 4) != iVar6)) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    iVar6 = *(int *)(param_1 + 0xd0);
    local_res8[0] = 7;
    iVar8 = FUN_140426d40(0,7);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if (((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
          ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) ||
         ((puVar4 == puVar3 || (*(int *)(puVar4 + 4) != iVar6)))) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    iVar6 = *(int *)(param_1 + 0xd8);
    local_res8[0] = 5;
    iVar8 = FUN_140426d40(0,5);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if (((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
          ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) ||
         ((puVar4 == puVar3 || (*(int *)(puVar4 + 4) != iVar6)))) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    iVar6 = *(int *)(param_1 + 0xf4);
    local_res8[0] = 0xb;
    iVar8 = FUN_140426d40(0,0xb);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if ((((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
           ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) || (puVar4 == puVar3)) ||
         (*(int *)(puVar4 + 4) != iVar6)) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    if (*(int *)(param_1 + 0x218) == 0) {
      iVar6 = *(int *)(param_1 + 0xec);
      local_res8[0] = 9;
      iVar8 = FUN_140426d40(0,9);
      if (iVar8 != 0) {
        puVar3 = *(undefined8 **)(param_1 + 0x408);
        cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
        puVar4 = puVar3;
        puVar12 = (undefined8 *)puVar3[1];
        while (cVar2 == '\0') {
          if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
            puVar11 = (undefined8 *)puVar12[2];
            puVar12 = puVar4;
          }
          else {
            puVar11 = (undefined8 *)*puVar12;
          }
          puVar4 = puVar12;
          puVar12 = puVar11;
          cVar2 = *(char *)((longlong)puVar11 + 0x19);
        }
        if (((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
            ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) ||
           ((puVar4 == puVar3 || (*(int *)(puVar4 + 4) != iVar6)))) {
          piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
          *piVar10 = iVar6;
          piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
          *piVar10 = iVar6;
        }
      }
    }
    iVar6 = *(int *)(param_1 + 0xc0);
    local_res8[0] = 1;
    iVar8 = FUN_140426d40(0,1);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if (((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
          ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) ||
         ((puVar4 == puVar3 || (*(int *)(puVar4 + 4) != iVar6)))) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    local_res8[0] = (uint)(*(int *)(param_1 + 0xc0) != 0);
    iVar6 = (*DAT_143ad5a68)(*(undefined8 *)(param_1 + 8),PTR_s_soBGMMute_143a44fb8,0,4,local_res8,4
                            );
    if (iVar6 != 0) {
      local_res10 = 0;
      FUN_1416051f0(&local_res10,iVar6);
      if (local_res10 != 0) {
        FUN_1401bebb0(local_res10 + -0x10);
      }
    }
    iVar6 = *(int *)(param_1 + 200);
    local_res8[0] = 3;
    iVar8 = FUN_140426d40(0,3);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if ((((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
           ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) || (puVar4 == puVar3)) ||
         (*(int *)(puVar4 + 4) != iVar6)) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    iVar6 = *(int *)(param_1 + 0xcc);
    local_res8[0] = 4;
    iVar8 = FUN_140426d40(0,4);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if (((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
          ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) ||
         ((puVar4 == puVar3 || (*(int *)(puVar4 + 4) != iVar6)))) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    iVar6 = *(int *)(param_1 + 0xd4);
    local_res8[0] = 8;
    iVar8 = FUN_140426d40(0,8);
    if (iVar8 != 0) {
      puVar3 = *(undefined8 **)(param_1 + 0x408);
      cVar2 = *(char *)((longlong)puVar3[1] + 0x19);
      puVar4 = puVar3;
      puVar12 = (undefined8 *)puVar3[1];
      while (cVar2 == '\0') {
        if (*(int *)((longlong)puVar12 + 0x1c) < (int)local_res8[0]) {
          puVar11 = (undefined8 *)puVar12[2];
          puVar12 = puVar4;
        }
        else {
          puVar11 = (undefined8 *)*puVar12;
        }
        puVar4 = puVar12;
        puVar12 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if (((*(char *)((longlong)puVar4 + 0x19) != '\0') ||
          ((int)local_res8[0] < *(int *)((longlong)puVar4 + 0x1c))) ||
         ((puVar4 == puVar3 || (*(int *)(puVar4 + 4) != iVar6)))) {
        piVar10 = (int *)FUN_141603b40(param_1 + 0x408,local_res8);
        *piVar10 = iVar6;
        piVar10 = (int *)FUN_141603b40(lVar1,local_res8);
        *piVar10 = iVar6;
      }
    }
    FUN_1415f4010(param_1,6,*(undefined4 *)(param_1 + 0xdc));
    FUN_1415f4010(param_1,0xc,*(undefined4 *)(param_1 + 0xf8));
    FUN_1415f4010(param_1,10,*(undefined4 *)(param_1 + 0xf0));
    FUN_1415f4010(param_1,0x12,*(undefined4 *)(param_1 + 0x104));
    FUN_1415f4010(param_1,0x40,*(undefined4 *)(param_1 + 0x108));
    FUN_1415f4010(param_1,0x22,*(undefined4 *)(param_1 + 0x114));
    FUN_1415f4010(param_1,0x23,*(undefined4 *)(param_1 + 0x118));
    FUN_1415f4010(param_1,0x24,*(undefined4 *)(param_1 + 0x168));
    FUN_1415f4010(param_1,0x25,*(undefined4 *)(param_1 + 0x16c));
    FUN_1415f4010(param_1,0x16,*(undefined4 *)(param_1 + 0x170));
    FUN_1415f4010(param_1,0x17,*(undefined4 *)(param_1 + 0x174));
    FUN_1415f4010(param_1,0x18,*(undefined4 *)(param_1 + 0x178));
    FUN_1415f4010(param_1,0x19,*(undefined4 *)(param_1 + 0x17c));
    FUN_1415f4010(param_1,0x1a,*(undefined4 *)(param_1 + 0x180));
    FUN_1415f4010(param_1,0x1d,*(undefined4 *)(param_1 + 0x124));
    FUN_1415f4010(param_1,0x1f,*(undefined4 *)(param_1 + 0x128));
    FUN_1415f4010(param_1,0x13,*(undefined4 *)(param_1 + 300));
    FUN_1415f4010(param_1,0x27,*(undefined4 *)(param_1 + 0x130));
    FUN_1415f4010(param_1,0xd,*(undefined4 *)(param_1 + 0x184));
    FUN_1415f4010(param_1,0xe,*(undefined4 *)(param_1 + 0x188));
    FUN_1415f4010(param_1,0xf,*(undefined4 *)(param_1 + 0x18c));
    FUN_1415f4010(param_1,0x10,*(undefined4 *)(param_1 + 400));
    FUN_1415f4010(param_1,0x11,*(undefined4 *)(param_1 + 0x194));
    FUN_1415ef2c0(param_1,0,PTR_s_MemoryMappedIO_143a44fc0,*(undefined4 *)(param_1 + 0x198));
    if (*(int *)(param_1 + 0x198) == 0) {
      FUN_1415ef8a0(param_1,0,PTR_s_64bitFlushMemorySize_143a44fc8,*(undefined4 *)(param_1 + 0x19c))
      ;
    }
    FUN_1415ef8a0(param_1,0,PTR_s_LSMSuggestionCount_143a44fa0,*(undefined4 *)(param_1 + 0x270));
    FUN_1415ef2c0(param_1,0,PTR_s_cEnable_143a456a0,*(undefined4 *)(param_1 + 0x1b8));
    FUN_1415ef8a0(param_1,0,PTR_s_cSense_143a456a8,*(undefined4 *)(param_1 + 0x1bc));
    FUN_1415ef8a0(param_1,0,PTR_s_cDpad_143a456b0,*(undefined4 *)(param_1 + 0x1c0));
    if (*(longlong *)(param_1 + 0x430) != 0) {
      FUN_141601d10(lVar1);
    }
  }
  return;
}



//===========================================================
// FUN_1415f4380 @ 1415f4380   (1301 bytes)
//===========================================================

void FUN_1415f4380(longlong param_1,undefined4 param_2)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  uint uVar4;
  int iVar5;
  uint uVar6;
  undefined4 uVar7;
  uint *puVar8;
  undefined8 *puVar9;
  uint *puVar10;
  undefined8 *puVar11;
  int local_res8 [2];
  int local_res10 [2];
  uint local_res18 [2];
  undefined8 in_stack_ffffffffffffffb8;
  ulonglong uVar12;
  undefined8 uVar13;
  undefined8 in_stack_ffffffffffffffc0;
  int *piVar14;
  ulonglong uVar15;
  
  uVar7 = (undefined4)((ulonglong)in_stack_ffffffffffffffb8 >> 0x20);
  uVar6 = (uint)((ulonglong)in_stack_ffffffffffffffc0 >> 0x20);
  FUN_141607590(param_1 + 0x428);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soBGMVol_143a44fb0,0,CONCAT44(uVar7,0xf),
                        (ulonglong)uVar6 << 0x20,0x13);
  piVar14 = local_res10;
  puVar10 = local_res18;
  local_res10[0] = 4;
  iVar5 = (*DAT_143ad5a60)(*(undefined8 *)(param_1 + 8),PTR_s_soBGMVol_143a44fb0,0,local_res8,
                           puVar10,piVar14);
  uVar6 = local_res18[0];
  if ((((iVar5 == 0) && (local_res8[0] == 4)) && (local_res10[0] == 4)) &&
     ((local_res18[0] < 0x14 && (uVar4 != local_res18[0])))) {
    *(uint *)(param_1 + 0xbc) = local_res18[0];
    local_res10[0] = 0;
    iVar5 = FUN_140426d40(0,0);
    if (iVar5 != 0) {
      puVar2 = *(undefined8 **)(param_1 + 0x408);
      cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
      puVar3 = puVar2;
      puVar11 = (undefined8 *)puVar2[1];
      while (cVar1 == '\0') {
        if (*(int *)((longlong)puVar11 + 0x1c) < local_res10[0]) {
          puVar9 = (undefined8 *)puVar11[2];
          puVar11 = puVar3;
        }
        else {
          puVar9 = (undefined8 *)*puVar11;
        }
        puVar3 = puVar11;
        puVar11 = puVar9;
        cVar1 = *(char *)((longlong)puVar9 + 0x19);
      }
      if (((*(char *)((longlong)puVar3 + 0x19) != '\0') ||
          (local_res10[0] < *(int *)((longlong)puVar3 + 0x1c))) ||
         ((puVar3 == puVar2 || (*(uint *)(puVar3 + 4) != uVar6)))) {
        puVar8 = (uint *)FUN_141603b40(param_1 + 0x408,local_res10);
        *puVar8 = uVar6;
        puVar8 = (uint *)FUN_141603b40(param_1 + 0x428,local_res10);
        *puVar8 = uVar6;
      }
    }
  }
  else {
    *(uint *)(param_1 + 0xbc) = uVar4;
  }
  if (((*(int *)(param_1 + 0xf0) == 0) && (*(int *)(param_1 + 0xc0) == 0)) &&
     (iVar5 = *(int *)(param_1 + 0xec) * *(int *)(param_1 + 0xbc), 0 < iVar5)) {
    iVar5 = ((iVar5 / 0x14 + 1) * 100) / 0x14;
  }
  else {
    iVar5 = 0;
  }
  *(int *)(param_1 + 0xe0) = iVar5;
  uVar6 = FUN_1415f40c0(param_1,param_2,PTR_s_soBGMMute_143a44fb8,1,
                        (ulonglong)puVar10 & 0xffffffff00000000,
                        (ulonglong)piVar14 & 0xffffffff00000000,1);
  piVar14 = local_res10;
  puVar10 = local_res18;
  local_res10[0] = 4;
  iVar5 = (*DAT_143ad5a60)(*(undefined8 *)(param_1 + 8),PTR_s_soBGMMute_143a44fb8,0,local_res8,
                           puVar10,piVar14);
  uVar7 = (undefined4)((ulonglong)puVar10 >> 0x20);
  if (((iVar5 == 0) && (local_res8[0] == 4)) &&
     ((local_res10[0] == 4 && (uVar4 = (uint)(local_res18[0] != 0), uVar6 != uVar4)))) {
    *(uint *)(param_1 + 0xc0) = uVar4;
    local_res10[0] = 1;
    iVar5 = FUN_140426d40(0,1);
    if (iVar5 != 0) {
      puVar2 = *(undefined8 **)(param_1 + 0x408);
      cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
      puVar3 = puVar2;
      puVar11 = (undefined8 *)puVar2[1];
      while (cVar1 == '\0') {
        if (*(int *)((longlong)puVar11 + 0x1c) < local_res10[0]) {
          puVar9 = (undefined8 *)puVar11[2];
          puVar11 = puVar3;
        }
        else {
          puVar9 = (undefined8 *)*puVar11;
        }
        puVar3 = puVar11;
        puVar11 = puVar9;
        cVar1 = *(char *)((longlong)puVar9 + 0x19);
      }
      if (((*(char *)((longlong)puVar3 + 0x19) != '\0') ||
          (local_res10[0] < *(int *)((longlong)puVar3 + 0x1c))) ||
         ((puVar3 == puVar2 || (*(uint *)(puVar3 + 4) != uVar4)))) {
        puVar10 = (uint *)FUN_141603b40(param_1 + 0x408,local_res10);
        *puVar10 = uVar4;
        puVar10 = (uint *)FUN_141603b40(param_1 + 0x428,local_res10);
        *puVar10 = uVar4;
      }
    }
  }
  else {
    *(uint *)(param_1 + 0xc0) = uVar6;
  }
  uVar15 = (ulonglong)piVar14 & 0xffffffff00000000;
  uVar12 = CONCAT44(uVar7,0xf);
  uVar7 = FUN_1415f40c0(param_1,param_2,PTR_s_soSEVol_143a455c0,2,uVar12,uVar15,0x13);
  *(undefined4 *)(param_1 + 0xc4) = uVar7;
  uVar15 = uVar15 & 0xffffffff00000000;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar7 = FUN_1415f40c0(param_1,param_2,PTR_s_soSEMute_143a455c8,3,uVar12,uVar15,1);
  *(undefined4 *)(param_1 + 200) = uVar7;
  uVar15 = uVar15 & 0xffffffff00000000;
  uVar13 = CONCAT44((int)(uVar12 >> 0x20),1);
  uVar7 = FUN_1415f40c0(param_1,param_2,PTR_s_soAndroidMute_143a455d0,4,uVar13,uVar15,1);
  *(undefined4 *)(param_1 + 0xcc) = uVar7;
  uVar15 = uVar15 & 0xffffffff00000000;
  uVar12 = CONCAT44((int)((ulonglong)uVar13 >> 0x20),0xf);
  uVar7 = FUN_1415f40c0(param_1,param_2,PTR_s_soVoiceVol_143a455e8,7,uVar12,uVar15,0x13);
  *(undefined4 *)(param_1 + 0xd0) = uVar7;
  uVar15 = uVar15 & 0xffffffff00000000;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar7 = FUN_1415f40c0(param_1,param_2,PTR_s_soVoiceMute_143a455f0,8,uVar12,uVar15,1);
  *(undefined4 *)(param_1 + 0xd4) = uVar7;
  uVar15 = uVar15 & 0xffffffff00000000;
  uVar12 = CONCAT44((int)(uVar12 >> 0x20),0xf);
  uVar7 = FUN_1415f40c0(param_1,param_2,PTR_s_soSSEVol_143a455d8,5,uVar12,uVar15,0x13);
  *(undefined4 *)(param_1 + 0xd8) = uVar7;
  uVar15 = uVar15 & 0xffffffff00000000;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar7 = FUN_1415f40c0(param_1,param_2,PTR_s_soSSEMute_143a455e0,6,uVar12,uVar15,1);
  *(undefined4 *)(param_1 + 0xdc) = uVar7;
  uVar15 = uVar15 & 0xffffffff00000000;
  uVar12 = CONCAT44((int)(uVar12 >> 0x20),0xf);
  uVar7 = FUN_1415f40c0(param_1,param_2,PTR_s_soMSEVol_143a45608,0xb,uVar12,uVar15,0x13);
  *(undefined4 *)(param_1 + 0xf4) = uVar7;
  uVar15 = uVar15 & 0xffffffff00000000;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar7 = FUN_1415f40c0(param_1,param_2,PTR_s_soMSEMute_143a45610,0xc,uVar12,uVar15,1);
  *(undefined4 *)(param_1 + 0xf8) = uVar7;
  uVar15 = uVar15 & 0xffffffff00000000;
  uVar12 = CONCAT44((int)(uVar12 >> 0x20),0x13);
  uVar7 = FUN_1415f40c0(param_1,param_2,PTR_s_soMasterVol_143a455f8,9,uVar12,uVar15,0x13);
  *(undefined4 *)(param_1 + 0xec) = uVar7;
  uVar7 = FUN_1415f40c0(param_1,param_2,PTR_s_soMasterMute_143a45600,10,uVar12 & 0xffffffff00000000,
                        uVar15 & 0xffffffff00000000,1);
  *(undefined4 *)(param_1 + 0xf0) = uVar7;
  FUN_1415f1350(param_1);
  FUN_141601d10(param_1 + 0x428);
  return;
}



//===========================================================
// FUN_1415f11b0 @ 1415f11b0   (175 bytes)
//===========================================================

void FUN_1415f11b0(longlong param_1,int param_2)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  int iVar4;
  undefined8 *puVar5;
  int *piVar6;
  undefined8 *puVar7;
  int local_res10 [6];
  
  *(int *)(param_1 + 0xf0) = param_2;
  local_res10[0] = 10;
  iVar4 = FUN_140426d40(0,10);
  if (iVar4 != 0) {
    puVar2 = *(undefined8 **)(param_1 + 0x408);
    cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
    puVar3 = puVar2;
    puVar7 = (undefined8 *)puVar2[1];
    while (cVar1 == '\0') {
      if (*(int *)((longlong)puVar7 + 0x1c) < local_res10[0]) {
        puVar5 = (undefined8 *)puVar7[2];
        puVar7 = puVar3;
      }
      else {
        puVar5 = (undefined8 *)*puVar7;
      }
      puVar3 = puVar7;
      puVar7 = puVar5;
      cVar1 = *(char *)((longlong)puVar5 + 0x19);
    }
    if ((((*(char *)((longlong)puVar3 + 0x19) != '\0') ||
         (local_res10[0] < *(int *)((longlong)puVar3 + 0x1c))) || (puVar3 == puVar2)) ||
       (*(int *)(puVar3 + 4) != param_2)) {
      piVar6 = (int *)FUN_141603b40(param_1 + 0x408,local_res10);
      *piVar6 = param_2;
      piVar6 = (int *)FUN_141603b40(param_1 + 0x428,local_res10);
      *piVar6 = param_2;
    }
  }
  FUN_141601dc0(param_1 + 0x408,10);
  return;
}



//===========================================================
// FUN_1415f1580 @ 1415f1580   (175 bytes)
//===========================================================

void FUN_1415f1580(longlong param_1,int param_2)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  int iVar4;
  undefined8 *puVar5;
  int *piVar6;
  undefined8 *puVar7;
  int local_res10 [6];
  
  *(int *)(param_1 + 0x168) = param_2;
  local_res10[0] = 0x24;
  iVar4 = FUN_140426d40(0,0x24);
  if (iVar4 != 0) {
    puVar2 = *(undefined8 **)(param_1 + 0x408);
    cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
    puVar3 = puVar2;
    puVar7 = (undefined8 *)puVar2[1];
    while (cVar1 == '\0') {
      if (*(int *)((longlong)puVar7 + 0x1c) < local_res10[0]) {
        puVar5 = (undefined8 *)puVar7[2];
        puVar7 = puVar3;
      }
      else {
        puVar5 = (undefined8 *)*puVar7;
      }
      puVar3 = puVar7;
      puVar7 = puVar5;
      cVar1 = *(char *)((longlong)puVar5 + 0x19);
    }
    if ((((*(char *)((longlong)puVar3 + 0x19) != '\0') ||
         (local_res10[0] < *(int *)((longlong)puVar3 + 0x1c))) || (puVar3 == puVar2)) ||
       (*(int *)(puVar3 + 4) != param_2)) {
      piVar6 = (int *)FUN_141603b40(param_1 + 0x408,local_res10);
      *piVar6 = param_2;
      piVar6 = (int *)FUN_141603b40(param_1 + 0x428,local_res10);
      *piVar6 = param_2;
    }
  }
  FUN_141601dc0(param_1 + 0x408,0x24);
  return;
}



//===========================================================
// FUN_1415f17a0 @ 1415f17a0   (198 bytes)
//===========================================================

void FUN_1415f17a0(longlong param_1,int param_2,int param_3)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  int iVar4;
  undefined8 *puVar5;
  int *piVar6;
  undefined8 *puVar7;
  int iVar8;
  int local_res10 [2];
  
  if (param_2 == 0) {
    *(int *)(param_1 + 0x54) = param_3;
    iVar8 = 0x1c;
  }
  else {
    *(int *)(param_1 + 0x50) = param_3;
    iVar8 = 0x1b;
  }
  local_res10[0] = iVar8;
  iVar4 = FUN_140426d40(0,iVar8);
  if (iVar4 != 0) {
    puVar2 = *(undefined8 **)(param_1 + 0x408);
    cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
    puVar3 = puVar2;
    puVar7 = (undefined8 *)puVar2[1];
    while (cVar1 == '\0') {
      if (*(int *)((longlong)puVar7 + 0x1c) < local_res10[0]) {
        puVar5 = (undefined8 *)puVar7[2];
        puVar7 = puVar3;
      }
      else {
        puVar5 = (undefined8 *)*puVar7;
      }
      puVar3 = puVar7;
      puVar7 = puVar5;
      cVar1 = *(char *)((longlong)puVar5 + 0x19);
    }
    if ((((*(char *)((longlong)puVar3 + 0x19) != '\0') ||
         (local_res10[0] < *(int *)((longlong)puVar3 + 0x1c))) || (puVar3 == puVar2)) ||
       (*(int *)(puVar3 + 4) != param_3)) {
      piVar6 = (int *)FUN_141603b40(param_1 + 0x408,local_res10);
      *piVar6 = param_3;
      piVar6 = (int *)FUN_141603b40(param_1 + 0x428,local_res10);
      *piVar6 = param_3;
    }
  }
  FUN_141601dc0(param_1 + 0x408,iVar8);
  return;
}



//===========================================================
// FUN_1415f1870 @ 1415f1870   (175 bytes)
//===========================================================

void FUN_1415f1870(longlong param_1,int param_2)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  int iVar4;
  undefined8 *puVar5;
  int *piVar6;
  undefined8 *puVar7;
  int local_res10 [6];
  
  *(int *)(param_1 + 0x124) = param_2;
  local_res10[0] = 0x1d;
  iVar4 = FUN_140426d40(0,0x1d);
  if (iVar4 != 0) {
    puVar2 = *(undefined8 **)(param_1 + 0x408);
    cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
    puVar3 = puVar2;
    puVar7 = (undefined8 *)puVar2[1];
    while (cVar1 == '\0') {
      if (*(int *)((longlong)puVar7 + 0x1c) < local_res10[0]) {
        puVar5 = (undefined8 *)puVar7[2];
        puVar7 = puVar3;
      }
      else {
        puVar5 = (undefined8 *)*puVar7;
      }
      puVar3 = puVar7;
      puVar7 = puVar5;
      cVar1 = *(char *)((longlong)puVar5 + 0x19);
    }
    if ((((*(char *)((longlong)puVar3 + 0x19) != '\0') ||
         (local_res10[0] < *(int *)((longlong)puVar3 + 0x1c))) || (puVar3 == puVar2)) ||
       (*(int *)(puVar3 + 4) != param_2)) {
      piVar6 = (int *)FUN_141603b40(param_1 + 0x408,local_res10);
      *piVar6 = param_2;
      piVar6 = (int *)FUN_141603b40(param_1 + 0x428,local_res10);
      *piVar6 = param_2;
    }
  }
  FUN_141601dc0(param_1 + 0x408,0x1d);
  return;
}



//===========================================================
// FUN_1415f1270 @ 1415f1270   (181 bytes)
//===========================================================

void FUN_1415f1270(longlong param_1,undefined4 param_2,undefined4 param_3)

{
  undefined8 uVar1;
  
  switch(param_2) {
  case 0:
    *(undefined4 *)(param_1 + 0xc0) = param_3;
    uVar1 = 1;
    break;
  case 1:
    *(undefined4 *)(param_1 + 200) = param_3;
    uVar1 = 3;
    break;
  case 2:
    *(undefined4 *)(param_1 + 0xcc) = param_3;
    uVar1 = 4;
    break;
  case 3:
    *(undefined4 *)(param_1 + 0xdc) = param_3;
    uVar1 = 6;
    break;
  case 4:
    *(undefined4 *)(param_1 + 0xd4) = param_3;
    uVar1 = 8;
    break;
  case 5:
    *(undefined4 *)(param_1 + 0xf0) = param_3;
    uVar1 = 10;
    break;
  case 6:
    *(undefined4 *)(param_1 + 0xf8) = param_3;
    uVar1 = 0xc;
    break;
  default:
    goto switchD_1415f129a_default;
  }
  FUN_1415f4010(param_1,uVar1,param_3,1);
  FUN_141601dc0(param_1 + 0x408,uVar1);
switchD_1415f129a_default:
  return;
}



//===========================================================
// FUN_1415f22f0 @ 1415f22f0   (2094 bytes)
//===========================================================

void FUN_1415f22f0(longlong param_1,int param_2)

{
  longlong lVar1;
  char cVar2;
  int iVar3;
  undefined4 uVar4;
  uint uVar5;
  uint uVar6;
  undefined4 uVar7;
  int local_res8 [2];
  int local_res18 [2];
  uint local_res20 [2];
  undefined8 in_stack_ffffffffffffffa8;
  undefined8 uVar8;
  ulonglong uVar9;
  uint *puVar10;
  undefined4 uVar11;
  undefined8 in_stack_ffffffffffffffb0;
  ulonglong uVar12;
  int *piVar13;
  ulonglong local_38 [2];
  
  lVar1 = DAT_143aa8518;
  uVar4 = (undefined4)((ulonglong)in_stack_ffffffffffffffa8 >> 0x20);
  uVar7 = (undefined4)((ulonglong)in_stack_ffffffffffffffb0 >> 0x20);
  if (*(longlong *)(param_1 + 8) == 0) {
    return;
  }
  if (DAT_143aa84a0 == 0) {
    return;
  }
  if (param_2 == 0) {
    if (DAT_143aa8518 == 0) {
      return;
    }
    iVar3 = FUN_142cb8470();
    if (iVar3 == *(int *)(param_1 + 0x294)) {
      return;
    }
    FUN_141607590(param_1 + 0x408);
    FUN_141607590(param_1 + 0x428);
LAB_1415f238a:
    FUN_142889070(lVar1);
  }
  else {
    FUN_141607590(param_1 + 0x408);
    FUN_141607590(param_1 + 0x428);
    if (lVar1 != 0) goto LAB_1415f238a;
  }
  uVar12 = CONCAT44(uVar7,0x80000000);
  uVar8 = CONCAT44(uVar4,1);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soVSync_143a45658,0x15,uVar8,uVar12,0x7fffffff);
  *(undefined4 *)(param_1 + 0x4c) = uVar4;
  uVar7 = 0;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar8 = CONCAT44((int)((ulonglong)uVar8 >> 0x20),0x50);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_goMySkillAlpha_143a45688,0x1b,uVar8,uVar12,0x50);
  *(undefined4 *)(param_1 + 0x50) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = CONCAT44((int)((ulonglong)uVar8 >> 0x20),100);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_goOtherSkillAlpha_143a45690,0x1c,uVar9,uVar12,100);
  *(undefined4 *)(param_1 + 0x54) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soDamageEffect_143a45650,0x14,
                        uVar9 & 0xffffffff00000000,uVar12,5);
  *(undefined4 *)(param_1 + 0x68) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soTremble_143a456b8,0x1e,1,uVar12,1);
  *(undefined4 *)(param_1 + 0x6c) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soPetBackword_143a456c8,0x20,1,uVar12,1);
  *(undefined4 *)(param_1 + 0x70) = uVar4;
  uVar6 = 0;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_UseWindowedHotkey_143a456f8,0x2a,1,
                        uVar12 & 0xffffffff00000000,1);
  *(undefined4 *)(param_1 + 0x48) = uVar4;
  piVar13 = local_res8;
  puVar10 = local_res20;
  local_res8[0] = 4;
  iVar3 = (*DAT_143ad5a60)(*(undefined8 *)(param_1 + 8),
                           PTR_s_soNotShowRemoteLargeScaleEffect_143a450f8,0,local_res18,puVar10,
                           piVar13);
  uVar5 = uVar6;
  if (((iVar3 == 0) && (local_res18[0] == 4)) && (local_res8[0] == 4)) {
    uVar5 = (uint)(local_res20[0] != 0);
  }
  uVar9 = CONCAT44((int)((ulonglong)piVar13 >> 0x20),0x32);
  uVar8 = CONCAT44((int)((ulonglong)puVar10 >> 0x20),100);
  *(uint *)(param_1 + 0x80) = uVar5;
  uVar4 = FUN_1415f40c0(param_1,param_2,0,0x3c,uVar8,uVar9,100);
  uVar9 = uVar9 & 0xffffffff00000000;
  uVar8 = CONCAT44((int)((ulonglong)uVar8 >> 0x20),100);
  *(undefined4 *)(param_1 + 0x58) = uVar4;
  uVar4 = FUN_1415f40c0(param_1,param_2,0,0x3d,uVar8,uVar9,100);
  *(undefined4 *)(param_1 + 0x5c) = uVar4;
  uVar12 = CONCAT44((int)(uVar9 >> 0x20),0x32);
  uVar8 = CONCAT44((int)((ulonglong)uVar8 >> 0x20),*(undefined4 *)(param_1 + 0x58));
  uVar4 = FUN_1415f40c0(param_1,param_2,0,0x3e,uVar8,uVar12,100);
  *(undefined4 *)(param_1 + 0x60) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar8 = CONCAT44((int)((ulonglong)uVar8 >> 0x20),*(undefined4 *)(param_1 + 0x5c));
  uVar4 = FUN_1415f40c0(param_1,param_2,0,0x3f,uVar8,uVar12,100);
  uVar11 = (undefined4)((ulonglong)uVar8 >> 0x20);
  *(undefined4 *)(param_1 + 100) = uVar4;
  FUN_1415f48a0(param_1,param_2);
  FUN_1415f4f00(param_1,param_2);
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = CONCAT44(uVar11,3);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soMobInfo_143a45640,0x12,uVar9,uVar12,3);
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = uVar9 & 0xffffffff00000000;
  *(undefined4 *)(param_1 + 0x104) = uVar4;
  uVar4 = FUN_1415f40c0(param_1,param_2,0,0x40,uVar9,uVar12,2);
  *(undefined4 *)(param_1 + 0x108) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = uVar9 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soCombatMessage_143a456d0,0x22,uVar9,uVar12,1);
  *(undefined4 *)(param_1 + 0x114) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar8 = CONCAT44((int)(uVar9 >> 0x20),1);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soAvatarMegaphone_143a456d8,0x23,uVar8,uVar12,1);
  *(undefined4 *)(param_1 + 0x118) = uVar4;
  uVar12 = CONCAT44((int)(uVar12 >> 0x20),0x80000000);
  uVar9 = CONCAT44((int)((ulonglong)uVar8 >> 0x20),1);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_magUI_143a45698,0x1d,uVar9,uVar12,0x7fffffff);
  *(undefined4 *)(param_1 + 0x124) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = uVar9 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_uiAlpha_143a456c0,0x1f,uVar9,uVar12,1);
  *(undefined4 *)(param_1 + 0x128) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = CONCAT44((int)(uVar9 >> 0x20),2);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soQuickSlotNum_143a45648,0x13,uVar9,uVar12,2);
  *(undefined4 *)(param_1 + 300) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = uVar9 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soQuickSlotEffect_143a456f0,0x27,uVar9,uVar12,1);
  *(undefined4 *)(param_1 + 0x130) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = uVar9 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soScreenShot_143a45618,0xd,uVar9,uVar12,2);
  *(undefined4 *)(param_1 + 0x184) = uVar4;
  uVar12 = CONCAT44((int)(uVar12 >> 0x20),1);
  uVar8 = CONCAT44((int)(uVar9 >> 0x20),1);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soShotFormat_143a45620,0xe,uVar8,uVar12,2);
  *(undefined4 *)(param_1 + 0x188) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = CONCAT44((int)((ulonglong)uVar8 >> 0x20),1);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soShotAuto_143a45628,0xf,uVar9,uVar12,1);
  *(undefined4 *)(param_1 + 0x18c) = uVar4;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soShotConti_143a45630,0x10,uVar9 & 0xffffffff00000000,
                        uVar12 & 0xffffffff00000000,1);
  *(undefined4 *)(param_1 + 400) = uVar4;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soShotContiCount_143a45638,0x11,5,2,10);
  *(undefined4 *)(param_1 + 0x194) = uVar4;
  local_res8[0] = 4;
  iVar3 = (*DAT_143ad5a60)(*(undefined8 *)(param_1 + 8),PTR_s_MemoryMappedIO_143a44fc0,0,local_res18
                           ,local_res20,local_res8);
  if (((iVar3 == 0) && (local_res18[0] == 4)) && (local_res8[0] == 4)) {
    uVar6 = (uint)(local_res20[0] != 0);
  }
  *(uint *)(param_1 + 0x198) = uVar6;
  local_38[0] = 0;
  uVar5 = 0x4000;
  iVar3 = (*DAT_1432623b8)(local_38);
  if (iVar3 != 0) {
    uVar5 = (uint)(local_38[0] >> 10);
  }
  piVar13 = local_res8;
  puVar10 = local_res20;
  local_res8[0] = 4;
  iVar3 = (*DAT_143ad5a60)(*(undefined8 *)(param_1 + 8),PTR_s_64bitFlushMemorySize_143a44fc8,0,
                           local_res18,puVar10,piVar13);
  if (((iVar3 != 0) || (local_res18[0] != 4)) ||
     ((local_res8[0] != 4 ||
      (((int)local_res20[0] < 0x800 || (uVar6 = local_res20[0], (int)uVar5 < (int)local_res20[0]))))
     )) {
    uVar6 = uVar5 >> 1;
  }
  *(uint *)(param_1 + 0x19c) = uVar6;
  uVar12 = (ulonglong)piVar13 & 0xffffffff00000000;
  uVar9 = (ulonglong)puVar10 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soPopupUIChatWnd_143a456e0,0x24,uVar9,uVar12,1);
  *(undefined4 *)(param_1 + 0x168) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = uVar9 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soShowChatTimeStamp_143a456e8,0x25,uVar9,uVar12,1);
  *(undefined4 *)(param_1 + 0x16c) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = uVar9 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_fontSizeType_143a45660,0x16,uVar9,uVar12,4);
  *(undefined4 *)(param_1 + 0x170) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = CONCAT44((int)(uVar9 >> 0x20),3);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_fontColorWhisper_143a45668,0x17,uVar9,uVar12,7);
  *(undefined4 *)(param_1 + 0x174) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_fontColorFriend_143a45670,0x18,
                        uVar9 & 0xffffffff00000000,uVar12,7);
  *(undefined4 *)(param_1 + 0x178) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_fontColorGuild_143a45678,0x19,5,uVar12,7);
  *(undefined4 *)(param_1 + 0x17c) = uVar4;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_fontColorAlliance_143a45680,0x1a,7,
                        uVar12 & 0xffffffff00000000,7);
  *(undefined4 *)(param_1 + 0x180) = uVar4;
  local_res8[0] = 4;
  iVar3 = (*DAT_143ad5a60)(*(undefined8 *)(param_1 + 8),PTR_s_LSMSuggestionCount_143a44fa0,0,
                           local_res18,local_res20,local_res8);
  if ((((iVar3 == 0) && (local_res18[0] == 4)) && (local_res8[0] == 4)) && (local_res20[0] < 6)) {
    *(uint *)(param_1 + 0x270) = local_res20[0];
    if (4 < (int)local_res20[0]) goto LAB_1415f2b01;
  }
  else {
    *(undefined4 *)(param_1 + 0x270) = 0;
  }
  cVar2 = FUN_1410a3fe0(800,600);
  if ((cVar2 != '\0') || (cVar2 = FUN_1410a3fe0(0x400,0x300), cVar2 != '\0')) {
    uVar7 = 1;
  }
  *(undefined4 *)(param_1 + 0x274) = uVar7;
LAB_1415f2b01:
  FUN_141601d10(param_1 + 0x428);
  return;
}



//===========================================================
// FUN_1416011d0 @ 1416011d0   (212 bytes)
//===========================================================

void FUN_1416011d0(undefined8 param_1,undefined8 *param_2)

{
  char cVar1;
  longlong *plVar2;
  longlong *plVar3;
  longlong *plVar4;
  undefined8 local_res10;
  
  if ((ulonglong)param_2[1] < 0xff) {
    FUN_1406ed840(param_1,param_2[1] & 0xff);
    plVar4 = *(longlong **)*param_2;
    cVar1 = *(char *)((longlong)plVar4 + 0x19);
    while (cVar1 == '\0') {
      local_res10 = *(undefined8 *)((longlong)plVar4 + 0x1c);
      FUN_1406ede20(param_1,&local_res10,4);
      FUN_1406ed9d0(param_1,local_res10._4_4_);
      plVar2 = (longlong *)plVar4[2];
      if (*(char *)((longlong)plVar2 + 0x19) == '\0') {
        cVar1 = *(char *)(*plVar2 + 0x19);
        plVar4 = plVar2;
        plVar2 = (longlong *)*plVar2;
        while (cVar1 == '\0') {
          cVar1 = *(char *)(*plVar2 + 0x19);
          plVar4 = plVar2;
          plVar2 = (longlong *)*plVar2;
        }
      }
      else {
        cVar1 = *(char *)(plVar4[1] + 0x19);
        plVar3 = (longlong *)plVar4[1];
        plVar2 = plVar4;
        while ((plVar4 = plVar3, cVar1 == '\0' && (plVar2 == (longlong *)plVar4[2]))) {
          cVar1 = *(char *)(plVar4[1] + 0x19);
          plVar3 = (longlong *)plVar4[1];
          plVar2 = plVar4;
        }
      }
      cVar1 = *(char *)((longlong)plVar4 + 0x19);
    }
  }
  return;
}



//===========================================================
// FUN_141601d10 @ 141601d10   (172 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141601d10(longlong param_1)

{
  undefined1 auStack_4a8 [32];
  int local_488 [2];
  undefined8 local_480;
  undefined8 uStack_478;
  undefined8 local_470;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  if (*(longlong *)(param_1 + 8) != 0) {
    FUN_1406ed520(local_468,0x2eb);
    local_488[0] = FUN_1411f4f50(param_1);
    if ((local_488[0] == 0) || (local_488[0] == 1)) {
      FUN_1406ede20(local_468,local_488,4);
      local_480 = 0;
      uStack_478 = 0;
      local_470 = 0;
      FUN_1416012b0(local_468,param_1);
      FUN_1415d01c0(local_468);
    }
    FUN_1406ed610(local_468);
  }
  return;
}



//===========================================================
// FUN_141601dc0 @ 141601dc0   (1106 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141601dc0(longlong *param_1,int param_2)

{
  ulonglong uVar1;
  char cVar2;
  int iVar3;
  undefined4 uVar4;
  bool bVar5;
  longlong *plVar6;
  int *piVar7;
  longlong *plVar8;
  undefined8 *puVar9;
  longlong *plVar10;
  undefined8 *puVar11;
  longlong *plVar12;
  undefined8 *puVar13;
  undefined8 *puVar14;
  undefined8 *puVar15;
  int *piVar16;
  int *piVar17;
  undefined1 auStack_518 [32];
  longlong *local_4f8;
  ulonglong local_4f0;
  int local_4e8 [2];
  undefined8 local_4e0;
  undefined8 local_4d8;
  longlong *local_4c8;
  uint uStack_4c0;
  int *local_4a0;
  int *local_498;
  int *local_490;
  undefined1 local_488 [1104];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_518;
  if (param_1[1] != 0) {
    FUN_1406ed520(local_488,0x2eb);
    local_4e8[0] = FUN_1411f4f50(param_1);
    if ((local_4e8[0] == 0) || (local_4e8[0] == 1)) {
      FUN_1406ede20(local_488,local_4e8,4);
      piVar7 = (int *)FUN_140197eb0(4);
      piVar17 = piVar7 + 1;
      *piVar7 = param_2;
      local_4f8 = (longlong *)0x0;
      local_4f0 = 0;
      local_4a0 = piVar7;
      local_498 = piVar17;
      local_490 = piVar17;
      plVar8 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x28);
      *plVar8 = (longlong)plVar8;
      plVar8[1] = (longlong)plVar8;
      plVar8[2] = (longlong)plVar8;
      *(undefined2 *)(plVar8 + 3) = 0x101;
      local_4f8 = plVar8;
      for (piVar16 = piVar7; piVar16 != piVar17; piVar16 = piVar16 + 1) {
        puVar11 = (undefined8 *)*param_1;
        puVar15 = (undefined8 *)puVar11[1];
        cVar2 = *(char *)((longlong)puVar15 + 0x19);
        puVar9 = puVar11;
        if (cVar2 == '\0') {
          puVar13 = puVar15;
          do {
            if (*(int *)((longlong)puVar13 + 0x1c) < *piVar16) {
              puVar14 = (undefined8 *)puVar13[2];
            }
            else {
              puVar14 = (undefined8 *)*puVar13;
              puVar9 = puVar13;
            }
            puVar13 = puVar14;
          } while (*(char *)((longlong)puVar14 + 0x19) == '\0');
        }
        if (((*(char *)((longlong)puVar9 + 0x19) == '\0') &&
            (iVar3 = *piVar16, *(int *)((longlong)puVar9 + 0x1c) <= iVar3)) && (puVar9 != puVar11))
        {
          while (cVar2 == '\0') {
            if (*(int *)((longlong)puVar15 + 0x1c) < iVar3) {
              puVar9 = (undefined8 *)puVar15[2];
              puVar15 = puVar11;
            }
            else {
              puVar9 = (undefined8 *)*puVar15;
            }
            puVar11 = puVar15;
            puVar15 = puVar9;
            cVar2 = *(char *)((longlong)puVar9 + 0x19);
          }
          if ((*(char *)((longlong)puVar11 + 0x19) != '\0') ||
             (iVar3 < *(int *)((longlong)puVar11 + 0x1c))) {
                    /* WARNING: Subroutine does not return */
            FUN_142ed308c("invalid map<K, T> key");
          }
          uVar4 = *(undefined4 *)(puVar11 + 4);
          plVar10 = (longlong *)plVar8[1];
          uStack_4c0 = 0;
          cVar2 = *(char *)((longlong)plVar10 + 0x19);
          local_4c8 = plVar10;
          plVar12 = plVar8;
          while (plVar6 = plVar10, cVar2 == '\0') {
            bVar5 = iVar3 <= *(int *)((longlong)plVar6 + 0x1c);
            if (bVar5) {
              plVar10 = (longlong *)*plVar6;
              plVar12 = plVar6;
            }
            else {
              plVar10 = (longlong *)plVar6[2];
            }
            uStack_4c0 = (uint)bVar5;
            cVar2 = *(char *)((longlong)plVar10 + 0x19);
            local_4c8 = plVar6;
          }
          if ((*(char *)((longlong)plVar12 + 0x19) != '\0') ||
             (iVar3 < *(int *)((longlong)plVar12 + 0x1c))) {
            if (local_4f0 == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
              FUN_14019f9d0();
            }
            local_4e0 = &local_4f8;
            local_4d8 = 0;
            puVar11 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x28);
            *(int *)((longlong)puVar11 + 0x1c) = *piVar16;
            *(undefined4 *)(puVar11 + 4) = 0;
            *puVar11 = plVar8;
            puVar11[1] = plVar8;
            puVar11[2] = plVar8;
            *(undefined2 *)(puVar11 + 3) = 0;
            local_4d8 = 0;
            plVar12 = (longlong *)FUN_1416067d0(&local_4f8,&local_4c8,puVar11);
          }
          *(undefined4 *)(plVar12 + 4) = uVar4;
          plVar8 = local_4f8;
        }
      }
      if (local_4f0 == 0) {
        cVar2 = *(char *)(plVar8[1] + 0x19);
        plVar10 = (longlong *)plVar8[1];
        while (cVar2 == '\0') {
          FUN_141602670(&local_4f8,&local_4f8,plVar10[2]);
          plVar8 = (longlong *)*plVar10;
          thunk_FUN_140205820(plVar10,0x28);
          cVar2 = *(char *)((longlong)plVar8 + 0x19);
          plVar10 = plVar8;
          plVar8 = local_4f8;
        }
        thunk_FUN_140205820(plVar8,0x28);
      }
      else {
        if (local_4f0 < 0xff) {
          FUN_1406ed840(local_488,local_4f0 & 0xff);
          plVar10 = (longlong *)*local_4f8;
          cVar2 = *(char *)((longlong)plVar10 + 0x19);
          plVar8 = local_4f8;
          while (local_4f8 = plVar8, cVar2 == '\0') {
            local_4e0 = *(longlong ***)((longlong)plVar10 + 0x1c);
            FUN_1406ede20(local_488,&local_4e0,4);
            FUN_1406ed9d0(local_488,local_4e0._4_4_);
            plVar8 = (longlong *)plVar10[2];
            if (*(char *)((longlong)plVar8 + 0x19) == '\0') {
              cVar2 = *(char *)(*plVar8 + 0x19);
              plVar10 = plVar8;
              plVar8 = (longlong *)*plVar8;
              while (cVar2 == '\0') {
                cVar2 = *(char *)(*plVar8 + 0x19);
                plVar10 = plVar8;
                plVar8 = (longlong *)*plVar8;
              }
            }
            else {
              cVar2 = *(char *)(plVar10[1] + 0x19);
              plVar12 = (longlong *)plVar10[1];
              plVar8 = plVar10;
              while ((plVar10 = plVar12, cVar2 == '\0' && (plVar8 == (longlong *)plVar10[2]))) {
                cVar2 = *(char *)(plVar10[1] + 0x19);
                plVar12 = (longlong *)plVar10[1];
                plVar8 = plVar10;
              }
            }
            plVar8 = local_4f8;
            cVar2 = *(char *)((longlong)plVar10 + 0x19);
          }
        }
        cVar2 = *(char *)(plVar8[1] + 0x19);
        plVar10 = (longlong *)plVar8[1];
        while (cVar2 == '\0') {
          FUN_141602670(&local_4f8,&local_4f8,plVar10[2]);
          plVar8 = (longlong *)*plVar10;
          thunk_FUN_140205820(plVar10,0x28);
          cVar2 = *(char *)((longlong)plVar8 + 0x19);
          plVar10 = plVar8;
          plVar8 = local_4f8;
        }
        thunk_FUN_140205820(plVar8,0x28);
        FUN_1415d01c0(local_488);
      }
      if (piVar7 != (int *)0x0) {
        uVar1 = ((longlong)piVar17 - (longlong)piVar7 >> 2) * 4;
        piVar17 = piVar7;
        if (0xfff < uVar1) {
          piVar17 = *(int **)(piVar7 + -2);
          if (0x1f < (ulonglong)((longlong)piVar7 + (-8 - (longlong)piVar17))) {
                    /* WARNING: Subroutine does not return */
            FUN_142f04804(piVar17,uVar1 + 0x27);
          }
        }
        thunk_FUN_140205820(piVar17);
      }
    }
    FUN_1406ed610(local_488);
  }
  return;
}



//===========================================================
// FUN_141604660 @ 141604660   (1196 bytes)
//===========================================================

uint FUN_141604660(longlong *param_1,undefined8 param_2,uint param_3,undefined4 param_4,int param_5,
                  uint param_6,int param_7,int param_8)

{
  char cVar1;
  longlong lVar2;
  bool bVar3;
  bool bVar4;
  uint uVar5;
  char cVar6;
  int iVar7;
  int iVar8;
  undefined8 *puVar9;
  longlong *plVar10;
  undefined8 *puVar11;
  undefined4 *puVar12;
  undefined8 *puVar13;
  uint *puVar14;
  undefined8 *puVar15;
  undefined8 *puVar16;
  uint uVar17;
  undefined8 uVar18;
  longlong *plVar19;
  int iVar20;
  undefined8 local_res8;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined4 local_58;
  longlong local_50;
  char local_48;
  undefined8 *local_40;
  uint uStack_38;
  undefined4 uStack_34;
  char local_30;
  
  uVar5 = param_6;
  bVar4 = true;
  local_48 = '\0';
  plVar19 = (longlong *)(*param_1 + 0x418);
  puVar11 = (undefined8 *)*plVar19;
  puVar9 = (undefined8 *)puVar11[1];
  cVar1 = *(char *)((longlong)puVar9 + 0x19);
  puVar16 = puVar11;
  puVar13 = puVar9;
  cVar6 = cVar1;
  while (cVar6 == '\0') {
    if (*(int *)((longlong)puVar13 + 0x1c) < (int)param_3) {
      puVar15 = (undefined8 *)puVar13[2];
      puVar13 = puVar16;
    }
    else {
      puVar15 = (undefined8 *)*puVar13;
    }
    puVar16 = puVar13;
    puVar13 = puVar15;
    cVar6 = *(char *)((longlong)puVar15 + 0x19);
  }
  puVar13 = puVar11;
  uVar17 = uVar5;
  local_res8 = param_1;
  if (((*(char *)((longlong)puVar16 + 0x19) == '\0') &&
      (*(int *)((longlong)puVar16 + 0x1c) <= (int)param_3)) &&
     (puVar13 = puVar16, puVar16 != puVar11)) {
    uStack_38 = 0;
    local_40 = puVar9;
    while (puVar16 = puVar9, cVar1 == '\0') {
      bVar3 = (int)param_3 <= *(int *)((longlong)puVar16 + 0x1c);
      if (bVar3) {
        puVar9 = (undefined8 *)*puVar16;
        puVar11 = puVar16;
      }
      else {
        puVar9 = (undefined8 *)puVar16[2];
      }
      uStack_38 = (uint)bVar3;
      cVar1 = *(char *)((longlong)puVar9 + 0x19);
      local_40 = puVar16;
    }
    if ((*(char *)((longlong)puVar11 + 0x19) != '\0') ||
       ((int)param_3 < *(int *)((longlong)puVar11 + 0x1c))) {
      if (*(longlong *)(*param_1 + 0x420) == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
        FUN_14019f9d0();
      }
      lVar2 = *plVar19;
      uStack_60 = 0;
      local_68 = plVar19;
      plVar10 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x28);
      *(uint *)((longlong)plVar10 + 0x1c) = param_3;
      *(undefined4 *)(plVar10 + 4) = 0;
      *plVar10 = lVar2;
      plVar10[1] = lVar2;
      plVar10[2] = lVar2;
      *(undefined2 *)(plVar10 + 3) = 0;
      local_68 = local_40;
      uStack_60 = CONCAT44(uStack_34,uStack_38);
      puVar11 = (undefined8 *)FUN_141606550(plVar19,&local_68,plVar10);
    }
    plVar19 = local_res8;
    uVar17 = *(uint *)(puVar11 + 4);
    iVar7 = param_7;
    iVar20 = param_8;
  }
  else {
    plVar19 = param_1;
    iVar7 = param_7;
    iVar20 = param_8;
    if (puVar13 == puVar11) {
      puVar12 = (undefined4 *)FUN_140426ed0(&local_40,param_1[1],param_3);
      uVar18 = 0;
      if (*(char *)(puVar12 + 4) == '\0') {
        if (local_48 != '\0') {
          if (local_50 != 0) {
            FUN_14019f2c0(local_50 + -0x10);
          }
          local_48 = '\0';
        }
      }
      else {
        local_58 = *puVar12;
        if (local_48 == '\0') {
          local_50 = 0;
          FUN_14019a260(&local_50,puVar12 + 2);
          local_48 = '\x01';
        }
        else {
          FUN_14019a260(&local_50,puVar12 + 2);
        }
      }
      if ((local_30 != '\0') && (CONCAT44(uStack_34,uStack_38) != 0)) {
        FUN_14019f2c0(CONCAT44(uStack_34,uStack_38) + -0x10);
      }
      plVar19 = local_res8;
      if (local_48 == '\0') {
        puVar12 = (undefined4 *)FUN_140426d60(&local_40,local_res8[1],param_4);
        if (*(char *)(puVar12 + 4) == '\0') {
          if (local_48 != '\0') {
            if (local_50 != 0) {
              FUN_14019f2c0(local_50 + -0x10);
            }
            local_48 = '\0';
          }
        }
        else {
          local_58 = *puVar12;
          if (local_48 == '\0') {
            local_50 = 0;
            FUN_14019a260(&local_50,puVar12 + 2);
            local_48 = '\x01';
          }
          else {
            FUN_14019a260(&local_50,puVar12 + 2);
          }
        }
        if ((local_30 != '\0') && (CONCAT44(uStack_34,uStack_38) != 0)) {
          FUN_14019f2c0(CONCAT44(uStack_34,uStack_38) + -0x10);
        }
        if (local_48 == '\0') {
          lVar2 = *plVar19;
          if (param_5 == 0) {
            uVar18 = *(undefined8 *)(lVar2 + 8);
          }
          else if (param_5 == 1) {
            uVar18 = *(undefined8 *)(lVar2 + 0x10);
          }
          else if (param_5 == 2) {
            uVar18 = *(undefined8 *)(lVar2 + 0x18);
          }
          else if (param_5 == 3) {
            uVar18 = *(undefined8 *)(lVar2 + 0x20);
          }
          param_6 = 4;
          iVar8 = (*DAT_143ad5a60)(uVar18,param_2,0,&local_res8,&local_68,&param_6);
          iVar20 = param_8;
          iVar7 = param_7;
          if ((((iVar8 == 0) && ((int)local_res8 == 4)) && (param_6 == 4)) &&
             ((param_7 <= (int)(uint)local_68 && ((int)(uint)local_68 <= param_8)))) {
            uVar17 = (uint)local_68;
          }
          lVar2 = *plVar19;
          param_6 = param_3;
          iVar8 = FUN_140426d40(1,param_3 & 0xff);
          if (iVar8 != 0) {
            puVar11 = *(undefined8 **)(lVar2 + 0x418);
            cVar6 = *(char *)((longlong)puVar11[1] + 0x19);
            puVar9 = puVar11;
            puVar16 = (undefined8 *)puVar11[1];
            while (cVar6 == '\0') {
              if (*(int *)((longlong)puVar16 + 0x1c) < (int)param_6) {
                puVar13 = (undefined8 *)puVar16[2];
                puVar16 = puVar9;
              }
              else {
                puVar13 = (undefined8 *)*puVar16;
              }
              puVar9 = puVar16;
              puVar16 = puVar13;
              cVar6 = *(char *)((longlong)puVar13 + 0x19);
            }
            if (((*(char *)((longlong)puVar9 + 0x19) != '\0') ||
                ((int)param_6 < *(int *)((longlong)puVar9 + 0x1c))) ||
               ((puVar9 == puVar11 || (*(uint *)(puVar9 + 4) != uVar17)))) {
              puVar14 = (uint *)FUN_141603a40(lVar2 + 0x418,&param_6);
              *puVar14 = uVar17;
              puVar14 = (uint *)FUN_141603a40(lVar2 + 0x438,&param_6);
              *puVar14 = uVar17;
            }
          }
        }
        else {
          FUN_1408ae6a0(&local_res8,&local_50);
          iVar7 = param_7;
          iVar20 = param_8;
          if ((char)local_res8 != '\0') {
            uVar17 = local_res8._4_4_;
          }
        }
      }
      else {
        FUN_1408ae6a0(&local_68,&local_50);
        if ((char)local_68 != '\0') {
          uVar17 = local_68._4_4_;
        }
        bVar4 = false;
        plVar19 = local_res8;
        iVar7 = param_7;
        iVar20 = param_8;
      }
    }
  }
  cVar6 = FUN_1408fcfc0(iVar7,iVar20,uVar17);
  if (cVar6 == '\0') {
    uVar17 = uVar5;
  }
  lVar2 = *plVar19;
  param_6 = param_3;
  iVar7 = FUN_140426d40(1,param_3 & 0xff);
  if (iVar7 != 0) {
    puVar11 = *(undefined8 **)(lVar2 + 0x418);
    cVar6 = *(char *)((longlong)puVar11[1] + 0x19);
    puVar9 = puVar11;
    puVar16 = (undefined8 *)puVar11[1];
    while (cVar6 == '\0') {
      if (*(int *)((longlong)puVar16 + 0x1c) < (int)param_6) {
        puVar13 = (undefined8 *)puVar16[2];
        puVar16 = puVar9;
      }
      else {
        puVar13 = (undefined8 *)*puVar16;
      }
      puVar9 = puVar16;
      puVar16 = puVar13;
      cVar6 = *(char *)((longlong)puVar13 + 0x19);
    }
    if ((((*(char *)((longlong)puVar9 + 0x19) != '\0') ||
         ((int)param_6 < *(int *)((longlong)puVar9 + 0x1c))) || (puVar9 == puVar11)) ||
       (*(uint *)(puVar9 + 4) != uVar17)) {
      puVar14 = (uint *)FUN_141603a40(lVar2 + 0x418,&param_6);
      *puVar14 = uVar17;
      if (bVar4) {
        puVar14 = (uint *)FUN_141603a40(lVar2 + 0x438,&param_6);
        *puVar14 = uVar17;
      }
    }
  }
  if ((local_48 != '\0') && (local_50 != 0)) {
    FUN_14019f2c0(local_50 + -0x10);
  }
  return uVar17;
}



//===========================================================
// FUN_141604450 @ 141604450   (515 bytes)
//===========================================================

undefined4
FUN_141604450(longlong *param_1,undefined8 param_2,int param_3,undefined4 param_4,int param_5)

{
  char cVar1;
  char cVar2;
  longlong lVar3;
  bool bVar4;
  longlong *plVar5;
  longlong *plVar6;
  longlong *plVar7;
  longlong *plVar8;
  undefined4 uVar9;
  longlong *plVar10;
  longlong *plVar11;
  char local_res8 [4];
  undefined4 local_resc;
  longlong *local_68;
  undefined8 uStack_60;
  undefined1 local_58 [8];
  longlong local_50;
  char local_48;
  longlong *local_40;
  uint uStack_38;
  undefined4 uStack_34;
  
  uVar9 = 1;
  plVar11 = (longlong *)(*param_1 + 0x418);
  plVar7 = (longlong *)*plVar11;
  plVar6 = (longlong *)plVar7[1];
  cVar1 = *(char *)((longlong)plVar6 + 0x19);
  plVar5 = plVar7;
  plVar10 = plVar6;
  cVar2 = cVar1;
  while (cVar2 == '\0') {
    if (*(int *)((longlong)plVar10 + 0x1c) < param_3) {
      plVar8 = (longlong *)plVar10[2];
      plVar10 = plVar5;
    }
    else {
      plVar8 = (longlong *)*plVar10;
    }
    plVar5 = plVar10;
    plVar10 = plVar8;
    cVar2 = *(char *)((longlong)plVar8 + 0x19);
  }
  if (((*(char *)((longlong)plVar5 + 0x19) == '\0') &&
      (*(int *)((longlong)plVar5 + 0x1c) <= param_3)) && (plVar5 != plVar7)) {
    uStack_38 = 0;
    local_40 = plVar6;
    while (plVar5 = plVar6, cVar1 == '\0') {
      bVar4 = param_3 <= *(int *)((longlong)plVar5 + 0x1c);
      if (bVar4) {
        plVar6 = (longlong *)*plVar5;
        plVar7 = plVar5;
      }
      else {
        plVar6 = (longlong *)plVar5[2];
      }
      uStack_38 = (uint)bVar4;
      cVar1 = *(char *)((longlong)plVar6 + 0x19);
      local_40 = plVar5;
    }
    if ((*(char *)((longlong)plVar7 + 0x19) != '\0') ||
       (param_3 < *(int *)((longlong)plVar7 + 0x1c))) {
      if (*(longlong *)(*param_1 + 0x420) == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
        FUN_14019f9d0();
      }
      lVar3 = *plVar11;
      uStack_60 = 0;
      local_68 = plVar11;
      plVar7 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x28);
      *(int *)((longlong)plVar7 + 0x1c) = param_3;
      *(undefined4 *)(plVar7 + 4) = 0;
      *plVar7 = lVar3;
      plVar7[1] = lVar3;
      plVar7[2] = lVar3;
      *(undefined2 *)(plVar7 + 3) = 0;
      local_68 = local_40;
      uStack_60 = CONCAT44(uStack_34,uStack_38);
      plVar7 = (longlong *)FUN_141606550(plVar11,&local_68,plVar7);
    }
    param_4 = (undefined4)plVar7[4];
  }
  else {
    FUN_140426ed0(local_58,param_1[1],param_3);
    if (local_48 == '\0') {
      param_4 = FUN_1415ef210(*param_1,(param_5 != 0) + '\x02',param_2,param_4);
      if ((local_48 != '\0') && (local_50 != 0)) {
        FUN_14019f2c0(local_50 + -0x10);
      }
    }
    else {
      FUN_1408ae6a0(local_res8,&local_50);
      if (local_res8[0] != '\0') {
        param_4 = local_resc;
      }
      uVar9 = 0;
      if ((local_48 != '\0') && (local_50 != 0)) {
        FUN_14019f2c0(local_50 + -0x10);
      }
    }
  }
  FUN_1415f65b0(*param_1,param_3,param_4,uVar9);
  return param_4;
}



//===========================================================
// FUN_141604b10 @ 141604b10   (493 bytes)
//===========================================================

undefined4
FUN_141604b10(longlong *param_1,int param_2,undefined4 param_3,undefined4 param_4,undefined4 param_5
             )

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  longlong lVar4;
  undefined8 uVar5;
  undefined4 *puVar6;
  undefined8 *puVar7;
  undefined8 *puVar8;
  undefined8 uVar9;
  char *local_res8;
  int local_res10 [2];
  longlong local_58;
  longlong local_50;
  char local_48 [4];
  undefined4 local_44;
  undefined1 local_40 [8];
  longlong local_38;
  char local_30;
  
  uVar9 = 1;
  local_58 = 0;
  local_res10[0] = param_2;
  uVar5 = FUN_14019ba10(&local_58,&DAT_143274298,param_4);
  local_50 = 0;
  FUN_14019a260(&local_50,uVar5);
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  lVar4 = local_50;
  puVar2 = *(undefined8 **)(*param_1 + 0x418);
  cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
  puVar3 = puVar2;
  puVar8 = (undefined8 *)puVar2[1];
  while (cVar1 == '\0') {
    if (*(int *)((longlong)puVar8 + 0x1c) < local_res10[0]) {
      puVar7 = (undefined8 *)puVar8[2];
      puVar8 = puVar3;
    }
    else {
      puVar7 = (undefined8 *)*puVar8;
    }
    puVar3 = puVar8;
    puVar8 = puVar7;
    cVar1 = *(char *)((longlong)puVar7 + 0x19);
  }
  puVar8 = puVar2;
  if (((*(char *)((longlong)puVar3 + 0x19) == '\0') &&
      (*(int *)((longlong)puVar3 + 0x1c) <= local_res10[0])) && (puVar8 = puVar3, puVar3 != puVar2))
  {
    puVar6 = (undefined4 *)FUN_141603a40(*param_1 + 0x418,local_res10);
    param_5 = *puVar6;
  }
  else if (puVar8 == puVar2) {
    FUN_140426ed0(local_40,param_1[2],local_res10[0]);
    if (local_30 == '\0') {
      FUN_1402e01c0(param_1[1],&local_res8,param_3,lVar4);
      if ((local_res8 == (char *)0x0) || (*local_res8 == '\0')) {
        if (local_res8 != (char *)0x0) {
          FUN_14019f2c0(local_res8 + -0x10);
        }
        if ((local_30 != '\0') && (local_38 != 0)) {
          FUN_14019f2c0(local_38 + -0x10);
        }
      }
      else {
        FUN_1408ae6a0(local_48,&local_res8);
        if (local_48[0] != '\0') {
          param_5 = local_44;
        }
        if (local_res8 != (char *)0x0) {
          FUN_14019f2c0(local_res8 + -0x10);
        }
        if ((local_30 != '\0') && (local_38 != 0)) {
          FUN_14019f2c0(local_38 + -0x10);
        }
      }
    }
    else {
      FUN_1408ae6a0(local_48,&local_38);
      if (local_48[0] != '\0') {
        param_5 = local_44;
      }
      uVar9 = 0;
      if ((local_30 != '\0') && (local_38 != 0)) {
        FUN_14019f2c0(local_38 + -0x10);
      }
    }
  }
  FUN_1415f65b0(*param_1,local_res10[0],param_5,uVar9);
  if (lVar4 != 0) {
    FUN_14019f2c0(lVar4 + -0x10);
  }
  return param_5;
}



//===========================================================
// FUN_140426d40 @ 140426d40   (25 bytes)
//===========================================================

bool FUN_140426d40(int param_1,byte param_2)

{
  bool bVar1;
  
  bVar1 = false;
  if (param_1 == 0) {
    bVar1 = param_2 < 0x41;
  }
  else if (param_1 == 1) {
    return param_2 < 0x15;
  }
  return bVar1;
}



//===========================================================
// FUN_1415f22f0 @ 1415f22f0   (2094 bytes)
//===========================================================

void FUN_1415f22f0(longlong param_1,int param_2)

{
  longlong lVar1;
  char cVar2;
  int iVar3;
  undefined4 uVar4;
  uint uVar5;
  uint uVar6;
  undefined4 uVar7;
  int local_res8 [2];
  int local_res18 [2];
  uint local_res20 [2];
  undefined8 in_stack_ffffffffffffffa8;
  undefined8 uVar8;
  ulonglong uVar9;
  uint *puVar10;
  undefined4 uVar11;
  undefined8 in_stack_ffffffffffffffb0;
  ulonglong uVar12;
  int *piVar13;
  ulonglong local_38 [2];
  
  lVar1 = DAT_143aa8518;
  uVar4 = (undefined4)((ulonglong)in_stack_ffffffffffffffa8 >> 0x20);
  uVar7 = (undefined4)((ulonglong)in_stack_ffffffffffffffb0 >> 0x20);
  if (*(longlong *)(param_1 + 8) == 0) {
    return;
  }
  if (DAT_143aa84a0 == 0) {
    return;
  }
  if (param_2 == 0) {
    if (DAT_143aa8518 == 0) {
      return;
    }
    iVar3 = FUN_142cb8470();
    if (iVar3 == *(int *)(param_1 + 0x294)) {
      return;
    }
    FUN_141607590(param_1 + 0x408);
    FUN_141607590(param_1 + 0x428);
LAB_1415f238a:
    FUN_142889070(lVar1);
  }
  else {
    FUN_141607590(param_1 + 0x408);
    FUN_141607590(param_1 + 0x428);
    if (lVar1 != 0) goto LAB_1415f238a;
  }
  uVar12 = CONCAT44(uVar7,0x80000000);
  uVar8 = CONCAT44(uVar4,1);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soVSync_143a45658,0x15,uVar8,uVar12,0x7fffffff);
  *(undefined4 *)(param_1 + 0x4c) = uVar4;
  uVar7 = 0;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar8 = CONCAT44((int)((ulonglong)uVar8 >> 0x20),0x50);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_goMySkillAlpha_143a45688,0x1b,uVar8,uVar12,0x50);
  *(undefined4 *)(param_1 + 0x50) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = CONCAT44((int)((ulonglong)uVar8 >> 0x20),100);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_goOtherSkillAlpha_143a45690,0x1c,uVar9,uVar12,100);
  *(undefined4 *)(param_1 + 0x54) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soDamageEffect_143a45650,0x14,
                        uVar9 & 0xffffffff00000000,uVar12,5);
  *(undefined4 *)(param_1 + 0x68) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soTremble_143a456b8,0x1e,1,uVar12,1);
  *(undefined4 *)(param_1 + 0x6c) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soPetBackword_143a456c8,0x20,1,uVar12,1);
  *(undefined4 *)(param_1 + 0x70) = uVar4;
  uVar6 = 0;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_UseWindowedHotkey_143a456f8,0x2a,1,
                        uVar12 & 0xffffffff00000000,1);
  *(undefined4 *)(param_1 + 0x48) = uVar4;
  piVar13 = local_res8;
  puVar10 = local_res20;
  local_res8[0] = 4;
  iVar3 = (*DAT_143ad5a60)(*(undefined8 *)(param_1 + 8),
                           PTR_s_soNotShowRemoteLargeScaleEffect_143a450f8,0,local_res18,puVar10,
                           piVar13);
  uVar5 = uVar6;
  if (((iVar3 == 0) && (local_res18[0] == 4)) && (local_res8[0] == 4)) {
    uVar5 = (uint)(local_res20[0] != 0);
  }
  uVar9 = CONCAT44((int)((ulonglong)piVar13 >> 0x20),0x32);
  uVar8 = CONCAT44((int)((ulonglong)puVar10 >> 0x20),100);
  *(uint *)(param_1 + 0x80) = uVar5;
  uVar4 = FUN_1415f40c0(param_1,param_2,0,0x3c,uVar8,uVar9,100);
  uVar9 = uVar9 & 0xffffffff00000000;
  uVar8 = CONCAT44((int)((ulonglong)uVar8 >> 0x20),100);
  *(undefined4 *)(param_1 + 0x58) = uVar4;
  uVar4 = FUN_1415f40c0(param_1,param_2,0,0x3d,uVar8,uVar9,100);
  *(undefined4 *)(param_1 + 0x5c) = uVar4;
  uVar12 = CONCAT44((int)(uVar9 >> 0x20),0x32);
  uVar8 = CONCAT44((int)((ulonglong)uVar8 >> 0x20),*(undefined4 *)(param_1 + 0x58));
  uVar4 = FUN_1415f40c0(param_1,param_2,0,0x3e,uVar8,uVar12,100);
  *(undefined4 *)(param_1 + 0x60) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar8 = CONCAT44((int)((ulonglong)uVar8 >> 0x20),*(undefined4 *)(param_1 + 0x5c));
  uVar4 = FUN_1415f40c0(param_1,param_2,0,0x3f,uVar8,uVar12,100);
  uVar11 = (undefined4)((ulonglong)uVar8 >> 0x20);
  *(undefined4 *)(param_1 + 100) = uVar4;
  FUN_1415f48a0(param_1,param_2);
  FUN_1415f4f00(param_1,param_2);
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = CONCAT44(uVar11,3);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soMobInfo_143a45640,0x12,uVar9,uVar12,3);
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = uVar9 & 0xffffffff00000000;
  *(undefined4 *)(param_1 + 0x104) = uVar4;
  uVar4 = FUN_1415f40c0(param_1,param_2,0,0x40,uVar9,uVar12,2);
  *(undefined4 *)(param_1 + 0x108) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = uVar9 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soCombatMessage_143a456d0,0x22,uVar9,uVar12,1);
  *(undefined4 *)(param_1 + 0x114) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar8 = CONCAT44((int)(uVar9 >> 0x20),1);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soAvatarMegaphone_143a456d8,0x23,uVar8,uVar12,1);
  *(undefined4 *)(param_1 + 0x118) = uVar4;
  uVar12 = CONCAT44((int)(uVar12 >> 0x20),0x80000000);
  uVar9 = CONCAT44((int)((ulonglong)uVar8 >> 0x20),1);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_magUI_143a45698,0x1d,uVar9,uVar12,0x7fffffff);
  *(undefined4 *)(param_1 + 0x124) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = uVar9 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_uiAlpha_143a456c0,0x1f,uVar9,uVar12,1);
  *(undefined4 *)(param_1 + 0x128) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = CONCAT44((int)(uVar9 >> 0x20),2);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soQuickSlotNum_143a45648,0x13,uVar9,uVar12,2);
  *(undefined4 *)(param_1 + 300) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = uVar9 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soQuickSlotEffect_143a456f0,0x27,uVar9,uVar12,1);
  *(undefined4 *)(param_1 + 0x130) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = uVar9 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soScreenShot_143a45618,0xd,uVar9,uVar12,2);
  *(undefined4 *)(param_1 + 0x184) = uVar4;
  uVar12 = CONCAT44((int)(uVar12 >> 0x20),1);
  uVar8 = CONCAT44((int)(uVar9 >> 0x20),1);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soShotFormat_143a45620,0xe,uVar8,uVar12,2);
  *(undefined4 *)(param_1 + 0x188) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = CONCAT44((int)((ulonglong)uVar8 >> 0x20),1);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soShotAuto_143a45628,0xf,uVar9,uVar12,1);
  *(undefined4 *)(param_1 + 0x18c) = uVar4;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soShotConti_143a45630,0x10,uVar9 & 0xffffffff00000000,
                        uVar12 & 0xffffffff00000000,1);
  *(undefined4 *)(param_1 + 400) = uVar4;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soShotContiCount_143a45638,0x11,5,2,10);
  *(undefined4 *)(param_1 + 0x194) = uVar4;
  local_res8[0] = 4;
  iVar3 = (*DAT_143ad5a60)(*(undefined8 *)(param_1 + 8),PTR_s_MemoryMappedIO_143a44fc0,0,local_res18
                           ,local_res20,local_res8);
  if (((iVar3 == 0) && (local_res18[0] == 4)) && (local_res8[0] == 4)) {
    uVar6 = (uint)(local_res20[0] != 0);
  }
  *(uint *)(param_1 + 0x198) = uVar6;
  local_38[0] = 0;
  uVar5 = 0x4000;
  iVar3 = (*DAT_1432623b8)(local_38);
  if (iVar3 != 0) {
    uVar5 = (uint)(local_38[0] >> 10);
  }
  piVar13 = local_res8;
  puVar10 = local_res20;
  local_res8[0] = 4;
  iVar3 = (*DAT_143ad5a60)(*(undefined8 *)(param_1 + 8),PTR_s_64bitFlushMemorySize_143a44fc8,0,
                           local_res18,puVar10,piVar13);
  if (((iVar3 != 0) || (local_res18[0] != 4)) ||
     ((local_res8[0] != 4 ||
      (((int)local_res20[0] < 0x800 || (uVar6 = local_res20[0], (int)uVar5 < (int)local_res20[0]))))
     )) {
    uVar6 = uVar5 >> 1;
  }
  *(uint *)(param_1 + 0x19c) = uVar6;
  uVar12 = (ulonglong)piVar13 & 0xffffffff00000000;
  uVar9 = (ulonglong)puVar10 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soPopupUIChatWnd_143a456e0,0x24,uVar9,uVar12,1);
  *(undefined4 *)(param_1 + 0x168) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = uVar9 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_soShowChatTimeStamp_143a456e8,0x25,uVar9,uVar12,1);
  *(undefined4 *)(param_1 + 0x16c) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = uVar9 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_fontSizeType_143a45660,0x16,uVar9,uVar12,4);
  *(undefined4 *)(param_1 + 0x170) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar9 = CONCAT44((int)(uVar9 >> 0x20),3);
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_fontColorWhisper_143a45668,0x17,uVar9,uVar12,7);
  *(undefined4 *)(param_1 + 0x174) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_fontColorFriend_143a45670,0x18,
                        uVar9 & 0xffffffff00000000,uVar12,7);
  *(undefined4 *)(param_1 + 0x178) = uVar4;
  uVar12 = uVar12 & 0xffffffff00000000;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_fontColorGuild_143a45678,0x19,5,uVar12,7);
  *(undefined4 *)(param_1 + 0x17c) = uVar4;
  uVar4 = FUN_1415f40c0(param_1,param_2,PTR_s_fontColorAlliance_143a45680,0x1a,7,
                        uVar12 & 0xffffffff00000000,7);
  *(undefined4 *)(param_1 + 0x180) = uVar4;
  local_res8[0] = 4;
  iVar3 = (*DAT_143ad5a60)(*(undefined8 *)(param_1 + 8),PTR_s_LSMSuggestionCount_143a44fa0,0,
                           local_res18,local_res20,local_res8);
  if ((((iVar3 == 0) && (local_res18[0] == 4)) && (local_res8[0] == 4)) && (local_res20[0] < 6)) {
    *(uint *)(param_1 + 0x270) = local_res20[0];
    if (4 < (int)local_res20[0]) goto LAB_1415f2b01;
  }
  else {
    *(undefined4 *)(param_1 + 0x270) = 0;
  }
  cVar2 = FUN_1410a3fe0(800,600);
  if ((cVar2 != '\0') || (cVar2 = FUN_1410a3fe0(0x400,0x300), cVar2 != '\0')) {
    uVar7 = 1;
  }
  *(undefined4 *)(param_1 + 0x274) = uVar7;
LAB_1415f2b01:
  FUN_141601d10(param_1 + 0x428);
  return;
}



//===========================================================
// FUN_1415f0b40 @ 1415f0b40   (72 bytes)
//===========================================================

void FUN_1415f0b40(longlong param_1)

{
  undefined4 uVar1;
  
  FUN_1415f22f0(param_1,1);
  FUN_1415f7140(param_1);
  uVar1 = FUN_1410a3dc0(1);
  *(undefined4 *)(param_1 + 0x44) = uVar1;
  *(undefined4 *)(param_1 + 0x3c) = 0x556;
  *(undefined4 *)(param_1 + 0x40) = 0x300;
  FUN_1415f4380(param_1,1);
  return;
}



//===========================================================
// FUN_140426ed0 @ 140426ed0   (361 bytes)
//===========================================================

undefined4 * FUN_140426ed0(undefined4 *param_1,longlong *param_2,undefined4 param_3)

{
  int *piVar1;
  undefined4 uVar2;
  char *pcVar3;
  undefined8 uVar4;
  char *pcVar5;
  undefined **ppuVar6;
  char *pcVar7;
  char *local_res10 [2];
  char *local_res20;
  char *local_38 [2];
  
  (**(code **)(*param_2 + 8))(param_2);
  uVar4 = (**(code **)(*param_2 + 0x18))(param_2);
  FUN_140427040(local_38,param_3);
  pcVar5 = local_38[0];
  if (((local_38[0] == (char *)0x0) || (*local_38[0] == '\0')) || (8 < *(int *)(local_38[0] + -8)))
  {
    *(undefined1 *)(param_1 + 4) = 0;
    if (local_38[0] != (char *)0x0) {
      FUN_14019f2c0(local_38[0] + -0x10);
    }
  }
  else {
    ppuVar6 = (undefined **)&DAT_1432874e8;
    do {
      uVar2 = *(undefined4 *)ppuVar6;
      FUN_1402e01c0(uVar4,local_res10,uVar2,pcVar5);
      pcVar3 = local_res10[0];
      if ((local_res10[0] != (char *)0x0) && (*local_res10[0] != '\0')) {
        piVar1 = (int *)(local_res10[0] + -0x10);
        pcVar7 = (char *)0x0;
        if (piVar1 != (int *)0x0) {
          if (*piVar1 == -1) {
            FUN_142e52d50(0xcb,0xffffff01);
            local_res20 = (char *)0x0;
            FUN_1401d66b0(&local_res20,local_res10[0],0xffffffff);
            pcVar7 = local_res20;
          }
          else {
            if (*piVar1 < 1) {
              FUN_142e52dd0(0xd2);
            }
            LOCK();
            *piVar1 = *piVar1 + 1;
            UNLOCK();
            pcVar5 = local_38[0];
            pcVar7 = pcVar3;
          }
        }
        *param_1 = uVar2;
        *(char **)(param_1 + 2) = pcVar7;
        *(undefined1 *)(param_1 + 4) = 1;
        if (local_res10[0] != (char *)0x0) {
          FUN_14019f2c0(local_res10[0] + -0x10);
        }
        FUN_14019f2c0(pcVar5 + -0x10);
        return param_1;
      }
      if (local_res10[0] != (char *)0x0) {
        FUN_14019f2c0(local_res10[0] + -0x10);
      }
      ppuVar6 = (undefined **)((longlong)ppuVar6 + 4);
    } while (ppuVar6 != &PTR_DAT_1432874f0);
    *(undefined1 *)(param_1 + 4) = 0;
    FUN_14019f2c0(pcVar5 + -0x10);
  }
  return param_1;
}



//===========================================================
// FUN_140426d60 @ 140426d60   (361 bytes)
//===========================================================

undefined4 * FUN_140426d60(undefined4 *param_1,longlong *param_2,undefined4 param_3)

{
  int *piVar1;
  undefined4 uVar2;
  char *pcVar3;
  undefined8 uVar4;
  char *pcVar5;
  undefined4 *puVar6;
  char *pcVar7;
  char *local_res10 [2];
  char *local_res20;
  char *local_38 [2];
  
  (**(code **)(*param_2 + 8))(param_2);
  uVar4 = (**(code **)(*param_2 + 0x18))(param_2);
  FUN_140427230(local_38,param_3);
  pcVar5 = local_38[0];
  if (((local_38[0] == (char *)0x0) || (*local_38[0] == '\0')) || (8 < *(int *)(local_38[0] + -8)))
  {
    *(undefined1 *)(param_1 + 4) = 0;
    if (local_38[0] != (char *)0x0) {
      FUN_14019f2c0(local_38[0] + -0x10);
    }
  }
  else {
    puVar6 = &DAT_1432874d8;
    do {
      uVar2 = *puVar6;
      FUN_1402e0430(uVar4,local_res10,uVar2,pcVar5);
      pcVar3 = local_res10[0];
      if ((local_res10[0] != (char *)0x0) && (*local_res10[0] != '\0')) {
        piVar1 = (int *)(local_res10[0] + -0x10);
        pcVar7 = (char *)0x0;
        if (piVar1 != (int *)0x0) {
          if (*piVar1 == -1) {
            FUN_142e52d50(0xcb,0xffffff01);
            local_res20 = (char *)0x0;
            FUN_1401d66b0(&local_res20,local_res10[0],0xffffffff);
            pcVar7 = local_res20;
          }
          else {
            if (*piVar1 < 1) {
              FUN_142e52dd0(0xd2);
            }
            LOCK();
            *piVar1 = *piVar1 + 1;
            UNLOCK();
            pcVar5 = local_38[0];
            pcVar7 = pcVar3;
          }
        }
        *param_1 = uVar2;
        *(char **)(param_1 + 2) = pcVar7;
        *(undefined1 *)(param_1 + 4) = 1;
        if (local_res10[0] != (char *)0x0) {
          FUN_14019f2c0(local_res10[0] + -0x10);
        }
        FUN_14019f2c0(pcVar5 + -0x10);
        return param_1;
      }
      if (local_res10[0] != (char *)0x0) {
        FUN_14019f2c0(local_res10[0] + -0x10);
      }
      puVar6 = puVar6 + 1;
    } while (puVar6 != &DAT_1432874e8);
    *(undefined1 *)(param_1 + 4) = 0;
    FUN_14019f2c0(pcVar5 + -0x10);
  }
  return param_1;
}



//===========================================================
// FUN_140427040 @ 140427040   (487 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Type propagation algorithm not settling */

longlong ****** FUN_140427040(longlong ******param_1,uint param_2)

{
  longlong *****ppppplVar1;
  uint uVar2;
  longlong lVar3;
  undefined4 *puVar4;
  longlong lVar5;
  longlong *******ppppppplVar6;
  undefined1 auStack_68 [32];
  undefined *local_48;
  ulonglong uStack_40;
  longlong *******local_38;
  longlong lStack_30;
  undefined8 local_28;
  ulonglong local_20;
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_68;
  lVar5 = 0;
  uVar2 = 0;
  local_38 = (longlong *******)param_1;
  if (param_2 < 0x16) {
    lVar3 = (longlong)(int)param_2;
    local_48 = (&PTR_DAT_143287b10)[lVar3 * 2];
    uStack_40 = *(ulonglong *)(&UNK_143287b18 + lVar3 * 0x10);
    if (uStack_40 != 0) {
      local_38 = (longlong *******)(&PTR_DAT_143287b10)[lVar3 * 2];
      lStack_30 = *(longlong *)(&UNK_143287b18 + lVar3 * 0x10);
      if (lStack_30 == 5) {
        do {
          lVar3 = lVar5 + 1;
          if (*(byte *)((longlong)local_38 + lVar5) != "COUNT"[lVar5]) {
            uVar2 = -(uint)(*(byte *)((longlong)local_38 + lVar5) < (byte)"COUNT"[lVar5]) | 1;
            break;
          }
          lVar5 = lVar3;
        } while (lVar3 != 5);
        if (uVar2 == 0) goto LAB_140427181;
      }
      if (uStack_40 < 9) {
        local_28 = 0;
        local_20 = 0xf;
        local_38 = (longlong *******)0x0;
        FUN_1401d69c0(&local_38,local_48);
        ppppppplVar6 = (longlong *******)&local_38;
        if (0xf < local_20) {
          ppppppplVar6 = local_38;
        }
        *param_1 = (longlong *****)0x0;
        FUN_1401d66b0(param_1,ppppppplVar6,0xffffffff);
        if (local_20 < 0x10) {
          return param_1;
        }
        if (0xfff < local_20 + 1) {
          if (0x1f < (ulonglong)((longlong)local_38 + (-8 - (longlong)local_38[-1]))) {
                    /* WARNING: Subroutine does not return */
            FUN_142f04804(local_38[-1],local_20 + 0x28);
          }
        }
        thunk_FUN_140205820();
        return param_1;
      }
    }
  }
LAB_140427181:
  *param_1 = (longlong *****)0x0;
  puVar4 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x11);
  puVar4[1] = 0;
  *puVar4 = 0xffffffff;
  *param_1 = (longlong *****)(puVar4 + 4);
  puVar4[2] = 0;
  *(undefined1 *)*param_1 = 0;
  ppppplVar1 = *param_1;
  if (*(int *)(ppppplVar1 + -2) != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (*(int *)((longlong)ppppplVar1 + -0xc) < 0) {
    FUN_142e54290(0x90,*(int *)((longlong)ppppplVar1 + -0xc),0);
  }
  *(undefined4 *)(ppppplVar1 + -2) = 1;
  *(undefined1 *)*param_1 = 0;
  if (*(int *)((longlong)ppppplVar1 + -0xc) + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  *(undefined4 *)(ppppplVar1 + -1) = 0;
  return param_1;
}



//===========================================================
// FUN_140427230 @ 140427230   (487 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Type propagation algorithm not settling */

longlong ****** FUN_140427230(longlong ******param_1,uint param_2)

{
  longlong *****ppppplVar1;
  uint uVar2;
  longlong lVar3;
  undefined4 *puVar4;
  longlong lVar5;
  longlong *******ppppppplVar6;
  undefined1 auStack_68 [32];
  undefined *local_48;
  ulonglong uStack_40;
  longlong *******local_38;
  longlong lStack_30;
  undefined8 local_28;
  ulonglong local_20;
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_68;
  lVar5 = 0;
  uVar2 = 0;
  local_38 = (longlong *******)param_1;
  if (param_2 < 0x42) {
    lVar3 = (longlong)(int)param_2;
    local_48 = (&PTR_DAT_1432874f0)[lVar3 * 2];
    uStack_40 = *(ulonglong *)(&UNK_1432874f8 + lVar3 * 0x10);
    if (uStack_40 != 0) {
      local_38 = (longlong *******)(&PTR_DAT_1432874f0)[lVar3 * 2];
      lStack_30 = *(longlong *)(&UNK_1432874f8 + lVar3 * 0x10);
      if (lStack_30 == 5) {
        do {
          lVar3 = lVar5 + 1;
          if (*(byte *)((longlong)local_38 + lVar5) != "COUNT"[lVar5]) {
            uVar2 = -(uint)(*(byte *)((longlong)local_38 + lVar5) < (byte)"COUNT"[lVar5]) | 1;
            break;
          }
          lVar5 = lVar3;
        } while (lVar3 != 5);
        if (uVar2 == 0) goto LAB_140427371;
      }
      if (uStack_40 < 9) {
        local_28 = 0;
        local_20 = 0xf;
        local_38 = (longlong *******)0x0;
        FUN_1401d69c0(&local_38,local_48);
        ppppppplVar6 = (longlong *******)&local_38;
        if (0xf < local_20) {
          ppppppplVar6 = local_38;
        }
        *param_1 = (longlong *****)0x0;
        FUN_1401d66b0(param_1,ppppppplVar6,0xffffffff);
        if (local_20 < 0x10) {
          return param_1;
        }
        if (0xfff < local_20 + 1) {
          if (0x1f < (ulonglong)((longlong)local_38 + (-8 - (longlong)local_38[-1]))) {
                    /* WARNING: Subroutine does not return */
            FUN_142f04804(local_38[-1],local_20 + 0x28);
          }
        }
        thunk_FUN_140205820();
        return param_1;
      }
    }
  }
LAB_140427371:
  *param_1 = (longlong *****)0x0;
  puVar4 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x11);
  puVar4[1] = 0;
  *puVar4 = 0xffffffff;
  *param_1 = (longlong *****)(puVar4 + 4);
  puVar4[2] = 0;
  *(undefined1 *)*param_1 = 0;
  ppppplVar1 = *param_1;
  if (*(int *)(ppppplVar1 + -2) != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (*(int *)((longlong)ppppplVar1 + -0xc) < 0) {
    FUN_142e54290(0x90,*(int *)((longlong)ppppplVar1 + -0xc),0);
  }
  *(undefined4 *)(ppppplVar1 + -2) = 1;
  *(undefined1 *)*param_1 = 0;
  if (*(int *)((longlong)ppppplVar1 + -0xc) + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  *(undefined4 *)(ppppplVar1 + -1) = 0;
  return param_1;
}



//===========================================================
// FUN_1402e01c0 @ 1402e01c0   (114 bytes)
//===========================================================

undefined8 * FUN_1402e01c0(longlong param_1,undefined8 *param_2,int param_3,undefined8 param_4)

{
  longlong lVar1;
  
  if (*(longlong *)(param_1 + 0x12bb) != 0) {
    for (lVar1 = *(longlong *)
                  (*(longlong *)(param_1 + 0x12bb) +
                  ((ulonglong)(longlong)param_3 % (ulonglong)*(uint *)(param_1 + 0x12c3)) * 8);
        lVar1 != 0; lVar1 = *(longlong *)(lVar1 + 8)) {
      if (*(int *)(lVar1 + 0x10) == param_3) {
        if ((lVar1 != -0x18) && (*(longlong *)(lVar1 + 0x20) != 0)) {
          FUN_140193890(*(longlong *)(lVar1 + 0x20),param_2,param_4);
          return param_2;
        }
        break;
      }
    }
  }
  *param_2 = 0;
  return param_2;
}



//===========================================================
// FUN_1402e0430 @ 1402e0430   (114 bytes)
//===========================================================

undefined8 * FUN_1402e0430(longlong param_1,undefined8 *param_2,int param_3,undefined8 param_4)

{
  longlong lVar1;
  
  if (*(longlong *)(param_1 + 0x12d3) != 0) {
    for (lVar1 = *(longlong *)
                  (*(longlong *)(param_1 + 0x12d3) +
                  ((ulonglong)(longlong)param_3 % (ulonglong)*(uint *)(param_1 + 0x12db)) * 8);
        lVar1 != 0; lVar1 = *(longlong *)(lVar1 + 8)) {
      if (*(int *)(lVar1 + 0x10) == param_3) {
        if ((lVar1 != -0x18) && (*(longlong *)(lVar1 + 0x20) != 0)) {
          FUN_140193890(*(longlong *)(lVar1 + 0x20),param_2,param_4);
          return param_2;
        }
        break;
      }
    }
  }
  *param_2 = 0;
  return param_2;
}


