
//===========================================================
// FUN_14019b4e0 @ 14019b4e0   (288 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14019b4e0(undefined8 *param_1)

{
  code *pcVar1;
  void *pvVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  int *piVar6;
  longlong lVar7;
  bool bVar8;
  
  pvVar2 = Self;
  pcVar1 = DAT_143ad5530;
  if (param_1 == (undefined8 *)0x0) {
    return;
  }
  uVar3 = param_1[-1];
  if ((longlong)uVar3 < 0) {
    uVar3 = ~uVar3;
  }
  if (uVar3 < 0x21) {
    uVar5 = (ulonglong)(0x10 < uVar3);
  }
  else {
    if (uVar3 < 0x41) {
      uVar5 = 2;
      goto LAB_14019b52b;
    }
    uVar5 = 0xffffffff;
    if (uVar3 < 0x81) {
      uVar5 = 3;
    }
  }
  if ((int)uVar5 < 0) {
    uVar4 = (*DAT_143ad5538)();
    (*pcVar1)(uVar4,0,param_1 + -1);
    return;
  }
LAB_14019b52b:
  lVar7 = uVar5 * 0x10;
  LOCK();
  bVar8 = *(longlong *)(&DAT_143ad68c8 + lVar7) == 0;
  if (bVar8) {
    *(void **)(&DAT_143ad68c8 + lVar7) = Self;
  }
  UNLOCK();
  if (bVar8) {
LAB_14019b5b5:
    *(undefined4 *)(&DAT_143ad68d0 + lVar7) = 1;
  }
  else {
    if (*(void **)(&DAT_143ad68c8 + lVar7) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar8 = *(longlong *)(&DAT_143ad68c8 + lVar7) == 0;
        if (bVar8) {
          *(void **)(&DAT_143ad68c8 + lVar7) = Self;
        }
        UNLOCK();
        if (bVar8) goto LAB_14019b5b5;
        if (*(void **)(&DAT_143ad68c8 + lVar7) == pvVar2) break;
        (*DAT_143262828)(0);
      }
    }
    *(int *)(&DAT_143ad68d0 + lVar7) = *(int *)(&DAT_143ad68d0 + lVar7) + 1;
  }
  piVar6 = (int *)(&DAT_143ad68d0 + lVar7);
  *param_1 = *(undefined8 *)(&DAT_143ad6908 + uVar5 * 8);
  *(undefined8 **)(&DAT_143ad6908 + uVar5 * 8) = param_1;
  _DAT_143ad6948 = *param_1;
  *(int *)(&DAT_143ad68b4 + uVar5 * 4) = *(int *)(&DAT_143ad68b4 + uVar5 * 4) + -1;
  *piVar6 = *piVar6 + -1;
  if (*piVar6 == 0) {
    *(undefined8 *)(&DAT_143ad68c8 + lVar7) = 0;
  }
  return;
}



//===========================================================
// FUN_1428336c0 @ 1428336c0   (1376 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1428336c0(longlong param_1)

{
  int *piVar1;
  longlong *plVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  code *pcVar5;
  void *pvVar6;
  undefined1 *puVar7;
  int iVar8;
  uint uVar9;
  longlong lVar10;
  int *piVar11;
  longlong lVar12;
  longlong *plVar13;
  ulonglong uVar14;
  undefined8 *puVar15;
  undefined8 uVar16;
  longlong *plVar17;
  int *piVar18;
  longlong lVar19;
  undefined8 *puVar20;
  uint uVar21;
  undefined8 *puVar22;
  longlong lVar23;
  longlong lVar24;
  ulonglong uVar25;
  bool bVar26;
  int local_res8 [2];
  undefined1 *local_res10;
  longlong *local_res18;
  undefined1 local_78 [8];
  int *local_70;
  undefined8 *local_68;
  ulonglong local_60;
  undefined4 local_58;
  undefined4 local_54;
  
  FUN_1429ba1f0(DAT_143ac1b90,*(undefined4 *)(param_1 + 0x10d0));
  local_res10 = (undefined1 *)(param_1 + 0x3bb0);
  FUN_14287f0c0();
  FUN_1408f6690();
  local_68 = (undefined8 *)0x0;
  local_60 = 0x1f;
  local_58 = 100;
  local_54 = 0x18;
  lVar24 = 1;
  piVar18 = (int *)(param_1 + 0x16d);
  do {
    if ((((0 < *piVar18) && (lVar10 = FUN_140388c60(DAT_143aa8328), lVar10 != 0)) &&
        (-1 < *(int *)(lVar10 + 0x2a0))) &&
       (piVar11 = (int *)FUN_1403de360(DAT_143aa8328), piVar11 != (int *)0x0)) {
      local_res8[0] = piVar11[1] + *piVar11;
      lVar10 = FUN_142876550(&local_68,local_res8);
      piVar1 = (int *)(lVar10 + 0x18);
      uVar21 = 0;
      iVar8 = FUN_1401c21f0();
      if (iVar8 != 0) {
        lVar19 = 0;
LAB_1428337b5:
        lVar12 = *(longlong *)(piVar11 + 8);
        if (lVar12 == 0) {
          uVar9 = 0;
        }
        else {
          uVar9 = *(uint *)(lVar12 + -8);
        }
        if (((int)uVar21 < 0) || (uVar9 <= uVar21)) {
          FUN_142e54290(0xc6,uVar21);
          lVar12 = *(longlong *)(piVar11 + 8);
        }
        FUN_140257e00(piVar1);
        uVar9 = 0;
        if (0 < *piVar1) {
          lVar23 = 0;
          do {
            if (((int)uVar9 < 0) || (99 < uVar9)) break;
            plVar17 = (longlong *)(lVar10 + 0x1c + lVar23);
            if ((int)*plVar17 == *(int *)(lVar12 + lVar19)) goto LAB_142833856;
            uVar9 = uVar9 + 1;
            lVar23 = lVar23 + 4;
          } while ((int)uVar9 < *piVar1);
        }
        if (*(longlong **)(lVar10 + 0x1b0) != (longlong *)0x0) {
          plVar2 = (longlong *)**(longlong **)(lVar10 + 0x1b0);
          plVar13 = (longlong *)*plVar2;
          if (plVar13 != plVar2) {
            do {
              plVar17 = plVar13 + 2;
              if ((int)*plVar17 == *(int *)(lVar12 + lVar19)) goto LAB_142833856;
              plVar13 = (longlong *)*plVar13;
            } while (plVar13 != plVar2);
          }
        }
        goto LAB_14283385b;
      }
    }
LAB_1428338cb:
    lVar24 = lVar24 + 1;
    piVar18 = piVar18 + 1;
  } while (lVar24 < 0x20);
  goto LAB_1428338df;
LAB_142833856:
  if (plVar17 == (longlong *)0x0) {
LAB_14283385b:
    lVar12 = *(longlong *)(piVar11 + 8);
    if (lVar12 == 0) {
      uVar9 = 0;
    }
    else {
      uVar9 = *(uint *)(lVar12 + -8);
    }
    if (((int)uVar21 < 0) || (uVar9 <= uVar21)) {
      FUN_142e54290(0xc6,uVar21);
      lVar12 = *(longlong *)(piVar11 + 8);
    }
    FUN_140257c50(piVar1,lVar12 + (longlong)(int)uVar21 * 4);
  }
  FUN_140257e00(piVar1);
  pvVar6 = Self;
  lVar12 = DAT_143adbcb0;
  if (*piVar1 == piVar11[3]) {
    local_70 = (int *)0x0;
    plVar17 = (longlong *)(DAT_143adbcb0 + 0x18);
    LOCK();
    lVar24 = *plVar17;
    if (lVar24 == 0) {
      *plVar17 = (longlong)Self;
    }
    UNLOCK();
    local_res18 = plVar17;
    if (lVar24 == 0) goto LAB_1428339b9;
    if ((void *)*plVar17 != pvVar6) goto LAB_142833990;
    *(int *)(lVar12 + 0x20) = *(int *)(lVar12 + 0x20) + 1;
    goto LAB_1428339c3;
  }
  uVar21 = uVar21 + 1;
  lVar19 = lVar19 + 4;
  uVar9 = FUN_1401c21f0();
  if (uVar9 <= uVar21) goto LAB_1428338cb;
  goto LAB_1428337b5;
LAB_142833990:
  pvVar6 = Self;
  LOCK();
  lVar24 = *plVar17;
  if (lVar24 == 0) {
    *plVar17 = (longlong)Self;
  }
  UNLOCK();
  if (lVar24 == 0) goto LAB_1428339b9;
  if ((void *)*plVar17 == pvVar6) goto LAB_1428339b1;
  (*DAT_143262828)(0);
  goto LAB_142833990;
LAB_1428339b9:
  *(undefined4 *)(lVar12 + 0x20) = 1;
  goto LAB_1428339c3;
LAB_1428339b1:
  *(int *)(lVar12 + 0x20) = *(int *)(lVar12 + 0x20) + 1;
LAB_1428339c3:
  piVar18 = (int *)(lVar12 + 0x20);
  puVar15 = *(undefined8 **)(lVar12 + 0x28);
  if (puVar15 == (undefined8 *)0x0) {
    puVar15 = (undefined8 *)FUN_14019d3c0(0x38,0x10);
    *(undefined8 **)(lVar12 + 0x28) = puVar15;
  }
  *(undefined8 *)(lVar12 + 0x28) = *puVar15;
  *piVar18 = *piVar18 + -1;
  if (*piVar18 == 0) {
    *plVar17 = 0;
  }
  *(undefined8 *)((longlong)puVar15 + 0x2c) = 0;
  *(undefined4 *)((longlong)puVar15 + 0x34) = 0;
  puVar15[3] = 0;
  puVar15[1] = 0;
  puVar15[2] = 0;
  *puVar15 = &PTR_FUN_143482af8;
  puVar15[4] = &PTR_LAB_143482b00;
  piVar18 = (int *)(puVar15 + 5);
  piVar18[0] = -1;
  piVar18[1] = -1;
  *(undefined4 *)(puVar15 + 6) = 1;
  *(undefined4 *)((longlong)puVar15 + 0x34) = 0xffffffff;
  if (puVar15[1] != 0) {
    FUN_142e541f0(0x2fe);
  }
  puVar15[1] = 1;
  local_70 = piVar18;
  if (piVar18 == (int *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  puVar7 = local_res10;
  iVar8 = piVar11[1];
  *piVar18 = *piVar11;
  *(int *)((longlong)puVar15 + 0x2c) = iVar8;
  if ((*(longlong *)(local_res10 + 8) - 1U < 999) || (*(longlong *)(local_res10 + 8) == -1)) {
    FUN_142e52ed0(0x447);
  }
  if (puVar7 == local_78) {
    FUN_142e52d50(0x45c,1);
  }
  if (piVar18 != (int *)0x0) {
    if (0xfffff < (ulonglong)puVar15[1]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    puVar15[1] = puVar15[1] + 1;
    UNLOCK();
  }
  FUN_14287f0c0(puVar7);
  *(int **)(puVar7 + 8) = piVar18;
  if (piVar18 != (int *)0x0) {
    if (0xffffe < puVar15[1] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar17 = puVar15 + 1;
    lVar24 = *plVar17;
    *plVar17 = *plVar17 + -1;
    UNLOCK();
    if ((int)lVar24 == 1) {
      if ((local_70 != (int *)0x0) && (puVar15[3] != 0)) {
        LOCK();
        *(undefined8 *)(puVar15[3] + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(puVar15[3] + 4) != 0);
      }
      if (puVar15 != (undefined8 *)0x0) {
        (**(code **)*puVar15)(puVar15,1);
      }
    }
  }
LAB_1428338df:
  puVar15 = local_68;
  if (local_68 == (undefined8 *)0x0) {
    return;
  }
  puVar20 = local_68 + (local_60 & 0xffffffff);
  pcVar5 = DAT_143ad5530;
  pvVar6 = Self;
  puVar22 = local_68;
  while (DAT_143ad5530 = pcVar5, Self = pvVar6, puVar22 < puVar20) {
    puVar3 = (undefined8 *)*puVar22;
    puVar22 = puVar22 + 1;
    while (pcVar5 = DAT_143ad5530, pvVar6 = Self, puVar3 != (undefined8 *)0x0) {
      puVar4 = (undefined8 *)puVar3[1];
      (**(code **)*puVar3)(puVar3,1);
      puVar3 = puVar4;
    }
  }
  uVar14 = puVar15[-1];
  if ((longlong)uVar14 < 0) {
    uVar14 = ~uVar14;
  }
  if (uVar14 < 0x21) {
    uVar25 = (ulonglong)(0x10 < uVar14);
  }
  else {
    if (uVar14 < 0x41) {
      uVar25 = 2;
      goto LAB_142833b53;
    }
    uVar25 = 0xffffffff;
    if (uVar14 < 0x81) {
      uVar25 = 3;
    }
  }
  if ((int)uVar25 < 0) {
    uVar16 = (*DAT_143ad5538)();
    (*pcVar5)(uVar16,0,puVar15 + -1);
    return;
  }
LAB_142833b53:
  lVar24 = uVar25 * 0x10;
  LOCK();
  bVar26 = *(longlong *)(&DAT_143ad68c8 + lVar24) == 0;
  if (bVar26) {
    *(void **)(&DAT_143ad68c8 + lVar24) = pvVar6;
  }
  UNLOCK();
  if (bVar26) {
LAB_142833be5:
    *(undefined4 *)(&DAT_143ad68d0 + lVar24) = 1;
  }
  else {
    if (*(void **)(&DAT_143ad68c8 + lVar24) != pvVar6) {
      while( true ) {
        pvVar6 = Self;
        LOCK();
        bVar26 = *(longlong *)(&DAT_143ad68c8 + lVar24) == 0;
        if (bVar26) {
          *(void **)(&DAT_143ad68c8 + lVar24) = Self;
        }
        UNLOCK();
        if (bVar26) goto LAB_142833be5;
        if (*(void **)(&DAT_143ad68c8 + lVar24) == pvVar6) break;
        (*DAT_143262828)(0);
      }
    }
    *(int *)(&DAT_143ad68d0 + lVar24) = *(int *)(&DAT_143ad68d0 + lVar24) + 1;
  }
  piVar18 = (int *)(&DAT_143ad68d0 + lVar24);
  *local_68 = *(undefined8 *)(&DAT_143ad6908 + uVar25 * 8);
  *(undefined8 **)(&DAT_143ad6908 + uVar25 * 8) = local_68;
  _DAT_143ad6948 = *local_68;
  *(int *)(&DAT_143ad68b4 + uVar25 * 4) = *(int *)(&DAT_143ad68b4 + uVar25 * 4) + -1;
  *piVar18 = *piVar18 + -1;
  if (*piVar18 == 0) {
    *(undefined8 *)(&DAT_143ad68c8 + lVar24) = 0;
  }
  return;
}



//===========================================================
// FUN_142ce51b0 @ 142ce51b0   (540 bytes)
//===========================================================

undefined4 FUN_142ce51b0(longlong param_1,int param_2,undefined4 param_3)

{
  undefined4 **ppuVar1;
  undefined4 uVar2;
  char cVar3;
  int iVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined4 *puVar7;
  undefined4 *puVar8;
  longlong lVar9;
  undefined4 *puVar10;
  uint uVar11;
  ulonglong uVar12;
  undefined4 *puVar13;
  undefined4 uVar14;
  undefined4 *local_res20;
  
  uVar14 = 1;
  cVar3 = FUN_141b1f960(DAT_143abea80,1);
  lVar9 = DAT_143aa9d98;
  if ((((cVar3 == '\0') && (0 < param_2)) && (iVar4 = FUN_140716250(DAT_143aa9d98), iVar4 != 0)) &&
     (*(int *)(lVar9 + 0x5dc) != 0)) {
    local_res20 = (undefined4 *)0x0;
    if ((*(longlong *)(lVar9 + 0x5d0) != 0) &&
       (lVar5 = *(longlong *)
                 (*(longlong *)(lVar9 + 0x5d0) +
                 ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(lVar9 + 0x5d8)) * 8),
       lVar5 != 0)) {
LAB_142ce5250:
      if (*(int *)(lVar5 + 0x10) != param_2) goto code_r0x000142ce5255;
      ppuVar1 = (undefined4 **)(lVar5 + 0x18);
      if ((&local_res20 != ppuVar1) && (*ppuVar1 != (undefined4 *)0x0)) {
        uVar11 = (*ppuVar1)[-2];
        uVar12 = (ulonglong)uVar11;
        if (uVar11 != 0) {
          lVar5 = FUN_14019b780(&DAT_143ad68a0,uVar12 * 4 + 8);
          if (lVar5 == 0) {
            puVar8 = (undefined4 *)0x0;
          }
          else {
            puVar8 = (undefined4 *)(lVar5 + 8);
          }
          *(ulonglong *)(puVar8 + -2) = uVar12;
          puVar7 = *ppuVar1;
          puVar10 = puVar7 + uVar12;
          puVar13 = puVar8;
          if (puVar7 < puVar10) {
            do {
              uVar2 = *puVar7;
              puVar7 = puVar7 + 1;
              *puVar13 = uVar2;
              puVar13 = puVar13 + 1;
            } while (puVar7 < puVar10);
            uVar11 = puVar8[-2];
          }
          local_res20 = puVar8;
          if (uVar11 == 0) goto LAB_142ce525e;
          puVar10 = puVar8;
          for (uVar11 = 0; uVar11 < (uint)puVar8[-2]; uVar11 = uVar11 + 1) {
            if ((int)uVar11 < 0) {
              FUN_142e54290(0xbc,uVar11);
            }
            iVar4 = FUN_1407159c0(lVar9,*puVar10);
            if (iVar4 != 0) {
              if (((int)uVar11 < 0) || ((uint)puVar8[-2] <= uVar11)) {
                FUN_142e54290(0xbc,uVar11);
              }
              FUN_141d1fe90(param_1 + 0x2ea0,puVar8 + (int)uVar11);
            }
            puVar10 = puVar10 + 1;
          }
          uVar6 = FUN_1408f6690();
          uVar11 = 0;
          iVar4 = puVar8[-2];
          if (0 < (longlong)iVar4) {
            lVar9 = 0;
            do {
              if ((uint)puVar8[-2] <= uVar11) {
                FUN_142e54290(0xbc,uVar11);
              }
              FUN_142ce5f80(param_1,puVar8[lVar9],param_3,uVar6,0);
              uVar11 = uVar11 + 1;
              lVar9 = lVar9 + 1;
            } while (lVar9 < iVar4);
          }
          goto LAB_142ce5261;
        }
      }
    }
LAB_142ce525e:
    uVar14 = 0;
    puVar8 = local_res20;
LAB_142ce5261:
    if (puVar8 != (undefined4 *)0x0) {
      thunk_FUN_140205820(puVar8 + -2,0);
    }
  }
  else {
    uVar14 = 0;
  }
  return uVar14;
code_r0x000142ce5255:
  lVar5 = *(longlong *)(lVar5 + 8);
  if (lVar5 == 0) goto LAB_142ce525e;
  goto LAB_142ce5250;
}



//===========================================================
// FUN_140391940 @ 140391940   (674 bytes)
//===========================================================

longlong FUN_140391940(undefined8 param_1,longlong param_2,longlong *param_3)

{
  undefined8 *puVar1;
  longlong *plVar2;
  undefined1 auVar3 [16];
  int iVar4;
  int iVar5;
  longlong lVar6;
  longlong lVar7;
  uint uVar8;
  uint uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  undefined1 local_48 [8];
  ulonglong local_40;
  undefined1 local_38 [8];
  ulonglong local_30;
  
  if (param_3 == (longlong *)0x0) {
    *(undefined8 *)(param_2 + 8) = 0;
  }
  else {
    iVar4 = (**(code **)(*param_3 + 0xa8))(param_3);
    if (param_3[7] != 0) {
      iVar4 = 1;
    }
    uVar10 = 0;
    uVar11 = 0;
    local_40 = 0;
    iVar5 = FUN_1401b0340(param_3 + 4);
    if ((iVar5 - 1000000U < 1000000) || (uVar12 = uVar10, iVar5 - 6000000U < 1000000)) {
      FUN_1401b0340(param_3 + 4);
      lVar6 = FUN_140388c60(param_1);
      uVar12 = uVar11;
      if ((lVar6 != 0) &&
         ((((*(longlong *)(lVar6 + 0x208) != 0 &&
            (lVar6 = FUN_1403fc480(lVar6 + 0x200), *(int *)(lVar6 + 0x14) != 0)) && (0 < iVar4)) &&
          (iVar5 = FUN_14038f220(param_1), iVar4 <= iVar5)))) {
        FUN_14019a5d0(param_3 + 4);
        lVar6 = FUN_140388c60(param_1);
        if ((lVar6 != 0) && (*(longlong *)(lVar6 + 0x208) != 0)) {
          lVar7 = FUN_1403fc480(lVar6 + 0x200);
          if ((*(longlong *)(lVar7 + 8) != 0) && (*(int *)(*(longlong *)(lVar7 + 8) + -8) != 0)) {
            lVar6 = FUN_1403fc480(lVar6 + 0x200);
            uVar9 = iVar4 - 1;
            lVar6 = *(longlong *)(lVar6 + 8);
            uVar8 = 0;
            if (lVar6 != 0) {
              uVar8 = *(uint *)(lVar6 + -8);
            }
            if (((int)uVar9 < 0) || (uVar8 <= uVar9)) {
              uVar11 = uVar10;
              if (lVar6 != 0) {
                uVar11 = (ulonglong)*(uint *)(lVar6 + -8);
              }
              FUN_142e54290(0xbc,uVar9,uVar11);
            }
            FUN_1403faa60(local_48);
            uVar12 = local_40;
          }
        }
      }
    }
    local_30 = 0;
    if (uVar12 != 0) {
      iVar4 = (**(code **)(*param_3 + 0xa0))(param_3);
      if (*(longlong *)(uVar12 + 0x28) != 0) {
        auVar3._8_8_ = 0;
        auVar3._0_8_ = (longlong)iVar4;
        auVar3 = auVar3 % ZEXT416(*(uint *)(uVar12 + 0x30));
        for (lVar6 = *(longlong *)(*(longlong *)(uVar12 + 0x28) + auVar3._0_8_ * 8); uVar10 = 0,
            lVar6 != 0; lVar6 = *(longlong *)(lVar6 + 8)) {
          if (*(int *)(lVar6 + 0x10) == iVar4) {
            if (local_38 == (undefined1 *)(lVar6 + 0x18)) {
              FUN_142e52d50(0x45c,CONCAT71(auVar3._1_7_,1));
            }
            lVar7 = *(longlong *)(lVar6 + 0x20);
            if (lVar7 != 0) {
              if (0xfffff < *(ulonglong *)(lVar7 + -0x20)) {
                FUN_142e541f0(0x30f);
              }
              LOCK();
              *(longlong *)(lVar7 + -0x20) = *(longlong *)(lVar7 + -0x20) + 1;
              UNLOCK();
              uVar12 = local_40;
            }
            uVar10 = *(ulonglong *)(lVar6 + 0x20);
            local_30 = uVar10;
            break;
          }
        }
      }
    }
    *(ulonglong *)(param_2 + 8) = uVar10;
    if (uVar12 != 0) {
      puVar1 = (undefined8 *)(uVar12 - 0x28);
      if (0xffffe < *(longlong *)(uVar12 - 0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar2 = (longlong *)(uVar12 - 0x20);
      lVar6 = *plVar2;
      *plVar2 = *plVar2 + -1;
      UNLOCK();
      if ((int)lVar6 == 1) {
        if (*(longlong *)(local_40 - 0x10) != 0) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_40 - 0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_40 - 0x10) + 4) != 0);
        }
        if (puVar1 != (undefined8 *)0x0) {
          (**(code **)*puVar1)(puVar1,1);
        }
      }
    }
  }
  return param_2;
}


