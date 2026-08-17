
//===========================================================
// FUN_141b30230 @ 141b30230   (1391 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x000141b30727) */
/* WARNING: Removing unreachable block (ram,0x000141b30538) */

void FUN_141b30230(int *param_1,undefined8 param_2)

{
  undefined2 *puVar1;
  longlong lVar2;
  char cVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  int *piVar7;
  int *piVar8;
  undefined8 uVar9;
  ulonglong uVar10;
  longlong lVar11;
  longlong lVar12;
  int *piVar13;
  int *piVar14;
  ulonglong uVar15;
  uint uVar16;
  undefined4 uVar17;
  undefined8 auStack_90 [5];
  undefined8 local_68;
  undefined4 local_60 [2];
  int *local_58;
  int local_50;
  longlong local_48;
  int *local_40;
  int *local_38;
  ulonglong local_30;
  
  local_30 = DAT_143a8b908 ^ (ulonglong)&local_58;
  if (DAT_143aa8520 != 0) {
    auStack_90[0] = 0x141b30274;
    FUN_142bf3f70();
    if (DAT_143aa8520 != 0) {
      auStack_90[0] = 0x141b3028e;
      (*(code *)**(undefined8 **)(DAT_143aa8520 + 8))((undefined8 *)(DAT_143aa8520 + 8),1);
    }
  }
  auStack_90[0] = 0x141b30296;
  cVar3 = FUN_1406e8ae0(param_2);
  auStack_90[0] = 0x141b302a6;
  FUN_1406e9050(param_2,&local_48);
  piVar13 = (int *)0x0;
  if (local_48 == 0) {
    iVar4 = 2;
  }
  else {
    local_60[0] = 0;
    local_68 = 0;
    auStack_90[0] = 0x141b302d1;
    iVar4 = (*DAT_1432627f8)(0xfde9,0,local_48,0xffffffff);
    iVar4 = iVar4 * 2;
  }
  lVar11 = local_48;
  uVar10 = (longlong)iVar4 + 0xf;
  if (uVar10 <= (ulonglong)(longlong)iVar4) {
    uVar10 = 0xffffffffffffff0;
  }
  auStack_90[0] = 0x141b302f8;
  lVar2 = -(uVar10 & 0xfffffffffffffff0);
  puVar1 = (undefined2 *)((longlong)&local_58 + lVar2);
  if (local_48 == 0) {
    if (puVar1 == (undefined2 *)0x0) goto LAB_141b3032f;
    *puVar1 = 0;
LAB_141b3033f:
    uVar10 = 0xffffffffffffffff;
    do {
      uVar10 = uVar10 + 1;
    } while (puVar1[uVar10] != 0);
    iVar5 = (int)uVar10;
    iVar4 = 0;
    if (0 < iVar5) {
      iVar4 = iVar5;
    }
    *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b30371;
    piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar4 * 2 + 0x12));
    piVar7[1] = iVar4;
    *piVar7 = -1;
    piVar13 = piVar7 + 4;
    piVar7[2] = 0;
    *(undefined2 *)piVar13 = 0;
    local_40 = piVar13;
    *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b3039f;
    FUN_142ef7ba0(piVar13,puVar1,(longlong)iVar5 * 2);
    if (*piVar7 != -1) {
      *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b303b1;
      FUN_142e52dd0(0x8b);
    }
    if ((iVar5 == -1) || (iVar4 = piVar7[1], iVar5 <= iVar4)) {
      *piVar7 = 1;
      if (iVar5 != -1) goto LAB_141b303d9;
      if (piVar13 == (int *)0x0) {
        uVar10 = 0;
      }
      else {
        uVar10 = 0xffffffffffffffff;
        do {
          uVar10 = uVar10 + 1;
        } while (*(short *)((longlong)piVar13 + uVar10 * 2) != 0);
      }
    }
    else {
      *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b303d2;
      FUN_142e54290(0x90,iVar4,uVar10 & 0xffffffff);
      *piVar7 = 1;
LAB_141b303d9:
      *(undefined2 *)((longlong)iVar5 * 2 + (longlong)piVar13) = 0;
    }
    iVar4 = (int)uVar10;
    if ((iVar4 < 0) || (piVar7[1] + 1 <= iVar4)) {
      *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b303fc;
      FUN_142e54290(0x9c,uVar10 & 0xffffffff);
    }
    piVar7[2] = iVar4 * 2;
  }
  else {
    *(undefined4 *)((longlong)local_60 + lVar2) = 0x100000;
    *(undefined2 **)((longlong)local_60 + lVar2 + -8) = puVar1;
    *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b3032f;
    (*DAT_1432627f8)(0xfde9,0,lVar11,0xffffffff);
LAB_141b3032f:
    local_40 = (int *)0x0;
    if (puVar1 != (undefined2 *)0x0) goto LAB_141b3033f;
  }
  uVar16 = 0;
  *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b3040c;
  iVar4 = FUN_1406e8c20(param_2);
  *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b30417;
  iVar5 = FUN_1406e8c20(param_2);
  if (cVar3 == '\0') {
    lVar11 = 0;
    while( true ) {
      lVar12 = *(longlong *)(param_1 + 0x40);
      if ((lVar12 == 0) || (*(uint *)(lVar12 + -8) <= uVar16)) goto LAB_141b30735;
      if ((int)uVar16 < 0) {
        *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b3066d;
        FUN_142e54290(0xc6,uVar16);
        lVar12 = *(longlong *)(param_1 + 0x40);
      }
      if (*(int *)(lVar11 + lVar12) == iVar4) break;
      uVar16 = uVar16 + 1;
      lVar11 = lVar11 + 0x68;
    }
    uVar17 = 0;
    uVar6 = 0;
    if (lVar12 != 0) {
      uVar6 = *(uint *)(lVar12 + -8);
    }
    if (((int)uVar16 < 0) || (uVar6 <= uVar16)) {
      if (lVar12 != 0) {
        uVar17 = *(undefined4 *)(lVar12 + -8);
      }
      *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b306ae;
      FUN_142e54290(0xc6,uVar16,uVar17);
      lVar12 = *(longlong *)(param_1 + 0x40);
    }
    if ((longlong)(int)uVar16 * 0x68 + lVar12 != 0) {
      local_58 = param_1;
      local_50 = iVar4;
      *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b306da;
      piVar7 = (int *)FUN_14019b780(&DAT_143ad68a0,0x300);
      local_38 = piVar7;
      if (piVar7 != (int *)0x0) {
        *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b306ef;
        FUN_141b6be60(piVar7,&local_58);
      }
      if (iVar5 != -1) {
        *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b306fb;
        lVar11 = FUN_141b41cc0();
        if (lVar11 != 0) {
          *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b3070b;
          FUN_141b6b5a0(lVar11,iVar5);
        }
      }
      if ((DAT_143abfdf8 != 0) && (DAT_143aa8520 != 0)) {
        *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b30733;
        FUN_142c0bf50(DAT_143abfdf8,DAT_143aa8520 + 8,0);
      }
      goto LAB_141b3074e;
    }
LAB_141b30735:
    *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b30743;
    uVar9 = FUN_1408a9e40(&local_58,0x1286);
    *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b3074d;
    FUN_140d84b70(uVar9,0);
    goto LAB_141b3074e;
  }
  local_58 = (int *)0x0;
  piVar14 = piVar13;
  piVar7 = local_58;
  if ((piVar13 != (int *)0x0) && (piVar8 = piVar13 + -4, piVar8 != (int *)0x0)) {
    if (*piVar8 == -1) {
      *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b30454;
      FUN_142e52d50(0xcb,0xffffff01);
      uVar10 = 0xffffffffffffffff;
      do {
        uVar10 = uVar10 + 1;
      } while (*(short *)((longlong)piVar13 + uVar10 * 2) != 0);
      uVar15 = 0;
      iVar5 = (int)uVar10;
      iVar4 = 0;
      if (0 < iVar5) {
        iVar4 = iVar5;
      }
      *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b3048b;
      piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar4 * 2 + 0x12));
      piVar8[1] = iVar4;
      *piVar8 = -1;
      piVar7 = piVar8 + 4;
      piVar8[2] = 0;
      *(undefined2 *)piVar7 = 0;
      local_38 = piVar7;
      *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b304b9;
      FUN_142ef7ba0(piVar7,piVar13,(longlong)iVar5 * 2);
      if (*piVar8 != -1) {
        *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b304cb;
        FUN_142e52dd0(0x8b);
      }
      if ((iVar5 == -1) || (iVar4 = piVar8[1], iVar5 <= iVar4)) {
        *piVar8 = 1;
        if (iVar5 != -1) goto LAB_141b304f3;
        if (piVar7 != (int *)0x0) {
          uVar15 = 0xffffffffffffffff;
          do {
            uVar15 = uVar15 + 1;
          } while (*(short *)((longlong)piVar7 + uVar15 * 2) != 0);
        }
      }
      else {
        *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b304ec;
        FUN_142e54290(0x90,iVar4,uVar10 & 0xffffffff);
        *piVar8 = 1;
LAB_141b304f3:
        *(undefined2 *)((longlong)piVar7 + (longlong)iVar5 * 2) = 0;
        uVar15 = uVar10;
      }
      iVar4 = (int)uVar15;
      if ((iVar4 < 0) || (piVar8[1] + 1 <= iVar4)) {
        *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b30514;
        FUN_142e54290(0x9c,uVar15 & 0xffffffff);
      }
      piVar8[2] = iVar4 * 2;
      if (local_58 != (int *)0x0) {
        piVar13 = local_58 + -4;
        *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b3052c;
        FUN_1401bebb0(piVar13);
      }
    }
    else {
      if (*piVar8 < 1) {
        *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b305c5;
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar8 = *piVar8 + 1;
      UNLOCK();
      piVar14 = local_40;
      piVar7 = piVar13;
      if (local_58 != (int *)0x0) {
        piVar13 = local_58 + -4;
        *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b305da;
        FUN_1401bebb0(piVar13);
        piVar14 = local_40;
      }
    }
  }
  local_58 = piVar7;
  *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b305fa;
  FUN_141b267c0(param_1,cVar3,0,&local_58);
  piVar13 = piVar14;
  if (DAT_143ad2230 != 0) {
    *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b30614;
    FUN_141b654a0(DAT_143ad2230,0xffffffff);
    lVar11 = DAT_143ad2230 + 8;
    if (DAT_143ad2230 == 0) {
      lVar11 = 0;
    }
    *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b30635;
    FUN_142c0bf50(DAT_143abfdf8,lVar11,0);
  }
LAB_141b3074e:
  param_1[0x35] = 0;
  if (piVar13 != (int *)0x0) {
    *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b30765;
    FUN_1401bebb0(piVar13 + -4);
  }
  if (local_48 != 0) {
    *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b30778;
    FUN_14019f2c0(local_48 + -0x10);
  }
  *(undefined8 *)((longlong)auStack_90 + lVar2) = 0x141b30785;
  return;
}



//===========================================================
// FUN_141b36f60 @ 141b36f60   (4821 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x000141b377e5) */
/* WARNING: Removing unreachable block (ram,0x000141b372f2) */
/* WARNING: Removing unreachable block (ram,0x000141b37988) */
/* WARNING: Removing unreachable block (ram,0x000141b37551) */

void FUN_141b36f60(longlong param_1,longlong param_2)

{
  undefined2 *puVar1;
  longlong lVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined2 uVar5;
  byte bVar6;
  undefined1 uVar7;
  char cVar8;
  int iVar9;
  undefined4 uVar10;
  uint uVar11;
  uint uVar12;
  int iVar13;
  int *piVar14;
  int *piVar15;
  undefined8 uVar16;
  undefined1 *puVar17;
  longlong lVar18;
  undefined1 *puVar19;
  undefined8 *puVar20;
  undefined8 uVar21;
  char *pcVar22;
  ulonglong uVar23;
  int **ppiVar24;
  longlong lVar25;
  wchar_t *pwVar26;
  undefined1 *puVar27;
  ulonglong uVar28;
  int *piVar29;
  int *piVar30;
  ulonglong uVar31;
  byte *pbVar32;
  uint uVar33;
  uint uVar34;
  undefined8 auStack_160 [5];
  undefined8 local_138;
  undefined4 local_130 [10];
  longlong local_108;
  wchar_t *local_100;
  int *local_f8;
  byte local_f0;
  undefined1 local_ef;
  uint local_e8 [2];
  undefined2 local_e0;
  undefined2 local_dc;
  int *local_d8;
  int *local_d0;
  int *local_c8;
  int *local_c0;
  int **local_b8;
  undefined4 local_b0;
  undefined4 local_ac;
  int local_a8;
  longlong local_a0;
  undefined4 local_98;
  undefined4 local_94;
  undefined4 local_90;
  undefined4 local_8c;
  longlong local_88;
  undefined8 local_80;
  undefined1 *local_78;
  int *local_70;
  undefined1 local_68 [8];
  undefined1 *local_60;
  undefined2 local_58;
  undefined2 local_56;
  undefined4 local_54;
  undefined8 local_50;
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)&local_108;
  piVar15 = (int *)0x0;
  *(undefined4 *)(param_1 + 0xd4) = 0;
  local_108 = param_1;
  local_a0 = param_2;
  if (DAT_143aca790 != 0) {
    auStack_160[0] = 0x141b36fb9;
    FUN_141179940();
  }
  auStack_160[0] = 0x141b36fc1;
  bVar6 = FUN_1406e8ae0(param_2);
  local_100 = (wchar_t *)CONCAT44(local_100._4_4_,(uint)bVar6);
  auStack_160[0] = 0x141b36fd8;
  FUN_1406e9050(param_2,&local_88);
  iVar9 = 2;
  if (local_88 != 0) {
    local_130[0] = 0;
    local_138 = 0;
    auStack_160[0] = 0x141b37005;
    iVar9 = (*DAT_1432627f8)(0xfde9,0,local_88,0xffffffff);
    iVar9 = iVar9 * 2;
  }
  lVar18 = local_88;
  uVar23 = (longlong)iVar9 + 0xf;
  if (uVar23 <= (ulonglong)(longlong)iVar9) {
    uVar23 = 0xffffffffffffff0;
  }
  auStack_160[0] = 0x141b3702f;
  lVar2 = -(uVar23 & 0xfffffffffffffff0);
  puVar1 = (undefined2 *)((longlong)&local_108 + lVar2);
  if (local_88 == 0) {
    if (puVar1 == (undefined2 *)0x0) goto LAB_141b37066;
    *puVar1 = 0;
LAB_141b37076:
    piVar30 = (int *)0xffffffffffffffff;
    do {
      piVar30 = (int *)((longlong)piVar30 + 1);
    } while (puVar1[(longlong)piVar30] != 0);
    iVar13 = (int)piVar30;
    iVar9 = 0;
    if (0 < iVar13) {
      iVar9 = iVar13;
    }
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b370a8;
    piVar14 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar9 * 2 + 0x12));
    piVar14[1] = iVar9;
    *piVar14 = -1;
    piVar29 = piVar14 + 4;
    piVar14[2] = 0;
    *(undefined2 *)piVar29 = 0;
    local_d8 = piVar29;
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b370d6;
    FUN_142ef7ba0(piVar29,puVar1,(longlong)iVar13 * 2);
    if (*piVar14 != -1) {
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b370e8;
      FUN_142e52dd0(0x8b);
    }
    if ((iVar13 == -1) || (iVar9 = piVar14[1], iVar13 <= iVar9)) {
      *piVar14 = 1;
      if (iVar13 != -1) goto LAB_141b37110;
      if (piVar29 != (int *)0x0) {
        piVar15 = (int *)0xffffffffffffffff;
        do {
          piVar15 = (int *)((longlong)piVar15 + 1);
        } while (*(short *)((longlong)piVar29 + (longlong)piVar15 * 2) != 0);
      }
    }
    else {
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37109;
      FUN_142e54290(0x90,iVar9,(ulonglong)piVar30 & 0xffffffff);
      *piVar14 = 1;
LAB_141b37110:
      *(undefined2 *)((longlong)iVar13 * 2 + (longlong)piVar29) = 0;
      piVar15 = piVar30;
    }
    iVar9 = (int)piVar15;
    if ((iVar9 < 0) || (piVar14[1] + 1 <= iVar9)) {
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37131;
      FUN_142e54290(0x9c,(ulonglong)piVar15 & 0xffffffff);
    }
    piVar14[2] = iVar9 * 2;
    piVar15 = piVar29;
    param_1 = local_108;
  }
  else {
    *(undefined4 *)((longlong)local_130 + lVar2) = 0x100000;
    *(undefined2 **)((longlong)local_130 + lVar2 + -8) = puVar1;
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37066;
    (*DAT_1432627f8)(0xfde9,0,lVar18,0xffffffff);
LAB_141b37066:
    local_d8 = (int *)0x0;
    if (puVar1 != (undefined2 *)0x0) goto LAB_141b37076;
  }
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37143;
  uVar7 = FUN_1406e8ae0(param_2);
  *(uint *)(param_1 + 0xd8) = (uint)(bVar6 == 0);
  if (bVar6 == 0x27) goto LAB_141b381e7;
  if (bVar6 == 0x37) {
    local_b8 = &local_f8;
    local_f8 = (int *)0x0;
    piVar29 = piVar15;
    piVar30 = local_f8;
    if ((piVar15 != (int *)0x0) && (piVar14 = piVar15 + -4, piVar14 != (int *)0x0)) {
      if (*piVar14 == -1) {
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b371a5;
        FUN_142e52d50(0xcb,0xffffff01);
        uVar23 = 0xffffffffffffffff;
        do {
          uVar23 = uVar23 + 1;
        } while (*(short *)((longlong)piVar15 + uVar23 * 2) != 0);
        iVar13 = (int)uVar23;
        iVar9 = 0;
        if (0 < iVar13) {
          iVar9 = iVar13;
        }
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b371db;
        piVar14 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar9 * 2 + 0x12));
        piVar14[1] = iVar9;
        *piVar14 = -1;
        piVar30 = piVar14 + 4;
        piVar14[2] = 0;
        *(undefined2 *)piVar30 = 0;
        local_c0 = piVar30;
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37209;
        FUN_142ef7ba0(piVar30,piVar15,(longlong)iVar13 * 2);
        if (*piVar14 != -1) {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3721b;
          FUN_142e52dd0(0x8b);
        }
        if ((iVar13 == -1) || (iVar9 = piVar14[1], iVar13 <= iVar9)) {
          *piVar14 = 1;
          if (iVar13 != -1) goto LAB_141b37243;
          if (piVar30 == (int *)0x0) {
            uVar23 = 0;
          }
          else {
            uVar23 = 0xffffffffffffffff;
            do {
              uVar23 = uVar23 + 1;
            } while (*(short *)((longlong)piVar30 + uVar23 * 2) != 0);
          }
        }
        else {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3723c;
          FUN_142e54290(0x90,iVar9,uVar23 & 0xffffffff);
          *piVar14 = 1;
LAB_141b37243:
          *(undefined2 *)((longlong)iVar13 * 2 + (longlong)piVar30) = 0;
        }
        iVar9 = (int)uVar23;
        if ((iVar9 < 0) || (piVar14[1] + 1 <= iVar9)) {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37264;
          FUN_142e54290(0x9c,uVar23 & 0xffffffff);
        }
        piVar14[2] = iVar9 * 2;
        param_1 = local_108;
        if (local_f8 != (int *)0x0) {
          piVar15 = local_f8 + -4;
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3727c;
          FUN_1401bebb0(piVar15);
          param_1 = local_108;
        }
      }
      else {
        if (*piVar14 < 1) {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37310;
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar14 = *piVar14 + 1;
        UNLOCK();
        piVar29 = local_d8;
        piVar30 = piVar15;
        if (local_f8 != (int *)0x0) {
          piVar15 = local_f8 + -4;
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37325;
          FUN_1401bebb0(piVar15);
          piVar29 = local_d8;
        }
      }
    }
    local_f8 = piVar30;
    local_100 = (wchar_t *)0x0;
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37346;
    piVar15 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3a);
    piVar15[1] = 0x14;
    *piVar15 = -1;
    local_100 = (wchar_t *)(piVar15 + 4);
    piVar15[2] = 0;
    *local_100 = L'\0';
    uVar16 = u_clientAlreadyRunning_1433fe7c8._8_8_;
    *(undefined8 *)local_100 = u_clientAlreadyRunning_1433fe7c8._0_8_;
    *(undefined8 *)(piVar15 + 6) = uVar16;
    uVar4 = u_clientAlreadyRunning_1433fe7c8._28_4_;
    uVar3 = u_clientAlreadyRunning_1433fe7c8._24_4_;
    uVar10 = u_clientAlreadyRunning_1433fe7c8._20_4_;
    piVar15[8] = u_clientAlreadyRunning_1433fe7c8._16_4_;
    piVar15[9] = uVar10;
    piVar15[10] = uVar3;
    piVar15[0xb] = uVar4;
    *(undefined8 *)(piVar15 + 0xc) = u_clientAlreadyRunning_1433fe7c8._32_8_;
    if (*piVar15 != -1) {
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b373a1;
      FUN_142e52dd0(0x8b);
    }
    iVar9 = piVar15[1];
    if (iVar9 < 0x14) {
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b373b7;
      FUN_142e54290(0x90,iVar9,0x14);
    }
    *piVar15 = 1;
    local_100[0x14] = L'\0';
    if (piVar15[1] + 1 < 0x15) {
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b373e2;
      FUN_142e54290(0x9c,0x14);
    }
    piVar15[2] = 0x28;
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b373f9;
    FUN_141b4ac80(&local_100,&local_f8,0);
    *(undefined4 *)(param_1 + 0xf0) = 1;
    piVar15 = piVar29;
    goto LAB_141b381e7;
  }
  if (bVar6 == 0x43) {
    local_b8 = &local_f8;
    local_f8 = (int *)0x0;
    piVar29 = piVar15;
    piVar30 = local_f8;
    if ((piVar15 != (int *)0x0) && (piVar14 = piVar15 + -4, piVar14 != (int *)0x0)) {
      if (*piVar14 == -1) {
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3744c;
        FUN_142e52d50(0xcb,0xffffff01);
        uVar23 = 0xffffffffffffffff;
        do {
          uVar23 = uVar23 + 1;
        } while (*(short *)((longlong)piVar15 + uVar23 * 2) != 0);
        iVar13 = (int)uVar23;
        iVar9 = 0;
        if (0 < iVar13) {
          iVar9 = iVar13;
        }
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3747e;
        piVar14 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar9 * 2 + 0x12));
        piVar14[1] = iVar9;
        *piVar14 = -1;
        piVar30 = piVar14 + 4;
        piVar14[2] = 0;
        *(undefined2 *)piVar30 = 0;
        local_c0 = piVar30;
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b374ac;
        FUN_142ef7ba0(piVar30,piVar15,(longlong)iVar13 * 2);
        if (*piVar14 != -1) {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b374be;
          FUN_142e52dd0(0x8b);
        }
        if ((iVar13 == -1) || (iVar9 = piVar14[1], iVar13 <= iVar9)) {
          *piVar14 = 1;
          if (iVar13 != -1) goto LAB_141b374de;
          if (piVar30 == (int *)0x0) {
            uVar23 = 0;
          }
          else {
            uVar23 = 0xffffffffffffffff;
            do {
              uVar23 = uVar23 + 1;
            } while (*(short *)((longlong)piVar30 + uVar23 * 2) != 0);
          }
        }
        else {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b374d7;
          FUN_142e54290(0x90,iVar9,uVar23 & 0xffffffff);
          *piVar14 = 1;
LAB_141b374de:
          *(undefined2 *)((longlong)iVar13 * 2 + (longlong)piVar30) = 0;
        }
        iVar9 = (int)uVar23;
        if ((iVar9 < 0) || (piVar14[1] + 1 <= iVar9)) {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b374ff;
          FUN_142e54290(0x9c,uVar23 & 0xffffffff);
        }
        piVar14[2] = iVar9 * 2;
        if (local_f8 != (int *)0x0) {
          piVar15 = local_f8 + -4;
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37517;
          FUN_1401bebb0(piVar15);
        }
      }
      else {
        if (*piVar14 < 1) {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3756b;
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar14 = *piVar14 + 1;
        UNLOCK();
        piVar29 = local_d8;
        piVar30 = piVar15;
        if (local_f8 != (int *)0x0) {
          piVar15 = local_f8 + -4;
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37580;
          FUN_1401bebb0(piVar15);
          piVar29 = local_d8;
        }
      }
    }
    local_f8 = piVar30;
    local_100 = (wchar_t *)0x0;
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b375a1;
    piVar15 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3e);
    piVar15[1] = 0x16;
    *piVar15 = -1;
    local_100 = (wchar_t *)(piVar15 + 4);
    piVar15[2] = 0;
    *local_100 = L'\0';
    uVar16 = u_loginTroubleAskSupport_1433d5d98._8_8_;
    *(undefined8 *)local_100 = u_loginTroubleAskSupport_1433d5d98._0_8_;
    *(undefined8 *)(piVar15 + 6) = uVar16;
    uVar4 = u_loginTroubleAskSupport_1433d5d98._28_4_;
    uVar3 = u_loginTroubleAskSupport_1433d5d98._24_4_;
    uVar10 = u_loginTroubleAskSupport_1433d5d98._20_4_;
    piVar15[8] = u_loginTroubleAskSupport_1433d5d98._16_4_;
    piVar15[9] = uVar10;
    piVar15[10] = uVar3;
    piVar15[0xb] = uVar4;
    *(undefined8 *)(piVar15 + 0xc) = u_loginTroubleAskSupport_1433d5d98._32_8_;
    piVar15[0xe] = u_loginTroubleAskSupport_1433d5d98._40_4_;
    if (*piVar15 != -1) {
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37605;
      FUN_142e52dd0(0x8b);
    }
    iVar9 = piVar15[1];
    if (iVar9 < 0x16) {
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3761b;
      FUN_142e54290(0x90,iVar9,0x16);
    }
    *piVar15 = 1;
    local_100[0x16] = L'\0';
    if (piVar15[1] + 1 < 0x17) {
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37646;
      FUN_142e54290(0x9c,0x16);
    }
    piVar15[2] = 0x2c;
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3765d;
    FUN_141b4ac80(&local_100,&local_f8,0);
    piVar15 = piVar29;
    goto LAB_141b381e7;
  }
  if (bVar6 == 0x80) {
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37679;
    uVar16 = FUN_1408a9e40(&local_108,0x162b);
    *(undefined4 *)((longlong)local_130 + lVar2 + 0x20) = 0;
    *(undefined4 *)((longlong)local_130 + lVar2 + 0x18) = 0;
    *(undefined4 *)((longlong)local_130 + lVar2 + 0x10) = 0;
    *(undefined4 *)((longlong)local_130 + lVar2 + 8) = 0;
    *(undefined4 *)((longlong)local_130 + lVar2) = 0;
    *(undefined4 *)((longlong)local_130 + lVar2 + -8) = 0;
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b376a2;
    FUN_142a26280(uVar16,0,0,1);
    goto LAB_141b381e7;
  }
  local_d0 = (int *)0x0;
  piVar29 = piVar15;
  piVar30 = local_d0;
  if ((piVar15 != (int *)0x0) && (piVar14 = piVar15 + -4, piVar14 != (int *)0x0)) {
    if (*piVar14 == -1) {
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b376d8;
      FUN_142e52d50(0xcb,0xffffff01);
      uVar23 = 0xffffffffffffffff;
      do {
        uVar23 = uVar23 + 1;
      } while (*(short *)((longlong)piVar15 + uVar23 * 2) != 0);
      iVar13 = (int)uVar23;
      iVar9 = 0;
      if (0 < iVar13) {
        iVar9 = iVar13;
      }
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3770b;
      piVar14 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar9 * 2 + 0x12));
      piVar14[1] = iVar9;
      *piVar14 = -1;
      piVar30 = piVar14 + 4;
      piVar14[2] = 0;
      *(undefined2 *)piVar30 = 0;
      local_f8 = piVar30;
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37739;
      FUN_142ef7ba0(piVar30,piVar15,(longlong)iVar13 * 2);
      if (*piVar14 != -1) {
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3774b;
        FUN_142e52dd0(0x8b);
      }
      if ((iVar13 == -1) || (iVar9 = piVar14[1], iVar13 <= iVar9)) {
        *piVar14 = 1;
        if (iVar13 != -1) goto LAB_141b3776b;
        if (piVar30 == (int *)0x0) {
          uVar23 = 0;
        }
        else {
          uVar23 = 0xffffffffffffffff;
          do {
            uVar23 = uVar23 + 1;
          } while (*(short *)((longlong)piVar30 + uVar23 * 2) != 0);
        }
      }
      else {
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37764;
        FUN_142e54290(0x90,iVar9,uVar23 & 0xffffffff);
        *piVar14 = 1;
LAB_141b3776b:
        *(undefined2 *)((longlong)iVar13 * 2 + (longlong)piVar30) = 0;
      }
      iVar9 = (int)uVar23;
      if ((iVar9 < 0) || (piVar14[1] + 1 <= iVar9)) {
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3778e;
        FUN_142e54290(0x9c,uVar23 & 0xffffffff);
      }
      piVar14[2] = iVar9 * 2;
      if (local_d0 != (int *)0x0) {
        piVar15 = local_d0 + -4;
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b377a6;
        FUN_1401bebb0(piVar15);
      }
      local_f8 = (int *)0x0;
      param_1 = local_108;
    }
    else {
      if (*piVar14 < 1) {
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37803;
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar14 = *piVar14 + 1;
      UNLOCK();
      piVar29 = local_d8;
      piVar30 = piVar15;
      if (local_d0 != (int *)0x0) {
        piVar15 = local_d0 + -4;
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37818;
        FUN_1401bebb0(piVar15);
        piVar29 = local_d8;
      }
    }
  }
  local_d0 = piVar30;
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37836;
  cVar8 = FUN_141b267c0(param_1,bVar6,0,&local_d0);
  piVar15 = piVar29;
  if (cVar8 == '\0') goto LAB_141b381e7;
  if (bVar6 == 0xc) {
    local_c8 = (int *)0x0;
    piVar30 = local_c8;
    if ((piVar29 != (int *)0x0) && (piVar14 = piVar29 + -4, piVar14 != (int *)0x0)) {
      if (*piVar14 == -1) {
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37879;
        FUN_142e52d50(0xcb,0xffffff01);
        uVar23 = 0xffffffffffffffff;
        do {
          uVar23 = uVar23 + 1;
        } while (*(short *)((longlong)piVar29 + uVar23 * 2) != 0);
        iVar13 = (int)uVar23;
        iVar9 = 0;
        if (0 < iVar13) {
          iVar9 = iVar13;
        }
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b378ab;
        piVar14 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar9 * 2 + 0x12));
        piVar14[1] = iVar9;
        *piVar14 = -1;
        piVar30 = piVar14 + 4;
        piVar14[2] = 0;
        *(undefined2 *)piVar30 = 0;
        local_70 = piVar30;
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b378dc;
        FUN_142ef7ba0(piVar30,piVar29,(longlong)iVar13 * 2);
        if (*piVar14 != -1) {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b378ee;
          FUN_142e52dd0(0x8b);
        }
        if ((iVar13 == -1) || (iVar9 = piVar14[1], iVar13 <= iVar9)) {
          *piVar14 = 1;
          if (iVar13 != -1) goto LAB_141b3790e;
          if (piVar30 == (int *)0x0) {
            uVar23 = 0;
          }
          else {
            uVar23 = 0xffffffffffffffff;
            do {
              uVar23 = uVar23 + 1;
            } while (*(short *)((longlong)piVar30 + uVar23 * 2) != 0);
          }
        }
        else {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37907;
          FUN_142e54290(0x90,iVar9,uVar23 & 0xffffffff);
          *piVar14 = 1;
LAB_141b3790e:
          *(undefined2 *)((longlong)iVar13 * 2 + (longlong)piVar30) = 0;
        }
        iVar9 = (int)uVar23;
        if ((iVar9 < 0) || (piVar14[1] + 1 <= iVar9)) {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3792f;
          FUN_142e54290(0x9c,uVar23 & 0xffffffff);
        }
        piVar14[2] = iVar9 * 2;
        if (local_c8 != (int *)0x0) {
          piVar29 = local_c8 + -4;
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37947;
          FUN_1401bebb0(piVar29);
        }
        local_70 = (int *)0x0;
        param_1 = local_108;
      }
      else {
        if (*piVar14 < 1) {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b379a6;
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar14 = *piVar14 + 1;
        UNLOCK();
        piVar15 = local_d8;
        piVar30 = piVar29;
        if (local_c8 != (int *)0x0) {
          piVar15 = local_c8 + -4;
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b379bb;
          FUN_1401bebb0(piVar15);
          piVar15 = local_d8;
        }
      }
    }
    local_c8 = piVar30;
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b379d6;
    cVar8 = FUN_141b27910(param_1,uVar7,&local_c8);
    if (cVar8 == '\0') goto LAB_141b381e7;
  }
  else if ((bVar6 == 0x22) || (bVar6 == 0x8e)) goto LAB_141b381e7;
  local_80 = DAT_143aa84a0;
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37a0c;
  local_ac = FUN_1406e8c20(param_2);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37a17;
  local_dc = FUN_1406e8b80(param_2);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37a23;
  local_a8 = FUN_1406e8c20(param_2);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37a38;
  FUN_1406e9170(param_2,&local_94,4);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37a4a;
  FUN_1406e9170(param_2,&local_98,4);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37a5c;
  FUN_1406e9170(param_2,&local_b0,4);
  local_90 = local_b0;
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37a6a;
  local_f0 = FUN_1406e8ae0(param_2);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37a75;
  local_8c = FUN_1406e8c20(param_2);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37a80;
  local_ef = FUN_1406e8ae0(param_2);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37a8b;
  FUN_1406e8ae0(param_2);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37a93;
  FUN_1406e8c20(param_2);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37a9b;
  FUN_1406e8ae0(param_2);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37aa3;
  FUN_1406e8ae0(param_2);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37ab8;
  FUN_1406e9170(param_2,local_68,8);
  local_e8[0] = 0;
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37ac3;
  uVar10 = FUN_1406e9b20(param_2);
  local_78 = (undefined1 *)CONCAT44(local_78._4_4_,uVar10);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37ad1;
  uVar11 = FUN_1406e8c20(param_2);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37adb;
  uVar12 = FUN_1406e8c20(param_2);
  uVar23 = (ulonglong)uVar12;
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37aee;
  puVar17 = (undefined1 *)FUN_14019b780(&DAT_143ad68a0,uVar12);
  if (puVar17 != (undefined1 *)0x0) {
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37b03;
    FUN_142ef8250(puVar17,0,uVar12);
  }
  uVar34 = *(uint *)(param_2 + 0x24);
  iVar9 = *(int *)(param_2 + 0x18);
  lVar18 = *(longlong *)(param_2 + 0x10);
  local_60 = puVar17;
  if (lVar18 == 0) {
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37b32;
    FUN_142e52d50(0xd0,1);
    lVar18 = *(longlong *)(param_2 + 0x10);
    if (lVar18 != 0) goto LAB_141b37b3c;
LAB_141b37b4a:
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37b56;
    FUN_142e54290(0xbc,0,0);
    lVar18 = *(longlong *)(param_2 + 0x10);
  }
  else {
LAB_141b37b3c:
    if (*(int *)(lVar18 + -8) == 0) goto LAB_141b37b4a;
  }
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37b6d;
  FUN_1406e8460(puVar17,uVar12,lVar18 + (ulonglong)uVar34,iVar9 - uVar34);
  uVar33 = 0;
  uVar31 = 0;
  if (3 < uVar23) {
    iVar9 = 0;
    do {
      *(uint *)(puVar17 + uVar33) =
           ((uVar11 ^ *(uint *)(puVar17 + uVar33)) + 0x369f144d + (uVar11 >> 7) ^ 0xaaaabbbb) -
           iVar9;
      uVar33 = (int)uVar31 + 4;
      uVar31 = (ulonglong)uVar33;
      iVar9 = iVar9 + uVar11 * 4;
    } while (uVar31 + 4 <= uVar23);
  }
  if ((uint)uVar31 < uVar12) {
    pbVar32 = puVar17 + uVar31;
    do {
      *pbVar32 = (((byte)(uVar11 >> 1) ^ *pbVar32) + 0x37 + (char)(uVar11 >> 7) ^ 0xab) -
                 (char)uVar31 * (char)uVar11;
      uVar33 = (int)uVar31 + 1;
      uVar31 = (ulonglong)uVar33;
      pbVar32 = pbVar32 + 1;
    } while (uVar33 < uVar12);
  }
  lVar18 = *(longlong *)(param_2 + 0x10);
  if (lVar18 == 0) {
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37c22;
    FUN_142e52d50(0xd0,1);
    lVar18 = *(longlong *)(param_2 + 0x10);
    if (lVar18 != 0) goto LAB_141b37c2c;
LAB_141b37c3a:
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37c46;
    FUN_142e54290(0xbc,0,0);
    lVar18 = *(longlong *)(param_2 + 0x10);
  }
  else {
LAB_141b37c2c:
    if (*(int *)(lVar18 + -8) == 0) goto LAB_141b37c3a;
  }
  puVar19 = (undefined1 *)(lVar18 + (ulonglong)uVar34);
  puVar27 = puVar17;
  if (0 < (int)uVar12) {
    do {
      *puVar19 = *puVar27;
      puVar19 = puVar19 + 1;
      uVar23 = uVar23 - 1;
      puVar27 = puVar27 + 1;
    } while (uVar23 != 0);
  }
  if (puVar17 != (undefined1 *)0x0) {
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37c81;
    FUN_14019b4e0(puVar17);
  }
  lVar18 = local_a0;
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37c97;
  FUN_1406e9170(lVar18,local_e8,4);
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37c9f;
  iVar9 = FUN_1406e9b20(lVar18);
  uVar11 = (uint)local_78;
  uVar34 = iVar9 - (uint)local_78;
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37cbb;
  uVar12 = FUN_140738db0(0x80000000,0x7fffffff);
  uVar23 = (ulonglong)uVar34;
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37ccf;
  puVar17 = (undefined1 *)FUN_14019b780(&DAT_143ad68a0,uVar34);
  if (puVar17 != (undefined1 *)0x0) {
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37ce4;
    FUN_142ef8250(puVar17,0,uVar34);
  }
  if (uVar11 == 0xffffffff) {
    uVar11 = *(uint *)(lVar18 + 0x24);
  }
  iVar9 = *(int *)(lVar18 + 0x18);
  lVar18 = *(longlong *)(param_2 + 0x10);
  local_78 = puVar17;
  if (lVar18 == 0) {
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37d17;
    FUN_142e52d50(0xd0,1);
    lVar18 = *(longlong *)(param_2 + 0x10);
    if (lVar18 != 0) goto LAB_141b37d21;
LAB_141b37d2f:
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37d3b;
    FUN_142e54290(0xbc,0,0);
    lVar18 = *(longlong *)(param_2 + 0x10);
  }
  else {
LAB_141b37d21:
    if (*(int *)(lVar18 + -8) == 0) goto LAB_141b37d2f;
  }
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37d52;
  FUN_1406e8460(puVar17,uVar34,lVar18 + (ulonglong)uVar11,iVar9 - uVar11);
  uVar28 = 0;
  uVar31 = uVar28;
  if (3 < uVar23) {
    do {
      *(uint *)(puVar17 + uVar31) =
           ((uVar12 ^ *(uint *)(puVar17 + uVar31)) + 0x369f144d + (uVar12 >> 7) ^ 0xaaaabbbb) -
           (int)uVar28;
      uVar31 = (ulonglong)((int)uVar31 + 4);
      uVar28 = (ulonglong)((int)uVar28 + uVar12 * 4);
    } while (uVar31 + 4 <= uVar23);
  }
  if ((uint)uVar31 < uVar34) {
    pbVar32 = puVar17 + uVar31;
    do {
      *pbVar32 = (((byte)(uVar12 >> 1) ^ *pbVar32) + 0x37 + (char)(uVar12 >> 7) ^ 0xab) -
                 (char)uVar12 * (char)uVar31;
      uVar33 = (int)uVar31 + 1;
      uVar31 = (ulonglong)uVar33;
      pbVar32 = pbVar32 + 1;
    } while (uVar33 < uVar34);
  }
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37e0c;
  lVar18 = FUN_1415e2f80(param_2 + 0x10);
  puVar19 = (undefined1 *)(lVar18 + (ulonglong)uVar11);
  puVar27 = puVar17;
  if (0 < (int)uVar34) {
    do {
      *puVar19 = *puVar27;
      puVar19 = puVar19 + 1;
      uVar23 = uVar23 - 1;
      puVar27 = puVar27 + 1;
    } while (uVar23 != 0);
  }
  if (puVar17 != (undefined1 *)0x0) {
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37e41;
    FUN_14019b4e0(puVar17);
  }
  DAT_143ac80b4 = DAT_143ac80b4 * 0x343fd + 0x269ec3;
  uVar11 = DAT_143ac80b4 >> 0x10 & 0x7fff;
  DAT_143ac80b8 = (char)uVar11 + (char)((ulonglong)uVar11 / 0xff) + 1;
  DAT_143ac80b0 = (uint)DAT_143ac80b8 * 0x1010101 ^ local_e8[0];
  local_e8[0] = DAT_143ac80b0;
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37ea1;
  (*DAT_143262df8)(0);
  uVar5 = local_dc;
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37eab;
  local_56 = (*DAT_143262e48)(uVar5);
  iVar9 = local_a8;
  local_50 = 0;
  local_58 = 2;
  local_54 = local_ac;
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37ed8;
  piVar30 = (int *)FUN_14108cae0(iVar9);
  piVar15 = local_d8;
  if (*piVar30 == iVar9) {
    local_a0 = CONCAT44(local_a0._4_4_,*(int *)((longlong)piVar30 + 0x37));
    local_e0 = CONCAT11(*(byte *)((longlong)piVar30 + 0x36) ^ *(byte *)(piVar30 + 0xd),
                        *(byte *)((longlong)piVar30 + 0x35) ^ *(byte *)((longlong)piVar30 + 0x33));
    uVar11 = *(byte *)((longlong)piVar30 + 0x33) ^ 0xbaadf00d;
    uVar11 = (uint)*(byte *)((longlong)piVar30 + 0x35) + (uVar11 >> 5 | uVar11 << 0x1b) ^
             (uint)*(byte *)(piVar30 + 0xd);
    iVar13 = (uVar11 >> 5 | uVar11 << 0x1b) + (uint)*(byte *)((longlong)piVar30 + 0x36);
    local_f8 = (int *)CONCAT44(local_f8._4_4_,iVar13);
    if (iVar13 != *(int *)((longlong)piVar30 + 0x37)) {
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37f37;
      local_c0 = (int *)FUN_1418039d0(5);
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37f50;
      puVar20 = (undefined8 *)FUN_1401a0ed0(&local_b8,&local_c0,&local_f8,&local_a0);
      uVar16 = *puVar20;
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37f69;
      FUN_141804970(&DAT_143271f04,0x53,5,uVar16);
      if (local_b8 != (int **)0x0) {
        ppiVar24 = local_b8 + -2;
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37f7c;
        FUN_14019f2c0(ppiVar24);
      }
    }
    iVar13 = (int)local_e0;
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37f8c;
    FUN_1408414d0(local_94,local_98);
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37f95;
    FUN_140842250(&local_90);
    uVar16 = local_80;
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37fa6;
    FUN_142cb9560(uVar16,iVar9);
    bVar6 = local_f0;
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37fb7;
    FUN_142cb9590(uVar16,bVar6);
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37fc7;
    FUN_142cb95a0(uVar16,bVar6 >> 1 & 1);
    uVar10 = local_8c;
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37fd2;
    FUN_142cb95b0(uVar16,uVar10);
    uVar7 = local_ef;
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37fde;
    FUN_142cb95c0(uVar16,uVar7);
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37fed;
    FUN_142caa360(uVar16,&local_58);
    uVar7 = *(undefined1 *)((longlong)piVar30 + 0x19);
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b37ff9;
    FUN_142cb83f0(uVar16,uVar7);
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b38003;
    FUN_142cb8440(uVar16,iVar13);
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3800b;
    FUN_142caebe0(uVar16);
    lVar18 = local_108;
    if (*(longlong *)(local_108 + 0x138) != 0) {
      lVar25 = local_108 + 0x130;
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b38025;
      uVar21 = FUN_141b43aa0(lVar25);
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3802d;
      FUN_141b49b70(uVar21);
    }
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b38040;
    FUN_1429f14c0(PTR_u_GameIn_143a47c08,100);
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3804c;
    iVar9 = FUN_142c4f6d0(DAT_143ac1898);
    if (iVar9 != 0) {
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b38059;
      cVar8 = FUN_14057e4c0();
      if (cVar8 != '\0') {
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b38066;
        uVar16 = FUN_1404c6160();
        local_108 = 0;
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3807d;
        pcVar22 = (char *)FUN_14019bd40(&local_108,0x12);
        lVar25 = local_108;
        uVar4 = s_GC_SelectCharacter_1433fe7f8._12_4_;
        uVar3 = s_GC_SelectCharacter_1433fe7f8._8_4_;
        uVar10 = s_GC_SelectCharacter_1433fe7f8._4_4_;
        *(undefined4 *)pcVar22 = s_GC_SelectCharacter_1433fe7f8._0_4_;
        *(undefined4 *)(pcVar22 + 4) = uVar10;
        *(undefined4 *)(pcVar22 + 8) = uVar3;
        *(undefined4 *)(pcVar22 + 0xc) = uVar4;
        *(undefined2 *)(pcVar22 + 0x10) = s_GC_SelectCharacter_1433fe7f8._16_2_;
        if (*(int *)(local_108 + -0x10) != -1) {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b380a8;
          FUN_142e52dd0(0x8b);
        }
        iVar9 = *(int *)(lVar25 + -0xc);
        if (iVar9 < 0x12) {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b380be;
          FUN_142e54290(0x90,iVar9,0x12);
        }
        *(undefined4 *)(lVar25 + -0x10) = 1;
        *(undefined1 *)(local_108 + 0x12) = 0;
        if (*(int *)(lVar25 + -0xc) + 1 < 0x13) {
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b380e9;
          FUN_142e54290(0x9c,0x12);
        }
        *(undefined4 *)(lVar25 + -8) = 0x12;
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b38101;
        FUN_1404c7800(uVar16,0xe,&local_108);
        uVar16 = local_80;
        if (local_108 != 0) {
          lVar25 = local_108 + -0x10;
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b38114;
          FUN_14019f2c0(lVar25);
          uVar16 = local_80;
        }
      }
    }
    piVar15 = local_d8;
    if ((int)local_100 == 0) {
      local_108 = 0;
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3813d;
      FUN_1401d66b0(&local_108,piVar30 + 3,0xffffffff);
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b38147;
      FUN_140d22010(&local_108);
      if (local_108 != 0) {
        lVar25 = local_108 + -0x10;
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3815a;
        FUN_14019f2c0(lVar25);
      }
      *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b38162;
      FUN_140d22050(1);
      *(undefined1 *)(lVar18 + 0x205) = 1;
      lVar18 = DAT_143ac8210;
      piVar15 = local_d8;
      if (DAT_143ac8210 != 0) {
        local_108 = 0;
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3818d;
        FUN_1401d66b0(&local_108,piVar30 + 3,0xffffffff);
        local_100 = (wchar_t *)0x0;
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3819a;
        uVar10 = FUN_142cb8460(uVar16);
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b381ad;
        uVar16 = FUN_14019ba10(&local_100,&DAT_1433fe164,uVar10);
        *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b381bc;
        FUN_140c7a870(lVar18,uVar16,&local_108);
        if (local_100 != (wchar_t *)0x0) {
          pwVar26 = local_100 + -8;
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b381cf;
          FUN_14019f2c0(pwVar26);
        }
        piVar15 = local_d8;
        if (local_108 != 0) {
          lVar18 = local_108 + -0x10;
          *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b381e2;
          FUN_14019f2c0(lVar18);
          piVar15 = local_d8;
        }
      }
    }
  }
LAB_141b381e7:
  if (piVar15 != (int *)0x0) {
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b381f5;
    FUN_1401bebb0(piVar15 + -4);
  }
  if (local_88 != 0) {
    *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3820b;
    FUN_14019f2c0(local_88 + -0x10);
  }
  *(undefined8 *)((longlong)auStack_160 + lVar2) = 0x141b3821b;
  return;
}



//===========================================================
// FUN_141b2f5f0 @ 141b2f5f0   (1158 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b2f5f0(longlong param_1,undefined8 param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  char cVar4;
  int iVar5;
  int *piVar6;
  undefined8 uVar7;
  wchar_t **ppwVar8;
  longlong *plVar9;
  wchar_t *pwVar10;
  wchar_t **ppwVar11;
  undefined1 auStack_468 [32];
  wchar_t *local_448;
  wchar_t *local_440;
  wchar_t **local_438;
  undefined1 local_428 [1024];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_468;
  *(undefined4 *)(param_1 + 0xd4) = 0;
  cVar4 = FUN_1406e8ae0(param_2);
  if (cVar4 == '\0') {
    local_448 = (wchar_t *)0x0;
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x2e);
    piVar6[1] = 0xe;
    *piVar6 = -1;
    local_448 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *local_448 = L'\0';
    uVar3 = u_umapleIDCreated_1433fe166._14_4_;
    uVar2 = u_umapleIDCreated_1433fe166._10_4_;
    uVar1 = u_umapleIDCreated_1433fe166._6_4_;
    *(undefined4 *)local_448 = u_umapleIDCreated_1433fe166._2_4_;
    piVar6[5] = uVar1;
    piVar6[6] = uVar2;
    piVar6[7] = uVar3;
    *(undefined8 *)(piVar6 + 8) = u_umapleIDCreated_1433fe166._18_8_;
    piVar6[10] = u_umapleIDCreated_1433fe166._26_4_;
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0xe) {
      FUN_142e54290(0x90,piVar6[1],0xe);
    }
    *piVar6 = 1;
    local_448[0xe] = L'\0';
    if (piVar6[1] + 1 < 0xf) {
      FUN_142e54290(0x9c,0xe);
    }
    piVar6[2] = 0x1c;
    FUN_141b4a840(&local_448,0);
    plVar9 = *(longlong **)(param_1 + 0x148);
    if (plVar9 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
      plVar9 = *(longlong **)(param_1 + 0x148);
    }
    (**(code **)(*plVar9 + 0x138))(plVar9,2000);
    iVar5 = FUN_142c4f6d0(DAT_143ac1898);
    if (iVar5 == 0) {
      return;
    }
    cVar4 = FUN_14057e4c0();
    if (cVar4 == '\0') {
      return;
    }
    uVar7 = FUN_1404c6160();
    local_448 = (wchar_t *)0x0;
    piVar6 = (int *)FUN_14019b600(&DAT_143ad6a30,0x26);
    piVar6[1] = 0x15;
    *piVar6 = -1;
    local_448 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *(undefined1 *)local_448 = 0;
    uVar3 = s_GC_CreateMapleAccount_1433fe188._12_4_;
    uVar2 = s_GC_CreateMapleAccount_1433fe188._8_4_;
    uVar1 = s_GC_CreateMapleAccount_1433fe188._4_4_;
    *(undefined4 *)local_448 = s_GC_CreateMapleAccount_1433fe188._0_4_;
    piVar6[5] = uVar1;
    piVar6[6] = uVar2;
    piVar6[7] = uVar3;
    piVar6[8] = s_GC_CreateMapleAccount_1433fe188._16_4_;
    *(char *)(piVar6 + 9) = s_GC_CreateMapleAccount_1433fe188[0x14];
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0x15) {
      FUN_142e54290(0x90,piVar6[1],0x15);
    }
    *piVar6 = 1;
    *(undefined1 *)((longlong)local_448 + 0x15) = 0;
    if (piVar6[1] + 1 < 0x16) {
      FUN_142e54290(0x9c,0x15);
    }
    piVar6[2] = 0x15;
    FUN_1404c7800(uVar7,0xf,&local_448);
    if (local_448 == (wchar_t *)0x0) {
      return;
    }
    FUN_14019f2c0(local_448 + -8);
    return;
  }
  switch(cVar4) {
  case '*':
    pwVar10 = L"alreadyExistID";
    break;
  default:
    local_438 = &local_440;
    local_440 = (wchar_t *)0x0;
    local_448 = (wchar_t *)0x0;
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3a);
    piVar6[1] = 0x14;
    *piVar6 = -1;
    local_448 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *local_448 = L'\0';
    uVar7 = u_cannotProcessRequest_1433fdbc8._8_8_;
    *(undefined8 *)local_448 = u_cannotProcessRequest_1433fdbc8._0_8_;
    *(undefined8 *)(piVar6 + 6) = uVar7;
    uVar3 = u_cannotProcessRequest_1433fdbc8._28_4_;
    uVar2 = u_cannotProcessRequest_1433fdbc8._24_4_;
    uVar1 = u_cannotProcessRequest_1433fdbc8._20_4_;
    piVar6[8] = u_cannotProcessRequest_1433fdbc8._16_4_;
    piVar6[9] = uVar1;
    piVar6[10] = uVar2;
    piVar6[0xb] = uVar3;
    *(undefined8 *)(piVar6 + 0xc) = u_cannotProcessRequest_1433fdbc8._32_8_;
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0x14) {
      FUN_142e54290(0x90,piVar6[1],0x14);
    }
    *piVar6 = 1;
    local_448[0x14] = L'\0';
    if (piVar6[1] + 1 < 0x15) {
      FUN_142e54290(0x9c,0x14);
    }
    piVar6[2] = 0x28;
    ppwVar11 = &local_440;
    ppwVar8 = &local_448;
    goto LAB_141b2fa17;
  case '-':
    uVar7 = FUN_1403edf80(&local_440,L"cannotCreateAccountNotAuthenticated",0xffffffff);
    iVar5 = FUN_141b49b80(2,uVar7,0);
    if (iVar5 != 0) {
      FUN_141d60b50(local_428);
      FUN_1429e4fa0(local_428,0,0);
    }
    goto LAB_141b2fa1f;
  case '.':
    pwVar10 = L"cannotCreateAccountMore";
    break;
  case '/':
    pwVar10 = L"blockCreateAccountOverCount";
    break;
  case '0':
    pwVar10 = L"blockCreateAccountToday";
    break;
  case '6':
    pwVar10 = L"cannotByCoopPermision";
  }
  local_438 = &local_448;
  local_448 = (wchar_t *)0x0;
  ppwVar8 = (wchar_t **)FUN_1403edf80(&local_440,pwVar10,0xffffffff);
  ppwVar11 = &local_448;
LAB_141b2fa17:
  FUN_141b4ac80(ppwVar8,ppwVar11,0);
LAB_141b2fa1f:
  plVar9 = *(longlong **)(param_1 + 0x148);
  if (plVar9 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
    plVar9 = *(longlong **)(param_1 + 0x148);
  }
  (**(code **)(*plVar9 + 0x138))(plVar9,0x7d1);
  FUN_141b3a560(param_1);
  return;
}



//===========================================================
// FUN_141b33f30 @ 141b33f30   (2453 bytes)
//===========================================================

void FUN_141b33f30(longlong param_1,undefined8 param_2)

{
  wchar_t *pwVar1;
  wchar_t **ppwVar2;
  longlong lVar3;
  wchar_t *pwVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined1 uVar8;
  undefined1 uVar9;
  char cVar10;
  int iVar11;
  int *piVar12;
  ulonglong uVar13;
  undefined8 uVar14;
  undefined8 *puVar15;
  int *piVar16;
  longlong *plVar17;
  ulonglong uVar18;
  int iVar19;
  ulonglong uVar20;
  wchar_t *local_res8;
  wchar_t *local_res18;
  wchar_t **local_res20;
  wchar_t **local_68;
  longlong local_60 [2];
  wchar_t *local_50;
  longlong *local_48;
  
  uVar18 = 0;
  *(undefined4 *)(param_1 + 0xd4) = 0;
  FUN_1406e9050(param_2,local_60);
  uVar8 = FUN_1406e8ae0(param_2);
  local_60[1] = 0;
  plVar17 = *(longlong **)(param_1 + 0x270);
  if (plVar17 != (longlong *)0x0) {
    (**(code **)(*plVar17 + 0x138))(plVar17,3);
  }
  switch(uVar8) {
  case 0:
    local_res8 = (wchar_t *)0x0;
    piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,0x2c);
    piVar16[1] = 0xd;
    uVar13 = 0xffffffffffffffff;
    *piVar16 = -1;
    local_res8 = (wchar_t *)(piVar16 + 4);
    piVar16[2] = 0;
    *local_res8 = L'\0';
    uVar7 = u_availableName_1433fe320._12_4_;
    uVar6 = u_availableName_1433fe320._8_4_;
    uVar5 = u_availableName_1433fe320._4_4_;
    *(undefined4 *)local_res8 = u_availableName_1433fe320._0_4_;
    piVar16[5] = uVar5;
    piVar16[6] = uVar6;
    piVar16[7] = uVar7;
    *(undefined8 *)(piVar16 + 8) = u_availableName_1433fe320._16_8_;
    *(wchar_t *)(piVar16 + 10) = u_availableName_1433fe320[0xc];
    if (*piVar16 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar16[1] < 0xd) {
      FUN_142e54290(0x90,piVar16[1],0xd);
    }
    *piVar16 = 1;
    local_res8[0xd] = L'\0';
    if (piVar16[1] + 1 < 0xe) {
      FUN_142e54290(0x9c,0xd);
    }
    piVar16[2] = 0x1a;
    iVar11 = FUN_141b49b80(2,&local_res8,0);
    if (iVar11 != 0) {
      FUN_14019a260(param_1 + 0x230,local_60);
      goto LAB_141b3489a;
    }
    piVar16 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar16[1] = 0;
    *piVar16 = -1;
    pwVar1 = (wchar_t *)(piVar16 + 4);
    piVar16[2] = 0;
    *(undefined1 *)pwVar1 = 0;
    local_res18 = pwVar1;
    if (*piVar16 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar16[1] < 0) {
      FUN_142e54290(0x90,piVar16[1],0);
    }
    *piVar16 = 1;
    *(undefined1 *)pwVar1 = 0;
    if (piVar16[1] + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    piVar16[2] = 0;
    ppwVar2 = (wchar_t **)(param_1 + 0x230);
    if (ppwVar2 != &local_res18) {
      pwVar4 = *ppwVar2;
      iVar11 = 0;
      if (pwVar4 != (wchar_t *)0x0) {
        iVar11 = *(int *)(pwVar4 + -4);
      }
      if ((((iVar11 != piVar16[2]) || (iVar11 == 0)) || (pwVar4 == (wchar_t *)0x0)) ||
         (iVar11 = memcmp(pwVar4,pwVar1,(longlong)iVar11), iVar11 != 0)) {
        if (piVar16 == (int *)0x0) {
          if (pwVar4 != (wchar_t *)0x0) {
            FUN_14019f2c0(pwVar4 + -8);
            *ppwVar2 = (wchar_t *)0x0;
          }
        }
        else if (*piVar16 == -1) {
          FUN_142e52d50(0xcb,0xffffff01);
          uVar20 = 0xffffffffffffffff;
          do {
            uVar20 = uVar20 + 1;
          } while (*(char *)((longlong)pwVar1 + uVar20) != '\0');
          iVar19 = (int)uVar20;
          iVar11 = 0;
          if (0 < iVar19) {
            iVar11 = iVar19;
          }
          piVar12 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
          piVar12[1] = iVar11;
          *piVar12 = -1;
          pwVar4 = (wchar_t *)(piVar12 + 4);
          piVar12[2] = 0;
          *(undefined1 *)pwVar4 = 0;
          local_res8 = (wchar_t *)(longlong)iVar19;
          local_50 = pwVar4;
          FUN_142ef7ba0(pwVar4,pwVar1,local_res8);
          if (*piVar12 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar19 == -1) || (iVar19 <= piVar12[1])) {
            *piVar12 = 1;
            if (iVar19 != -1) goto LAB_141b34206;
            if (pwVar4 != (wchar_t *)0x0) {
              do {
                uVar13 = uVar13 + 1;
              } while (*(char *)((longlong)pwVar4 + uVar13) != '\0');
              uVar18 = uVar13 & 0xffffffff;
            }
          }
          else {
            FUN_142e54290(0x90,piVar12[1],uVar20 & 0xffffffff);
            *piVar12 = 1;
LAB_141b34206:
            *(undefined1 *)((longlong)local_res8 + (longlong)pwVar4) = 0;
            uVar18 = uVar20;
          }
          iVar11 = (int)uVar18;
          if ((iVar11 < 0) || (piVar12[1] + 1 <= iVar11)) {
            FUN_142e54290(0x9c,uVar18 & 0xffffffff);
          }
          piVar12[2] = iVar11;
          if (*ppwVar2 != (wchar_t *)0x0) {
            FUN_14019f2c0(*ppwVar2 + -8);
          }
          *ppwVar2 = pwVar4;
        }
        else {
          if (*piVar16 < 1) {
            FUN_142e52dd0(0xd2);
          }
          LOCK();
          *piVar16 = *piVar16 + 1;
          UNLOCK();
          if (*ppwVar2 != (wchar_t *)0x0) {
            FUN_14019f2c0(*ppwVar2 + -8);
          }
          *ppwVar2 = (wchar_t *)(piVar16 + 4);
        }
      }
    }
    FUN_14019f2c0(piVar16);
    goto LAB_141b3489a;
  default:
    local_res20 = &local_res18;
    local_res18 = (wchar_t *)0x0;
    local_res8 = (wchar_t *)0x0;
    piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3a);
    piVar16[1] = 0x14;
    *piVar16 = -1;
    local_res8 = (wchar_t *)(piVar16 + 4);
    piVar16[2] = 0;
    *local_res8 = L'\0';
    uVar14 = u_cannotProcessRequest_1433fdbc8._8_8_;
    *(undefined8 *)local_res8 = u_cannotProcessRequest_1433fdbc8._0_8_;
    *(undefined8 *)(piVar16 + 6) = uVar14;
    uVar7 = u_cannotProcessRequest_1433fdbc8._28_4_;
    uVar6 = u_cannotProcessRequest_1433fdbc8._24_4_;
    uVar5 = u_cannotProcessRequest_1433fdbc8._20_4_;
    piVar16[8] = u_cannotProcessRequest_1433fdbc8._16_4_;
    piVar16[9] = uVar5;
    piVar16[10] = uVar6;
    piVar16[0xb] = uVar7;
    *(undefined8 *)(piVar16 + 0xc) = u_cannotProcessRequest_1433fdbc8._32_8_;
    if (*piVar16 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar16[1] < 0x14) {
      FUN_142e54290(0x90,piVar16[1],0x14);
    }
    *piVar16 = 1;
    local_res8[0x14] = L'\0';
    if (piVar16[1] + 1 < 0x15) {
      FUN_142e54290(0x9c,0x14);
    }
    piVar16[2] = 0x28;
    break;
  case 0x45:
    cVar10 = FUN_1406e8ae0(param_2);
    FUN_1406e8ae0(param_2);
    uVar8 = FUN_1406e8ae0(param_2);
    uVar9 = FUN_1406e8ae0(param_2);
    if (cVar10 != '\0') {
      local_res20 = &local_res8;
      local_res8 = (wchar_t *)0x0;
      uVar14 = FUN_1403edf80(&local_res18,L"antimacroTextMismatch",0xffffffff);
      FUN_141b4ac80(uVar14,&local_res8,param_1 + 0x140);
    }
    FUN_141029040(&local_68,param_2);
    local_res20 = (wchar_t **)FUN_14019b780(&DAT_143ad68a0,0x340);
    uVar13 = uVar18;
    if (local_res20 != (wchar_t **)0x0) {
      uVar13 = FUN_14130a540(local_res20,param_1,0x3c);
    }
    uVar20 = uVar13 + 0x18;
    if (uVar13 == 0) {
      uVar20 = uVar18;
    }
    if (uVar20 == 0) {
      local_48 = (longlong *)0x0;
    }
    else {
      local_48 = (longlong *)(uVar20 - 0x18);
      if (local_48 != (longlong *)0x0) {
        if (0xfffff < *(ulonglong *)(uVar20 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(uVar20 + 8) = *(longlong *)(uVar20 + 8) + 1;
        UNLOCK();
      }
    }
    plVar17 = local_48;
    if (local_48 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    local_res20 = local_68;
    if (local_68 != (wchar_t **)0x0) {
      (**(code **)(*local_68 + 4))();
    }
    FUN_14130cbc0(plVar17,&local_res20,uVar8,uVar9);
    if (plVar17 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    (**(code **)(*plVar17 + 0x130))(plVar17);
    if (0xffffe < plVar17[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar17 = plVar17 + 4;
    lVar3 = *plVar17;
    *plVar17 = *plVar17 + -1;
    UNLOCK();
    if (((int)lVar3 == 1) && (plVar17 = local_48 + 3, plVar17 != (longlong *)0x0)) {
      (**(code **)*plVar17)(plVar17,1);
    }
    if (local_68 != (wchar_t **)0x0) {
      (**(code **)(*local_68 + 8))();
    }
    goto LAB_141b3489a;
  case 0x46:
    local_res20 = &local_res18;
    local_res18 = (wchar_t *)0x0;
    local_res8 = (wchar_t *)0x0;
    piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3a);
    piVar16[1] = 0x14;
    *piVar16 = -1;
    local_res8 = (wchar_t *)(piVar16 + 4);
    piVar16[2] = 0;
    *local_res8 = L'\0';
    uVar14 = u_cannotProcessRequest_1433fdbc8._8_8_;
    *(undefined8 *)local_res8 = u_cannotProcessRequest_1433fdbc8._0_8_;
    *(undefined8 *)(piVar16 + 6) = uVar14;
    uVar7 = u_cannotProcessRequest_1433fdbc8._28_4_;
    uVar6 = u_cannotProcessRequest_1433fdbc8._24_4_;
    uVar5 = u_cannotProcessRequest_1433fdbc8._20_4_;
    piVar16[8] = u_cannotProcessRequest_1433fdbc8._16_4_;
    piVar16[9] = uVar5;
    piVar16[10] = uVar6;
    piVar16[0xb] = uVar7;
    *(undefined8 *)(piVar16 + 0xc) = u_cannotProcessRequest_1433fdbc8._32_8_;
    if (*piVar16 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar16[1] < 0x14) {
      FUN_142e54290(0x90,piVar16[1],0x14);
    }
    *piVar16 = 1;
    local_res8[0x14] = L'\0';
    if (piVar16[1] + 1 < 0x15) {
      FUN_142e54290(0x9c,0x14);
    }
    piVar16[2] = 0x28;
    param_1 = param_1 + 0x140;
    goto LAB_141b3488c;
  case 0x47:
    cVar10 = FUN_1406e8ae0(param_2);
    local_res8 = (wchar_t *)0x0;
    local_res20 = &local_res8;
    if (cVar10 == '\0') {
      uVar14 = FUN_1403edf80(&local_res18,L"antimacroTextFailTooMuch",0xffffffffffffffff);
      FUN_141b4ac80(uVar14,&local_res8,param_1 + 0x140);
      local_res8 = (wchar_t *)0x0;
      puVar15 = (undefined8 *)FUN_1408a9e40(&local_res18,3);
      FUN_14019ba10(&local_res8,*puVar15);
      if (local_res18 != (wchar_t *)0x0) {
        FUN_14019f2c0(local_res18 + -8);
      }
      pwVar1 = local_res8;
      FUN_1429e4fa0(local_res8,0,0);
      *(undefined4 *)(param_1 + 0xf0) = 1;
      if (pwVar1 != (wchar_t *)0x0) {
        FUN_14019f2c0(pwVar1 + -8);
      }
    }
    else {
      uVar14 = FUN_1403edf80(&local_res18,L"antimacroTextMismatch",0xffffffffffffffff);
      FUN_141b4ac80(uVar14,&local_res8,param_1 + 0x140);
      FUN_141b2d290(param_1,0,0);
    }
    goto LAB_141b3489a;
  case 0x79:
    goto LAB_141b343a9;
  case 0x7a:
    local_res20 = &local_res18;
    local_res18 = (wchar_t *)0x0;
    local_res8 = (wchar_t *)0x0;
    piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,0x30);
    piVar16[1] = 0xf;
    *piVar16 = -1;
    local_res8 = (wchar_t *)(piVar16 + 4);
    piVar16[2] = 0;
    *local_res8 = L'\0';
    uVar7 = u_alreadyUsedName_1433fe340._12_4_;
    uVar6 = u_alreadyUsedName_1433fe340._8_4_;
    uVar5 = u_alreadyUsedName_1433fe340._4_4_;
    *(undefined4 *)local_res8 = u_alreadyUsedName_1433fe340._0_4_;
    piVar16[5] = uVar5;
    piVar16[6] = uVar6;
    piVar16[7] = uVar7;
    *(undefined8 *)(piVar16 + 8) = u_alreadyUsedName_1433fe340._16_8_;
    piVar16[10] = u_alreadyUsedName_1433fe340._24_4_;
    *(wchar_t *)(piVar16 + 0xb) = u_alreadyUsedName_1433fe340[0xe];
    if (*piVar16 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar16[1] < 0xf) {
      FUN_142e54290(0x90,piVar16[1],0xf);
    }
    *piVar16 = 1;
    local_res8[0xf] = L'\0';
    if (piVar16[1] + 1 < 0x10) {
      FUN_142e54290(0x9c,0xf);
    }
    piVar16[2] = 0x1e;
    break;
  case 0x7b:
LAB_141b343a9:
    local_res20 = &local_res18;
    local_res18 = (wchar_t *)0x0;
    local_res8 = (wchar_t *)0x0;
    piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,0x34);
    piVar16[1] = 0x11;
    *piVar16 = -1;
    local_res8 = (wchar_t *)(piVar16 + 4);
    piVar16[2] = 0;
    *local_res8 = L'\0';
    uVar7 = u_cannotUseThisName_1433fde50._12_4_;
    uVar6 = u_cannotUseThisName_1433fde50._8_4_;
    uVar5 = u_cannotUseThisName_1433fde50._4_4_;
    *(undefined4 *)local_res8 = u_cannotUseThisName_1433fde50._0_4_;
    piVar16[5] = uVar5;
    piVar16[6] = uVar6;
    piVar16[7] = uVar7;
    uVar7 = u_cannotUseThisName_1433fde50._28_4_;
    uVar6 = u_cannotUseThisName_1433fde50._24_4_;
    uVar5 = u_cannotUseThisName_1433fde50._20_4_;
    piVar16[8] = u_cannotUseThisName_1433fde50._16_4_;
    piVar16[9] = uVar5;
    piVar16[10] = uVar6;
    piVar16[0xb] = uVar7;
    *(wchar_t *)(piVar16 + 0xc) = u_cannotUseThisName_1433fde50[0x10];
    if (*piVar16 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar16[1] < 0x11) {
      FUN_142e54290(0x90,piVar16[1],0x11);
    }
    *piVar16 = 1;
    local_res8[0x11] = L'\0';
    if (piVar16[1] + 1 < 0x12) {
      FUN_142e54290(0x9c,0x11);
    }
    piVar16[2] = 0x22;
  }
  param_1 = 0;
LAB_141b3488c:
  FUN_141b4ac80(&local_res8,&local_res18,param_1);
LAB_141b3489a:
  if (local_60[0] != 0) {
    FUN_14019f2c0(local_60[0] + -0x10);
  }
  return;
}



//===========================================================
// FUN_141b36a10 @ 141b36a10   (1351 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b36a10(longlong param_1,undefined8 param_2)

{
  longlong *plVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  char cVar4;
  int iVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  int *piVar8;
  wchar_t **ppwVar9;
  wchar_t *pwVar10;
  wchar_t **ppwVar11;
  undefined1 auStack_3a8 [32];
  wchar_t *local_388;
  wchar_t *local_380;
  wchar_t **local_378;
  undefined4 local_368 [53];
  undefined1 local_291 [36];
  longlong local_26d;
  undefined1 local_232 [522];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_3a8;
  plVar1 = *(longlong **)(param_1 + 0x270);
  if (plVar1 != (longlong *)0x0) {
    (**(code **)(*plVar1 + 0x138))(plVar1,3);
  }
  *(undefined4 *)(param_1 + 0xd4) = 0;
  cVar4 = FUN_1406e8ae0(param_2);
  if (*(int *)(param_1 + 0xd0) < 3) {
    return;
  }
  if (cVar4 == '\n') {
    local_378 = &local_380;
    local_380 = (wchar_t *)0x0;
    local_388 = (wchar_t *)0x0;
    piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,0x2a);
    piVar8[1] = 0xc;
    *piVar8 = -1;
    local_388 = (wchar_t *)(piVar8 + 4);
    piVar8[2] = 0;
    *local_388 = L'\0';
    uVar3 = u_loginTimeOut_1433fdcd8._12_4_;
    uVar2 = u_loginTimeOut_1433fdcd8._8_4_;
    uVar6 = u_loginTimeOut_1433fdcd8._4_4_;
    *(undefined4 *)local_388 = u_loginTimeOut_1433fdcd8._0_4_;
    piVar8[5] = uVar6;
    piVar8[6] = uVar2;
    piVar8[7] = uVar3;
    *(undefined8 *)(piVar8 + 8) = u_loginTimeOut_1433fdcd8._16_8_;
    if (*piVar8 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar8[1] < 0xc) {
      FUN_142e54290(0x90,piVar8[1],0xc);
    }
    *piVar8 = 1;
    local_388[0xc] = L'\0';
    if (piVar8[1] + 1 < 0xd) {
      FUN_142e54290(0x9c,0xc);
    }
    piVar8[2] = 0x18;
  }
  else {
    if (cVar4 != '\x1e') {
      if (cVar4 == '>') {
        uVar7 = FUN_1408a9e40(&local_380,0x1286);
        FUN_140d84b70(uVar7,0);
        return;
      }
      if (cVar4 == 'c') {
        pwVar10 = L"unavailableClass";
      }
      else {
        if (cVar4 == 'i') {
          iVar5 = FUN_1406e8c20(param_2);
          if (iVar5 != *(int *)(param_1 + 0x1c0)) {
            return;
          }
          local_378 = &local_388;
          local_388 = (wchar_t *)0x0;
          uVar7 = FUN_1403edf80(&local_380,L"characterCreationRestrictedWorld",0xffffffff);
          FUN_141b4ac80(uVar7,&local_388,0);
          if (DAT_143aca790 != 0) {
            FUN_141177e40();
          }
          FUN_141b3f050(param_1,4,0x14a);
          return;
        }
        if (cVar4 == '\0') {
          cVar4 = FUN_14108cd60();
          if (cVar4 == '\0') {
            iVar5 = FUN_1406e8c20(param_2);
            if (iVar5 != *(int *)(param_1 + 0x1c0)) {
              return;
            }
            *(undefined4 *)(param_1 + 0x1a4) = 0;
            FUN_14108e7f0(local_368);
            FUN_1403094b0(local_368,param_2);
            FUN_14108c210(local_368);
            uVar6 = FUN_14108cd40();
            *(undefined4 *)(param_1 + 0xe8) = uVar6;
            uVar6 = FUN_14108d9b0(local_368[0]);
            if (DAT_143aa84a0 != 0) {
              FUN_142d129d0();
            }
            FUN_1406e8ae0(param_2);
            if (DAT_143aca790 != 0) {
              FUN_141177e40();
              FUN_141179200(DAT_143aca790,uVar6,1);
            }
            FUN_141b3f050(param_1,4,0x14a);
            iVar5 = FUN_142c4f6d0(DAT_143ac1898);
            if ((iVar5 != 0) && (cVar4 = FUN_14057e4c0(), cVar4 != '\0')) {
              uVar7 = FUN_1404c6160();
              local_388 = (wchar_t *)0x0;
              piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,0x23);
              piVar8[1] = 0x12;
              *piVar8 = -1;
              local_388 = (wchar_t *)(piVar8 + 4);
              piVar8[2] = 0;
              *(undefined1 *)local_388 = 0;
              uVar3 = s_GC_CreateCharacter_1433fe7b0._12_4_;
              uVar2 = s_GC_CreateCharacter_1433fe7b0._8_4_;
              uVar6 = s_GC_CreateCharacter_1433fe7b0._4_4_;
              *(undefined4 *)local_388 = s_GC_CreateCharacter_1433fe7b0._0_4_;
              piVar8[5] = uVar6;
              piVar8[6] = uVar2;
              piVar8[7] = uVar3;
              *(undefined2 *)(piVar8 + 8) = s_GC_CreateCharacter_1433fe7b0._16_2_;
              if (*piVar8 != -1) {
                FUN_142e52dd0(0x8b);
              }
              if (piVar8[1] < 0x12) {
                FUN_142e54290(0x90,piVar8[1],0x12);
              }
              *piVar8 = 1;
              *(undefined1 *)(local_388 + 9) = 0;
              if (piVar8[1] + 1 < 0x13) {
                FUN_142e54290(0x9c,0x12);
              }
              piVar8[2] = 0x12;
              FUN_1404c7800(uVar7,0x10,&local_388);
              if (local_388 != (wchar_t *)0x0) {
                FUN_14019f2c0(local_388 + -8);
              }
            }
            FUN_1401d5120(local_232);
            if (local_26d != 0) {
              thunk_FUN_140205820(local_26d,0xc);
            }
            FUN_141b43570(local_291);
            return;
          }
        }
        else if (cVar4 == '\t') {
          pwVar10 = L"insufficientCharacterSlot";
          goto LAB_141b36d18;
        }
        pwVar10 = L"cannotProcessRequest";
      }
LAB_141b36d18:
      local_378 = &local_388;
      local_388 = (wchar_t *)0x0;
      ppwVar9 = (wchar_t **)FUN_1403edf80(&local_380,pwVar10,0xffffffff);
      ppwVar11 = &local_388;
      goto LAB_141b36f28;
    }
    local_378 = &local_380;
    local_380 = (wchar_t *)0x0;
    local_388 = (wchar_t *)0x0;
    piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,0x34);
    piVar8[1] = 0x11;
    *piVar8 = -1;
    local_388 = (wchar_t *)(piVar8 + 4);
    piVar8[2] = 0;
    *local_388 = L'\0';
    uVar3 = u_cannotUseThisName_1433fde50._12_4_;
    uVar2 = u_cannotUseThisName_1433fde50._8_4_;
    uVar6 = u_cannotUseThisName_1433fde50._4_4_;
    *(undefined4 *)local_388 = u_cannotUseThisName_1433fde50._0_4_;
    piVar8[5] = uVar6;
    piVar8[6] = uVar2;
    piVar8[7] = uVar3;
    uVar3 = u_cannotUseThisName_1433fde50._28_4_;
    uVar2 = u_cannotUseThisName_1433fde50._24_4_;
    uVar6 = u_cannotUseThisName_1433fde50._20_4_;
    piVar8[8] = u_cannotUseThisName_1433fde50._16_4_;
    piVar8[9] = uVar6;
    piVar8[10] = uVar2;
    piVar8[0xb] = uVar3;
    *(wchar_t *)(piVar8 + 0xc) = u_cannotUseThisName_1433fde50[0x10];
    if (*piVar8 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar8[1] < 0x11) {
      FUN_142e54290(0x90,piVar8[1],0x11);
    }
    *piVar8 = 1;
    local_388[0x11] = L'\0';
    if (piVar8[1] + 1 < 0x12) {
      FUN_142e54290(0x9c,0x11);
    }
    piVar8[2] = 0x22;
  }
  ppwVar9 = &local_388;
  ppwVar11 = &local_380;
LAB_141b36f28:
  FUN_141b4ac80(ppwVar9,ppwVar11,0);
  return;
}



//===========================================================
// FUN_141b34970 @ 141b34970   (4001 bytes)
//===========================================================

void FUN_141b34970(longlong param_1,undefined8 param_2)

{
  longlong lVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  char cVar5;
  undefined1 uVar6;
  undefined1 uVar7;
  char cVar8;
  undefined4 uVar9;
  int iVar10;
  wchar_t *pwVar11;
  undefined8 *puVar12;
  undefined8 uVar13;
  char *pcVar14;
  int *piVar15;
  wchar_t *pwVar16;
  wchar_t *pwVar17;
  int local_res8 [2];
  int local_res18 [2];
  wchar_t *local_res20;
  wchar_t *local_c8;
  wchar_t **local_c0;
  wchar_t **local_b8;
  wchar_t *local_b0 [2];
  wchar_t *local_a0;
  
  if (DAT_143aca790 != (longlong *)0x0) {
    FUN_141179940();
  }
  pwVar17 = (wchar_t *)0x0;
  *(undefined4 *)(param_1 + 0xd4) = 0;
  uVar9 = FUN_1406e8c20(param_2);
  cVar5 = FUN_1406e8ae0(param_2);
  local_res8[0] = (int)cVar5;
  pwVar16 = (wchar_t *)0x0;
  local_res20 = (wchar_t *)0x0;
  switch(local_res8[0]) {
  case 6:
    piVar15 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3e);
    piVar15[1] = 0x16;
    *piVar15 = -1;
    pwVar17 = (wchar_t *)(piVar15 + 4);
    piVar15[2] = 0;
    *pwVar17 = L'\0';
    uVar13 = u_loginTroubleAskSupport_1433d5d98._8_8_;
    *(undefined8 *)pwVar17 = u_loginTroubleAskSupport_1433d5d98._0_8_;
    *(undefined8 *)(piVar15 + 6) = uVar13;
    uVar3 = u_loginTroubleAskSupport_1433d5d98._28_4_;
    uVar2 = u_loginTroubleAskSupport_1433d5d98._24_4_;
    uVar9 = u_loginTroubleAskSupport_1433d5d98._20_4_;
    piVar15[8] = u_loginTroubleAskSupport_1433d5d98._16_4_;
    piVar15[9] = uVar9;
    piVar15[10] = uVar2;
    piVar15[0xb] = uVar3;
    *(undefined8 *)(piVar15 + 0xc) = u_loginTroubleAskSupport_1433d5d98._32_8_;
    piVar15[0xe] = u_loginTroubleAskSupport_1433d5d98._40_4_;
    if (*piVar15 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar15[1] < 0x16) {
      FUN_142e54290(0x90,piVar15[1],0x16);
    }
    *piVar15 = 1;
    *(undefined2 *)(piVar15 + 0xf) = 0;
    if (piVar15[1] + 1 < 0x17) {
      FUN_142e54290(0x9c,0x16);
    }
    piVar15[2] = 0x2c;
    local_res20 = pwVar17;
    break;
  default:
    local_res18[0] = FUN_14108d9b0(uVar9);
    if ((local_res8[0] == 0) && (-1 < local_res18[0])) {
LAB_141b357ac:
      iVar10 = FUN_142c4f6d0(DAT_143ac1898);
      if ((iVar10 != 0) && (cVar5 = FUN_14057e4c0(), cVar5 != '\0')) {
        uVar13 = FUN_1404c6160();
        local_c8 = (wchar_t *)0x0;
        pcVar14 = (char *)FUN_14019bd40(&local_c8,0x12);
        pwVar11 = local_c8;
        uVar4 = s_GC_DeleteCharacter_1433fe6e8._12_4_;
        uVar3 = s_GC_DeleteCharacter_1433fe6e8._8_4_;
        uVar2 = s_GC_DeleteCharacter_1433fe6e8._4_4_;
        *(undefined4 *)pcVar14 = s_GC_DeleteCharacter_1433fe6e8._0_4_;
        *(undefined4 *)(pcVar14 + 4) = uVar2;
        *(undefined4 *)(pcVar14 + 8) = uVar3;
        *(undefined4 *)(pcVar14 + 0xc) = uVar4;
        *(undefined2 *)(pcVar14 + 0x10) = s_GC_DeleteCharacter_1433fe6e8._16_2_;
        if (*(int *)(local_c8 + -8) != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (*(int *)(pwVar11 + -6) < 0x12) {
          FUN_142e54290(0x90,*(int *)(pwVar11 + -6),0x12);
        }
        pwVar11[-8] = L'\x01';
        pwVar11[-7] = L'\0';
        *(undefined1 *)(local_c8 + 9) = 0;
        if (*(int *)(pwVar11 + -6) + 1 < 0x13) {
          FUN_142e54290(0x9c,0x12);
        }
        pwVar11[-4] = L'\x12';
        pwVar11[-3] = L'\0';
        FUN_1404c7800(uVar13,0x11,&local_c8);
        if (local_c8 != (wchar_t *)0x0) {
          FUN_14019f2c0(local_c8 + -8);
        }
      }
    }
    else {
      local_b0[0] = (wchar_t *)FUN_1418039d0(0x22000008);
      puVar12 = (undefined8 *)FUN_14105cbe0(&local_c8,local_b0,local_res8,local_res18);
      FUN_141804870(&DAT_143271f04,0xc1d,0x22000008,*puVar12);
      if (local_c8 != (wchar_t *)0x0) {
        FUN_14019f2c0(local_c8 + -8);
      }
      if (local_res8[0] == 0) goto LAB_141b357ac;
    }
    FUN_14108c500(uVar9);
    uVar9 = FUN_14108cd40();
    *(undefined4 *)(param_1 + 0xe8) = uVar9;
    if (DAT_143aca790 != (longlong *)0x0) {
      FUN_141177e40();
      iVar10 = *(int *)(param_1 + 0xe8);
      piVar15 = (int *)FUN_14108cc50(local_res18[0]);
      iVar10 = (0 < iVar10) - 1;
      if (*piVar15 != 0) {
        iVar10 = local_res18[0];
      }
      FUN_141179200(DAT_143aca790,iVar10,1);
      (**(code **)(*DAT_143aca790 + 0x90))(DAT_143aca790,0);
      pwVar17 = pwVar16;
    }
    goto LAB_141b358e7;
  case 9:
    FUN_1403edf80(local_b0,L"cannotProcessRequest",0xffffffffffffffff);
    local_res20 = local_b0[0];
    pwVar17 = local_b0[0];
    break;
  case 10:
    piVar15 = (int *)FUN_1401bc720(&DAT_143ad6980,0x2a);
    piVar15[1] = 0xc;
    *piVar15 = -1;
    pwVar17 = (wchar_t *)(piVar15 + 4);
    piVar15[2] = 0;
    *pwVar17 = L'\0';
    uVar3 = u_loginTimeOut_1433fdcd8._12_4_;
    uVar2 = u_loginTimeOut_1433fdcd8._8_4_;
    uVar9 = u_loginTimeOut_1433fdcd8._4_4_;
    *(undefined4 *)pwVar17 = u_loginTimeOut_1433fdcd8._0_4_;
    piVar15[5] = uVar9;
    piVar15[6] = uVar2;
    piVar15[7] = uVar3;
    *(undefined8 *)(piVar15 + 8) = u_loginTimeOut_1433fdcd8._16_8_;
    if (*piVar15 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar15[1] < 0xc) {
      FUN_142e54290(0x90,piVar15[1],0xc);
    }
    *piVar15 = 1;
    *(undefined2 *)(piVar15 + 10) = 0;
    if (piVar15[1] + 1 < 0xd) {
      FUN_142e54290(0x9c,0xc);
    }
    piVar15[2] = 0x18;
    local_res20 = pwVar17;
    break;
  case 0x10:
    piVar15 = (int *)FUN_1401bc720(&DAT_143ad6980,0x38);
    piVar15[1] = 0x13;
    *piVar15 = -1;
    pwVar17 = (wchar_t *)(piVar15 + 4);
    piVar15[2] = 0;
    *pwVar17 = L'\0';
    uVar3 = u_invalidBirthdayForm_1433fe3c8._12_4_;
    uVar2 = u_invalidBirthdayForm_1433fe3c8._8_4_;
    uVar9 = u_invalidBirthdayForm_1433fe3c8._4_4_;
    *(undefined4 *)pwVar17 = u_invalidBirthdayForm_1433fe3c8._0_4_;
    piVar15[5] = uVar9;
    piVar15[6] = uVar2;
    piVar15[7] = uVar3;
    uVar3 = u_invalidBirthdayForm_1433fe3c8._28_4_;
    uVar2 = u_invalidBirthdayForm_1433fe3c8._24_4_;
    uVar9 = u_invalidBirthdayForm_1433fe3c8._20_4_;
    piVar15[8] = u_invalidBirthdayForm_1433fe3c8._16_4_;
    piVar15[9] = uVar9;
    piVar15[10] = uVar2;
    piVar15[0xb] = uVar3;
    piVar15[0xc] = u_invalidBirthdayForm_1433fe3c8._32_4_;
    *(wchar_t *)(piVar15 + 0xd) = u_invalidBirthdayForm_1433fe3c8[0x12];
    if (*piVar15 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar15[1] < 0x13) {
      FUN_142e54290(0x90,piVar15[1],0x13);
    }
    *piVar15 = 1;
    *(undefined2 *)((longlong)piVar15 + 0x36) = 0;
    if (piVar15[1] + 1 < 0x14) {
      FUN_142e54290(0x9c,0x13);
    }
    piVar15[2] = 0x26;
    local_res20 = pwVar17;
    break;
  case 0x12:
    piVar15 = (int *)FUN_1401bc720(&DAT_143ad6980,0x40);
    piVar15[1] = 0x17;
    *piVar15 = -1;
    pwVar17 = (wchar_t *)(piVar15 + 4);
    piVar15[2] = 0;
    *pwVar17 = L'\0';
    uVar13 = u_cannotDeleteGuildmaster_1433fe490._8_8_;
    *(undefined8 *)pwVar17 = u_cannotDeleteGuildmaster_1433fe490._0_8_;
    *(undefined8 *)(piVar15 + 6) = uVar13;
    uVar3 = u_cannotDeleteGuildmaster_1433fe490._28_4_;
    uVar2 = u_cannotDeleteGuildmaster_1433fe490._24_4_;
    uVar9 = u_cannotDeleteGuildmaster_1433fe490._20_4_;
    piVar15[8] = u_cannotDeleteGuildmaster_1433fe490._16_4_;
    piVar15[9] = uVar9;
    piVar15[10] = uVar2;
    piVar15[0xb] = uVar3;
    *(undefined8 *)(piVar15 + 0xc) = u_cannotDeleteGuildmaster_1433fe490._32_8_;
    piVar15[0xe] = u_cannotDeleteGuildmaster_1433fe490._40_4_;
    *(wchar_t *)(piVar15 + 0xf) = u_cannotDeleteGuildmaster_1433fe490[0x16];
    if (*piVar15 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar15[1] < 0x17) {
      FUN_142e54290(0x90,piVar15[1],0x17);
    }
    *piVar15 = 1;
    *(undefined2 *)((longlong)piVar15 + 0x3e) = 0;
    if (piVar15[1] + 1 < 0x18) {
      FUN_142e54290(0x9c,0x17);
    }
    piVar15[2] = 0x2e;
    local_res20 = pwVar17;
    break;
  case 0x14:
    piVar15 = (int *)FUN_1401bc720(&DAT_143ad6980,0x2a);
    piVar15[1] = 0xc;
    *piVar15 = -1;
    pwVar17 = (wchar_t *)(piVar15 + 4);
    piVar15[2] = 0;
    *pwVar17 = L'\0';
    uVar3 = u_incorrectPIC_1433fe3f0._12_4_;
    uVar2 = u_incorrectPIC_1433fe3f0._8_4_;
    uVar9 = u_incorrectPIC_1433fe3f0._4_4_;
    *(undefined4 *)pwVar17 = u_incorrectPIC_1433fe3f0._0_4_;
    piVar15[5] = uVar9;
    piVar15[6] = uVar2;
    piVar15[7] = uVar3;
    *(undefined8 *)(piVar15 + 8) = u_incorrectPIC_1433fe3f0._16_8_;
    if (*piVar15 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar15[1] < 0xc) {
      FUN_142e54290(0x90,piVar15[1],0xc);
    }
    *piVar15 = 1;
    *(undefined2 *)(piVar15 + 10) = 0;
    if (piVar15[1] + 1 < 0xd) {
      FUN_142e54290(0x9c,0xc);
    }
    piVar15[2] = 0x18;
    local_res20 = pwVar17;
    break;
  case 0x15:
    piVar15 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3e);
    piVar15[1] = 0x16;
    *piVar15 = -1;
    pwVar17 = (wchar_t *)(piVar15 + 4);
    piVar15[2] = 0;
    *pwVar17 = L'\0';
    uVar13 = u_cannotDeleteOnMarriage_1433fe4c0._8_8_;
    *(undefined8 *)pwVar17 = u_cannotDeleteOnMarriage_1433fe4c0._0_8_;
    *(undefined8 *)(piVar15 + 6) = uVar13;
    uVar3 = u_cannotDeleteOnMarriage_1433fe4c0._28_4_;
    uVar2 = u_cannotDeleteOnMarriage_1433fe4c0._24_4_;
    uVar9 = u_cannotDeleteOnMarriage_1433fe4c0._20_4_;
    piVar15[8] = u_cannotDeleteOnMarriage_1433fe4c0._16_4_;
    piVar15[9] = uVar9;
    piVar15[10] = uVar2;
    piVar15[0xb] = uVar3;
    *(undefined8 *)(piVar15 + 0xc) = u_cannotDeleteOnMarriage_1433fe4c0._32_8_;
    piVar15[0xe] = u_cannotDeleteOnMarriage_1433fe4c0._40_4_;
    if (*piVar15 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar15[1] < 0x16) {
      FUN_142e54290(0x90,piVar15[1],0x16);
    }
    *piVar15 = 1;
    *(undefined2 *)(piVar15 + 0xf) = 0;
    if (piVar15[1] + 1 < 0x17) {
      FUN_142e54290(0x9c,0x16);
    }
    piVar15[2] = 0x2c;
    local_res20 = pwVar17;
    break;
  case 0x1a:
    piVar15 = (int *)FUN_1401bc720(&DAT_143ad6980,0x2e);
    piVar15[1] = 0xe;
    *piVar15 = -1;
    pwVar17 = (wchar_t *)(piVar15 + 4);
    piVar15[2] = 0;
    *pwVar17 = L'\0';
    uVar3 = u_systemErrorOTP_1433fd820._12_4_;
    uVar2 = u_systemErrorOTP_1433fd820._8_4_;
    uVar9 = u_systemErrorOTP_1433fd820._4_4_;
    *(undefined4 *)pwVar17 = u_systemErrorOTP_1433fd820._0_4_;
    piVar15[5] = uVar9;
    piVar15[6] = uVar2;
    piVar15[7] = uVar3;
    *(undefined8 *)(piVar15 + 8) = u_systemErrorOTP_1433fd820._16_8_;
    piVar15[10] = u_systemErrorOTP_1433fd820._24_4_;
    if (*piVar15 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar15[1] < 0xe) {
      FUN_142e54290(0x90,piVar15[1],0xe);
    }
    *piVar15 = 1;
    *(undefined2 *)(piVar15 + 0xb) = 0;
    if (piVar15[1] + 1 < 0xf) {
      FUN_142e54290(0x9c,0xe);
    }
    piVar15[2] = 0x1c;
    local_res20 = pwVar17;
    break;
  case 0x1c:
    piVar15 = (int *)FUN_1401bc720(&DAT_143ad6980,0x26);
    piVar15[1] = 10;
    *piVar15 = -1;
    pwVar17 = (wchar_t *)(piVar15 + 4);
    piVar15[2] = 0;
    *pwVar17 = L'\0';
    uVar3 = u_expiredOTP_1433fd808._12_4_;
    uVar2 = u_expiredOTP_1433fd808._8_4_;
    uVar9 = u_expiredOTP_1433fd808._4_4_;
    *(undefined4 *)pwVar17 = u_expiredOTP_1433fd808._0_4_;
    piVar15[5] = uVar9;
    piVar15[6] = uVar2;
    piVar15[7] = uVar3;
    piVar15[8] = u_expiredOTP_1433fd808._16_4_;
    if (*piVar15 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar15[1] < 10) {
      FUN_142e54290(0x90,piVar15[1],10);
    }
    *piVar15 = 1;
    *(undefined2 *)(piVar15 + 9) = 0;
    if (piVar15[1] + 1 < 0xb) {
      FUN_142e54290(0x9c,10);
    }
    piVar15[2] = 0x14;
    local_res20 = pwVar17;
    break;
  case 0x1d:
    piVar15 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3a);
    piVar15[1] = 0x14;
    *piVar15 = -1;
    pwVar17 = (wchar_t *)(piVar15 + 4);
    piVar15[2] = 0;
    *pwVar17 = L'\0';
    uVar13 = u_cannotDeleteInFamily_1433fe4f0._8_8_;
    *(undefined8 *)pwVar17 = u_cannotDeleteInFamily_1433fe4f0._0_8_;
    *(undefined8 *)(piVar15 + 6) = uVar13;
    uVar3 = u_cannotDeleteInFamily_1433fe4f0._28_4_;
    uVar2 = u_cannotDeleteInFamily_1433fe4f0._24_4_;
    uVar9 = u_cannotDeleteInFamily_1433fe4f0._20_4_;
    piVar15[8] = u_cannotDeleteInFamily_1433fe4f0._16_4_;
    piVar15[9] = uVar9;
    piVar15[10] = uVar2;
    piVar15[0xb] = uVar3;
    *(undefined8 *)(piVar15 + 0xc) = u_cannotDeleteInFamily_1433fe4f0._32_8_;
    if (*piVar15 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar15[1] < 0x14) {
      FUN_142e54290(0x90,piVar15[1],0x14);
    }
    *piVar15 = 1;
    *(undefined2 *)(piVar15 + 0xe) = 0;
    if (piVar15[1] + 1 < 0x15) {
      FUN_142e54290(0x9c,0x14);
    }
    piVar15[2] = 0x28;
    local_res20 = pwVar17;
    break;
  case 0x23:
    piVar15 = (int *)FUN_1401bc720(&DAT_143ad6980,0x2e);
    piVar15[1] = 0xe;
    *piVar15 = -1;
    pwVar17 = (wchar_t *)(piVar15 + 4);
    piVar15[2] = 0;
    *pwVar17 = L'\0';
    uVar3 = u_systemErrorOTP_1433fd820._12_4_;
    uVar2 = u_systemErrorOTP_1433fd820._8_4_;
    uVar9 = u_systemErrorOTP_1433fd820._4_4_;
    *(undefined4 *)pwVar17 = u_systemErrorOTP_1433fd820._0_4_;
    piVar15[5] = uVar9;
    piVar15[6] = uVar2;
    piVar15[7] = uVar3;
    *(undefined8 *)(piVar15 + 8) = u_systemErrorOTP_1433fd820._16_8_;
    piVar15[10] = u_systemErrorOTP_1433fd820._24_4_;
    if (*piVar15 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar15[1] < 0xe) {
      FUN_142e54290(0x90,piVar15[1],0xe);
    }
    *piVar15 = 1;
    *(undefined2 *)(piVar15 + 0xb) = 0;
    if (piVar15[1] + 1 < 0xf) {
      FUN_142e54290(0x9c,0xe);
    }
    piVar15[2] = 0x1c;
    local_res20 = pwVar17;
    break;
  case 0x24:
    local_c8 = (wchar_t *)0x0;
    pwVar16 = (wchar_t *)FUN_1401bd080(&local_c8,0xe);
    pwVar17 = local_c8;
    uVar3 = u_systemErrorOTP_1433fd820._12_4_;
    uVar2 = u_systemErrorOTP_1433fd820._8_4_;
    uVar9 = u_systemErrorOTP_1433fd820._4_4_;
    *(undefined4 *)pwVar16 = u_systemErrorOTP_1433fd820._0_4_;
    *(undefined4 *)(pwVar16 + 2) = uVar9;
    *(undefined4 *)(pwVar16 + 4) = uVar2;
    *(undefined4 *)(pwVar16 + 6) = uVar3;
    *(undefined8 *)(pwVar16 + 8) = u_systemErrorOTP_1433fd820._16_8_;
    *(undefined4 *)(pwVar16 + 0xc) = u_systemErrorOTP_1433fd820._24_4_;
    if (*(int *)(local_c8 + -8) != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (*(int *)(pwVar17 + -6) < 0xe) {
      FUN_142e54290(0x90,*(int *)(pwVar17 + -6),0xe);
    }
    pwVar17[-8] = L'\x01';
    pwVar17[-7] = L'\0';
    pwVar17[0xe] = L'\0';
    if (*(int *)(pwVar17 + -6) + 1 < 0xf) {
      FUN_142e54290(0x9c,0xe);
    }
    pwVar17[-4] = L'\x1c';
    pwVar17[-3] = L'\0';
    local_res20 = pwVar17;
    break;
  case 0x2c:
    FUN_1403edf80(local_b0,L"cannotDeleteCharacterByItemGuard",0xffffffffffffffff);
    local_res20 = local_b0[0];
    pwVar17 = local_b0[0];
    break;
  case 0x33:
    FUN_1403edf80(local_b0,L"cannotUsePINPartOfPIC",0xffffffffffffffff);
    local_res20 = local_b0[0];
    pwVar17 = local_b0[0];
    break;
  case 0x38:
    FUN_1403edf80(local_b0,L"cannotDeleteCharacterNotActive",0xffffffffffffffff);
    local_res20 = local_b0[0];
    pwVar17 = local_b0[0];
    break;
  case 0x39:
    piVar15 = (int *)FUN_1401bc720(&DAT_143ad6980,0x4a);
    piVar15[1] = 0x1c;
    *piVar15 = -1;
    pwVar17 = (wchar_t *)(piVar15 + 4);
    piVar15[2] = 0;
    *pwVar17 = L'\0';
    uVar13 = u_incorrectPICWarningOverCount_1433fe410._8_8_;
    *(undefined8 *)pwVar17 = u_incorrectPICWarningOverCount_1433fe410._0_8_;
    *(undefined8 *)(piVar15 + 6) = uVar13;
    uVar13 = u_incorrectPICWarningOverCount_1433fe410._24_8_;
    *(undefined8 *)(piVar15 + 8) = u_incorrectPICWarningOverCount_1433fe410._16_8_;
    *(undefined8 *)(piVar15 + 10) = uVar13;
    uVar3 = u_incorrectPICWarningOverCount_1433fe410._44_4_;
    uVar2 = u_incorrectPICWarningOverCount_1433fe410._40_4_;
    uVar9 = u_incorrectPICWarningOverCount_1433fe410._36_4_;
    piVar15[0xc] = u_incorrectPICWarningOverCount_1433fe410._32_4_;
    piVar15[0xd] = uVar9;
    piVar15[0xe] = uVar2;
    piVar15[0xf] = uVar3;
    *(undefined8 *)(piVar15 + 0x10) = u_incorrectPICWarningOverCount_1433fe410._48_8_;
    if (*piVar15 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar15[1] < 0x1c) {
      FUN_142e54290(0x90,piVar15[1],0x1c);
    }
    *piVar15 = 1;
    *(undefined2 *)(piVar15 + 0x12) = 0;
    if (piVar15[1] + 1 < 0x1d) {
      FUN_142e54290(0x9c,0x1c);
    }
    piVar15[2] = 0x38;
    local_res20 = pwVar17;
    break;
  case 0x3a:
    piVar15 = (int *)FUN_1401bc720(&DAT_143ad6980,0x4a);
    piVar15[1] = 0x1c;
    *piVar15 = -1;
    pwVar17 = (wchar_t *)(piVar15 + 4);
    piVar15[2] = 0;
    *pwVar17 = L'\0';
    uVar13 = u_incorrectPICCloseByOverCount_1433fe450._8_8_;
    *(undefined8 *)pwVar17 = u_incorrectPICCloseByOverCount_1433fe450._0_8_;
    *(undefined8 *)(piVar15 + 6) = uVar13;
    uVar13 = u_incorrectPICCloseByOverCount_1433fe450._24_8_;
    *(undefined8 *)(piVar15 + 8) = u_incorrectPICCloseByOverCount_1433fe450._16_8_;
    *(undefined8 *)(piVar15 + 10) = uVar13;
    uVar3 = u_incorrectPICCloseByOverCount_1433fe450._44_4_;
    uVar2 = u_incorrectPICCloseByOverCount_1433fe450._40_4_;
    uVar9 = u_incorrectPICCloseByOverCount_1433fe450._36_4_;
    piVar15[0xc] = u_incorrectPICCloseByOverCount_1433fe450._32_4_;
    piVar15[0xd] = uVar9;
    piVar15[0xe] = uVar2;
    piVar15[0xf] = uVar3;
    *(undefined8 *)(piVar15 + 0x10) = u_incorrectPICCloseByOverCount_1433fe450._48_8_;
    if (*piVar15 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar15[1] < 0x1c) {
      FUN_142e54290(0x90,piVar15[1],0x1c);
    }
    *piVar15 = 1;
    *(undefined2 *)(piVar15 + 0x12) = 0;
    if (piVar15[1] + 1 < 0x1d) {
      FUN_142e54290(0x9c,0x1c);
    }
    piVar15[2] = 0x38;
    goto LAB_141b3565f;
  case 0x45:
    cVar5 = FUN_1406e8ae0(param_2);
    FUN_1406e8ae0(param_2);
    uVar6 = FUN_1406e8ae0(param_2);
    uVar7 = FUN_1406e8ae0(param_2);
    if (cVar5 != '\0') {
      local_c0 = &local_c8;
      local_c8 = (wchar_t *)0x0;
      uVar13 = FUN_1403edf80(local_b0,L"cannotDeleteCharacterTextMismatch",0xffffffff);
      FUN_141b4ac80(uVar13,&local_c8,param_1 + 0x140);
    }
    FUN_141029040(&local_b8,param_2);
    local_c0 = (wchar_t **)FUN_14019b780(&DAT_143ad68a0,0x340);
    pwVar16 = pwVar17;
    if (local_c0 != (wchar_t **)0x0) {
      pwVar16 = (wchar_t *)FUN_14130a540(local_c0,param_1,0x3c);
    }
    pwVar11 = pwVar16 + 0xc;
    if (pwVar16 == (wchar_t *)0x0) {
      pwVar11 = pwVar17;
    }
    if (pwVar11 == (wchar_t *)0x0) {
      local_a0 = (wchar_t *)0x0;
    }
    else {
      local_a0 = pwVar11 + -0xc;
      if (local_a0 != (wchar_t *)0x0) {
        if (0xfffff < *(ulonglong *)(pwVar11 + 4)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(pwVar11 + 4) = *(longlong *)(pwVar11 + 4) + 1;
        UNLOCK();
      }
    }
    pwVar17 = local_a0;
    if (local_a0 == (wchar_t *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    local_c0 = local_b8;
    if (local_b8 != (wchar_t **)0x0) {
      (**(code **)(*local_b8 + 4))();
    }
    FUN_14130cbc0(pwVar17,&local_c0,uVar6,uVar7);
    if (pwVar17 == (wchar_t *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    (**(code **)(*(longlong *)pwVar17 + 0x130))(pwVar17);
    if (0xffffe < *(longlong *)(pwVar17 + 0x10) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    pwVar17 = pwVar17 + 0x10;
    lVar1 = *(longlong *)pwVar17;
    *(longlong *)pwVar17 = *(longlong *)pwVar17 + -1;
    UNLOCK();
    if (((int)lVar1 == 1) && (pwVar17 = local_a0 + 0xc, pwVar17 != (wchar_t *)0x0)) {
      (*(code *)**(undefined8 **)pwVar17)(pwVar17,1);
    }
    pwVar17 = local_res20;
    if (local_b8 != (wchar_t **)0x0) {
      (**(code **)(*local_b8 + 8))();
      pwVar17 = local_res20;
    }
    goto LAB_141b358e7;
  case 0x46:
    FUN_1403edf80(local_b0,L"cannotProcessRequest",0xffffffffffffffff);
    local_res20 = local_b0[0];
    pwVar17 = local_b0[0];
    break;
  case 0x47:
    cVar5 = FUN_1406e8ae0(param_2);
    iVar10 = FUN_142c4f6d0(DAT_143ac1898);
    if ((iVar10 != 0) && (cVar8 = FUN_14057e4c0(), cVar8 != '\0')) {
      uVar13 = FUN_1404c6160();
      local_c8 = (wchar_t *)0x0;
      pcVar14 = (char *)FUN_14019bd40(&local_c8,0x1a);
      pwVar17 = local_c8;
      uVar3 = s_GC_LoginAntiMacroIncorrect_1433fe6c8._12_4_;
      uVar2 = s_GC_LoginAntiMacroIncorrect_1433fe6c8._8_4_;
      uVar9 = s_GC_LoginAntiMacroIncorrect_1433fe6c8._4_4_;
      *(undefined4 *)pcVar14 = s_GC_LoginAntiMacroIncorrect_1433fe6c8._0_4_;
      *(undefined4 *)(pcVar14 + 4) = uVar9;
      *(undefined4 *)(pcVar14 + 8) = uVar2;
      *(undefined4 *)(pcVar14 + 0xc) = uVar3;
      *(undefined8 *)(pcVar14 + 0x10) = s_GC_LoginAntiMacroIncorrect_1433fe6c8._16_8_;
      *(undefined2 *)(pcVar14 + 0x18) = s_GC_LoginAntiMacroIncorrect_1433fe6c8._24_2_;
      if (*(int *)(local_c8 + -8) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (*(int *)(pwVar17 + -6) < 0x1a) {
        FUN_142e54290(0x90,*(int *)(pwVar17 + -6),0x1a);
      }
      pwVar17[-8] = L'\x01';
      pwVar17[-7] = L'\0';
      *(undefined1 *)(local_c8 + 0xd) = 0;
      if (*(int *)(pwVar17 + -6) + 1 < 0x1b) {
        FUN_142e54290(0x9c,0x1a);
      }
      pwVar17[-4] = L'\x1a';
      pwVar17[-3] = L'\0';
      FUN_1404c7800(uVar13,0x12,&local_c8);
      if (local_c8 != (wchar_t *)0x0) {
        FUN_14019f2c0(local_c8 + -8);
      }
    }
    if (cVar5 == '\0') {
      local_b8 = local_b0;
      local_b0[0] = (wchar_t *)0x0;
      uVar13 = FUN_1403edf80(&local_c8,L"antimacroTextFailTooMuch",0xffffffffffffffff);
      FUN_141b4ac80(uVar13,local_b0,param_1 + 0x140);
      local_b0[0] = (wchar_t *)0x0;
      puVar12 = (undefined8 *)FUN_1408a9e40(&local_c8,3);
      FUN_14019ba10(local_b0,*puVar12);
      if (local_c8 != (wchar_t *)0x0) {
        FUN_14019f2c0(local_c8 + -8);
      }
      pwVar11 = local_b0[0];
      FUN_1429e4fa0(local_b0[0],0,0);
      *(undefined4 *)(param_1 + 0xf0) = 1;
      pwVar17 = pwVar16;
      if (pwVar11 != (wchar_t *)0x0) {
        FUN_14019f2c0(pwVar11 + -8);
      }
      goto LAB_141b358e7;
    }
    FUN_1403edf80(local_b0,L"cannotDeleteCharacterTextMismatch",0xffffffffffffffff);
    pwVar17 = local_b0[0];
LAB_141b3565f:
    local_res20 = pwVar17;
    FUN_141b2d290(param_1,0,0);
    break;
  case 0x5d:
    FUN_1403edf80(local_b0,L"cannotDeleteCharacterMapleAuction",0xffffffffffffffff);
    local_res20 = local_b0[0];
    pwVar17 = local_b0[0];
    break;
  case 0x60:
    FUN_1403edf80(local_b0,L"cannotDeleteCharacterAssignedLegion",0xffffffffffffffff);
    local_res20 = local_b0[0];
    pwVar17 = local_b0[0];
  }
  local_b8 = local_b0;
  local_b0[0] = (wchar_t *)0x0;
  local_c8 = (wchar_t *)0x0;
  FUN_1401c1fb0(&local_c8,&local_res20);
  FUN_141b4ac80(&local_c8,local_b0,param_1 + 0x140);
LAB_141b358e7:
  if (pwVar17 != (wchar_t *)0x0) {
    FUN_1401bebb0(pwVar17 + -8);
  }
  return;
}



//===========================================================
// FUN_141b359e0 @ 141b359e0   (2903 bytes)
//===========================================================

void FUN_141b359e0(longlong param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  char cVar4;
  undefined4 uVar5;
  int *piVar6;
  undefined8 *puVar7;
  wchar_t *pwVar8;
  int local_res8 [2];
  int local_res18 [2];
  wchar_t *local_res20;
  longlong local_b8;
  wchar_t *local_b0;
  undefined8 local_a8;
  undefined8 local_a0;
  wchar_t **local_98;
  
  *(undefined4 *)(param_1 + 0xd4) = 0;
  uVar5 = FUN_1406e8c20(param_2);
  cVar4 = FUN_1406e8ae0(param_2);
  local_res8[0] = (int)cVar4;
  local_res20 = (wchar_t *)0x0;
  switch(local_res8[0]) {
  case 6:
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3e);
    piVar6[1] = 0x16;
    *piVar6 = -1;
    pwVar8 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *pwVar8 = L'\0';
    uVar1 = u_loginTroubleAskSupport_1433d5d98._8_8_;
    *(undefined8 *)pwVar8 = u_loginTroubleAskSupport_1433d5d98._0_8_;
    *(undefined8 *)(piVar6 + 6) = uVar1;
    uVar3 = u_loginTroubleAskSupport_1433d5d98._28_4_;
    uVar2 = u_loginTroubleAskSupport_1433d5d98._24_4_;
    uVar5 = u_loginTroubleAskSupport_1433d5d98._20_4_;
    piVar6[8] = u_loginTroubleAskSupport_1433d5d98._16_4_;
    piVar6[9] = uVar5;
    piVar6[10] = uVar2;
    piVar6[0xb] = uVar3;
    *(undefined8 *)(piVar6 + 0xc) = u_loginTroubleAskSupport_1433d5d98._32_8_;
    piVar6[0xe] = u_loginTroubleAskSupport_1433d5d98._40_4_;
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0x16) {
      FUN_142e54290(0x90,piVar6[1],0x16);
    }
    *piVar6 = 1;
    *(undefined2 *)(piVar6 + 0xf) = 0;
    if (piVar6[1] + 1 < 0x17) {
      FUN_142e54290(0x9c,0x16);
    }
    piVar6[2] = 0x2c;
    local_res20 = pwVar8;
    break;
  default:
    local_a8 = DAT_1433fd510;
    local_a0 = DAT_1433fd510;
    if (cVar4 == '\0') {
      FUN_1406e9170(param_2,&local_a0,8);
      FUN_1406e9170(param_2,&local_a8,8);
    }
    FUN_14108d140(uVar5,local_a8);
    local_res18[0] = FUN_14108d9b0(uVar5);
    if ((local_res8[0] != 0) || (local_res18[0] < 0)) {
      local_b0 = (wchar_t *)FUN_1418039d0(0x22000008);
      puVar7 = (undefined8 *)FUN_14105cbe0(&local_b8,&local_b0,local_res8,local_res18);
      FUN_141804870(&DAT_143271f04,0xc9e,0x22000008,*puVar7);
      if (local_b8 != 0) {
        FUN_14019f2c0(local_b8 + -0x10);
      }
    }
    pwVar8 = (wchar_t *)0x0;
    if (DAT_143aca790 != 0) {
      FUN_141179200(DAT_143aca790,*(undefined4 *)(param_1 + 0x128),0);
      FUN_141177e40(DAT_143aca790);
      pwVar8 = (wchar_t *)0x0;
    }
    goto LAB_141b36511;
  case 10:
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x2a);
    piVar6[1] = 0xc;
    *piVar6 = -1;
    pwVar8 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *pwVar8 = L'\0';
    uVar3 = u_loginTimeOut_1433fdcd8._12_4_;
    uVar2 = u_loginTimeOut_1433fdcd8._8_4_;
    uVar5 = u_loginTimeOut_1433fdcd8._4_4_;
    *(undefined4 *)pwVar8 = u_loginTimeOut_1433fdcd8._0_4_;
    piVar6[5] = uVar5;
    piVar6[6] = uVar2;
    piVar6[7] = uVar3;
    *(undefined8 *)(piVar6 + 8) = u_loginTimeOut_1433fdcd8._16_8_;
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0xc) {
      FUN_142e54290(0x90,piVar6[1],0xc);
    }
    *piVar6 = 1;
    *(undefined2 *)(piVar6 + 10) = 0;
    if (piVar6[1] + 1 < 0xd) {
      FUN_142e54290(0x9c,0xc);
    }
    piVar6[2] = 0x18;
    local_res20 = pwVar8;
    break;
  case 0x10:
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x38);
    piVar6[1] = 0x13;
    *piVar6 = -1;
    pwVar8 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *pwVar8 = L'\0';
    uVar3 = u_invalidBirthdayForm_1433fe3c8._12_4_;
    uVar2 = u_invalidBirthdayForm_1433fe3c8._8_4_;
    uVar5 = u_invalidBirthdayForm_1433fe3c8._4_4_;
    *(undefined4 *)pwVar8 = u_invalidBirthdayForm_1433fe3c8._0_4_;
    piVar6[5] = uVar5;
    piVar6[6] = uVar2;
    piVar6[7] = uVar3;
    uVar3 = u_invalidBirthdayForm_1433fe3c8._28_4_;
    uVar2 = u_invalidBirthdayForm_1433fe3c8._24_4_;
    uVar5 = u_invalidBirthdayForm_1433fe3c8._20_4_;
    piVar6[8] = u_invalidBirthdayForm_1433fe3c8._16_4_;
    piVar6[9] = uVar5;
    piVar6[10] = uVar2;
    piVar6[0xb] = uVar3;
    piVar6[0xc] = u_invalidBirthdayForm_1433fe3c8._32_4_;
    *(wchar_t *)(piVar6 + 0xd) = u_invalidBirthdayForm_1433fe3c8[0x12];
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0x13) {
      FUN_142e54290(0x90,piVar6[1],0x13);
    }
    *piVar6 = 1;
    *(undefined2 *)((longlong)piVar6 + 0x36) = 0;
    if (piVar6[1] + 1 < 0x14) {
      FUN_142e54290(0x9c,0x13);
    }
    piVar6[2] = 0x26;
    local_res20 = pwVar8;
    break;
  case 0x12:
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x40);
    piVar6[1] = 0x17;
    *piVar6 = -1;
    pwVar8 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *pwVar8 = L'\0';
    uVar1 = u_cannotDeleteGuildmaster_1433fe490._8_8_;
    *(undefined8 *)pwVar8 = u_cannotDeleteGuildmaster_1433fe490._0_8_;
    *(undefined8 *)(piVar6 + 6) = uVar1;
    uVar3 = u_cannotDeleteGuildmaster_1433fe490._28_4_;
    uVar2 = u_cannotDeleteGuildmaster_1433fe490._24_4_;
    uVar5 = u_cannotDeleteGuildmaster_1433fe490._20_4_;
    piVar6[8] = u_cannotDeleteGuildmaster_1433fe490._16_4_;
    piVar6[9] = uVar5;
    piVar6[10] = uVar2;
    piVar6[0xb] = uVar3;
    *(undefined8 *)(piVar6 + 0xc) = u_cannotDeleteGuildmaster_1433fe490._32_8_;
    piVar6[0xe] = u_cannotDeleteGuildmaster_1433fe490._40_4_;
    *(wchar_t *)(piVar6 + 0xf) = u_cannotDeleteGuildmaster_1433fe490[0x16];
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0x17) {
      FUN_142e54290(0x90,piVar6[1],0x17);
    }
    *piVar6 = 1;
    *(undefined2 *)((longlong)piVar6 + 0x3e) = 0;
    if (piVar6[1] + 1 < 0x18) {
      FUN_142e54290(0x9c,0x17);
    }
    piVar6[2] = 0x2e;
    local_res20 = pwVar8;
    break;
  case 0x14:
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x2a);
    piVar6[1] = 0xc;
    *piVar6 = -1;
    pwVar8 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *pwVar8 = L'\0';
    uVar3 = u_incorrectPIC_1433fe3f0._12_4_;
    uVar2 = u_incorrectPIC_1433fe3f0._8_4_;
    uVar5 = u_incorrectPIC_1433fe3f0._4_4_;
    *(undefined4 *)pwVar8 = u_incorrectPIC_1433fe3f0._0_4_;
    piVar6[5] = uVar5;
    piVar6[6] = uVar2;
    piVar6[7] = uVar3;
    *(undefined8 *)(piVar6 + 8) = u_incorrectPIC_1433fe3f0._16_8_;
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0xc) {
      FUN_142e54290(0x90,piVar6[1],0xc);
    }
    *piVar6 = 1;
    *(undefined2 *)(piVar6 + 10) = 0;
    if (piVar6[1] + 1 < 0xd) {
      FUN_142e54290(0x9c,0xc);
    }
    piVar6[2] = 0x18;
    local_res20 = pwVar8;
    break;
  case 0x15:
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3e);
    piVar6[1] = 0x16;
    *piVar6 = -1;
    pwVar8 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *pwVar8 = L'\0';
    uVar1 = u_cannotDeleteOnMarriage_1433fe4c0._8_8_;
    *(undefined8 *)pwVar8 = u_cannotDeleteOnMarriage_1433fe4c0._0_8_;
    *(undefined8 *)(piVar6 + 6) = uVar1;
    uVar3 = u_cannotDeleteOnMarriage_1433fe4c0._28_4_;
    uVar2 = u_cannotDeleteOnMarriage_1433fe4c0._24_4_;
    uVar5 = u_cannotDeleteOnMarriage_1433fe4c0._20_4_;
    piVar6[8] = u_cannotDeleteOnMarriage_1433fe4c0._16_4_;
    piVar6[9] = uVar5;
    piVar6[10] = uVar2;
    piVar6[0xb] = uVar3;
    *(undefined8 *)(piVar6 + 0xc) = u_cannotDeleteOnMarriage_1433fe4c0._32_8_;
    piVar6[0xe] = u_cannotDeleteOnMarriage_1433fe4c0._40_4_;
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0x16) {
      FUN_142e54290(0x90,piVar6[1],0x16);
    }
    *piVar6 = 1;
    *(undefined2 *)(piVar6 + 0xf) = 0;
    if (piVar6[1] + 1 < 0x17) {
      FUN_142e54290(0x9c,0x16);
    }
    piVar6[2] = 0x2c;
    local_res20 = pwVar8;
    break;
  case 0x1a:
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x2e);
    piVar6[1] = 0xe;
    *piVar6 = -1;
    pwVar8 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *pwVar8 = L'\0';
    uVar3 = u_systemErrorOTP_1433fd820._12_4_;
    uVar2 = u_systemErrorOTP_1433fd820._8_4_;
    uVar5 = u_systemErrorOTP_1433fd820._4_4_;
    *(undefined4 *)pwVar8 = u_systemErrorOTP_1433fd820._0_4_;
    piVar6[5] = uVar5;
    piVar6[6] = uVar2;
    piVar6[7] = uVar3;
    *(undefined8 *)(piVar6 + 8) = u_systemErrorOTP_1433fd820._16_8_;
    piVar6[10] = u_systemErrorOTP_1433fd820._24_4_;
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0xe) {
      FUN_142e54290(0x90,piVar6[1],0xe);
    }
    *piVar6 = 1;
    *(undefined2 *)(piVar6 + 0xb) = 0;
    if (piVar6[1] + 1 < 0xf) {
      FUN_142e54290(0x9c,0xe);
    }
    piVar6[2] = 0x1c;
    local_res20 = pwVar8;
    break;
  case 0x1c:
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x26);
    piVar6[1] = 10;
    *piVar6 = -1;
    pwVar8 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *pwVar8 = L'\0';
    uVar3 = u_expiredOTP_1433fd808._12_4_;
    uVar2 = u_expiredOTP_1433fd808._8_4_;
    uVar5 = u_expiredOTP_1433fd808._4_4_;
    *(undefined4 *)pwVar8 = u_expiredOTP_1433fd808._0_4_;
    piVar6[5] = uVar5;
    piVar6[6] = uVar2;
    piVar6[7] = uVar3;
    piVar6[8] = u_expiredOTP_1433fd808._16_4_;
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 10) {
      FUN_142e54290(0x90,piVar6[1],10);
    }
    *piVar6 = 1;
    *(undefined2 *)(piVar6 + 9) = 0;
    if (piVar6[1] + 1 < 0xb) {
      FUN_142e54290(0x9c,10);
    }
    piVar6[2] = 0x14;
    local_res20 = pwVar8;
    break;
  case 0x1d:
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3a);
    piVar6[1] = 0x14;
    *piVar6 = -1;
    pwVar8 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *pwVar8 = L'\0';
    uVar1 = u_cannotDeleteInFamily_1433fe4f0._8_8_;
    *(undefined8 *)pwVar8 = u_cannotDeleteInFamily_1433fe4f0._0_8_;
    *(undefined8 *)(piVar6 + 6) = uVar1;
    uVar3 = u_cannotDeleteInFamily_1433fe4f0._28_4_;
    uVar2 = u_cannotDeleteInFamily_1433fe4f0._24_4_;
    uVar5 = u_cannotDeleteInFamily_1433fe4f0._20_4_;
    piVar6[8] = u_cannotDeleteInFamily_1433fe4f0._16_4_;
    piVar6[9] = uVar5;
    piVar6[10] = uVar2;
    piVar6[0xb] = uVar3;
    *(undefined8 *)(piVar6 + 0xc) = u_cannotDeleteInFamily_1433fe4f0._32_8_;
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0x14) {
      FUN_142e54290(0x90,piVar6[1],0x14);
    }
    *piVar6 = 1;
    *(undefined2 *)(piVar6 + 0xe) = 0;
    if (piVar6[1] + 1 < 0x15) {
      FUN_142e54290(0x9c,0x14);
    }
    piVar6[2] = 0x28;
    local_res20 = pwVar8;
    break;
  case 0x23:
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x2e);
    piVar6[1] = 0xe;
    *piVar6 = -1;
    pwVar8 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *pwVar8 = L'\0';
    uVar3 = u_systemErrorOTP_1433fd820._12_4_;
    uVar2 = u_systemErrorOTP_1433fd820._8_4_;
    uVar5 = u_systemErrorOTP_1433fd820._4_4_;
    *(undefined4 *)pwVar8 = u_systemErrorOTP_1433fd820._0_4_;
    piVar6[5] = uVar5;
    piVar6[6] = uVar2;
    piVar6[7] = uVar3;
    *(undefined8 *)(piVar6 + 8) = u_systemErrorOTP_1433fd820._16_8_;
    piVar6[10] = u_systemErrorOTP_1433fd820._24_4_;
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0xe) {
      FUN_142e54290(0x90,piVar6[1],0xe);
    }
    *piVar6 = 1;
    *(undefined2 *)(piVar6 + 0xb) = 0;
    if (piVar6[1] + 1 < 0xf) {
      FUN_142e54290(0x9c,0xe);
    }
    piVar6[2] = 0x1c;
    local_res20 = pwVar8;
    break;
  case 0x24:
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x2e);
    piVar6[1] = 0xe;
    *piVar6 = -1;
    pwVar8 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *pwVar8 = L'\0';
    uVar3 = u_systemErrorOTP_1433fd820._12_4_;
    uVar2 = u_systemErrorOTP_1433fd820._8_4_;
    uVar5 = u_systemErrorOTP_1433fd820._4_4_;
    *(undefined4 *)pwVar8 = u_systemErrorOTP_1433fd820._0_4_;
    piVar6[5] = uVar5;
    piVar6[6] = uVar2;
    piVar6[7] = uVar3;
    *(undefined8 *)(piVar6 + 8) = u_systemErrorOTP_1433fd820._16_8_;
    piVar6[10] = u_systemErrorOTP_1433fd820._24_4_;
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0xe) {
      FUN_142e54290(0x90,piVar6[1],0xe);
    }
    *piVar6 = 1;
    *(undefined2 *)(piVar6 + 0xb) = 0;
    if (piVar6[1] + 1 < 0xf) {
      FUN_142e54290(0x9c,0xe);
    }
    piVar6[2] = 0x1c;
    local_res20 = pwVar8;
    break;
  case 0x2c:
    FUN_1403edf80(&local_b0,L"cannotDeleteCharacterByItemGuard",0xffffffff);
    local_res20 = local_b0;
    pwVar8 = local_b0;
    break;
  case 0x33:
    FUN_1403edf80(&local_b0,L"cannotUsePINPartOfPIC",0xffffffff);
    local_res20 = local_b0;
    pwVar8 = local_b0;
    break;
  case 0x38:
    FUN_1403edf80(&local_b0,L"cannotDeleteCharacterNotActive",0xffffffff);
    local_res20 = local_b0;
    pwVar8 = local_b0;
    break;
  case 0x39:
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x4a);
    piVar6[1] = 0x1c;
    *piVar6 = -1;
    pwVar8 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *pwVar8 = L'\0';
    uVar1 = u_incorrectPICWarningOverCount_1433fe410._8_8_;
    *(undefined8 *)pwVar8 = u_incorrectPICWarningOverCount_1433fe410._0_8_;
    *(undefined8 *)(piVar6 + 6) = uVar1;
    uVar1 = u_incorrectPICWarningOverCount_1433fe410._24_8_;
    *(undefined8 *)(piVar6 + 8) = u_incorrectPICWarningOverCount_1433fe410._16_8_;
    *(undefined8 *)(piVar6 + 10) = uVar1;
    uVar3 = u_incorrectPICWarningOverCount_1433fe410._44_4_;
    uVar2 = u_incorrectPICWarningOverCount_1433fe410._40_4_;
    uVar5 = u_incorrectPICWarningOverCount_1433fe410._36_4_;
    piVar6[0xc] = u_incorrectPICWarningOverCount_1433fe410._32_4_;
    piVar6[0xd] = uVar5;
    piVar6[0xe] = uVar2;
    piVar6[0xf] = uVar3;
    *(undefined8 *)(piVar6 + 0x10) = u_incorrectPICWarningOverCount_1433fe410._48_8_;
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0x1c) {
      FUN_142e54290(0x90,piVar6[1],0x1c);
    }
    *piVar6 = 1;
    *(undefined2 *)(piVar6 + 0x12) = 0;
    if (piVar6[1] + 1 < 0x1d) {
      FUN_142e54290(0x9c,0x1c);
    }
    piVar6[2] = 0x38;
    local_res20 = pwVar8;
    break;
  case 0x3a:
    piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x4a);
    piVar6[1] = 0x1c;
    *piVar6 = -1;
    pwVar8 = (wchar_t *)(piVar6 + 4);
    piVar6[2] = 0;
    *pwVar8 = L'\0';
    uVar1 = u_incorrectPICCloseByOverCount_1433fe450._8_8_;
    *(undefined8 *)pwVar8 = u_incorrectPICCloseByOverCount_1433fe450._0_8_;
    *(undefined8 *)(piVar6 + 6) = uVar1;
    uVar1 = u_incorrectPICCloseByOverCount_1433fe450._24_8_;
    *(undefined8 *)(piVar6 + 8) = u_incorrectPICCloseByOverCount_1433fe450._16_8_;
    *(undefined8 *)(piVar6 + 10) = uVar1;
    uVar3 = u_incorrectPICCloseByOverCount_1433fe450._44_4_;
    uVar2 = u_incorrectPICCloseByOverCount_1433fe450._40_4_;
    uVar5 = u_incorrectPICCloseByOverCount_1433fe450._36_4_;
    piVar6[0xc] = u_incorrectPICCloseByOverCount_1433fe450._32_4_;
    piVar6[0xd] = uVar5;
    piVar6[0xe] = uVar2;
    piVar6[0xf] = uVar3;
    *(undefined8 *)(piVar6 + 0x10) = u_incorrectPICCloseByOverCount_1433fe450._48_8_;
    if (*piVar6 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar6[1] < 0x1c) {
      FUN_142e54290(0x90,piVar6[1],0x1c);
    }
    *piVar6 = 1;
    *(undefined2 *)(piVar6 + 0x12) = 0;
    if (piVar6[1] + 1 < 0x1d) {
      FUN_142e54290(0x9c,0x1c);
    }
    piVar6[2] = 0x38;
    local_res20 = pwVar8;
    FUN_141b2d290(param_1,0,0);
    break;
  case 0x41:
    FUN_1403edf80(&local_b0,L"cannotDeleteCharacterOvercountToday",0xffffffff);
    local_res20 = local_b0;
    pwVar8 = local_b0;
    break;
  case 0x5d:
    FUN_1403edf80(&local_b0,L"cannotDeleteCharacterMapleAuction",0xffffffff);
    local_res20 = local_b0;
    pwVar8 = local_b0;
    break;
  case 0x60:
    FUN_1403edf80(&local_b0,L"cannotDeleteCharacterAssignedLegion",0xffffffff);
    local_res20 = local_b0;
    pwVar8 = local_b0;
  }
  local_98 = &local_b0;
  local_b0 = (wchar_t *)0x0;
  local_b8 = 0;
  FUN_1401c1fb0(&local_b8,&local_res20);
  FUN_141b4ac80(&local_b8,&local_b0,param_1 + 0x140);
LAB_141b36511:
  if (pwVar8 != (wchar_t *)0x0) {
    FUN_1401bebb0(pwVar8 + -8);
  }
  return;
}



//===========================================================
// FUN_141b365f0 @ 141b365f0   (1046 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000141b3695e) */

void FUN_141b365f0(longlong param_1,undefined8 param_2)

{
  wchar_t *pwVar1;
  undefined8 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  wchar_t *pwVar5;
  char cVar6;
  undefined4 uVar7;
  int *piVar8;
  undefined8 *puVar9;
  wchar_t *pwVar10;
  int iVar11;
  int iVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  int local_res8 [2];
  int local_res18 [2];
  wchar_t *local_res20;
  longlong local_60;
  longlong *local_58;
  wchar_t *local_50;
  
  *(undefined4 *)(param_1 + 0xd4) = 0;
  uVar7 = FUN_1406e8c20(param_2);
  cVar6 = FUN_1406e8ae0(param_2);
  local_res8[0] = (int)cVar6;
  if (local_res8[0] == 6) {
    piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3e);
    piVar8[1] = 0x16;
    *piVar8 = -1;
    pwVar10 = (wchar_t *)(piVar8 + 4);
    piVar8[2] = 0;
    *pwVar10 = L'\0';
    uVar2 = u_loginTroubleAskSupport_1433d5d98._8_8_;
    *(undefined8 *)pwVar10 = u_loginTroubleAskSupport_1433d5d98._0_8_;
    *(undefined8 *)(piVar8 + 6) = uVar2;
    uVar4 = u_loginTroubleAskSupport_1433d5d98._28_4_;
    uVar3 = u_loginTroubleAskSupport_1433d5d98._24_4_;
    uVar7 = u_loginTroubleAskSupport_1433d5d98._20_4_;
    piVar8[8] = u_loginTroubleAskSupport_1433d5d98._16_4_;
    piVar8[9] = uVar7;
    piVar8[10] = uVar3;
    piVar8[0xb] = uVar4;
    *(undefined8 *)(piVar8 + 0xc) = u_loginTroubleAskSupport_1433d5d98._32_8_;
    piVar8[0xe] = u_loginTroubleAskSupport_1433d5d98._40_4_;
    if (*piVar8 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar8[1] < 0x16) {
      FUN_142e54290(0x90,piVar8[1],0x16);
    }
    *piVar8 = 1;
    *(undefined2 *)(piVar8 + 0xf) = 0;
    if (piVar8[1] + 1 < 0x17) {
      FUN_142e54290(0x9c,0x16);
    }
    piVar8[2] = 0x2c;
  }
  else {
    if (local_res8[0] != 10) {
      FUN_14108d260(uVar7);
      local_res18[0] = FUN_14108d9b0(uVar7);
      if ((local_res8[0] != 0) || (local_res18[0] < 0)) {
        local_res20 = (wchar_t *)FUN_1418039d0(0x22000008);
        puVar9 = (undefined8 *)FUN_14105cbe0(&local_60,&local_res20,local_res8,local_res18);
        FUN_141804870(&DAT_143271f04,0xcc9,0x22000008,*puVar9);
        if (local_60 != 0) {
          FUN_14019f2c0(local_60 + -0x10);
        }
      }
      pwVar10 = (wchar_t *)0x0;
      if (DAT_143aca790 != 0) {
        FUN_141179200(DAT_143aca790,*(undefined4 *)(param_1 + 0x128),0);
        FUN_141177e40(DAT_143aca790);
        pwVar10 = (wchar_t *)0x0;
      }
      goto LAB_141b369df;
    }
    piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,0x2a);
    piVar8[1] = 0xc;
    *piVar8 = -1;
    pwVar10 = (wchar_t *)(piVar8 + 4);
    piVar8[2] = 0;
    *pwVar10 = L'\0';
    uVar4 = u_loginTimeOut_1433fdcd8._12_4_;
    uVar3 = u_loginTimeOut_1433fdcd8._8_4_;
    uVar7 = u_loginTimeOut_1433fdcd8._4_4_;
    *(undefined4 *)pwVar10 = u_loginTimeOut_1433fdcd8._0_4_;
    piVar8[5] = uVar7;
    piVar8[6] = uVar3;
    piVar8[7] = uVar4;
    *(undefined8 *)(piVar8 + 8) = u_loginTimeOut_1433fdcd8._16_8_;
    if (*piVar8 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar8[1] < 0xc) {
      FUN_142e54290(0x90,piVar8[1],0xc);
    }
    *piVar8 = 1;
    *(undefined2 *)(piVar8 + 10) = 0;
    if (piVar8[1] + 1 < 0xd) {
      FUN_142e54290(0x9c,0xc);
    }
    piVar8[2] = 0x18;
  }
  uVar14 = 0xffffffffffffffff;
  local_58 = &local_60;
  local_60 = 0;
  local_res20 = (wchar_t *)0x0;
  pwVar5 = local_res20;
  if ((pwVar10 != (wchar_t *)0x0) && (pwVar1 = pwVar10 + -8, pwVar1 != (wchar_t *)0x0)) {
    if (*(int *)pwVar1 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar13 = 0xffffffffffffffff;
      do {
        uVar13 = uVar13 + 1;
      } while (pwVar10[uVar13] != L'\0');
      iVar11 = (int)uVar13;
      iVar12 = 0;
      if (0 < iVar11) {
        iVar12 = iVar11;
      }
      piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar12 * 2 + 0x12));
      piVar8[1] = iVar12;
      *piVar8 = -1;
      pwVar5 = (wchar_t *)(piVar8 + 4);
      piVar8[2] = 0;
      *pwVar5 = L'\0';
      local_50 = pwVar5;
      FUN_142ef7ba0(pwVar5,pwVar10,(longlong)iVar11 * 2);
      if (*piVar8 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar11 == -1) || (iVar11 <= piVar8[1])) {
        *piVar8 = 1;
        if (iVar11 != -1) goto LAB_141b36916;
        if (pwVar5 == (wchar_t *)0x0) {
          uVar13 = 0;
        }
        else {
          do {
            uVar14 = uVar14 + 1;
          } while (pwVar5[uVar14] != L'\0');
          uVar13 = uVar14 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar8[1],uVar13 & 0xffffffff);
        *piVar8 = 1;
LAB_141b36916:
        pwVar5[iVar11] = L'\0';
      }
      iVar12 = (int)uVar13;
      if ((iVar12 < 0) || (piVar8[1] + 1 <= iVar12)) {
        FUN_142e54290(0x9c,uVar13 & 0xffffffff);
      }
      piVar8[2] = iVar12 * 2;
      if (local_res20 != (wchar_t *)0x0) {
        FUN_1401bebb0(local_res20 + -8);
      }
    }
    else {
      if (*(int *)pwVar1 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *(int *)pwVar1 = *(int *)pwVar1 + 1;
      UNLOCK();
      pwVar5 = pwVar10;
      if (local_res20 != (wchar_t *)0x0) {
        FUN_1401bebb0(local_res20 + -8);
      }
    }
  }
  local_res20 = pwVar5;
  FUN_141b4ac80(&local_res20,&local_60,param_1 + 0x140);
LAB_141b369df:
  if (pwVar10 != (wchar_t *)0x0) {
    FUN_1401bebb0(pwVar10 + -8);
  }
  return;
}



//===========================================================
// FUN_141b38240 @ 141b38240   (733 bytes)
//===========================================================

void FUN_141b38240(longlong param_1,undefined8 param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined1 uVar4;
  undefined8 uVar5;
  wchar_t **ppwVar6;
  int *piVar7;
  wchar_t **ppwVar8;
  wchar_t *local_res8;
  wchar_t *local_res18;
  wchar_t **local_res20;
  
  uVar4 = FUN_1406e8ae0(param_2);
  switch(uVar4) {
  case 0:
    uVar5 = FUN_1403edf80(&local_res8,L"resetSecondPasswordSuccess",0xffffffff);
    FUN_141b4a840(uVar5,param_1 + 0x140);
  default:
    goto switchD_141b38281_caseD_1;
  case 6:
  case 9:
    local_res20 = &local_res18;
    local_res18 = (wchar_t *)0x0;
    local_res8 = (wchar_t *)0x0;
    piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3a);
    piVar7[1] = 0x14;
    *piVar7 = -1;
    local_res8 = (wchar_t *)(piVar7 + 4);
    piVar7[2] = 0;
    *local_res8 = L'\0';
    uVar5 = u_cannotProcessRequest_1433fdbc8._8_8_;
    *(undefined8 *)local_res8 = u_cannotProcessRequest_1433fdbc8._0_8_;
    *(undefined8 *)(piVar7 + 6) = uVar5;
    uVar3 = u_cannotProcessRequest_1433fdbc8._28_4_;
    uVar2 = u_cannotProcessRequest_1433fdbc8._24_4_;
    uVar1 = u_cannotProcessRequest_1433fdbc8._20_4_;
    piVar7[8] = u_cannotProcessRequest_1433fdbc8._16_4_;
    piVar7[9] = uVar1;
    piVar7[10] = uVar2;
    piVar7[0xb] = uVar3;
    *(undefined8 *)(piVar7 + 0xc) = u_cannotProcessRequest_1433fdbc8._32_8_;
    if (*piVar7 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar7[1] < 0x14) {
      FUN_142e54290(0x90,piVar7[1],0x14);
    }
    *piVar7 = 1;
    local_res8[0x14] = L'\0';
    if (piVar7[1] + 1 < 0x15) {
      FUN_142e54290(0x9c,0x14);
    }
    piVar7[2] = 0x28;
    ppwVar8 = &local_res18;
    ppwVar6 = &local_res8;
    break;
  case 0x14:
    local_res20 = &local_res8;
    local_res8 = (wchar_t *)0x0;
    ppwVar6 = (wchar_t **)FUN_1403edf80(&local_res18,L"incorrectPIC",0xffffffff);
    ppwVar8 = &local_res8;
    break;
  case 0x16:
    local_res20 = &local_res8;
    local_res8 = (wchar_t *)0x0;
    ppwVar6 = (wchar_t **)FUN_1403edf80(&local_res18,L"incorrectSecondPassword",0xffffffff);
    ppwVar8 = &local_res8;
    break;
  case 0x33:
    local_res20 = &local_res8;
    local_res8 = (wchar_t *)0x0;
    ppwVar6 = (wchar_t **)FUN_1403edf80(&local_res18,L"incorrectPICTryAgain",0xffffffff);
    ppwVar8 = &local_res8;
    break;
  case 0x39:
    local_res20 = &local_res8;
    local_res8 = (wchar_t *)0x0;
    ppwVar6 = (wchar_t **)FUN_1403edf80(&local_res18,L"incorrectPICWarningOverCount",0xffffffff);
    ppwVar8 = &local_res8;
    break;
  case 0x3a:
    local_res20 = &local_res8;
    local_res8 = (wchar_t *)0x0;
    uVar5 = FUN_1403edf80(&local_res18,L"incorrectPICCloseByOverCount",0xffffffff);
    FUN_141b4ac80(uVar5,&local_res8,param_1 + 0x140);
    FUN_141b2d290(param_1,0,0);
    goto switchD_141b38281_caseD_1;
  case 0x3b:
    local_res20 = &local_res8;
    local_res8 = (wchar_t *)0x0;
    ppwVar6 = (wchar_t **)FUN_1403edf80(&local_res18,L"invalidPICForm",0xffffffff);
    ppwVar8 = &local_res8;
    break;
  case 0x3c:
    local_res20 = &local_res8;
    local_res8 = (wchar_t *)0x0;
    ppwVar6 = (wchar_t **)
              FUN_1403edf80(&local_res18,L"invalidPICThreeSameCharacterInRow",0xffffffff);
    ppwVar8 = &local_res8;
  }
  FUN_141b4ac80(ppwVar6,ppwVar8,param_1 + 0x140);
switchD_141b38281_caseD_1:
  *(undefined4 *)(param_1 + 0xd4) = 0;
  return;
}



//===========================================================
// FUN_141b391e0 @ 141b391e0   (629 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b391e0(longlong param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  int *piVar5;
  undefined1 auStack_4a8 [32];
  wchar_t *local_488;
  longlong local_480;
  undefined1 local_478 [1104];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  local_488 = (wchar_t *)0x0;
  piVar5 = (int *)FUN_1401bc720(&DAT_143ad6980,0x44);
  piVar5[1] = 0x19;
  *piVar5 = -1;
  local_488 = (wchar_t *)(piVar5 + 4);
  piVar5[2] = 0;
  *local_488 = L'\0';
  uVar1 = u_goToNexonAuthPageToVerify_1433fe810._8_8_;
  *(undefined8 *)local_488 = u_goToNexonAuthPageToVerify_1433fe810._0_8_;
  *(undefined8 *)(piVar5 + 6) = uVar1;
  uVar4 = u_goToNexonAuthPageToVerify_1433fe810._28_4_;
  uVar3 = u_goToNexonAuthPageToVerify_1433fe810._24_4_;
  uVar2 = u_goToNexonAuthPageToVerify_1433fe810._20_4_;
  piVar5[8] = u_goToNexonAuthPageToVerify_1433fe810._16_4_;
  piVar5[9] = uVar2;
  piVar5[10] = uVar3;
  piVar5[0xb] = uVar4;
  uVar4 = u_goToNexonAuthPageToVerify_1433fe810._44_4_;
  uVar3 = u_goToNexonAuthPageToVerify_1433fe810._40_4_;
  uVar2 = u_goToNexonAuthPageToVerify_1433fe810._36_4_;
  piVar5[0xc] = u_goToNexonAuthPageToVerify_1433fe810._32_4_;
  piVar5[0xd] = uVar2;
  piVar5[0xe] = uVar3;
  piVar5[0xf] = uVar4;
  *(wchar_t *)(piVar5 + 0x10) = u_goToNexonAuthPageToVerify_1433fe810[0x18];
  if (*piVar5 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar5[1] < 0x19) {
    FUN_142e54290(0x90,piVar5[1],0x19);
  }
  *piVar5 = 1;
  local_488[0x19] = L'\0';
  if (piVar5[1] + 1 < 0x1a) {
    FUN_142e54290(0x9c,0x19);
  }
  piVar5[2] = 0x32;
  FUN_141b4a840(&local_488,param_1 + 0x140);
  FUN_1406e9050(param_2,&local_480);
  FUN_1429e4fa0(local_480,0,0);
  local_488 = (wchar_t *)0x0;
  piVar5 = (int *)FUN_1401bc720(&DAT_143ad6980,0x34);
  piVar5[1] = 0x11;
  *piVar5 = -1;
  local_488 = (wchar_t *)(piVar5 + 4);
  piVar5[2] = 0;
  *local_488 = L'\0';
  uVar4 = u_clickOKIfVerified_1433fe848._12_4_;
  uVar3 = u_clickOKIfVerified_1433fe848._8_4_;
  uVar2 = u_clickOKIfVerified_1433fe848._4_4_;
  *(undefined4 *)local_488 = u_clickOKIfVerified_1433fe848._0_4_;
  piVar5[5] = uVar2;
  piVar5[6] = uVar3;
  piVar5[7] = uVar4;
  uVar4 = u_clickOKIfVerified_1433fe848._28_4_;
  uVar3 = u_clickOKIfVerified_1433fe848._24_4_;
  uVar2 = u_clickOKIfVerified_1433fe848._20_4_;
  piVar5[8] = u_clickOKIfVerified_1433fe848._16_4_;
  piVar5[9] = uVar2;
  piVar5[10] = uVar3;
  piVar5[0xb] = uVar4;
  *(wchar_t *)(piVar5 + 0xc) = u_clickOKIfVerified_1433fe848[0x10];
  if (*piVar5 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar5[1] < 0x11) {
    FUN_142e54290(0x90,piVar5[1],0x11);
  }
  *piVar5 = 1;
  local_488[0x11] = L'\0';
  if (piVar5[1] + 1 < 0x12) {
    FUN_142e54290(0x9c,0x11);
  }
  piVar5[2] = 0x22;
  FUN_141b4a840(&local_488,param_1 + 0x140);
  if (*(int *)(param_1 + 0xd4) == 0) {
    FUN_1406ed520(local_478,0x7b);
    FUN_1415d01c0(local_478);
    *(undefined4 *)(param_1 + 0xb8) = 6;
    *(undefined4 *)(param_1 + 0xd4) = 1;
    FUN_1406ed610(local_478);
  }
  if (local_480 != 0) {
    FUN_14019f2c0(local_480 + -0x10);
  }
  return;
}



//===========================================================
// FUN_141b28cc0 @ 141b28cc0   (891 bytes)
//===========================================================

void FUN_141b28cc0(longlong param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  char cVar5;
  undefined1 uVar6;
  undefined1 uVar7;
  int *piVar8;
  longlong lVar9;
  longlong lVar10;
  longlong *plVar11;
  wchar_t *local_res18;
  longlong local_res20;
  longlong *local_48;
  longlong *local_40 [2];
  longlong *local_30;
  
  cVar5 = FUN_1406e8ae0(param_2);
  if (cVar5 == 'F') {
    local_48 = &local_res20;
    local_res20 = 0;
    local_res18 = (wchar_t *)0x0;
    piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3a);
    piVar8[1] = 0x14;
    *piVar8 = -1;
    local_res18 = (wchar_t *)(piVar8 + 4);
    piVar8[2] = 0;
    *local_res18 = L'\0';
    uVar1 = u_cannotProcessRequest_1433fdbc8._8_8_;
    *(undefined8 *)local_res18 = u_cannotProcessRequest_1433fdbc8._0_8_;
    *(undefined8 *)(piVar8 + 6) = uVar1;
    uVar4 = u_cannotProcessRequest_1433fdbc8._28_4_;
    uVar3 = u_cannotProcessRequest_1433fdbc8._24_4_;
    uVar2 = u_cannotProcessRequest_1433fdbc8._20_4_;
    piVar8[8] = u_cannotProcessRequest_1433fdbc8._16_4_;
    piVar8[9] = uVar2;
    piVar8[10] = uVar3;
    piVar8[0xb] = uVar4;
    *(undefined8 *)(piVar8 + 0xc) = u_cannotProcessRequest_1433fdbc8._32_8_;
    if (*piVar8 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar8[1] < 0x14) {
      FUN_142e54290(0x90,piVar8[1],0x14);
    }
    *piVar8 = 1;
    local_res18[0x14] = L'\0';
    if (piVar8[1] + 1 < 0x15) {
      FUN_142e54290(0x9c,0x14);
    }
    piVar8[2] = 0x28;
    FUN_141b4ac80(&local_res18,&local_res20,param_1 + 0x140);
    return;
  }
  cVar5 = FUN_1406e8ae0(param_2);
  FUN_1406e8ae0(param_2);
  uVar6 = FUN_1406e8ae0(param_2);
  uVar7 = FUN_1406e8ae0(param_2);
  if (cVar5 == '\0') {
    FUN_141029040(local_40,param_2);
    local_res18 = (wchar_t *)FUN_14019b780(&DAT_143ad68a0,0x340);
    lVar9 = 0;
    if (local_res18 != (wchar_t *)0x0) {
      lVar9 = FUN_14130a540(local_res18,param_1,0x3c);
    }
    lVar10 = lVar9 + 0x18;
    if (lVar9 == 0) {
      lVar10 = 0;
    }
    if (lVar10 == 0) {
      local_30 = (longlong *)0x0;
    }
    else {
      local_30 = (longlong *)(lVar10 + -0x18);
      if (local_30 != (longlong *)0x0) {
        if (0xfffff < *(ulonglong *)(lVar10 + 8)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar10 + 8) = *(longlong *)(lVar10 + 8) + 1;
        UNLOCK();
      }
    }
    plVar11 = local_30;
    if (local_30 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    local_48 = local_40[0];
    if (local_40[0] != (longlong *)0x0) {
      (**(code **)(*local_40[0] + 8))();
    }
    FUN_14130cbc0(plVar11,&local_48,uVar6,uVar7);
    if (plVar11 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    (**(code **)(*plVar11 + 0x130))(plVar11);
    if (0xffffe < plVar11[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar11 = plVar11 + 4;
    lVar9 = *plVar11;
    *plVar11 = *plVar11 + -1;
    UNLOCK();
    if (((int)lVar9 == 1) && (plVar11 = local_30 + 3, plVar11 != (longlong *)0x0)) {
      (**(code **)*plVar11)(plVar11,1);
    }
    if (local_40[0] != (longlong *)0x0) {
      (**(code **)(*local_40[0] + 0x10))();
    }
    return;
  }
  local_48 = &local_res20;
  local_res20 = 0;
  local_res18 = (wchar_t *)0x0;
  piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,0x3c);
  piVar8[1] = 0x15;
  *piVar8 = -1;
  local_res18 = (wchar_t *)(piVar8 + 4);
  piVar8[2] = 0;
  *local_res18 = L'\0';
  uVar1 = u_antimacroTextMismatch_1433fe360._8_8_;
  *(undefined8 *)local_res18 = u_antimacroTextMismatch_1433fe360._0_8_;
  *(undefined8 *)(piVar8 + 6) = uVar1;
  uVar4 = u_antimacroTextMismatch_1433fe360._28_4_;
  uVar3 = u_antimacroTextMismatch_1433fe360._24_4_;
  uVar2 = u_antimacroTextMismatch_1433fe360._20_4_;
  piVar8[8] = u_antimacroTextMismatch_1433fe360._16_4_;
  piVar8[9] = uVar2;
  piVar8[10] = uVar3;
  piVar8[0xb] = uVar4;
  *(undefined8 *)(piVar8 + 0xc) = u_antimacroTextMismatch_1433fe360._32_8_;
  *(wchar_t *)(piVar8 + 0xe) = u_antimacroTextMismatch_1433fe360[0x14];
  if (*piVar8 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar8[1] < 0x15) {
    FUN_142e54290(0x90,piVar8[1],0x15);
  }
  *piVar8 = 1;
  local_res18[0x15] = L'\0';
  if (piVar8[1] + 1 < 0x16) {
    FUN_142e54290(0x9c,0x15);
  }
  piVar8[2] = 0x2a;
  FUN_141b4ac80(&local_res18,&local_res20,param_1 + 0x140);
  return;
}



//===========================================================
// FUN_141b3a180 @ 141b3a180   (40 bytes)
//===========================================================

/* WARNING: Control flow encountered bad instruction data */

void FUN_141b3a180(void)

{
                    /* WARNING: Bad instruction - Truncating control flow here */
  halt_baddata();
}



//===========================================================
// FUN_141b3a2f0 @ 141b3a2f0   (528 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141b3a2f0(longlong param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  char cVar5;
  int iVar6;
  int *piVar7;
  longlong *plVar8;
  undefined1 auStack_498 [32];
  wchar_t *local_478;
  longlong local_470;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  cVar5 = FUN_1406e8ae0(param_2);
  *(undefined4 *)(param_1 + 0xd4) = 0;
  if (cVar5 == '\0') {
    *(undefined4 *)(param_1 + 0xec) = 0xbb9;
    FUN_141b3a560(param_1);
  }
  else {
    FUN_1406e9050(param_2,&local_470);
    FUN_1429e5410(local_470,0,0);
    local_478 = (wchar_t *)0x0;
    piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,0x46);
    piVar7[1] = 0x1a;
    *piVar7 = -1;
    local_478 = (wchar_t *)(piVar7 + 4);
    piVar7[2] = 0;
    *local_478 = L'\0';
    uVar1 = u_confirmPlayerAuthCompleted_1433feb10._8_8_;
    *(undefined8 *)local_478 = u_confirmPlayerAuthCompleted_1433feb10._0_8_;
    *(undefined8 *)(piVar7 + 6) = uVar1;
    uVar4 = u_confirmPlayerAuthCompleted_1433feb10._28_4_;
    uVar3 = u_confirmPlayerAuthCompleted_1433feb10._24_4_;
    uVar2 = u_confirmPlayerAuthCompleted_1433feb10._20_4_;
    piVar7[8] = u_confirmPlayerAuthCompleted_1433feb10._16_4_;
    piVar7[9] = uVar2;
    piVar7[10] = uVar3;
    piVar7[0xb] = uVar4;
    uVar4 = u_confirmPlayerAuthCompleted_1433feb10._44_4_;
    uVar3 = u_confirmPlayerAuthCompleted_1433feb10._40_4_;
    uVar2 = u_confirmPlayerAuthCompleted_1433feb10._36_4_;
    piVar7[0xc] = u_confirmPlayerAuthCompleted_1433feb10._32_4_;
    piVar7[0xd] = uVar2;
    piVar7[0xe] = uVar3;
    piVar7[0xf] = uVar4;
    piVar7[0x10] = u_confirmPlayerAuthCompleted_1433feb10._48_4_;
    if (*piVar7 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar7[1] < 0x1a) {
      FUN_142e54290(0x90,piVar7[1],0x1a);
    }
    *piVar7 = 1;
    local_478[0x1a] = L'\0';
    if (piVar7[1] + 1 < 0x1b) {
      FUN_142e54290(0x9c,0x1a);
    }
    piVar7[2] = 0x34;
    iVar6 = FUN_141b4c230(&local_478,0);
    plVar8 = *(longlong **)(param_1 + 0x148);
    if (plVar8 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
      plVar8 = *(longlong **)(param_1 + 0x148);
    }
    (**(code **)(*plVar8 + 0x138))(plVar8,3000);
    FUN_1406ed520(local_468,0xb3);
    if (iVar6 == 1) {
      FUN_1406ed840(local_468,1);
      FUN_1415d01c0(local_468);
      *(undefined4 *)(param_1 + 0xd4) = 1;
    }
    else {
      FUN_1406ed840(local_468,0);
      FUN_1415d01c0(local_468);
      *(undefined4 *)(param_1 + 0xd4) = 1;
    }
    FUN_1406ed610(local_468);
    if (local_470 != 0) {
      FUN_14019f2c0(local_470 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_141b29170 @ 141b29170   (1036 bytes)
//===========================================================

void FUN_141b29170(longlong param_1,undefined8 param_2)

{
  undefined1 *puVar1;
  int iVar2;
  undefined4 uVar3;
  uint uVar4;
  longlong lVar5;
  longlong lVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  longlong *plVar9;
  ulonglong uVar10;
  longlong local_res8;
  longlong local_res18;
  longlong local_res20;
  longlong local_78;
  longlong local_70;
  longlong local_68;
  longlong local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined1 local_48 [8];
  longlong local_40;
  
  iVar2 = FUN_1406e8c20(param_2);
  if (iVar2 == *(int *)(param_1 + 0x1d8)) {
    uVar3 = FUN_1406e8c20(param_2);
    local_res8 = CONCAT44(local_res8._4_4_,uVar3);
    *(int *)(param_1 + 0x1dc) = *(int *)(param_1 + 0x1dc) + 1;
    uVar4 = FUN_1406e8c20(param_2);
    if (0 < (int)uVar4) {
      uVar10 = (ulonglong)uVar4;
      do {
        FUN_1406e9050(param_2,&local_78);
        FUN_1406e9050(param_2,&local_res20);
        FUN_1406e9050(param_2,&local_res18);
        FUN_1406e9170(param_2,&local_58,8);
        uVar3 = FUN_1406e8c20(param_2);
        lVar5 = FUN_141b44680(param_1 + 0x1d0,0xffffffff);
        FUN_14019a260(lVar5,&local_78);
        FUN_14019a260(lVar5 + 8,&local_res20);
        FUN_14019a260(lVar5 + 0x10,&local_res18);
        *(undefined8 *)(lVar5 + 0x18) = local_58;
        *(undefined1 *)(lVar5 + 0x20) = 0;
        *(undefined4 *)(lVar5 + 0x24) = uVar3;
        if (local_res18 != 0) {
          FUN_14019f2c0(local_res18 + -0x10);
        }
        if (local_res20 != 0) {
          FUN_14019f2c0(local_res20 + -0x10);
        }
        if (local_78 != 0) {
          FUN_14019f2c0(local_78 + -0x10);
        }
        uVar10 = uVar10 - 1;
      } while (uVar10 != 0);
    }
    uVar4 = FUN_1406e8c20(param_2);
    if (0 < (int)uVar4) {
      uVar10 = (ulonglong)uVar4;
      do {
        FUN_1406e9050(param_2,&local_60);
        FUN_1406e9050(param_2,&local_68);
        FUN_1406e9050(param_2,&local_70);
        FUN_1406e9170(param_2,&local_50,8);
        lVar5 = FUN_141b44680(param_1 + 0x1d0,0xffffffff);
        FUN_14019a260(lVar5,&local_60);
        FUN_14019a260(lVar5 + 8,&local_68);
        FUN_14019a260(lVar5 + 0x10,&local_70);
        *(undefined8 *)(lVar5 + 0x18) = local_50;
        *(undefined1 *)(lVar5 + 0x20) = 1;
        *(undefined4 *)(lVar5 + 0x24) = 0;
        if (local_70 != 0) {
          FUN_14019f2c0(local_70 + -0x10);
        }
        if (local_68 != 0) {
          FUN_14019f2c0(local_68 + -0x10);
        }
        if (local_60 != 0) {
          FUN_14019f2c0(local_60 + -0x10);
        }
        uVar10 = uVar10 - 1;
      } while (uVar10 != 0);
    }
    if ((int)local_res8 == *(int *)(param_1 + 0x1dc)) {
      if (*(longlong *)(param_1 + 0x1e8) == 0) {
        if ((*(longlong *)(param_1 + 0x1d0) == 0) ||
           (*(int *)(*(longlong *)(param_1 + 0x1d0) + -8) == 0)) {
          uVar7 = FUN_1408a9e40(&local_res8,0x124e);
          FUN_142a26280(uVar7,0,param_1 + 0x140,1,0,0,0,0,0,0);
        }
        else {
          local_res8 = FUN_14019b780(&DAT_143ad68a0,0x2e8);
          lVar5 = 0;
          if (local_res8 != 0) {
            lVar5 = FUN_141b55c60(local_res8,param_1);
          }
          lVar6 = lVar5 + 0x18;
          if (lVar5 == 0) {
            lVar6 = 0;
          }
          if (lVar6 == 0) {
            local_40 = 0;
          }
          else {
            local_40 = lVar6 + -0x18;
            if (local_40 != 0) {
              if (0xfffff < *(ulonglong *)(lVar6 + 8)) {
                FUN_142e541f0(0x30f);
              }
              LOCK();
              *(longlong *)(lVar6 + 8) = *(longlong *)(lVar6 + 8) + 1;
              UNLOCK();
            }
          }
          lVar5 = local_40;
          puVar1 = (undefined1 *)(param_1 + 0x1e0);
          if ((*(longlong *)(param_1 + 0x1e8) - 1U < 999) || (*(longlong *)(param_1 + 0x1e8) == -1))
          {
            FUN_142e52ed0(0x447);
          }
          if (puVar1 == local_48) {
            FUN_142e52d50(0x45c,1);
          }
          if (lVar5 != 0) {
            if (0xfffff < *(ulonglong *)(lVar5 + 0x20)) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            *(longlong *)(lVar5 + 0x20) = *(longlong *)(lVar5 + 0x20) + 1;
            UNLOCK();
            lVar5 = local_40;
          }
          FUN_141b45630(puVar1);
          *(longlong *)(param_1 + 0x1e8) = lVar5;
          if (lVar5 != 0) {
            if (0xffffe < *(longlong *)(lVar5 + 0x20) - 1U) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            plVar9 = (longlong *)(lVar5 + 0x20);
            lVar5 = *plVar9;
            *plVar9 = *plVar9 + -1;
            UNLOCK();
            if (((int)lVar5 == 1) &&
               (puVar8 = (undefined8 *)(local_40 + 0x18), puVar8 != (undefined8 *)0x0)) {
              (**(code **)*puVar8)(puVar8,1);
            }
          }
          if (*(longlong *)(param_1 + 0x1e8) != 0) {
            FUN_141b57d70(*(longlong *)(param_1 + 0x1e8),param_1 + 0x1d0);
            plVar9 = *(longlong **)(param_1 + 0x1e8);
            if (plVar9 == (longlong *)0x0) {
              FUN_142e52ed0(0x431,0);
              plVar9 = *(longlong **)(param_1 + 0x1e8);
            }
            (**(code **)(*plVar9 + 0x130))();
            FUN_141b45630(puVar1);
          }
        }
      }
      else {
        FUN_141b57d70(*(longlong *)(param_1 + 0x1e8),param_1 + 0x1d0);
      }
    }
  }
  return;
}


