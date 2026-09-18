
//===========================================================
// FUN_1417dd7e0 @ 1417dd7e0   (4799 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 FUN_1417dd7e0(int param_1,int param_2,int param_3)

{
  longlong *plVar1;
  longlong lVar2;
  longlong *plVar3;
  uint uVar4;
  ulonglong uVar5;
  undefined8 *puVar6;
  char cVar7;
  byte bVar8;
  int iVar9;
  undefined4 uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  undefined4 uVar14;
  longlong lVar15;
  longlong lVar16;
  undefined8 uVar17;
  longlong lVar18;
  longlong lVar19;
  int *piVar20;
  undefined8 *puVar21;
  undefined8 uVar22;
  undefined1 auStackY_808 [32];
  longlong *local_788;
  longlong **local_780;
  ulonglong local_778;
  int local_770;
  undefined4 uStack_76c;
  int local_768;
  undefined4 uStack_764;
  undefined4 local_760;
  uint local_758;
  longlong local_750;
  int local_748 [2];
  longlong local_740;
  undefined1 local_738 [8];
  undefined8 *local_730;
  ulonglong local_728 [4];
  longlong local_708;
  undefined1 local_700 [8];
  undefined1 local_6f8 [8];
  undefined8 *local_6f0;
  undefined1 local_6e8 [8];
  undefined8 *local_6e0;
  undefined8 *local_6d0;
  undefined8 *local_6c0;
  undefined8 local_6b8;
  undefined8 uStack_6b0;
  undefined8 local_6a8;
  undefined8 uStack_6a0;
  undefined8 local_698;
  undefined8 uStack_690;
  undefined8 local_688;
  undefined8 uStack_680;
  undefined8 local_678;
  undefined8 uStack_670;
  undefined8 local_668;
  undefined8 uStack_660;
  undefined8 local_658;
  undefined8 uStack_650;
  undefined8 local_648;
  undefined4 local_640;
  undefined1 local_638 [8];
  undefined1 local_630 [40];
  undefined1 local_608 [8];
  undefined4 local_600;
  undefined1 local_5fc [8];
  undefined4 local_5f4;
  undefined1 local_5f0 [8];
  undefined4 local_5e8;
  undefined1 local_5e4 [8];
  undefined4 local_5dc;
  undefined1 local_5d8 [8];
  undefined4 local_5d0;
  undefined1 local_5cc [8];
  undefined4 local_5c4;
  undefined1 local_5c0 [8];
  undefined4 local_5b8;
  undefined1 local_5b4 [8];
  undefined4 local_5ac;
  undefined1 local_548 [48];
  undefined1 local_518 [208];
  undefined1 local_448 [512];
  undefined1 local_248 [512];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStackY_808;
  if (DAT_143aa84a0 == (longlong *)0x0) {
    return 0;
  }
  if (DAT_143aa8520 != 0) {
    return 0;
  }
  local_770 = param_2;
  local_768 = param_1;
  local_748[0] = param_3;
  iVar9 = FUN_1417da100(1);
  lVar19 = DAT_143aa8518;
  if (iVar9 == 0) {
    return 0;
  }
  lVar15 = FUN_142cbe730(DAT_143aa84a0);
  local_780 = (longlong **)lVar15;
  lVar16 = (**(code **)(*DAT_143aa84a0 + 0x30))();
  local_788 = DAT_143aa84a0;
  iVar9 = FUN_142cc42d0(DAT_143aa84a0,500,0);
  if (iVar9 == 0) {
    return 0;
  }
  if ((param_1 != 1) && (param_1 != 6)) {
    return 0;
  }
  if (param_3 == 0) {
    return 0;
  }
  iVar9 = FUN_1401ba9d0(lVar16 + 0x3574,*(undefined4 *)(lVar16 + 0x357c));
  if ((iVar9 == 0) ||
     (iVar9 = FUN_1401ba9d0(lVar16 + 0x3598,*(undefined4 *)(lVar16 + 0x35a0)), iVar9 == 0)) {
    lVar18 = FUN_1402e3cd0(lVar15,local_6f8,param_1,param_2);
    puVar21 = local_6f0;
    lVar18 = *(longlong *)(lVar18 + 8);
    iVar9 = 0;
    if (local_6f0 != (undefined8 *)0x0) {
      if (0xffffe < local_6f0[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar3 = puVar21 + 1;
      lVar2 = *plVar3;
      *plVar3 = *plVar3 + -1;
      UNLOCK();
      if (((int)lVar2 == 1) && (local_6f0 != (undefined8 *)0x0)) {
        (**(code **)*local_6f0)(local_6f0,1);
      }
      local_6f0 = (undefined8 *)0x0;
    }
    if (lVar18 == 0) {
      return 0;
    }
    if ((lVar19 == 0) || (cVar7 = FUN_1429289b0(lVar19), cVar7 == '\0')) {
      lVar19 = DAT_143aa8328;
      lVar18 = lVar18 + 0x20;
      uVar10 = FUN_14019a5d0(lVar18);
      iVar11 = FUN_140388e30(lVar19,uVar10);
      if ((iVar11 != 0) && (iVar11 = FUN_142cf5c80(DAT_143aa84a0), iVar11 == 0)) {
        return 0;
      }
      iVar11 = FUN_140f8a9e0(DAT_143aa8518 + 0x100);
      if ((iVar11 != 0) &&
         ((iVar11 = FUN_14019a5d0(lVar18), iVar11 - 1900000U < 10000 ||
          (iVar11 = FUN_14019a5d0(lVar18), iVar11 - 0x1d24f0U < 10000)))) {
        local_6b8 = 0;
        uStack_6b0 = 0;
        local_6a8 = 0;
        uStack_6a0 = 0;
        local_698 = 0;
        uStack_690 = 0;
        local_688 = 0;
        uStack_680 = 0;
        local_678 = 0;
        uStack_670 = 0;
        local_668 = 0;
        uStack_660 = 0;
        local_658 = 0;
        uStack_650 = 0;
        local_648 = 0;
        local_640 = 0;
        FUN_142973160(DAT_143aa8518,0x4c4b7e8,0,&local_6b8);
        return 0;
      }
      iVar11 = FUN_142cbeca0(DAT_143aa84a0);
      if (iVar11 == 0) {
        local_778 = CONCAT44(local_778._4_4_,-local_770);
        if (-local_770 == 0xb) {
          iVar11 = FUN_1401ba9d0(lVar16 + 0x22f4,*(undefined4 *)(lVar16 + 0x22fc));
          if ((iVar11 != 0) &&
             (iVar11 = FUN_1401ba9d0(lVar16 + 0x2300,*(undefined4 *)(lVar16 + 0x2308)),
             lVar19 = DAT_143aa8328, iVar11 == 0x17d84b59)) {
            uVar10 = FUN_14019a5d0(lVar18);
            iVar11 = FUN_140389c10(lVar19,uVar10);
            if (iVar11 == 0) {
              uVar17 = FUN_1408a9e40(&local_788,0x4fa);
              goto LAB_1417dd8e5;
            }
          }
          lVar19 = DAT_143aa8328;
          uVar10 = FUN_14019a5d0(lVar18);
          iVar11 = FUN_140389c10(lVar19,uVar10);
          if ((iVar11 == 0) &&
             ((((iVar11 = FUN_140ae0f30(lVar16,0x4c4b9be), iVar11 != 0 ||
                (iVar11 = FUN_140ae0f30(lVar16,0x4c4b9bf), iVar11 != 0)) ||
               (iVar11 = FUN_140ae0f30(lVar16,0x4c4b9c0), iVar11 != 0)) ||
              (iVar11 = FUN_140ae0f30(lVar16,0x4c4b9c1), iVar11 != 0)))) {
            uVar17 = FUN_1408a9e40(&local_788,0x150b);
            FUN_140d84b70(uVar17,0);
            return 0;
          }
        }
        iVar11 = local_748[0];
        lVar19 = FUN_1402e3cd0(lVar15,local_6e8,local_768,local_748[0]);
        puVar21 = local_6e0;
        plVar3 = *(longlong **)(lVar19 + 8);
        if (local_6e0 != (undefined8 *)0x0) {
          if (0xffffe < local_6e0[1] - 1) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar1 = puVar21 + 1;
          lVar19 = *plVar1;
          *plVar1 = *plVar1 + -1;
          UNLOCK();
          if (((int)lVar19 == 1) && (local_6e0 != (undefined8 *)0x0)) {
            (**(code **)*local_6e0)(local_6e0,1);
          }
          local_6e0 = (undefined8 *)0x0;
        }
        if (plVar3 != (longlong *)0x0) {
          iVar11 = FUN_1417ea700(-iVar11);
          if (iVar11 != 0) {
            return 0;
          }
          iVar11 = FUN_1402536d0(local_778 & 0xffffffff);
          lVar19 = DAT_143aa8328;
          if ((iVar11 != 0) && (DAT_143aa8328 != 0)) {
            uVar10 = FUN_14019a5d0(plVar3 + 4);
            cVar7 = FUN_14038cfa0(lVar19,uVar10);
            if (cVar7 != '\0') {
              local_740 = 0;
              local_780 = &local_788;
              local_788 = (longlong *)0x0;
              piVar20 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
              piVar20[1] = 0;
              *piVar20 = -1;
              local_788 = (longlong *)(piVar20 + 4);
              piVar20[2] = 0;
              *(undefined1 *)local_788 = 0;
              if (*piVar20 != -1) {
                FUN_142e52dd0(0x8b);
              }
              if (piVar20[1] < 0) {
                FUN_142e54290(0x90,piVar20[1],0);
              }
              *piVar20 = 1;
              *(undefined1 *)local_788 = 0;
              if (piVar20[1] + 1 < 1) {
                FUN_142e54290(0x9c,0);
              }
              piVar20[2] = 0;
              lVar19 = DAT_143aa8328;
              uVar10 = FUN_14019a5d0(plVar3 + 4);
              uVar17 = FUN_140398ba0(lVar19,local_748,uVar10);
              puVar21 = (undefined8 *)FUN_1408e55d0(&local_770,uVar17,&local_788);
              uVar17 = *puVar21;
              puVar21 = (undefined8 *)FUN_1408a9e40(&local_768,0x4e7);
              uVar17 = FUN_14019ba10(&local_740,*puVar21,uVar17);
              local_728[0] = 0;
              FUN_14019a260(local_728,uVar17);
              FUN_140d84b70(local_728,0);
              if (CONCAT44(uStack_764,local_768) != 0) {
                FUN_14019f2c0(CONCAT44(uStack_764,local_768) + -0x10);
              }
              if (CONCAT44(uStack_76c,local_770) != 0) {
                FUN_14019f2c0(CONCAT44(uStack_76c,local_770) + -0x10);
              }
              if (local_740 == 0) {
                return 0;
              }
              FUN_14019f2c0(local_740 + -0x10);
              return 0;
            }
          }
          local_728[2] = FUN_140192f00(plVar3);
          lVar19 = DAT_143aa8328;
          plVar1 = plVar3 + 4;
          uVar10 = FUN_14019a5d0(plVar1);
          iVar11 = FUN_140388e30(lVar19,uVar10);
          if ((iVar11 != 0) && (iVar11 = FUN_142cf5c80(DAT_143aa84a0), iVar11 == 0)) {
            return 0;
          }
          if (local_728[2] == 0) {
            return 0;
          }
          iVar11 = FUN_1401ba9d0(local_728[2] + 0x13a,*(undefined4 *)(local_728[2] + 0x142));
          if (iVar11 == 0) {
            return 0;
          }
          iVar11 = FUN_14019a5d0(lVar18);
          if ((99 < iVar11 - 0x10f7c0U) &&
             (iVar12 = FUN_14019a5d0(plVar1), iVar11 = iVar9, iVar12 - 0x10f7c0U < 100)) {
            do {
              iVar12 = FUN_1402537f0(iVar11);
              lVar19 = *(longlong *)(lVar15 + ((longlong)iVar12 + 0x3b) * 0x10);
              if ((lVar19 != 0) && (iVar12 = FUN_14019a5d0(lVar19 + 0x20), iVar12 - 0x10f7c0U < 100)
                 ) {
                uVar17 = FUN_1408a9e40(&local_788,0x4e8);
                goto LAB_1417dd8e5;
              }
              iVar11 = iVar11 + 1;
            } while (iVar11 < 4);
          }
          iVar11 = FUN_14019a5d0(lVar18);
          if ((99 < iVar11 - 0x10fae0U) &&
             (iVar12 = FUN_14019a5d0(plVar1), iVar11 = iVar9, iVar12 - 0x10fae0U < 100)) {
            do {
              iVar12 = FUN_1402537f0(iVar11);
              lVar19 = *(longlong *)(lVar15 + ((longlong)iVar12 + 0x3b) * 0x10);
              if ((lVar19 != 0) && (iVar12 = FUN_14019a5d0(lVar19 + 0x20), iVar12 - 0x10fae0U < 100)
                 ) {
                uVar17 = FUN_1408a9e40(&local_788,0x4e9);
                goto LAB_1417dd8e5;
              }
              iVar11 = iVar11 + 1;
            } while (iVar11 < 4);
          }
          iVar11 = FUN_14019a5d0(lVar18);
          if ((99 < iVar11 - 0x10f8ecU) &&
             (iVar12 = FUN_14019a5d0(plVar1), iVar11 = iVar9, iVar12 - 0x10f8ecU < 100)) {
            do {
              iVar12 = FUN_1402537f0(iVar11);
              lVar19 = *(longlong *)(lVar15 + ((longlong)iVar12 + 0x3b) * 0x10);
              if ((lVar19 != 0) && (iVar12 = FUN_14019a5d0(lVar19 + 0x20), iVar12 - 0x10f8ecU < 100)
                 ) {
                uVar17 = FUN_1408a9e40(&local_788,0xb63);
                goto LAB_1417dd8e5;
              }
              iVar11 = iVar11 + 1;
            } while (iVar11 < 4);
          }
          iVar11 = FUN_14019a5d0(lVar18);
          if ((99 < iVar11 - 0x10ff90U) &&
             (iVar12 = FUN_14019a5d0(plVar1), iVar11 = iVar9, iVar12 - 0x10ff90U < 100)) {
            do {
              iVar12 = FUN_1402537f0(iVar11);
              lVar19 = *(longlong *)(lVar15 + ((longlong)iVar12 + 0x3b) * 0x10);
              if ((lVar19 != 0) && (iVar12 = FUN_14019a5d0(lVar19 + 0x20), iVar12 - 0x10ff90U < 100)
                 ) {
                uVar17 = FUN_1408a9e40(&local_788,0x4ea);
                goto LAB_1417dd8e5;
              }
              iVar11 = iVar11 + 1;
            } while (iVar11 < 4);
          }
          iVar11 = FUN_14019a5d0(lVar18);
          if ((99 < iVar11 - 0x110058U) &&
             (iVar12 = FUN_14019a5d0(plVar1), iVar11 = iVar9, iVar12 - 0x110058U < 100)) {
            do {
              iVar12 = FUN_1402537f0(iVar11);
              lVar19 = *(longlong *)(lVar15 + ((longlong)iVar12 + 0x1b) * 0x10);
              if ((lVar19 != 0) && (iVar12 = FUN_14019a5d0(lVar19 + 0x20), iVar12 - 0x110058U < 100)
                 ) {
                uVar17 = FUN_1408a9e40(&local_788,0x4eb);
                goto LAB_1417dd8e5;
              }
              iVar11 = iVar11 + 1;
            } while (iVar11 < 4);
          }
          iVar11 = FUN_14019a5d0(lVar18);
          if ((99 < iVar11 - 0x1122b8U) &&
             (iVar12 = FUN_14019a5d0(plVar1), iVar11 = iVar9, iVar12 - 0x1122b8U < 100)) {
            do {
              iVar12 = FUN_140253830(iVar11);
              lVar19 = *(longlong *)(lVar15 + ((longlong)iVar12 + 0x1b) * 0x10);
              if ((lVar19 != 0) && (iVar12 = FUN_14019a5d0(lVar19 + 0x20), iVar12 - 0x1122b8U < 100)
                 ) {
                uVar17 = FUN_1408a9e40(&local_788,0x4ec);
                goto LAB_1417dd8e5;
              }
              iVar11 = iVar11 + 1;
            } while (iVar11 < 2);
          }
          cVar7 = FUN_140841850();
          if ((cVar7 == '\0') &&
             (iVar11 = (**(code **)(*plVar3 + 0x28))(plVar3), lVar19 = DAT_143aa8328, iVar11 == 0))
          {
            uVar10 = FUN_14019a5d0(plVar1);
            iVar11 = FUN_14038ce90(lVar19,uVar10);
            if ((iVar11 != 0) ||
               ((cVar7 = (**(code **)(*plVar3 + 0x200))(plVar3), cVar7 != '\0' &&
                (iVar11 = FUN_14038ab40(DAT_143aa8328,plVar3), iVar11 == 0)))) {
              uVar17 = FUN_1408a9e40(local_700,0x4d4);
              iVar11 = FUN_142a269c0(uVar17,0,0,1);
              if (iVar11 != 6) {
                return 0;
              }
            }
          }
          if ((plVar3[7] != 0) &&
             (iVar11 = (**(code **)(*plVar3 + 0x38))(plVar3), lVar19 = DAT_143aa8328, iVar11 != 0))
          {
            uVar10 = FUN_14019a5d0(plVar1);
            iVar11 = FUN_14038ac80(lVar19,uVar10);
            if (iVar11 == 0) {
              uVar17 = FUN_1408a9e40(&local_750,0xcba);
              iVar11 = FUN_142a269c0(uVar17,0,0,1);
              if (iVar11 != 6) {
                return 0;
              }
            }
          }
          lVar19 = DAT_143aa8328;
          uVar10 = FUN_14019a5d0(plVar1);
          lVar19 = FUN_140388c60(lVar19,uVar10);
          if ((((lVar19 != 0) && (*(longlong *)(lVar19 + 0x338) != 0)) &&
              (*(int *)(*(longlong *)(lVar19 + 0x338) + -8) != 0)) &&
             (iVar11 = iVar9, *(int *)(lVar19 + 0x38) - 0x111700U < 10000)) {
            do {
              iVar12 = FUN_140253830(iVar11);
              lVar16 = DAT_143aa8328;
              lVar19 = *(longlong *)(lVar15 + ((longlong)iVar12 + 0x1b) * 0x10);
              if (lVar19 != 0) {
                uVar10 = FUN_14019a5d0(lVar19 + 0x20);
                lVar19 = FUN_140388c60(lVar16,uVar10);
                if (((lVar19 != 0) && (*(longlong *)(lVar19 + 0x338) != 0)) &&
                   (*(int *)(*(longlong *)(lVar19 + 0x338) + -8) != 0)) {
                  uVar17 = FUN_1408a9e40(&local_788,0x4ed);
                  goto LAB_1417dd8e5;
                }
              }
              iVar11 = iVar11 + 1;
            } while (iVar11 < 2);
          }
          iVar11 = FUN_140253450(local_770);
          local_758 = iVar11;
          uVar10 = FUN_14019a5d0(plVar3 + 4);
          local_760 = uVar10;
          iVar11 = FUN_140253980(uVar10,iVar11,*(undefined1 *)(lVar15 + 0x19),0);
          if (iVar11 == 0) {
            return 0;
          }
          iVar11 = FUN_1417ea500(DAT_143aa8328,plVar3);
          iVar12 = FUN_140397680(DAT_143aa8328,lVar15,uVar10,lVar15 + 0x1a8);
          if (iVar12 != 0) {
            uVar10 = FUN_140253420(iVar12,iVar11);
            iVar13 = FUN_1417ea730(uVar10);
            iVar12 = local_768;
            if (iVar13 != iVar11) {
              return 0;
            }
            FUN_1402e3cd0(lVar15,local_738,local_768,uVar10);
            if (local_730 == (undefined8 *)0x0) {
LAB_1417de3a6:
              puVar21 = local_730;
              if (local_730 == (undefined8 *)0x0) {
                return 0;
              }
              if (0xffffe < local_730[1] - 1) {
                FUN_142e541f0(0x31e);
              }
              LOCK();
              plVar3 = puVar21 + 1;
              lVar19 = *plVar3;
              *plVar3 = *plVar3 + -1;
              UNLOCK();
              if ((int)lVar19 != 1) {
                return 0;
              }
              if (local_730 == (undefined8 *)0x0) {
                return 0;
              }
              (**(code **)*local_730)(local_730,1);
              return 0;
            }
            if ((iVar12 == 1) && (iVar11 = FUN_14019a5d0(local_730 + 4), iVar11 - 0x10a1d0U < 10000)
               ) {
              puVar21 = *(undefined8 **)(lVar15 + 0x450);
              local_6c0 = puVar21;
              if (puVar21 != (undefined8 *)0x0) {
                if (0xfffff < (ulonglong)puVar21[1]) {
                  FUN_142e541f0(0x30f);
                }
                LOCK();
                puVar21[1] = puVar21[1] + 1;
                UNLOCK();
              }
              puVar6 = local_6c0;
              if ((puVar21 != (undefined8 *)0x0) && (iVar11 = FUN_14022fa10(lVar15,6), iVar11 < 1))
              {
                if (puVar6 != (undefined8 *)0x0) {
                  if (0xffffe < puVar21[1] - 1) {
                    FUN_142e541f0(0x31e);
                  }
                  LOCK();
                  plVar3 = puVar21 + 1;
                  lVar19 = *plVar3;
                  *plVar3 = *plVar3 + -1;
                  UNLOCK();
                  if ((int)lVar19 == 1) {
                    (**(code **)*puVar21)(puVar21,1);
                  }
                }
                goto LAB_1417de3a6;
              }
              if (puVar6 != (undefined8 *)0x0) {
                if (0xffffe < puVar21[1] - 1) {
                  FUN_142e541f0(0x31e);
                }
                LOCK();
                plVar3 = puVar21 + 1;
                lVar19 = *plVar3;
                *plVar3 = *plVar3 + -1;
                UNLOCK();
                if ((int)lVar19 == 1) {
                  (**(code **)*puVar21)(puVar21,1);
                }
              }
            }
            lVar19 = DAT_143aa8328;
            if (local_730 == (undefined8 *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            uVar10 = FUN_14019a5d0(local_730 + 4);
            iVar11 = FUN_140388e30(lVar19,uVar10);
            if ((iVar11 != 0) && (iVar11 = FUN_142cf5c80(DAT_143aa84a0), iVar11 == 0))
            goto LAB_1417de3a6;
            puVar21 = local_730;
            if (local_730 != (undefined8 *)0x0) {
              if (0xffffe < local_730[1] - 1) {
                FUN_142e541f0(0x31e);
              }
              LOCK();
              plVar3 = puVar21 + 1;
              lVar19 = *plVar3;
              *plVar3 = *plVar3 + -1;
              UNLOCK();
              if (((int)lVar19 == 1) && (local_730 != (undefined8 *)0x0)) {
                (**(code **)*local_730)(local_730,1);
              }
            }
            iVar11 = FUN_14022fa10(lVar15,iVar12);
            uVar10 = local_760;
            if (iVar11 < 1) {
              return 0;
            }
          }
          uVar17 = (**(code **)(*DAT_143aa84a0 + 0x30))();
          local_778 = uVar17;
          lVar19 = FUN_141892840();
          if (lVar19 != 0) {
            uVar22 = FUN_141892840();
            iVar9 = FUN_141829fd0(uVar22);
          }
          _eh_vector_constructor_iterator_
                    (local_448,0x10,0x20,(_func_void_void_ptr *)&LAB_1402f7850,FUN_1401ab4e0);
          _eh_vector_constructor_iterator_
                    (local_248,0x10,0x20,(_func_void_void_ptr *)&LAB_1402f7850,FUN_1401ab4e0);
          _eh_vector_constructor_iterator_
                    (local_630,8,5,(_func_void_void_ptr *)&LAB_1402f77e0,
                     (_func_void_void_ptr *)&LAB_1401d33e0);
          FUN_1402fc720(local_630);
          FUN_1403f3210(local_638);
          local_6d0 = (undefined8 *)0x0;
          local_708 = 0;
          FUN_14087f030(iVar9,lVar15,uVar17,local_448);
          lVar19 = local_708;
          if (local_708 != 0) {
            puVar21 = (undefined8 *)(local_708 + -0x28);
            if (0xffffe < *(longlong *)(local_708 + -0x20) - 1U) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            plVar3 = (longlong *)(lVar19 + -0x20);
            lVar19 = *plVar3;
            *plVar3 = *plVar3 + -1;
            UNLOCK();
            if ((int)lVar19 == 1) {
              if ((local_708 != 0) && (*(longlong *)(local_708 + -0x10) != 0)) {
                LOCK();
                *(undefined8 *)(*(longlong *)(local_708 + -0x10) + 8) = 0;
                UNLOCK();
                do {
                } while (*(int *)(*(longlong *)(local_708 + -0x10) + 4) != 0);
              }
              if (puVar21 != (undefined8 *)0x0) {
                (**(code **)*puVar21)(puVar21,1);
              }
            }
            local_708 = 0;
          }
          puVar21 = local_6d0;
          if (local_6d0 != (undefined8 *)0x0) {
            if (0xffffe < local_6d0[1] - 1) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            plVar3 = puVar21 + 1;
            lVar19 = *plVar3;
            *plVar3 = *plVar3 + -1;
            UNLOCK();
            if (((int)lVar19 == 1) && (local_6d0 != (undefined8 *)0x0)) {
              (**(code **)*local_6d0)(local_6d0,1);
            }
          }
          FUN_1408a5630(local_608);
          FUN_140885de0(local_518);
          local_728[1] = 0;
          FUN_14085b3b0(local_608,iVar9,lVar15,local_778);
          uVar14 = FUN_1401ba9d0(local_5fc,local_5f4);
          uVar4 = local_758;
          uVar10 = FUN_1402ea1e0(lVar15,uVar14,local_758,uVar10);
          local_728[0] = CONCAT44(local_728[0]._4_4_,uVar10);
          uVar10 = FUN_1401ba9d0(local_608,local_600);
          uVar5 = local_728[2];
          if (uVar4 - 0x4b0 < 0xe) {
            uVar10 = 2;
          }
          local_788 = (longlong *)CONCAT44(local_788._4_4_,uVar10);
          local_750 = DAT_143aa8328;
          bVar8 = FUN_1401b0050(local_728[2] + 0x16e,*(undefined4 *)(local_728[2] + 0x172));
          local_758 = (uint)bVar8;
          bVar8 = FUN_1401b0050(uVar5 + 0x152,*(undefined4 *)(uVar5 + 0x156));
          local_778 = CONCAT44(local_778._4_4_,(uint)bVar8);
          uVar10 = FUN_1401ba9d0(uVar5 + 0x13a,*(undefined4 *)(uVar5 + 0x142));
          local_740 = CONCAT44(local_740._4_4_,uVar10);
          FUN_1402e51d0(lVar15,uVar4);
          FUN_1401ba9d0(local_5b4,local_5ac);
          FUN_1401ba9d0(local_5c0,local_5b8);
          FUN_1401ba9d0(local_5cc,local_5c4);
          FUN_1401ba9d0(local_5d8,local_5d0);
          FUN_1401ba9d0(local_5e4,local_5dc);
          uVar10 = FUN_1401ba9d0(local_5f0,local_5e8);
          iVar9 = FUN_140397db0(local_750,(ulonglong)local_788 & 0xffffffff,
                                local_728[0] & 0xffffffff,uVar10);
          if (iVar9 == 0) {
            FUN_14025cab0(local_548);
            _eh_vector_destructor_iterator_(local_638,8,1,(_func_void_void_ptr *)&LAB_1401d33e0);
            _eh_vector_destructor_iterator_(local_630,8,5,(_func_void_void_ptr *)&LAB_1401d33e0);
            _eh_vector_destructor_iterator_(local_248,0x10,0x20,FUN_1401ab4e0);
            _eh_vector_destructor_iterator_(local_448,0x10,0x20,FUN_1401ab4e0);
            return 0;
          }
          FUN_14025cab0(local_548);
          _eh_vector_destructor_iterator_(local_638,8,1,(_func_void_void_ptr *)&LAB_1401d33e0);
          _eh_vector_destructor_iterator_(local_630,8,5,(_func_void_void_ptr *)&LAB_1401d33e0);
          _eh_vector_destructor_iterator_(local_248,0x10,0x20,FUN_1401ab4e0);
          _eh_vector_destructor_iterator_(local_448,0x10,0x20,FUN_1401ab4e0);
          iVar11 = local_748[0];
        }
        FUN_142cc5b00(DAT_143aa84a0,local_768,local_770,iVar11);
        return 1;
      }
      uVar17 = FUN_1408a9e40(&local_788,0x4fd);
    }
    else {
      uVar17 = FUN_1408a9e40(&local_788,0x14c7);
    }
  }
  else {
    uVar17 = FUN_1408a9e40(&local_788,0x509);
  }
LAB_1417dd8e5:
  FUN_142a26280(uVar17,0,0,1);
  return 0;
}



//===========================================================
// FUN_140253980 @ 140253980   (739 bytes)
//===========================================================

bool FUN_140253980(int param_1,uint param_2,int param_3,char param_4)

{
  int iVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  bool bVar5;
  
  iVar1 = FUN_1402531f0();
  if ((((iVar1 == 0) && (iVar1 = FUN_140416760(param_1), iVar1 == 0)) &&
      (iVar1 = FUN_140416820(param_1), iVar1 == 0)) &&
     (((iVar1 = FUN_140253130(param_1), param_3 != 2 && (iVar1 != 2)) && (iVar1 != param_3)))) {
    return false;
  }
  uVar2 = param_2 - 0x708;
  iVar1 = 0;
  if (uVar2 < 0x33) {
    uVar3 = 0;
    iVar4 = iVar1;
    do {
      if ((iVar1 <= (int)uVar2) && ((int)uVar2 < iVar1 + 0x11)) {
        uVar3 = param_2 + iVar4 * -0x11 + -0x708;
        break;
      }
      iVar4 = iVar4 + 1;
      iVar1 = iVar1 + 0x11;
    } while (iVar1 < 0x33);
  }
  else if (((param_2 - 3000 < 0x20) || (param_2 - 0xc1c < 0x20)) ||
          (uVar3 = param_2, param_2 - 0xc80 < 0x20)) {
    iVar1 = (int)((ulonglong)((longlong)(int)param_2 * -0x51eb851f) >> 0x20);
    uVar3 = param_2 + ((iVar1 >> 5) - (iVar1 >> 0x1f)) * 100;
  }
  switch(param_1 / 10000) {
  case 100:
    if (uVar3 == 1) {
      return true;
    }
    bVar5 = uVar3 == 0x4b0;
    break;
  case 0x65:
    if (uVar3 == 2) {
      return true;
    }
    bVar5 = uVar3 == 0x4b2;
    break;
  case 0x66:
    if (uVar3 == 3) {
      return true;
    }
    bVar5 = uVar3 == 0x4b8;
    break;
  case 0x67:
    if (uVar3 == 4) {
      return true;
    }
    bVar5 = uVar3 == 0x4b9;
    break;
  case 0x68:
  case 0x69:
    if (uVar3 == 5) {
      return true;
    }
    bVar5 = uVar3 == 0x4b3;
    break;
  case 0x6a:
    if (uVar3 == 6) {
      return true;
    }
    bVar5 = uVar3 == 0x4b4;
    break;
  case 0x6b:
    if (uVar3 == 7) {
      return true;
    }
    bVar5 = uVar3 == 0x4b5;
    break;
  case 0x6c:
    if (uVar3 == 8) {
      return true;
    }
    bVar5 = uVar3 == 0x4b6;
    break;
  case 0x6e:
    if (uVar3 == 9) {
      return true;
    }
    bVar5 = uVar3 == 0x4b1;
    break;
  case 0x6f:
    if ((uVar3 < 0x11) && ((0x1b000U >> (uVar3 & 0x1f) & 1) != 0)) {
      return true;
    }
    if (uVar3 - 0x4ba < 4) {
      return true;
    }
    return false;
  case 0x70:
    if (uVar3 == 0x11) {
      return true;
    }
    bVar5 = uVar3 == 0x1f;
    break;
  case 0x71:
    return uVar3 == 0x16;
  case 0x72:
    return uVar3 == 0x15;
  case 0x73:
    return uVar3 == 0x17;
  case 0x74:
    return uVar3 == 0x1a;
  default:
    iVar1 = FUN_140255180(param_1);
    if (((iVar1 == 0) && (99999 < param_1 - 1600000U)) && (param_1 / 10000 != 0xaa)) {
      return false;
    }
    if (uVar3 == 0xb) {
      return true;
    }
    bVar5 = uVar3 == 0x4b7;
    break;
  case 0x76:
    return uVar3 == 0x1d;
  case 0x77:
    return uVar3 == 0x1e;
  case 0x9c:
    if (param_4 != '\0') {
      return uVar3 - 10 < 2;
    }
  case 0x6d:
  case 0x86:
  case 0x87:
    return uVar3 == 10;
  case 0xa6:
    return uVar3 == 0x1b;
  case 0xa7:
    return (uVar3 - 0x1c & 0xfffffffd) == 0;
  case 0xb4:
    if (uVar3 == 0xe) {
      return true;
    }
    if (uVar3 - 0x18 < 2) {
      return true;
    }
    return false;
  case 0xbe:
    return uVar3 == 0x12;
  case 0xbf:
    return uVar3 == 0x13;
  case 0xc0:
    return uVar3 == 0x14;
  }
  if (bVar5) {
    return true;
  }
  return false;
}



//===========================================================
// FUN_140397db0 @ 140397db0   (2664 bytes)
//===========================================================

undefined4
FUN_140397db0(undefined8 param_1,undefined4 param_2,uint param_3,int param_4,short param_5,
             int param_6,int param_7,int param_8,int param_9,int param_10,longlong param_11,
             int param_12,int param_13,longlong param_14,int param_15,int param_16)

{
  uint uVar1;
  byte bVar2;
  byte bVar3;
  char cVar4;
  undefined4 uVar5;
  int iVar6;
  longlong lVar7;
  longlong lVar8;
  int *piVar9;
  int iVar10;
  int iVar11;
  byte *pbVar12;
  uint uVar13;
  uint local_res18;
  undefined4 local_c8;
  ulonglong local_c0;
  int local_b8;
  int local_b4;
  byte *local_b0;
  int local_a8;
  int local_a0;
  int local_9c;
  byte *local_98;
  int local_90;
  int local_88;
  int local_84;
  byte *local_80;
  int local_78;
  longlong local_70;
  longlong local_68;
  int local_60;
  longlong local_58;
  
  local_c0 = local_c0 & 0xffffffff00000000;
  local_58 = FUN_140388c60(param_1,param_12);
  if ((local_58 == 0) || ((0 < *(int *)(local_58 + 0x390) && (param_13 == 0)))) {
    return 0;
  }
  local_60 = FUN_140255180(param_12);
  if (*(longlong *)(param_14 + 0xb8) == 0) {
    local_a0 = 0;
    local_98 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
    iVar10 = (int)&local_a0 + -0x3ff8;
    local_9c = FUN_142f04924();
    local_9c = local_9c + iVar10;
    local_90 = FUN_142f04924();
    pbVar12 = local_98;
    local_90 = local_90 + iVar10;
    local_98[5] = (byte)local_9c;
    local_98[6] = (byte)local_90;
    local_c8 = 0;
    local_a0 = local_a0 + 1;
    if (local_a0 == (local_a0 / 0x6f) * 0x6f) {
      local_98 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
      *(undefined8 *)local_98 = *(undefined8 *)pbVar12;
      *(undefined4 *)(local_98 + 8) = *(undefined4 *)(pbVar12 + 8);
      thunk_FUN_140205820(pbVar12,0xc);
    }
    bVar2 = FUN_142f04924();
    local_98[4] = bVar2;
    local_98[8] = 0x65;
    local_98[9] = 0x9a;
    uVar13 = 0;
    local_68 = (longlong)&local_c8 + (1 - (longlong)local_98);
    local_70 = (longlong)&local_c8 + (2 - (longlong)local_98);
    pbVar12 = local_98;
    do {
      if (bVar2 == 0) {
        bVar2 = 0x2a;
      }
      bVar3 = pbVar12[(longlong)&local_c8 - (longlong)local_98];
      *pbVar12 = bVar2 ^ bVar3;
      bVar2 = bVar2 + (bVar2 ^ bVar3) + 0x2a;
      *(ushort *)(local_98 + 8) =
           (*(ushort *)(local_98 + 8) >> 0xd) + (ushort)bVar2 | *(ushort *)(local_98 + 8) << 3;
      bVar3 = 0x2a;
      if (bVar2 != 0) {
        bVar3 = bVar2;
      }
      bVar2 = pbVar12[local_68];
      pbVar12[1] = bVar3 ^ bVar2;
      bVar3 = (bVar3 ^ bVar2) + bVar3 + 0x2a;
      *(ushort *)(local_98 + 8) =
           (*(ushort *)(local_98 + 8) >> 0xd) + (ushort)bVar3 | *(ushort *)(local_98 + 8) << 3;
      bVar2 = 0x2a;
      if (bVar3 != 0) {
        bVar2 = bVar3;
      }
      bVar3 = pbVar12[local_70];
      pbVar12[2] = bVar2 ^ bVar3;
      bVar3 = (bVar2 ^ bVar3) + bVar2 + 0x2a;
      *(ushort *)(local_98 + 8) =
           (*(ushort *)(local_98 + 8) >> 0xd) + (ushort)bVar3 | *(ushort *)(local_98 + 8) << 3;
      bVar2 = 0x2a;
      if (bVar3 != 0) {
        bVar2 = bVar3;
      }
      bVar3 = pbVar12[(longlong)&local_c8 + (3 - (longlong)local_98)];
      pbVar12[3] = bVar2 ^ bVar3;
      bVar2 = (bVar2 ^ bVar3) + bVar2 + 0x2a;
      *(ushort *)(local_98 + 8) =
           (*(ushort *)(local_98 + 8) >> 0xd) + (ushort)bVar2 | *(ushort *)(local_98 + 8) << 3;
      uVar13 = uVar13 + 4;
      pbVar12 = pbVar12 + 4;
    } while (uVar13 < 4);
    local_c0 = CONCAT44(local_c0._4_4_,2);
    uVar5 = FUN_14019a5d0(&local_a0);
    local_b8 = 0;
    local_b0 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
    iVar10 = (int)&local_b8 + -0x3ff8;
    local_b4 = FUN_142f04924();
    local_b4 = local_b4 + iVar10;
    local_a8 = FUN_142f04924();
    pbVar12 = local_b0;
    local_a8 = local_a8 + iVar10;
    local_b0[5] = (byte)local_b4;
    local_b0[6] = (byte)local_a8;
    local_b8 = local_b8 + 1;
    local_c8 = uVar5;
    if (local_b8 == (local_b8 / 0x6f) * 0x6f) {
      local_b0 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
      *(undefined8 *)local_b0 = *(undefined8 *)pbVar12;
      *(undefined4 *)(local_b0 + 8) = *(undefined4 *)(pbVar12 + 8);
      thunk_FUN_140205820(pbVar12,0xc);
    }
    bVar2 = FUN_142f04924();
    local_b0[4] = bVar2;
    local_b0[8] = 0x65;
    local_b0[9] = 0x9a;
    uVar13 = 0;
    local_68 = (longlong)&local_c8 + (1 - (longlong)local_b0);
    local_70 = (longlong)&local_c8 + (2 - (longlong)local_b0);
    pbVar12 = local_b0;
    do {
      if (bVar2 == 0) {
        bVar2 = 0x2a;
      }
      bVar3 = pbVar12[(longlong)&local_c8 - (longlong)local_b0];
      *pbVar12 = bVar2 ^ bVar3;
      bVar2 = bVar2 + (bVar2 ^ bVar3) + 0x2a;
      *(ushort *)(local_b0 + 8) =
           (*(ushort *)(local_b0 + 8) >> 0xd) + (ushort)bVar2 | *(ushort *)(local_b0 + 8) << 3;
      bVar3 = 0x2a;
      if (bVar2 != 0) {
        bVar3 = bVar2;
      }
      bVar2 = pbVar12[local_68];
      pbVar12[1] = bVar3 ^ bVar2;
      bVar3 = (bVar3 ^ bVar2) + bVar3 + 0x2a;
      *(ushort *)(local_b0 + 8) =
           (*(ushort *)(local_b0 + 8) >> 0xd) + (ushort)bVar3 | *(ushort *)(local_b0 + 8) << 3;
      bVar2 = 0x2a;
      if (bVar3 != 0) {
        bVar2 = bVar3;
      }
      bVar3 = pbVar12[local_70];
      pbVar12[2] = bVar2 ^ bVar3;
      bVar3 = (bVar2 ^ bVar3) + bVar2 + 0x2a;
      *(ushort *)(local_b0 + 8) =
           (*(ushort *)(local_b0 + 8) >> 0xd) + (ushort)bVar3 | *(ushort *)(local_b0 + 8) << 3;
      bVar2 = 0x2a;
      if (bVar3 != 0) {
        bVar2 = bVar3;
      }
      bVar3 = pbVar12[(longlong)&local_c8 + (3 - (longlong)local_b0)];
      pbVar12[3] = bVar2 ^ bVar3;
      bVar2 = (bVar2 ^ bVar3) + bVar2 + 0x2a;
      *(ushort *)(local_b0 + 8) =
           (*(ushort *)(local_b0 + 8) >> 0xd) + (ushort)bVar2 | *(ushort *)(local_b0 + 8) << 3;
      uVar13 = uVar13 + 4;
      pbVar12 = pbVar12 + 4;
    } while (uVar13 < 4);
    piVar9 = &local_b8;
    uVar13 = 6;
  }
  else {
    uVar5 = FUN_14019a5d0(*(longlong *)(param_14 + 0xb8) + 0x20);
    local_88 = 0;
    local_80 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
    iVar10 = (int)&local_88 + -0x3ff8;
    local_84 = FUN_142f04924();
    local_84 = local_84 + iVar10;
    local_78 = FUN_142f04924();
    pbVar12 = local_80;
    local_78 = local_78 + iVar10;
    local_80[5] = (byte)local_84;
    local_80[6] = (byte)local_78;
    local_88 = local_88 + 1;
    local_c8 = uVar5;
    if (local_88 == (local_88 / 0x6f) * 0x6f) {
      local_80 = (byte *)FUN_14019b780(&DAT_143ad68a0,0xc);
      *(undefined8 *)local_80 = *(undefined8 *)pbVar12;
      *(undefined4 *)(local_80 + 8) = *(undefined4 *)(pbVar12 + 8);
      thunk_FUN_140205820(pbVar12,0xc);
    }
    bVar2 = FUN_142f04924();
    local_80[4] = bVar2;
    local_80[8] = 0x65;
    local_80[9] = 0x9a;
    uVar13 = 0;
    local_c0 = (longlong)&local_c8 + (1 - (longlong)local_80);
    local_70 = (longlong)&local_c8 + (2 - (longlong)local_80);
    pbVar12 = local_80;
    do {
      if (bVar2 == 0) {
        bVar2 = 0x2a;
      }
      bVar3 = pbVar12[(longlong)&local_c8 - (longlong)local_80];
      *pbVar12 = bVar2 ^ bVar3;
      bVar2 = bVar2 + (bVar2 ^ bVar3) + 0x2a;
      *(ushort *)(local_80 + 8) =
           (*(ushort *)(local_80 + 8) >> 0xd) + (ushort)bVar2 | *(ushort *)(local_80 + 8) << 3;
      bVar3 = 0x2a;
      if (bVar2 != 0) {
        bVar3 = bVar2;
      }
      bVar2 = pbVar12[local_c0];
      pbVar12[1] = bVar3 ^ bVar2;
      bVar3 = (bVar3 ^ bVar2) + bVar3 + 0x2a;
      *(ushort *)(local_80 + 8) =
           (*(ushort *)(local_80 + 8) >> 0xd) + (ushort)bVar3 | *(ushort *)(local_80 + 8) << 3;
      bVar2 = 0x2a;
      if (bVar3 != 0) {
        bVar2 = bVar3;
      }
      bVar3 = pbVar12[local_70];
      pbVar12[2] = bVar2 ^ bVar3;
      bVar3 = (bVar2 ^ bVar3) + bVar2 + 0x2a;
      *(ushort *)(local_80 + 8) =
           (*(ushort *)(local_80 + 8) >> 0xd) + (ushort)bVar3 | *(ushort *)(local_80 + 8) << 3;
      bVar2 = 0x2a;
      if (bVar3 != 0) {
        bVar2 = bVar3;
      }
      bVar3 = pbVar12[(longlong)&local_c8 + (3 - (longlong)local_80)];
      pbVar12[3] = bVar2 ^ bVar3;
      bVar2 = (bVar2 ^ bVar3) + bVar2 + 0x2a;
      *(ushort *)(local_80 + 8) =
           (*(ushort *)(local_80 + 8) >> 0xd) + (ushort)bVar2 | *(ushort *)(local_80 + 8) << 3;
      uVar13 = uVar13 + 4;
      pbVar12 = pbVar12 + 4;
    } while (uVar13 < 4);
    piVar9 = &local_88;
    uVar13 = 1;
  }
  lVar8 = local_58;
  local_c0 = CONCAT44(local_c0._4_4_,uVar13);
  uVar5 = FUN_14019a5d0(piVar9);
  if (((uVar13 & 4) != 0) && (uVar13 = uVar13 & 0xfffffffb, local_b0 != (byte *)0x0)) {
    thunk_FUN_140205820(local_b0,0xc);
  }
  if (((uVar13 & 2) != 0) && (uVar13 = uVar13 & 0xfffffffd, local_98 != (byte *)0x0)) {
    thunk_FUN_140205820(local_98,0xc);
  }
  if (((uVar13 & 1) != 0) && (local_80 != (byte *)0x0)) {
    thunk_FUN_140205820(local_80,0xc);
  }
  if (local_60 == 0x22) {
    iVar10 = FUN_140255430(uVar5);
    if (iVar10 != 0) {
      return 0;
    }
    if ((4 < param_4 - 0x1aeU) && (param_4 != 900)) {
      return 0;
    }
  }
  cVar4 = FUN_140397c50(param_1,param_12,uVar5,param_4,(int)param_5,*(undefined4 *)(lVar8 + 0x18));
  if (cVar4 == '\0') {
    return 0;
  }
  iVar10 = FUN_140253980(param_12,0xe,param_2);
  local_res18 = param_3;
  if (iVar10 != 0) {
    if (param_11 == 0) {
      return 0;
    }
    iVar10 = FUN_14019a5d0(param_11 + 0x20);
    if (9999 < *(int *)(lVar8 + 0x38) - 1800000U) {
      return 0;
    }
    if (9999 < iVar10 - 5000000U) {
      return 0;
    }
    piVar9 = *(int **)(lVar8 + 0x1e8);
    while( true ) {
      if (piVar9 == *(int **)(lVar8 + 0x1f0)) {
        return 0;
      }
      if (*piVar9 == iVar10) break;
      piVar9 = piVar9 + 1;
    }
    bVar2 = FUN_1401b0050(param_11 + 0x5a,*(undefined4 *)(param_11 + 0x5e));
    local_res18 = (uint)bVar2;
  }
  if ((param_12 - 0x195460U < 10000) && (*(longlong *)(param_14 + 0x1c8) != 0)) {
    lVar7 = FUN_1401a19e0(param_14 + 0x1c0);
    uVar5 = FUN_14019a5d0(lVar7 + 0x20);
    lVar7 = FUN_140388c60(param_1,uVar5);
    if ((lVar7 != 0) && (*(byte *)(lVar8 + 0x173) < *(byte *)(lVar7 + 0x173))) {
      return 0;
    }
  }
  else {
    iVar10 = FUN_140416820(param_12);
    if ((iVar10 != 0) && (*(longlong *)(param_14 + 0x1b8) != 0)) {
      lVar7 = FUN_1401a19e0(param_14 + 0x1b0);
      uVar5 = FUN_14019a5d0(lVar7 + 0x20);
      lVar7 = FUN_140388c60(param_1,uVar5);
      if ((lVar7 != 0) && (*(byte *)(lVar7 + 0x173) < *(byte *)(lVar8 + 0x173))) {
        return 0;
      }
    }
  }
  iVar10 = FUN_140398ac0(param_1,lVar8 + 0x80,param_12,param_4,*(undefined4 *)(lVar8 + 0x18));
  if (iVar10 == 0) {
    return 0;
  }
  if ((*(int *)(lVar8 + 0x70) != 0) && (param_4 / 100 != *(int *)(lVar8 + 0x70))) {
    return 0;
  }
  if ((param_12 - 0x10fc0aU < 0x1f) || (param_12 - 0x10fcf1U < 2)) {
    iVar11 = 0;
    iVar10 = 0;
    do {
      iVar6 = FUN_1402537f0(iVar10);
      lVar8 = *(longlong *)(param_14 + 8 + (longlong)iVar6 * 0x10);
      if ((lVar8 != 0) &&
         ((iVar6 = FUN_14019a5d0(lVar8 + 0x20), iVar6 - 0x10fc0aU < 0x1f || (iVar6 - 0x10fcf1U < 2))
         )) {
        iVar11 = iVar11 + 1;
      }
      iVar10 = iVar10 + 1;
    } while (iVar10 < 4);
    if (1 < iVar11) {
      return 0;
    }
  }
  lVar8 = FUN_140388c60(param_1,param_12);
  if (lVar8 == 0) {
    return 0;
  }
  iVar10 = (param_4 % 1000) / 100;
  if (iVar10 == 0) {
    uVar13 = 0;
  }
  else {
    uVar13 = 1 << ((char)iVar10 - 1U & 0x1f);
    if (iVar10 == 9) {
      return 1;
    }
    if (iVar10 == 8) {
      return 1;
    }
  }
  iVar10 = FUN_140253330(param_12,param_2);
  if (iVar10 == 0) {
    return 0;
  }
  if ((*(int *)(lVar8 + 0x78) - param_15) + param_16 <= (int)local_res18) {
    if (param_6 < *(int *)(lVar8 + 0x58)) {
      return 0;
    }
    if (param_7 < *(int *)(lVar8 + 0x60)) {
      return 0;
    }
    if (param_8 < *(int *)(lVar8 + 0x5c)) {
      return 0;
    }
    if (*(int *)(lVar8 + 100) <= param_9) {
      if ((*(int *)(lVar8 + 0x68) != 0) && (param_10 < *(int *)(lVar8 + 0x68))) {
        return 0;
      }
      uVar1 = *(uint *)(lVar8 + 0x6c);
      if (uVar1 != 0) {
        if (uVar1 == 0xffffffff) {
          if (uVar13 != 0) {
            return 0;
          }
        }
        else {
          if ((int)uVar1 < 1) {
            return 0;
          }
          if ((uVar13 & uVar1) == 0) {
            return 0;
          }
        }
      }
      return 1;
    }
    return 0;
  }
  return 0;
}



//===========================================================
// FUN_140397680 @ 140397680   (753 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

ulonglong FUN_140397680(undefined8 param_1,undefined8 param_2,int param_3,longlong param_4,
                       longlong param_5,longlong *param_6,int param_7,int param_8)

{
  longlong lVar1;
  char cVar2;
  int iVar3;
  undefined4 uVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  longlong lVar8;
  ulonglong uVar9;
  undefined1 auStack_78 [32];
  undefined8 local_58;
  ulonglong local_50;
  
  local_50 = DAT_143a8b908 ^ (ulonglong)auStack_78;
  if (param_8 - 0x4b0U < 0xe) {
    lVar8 = *param_6;
    uVar7 = 0;
    if ((lVar8 == 0) || (uVar7 = *(uint *)(lVar8 + -8), uVar7 < 4)) {
      FUN_142e54290(0xc6,3,uVar7);
      lVar8 = *param_6;
    }
    lVar1 = *(longlong *)(lVar8 + 0x38);
    if (*(uint *)(lVar8 + -8) < 5) {
      FUN_142e54290(0xc6,4);
      lVar8 = *param_6;
    }
    if (param_3 == 0) {
      if (lVar1 == 0) {
        return 0;
      }
      if (*(longlong *)(lVar8 + 0x48) == 0) {
        return 0;
      }
      iVar3 = FUN_1401b0340(lVar1 + 0x20);
      uVar7 = 0;
      if (iVar3 / 10000 == 0x69) {
        uVar7 = 0x4b3;
      }
      return (ulonglong)uVar7;
    }
    if ((param_3 - 0x100590U < 10000) && (*(longlong *)(lVar8 + 0x48) != 0)) {
      return 0x4b4;
    }
    if (9999 < param_3 - 0x102ca0U) {
      return 0;
    }
    if (lVar1 == 0) {
      return 0;
    }
    iVar3 = FUN_1401b0340(lVar1 + 0x20);
    if (iVar3 / 10000 != 0x69) {
      return 0;
    }
    return 0x4b3;
  }
  iVar3 = FUN_1402536d0(param_8);
  if (iVar3 != 0) {
    cVar2 = FUN_1402e9d20(param_2);
    uVar4 = FUN_1402e9c70(param_2);
    iVar3 = FUN_1402e9dd0(param_2,uVar4);
    iVar5 = FUN_140253660(param_8);
    if ((iVar3 == iVar5) && (cVar2 != '\0')) {
      FUN_1403f3210(&local_58);
      FUN_1402e3e60(param_2,&local_58,0xffffffff);
      uVar9 = 0;
      iVar5 = 0;
      if (param_8 - 0x708U < 0x33) {
        iVar5 = FUN_140397a80(param_1,local_58,param_3);
        iVar6 = FUN_140253500(iVar5);
        if (iVar6 != 0) {
          iVar5 = iVar5 + 100;
        }
      }
      iVar3 = FUN_1402535e0(-iVar5,iVar3);
      iVar6 = FUN_1402536d0(iVar3 + iVar5);
      if (iVar6 != 0) {
        uVar9 = (ulonglong)(uint)(iVar3 + iVar5);
      }
      _eh_vector_destructor_iterator_(&local_58,8,1,(_func_void_void_ptr *)&LAB_1401d33e0);
      return uVar9;
    }
    return 0;
  }
  if (param_7 != 0) {
    param_4 = param_5;
  }
  lVar8 = *(longlong *)(param_4 + 0xa8);
  lVar1 = *(longlong *)(param_4 + 0xb8);
  if (param_3 == 0) {
    if ((lVar8 != 0) && (lVar1 != 0)) {
      uVar4 = FUN_1401b0340(lVar1 + 0x20);
      iVar3 = FUN_140255430(uVar4);
      uVar7 = -(uint)(iVar3 != 0) & 0xb;
      if (uVar7 != 0) {
        return (ulonglong)uVar7;
      }
    }
  }
  else {
    iVar3 = FUN_140255430(param_3);
    if ((iVar3 != 0) && (lVar8 != 0)) {
      uVar4 = FUN_1401b0340(lVar8 + 0x20);
      iVar3 = FUN_1404166a0(uVar4);
      if (iVar3 != 0) goto LAB_140397945;
    }
    iVar3 = FUN_140255430(param_3);
    if ((iVar3 != 0) && (lVar8 != 0)) {
      return 10;
    }
    if ((param_3 - 0x10a1d0U < 10000) && (lVar1 != 0)) {
      uVar4 = FUN_1401b0340(lVar1 + 0x20);
      iVar3 = FUN_140255430(uVar4);
      if (iVar3 != 0) {
        return 0xb;
      }
    }
  }
LAB_140397945:
  uVar9 = FUN_140397a80(param_1,param_4,param_3);
  return uVar9;
}


