
//===========================================================
// FUN_1429ba3e0 @ 1429ba3e0   (1429 bytes)
//===========================================================

void FUN_1429ba3e0(longlong param_1,undefined8 param_2)

{
  undefined8 *puVar1;
  char cVar2;
  undefined4 uVar3;
  uint uVar4;
  int iVar5;
  uint uVar6;
  int iVar7;
  longlong lVar8;
  undefined8 uVar9;
  longlong lVar10;
  longlong lVar11;
  undefined1 *puVar12;
  undefined8 *puVar13;
  undefined8 uVar14;
  longlong *plVar15;
  undefined ***local_res18;
  undefined8 local_res20;
  undefined1 local_c0 [8];
  longlong local_b8;
  undefined1 local_b0 [8];
  undefined8 *local_a8;
  undefined1 local_a0 [8];
  longlong local_98;
  undefined **local_90;
  undefined1 *local_88;
  undefined ***local_58;
  undefined1 local_50 [24];
  
  uVar3 = FUN_1406e8c20(param_2);
  local_90 = &PTR_LAB_143409208;
  local_88 = &LAB_140c93920;
  local_58 = &local_90;
  local_res18 = &local_90;
  uVar4 = FUN_1406e8c20(param_2);
  if (uVar4 == 0) {
    uVar4 = FUN_1406e8c20(param_2);
    if (uVar4 == (uVar4 / 0x1f) * 0x1f) goto LAB_1429ba481;
  }
  else if (uVar4 != (uVar4 / 0x1f) * 0x1f) goto LAB_1429ba481;
  if (local_58 == (undefined ***)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ed3024();
  }
  (*(code *)(*local_58)[2])();
LAB_1429ba481:
  if (local_58 != (undefined ***)0x0) {
    (*(code *)(*local_58)[4])();
  }
  local_res18 = (undefined ***)CONCAT44(local_res18._4_4_,uVar4);
  iVar5 = FUN_1406e8c20(param_2);
  uVar9 = DAT_143aa84a0;
  local_res20 = DAT_143aa84a0;
  uVar6 = FUN_142cb9550(DAT_143aa84a0);
  if (uVar4 == uVar6) {
    lVar8 = FUN_141892840();
    if (lVar8 != 0) {
      uVar9 = FUN_141892840();
      cVar2 = FUN_141883ea0(uVar9);
      if (cVar2 == '\0') {
        FUN_142d09590(DAT_143aa84a0,0);
      }
    }
  }
  else {
    if (*(longlong *)(param_1 + 0xf8) != 0) {
      for (lVar8 = *(longlong *)
                    (*(longlong *)(param_1 + 0xf8) +
                    ((ulonglong)uVar4 % (ulonglong)*(uint *)(param_1 + 0x100)) * 8); lVar8 != 0;
          lVar8 = *(longlong *)(lVar8 + 8)) {
        if (*(uint *)(lVar8 + 0x10) == uVar4) {
          if (lVar8 != -0x18) {
            return;
          }
          break;
        }
      }
    }
    iVar7 = FUN_142cc3d80(uVar9);
    if (iVar7 == 0) {
      lVar8 = FUN_141892840();
      if (lVar8 != 0) {
        uVar9 = FUN_141892840();
        cVar2 = FUN_141bc8c60(uVar9);
        if (cVar2 != '\0') {
          return;
        }
      }
      lVar8 = FUN_141892840();
      if (lVar8 != 0) {
        uVar9 = FUN_141892840();
        cVar2 = FUN_141bc8c80(uVar9);
        if (cVar2 != '\0') {
          iVar7 = thunk_FUN_1413b8e00(DAT_143aa84a0);
          if (iVar7 == 0) {
            return;
          }
          iVar7 = FUN_142dec860(DAT_143aa84a0,uVar4);
          if (iVar7 == 0) {
            return;
          }
        }
      }
      lVar8 = FUN_141892840();
      if (lVar8 != 0) {
        uVar9 = FUN_141892840();
        cVar2 = FUN_141bc8ca0(uVar9);
        if (cVar2 != '\0') {
          iVar7 = FUN_142cc0400(DAT_143aa84a0);
          if (iVar7 == 0) {
            return;
          }
          if (iVar7 != iVar5) {
            return;
          }
        }
      }
      local_a8 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x40);
      lVar8 = 0;
      if (local_a8 == (undefined8 *)0x0) {
        local_a8 = (undefined8 *)0x0;
      }
      else {
        local_a8[3] = 0;
        local_a8[1] = 0;
        local_a8[2] = 0;
        *local_a8 = &PTR_FUN_143486628;
        local_a8[5] = 0;
        if (local_a8 != (undefined8 *)0x0) {
          LOCK();
          local_a8[1] = local_a8[1] + 1;
          UNLOCK();
        }
      }
      puVar1 = local_a8;
      lVar10 = FUN_14019b780(&DAT_143ad68a0,0x4438);
      lVar11 = lVar8;
      if (lVar10 != 0) {
        lVar11 = FUN_1429cdb70(lVar10,uVar3,uVar4);
      }
      if (puVar1 == (undefined8 *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      if ((puVar1[5] - 1 < 999) || (puVar1[5] == -1)) {
        FUN_142e52ed0(0x447);
      }
      local_b8 = lVar8;
      if ((lVar11 != 0) && (lVar11 != -0x10)) {
        local_b8 = lVar11;
        if (0xfffff < *(ulonglong *)(lVar11 + 0x18)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar11 + 0x18) = *(longlong *)(lVar11 + 0x18) + 1;
        UNLOCK();
      }
      uVar9 = puVar1[5];
      puVar1[5] = local_b8;
      local_b8 = uVar9;
      FUN_1429623c0(local_c0);
      puVar12 = (undefined1 *)FUN_1429be040(param_1 + 200);
      if ((*(longlong *)(puVar12 + 8) - 1U < 999) || (*(longlong *)(puVar12 + 8) == -1)) {
        FUN_142e52ed0(0x447);
      }
      if (puVar12 == local_b0) {
        FUN_142e52d50(0x45c,1);
      }
      if (puVar1 == (undefined8 *)0x0) {
        FUN_1429c1190(puVar12);
        *(undefined8 *)(puVar12 + 8) = 0;
        FUN_142e52ed0(0x431,0);
      }
      else {
        if (0xfffff < (ulonglong)puVar1[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar1[1] = puVar1[1] + 1;
        UNLOCK();
        FUN_1429c1190(puVar12);
        *(undefined8 **)(puVar12 + 8) = puVar1;
      }
      puVar1[6] = puVar12;
      FUN_1429be2e0(param_1 + 0xf8,&local_res18,local_b0);
      puVar13 = (undefined8 *)FUN_142956890(param_1 + 0x38);
      if ((puVar13[1] - 1 < 999) || (puVar13[1] == -1)) {
        FUN_142e52ed0(0x447);
      }
      if (puVar13 == puVar1 + 4) {
        FUN_142e52d50(0x45c,1);
      }
      lVar8 = puVar1[5];
      if (lVar8 != 0) {
        if (0xfffff < *(ulonglong *)(lVar8 + 0x18)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar8 + 0x18) = *(longlong *)(lVar8 + 0x18) + 1;
        UNLOCK();
      }
      FUN_1429623c0(puVar13);
      puVar13[1] = puVar1[5];
      puVar1[7] = puVar13;
      uVar14 = FUN_1429543c0(puVar1 + 4);
      uVar9 = local_res20;
      uVar3 = FUN_142dec860(local_res20,uVar4);
      FUN_1429ce270(uVar14,param_2,uVar3,1);
      lVar8 = FUN_141892840();
      if (lVar8 != 0) {
        plVar15 = (longlong *)FUN_141892840();
        (**(code **)(*plVar15 + 0x78))(plVar15,puVar1[5],param_2);
      }
      if (DAT_143aca960 != 0) {
        FUN_14118f5d0();
      }
      iVar5 = FUN_142dec860(uVar9,uVar4);
      if ((iVar5 != 0) && (*(longlong *)(DAT_143ac1b90 + 0x10) != 0)) {
        FUN_1428deee0(*(longlong *)(DAT_143ac1b90 + 0x10),uVar4,1);
      }
      lVar8 = FUN_141892840();
      if (lVar8 != 0) {
        plVar15 = (longlong *)FUN_141892840();
        (**(code **)(*plVar15 + 0x118))(plVar15,puVar1[5]);
        uVar9 = FUN_141892840();
        FUN_1418826c0(uVar9,local_a0);
        if (local_98 != 0) {
          FUN_140804f50(local_98 + 0x48,local_50,&local_res18);
        }
        FUN_1418a2560(local_a0);
      }
      FUN_142ca5110((ulonglong)local_res18 & 0xffffffff);
      if (0xffffe < puVar1[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar15 = puVar1 + 1;
      lVar8 = *plVar15;
      *plVar15 = *plVar15 + -1;
      UNLOCK();
      if ((int)lVar8 == 1) {
        (**(code **)*puVar1)(puVar1,1);
      }
    }
  }
  return;
}



//===========================================================
// FUN_1429ba980 @ 1429ba980   (1568 bytes)
//===========================================================

ulonglong FUN_1429ba980(longlong param_1,undefined8 param_2)

{
  uint uVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  undefined4 *puVar4;
  undefined8 *puVar5;
  undefined8 *puVar6;
  undefined8 *puVar7;
  int iVar8;
  ulonglong uVar9;
  longlong *plVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  undefined8 uVar14;
  undefined4 *puVar15;
  undefined8 *puVar16;
  int *piVar17;
  undefined8 *puVar18;
  int *piVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  uint local_res8 [2];
  undefined1 local_48 [8];
  undefined8 *local_40;
  undefined1 local_38 [8];
  undefined8 *local_30;
  
  uVar9 = FUN_1406e8c20(param_2);
  uVar21 = uVar9 & 0xffffffff;
  local_res8[0] = (uint)uVar9;
  if (*(longlong *)(param_1 + 0xf8) == 0) {
    return uVar9;
  }
  uVar20 = CONCAT44(0,*(uint *)(param_1 + 0x100));
  auVar2._8_8_ = 0;
  auVar2._0_8_ = uVar20;
  auVar3._8_8_ = 0;
  auVar3._0_8_ = uVar9 & 0xffffffff;
  lVar13 = *(longlong *)(*(longlong *)(param_1 + 0xf8) + SUB168(auVar3 % auVar2,0) * 8);
  while( true ) {
    if (lVar13 == 0) {
      return (uVar9 & 0xffffffff) / uVar20;
    }
    if (*(uint *)(lVar13 + 0x10) == local_res8[0]) break;
    lVar13 = *(longlong *)(lVar13 + 8);
  }
  local_30 = (undefined8 *)0x0;
  if (local_38 == (undefined1 *)(lVar13 + 0x18)) {
    FUN_142e52d50(0x45c,CONCAT71(SUB167(auVar3 % auVar2,1),1));
  }
  lVar11 = *(longlong *)(lVar13 + 0x20);
  if (lVar11 != 0) {
    if (0xfffff < *(ulonglong *)(lVar11 + 8)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar11 + 8) = *(longlong *)(lVar11 + 8) + 1;
    UNLOCK();
    uVar21 = (ulonglong)local_res8[0];
  }
  puVar18 = *(undefined8 **)(lVar13 + 0x20);
  local_30 = puVar18;
  if (puVar18 == (undefined8 *)0x0) {
    FUN_142e52ed0(0x431,0);
    FUN_142ca5190(0x20);
    FUN_142e52ed0(0x431,0);
  }
  else {
    FUN_142ca5190(puVar18 + 4);
  }
  lVar11 = puVar18[5];
  if (lVar11 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar11 = puVar18[5];
  }
  plVar10 = (longlong *)FUN_1427b2d90(lVar11);
  lVar11 = *plVar10;
  if (lVar11 != 0) {
    puVar15 = *(undefined4 **)(param_1 + 0x20);
    do {
      puVar4 = puVar15;
      if (puVar4 == (undefined4 *)0x0) goto LAB_1429bab48;
      uVar9 = *(ulonglong *)(puVar4 + -8);
      if ((uVar9 != 0) && (uVar9 < 0x10001)) {
        FUN_142e52ed0(0x33e);
        uVar9 = *(ulonglong *)(puVar4 + -8);
        lVar11 = *plVar10;
      }
      puVar15 = (undefined4 *)0x0;
      if (uVar9 != 0) {
        puVar15 = (undefined4 *)(uVar9 + 0x28);
      }
    } while ((*(longlong *)(puVar4 + 2) != lVar11) && (*(longlong *)(puVar4 + 4) != lVar11));
    lVar11 = FUN_1429b6c90(param_1,*puVar4);
    lVar12 = FUN_1429b6c90(param_1,puVar4[1]);
    if ((lVar11 != 0) && (lVar12 != 0)) {
      FUN_1427b1d60(lVar11,0,lVar12,puVar4[6]);
    }
    FUN_1429bf430(param_1 + 0x18,puVar4);
  }
LAB_1429bab48:
  if (puVar18 == (undefined8 *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  lVar11 = puVar18[5];
  if (lVar11 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar11 = puVar18[5];
  }
  plVar10 = (longlong *)FUN_1427b4c60(lVar11);
  lVar11 = *plVar10;
  if (lVar11 != 0) {
    puVar15 = *(undefined4 **)(param_1 + 0x58);
    do {
      puVar4 = puVar15;
      if (puVar4 == (undefined4 *)0x0) goto LAB_1429bac1b;
      uVar9 = *(ulonglong *)(puVar4 + -8);
      if ((uVar9 != 0) && (uVar9 < 0x10001)) {
        FUN_142e52ed0(0x33e);
        uVar9 = *(ulonglong *)(puVar4 + -8);
        lVar11 = *plVar10;
      }
      puVar15 = (undefined4 *)0x0;
      if (uVar9 != 0) {
        puVar15 = (undefined4 *)(uVar9 + 0x28);
      }
    } while ((*(longlong *)(puVar4 + 2) != lVar11) && (*(longlong *)(puVar4 + 4) != lVar11));
    lVar11 = FUN_1429b6c90(param_1,*puVar4);
    lVar12 = FUN_1429b6c90(param_1,puVar4[1]);
    if ((lVar11 != 0) && (lVar12 != 0)) {
      FUN_1427b3cb0(lVar11,0,lVar12,puVar4[6]);
    }
    FUN_1429bfab0(param_1 + 0x50,puVar4);
  }
LAB_1429bac1b:
  if (puVar18 == (undefined8 *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  if (puVar18[5] == 0) {
    FUN_142e52ed0(0x431,0);
  }
  iVar8 = FUN_142835920();
  puVar15 = *(undefined4 **)(param_1 + 0x70);
  do {
    puVar4 = puVar15;
    if (puVar4 == (undefined4 *)0x0) goto LAB_1429bace5;
    uVar9 = *(ulonglong *)(puVar4 + -8);
    if ((uVar9 != 0) && (uVar9 < 0x10001)) {
      FUN_142e52ed0();
      uVar9 = *(ulonglong *)(puVar4 + -8);
    }
    puVar15 = (undefined4 *)0x0;
    if (uVar9 != 0) {
      puVar15 = (undefined4 *)(uVar9 + 0x28);
    }
  } while ((puVar4[2] != iVar8) && (puVar4[3] != iVar8));
  lVar11 = FUN_1429b6c90(param_1,*puVar4);
  lVar12 = FUN_1429b6c90(param_1,puVar4[1]);
  if ((lVar11 != 0) && (lVar12 != 0)) {
    FUN_1427b4c70(lVar11,0,lVar12,0x10faf8);
  }
  FUN_1429bf770(param_1 + 0x68,puVar4);
LAB_1429bace5:
  if (puVar18 == (undefined8 *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  lVar11 = puVar18[5];
  if (lVar11 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar11 = puVar18[5];
  }
  iVar8 = FUN_14276df20(lVar11);
  if (iVar8 != 0) {
    piVar19 = *(int **)(param_1 + 0x88);
    do {
      do {
        piVar17 = piVar19;
        if (piVar17 == (int *)0x0) goto LAB_1429badb3;
        uVar9 = *(ulonglong *)(piVar17 + -8);
        if ((uVar9 != 0) && (uVar9 < 0x10001)) {
          FUN_142e52ed0(0x33e);
          uVar9 = *(ulonglong *)(piVar17 + -8);
        }
        piVar19 = (int *)0x0;
        if (uVar9 != 0) {
          piVar19 = (int *)(uVar9 + 0x28);
        }
      } while ((*piVar17 != iVar8) && (piVar17[1] != iVar8));
      lVar11 = FUN_1429b6c90(param_1);
      lVar12 = FUN_1429b6c90(param_1,piVar17[1]);
    } while ((lVar11 == 0) || (lVar12 == 0));
    FUN_1427b2da0(lVar11,0,lVar12,0);
    FUN_1429c0130(param_1 + 0x80,piVar17);
  }
LAB_1429badb3:
  FUN_1429b9f20(param_1,uVar21);
  if (puVar18 == (undefined8 *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  FUN_1429c0470(param_1 + 200);
  lVar11 = *(longlong *)(param_1 + 0xf8);
  if (lVar11 == 0) goto LAB_1429bae5b;
  uVar1 = *(uint *)(lVar13 + 0x10);
  uVar9 = (ulonglong)uVar1 % (ulonglong)*(uint *)(param_1 + 0x100);
  puVar16 = *(undefined8 **)(lVar11 + uVar9 * 8);
  if (puVar16 == (undefined8 *)0x0) goto LAB_1429bae5b;
  puVar6 = puVar16;
  puVar7 = (undefined8 *)puVar16[1];
  if (*(uint *)(puVar16 + 2) == uVar1) {
    *(undefined8 **)(lVar11 + uVar9 * 8) = (undefined8 *)puVar16[1];
LAB_1429bae43:
    (**(code **)*puVar16)(puVar16,1);
  }
  else {
    do {
      puVar16 = puVar7;
      puVar5 = puVar6;
      if (puVar16 == (undefined8 *)0x0) goto LAB_1429bae5b;
      puVar6 = puVar16;
      puVar7 = (undefined8 *)puVar16[1];
    } while (*(uint *)(puVar16 + 2) != uVar1);
    puVar5[1] = (undefined8 *)puVar16[1];
    if (puVar16 != (undefined8 *)0x0) goto LAB_1429bae43;
  }
  FUN_141d4d0d0(param_1 + 0x108,0);
LAB_1429bae5b:
  if (puVar18[7] != 0) {
    FUN_1429c07c0(param_1 + 0x38);
  }
  if (DAT_143aca960 != 0) {
    FUN_14118f5d0();
  }
  lVar13 = FUN_141892840();
  if (lVar13 != 0) {
    plVar10 = (longlong *)FUN_141892840();
    (**(code **)(*plVar10 + 0x120))(plVar10,puVar18[5]);
    uVar14 = FUN_141892840();
    FUN_1418826c0(uVar14,local_48);
    if (local_40 != (undefined8 *)0x0) {
      FUN_1418baf30(local_40 + 0xb,local_res8);
      if (local_40 == (undefined8 *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_1418baf30(local_40 + 9,local_res8);
    }
    puVar16 = local_40;
    if (local_40 != (undefined8 *)0x0) {
      if (0xffffe < local_40[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar10 = puVar16 + 1;
      lVar13 = *plVar10;
      *plVar10 = *plVar10 + -1;
      UNLOCK();
      puVar18 = local_30;
      if (((int)lVar13 == 1) && (local_40 != (undefined8 *)0x0)) {
        (**(code **)*local_40)(local_40,1);
        puVar18 = local_30;
      }
    }
  }
  uVar9 = puVar18[1] - 1;
  if (0xffffe < uVar9) {
    uVar9 = FUN_142e541f0(0x31e);
  }
  LOCK();
  plVar10 = puVar18 + 1;
  lVar13 = *plVar10;
  *plVar10 = *plVar10 + -1;
  UNLOCK();
  if ((int)lVar13 == 1) {
    uVar9 = (**(code **)*local_30)(local_30,1);
  }
  return uVar9;
}



//===========================================================
// FUN_1429ce270 @ 1429ce270   (8326 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1429ce270(longlong *param_1,undefined8 param_2,int param_3,undefined4 param_4)

{
  code *pcVar1;
  undefined8 *puVar2;
  bool bVar3;
  undefined8 *puVar4;
  undefined1 uVar5;
  byte bVar6;
  char cVar7;
  undefined2 uVar8;
  ushort uVar9;
  short sVar10;
  int iVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  int iVar14;
  int iVar15;
  longlong *plVar16;
  int *piVar17;
  undefined8 uVar18;
  longlong lVar19;
  longlong *plVar20;
  char *pcVar21;
  undefined4 *puVar22;
  undefined8 *puVar23;
  longlong lVar24;
  ulonglong uVar25;
  ulonglong uVar26;
  longlong *plVar27;
  undefined8 *puVar28;
  uint uVar29;
  ulonglong uVar30;
  char *pcVar31;
  longlong *plVar32;
  int iVar33;
  longlong **pplVar34;
  undefined1 auStack_518 [32];
  undefined8 local_4f8;
  int local_4f0;
  undefined4 local_4e8;
  longlong *local_4e0;
  uint local_4d8;
  uint local_4d4;
  longlong *local_4d0;
  int local_4c8;
  undefined1 local_4c4;
  ulonglong local_4c0;
  longlong *local_4b8;
  int local_4b0;
  int local_4ac;
  longlong *local_4a8;
  int *local_4a0;
  longlong *local_498;
  longlong *local_490;
  longlong *local_488;
  undefined8 local_480;
  uint uStack_478;
  undefined4 uStack_474;
  longlong *local_468;
  longlong *plStack_460;
  longlong *local_458;
  undefined8 local_450;
  longlong local_448;
  longlong *local_440;
  longlong *local_438;
  longlong *local_430;
  longlong local_428;
  longlong local_420;
  longlong local_418;
  longlong local_410;
  longlong local_408;
  undefined1 local_400 [8];
  longlong local_3f8;
  undefined1 local_3f0 [8];
  longlong *local_3e8;
  longlong *local_3e0;
  longlong local_3d8;
  longlong local_3d0;
  longlong local_3c8;
  longlong local_3c0;
  longlong local_3b8;
  longlong *local_3b0;
  longlong *local_3a8;
  longlong local_3a0;
  longlong *local_398;
  longlong *local_390;
  longlong *local_388;
  undefined8 local_380;
  longlong *local_378;
  undefined8 local_370;
  undefined8 local_368;
  longlong lStack_360;
  undefined1 local_358 [8];
  longlong local_350;
  longlong **local_348;
  undefined1 local_340 [24];
  undefined4 local_328;
  undefined4 uStack_324;
  uint uStack_320;
  undefined4 uStack_31c;
  undefined4 local_318;
  undefined4 uStack_314;
  uint uStack_310;
  undefined4 uStack_30c;
  undefined1 local_308 [8];
  longlong local_300;
  undefined1 local_2f8 [8];
  longlong local_2f0;
  longlong **local_2e8;
  undefined1 local_2e0 [33];
  undefined1 local_2bf;
  undefined **local_2b8;
  undefined8 local_2b0;
  undefined8 local_2a8;
  int *local_2a0;
  undefined1 local_298;
  undefined8 local_297;
  undefined8 local_28f;
  undefined8 local_287;
  undefined8 local_27f;
  undefined8 uStack_277;
  undefined8 local_26f;
  undefined8 uStack_267;
  undefined8 local_25f;
  undefined8 uStack_257;
  undefined8 local_24f;
  undefined8 uStack_247;
  undefined8 local_23f;
  undefined8 uStack_237;
  undefined8 local_22f;
  undefined8 uStack_227;
  undefined8 local_21f;
  undefined8 uStack_217;
  undefined8 local_20f;
  undefined8 uStack_207;
  undefined8 local_17f;
  undefined8 uStack_177;
  undefined8 local_16f;
  undefined8 uStack_167;
  undefined8 local_15f;
  undefined8 uStack_157;
  undefined8 local_14f;
  undefined8 uStack_147;
  undefined8 local_13f;
  undefined8 uStack_137;
  undefined8 local_12f;
  undefined8 uStack_127;
  undefined8 local_11f;
  undefined8 uStack_117;
  undefined8 local_10f;
  undefined8 uStack_107;
  undefined4 local_ff;
  undefined8 local_fb;
  undefined4 local_f3;
  undefined1 local_ef;
  undefined8 local_ee;
  undefined4 local_e6;
  undefined8 local_e2;
  undefined4 local_da;
  undefined1 local_d6;
  undefined1 local_c8 [128];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_518;
  plVar27 = (longlong *)0x0;
  iVar15 = 0;
  local_4d8 = 0;
  local_4d4 = 0;
  local_4c8 = param_3;
  if (DAT_143ac87a0 != 0) {
    FUN_1415ed8e0(local_2e0);
    iVar11 = (**(code **)(*param_1 + 0x50))(param_1);
    local_2bf = iVar11 != 0;
    FUN_1415f0bf0(DAT_143ac87a0,local_2e0);
    FUN_1415ed910(local_2e0);
  }
  *(undefined4 *)((longlong)param_1 + 0x6ac) = param_4;
  uVar12 = FUN_1406e8c20(param_2);
  *(undefined4 *)((longlong)param_1 + 0x406c) = uVar12;
  plVar16 = (longlong *)FUN_1406e9050(param_2,&local_3d8);
  if (param_1[0x21b] != 0) {
    FUN_14019f2c0(param_1[0x21b] + -0x10);
    param_1[0x21b] = 0;
  }
  param_1[0x21b] = *plVar16;
  *plVar16 = 0;
  if (local_3d8 != 0) {
    FUN_14019f2c0(local_3d8 + -0x10);
  }
  plVar16 = (longlong *)FUN_1406e9050(param_2,&local_3d0);
  if (param_1[0x21c] != 0) {
    FUN_14019f2c0(param_1[0x21c] + -0x10);
    param_1[0x21c] = 0;
  }
  param_1[0x21c] = *plVar16;
  *plVar16 = 0;
  if (local_3d0 != 0) {
    FUN_14019f2c0(local_3d0 + -0x10);
  }
  uVar12 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x21d) = uVar12;
  plVar16 = (longlong *)FUN_1406e9050(param_2,&local_3c8);
  if (param_1[0x21e] != 0) {
    FUN_14019f2c0(param_1[0x21e] + -0x10);
    param_1[0x21e] = 0;
  }
  param_1[0x21e] = *plVar16;
  *plVar16 = 0;
  if (local_3c8 != 0) {
    FUN_14019f2c0(local_3c8 + -0x10);
  }
  uVar8 = FUN_1406e8b80(param_2);
  *(undefined2 *)((longlong)param_1 + 0x10fe) = uVar8;
  uVar5 = FUN_1406e8ae0(param_2);
  *(undefined1 *)(param_1 + 0x220) = uVar5;
  uVar8 = FUN_1406e8b80(param_2);
  *(undefined2 *)((longlong)param_1 + 0x1102) = uVar8;
  uVar5 = FUN_1406e8ae0(param_2);
  *(undefined1 *)((longlong)param_1 + 0x1104) = uVar5;
  uVar12 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x221) = uVar12;
  uVar12 = FUN_1406e8c20(param_2);
  *(undefined4 *)((longlong)param_1 + 0x110c) = uVar12;
  uVar5 = FUN_1406e8ae0(param_2);
  *(undefined1 *)((longlong)param_1 + 0x10fc) = uVar5;
  uVar12 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x765) = uVar12;
  uVar12 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x6ee) = uVar12;
  bVar6 = FUN_1406e8ae0(param_2);
  *(uint *)((longlong)param_1 + 0x3774) = (uint)bVar6;
  uVar12 = FUN_1406e8c20(param_2);
  *(undefined4 *)((longlong)param_1 + 0x13b4) = uVar12;
  cVar7 = FUN_1406e8ae0(param_2);
  *(bool *)((longlong)param_1 + 0x4064) = cVar7 == '\x01';
  FUN_140a46e50(param_1[0x80e],local_c8,param_2);
  uVar9 = FUN_1406e8b80(param_2);
  *(uint *)(param_1 + 0x818) = (uint)uVar9;
  uVar9 = FUN_1406e8b80(param_2);
  *(uint *)((longlong)param_1 + 0x40c4) = (uint)uVar9;
  uVar12 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x86b) = uVar12;
  local_2a0 = (int *)0x0;
  local_2b0 = 0;
  local_2a8 = 0;
  local_2b8 = &PTR_FUN_14327d968;
  local_ee = 0;
  local_298 = 0;
  local_297 = 0;
  local_28f = 0;
  local_287 = 0;
  local_fb = 0;
  local_f3 = 0;
  local_ef = 0;
  local_27f = 0;
  uStack_277 = 0;
  local_26f = 0;
  uStack_267 = 0;
  local_25f = 0;
  uStack_257 = 0;
  local_24f = 0;
  uStack_247 = 0;
  local_23f = 0;
  uStack_237 = 0;
  local_22f = 0;
  uStack_227 = 0;
  local_21f = 0;
  uStack_217 = 0;
  local_20f = 0;
  uStack_207 = 0;
  local_ff = 0;
  local_17f = 0;
  uStack_177 = 0;
  local_16f = 0;
  uStack_167 = 0;
  local_15f = 0;
  uStack_157 = 0;
  local_14f = 0;
  uStack_147 = 0;
  local_13f = 0;
  uStack_137 = 0;
  local_12f = 0;
  uStack_127 = 0;
  local_11f = 0;
  uStack_117 = 0;
  local_10f = 0;
  uStack_107 = 0;
  local_e2 = 0;
  local_da = 0;
  local_d6 = 0;
  local_e6 = 0xffffffff;
  local_4a0 = (int *)0x0;
  piVar17 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
  piVar17[1] = 0;
  *piVar17 = -1;
  local_4a0 = piVar17 + 4;
  piVar17[2] = 0;
  *(undefined1 *)local_4a0 = 0;
  if (*piVar17 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar17[1] < 0) {
    FUN_142e54290(0x90,piVar17[1],0);
  }
  *piVar17 = 1;
  *(undefined1 *)local_4a0 = 0;
  if (piVar17[1] + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  piVar17[2] = 0;
  FUN_1402ee8d0(&local_2b8,param_2,&local_4a0,0);
  uVar12 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x24c) = uVar12;
  uVar12 = FUN_1406e8c20(param_2);
  *(undefined4 *)((longlong)param_1 + 0x1264) = uVar12;
  pcVar1 = *(code **)(*param_1 + 0x178);
  uVar5 = FUN_1406e8ae0(param_2);
  (*pcVar1)(param_1,uVar5);
  iVar11 = FUN_1406e8c20(param_2);
  uVar12 = FUN_1406e8c20(param_2);
  local_4ac = FUN_1406e8c20(param_2);
  cVar7 = FUN_1406e8ae0(param_2);
  if (cVar7 != '\0') {
    plVar16 = (longlong *)FUN_1406e9050(param_2,&local_3c0);
    if (param_1[0x5b5] != 0) {
      FUN_14019f2c0(param_1[0x5b5] + -0x10);
      param_1[0x5b5] = 0;
    }
    param_1[0x5b5] = *plVar16;
    *plVar16 = 0;
    if (local_3c0 != 0) {
      FUN_14019f2c0(local_3c0 + -0x10);
    }
  }
  uVar13 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x261) = uVar13;
  plVar16 = (longlong *)FUN_1406e9050(param_2,&local_3b8);
  if (param_1[0x262] != 0) {
    FUN_14019f2c0(param_1[0x262] + -0x10);
    param_1[0x262] = 0;
  }
  param_1[0x262] = *plVar16;
  *plVar16 = 0;
  if (local_3b8 != 0) {
    FUN_14019f2c0(local_3b8 + -0x10);
  }
  plVar16 = (longlong *)FUN_1406e9050(param_2,&local_3a0);
  if (param_1[0x263] != 0) {
    FUN_14019f2c0(param_1[0x263] + -0x10);
    param_1[0x263] = 0;
  }
  param_1[0x263] = *plVar16;
  *plVar16 = 0;
  if (local_3a0 != 0) {
    FUN_14019f2c0(local_3a0 + -0x10);
  }
  uVar13 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x5c1) = uVar13;
  sVar10 = thunk_FUN_1406e8b80(param_2);
  *(int *)(param_1 + 0x785) = (int)sVar10;
  uVar13 = FUN_1406e8c20(param_2);
  local_4b8 = (longlong *)CONCAT44(local_4b8._4_4_,uVar13);
  *(undefined4 *)(param_1 + 0x810) = uVar13;
  uVar13 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x76f) = uVar13;
  sVar10 = FUN_1406e8b80(param_2);
  local_4b0 = (int)sVar10;
  sVar10 = FUN_1406e8b80(param_2);
  local_4c0 = CONCAT44(local_4c0._4_4_,(int)sVar10);
  bVar6 = FUN_1406e8ae0(param_2);
  *(uint *)((longlong)param_1 + 0x6e4) = (uint)bVar6;
  uVar18 = DAT_143ac18d8;
  sVar10 = FUN_1406e8b80(param_2);
  local_490 = (longlong *)FUN_142df6c50(uVar18,(int)sVar10);
  cVar7 = FUN_1406e8ae0(param_2);
  FUN_141b0e070(param_1,cVar7 != '\0');
  cVar7 = FUN_1406e8ae0(param_2);
  FUN_141b0e080(param_1,cVar7 != '\0');
  local_488 = (longlong *)FUN_1409e0a60();
  plVar16 = plVar27;
  if (local_488 == (longlong *)0x0) {
    iVar14 = -0x7fffbffe;
  }
  else {
    local_458 = (longlong *)0x0;
    iVar14 = (**(code **)local_488[4])(local_488 + 4,&DAT_143273488,&local_458);
    if (-1 < iVar14) {
      plVar16 = local_458;
    }
  }
  if (((iVar14 + 0x80000000U & 0x80000000) == 0) && (iVar14 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0();
  }
  plVar20 = (longlong *)param_1[0x237];
  if ((plVar20 != plVar16) &&
     (param_1[0x237] = (longlong)plVar16, plVar16 = plVar27, plVar20 != (longlong *)0x0)) {
    (**(code **)(*plVar20 + 0x10))();
  }
  if (plVar16 != (longlong *)0x0) {
    (**(code **)(*plVar16 + 0x10))(plVar16);
  }
  plVar16 = local_488;
  FUN_1409526f0(local_488,param_1 + 1);
  local_4e0 = local_490;
  local_4e8 = *(undefined4 *)((longlong)param_1 + 0x6e4);
  local_4f0 = 0;
  local_4f8 = (longlong *)((ulonglong)local_4f8._4_4_ << 0x20);
  (**(code **)(*plVar16 + 0x118))(plVar16,0,local_4b0,local_4c0 & 0xffffffff);
  uVar18 = (**(code **)(param_1[1] + 0x50))();
  uVar13 = FUN_1409c6d00(uVar18);
  *(undefined4 *)((longlong)param_1 + 0x6e4) = uVar13;
  local_450 = 0;
  FUN_14276c240(param_1,&local_2b8,&local_450);
  uVar13 = FUN_1401ba9d0(param_1[0x80e] + 0x2e3c,*(undefined4 *)(param_1[0x80e] + 0x2e44));
  FUN_1428335a0(param_1,uVar13,1);
  uVar13 = FUN_1401ba9d0(param_1[0x80e] + 0xb08,*(undefined4 *)(param_1[0x80e] + 0xb10));
  FUN_1427b5ca0(param_1,uVar13,1);
  uVar13 = FUN_1401ba9d0(param_1[0x80e] + 0x1fd0,*(undefined4 *)(param_1[0x80e] + 0x1fd8));
  FUN_140f8d8d0(param_1 + 0x20,uVar13);
  if ((int)param_1[0x283] != iVar11) {
    *(int *)(param_1 + 0x283) = iVar11;
    FUN_1427eb600(param_1,iVar11);
  }
  lVar19 = FUN_141892840();
  if (lVar19 != 0) {
    uVar18 = FUN_141892840();
    iVar11 = FUN_1418833f0(uVar18);
    if (iVar11 != 0) {
      uVar12 = 0;
    }
  }
  FUN_14277cd40(param_1,uVar12);
  cVar7 = FUN_1406e8ae0(param_2);
  plVar16 = plVar27;
  if (cVar7 != '\0') {
    FUN_141712040(&local_428,param_1,(ulonglong)local_4b8 & 0xffffffff);
    if ((param_1[0x783] - 1U < 999) || (param_1[0x783] == -1)) {
      FUN_142e52ed0(0x447);
    }
    if (param_1 + 0x782 == &local_428) {
      FUN_142e52d50(0x45c,1);
    }
    lVar19 = local_420;
    if (local_420 != 0) {
      if (0xfffff < *(ulonglong *)(local_420 + 8)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar19 + 8) = *(longlong *)(lVar19 + 8) + 1;
      UNLOCK();
      plVar16 = (longlong *)(ulonglong)local_4d4;
      local_4d8 = local_4d4;
    }
    FUN_142962500(param_1 + 0x782);
    param_1[0x783] = local_420;
    if (local_420 == 0) {
      FUN_142e52ed0(0x431,param_1[0x783]);
    }
    (**(code **)(*(longlong *)param_1[0x783] + 0x18))((longlong *)param_1[0x783],param_2);
    iVar11 = FUN_140f8abc0(param_1 + 0x20);
    if (iVar11 == 0) {
      plVar20 = (longlong *)param_1[0x783];
      if (plVar20 == (longlong *)0x0) {
        FUN_142e52ed0(0x431,0);
        plVar20 = (longlong *)param_1[0x783];
      }
      (**(code **)(*plVar20 + 0x40))(plVar20,param_1);
    }
    FUN_142962500(&local_428);
  }
  if (((int)local_4b8 != 0) && ((int)param_1[0x810] == 0)) {
    uVar18 = FUN_141715d80(local_340,param_1[0x783],0);
    FUN_142952c10(param_1 + 0x782,uVar18);
    FUN_142962500(local_340);
  }
  uVar13 = FUN_1401ba9d0(param_1[0x80e] + 0xb2c,*(undefined4 *)(param_1[0x80e] + 0xb34));
  FUN_1427b5e80(param_1,uVar13,1);
  iVar11 = FUN_140f8abc0(param_1 + 0x20);
  if (iVar11 != 0) {
    *(undefined4 *)(param_1 + 0x5b9) = uVar12;
  }
  FUN_14277e0c0(param_1,local_4ac,0);
  cVar7 = FUN_1406e8ae0(param_2);
  while (cVar7 != '\0') {
    uVar12 = FUN_1406e8c20(param_2);
    local_4a8 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x668);
    plVar20 = plVar27;
    if (local_4a8 != (longlong *)0x0) {
      plVar20 = (longlong *)FUN_141eb62e0(local_4a8);
    }
    cVar7 = FUN_141eb9760(plVar20,param_1,uVar12,param_2);
    if (cVar7 != '\0') {
      FUN_1427707e0(param_1,uVar12,plVar20);
    }
    cVar7 = FUN_1406e8ae0(param_2);
  }
  cVar7 = FUN_1406e8ae0(param_2);
  while (cVar7 != '\0') {
    (**(code **)(*param_1 + 0x150))(param_1,param_2);
    cVar7 = FUN_1406e8ae0(param_2);
  }
  uVar12 = FUN_14087bc10(param_1[0x80e]);
  FUN_140f8a0b0(param_1 + 0x20,uVar12,0);
  uVar12 = FUN_1406e8c20(param_2);
  *(undefined4 *)((longlong)param_1 + 0x34c4) = uVar12;
  uVar12 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x699) = uVar12;
  uVar12 = FUN_1406e8c20(param_2);
  *(undefined4 *)((longlong)param_1 + 0x34cc) = uVar12;
  uVar12 = FUN_1427b5fe0(param_1);
  FUN_140f8aaf0(param_1 + 0x20,uVar12);
  cVar7 = FUN_1406e8ae0(param_2);
  if (cVar7 != '\0') {
    FUN_141775d90(&local_418,param_1,*(undefined4 *)((longlong)param_1 + 0x62c));
    if ((param_1[0x77a] - 1U < 999) || (param_1[0x77a] == -1)) {
      FUN_142e52ed0(0x447);
    }
    if (param_1 + 0x779 == &local_418) {
      FUN_142e52d50(0x45c,1);
    }
    lVar19 = local_410;
    if (local_410 != 0) {
      if (0xfffff < *(ulonglong *)(local_410 + 8)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar19 + 8) = *(longlong *)(lVar19 + 8) + 1;
      UNLOCK();
      plVar16 = (longlong *)(ulonglong)local_4d4;
      local_4d8 = local_4d4;
    }
    FUN_14287fca0(param_1 + 0x779);
    param_1[0x77a] = local_410;
    if (local_410 == 0) {
      FUN_142e52ed0(0x431,param_1[0x77a]);
    }
    (**(code **)(*(longlong *)param_1[0x77a] + 8))((longlong *)param_1[0x77a],param_2);
    plVar20 = (longlong *)param_1[0x77a];
    if (plVar20 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
      plVar20 = (longlong *)param_1[0x77a];
    }
    (**(code **)(*plVar20 + 0x10))(plVar20,param_1);
    FUN_14287fca0(&local_418);
  }
  FUN_1406e9170(param_2,&local_4ac,4);
  *(int *)(param_1 + 0x223) = local_4ac;
  if (local_4ac != 0) {
    uVar12 = FUN_1406e8c20(param_2);
    *(undefined4 *)(param_1 + 0x224) = uVar12;
    FUN_1406e9170(param_2,&local_4b0,4);
    *(int *)((longlong)param_1 + 0x111c) = local_4b0;
    plVar16 = (longlong *)FUN_1406e9050(param_2,&local_408);
    if (param_1[0x225] != 0) {
      FUN_14019f2c0(param_1[0x225] + -0x10);
      param_1[0x225] = 0;
    }
    param_1[0x225] = *plVar16;
    *plVar16 = 0;
    if (local_408 != 0) {
      FUN_14019f2c0(local_408 + -0x10);
    }
    uVar12 = FUN_1406e8c20(param_2);
    *(undefined4 *)((longlong)param_1 + 0x113c) = uVar12;
    bVar6 = FUN_1406e8ae0(param_2);
    *(uint *)(param_1 + 0x226) = (uint)bVar6;
    bVar6 = FUN_1406e8ae0(param_2);
    *(uint *)(param_1 + 0x227) = (uint)bVar6;
    bVar6 = FUN_1406e8ae0(param_2);
    *(uint *)((longlong)param_1 + 0x1134) = (uint)bVar6;
    bVar6 = FUN_1406e8ae0(param_2);
    *(uint *)(param_1 + 0x228) = (uint)bVar6;
    iVar11 = FUN_140f8abc0(param_1 + 0x20);
    if (iVar11 == 0) {
      lVar19 = param_1[8];
      uVar12 = (**(code **)(param_1[1] + 0x40))(param_1 + 1);
      uVar18 = (**(code **)(param_1[1] + 0x50))(param_1 + 1);
      uVar13 = FUN_1409c6cc0(uVar18);
      uVar18 = FUN_1409397b0(param_1 + 1,local_400);
      local_4f8 = (longlong *)CONCAT44(local_4f8._4_4_,uVar12);
      FUN_141594970(lVar19,param_1 + 0x223,uVar18,uVar13);
    }
    uVar18 = FUN_1408a9e40(&local_3f8,0x1422);
    lVar19 = param_1[0x225];
    plVar16 = plVar27;
    if (lVar19 != 0) {
      plVar16 = (longlong *)(ulonglong)*(uint *)(lVar19 + -8);
    }
    FUN_1401abc80(uVar18,&local_448,lVar19,plVar16);
    if (local_3f8 != 0) {
      FUN_14019f2c0(local_3f8 + -0x10);
    }
    local_468 = (longlong *)0x0;
    plStack_460 = (longlong *)0x0;
    plVar16 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x58);
    plVar20 = plVar27;
    local_4a8 = plVar16;
    if (plVar16 != (longlong *)0x0) {
      *plVar16 = 0;
      plVar16[1] = 0;
      *(int *)(plVar16 + 1) = 1;
      *(int *)((longlong)plVar16 + 0xc) = 1;
      *plVar16 = (longlong)&PTR_FUN_143300d78;
      FUN_1408d6690(plVar16 + 2);
      plVar20 = plVar16;
    }
    plVar32 = plStack_460;
    plVar16 = (longlong *)&DAT_00000028;
    local_4d8 = 0x28;
    local_4d4 = 0x28;
    local_468 = plVar20 + 2;
    if (plStack_460 != (longlong *)0x0) {
      LOCK();
      plVar16 = plStack_460 + 1;
      lVar19 = *plVar16;
      *(int *)plVar16 = (int)*plVar16 + -1;
      UNLOCK();
      if ((int)lVar19 == 1) {
        puVar2 = (undefined8 *)*plStack_460;
        plStack_460 = plVar20;
        (*(code *)*puVar2)(plVar32);
        LOCK();
        piVar17 = (int *)((longlong)plVar32 + 0xc);
        iVar11 = *piVar17;
        *piVar17 = *piVar17 + -1;
        UNLOCK();
        plVar20 = plStack_460;
        if (iVar11 == 1) {
          (**(code **)(*plVar32 + 8))(plVar32);
          plVar20 = plStack_460;
        }
      }
      plStack_460 = plVar20;
      plVar16 = (longlong *)(ulonglong)local_4d4;
      plVar20 = plStack_460;
    }
    plStack_460 = plVar20;
    local_4d8 = local_4d4;
    FUN_1408d6760(local_468,param_2);
    FUN_1415ed1c0(&local_468,&local_448,0x1f);
    plVar20 = plStack_460;
    if (plStack_460 != (longlong *)0x0) {
      LOCK();
      plVar16 = plStack_460 + 1;
      lVar19 = *plVar16;
      *(int *)plVar16 = (int)*plVar16 + -1;
      UNLOCK();
      if ((int)lVar19 == 1) {
        (**(code **)*plStack_460)(plStack_460);
        LOCK();
        piVar17 = (int *)((longlong)plVar20 + 0xc);
        iVar11 = *piVar17;
        *piVar17 = *piVar17 + -1;
        UNLOCK();
        if (iVar11 == 1) {
          (**(code **)(*plVar20 + 8))(plVar20);
        }
      }
      plVar16 = (longlong *)(ulonglong)local_4d4;
      local_4d8 = local_4d4;
    }
    if (local_448 != 0) {
      FUN_14019f2c0(local_448 + -0x10);
    }
  }
  FUN_14073a7b0(param_1 + 0x229,param_2);
  if (((int)param_1[0x229] != 0) && (iVar11 = FUN_140f8abc0(param_1 + 0x20), iVar11 == 0)) {
    lVar19 = param_1[8];
    uVar12 = (**(code **)(param_1[1] + 0x40))(param_1 + 1);
    uVar18 = (**(code **)(param_1[1] + 0x50))(param_1 + 1);
    uVar13 = FUN_1409c6cc0(uVar18);
    uVar18 = FUN_1409397b0(param_1 + 1,local_3f0);
    local_4f8 = (longlong *)CONCAT44(local_4f8._4_4_,uVar12);
    FUN_141595e60(lVar19,param_1 + 0x229,uVar18,uVar13);
  }
  bVar6 = FUN_1406e8ae0(param_2);
  *(uint *)(param_1 + 0x816) = (uint)bVar6;
  if (bVar6 != 0) {
    FUN_1406e9050(param_2,&local_4d0);
    if (((local_4d0 != (longlong *)0x0) && ((char)*local_4d0 != '\0')) &&
       (lVar19 = FUN_142ef83e0(&DAT_143273878,
                               (int)*(char *)((longlong)(int)local_4d0[-1] + -1 +
                                             (longlong)local_4d0)), lVar19 != 0)) {
      pcVar21 = (char *)FUN_14019bd40(&local_4d0,0,1);
      iVar11 = iVar15;
      if (local_4d0 != (longlong *)0x0) {
        iVar11 = (int)local_4d0[-1];
      }
      pcVar31 = pcVar21 + (longlong)iVar11 + -2;
      while( true ) {
        if (pcVar31 < pcVar21) goto LAB_1429cf205;
        lVar19 = FUN_142ef83e0(&DAT_143273878,(int)*pcVar31);
        plVar16 = local_4d0;
        if (lVar19 == 0) break;
        pcVar31 = pcVar31 + -1;
      }
      if (pcVar31 < pcVar21) goto LAB_1429cf205;
      pcVar31[1] = '\0';
      uVar29 = (int)(pcVar31 + 1) - (int)pcVar21;
      if ((int)local_4d0[-2] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((uVar29 != 0xffffffff) && (*(int *)((longlong)plVar16 + -0xc) < (int)uVar29)) {
        FUN_142e54290(0x90,*(int *)((longlong)plVar16 + -0xc),uVar29);
        *(int *)(plVar16 + -2) = 1;
LAB_1429cf320:
        *(char *)((longlong)(int)uVar29 + (longlong)local_4d0) = '\0';
        plVar20 = (longlong *)(ulonglong)uVar29;
        goto LAB_1429cf32c;
      }
      *(int *)(plVar16 + -2) = 1;
      if (uVar29 != 0xffffffff) goto LAB_1429cf320;
      plVar20 = plVar27;
      if (plVar16 != (longlong *)0x0) {
        plVar20 = (longlong *)0xffffffffffffffff;
        do {
          plVar20 = (longlong *)((longlong)plVar20 + 1);
        } while (*(char *)((longlong)plVar16 + (longlong)plVar20) != '\0');
      }
LAB_1429cf32c:
      iVar11 = (int)plVar20;
      if ((iVar11 < 0) || (*(int *)((longlong)plVar16 + -0xc) + 1 <= iVar11)) {
        FUN_142e54290(0x9c,(ulonglong)plVar20 & 0xffffffff);
      }
      *(int *)(plVar16 + -1) = iVar11;
    }
    goto LAB_1429cf26e;
  }
LAB_1429cf9f5:
  cVar7 = FUN_1406e8ae0(param_2);
  if (cVar7 != '\0') {
    FUN_1406e9170(param_2,param_1 + 0x245,8);
    FUN_1406e9170(param_2,param_1 + 0x246,8);
    uVar18 = DAT_143ac1b90;
    uVar12 = FUN_1406e8c20(param_2);
    FUN_1429b9490(uVar18,param_1 + 0x245,param_1,uVar12);
  }
  cVar7 = FUN_1406e8ae0(param_2);
  if (cVar7 != '\0') {
    FUN_1406e9170(param_2,param_1 + 0x2a4,8);
    FUN_1406e9170(param_2,param_1 + 0x2a5,8);
    uVar18 = DAT_143ac1b90;
    uVar12 = FUN_1406e8c20(param_2);
    FUN_1429b9740(uVar18,param_1 + 0x2a4,param_1,uVar12);
  }
  cVar7 = FUN_1406e8ae0(param_2);
  if (cVar7 != '\0') {
    uVar12 = FUN_1406e8c20(param_2);
    *(undefined4 *)(param_1 + 0x275) = uVar12;
    uVar12 = FUN_1406e8c20(param_2);
    *(undefined4 *)((longlong)param_1 + 0x13ac) = uVar12;
    uVar12 = FUN_1406e8c20(param_2);
    *(undefined4 *)(param_1 + 0x276) = uVar12;
    FUN_1429b9c80(DAT_143ac1b90,(int)param_1[0x275],param_1,uVar12);
  }
  cVar7 = FUN_1406e8ae0(param_2);
  if ((cVar7 != '\0') && (uVar29 = FUN_1406e8c20(param_2), 0 < (int)uVar29)) {
    uVar30 = (ulonglong)uVar29;
    do {
      FUN_141404ee0(param_1[0x784],param_2);
      uVar30 = uVar30 - 1;
    } while (uVar30 != 0);
  }
  bVar6 = FUN_1406e8ae0(param_2);
  if (((bVar6 & 2) != 0) && (iVar15 = FUN_140f8abc0(param_1 + 0x20), iVar15 != 0)) {
    *(uint *)(param_1 + 0x81c) = *(uint *)(param_1 + 0x81c) | 2;
  }
  if (((bVar6 & 0x20) != 0) && (iVar15 = FUN_1406e8c20(param_2), iVar15 - 1U < 4999)) {
    FUN_1427bbf40(param_1,iVar15);
  }
  if (local_4c8 != 0) {
    FUN_1427b7870(param_1,1);
  }
  uVar12 = FUN_1406e8c20(param_2);
  FUN_1427eec30(param_1,uVar12);
  uVar12 = FUN_1406e8c20(param_2);
  FUN_142833030(param_1,uVar12,param_2);
  iVar15 = FUN_14087b630(param_1[0x80e]);
  if (iVar15 != 0) {
    uVar12 = FUN_14087bc10(param_1[0x80e]);
    iVar15 = FUN_1407e6080(uVar12);
    if (iVar15 != 0) {
      if (param_1[0x1ee] != 0) {
        thunk_FUN_140205820(param_1[0x1ee] + -8,0);
        param_1[0x1ee] = 0;
      }
      uVar29 = FUN_1406e8c20(param_2);
      if (0 < (int)uVar29) {
        local_4c0 = (ulonglong)uVar29;
        do {
          local_4c8 = FUN_1406e8c20(param_2);
          lVar19 = param_1[0x1ee];
          uVar29 = 0;
          if (lVar19 == 0) {
LAB_1429cfc65:
            uVar30 = 1;
            plVar20 = plVar27;
            if (lVar19 != 0) {
LAB_1429cfc79:
              uVar25 = *(ulonglong *)(lVar19 + -0x10);
              uVar26 = ~uVar25;
              if (-1 < (longlong)uVar25) {
                uVar26 = uVar25;
              }
              if ((int)(uVar26 - 8 >> 2) == (int)uVar30) goto LAB_1429cfd01;
              if (lVar19 == 0) {
                plVar20 = (longlong *)0x0;
              }
              else {
                plVar20 = (longlong *)(ulonglong)*(uint *)(lVar19 + -8);
              }
            }
            lVar19 = FUN_14019b780(&DAT_143ad68a0,uVar30 * 4 + 8);
            plVar32 = (longlong *)(lVar19 + 8);
            if (lVar19 == 0) {
              plVar32 = plVar27;
            }
            if (param_1[0x1ee] != 0) {
              FUN_142ef7ba0(plVar32,param_1[0x1ee],(longlong)plVar20 << 2);
              thunk_FUN_140205820(param_1[0x1ee] + -8,0);
            }
            param_1[0x1ee] = (longlong)plVar32;
            plVar32[-1] = (longlong)plVar20;
          }
          else {
            uVar29 = *(uint *)(lVar19 + -8);
            uVar30 = *(ulonglong *)(lVar19 + -0x10);
            uVar25 = ~uVar30;
            if (-1 < (longlong)uVar30) {
              uVar25 = uVar30;
            }
            if ((uint)(uVar25 - 8 >> 2) <= uVar29) {
              if (uVar29 == 0) goto LAB_1429cfc65;
              uVar30 = (ulonglong)(uVar29 * 2);
              goto LAB_1429cfc79;
            }
          }
LAB_1429cfd01:
          *(longlong *)(param_1[0x1ee] + -8) = *(longlong *)(param_1[0x1ee] + -8) + 1;
          *(int *)(param_1[0x1ee] + (longlong)(int)uVar29 * 4) = local_4c8;
          local_4c0 = local_4c0 - 1;
        } while (local_4c0 != 0);
      }
    }
  }
  FUN_142834df0(param_1,param_2);
  FUN_142835840(param_1,param_2);
  FUN_1428358a0(param_1,param_2);
  local_4c8 = FUN_1406e8c20(param_2);
  local_4b8 = (longlong *)((ulonglong)local_4b8 & 0xffffffff00000000);
  if (0 < local_4c8) {
    do {
      plVar20 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x88);
      plVar16 = plVar27;
      local_430 = plVar20;
      if (plVar20 != (longlong *)0x0) {
        *plVar20 = 0;
        plVar20[1] = 0;
        *(int *)(plVar20 + 1) = 1;
        *(int *)((longlong)plVar20 + 0xc) = 1;
        *plVar20 = (longlong)&PTR_FUN_143482b78;
        FUN_1413ebba0(plVar20 + 2);
        plVar16 = plVar20;
      }
      local_4d8 = local_4d8 | 0x100;
      plVar20 = plVar16 + 2;
      local_4d4 = local_4d8;
      local_398 = plVar20;
      local_390 = plVar16;
      FUN_1413ebe00(plVar20,param_2,0);
      uVar29 = FUN_1413ec4c0(plVar20);
      puVar2 = (undefined8 *)param_1[0x7ef];
      puVar23 = (undefined8 *)puVar2[1];
      uStack_478 = 0;
      cVar7 = *(char *)((longlong)puVar23 + 0x19);
      puVar28 = puVar2;
      local_480 = puVar23;
      while (puVar4 = puVar23, cVar7 == '\0') {
        if (uVar29 <= *(uint *)(puVar4 + 4)) {
          puVar23 = (undefined8 *)*puVar4;
          puVar28 = puVar4;
        }
        else {
          puVar23 = (undefined8 *)puVar4[2];
        }
        uStack_478 = (uint)(uVar29 <= *(uint *)(puVar4 + 4));
        cVar7 = *(char *)((longlong)puVar23 + 0x19);
        local_480 = puVar4;
      }
      if ((*(char *)((longlong)puVar28 + 0x19) != '\0') || (uVar29 < *(uint *)(puVar28 + 4))) {
        if (param_1[0x7f0] == 0x492492492492492) {
                    /* WARNING: Subroutine does not return */
          FUN_14019f9d0();
        }
        local_380 = 0;
        local_388 = param_1 + 0x7ef;
        plVar32 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x38);
        *(uint *)(plVar32 + 4) = uVar29;
        plVar32[5] = 0;
        plVar32[6] = 0;
        if (plVar16 != (longlong *)0x0) {
          LOCK();
          *(int *)(plVar16 + 1) = (int)plVar16[1] + 1;
          UNLOCK();
          local_4d8 = local_4d4;
          plVar20 = local_398;
        }
        plVar32[5] = (longlong)plVar20;
        plVar32[6] = (longlong)plVar16;
        *plVar32 = (longlong)puVar2;
        plVar32[1] = (longlong)puVar2;
        plVar32[2] = (longlong)puVar2;
        *(undefined2 *)(plVar32 + 3) = 0;
        local_380 = 0;
        local_328 = (undefined4)local_480;
        uStack_324 = local_480._4_4_;
        uStack_320 = uStack_478;
        uStack_31c = uStack_474;
        FUN_14287de90(param_1 + 0x7ef,&local_328);
      }
      if (plVar16 != (longlong *)0x0) {
        FUN_1402abcb0(plVar16);
      }
      iVar15 = (int)local_4b8 + 1;
      local_4b8 = (longlong *)CONCAT44(local_4b8._4_4_,iVar15);
    } while (iVar15 < local_4c8);
    plVar16 = (longlong *)(ulonglong)local_4d8;
  }
  local_4c8 = FUN_1406e8c20(param_2);
  local_4b8 = (longlong *)((ulonglong)local_4b8 & 0xffffffff00000000);
  if (0 < local_4c8) {
    plVar20 = param_1 + 0x7b9;
    do {
      iVar15 = FUN_1406e8c20(param_2);
      cVar7 = FUN_1406e8ae0(param_2);
      local_4c4 = cVar7 != '\0';
      puVar2 = (undefined8 *)*plVar20;
      puVar23 = (undefined8 *)puVar2[1];
      uStack_478 = 0;
      cVar7 = *(char *)((longlong)puVar23 + 0x19);
      puVar28 = puVar2;
      local_480 = puVar23;
      while (puVar4 = puVar23, cVar7 == '\0') {
        bVar3 = iVar15 <= *(int *)((longlong)puVar4 + 0x1c);
        if (bVar3) {
          puVar23 = (undefined8 *)*puVar4;
          puVar28 = puVar4;
        }
        else {
          puVar23 = (undefined8 *)puVar4[2];
        }
        uStack_478 = (uint)bVar3;
        cVar7 = *(char *)((longlong)puVar23 + 0x19);
        local_480 = puVar4;
      }
      if ((*(char *)((longlong)puVar28 + 0x19) != '\0') ||
         (iVar15 < *(int *)((longlong)puVar28 + 0x1c))) {
        if (param_1[0x7ba] == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
          FUN_14019f9d0();
        }
        local_370 = 0;
        local_378 = plVar20;
        puVar23 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x28);
        *(int *)((longlong)puVar23 + 0x1c) = iVar15;
        *(undefined1 *)(puVar23 + 4) = local_4c4;
        *puVar23 = puVar2;
        puVar23[1] = puVar2;
        puVar23[2] = puVar2;
        *(undefined2 *)(puVar23 + 3) = 0;
        local_370 = 0;
        local_318 = (undefined4)local_480;
        uStack_314 = local_480._4_4_;
        uStack_310 = uStack_478;
        uStack_30c = uStack_474;
        FUN_141004ac0(plVar20,&local_318,puVar23);
      }
      iVar15 = (int)local_4b8 + 1;
      local_4b8 = (longlong *)CONCAT44(local_4b8._4_4_,iVar15);
    } while (iVar15 < local_4c8);
  }
  if (DAT_143ac87a0 != 0) {
    FUN_1415f0d70();
  }
  if ((DAT_143aa8518 != 0) && (iVar15 = FUN_142889160(), iVar15 == 0xbe)) {
    (**(code **)(*param_1 + 0x168))(param_1,0xffffffff);
  }
  lVar19 = FUN_141892840();
  if (lVar19 != 0) {
    uVar18 = FUN_141892840();
    plVar20 = (longlong *)FUN_141bc85c0(uVar18,local_308);
    local_4d4 = (uint)plVar16 | 4;
    plVar16 = (longlong *)(ulonglong)local_4d4;
    if (*plVar20 != 0) {
      bVar3 = true;
      goto LAB_1429d0094;
    }
  }
  bVar3 = false;
LAB_1429d0094:
  if ((((ulonglong)plVar16 & 4) != 0) && (local_300 != 0)) {
    FUN_1402abcb0();
  }
  if (bVar3) {
    local_430 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x30);
    if (local_430 != (longlong *)0x0) {
      *local_430 = 0;
      local_430[1] = 0;
      *(int *)(local_430 + 1) = 1;
      *(int *)((longlong)local_430 + 0xc) = 1;
      *local_430 = (longlong)&PTR_LAB_143487070;
      local_430[2] = 0;
      local_430[3] = 0;
      local_430[4] = 0;
      local_430[5] = 0;
      *(int *)(local_430 + 2) = 0xff;
      local_430[3] = 0;
      local_430[4] = 0;
      local_430[5] = 0;
      plVar27 = local_430;
    }
    local_3b0 = plVar27 + 2;
    local_3a8 = plVar27;
    FUN_1429e1020(param_1 + 0x879,&local_3b0);
    if (local_3a8 != (longlong *)0x0) {
      FUN_1402abcb0();
    }
    lVar19 = param_1[0x879];
    uVar18 = FUN_141892840();
    uVar18 = FUN_141bc85c0(uVar18,local_2f8);
    cVar7 = FUN_140321cc0(lVar19,uVar18,param_1);
    if (local_2f0 != 0) {
      FUN_1402abcb0();
    }
    if (cVar7 == '\0') {
      local_368 = 0;
      lStack_360 = 0;
      FUN_1429e1020(param_1 + 0x879,&local_368);
      if (lStack_360 != 0) {
        FUN_1402abcb0();
      }
    }
  }
  lVar19 = DAT_143aa84a0;
  if (((((int)param_1[0x221] != 0) && (*(int *)((longlong)param_1 + 0x110c) != 0)) &&
      (DAT_143aa84a0 != 0)) &&
     ((lVar24 = FUN_142cc0b90(DAT_143aa84a0), lVar24 == 0 ||
      (*(int *)(lVar24 + 0x10) != *(int *)((longlong)param_1 + 0x110c))))) {
    FUN_142cc0ca0(lVar19,(int)param_1[0x221]);
  }
  uVar12 = FUN_14087bc10(param_1[0x80e]);
  FUN_14284bf00(param_1,uVar12);
  if ((0 < (int)param_1[0x76f]) && (DAT_143ac0768 != 0)) {
    FUN_1418c2d40(DAT_143ac0768,local_358);
    if (local_350 != 0) {
      uVar12 = FUN_14276df20(param_1);
      FUN_1418c15b0(local_350,1,uVar12);
    }
    FUN_140456f60(local_358);
  }
  local_2b8 = &PTR_FUN_143273970;
  if (local_2a0 != (int *)0x0) {
    LOCK();
    local_2a0[2] = 0;
    local_2a0[3] = 0;
    UNLOCK();
    do {
    } while (local_2a0[1] != 0);
    if (local_2a0 != (int *)0x0) {
      LOCK();
      iVar15 = *local_2a0;
      *local_2a0 = *local_2a0 + -1;
      UNLOCK();
      if (iVar15 == 1) {
        thunk_FUN_140205820(local_2a0,0x10);
      }
    }
  }
  return;
LAB_1429cf205:
  plVar16 = local_4d0;
  if ((int)local_4d0[-2] != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (*(int *)((longlong)plVar16 + -0xc) < 0) {
    FUN_142e54290(0x90,*(int *)((longlong)plVar16 + -0xc),0);
  }
  *(int *)(plVar16 + -2) = 1;
  *(char *)local_4d0 = '\0';
  if (*(int *)((longlong)plVar16 + -0xc) + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  *(int *)(plVar16 + -1) = 0;
  if (local_4d0 != (longlong *)0x0) {
    FUN_14019f2c0(local_4d0 + -2);
    local_4d0 = (longlong *)0x0;
  }
LAB_1429cf26e:
  FUN_1401d12a0(&local_4d0,0);
  local_4c0 = DAT_143ac2f58;
  plVar16 = plVar27;
  iVar11 = iVar15;
  if ((local_4d0 == (longlong *)0x0) || (plVar16 = local_4d0 + -2, plVar16 == (longlong *)0x0)) {
LAB_1429cf38b:
    if (iVar11 < 0x400) {
      iVar11 = 0x400;
    }
    puVar22 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
    puVar22[1] = iVar11;
    *puVar22 = 0xffffffff;
    local_4d0 = (longlong *)(puVar22 + 4);
    if (plVar16 == (longlong *)0x0) {
      puVar22[2] = 0;
      *(char *)local_4d0 = '\0';
    }
    else {
      iVar14 = (int)plVar16[1] + 1;
      iVar33 = iVar11 + 1;
      if (iVar33 < iVar14) {
        FUN_142e54290(0x5c,iVar14,iVar33);
        iVar14 = iVar33;
      }
      FUN_142ef7ba0(local_4d0,plVar16 + 2,(longlong)iVar14);
      puVar22[2] = (int)plVar16[1];
      *(char *)((longlong)iVar11 + (longlong)local_4d0) = '\0';
      FUN_14019f2c0(plVar16);
    }
  }
  else {
    if ((1 < (int)*plVar16) || (*(int *)((longlong)local_4d0 + -0xc) < 0x400)) {
      iVar11 = (int)local_4d0[-1];
      goto LAB_1429cf38b;
    }
    if ((int)*plVar16 != 1) {
      FUN_142e52dd0(0x74);
    }
    *(int *)plVar16 = -1;
  }
  local_4f8 = (longlong *)0x0;
  FUN_1408bed00(local_4c0,local_4d0,0,1);
  plVar16 = local_4d0;
  if ((int)local_4d0[-2] != -1) {
    FUN_142e52dd0(0x8b);
  }
  *(int *)(plVar16 + -2) = 1;
  plVar20 = plVar27;
  iVar11 = iRamfffffffffffffff4;
  if (plVar16 == (longlong *)0x0) {
LAB_1429cf483:
    iVar14 = (int)plVar20;
    if (iVar11 + 1 <= iVar14) goto LAB_1429cf487;
  }
  else {
    plVar20 = (longlong *)0xffffffffffffffff;
    do {
      plVar20 = (longlong *)((longlong)plVar20 + 1);
    } while (*(char *)((longlong)plVar16 + (longlong)plVar20) != '\0');
    iVar11 = *(int *)((longlong)plVar16 + -0xc);
    if (-1 < (int)plVar20) goto LAB_1429cf483;
LAB_1429cf487:
    iVar14 = (int)plVar20;
    FUN_142e54290(0x9c,(ulonglong)plVar20 & 0xffffffff);
  }
  plVar20 = local_4d0;
  *(int *)(plVar16 + -1) = iVar14;
  if ((local_4d0 == (longlong *)0x0) || ((int)local_4d0[-1] < 0x29)) {
    local_498 = (longlong *)0x0;
    if ((local_4d0 != (longlong *)0x0) && (plVar16 = local_4d0 + -2, plVar16 != (longlong *)0x0)) {
      if ((int)*plVar16 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        plVar20 = local_4d0;
        local_3e8 = (longlong *)0x0;
        plVar16 = plVar27;
        if (local_4d0 != (longlong *)0x0) {
          plVar32 = (longlong *)0xffffffffffffffff;
          do {
            plVar32 = (longlong *)((longlong)plVar32 + 1);
          } while (*(char *)((longlong)local_4d0 + (longlong)plVar32) != '\0');
          iVar14 = (int)plVar32;
          iVar11 = iVar15;
          if (0 < iVar14) {
            iVar11 = iVar14;
          }
          piVar17 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
          piVar17[1] = iVar11;
          *piVar17 = -1;
          plVar16 = (longlong *)(piVar17 + 4);
          piVar17[2] = 0;
          *(char *)plVar16 = '\0';
          local_3e8 = plVar16;
          FUN_142ef7ba0(plVar16,plVar20,(longlong)iVar14);
          if (*piVar17 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar14 == -1) || (iVar14 <= piVar17[1])) {
            *piVar17 = 1;
            if (iVar14 != -1) goto LAB_1429cf5a2;
            plVar32 = plVar27;
            if (plVar16 != (longlong *)0x0) {
              plVar32 = (longlong *)0xffffffffffffffff;
              do {
                plVar32 = (longlong *)((longlong)plVar32 + 1);
              } while (*(char *)((longlong)plVar16 + (longlong)plVar32) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar17[1],(ulonglong)plVar32 & 0xffffffff);
            *piVar17 = 1;
LAB_1429cf5a2:
            *(char *)((longlong)plVar16 + (longlong)iVar14) = '\0';
          }
          iVar11 = (int)plVar32;
          if ((iVar11 < 0) || (piVar17[1] + 1 <= iVar11)) {
            FUN_142e54290(0x9c,(ulonglong)plVar32 & 0xffffffff);
          }
          piVar17[2] = iVar11;
        }
        local_498 = plVar16;
      }
      else {
        if ((int)*plVar16 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *(int *)plVar16 = (int)*plVar16 + 1;
        UNLOCK();
        local_498 = plVar20;
        local_4d8 = local_4d4;
      }
    }
    pplVar34 = &local_498;
    uVar29 = local_4d8 | 2;
  }
  else {
    FUN_14019ce60(&local_4d0,&local_440,0,0x28);
    pplVar34 = &local_440;
    uVar29 = local_4d8 | 0x41;
  }
  local_4d8 = uVar29;
  local_4d4 = uVar29;
  if (local_4d0 != (longlong *)0x0) {
    FUN_14019f2c0(local_4d0 + -2);
  }
  local_4d0 = *pplVar34;
  *pplVar34 = (longlong *)0x0;
  if (((uVar29 & 2) != 0) &&
     (uVar29 = uVar29 & 0xfffffffd, local_4d8 = uVar29, local_4d4 = uVar29,
     local_498 != (longlong *)0x0)) {
    FUN_14019f2c0(local_498 + -2);
  }
  if (((uVar29 & 1) != 0) &&
     (local_4d8 = uVar29 & 0xfffffffe, local_4d4 = local_4d8, local_440 != (longlong *)0x0)) {
    FUN_14019f2c0(local_440 + -2);
  }
  pplVar34 = (longlong **)(param_1 + 0x817);
  if (pplVar34 == &local_4d0) goto LAB_1429cf889;
  plVar16 = *pplVar34;
  iVar11 = iVar15;
  if (plVar16 != (longlong *)0x0) {
    iVar11 = (int)plVar16[-1];
  }
  plVar20 = plVar27;
  iVar14 = iVar15;
  if (local_4d0 != (longlong *)0x0) {
    plVar20 = local_4d0;
    iVar14 = (int)local_4d0[-1];
  }
  if (((iVar11 == iVar14) && (iVar11 != 0)) && (plVar16 != (longlong *)0x0)) {
    if (plVar20 != (longlong *)0x0) {
      iVar11 = memcmp(plVar16,plVar20,(longlong)iVar11);
      if (iVar11 == 0) goto LAB_1429cf889;
      goto LAB_1429cf70c;
    }
LAB_1429cf878:
    FUN_14019f2c0(plVar16 + -2);
    *pplVar34 = (longlong *)0x0;
  }
  else {
LAB_1429cf70c:
    if ((plVar20 == (longlong *)0x0) || (plVar32 = plVar20 + -2, plVar32 == (longlong *)0x0)) {
      if (plVar16 != (longlong *)0x0) goto LAB_1429cf878;
    }
    else if ((int)*plVar32 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      plVar20 = local_4d0;
      local_3e0 = (longlong *)0x0;
      plVar16 = plVar27;
      if (local_4d0 != (longlong *)0x0) {
        plVar32 = (longlong *)0xffffffffffffffff;
        do {
          plVar32 = (longlong *)((longlong)plVar32 + 1);
        } while (*(char *)((longlong)plVar32 + (longlong)local_4d0) != '\0');
        iVar11 = (int)plVar32;
        if (0 < iVar11) {
          iVar15 = iVar11;
        }
        piVar17 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
        piVar17[1] = iVar15;
        *piVar17 = -1;
        plVar16 = (longlong *)(piVar17 + 4);
        piVar17[2] = 0;
        *(char *)plVar16 = '\0';
        local_4c0 = (ulonglong)iVar11;
        local_3e0 = plVar16;
        FUN_142ef7ba0(plVar16,plVar20,local_4c0);
        if (*piVar17 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar11 == -1) || (iVar11 <= piVar17[1])) {
          *piVar17 = 1;
          if (iVar11 != -1) goto LAB_1429cf7da;
          plVar32 = plVar27;
          if (plVar16 != (longlong *)0x0) {
            plVar32 = (longlong *)0xffffffffffffffff;
            do {
              plVar32 = (longlong *)((longlong)plVar32 + 1);
            } while (*(char *)((longlong)plVar16 + (longlong)plVar32) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar17[1],(ulonglong)plVar32 & 0xffffffff);
          *piVar17 = 1;
LAB_1429cf7da:
          *(char *)((longlong)plVar16 + local_4c0) = '\0';
        }
        iVar15 = (int)plVar32;
        if ((iVar15 < 0) || (piVar17[1] + 1 <= iVar15)) {
          FUN_142e54290(0x9c,(ulonglong)plVar32 & 0xffffffff);
        }
        piVar17[2] = iVar15;
      }
      if (*pplVar34 != (longlong *)0x0) {
        FUN_14019f2c0(*pplVar34 + -2);
      }
      *pplVar34 = plVar16;
    }
    else {
      if ((int)*plVar32 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *(int *)plVar32 = (int)*plVar32 + 1;
      UNLOCK();
      if (*pplVar34 != (longlong *)0x0) {
        FUN_14019f2c0(*pplVar34 + -2);
      }
      *pplVar34 = plVar20;
      local_4d8 = local_4d4;
    }
  }
LAB_1429cf889:
  plVar16 = (longlong *)(ulonglong)local_4d8;
  iVar15 = FUN_140f8abc0(param_1 + 0x20);
  if (iVar15 == 0) {
    local_4a8 = (longlong *)param_1[8];
    local_348 = &local_438;
    local_438 = (longlong *)param_1[0x1c1];
    if (local_438 != (longlong *)0x0) {
      (**(code **)(*local_438 + 8))();
    }
    local_2e8 = &local_490;
    local_490 = (longlong *)param_1[0x1cb];
    if (local_490 != (longlong *)0x0) {
      (**(code **)(*local_490 + 8))();
    }
    plVar20 = *pplVar34;
    local_4b8 = plVar20;
    plVar32 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x18);
    local_430 = plVar32;
    if (plVar32 == (longlong *)0x0) {
      local_488 = (longlong *)0x0;
    }
    else {
      plVar32[1] = 0;
      *(int *)(plVar32 + 2) = 1;
      if (plVar20 == (longlong *)0x0) {
        *plVar32 = 0;
        local_488 = plVar32;
      }
      else {
        local_4f0 = 0;
        local_4f8 = (longlong *)0x0;
        iVar15 = (*DAT_1432627f8)(0xfde9,0,plVar20,0xffffffff);
        uVar30 = (ulonglong)(longlong)(iVar15 * 2) >> 1;
        iVar15 = (int)uVar30;
        local_4c0 = CONCAT44(local_4c0._4_4_,iVar15 + -1);
        piVar17 = (int *)(*DAT_143ad5980)((uVar30 & 0xffffffff) * 2 + 4);
        plVar20 = plVar27;
        if (piVar17 != (int *)0x0) {
          *piVar17 = (int)local_4c0 * 2;
          pcVar21 = (char *)((longlong)(piVar17 + 1) + (local_4c0 & 0xffffffff) * 2);
          pcVar21[0] = '\0';
          pcVar21[1] = '\0';
          plVar20 = (longlong *)(piVar17 + 1);
        }
        local_4f8 = plVar20;
        local_4f0 = iVar15;
        (*DAT_1432627f8)(0xfde9,0,local_4b8,0xffffffff);
        *plVar32 = (longlong)plVar20;
        local_488 = plVar32;
      }
    }
    if (local_488 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x8007000e);
    }
    local_4f8 = (longlong *)((ulonglong)local_4f8 & 0xffffffff00000000);
    FUN_141597250(local_4a8,&local_488,&local_490,&local_438);
  }
  if (local_4d0 != (longlong *)0x0) {
    FUN_14019f2c0(local_4d0 + -2);
  }
  goto LAB_1429cf9f5;
}



//===========================================================
// FUN_1429cdb70 @ 1429cdb70   (956 bytes)
//===========================================================

undefined8 * FUN_1429cdb70(undefined8 *param_1,undefined4 param_2,uint param_3)

{
  char cVar1;
  undefined8 *puVar2;
  undefined4 uVar3;
  longlong lVar4;
  undefined8 uVar5;
  undefined8 *puVar6;
  undefined8 *puVar7;
  bool bVar8;
  undefined8 *local_48;
  undefined8 uStack_40;
  undefined8 *local_38;
  uint uStack_30;
  undefined4 uStack_2c;
  
  FUN_142768ee0(param_1,param_3);
  *param_1 = &PTR_FUN_143486b70;
  param_1[1] = &PTR_LAB_143486d10;
  param_1[2] = &PTR_LAB_143486d80;
  param_1[0x20] = &PTR_LAB_143486d88;
  *(undefined4 *)(param_1 + 0x80d) = param_2;
  lVar4 = FUN_14019b780(&DAT_143ad68a0,0x4478);
  uVar5 = 0;
  if (lVar4 != 0) {
    uVar5 = FUN_140862520(lVar4,0);
  }
  param_1[0x80e] = uVar5;
  *(undefined8 *)((longlong)param_1 + 0x4084) = 0;
  *(undefined8 *)((longlong)param_1 + 0x4094) = 0;
  *(undefined8 *)((longlong)param_1 + 0x409c) = 0;
  *(undefined4 *)((longlong)param_1 + 0x40a4) = 0;
  *(undefined4 *)((longlong)param_1 + 0x40ac) = 0;
  param_1[0x817] = 0;
  *(undefined4 *)(param_1 + 0x818) = 0xffffffff;
  *(undefined4 *)(param_1 + 0x819) = 0xffffffff;
  *(undefined4 *)((longlong)param_1 + 0x40cc) = 0;
  uVar3 = (*DAT_143262db0)();
  *(undefined4 *)(param_1 + 0x81a) = uVar3;
  *(undefined8 *)((longlong)param_1 + 0x40d4) = 0;
  *(undefined8 *)((longlong)param_1 + 0x40dc) = 0;
  *(undefined4 *)((longlong)param_1 + 0x40e4) = 0;
  param_1[0x81d] = 0;
  param_1[0x81e] = 0;
  param_1[0x81f] = 0;
  param_1[0x820] = 0;
  param_1[0x821] = 0;
  param_1[0x822] = 0;
  param_1[0x823] = 0;
  param_1[0x824] = 0;
  param_1[0x825] = 0;
  param_1[0x826] = 0;
  param_1[0x827] = 0;
  param_1[0x828] = 0;
  param_1[0x829] = 0;
  param_1[0x82a] = 0;
  param_1[0x82b] = 0;
  *(undefined4 *)(param_1 + 0x82c) = 0;
  *(undefined8 *)((longlong)param_1 + 0x4164) = 0;
  *(undefined8 *)((longlong)param_1 + 0x416c) = 0;
  *(undefined8 *)((longlong)param_1 + 0x4174) = 0;
  *(undefined8 *)((longlong)param_1 + 0x417c) = 0;
  *(undefined8 *)((longlong)param_1 + 0x4184) = 0;
  *(undefined8 *)((longlong)param_1 + 0x418c) = 0;
  *(undefined8 *)((longlong)param_1 + 0x4194) = 0;
  *(undefined8 *)((longlong)param_1 + 0x419c) = 0;
  *(undefined8 *)((longlong)param_1 + 0x41a4) = 0;
  *(undefined8 *)((longlong)param_1 + 0x41ac) = 0;
  *(undefined8 *)((longlong)param_1 + 0x41b4) = 0;
  *(undefined8 *)((longlong)param_1 + 0x41bc) = 0;
  *(undefined8 *)((longlong)param_1 + 0x41c4) = 0;
  *(undefined8 *)((longlong)param_1 + 0x41cc) = 0;
  *(undefined8 *)((longlong)param_1 + 0x41d4) = 0;
  *(undefined4 *)((longlong)param_1 + 0x41dc) = 0;
  FUN_140944d70(param_1 + 0x83c,0);
  param_1[0x861] = 0;
  param_1[0x862] = 0;
  param_1[0x863] = 0;
  param_1[0x864] = 0;
  *(undefined4 *)((longlong)param_1 + 0x432c) = 0;
  param_1[0x866] = 0;
  param_1[0x867] = 0;
  *(undefined4 *)((longlong)param_1 + 0x4344) = 0;
  param_1[0x869] = 0;
  param_1[0x86a] = 0;
  *(undefined4 *)(param_1 + 0x86b) = 0;
  param_1[0x86d] = 0;
  *(undefined4 *)(param_1 + 0x86e) = 0;
  param_1[0x86f] = 0;
  param_1[0x870] = 0;
  param_1[0x871] = 0;
  *(undefined4 *)((longlong)param_1 + 0x43b4) = 0;
  param_1[0x878] = 0;
  param_1[0x879] = 0;
  param_1[0x87a] = 0;
  *(undefined4 *)(param_1 + 0x87b) = 0;
  param_1[0x87c] = 0;
  param_1[0x87d] = 0;
  lVar4 = FUN_14019b780(&DAT_143ad68a0,0x40);
  *(longlong *)lVar4 = lVar4;
  *(longlong *)(lVar4 + 8) = lVar4;
  param_1[0x87c] = lVar4;
  *(undefined1 *)(param_1 + 0x87e) = 0;
  param_1[0x87f] = 0;
  *(undefined1 *)(param_1 + 0x881) = 0;
  puVar7 = param_1 + 0x882;
  *puVar7 = 0;
  param_1[0x883] = 0;
  param_1[0x884] = 0;
  param_1[0x885] = 0;
  param_1[0x886] = 0;
  puVar6 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x10);
  puVar6[1] = 0;
  *puVar7 = puVar6;
  *puVar6 = puVar7;
  param_1[0x80f] = 0;
  *(undefined8 *)((longlong)param_1 + 0x68c) = 0;
  param_1[0x872] = 0;
  param_1[0x873] = 0;
  *(undefined4 *)(param_1 + 0x874) = 0;
  *(undefined8 *)((longlong)param_1 + 0x43a4) = 0;
  *(undefined8 *)((longlong)param_1 + 0x43ac) = 0;
  *(undefined8 *)((longlong)param_1 + 0x43b4) = 0;
  puVar7 = DAT_143adc538;
  local_48 = &DAT_143adc538;
  uStack_40 = (undefined8 *)0x0;
  uStack_40 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x20);
  *(uint *)((longlong)uStack_40 + 0x1c) = param_3;
  *uStack_40 = puVar7;
  uStack_40[1] = puVar7;
  uStack_40[2] = puVar7;
  *(undefined2 *)(uStack_40 + 3) = 0;
  puVar7 = (undefined8 *)DAT_143adc538[1];
  uStack_30 = 0;
  cVar1 = *(char *)((longlong)puVar7 + 0x19);
  local_38 = puVar7;
  puVar6 = DAT_143adc538;
  while (puVar2 = puVar7, cVar1 == '\0') {
    bVar8 = param_3 <= *(uint *)((longlong)puVar2 + 0x1c);
    if (bVar8) {
      puVar7 = (undefined8 *)*puVar2;
      puVar6 = puVar2;
    }
    else {
      puVar7 = (undefined8 *)puVar2[2];
    }
    uStack_30 = (uint)bVar8;
    cVar1 = *(char *)((longlong)puVar7 + 0x19);
    local_38 = puVar2;
  }
  if ((*(char *)((longlong)puVar6 + 0x19) == '\0') &&
     (*(uint *)((longlong)puVar6 + 0x1c) <= param_3)) {
    thunk_FUN_140205820(uStack_40,0x20);
  }
  else {
    if (DAT_143adc540 == 0x7ffffffffffffff) {
                    /* WARNING: Subroutine does not return */
      FUN_14019f9d0();
    }
    uStack_40 = (undefined8 *)CONCAT44(uStack_2c,uStack_30);
    local_48 = local_38;
    FUN_140cac9b0(&DAT_143adc538,&local_48);
  }
  return param_1;
}



//===========================================================
// FUN_140a46e50 @ 140a46e50   (14143 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 FUN_140a46e50(longlong param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined1 uVar2;
  undefined2 uVar3;
  short sVar4;
  undefined4 uVar5;
  int iVar6;
  undefined8 uVar7;
  undefined1 auStack_178 [32];
  undefined1 local_158;
  int local_154;
  int local_150;
  int local_14c;
  int local_148;
  int local_144;
  int local_140;
  int local_13c;
  longlong *local_138;
  longlong local_130;
  code *local_128;
  undefined1 local_118 [128];
  undefined1 local_98 [128];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_178;
  FUN_1402c24f0(local_118);
  FUN_1406e9170(param_3,local_118,0x7c);
  cVar1 = FUN_1402bf6d0(local_118,0x6a);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3c500(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c415a0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x6d);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3bab0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c40b50(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x71);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3b6a0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c40740(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x5c);
  if (cVar1 != '\0') {
    cVar1 = FUN_1406e8ae0(param_3);
    FUN_140c3ba60(param_1,(int)cVar1);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x74);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3bd30(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c40dd0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xbc);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39a30(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3ead0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xd5);
  if (cVar1 != '\0') {
    cVar1 = FUN_1406e8ae0(param_3);
    FUN_140c3b6f0(param_1,(int)cVar1);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x77);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c38860(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3d900(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x76);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3b510(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c405b0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x7f);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3c4b0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c41550(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xd3);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3c460(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c41500(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x80);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c38590(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3d630(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x81);
  if (cVar1 != '\0') {
    sVar4 = FUN_1406e8b80(param_3);
    FUN_14089a250(param_1,(int)sVar4);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089d400(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,199);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3bfb0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c41050(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xca);
  if (cVar1 != '\0') {
    uVar2 = FUN_1406e8ae0(param_3);
    FUN_140c3bd80(param_1,uVar2);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xc9);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3bf10(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c40fb0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x75);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3aed0(param_1,uVar3);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x75);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3aed0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3ff70(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,99);
  if (cVar1 != '\0') {
    FUN_140897670(param_1,1);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x68);
  if (cVar1 != '\0') {
    FUN_140c3b920(param_1,1);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x82);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3ab60(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3fc00(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x93);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39990(param_1,uVar3);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x88);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c37a50(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3caf0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x101);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_1408992b0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089c6e0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x102);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3a7f0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3f890(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x89);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_1408996c0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x8f);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140896e00(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089acf0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x94);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c37aa0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3cb40(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x9d);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c38c70(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3dd10(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x95);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3b380(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c40420(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x97);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140899e40(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x98);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140899da0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x99);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140897810(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x9a);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_1408978c0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x9b);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c38b80(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3dc20(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x9c);
  if (cVar1 != '\0') {
    FUN_140c38bd0(param_1,1);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xa2);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3b240(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c402e0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xa4);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3bc90(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c40d30(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xa5);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3bc40(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c40ce0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xa6);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39530(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3e5d0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xaa);
  if (cVar1 != '\0') {
    FUN_140c39850(param_1,1);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xab);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c398f0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3e990(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xd4);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c398a0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3e940(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xc6);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3c550(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c415f0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xae);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c38d60(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3de00(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xb0);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39620(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3e6c0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xb2);
  if (cVar1 != '\0') {
    FUN_140c37be0(param_1,1);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xc4);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3a1b0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3f250(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xcb);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c394e0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3e580(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xd2);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c38810(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3d8b0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xd8);
  if (cVar1 != '\0') {
    FUN_1408982c0(param_1,1);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xdf);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c38a90(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3db30(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xe0);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3bb00(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c40ba0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xe2);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39490(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3e530(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xea);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c392b0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3e350(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xee);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c388b0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3d950(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x100);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3ad90(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3fe30(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xf0);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3a570(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3f5c0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x109);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_14089a7a0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089d8b0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x10f);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3c280(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c41320(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xf1);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3c320(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c413c0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xf6);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_1408999e0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xf9);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_1408984a0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089c0a0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xfb);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3a250(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3f2f0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xfe);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140898c70(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089c320(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x10e);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3ba10(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c40ab0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x110);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_14089a3e0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089d540(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x112);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3af70(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c40010(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x18a);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140897d20(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089b9c0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xa7);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39bc0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3ec60(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x115);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140899530(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089c910(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x116);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c399e0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3ea80(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x117);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3c0a0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c41140(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x118);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c384f0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3d590(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x118);
  if (cVar1 != '\0') {
    local_150 = (*DAT_143262db0)();
    iVar6 = FUN_1406e8c20(param_3);
    FUN_140c425e0(param_1,local_150 + iVar6);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x119);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39e40(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3eee0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x11a);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39f80(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3f020(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x11b);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39e90(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3ef30(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x11c);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39df0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3ee90(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x11d);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39670(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3e710(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x11e);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c396c0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3e760(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x11f);
  if (cVar1 != '\0') {
    FUN_140c3c2d0(param_1,1);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x120);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39b70(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3ec10(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xde);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c386d0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3d770(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x10a);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140896cc0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089ac00(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xaf);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3aca0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3fd40(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x125);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c37c30(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3ccd0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x9e);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3a430(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3f4d0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x126);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39ee0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3ef80(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x127);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c37a00(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3caa0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x129);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3c1e0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c41280(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x12a);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3bbf0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c40c90(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,299);
  if (cVar1 != '\0') {
    uVar2 = FUN_1406e8ae0(param_3);
    FUN_140c3b330(param_1,uVar2);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c403d0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x12f);
  if (cVar1 != '\0') {
    uVar2 = FUN_1406e8ae0(param_3);
    FUN_140c39710(param_1,uVar2);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3e7b0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x133);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3ac50(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3fcf0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x145);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3ac00(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3fca0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x134);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39350(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3e3f0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x135);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39300(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3e3a0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x138);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3c0f0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c41190(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x13e);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140899c60(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089cf50(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xc2);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140896e50(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089ad40(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x147);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140899a30(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089cd70(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x148);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_1408984f0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x148);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_1408a1230(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x13a);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089bbf0(param_1,uVar5);
    local_14c = (*DAT_143262db0)();
    iVar6 = FUN_1406e8c20(param_3);
    FUN_1408a10a0(param_1,local_14c + iVar6);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x10a);
  if (cVar1 != '\0') {
    cVar1 = FUN_1406e8ae0(param_3);
    local_158 = cVar1 != '\0';
    FUN_1408963e0(param_1,local_158);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089dd10(param_1,uVar5);
    iVar6 = FUN_140c33e60(param_1);
    if (iVar6 != 0) {
      local_148 = FUN_140c33e60(param_1);
      iVar6 = (*DAT_143262db0)();
      FUN_14089dd10(param_1,local_148 + iVar6);
    }
  }
  cVar1 = FUN_1402bf6d0(local_118,0xb3);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140897620(param_1,uVar5);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089b3d0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x16a);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3a890(param_1,uVar5);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3f930(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x130);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39b20(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3ebc0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x160);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140897350(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089b1a0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x163);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140896f40(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089ae30(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x17f);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_14089a5c0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089d720(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x180);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_1408970d0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089afc0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x181);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140898f40(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089c5a0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x182);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140896ea0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089ad90(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x162);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140896d60(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089ac50(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x167);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140897d70(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089ba10(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xf8);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140898d60(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089c3c0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x173);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140896ef0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089ade0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x173);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_1408a0d30(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x177);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140897c80(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089b920(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1cd);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3b560(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1e2);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140896f90(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089ae80(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x103);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39ad0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3eb70(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x104);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3bce0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c40d80(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x105);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_14089a520(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089d680(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x106);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_1408972b0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089b100(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x17d);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3bba0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c40c40(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x169);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140899940(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089cc80(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x185);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089a980(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x18d);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_1408997b0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089cb90(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x18c);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140897f50(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089bba0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,399);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_1408973a0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089b1f0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,400);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140897be0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089b880(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x191);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140899b70(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089ceb0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x192);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_1408973f0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089b240(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x193);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140898130(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089bd80(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x194);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140897030(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089af20(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x194);
  if (cVar1 != '\0') {
    local_144 = (*DAT_143262db0)();
    iVar6 = FUN_1406e8c20(param_3);
    FUN_14089e030(param_1,local_144 + iVar6);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x195);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_1408979b0(param_1,uVar5);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089b6a0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x195);
  if (cVar1 != '\0') {
    local_140 = (*DAT_143262db0)();
    iVar6 = FUN_1406e8c20(param_3);
    FUN_14089e7b0(param_1,local_140 + iVar6);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x107);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140898040(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089bc90(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1a9);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c393a0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3e440(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1aa);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c393f0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3e490(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1ae);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140899440(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089c820(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x16b);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140899a80(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089cdc0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1b2);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c38900(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3d9a0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1b3);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c39da0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3ee40(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1b4);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3a4d0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3f520(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1b5);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3c5a0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c41640(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1ba);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_1408976c0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089b470(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1b7);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_14089a070(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089d220(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1c6);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c37640(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3c6e0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1d4);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3c230(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c412d0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1d6);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_1408983b0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089bfb0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1db);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140898220(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089be70(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x8a);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140898900(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089c280(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xc0);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3bdd0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c40e70(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1dd);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_14089a570(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089d6d0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1e5);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140897e60(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089bb00(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1e3);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140897ff0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089bc40(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1e4);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c387c0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3d860(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1e6);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_14089a6b0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089d810(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1e7);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_14089a700(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_14089d860(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1ec);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_140c3abb0(param_1,uVar3);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3fc50(param_1,uVar5);
  }
  uVar2 = FUN_1406e8ae0(param_3);
  FUN_1408977b0(param_1,uVar2);
  uVar2 = FUN_1406e8ae0(param_3);
  FUN_140897860(param_1,uVar2);
  uVar5 = FUN_1406e8c20(param_3);
  FUN_1408a0fb0(param_1,uVar5);
  cVar1 = FUN_1402bf6d0(local_118,0x17d);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c374b0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x169);
  if (cVar1 != '\0') {
    uVar3 = FUN_1406e8b80(param_3);
    FUN_1408a14b0(param_1,uVar3);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x109);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140896630(param_1,uVar5);
  }
  local_130 = param_1 + 0x24e0;
  FUN_140862470(local_130,param_3);
  FUN_140822790(param_1 + 0x2530);
  for (local_154 = 0; local_154 < 8; local_154 = local_154 + 1) {
    uVar7 = FUN_1402bf710(local_154);
    uVar7 = FUN_14080fb80(local_118,local_98,uVar7);
    cVar1 = FUN_14080fa00(uVar7);
    if (cVar1 != '\0') {
      local_138 = (longlong *)FUN_140878810(param_1,local_154);
      local_128 = *(code **)(*local_138 + 0x30);
      (*local_128)(local_138,param_3);
    }
  }
  FUN_14087ae30(param_1,local_118,param_3,1);
  cVar1 = FUN_1402bf6d0(local_118,0x18d);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_1408a1460(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0xf8);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_1408a1320(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1ae);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_1408a13c0(param_1,uVar5);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140896530(param_1,uVar5);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140896720(param_1,uVar5);
  }
  uVar2 = FUN_1406e8ae0(param_3);
  FUN_14089a340(param_1,uVar2);
  cVar1 = FUN_1402bf6d0(local_118,0x1b3);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c469b0(param_1,uVar5);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c372d0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1b4);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c46a00(param_1,uVar5);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c37370(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1b5);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c46d20(param_1,uVar5);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c37500(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1b7);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_1408a16e0(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x8a);
  if (cVar1 != '\0') {
    local_13c = (*DAT_143262db0)();
    iVar6 = FUN_1406e8c20(param_3);
    FUN_14089f390(param_1,local_13c + iVar6);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140896490(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x1e3);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c46960(param_1,uVar5);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c37280(param_1,uVar5);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c46e60(param_1,uVar5);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c46d70(param_1,uVar5);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c46e10(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x74);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c46cd0(param_1,uVar5);
    uVar5 = FUN_1406e8c20(param_3);
    FUN_140c3c640(param_1,uVar5);
  }
  cVar1 = FUN_1402bf6d0(local_118,0x6b);
  if (cVar1 != '\0') {
    uVar5 = FUN_1406e8c20(param_3);
    FUN_1408a0d80(param_1,uVar5);
  }
  FUN_1402c23d0(param_2,local_118,0x3e0);
  return param_2;
}



//===========================================================
// FUN_1429be2e0 @ 1429be2e0   (642 bytes)
//===========================================================

undefined8 * FUN_1429be2e0(longlong *param_1,uint *param_2,longlong param_3)

{
  uint uVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  void *pvVar5;
  uint uVar6;
  uint uVar7;
  longlong lVar8;
  undefined8 *puVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  longlong *plVar12;
  int *piVar13;
  uint *puVar14;
  
  if (*param_1 == 0) {
    uVar6 = *(uint *)(param_1 + 1);
    lVar8 = 0;
    uVar1 = uVar6;
  }
  else {
    uVar6 = FUN_1402fa540(param_1 + 2);
    if (uVar6 <= *(uint *)((longlong)param_1 + 0x2c)) goto LAB_1429be42e;
    uVar6 = *(uint *)(param_1 + 1);
    uVar1 = uVar6 * 2;
    lVar8 = *param_1;
  }
  if ((uVar1 != 0) && ((uVar6 != uVar1 || (lVar8 == 0)))) {
    puVar14 = (uint *)&DAT_143486200;
    uVar11 = 0xf6;
    do {
      uVar10 = uVar11 >> 1;
      if (puVar14[uVar10] < uVar1) {
        puVar14 = puVar14 + uVar10 + 1;
        uVar10 = uVar11 + (-1 - uVar10);
      }
      uVar11 = uVar10;
    } while (0 < (longlong)uVar10);
    uVar1 = *puVar14;
    plVar2 = (longlong *)*param_1;
    *(uint *)(param_1 + 1) = uVar1;
    uVar7 = 0xffffffff;
    if ((int)param_1[5] != -1) {
      uVar7 = (int)param_1[5] * uVar1 >> 7;
    }
    *(uint *)((longlong)param_1 + 0x2c) = uVar7;
    lVar8 = FUN_14019b780(&DAT_143ad68a0);
    *param_1 = lVar8;
    FUN_142ef8250(lVar8,0,(ulonglong)uVar1 * 8);
    plVar12 = plVar2;
    if (plVar2 != (longlong *)0x0) {
      while (plVar12 < plVar2 + uVar6) {
        lVar3 = *plVar12;
        plVar12 = plVar12 + 1;
        while (lVar3 != 0) {
          uVar11 = (ulonglong)*(uint *)(lVar3 + 0x10) % (ulonglong)uVar1;
          lVar4 = *(longlong *)(lVar3 + 8);
          *(undefined8 *)(lVar3 + 8) = *(undefined8 *)(lVar8 + uVar11 * 8);
          *(longlong *)(lVar8 + uVar11 * 8) = lVar3;
          lVar3 = lVar4;
        }
      }
      FUN_14019b4e0(plVar2);
    }
  }
LAB_1429be42e:
  plVar2 = (longlong *)(*param_1 + ((ulonglong)*param_2 % (ulonglong)*(uint *)(param_1 + 1)) * 8);
  for (puVar9 = (undefined8 *)*plVar2; puVar9 != (undefined8 *)0x0; puVar9 = (undefined8 *)puVar9[1]
      ) {
    if (*(uint *)(puVar9 + 2) == *param_2) {
      if (param_3 == 0) {
        return puVar9;
      }
      FUN_1429bd510(puVar9 + 3,param_3);
      return puVar9;
    }
  }
  FUN_141d4caf0(param_1 + 2);
  pvVar5 = Self;
  lVar3 = DAT_143adc380;
  plVar12 = (longlong *)(DAT_143adc380 + 0x18);
  LOCK();
  lVar8 = *plVar12;
  if (lVar8 == 0) {
    *plVar12 = (longlong)Self;
  }
  UNLOCK();
  if (lVar8 == 0) {
LAB_1429be4d9:
    *(undefined4 *)(lVar3 + 0x20) = 1;
  }
  else if ((void *)*plVar12 == pvVar5) {
    *(int *)(lVar3 + 0x20) = *(int *)(lVar3 + 0x20) + 1;
  }
  else {
    while( true ) {
      pvVar5 = Self;
      LOCK();
      lVar8 = *plVar12;
      if (lVar8 == 0) {
        *plVar12 = (longlong)Self;
      }
      UNLOCK();
      if (lVar8 == 0) goto LAB_1429be4d9;
      if ((void *)*plVar12 == pvVar5) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar3 + 0x20) = *(int *)(lVar3 + 0x20) + 1;
  }
  piVar13 = (int *)(lVar3 + 0x20);
  puVar9 = *(undefined8 **)(lVar3 + 0x28);
  if (puVar9 == (undefined8 *)0x0) {
    puVar9 = (undefined8 *)FUN_14019d3c0(0x28);
    *(undefined8 **)(lVar3 + 0x28) = puVar9;
  }
  *(undefined8 *)(lVar3 + 0x28) = *puVar9;
  *piVar13 = *piVar13 + -1;
  if (*piVar13 == 0) {
    *plVar12 = 0;
  }
  lVar8 = *plVar2;
  *puVar9 = &PTR_FUN_143486650;
  puVar9[1] = lVar8;
  *(undefined4 *)(puVar9 + 2) = 0;
  puVar9[4] = 0;
  *(uint *)(puVar9 + 2) = *param_2;
  if (param_3 != 0) {
    FUN_1429bd510(puVar9 + 3,param_3);
  }
  *plVar2 = (longlong)puVar9;
  return puVar9;
}



//===========================================================
// FUN_1429b9300 @ 1429b9300   (386 bytes)
//===========================================================

void FUN_1429b9300(longlong param_1,int param_2,undefined8 param_3)

{
  longlong lVar1;
  undefined1 local_38 [33];
  undefined1 local_17;
  
  if (param_2 == 0x224) {
    FUN_1429ba3e0(param_1,param_3);
  }
  else {
    if (param_2 == 0x225) {
      FUN_1429ba980(param_1,param_3);
      return;
    }
    if (param_2 - 0x226U < 0x6d) {
      FUN_1429bafb0(param_1);
      return;
    }
    if (param_2 - 0x293U < 0x32) {
      FUN_1429bb720(param_1);
      return;
    }
    if (param_2 - 0x2c5U < 0xda) {
      if (DAT_143ac87a0 != 0) {
        FUN_1415ed8e0(local_38);
        local_17 = 1;
        FUN_1415f0bf0(DAT_143ac87a0,local_38);
        FUN_1415ed910(local_38);
      }
      if (param_2 == 0x33b) {
        lVar1 = *(longlong *)(param_1 + 0x10);
        if (lVar1 == 0) {
          FUN_142e52ed0(0x431,0);
          lVar1 = *(longlong *)(param_1 + 0x10);
        }
        *(undefined4 *)(lVar1 + 0x5528) = 0;
      }
      if (*(longlong *)(param_1 + 0x10) != 0) {
        FUN_14289a3a0(*(longlong *)(param_1 + 0x10),param_2,param_3);
      }
      if (param_2 == 0x33b) {
        lVar1 = *(longlong *)(param_1 + 0x10);
        if (lVar1 == 0) {
          FUN_142e52ed0(0x431,0);
          lVar1 = *(longlong *)(param_1 + 0x10);
        }
        if (*(int *)(lVar1 + 0x5528) == 0) {
          FUN_142d142a0(DAT_143aa84a0);
        }
      }
      if (DAT_143ac87a0 != 0) {
        FUN_1415f0d70();
        return;
      }
    }
  }
  return;
}



//===========================================================
// FUN_1429bb720 @ 1429bb720   (1300 bytes)
//===========================================================

ulonglong FUN_1429bb720(longlong param_1,int param_2,undefined8 param_3)

{
  longlong *plVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  longlong lVar8;
  undefined8 uVar9;
  int iVar10;
  undefined1 local_38 [33];
  undefined1 local_17;
  
  uVar6 = FUN_1406e8c20(param_3);
  if (*(longlong *)(param_1 + 0xf8) == 0) {
    return uVar6;
  }
  uVar7 = (uVar6 & 0xffffffff) / (ulonglong)*(uint *)(param_1 + 0x100);
  lVar8 = *(longlong *)
           (*(longlong *)(param_1 + 0xf8) +
           ((uVar6 & 0xffffffff) % (ulonglong)*(uint *)(param_1 + 0x100)) * 8);
  if (lVar8 == 0) {
    return uVar7;
  }
  while (*(int *)(lVar8 + 0x10) != (int)uVar6) {
    lVar8 = *(longlong *)(lVar8 + 8);
    if (lVar8 == 0) {
      return uVar7;
    }
  }
  uVar7 = lVar8 + 0x18;
  if (uVar7 == 0) {
    return 0;
  }
  if (*(longlong *)(lVar8 + 0x20) == 0) {
    return uVar7;
  }
  plVar1 = *(longlong **)(*(longlong *)(lVar8 + 0x20) + 0x28);
  if (plVar1 == (longlong *)0x0) {
    return uVar7;
  }
  uVar7 = FUN_142cc3d80(DAT_143aa84a0);
  if ((int)uVar7 != 0) {
    return uVar7;
  }
  lVar8 = FUN_141892840();
  if (lVar8 != 0) {
    uVar9 = FUN_141892840();
    uVar7 = FUN_141bc8c60(uVar9);
    if ((char)uVar7 != '\0') {
      return uVar7;
    }
  }
  lVar8 = FUN_141892840();
  if (lVar8 != 0) {
    uVar9 = FUN_141892840();
    cVar2 = FUN_141bc8c80(uVar9);
    if (cVar2 != '\0') {
      uVar7 = thunk_FUN_1413b8e00(DAT_143aa84a0);
      if ((int)uVar7 == 0) {
        return uVar7;
      }
      uVar7 = FUN_142dec860(DAT_143aa84a0,uVar6 & 0xffffffff);
      if ((int)uVar7 == 0) {
        return uVar7;
      }
    }
  }
  lVar8 = FUN_141892840();
  if (lVar8 != 0) {
    uVar9 = FUN_141892840();
    cVar2 = FUN_141bc8ca0(uVar9);
    if (cVar2 != '\0') {
      uVar7 = FUN_142cc03f0(DAT_143aa84a0);
      if ((int)uVar7 == 0) {
        return uVar7;
      }
      uVar6 = FUN_142cc0980(DAT_143aa84a0,uVar6 & 0xffffffff);
      if ((char)uVar6 == '\0') {
        return uVar6;
      }
    }
  }
  if (DAT_143ac87a0 != 0) {
    FUN_1415ed8e0(local_38);
    iVar3 = (**(code **)(*plVar1 + 0x50))(plVar1);
    local_17 = iVar3 != 0;
    FUN_1415f0bf0(DAT_143ac87a0,local_38);
    FUN_1415ed910(local_38);
  }
  uVar6 = FUN_140f8abc0(plVar1 + 0x20);
  if (((int)uVar6 != 0) || ((int)plVar1[0x81b] != 0)) {
    if ((param_2 == 0x29e) || (((param_2 == 0x29f || (param_2 == 0x2a0)) || (param_2 == 0x2a1)))) {
      uVar6 = FUN_1429df530(plVar1,param_2,param_3);
    }
    goto switchD_1429bb95c_caseD_c;
  }
  iVar3 = (*DAT_143262db0)();
  lVar8 = (**(code **)(*plVar1 + 0x48))(plVar1);
  iVar4 = FUN_1401ba9d0(lVar8 + 0xd00,*(undefined4 *)(lVar8 + 0xd08));
  iVar10 = 100;
  if (0 < iVar4) {
    iVar10 = 200;
  }
  if (iVar3 - (int)plVar1[0x81a] < 1000) {
    *(int *)((longlong)plVar1 + 0x40d4) = *(int *)((longlong)plVar1 + 0x40d4) + 1;
    if (iVar10 < *(int *)((longlong)plVar1 + 0x40d4)) {
      *(undefined4 *)(plVar1 + 0x81b) = 1;
    }
  }
  else {
    *(int *)(plVar1 + 0x81a) = iVar3;
    *(undefined4 *)((longlong)plVar1 + 0x40d4) = 1;
  }
  uVar5 = param_2 - 0x29e;
  uVar6 = (ulonglong)uVar5;
  if (0x26 < uVar5) goto switchD_1429bb95c_caseD_c;
  uVar6 = (ulonglong)(int)uVar5;
  switch(uVar5) {
  case 0:
  case 1:
  case 2:
  case 3:
    uVar6 = FUN_1429d2ee0(plVar1,param_2,param_3);
  default:
switchD_1429bb95c_caseD_c:
    if (0x30 < param_2 - 0x293U) goto switchD_1429bbb36_caseD_1;
    break;
  case 4:
    FUN_1429d4100(plVar1,param_3);
    break;
  case 5:
    FUN_1429d4390(plVar1,param_3);
    break;
  case 6:
    FUN_1429d4680(plVar1,param_3);
    break;
  case 7:
    FUN_1429d48c0(plVar1,param_3);
    break;
  case 8:
    FUN_1427862e0(plVar1,param_3);
    break;
  case 9:
    FUN_142786350(plVar1,param_3);
    break;
  case 10:
    FUN_1429d4f20(plVar1,param_3);
    break;
  case 0xb:
    thunk_FUN_14277e150(plVar1,param_3);
    break;
  case 0xe:
    FUN_1429d4f60(plVar1,param_3);
    break;
  case 0x11:
    FUN_1427863f0(plVar1,param_3);
    break;
  case 0x19:
    FUN_1429d5f70(plVar1,param_3);
    break;
  case 0x1d:
    FUN_1429d6010(plVar1,param_3);
    break;
  case 0x1e:
    FUN_1429d6100(plVar1,param_3);
    break;
  case 0x1f:
    FUN_1429d7130(plVar1,param_3);
    break;
  case 0x21:
    FUN_1429de1b0(plVar1,param_3);
    break;
  case 0x22:
    FUN_1429deb60(plVar1,param_3);
    break;
  case 0x23:
    FUN_1429deae0(plVar1,param_3);
    break;
  case 0x26:
    uVar6 = FUN_1429d5d90(plVar1,param_3);
    goto switchD_1429bbb36_caseD_1;
  }
  uVar6 = (ulonglong)(&switchD_1429bbb36::switchdataD_1429bbd10)[param_2 + -0x293];
                    /* WARNING: Could not find normalized switch variable to match jumptable */
  switch((&switchD_1429bbb36::switchdataD_1429bbd10)[param_2 + -0x293]) {
  case 0:
    uVar6 = FUN_1429d2e70(plVar1,param_3);
    break;
  case 0x17:
    uVar6 = FUN_1429d6f50(plVar1,param_3);
    break;
  case 0x1a:
    uVar6 = FUN_1429d4fd0(plVar1,param_3);
    break;
  case 0x1b:
    uVar6 = FUN_1429d5290(plVar1,param_3);
    break;
  case 0x1d:
    uVar6 = FUN_1429d62f0(plVar1,param_3);
    break;
  case 0x1e:
    uVar6 = FUN_1429d6500(plVar1,param_3);
    break;
  case 0x1f:
    uVar6 = FUN_1429d5610(plVar1,param_3);
    break;
  case 0x20:
    uVar6 = FUN_1429d5710(plVar1,param_3);
    break;
  case 0x21:
    uVar6 = FUN_1429d5a40(plVar1,param_3);
    break;
  case 0x22:
    uVar6 = FUN_1429d5ac0(plVar1,param_3);
    break;
  case 0x23:
    uVar6 = FUN_1429d7020(plVar1,param_3);
    break;
  case 0x26:
    uVar6 = FUN_1429d70f0(plVar1,param_3);
    break;
  case 0x2b:
    uVar6 = FUN_1429ddb20(plVar1,param_3);
    break;
  case 0x2f:
    uVar6 = FUN_1429d7170(plVar1,param_3);
    break;
  case 0x30:
    uVar6 = FUN_1429d7220(plVar1,param_3);
  }
switchD_1429bbb36_caseD_1:
  if (DAT_143ac87a0 != 0) {
    uVar6 = FUN_1415f0d70();
  }
  return uVar6;
}



//===========================================================
// FUN_1402ee8d0 @ 1402ee8d0   (621 bytes)
//===========================================================

void FUN_1402ee8d0(longlong param_1,undefined8 param_2,longlong *param_3)

{
  undefined1 uVar1;
  byte bVar2;
  char cVar3;
  undefined4 uVar4;
  int iVar5;
  
  uVar1 = FUN_1406e8ae0(param_2);
  *(undefined1 *)(param_1 + 0x20) = uVar1;
  bVar2 = FUN_1406e8ae0(param_2);
  *(uint *)(param_1 + 0x21) = (uint)bVar2;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x25) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x29) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x1bd) = uVar4;
  *(undefined8 *)(param_1 + 0x39) = 0;
  *(undefined8 *)(param_1 + 0x41) = 0;
  *(undefined8 *)(param_1 + 0x49) = 0;
  *(undefined8 *)(param_1 + 0x51) = 0;
  *(undefined8 *)(param_1 + 0x59) = 0;
  *(undefined8 *)(param_1 + 0x61) = 0;
  *(undefined8 *)(param_1 + 0x69) = 0;
  *(undefined8 *)(param_1 + 0x71) = 0;
  *(undefined8 *)(param_1 + 0x79) = 0;
  *(undefined8 *)(param_1 + 0x81) = 0;
  *(undefined8 *)(param_1 + 0x89) = 0;
  *(undefined8 *)(param_1 + 0x91) = 0;
  *(undefined8 *)(param_1 + 0x99) = 0;
  *(undefined8 *)(param_1 + 0xa1) = 0;
  *(undefined8 *)(param_1 + 0xa9) = 0;
  *(undefined8 *)(param_1 + 0xb1) = 0;
  *(undefined8 *)(param_1 + 0xb9) = 0;
  *(undefined8 *)(param_1 + 0xc1) = 0;
  *(undefined8 *)(param_1 + 0xc9) = 0;
  *(undefined8 *)(param_1 + 0xd1) = 0;
  *(undefined8 *)(param_1 + 0xd9) = 0;
  *(undefined8 *)(param_1 + 0xe1) = 0;
  *(undefined8 *)(param_1 + 0xe9) = 0;
  *(undefined8 *)(param_1 + 0xf1) = 0;
  *(undefined8 *)(param_1 + 0xf9) = 0;
  *(undefined8 *)(param_1 + 0x101) = 0;
  *(undefined8 *)(param_1 + 0x109) = 0;
  *(undefined8 *)(param_1 + 0x111) = 0;
  *(undefined8 *)(param_1 + 0x119) = 0;
  *(undefined8 *)(param_1 + 0x121) = 0;
  *(undefined8 *)(param_1 + 0x129) = 0;
  *(undefined8 *)(param_1 + 0x131) = 0;
  FUN_1406e8ae0(param_2);
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x39) = uVar4;
  bVar2 = FUN_1406e8ae0(param_2);
  while (bVar2 != 0xff) {
    uVar4 = FUN_1406e8c20(param_2);
    if (((byte)(bVar2 - 1) < 0x1f) && (iVar5 = FUN_140253980(uVar4,bVar2,2,1), iVar5 != 0)) {
      *(undefined4 *)(param_1 + 0x39 + (ulonglong)bVar2 * 4) = uVar4;
    }
    bVar2 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1406e8ae0(param_2);
  while (bVar2 != 0xff) {
    uVar4 = FUN_1406e8c20(param_2);
    if (((byte)(bVar2 - 1) < 0x1f) && (iVar5 = FUN_140253980(uVar4,bVar2,2,1), iVar5 != 0)) {
      *(undefined4 *)(param_1 + 0xb9 + (ulonglong)bVar2 * 4) = uVar4;
    }
    bVar2 = FUN_1406e8ae0(param_2);
  }
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x2d) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x31) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x35) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x1c1) = uVar4;
  iVar5 = FUN_1406e8c20(param_2);
  if (iVar5 < 1) {
    iVar5 = 0;
  }
  else {
    iVar5 = iVar5 % 0x168;
  }
  *(int *)(param_1 + 0x1c5) = iVar5;
  cVar3 = FUN_1406e8ae0(param_2);
  *(bool *)(param_1 + 0x1c9) = cVar3 != '\0';
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x1ca) = uVar4;
  FUN_1406e9170(param_2,param_1 + 0x1b9,4);
  FUN_1406e9170(param_2,param_1 + 0x139,0x80);
  uVar4 = thunk_FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x1d2) = uVar4;
  FUN_1406e9170(param_2,param_1 + 0x1d6,0xd);
  if (*param_3 != 0) {
    FUN_14019f2c0(*param_3 + -0x10);
  }
  return;
}


