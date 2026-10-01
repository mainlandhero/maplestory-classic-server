
//===========================================================
// FUN_142defd40 @ 142defd40   (9482 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x000142df0d48) */
/* WARNING: Removing unreachable block (ram,0x000142df1a7a) */
/* WARNING: Removing unreachable block (ram,0x000142df0f8d) */
/* WARNING: Type propagation algorithm not settling */

void FUN_142defd40(longlong param_1,undefined8 param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  int *piVar3;
  undefined1 uVar4;
  byte bVar5;
  char cVar6;
  byte bVar7;
  uint uVar8;
  uint uVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  undefined4 uVar13;
  undefined8 *puVar14;
  undefined4 *puVar15;
  longlong lVar16;
  undefined4 *puVar17;
  int *piVar18;
  int *piVar19;
  longlong *plVar20;
  int *piVar21;
  undefined4 *puVar22;
  longlong lVar23;
  undefined8 *puVar24;
  undefined8 uVar25;
  int *piVar26;
  uint uVar27;
  undefined8 *puVar28;
  undefined8 *puVar29;
  ulonglong uVar30;
  int iVar31;
  ulonglong uVar32;
  int **ppiVar33;
  int **ppiVar34;
  int **ppiVar35;
  int **ppiVar36;
  ulonglong uVar37;
  longlong lVar38;
  ulonglong uVar39;
  int **ppiVar40;
  int **ppiVar41;
  undefined1 auStack_4c8 [32];
  undefined8 local_4a8;
  undefined4 local_4a0;
  undefined4 local_498;
  uint local_490;
  undefined4 local_488;
  undefined4 local_480;
  int *local_478;
  int *local_470;
  undefined8 local_468;
  int *local_460;
  int **local_458;
  undefined8 local_450;
  int **local_448;
  uint local_440;
  undefined8 *local_438;
  undefined1 local_430;
  int **local_428;
  int *local_420;
  int **local_418;
  uint local_410;
  int local_40c;
  int *local_408;
  int *piStack_400;
  undefined8 local_3f8;
  int *local_3f0;
  int *local_3e8 [2];
  undefined4 local_3d8;
  undefined4 uStack_3d4;
  int *piStack_3d0;
  longlong local_3c8;
  longlong local_3c0;
  longlong local_3b8;
  int ***local_3b0;
  undefined8 local_3a8;
  undefined8 local_3a0;
  int *local_398;
  undefined8 local_390;
  undefined8 local_388;
  longlong local_380;
  longlong local_378 [5];
  int **local_350;
  undefined8 local_348;
  undefined8 uStack_340;
  int **local_328;
  int *local_320;
  undefined1 local_318 [8];
  int **local_310;
  undefined1 local_308 [8];
  int **local_300;
  undefined1 local_2f8 [8];
  longlong local_2f0;
  undefined8 local_2e8 [42];
  undefined8 local_198;
  undefined8 uStack_190;
  undefined2 local_188;
  undefined4 uStack_186;
  undefined8 uStack_182;
  undefined8 uStack_17a;
  undefined2 uStack_172;
  int aiStack_170 [68];
  undefined8 local_5f;
  undefined8 local_57;
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_4c8;
  local_3c0 = param_1;
  uVar4 = FUN_1406e8ae0(param_2);
  switch(uVar4) {
  case 0x15:
    uVar25 = FUN_142d2e9e0(param_1 + 0x23c0);
    FUN_142deb010(uVar25,param_2);
    FUN_1411bb5e0();
    FUN_142dec8f0(param_1);
    FUN_142cfe810(param_1,0);
    return;
  default:
    goto switchD_142defda5_caseD_16;
  case 0x18:
    uVar25 = FUN_142d2e9e0(param_1 + 0x23c0);
    FUN_142deb620(uVar25,param_2,1);
    FUN_142deea90(param_1);
    FUN_1411bb5e0();
    FUN_1415aafa0(0x12);
    uVar25 = FUN_142d2e9e0(param_1 + 0x23c0);
    FUN_142debd10(uVar25);
    return;
  case 0x19:
    if (*(longlong *)(param_1 + 0x23c8) == 0) {
      return;
    }
    lVar16 = FUN_142d2e9e0(param_1 + 0x23c0);
    lVar23 = *(longlong *)(lVar16 + 0x18);
    FUN_1402f2800(lVar16 + 0x18,lVar16 + 0x18,*(undefined8 *)(lVar23 + 8));
    *(longlong *)(lVar23 + 8) = lVar23;
    *(longlong *)lVar23 = lVar23;
    *(longlong *)(lVar23 + 0x10) = lVar23;
    *(undefined8 *)(lVar16 + 0x20) = 0;
    iVar10 = FUN_1406e8c20(param_2);
    local_468 = (int *)CONCAT44(local_468._4_4_,iVar10);
    if (0 < iVar10) {
      local_448 = &local_470;
      piVar18 = (int *)0x0;
      do {
        uVar13 = FUN_1406e8c20(param_2);
        FUN_1406e9050(param_2,&local_478);
        local_470 = (int *)0x0;
        FUN_14019a260(&local_470,&local_478);
        lVar23 = *(longlong *)(param_1 + 0x23c8);
        if (lVar23 == 0) {
          if (local_470 != (int *)0x0) {
            FUN_14019f2c0(local_470 + -4);
          }
        }
        else {
          ppiVar35 = (int **)(lVar23 + 0x18);
          local_408 = (int *)CONCAT44(local_408._4_4_,uVar13);
          piStack_400 = (int *)0x0;
          FUN_14019a260(&piStack_400,&local_470);
          piVar19 = *ppiVar35;
          piVar26 = *(int **)(piVar19 + 2);
          uStack_340 = (int *)((ulonglong)uStack_340 & 0xffffffff00000000);
          iVar10 = (int)local_408;
          cVar6 = *(char *)((longlong)piVar26 + 0x19);
          piVar21 = piVar19;
          local_348 = piVar26;
          while (piVar3 = piVar26, cVar6 == '\0') {
            uStack_340._4_4_ = (uint)((ulonglong)uStack_340 >> 0x20);
            if (piVar3[8] < (int)local_408) {
              uStack_340 = (int *)((ulonglong)uStack_340._4_4_ << 0x20);
              piVar26 = *(int **)(piVar3 + 4);
            }
            else {
              uStack_340 = (int *)CONCAT44(uStack_340._4_4_,1);
              piVar26 = *(int **)piVar3;
              piVar21 = piVar3;
            }
            cVar6 = *(char *)((longlong)piVar26 + 0x19);
            local_348 = piVar3;
          }
          if ((*(char *)((longlong)piVar21 + 0x19) != '\0') ||
             (piVar26 = piStack_400, (int)local_408 < piVar21[8])) {
            if (*(longlong *)(lVar23 + 0x20) == 0x555555555555555) {
                    /* WARNING: Subroutine does not return */
              FUN_14019f9d0();
            }
            local_450 = 0;
            local_458 = ppiVar35;
            plVar20 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x30);
            *(int *)(plVar20 + 4) = iVar10;
            plVar20[5] = (longlong)piStack_400;
            *plVar20 = (longlong)piVar19;
            plVar20[1] = (longlong)piVar19;
            plVar20[2] = (longlong)piVar19;
            *(undefined2 *)(plVar20 + 3) = 0;
            local_450 = 0;
            local_3d8 = (undefined4)local_348;
            uStack_3d4 = local_348._4_4_;
            piStack_3d0 = uStack_340;
            FUN_140301600(ppiVar35,&local_3d8,plVar20);
            piVar26 = (int *)0x0;
          }
          if (piVar26 != (int *)0x0) {
            FUN_14019f2c0(piVar26 + -4);
          }
          if (local_470 != (int *)0x0) {
            FUN_14019f2c0(local_470 + -4);
          }
          iVar10 = (int)local_468;
        }
        if (local_478 != (int *)0x0) {
          FUN_14019f2c0(local_478 + -4);
        }
        uVar8 = (int)piVar18 + 1;
        piVar18 = (int *)(ulonglong)uVar8;
      } while ((int)uVar8 < iVar10);
    }
    FUN_142deea90(param_1);
    return;
  case 0x1a:
    bVar7 = FUN_1406e8ae0(param_2);
    local_410 = (uint)bVar7;
    uVar13 = FUN_1406e8c20(param_2);
    local_460 = (int *)CONCAT44(local_460._4_4_,uVar13);
    iVar10 = FUN_1406e8c20(param_2);
    local_408 = (int *)0x0;
    local_3c8 = 0;
    local_40c = iVar10;
    plVar20 = (longlong *)FUN_1406e9050(param_2,&local_458);
    lVar23 = *plVar20;
    *plVar20 = 0;
    local_3c8 = lVar23;
    local_380 = lVar23;
    if (local_458 != (int **)0x0) {
      FUN_14019f2c0(local_458 + -2);
    }
    uVar13 = FUN_1406e8c20(param_2);
    local_428 = (int **)CONCAT44(local_428._4_4_,uVar13);
    uVar13 = FUN_1406e8c20(param_2);
    local_420 = (int *)CONCAT44(local_420._4_4_,uVar13);
    local_440 = FUN_1406e8c20(param_2);
    local_468 = (int *)((ulonglong)local_468._4_4_ << 0x20);
    iVar11 = FUN_142d01050(DAT_143aa84a0,iVar10);
    local_430 = iVar11 != 0;
    local_378[0] = FUN_142d2e9e0(param_1 + 0x23c0);
    iVar11 = FUN_142deaf60(local_378[0]);
    lVar16 = local_378[0];
    uVar8 = (uint)bVar7;
    if (iVar11 == 0) {
      puVar17 = (undefined4 *)FUN_142df2e20(local_378[0],0xffffffff);
      puVar15 = (undefined4 *)FUN_140369c80(lVar16 + 8,0xffffffff);
      puVar22 = (undefined4 *)FUN_140369c80(lVar16 + 0x10,0xffffffff);
      FUN_1402d4870(puVar17,param_2);
      uVar13 = FUN_1415fe510(DAT_143ac87a0,*puVar17,0);
      *puVar15 = uVar13;
      uVar13 = FUN_1415fe510(DAT_143ac87a0,*puVar17,1);
      *puVar22 = uVar13;
      lVar23 = local_380;
      uVar8 = local_410;
      iVar10 = local_40c;
    }
    piVar18 = local_460;
    local_4a0 = CONCAT31(local_4a0._1_3_,local_430);
    local_4a8 = &local_468;
    iVar11 = FUN_142decf30(param_1,(ulonglong)local_460 & 0xffffffff,iVar10,uVar8);
    if (iVar11 == 0) {
      if (lVar23 == 0) {
        return;
      }
      FUN_14019f2c0(lVar23 + -0x10);
      return;
    }
    local_448 = (int **)FUN_14019b780(&DAT_143ad68a0,0x370);
    lVar23 = 0;
    if (local_448 != (int **)0x0) {
      lVar23 = FUN_141808b90(local_448);
    }
    lVar16 = lVar23 + 0x18;
    if (lVar23 == 0) {
      lVar16 = 0;
    }
    if (lVar16 == 0) {
      piStack_3d0 = (int *)0x0;
    }
    else {
      piStack_3d0 = (int *)(lVar16 + -0x18);
      if (piStack_3d0 != (int *)0x0) {
        if (0xfffff < *(ulonglong *)(lVar16 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar16 + 8) = *(longlong *)(lVar16 + 8) + 1;
        UNLOCK();
      }
    }
    piVar19 = piStack_3d0;
    if (uVar8 == 0) {
      if (piStack_3d0 == (int *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      local_388 = 0;
      FUN_14019a260(&local_388,&local_3c8);
      local_4a0 = local_440;
      local_4a8 = (undefined8 *)CONCAT44(local_4a8._4_4_,(int)local_420);
      FUN_14180e3d0(piVar19,&local_388,(ulonglong)piVar18 & 0xffffffff,
                    (ulonglong)local_428 & 0xffffffff);
    }
    else {
      if (piStack_3d0 == (int *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      local_378[1] = 0;
      FUN_14019a260(local_378 + 1,&local_3c8);
      local_498 = (int)local_468;
      local_4a0 = local_440;
      local_4a8 = (undefined8 *)CONCAT44(local_4a8._4_4_,(int)local_420);
      FUN_14180e8d0(piVar19,local_378 + 1,iVar10,(ulonglong)local_428 & 0xffffffff);
    }
    local_2f0 = (longlong)piVar19;
    if (piVar19 != (int *)0x0) {
      if (0xfffff < *(ulonglong *)((longlong)piVar19 + 0x20)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)((longlong)piVar19 + 0x20) = *(longlong *)((longlong)piVar19 + 0x20) + 1;
      UNLOCK();
      piVar19 = piStack_3d0;
    }
    FUN_142d97880(param_1,local_2f8);
    if (piVar19 != (int *)0x0) {
      if (0xffffe < *(longlong *)((longlong)piVar19 + 0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar20 = (longlong *)((longlong)piVar19 + 0x20);
      lVar23 = *plVar20;
      *plVar20 = *plVar20 + -1;
      UNLOCK();
      if (((int)lVar23 == 1) &&
         (puVar24 = (undefined8 *)((longlong)piStack_3d0 + 0x18), puVar24 != (undefined8 *)0x0)) {
        (**(code **)*puVar24)(puVar24,1);
      }
    }
    if (local_3c8 == 0) {
      return;
    }
    FUN_14019f2c0(local_3c8 + -0x10);
    return;
  case 0x1b:
    FUN_1406e9050(param_2,&local_470);
    piVar18 = local_470;
    local_468 = (int *)0x0;
    puVar24 = (undefined8 *)FUN_1408a9e40(&local_3d8,0x126f);
    uVar25 = FUN_14019ba10(&local_468,*puVar24,piVar18);
    local_478 = (int *)0x0;
    FUN_14019a260(&local_478,uVar25);
    if (CONCAT44(uStack_3d4,local_3d8) != 0) {
      FUN_14019f2c0(CONCAT44(uStack_3d4,local_3d8) + -0x10);
    }
    if (local_468 != (int *)0x0) {
      FUN_14019f2c0(local_468 + -4);
    }
    FUN_1415eca30(&local_478,0xb);
    piVar18 = local_470;
    if (local_478 != (int *)0x0) {
      FUN_14019f2c0(local_478 + -4);
      piVar18 = local_470;
    }
    goto LAB_142df2014;
  case 0x1c:
    uVar25 = 0x3f4;
    break;
  case 0x1d:
    uVar25 = 0x3f5;
    break;
  case 0x1e:
    uVar25 = 0x3f6;
    break;
  case 0x1f:
    uVar25 = 0x19e;
    break;
  case 0x20:
    uVar25 = 0x3f7;
    break;
  case 0x21:
    uVar25 = 0x19b;
    break;
  case 0x22:
    uVar25 = 0x3f9;
    break;
  case 0x23:
    uVar25 = 0x3f8;
    break;
  case 0x24:
  case 0x29:
  case 0x2c:
  case 0x30:
    uVar25 = 0x3f2;
    break;
  case 0x25:
    uVar25 = 0x3f3;
    break;
  case 0x26:
    FUN_1406e9050(param_2,&local_468);
    uVar13 = FUN_1406e8c20(param_2);
    FUN_1406e8c20(param_2);
    local_408 = (int *)0x0;
    piStack_400 = (int *)0x0;
    local_3f8 = 0;
    local_3f0 = (int *)0x0;
    local_478 = (int *)0x0;
    FUN_14019bd40(&local_478,0,0);
    piVar18 = local_478;
    if (local_478[-4] != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar18[-3] < 0) {
      FUN_142e54290(0x90,piVar18[-3],0);
    }
    piVar18[-4] = 1;
    *(char *)piVar18 = '\0';
    if (piVar18[-3] + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    iVar10 = 0;
    piVar18[-2] = 0;
    if (local_408 != (int *)0x0) {
      FUN_14019f2c0(local_408 + -4);
    }
    local_408 = piVar18;
    local_478 = (int *)0x0;
    FUN_14019bd40(&local_478,0,0);
    piVar18 = local_478;
    if (local_478[-4] != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar18[-3] < 0) {
      FUN_142e54290(0x90,piVar18[-3],0);
    }
    piVar18[-4] = 1;
    *(char *)piVar18 = '\0';
    if (piVar18[-3] + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    piVar18[-2] = 0;
    if (piStack_400 != (int *)0x0) {
      FUN_14019f2c0(piStack_400 + -4);
    }
    piStack_400 = piVar18;
    local_3f8 = 0;
    local_478 = (int *)0x0;
    FUN_14019bd40(&local_478,0,0);
    piVar18 = local_478;
    if (local_478[-4] != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar18[-3] < 0) {
      FUN_142e54290(0x90,piVar18[-3],0);
    }
    piVar18[-4] = 1;
    *(char *)piVar18 = '\0';
    if (piVar18[-3] + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    piVar18[-2] = 0;
    if (local_3f0 != (int *)0x0) {
      FUN_14019f2c0((longlong)local_3f0 + -0x10);
    }
    local_3f0 = piVar18;
    lVar23 = FUN_142cfe7d0(param_1,0);
    if (lVar23 == 0) {
      if (local_3f0 != (int *)0x0) {
        FUN_14019f2c0(local_3f0 + -4);
      }
      if (piStack_400 != (int *)0x0) {
        FUN_14019f2c0(piStack_400 + -4);
      }
      piVar18 = local_468;
      if (local_408 != (int *)0x0) {
        FUN_14019f2c0(local_408 + -4);
        piVar18 = local_468;
      }
    }
    else {
      FUN_1408cef00(lVar23,uVar13,&local_408);
      piVar18 = local_468;
      if (((piStack_400 == (int *)0x0) || ((char)*piStack_400 == '\0')) ||
         (piStack_400 == local_468)) {
LAB_142df1c41:
        local_470 = (int *)0x0;
        puVar24 = (undefined8 *)FUN_1408a9e40(&local_458,0x145f);
        uVar25 = FUN_14019ba10(&local_470,*puVar24,piVar18);
        local_478 = (int *)0x0;
        FUN_14019a260(&local_478,uVar25);
        local_480 = 0;
        local_488 = 0;
        local_490 = 0;
        local_498 = 0;
        local_4a0 = 0;
        local_4a8 = (undefined8 *)((ulonglong)local_4a8._4_4_ << 0x20);
        FUN_142a26280(&local_478,0,0,1);
        if (local_458 != (int **)0x0) {
          FUN_14019f2c0(local_458 + -2);
        }
      }
      else {
        iVar11 = piStack_400[-2];
        if (local_468 != (int *)0x0) {
          iVar10 = local_468[-2];
        }
        if ((iVar11 == iVar10) &&
           ((iVar11 == 0 || (iVar10 = memcmp(piStack_400,local_468,(longlong)iVar11), iVar10 == 0)))
           ) goto LAB_142df1c41;
        local_470 = (int *)0x0;
        puVar24 = (undefined8 *)FUN_1408a9e40(&local_458,0x1460);
        uVar25 = FUN_14019ba10(&local_470,*puVar24,piVar18,piStack_400);
        local_478 = (int *)0x0;
        FUN_14019a260(&local_478,uVar25);
        local_480 = 0;
        local_488 = 0;
        local_490 = 0;
        local_498 = 0;
        local_4a0 = 0;
        local_4a8 = (undefined8 *)((ulonglong)local_4a8._4_4_ << 0x20);
        FUN_142a26280(&local_478,0,0,1);
        if (local_458 != (int **)0x0) {
          FUN_14019f2c0(local_458 + -2);
        }
      }
      if (local_470 != (int *)0x0) {
        FUN_14019f2c0(local_470 + -4);
      }
      if (local_3f0 != (int *)0x0) {
        FUN_14019f2c0(local_3f0 + -4);
      }
      if (piStack_400 != (int *)0x0) {
        FUN_14019f2c0(piStack_400 + -4);
      }
      piVar18 = local_468;
      if (local_408 != (int *)0x0) {
        FUN_14019f2c0(local_408 + -4);
        piVar18 = local_468;
      }
    }
    goto LAB_142df2014;
  case 0x27:
    uVar13 = FUN_1406e8c20(param_2);
    *(undefined4 *)(param_1 + 0x33b8) = uVar13;
    FUN_1411bb820();
    return;
  case 0x28:
    local_198 = 0;
    uStack_190 = 0;
    local_188 = 0;
    uStack_186 = 0xffffffff;
    uStack_182 = 0;
    uStack_17a = 0;
    uStack_172 = 0;
    FUN_142ef8250(aiStack_170,0,0x111);
    uVar39 = 0;
    local_5f = 0;
    local_57 = 0;
    FUN_1402d4870(&local_198,param_2);
    if ((aiStack_170[0] == 0) || (3 < (byte)(local_188._1_1_ - 5U))) {
      plVar20 = (longlong *)FUN_142d2e9e0(param_1 + 0x23c0);
      iVar10 = (int)local_198;
      iVar11 = FUN_142deaf60(plVar20);
      uVar30 = uVar39;
      uVar32 = uVar39;
      if (iVar11 == 0) {
        do {
          uVar8 = (uint)uVar30;
          FUN_142deaf60(plVar20);
          lVar23 = *plVar20;
          iVar11 = 0;
          if (lVar23 != 0) {
            iVar11 = *(int *)(lVar23 + -8);
          }
          if (iVar11 <= (int)uVar8) break;
          if (iVar10 != 0) {
            uVar27 = 0;
            if (lVar23 != 0) {
              uVar27 = *(uint *)(lVar23 + -8);
            }
            if (((int)uVar8 < 0) || (uVar27 <= uVar8)) {
              uVar37 = uVar39;
              if (lVar23 != 0) {
                uVar37 = (ulonglong)*(uint *)(lVar23 + -8);
              }
              FUN_142e54290(0xbc,uVar30,uVar37);
              lVar23 = *plVar20;
            }
            if (*(int *)(uVar32 + lVar23) == iVar10) goto LAB_142deffa1;
          }
          uVar30 = (ulonglong)(uVar8 + 1);
          uVar32 = uVar32 + 0x149;
        } while( true );
      }
      uVar8 = 0xffffffff;
    }
    else {
      uVar25 = FUN_142d2e9e0(param_1 + 0x23c0);
      uVar8 = FUN_142debab0(uVar25,0,aiStack_170[0]);
    }
LAB_142deffa1:
    plVar20 = (longlong *)FUN_142d2e9e0(param_1 + 0x23c0);
    iVar10 = FUN_142deaf60(plVar20);
    if (iVar10 == 0) {
      if (uVar8 == 0xffffffff) {
        puVar14 = (undefined8 *)FUN_142df2e20(plVar20,0xffffffff);
        puVar17 = (undefined4 *)FUN_140369c80(plVar20 + 1,0xffffffff);
        puVar15 = (undefined4 *)FUN_140369c80(plVar20 + 2,0xffffffff);
        uVar13 = FUN_1415fe510(DAT_143ac87a0,*(undefined4 *)puVar14,0);
        *puVar17 = uVar13;
        uVar13 = FUN_1415fe510(DAT_143ac87a0,*(undefined4 *)puVar14,1);
        *puVar15 = uVar13;
        lVar23 = 2;
        puVar24 = &local_198;
        do {
          puVar29 = puVar14;
          puVar28 = puVar24;
          uVar25 = puVar28[1];
          *puVar29 = *puVar28;
          puVar29[1] = uVar25;
          uVar25 = puVar28[3];
          puVar29[2] = puVar28[2];
          puVar29[3] = uVar25;
          uVar25 = puVar28[5];
          puVar29[4] = puVar28[4];
          puVar29[5] = uVar25;
          uVar25 = puVar28[7];
          puVar29[6] = puVar28[6];
          puVar29[7] = uVar25;
          uVar25 = puVar28[9];
          puVar29[8] = puVar28[8];
          puVar29[9] = uVar25;
          uVar25 = puVar28[0xb];
          puVar29[10] = puVar28[10];
          puVar29[0xb] = uVar25;
          uVar25 = puVar28[0xd];
          puVar29[0xc] = puVar28[0xc];
          puVar29[0xd] = uVar25;
          uVar25 = puVar28[0xf];
          puVar29[0xe] = puVar28[0xe];
          puVar29[0xf] = uVar25;
          lVar23 = lVar23 + -1;
          puVar24 = puVar28 + 0x10;
          puVar14 = puVar29 + 0x10;
        } while (lVar23 != 0);
        uVar25 = puVar28[0x11];
        puVar29[0x10] = puVar28[0x10];
        puVar29[0x11] = uVar25;
        uVar25 = puVar28[0x13];
        puVar29[0x12] = puVar28[0x12];
        puVar29[0x13] = uVar25;
        uVar13 = *(undefined4 *)((longlong)puVar28 + 0xa4);
        uVar1 = *(undefined4 *)(puVar28 + 0x15);
        uVar2 = *(undefined4 *)((longlong)puVar28 + 0xac);
        *(undefined4 *)(puVar29 + 0x14) = *(undefined4 *)(puVar28 + 0x14);
        *(undefined4 *)((longlong)puVar29 + 0xa4) = uVar13;
        *(undefined4 *)(puVar29 + 0x15) = uVar1;
        *(undefined4 *)((longlong)puVar29 + 0xac) = uVar2;
        uVar13 = *(undefined4 *)((longlong)puVar28 + 0xb4);
        uVar1 = *(undefined4 *)(puVar28 + 0x17);
        uVar2 = *(undefined4 *)((longlong)puVar28 + 0xbc);
        *(undefined4 *)(puVar29 + 0x16) = *(undefined4 *)(puVar28 + 0x16);
        *(undefined4 *)((longlong)puVar29 + 0xb4) = uVar13;
        *(undefined4 *)(puVar29 + 0x17) = uVar1;
        *(undefined4 *)((longlong)puVar29 + 0xbc) = uVar2;
        puVar29[0x18] = puVar28[0x18];
        *(undefined1 *)(puVar29 + 0x19) = *(undefined1 *)(puVar28 + 0x19);
        FUN_142deea90(param_1);
        FUN_1411bb5e0();
        return;
      }
      lVar23 = *plVar20;
      if (((lVar23 != 0) && (uVar8 < *(uint *)(lVar23 + -8))) && (-1 < (int)uVar8)) {
        lVar38 = 2;
        lVar16 = 2;
        puVar24 = local_2e8;
        puVar14 = &local_198;
        do {
          puVar29 = puVar14;
          puVar28 = puVar24;
          uVar25 = puVar29[1];
          *puVar28 = *puVar29;
          puVar28[1] = uVar25;
          uVar25 = puVar29[3];
          puVar28[2] = puVar29[2];
          puVar28[3] = uVar25;
          uVar25 = puVar29[5];
          puVar28[4] = puVar29[4];
          puVar28[5] = uVar25;
          uVar25 = puVar29[7];
          puVar28[6] = puVar29[6];
          puVar28[7] = uVar25;
          uVar25 = puVar29[9];
          puVar28[8] = puVar29[8];
          puVar28[9] = uVar25;
          uVar25 = puVar29[0xb];
          puVar28[10] = puVar29[10];
          puVar28[0xb] = uVar25;
          uVar25 = puVar29[0xd];
          puVar28[0xc] = puVar29[0xc];
          puVar28[0xd] = uVar25;
          uVar25 = puVar29[0xf];
          puVar28[0xe] = puVar29[0xe];
          puVar28[0xf] = uVar25;
          lVar16 = lVar16 + -1;
          puVar24 = puVar28 + 0x10;
          puVar14 = puVar29 + 0x10;
        } while (lVar16 != 0);
        uVar25 = puVar29[0x11];
        puVar28[0x10] = puVar29[0x10];
        puVar28[0x11] = uVar25;
        uVar25 = puVar29[0x13];
        puVar28[0x12] = puVar29[0x12];
        puVar28[0x13] = uVar25;
        uVar25 = puVar29[0x15];
        puVar28[0x14] = puVar29[0x14];
        puVar28[0x15] = uVar25;
        uVar25 = puVar29[0x17];
        puVar28[0x16] = puVar29[0x16];
        puVar28[0x17] = uVar25;
        puVar28[0x18] = puVar29[0x18];
        *(undefined1 *)(puVar28 + 0x19) = *(undefined1 *)(puVar29 + 0x19);
        puVar24 = local_2e8;
        puVar14 = (undefined8 *)((longlong)(int)uVar8 * 0x149 + lVar23);
        do {
          puVar29 = puVar14;
          puVar28 = puVar24;
          uVar25 = puVar28[1];
          *puVar29 = *puVar28;
          puVar29[1] = uVar25;
          uVar25 = puVar28[3];
          puVar29[2] = puVar28[2];
          puVar29[3] = uVar25;
          uVar25 = puVar28[5];
          puVar29[4] = puVar28[4];
          puVar29[5] = uVar25;
          uVar25 = puVar28[7];
          puVar29[6] = puVar28[6];
          puVar29[7] = uVar25;
          uVar25 = puVar28[9];
          puVar29[8] = puVar28[8];
          puVar29[9] = uVar25;
          uVar25 = puVar28[0xb];
          puVar29[10] = puVar28[10];
          puVar29[0xb] = uVar25;
          uVar25 = puVar28[0xd];
          puVar29[0xc] = puVar28[0xc];
          puVar29[0xd] = uVar25;
          uVar25 = puVar28[0xf];
          puVar29[0xe] = puVar28[0xe];
          puVar29[0xf] = uVar25;
          lVar38 = lVar38 + -1;
          puVar24 = puVar28 + 0x10;
          puVar14 = puVar29 + 0x10;
        } while (lVar38 != 0);
        uVar25 = puVar28[0x11];
        puVar29[0x10] = puVar28[0x10];
        puVar29[0x11] = uVar25;
        uVar25 = puVar28[0x13];
        puVar29[0x12] = puVar28[0x12];
        puVar29[0x13] = uVar25;
        uVar13 = *(undefined4 *)((longlong)puVar28 + 0xa4);
        uVar1 = *(undefined4 *)(puVar28 + 0x15);
        uVar2 = *(undefined4 *)((longlong)puVar28 + 0xac);
        *(undefined4 *)(puVar29 + 0x14) = *(undefined4 *)(puVar28 + 0x14);
        *(undefined4 *)((longlong)puVar29 + 0xa4) = uVar13;
        *(undefined4 *)(puVar29 + 0x15) = uVar1;
        *(undefined4 *)((longlong)puVar29 + 0xac) = uVar2;
        uVar25 = puVar28[0x17];
        puVar29[0x16] = puVar28[0x16];
        puVar29[0x17] = uVar25;
        puVar29[0x18] = puVar28[0x18];
        *(undefined1 *)(puVar29 + 0x19) = *(undefined1 *)(puVar28 + 0x19);
      }
    }
    FUN_142deea90(param_1);
    FUN_1411bb5e0();
    return;
  case 0x2a:
    uVar25 = 0x199;
    break;
  case 0x2b:
    cVar6 = FUN_1406e8ae0(param_2);
    iVar10 = FUN_1406e8c20(param_2);
    plVar20 = (longlong *)FUN_142d2e9e0(param_1 + 0x23c0);
    if (cVar6 == '\0') {
      iVar11 = FUN_142deaf60(plVar20);
      if (iVar11 == 0) {
        uVar8 = 0;
        lVar23 = 0;
        do {
          FUN_142deaf60(plVar20);
          lVar16 = *plVar20;
          iVar11 = 0;
          if (lVar16 != 0) {
            iVar11 = *(int *)(lVar16 + -8);
          }
          if (iVar11 <= (int)uVar8) break;
          if (iVar10 != 0) {
            uVar27 = 0;
            if (lVar16 != 0) {
              uVar27 = *(uint *)(lVar16 + -8);
            }
            if (((int)uVar8 < 0) || (uVar27 <= uVar8)) {
              uVar13 = 0;
              if (lVar16 != 0) {
                uVar13 = *(undefined4 *)(lVar16 + -8);
              }
              FUN_142e54290(0xbc,uVar8,uVar13);
              lVar16 = *plVar20;
            }
            if (*(int *)(lVar23 + lVar16) == iVar10) goto LAB_142df030e;
          }
          uVar8 = uVar8 + 1;
          lVar23 = lVar23 + 0x149;
        } while( true );
      }
      goto LAB_142df0332;
    }
    uVar8 = FUN_142debab0(plVar20,0,iVar10);
    if ((int)uVar8 < 0) goto LAB_142df0332;
    FUN_142df3060(plVar20,uVar8);
    FUN_142df2fe0(plVar20 + 1,uVar8);
    goto LAB_142df032d;
  case 0x2d:
    uVar8 = FUN_1406e8c20(param_2);
    ppiVar35 = (int **)(ulonglong)uVar8;
    local_460 = (int *)CONCAT44(local_460._4_4_,uVar8);
    uVar9 = FUN_1406e8c20(param_2);
    bVar7 = FUN_1406e8ae0(param_2);
    local_440 = (uint)bVar7;
    uVar13 = FUN_1406e8c20(param_2);
    local_428 = (int **)CONCAT44(local_428._4_4_,uVar13);
    bVar7 = FUN_1406e8ae0(param_2);
    local_420 = (int *)CONCAT44(local_420._4_4_,(uint)bVar7);
    bVar5 = FUN_1406e8ae0(param_2);
    local_470 = (int *)CONCAT44(local_470._4_4_,(uint)bVar5);
    ppiVar41 = (int **)0x0;
    uVar8 = 0;
    uVar27 = uVar8;
    if (bVar7 != 0) {
      ppiVar35 = ppiVar41;
      uVar27 = uVar9;
    }
    uVar25 = FUN_142d2e9e0(param_1 + 0x23c0);
    iVar10 = FUN_142debab0(uVar25,ppiVar35,uVar27);
    if (iVar10 < 0) {
      return;
    }
    local_478 = (int *)0x0;
    local_418 = (int **)0x0;
    ppiVar35 = local_418;
    local_40c = iVar10;
    if ((int)local_420 != 0) {
      puVar24 = (undefined8 *)FUN_1406e9050(param_2,&local_3d8);
      piVar18 = (int *)*puVar24;
      *puVar24 = 0;
      local_478 = piVar18;
      if (CONCAT44(uStack_3d4,local_3d8) != 0) {
        FUN_14019f2c0(CONCAT44(uStack_3d4,local_3d8) + -0x10);
      }
      uVar13 = FUN_1406e8c20(param_2);
      local_468 = (int *)CONCAT44(local_468._4_4_,uVar13);
      local_410 = FUN_1406e8c20(param_2);
      iVar11 = FUN_1406e8c20(param_2);
      piVar19 = local_478;
      lVar23 = param_1 + 0x23c0;
      if ((piVar18 != (int *)0x0) && ((char)*local_478 != '\0')) {
        uVar25 = FUN_142d2e9e0(lVar23);
        lVar16 = FUN_142d241a0(uVar25,iVar10);
        if (lVar16 + 4 != 0) {
          (*DAT_143262858)(lVar16 + 4,piVar19);
        }
      }
      iVar12 = (int)local_468;
      if ((int)local_468 != -1) {
        uVar25 = FUN_142d2e9e0(param_1 + 0x23c0);
        lVar16 = FUN_142d241a0(uVar25,iVar10);
        *(int *)(lVar16 + 0x139) = iVar12;
      }
      uVar27 = local_410;
      if (local_410 != -1) {
        uVar25 = FUN_142d2e9e0(param_1 + 0x23c0);
        lVar16 = FUN_142d241a0(uVar25,iVar10);
        *(uint *)(lVar16 + 0x13d) = uVar27;
      }
      if (iVar11 != -1) {
        uVar25 = FUN_142d2e9e0(param_1 + 0x23c0);
        lVar16 = FUN_142d241a0(uVar25,iVar10);
        *(int *)(lVar16 + 0x141) = iVar11;
      }
      uVar25 = FUN_142d2e9e0(lVar23);
      puVar17 = (undefined4 *)FUN_142d241a0(uVar25,iVar10);
      *puVar17 = (int)local_460;
      uVar25 = FUN_142d2e9e0(lVar23);
      lVar23 = FUN_142d241a0(uVar25,iVar10);
      lVar23 = lVar23 + 0x2c;
      local_458 = (int **)0x0;
      ppiVar35 = ppiVar41;
      if (lVar23 != 0) {
        ppiVar33 = (int **)0xffffffffffffffff;
        do {
          ppiVar33 = (int **)((longlong)ppiVar33 + 1);
        } while (*(char *)(lVar23 + (longlong)ppiVar33) != '\0');
        uVar9 = (uint)ppiVar33;
        uVar27 = uVar8;
        if (0 < (int)uVar9) {
          uVar27 = uVar9;
        }
        piVar18 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(int)(uVar27 + 0x11));
        piVar18[1] = uVar27;
        *piVar18 = -1;
        ppiVar35 = (int **)(piVar18 + 4);
        piVar18[2] = 0;
        *(char *)ppiVar35 = '\0';
        local_458 = ppiVar35;
        FUN_142ef7ba0(ppiVar35,lVar23,(longlong)(int)uVar9);
        if (*piVar18 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((uVar9 == 0xffffffff) || ((int)uVar9 <= piVar18[1])) {
          *piVar18 = 1;
          if (uVar9 != 0xffffffff) goto LAB_142df05a5;
          ppiVar33 = ppiVar41;
          if (ppiVar35 != (int **)0x0) {
            ppiVar33 = (int **)0xffffffffffffffff;
            do {
              ppiVar33 = (int **)((longlong)ppiVar33 + 1);
            } while (*(char *)((longlong)ppiVar35 + (longlong)ppiVar33) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar18[1],(ulonglong)ppiVar33 & 0xffffffff);
          *piVar18 = 1;
LAB_142df05a5:
          *(char *)((longlong)ppiVar35 + (longlong)(int)uVar9) = '\0';
        }
        iVar10 = (int)ppiVar33;
        if ((iVar10 < 0) || (piVar18[1] + 1 <= iVar10)) {
          FUN_142e54290(0x9c,(ulonglong)ppiVar33 & 0xffffffff);
        }
        piVar18[2] = iVar10;
        iVar10 = local_40c;
      }
    }
    local_418 = ppiVar35;
    ppiVar35 = local_418;
    lVar23 = param_1 + 0x23c0;
    uVar25 = FUN_142d2e9e0(lVar23);
    lVar16 = FUN_142d241a0(uVar25,iVar10);
    uVar27 = *(uint *)(lVar16 + 0x145);
    uVar25 = FUN_142d2e9e0(lVar23);
    lVar16 = FUN_142d241a0(uVar25,iVar10);
    local_468 = (int *)CONCAT44(local_468._4_4_,*(undefined4 *)(lVar16 + 0x12));
    uVar25 = FUN_142d2e9e0(lVar23);
    piVar18 = (int *)FUN_142d241a0(uVar25,iVar10);
    iVar11 = (int)local_460;
    uVar13 = SUB84(local_470,0);
    if (((int)local_468 == (int)local_428) && (uVar27 == local_440)) {
      if ((int)local_420 == 0) {
        if (ppiVar35 != (int **)0x0) {
          FUN_14019f2c0(ppiVar35 + -2);
        }
      }
      else {
        if (*piVar18 != (int)local_460) goto LAB_142df06cc;
        if (ppiVar35 != (int **)0x0) {
          FUN_14019f2c0(ppiVar35 + -2);
        }
      }
    }
    else {
      if (((int)local_420 != 0) && (*piVar18 == (int)local_460)) {
        ppiVar33 = (int **)((ulonglong)local_470 & 0xffffffff);
        if (-1 < (int)local_468) {
          ppiVar33 = ppiVar41;
        }
        uVar13 = SUB84(ppiVar33,0);
        local_470 = (int *)CONCAT44(local_470._4_4_,uVar13);
      }
LAB_142df06cc:
      if (DAT_143ac87a0 != 0) {
        if (*(int *)(DAT_143ac87a0 + 0x164) == 0) {
          uVar13 = 0;
        }
        local_470 = (int *)CONCAT44(local_470._4_4_,uVar13);
      }
      uVar25 = FUN_141892840();
      cVar6 = FUN_14031f3b0(uVar25);
      iVar31 = 0;
      iVar12 = (int)local_470;
      if (cVar6 != '\0') {
        iVar12 = iVar31;
      }
      local_470 = (int *)CONCAT44(local_470._4_4_,iVar12);
      uVar25 = FUN_142d2e9e0(lVar23);
      lVar16 = FUN_142d241a0(uVar25,iVar10);
      *(int *)(lVar16 + 0x12) = (int)local_428;
      uVar25 = FUN_142d2e9e0(lVar23);
      lVar16 = FUN_142d241a0(uVar25,iVar10);
      *(uint *)(lVar16 + 0x145) = local_440;
      uVar25 = FUN_142d2e9e0(lVar23);
      piVar18 = (int *)FUN_142d241a0(uVar25,iVar10);
      *piVar18 = iVar11;
      uVar25 = FUN_142d2e9e0(lVar23);
      local_3b8 = FUN_142d241a0(uVar25,iVar10);
      local_3b8 = local_3b8 + 4;
      local_3e8[0] = (int *)0x0;
      if (local_3b8 != 0) {
        ppiVar33 = (int **)0xffffffffffffffff;
        do {
          ppiVar33 = (int **)((longlong)ppiVar33 + 1);
        } while (*(char *)(local_3b8 + (longlong)ppiVar33) != '\0');
        iVar11 = (int)ppiVar33;
        iVar10 = iVar31;
        if (0 < iVar11) {
          iVar10 = iVar11;
        }
        piVar19 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar10 + 0x11));
        piVar19[1] = iVar10;
        *piVar19 = -1;
        piVar18 = piVar19 + 4;
        piVar19[2] = 0;
        *(char *)piVar18 = '\0';
        local_3e8[0] = piVar18;
        FUN_142ef7ba0(piVar18,local_3b8,(longlong)iVar11);
        if (*piVar19 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar11 == -1) || (iVar11 <= piVar19[1])) {
          *piVar19 = 1;
          if (iVar11 != -1) goto LAB_142df07fa;
          ppiVar33 = ppiVar41;
          if (piVar18 != (int *)0x0) {
            ppiVar33 = (int **)0xffffffffffffffff;
            do {
              ppiVar33 = (int **)((longlong)ppiVar33 + 1);
            } while (*(char *)((longlong)piVar18 + (longlong)ppiVar33) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar19[1],(ulonglong)ppiVar33 & 0xffffffff);
          *piVar19 = 1;
LAB_142df07fa:
          *(char *)((longlong)local_3e8[0] + (longlong)iVar11) = '\0';
        }
        iVar10 = (int)ppiVar33;
        if ((iVar10 < 0) || (piVar19[1] + 1 <= iVar10)) {
          FUN_142e54290(0x9c,(ulonglong)ppiVar33 & 0xffffffff);
        }
        piVar19[2] = iVar10;
      }
      piVar18 = local_3e8[0];
      local_460 = (int *)0x0;
      if ((local_3e8[0] != (int *)0x0) && (piVar19 = local_3e8[0] + -4, piVar19 != (int *)0x0)) {
        if (*piVar19 == -1) {
          FUN_142e52d50(0xcb,0xffffff01);
          ppiVar33 = (int **)0xffffffffffffffff;
          do {
            ppiVar33 = (int **)((longlong)ppiVar33 + 1);
          } while (*(char *)((longlong)piVar18 + (longlong)ppiVar33) != '\0');
          iVar11 = (int)ppiVar33;
          iVar10 = iVar31;
          if (0 < iVar11) {
            iVar10 = iVar11;
          }
          piVar19 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar10 + 0x11));
          piVar19[1] = iVar10;
          *piVar19 = -1;
          piVar18 = piVar19 + 4;
          piVar19[2] = 0;
          *(char *)piVar18 = '\0';
          local_408 = piVar18;
          FUN_142ef7ba0(piVar18,local_3e8[0],(longlong)iVar11);
          if (*piVar19 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar11 == -1) || (iVar11 <= piVar19[1])) {
            *piVar19 = 1;
            if (iVar11 != -1) goto LAB_142df08eb;
            ppiVar33 = ppiVar41;
            if (piVar18 != (int *)0x0) {
              ppiVar33 = (int **)0xffffffffffffffff;
              do {
                ppiVar33 = (int **)((longlong)ppiVar33 + 1);
              } while (*(char *)((longlong)piVar18 + (longlong)ppiVar33) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar19[1],(ulonglong)ppiVar33 & 0xffffffff);
            *piVar19 = 1;
LAB_142df08eb:
            *(char *)((longlong)piVar18 + (longlong)iVar11) = '\0';
          }
          iVar10 = (int)ppiVar33;
          if ((iVar10 < 0) || (piVar19[1] + 1 <= iVar10)) {
            FUN_142e54290(0x9c,(ulonglong)ppiVar33 & 0xffffffff);
          }
          piVar19[2] = iVar10;
          local_460 = piVar18;
        }
        else {
          if (*piVar19 < 1) {
            FUN_142e52dd0(0xd2);
          }
          LOCK();
          *piVar19 = *piVar19 + 1;
          UNLOCK();
          local_460 = piVar18;
          ppiVar35 = local_418;
        }
      }
      piVar19 = local_460;
      piVar18 = local_460;
      if ((ppiVar35 != (int **)0x0) && (*(char *)ppiVar35 != '\0')) {
        local_378[3] = 0;
        plVar20 = (longlong *)FUN_14019ba10(local_378 + 3,&DAT_14339ee94,ppiVar35);
        local_3b0 = (int ***)*plVar20;
        piVar18 = piVar19;
        if (local_3b0 != (int ***)0x0) {
          uVar27 = *(uint *)(local_3b0 + -1);
          ppiVar33 = (int **)(longlong)(int)uVar27;
          if (uVar27 != 0) {
            ppiVar34 = ppiVar41;
            if (piVar19 == (int *)0x0) goto LAB_142df0b31;
            if ((char)*piVar19 != '\0') {
              local_410 = piVar19[-2] + uVar27;
              for (uVar27 = piVar19[-3]; (int)uVar27 < (int)local_410; uVar27 = uVar27 * 2) {
              }
              piVar26 = piVar19 + -4;
              uVar9 = uVar8;
              if (piVar26 == (int *)0x0) {
LAB_142df0a3d:
                if ((int)uVar9 < (int)uVar27) {
                  uVar9 = uVar27;
                }
                local_440 = uVar9;
                local_438 = (undefined8 *)
                            FUN_14019b600(&DAT_143ad6a30,(longlong)(int)(uVar9 + 0x11));
                *(uint *)((longlong)local_438 + 4) = uVar9;
                *(undefined4 *)local_438 = 0xffffffff;
                piVar18 = (int *)(local_438 + 2);
                local_460 = piVar18;
                if (piVar26 == (int *)0x0) {
                  *(undefined4 *)(local_438 + 1) = 0;
                  *(char *)piVar18 = '\0';
                }
                else {
                  iVar10 = piVar19[-2] + 1;
                  local_40c = local_440 + 1;
                  if (local_40c < iVar10) {
                    FUN_142e54290(0x5c,iVar10,local_40c);
                    iVar10 = local_40c;
                  }
                  FUN_142ef7ba0(piVar18,piVar19,(longlong)iVar10);
                  *(int *)(local_438 + 1) = piVar19[-2];
                  *(char *)((longlong)(int)local_440 + (longlong)piVar18) = '\0';
                  FUN_14019f2c0(piVar26);
                }
              }
              else {
                if ((1 < *piVar26) || (piVar19[-3] < (int)uVar27)) {
                  uVar9 = piVar19[-2];
                  goto LAB_142df0a3d;
                }
                if (*piVar26 != 1) {
                  FUN_142e52dd0(0x74);
                }
                *piVar26 = -1;
              }
              if (piVar18 != (int *)0x0) {
                uVar8 = piVar18[-2];
              }
              FUN_142ef7ba0((char *)((longlong)(int)uVar8 + (longlong)piVar18),local_3b0,ppiVar33);
              FUN_14019c870(&local_460,local_410);
              goto LAB_142df0bd7;
            }
            if ((piVar19 == (int *)0x0) ||
               (ppiVar34 = (int **)(piVar19 + -4), ppiVar34 == (int **)0x0)) {
LAB_142df0b31:
              if ((int)uVar8 < (int)uVar27) {
                uVar8 = uVar27;
              }
              puVar17 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(int)(uVar8 + 0x11));
              puVar17[1] = uVar8;
              *puVar17 = 0xffffffff;
              piVar18 = puVar17 + 4;
              puVar17[2] = 0;
              *(char *)piVar18 = '\0';
              local_460 = piVar18;
              if (ppiVar34 != (int **)0x0) {
                FUN_14019f2c0(ppiVar34);
              }
            }
            else {
              if ((1 < *(int *)ppiVar34) || (piVar19[-3] < (int)uVar27)) {
                uVar8 = piVar19[-2];
                goto LAB_142df0b31;
              }
              if (*(int *)ppiVar34 != 1) {
                FUN_142e52dd0(0x74);
              }
              *(int *)ppiVar34 = -1;
            }
            FUN_142ef7ba0(piVar18,local_3b0,ppiVar33);
            if (piVar18[-4] != -1) {
              FUN_142e52dd0(0x8b);
            }
            if ((uVar27 == 0xffffffff) || ((int)uVar27 <= piVar18[-3])) {
              piVar18[-4] = 1;
              if (uVar27 != 0xffffffff) goto LAB_142df0bb4;
              ppiVar33 = ppiVar41;
              if (piVar18 != (int *)0x0) {
                ppiVar33 = (int **)0xffffffffffffffff;
                do {
                  ppiVar33 = (int **)((longlong)ppiVar33 + 1);
                } while (*(char *)((longlong)piVar18 + (longlong)ppiVar33) != '\0');
              }
            }
            else {
              FUN_142e54290(0x90,piVar18[-3],uVar27);
              piVar18[-4] = 1;
LAB_142df0bb4:
              *(char *)((longlong)ppiVar33 + (longlong)piVar18) = '\0';
            }
            iVar10 = (int)ppiVar33;
            if ((iVar10 < 0) || (piVar18[-3] + 1 <= iVar10)) {
              FUN_142e54290(0x9c,(ulonglong)ppiVar33 & 0xffffffff);
            }
            piVar18[-2] = iVar10;
          }
        }
LAB_142df0bd7:
        if (local_378[3] != 0) {
          FUN_14019f2c0(local_378[3] + -0x10);
        }
      }
      if ((int)local_470 != 0) {
        if ((((int)local_468 < 0) && (-1 < (int)local_428)) ||
           (((int)local_420 != 0 &&
            (((int)local_428 != *(int *)(param_1 + 0x2260) && (-1 < (int)local_428)))))) {
          piVar19 = piVar18;
          if (*(int *)(DAT_143ac87a0 + 0x318) != 0) {
            local_420 = (int *)0x0;
            piVar26 = local_420;
            if ((piVar18 != (int *)0x0) && (piVar21 = piVar18 + -4, piVar21 != (int *)0x0)) {
              if (*piVar21 == -1) {
                FUN_142e52d50(0xcb,0xffffff01);
                ppiVar33 = (int **)0xffffffffffffffff;
                do {
                  ppiVar33 = (int **)((longlong)ppiVar33 + 1);
                } while (*(char *)((longlong)piVar18 + (longlong)ppiVar33) != '\0');
                iVar11 = (int)ppiVar33;
                iVar10 = iVar31;
                if (0 < iVar11) {
                  iVar10 = iVar11;
                }
                piVar21 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar10 + 0x11));
                piVar21[1] = iVar10;
                *piVar21 = -1;
                piVar26 = piVar21 + 4;
                piVar21[2] = 0;
                *(char *)piVar26 = '\0';
                local_438 = (undefined8 *)(longlong)iVar11;
                local_320 = piVar26;
                FUN_142ef7ba0(piVar26,piVar18,local_438);
                if (*piVar21 != -1) {
                  FUN_142e52dd0(0x8b);
                }
                if ((iVar11 == -1) || (iVar11 <= piVar21[1])) {
                  *piVar21 = 1;
                  if (iVar11 != -1) goto LAB_142df0cff;
                  ppiVar33 = ppiVar41;
                  if (piVar26 != (int *)0x0) {
                    ppiVar33 = (int **)0xffffffffffffffff;
                    do {
                      ppiVar33 = (int **)((longlong)ppiVar33 + 1);
                    } while (*(char *)((longlong)piVar26 + (longlong)ppiVar33) != '\0');
                  }
                }
                else {
                  FUN_142e54290(0x90,piVar21[1],(ulonglong)ppiVar33 & 0xffffffff);
                  *piVar21 = 1;
LAB_142df0cff:
                  *(char *)((longlong)piVar26 + (longlong)local_438) = '\0';
                }
                iVar10 = (int)ppiVar33;
                if ((iVar10 < 0) || (piVar21[1] + 1 <= iVar10)) {
                  FUN_142e54290(0x9c,(ulonglong)ppiVar33 & 0xffffffff);
                }
                piVar21[2] = iVar10;
                if (local_420 != (int *)0x0) {
                  FUN_14019f2c0(local_420 + -4);
                }
              }
              else {
                if (*piVar21 < 1) {
                  FUN_142e52dd0(0xd2);
                }
                LOCK();
                *piVar21 = *piVar21 + 1;
                UNLOCK();
                piVar19 = local_460;
                ppiVar35 = local_418;
                piVar26 = piVar18;
                if (local_420 != (int *)0x0) {
                  FUN_14019f2c0(local_420 + -4);
                  piVar19 = local_460;
                  ppiVar35 = local_418;
                }
              }
            }
            local_420 = piVar26;
            iVar10 = FUN_142ddbd20(param_1,&local_420);
            piVar18 = piVar19;
            if (iVar10 != 0) goto LAB_142df12ed;
          }
          local_438 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x370);
          ppiVar33 = ppiVar41;
          if (local_438 != (undefined8 *)0x0) {
            ppiVar33 = (int **)FUN_141808b90(local_438);
          }
          ppiVar34 = ppiVar33 + 3;
          if (ppiVar33 == (int **)0x0) {
            ppiVar34 = ppiVar41;
          }
          if (ppiVar34 == (int **)0x0) {
            local_350 = (int **)0x0;
          }
          else {
            local_350 = ppiVar34 + -3;
            if (local_350 != (int **)0x0) {
              if ((int *)0xfffff < ppiVar34[1]) {
                FUN_142e541f0(0x30f);
              }
              LOCK();
              ppiVar34[1] = (int *)((longlong)ppiVar34[1] + 1);
              UNLOCK();
              piVar19 = local_460;
              ppiVar35 = local_418;
            }
          }
          ppiVar33 = local_350;
          if (local_350 == (int **)0x0) {
            FUN_142e52ed0(0x431,0);
          }
          local_3b0 = &local_428;
          local_428 = (int **)0x0;
          ppiVar36 = ppiVar35;
          ppiVar34 = local_428;
          if ((ppiVar35 != (int **)0x0) && (ppiVar40 = ppiVar35 + -2, ppiVar40 != (int **)0x0)) {
            if (*(int *)ppiVar40 == -1) {
              FUN_142e52d50(0xcb,0xffffff01);
              ppiVar40 = (int **)0xffffffffffffffff;
              do {
                ppiVar40 = (int **)((longlong)ppiVar40 + 1);
              } while (*(char *)((longlong)ppiVar35 + (longlong)ppiVar40) != '\0');
              iVar10 = (int)ppiVar40;
              if (0 < iVar10) {
                iVar31 = iVar10;
              }
              piVar18 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar31 + 0x11));
              piVar18[1] = iVar31;
              *piVar18 = -1;
              ppiVar34 = (int **)(piVar18 + 4);
              piVar18[2] = 0;
              *(char *)ppiVar34 = '\0';
              local_438 = (undefined8 *)(longlong)iVar10;
              local_448 = ppiVar34;
              FUN_142ef7ba0(ppiVar34,ppiVar35,local_438);
              if (*piVar18 != -1) {
                FUN_142e52dd0(0x8b);
              }
              if ((iVar10 == -1) || (iVar10 <= piVar18[1])) {
                *piVar18 = 1;
                if (iVar10 != -1) goto LAB_142df0f44;
                if (ppiVar34 != (int **)0x0) {
                  ppiVar41 = (int **)0xffffffffffffffff;
                  do {
                    ppiVar41 = (int **)((longlong)ppiVar41 + 1);
                  } while (*(char *)((longlong)ppiVar34 + (longlong)ppiVar41) != '\0');
                }
              }
              else {
                FUN_142e54290(0x90,piVar18[1],(ulonglong)ppiVar40 & 0xffffffff);
                *piVar18 = 1;
LAB_142df0f44:
                *(char *)((longlong)ppiVar34 + (longlong)local_438) = '\0';
                ppiVar41 = ppiVar40;
              }
              iVar10 = (int)ppiVar41;
              if ((iVar10 < 0) || (piVar18[1] + 1 <= iVar10)) {
                FUN_142e54290(0x9c,(ulonglong)ppiVar41 & 0xffffffff);
              }
              piVar18[2] = iVar10;
              if (local_428 != (int **)0x0) {
                FUN_14019f2c0(local_428 + -2);
              }
            }
            else {
              if (*(int *)ppiVar40 < 1) {
                FUN_142e52dd0(0xd2);
              }
              LOCK();
              *(int *)ppiVar40 = *(int *)ppiVar40 + 1;
              UNLOCK();
              piVar19 = local_460;
              ppiVar36 = local_418;
              ppiVar33 = local_350;
              ppiVar34 = ppiVar35;
              if (local_428 != (int **)0x0) {
                FUN_14019f2c0(local_428 + -2);
                piVar19 = local_460;
                ppiVar36 = local_418;
                ppiVar33 = local_350;
              }
            }
          }
          local_428 = ppiVar34;
          local_3a8 = 0;
          FUN_14019a260(&local_3a8,local_3e8);
          FUN_14180df80(ppiVar33,&local_3a8,&local_428,0);
          ppiVar41 = (int **)0x0;
          local_310 = ppiVar33;
          if (ppiVar33 != (int **)0x0) {
            if ((int *)0xfffff < ppiVar33[4]) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            ppiVar33[4] = (int *)((longlong)ppiVar33[4] + 1);
            UNLOCK();
            piVar19 = local_460;
            ppiVar36 = local_418;
            ppiVar41 = local_350;
          }
          FUN_142d97880(local_3c0,local_318);
          local_3c0 = 0;
          puVar24 = (undefined8 *)FUN_1408a9e40(&local_380,0x3ee);
          uVar25 = FUN_14019ba10(&local_3c0,*puVar24,piVar19);
          FUN_1415eca30(uVar25,0xb);
          if (local_380 != 0) {
            FUN_14019f2c0(local_380 + -0x10);
          }
          if (local_3c0 != 0) {
            FUN_14019f2c0(local_3c0 + -0x10);
          }
          piVar18 = piVar19;
          ppiVar35 = ppiVar36;
          if (ppiVar41 != (int **)0x0) {
            if (0xffffe < (longlong)ppiVar41[4] - 1U) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            ppiVar41 = ppiVar41 + 4;
            piVar19 = *ppiVar41;
            *ppiVar41 = (int *)((longlong)*ppiVar41 + -1);
            UNLOCK();
            piVar18 = local_460;
            ppiVar35 = local_418;
            if (((int)piVar19 == 1) && (ppiVar41 = local_350 + 3, ppiVar41 != (int **)0x0)) {
              (**(code **)*ppiVar41)(ppiVar41,1);
              piVar18 = local_460;
              ppiVar35 = local_418;
            }
          }
        }
        else if ((((int)local_468 != (int)local_428) &&
                 ((int)local_428 == *(int *)(param_1 + 0x2260))) ||
                (((int)local_420 != 0 && ((int)local_428 == *(int *)(param_1 + 0x2260))))) {
          local_438 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x370);
          ppiVar33 = ppiVar41;
          if (local_438 != (undefined8 *)0x0) {
            ppiVar33 = (int **)FUN_141808b90(local_438);
          }
          ppiVar34 = ppiVar33 + 3;
          if (ppiVar33 == (int **)0x0) {
            ppiVar34 = ppiVar41;
          }
          if (ppiVar34 == (int **)0x0) {
            local_328 = (int **)0x0;
          }
          else {
            local_328 = ppiVar34 + -3;
            if (local_328 != (int **)0x0) {
              if ((int *)0xfffff < ppiVar34[1]) {
                FUN_142e541f0(0x30f);
              }
              LOCK();
              ppiVar34[1] = (int *)((longlong)ppiVar34[1] + 1);
              UNLOCK();
              piVar18 = local_460;
              ppiVar35 = local_418;
            }
          }
          ppiVar41 = local_328;
          if (local_328 == (int **)0x0) {
            FUN_142e52ed0(0x431,0);
          }
          local_438 = &local_3a0;
          local_3a0 = 0;
          FUN_14019a260(&local_3a0,&local_418);
          local_378[2] = 0;
          FUN_14019a260(local_378 + 2,local_3e8);
          FUN_14180df80(ppiVar41,local_378 + 2,&local_3a0,1);
          local_300 = ppiVar41;
          if (ppiVar41 != (int **)0x0) {
            if ((int *)0xfffff < ppiVar41[4]) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            ppiVar41[4] = (int *)((longlong)ppiVar41[4] + 1);
            UNLOCK();
            piVar18 = local_460;
            ppiVar41 = local_328;
            ppiVar35 = local_418;
          }
          FUN_142d97880(param_1,local_308);
          local_3b8 = 0;
          puVar24 = (undefined8 *)FUN_1408a9e40(local_378,0x3ee);
          uVar25 = FUN_14019ba10(&local_3b8,*puVar24,piVar18);
          FUN_1415eca30(uVar25,0xb);
          if (local_378[0] != 0) {
            FUN_14019f2c0(local_378[0] + -0x10);
          }
          if (local_3b8 != 0) {
            FUN_14019f2c0(local_3b8 + -0x10);
          }
          if (ppiVar41 != (int **)0x0) {
            if (0xffffe < (longlong)ppiVar41[4] - 1U) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            ppiVar41 = ppiVar41 + 4;
            piVar19 = *ppiVar41;
            *ppiVar41 = (int *)((longlong)*ppiVar41 + -1);
            UNLOCK();
            piVar18 = local_460;
            ppiVar35 = local_418;
            if (((int)piVar19 == 1) && (ppiVar41 = local_328 + 3, ppiVar41 != (int **)0x0)) {
              (**(code **)*ppiVar41)(ppiVar41,1);
              piVar18 = local_460;
              ppiVar35 = local_418;
            }
          }
        }
      }
LAB_142df12ed:
      FUN_1411bb5e0();
      FUN_1415aafa0(0x12);
      if (piVar18 != (int *)0x0) {
        FUN_14019f2c0(piVar18 + -4);
      }
      if (local_3e8[0] != (int *)0x0) {
        FUN_14019f2c0(local_3e8[0] + -4);
      }
      if (ppiVar35 != (int **)0x0) {
        FUN_14019f2c0(ppiVar35 + -2);
      }
    }
    piVar18 = local_478;
    if (local_478 == (int *)0x0) {
      return;
    }
    goto LAB_142df201d;
  case 0x2f:
    bVar7 = FUN_1406e8ae0(param_2);
    lVar23 = FUN_142cbe730(param_1);
    *(uint *)(lVar23 + 0x118b) = (uint)bVar7;
    local_4a8._0_4_ = 0;
    FUN_142cb1020(DAT_143aa84a0,7,0xffffffffffffffff,0);
    FUN_1411d0270(DAT_143ac9e90,0);
    local_468 = (int *)0x0;
    puVar24 = (undefined8 *)FUN_1408a9e40(&local_458,0x3ec);
    uVar25 = FUN_14019ba10(&local_468,*puVar24,5,100000);
    local_478 = (int *)0x0;
    FUN_14019a260(&local_478,uVar25);
    local_480 = 0;
    local_488 = 0;
    local_490 = 0;
    local_498 = 0;
    local_4a0 = 0;
    local_4a8 = (undefined8 *)((ulonglong)local_4a8._4_4_ << 0x20);
    FUN_142a26280(&local_478,0,0,1);
    piVar18 = local_468;
    if (local_458 != (int **)0x0) {
      FUN_14019f2c0(local_458 + -2);
      piVar18 = local_468;
    }
    goto LAB_142df2014;
  case 0x31:
    FUN_1406e9050(param_2,&local_470);
    uVar13 = (**(code **)(*DAT_143aa84a0 + 0xa8))();
    puVar24 = (undefined8 *)FUN_14025b440(0xc,uVar13,2);
    local_468 = (int *)0x0;
    uVar25 = FUN_14019ba10(&local_468,*puVar24,local_470);
    local_478 = (int *)0x0;
    FUN_14019a260(&local_478,uVar25);
    local_480 = 0;
    local_488 = 0;
    local_490 = 0;
    local_498 = 0;
    local_4a0 = 0;
    local_4a8 = (undefined8 *)((ulonglong)local_4a8._4_4_ << 0x20);
    FUN_142a26280(&local_478,0,0,1);
    piVar18 = local_470;
    if (local_468 != (int *)0x0) {
      FUN_14019f2c0(local_468 + -4);
      piVar18 = local_470;
    }
    goto LAB_142df2014;
  case 0x32:
    FUN_1406e9050(param_2,&local_470);
    piVar18 = local_470;
    local_468 = (int *)0x0;
    puVar24 = (undefined8 *)FUN_1408a9e40(&local_458,0x125d);
    uVar25 = FUN_14019ba10(&local_468,*puVar24,piVar18);
    local_478 = (int *)0x0;
    FUN_14019a260(&local_478,uVar25);
    if (local_458 != (int **)0x0) {
      FUN_14019f2c0(local_458 + -2);
    }
    if (local_468 != (int *)0x0) {
      FUN_14019f2c0(local_468 + -4);
    }
    FUN_1415eca30(&local_478,0xb);
    piVar18 = local_470;
    if (local_478 != (int *)0x0) {
      FUN_14019f2c0(local_478 + -4);
      piVar18 = local_470;
    }
    goto LAB_142df2014;
  case 0x33:
    FUN_1406e9050(param_2,&local_468);
    uVar13 = FUN_1406e8c20(param_2);
    local_478 = (int *)0x0;
    FUN_14019a260(&local_478,&local_468);
    local_490 = local_490 & 0xffffff00;
    local_498 = 0;
    local_4a0 = 0;
    local_4a8 = (undefined8 *)((ulonglong)local_4a8._4_4_ << 0x20);
    FUN_142d97e30(param_1,0xe,&local_478,uVar13);
    piVar18 = local_468;
    goto LAB_142df2014;
  case 0x34:
    uVar13 = FUN_1406e8c20(param_2);
    local_398 = (int *)0x0;
    puVar24 = (undefined8 *)FUN_1406e9050(param_2,&local_458);
    local_398 = (int *)*puVar24;
    *puVar24 = 0;
    if (local_458 != (int **)0x0) {
      FUN_14019f2c0(local_458 + -2);
    }
    local_448 = (int **)FUN_14019b780(&DAT_143ad68a0,0x370);
    lVar23 = 0;
    if (local_448 != (int **)0x0) {
      lVar23 = FUN_141808b90(local_448);
    }
    lVar16 = lVar23 + 0x18;
    if (lVar23 == 0) {
      lVar16 = 0;
    }
    if (lVar16 == 0) {
      piStack_400 = (int *)0x0;
    }
    else {
      piStack_400 = (int *)(lVar16 + -0x18);
      if (piStack_400 != (int *)0x0) {
        if (0xfffff < *(ulonglong *)(lVar16 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar16 + 8) = *(longlong *)(lVar16 + 8) + 1;
        UNLOCK();
      }
    }
    piVar19 = piStack_400;
    if (piStack_400 == (int *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    local_390 = 0;
    FUN_14019a260(&local_390,&local_398);
    FUN_14180ede0(piVar19,&local_390,uVar13);
    uStack_340 = piVar19;
    if (piVar19 != (int *)0x0) {
      if (0xfffff < *(ulonglong *)(piVar19 + 8)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(piVar19 + 8) = *(longlong *)(piVar19 + 8) + 1;
      UNLOCK();
      piVar19 = piStack_400;
    }
    FUN_142d97880(param_1,&local_348);
    piVar18 = local_398;
    if (piVar19 != (int *)0x0) {
      if (0xffffe < *(longlong *)(piVar19 + 8) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar20 = (longlong *)(piVar19 + 8);
      lVar23 = *plVar20;
      *plVar20 = *plVar20 + -1;
      UNLOCK();
      piVar18 = local_398;
      if (((int)lVar23 == 1) &&
         (piVar19 = piStack_400 + 6, piVar18 = local_398, piVar19 != (int *)0x0)) {
        (*(code *)**(undefined8 **)piVar19)(piVar19,1);
        piVar18 = local_398;
      }
    }
    goto LAB_142df2014;
  case 0x35:
    FUN_1406e9050(param_2,&local_470);
    piVar18 = local_470;
    local_468 = (int *)0x0;
    puVar24 = (undefined8 *)FUN_1408a9e40(&local_458,0x1262);
    uVar25 = FUN_14019ba10(&local_468,*puVar24,piVar18);
    local_478 = (int *)0x0;
    FUN_14019a260(&local_478,uVar25);
    if (local_458 != (int **)0x0) {
      FUN_14019f2c0(local_458 + -2);
    }
    if (local_468 != (int *)0x0) {
      FUN_14019f2c0(local_468 + -4);
    }
    FUN_1415eca30(&local_478,0xb);
    piVar18 = local_470;
    if (local_478 != (int *)0x0) {
      FUN_14019f2c0(local_478 + -4);
      piVar18 = local_470;
    }
    goto LAB_142df2014;
  case 0x36:
    FUN_1406e9050(param_2,&local_470);
    piVar18 = local_470;
    local_468 = (int *)0x0;
    puVar24 = (undefined8 *)FUN_1408a9e40(&local_458,0x125e);
    uVar25 = FUN_14019ba10(&local_468,*puVar24,piVar18);
    local_478 = (int *)0x0;
    FUN_14019a260(&local_478,uVar25);
    if (local_458 != (int **)0x0) {
      FUN_14019f2c0(local_458 + -2);
    }
    if (local_468 != (int *)0x0) {
      FUN_14019f2c0(local_468 + -4);
    }
    FUN_1415eca30(&local_478,0xb);
    piVar18 = local_470;
    if (local_478 != (int *)0x0) {
      FUN_14019f2c0(local_478 + -4);
      piVar18 = local_470;
    }
LAB_142df2014:
    if (piVar18 != (int *)0x0) {
LAB_142df201d:
      FUN_14019f2c0(piVar18 + -4);
    }
    goto switchD_142defda5_caseD_16;
  }
  uVar25 = FUN_1408a9e40(&local_458,uVar25);
  local_480 = 0;
  local_488 = 0;
  local_490 = 0;
  local_498 = 0;
  local_4a0 = 0;
  local_4a8 = (undefined8 *)((ulonglong)local_4a8 & 0xffffffff00000000);
  FUN_142a26280(uVar25,0,0,1);
switchD_142defda5_caseD_16:
  return;
LAB_142df030e:
  if ((int)uVar8 < 0) goto LAB_142df0332;
  FUN_142df3060(plVar20,uVar8);
  FUN_142df2fe0(plVar20 + 1,uVar8);
LAB_142df032d:
  FUN_142df2fe0(plVar20 + 2,uVar8);
LAB_142df0332:
  FUN_142deea90(param_1);
  FUN_1411bb5e0();
  return;
}



//===========================================================
// FUN_14180e3d0 @ 14180e3d0   (1274 bytes)
//===========================================================

void FUN_14180e3d0(longlong param_1,longlong *param_2,undefined4 param_3,undefined4 param_4,
                  undefined4 param_5,undefined4 param_6)

{
  wchar_t *pwVar1;
  longlong *plVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  int iVar5;
  int iVar6;
  longlong lVar7;
  int *piVar8;
  int *piVar9;
  int *piVar10;
  undefined8 uVar11;
  uint uVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  uint uVar15;
  undefined8 local_res8;
  int *piVar16;
  undefined4 uVar17;
  longlong *local_78;
  longlong *local_70;
  wchar_t *local_68;
  int *local_60;
  short local_58 [4];
  longlong local_50;
  
  *(undefined4 *)(param_1 + 0x300) = 0xe;
  FUN_14019a260(param_1 + 800);
  *(undefined4 *)(param_1 + 0x328) = param_3;
  *(undefined4 *)(param_1 + 0x338) = param_4;
  lVar7 = FUN_1402b0250(param_5,param_6);
  piVar9 = (int *)0x0;
  uVar13 = 0xffffffffffffffff;
  uVar14 = uVar13;
  if (lVar7 != 0) {
    do {
      uVar14 = uVar14 + 1;
    } while (*(char *)(lVar7 + uVar14) != '\0');
    iVar6 = (int)uVar14;
    iVar5 = 0;
    if (0 < iVar6) {
      iVar5 = iVar6;
    }
    piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar5 + 0x11));
    piVar8[1] = iVar5;
    *piVar8 = -1;
    piVar9 = piVar8 + 4;
    piVar8[2] = 0;
    *(undefined1 *)piVar9 = 0;
    FUN_142ef7ba0(piVar9,lVar7,(longlong)iVar6);
    if (*piVar8 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar6 == -1) || (iVar6 <= piVar8[1])) {
      *piVar8 = 1;
      if (iVar6 != -1) goto LAB_14180e4d0;
      uVar14 = uVar13;
      if (piVar9 != (int *)0x0) {
        do {
          uVar14 = uVar14 + 1;
        } while (*(char *)((longlong)piVar9 + uVar14) != '\0');
        goto LAB_14180e4d5;
      }
      iVar5 = 0;
    }
    else {
      FUN_142e54290(0x90,piVar8[1],uVar14 & 0xffffffff);
      *piVar8 = 1;
LAB_14180e4d0:
      *(undefined1 *)((longlong)iVar6 + (longlong)piVar9) = 0;
LAB_14180e4d5:
      iVar5 = (int)uVar14;
    }
    if ((iVar5 < 0) || (piVar8[1] + 1 <= iVar5)) {
      FUN_142e54290(0x9c,iVar5);
    }
    piVar8[2] = iVar5;
  }
  if (*(longlong *)(param_1 + 0x340) != 0) {
    FUN_14019f2c0(*(longlong *)(param_1 + 0x340) + -0x10);
  }
  *(int **)(param_1 + 0x340) = piVar9;
  *(undefined4 *)(param_1 + 0x2e0) = 0;
  *(undefined4 *)(param_1 + 0x2e4) = 0xad;
  *(undefined4 *)(param_1 + 0x2e8) = 0x2c;
  iVar5 = FUN_1410a4dc0();
  iVar6 = FUN_1410a4dd0();
  local_res8 = CONCAT44(iVar6 + DAT_143acee70 * -5 + -0x62,iVar5 / 2 + 0x17);
  piVar9 = (int *)FUN_1401bc720(&DAT_143ad6980,0x5a);
  piVar9[1] = 0x24;
  *piVar9 = -1;
  pwVar1 = (wchar_t *)(piVar9 + 4);
  piVar9[2] = 0;
  *pwVar1 = L'\0';
  uVar11 = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6960._8_8_;
  *(undefined8 *)pwVar1 = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6960._0_8_;
  *(undefined8 *)(piVar9 + 6) = uVar11;
  uVar11 = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6960._24_8_;
  *(undefined8 *)(piVar9 + 8) = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6960._16_8_;
  *(undefined8 *)(piVar9 + 10) = uVar11;
  uVar11 = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6960._40_8_;
  *(undefined8 *)(piVar9 + 0xc) = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6960._32_8_;
  *(undefined8 *)(piVar9 + 0xe) = uVar11;
  uVar4 = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6960._60_4_;
  uVar3 = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6960._56_4_;
  uVar17 = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6960._52_4_;
  piVar9[0x10] = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6960._48_4_;
  piVar9[0x11] = uVar17;
  piVar9[0x12] = uVar3;
  piVar9[0x13] = uVar4;
  *(undefined8 *)(piVar9 + 0x14) = u_UI_FadeYesNo_img_FadeYesNo_backg_1433d6960._64_8_;
  local_68 = pwVar1;
  if (*piVar9 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar9[1] < 0x24) {
    FUN_142e54290(0x90,piVar9[1],0x24);
  }
  *piVar9 = 1;
  *(undefined2 *)(piVar9 + 0x16) = 0;
  if (piVar9[1] + 1 < 0x25) {
    FUN_142e54290(0x9c);
  }
  piVar9[2] = 0x48;
  iVar5 = (*DAT_1432627f0)(0xfde9,0,L"UI/FadeYesNo.img/FadeYesNo/icon1",0xffffffff,0,0,0,0);
  uVar15 = iVar5 - 1;
  uVar12 = 0;
  if (0 < (int)uVar15) {
    uVar12 = uVar15;
  }
  piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(int)(uVar12 + 0x11));
  piVar10[1] = uVar12;
  *piVar10 = -1;
  piVar8 = piVar10 + 4;
  piVar10[2] = 0;
  *(undefined1 *)piVar8 = 0;
  uVar14 = 0;
  piVar16 = piVar8;
  local_60 = piVar8;
  (*DAT_1432627f0)(0xfde9,0,L"UI/FadeYesNo.img/FadeYesNo/icon1",0xffffffff,piVar8,iVar5,0,0);
  uVar17 = (undefined4)((ulonglong)piVar16 >> 0x20);
  if (*piVar10 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((uVar15 != 0xffffffff) && (piVar10[1] < (int)uVar15)) {
    FUN_142e54290(0x90,piVar10[1],uVar15);
  }
  *piVar10 = 1;
  if (uVar15 == 0xffffffff) {
    if (piVar8 == (int *)0x0) {
      iVar5 = 0;
      goto LAB_14180e73e;
    }
    do {
      uVar13 = uVar13 + 1;
    } while (*(char *)((longlong)piVar8 + uVar13) != '\0');
  }
  else {
    *(undefined1 *)((longlong)(int)uVar15 + (longlong)piVar8) = 0;
    uVar13 = (ulonglong)uVar15;
  }
  iVar5 = (int)uVar13;
LAB_14180e73e:
  if ((iVar5 < 0) || (piVar10[1] + 1 <= iVar5)) {
    FUN_142e54290(0x9c,iVar5);
  }
  piVar10[2] = iVar5;
  uVar11 = FUN_14090df00(local_58,piVar8);
  uVar11 = FUN_1409339d0(&local_70,uVar11);
  FUN_1403ee040(&local_78,uVar11);
  plVar2 = *(longlong **)(param_1 + 0x308);
  if (plVar2 != local_78) {
    *(longlong **)(param_1 + 0x308) = local_78;
    local_78 = (longlong *)0x0;
    if (plVar2 != (longlong *)0x0) {
      (**(code **)(*plVar2 + 0x10))();
      local_78 = (longlong *)0x0;
    }
  }
  if (local_78 != (longlong *)0x0) {
    (**(code **)(*local_78 + 0x10))(local_78);
  }
  if (local_70 != (longlong *)0x0) {
    (**(code **)(*local_70 + 0x10))();
  }
  if (local_58[0] == 8) {
    local_58[0] = 0;
    if (local_50 != 0) {
      (*DAT_143ad5990)(local_50 + -4);
    }
  }
  else {
    (*DAT_143262a18)(local_58);
  }
  *(undefined4 *)(param_1 + 0x298) = 0;
  *(undefined8 *)(param_1 + 0x29c) = 0xff;
  *(undefined8 *)(param_1 + 0x2b0) = local_res8;
  *(undefined8 *)(param_1 + 0x2b8) = local_res8;
  *(undefined8 *)(param_1 + 0x2c0) = local_res8;
  *(undefined4 *)(param_1 + 0x2a4) = 1000;
  *(undefined4 *)(param_1 + 0x2a8) = 180000;
  *(undefined4 *)(param_1 + 0x2ac) = 1000;
  FUN_1418060c0(param_1,0xad,0x2c,pwVar1,CONCAT44(uVar17,0x271a),1,0,uVar14 & 0xffffffff00000000,0);
  FUN_14180cc00(param_1);
  FUN_14019f2c0(piVar10);
  FUN_1401bebb0(piVar9);
  if (*param_2 != 0) {
    FUN_14019f2c0(*param_2 + -0x10);
  }
  return;
}



//===========================================================
// FUN_141188700 @ 141188700   (971 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x0001411888b0) */

void FUN_141188700(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  int *piVar3;
  int iVar4;
  int *piVar5;
  longlong lVar6;
  int *piVar7;
  int iVar8;
  undefined8 *puVar9;
  undefined8 *puVar10;
  undefined8 *puVar11;
  undefined1 auStack_4e8 [32];
  int *local_4c8;
  undefined4 local_4c0 [2];
  int *local_4b8;
  undefined1 local_4b0 [8];
  longlong local_4a8;
  int *local_4a0;
  int **local_498;
  undefined1 local_488 [1104];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_4e8;
  FUN_141ad7ec0(param_1 + 0x2e8,local_4b0,L"chatinput");
  lVar2 = local_4a8;
  puVar11 = (undefined8 *)0x0;
  puVar10 = (undefined8 *)0xffffffffffffffff;
  if ((local_4a8 == 0) || (iVar4 = (**(code **)(*(longlong *)(local_4a8 + 8) + 0x78))(), iVar4 == 0)
     ) goto LAB_141188a54;
  lVar6 = *(longlong *)(lVar2 + 0xc0);
  iVar4 = 0;
  if (lVar6 != 0) {
    iVar4 = *(int *)(lVar6 + -8);
  }
  local_4b8 = (int *)0x0;
  lVar6 = 0xc0;
  if (iVar4 < 1) {
    lVar6 = 200;
  }
  FUN_14019a260(&local_4b8,lVar6 + lVar2);
  piVar7 = local_4b8;
  if ((local_4b8 != (int *)0x0) && ((char)*local_4b8 != '\0')) {
    local_4c8 = (int *)0x0;
    piVar5 = local_4b8 + -4;
    piVar3 = local_4c8;
    if (piVar5 != (int *)0x0) {
      if (*piVar5 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        puVar9 = puVar10;
        do {
          puVar9 = (undefined8 *)((longlong)puVar9 + 1);
        } while (*(char *)((longlong)piVar7 + (longlong)puVar9) != '\0');
        iVar8 = (int)puVar9;
        iVar4 = 0;
        if (0 < iVar8) {
          iVar4 = iVar8;
        }
        piVar5 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
        piVar5[1] = iVar4;
        *piVar5 = -1;
        piVar3 = piVar5 + 4;
        piVar5[2] = 0;
        *(char *)piVar3 = '\0';
        local_4a0 = piVar3;
        FUN_142ef7ba0(piVar3,piVar7,(longlong)iVar8);
        if (*piVar5 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar8 == -1) || (iVar8 <= piVar5[1])) {
          *piVar5 = 1;
          if (iVar8 != -1) goto LAB_14118886c;
          puVar9 = puVar11;
          if (piVar3 != (int *)0x0) {
            do {
              puVar10 = (undefined8 *)((longlong)puVar10 + 1);
              puVar9 = puVar10;
            } while (*(char *)((longlong)piVar3 + (longlong)puVar10) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar5[1],(ulonglong)puVar9 & 0xffffffff);
          *piVar5 = 1;
LAB_14118886c:
          *(char *)((longlong)iVar8 + (longlong)piVar3) = '\0';
        }
        iVar4 = (int)puVar9;
        if ((iVar4 < 0) || (piVar5[1] + 1 <= iVar4)) {
          FUN_142e54290(0x9c,(ulonglong)puVar9 & 0xffffffff);
        }
        piVar5[2] = iVar4;
        if (local_4c8 != (int *)0x0) {
          FUN_14019f2c0(local_4c8 + -4);
        }
      }
      else {
        if (*piVar5 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar5 = *piVar5 + 1;
        UNLOCK();
        if (local_4c8 != (int *)0x0) {
          FUN_14019f2c0(local_4c8 + -4);
        }
        local_4c8 = piVar7;
        piVar7 = local_4b8;
        piVar3 = local_4c8;
      }
    }
    local_4c8 = piVar3;
    local_498 = &local_4c8;
    if ((((DAT_143a872a0 < 0) && (DAT_143aca618 == 0)) ||
        (iVar4 = FUN_1415c0460(&DAT_143aca848,&local_4c8,1), iVar4 == 0)) ||
       ((DAT_143aa84a0 == 0 || (iVar4 = FUN_142cc42d0(DAT_143aa84a0,500,0), iVar4 == 0)))) {
      if (local_4c8 != (int *)0x0) {
        FUN_14019f2c0(local_4c8 + -4);
      }
    }
    else {
      FUN_1406ed520(local_488,0x1fd);
      local_4c0[0] = 3;
      FUN_1406ede20(local_488,local_4c0,4);
      FUN_1406edc80(local_488,&local_4c8);
      FUN_1415d01c0(local_488);
      FUN_1406ed610(local_488);
      if (local_4c8 != (int *)0x0) {
        FUN_14019f2c0(local_4c8 + -4);
      }
      if (local_4a8 == 0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_1416a7070(local_4a8,&DAT_1434b2af1);
    }
    puVar10 = (undefined8 *)(local_4a8 + 8);
    if (local_4a8 == 0) {
      puVar10 = puVar11;
    }
    FUN_142c0bf50(DAT_143abfdf8,puVar10,0);
  }
  if (piVar7 != (int *)0x0) {
    FUN_14019f2c0(piVar7 + -4);
  }
LAB_141188a54:
  lVar2 = local_4a8;
  if (local_4a8 != 0) {
    if (0xffffe < *(longlong *)(local_4a8 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + 0x20);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar2 == 1) {
      puVar10 = (undefined8 *)(local_4a8 + 0x18);
      if (local_4a8 == 0) {
        puVar10 = puVar11;
      }
      if (puVar10 != (undefined8 *)0x0) {
        (**(code **)*puVar10)(puVar10,1);
      }
    }
  }
  return;
}



//===========================================================
// FUN_141183ec0 @ 141183ec0   (1136 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x00014118426f) */

void FUN_141183ec0(undefined8 param_1)

{
  int *piVar1;
  longlong lVar2;
  int *piVar3;
  char cVar4;
  int iVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  int *piVar9;
  int *piVar10;
  int iVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  int *local_res10;
  ulonglong local_res18;
  int *local_res20;
  longlong local_68;
  int *local_60;
  longlong local_58;
  undefined4 local_50 [2];
  longlong local_48;
  longlong lStack_40;
  
  lVar2 = DAT_143aa84a0;
  if (DAT_143aa84a0 == 0) {
    return;
  }
  iVar5 = FUN_1406e8c20();
  FUN_1406e9170(param_1,&local_res10,4);
  uVar13 = 0;
  uVar15 = 0;
  local_60 = (int *)0x0;
  uVar14 = 0xffffffffffffffff;
  uVar12 = uVar13;
  switch(local_res10._0_4_) {
  case 0:
    FUN_1406e9170(param_1,&local_res18,4);
    uVar12 = local_res18 & 0xffffffff;
    if ((int)local_res18 == 0) {
LAB_141183f50:
      DAT_143a872a0 = iVar5;
      FUN_1411574a0(0x1b);
      FUN_1411571c0(0x1b,1);
    }
    goto LAB_141183f6d;
  case 1:
    FUN_1406e9170(param_1,&local_res18,4);
    uVar12 = local_res18 & 0xffffffff;
  case 2:
    FUN_141183cc0();
    FUN_1411574a0(0x1b);
    FUN_142cc4430(lVar2,0);
    break;
  case 3:
    uVar12 = uVar15;
    if (iVar5 == DAT_143a872a0) {
      local_50[0] = 0xffffffff;
      local_48 = 0;
      lStack_40 = 0;
      FUN_140426220(local_50,param_1);
      FUN_141183b20(local_50);
      if (lStack_40 != 0) {
        FUN_14019f2c0(lStack_40 + -0x10);
      }
      if (local_48 != 0) {
        FUN_14019f2c0(local_48 + -0x10);
      }
    }
    break;
  case 4:
    FUN_141184360(param_1);
    uVar12 = uVar15;
    break;
  case 5:
    FUN_1406e9170(param_1,&local_res18,4);
    uVar12 = local_res18 & 0xffffffff;
    FUN_142cc4430(lVar2,0);
    break;
  case 6:
    cVar4 = FUN_1406e8ae0(param_1);
    uVar6 = FUN_1406e8c20(param_1);
    FUN_1406e9050(param_1,&local_68);
    local_res18 = 0;
    FUN_14019a260(&local_res18,&local_68);
    FUN_1411838e0(iVar5,cVar4 != '\0',uVar6,&local_res18);
    uVar12 = uVar15;
    if (local_68 != 0) {
      FUN_14019f2c0(local_68 + -0x10);
    }
    break;
  case 7:
    FUN_1406e9170(param_1,&local_res18,4);
    uVar12 = local_res18 & 0xffffffff;
    if ((int)local_res18 == 0) {
      FUN_141183cc0();
      goto LAB_141183f50;
    }
LAB_141183f6d:
    FUN_142cc4430(lVar2,0);
    break;
  case 8:
    uVar12 = uVar15;
    if (iVar5 == DAT_143a872a0) {
      FUN_1406e9050(param_1,&local_res18);
      uVar12 = local_res18;
      local_50[0] = 0xffffffff;
      local_48 = 0;
      lStack_40 = 0;
      uVar7 = FUN_1408a9f20("SID_MAPLECHAT_DECLINED");
      FUN_14019ba10(&lStack_40,uVar7,uVar12);
      FUN_141183b20(local_50);
      if (lStack_40 != 0) {
        FUN_14019f2c0(lStack_40 + -0x10);
      }
      if (local_48 != 0) {
        FUN_14019f2c0(local_48 + -0x10);
      }
      uVar12 = uVar15;
      if (local_res18 != 0) {
        FUN_14019f2c0(local_res18 - 0x10);
      }
    }
  }
  puVar8 = (undefined8 *)FUN_141189c50(&local_58,uVar12);
  piVar1 = (int *)*puVar8;
  *puVar8 = 0;
  local_60 = piVar1;
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  piVar10 = piVar1;
  if ((piVar1 == (int *)0x0) || ((char)*piVar1 == '\0')) goto LAB_141184306;
  local_res20 = (int *)0x0;
  piVar9 = piVar1 + -4;
  piVar3 = local_res20;
  if (piVar9 != (int *)0x0) {
    if (*piVar9 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar12 = uVar14;
      do {
        uVar12 = uVar12 + 1;
      } while (*(char *)((longlong)piVar1 + uVar12) != '\0');
      iVar11 = (int)uVar12;
      iVar5 = 0;
      if (0 < iVar11) {
        iVar5 = iVar11;
      }
      piVar9 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar5 + 0x11));
      piVar9[1] = iVar5;
      *piVar9 = -1;
      piVar3 = piVar9 + 4;
      piVar9[2] = 0;
      *(char *)piVar3 = '\0';
      local_res10 = piVar3;
      FUN_142ef7ba0(piVar3,piVar1,(longlong)iVar11);
      if (*piVar9 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar11 == -1) || (iVar11 <= piVar9[1])) {
        *piVar9 = 1;
        if (iVar11 != -1) goto LAB_14118422d;
        if (piVar3 != (int *)0x0) {
          do {
            uVar14 = uVar14 + 1;
          } while (*(char *)((longlong)piVar3 + uVar14) != '\0');
          uVar13 = uVar14 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar9[1],uVar12 & 0xffffffff);
        *piVar9 = 1;
LAB_14118422d:
        *(char *)((longlong)iVar11 + (longlong)piVar3) = '\0';
        uVar13 = uVar12;
      }
      iVar5 = (int)uVar13;
      if ((iVar5 < 0) || (piVar9[1] + 1 <= iVar5)) {
        FUN_142e54290(0x9c,uVar13 & 0xffffffff);
      }
      piVar9[2] = iVar5;
      if (local_res20 != (int *)0x0) {
        FUN_14019f2c0(local_res20 + -4);
      }
    }
    else {
      if (*piVar9 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar9 = *piVar9 + 1;
      UNLOCK();
      piVar10 = local_60;
      piVar3 = piVar1;
      if (local_res20 != (int *)0x0) {
        FUN_14019f2c0(local_res20 + -4);
        piVar10 = local_60;
      }
    }
  }
  local_res20 = piVar3;
  FUN_142a26280(&local_res20,0,0,1,0,0,0,0,0,0);
LAB_141184306:
  if (piVar10 != (int *)0x0) {
    FUN_14019f2c0(piVar10 + -4);
  }
  return;
}


