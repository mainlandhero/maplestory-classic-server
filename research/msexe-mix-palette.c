
//===========================================================
// FUN_142a92890 @ 142a92890   (1356 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000142a92b9f) */

void FUN_142a92890(longlong *param_1,uint param_2,longlong *param_3)

{
  undefined8 uVar1;
  int *piVar2;
  int iVar3;
  bool bVar4;
  bool bVar5;
  bool bVar6;
  bool bVar7;
  int *piVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined8 *puVar13;
  int *piVar14;
  longlong *plVar15;
  longlong lVar16;
  int *piVar17;
  uint uVar18;
  int iVar19;
  int iVar20;
  ulonglong uVar21;
  longlong lVar22;
  int *local_res20;
  uint in_stack_ffffffffffffff38;
  undefined1 local_a8 [8];
  longlong *local_a0;
  int *local_98;
  undefined1 local_90 [8];
  longlong local_88;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  undefined8 local_68;
  int **local_60;
  int *local_58;
  
  lVar22 = param_3[1];
  lVar16 = *param_3;
  if (lVar16 != lVar22) {
    do {
      FUN_140cbb020(lVar16);
      lVar16 = lVar16 + 0x10;
    } while (lVar16 != lVar22);
    lVar16 = *param_3;
  }
  param_3[1] = lVar16;
  iVar3 = (param_2 + 900) * 10;
  uVar1 = *(undefined8 *)(*param_1 + 0x6c8);
  lVar22 = -1;
  do {
    lVar22 = lVar22 + 1;
  } while (L"/BtColor"[lVar22] != L'\0');
  FUN_14040ea40(param_1[1],&local_80);
  bVar7 = false;
  bVar4 = false;
  bVar6 = false;
  bVar5 = false;
  uVar18 = param_2 & 0x80000001;
  if ((int)uVar18 < 0) {
    uVar18 = (uVar18 - 1 | 0xfffffffe) + 1;
  }
  FUN_141ac3370(uVar1,local_80,iVar3,uVar18 * 0x133,((int)param_2 / 2) * 0x5f,1,
                in_stack_ffffffffffffff38 & 0xffffff00,0);
  if (local_80 != 0) {
    FUN_1401bebb0(local_80 + -0x10);
  }
  uVar18 = 0;
  if (-1 < *(int *)param_1[2]) {
    do {
      FUN_141ad46a0(*(undefined8 *)(*param_1 + 0x6c8),local_a8,uVar18 + iVar3);
      plVar15 = local_a0;
      lVar22 = param_3[1];
      if (lVar22 == param_3[2]) {
        FUN_141473950(param_3,lVar22,local_a8);
      }
      else {
        *(longlong **)(lVar22 + 8) = local_a0;
        if (local_a0 != (longlong *)0x0) {
          if (0xfffff < (ulonglong)local_a0[4]) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          plVar15[4] = plVar15[4] + 1;
          UNLOCK();
          bVar4 = bVar5;
          bVar7 = bVar6;
        }
        param_3[1] = param_3[1] + 0x10;
      }
      if (local_a0 != (longlong *)0x0) {
        if (*(char *)param_1[3] == '\0') {
          puVar13 = (undefined8 *)FUN_1401a8a90(&local_78,uVar18 & 0xff);
          bVar4 = true;
          bVar6 = bVar7;
        }
        else {
          puVar13 = (undefined8 *)FUN_1401a88a0(&local_70);
          bVar6 = true;
        }
        piVar2 = (int *)*puVar13;
        *puVar13 = 0;
        local_98 = piVar2;
        if ((bVar4) && (bVar4 = false, local_78 != 0)) {
          FUN_14019f2c0(local_78 + -0x10);
        }
        if ((bVar6) && (bVar6 = false, local_70 != 0)) {
          FUN_14019f2c0(local_70 + -0x10);
        }
        local_68 = *(undefined8 *)(*param_1 + 0x6c8);
        local_60 = &local_res20;
        local_res20 = (int *)0x0;
        piVar17 = piVar2;
        piVar8 = local_res20;
        if ((piVar2 != (int *)0x0) && (piVar14 = piVar2 + -4, piVar14 != (int *)0x0)) {
          if (*piVar14 == -1) {
            FUN_142e52d50(0xcb,0xffffff01);
            uVar21 = 0xffffffffffffffff;
            do {
              uVar21 = uVar21 + 1;
            } while (*(char *)((longlong)piVar2 + uVar21) != '\0');
            iVar19 = (int)uVar21;
            iVar20 = 0;
            if (0 < iVar19) {
              iVar20 = iVar19;
            }
            piVar14 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar20 + 0x11));
            piVar14[1] = iVar20;
            *piVar14 = -1;
            piVar8 = piVar14 + 4;
            piVar14[2] = 0;
            *(undefined1 *)piVar8 = 0;
            local_58 = piVar8;
            FUN_142ef7ba0(piVar8,piVar2,(longlong)iVar19);
            if (*piVar14 != -1) {
              FUN_142e52dd0(0x8b);
            }
            if ((iVar19 == -1) || (iVar19 <= piVar14[1])) {
              *piVar14 = 1;
              if (iVar19 != -1) goto LAB_142a92b5e;
              if (piVar8 == (int *)0x0) {
                uVar21 = 0;
              }
              else {
                uVar21 = 0xffffffffffffffff;
                do {
                  uVar21 = uVar21 + 1;
                } while (*(char *)((longlong)piVar8 + uVar21) != '\0');
              }
            }
            else {
              FUN_142e54290(0x90,piVar14[1],uVar21 & 0xffffffff);
              *piVar14 = 1;
LAB_142a92b5e:
              *(undefined1 *)((longlong)iVar19 + (longlong)piVar8) = 0;
            }
            iVar20 = (int)uVar21;
            if ((iVar20 < 0) || (piVar14[1] + 1 <= iVar20)) {
              FUN_142e54290(0x9c,uVar21 & 0xffffffff);
            }
            piVar14[2] = iVar20;
            if (local_res20 != (int *)0x0) {
              FUN_14019f2c0();
            }
          }
          else {
            if (*piVar14 < 1) {
              FUN_142e52dd0(0xd2);
            }
            LOCK();
            *piVar14 = *piVar14 + 1;
            UNLOCK();
            piVar17 = local_98;
            piVar8 = piVar2;
            if (local_res20 != (int *)0x0) {
              FUN_14019f2c0(local_res20 + -4);
              piVar17 = local_98;
            }
          }
        }
        local_res20 = piVar8;
        if (local_a0 == (longlong *)0x0) {
          FUN_142e52ed0(0x431,0);
        }
        uVar9 = FUN_141711e10(local_a0);
        if (local_a0 == (longlong *)0x0) {
          FUN_142e52ed0(0x431,0);
        }
        uVar10 = FUN_141711e00(local_a0);
        if (local_a0 == (longlong *)0x0) {
          FUN_142e52ed0(0x431,0);
        }
        uVar11 = (**(code **)(*local_a0 + 0x68))();
        if (local_a0 == (longlong *)0x0) {
          FUN_142e52ed0(0x431,0);
        }
        uVar12 = (**(code **)(*local_a0 + 0x60))();
        FUN_141aaae90(local_68,local_90,iVar3 + 10000 + uVar18,uVar12,uVar11,uVar10,uVar9,
                      &local_res20,2);
        if (local_88 == 0) {
          FUN_142e52ed0(0x431,0);
        }
        lVar22 = local_88;
        *(undefined4 *)(local_88 + 0x11cc) = 0;
        if (local_88 != 0) {
          if (0xffffe < *(longlong *)(local_88 + 0x20) - 1U) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar15 = (longlong *)(lVar22 + 0x20);
          lVar22 = *plVar15;
          *plVar15 = *plVar15 + -1;
          UNLOCK();
          if ((int)lVar22 == 1) {
            puVar13 = (undefined8 *)(local_88 + 0x18);
            if (local_88 == 0) {
              puVar13 = (undefined8 *)0x0;
            }
            if (puVar13 != (undefined8 *)0x0) {
              (**(code **)*puVar13)(puVar13,1);
            }
          }
          local_88 = 0;
          piVar17 = local_98;
        }
        bVar5 = bVar4;
        bVar7 = bVar6;
        if (piVar17 != (int *)0x0) {
          FUN_14019f2c0(piVar17 + -4);
        }
      }
      plVar15 = local_a0;
      if (local_a0 != (longlong *)0x0) {
        if (0xffffe < local_a0[4] - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar15 = plVar15 + 4;
        lVar22 = *plVar15;
        *plVar15 = *plVar15 + -1;
        UNLOCK();
        if ((int)lVar22 == 1) {
          plVar15 = local_a0 + 3;
          if (local_a0 == (longlong *)0x0) {
            plVar15 = (longlong *)0x0;
          }
          if (plVar15 != (longlong *)0x0) {
            (**(code **)*plVar15)(plVar15,1);
          }
        }
        local_a0 = (longlong *)0x0;
        bVar4 = bVar5;
        bVar7 = bVar6;
      }
      uVar18 = uVar18 + 1;
    } while ((int)uVar18 <= *(int *)param_1[2]);
  }
  return;
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
// FUN_142ef7ba0 @ 142ef7ba0   (1379 bytes)
//===========================================================

undefined8 * FUN_142ef7ba0(undefined8 *param_1,undefined8 *param_2,ulonglong param_3)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined1 auVar3 [32];
  undefined1 auVar4 [32];
  undefined1 auVar5 [32];
  undefined1 auVar6 [32];
  undefined1 uVar7;
  undefined2 uVar8;
  undefined4 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  undefined8 uVar22;
  undefined8 *puVar23;
  undefined1 (*pauVar24) [32];
  undefined1 (*pauVar25) [32];
  undefined8 *puVar26;
  undefined1 (*pauVar27) [32];
  undefined1 (*pauVar28) [32];
  ulonglong uVar29;
  longlong lVar30;
  ulonglong uVar31;
  undefined8 uVar32;
  undefined8 uVar33;
  
  puVar23 = param_1;
  switch(param_3) {
  case 0:
    return puVar23;
  case 1:
    *(undefined1 *)param_1 = *(undefined1 *)param_2;
    return puVar23;
  case 2:
    *(undefined2 *)param_1 = *(undefined2 *)param_2;
    return puVar23;
  case 3:
    uVar7 = *(undefined1 *)((longlong)param_2 + 2);
    *(undefined2 *)param_1 = *(undefined2 *)param_2;
    *(undefined1 *)((longlong)param_1 + 2) = uVar7;
    return puVar23;
  case 4:
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    return puVar23;
  case 5:
    uVar7 = *(undefined1 *)((longlong)param_2 + 4);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined1 *)((longlong)param_1 + 4) = uVar7;
    return puVar23;
  case 6:
    uVar8 = *(undefined2 *)((longlong)param_2 + 4);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined2 *)((longlong)param_1 + 4) = uVar8;
    return puVar23;
  case 7:
    uVar8 = *(undefined2 *)((longlong)param_2 + 4);
    uVar7 = *(undefined1 *)((longlong)param_2 + 6);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined2 *)((longlong)param_1 + 4) = uVar8;
    *(undefined1 *)((longlong)param_1 + 6) = uVar7;
    return puVar23;
  case 8:
    *param_1 = *param_2;
    return puVar23;
  case 9:
    uVar7 = *(undefined1 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined1 *)(param_1 + 1) = uVar7;
    return puVar23;
  case 10:
    uVar8 = *(undefined2 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined2 *)(param_1 + 1) = uVar8;
    return puVar23;
  case 0xb:
    uVar8 = *(undefined2 *)(param_2 + 1);
    uVar7 = *(undefined1 *)((longlong)param_2 + 10);
    *param_1 = *param_2;
    *(undefined2 *)(param_1 + 1) = uVar8;
    *(undefined1 *)((longlong)param_1 + 10) = uVar7;
    return puVar23;
  case 0xc:
    uVar9 = *(undefined4 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    return puVar23;
  case 0xd:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar7 = *(undefined1 *)((longlong)param_2 + 0xc);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined1 *)((longlong)param_1 + 0xc) = uVar7;
    return puVar23;
  case 0xe:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar8 = *(undefined2 *)((longlong)param_2 + 0xc);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined2 *)((longlong)param_1 + 0xc) = uVar8;
    return puVar23;
  case 0xf:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar8 = *(undefined2 *)((longlong)param_2 + 0xc);
    uVar7 = *(undefined1 *)((longlong)param_2 + 0xe);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined2 *)((longlong)param_1 + 0xc) = uVar8;
    *(undefined1 *)((longlong)param_1 + 0xe) = uVar7;
    return puVar23;
  }
  if (param_3 < 0x21) {
    uVar10 = param_2[1];
    puVar26 = (undefined8 *)((longlong)param_2 + (param_3 - 0x10));
    uVar11 = *puVar26;
    uVar32 = puVar26[1];
    *param_1 = *param_2;
    param_1[1] = uVar10;
    param_1 = (undefined8 *)((longlong)param_1 + (param_3 - 0x10));
    *param_1 = uVar11;
    param_1[1] = uVar32;
    return puVar23;
  }
  if ((param_2 < param_1) && (param_1 < (undefined8 *)((longlong)param_2 + param_3))) {
    lVar30 = (longlong)param_2 - (longlong)param_1;
    puVar23 = (undefined8 *)((longlong)param_1 + lVar30 + (param_3 - 0x10));
    uVar10 = *puVar23;
    uVar11 = puVar23[1];
    puVar26 = (undefined8 *)((longlong)param_1 + (param_3 - 0x10));
    uVar29 = param_3 - 0x10;
    puVar23 = puVar26;
    uVar32 = uVar10;
    uVar33 = uVar11;
    if (((ulonglong)puVar26 & 0xf) != 0) {
      puVar23 = (undefined8 *)((ulonglong)puVar26 & 0xfffffffffffffff0);
      uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
      uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
      *puVar26 = uVar10;
      *(undefined8 *)((longlong)param_1 + (param_3 - 8)) = uVar11;
      uVar29 = (longlong)puVar23 - (longlong)param_1;
    }
    uVar31 = uVar29 >> 7;
    if (uVar31 != 0) {
      *puVar23 = uVar32;
      puVar23[1] = uVar33;
      puVar26 = puVar23;
      while( true ) {
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x10);
        uVar10 = puVar1[1];
        puVar23 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x20);
        uVar11 = *puVar23;
        uVar32 = puVar23[1];
        puVar23 = puVar26 + -0x10;
        puVar26[-2] = *puVar1;
        puVar26[-1] = uVar10;
        puVar26[-4] = uVar11;
        puVar26[-3] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x30);
        uVar10 = puVar1[1];
        puVar2 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x40);
        uVar11 = *puVar2;
        uVar32 = puVar2[1];
        uVar31 = uVar31 - 1;
        puVar26[-6] = *puVar1;
        puVar26[-5] = uVar10;
        puVar26[-8] = uVar11;
        puVar26[-7] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x50);
        uVar10 = puVar1[1];
        puVar2 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x60);
        uVar11 = *puVar2;
        uVar32 = puVar2[1];
        puVar26[-10] = *puVar1;
        puVar26[-9] = uVar10;
        puVar26[-0xc] = uVar11;
        puVar26[-0xb] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x70);
        uVar10 = *puVar1;
        uVar11 = puVar1[1];
        uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
        uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
        if (uVar31 == 0) break;
        puVar26[-0xe] = uVar10;
        puVar26[-0xd] = uVar11;
        *puVar23 = uVar32;
        puVar26[-0xf] = uVar33;
        puVar26 = puVar23;
      }
      puVar26[-0xe] = uVar10;
      puVar26[-0xd] = uVar11;
      uVar29 = uVar29 & 0x7f;
    }
    for (uVar31 = uVar29 >> 4; uVar31 != 0; uVar31 = uVar31 - 1) {
      *puVar23 = uVar32;
      puVar23[1] = uVar33;
      puVar23 = puVar23 + -2;
      uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
      uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
    }
    if ((uVar29 & 0xf) != 0) {
      uVar10 = param_2[1];
      *param_1 = *param_2;
      param_1[1] = uVar10;
    }
    *puVar23 = uVar32;
    puVar23[1] = uVar33;
    return param_1;
  }
  if (DAT_143a8b918 < 3) {
    if ((param_3 < 0x801) || (((byte)DAT_143ae2c20 & 2) == 0)) {
      if (0x80 < param_3) {
        lVar30 = ((ulonglong)param_1 & 0xf) - 0x10;
        param_1 = (undefined8 *)((longlong)param_1 - lVar30);
        param_2 = (undefined8 *)((longlong)param_2 - lVar30);
        param_3 = param_3 + lVar30;
        if (0x80 < param_3) {
          do {
            uVar10 = param_2[1];
            uVar11 = param_2[2];
            uVar32 = param_2[3];
            uVar33 = param_2[4];
            uVar12 = param_2[5];
            uVar13 = param_2[6];
            uVar14 = param_2[7];
            *param_1 = *param_2;
            param_1[1] = uVar10;
            param_1[2] = uVar11;
            param_1[3] = uVar32;
            param_1[4] = uVar33;
            param_1[5] = uVar12;
            param_1[6] = uVar13;
            param_1[7] = uVar14;
            uVar10 = param_2[9];
            uVar11 = param_2[10];
            uVar32 = param_2[0xb];
            uVar33 = param_2[0xc];
            uVar12 = param_2[0xd];
            uVar13 = param_2[0xe];
            uVar14 = param_2[0xf];
            param_1[8] = param_2[8];
            param_1[9] = uVar10;
            param_1[10] = uVar11;
            param_1[0xb] = uVar32;
            param_1[0xc] = uVar33;
            param_1[0xd] = uVar12;
            param_1[0xe] = uVar13;
            param_1[0xf] = uVar14;
            param_1 = param_1 + 0x10;
            param_2 = param_2 + 0x10;
            param_3 = param_3 - 0x80;
          } while (0x7f < param_3);
        }
      }
                    /* WARNING: Could not recover jumptable at 0x000142ef80b6. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      puVar23 = (undefined8 *)
                (*(code *)(IMAGE_DOS_HEADER_140000000.e_magic +
                          *(uint *)(&DAT_143c47088 + (param_3 + 0xf >> 4) * 4)))();
      return puVar23;
    }
  }
  else if (((param_3 < 0x2001) || (0x180000 < param_3)) || (((byte)DAT_143ae2c20 & 2) == 0)) {
    uVar10 = *param_2;
    uVar11 = param_2[1];
    uVar32 = param_2[2];
    uVar33 = param_2[3];
    puVar26 = (undefined8 *)((longlong)param_2 + (param_3 - 0x20));
    uVar12 = *puVar26;
    uVar13 = puVar26[1];
    uVar14 = puVar26[2];
    uVar15 = puVar26[3];
    if (0x100 < param_3) {
      lVar30 = ((ulonglong)param_1 & 0x1f) - 0x20;
      pauVar24 = (undefined1 (*) [32])((longlong)param_1 - lVar30);
      pauVar27 = (undefined1 (*) [32])((longlong)param_2 - lVar30);
      param_3 = param_3 + lVar30;
      if (0x100 < param_3) {
        if (0x180000 < param_3) {
          do {
            uVar29 = param_3;
            pauVar28 = pauVar27;
            pauVar25 = pauVar24;
            auVar3 = pauVar28[1];
            auVar4 = pauVar28[2];
            auVar5 = pauVar28[3];
            auVar6 = vmovntdq_avx(*pauVar28);
            *pauVar25 = auVar6;
            auVar3 = vmovntdq_avx(auVar3);
            pauVar25[1] = auVar3;
            auVar3 = vmovntdq_avx(auVar4);
            pauVar25[2] = auVar3;
            auVar3 = vmovntdq_avx(auVar5);
            pauVar25[3] = auVar3;
            auVar3 = pauVar28[5];
            auVar4 = pauVar28[6];
            auVar5 = pauVar28[7];
            auVar6 = vmovntdq_avx(pauVar28[4]);
            pauVar25[4] = auVar6;
            auVar3 = vmovntdq_avx(auVar3);
            pauVar25[5] = auVar3;
            auVar3 = vmovntdq_avx(auVar4);
            pauVar25[6] = auVar3;
            auVar3 = vmovntdq_avx(auVar5);
            pauVar25[7] = auVar3;
            pauVar24 = pauVar25 + 8;
            pauVar27 = pauVar28 + 8;
            param_3 = uVar29 - 0x100;
          } while (0xff < uVar29 - 0x100);
          uVar31 = uVar29 - 0xe1 & 0xffffffffffffffe0;
          switch(uVar29) {
          case 0x1e1:
          case 0x1e2:
          case 0x1e3:
          case 0x1e4:
          case 0x1e5:
          case 0x1e6:
          case 0x1e7:
          case 0x1e8:
          case 0x1e9:
          case 0x1ea:
          case 0x1eb:
          case 0x1ec:
          case 0x1ed:
          case 0x1ee:
          case 0x1ef:
          case 0x1f0:
          case 0x1f1:
          case 0x1f2:
          case 499:
          case 500:
          case 0x1f5:
          case 0x1f6:
          case 0x1f7:
          case 0x1f8:
          case 0x1f9:
          case 0x1fa:
          case 0x1fb:
          case 0x1fc:
          case 0x1fd:
          case 0x1fe:
          case 0x1ff:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(*pauVar28 + uVar31));
            *(undefined1 (*) [32])(*pauVar25 + uVar31) = auVar3;
          case 0x1c1:
          case 0x1c2:
          case 0x1c3:
          case 0x1c4:
          case 0x1c5:
          case 0x1c6:
          case 0x1c7:
          case 0x1c8:
          case 0x1c9:
          case 0x1ca:
          case 0x1cb:
          case 0x1cc:
          case 0x1cd:
          case 0x1ce:
          case 0x1cf:
          case 0x1d0:
          case 0x1d1:
          case 0x1d2:
          case 0x1d3:
          case 0x1d4:
          case 0x1d5:
          case 0x1d6:
          case 0x1d7:
          case 0x1d8:
          case 0x1d9:
          case 0x1da:
          case 0x1db:
          case 0x1dc:
          case 0x1dd:
          case 0x1de:
          case 0x1df:
          case 0x1e0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[1] + uVar31));
            *(undefined1 (*) [32])(pauVar25[1] + uVar31) = auVar3;
          case 0x1a1:
          case 0x1a2:
          case 0x1a3:
          case 0x1a4:
          case 0x1a5:
          case 0x1a6:
          case 0x1a7:
          case 0x1a8:
          case 0x1a9:
          case 0x1aa:
          case 0x1ab:
          case 0x1ac:
          case 0x1ad:
          case 0x1ae:
          case 0x1af:
          case 0x1b0:
          case 0x1b1:
          case 0x1b2:
          case 0x1b3:
          case 0x1b4:
          case 0x1b5:
          case 0x1b6:
          case 0x1b7:
          case 0x1b8:
          case 0x1b9:
          case 0x1ba:
          case 0x1bb:
          case 0x1bc:
          case 0x1bd:
          case 0x1be:
          case 0x1bf:
          case 0x1c0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[2] + uVar31));
            *(undefined1 (*) [32])(pauVar25[2] + uVar31) = auVar3;
          case 0x181:
          case 0x182:
          case 0x183:
          case 0x184:
          case 0x185:
          case 0x186:
          case 0x187:
          case 0x188:
          case 0x189:
          case 0x18a:
          case 0x18b:
          case 0x18c:
          case 0x18d:
          case 0x18e:
          case 399:
          case 400:
          case 0x191:
          case 0x192:
          case 0x193:
          case 0x194:
          case 0x195:
          case 0x196:
          case 0x197:
          case 0x198:
          case 0x199:
          case 0x19a:
          case 0x19b:
          case 0x19c:
          case 0x19d:
          case 0x19e:
          case 0x19f:
          case 0x1a0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[3] + uVar31));
            *(undefined1 (*) [32])(pauVar25[3] + uVar31) = auVar3;
          case 0x161:
          case 0x162:
          case 0x163:
          case 0x164:
          case 0x165:
          case 0x166:
          case 0x167:
          case 0x168:
          case 0x169:
          case 0x16a:
          case 0x16b:
          case 0x16c:
          case 0x16d:
          case 0x16e:
          case 0x16f:
          case 0x170:
          case 0x171:
          case 0x172:
          case 0x173:
          case 0x174:
          case 0x175:
          case 0x176:
          case 0x177:
          case 0x178:
          case 0x179:
          case 0x17a:
          case 0x17b:
          case 0x17c:
          case 0x17d:
          case 0x17e:
          case 0x17f:
          case 0x180:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[4] + uVar31));
            *(undefined1 (*) [32])(pauVar25[4] + uVar31) = auVar3;
          case 0x141:
          case 0x142:
          case 0x143:
          case 0x144:
          case 0x145:
          case 0x146:
          case 0x147:
          case 0x148:
          case 0x149:
          case 0x14a:
          case 0x14b:
          case 0x14c:
          case 0x14d:
          case 0x14e:
          case 0x14f:
          case 0x150:
          case 0x151:
          case 0x152:
          case 0x153:
          case 0x154:
          case 0x155:
          case 0x156:
          case 0x157:
          case 0x158:
          case 0x159:
          case 0x15a:
          case 0x15b:
          case 0x15c:
          case 0x15d:
          case 0x15e:
          case 0x15f:
          case 0x160:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[5] + uVar31));
            *(undefined1 (*) [32])(pauVar25[5] + uVar31) = auVar3;
          case 0x121:
          case 0x122:
          case 0x123:
          case 0x124:
          case 0x125:
          case 0x126:
          case 0x127:
          case 0x128:
          case 0x129:
          case 0x12a:
          case 299:
          case 300:
          case 0x12d:
          case 0x12e:
          case 0x12f:
          case 0x130:
          case 0x131:
          case 0x132:
          case 0x133:
          case 0x134:
          case 0x135:
          case 0x136:
          case 0x137:
          case 0x138:
          case 0x139:
          case 0x13a:
          case 0x13b:
          case 0x13c:
          case 0x13d:
          case 0x13e:
          case 0x13f:
          case 0x140:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[6] + uVar31));
            *(undefined1 (*) [32])(pauVar25[6] + uVar31) = auVar3;
          default:
            puVar26 = (undefined8 *)(pauVar25[-1] + uVar29);
            *puVar26 = uVar12;
            puVar26[1] = uVar13;
            puVar26[2] = uVar14;
            puVar26[3] = uVar15;
          case 0x100:
            *param_1 = uVar10;
            param_1[1] = uVar11;
            param_1[2] = uVar32;
            param_1[3] = uVar33;
            return puVar23;
          }
        }
        do {
          uVar10 = *(undefined8 *)(*pauVar27 + 8);
          uVar11 = *(undefined8 *)(*pauVar27 + 0x10);
          uVar32 = *(undefined8 *)(*pauVar27 + 0x18);
          uVar33 = *(undefined8 *)pauVar27[1];
          uVar12 = *(undefined8 *)(pauVar27[1] + 8);
          uVar13 = *(undefined8 *)(pauVar27[1] + 0x10);
          uVar14 = *(undefined8 *)(pauVar27[1] + 0x18);
          uVar15 = *(undefined8 *)pauVar27[2];
          uVar16 = *(undefined8 *)(pauVar27[2] + 8);
          uVar17 = *(undefined8 *)(pauVar27[2] + 0x10);
          uVar18 = *(undefined8 *)(pauVar27[2] + 0x18);
          uVar19 = *(undefined8 *)pauVar27[3];
          uVar20 = *(undefined8 *)(pauVar27[3] + 8);
          uVar21 = *(undefined8 *)(pauVar27[3] + 0x10);
          uVar22 = *(undefined8 *)(pauVar27[3] + 0x18);
          *(undefined8 *)*pauVar24 = *(undefined8 *)*pauVar27;
          *(undefined8 *)(*pauVar24 + 8) = uVar10;
          *(undefined8 *)(*pauVar24 + 0x10) = uVar11;
          *(undefined8 *)(*pauVar24 + 0x18) = uVar32;
          *(undefined8 *)pauVar24[1] = uVar33;
          *(undefined8 *)(pauVar24[1] + 8) = uVar12;
          *(undefined8 *)(pauVar24[1] + 0x10) = uVar13;
          *(undefined8 *)(pauVar24[1] + 0x18) = uVar14;
          *(undefined8 *)pauVar24[2] = uVar15;
          *(undefined8 *)(pauVar24[2] + 8) = uVar16;
          *(undefined8 *)(pauVar24[2] + 0x10) = uVar17;
          *(undefined8 *)(pauVar24[2] + 0x18) = uVar18;
          *(undefined8 *)pauVar24[3] = uVar19;
          *(undefined8 *)(pauVar24[3] + 8) = uVar20;
          *(undefined8 *)(pauVar24[3] + 0x10) = uVar21;
          *(undefined8 *)(pauVar24[3] + 0x18) = uVar22;
          uVar10 = *(undefined8 *)(pauVar27[4] + 8);
          uVar11 = *(undefined8 *)(pauVar27[4] + 0x10);
          uVar32 = *(undefined8 *)(pauVar27[4] + 0x18);
          uVar33 = *(undefined8 *)pauVar27[5];
          uVar12 = *(undefined8 *)(pauVar27[5] + 8);
          uVar13 = *(undefined8 *)(pauVar27[5] + 0x10);
          uVar14 = *(undefined8 *)(pauVar27[5] + 0x18);
          uVar15 = *(undefined8 *)pauVar27[6];
          uVar16 = *(undefined8 *)(pauVar27[6] + 8);
          uVar17 = *(undefined8 *)(pauVar27[6] + 0x10);
          uVar18 = *(undefined8 *)(pauVar27[6] + 0x18);
          uVar19 = *(undefined8 *)pauVar27[7];
          uVar20 = *(undefined8 *)(pauVar27[7] + 8);
          uVar21 = *(undefined8 *)(pauVar27[7] + 0x10);
          uVar22 = *(undefined8 *)(pauVar27[7] + 0x18);
          *(undefined8 *)pauVar24[4] = *(undefined8 *)pauVar27[4];
          *(undefined8 *)(pauVar24[4] + 8) = uVar10;
          *(undefined8 *)(pauVar24[4] + 0x10) = uVar11;
          *(undefined8 *)(pauVar24[4] + 0x18) = uVar32;
          *(undefined8 *)pauVar24[5] = uVar33;
          *(undefined8 *)(pauVar24[5] + 8) = uVar12;
          *(undefined8 *)(pauVar24[5] + 0x10) = uVar13;
          *(undefined8 *)(pauVar24[5] + 0x18) = uVar14;
          *(undefined8 *)pauVar24[6] = uVar15;
          *(undefined8 *)(pauVar24[6] + 8) = uVar16;
          *(undefined8 *)(pauVar24[6] + 0x10) = uVar17;
          *(undefined8 *)(pauVar24[6] + 0x18) = uVar18;
          *(undefined8 *)pauVar24[7] = uVar19;
          *(undefined8 *)(pauVar24[7] + 8) = uVar20;
          *(undefined8 *)(pauVar24[7] + 0x10) = uVar21;
          *(undefined8 *)(pauVar24[7] + 0x18) = uVar22;
          pauVar24 = pauVar24 + 8;
          pauVar27 = pauVar27 + 8;
          param_3 = param_3 - 0x100;
        } while (0xff < param_3);
      }
    }
                    /* WARNING: Could not recover jumptable at 0x000142ef7e12. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    puVar23 = (undefined8 *)
              (*(code *)(IMAGE_DOS_HEADER_140000000.e_magic +
                        *(uint *)(&DAT_143c47040 + (param_3 + 0x1f >> 5) * 4)))();
    return puVar23;
  }
  for (; param_3 != 0; param_3 = param_3 - 1) {
    *(undefined1 *)param_1 = *(undefined1 *)param_2;
    param_2 = (undefined8 *)((longlong)param_2 + 1);
    param_1 = (undefined8 *)((longlong)param_1 + 1);
  }
  return puVar23;
}



//===========================================================
// FUN_14040ea40 @ 14040ea40   (275 bytes)
//===========================================================

undefined8 * FUN_14040ea40(longlong *param_1,undefined8 *param_2,undefined8 param_3,int param_4)

{
  undefined4 *puVar1;
  longlong lVar2;
  int iVar3;
  undefined4 *puVar4;
  int iVar5;
  int iVar6;
  undefined4 *local_28 [2];
  
  iVar6 = 0;
  if (param_4 == 0) {
    *param_2 = 0;
    FUN_1401c1fb0(param_2,param_1);
  }
  else {
    local_28[0] = (undefined4 *)0x0;
    iVar3 = iVar6;
    if (*param_1 != 0) {
      iVar3 = (int)((ulonglong)(longlong)*(int *)(*param_1 + -8) >> 1);
    }
    iVar5 = iVar6;
    if (0 < iVar3 + param_4) {
      iVar5 = iVar3 + param_4;
    }
    puVar4 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar5 * 2 + 0x12));
    puVar4[1] = iVar5;
    *puVar4 = 0xffffffff;
    puVar1 = puVar4 + 4;
    puVar4[2] = 0;
    *(undefined2 *)puVar1 = 0;
    lVar2 = *param_1;
    if (lVar2 == 0) {
      iVar3 = 0;
    }
    else {
      iVar3 = (int)((ulonglong)(longlong)*(int *)(lVar2 + -8) >> 1);
    }
    local_28[0] = puVar1;
    FUN_142ef7ba0(puVar1,lVar2,(longlong)iVar3 * 2);
    iVar3 = iVar6;
    if (*param_1 != 0) {
      iVar3 = (int)((ulonglong)(longlong)*(int *)(*param_1 + -8) >> 1);
    }
    FUN_142ef7ba0((undefined2 *)((longlong)puVar1 + (longlong)iVar3 * 2),param_3,
                  (longlong)param_4 * 2);
    if (*param_1 != 0) {
      iVar6 = (int)((ulonglong)(longlong)*(int *)(*param_1 + -8) >> 1);
    }
    FUN_1401bd8a0(local_28,iVar6 + param_4);
    *param_2 = puVar1;
  }
  return param_2;
}



//===========================================================
// FUN_142e52d50 @ 142e52d50   (119 bytes)
//===========================================================

void FUN_142e52d50(undefined4 param_1)

{
  char cVar1;
  undefined4 local_res8 [4];
  undefined4 local_res18 [2];
  longlong local_res20;
  
  local_res8[0] = param_1;
  cVar1 = FUN_142e559e0();
  if (cVar1 != '\0') {
    FUN_140194c60(&local_res20);
    local_res18[0] = FUN_14091a3e0(&local_res20);
    FUN_142e5d030("LogCallStack1",&DAT_1434997dc,local_res18,&DAT_1434997f8,local_res8,&local_res20)
    ;
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_1401a88a0 @ 1401a88a0   (453 bytes)
//===========================================================

longlong * FUN_1401a88a0(longlong *param_1,undefined1 param_2)

{
  longlong lVar1;
  undefined4 *puVar2;
  
  switch(param_2) {
  case 0:
    FUN_140196ed0(param_1,PTR_s_Black_143a45418,0xffffffff);
    return param_1;
  case 1:
    FUN_140196ed0(param_1,PTR_DAT_143a45420,0xffffffff);
    return param_1;
  case 2:
    FUN_140196ed0(param_1,PTR_s_Orange_143a45428,0xffffffff);
    return param_1;
  case 3:
    FUN_140196ed0(param_1,PTR_s_Yellow_143a45430,0xffffffff);
    return param_1;
  case 4:
    FUN_140196ed0(param_1,PTR_s_Green_143a45438,0xffffffff);
    return param_1;
  case 5:
    FUN_140196ed0(param_1,PTR_DAT_143a45440,0xffffffff);
    return param_1;
  case 6:
    FUN_140196ed0(param_1,PTR_s_Violet_143a45448,0xffffffff);
    return param_1;
  case 7:
    FUN_140196ed0(param_1,PTR_s_Hazel_143a45450,0xffffffff);
    return param_1;
  }
  *param_1 = 0;
  puVar2 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x15);
  puVar2[1] = 4;
  *puVar2 = 0xffffffff;
  *param_1 = (longlong)(puVar2 + 4);
  puVar2[2] = 0;
  *(undefined1 *)*param_1 = 0;
  *(undefined4 *)*param_1 = DAT_143272524;
  lVar1 = *param_1;
  if (*(int *)(lVar1 + -0x10) != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (*(int *)(lVar1 + -0xc) < 4) {
    FUN_142e54290(0x90,*(int *)(lVar1 + -0xc),4);
  }
  *(undefined4 *)(lVar1 + -0x10) = 1;
  *(undefined1 *)(*param_1 + 4) = 0;
  if (*(int *)(lVar1 + -0xc) + 1 < 5) {
    FUN_142e54290(0x9c,4);
  }
  *(undefined4 *)(lVar1 + -8) = 4;
  return param_1;
}



//===========================================================
// FUN_140cbb020 @ 140cbb020   (15 bytes)
//===========================================================

void FUN_140cbb020(void)

{
  FUN_140cbca60();
  return;
}



//===========================================================
// FUN_141aaae90 @ 141aaae90   (809 bytes)
//===========================================================

longlong FUN_141aaae90(longlong *param_1,longlong param_2,undefined4 param_3,int param_4,int param_5
                      ,undefined4 param_6,undefined4 param_7,longlong *param_8,int param_9)

{
  undefined8 *puVar1;
  longlong *plVar2;
  code *pcVar3;
  longlong lVar4;
  undefined8 uVar5;
  longlong lVar6;
  ulonglong uVar7;
  longlong *plVar8;
  longlong local_res8;
  longlong local_res10;
  undefined4 local_res18 [4];
  undefined4 uVar9;
  int local_60;
  int local_5c;
  longlong *local_58;
  longlong *local_50;
  longlong local_48;
  longlong *local_40;
  
  local_res10 = param_2;
  local_res18[0] = param_3;
  local_res8 = FUN_14019b780(&DAT_143ad68a0,0x11e8);
  lVar4 = 0;
  if (local_res8 != 0) {
    lVar4 = FUN_141aeed80(local_res8);
  }
  lVar6 = lVar4 + 0x18;
  if (lVar4 == 0) {
    lVar6 = 0;
  }
  if (lVar6 == 0) {
    *(undefined8 *)(param_2 + 8) = 0;
  }
  else {
    *(longlong *)(param_2 + 8) = lVar6 + -0x18;
    if (lVar6 + -0x18 != 0) {
      if (0xfffff < *(ulonglong *)(lVar6 + 8)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar6 + 8) = *(longlong *)(lVar6 + 8) + 1;
      UNLOCK();
      param_3 = local_res18[0];
    }
  }
  uVar9 = 1;
  plVar8 = *(longlong **)(param_2 + 8);
  if (plVar8 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
    plVar8 = *(longlong **)(param_2 + 8);
  }
  puVar1 = (undefined8 *)*param_1;
  (**(code **)(*plVar8 + 0x10))
            (plVar8,*puVar1,param_3,*(int *)((longlong)puVar1 + 0xc) + param_4,
             param_5 + *(int *)(puVar1 + 2),param_6,param_7,0,uVar9);
  lVar4 = *(longlong *)(param_2 + 8);
  if (lVar4 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar4 = *(longlong *)(param_2 + 8);
  }
  local_res8 = 0;
  FUN_14019a260(&local_res8,param_8);
  local_50 = &local_res8;
  (**(code **)(*(longlong *)(lVar4 + 8) + 0x80))((longlong *)(lVar4 + 8),1);
  FUN_14019a260(lVar4 + 0x11a0,&local_res8);
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  if (param_9 != 0) {
    lVar4 = *(longlong *)(param_2 + 8);
    if (lVar4 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar4 = *(longlong *)(param_2 + 8);
    }
    uVar7 = *(ulonglong *)(lVar4 + 0x11c4);
    FUN_14170f7d0(lVar4,&local_60);
    if (param_9 == 2) {
      uVar7 = (ulonglong)(uint)-local_60;
    }
    else if (param_9 == 6) {
      uVar7 = (ulonglong)(uint)-local_5c << 0x20;
    }
    else if (param_9 == 8) {
      uVar7 = CONCAT44(-local_5c,-local_60);
    }
    lVar4 = *(longlong *)(param_2 + 8);
    if (lVar4 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar4 = *(longlong *)(param_2 + 8);
    }
    *(ulonglong *)(lVar4 + 0x11c4) = uVar7;
  }
  plVar8 = *(longlong **)(param_2 + 8);
  if (plVar8 != (longlong *)0x0) {
    if (plVar8 == (longlong *)0xffffffffffffffe8) {
      plVar8 = (longlong *)0x0;
    }
    local_40 = plVar8;
    if (plVar8 != (longlong *)0x0) {
      if (0xfffff < (ulonglong)plVar8[4]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar8[4] = plVar8[4] + 1;
      UNLOCK();
    }
    plVar8 = local_40;
    local_50 = &local_48;
    if (local_40 == (longlong *)0x0) {
      FUN_14126b0d0(&local_48);
    }
    else {
      plVar2 = *(longlong **)(*param_1 + 0x18);
      if (plVar2 != (longlong *)0x0) {
        pcVar3 = *(code **)(*local_40 + 0x78);
        local_58 = plVar2;
        (**(code **)(*plVar2 + 8))();
        (*pcVar3)(plVar8,&local_58);
      }
      uVar5 = FUN_141afab70(*param_1 + 0x28,0xffffffff);
      FUN_14126acd0(uVar5,&local_48);
      FUN_14126b0d0(&local_48);
    }
    lVar4 = FUN_141af8580(*param_1 + 0x1f8,local_res18);
    if ((*(longlong *)(lVar4 + 0x20) - 1U < 999) || (*(longlong *)(lVar4 + 0x20) == -1)) {
      FUN_142e52ed0(0x447);
    }
    if (lVar4 + 0x18 == param_2) {
      FUN_142e52d50(0x45c,1);
    }
    lVar6 = *(longlong *)(param_2 + 8);
    if (lVar6 != 0) {
      if (0xfffff < *(ulonglong *)(lVar6 + 0x20)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar6 + 0x20) = *(longlong *)(lVar6 + 0x20) + 1;
      UNLOCK();
    }
    FUN_14147d360(lVar4 + 0x18);
    *(undefined8 *)(lVar4 + 0x20) = *(undefined8 *)(param_2 + 8);
  }
  if (*param_8 != 0) {
    FUN_14019f2c0(*param_8 + -0x10);
  }
  return param_2;
}



//===========================================================
// FUN_141ac3370 @ 141ac3370   (1238 bytes)
//===========================================================

void FUN_141ac3370(undefined8 param_1,longlong param_2,undefined4 param_3,undefined4 param_4,
                  undefined4 param_5,undefined1 param_6,undefined1 param_7,undefined8 param_8)

{
  IUnknown *pIVar1;
  int iVar2;
  int *piVar3;
  longlong *plVar4;
  undefined8 uVar5;
  uint uVar6;
  uint uVar7;
  ulonglong uVar8;
  int iVar9;
  ulonglong uVar10;
  longlong *local_res10;
  uint *puVar11;
  undefined4 uVar12;
  undefined4 local_f8;
  undefined4 uStack_f4;
  undefined8 uStack_f0;
  undefined8 local_e8;
  short local_e0;
  undefined6 uStack_de;
  longlong lStack_d8;
  undefined8 local_d0;
  uint local_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 local_b8;
  undefined1 local_b0 [8];
  longlong *local_a8;
  longlong local_a0;
  longlong *local_98;
  uint local_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined8 local_80;
  undefined8 local_78;
  longlong lStack_70;
  undefined8 local_68;
  uint local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  
  plVar4 = (longlong *)0x0;
  local_res10 = (longlong *)0x0;
  uVar8 = 0xffffffffffffffff;
  uVar10 = uVar8;
  if (param_2 != 0) {
    do {
      uVar10 = uVar10 + 1;
    } while (*(short *)(param_2 + uVar10 * 2) != 0);
    iVar9 = (int)uVar10;
    iVar2 = 0;
    if (0 < iVar9) {
      iVar2 = iVar9;
    }
    piVar3 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar2 * 2 + 0x12));
    piVar3[1] = iVar2;
    *piVar3 = -1;
    plVar4 = (longlong *)(piVar3 + 4);
    piVar3[2] = 0;
    *(undefined2 *)plVar4 = 0;
    local_res10 = plVar4;
    FUN_142ef7ba0(plVar4,param_2,(longlong)iVar9 * 2);
    if (*piVar3 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar9 == -1) || (iVar9 <= piVar3[1])) {
      *piVar3 = 1;
      if (iVar9 != -1) goto LAB_141ac343e;
      uVar10 = uVar8;
      if (plVar4 == (longlong *)0x0) {
        uVar10 = 0;
      }
      else {
        do {
          uVar10 = uVar10 + 1;
        } while (*(short *)((longlong)plVar4 + uVar10 * 2) != 0);
      }
    }
    else {
      FUN_142e54290(0x90,piVar3[1],uVar10 & 0xffffffff);
      *piVar3 = 1;
LAB_141ac343e:
      *(undefined2 *)((longlong)iVar9 * 2 + (longlong)plVar4) = 0;
    }
    iVar2 = (int)uVar10;
    if ((iVar2 < 0) || (piVar3[1] + 1 <= iVar2)) {
      FUN_142e54290(0x9c,uVar10 & 0xffffffff);
    }
    piVar3[2] = iVar2 * 2;
  }
  uVar7 = 0;
  do {
    uVar8 = uVar8 + 1;
  } while ((&DAT_1432ac608)[uVar8] != 0);
  FUN_14040ea40(&local_res10,&local_a0,&DAT_1432ac608,uVar8);
  if (plVar4 != (longlong *)0x0) {
    FUN_1401bebb0(plVar4 + -2);
  }
  pIVar1 = DAT_143add058;
  if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_e0);
  if (DAT_143a8b8d8 == 8) {
    if (local_e0 == 8) {
      local_e0 = 0;
      if (lStack_d8 != 0) {
        (*DAT_143ad5990)(lStack_d8 + -4);
      }
    }
    else {
      iVar2 = (*DAT_143262a18)(&local_e0);
      if (iVar2 < 0) goto LAB_141ac3839;
    }
    local_e0 = 8;
    uVar6 = uVar7;
    if (DAT_143a8b8e0 != 0) {
      uVar6 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_d8 = FUN_1401a5fa0(DAT_143a8b8e0,uVar6);
  }
  else {
    if ((local_e0 == 8) && (local_e0 = 0, lStack_d8 != 0)) {
      (*DAT_143ad5990)(lStack_d8 + -4);
    }
    iVar2 = (*DAT_143262a28)(&local_e0,&DAT_143a8b8d8);
    if (iVar2 < 0) {
LAB_141ac3839:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar2);
    }
  }
  (*DAT_143262a20)(&local_f8);
  if (DAT_143a8b8d8 == 8) {
    if ((short)local_f8 == 8) {
      local_f8 = (uint)local_f8._2_2_ << 0x10;
      if (uStack_f0 != 0) {
        (*DAT_143ad5990)(uStack_f0 + -4);
      }
    }
    else {
      iVar2 = (*DAT_143262a18)(&local_f8);
      if (iVar2 < 0) goto LAB_141ac3841;
    }
    local_f8 = CONCAT22(local_f8._2_2_,8);
    if (DAT_143a8b8e0 != 0) {
      uVar7 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    uStack_f0 = FUN_1401a5fa0(DAT_143a8b8e0,uVar7);
  }
  else {
    if (((short)local_f8 == 8) && (local_f8 = (uint)local_f8._2_2_ << 0x10, uStack_f0 != 0)) {
      (*DAT_143ad5990)(uStack_f0 + -4);
    }
    iVar2 = (*DAT_143262a28)(&local_f8,&DAT_143a8b8d8);
    if (iVar2 < 0) {
LAB_141ac3841:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar2);
    }
  }
  plVar4 = (longlong *)FUN_1401a5890(local_b0,param_2);
  local_res10 = plVar4;
  (*DAT_143262a20)(&local_90);
  uVar5 = 0;
  if ((undefined8 *)*plVar4 != (undefined8 *)0x0) {
    uVar5 = *(undefined8 *)*plVar4;
  }
  local_78 = CONCAT62(uStack_de,local_e0);
  lStack_70 = lStack_d8;
  local_68 = local_d0;
  local_58 = local_f8;
  uStack_54 = uStack_f4;
  uStack_50 = (undefined4)uStack_f0;
  uStack_4c = uStack_f0._4_4_;
  local_48 = local_e8;
  puVar11 = &local_90;
  iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x48))(pIVar1,uVar5,&local_58,&local_78,puVar11);
  uVar12 = (undefined4)((ulonglong)puVar11 >> 0x20);
  if (iVar2 < 0) {
    _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_1432743e8);
  }
  local_c8 = local_90;
  uStack_c4 = uStack_8c;
  uStack_c0 = uStack_88;
  uStack_bc = uStack_84;
  local_b8 = local_80;
  local_90 = local_90 & 0xffff0000;
  FUN_1401be120(plVar4);
  uVar5 = FUN_1409339d0(&local_a8,&local_c8);
  FUN_1401a5040(&local_98,uVar5);
  if (local_a8 != (longlong *)0x0) {
    (**(code **)(*local_a8 + 0x10))();
  }
  if ((short)local_c8 == 8) {
    local_c8 = local_c8 & 0xffff0000;
    if (CONCAT44(uStack_bc,uStack_c0) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_bc,uStack_c0) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_c8);
  }
  if ((short)local_f8 == 8) {
    local_f8 = local_f8 & 0xffff0000;
    if (uStack_f0 != 0) {
      (*DAT_143ad5990)(uStack_f0 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_f8);
  }
  if (local_e0 == 8) {
    local_e0 = 0;
    if (lStack_d8 != 0) {
      (*DAT_143ad5990)(lStack_d8 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_e0);
  }
  if (local_98 != (longlong *)0x0) {
    local_res10 = local_98;
    (**(code **)(*local_98 + 8))(local_98);
    FUN_141ac3850(param_1,local_a0,&local_res10,param_3,CONCAT44(uVar12,param_4),param_5,param_6,
                  param_7,param_8);
    (**(code **)(*local_98 + 0x10))(local_98);
  }
  if (local_a0 != 0) {
    FUN_1401bebb0(local_a0 + -0x10);
  }
  return;
}



//===========================================================
// FUN_141ad46a0 @ 141ad46a0   (90 bytes)
//===========================================================

longlong FUN_141ad46a0(longlong *param_1,longlong param_2,undefined4 param_3)

{
  longlong lVar1;
  undefined4 local_res18 [4];
  
  local_res18[0] = param_3;
  lVar1 = FUN_141af79f0(*param_1 + 0x60,local_res18);
  lVar1 = *(longlong *)(lVar1 + 0x20);
  *(longlong *)(param_2 + 8) = lVar1;
  if (lVar1 != 0) {
    if (0xfffff < *(ulonglong *)(lVar1 + 0x20)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar1 + 0x20) = *(longlong *)(lVar1 + 0x20) + 1;
    UNLOCK();
  }
  return param_2;
}



//===========================================================
// FUN_141473950 @ 141473950   (464 bytes)
//===========================================================

longlong FUN_141473950(longlong *param_1,undefined8 *param_2,longlong param_3)

{
  longlong lVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 *puVar4;
  ulonglong uVar5;
  undefined8 *puVar6;
  undefined8 *puVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  longlong lVar10;
  longlong lVar11;
  
  lVar10 = *param_1;
  lVar2 = param_1[1] - lVar10 >> 4;
  if (lVar2 == 0xfffffffffffffff) {
                    /* WARNING: Subroutine does not return */
    FUN_14131c5a0();
  }
  uVar9 = lVar2 + 1;
  uVar5 = param_1[2] - lVar10 >> 4;
  if (0xfffffffffffffff - (uVar5 >> 1) < uVar5) {
    uVar8 = 0xfffffffffffffff;
  }
  else {
    uVar5 = (uVar5 >> 1) + uVar5;
    uVar8 = uVar9;
    if (uVar9 <= uVar5) {
      uVar8 = uVar5;
    }
    if (0xfffffffffffffff < uVar8) {
                    /* WARNING: Subroutine does not return */
      FUN_14019f970();
    }
  }
  lVar3 = FUN_140197eb0(uVar8 << 4);
  uVar5 = (longlong)param_2 - lVar10 & 0xfffffffffffffff0;
  lVar2 = uVar5 + lVar3;
  lVar10 = lVar2 + 0x10;
  lVar1 = *(longlong *)(param_3 + 8);
  *(longlong *)(lVar2 + 8) = lVar1;
  lVar11 = lVar3;
  if (lVar1 != 0) {
    if (0xfffff < *(ulonglong *)(lVar1 + 0x20)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar1 + 0x20) = *(longlong *)(lVar1 + 0x20) + 1;
    UNLOCK();
  }
  puVar7 = (undefined8 *)param_1[1];
  puVar4 = (undefined8 *)*param_1;
  if (param_2 == puVar7) {
    if (puVar4 != puVar7) {
      puVar6 = (undefined8 *)(lVar3 + 8);
      do {
        *puVar6 = 0;
        *puVar6 = puVar4[1];
        puVar4[1] = 0;
        puVar6 = puVar6 + 2;
        puVar4 = puVar4 + 2;
      } while (puVar4 != puVar7);
    }
  }
  else {
    if (puVar4 != param_2) {
      puVar7 = (undefined8 *)(lVar3 + 8);
      do {
        *puVar7 = 0;
        *puVar7 = puVar4[1];
        puVar4[1] = 0;
        puVar7 = puVar7 + 2;
        puVar4 = puVar4 + 2;
      } while (puVar4 != param_2);
      puVar7 = (undefined8 *)param_1[1];
    }
    if (param_2 != puVar7) {
      lVar1 = (uVar5 - (longlong)param_2) + lVar3;
      puVar4 = param_2 + 1;
      do {
        *(undefined8 *)(lVar1 + 0x10 + (longlong)puVar4) = 0;
        *(undefined8 *)(lVar1 + 0x10 + (longlong)puVar4) = *puVar4;
        *puVar4 = 0;
        puVar6 = puVar4 + 1;
        puVar4 = puVar4 + 2;
      } while (puVar6 != puVar7);
    }
  }
  FUN_14147caa0(param_1,lVar3,uVar9,uVar8,uVar9,lVar10,lVar11);
  return lVar2;
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
// FUN_141711e10 @ 141711e10   (4 bytes)
//===========================================================

undefined4 FUN_141711e10(longlong param_1)

{
  return *(undefined4 *)(param_1 + 0x4c);
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
// FUN_1401a8a90 @ 1401a8a90   (483 bytes)
//===========================================================

longlong * FUN_1401a8a90(longlong *param_1,undefined1 param_2)

{
  longlong lVar1;
  undefined4 *puVar2;
  
  switch(param_2) {
  case 0:
    FUN_140196ed0(param_1,PTR_s_Black_143a45418,0xffffffff);
    return param_1;
  case 1:
    FUN_140196ed0(param_1,PTR_DAT_143a45440,0xffffffff);
    return param_1;
  case 2:
    FUN_140196ed0(param_1,PTR_DAT_143a45420,0xffffffff);
    return param_1;
  case 3:
    FUN_140196ed0(param_1,PTR_s_Green_143a45438,0xffffffff);
    return param_1;
  case 4:
    FUN_140196ed0(param_1,PTR_s_Hazel_143a45450,0xffffffff);
    return param_1;
  case 5:
    FUN_140196ed0(param_1,PTR_s_Sapphire_143a45768,0xffffffff);
    return param_1;
  case 6:
    FUN_140196ed0(param_1,PTR_s_Violet_143a45448,0xffffffff);
    return param_1;
  case 7:
    FUN_140196ed0(param_1,PTR_s_Amethyst_143a45770,0xffffffff);
    return param_1;
  case 8:
    FUN_140196ed0(param_1,PTR_s_White_143a45410,0xffffffff);
    return param_1;
  }
  *param_1 = 0;
  puVar2 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x15);
  puVar2[1] = 4;
  *puVar2 = 0xffffffff;
  *param_1 = (longlong)(puVar2 + 4);
  puVar2[2] = 0;
  *(undefined1 *)*param_1 = 0;
  *(undefined4 *)*param_1 = DAT_143272524;
  lVar1 = *param_1;
  if (*(int *)(lVar1 + -0x10) != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (*(int *)(lVar1 + -0xc) < 4) {
    FUN_142e54290(0x90,*(int *)(lVar1 + -0xc),4);
  }
  *(undefined4 *)(lVar1 + -0x10) = 1;
  *(undefined1 *)(*param_1 + 4) = 0;
  if (*(int *)(lVar1 + -0xc) + 1 < 5) {
    FUN_142e54290(0x9c,4);
  }
  *(undefined4 *)(lVar1 + -8) = 4;
  return param_1;
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
// FUN_142e52ed0 @ 142e52ed0   (4890 bytes)
//===========================================================

void FUN_142e52ed0(undefined4 param_1,undefined8 param_2)

{
  char *pcVar1;
  longlong lVar2;
  char cVar3;
  undefined8 uVar4;
  undefined4 *puVar5;
  longlong lVar6;
  int iVar7;
  int *piVar8;
  int iVar9;
  int iVar10;
  int *piVar11;
  int *piVar12;
  int *piVar13;
  int iVar14;
  int iVar15;
  longlong lVar16;
  undefined4 local_res8 [2];
  undefined8 local_res10;
  undefined4 local_res18;
  undefined4 local_res20 [2];
  char *local_a8;
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
  longlong local_50 [2];
  
  piVar13 = (int *)0x0;
  iVar9 = 0;
  local_res18 = 0;
  local_res8[0] = param_1;
  local_res10 = param_2;
  cVar3 = FUN_142e559e0();
  if (cVar3 == '\0') {
    return;
  }
  FUN_140194c60(&local_78);
  local_res20[0] = FUN_14091a3e0(&local_78);
  uVar4 = FUN_142a1d8a0(local_50);
  local_a8 = (char *)0x0;
  local_res18 = 1;
  FUN_1408bc980(&local_98,uVar4);
  lVar2 = local_98;
  pcVar1 = local_a8;
  local_res18 = 7;
  piVar12 = (int *)0xffffffffffffffff;
  if (local_98 != 0) {
    iVar10 = *(int *)(local_98 + -8);
    piVar8 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar11 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e530bb;
      if (*local_a8 == '\0') {
        if ((local_a8 == (char *)0x0) ||
           (piVar11 = (int *)(local_a8 + -0x10), piVar11 == (int *)0x0)) {
LAB_142e530bb:
          if (iVar15 < iVar10) {
            iVar15 = iVar10;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
          puVar5[1] = iVar15;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          puVar5[2] = 0;
          *local_a8 = '\0';
          if (piVar11 != (int *)0x0) {
            FUN_14019f2c0(piVar11);
          }
        }
        else {
          if ((1 < *piVar11) || (*(int *)(local_a8 + -0xc) < iVar10)) {
            iVar15 = *(int *)(local_a8 + -8);
            goto LAB_142e530bb;
          }
          if (*piVar11 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar11 = -1;
        }
        FUN_142ef7ba0(local_a8,lVar2,piVar8);
        pcVar1 = local_a8;
        if (*(int *)(local_a8 + -0x10) != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
          if (iVar10 != -1) goto LAB_142e53148;
          piVar8 = piVar13;
          if (pcVar1 != (char *)0x0) {
            do {
              piVar12 = (int *)((longlong)piVar12 + 1);
              piVar8 = piVar12;
            } while (pcVar1[(longlong)piVar12] != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
LAB_142e53148:
          *(char *)((longlong)piVar8 + (longlong)local_a8) = '\0';
        }
        iVar10 = (int)piVar8;
        if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
          FUN_142e54290(0x9c,(ulonglong)piVar8 & 0xffffffff);
        }
        *(int *)(pcVar1 + -8) = iVar10;
        goto LAB_142e53173;
      }
      iVar15 = *(int *)(local_a8 + -8);
      for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
      }
      piVar12 = (int *)(local_a8 + -0x10);
      iVar14 = iVar9;
      if (piVar12 == (int *)0x0) {
LAB_142e52fbf:
        if (iVar14 < iVar7) {
          iVar14 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
        puVar5[1] = iVar14;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        if (piVar12 == (int *)0x0) {
          puVar5[2] = 0;
          *local_a8 = '\0';
        }
        else {
          iVar7 = *(int *)(pcVar1 + -8) + 1;
          if (iVar14 + 1 < iVar7) {
            FUN_142e54290(0x5c,iVar7,iVar14 + 1);
            iVar7 = iVar14 + 1;
          }
          FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
          puVar5[2] = *(undefined4 *)(pcVar1 + -8);
          local_a8[iVar14] = '\0';
          FUN_14019f2c0(piVar12);
        }
      }
      else {
        if ((1 < *piVar12) || (*(int *)(local_a8 + -0xc) < iVar7)) {
          iVar14 = *(int *)(local_a8 + -8);
          goto LAB_142e52fbf;
        }
        if (*piVar12 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar12 = -1;
      }
      iVar7 = iVar9;
      if (local_a8 != (char *)0x0) {
        iVar7 = *(int *)(local_a8 + -8);
      }
      FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar8);
      FUN_14019c870(&local_a8,iVar15 + iVar10);
    }
  }
LAB_142e53173:
  piVar12 = (int *)0xffffffffffffffff;
  if (local_98 != 0) {
    FUN_14019f2c0(local_98 + -0x10);
  }
  local_70 = 0;
  uVar4 = FUN_14019ba10(&local_70,&DAT_143272338,"LogCallStack3");
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x27;
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  lVar2 = local_a0;
  pcVar1 = local_a8;
  local_res18 = 0x1f;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    piVar8 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar11 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e53377;
      if (*local_a8 != '\0') {
        iVar15 = *(int *)(local_a8 + -8);
        for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
        }
        piVar12 = (int *)(local_a8 + -0x10);
        iVar14 = iVar9;
        if (piVar12 == (int *)0x0) {
LAB_142e5327f:
          if (iVar14 < iVar7) {
            iVar14 = iVar7;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
          puVar5[1] = iVar14;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          if (piVar12 == (int *)0x0) {
            puVar5[2] = 0;
            *local_a8 = '\0';
          }
          else {
            iVar7 = *(int *)(pcVar1 + -8) + 1;
            if (iVar14 + 1 < iVar7) {
              FUN_142e54290(0x5c,iVar7,iVar14 + 1);
              iVar7 = iVar14 + 1;
            }
            FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
            puVar5[2] = *(undefined4 *)(pcVar1 + -8);
            local_a8[iVar14] = '\0';
            FUN_14019f2c0(piVar12);
          }
        }
        else {
          if ((1 < *piVar12) || (*(int *)(local_a8 + -0xc) < iVar7)) {
            iVar14 = *(int *)(local_a8 + -8);
            goto LAB_142e5327f;
          }
          if (*piVar12 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar12 = -1;
        }
        iVar7 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar7 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar8);
        FUN_14019c870(&local_a8,iVar15 + iVar10);
        goto LAB_142e5342f;
      }
      if ((local_a8 == (char *)0x0) || (piVar11 = (int *)(local_a8 + -0x10), piVar11 == (int *)0x0))
      {
LAB_142e53377:
        if (iVar15 < iVar10) {
          iVar15 = iVar10;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
        puVar5[1] = iVar15;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        puVar5[2] = 0;
        *local_a8 = '\0';
        if (piVar11 != (int *)0x0) {
          FUN_14019f2c0(piVar11);
        }
      }
      else {
        if ((1 < *piVar11) || (*(int *)(local_a8 + -0xc) < iVar10)) {
          iVar15 = *(int *)(local_a8 + -8);
          goto LAB_142e53377;
        }
        if (*piVar11 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar11 = -1;
      }
      FUN_142ef7ba0(local_a8,lVar2,piVar8);
      pcVar1 = local_a8;
      if (*(int *)(local_a8 + -0x10) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
        if (iVar10 != -1) goto LAB_142e53407;
        piVar8 = piVar13;
        if (pcVar1 != (char *)0x0) {
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
            piVar8 = piVar12;
          } while (pcVar1[(longlong)piVar12] != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
LAB_142e53407:
        *(char *)((longlong)piVar8 + (longlong)local_a8) = '\0';
      }
      iVar10 = (int)piVar8;
      if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
        FUN_142e54290(0x9c,(ulonglong)piVar8 & 0xffffffff);
      }
      *(int *)(pcVar1 + -8) = iVar10;
    }
  }
LAB_142e5342f:
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  local_68 = 0;
  uVar4 = FUN_14019ba10(&local_68,&DAT_143272338,&DAT_1434997dc);
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x11f;
  if (local_68 != 0) {
    FUN_14019f2c0(local_68 + -0x10);
  }
  lVar2 = local_a0;
  pcVar1 = local_a8;
  local_res18 = 0xdf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    piVar12 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar8 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e5363e;
      if (*local_a8 != '\0') {
        iVar15 = *(int *)(local_a8 + -8);
        for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
        }
        piVar8 = (int *)(local_a8 + -0x10);
        iVar14 = iVar9;
        if (piVar8 == (int *)0x0) {
LAB_142e5353f:
          if (iVar14 < iVar7) {
            iVar14 = iVar7;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
          puVar5[1] = iVar14;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          if (piVar8 == (int *)0x0) {
            puVar5[2] = 0;
            *local_a8 = '\0';
          }
          else {
            iVar7 = *(int *)(pcVar1 + -8) + 1;
            if (iVar14 + 1 < iVar7) {
              FUN_142e54290(0x5c,iVar7,iVar14 + 1);
              iVar7 = iVar14 + 1;
            }
            FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
            puVar5[2] = *(undefined4 *)(pcVar1 + -8);
            local_a8[iVar14] = '\0';
            FUN_14019f2c0(piVar8);
          }
        }
        else {
          if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar7)) {
            iVar14 = *(int *)(local_a8 + -8);
            goto LAB_142e5353f;
          }
          if (*piVar8 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar8 = -1;
        }
        iVar7 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar7 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
        FUN_14019c870(&local_a8,iVar15 + iVar10);
        goto LAB_142e536fd;
      }
      if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0)) {
LAB_142e5363e:
        if (iVar15 < iVar10) {
          iVar15 = iVar10;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
        puVar5[1] = iVar15;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        puVar5[2] = 0;
        *local_a8 = '\0';
        if (piVar8 != (int *)0x0) {
          FUN_14019f2c0(piVar8);
        }
      }
      else {
        if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
          iVar15 = *(int *)(local_a8 + -8);
          goto LAB_142e5363e;
        }
        if (*piVar8 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar8 = -1;
      }
      FUN_142ef7ba0(local_a8,lVar2,piVar12);
      pcVar1 = local_a8;
      if (*(int *)(local_a8 + -0x10) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
        if (iVar10 != -1) goto LAB_142e536d5;
        piVar12 = piVar13;
        if (pcVar1 != (char *)0x0) {
          piVar12 = (int *)0xffffffffffffffff;
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
          } while (pcVar1[(longlong)piVar12] != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
LAB_142e536d5:
        *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
      }
      iVar10 = (int)piVar12;
      if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
        FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
      }
      *(int *)(pcVar1 + -8) = iVar10;
    }
  }
LAB_142e536fd:
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  FUN_1408bc140(&local_90,local_res20);
  lVar2 = local_90;
  pcVar1 = local_a8;
  local_res18 = 0x6df;
  if (local_90 != 0) {
    iVar10 = *(int *)(local_90 + -8);
    piVar12 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar8 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e538ca;
      if (*local_a8 == '\0') {
        if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0))
        {
LAB_142e538ca:
          if (iVar15 < iVar10) {
            iVar15 = iVar10;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
          puVar5[1] = iVar15;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          puVar5[2] = 0;
          *local_a8 = '\0';
          if (piVar8 != (int *)0x0) {
            FUN_14019f2c0(piVar8);
          }
        }
        else {
          if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
            iVar15 = *(int *)(local_a8 + -8);
            goto LAB_142e538ca;
          }
          if (*piVar8 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar8 = -1;
        }
        FUN_142ef7ba0(local_a8,lVar2,piVar12);
        pcVar1 = local_a8;
        if (*(int *)(local_a8 + -0x10) != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
          if (iVar10 != -1) goto LAB_142e5395e;
          piVar12 = piVar13;
          if (pcVar1 != (char *)0x0) {
            piVar12 = (int *)0xffffffffffffffff;
            do {
              piVar12 = (int *)((longlong)piVar12 + 1);
            } while (pcVar1[(longlong)piVar12] != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
LAB_142e5395e:
          *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
        }
        iVar10 = (int)piVar12;
        if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
          FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
        }
        *(int *)(pcVar1 + -8) = iVar10;
        goto LAB_142e53989;
      }
      iVar15 = *(int *)(local_a8 + -8);
      for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
      }
      piVar8 = (int *)(local_a8 + -0x10);
      iVar14 = iVar9;
      if (piVar8 == (int *)0x0) {
LAB_142e537cf:
        if (iVar14 < iVar7) {
          iVar14 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
        puVar5[1] = iVar14;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        if (piVar8 == (int *)0x0) {
          puVar5[2] = 0;
          *local_a8 = '\0';
        }
        else {
          iVar7 = *(int *)(pcVar1 + -8) + 1;
          if (iVar14 + 1 < iVar7) {
            FUN_142e54290(0x5c,iVar7,iVar14 + 1);
            iVar7 = iVar14 + 1;
          }
          FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
          puVar5[2] = *(undefined4 *)(pcVar1 + -8);
          local_a8[iVar14] = '\0';
          FUN_14019f2c0(piVar8);
        }
      }
      else {
        if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar7)) {
          iVar14 = *(int *)(local_a8 + -8);
          goto LAB_142e537cf;
        }
        if (*piVar8 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar8 = -1;
      }
      iVar7 = iVar9;
      if (local_a8 != (char *)0x0) {
        iVar7 = *(int *)(local_a8 + -8);
      }
      FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
      FUN_14019c870(&local_a8,iVar15 + iVar10);
    }
  }
LAB_142e53989:
  if (local_90 != 0) {
    FUN_14019f2c0(local_90 + -0x10);
  }
  local_60 = 0;
  uVar4 = FUN_14019ba10(&local_60,&DAT_143272338,&DAT_1434997f8);
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x26df;
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  lVar2 = local_a0;
  pcVar1 = local_a8;
  local_res18 = 0x1edf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    piVar12 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar8 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e53b90;
      if (*local_a8 != '\0') {
        iVar15 = *(int *)(local_a8 + -8);
        for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
        }
        piVar8 = (int *)(local_a8 + -0x10);
        iVar14 = iVar9;
        if (piVar8 == (int *)0x0) {
LAB_142e53a91:
          if (iVar14 < iVar7) {
            iVar14 = iVar7;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
          puVar5[1] = iVar14;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          if (piVar8 == (int *)0x0) {
            puVar5[2] = 0;
            *local_a8 = '\0';
          }
          else {
            iVar7 = *(int *)(pcVar1 + -8) + 1;
            if (iVar14 + 1 < iVar7) {
              FUN_142e54290(0x5c,iVar7,iVar14 + 1);
              iVar7 = iVar14 + 1;
            }
            FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
            puVar5[2] = *(undefined4 *)(pcVar1 + -8);
            local_a8[iVar14] = '\0';
            FUN_14019f2c0(piVar8);
          }
        }
        else {
          if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar7)) {
            iVar14 = *(int *)(local_a8 + -8);
            goto LAB_142e53a91;
          }
          if (*piVar8 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar8 = -1;
        }
        iVar7 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar7 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
        FUN_14019c870(&local_a8,iVar15 + iVar10);
        goto LAB_142e53c4f;
      }
      if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0)) {
LAB_142e53b90:
        if (iVar15 < iVar10) {
          iVar15 = iVar10;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
        puVar5[1] = iVar15;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        puVar5[2] = 0;
        *local_a8 = '\0';
        if (piVar8 != (int *)0x0) {
          FUN_14019f2c0(piVar8);
        }
      }
      else {
        if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
          iVar15 = *(int *)(local_a8 + -8);
          goto LAB_142e53b90;
        }
        if (*piVar8 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar8 = -1;
      }
      FUN_142ef7ba0(local_a8,lVar2,piVar12);
      pcVar1 = local_a8;
      if (*(int *)(local_a8 + -0x10) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
        if (iVar10 != -1) goto LAB_142e53c27;
        piVar12 = piVar13;
        if (pcVar1 != (char *)0x0) {
          piVar12 = (int *)0xffffffffffffffff;
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
          } while (pcVar1[(longlong)piVar12] != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
LAB_142e53c27:
        *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
      }
      iVar10 = (int)piVar12;
      if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
        FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
      }
      *(int *)(pcVar1 + -8) = iVar10;
    }
  }
LAB_142e53c4f:
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  FUN_1408bc020(&local_88,local_res8);
  lVar2 = local_88;
  pcVar1 = local_a8;
  local_res18 = 0xdedf;
  if (local_88 == 0) goto LAB_142e53edb;
  iVar10 = *(int *)(local_88 + -8);
  piVar12 = (int *)(longlong)iVar10;
  if (iVar10 == 0) goto LAB_142e53edb;
  piVar8 = piVar13;
  iVar15 = iVar9;
  if (local_a8 == (char *)0x0) goto LAB_142e53e1c;
  if (*local_a8 == '\0') {
    if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0)) {
LAB_142e53e1c:
      if (iVar15 < iVar10) {
        iVar15 = iVar10;
      }
      puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
      puVar5[1] = iVar15;
      *puVar5 = 0xffffffff;
      local_a8 = (char *)(puVar5 + 4);
      puVar5[2] = 0;
      *local_a8 = '\0';
      if (piVar8 != (int *)0x0) {
        FUN_14019f2c0(piVar8);
      }
    }
    else {
      if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
        iVar15 = *(int *)(local_a8 + -8);
        goto LAB_142e53e1c;
      }
      if (*piVar8 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar8 = -1;
    }
    piVar8 = (int *)0xffffffffffffffff;
    FUN_142ef7ba0(local_a8,lVar2,piVar12);
    pcVar1 = local_a8;
    if (*(int *)(local_a8 + -0x10) != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
      pcVar1[-0x10] = '\x01';
      pcVar1[-0xf] = '\0';
      pcVar1[-0xe] = '\0';
      pcVar1[-0xd] = '\0';
      if (iVar10 != -1) goto LAB_142e53eb0;
      if (pcVar1 != (char *)0x0) {
        do {
          piVar13 = (int *)((longlong)piVar8 + 1);
          piVar8 = piVar13;
        } while (pcVar1[(longlong)piVar13] != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
      pcVar1[-0x10] = '\x01';
      pcVar1[-0xf] = '\0';
      pcVar1[-0xe] = '\0';
      pcVar1[-0xd] = '\0';
LAB_142e53eb0:
      *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
      piVar13 = piVar12;
    }
    iVar10 = (int)piVar13;
    if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
      FUN_142e54290(0x9c,(ulonglong)piVar13 & 0xffffffff);
    }
    *(int *)(pcVar1 + -8) = iVar10;
    goto LAB_142e53edb;
  }
  iVar15 = *(int *)(local_a8 + -8);
  for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
  }
  piVar13 = (int *)(local_a8 + -0x10);
  iVar14 = iVar9;
  if (piVar13 == (int *)0x0) {
LAB_142e53d21:
    if (iVar14 < iVar7) {
      iVar14 = iVar7;
    }
    puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
    puVar5[1] = iVar14;
    *puVar5 = 0xffffffff;
    local_a8 = (char *)(puVar5 + 4);
    if (piVar13 == (int *)0x0) {
      puVar5[2] = 0;
      *local_a8 = '\0';
    }
    else {
      iVar7 = *(int *)(pcVar1 + -8) + 1;
      if (iVar14 + 1 < iVar7) {
        FUN_142e54290(0x5c,iVar7,iVar14 + 1);
        iVar7 = iVar14 + 1;
      }
      FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
      puVar5[2] = *(undefined4 *)(pcVar1 + -8);
      local_a8[iVar14] = '\0';
      FUN_14019f2c0(piVar13);
    }
  }
  else {
    if ((1 < *piVar13) || (*(int *)(local_a8 + -0xc) < iVar7)) {
      iVar14 = *(int *)(local_a8 + -8);
      goto LAB_142e53d21;
    }
    if (*piVar13 != 1) {
      FUN_142e52dd0(0x74);
    }
    *piVar13 = -1;
  }
  iVar7 = iVar9;
  if (local_a8 != (char *)0x0) {
    iVar7 = *(int *)(local_a8 + -8);
  }
  FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
  FUN_14019c870(&local_a8,iVar15 + iVar10);
LAB_142e53edb:
  if (local_88 != 0) {
    FUN_14019f2c0(local_88 + -0x10);
  }
  local_58 = 0;
  uVar4 = FUN_14019ba10(&local_58,&DAT_143272338,"Info1");
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x4dedf;
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  lVar2 = local_a0;
  local_res18 = 0x3dedf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    lVar16 = (longlong)iVar10;
    if (iVar10 != 0) {
      if ((local_a8 == (char *)0x0) || (*local_a8 == '\0')) {
        uVar4 = FUN_14019bd40(&local_a8,iVar10,0);
        FUN_142ef7ba0(uVar4,lVar2,lVar16);
      }
      else {
        iVar10 = *(int *)(local_a8 + -8) + iVar10;
        for (iVar15 = *(int *)(local_a8 + -0xc); iVar15 < iVar10; iVar15 = iVar15 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_a8,iVar15,1);
        iVar15 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar15 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(iVar15 + lVar6,lVar2,lVar16);
      }
      FUN_14019c870(&local_a8,iVar10);
    }
  }
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  FUN_1408bc3a0(&local_80,&local_res10);
  lVar2 = local_80;
  local_res18 = 0x1bdedf;
  if (local_80 != 0) {
    iVar10 = *(int *)(local_80 + -8);
    lVar16 = (longlong)iVar10;
    if (iVar10 != 0) {
      if ((local_a8 == (char *)0x0) || (*local_a8 == '\0')) {
        uVar4 = FUN_14019bd40(&local_a8,iVar10,0);
        FUN_142ef7ba0(uVar4,lVar2,lVar16);
      }
      else {
        iVar10 = *(int *)(local_a8 + -8) + iVar10;
        for (iVar15 = *(int *)(local_a8 + -0xc); iVar15 < iVar10; iVar15 = iVar15 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_a8,iVar15,1);
        iVar15 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar15 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(iVar15 + lVar6,lVar2,lVar16);
      }
      FUN_14019c870(&local_a8,iVar10);
    }
  }
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  FUN_1408bc980(&local_a0,&local_78);
  lVar2 = local_a0;
  local_res18 = 0x7bdedf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    lVar16 = (longlong)iVar10;
    if (iVar10 != 0) {
      if ((local_a8 == (char *)0x0) || (*local_a8 == '\0')) {
        uVar4 = FUN_14019bd40(&local_a8,iVar10,0);
        FUN_142ef7ba0(uVar4,lVar2,lVar16);
      }
      else {
        iVar10 = *(int *)(local_a8 + -8) + iVar10;
        for (iVar15 = *(int *)(local_a8 + -0xc); iVar15 < iVar10; iVar15 = iVar15 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_a8,iVar15,1);
        if (local_a8 != (char *)0x0) {
          iVar9 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(iVar9 + lVar6,lVar2,lVar16);
      }
      FUN_14019c870(&local_a8,iVar10);
    }
  }
  if (local_a0 != 0) {
    FUN_14019f2c0(local_a0 + -0x10);
  }
  FUN_142a1ec10(&local_a8);
  if (local_a8 != (char *)0x0) {
    FUN_14019f2c0(local_a8 + -0x10);
  }
  if (local_50[0] != 0) {
    FUN_14019f2c0(local_50[0] + -0x10);
  }
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  return;
}



//===========================================================
// FUN_1401bebb0 @ 1401bebb0   (347 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1401bebb0(int *param_1)

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
  if (uVar3 < 0x61) {
    uVar6 = (uint)(0x40 < uVar3);
  }
  else {
    if (uVar3 < 0xa1) {
      uVar6 = 2;
      goto LAB_1401bec24;
    }
    uVar6 = 0xffffffff;
    if (uVar3 < 0x121) {
      uVar6 = 3;
    }
  }
  if ((int)uVar6 < 0) {
    uVar4 = (*DAT_143ad5538)();
                    /* WARNING: Could not recover jumptable at 0x0001401bec98. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(uVar4,0,param_1 + -2);
    return;
  }
LAB_1401bec24:
  uVar3 = (ulonglong)uVar6;
  lVar7 = (ulonglong)uVar6 * 0x10;
  LOCK();
  bVar8 = *(longlong *)(&DAT_143ad69a8 + lVar7) == 0;
  if (bVar8) {
    *(void **)(&DAT_143ad69a8 + lVar7) = Self;
  }
  UNLOCK();
  if (bVar8) {
LAB_1401becc5:
    *(undefined4 *)(&DAT_143ad69b0 + lVar7) = 1;
  }
  else {
    if (*(void **)(&DAT_143ad69a8 + lVar7) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar8 = *(longlong *)(&DAT_143ad69a8 + lVar7) == 0;
        if (bVar8) {
          *(void **)(&DAT_143ad69a8 + lVar7) = Self;
        }
        UNLOCK();
        if (bVar8) goto LAB_1401becc5;
        if (*(void **)(&DAT_143ad69a8 + lVar7) == pvVar2) break;
        (*DAT_143262828)(0);
      }
    }
    *(int *)(&DAT_143ad69b0 + lVar7) = *(int *)(&DAT_143ad69b0 + lVar7) + 1;
  }
  piVar5 = (int *)(&DAT_143ad69b0 + lVar7);
  *(undefined8 *)param_1 = *(undefined8 *)(&DAT_143ad69e8 + uVar3 * 8);
  *(int **)(&DAT_143ad69e8 + uVar3 * 8) = param_1;
  _DAT_143ad6a28 = *(undefined8 *)param_1;
  *(int *)(&DAT_143ad6994 + uVar3 * 4) = *(int *)(&DAT_143ad6994 + uVar3 * 4) + -1;
  *piVar5 = *piVar5 + -1;
  if (*piVar5 == 0) {
    *(undefined8 *)(&DAT_143ad69a8 + lVar7) = 0;
  }
  return;
}



//===========================================================
// FUN_141711e00 @ 141711e00   (4 bytes)
//===========================================================

undefined4 FUN_141711e00(longlong param_1)

{
  return *(undefined4 *)(param_1 + 0x48);
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
// FUN_142e5cd30 @ 142e5cd30   (764 bytes)
//===========================================================

void FUN_142e5cd30(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                  undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8)

{
  undefined8 uVar1;
  undefined1 local_84;
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
  longlong local_28;
  longlong local_20;
  
  uVar1 = FUN_142a1d8a0(&local_20);
  local_80 = 0;
  FUN_140198700(&local_80,uVar1,local_84);
  local_58 = 0;
  uVar1 = FUN_14019ba10(&local_58,&DAT_143272338,param_1);
  local_78 = 0;
  FUN_14019a260(&local_78,uVar1);
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_78);
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  local_50 = 0;
  uVar1 = FUN_14019ba10(&local_50,&DAT_143272338,param_2);
  local_70 = 0;
  FUN_14019a260(&local_70,uVar1);
  if (local_50 != 0) {
    FUN_14019f2c0(local_50 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_70);
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  FUN_1408bc140(&local_48,param_3);
  FUN_1401a1c50(&local_80,&local_48);
  if (local_48 != 0) {
    FUN_14019f2c0(local_48 + -0x10);
  }
  local_40 = 0;
  uVar1 = FUN_14019ba10(&local_40,&DAT_143272338,param_4);
  local_68 = 0;
  FUN_14019a260(&local_68,uVar1);
  if (local_40 != 0) {
    FUN_14019f2c0(local_40 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_68);
  if (local_68 != 0) {
    FUN_14019f2c0(local_68 + -0x10);
  }
  FUN_1408bc020(&local_38,param_5);
  FUN_1401a1c50(&local_80,&local_38);
  if (local_38 != 0) {
    FUN_14019f2c0(local_38 + -0x10);
  }
  local_30 = 0;
  uVar1 = FUN_14019ba10(&local_30,&DAT_143272338,param_6);
  local_60 = 0;
  FUN_14019a260(&local_60,uVar1);
  if (local_30 != 0) {
    FUN_14019f2c0(local_30 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_60);
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  FUN_1408bc280(&local_28,param_7);
  FUN_1401a1c50(&local_80,&local_28);
  if (local_28 != 0) {
    FUN_14019f2c0(local_28 + -0x10);
  }
  FUN_140198700(&local_80,param_8,local_84);
  FUN_142a1ec10(&local_80);
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  if (local_20 != 0) {
    FUN_14019f2c0(local_20 + -0x10);
  }
  return;
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
// FUN_1401bd8a0 @ 1401bd8a0   (178 bytes)
//===========================================================

void FUN_1401bd8a0(longlong *param_1,int param_2)

{
  longlong lVar1;
  int iVar2;
  ulonglong uVar3;
  
  lVar1 = *param_1;
  uVar3 = (ulonglong)param_2;
  if (*(int *)(lVar1 + -0x10) != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((param_2 == -1) || (param_2 <= *(int *)(lVar1 + -0xc))) {
    *(undefined4 *)(lVar1 + -0x10) = 1;
    if (param_2 != -1) goto LAB_1401bd8ea;
    if (lVar1 != 0) {
      uVar3 = 0xffffffffffffffff;
      do {
        uVar3 = uVar3 + 1;
      } while (*(short *)(lVar1 + uVar3 * 2) != 0);
      goto LAB_1401bd8f3;
    }
    uVar3 = 0;
LAB_1401bd8f7:
    iVar2 = (int)uVar3;
    if (iVar2 < *(int *)(lVar1 + -0xc) + 1) goto LAB_1401bd910;
  }
  else {
    FUN_142e54290(0x90,*(int *)(lVar1 + -0xc),param_2);
    *(undefined4 *)(lVar1 + -0x10) = 1;
LAB_1401bd8ea:
    *(undefined2 *)(*param_1 + uVar3 * 2) = 0;
LAB_1401bd8f3:
    if (-1 < (int)uVar3) goto LAB_1401bd8f7;
  }
  iVar2 = (int)uVar3;
  FUN_142e54290(0x9c,uVar3 & 0xffffffff,*(undefined4 *)(lVar1 + -0xc));
LAB_1401bd910:
  *(int *)(lVar1 + -8) = iVar2 * 2;
  return;
}



//===========================================================
// FUN_1401bc720 @ 1401bc720   (378 bytes)
//===========================================================

void FUN_1401bc720(longlong param_1,ulonglong param_2)

{
  longlong *plVar1;
  int *piVar2;
  void *pvVar3;
  longlong lVar4;
  undefined8 *puVar5;
  undefined8 uVar6;
  int iVar7;
  longlong lVar8;
  int *piVar9;
  uint uVar10;
  longlong lVar11;
  
  pvVar3 = Self;
  uVar6 = 0x120;
  if (param_2 < 0x61) {
    uVar10 = (uint)(0x40 < param_2);
LAB_1401bc782:
    if ((int)uVar10 < 0) {
      FUN_14019d350(param_2);
      return;
    }
    if (uVar10 == 0) {
      iVar7 = 0x40;
      uVar6 = 0x40;
      goto LAB_1401bc7c9;
    }
    if (uVar10 == 1) {
      iVar7 = 0x20;
      uVar6 = 0x60;
      goto LAB_1401bc7c9;
    }
    if (uVar10 != 2) {
      if (uVar10 == 3) {
        iVar7 = 8;
      }
      else {
        iVar7 = 0;
        uVar6 = 0;
      }
      goto LAB_1401bc7c9;
    }
  }
  else {
    if (0xa0 < param_2) {
      uVar10 = 0xffffffff;
      if (param_2 < 0x121) {
        uVar10 = 3;
      }
      goto LAB_1401bc782;
    }
    uVar10 = 2;
  }
  iVar7 = 0x10;
  uVar6 = 0xa0;
LAB_1401bc7c9:
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
LAB_1401bc829:
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
      if (lVar4 == 0) goto LAB_1401bc829;
      if ((void *)*plVar1 == pvVar3) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  piVar9 = (int *)(lVar8 + 0x30);
  puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  if (puVar5 == (undefined8 *)0x0) {
    lVar4 = FUN_14019d3c0(uVar6,iVar7);
    *(undefined8 *)(lVar4 + -0x10) = *(undefined8 *)(param_1 + 0x88 + lVar11 * 8);
    *(longlong *)(param_1 + 0x88 + lVar11 * 8) = lVar4;
    *(longlong *)(param_1 + 0x68 + lVar11 * 8) = lVar4;
    piVar2 = (int *)(param_1 + 4 + lVar11 * 4);
    *piVar2 = *piVar2 + iVar7;
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
// FUN_1401c1fb0 @ 1401c1fb0   (518 bytes)
//===========================================================

longlong * FUN_1401c1fb0(longlong *param_1,longlong *param_2)

{
  void *_Buf1;
  void *_Buf2;
  longlong lVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  int *piVar5;
  int *piVar6;
  int *piVar7;
  ulonglong uVar8;
  int iVar9;
  
  if (param_1 == param_2) {
    return param_1;
  }
  _Buf1 = (void *)*param_1;
  piVar7 = (int *)0x0;
  iVar9 = 0;
  iVar2 = iVar9;
  if (_Buf1 != (void *)0x0) {
    iVar2 = (int)((ulonglong)(longlong)*(int *)((longlong)_Buf1 + -8) >> 1);
  }
  _Buf2 = (void *)*param_2;
  iVar4 = iVar9;
  if (_Buf2 != (void *)0x0) {
    iVar4 = (int)((ulonglong)(longlong)*(int *)((longlong)_Buf2 + -8) >> 1);
  }
  if ((((iVar2 == iVar4) && (iVar2 != 0)) && (_Buf1 != (void *)0x0)) &&
     ((_Buf2 != (void *)0x0 && (iVar2 = memcmp(_Buf1,_Buf2,(longlong)iVar2 * 2), iVar2 == 0)))) {
    return param_1;
  }
  piVar5 = (int *)((longlong)_Buf2 + -0x10);
  if (_Buf2 == (void *)0x0) {
    piVar5 = piVar7;
  }
  if (piVar5 == (int *)0x0) {
    if (_Buf1 == (void *)0x0) {
      return param_1;
    }
    FUN_1401bebb0((longlong)_Buf1 + -0x10);
    *param_1 = 0;
    return param_1;
  }
  if (*piVar5 != -1) {
    if (*piVar5 < 1) {
      FUN_142e52dd0(0xd2);
    }
    LOCK();
    *piVar5 = *piVar5 + 1;
    UNLOCK();
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
    }
    *param_1 = (longlong)(piVar5 + 4);
    return param_1;
  }
  FUN_142e52d50(0xcb,0xffffff01);
  lVar1 = *param_2;
  piVar5 = piVar7;
  if (lVar1 == 0) goto LAB_1401c2137;
  uVar8 = 0xffffffffffffffff;
  piVar6 = (int *)0xffffffffffffffff;
  do {
    piVar6 = (int *)((longlong)piVar6 + 1);
  } while (*(short *)(lVar1 + (longlong)piVar6 * 2) != 0);
  iVar2 = (int)piVar6;
  if (0 < iVar2) {
    iVar9 = iVar2;
  }
  piVar3 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar9 * 2 + 0x12));
  piVar3[1] = iVar9;
  *piVar3 = -1;
  piVar5 = piVar3 + 4;
  piVar3[2] = 0;
  *(undefined2 *)piVar5 = 0;
  FUN_142ef7ba0(piVar5,lVar1,(longlong)iVar2 * 2);
  if (*piVar3 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar2 == -1) || (iVar2 <= piVar3[1])) {
    *piVar3 = 1;
    if (iVar2 != -1) goto LAB_1401c2110;
    if (piVar5 != (int *)0x0) {
      do {
        uVar8 = uVar8 + 1;
      } while (*(short *)((longlong)piVar5 + uVar8 * 2) != 0);
      piVar7 = (int *)(uVar8 & 0xffffffff);
    }
  }
  else {
    FUN_142e54290(0x90,piVar3[1],(ulonglong)piVar6 & 0xffffffff);
    *piVar3 = 1;
LAB_1401c2110:
    *(undefined2 *)((longlong)piVar5 + (longlong)iVar2 * 2) = 0;
    piVar7 = piVar6;
  }
  iVar2 = (int)piVar7;
  if ((iVar2 < 0) || (piVar3[1] + 1 <= iVar2)) {
    FUN_142e54290(0x9c,(ulonglong)piVar7 & 0xffffffff);
  }
  piVar3[2] = iVar2 * 2;
LAB_1401c2137:
  if (*param_1 != 0) {
    FUN_1401bebb0(*param_1 + -0x10);
  }
  *param_1 = (longlong)piVar5;
  return param_1;
}



//===========================================================
// FUN_142e5d030 @ 142e5d030   (509 bytes)
//===========================================================

void FUN_142e5d030(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                  undefined8 param_5,undefined8 param_6)

{
  undefined8 uVar1;
  undefined1 local_68;
  longlong local_60;
  longlong local_58;
  longlong local_50;
  longlong local_48;
  longlong local_40;
  longlong local_38;
  longlong local_30;
  longlong local_28 [2];
  
  uVar1 = FUN_142a1d8a0(local_28);
  local_60 = 0;
  FUN_140198700(&local_60,uVar1,local_68);
  FUN_1408b6250(&local_60,param_1,local_68);
  local_48 = 0;
  uVar1 = FUN_14019ba10(&local_48,&DAT_143272338,param_2);
  local_58 = 0;
  FUN_14019a260(&local_58,uVar1);
  if (local_48 != 0) {
    FUN_14019f2c0(local_48 + -0x10);
  }
  FUN_1401a1c50(&local_60,&local_58);
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  FUN_1408bc140(&local_40,param_3);
  FUN_1401a1c50(&local_60,&local_40);
  if (local_40 != 0) {
    FUN_14019f2c0(local_40 + -0x10);
  }
  local_38 = 0;
  uVar1 = FUN_14019ba10(&local_38,&DAT_143272338,param_4);
  local_50 = 0;
  FUN_14019a260(&local_50,uVar1);
  if (local_38 != 0) {
    FUN_14019f2c0(local_38 + -0x10);
  }
  FUN_1401a1c50(&local_60,&local_50);
  if (local_50 != 0) {
    FUN_14019f2c0(local_50 + -0x10);
  }
  FUN_1408bc020(&local_30,param_5);
  FUN_1401a1c50(&local_60,&local_30);
  if (local_30 != 0) {
    FUN_14019f2c0(local_30 + -0x10);
  }
  FUN_140198700(&local_60,param_6,local_68);
  FUN_142a1ec10(&local_60);
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  if (local_28[0] != 0) {
    FUN_14019f2c0(local_28[0] + -0x10);
  }
  return;
}



//===========================================================
// FUN_140196ed0 @ 140196ed0   (166 bytes)
//===========================================================

undefined8 * FUN_140196ed0(undefined8 *param_1,longlong param_2,ulonglong param_3)

{
  undefined4 *puVar1;
  int iVar2;
  ulonglong uVar3;
  int iVar4;
  
  *param_1 = 0;
  uVar3 = param_3 & 0xffffffff;
  if (param_2 != 0) {
    if ((int)param_3 == -1) {
      uVar3 = 0xffffffffffffffff;
      do {
        uVar3 = uVar3 + 1;
      } while (*(char *)(param_2 + uVar3) != '\0');
    }
    iVar2 = (int)uVar3;
    iVar4 = 0;
    if (0 < iVar2) {
      iVar4 = iVar2;
    }
    puVar1 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
    puVar1[1] = iVar4;
    *puVar1 = 0xffffffff;
    *param_1 = puVar1 + 4;
    puVar1[2] = 0;
    *(undefined1 *)*param_1 = 0;
    FUN_142ef7ba0(*param_1,param_2,(longlong)iVar2);
    FUN_14019c870(param_1,uVar3 & 0xffffffff);
  }
  return param_1;
}



//===========================================================
// FUN_140cbca60 @ 140cbca60   (113 bytes)
//===========================================================

void FUN_140cbca60(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 *puVar3;
  
  lVar2 = *(longlong *)(param_1 + 8);
  if (lVar2 != 0) {
    if (0xffffe < *(longlong *)(lVar2 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + 0x20);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar2 == 1) {
      puVar3 = (undefined8 *)(*(longlong *)(param_1 + 8) + 0x18);
      if (*(longlong *)(param_1 + 8) == 0) {
        puVar3 = (undefined8 *)0x0;
      }
      if (puVar3 != (undefined8 *)0x0) {
        (**(code **)*puVar3)(puVar3,1);
      }
    }
    *(undefined8 *)(param_1 + 8) = 0;
  }
  return;
}



//===========================================================
// FUN_14126acd0 @ 14126acd0   (142 bytes)
//===========================================================

longlong FUN_14126acd0(longlong param_1,longlong param_2)

{
  longlong lVar1;
  
  if ((*(longlong *)(param_1 + 8) - 1U < 999) || (*(longlong *)(param_1 + 8) == -1)) {
    FUN_142e52ed0(0x447);
  }
  if (param_1 == param_2) {
    FUN_142e52d50(0x45c,1);
  }
  lVar1 = *(longlong *)(param_2 + 8);
  if (lVar1 != 0) {
    if (0xfffff < *(ulonglong *)(lVar1 + 0x20)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar1 + 0x20) = *(longlong *)(lVar1 + 0x20) + 1;
    UNLOCK();
  }
  FUN_14126b0d0(param_1);
  *(undefined8 *)(param_1 + 8) = *(undefined8 *)(param_2 + 8);
  return param_1;
}



//===========================================================
// FUN_141afab70 @ 141afab70   (322 bytes)
//===========================================================

longlong FUN_141afab70(longlong *param_1,uint param_2)

{
  longlong lVar1;
  int iVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  uint uVar6;
  uint uVar7;
  
  lVar1 = *param_1;
  if (lVar1 == 0) {
    uVar7 = 0;
  }
  else {
    uVar7 = *(uint *)(lVar1 + -8);
  }
  if (param_2 == 0xffffffff) {
    param_2 = uVar7;
  }
  if (lVar1 != 0) {
    uVar5 = *(ulonglong *)(lVar1 + -0x10);
    uVar3 = ~uVar5;
    if (-1 < (longlong)uVar5) {
      uVar3 = uVar5;
    }
    if (uVar7 < (uint)(uVar3 - 8 >> 4)) goto LAB_141afac63;
  }
  if (uVar7 == 0) {
    uVar5 = 1;
  }
  else {
    uVar5 = (ulonglong)(uVar7 * 2);
  }
  if (lVar1 == 0) {
    iVar2 = 0;
  }
  else {
    uVar3 = *(ulonglong *)(lVar1 + -0x10);
    uVar4 = ~uVar3;
    if (-1 < (longlong)uVar3) {
      uVar4 = uVar3;
    }
    iVar2 = (int)(uVar4 - 8 >> 4);
  }
  if (iVar2 != (int)uVar5) {
    if (lVar1 == 0) {
      uVar6 = 0;
    }
    else {
      uVar6 = *(uint *)(lVar1 + -8);
    }
    lVar1 = FUN_14019b780(&DAT_143ad68a0,uVar5 * 0x10 + 8);
    if (lVar1 == 0) {
      lVar1 = 0;
    }
    else {
      lVar1 = lVar1 + 8;
    }
    if (*param_1 != 0) {
      FUN_142ef7ba0(lVar1,*param_1,(ulonglong)uVar6 << 4);
      thunk_FUN_140205820(*param_1 + -8,0);
    }
    *param_1 = lVar1;
    *(ulonglong *)(lVar1 + -8) = (ulonglong)uVar6;
  }
LAB_141afac63:
  lVar1 = (longlong)(int)param_2 * 0x10;
  *(longlong *)(*param_1 + -8) = *(longlong *)(*param_1 + -8) + 1;
  FUN_142ef7ba0(*param_1 + lVar1 + 0x10,*param_1 + lVar1,(ulonglong)(uVar7 - param_2) << 4);
  *(undefined8 *)(lVar1 + 8 + *param_1) = 0;
  return *param_1 + lVar1;
}



//===========================================================
// FUN_14126b0d0 @ 14126b0d0   (113 bytes)
//===========================================================

void FUN_14126b0d0(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 *puVar3;
  
  lVar2 = *(longlong *)(param_1 + 8);
  if (lVar2 != 0) {
    if (0xffffe < *(longlong *)(lVar2 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + 0x20);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar2 == 1) {
      puVar3 = (undefined8 *)(*(longlong *)(param_1 + 8) + 0x18);
      if (*(longlong *)(param_1 + 8) == 0) {
        puVar3 = (undefined8 *)0x0;
      }
      if (puVar3 != (undefined8 *)0x0) {
        (**(code **)*puVar3)(puVar3,1);
      }
    }
    *(undefined8 *)(param_1 + 8) = 0;
  }
  return;
}



//===========================================================
// FUN_141af8580 @ 141af8580   (581 bytes)
//===========================================================

undefined8 * FUN_141af8580(longlong *param_1,uint *param_2)

{
  uint uVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  void *pvVar5;
  uint uVar6;
  longlong lVar7;
  undefined8 *puVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong *plVar11;
  int *piVar12;
  uint *puVar13;
  uint uVar14;
  
  plVar2 = (longlong *)*param_1;
  if (plVar2 == (longlong *)0x0) {
    uVar14 = *(uint *)(param_1 + 1);
    uVar1 = uVar14;
  }
  else {
    if (*(uint *)((longlong)param_1 + 0xc) <= *(uint *)((longlong)param_1 + 0x14))
    goto LAB_141af86be;
    uVar14 = *(uint *)(param_1 + 1);
    uVar1 = uVar14 * 2;
  }
  if ((uVar1 != 0) && ((uVar14 != uVar1 || (plVar2 == (longlong *)0x0)))) {
    puVar13 = (uint *)&DAT_1433f9c40;
    uVar10 = 0xf6;
    do {
      uVar9 = uVar10 >> 1;
      if (puVar13[uVar9] < uVar1) {
        puVar13 = puVar13 + uVar9 + 1;
        uVar9 = uVar10 + (-1 - uVar9);
      }
      uVar10 = uVar9;
    } while (0 < (longlong)uVar9);
    uVar1 = *puVar13;
    *(uint *)(param_1 + 1) = uVar1;
    uVar6 = 0xffffffff;
    if ((int)param_1[2] != -1) {
      uVar6 = (int)param_1[2] * uVar1 >> 7;
    }
    *(uint *)((longlong)param_1 + 0x14) = uVar6;
    lVar7 = FUN_14019b780(&DAT_143ad68a0);
    *param_1 = lVar7;
    FUN_142ef8250(lVar7,0,(ulonglong)uVar1 * 8);
    plVar11 = plVar2;
    if (plVar2 != (longlong *)0x0) {
      while (plVar11 < plVar2 + uVar14) {
        lVar3 = *plVar11;
        plVar11 = plVar11 + 1;
        while (lVar3 != 0) {
          uVar10 = (ulonglong)*(uint *)(lVar3 + 0x10) % (ulonglong)uVar1;
          lVar4 = *(longlong *)(lVar3 + 8);
          *(undefined8 *)(lVar3 + 8) = *(undefined8 *)(lVar7 + uVar10 * 8);
          *(longlong *)(lVar7 + uVar10 * 8) = lVar3;
          lVar3 = lVar4;
        }
      }
      FUN_14019b4e0(plVar2);
    }
  }
LAB_141af86be:
  plVar2 = (longlong *)(*param_1 + ((ulonglong)*param_2 % (ulonglong)*(uint *)(param_1 + 1)) * 8);
  for (puVar8 = (undefined8 *)*plVar2; puVar8 != (undefined8 *)0x0; puVar8 = (undefined8 *)puVar8[1]
      ) {
    if (*(uint *)(puVar8 + 2) == *param_2) {
      return puVar8;
    }
  }
  *(int *)((longlong)param_1 + 0xc) = *(int *)((longlong)param_1 + 0xc) + 1;
  pvVar5 = Self;
  lVar3 = DAT_143ad1e58;
  plVar11 = (longlong *)(DAT_143ad1e58 + 0x18);
  LOCK();
  lVar7 = *plVar11;
  if (lVar7 == 0) {
    *plVar11 = (longlong)Self;
  }
  UNLOCK();
  if (lVar7 == 0) {
LAB_141af8759:
    *(undefined4 *)(lVar3 + 0x20) = 1;
  }
  else if ((void *)*plVar11 == pvVar5) {
    *(int *)(lVar3 + 0x20) = *(int *)(lVar3 + 0x20) + 1;
  }
  else {
    while( true ) {
      pvVar5 = Self;
      LOCK();
      lVar7 = *plVar11;
      if (lVar7 == 0) {
        *plVar11 = (longlong)Self;
      }
      UNLOCK();
      if (lVar7 == 0) goto LAB_141af8759;
      if ((void *)*plVar11 == pvVar5) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar3 + 0x20) = *(int *)(lVar3 + 0x20) + 1;
  }
  piVar12 = (int *)(lVar3 + 0x20);
  puVar8 = *(undefined8 **)(lVar3 + 0x28);
  if (puVar8 == (undefined8 *)0x0) {
    puVar8 = (undefined8 *)FUN_14019d3c0(0x28);
    *(undefined8 **)(lVar3 + 0x28) = puVar8;
  }
  *(undefined8 *)(lVar3 + 0x28) = *puVar8;
  *piVar12 = *piVar12 + -1;
  if (*piVar12 == 0) {
    *plVar11 = 0;
  }
  lVar7 = *plVar2;
  *puVar8 = &PTR_FUN_1433fb8c0;
  puVar8[1] = lVar7;
  *(undefined4 *)(puVar8 + 2) = 0;
  puVar8[4] = 0;
  *(uint *)(puVar8 + 2) = *param_2;
  *plVar2 = (longlong)puVar8;
  return puVar8;
}



//===========================================================
// FUN_14019b780 @ 14019b780   (374 bytes)
//===========================================================

void FUN_14019b780(longlong param_1,ulonglong param_2)

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
  uVar7 = 0x80;
  if (param_2 < 0x21) {
    uVar10 = (uint)(0x10 < param_2);
LAB_14019b7de:
    if ((int)uVar10 < 0) {
      FUN_14019d350(param_2);
      return;
    }
    if (uVar10 == 0) {
      iVar6 = 0x40;
      uVar7 = 0x10;
      goto LAB_14019b825;
    }
    if (uVar10 == 1) {
      iVar6 = 0x20;
      uVar7 = 0x20;
      goto LAB_14019b825;
    }
    if (uVar10 != 2) {
      if (uVar10 == 3) {
        iVar6 = 8;
      }
      else {
        iVar6 = 0;
        uVar7 = 0;
      }
      goto LAB_14019b825;
    }
  }
  else {
    if (0x40 < param_2) {
      uVar10 = 0xffffffff;
      if (param_2 < 0x81) {
        uVar10 = 3;
      }
      goto LAB_14019b7de;
    }
    uVar10 = 2;
  }
  iVar6 = 0x10;
  uVar7 = 0x40;
LAB_14019b825:
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
LAB_14019b889:
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
      if (lVar4 == 0) goto LAB_14019b889;
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
// FUN_14147d360 @ 14147d360   (113 bytes)
//===========================================================

void FUN_14147d360(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 *puVar3;
  
  lVar2 = *(longlong *)(param_1 + 8);
  if (lVar2 != 0) {
    if (0xffffe < *(longlong *)(lVar2 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + 0x20);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar2 == 1) {
      puVar3 = (undefined8 *)(*(longlong *)(param_1 + 8) + 0x18);
      if (*(longlong *)(param_1 + 8) == 0) {
        puVar3 = (undefined8 *)0x0;
      }
      if (puVar3 != (undefined8 *)0x0) {
        (**(code **)*puVar3)(puVar3,1);
      }
    }
    *(undefined8 *)(param_1 + 8) = 0;
  }
  return;
}



//===========================================================
// FUN_14170f7d0 @ 14170f7d0   (202 bytes)
//===========================================================

int * FUN_14170f7d0(longlong param_1,int *param_2)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  undefined8 local_res8;
  
  iVar1 = *(int *)(param_1 + 0xb8);
  iVar2 = *(int *)(param_1 + 0xb4);
  *param_2 = iVar1;
  param_2[1] = iVar2;
  if (iVar1 == 0) {
    uVar3 = *(undefined4 *)(param_1 + 0x11a8);
    local_res8 = 0;
    FUN_14019a260(&local_res8,param_1 + 0x11a0);
    FUN_142645c50(param_1 + 0x78,0,0,&local_res8,uVar3,1,1,0,0xffffffff);
    local_res8 = CONCAT44(*(undefined4 *)(param_1 + 0xb4),*(undefined4 *)(param_1 + 0xb8));
    *(undefined8 *)param_2 = local_res8;
    FUN_142645170(param_1 + 0x78);
  }
  return param_2;
}



//===========================================================
// FUN_141aeed80 @ 141aeed80   (278 bytes)
//===========================================================

undefined8 * FUN_141aeed80(undefined8 *param_1)

{
  longlong lVar1;
  undefined4 *puVar2;
  
  FUN_14170fd10();
  *param_1 = &PTR_FUN_1433fa300;
  param_1[1] = &PTR_LAB_1433fa380;
  param_1[3] = &PTR_FUN_1433fa458;
  FUN_142644810(param_1 + 0xf);
  param_1[0x234] = 0;
  puVar2 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,0x11);
  puVar2[1] = 0;
  *puVar2 = 0xffffffff;
  param_1[0x234] = puVar2 + 4;
  puVar2[2] = 0;
  *(undefined1 *)param_1[0x234] = 0;
  lVar1 = param_1[0x234];
  if (*(int *)(lVar1 + -0x10) != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (*(int *)(lVar1 + -0xc) < 0) {
    FUN_142e54290(0x90,*(int *)(lVar1 + -0xc),0);
  }
  *(undefined4 *)(lVar1 + -0x10) = 1;
  *(undefined1 *)param_1[0x234] = 0;
  if (*(int *)(lVar1 + -0xc) + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  *(undefined4 *)(lVar1 + -8) = 0;
  param_1[0x235] = 0;
  *(undefined4 *)(param_1 + 0x236) = 0;
  *(undefined4 *)(param_1 + 0x238) = 0;
  *(undefined4 *)((longlong)param_1 + 0x11cc) = 1;
  param_1[0x23b] = 0;
  *(undefined4 *)(param_1 + 0x23c) = 0;
  return param_1;
}



//===========================================================
// FUN_14019a260 @ 14019a260   (485 bytes)
//===========================================================

longlong * FUN_14019a260(longlong *param_1,longlong *param_2)

{
  void *_Buf1;
  void *_Buf2;
  longlong lVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  int *piVar5;
  int *piVar6;
  int *piVar7;
  ulonglong uVar8;
  int iVar9;
  
  if (param_1 == param_2) {
    return param_1;
  }
  _Buf1 = (void *)*param_1;
  piVar7 = (int *)0x0;
  iVar9 = 0;
  iVar2 = iVar9;
  if (_Buf1 != (void *)0x0) {
    iVar2 = *(int *)((longlong)_Buf1 + -8);
  }
  _Buf2 = (void *)*param_2;
  iVar4 = iVar9;
  if (_Buf2 != (void *)0x0) {
    iVar4 = *(int *)((longlong)_Buf2 + -8);
  }
  if ((((iVar2 == iVar4) && (iVar2 != 0)) && (_Buf1 != (void *)0x0)) &&
     ((_Buf2 != (void *)0x0 && (iVar2 = memcmp(_Buf1,_Buf2,(longlong)iVar2), iVar2 == 0)))) {
    return param_1;
  }
  piVar5 = (int *)((longlong)_Buf2 + -0x10);
  if (_Buf2 == (void *)0x0) {
    piVar5 = piVar7;
  }
  if (piVar5 == (int *)0x0) {
    if (_Buf1 == (void *)0x0) {
      return param_1;
    }
    FUN_14019f2c0((longlong)_Buf1 + -0x10);
    *param_1 = 0;
    return param_1;
  }
  if (*piVar5 != -1) {
    if (*piVar5 < 1) {
      FUN_142e52dd0(0xd2);
    }
    LOCK();
    *piVar5 = *piVar5 + 1;
    UNLOCK();
    if (*param_1 != 0) {
      FUN_14019f2c0(*param_1 + -0x10);
    }
    *param_1 = (longlong)(piVar5 + 4);
    return param_1;
  }
  FUN_142e52d50(0xcb,0xffffff01);
  lVar1 = *param_2;
  piVar5 = piVar7;
  if (lVar1 == 0) goto LAB_14019a3c9;
  uVar8 = 0xffffffffffffffff;
  piVar6 = (int *)0xffffffffffffffff;
  do {
    piVar6 = (int *)((longlong)piVar6 + 1);
  } while (*(char *)(lVar1 + (longlong)piVar6) != '\0');
  iVar2 = (int)piVar6;
  if (0 < iVar2) {
    iVar9 = iVar2;
  }
  piVar3 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
  piVar3[1] = iVar9;
  *piVar3 = -1;
  piVar5 = piVar3 + 4;
  piVar3[2] = 0;
  *(undefined1 *)piVar5 = 0;
  FUN_142ef7ba0(piVar5,lVar1,(longlong)iVar2);
  if (*piVar3 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar2 == -1) || (iVar2 <= piVar3[1])) {
    *piVar3 = 1;
    if (iVar2 != -1) goto LAB_14019a3a6;
    if (piVar5 != (int *)0x0) {
      do {
        uVar8 = uVar8 + 1;
      } while (*(char *)((longlong)piVar5 + uVar8) != '\0');
      piVar7 = (int *)(uVar8 & 0xffffffff);
    }
  }
  else {
    FUN_142e54290(0x90,piVar3[1],(ulonglong)piVar6 & 0xffffffff);
    *piVar3 = 1;
LAB_14019a3a6:
    *(undefined1 *)((longlong)piVar5 + (longlong)iVar2) = 0;
    piVar7 = piVar6;
  }
  iVar2 = (int)piVar7;
  if ((iVar2 < 0) || (piVar3[1] + 1 <= iVar2)) {
    FUN_142e54290(0x9c,(ulonglong)piVar7 & 0xffffffff);
  }
  piVar3[2] = iVar2;
LAB_14019a3c9:
  if (*param_1 != 0) {
    FUN_14019f2c0(*param_1 + -0x10);
  }
  *param_1 = (longlong)piVar5;
  return param_1;
}



//===========================================================
// FUN_1401a5fa0 @ 1401a5fa0   (120 bytes)
//===========================================================

int * FUN_1401a5fa0(longlong param_1,uint param_2)

{
  int *piVar1;
  
  piVar1 = (int *)(*DAT_143ad5980)();
  if (piVar1 == (int *)0x0) {
    return (int *)0x0;
  }
  *piVar1 = param_2 * 2;
  if (param_1 != 0) {
    FUN_142ef7ba0(piVar1 + 1,param_1,(ulonglong)param_2 * 2);
  }
  *(undefined2 *)((longlong)piVar1 + (ulonglong)param_2 * 2 + 4) = 0;
  return piVar1 + 1;
}



//===========================================================
// FUN_1401be120 @ 1401be120   (114 bytes)
//===========================================================

void FUN_1401be120(undefined8 *param_1)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong lVar3;
  
  plVar2 = (longlong *)*param_1;
  if (plVar2 != (longlong *)0x0) {
    LOCK();
    plVar1 = plVar2 + 2;
    lVar3 = *plVar1;
    *(int *)plVar1 = (int)*plVar1 + -1;
    UNLOCK();
    if ((int)lVar3 == 1) {
      if (*plVar2 != 0) {
        (*DAT_143ad5990)(*plVar2 + -4);
        *plVar2 = 0;
      }
      if (plVar2[1] != 0) {
        FUN_14019b4e0();
        plVar2[1] = 0;
      }
      thunk_FUN_140205820(plVar2,0x18);
    }
    *param_1 = 0;
  }
  return;
}



//===========================================================
// _com_issue_errorex @ 142ef3ad0   (184 bytes)
//===========================================================

/* Library Function - Single Match
    void __cdecl _com_issue_errorex(long,struct IUnknown * __ptr64,struct _GUID const & __ptr64)
   
   Libraries: Visual Studio 2017 Release, Visual Studio 2019 Release */

void __cdecl _com_issue_errorex(long param_1,IUnknown *param_2,_GUID *param_3)

{
  int iVar1;
  undefined8 local_res10;
  undefined8 local_res20;
  
  local_res10 = 0;
  if ((param_2 != (IUnknown *)0x0) &&
     (iVar1 = (*(code *)PTR_FUN_1432630d8)(param_2,&DAT_1434a1968,&local_res20), -1 < iVar1)) {
    iVar1 = (*(code *)PTR_FUN_1432630d8)(local_res20,param_3);
    (*(code *)PTR_FUN_1432630d8)();
    if ((iVar1 == 0) && (iVar1 = (*DAT_1432629c0)(0,&local_res10), iVar1 != 0)) {
      local_res10 = 0;
    }
  }
  (*(code *)PTR_FUN_1432630d8)(param_1,local_res10);
  return;
}



//===========================================================
// FUN_1401a5040 @ 1401a5040   (195 bytes)
//===========================================================

longlong * FUN_1401a5040(longlong *param_1,longlong *param_2)

{
  longlong *plVar1;
  int iVar2;
  longlong lVar3;
  longlong local_res8;
  
  *param_1 = 0;
  param_2 = (longlong *)*param_2;
  if (param_2 == (longlong *)0x0) {
    plVar1 = (longlong *)*param_1;
    if (plVar1 != (longlong *)0x0) {
      *param_1 = 0;
      (**(code **)(*plVar1 + 0x10))();
    }
    iVar2 = -0x7fffbffe;
  }
  else {
    (**(code **)(*param_2 + 8))(param_2);
    local_res8 = 0;
    iVar2 = (**(code **)*param_2)(param_2,&DAT_143272478,&local_res8);
    lVar3 = 0;
    if (-1 < iVar2) {
      lVar3 = local_res8;
    }
    if ((longlong *)*param_1 != (longlong *)0x0) {
      (**(code **)(*(longlong *)*param_1 + 0x10))();
    }
    *param_1 = lVar3;
  }
  if (param_2 != (longlong *)0x0) {
    (**(code **)(*param_2 + 0x10))(param_2);
  }
  if (((iVar2 + 0x80000000U & 0x80000000) == 0) && (iVar2 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar2);
  }
  return param_1;
}



//===========================================================
// FUN_1401a5890 @ 1401a5890   (236 bytes)
//===========================================================

undefined8 * FUN_1401a5890(undefined8 *param_1,longlong param_2)

{
  uint uVar1;
  undefined8 *puVar2;
  int *piVar3;
  longlong lVar4;
  
  puVar2 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
  if (puVar2 == (undefined8 *)0x0) {
    puVar2 = (undefined8 *)0x0;
  }
  else {
    puVar2[1] = 0;
    *(undefined4 *)(puVar2 + 2) = 1;
    if (param_2 != 0) {
      lVar4 = -1;
      do {
        lVar4 = lVar4 + 1;
      } while (*(short *)(param_2 + lVar4 * 2) != 0);
      uVar1 = (int)lVar4 + 1;
      piVar3 = (int *)(*DAT_143ad5980)((ulonglong)uVar1 * 2 + 4);
      if (piVar3 == (int *)0x0) {
        *puVar2 = 0;
      }
      else {
        *piVar3 = (int)lVar4 * 2;
        piVar3 = piVar3 + 1;
        FUN_142ef7ba0(piVar3,param_2,(ulonglong)uVar1 * 2);
        *puVar2 = piVar3;
        if (piVar3 != (int *)0x0) goto LAB_1401a5940;
      }
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x8007000e);
    }
    *puVar2 = 0;
  }
LAB_1401a5940:
  *param_1 = puVar2;
  if (puVar2 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x8007000e);
  }
  return param_1;
}



//===========================================================
// FUN_141ac3850 @ 141ac3850   (6956 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000141ac41a4) */
/* WARNING: Removing unreachable block (ram,0x000141ac3f48) */

void FUN_141ac3850(longlong *param_1,longlong param_2,longlong *param_3,undefined4 param_4,
                  undefined4 param_5,undefined4 param_6,undefined1 param_7,undefined1 param_8,
                  longlong param_9)

{
  longlong *plVar1;
  char cVar2;
  ushort uVar3;
  IUnknown *pIVar4;
  undefined8 *puVar5;
  int iVar6;
  uint uVar7;
  undefined4 uVar8;
  int *piVar9;
  longlong lVar10;
  int *piVar11;
  ushort *puVar12;
  undefined4 *puVar13;
  undefined8 uVar14;
  ushort *puVar15;
  int *piVar16;
  longlong *plVar17;
  int iVar18;
  ulonglong uVar19;
  ulonglong uVar20;
  longlong *plVar21;
  longlong *plVar22;
  int *piVar23;
  undefined8 *puVar24;
  undefined8 *puVar25;
  undefined4 uVar26;
  ushort *puVar27;
  longlong *plVar28;
  bool bVar29;
  longlong **in_stack_fffffffffffffc08;
  longlong **in_stack_fffffffffffffc10;
  ulonglong in_stack_fffffffffffffc18;
  undefined4 uVar30;
  longlong **in_stack_fffffffffffffc20;
  longlong **in_stack_fffffffffffffc28;
  undefined4 uVar31;
  int *local_3c0;
  longlong *local_3b8;
  longlong **local_3b0;
  longlong **local_3a8;
  longlong *local_3a0;
  longlong *local_398;
  int *local_390;
  longlong *local_388;
  longlong *local_380;
  longlong *local_378;
  longlong *local_370;
  longlong *local_368;
  longlong *local_360;
  undefined8 local_358;
  undefined8 local_350;
  undefined8 local_348;
  undefined8 local_340;
  undefined8 local_338;
  undefined8 local_330;
  longlong *local_328;
  longlong *local_320;
  longlong *local_318;
  longlong *local_310;
  undefined8 local_308;
  undefined8 local_300;
  undefined8 local_2f8;
  longlong *local_2f0;
  longlong *local_2e8;
  int *local_2e0;
  longlong *local_2d8;
  int *local_2d0;
  longlong *local_2c8;
  int *local_2c0;
  longlong *local_2b8;
  int *local_2b0;
  longlong *local_2a8;
  undefined8 local_2a0;
  longlong *local_298;
  undefined8 local_290;
  longlong *local_288;
  undefined8 local_280;
  longlong *local_278;
  longlong *local_270;
  longlong *local_268;
  int *local_260;
  int *local_258;
  int *local_250;
  longlong *local_248;
  undefined8 local_240;
  longlong *local_238;
  undefined8 local_230;
  longlong *local_228;
  int *local_220;
  longlong *local_218;
  int *local_210;
  longlong *local_208;
  longlong *local_200;
  longlong *local_1f8;
  longlong *local_1f0;
  longlong *local_1e8;
  longlong *local_1e0;
  longlong *local_1d8;
  longlong *local_1d0;
  undefined4 local_1c8 [2];
  longlong *local_1c0;
  ushort *local_1b8;
  longlong *local_1b0;
  undefined1 *local_1a8;
  short local_1a0 [4];
  longlong local_198;
  longlong *local_188;
  longlong *local_180;
  longlong *local_178;
  longlong *local_170;
  longlong *local_168;
  longlong *local_160;
  longlong *local_158;
  longlong *local_150;
  longlong *local_148;
  longlong *local_140;
  longlong *local_138;
  longlong *local_130;
  longlong *local_128;
  longlong *local_120;
  longlong *local_118;
  longlong *local_110;
  longlong *local_108;
  longlong *local_100;
  longlong *local_f8;
  longlong *local_f0;
  longlong *local_e8;
  longlong *local_e0;
  longlong *local_d8;
  undefined1 local_d0 [8];
  longlong *local_c8;
  longlong *local_c0;
  undefined1 local_b8 [8];
  longlong *local_b0;
  longlong *local_a8;
  undefined1 local_a0 [8];
  undefined1 local_98 [8];
  undefined1 local_90 [8];
  int *local_88;
  undefined1 local_80 [8];
  undefined1 local_78 [8];
  longlong *local_70;
  undefined1 local_68 [8];
  int *local_60;
  longlong *local_58;
  
  pIVar4 = (IUnknown *)*param_3;
  if (pIVar4 != (IUnknown *)0x0) {
    local_188 = (longlong *)0x0;
    iVar6 = (**(code **)(*(longlong *)pIVar4 + 0x38))(pIVar4,&local_188);
    if (iVar6 < 0) {
      _com_issue_errorex(iVar6,pIVar4,(_GUID *)&DAT_143272478);
    }
    plVar28 = local_188;
    local_c0 = local_188;
    FUN_14023b290(&local_1c0,&local_c0);
    if (plVar28 != (longlong *)0x0) {
      (**(code **)(*plVar28 + 0x10))(plVar28);
    }
    (*DAT_143262a20)(local_1a0);
    local_1c8[0] = 0;
    plVar28 = local_1c0;
    while( true ) {
      piVar16 = (int *)0x0;
      if (plVar28 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      iVar6 = (**(code **)(*plVar28 + 0x18))(plVar28,1,local_1a0,local_1c8);
      lVar10 = local_198;
      if (iVar6 != 0) break;
      local_3c0 = (int *)0x0;
      if (local_198 != 0) {
        uVar19 = 0xffffffffffffffff;
        do {
          uVar19 = uVar19 + 1;
        } while (*(short *)(local_198 + uVar19 * 2) != 0);
        iVar6 = (int)uVar19;
        if (0 < iVar6) {
          piVar16 = (int *)(uVar19 & 0xffffffff);
        }
        piVar9 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)((int)piVar16 * 2 + 0x12));
        piVar9[1] = (int)piVar16;
        *piVar9 = -1;
        piVar16 = piVar9 + 4;
        piVar9[2] = 0;
        *(undefined2 *)piVar16 = 0;
        local_3c0 = piVar16;
        FUN_142ef7ba0(piVar16,lVar10,(longlong)iVar6 * 2);
        if (*piVar9 != -1) {
          FUN_142e52dd0();
        }
        if ((iVar6 == -1) || (iVar6 <= piVar9[1])) {
          *piVar9 = 1;
          if (iVar6 != -1) goto LAB_141ac39ff;
          if (piVar16 == (int *)0x0) {
            uVar19 = 0;
          }
          else {
            uVar19 = 0xffffffffffffffff;
            do {
              uVar19 = uVar19 + 1;
            } while (*(short *)((longlong)piVar16 + uVar19 * 2) != 0);
          }
        }
        else {
          FUN_142e54290(0x90,piVar9[1],uVar19 & 0xffffffff);
          *piVar9 = 1;
LAB_141ac39ff:
          *(undefined2 *)((longlong)iVar6 * 2 + (longlong)piVar16) = 0;
        }
        iVar6 = (int)uVar19;
        if ((iVar6 < 0) || (piVar9[1] + 1 <= iVar6)) {
          FUN_142e54290(0x9c,uVar19 & 0xffffffff);
        }
        piVar9[2] = iVar6 * 2;
      }
      if ((piVar16 == (int *)0x0) || (lVar10 = FUN_142ef8508(piVar16,0x3a), lVar10 == 0)) {
LAB_141ac52b3:
        if (piVar16 != (int *)0x0) {
          FUN_1401bebb0(piVar16 + -4);
        }
      }
      else {
        uVar19 = lVar10 - (longlong)piVar16 >> 1;
        iVar6 = (int)uVar19;
        if (iVar6 < 0) goto LAB_141ac52b3;
        FUN_1404ac7b0(&local_3c0,&local_1b8,0,uVar19 & 0xffffffff);
        FUN_1404ac7b0(&local_3c0,&local_3b8,iVar6 + 1,0xffffffff);
        plVar22 = local_3b8;
        if (param_9 != 0) {
          uVar19 = 0xffffffffffffffff;
          do {
            uVar19 = uVar19 + 1;
          } while (*(short *)(param_9 + uVar19 * 2) != 0);
          iVar18 = (int)uVar19;
          iVar6 = 0;
          if (0 < iVar18) {
            iVar6 = iVar18;
          }
          piVar11 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar6 * 2 + 0x12));
          piVar11[1] = iVar6;
          *piVar11 = -1;
          piVar9 = piVar11 + 4;
          piVar11[2] = 0;
          *(undefined2 *)piVar9 = 0;
          local_88 = piVar9;
          FUN_142ef7ba0(piVar9,param_9,(longlong)iVar18 * 2);
          if (*piVar11 != -1) {
            FUN_142e52dd0();
          }
          if ((iVar18 == -1) || (iVar18 <= piVar11[1])) {
            *piVar11 = 1;
            if (iVar18 != -1) goto LAB_141ac3b3c;
            if (piVar11 + 4 == (int *)0x0) {
              uVar19 = 0;
            }
            else {
              uVar19 = 0xffffffffffffffff;
              do {
                uVar19 = uVar19 + 1;
              } while (*(short *)((longlong)(piVar11 + 4) + uVar19 * 2) != 0);
            }
          }
          else {
            FUN_142e54290(0x90,piVar11[1],uVar19 & 0xffffffff);
            *piVar11 = 1;
LAB_141ac3b3c:
            *(undefined2 *)((longlong)iVar18 * 2 + (longlong)piVar9) = 0;
          }
          iVar6 = (int)uVar19;
          if ((iVar6 < 0) || (piVar11[1] + 1 <= iVar6)) {
            FUN_142e54290(0x9c,uVar19 & 0xffffffff);
          }
          plVar17 = local_3b8;
          piVar11[2] = iVar6 * 2;
          if (local_3b8 == (longlong *)0x0) {
            uVar8 = 0;
          }
          else {
            uVar8 = (undefined4)((ulonglong)(longlong)(int)local_3b8[-1] >> 1);
          }
          FUN_14040ea40(&local_88,&local_e8,local_3b8,uVar8);
          if (plVar17 != (longlong *)0x0) {
            FUN_1401bebb0(plVar17 + -2);
            plVar17 = (longlong *)0x0;
          }
          plVar22 = local_e8;
          local_3b8 = local_e8;
          local_e8 = plVar17;
          if (plVar17 != (longlong *)0x0) {
            FUN_1401bebb0(plVar17 + -2);
          }
          FUN_1401bebb0(piVar11);
        }
        puVar27 = local_1b8;
        uVar8 = (undefined4)((ulonglong)in_stack_fffffffffffffc08 >> 0x20);
        uVar30 = (undefined4)((ulonglong)in_stack_fffffffffffffc20 >> 0x20);
        uVar31 = (undefined4)((ulonglong)in_stack_fffffffffffffc28 >> 0x20);
        cVar2 = *(char *)((longlong)DAT_143ad1d58[1] + 0x19);
        puVar24 = (undefined8 *)DAT_143ad1d58[1];
        puVar5 = DAT_143ad1d58;
        while (puVar25 = puVar24, cVar2 == '\0') {
          puVar15 = DAT_143aa9d40;
          if (local_1b8 != (ushort *)0x0) {
            puVar15 = local_1b8;
          }
          puVar12 = DAT_143aa9d40;
          if ((ushort *)puVar25[4] != (ushort *)0x0) {
            puVar12 = (ushort *)puVar25[4];
          }
          lVar10 = (longlong)puVar15 - (longlong)puVar12;
          do {
            uVar3 = *puVar12;
            uVar7 = (uint)*(ushort *)((longlong)puVar12 + lVar10);
            if (uVar3 != uVar7) break;
            puVar12 = puVar12 + 1;
          } while (uVar7 != 0);
          if ((int)(uVar3 - uVar7) < 0) {
            puVar24 = (undefined8 *)puVar25[2];
            puVar25 = puVar5;
          }
          else {
            puVar24 = (undefined8 *)*puVar25;
          }
          cVar2 = *(char *)((longlong)puVar24 + 0x19);
          puVar5 = puVar25;
        }
        if (*(char *)((longlong)puVar5 + 0x19) == '\0') {
          puVar15 = DAT_143aa9d40;
          if ((ushort *)puVar5[4] != (ushort *)0x0) {
            puVar15 = (ushort *)puVar5[4];
          }
          puVar12 = DAT_143aa9d40;
          if (local_1b8 != (ushort *)0x0) {
            puVar12 = local_1b8;
          }
          lVar10 = (longlong)puVar15 - (longlong)puVar12;
          do {
            uVar3 = *puVar12;
            uVar7 = (uint)*(ushort *)((longlong)puVar12 + lVar10);
            if (uVar3 != uVar7) break;
            puVar12 = puVar12 + 1;
          } while (uVar7 != 0);
          if ((-1 < (int)(uVar3 - uVar7)) && (puVar5 != DAT_143ad1d58)) {
            switch(*(undefined4 *)(puVar5 + 5)) {
            case 0:
              local_3a8 = &local_3a0;
              local_3a0 = (longlong *)0x0;
              if (param_2 != 0) {
                uVar19 = 0xffffffffffffffff;
                do {
                  uVar19 = uVar19 + 1;
                } while (*(short *)(param_2 + uVar19 * 2) != 0);
                iVar18 = (int)uVar19;
                iVar6 = 0;
                if (0 < iVar18) {
                  iVar6 = iVar18;
                }
                puVar13 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar6 * 2 + 0x12));
                puVar13[1] = iVar6;
                *puVar13 = 0xffffffff;
                local_3a0 = (longlong *)(puVar13 + 4);
                puVar13[2] = 0;
                *(undefined2 *)local_3a0 = 0;
                FUN_142ef7ba0(local_3a0,param_2,(longlong)iVar18 * 2);
                plVar22 = local_3a0;
                if ((int)local_3a0[-2] != -1) {
                  FUN_142e52dd0();
                }
                if ((iVar18 == -1) || (iVar18 <= *(int *)((longlong)plVar22 + -0xc))) {
                  *(undefined4 *)(plVar22 + -2) = 1;
                  if (iVar18 != -1) goto LAB_141ac3e0e;
                  if (plVar22 == (longlong *)0x0) {
                    uVar19 = 0;
                  }
                  else {
                    uVar19 = 0xffffffffffffffff;
                    do {
                      uVar19 = uVar19 + 1;
                    } while (*(short *)((longlong)plVar22 + uVar19 * 2) != 0);
                  }
                }
                else {
                  FUN_142e54290(0x90,*(int *)((longlong)plVar22 + -0xc),uVar19 & 0xffffffff);
                  *(undefined4 *)(plVar22 + -2) = 1;
LAB_141ac3e0e:
                  *(undefined2 *)((longlong)iVar18 * 2 + (longlong)local_3a0) = 0;
                }
                iVar6 = (int)uVar19;
                if ((iVar6 < 0) || (*(int *)((longlong)plVar22 + -0xc) + 1 <= iVar6)) {
                  FUN_142e54290(0x9c,uVar19 & 0xffffffff);
                }
                *(int *)(plVar22 + -1) = iVar6 * 2;
              }
              uVar8 = (undefined4)((ulonglong)in_stack_fffffffffffffc10 >> 0x20);
              uVar30 = (undefined4)(in_stack_fffffffffffffc18 >> 0x20);
              uVar19 = 0;
              local_390 = (int *)0x0;
              piVar9 = piVar16 + -4;
              piVar23 = piVar16;
              piVar11 = local_390;
              if (piVar9 != (int *)0x0) {
                if (*piVar9 == -1) {
                  FUN_142e52d50(0xcb,0xffffff01);
                  uVar20 = 0xffffffffffffffff;
                  do {
                    uVar20 = uVar20 + 1;
                  } while (*(short *)((longlong)piVar16 + uVar20 * 2) != 0);
                  iVar18 = (int)uVar20;
                  iVar6 = 0;
                  if (0 < iVar18) {
                    iVar6 = iVar18;
                  }
                  piVar9 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar6 * 2 + 0x12));
                  piVar9[1] = iVar6;
                  *piVar9 = -1;
                  piVar11 = piVar9 + 4;
                  piVar9[2] = 0;
                  *(undefined2 *)piVar11 = 0;
                  local_60 = piVar11;
                  FUN_142ef7ba0(piVar11,piVar16,(longlong)iVar18 * 2);
                  if (*piVar9 != -1) {
                    FUN_142e52dd0(0x8b);
                  }
                  if ((iVar18 == -1) || (iVar18 <= piVar9[1])) {
                    *piVar9 = 1;
                    if (iVar18 != -1) goto LAB_141ac3f03;
                    if (piVar11 != (int *)0x0) {
                      uVar19 = 0xffffffffffffffff;
                      do {
                        uVar19 = uVar19 + 1;
                      } while (*(short *)((longlong)piVar11 + uVar19 * 2) != 0);
                    }
                  }
                  else {
                    FUN_142e54290(0x90,piVar9[1],uVar20 & 0xffffffff);
                    *piVar9 = 1;
LAB_141ac3f03:
                    *(undefined2 *)((longlong)iVar18 * 2 + (longlong)piVar11) = 0;
                    uVar19 = uVar20;
                  }
                  iVar6 = (int)uVar19;
                  if ((iVar6 < 0) || (piVar9[1] + 1 <= iVar6)) {
                    FUN_142e54290(0x9c,uVar19 & 0xffffffff);
                  }
                  piVar9[2] = iVar6 * 2;
                  if (local_390 != (int *)0x0) {
                    FUN_1401bebb0(local_390 + -4);
                  }
                }
                else {
                  if (*piVar9 < 1) {
                    FUN_142e52dd0(0xd2);
                  }
                  LOCK();
                  *piVar9 = *piVar9 + 1;
                  UNLOCK();
                  piVar23 = local_3c0;
                  puVar27 = local_1b8;
                  plVar28 = local_1c0;
                  piVar11 = piVar16;
                  if (local_390 != (int *)0x0) {
                    FUN_1401bebb0(local_390 + -4);
                    piVar23 = local_3c0;
                    puVar27 = local_1b8;
                    plVar28 = local_1c0;
                  }
                }
              }
              local_390 = piVar11;
              local_208 = (longlong *)*param_3;
              if (local_208 != (longlong *)0x0) {
                (**(code **)(*local_208 + 8))();
              }
              in_stack_fffffffffffffc28 = &local_3b8;
              in_stack_fffffffffffffc20 =
                   (longlong **)CONCAT71((int7)((ulonglong)in_stack_fffffffffffffc20 >> 8),param_8);
              in_stack_fffffffffffffc18 = CONCAT44(uVar30,param_6);
              in_stack_fffffffffffffc10 = (longlong **)CONCAT44(uVar8,param_5);
              in_stack_fffffffffffffc08 = &local_3a0;
              FUN_141ac6a30(param_1,&local_208,&local_390,param_4,in_stack_fffffffffffffc08,
                            in_stack_fffffffffffffc10,in_stack_fffffffffffffc18,
                            in_stack_fffffffffffffc20,in_stack_fffffffffffffc28);
              plVar22 = local_3b8;
              piVar16 = piVar23;
              break;
            case 1:
              local_3a8 = &local_388;
              local_388 = (longlong *)0x0;
              plVar21 = plVar22;
              plVar17 = local_388;
              if ((plVar22 != (longlong *)0x0) && (plVar1 = plVar22 + -2, plVar1 != (longlong *)0x0)
                 ) {
                if ((int)*plVar1 == -1) {
                  FUN_142e52d50(0xcb,0xffffff01);
                  uVar19 = 0xffffffffffffffff;
                  do {
                    uVar19 = uVar19 + 1;
                  } while (*(short *)((longlong)plVar22 + uVar19 * 2) != 0);
                  iVar18 = (int)uVar19;
                  iVar6 = 0;
                  if (0 < iVar18) {
                    iVar6 = iVar18;
                  }
                  piVar9 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar6 * 2 + 0x12));
                  piVar9[1] = iVar6;
                  *piVar9 = -1;
                  plVar17 = (longlong *)(piVar9 + 4);
                  piVar9[2] = 0;
                  *(undefined2 *)plVar17 = 0;
                  local_58 = plVar17;
                  FUN_142ef7ba0(plVar17,plVar22,(longlong)iVar18 * 2);
                  if (*piVar9 != -1) {
                    FUN_142e52dd0(0x8b);
                  }
                  if ((iVar18 == -1) || (iVar18 <= piVar9[1])) {
                    *piVar9 = 1;
                    if (iVar18 != -1) goto LAB_141ac4156;
                    if (plVar17 == (longlong *)0x0) {
                      uVar19 = 0;
                    }
                    else {
                      uVar19 = 0xffffffffffffffff;
                      do {
                        uVar19 = uVar19 + 1;
                      } while (*(short *)((longlong)plVar17 + uVar19 * 2) != 0);
                    }
                  }
                  else {
                    FUN_142e54290(0x90,piVar9[1],uVar19 & 0xffffffff);
                    *piVar9 = 1;
LAB_141ac4156:
                    *(undefined2 *)((longlong)plVar17 + (longlong)iVar18 * 2) = 0;
                  }
                  iVar6 = (int)uVar19;
                  if ((iVar6 < 0) || (piVar9[1] + 1 <= iVar6)) {
                    FUN_142e54290(0x9c,uVar19 & 0xffffffff);
                  }
                  piVar9[2] = iVar6 * 2;
                  if (local_388 != (longlong *)0x0) {
                    FUN_1401bebb0(local_388 + -2);
                  }
                }
                else {
                  if ((int)*plVar1 < 1) {
                    FUN_142e52dd0(0xd2);
                  }
                  LOCK();
                  *(int *)plVar1 = (int)*plVar1 + 1;
                  UNLOCK();
                  plVar21 = local_3b8;
                  piVar16 = local_3c0;
                  puVar27 = local_1b8;
                  plVar28 = local_1c0;
                  plVar17 = plVar22;
                  if (local_388 != (longlong *)0x0) {
                    FUN_1401bebb0(local_388 + -2);
                    plVar21 = local_3b8;
                    piVar16 = local_3c0;
                    puVar27 = local_1b8;
                    plVar28 = local_1c0;
                  }
                }
              }
              local_388 = plVar17;
              uVar8 = (undefined4)((ulonglong)in_stack_fffffffffffffc10 >> 0x20);
              uVar30 = (undefined4)(in_stack_fffffffffffffc18 >> 0x20);
              local_398 = (longlong *)0x0;
              if (param_2 != 0) {
                uVar19 = 0xffffffffffffffff;
                do {
                  uVar19 = uVar19 + 1;
                } while (*(short *)(param_2 + uVar19 * 2) != 0);
                iVar18 = (int)uVar19;
                iVar6 = 0;
                if (0 < iVar18) {
                  iVar6 = iVar18;
                }
                puVar13 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar6 * 2 + 0x12));
                puVar13[1] = iVar6;
                *puVar13 = 0xffffffff;
                local_398 = (longlong *)(puVar13 + 4);
                puVar13[2] = 0;
                *(undefined2 *)local_398 = 0;
                FUN_142ef7ba0(local_398,param_2,(longlong)iVar18 * 2);
                plVar22 = local_398;
                if ((int)local_398[-2] != -1) {
                  FUN_142e52dd0();
                }
                if ((iVar18 == -1) || (iVar18 <= *(int *)((longlong)plVar22 + -0xc))) {
                  *(undefined4 *)(plVar22 + -2) = 1;
                  if (iVar18 != -1) goto LAB_141ac42f5;
                  if (plVar22 == (longlong *)0x0) {
                    uVar19 = 0;
                  }
                  else {
                    uVar19 = 0xffffffffffffffff;
                    do {
                      uVar19 = uVar19 + 1;
                    } while (*(short *)((longlong)plVar22 + uVar19 * 2) != 0);
                  }
                }
                else {
                  FUN_142e54290(0x90,*(int *)((longlong)plVar22 + -0xc),uVar19 & 0xffffffff);
                  *(undefined4 *)(plVar22 + -2) = 1;
LAB_141ac42f5:
                  *(undefined2 *)((longlong)local_398 + (longlong)iVar18 * 2) = 0;
                }
                iVar6 = (int)uVar19;
                if ((iVar6 < 0) || (*(int *)((longlong)plVar22 + -0xc) + 1 <= iVar6)) {
                  FUN_142e54290(0x9c,uVar19 & 0xffffffff);
                }
                *(int *)(plVar22 + -1) = iVar6 * 2;
              }
              local_348 = 0;
              FUN_1401c1fb0(&local_348,&local_3c0);
              local_200 = (longlong *)*param_3;
              if (local_200 != (longlong *)0x0) {
                (**(code **)(*local_200 + 8))();
              }
              in_stack_fffffffffffffc20 = &local_388;
              in_stack_fffffffffffffc18 = CONCAT44(uVar30,param_6);
              in_stack_fffffffffffffc10 = (longlong **)CONCAT44(uVar8,param_5);
              in_stack_fffffffffffffc08 = &local_398;
              FUN_141ac8990(param_1,&local_200,&local_348,param_4,in_stack_fffffffffffffc08,
                            in_stack_fffffffffffffc10,in_stack_fffffffffffffc18,
                            in_stack_fffffffffffffc20);
              plVar22 = plVar21;
              break;
            case 2:
              in_stack_fffffffffffffc08 = (longlong **)FUN_1403edf80(local_80,param_2,0xffffffff);
              local_3a8 = &local_1b0;
              local_1b0 = (longlong *)0x0;
              FUN_1401c1fb0(&local_1b0,&local_3b8);
              local_338 = 0;
              FUN_1401c1fb0(&local_338,&local_3c0);
              local_1f8 = (longlong *)*param_3;
              if (local_1f8 != (longlong *)0x0) {
                (**(code **)(*local_1f8 + 8))();
              }
              FUN_141acd480(param_1,&local_1f8,&local_338,&local_1b0,in_stack_fffffffffffffc08);
              break;
            case 3:
              in_stack_fffffffffffffc08 = (longlong **)FUN_1403edf80(local_78,param_2,0xffffffff);
              local_3a8 = &local_328;
              local_328 = (longlong *)0x0;
              FUN_1401c1fb0(&local_328,&local_3b8);
              local_368 = (longlong *)0x0;
              FUN_1401c1fb0(&local_368,&local_3c0);
              local_1f0 = (longlong *)*param_3;
              if (local_1f0 != (longlong *)0x0) {
                (**(code **)(*local_1f0 + 8))();
              }
              FUN_141acdb60(param_1,&local_1f0,&local_368,&local_328,in_stack_fffffffffffffc08);
              break;
            case 4:
              local_320 = (longlong *)0x0;
              FUN_1401c1fb0(&local_320,&local_3b8);
              local_3a8 = &local_70;
              in_stack_fffffffffffffc08 = (longlong **)FUN_1403edf80(&local_70,param_2,0xffffffff);
              local_350 = 0;
              FUN_1401c1fb0(&local_350,&local_3c0);
              local_1e8 = (longlong *)*param_3;
              if (local_1e8 != (longlong *)0x0) {
                (**(code **)(*local_1e8 + 8))();
              }
              in_stack_fffffffffffffc10 = &local_320;
              FUN_141aca4e0(param_1,&local_1e8,&local_350,param_4,in_stack_fffffffffffffc08,
                            in_stack_fffffffffffffc10);
              break;
            case 5:
              local_318 = (longlong *)0x0;
              FUN_1401c1fb0(&local_318,&local_3b8);
              local_3a8 = &local_e0;
              in_stack_fffffffffffffc08 = (longlong **)FUN_1403edf80(&local_e0,param_2,0xffffffff);
              local_330 = 0;
              FUN_1401c1fb0(&local_330,&local_3c0);
              local_1e0 = (longlong *)*param_3;
              if (local_1e0 != (longlong *)0x0) {
                (**(code **)(*local_1e0 + 8))();
              }
              in_stack_fffffffffffffc10 = &local_318;
              FUN_141aca7e0(param_1,&local_1e0,&local_330,param_4,in_stack_fffffffffffffc08,
                            &local_318);
              break;
            case 6:
              local_310 = (longlong *)0x0;
              FUN_1401c1fb0(&local_310,&local_3b8);
              uVar8 = (undefined4)((ulonglong)in_stack_fffffffffffffc10 >> 0x20);
              local_3a8 = &local_d8;
              uVar14 = FUN_1403edf80(&local_d8,param_2,0xffffffff);
              local_340 = 0;
              FUN_1401c1fb0(&local_340,&local_3c0);
              local_1d8 = (longlong *)*param_3;
              if (local_1d8 != (longlong *)0x0) {
                (**(code **)(*local_1d8 + 8))();
              }
              in_stack_fffffffffffffc18 = in_stack_fffffffffffffc18 & 0xffffffffffffff00;
              in_stack_fffffffffffffc10 = (longlong **)CONCAT44(uVar8,param_4);
              in_stack_fffffffffffffc08 = &local_310;
              FUN_141acaa70(param_1,&local_1d8,&local_340,uVar14,in_stack_fffffffffffffc08,
                            in_stack_fffffffffffffc10,in_stack_fffffffffffffc18);
              break;
            case 7:
              in_stack_fffffffffffffc18 = FUN_1403edf80(local_d0,param_2,0xffffffff);
              local_308 = 0;
              FUN_1401c1fb0(&local_308,&local_3c0);
              local_100 = (longlong *)*param_3;
              if (local_100 != (longlong *)0x0) {
                (**(code **)(*local_100 + 8))();
              }
              in_stack_fffffffffffffc28 = (longlong **)CONCAT44(uVar31,param_6);
              in_stack_fffffffffffffc20 = (longlong **)CONCAT44(uVar30,param_5);
              in_stack_fffffffffffffc10 = &local_3b8;
              in_stack_fffffffffffffc08 =
                   (longlong **)CONCAT71((int7)((ulonglong)in_stack_fffffffffffffc08 >> 8),param_8);
              FUN_141ac93f0(param_1,&local_100,&local_308,param_4,in_stack_fffffffffffffc08,
                            in_stack_fffffffffffffc10,in_stack_fffffffffffffc18,
                            in_stack_fffffffffffffc20,in_stack_fffffffffffffc28);
              break;
            case 8:
              local_300 = 0;
              FUN_1401c1fb0(&local_300,&local_3c0);
              local_f8 = (longlong *)*param_3;
              if (local_f8 != (longlong *)0x0) {
                (**(code **)(*local_f8 + 8))();
              }
              in_stack_fffffffffffffc10 = &local_3b8;
              in_stack_fffffffffffffc08 =
                   (longlong **)CONCAT71((int7)((ulonglong)in_stack_fffffffffffffc08 >> 8),param_8);
              FUN_141acbcc0(param_1,&local_f8,&local_300,param_4,in_stack_fffffffffffffc08,
                            in_stack_fffffffffffffc10);
              break;
            case 9:
              local_2f8 = 0;
              FUN_1401c1fb0(&local_2f8,&local_3c0);
              local_f0 = (longlong *)*param_3;
              if (local_f0 != (longlong *)0x0) {
                (**(code **)(*local_f0 + 8))();
              }
              in_stack_fffffffffffffc10 = &local_3b8;
              in_stack_fffffffffffffc08 =
                   (longlong **)CONCAT71((int7)((ulonglong)in_stack_fffffffffffffc08 >> 8),param_8);
              FUN_141acc1f0(param_1,&local_f0,&local_2f8,param_4,in_stack_fffffffffffffc08,
                            in_stack_fffffffffffffc10);
              break;
            case 10:
              bVar29 = *(char *)(*param_1 + 0x419) != '\0';
              uVar8 = 0;
              if (bVar29) {
                uVar8 = param_6;
              }
              uVar26 = 0;
              if (bVar29) {
                uVar26 = param_5;
              }
              local_2f0 = (longlong *)0x0;
              FUN_1401c1fb0(&local_2f0,&local_3b8);
              local_3a8 = &local_c8;
              in_stack_fffffffffffffc08 = (longlong **)FUN_1403edf80(&local_c8,param_2,0xffffffff);
              local_358 = 0;
              FUN_1401c1fb0(&local_358,&local_3c0);
              local_1d0 = (longlong *)*param_3;
              if (local_1d0 != (longlong *)0x0) {
                (**(code **)(*local_1d0 + 8))();
              }
              in_stack_fffffffffffffc28 = (longlong **)CONCAT44(uVar31,uVar8);
              in_stack_fffffffffffffc20 = (longlong **)CONCAT44(uVar30,uVar26);
              in_stack_fffffffffffffc18 = CONCAT71((int7)(in_stack_fffffffffffffc18 >> 8),param_7);
              in_stack_fffffffffffffc10 = &local_2f0;
              FUN_141acc7e0(param_1,&local_1d0,&local_358,param_4,in_stack_fffffffffffffc08,
                            in_stack_fffffffffffffc10,in_stack_fffffffffffffc18,
                            in_stack_fffffffffffffc20,in_stack_fffffffffffffc28);
              break;
            case 0xb:
              local_370 = (longlong *)0x0;
              FUN_1401c1fb0(&local_370,&local_3b8);
              local_3a8 = &local_370;
              local_378 = (longlong *)0x0;
              FUN_1401c1fb0(&local_378,&local_3c0);
              local_3b0 = &local_378;
              local_360 = (longlong *)*param_3;
              if (local_360 != (longlong *)0x0) {
                (**(code **)(*local_360 + 8))();
              }
              plVar21 = local_370;
              plVar17 = local_378;
              local_380 = local_360;
              if (local_360 != (longlong *)0x0) {
                (**(code **)(*local_360 + 8))();
              }
              uVar14 = FUN_14090f750(local_68,&local_380,plVar17);
              FUN_141aa8020(param_1,uVar14,plVar21);
              if (local_360 != (longlong *)0x0) {
                (**(code **)(*local_360 + 0x10))();
              }
              if (local_378 != (longlong *)0x0) {
                FUN_1401bebb0(local_378 + -2);
              }
              if (local_370 != (longlong *)0x0) {
                FUN_1401bebb0(local_370 + -2);
              }
              break;
            case 0xc:
              local_3b0 = &local_2e8;
              local_2e8 = (longlong *)0x0;
              FUN_1401c1fb0(&local_2e8,&local_3b8);
              uVar30 = (undefined4)((ulonglong)in_stack_fffffffffffffc10 >> 0x20);
              local_2e0 = (int *)0x0;
              FUN_1401c1fb0(&local_2e0,&local_3c0);
              local_180 = (longlong *)*param_3;
              if (local_180 != (longlong *)0x0) {
                (**(code **)(*local_180 + 8))();
              }
              in_stack_fffffffffffffc10 = (longlong **)CONCAT44(uVar30,param_6);
              in_stack_fffffffffffffc08 = (longlong **)CONCAT44(uVar8,param_5);
              FUN_141ace550(param_1,&local_180,&local_2e0,&local_2e8,in_stack_fffffffffffffc08,
                            in_stack_fffffffffffffc10);
              break;
            case 0xd:
              local_3b0 = &local_2d8;
              local_2d8 = (longlong *)0x0;
              FUN_1401c1fb0(&local_2d8,&local_3b8);
              local_2d0 = (int *)0x0;
              FUN_1401c1fb0(&local_2d0,&local_3c0);
              local_178 = (longlong *)*param_3;
              if (local_178 != (longlong *)0x0) {
                (**(code **)(*local_178 + 8))();
              }
              FUN_141ace800(param_1,&local_178,&local_2d0,&local_2d8);
              break;
            case 0xe:
              local_3b0 = &local_2b8;
              local_2b8 = (longlong *)0x0;
              FUN_1401c1fb0(&local_2b8,&local_3b8);
              local_2b0 = (int *)0x0;
              FUN_1401c1fb0(&local_2b0,&local_3c0);
              local_168 = (longlong *)*param_3;
              if (local_168 != (longlong *)0x0) {
                (**(code **)(*local_168 + 8))();
              }
              in_stack_fffffffffffffc10 =
                   (longlong **)((ulonglong)in_stack_fffffffffffffc10 & 0xffffffffffffff00);
              in_stack_fffffffffffffc08 =
                   (longlong **)CONCAT71((int7)((ulonglong)in_stack_fffffffffffffc08 >> 8),param_8);
              FUN_141acebf0(param_1,&local_168,&local_2b0,&local_2b8,in_stack_fffffffffffffc08,
                            in_stack_fffffffffffffc10);
              break;
            case 0xf:
              local_3b0 = &local_2a8;
              local_2a8 = (longlong *)0x0;
              FUN_1401c1fb0(&local_2a8,&local_3b8);
              local_2a0 = 0;
              FUN_1401c1fb0(&local_2a0,&local_3c0);
              local_160 = (longlong *)*param_3;
              if (local_160 != (longlong *)0x0) {
                (**(code **)(*local_160 + 8))();
              }
              in_stack_fffffffffffffc08 = &local_2a8;
              FUN_141acfc20(param_1,&local_160,&local_2a0,param_4,in_stack_fffffffffffffc08);
              break;
            case 0x10:
              local_3b0 = &local_298;
              local_298 = (longlong *)0x0;
              FUN_1401c1fb0(&local_298,&local_3b8);
              local_290 = 0;
              FUN_1401c1fb0(&local_290,&local_3c0);
              local_158 = (longlong *)*param_3;
              if (local_158 != (longlong *)0x0) {
                (**(code **)(*local_158 + 8))();
              }
              in_stack_fffffffffffffc08 = &local_298;
              FUN_141ad05c0(param_1,&local_158,&local_290,param_4,in_stack_fffffffffffffc08);
              break;
            case 0x11:
              local_3b0 = &local_288;
              local_288 = (longlong *)0x0;
              FUN_1401c1fb0(&local_288,&local_3b8);
              local_280 = 0;
              FUN_1401c1fb0(&local_280,&local_3c0);
              local_150 = (longlong *)*param_3;
              if (local_150 != (longlong *)0x0) {
                (**(code **)(*local_150 + 8))();
              }
              in_stack_fffffffffffffc08 = &local_288;
              FUN_141ad0df0(param_1,&local_150,&local_280,param_4,in_stack_fffffffffffffc08);
              break;
            case 0x12:
              local_3b0 = &local_278;
              local_278 = (longlong *)0x0;
              FUN_1401c1fb0(&local_278,&local_3b8);
              local_270 = (longlong *)0x0;
              FUN_1401c1fb0(&local_270,&local_3c0);
              uVar14 = FUN_1403edf80(local_b8,param_2,0xffffffff);
              local_148 = (longlong *)*param_3;
              if (local_148 != (longlong *)0x0) {
                (**(code **)(*local_148 + 8))();
              }
              in_stack_fffffffffffffc10 = &local_278;
              in_stack_fffffffffffffc08 = &local_270;
              FUN_141ad1500(param_1,&local_148,uVar14,param_4,in_stack_fffffffffffffc08,
                            in_stack_fffffffffffffc10);
              break;
            case 0x13:
              local_3b0 = &local_268;
              local_268 = (longlong *)0x0;
              FUN_1401c1fb0(&local_268,&local_3b8);
              local_260 = (int *)0x0;
              FUN_1401c1fb0(&local_260,&local_3c0);
              local_140 = (longlong *)*param_3;
              if (local_140 != (longlong *)0x0) {
                (**(code **)(*local_140 + 8))();
              }
              in_stack_fffffffffffffc10 =
                   (longlong **)CONCAT71((int7)((ulonglong)in_stack_fffffffffffffc10 >> 8),1);
              in_stack_fffffffffffffc08 =
                   (longlong **)CONCAT71((int7)((ulonglong)in_stack_fffffffffffffc08 >> 8),param_8);
              FUN_141acebf0(param_1,&local_140,&local_260,&local_268,in_stack_fffffffffffffc08,
                            in_stack_fffffffffffffc10);
              break;
            case 0x14:
              local_3b0 = &local_b0;
              in_stack_fffffffffffffc08 = (longlong **)FUN_1403edf80(&local_b0,param_2,0xffffffff);
              local_258 = (int *)0x0;
              FUN_1401c1fb0(&local_258,&local_3c0);
              local_138 = (longlong *)*param_3;
              if (local_138 != (longlong *)0x0) {
                (**(code **)(*local_138 + 8))();
              }
              FUN_141ac9a40(param_1,&local_138,&local_258,param_4,in_stack_fffffffffffffc08);
              break;
            case 0x15:
              local_3b0 = &local_a8;
              in_stack_fffffffffffffc08 = (longlong **)FUN_1403edf80(&local_a8,param_2,0xffffffff);
              local_250 = (int *)0x0;
              FUN_1401c1fb0(&local_250,&local_3c0);
              local_130 = (longlong *)*param_3;
              if (local_130 != (longlong *)0x0) {
                (**(code **)(*local_130 + 8))();
              }
              FUN_141ac9f70(param_1,&local_130,&local_250,param_4,in_stack_fffffffffffffc08);
              break;
            case 0x16:
              local_3b0 = &local_248;
              local_248 = (longlong *)0x0;
              FUN_1401c1fb0(&local_248,&local_3b8);
              local_240 = 0;
              FUN_1401c1fb0(&local_240,&local_3c0);
              local_128 = (longlong *)*param_3;
              if (local_128 != (longlong *)0x0) {
                (**(code **)(*local_128 + 8))();
              }
              in_stack_fffffffffffffc08 = &local_248;
              FUN_141ad1830(param_1,&local_128,&local_240,param_4,in_stack_fffffffffffffc08);
              break;
            case 0x18:
              local_3b0 = &local_238;
              local_238 = (longlong *)0x0;
              FUN_1401c1fb0(&local_238,&local_3b8);
              local_230 = 0;
              FUN_1401c1fb0(&local_230,&local_3c0);
              local_3a8 = &local_108;
              local_108 = (longlong *)*param_3;
              if (local_108 != (longlong *)0x0) {
                (**(code **)(*local_108 + 8))();
              }
              uVar14 = FUN_1403edf80(local_a0,param_2,0xffffffff);
              in_stack_fffffffffffffc08 = &local_238;
              FUN_141ad1d50(param_1,uVar14,&local_108,&local_230,in_stack_fffffffffffffc08);
              break;
            case 0x19:
              local_1a8 = local_98;
              in_stack_fffffffffffffc08 = (longlong **)FUN_1403edf80(local_98,param_2,0xffffffff);
              local_3b0 = &local_228;
              local_228 = (longlong *)0x0;
              FUN_1401c1fb0(&local_228,&local_3b8);
              local_220 = (int *)0x0;
              FUN_1401c1fb0(&local_220,&local_3c0);
              local_120 = (longlong *)*param_3;
              if (local_120 != (longlong *)0x0) {
                (**(code **)(*local_120 + 8))();
              }
              FUN_141ad2df0(param_1,&local_120,&local_220,&local_228,in_stack_fffffffffffffc08);
              break;
            case 0x1a:
              local_1a8 = local_90;
              in_stack_fffffffffffffc08 = (longlong **)FUN_1403edf80(local_90,param_2,0xffffffff);
              local_3b0 = &local_218;
              local_218 = (longlong *)0x0;
              FUN_1401c1fb0(&local_218,&local_3b8);
              local_210 = (int *)0x0;
              FUN_1401c1fb0(&local_210,&local_3c0);
              local_118 = (longlong *)*param_3;
              if (local_118 != (longlong *)0x0) {
                (**(code **)(*local_118 + 8))();
              }
              FUN_141ad3330(param_1,&local_118,&local_210,&local_218,in_stack_fffffffffffffc08);
              break;
            case 0x1b:
              local_110 = (longlong *)*param_3;
              if (local_110 != (longlong *)0x0) {
                (**(code **)(*local_110 + 8))();
              }
              uVar8 = FUN_140910eb0(&local_110,piVar16,0);
              FUN_141ac31c0(param_1,uVar8);
              break;
            case 0x1c:
              local_3b0 = &local_2c8;
              local_2c8 = (longlong *)0x0;
              FUN_1401c1fb0(&local_2c8,&local_3b8);
              local_2c0 = (int *)0x0;
              FUN_1401c1fb0(&local_2c0,&local_3c0);
              local_170 = (longlong *)*param_3;
              if (local_170 != (longlong *)0x0) {
                (**(code **)(*local_170 + 8))();
              }
              FUN_141ace9f0(param_1,&local_170,&local_2c0,&local_2c8);
            }
          }
        }
        if (plVar22 != (longlong *)0x0) {
          FUN_1401bebb0(plVar22 + -2);
        }
        if (puVar27 != (ushort *)0x0) {
          FUN_1401bebb0(puVar27 + -8);
        }
        FUN_1401bebb0(piVar16 + -4);
      }
      if (local_1a0[0] == 8) {
        local_1a0[0] = 0;
        if (local_198 != 0) {
          (*DAT_143ad5990)(local_198 + -4);
        }
      }
      else {
        iVar6 = (*DAT_143262a18)(local_1a0);
        if (iVar6 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar6);
        }
      }
    }
    if (local_1a0[0] == 8) {
      local_1a0[0] = 0;
      if (local_198 != 0) {
        (*DAT_143ad5990)(local_198 + -4);
      }
    }
    else {
      (*DAT_143262a18)(local_1a0);
    }
    (**(code **)(*plVar28 + 0x10))(plVar28);
    if ((longlong *)*param_3 != (longlong *)0x0) {
      (**(code **)(*(longlong *)*param_3 + 0x10))();
    }
  }
  return;
}



//===========================================================
// FUN_142ef3ac0 @ 142ef3ac0   (16 bytes)
//===========================================================

void FUN_142ef3ac0(undefined8 param_1)

{
  (*(code *)PTR_FUN_1432630d8)(param_1,0);
  return;
}



//===========================================================
// FUN_1409339d0 @ 1409339d0   (68 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_1409339d0(undefined8 param_1)

{
  undefined4 local_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined8 local_18;
  
  local_28 = _DAT_143a8b8d8;
  uStack_24 = uRam0000000143a8b8dc;
  uStack_20 = (undefined4)DAT_143a8b8e0;
  uStack_1c = DAT_143a8b8e0._4_4_;
  local_18 = DAT_143a8b8e8;
  FUN_140930ec0(_DAT_143a8b8d8,DAT_143a8b8e8,&local_28);
  return param_1;
}



//===========================================================
// FUN_141af79f0 @ 141af79f0   (581 bytes)
//===========================================================

undefined8 * FUN_141af79f0(longlong *param_1,uint *param_2)

{
  uint uVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  void *pvVar5;
  uint uVar6;
  longlong lVar7;
  undefined8 *puVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong *plVar11;
  int *piVar12;
  uint *puVar13;
  uint uVar14;
  
  plVar2 = (longlong *)*param_1;
  if (plVar2 == (longlong *)0x0) {
    uVar14 = *(uint *)(param_1 + 1);
    uVar1 = uVar14;
  }
  else {
    if (*(uint *)((longlong)param_1 + 0xc) <= *(uint *)((longlong)param_1 + 0x14))
    goto LAB_141af7b2e;
    uVar14 = *(uint *)(param_1 + 1);
    uVar1 = uVar14 * 2;
  }
  if ((uVar1 != 0) && ((uVar14 != uVar1 || (plVar2 == (longlong *)0x0)))) {
    puVar13 = (uint *)&DAT_1433f9c40;
    uVar10 = 0xf6;
    do {
      uVar9 = uVar10 >> 1;
      if (puVar13[uVar9] < uVar1) {
        puVar13 = puVar13 + uVar9 + 1;
        uVar9 = uVar10 + (-1 - uVar9);
      }
      uVar10 = uVar9;
    } while (0 < (longlong)uVar9);
    uVar1 = *puVar13;
    *(uint *)(param_1 + 1) = uVar1;
    uVar6 = 0xffffffff;
    if ((int)param_1[2] != -1) {
      uVar6 = (int)param_1[2] * uVar1 >> 7;
    }
    *(uint *)((longlong)param_1 + 0x14) = uVar6;
    lVar7 = FUN_14019b780(&DAT_143ad68a0);
    *param_1 = lVar7;
    FUN_142ef8250(lVar7,0,(ulonglong)uVar1 * 8);
    plVar11 = plVar2;
    if (plVar2 != (longlong *)0x0) {
      while (plVar11 < plVar2 + uVar14) {
        lVar3 = *plVar11;
        plVar11 = plVar11 + 1;
        while (lVar3 != 0) {
          uVar10 = (ulonglong)*(uint *)(lVar3 + 0x10) % (ulonglong)uVar1;
          lVar4 = *(longlong *)(lVar3 + 8);
          *(undefined8 *)(lVar3 + 8) = *(undefined8 *)(lVar7 + uVar10 * 8);
          *(longlong *)(lVar7 + uVar10 * 8) = lVar3;
          lVar3 = lVar4;
        }
      }
      FUN_14019b4e0(plVar2);
    }
  }
LAB_141af7b2e:
  plVar2 = (longlong *)(*param_1 + ((ulonglong)*param_2 % (ulonglong)*(uint *)(param_1 + 1)) * 8);
  for (puVar8 = (undefined8 *)*plVar2; puVar8 != (undefined8 *)0x0; puVar8 = (undefined8 *)puVar8[1]
      ) {
    if (*(uint *)(puVar8 + 2) == *param_2) {
      return puVar8;
    }
  }
  *(int *)((longlong)param_1 + 0xc) = *(int *)((longlong)param_1 + 0xc) + 1;
  pvVar5 = Self;
  lVar3 = DAT_143ad1e18;
  plVar11 = (longlong *)(DAT_143ad1e18 + 0x18);
  LOCK();
  lVar7 = *plVar11;
  if (lVar7 == 0) {
    *plVar11 = (longlong)Self;
  }
  UNLOCK();
  if (lVar7 == 0) {
LAB_141af7bc9:
    *(undefined4 *)(lVar3 + 0x20) = 1;
  }
  else if ((void *)*plVar11 == pvVar5) {
    *(int *)(lVar3 + 0x20) = *(int *)(lVar3 + 0x20) + 1;
  }
  else {
    while( true ) {
      pvVar5 = Self;
      LOCK();
      lVar7 = *plVar11;
      if (lVar7 == 0) {
        *plVar11 = (longlong)Self;
      }
      UNLOCK();
      if (lVar7 == 0) goto LAB_141af7bc9;
      if ((void *)*plVar11 == pvVar5) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar3 + 0x20) = *(int *)(lVar3 + 0x20) + 1;
  }
  piVar12 = (int *)(lVar3 + 0x20);
  puVar8 = *(undefined8 **)(lVar3 + 0x28);
  if (puVar8 == (undefined8 *)0x0) {
    puVar8 = (undefined8 *)FUN_14019d3c0(0x28);
    *(undefined8 **)(lVar3 + 0x28) = puVar8;
  }
  *(undefined8 *)(lVar3 + 0x28) = *puVar8;
  *piVar12 = *piVar12 + -1;
  if (*piVar12 == 0) {
    *plVar11 = 0;
  }
  lVar7 = *plVar2;
  *puVar8 = &PTR_FUN_1433fb940;
  puVar8[1] = lVar7;
  *(undefined4 *)(puVar8 + 2) = 0;
  puVar8[4] = 0;
  *(uint *)(puVar8 + 2) = *param_2;
  *plVar2 = (longlong)puVar8;
  return puVar8;
}



//===========================================================
// FUN_14019f970 @ 14019f970   (31 bytes)
//===========================================================

void FUN_14019f970(void)

{
  undefined1 local_28 [40];
  
  FUN_140199120(local_28);
                    /* WARNING: Subroutine does not return */
  _CxxThrowException(local_28,(ThrowInfo *)&DAT_143a3aeb0);
}



//===========================================================
// FUN_140197eb0 @ 140197eb0   (100 bytes)
//===========================================================

ulonglong FUN_140197eb0(ulonglong param_1)

{
  longlong lVar1;
  ulonglong uVar2;
  
  if (param_1 < 0x1000) {
    if (param_1 != 0) {
      uVar2 = FUN_14019b780(&DAT_143ad68a0,param_1);
      return uVar2;
    }
    return 0;
  }
  if (param_1 < param_1 + 0x27) {
    lVar1 = FUN_14019b780(&DAT_143ad68a0);
    if (lVar1 != 0) {
      uVar2 = lVar1 + 0x27U & 0xffffffffffffffe0;
      *(longlong *)(uVar2 - 8) = lVar1;
      return uVar2;
    }
                    /* WARNING: Subroutine does not return */
    FUN_142f04804();
  }
                    /* WARNING: Subroutine does not return */
  FUN_14019f970();
}



//===========================================================
// FUN_14147caa0 @ 14147caa0   (181 bytes)
//===========================================================

void FUN_14147caa0(longlong *param_1,longlong param_2,longlong param_3,longlong param_4)

{
  ulonglong uVar1;
  longlong lVar2;
  longlong lVar3;
  
  lVar2 = *param_1;
  if (lVar2 != 0) {
    lVar3 = param_1[1];
    if (lVar2 != lVar3) {
      do {
        FUN_140cbca60(lVar2);
        lVar2 = lVar2 + 0x10;
      } while (lVar2 != lVar3);
      lVar2 = *param_1;
    }
    uVar1 = param_1[2] - lVar2 & 0xfffffffffffffff0;
    lVar3 = lVar2;
    if (0xfff < uVar1) {
      lVar3 = *(longlong *)(lVar2 + -8);
      if (0x1f < (lVar2 - lVar3) - 8U) {
                    /* WARNING: Subroutine does not return */
        FUN_142f04804(lVar3,uVar1 + 0x27);
      }
    }
    thunk_FUN_140205820(lVar3);
  }
  *param_1 = param_2;
  param_1[1] = param_3 * 0x10 + param_2;
  param_1[2] = param_4 * 0x10 + param_2;
  return;
}



//===========================================================
// FUN_14131c5a0 @ 14131c5a0   (16 bytes)
//===========================================================

void FUN_14131c5a0(void)

{
                    /* WARNING: Subroutine does not return */
  FUN_142ed3068("vector too long");
}


