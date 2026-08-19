
//===========================================================
// FUN_1426b20f0 @ 1426b20f0   (7466 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x0001426b2b3a) */
/* WARNING: Removing unreachable block (ram,0x0001426b275b) */
/* WARNING: Removing unreachable block (ram,0x0001426b258d) */
/* WARNING: Removing unreachable block (ram,0x0001426b2925) */
/* WARNING: Removing unreachable block (ram,0x0001426b2d22) */
/* WARNING: Type propagation algorithm not settling */

void FUN_1426b20f0(longlong param_1,longlong *param_2,longlong param_3)

{
  longlong lVar1;
  char cVar2;
  undefined1 uVar3;
  byte bVar4;
  short sVar5;
  short sVar6;
  undefined4 uVar7;
  int iVar8;
  int iVar9;
  undefined1 *puVar10;
  undefined8 uVar11;
  int *piVar12;
  int *piVar13;
  int *piVar14;
  longlong lVar15;
  longlong *plVar16;
  uint uVar17;
  undefined8 *puVar18;
  ulonglong uVar19;
  int iVar20;
  ulonglong uVar21;
  ulonglong uVar22;
  ulonglong uVar23;
  undefined4 local_res10 [2];
  int *local_res18;
  int *local_res20;
  int *local_328;
  int *local_320;
  int *local_318;
  int *local_310;
  longlong local_308;
  longlong local_300;
  longlong local_2f8;
  longlong local_2f0;
  longlong local_2e8 [38];
  int *local_1b8;
  undefined8 local_1b0;
  longlong local_1a8 [6];
  undefined4 local_178;
  undefined8 local_174;
  undefined8 local_16c;
  undefined8 local_164;
  undefined8 local_15c;
  undefined8 local_154;
  undefined8 local_14c;
  undefined8 local_144;
  undefined8 local_13c;
  undefined8 local_134;
  undefined8 local_12c;
  longlong local_118;
  undefined1 local_110 [8];
  longlong local_108;
  longlong local_100;
  longlong local_f8;
  longlong local_f0;
  longlong local_e8;
  longlong local_e0;
  longlong local_d8;
  longlong local_d0;
  longlong local_c8;
  longlong local_c0;
  longlong local_b8;
  longlong local_b0;
  longlong local_a8;
  longlong local_a0;
  longlong local_98;
  longlong local_90;
  longlong local_88;
  undefined1 local_80 [8];
  longlong local_78;
  undefined1 local_70 [8];
  longlong local_68;
  int *local_60;
  int *local_58;
  int *local_50;
  int *local_48;
  int *local_40;
  
  uVar11 = DAT_143aa8328;
  uVar19 = 0;
  local_108 = 0;
  uVar7 = FUN_1401b0340(param_2 + 4);
  local_300 = FUN_140388c60(uVar11,uVar7);
  uVar23 = 0xffffffffffffffff;
  lVar15 = local_300;
  if (param_3 != 0) {
    puVar10 = (undefined1 *)FUN_140385460(DAT_143aa8328,local_70,param_3);
    if (local_110 == puVar10) {
      FUN_142e52d50(0x45c,1);
    }
    lVar15 = *(longlong *)(puVar10 + 8);
    if (lVar15 != 0) {
      if (0xfffff < *(ulonglong *)(lVar15 + -0x20)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar15 + -0x20) = *(longlong *)(lVar15 + -0x20) + 1;
      UNLOCK();
    }
    lVar1 = local_68;
    local_108 = *(longlong *)(puVar10 + 8);
    lVar15 = local_108;
    if (local_68 != 0) {
      puVar18 = (undefined8 *)(local_68 + -0x28);
      if (0xffffe < *(longlong *)(local_68 + -0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar16 = (longlong *)(lVar1 + -0x20);
      lVar1 = *plVar16;
      *plVar16 = *plVar16 + -1;
      UNLOCK();
      lVar15 = local_108;
      if ((int)lVar1 == 1) {
        if ((local_68 != 0) && (*(longlong *)(local_68 + -0x10) != 0)) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_68 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_68 + -0x10) + 4) != 0);
        }
        if (puVar18 != (undefined8 *)0x0) {
          (**(code **)*puVar18)(puVar18,1);
          lVar15 = local_108;
        }
      }
    }
  }
  local_300 = lVar15;
  if (lVar15 == 0) goto LAB_1426b3d8e;
  local_14c = 0;
  local_12c = 0;
  local_134 = 0;
  local_13c = 0;
  local_144 = 0;
  local_154 = 0;
  local_15c = 0;
  local_164 = 0;
  local_16c = 0;
  local_174 = 0;
  local_178 = 0;
  FUN_140396250(DAT_143aa8328,&local_178,param_2,0,0);
  FUN_1426aed50(param_1,param_2,lVar15);
  FUN_1426af680(param_1,param_2,lVar15,&local_178);
  FUN_1426af8c0(param_1,param_2,lVar15,&local_178);
  FUN_1426afdc0(param_1,param_2,lVar15,&local_178);
  local_res18 = (int *)0x0;
  uVar11 = FUN_1408a9e40(&local_100,0x38a);
  sVar5 = FUN_1401ab420((longlong)param_2 + 0x343,*(undefined4 *)((longlong)param_2 + 0x347));
  sVar6 = FUN_1401ab420((longlong)param_2 + 0x92,*(undefined4 *)((longlong)param_2 + 0x96));
  FUN_142699710(param_1,&local_res18,(int)*(short *)(lVar15 + 0xdc),local_13c & 0xffffffff,
                (int)sVar6,(int)sVar5,uVar11);
  if (local_100 != 0) {
    FUN_14019f2c0(local_100 + -0x10);
  }
  uVar11 = FUN_1408a9e40(&local_f8,0x38b);
  sVar5 = FUN_1401ab420((longlong)param_2 + 0x34b,*(undefined4 *)((longlong)param_2 + 0x34f));
  sVar6 = FUN_1401ab420((longlong)param_2 + 0x9a,*(undefined4 *)((longlong)param_2 + 0x9e));
  FUN_142699710(param_1,&local_res18,(int)*(short *)(lVar15 + 0xde),local_13c._4_4_,(int)sVar6,
                (int)sVar5,uVar11);
  if (local_f8 != 0) {
    FUN_14019f2c0(local_f8 + -0x10);
  }
  FUN_1426b0250(param_1,lVar15);
  FUN_1426b04a0(param_1,param_2);
  uVar11 = DAT_143aa8328;
  local_res10[0] = 0;
  uVar7 = FUN_1401b0340(param_2 + 4);
  iVar8 = FUN_140388eb0(uVar11,uVar7,local_res10);
  iVar9 = 0;
  if (0 < iVar8) {
    puVar18 = (undefined8 *)FUN_1408a9e40(&local_f0,0x38c);
    FUN_14019ba10(&local_res18,*puVar18,iVar8);
    if (local_f0 != 0) {
      FUN_14019f2c0(local_f0 + -0x10);
    }
    piVar12 = local_res18;
    local_res20 = (int *)0x0;
    piVar13 = local_res20;
    if ((local_res18 != (int *)0x0) && (piVar14 = local_res18 + -4, piVar14 != (int *)0x0)) {
      if (*piVar14 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar21 = uVar23;
        do {
          uVar21 = uVar21 + 1;
        } while (*(char *)((longlong)piVar12 + uVar21) != '\0');
        iVar20 = (int)uVar21;
        iVar8 = iVar9;
        if (0 < iVar20) {
          iVar8 = iVar20;
        }
        piVar12 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar8 + 0x11));
        piVar12[1] = iVar8;
        *piVar12 = -1;
        piVar13 = piVar12 + 4;
        piVar12[2] = 0;
        *(undefined1 *)piVar13 = 0;
        local_60 = piVar13;
        FUN_142ef7ba0(piVar13,local_res18,(longlong)iVar20);
        if (*piVar12 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar20 == -1) || (iVar20 <= piVar12[1])) {
          *piVar12 = 1;
          if (iVar20 != -1) goto LAB_1426b2545;
          uVar22 = uVar23;
          uVar21 = uVar19;
          if (piVar13 != (int *)0x0) {
            do {
              uVar21 = uVar22 + 1;
              uVar22 = uVar21;
            } while (*(char *)((longlong)piVar13 + uVar21) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar12[1],uVar21 & 0xffffffff);
          *piVar12 = 1;
LAB_1426b2545:
          *(undefined1 *)((longlong)piVar13 + (longlong)iVar20) = 0;
        }
        iVar8 = (int)uVar21;
        if ((iVar8 < 0) || (piVar12[1] + 1 <= iVar8)) {
          FUN_142e54290(0x9c,uVar21 & 0xffffffff);
        }
        piVar12[2] = iVar8;
        if (local_res20 != (int *)0x0) {
          FUN_14019f2c0(local_res20 + -4);
        }
      }
      else {
        if (*piVar14 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar14 = *piVar14 + 1;
        UNLOCK();
        if (local_res20 != (int *)0x0) {
          FUN_14019f2c0(local_res20 + -4);
        }
        local_res20 = piVar12;
        piVar13 = local_res20;
      }
    }
    local_res20 = piVar13;
    FUN_14269a1d0(param_1,0x28,&local_res20,0x3e9,0,0);
  }
  iVar8 = (**(code **)(*param_2 + 0x18))(param_2);
  if (iVar8 != 0) {
    puVar18 = (undefined8 *)FUN_1408a9e40(&local_e8,0x392);
    FUN_14019ba10(&local_res18,*puVar18);
    if (local_e8 != 0) {
      FUN_14019f2c0(local_e8 + -0x10);
    }
    piVar12 = local_res18;
    local_328 = (int *)0x0;
    piVar13 = local_328;
    if ((local_res18 != (int *)0x0) && (piVar14 = local_res18 + -4, piVar14 != (int *)0x0)) {
      if (*piVar14 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar21 = uVar23;
        do {
          uVar21 = uVar21 + 1;
        } while (*(char *)((longlong)piVar12 + uVar21) != '\0');
        iVar20 = (int)uVar21;
        iVar8 = iVar9;
        if (0 < iVar20) {
          iVar8 = iVar20;
        }
        piVar12 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar8 + 0x11));
        piVar12[1] = iVar8;
        *piVar12 = -1;
        piVar13 = piVar12 + 4;
        piVar12[2] = 0;
        *(undefined1 *)piVar13 = 0;
        local_58 = piVar13;
        FUN_142ef7ba0(piVar13,local_res18,(longlong)iVar20);
        if (*piVar12 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar20 == -1) || (iVar20 <= piVar12[1])) {
          *piVar12 = 1;
          if (iVar20 != -1) goto LAB_1426b2717;
          uVar22 = uVar23;
          uVar21 = uVar19;
          if (piVar13 != (int *)0x0) {
            do {
              uVar21 = uVar22 + 1;
              uVar22 = uVar21;
            } while (*(char *)((longlong)piVar13 + uVar21) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar12[1],uVar21 & 0xffffffff);
          *piVar12 = 1;
LAB_1426b2717:
          *(undefined1 *)((longlong)iVar20 + (longlong)piVar13) = 0;
        }
        iVar8 = (int)uVar21;
        if ((iVar8 < 0) || (piVar12[1] + 1 <= iVar8)) {
          FUN_142e54290(0x9c,uVar21 & 0xffffffff);
        }
        piVar12[2] = iVar8;
        if (local_328 != (int *)0x0) {
          FUN_14019f2c0(local_328 + -4);
        }
      }
      else {
        if (*piVar14 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar14 = *piVar14 + 1;
        UNLOCK();
        if (local_328 != (int *)0x0) {
          FUN_14019f2c0(local_328 + -4);
        }
        local_328 = piVar12;
        piVar13 = local_328;
      }
    }
    local_328 = piVar13;
    FUN_14269a1d0(param_1,0x28,&local_328,0x3e9,0,0);
  }
  iVar8 = (**(code **)(*param_2 + 0x20))(param_2);
  if (iVar8 != 0) {
    puVar18 = (undefined8 *)FUN_1408a9e40(&local_e0,0x393);
    FUN_14019ba10(&local_res18,*puVar18);
    if (local_e0 != 0) {
      FUN_14019f2c0(local_e0 + -0x10);
    }
    piVar12 = local_res18;
    local_320 = (int *)0x0;
    piVar13 = local_320;
    if ((local_res18 != (int *)0x0) && (piVar14 = local_res18 + -4, piVar14 != (int *)0x0)) {
      if (*piVar14 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar21 = uVar23;
        do {
          uVar21 = uVar21 + 1;
        } while (*(char *)((longlong)piVar12 + uVar21) != '\0');
        iVar20 = (int)uVar21;
        iVar8 = 0;
        if (0 < iVar20) {
          iVar8 = iVar20;
        }
        piVar12 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar8 + 0x11));
        piVar12[1] = iVar8;
        *piVar12 = -1;
        piVar13 = piVar12 + 4;
        piVar12[2] = 0;
        *(undefined1 *)piVar13 = 0;
        local_50 = piVar13;
        FUN_142ef7ba0(piVar13,local_res18,(longlong)iVar20);
        if (*piVar12 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar20 == -1) || (iVar20 <= piVar12[1])) {
          *piVar12 = 1;
          if (iVar20 != -1) goto LAB_1426b28e1;
          uVar22 = uVar23;
          uVar21 = uVar19;
          if (piVar13 != (int *)0x0) {
            do {
              uVar21 = uVar22 + 1;
              uVar22 = uVar21;
            } while (*(char *)((longlong)piVar13 + uVar21) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar12[1],uVar21 & 0xffffffff);
          *piVar12 = 1;
LAB_1426b28e1:
          *(undefined1 *)((longlong)iVar20 + (longlong)piVar13) = 0;
        }
        iVar8 = (int)uVar21;
        if ((iVar8 < 0) || (piVar12[1] + 1 <= iVar8)) {
          FUN_142e54290(0x9c,uVar21 & 0xffffffff);
        }
        piVar12[2] = iVar8;
        if (local_320 != (int *)0x0) {
          FUN_14019f2c0(local_320 + -4);
        }
      }
      else {
        if (*piVar14 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar14 = *piVar14 + 1;
        UNLOCK();
        if (local_320 != (int *)0x0) {
          FUN_14019f2c0(local_320 + -4);
        }
        local_320 = piVar12;
        piVar13 = local_320;
      }
    }
    local_320 = piVar13;
    FUN_14269a1d0(param_1,0x28,&local_320,0x3e9,0,0);
  }
  cVar2 = FUN_1401b0050((longlong)param_2 + 0x186,*(undefined4 *)((longlong)param_2 + 0x18a));
  lVar15 = local_300;
  if (cVar2 != '\0') {
    uVar3 = FUN_1401b0050((longlong)param_2 + 0x186,*(undefined4 *)((longlong)param_2 + 0x18a));
    puVar18 = (undefined8 *)FUN_1408a9e40(&local_90,0xd65);
    FUN_14019ba10(&local_res18,*puVar18,uVar3);
    if (local_90 != 0) {
      FUN_14019f2c0(local_90 + -0x10);
    }
    bVar4 = FUN_1401b0050((longlong)param_2 + 0x186,*(undefined4 *)((longlong)param_2 + 0x18a));
    lVar15 = local_300;
    piVar13 = local_res18;
    if ((int)((uint)bVar4 - (uint)*(byte *)(local_300 + 0x119)) < 1) {
      local_2e8[6] = 0;
      FUN_14019a260(local_2e8 + 6,&local_res18);
      FUN_14269a1d0(param_1,0x28,local_2e8 + 6,0x3e9,0,0);
    }
    else {
      local_318 = (int *)0x0;
      piVar12 = local_318;
      if ((local_res18 != (int *)0x0) && (piVar14 = local_res18 + -4, piVar14 != (int *)0x0)) {
        if (*piVar14 == -1) {
          FUN_142e52d50(0xcb,0xffffff01);
          uVar21 = uVar23;
          do {
            uVar21 = uVar21 + 1;
          } while (*(char *)((longlong)piVar13 + uVar21) != '\0');
          iVar20 = (int)uVar21;
          iVar8 = iVar9;
          if (0 < iVar20) {
            iVar8 = iVar20;
          }
          piVar13 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar8 + 0x11));
          piVar13[1] = iVar8;
          *piVar13 = -1;
          piVar12 = piVar13 + 4;
          piVar13[2] = 0;
          *(undefined1 *)piVar12 = 0;
          local_48 = piVar12;
          FUN_142ef7ba0(piVar12,local_res18,(longlong)iVar20);
          if (*piVar13 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar20 == -1) || (iVar20 <= piVar13[1])) {
            *piVar13 = 1;
            if (iVar20 != -1) goto LAB_1426b2af6;
            uVar22 = uVar23;
            uVar21 = uVar19;
            if (piVar12 != (int *)0x0) {
              do {
                uVar21 = uVar22 + 1;
                uVar22 = uVar21;
              } while (*(char *)((longlong)piVar12 + uVar21) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar13[1],uVar21 & 0xffffffff);
            *piVar13 = 1;
LAB_1426b2af6:
            *(undefined1 *)((longlong)iVar20 + (longlong)piVar12) = 0;
          }
          iVar8 = (int)uVar21;
          if ((iVar8 < 0) || (piVar13[1] + 1 <= iVar8)) {
            FUN_142e54290(0x9c,uVar21 & 0xffffffff);
          }
          piVar13[2] = iVar8;
          lVar15 = local_300;
          if (local_318 != (int *)0x0) {
            FUN_14019f2c0(local_318 + -4);
            lVar15 = local_300;
          }
        }
        else {
          if (*piVar14 < 1) {
            FUN_142e52dd0(0xd2);
          }
          LOCK();
          *piVar14 = *piVar14 + 1;
          UNLOCK();
          if (local_318 != (int *)0x0) {
            FUN_14019f2c0(local_318 + -4);
          }
          local_318 = piVar13;
          piVar12 = local_318;
        }
      }
      local_318 = piVar12;
      FUN_14269a1d0(param_1,0x2d,&local_318,0x3e9,0,0);
      local_118 = 0;
      uVar11 = FUN_14019ba10(&local_118," (%d%%",*(undefined1 *)(lVar15 + 0x119));
      local_1b8 = (int *)0x0;
      FUN_14019a260(&local_1b8,uVar11);
      if (local_118 != 0) {
        FUN_14019f2c0(local_118 + -0x10);
      }
      piVar12 = local_1b8;
      local_310 = (int *)0x0;
      piVar13 = local_310;
      if ((local_1b8 != (int *)0x0) && (piVar14 = local_1b8 + -4, piVar14 != (int *)0x0)) {
        if (*piVar14 == -1) {
          FUN_142e52d50(0xcb,0xffffff01);
          uVar21 = uVar23;
          do {
            uVar21 = uVar21 + 1;
          } while (*(char *)((longlong)piVar12 + uVar21) != '\0');
          iVar8 = (int)uVar21;
          if (0 < iVar8) {
            iVar9 = iVar8;
          }
          piVar14 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
          piVar14[1] = iVar9;
          *piVar14 = -1;
          piVar13 = piVar14 + 4;
          piVar14[2] = 0;
          *(undefined1 *)piVar13 = 0;
          local_308 = (longlong)iVar8;
          local_40 = piVar13;
          FUN_142ef7ba0(piVar13,piVar12,local_308);
          if (*piVar14 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar8 == -1) || (iVar8 <= piVar14[1])) {
            *piVar14 = 1;
            if (iVar8 != -1) goto LAB_1426b2cd6;
            if (piVar13 != (int *)0x0) {
              do {
                uVar23 = uVar23 + 1;
                uVar19 = uVar23;
              } while (*(char *)((longlong)piVar13 + uVar23) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar14[1],uVar21 & 0xffffffff);
            *piVar14 = 1;
LAB_1426b2cd6:
            *(undefined1 *)(local_308 + (longlong)piVar13) = 0;
            uVar19 = uVar21;
          }
          iVar9 = (int)uVar19;
          if ((iVar9 < 0) || (piVar14[1] + 1 <= iVar9)) {
            FUN_142e54290(0x9c,uVar19 & 0xffffffff);
          }
          piVar14[2] = iVar9;
          lVar15 = local_300;
          if (local_310 != (int *)0x0) {
            FUN_14019f2c0(local_310 + -4);
            lVar15 = local_300;
          }
        }
        else {
          if (*piVar14 < 1) {
            FUN_142e52dd0(0xd2);
          }
          LOCK();
          *piVar14 = *piVar14 + 1;
          UNLOCK();
          if (local_310 != (int *)0x0) {
            FUN_14019f2c0(local_310 + -4);
          }
          local_310 = piVar12;
          piVar12 = local_1b8;
          piVar13 = local_310;
        }
      }
      local_310 = piVar13;
      FUN_14269ac40(param_1,0x28,&local_310,0xffffffff);
      if (0 < (int)local_134) {
        local_1a8[0] = 0;
        FUN_14019ba10(local_1a8," +%d%%");
        local_1b0 = 0;
        FUN_14019a260(&local_1b0,local_1a8);
        FUN_14269ac40(param_1,0x2c,&local_1b0,0xffffffff);
        if (local_1a8[0] != 0) {
          FUN_14019f2c0(local_1a8[0] + -0x10);
        }
      }
      bVar4 = FUN_1401b0050((longlong)param_2 + 0x186,*(undefined4 *)((longlong)param_2 + 0x18a));
      if (0 < (int)(((uint)bVar4 - (uint)*(byte *)(lVar15 + 0x119)) - (int)local_134)) {
        local_1a8[2] = 0;
        bVar4 = FUN_1401b0050((longlong)param_2 + 0x186,*(undefined4 *)((longlong)param_2 + 0x18a));
        FUN_14019ba10(local_1a8 + 2," +%d%%",
                      ((uint)bVar4 - (uint)*(byte *)(lVar15 + 0x119)) - (int)local_134);
        local_1a8[1] = 0;
        FUN_14019a260(local_1a8 + 1,local_1a8 + 2);
        FUN_14269ac40(param_1,0x2d,local_1a8 + 1,0xffffffff);
        if (local_1a8[2] != 0) {
          FUN_14019f2c0(local_1a8[2] + -0x10);
        }
      }
      local_308 = 0;
      FUN_14019ba10(&local_308,&DAT_1432841a8);
      local_2e8[3] = 0;
      FUN_14019a260(local_2e8 + 3,&local_308);
      FUN_14269ac40(param_1,0x28,local_2e8 + 3,0xffffffff);
      if (local_308 != 0) {
        FUN_14019f2c0(local_308 + -0x10);
      }
      if (piVar12 != (int *)0x0) {
        FUN_14019f2c0(piVar12 + -4);
      }
    }
  }
  cVar2 = FUN_1401b0050((longlong)param_2 + 0x18e,*(undefined4 *)((longlong)param_2 + 0x192));
  if (cVar2 != '\0') {
    uVar3 = FUN_1401b0050((longlong)param_2 + 0x18e,*(undefined4 *)((longlong)param_2 + 0x192));
    puVar18 = (undefined8 *)FUN_1408a9e40(&local_d8,0x672);
    FUN_14019ba10(&local_res18,*puVar18,uVar3);
    if (local_d8 != 0) {
      FUN_14019f2c0(local_d8 + -0x10);
    }
    bVar4 = FUN_1401b0050((longlong)param_2 + 0x18e,*(undefined4 *)((longlong)param_2 + 0x192));
    if ((int)((uint)bVar4 - (uint)*(byte *)(lVar15 + 0x11a)) < 1) {
      local_2e8[5] = 0;
      FUN_14019a260(local_2e8 + 5,&local_res18);
      FUN_14269a1d0(param_1,0x28,local_2e8 + 5,0x3e9,0,0);
    }
    else {
      local_2e8[0x11] = 0;
      FUN_14019a260(local_2e8 + 0x11,&local_res18);
      FUN_14269a1d0(param_1,0x2d,local_2e8 + 0x11,0x3e9,0,0);
      local_1a8[3] = 0;
      uVar11 = FUN_14019ba10(local_1a8 + 3," (%d%%",*(undefined1 *)(lVar15 + 0x11a));
      local_2e8[0x17] = 0;
      FUN_14019a260(local_2e8 + 0x17,uVar11);
      if (local_1a8[3] != 0) {
        FUN_14019f2c0(local_1a8[3] + -0x10);
      }
      local_2e8[0x12] = 0;
      FUN_14019a260(local_2e8 + 0x12,local_2e8 + 0x17);
      FUN_14269ac40(param_1,0x28,local_2e8 + 0x12,0xffffffff);
      if (0 < local_134._4_4_) {
        local_2e8[0x14] = 0;
        FUN_14019ba10(local_2e8 + 0x14," +%d%%");
        local_2e8[0x13] = 0;
        FUN_14019a260(local_2e8 + 0x13,local_2e8 + 0x14);
        FUN_14269ac40(param_1,0x2c,local_2e8 + 0x13,0xffffffff);
        if (local_2e8[0x14] != 0) {
          FUN_14019f2c0(local_2e8[0x14] + -0x10);
        }
      }
      bVar4 = FUN_1401b0050((longlong)param_2 + 0x18e,*(undefined4 *)((longlong)param_2 + 0x192));
      if (0 < (int)(((uint)bVar4 - (uint)*(byte *)(lVar15 + 0x11a)) - local_134._4_4_)) {
        local_2e8[0x16] = 0;
        bVar4 = FUN_1401b0050((longlong)param_2 + 0x18e,*(undefined4 *)((longlong)param_2 + 0x192));
        FUN_14019ba10(local_2e8 + 0x16," +%d%%",
                      ((uint)bVar4 - (uint)*(byte *)(lVar15 + 0x11a)) - local_134._4_4_);
        local_2e8[0x15] = 0;
        FUN_14019a260(local_2e8 + 0x15,local_2e8 + 0x16);
        FUN_14269ac40(param_1,0x2d,local_2e8 + 0x15,0xffffffff);
        if (local_2e8[0x16] != 0) {
          FUN_14019f2c0(local_2e8[0x16] + -0x10);
        }
      }
      local_2e8[0] = 0;
      FUN_14019ba10(local_2e8,&DAT_1432841a8);
      local_2e8[2] = 0;
      FUN_14019a260(local_2e8 + 2,local_2e8);
      FUN_14269ac40(param_1,0x28,local_2e8 + 2,0xffffffff);
      if (local_2e8[0] != 0) {
        FUN_14019f2c0(local_2e8[0] + -0x10);
      }
      if (local_2e8[0x17] != 0) {
        FUN_14019f2c0(local_2e8[0x17] + -0x10);
      }
    }
  }
  cVar2 = FUN_1401b0050((longlong)param_2 + 0x196,*(undefined4 *)((longlong)param_2 + 0x19a));
  if (cVar2 != '\0') {
    uVar3 = FUN_1401b0050((longlong)param_2 + 0x196,*(undefined4 *)((longlong)param_2 + 0x19a));
    puVar18 = (undefined8 *)FUN_1408a9e40(&local_d0,0x389);
    FUN_14019ba10(&local_res18,*puVar18,uVar3);
    if (local_d0 != 0) {
      FUN_14019f2c0(local_d0 + -0x10);
    }
    bVar4 = FUN_1401b0050((longlong)param_2 + 0x196,*(undefined4 *)((longlong)param_2 + 0x19a));
    if ((int)((uint)bVar4 - (uint)*(byte *)(lVar15 + 0x11b)) < 1) {
      local_2e8[8] = 0;
      FUN_14019a260(local_2e8 + 8,&local_res18);
      FUN_14269a1d0(param_1,0x28,local_2e8 + 8,0x3e9,0,0);
    }
    else {
      local_2e8[0x18] = 0;
      FUN_14019a260(local_2e8 + 0x18,&local_res18);
      FUN_14269a1d0(param_1,0x2d,local_2e8 + 0x18,0x3e9,0,0);
      local_1a8[4] = 0;
      uVar11 = FUN_14019ba10(local_1a8 + 4," (%d%%",*(undefined1 *)(lVar15 + 0x11b));
      local_2e8[0x1e] = 0;
      FUN_14019a260(local_2e8 + 0x1e,uVar11);
      if (local_1a8[4] != 0) {
        FUN_14019f2c0(local_1a8[4] + -0x10);
      }
      local_2e8[0x19] = 0;
      FUN_14019a260(local_2e8 + 0x19,local_2e8 + 0x1e);
      FUN_14269ac40(param_1,0x28,local_2e8 + 0x19,0xffffffff);
      if (0 < (int)local_12c) {
        local_2e8[0x1b] = 0;
        FUN_14019ba10(local_2e8 + 0x1b," +%d%%");
        local_2e8[0x1a] = 0;
        FUN_14019a260(local_2e8 + 0x1a,local_2e8 + 0x1b);
        FUN_14269ac40(param_1,0x2c,local_2e8 + 0x1a,0xffffffff);
        if (local_2e8[0x1b] != 0) {
          FUN_14019f2c0(local_2e8[0x1b] + -0x10);
        }
      }
      bVar4 = FUN_1401b0050((longlong)param_2 + 0x196,*(undefined4 *)((longlong)param_2 + 0x19a));
      if (0 < (int)(((uint)bVar4 - (uint)*(byte *)(lVar15 + 0x11b)) - (int)local_12c)) {
        local_2e8[0x1d] = 0;
        bVar4 = FUN_1401b0050((longlong)param_2 + 0x196,*(undefined4 *)((longlong)param_2 + 0x19a));
        FUN_14019ba10(local_2e8 + 0x1d," +%d%%",
                      ((uint)bVar4 - (uint)*(byte *)(lVar15 + 0x11b)) - (int)local_12c);
        local_2e8[0x1c] = 0;
        FUN_14019a260(local_2e8 + 0x1c,local_2e8 + 0x1d);
        FUN_14269ac40(param_1,0x2d,local_2e8 + 0x1c,0xffffffff);
        if (local_2e8[0x1d] != 0) {
          FUN_14019f2c0(local_2e8[0x1d] + -0x10);
        }
      }
      local_2f8 = 0;
      FUN_14019ba10(&local_2f8,&DAT_1432841a8);
      local_2e8[4] = 0;
      FUN_14019a260(local_2e8 + 4,&local_2f8);
      FUN_14269ac40(param_1,0x28,local_2e8 + 4,0xffffffff);
      if (local_2f8 != 0) {
        FUN_14019f2c0(local_2f8 + -0x10);
      }
      if (local_2e8[0x1e] != 0) {
        FUN_14019f2c0(local_2e8[0x1e] + -0x10);
      }
    }
  }
  cVar2 = FUN_1401b0050((longlong)param_2 + 0x19e,*(undefined4 *)((longlong)param_2 + 0x1a2));
  if (cVar2 != '\0') {
    uVar3 = FUN_1401b0050((longlong)param_2 + 0x19e,*(undefined4 *)((longlong)param_2 + 0x1a2));
    puVar18 = (undefined8 *)FUN_1408a9e40(&local_c8,0x671);
    FUN_14019ba10(&local_res18,*puVar18,uVar3);
    if (local_c8 != 0) {
      FUN_14019f2c0(local_c8 + -0x10);
    }
    bVar4 = FUN_1401b0050((longlong)param_2 + 0x19e,*(undefined4 *)((longlong)param_2 + 0x1a2));
    if ((int)((uint)bVar4 - (uint)*(byte *)(lVar15 + 0x11c)) < 1) {
      local_2e8[7] = 0;
      FUN_14019a260(local_2e8 + 7,&local_res18);
      FUN_14269a1d0(param_1,0x28,local_2e8 + 7,0x3e9,0,0);
    }
    else {
      local_2e8[0x1f] = 0;
      FUN_14019a260(local_2e8 + 0x1f,&local_res18);
      FUN_14269a1d0(param_1,0x2d,local_2e8 + 0x1f,0x3e9,0,0);
      local_1a8[5] = 0;
      uVar11 = FUN_14019ba10(local_1a8 + 5," (%d%%",*(undefined1 *)(lVar15 + 0x11c));
      local_2e8[0x24] = 0;
      FUN_14019a260(local_2e8 + 0x24,uVar11);
      if (local_1a8[5] != 0) {
        FUN_14019f2c0(local_1a8[5] + -0x10);
      }
      local_2e8[0x20] = 0;
      FUN_14019a260(local_2e8 + 0x20,local_2e8 + 0x24);
      FUN_14269ac40(param_1,0x28,local_2e8 + 0x20,0xffffffff);
      if (0 < local_12c._4_4_) {
        local_2e8[0x21] = 0;
        FUN_14019ba10(local_2e8 + 0x21," +%d%%");
        local_2e8[0x25] = 0;
        FUN_14019a260(local_2e8 + 0x25,local_2e8 + 0x21);
        FUN_14269ac40(param_1,0x2c,local_2e8 + 0x25,0xffffffff);
        if (local_2e8[0x21] != 0) {
          FUN_14019f2c0(local_2e8[0x21] + -0x10);
        }
      }
      bVar4 = FUN_1401b0050((longlong)param_2 + 0x19e,*(undefined4 *)((longlong)param_2 + 0x1a2));
      if (0 < (int)(((uint)bVar4 - (uint)*(byte *)(lVar15 + 0x11c)) - local_12c._4_4_)) {
        local_2e8[0x23] = 0;
        bVar4 = FUN_1401b0050((longlong)param_2 + 0x19e,*(undefined4 *)((longlong)param_2 + 0x1a2));
        FUN_14019ba10(local_2e8 + 0x23," +%d%%",
                      ((uint)bVar4 - (uint)*(byte *)(lVar15 + 0x11c)) - local_12c._4_4_);
        local_2e8[0x22] = 0;
        FUN_14019a260(local_2e8 + 0x22,local_2e8 + 0x23);
        FUN_14269ac40(param_1,0x2d,local_2e8 + 0x22,0xffffffff);
        if (local_2e8[0x23] != 0) {
          FUN_14019f2c0(local_2e8[0x23] + -0x10);
        }
      }
      local_2f0 = 0;
      FUN_14019ba10(&local_2f0,&DAT_1432841a8);
      local_2e8[10] = 0;
      FUN_14019a260(local_2e8 + 10,&local_2f0);
      FUN_14269ac40(param_1,0x28,local_2e8 + 10,0xffffffff);
      if (local_2f0 != 0) {
        FUN_14019f2c0(local_2f0 + -0x10);
      }
      if (local_2e8[0x24] != 0) {
        FUN_14019f2c0(local_2e8[0x24] + -0x10);
      }
    }
  }
  cVar2 = FUN_1401b0050((longlong)param_2 + 0x152,*(undefined4 *)((longlong)param_2 + 0x156));
  if ((cVar2 != '\0') || (local_16c._4_4_ != 0)) {
    cVar2 = FUN_1401b0050((longlong)param_2 + 0x152,*(undefined4 *)((longlong)param_2 + 0x156));
    if (cVar2 == '\0') {
      uVar17 = local_16c._4_4_;
    }
    else {
      bVar4 = FUN_1401b0050((longlong)param_2 + 0x152,*(undefined4 *)((longlong)param_2 + 0x156));
      uVar17 = (uint)bVar4;
    }
    puVar18 = (undefined8 *)FUN_1408a9e40(&local_c0,0x394);
    FUN_14019ba10(&local_res18,*puVar18,uVar17);
    if (local_c0 != 0) {
      FUN_14019f2c0(local_c0 + -0x10);
    }
    local_2e8[9] = 0;
    FUN_14019a260(local_2e8 + 9,&local_res18);
    FUN_14269a1d0(param_1,0x2c,local_2e8 + 9,0x3e9,0,0);
  }
  iVar9 = *(int *)(lVar15 + 0x184);
  if (0 < iVar9) {
    puVar18 = (undefined8 *)FUN_1408a9e40(&local_b8,0x38f);
    FUN_14019ba10(&local_res18,*puVar18,iVar9);
    if (local_b8 != 0) {
      FUN_14019f2c0(local_b8 + -0x10);
    }
    local_2e8[1] = 0;
    FUN_14019a260(local_2e8 + 1,&local_res18);
    FUN_14269a1d0(param_1,0x28,local_2e8 + 1,0x3e9,0,0);
  }
  if (*(int *)(lVar15 + 0x328) == 0) {
    cVar2 = FUN_1408419d0();
    if (cVar2 == '\0') {
      if (*(char *)(lVar15 + 0xb8) != '\0') {
        uVar3 = FUN_1401b0050((longlong)param_2 + 0xfa,*(undefined4 *)((longlong)param_2 + 0xfe));
        puVar18 = (undefined8 *)FUN_1408a9e40(&local_a0,0x39e);
        FUN_14019ba10(&local_res18,*puVar18,uVar3);
        if (local_a0 != 0) {
          FUN_14019f2c0(local_a0 + -0x10);
        }
        local_2e8[0xd] = 0;
        FUN_14019a260(local_2e8 + 0xd,&local_res18);
        FUN_14269a1d0(param_1,0x28,local_2e8 + 0xd,0x3e9,0,0);
      }
      FUN_1426b1330(param_1,param_2);
    }
    iVar9 = (**(code **)(*param_2 + 0x3a8))(param_2);
    if ((iVar9 != 0) && (iVar9 = FUN_1401b0340(param_2 + 4), iVar9 - 1800000U < 100000)) {
      puVar18 = (undefined8 *)FUN_1408a9e40(&local_98,0x39f);
      FUN_14019ba10(&local_res18,*puVar18);
      if (local_98 != 0) {
        FUN_14019f2c0(local_98 + -0x10);
      }
      local_2e8[0xe] = 0;
      FUN_14019a260(local_2e8 + 0xe,&local_res18);
      FUN_14269a1d0(param_1,0x28,local_2e8 + 0xe,0x3e9,0,0);
    }
    iVar9 = FUN_1401b0340(param_2 + 4);
    lVar15 = FUN_140388c60(DAT_143aa8328,iVar9);
    cVar2 = *(char *)(lVar15 + 0xb8);
    iVar8 = (**(code **)(*param_2 + 0x3a8))(param_2);
    if ((iVar8 != 0) && (iVar9 - 1800000U < 100000)) {
      cVar2 = FUN_1401b0050((longlong)param_2 + 0xfa,*(undefined4 *)((longlong)param_2 + 0xfe));
    }
    if ((((cVar2 != '\0') && (99999 < iVar9 - 1800000U)) && (*(char *)(lVar15 + 0x1a4) == '\0')) &&
       (iVar9 = FUN_1401ba9d0((longlong)param_2 + 0x146,*(undefined4 *)((longlong)param_2 + 0x14e)),
       0 < iVar9)) {
      puVar18 = (undefined8 *)FUN_1408a9e40(&local_78,0xda7);
      FUN_14019ba10(&local_res18,*puVar18);
      if (local_78 != 0) {
        FUN_14019f2c0(local_78 + -0x10);
      }
      local_2e8[0xf] = 0;
      FUN_14019a260(local_2e8 + 0xf,&local_res18);
      FUN_14269a1d0(param_1,0x28,local_2e8 + 0xf,0x3e9,0,0);
    }
    cVar2 = FUN_140841850();
    if ((cVar2 == '\0') && (cVar2 = FUN_1403e8c40(param_2), cVar2 != '\0')) {
      uVar3 = FUN_1401b0050((longlong)param_2 + 0x1a6,*(undefined4 *)((longlong)param_2 + 0x1aa));
      puVar18 = (undefined8 *)FUN_1408a9e40(&local_88,0x3a0);
      FUN_14019ba10(&local_res18,*puVar18,uVar3);
      if (local_88 != 0) {
        FUN_14019f2c0(local_88 + -0x10);
      }
      local_2e8[0x10] = 0;
      FUN_14019a260(local_2e8 + 0x10,&local_res18);
      FUN_14269a1d0(param_1,0x29,local_2e8 + 0x10,0x3e9,0,0);
    }
    uVar11 = DAT_143aa8328;
    uVar7 = FUN_1401b0340(param_2 + 4);
    iVar9 = FUN_14038aa60(uVar11,uVar7);
    if (iVar9 != 0) {
      plVar16 = (longlong *)FUN_1408a9e40(local_80,0x3e2);
      uVar7 = 1;
      uVar11 = 0x2c;
      goto LAB_1426b3d64;
    }
  }
  else {
    puVar18 = (undefined8 *)FUN_1408a9e40(&local_b0,0xc72);
    FUN_14019ba10(&local_res18,*puVar18);
    if (local_b0 != 0) {
      FUN_14019f2c0(local_b0 + -0x10);
    }
    local_2e8[0xb] = 0;
    FUN_14019a260(local_2e8 + 0xb,&local_res18);
    FUN_14269a1d0(param_1,0x28,local_2e8 + 0xb,0x3e9,0,0);
    FUN_1426b1330(param_1,param_2);
    iVar9 = (**(code **)(*param_2 + 0x3a8))(param_2);
    if ((iVar9 != 0) && (iVar9 = FUN_1401b0340(param_2 + 4), iVar9 - 1800000U < 100000)) {
      puVar18 = (undefined8 *)FUN_1408a9e40(&local_a8,0x39f);
      FUN_14019ba10(&local_res18,*puVar18);
      if (local_a8 != 0) {
        FUN_14019f2c0(local_a8 + -0x10);
      }
      local_2e8[0xc] = 0;
      FUN_14019a260(local_2e8 + 0xc,&local_res18);
      uVar7 = 0;
      plVar16 = local_2e8 + 0xc;
      uVar11 = 0x28;
LAB_1426b3d64:
      FUN_14269a1d0(param_1,uVar11,plVar16,0x3e9,uVar7,0);
    }
  }
  *(int *)(param_1 + 0x3c) = *(int *)(param_1 + 0x3c) + 8;
  if (local_res18 != (int *)0x0) {
    FUN_14019f2c0(local_res18 + -4);
  }
LAB_1426b3d8e:
  lVar15 = local_108;
  if (local_108 != 0) {
    puVar18 = (undefined8 *)(local_108 + -0x28);
    if (0xffffe < *(longlong *)(local_108 + -0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar16 = (longlong *)(lVar15 + -0x20);
    lVar15 = *plVar16;
    *plVar16 = *plVar16 + -1;
    UNLOCK();
    if ((int)lVar15 == 1) {
      if (*(longlong *)(local_108 + -0x10) != 0) {
        LOCK();
        *(undefined8 *)(*(longlong *)(local_108 + -0x10) + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(local_108 + -0x10) + 4) != 0);
      }
      if (puVar18 != (undefined8 *)0x0) {
        (**(code **)*puVar18)(puVar18,1);
      }
    }
  }
  return;
}



//===========================================================
// FUN_1403e8e00 @ 1403e8e00   (490 bytes)
//===========================================================

undefined8 * FUN_1403e8e00(undefined8 *param_1,int param_2,char param_3)

{
  char cVar1;
  int iVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  bool bVar5;
  longlong local_res20;
  longlong local_10;
  
  if ((param_3 == '\0') &&
     ((cVar1 = FUN_140841850(), cVar1 == '\0' || (cVar1 = FUN_1408418c0(param_2), cVar1 == '\0'))))
  {
    if ((param_2 - 1000000U < 1000000) || (param_2 - 6000000U < 1000000)) {
      iVar2 = FUN_14038abd0(DAT_143aa8328,param_2);
      if (iVar2 != 0) {
        cVar1 = FUN_140841850();
        bVar5 = cVar1 == '\0';
        if (bVar5) {
          puVar3 = (undefined8 *)FUN_1408a9e40(&local_res20,0x3c8);
        }
        else {
          puVar3 = (undefined8 *)FUN_1408a9e40(&local_10,0x3ca);
        }
        *param_1 = 0;
        *param_1 = *puVar3;
        *puVar3 = 0;
        if ((bVar5) && (local_res20 != 0)) {
          FUN_14019f2c0(local_res20 + -0x10);
        }
        if (bVar5) {
          return param_1;
        }
        if (local_10 != 0) {
          FUN_14019f2c0(local_10 + -0x10);
        }
        return param_1;
      }
      uVar4 = 0x3c6;
    }
    else {
      iVar2 = FUN_14038abd0(DAT_143aa8328,param_2);
      if (iVar2 != 0) {
        cVar1 = FUN_140841850();
        bVar5 = cVar1 == '\0';
        if (bVar5) {
          puVar3 = (undefined8 *)FUN_1408a9e40(&local_res20,0x3c9);
        }
        else {
          puVar3 = (undefined8 *)FUN_1408a9e40(&local_10,0x3cb);
        }
        *param_1 = 0;
        *param_1 = *puVar3;
        *puVar3 = 0;
        if ((bVar5) && (local_res20 != 0)) {
          FUN_14019f2c0(local_res20 + -0x10);
        }
        if (bVar5) {
          return param_1;
        }
        if (local_10 != 0) {
          FUN_14019f2c0(local_10 + -0x10);
        }
        return param_1;
      }
      uVar4 = 0x3c7;
    }
  }
  else {
    uVar4 = 0x3bb;
  }
  FUN_1408a9e40(param_1,uVar4);
  return param_1;
}



//===========================================================
// FUN_14275dfa0 @ 14275dfa0   (354 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14275dfa0(void)

{
  int iVar1;
  longlong lVar2;
  char cVar3;
  int iVar4;
  int *piVar5;
  undefined1 auStack_498 [32];
  int *local_478 [2];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  lVar2 = DAT_143ad00b0;
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  local_478[0] = (int *)0x0;
  piVar5 = (int *)FUN_14019b600(&DAT_143ad6a30,0x17);
  piVar5[1] = 6;
  *piVar5 = -1;
  local_478[0] = piVar5 + 4;
  piVar5[2] = 0;
  *(undefined1 *)local_478[0] = 0;
  *local_478[0] = s_myTurn_143480ebc._0_4_;
  *(undefined2 *)(piVar5 + 5) = s_myTurn_143480ebc._4_2_;
  if (*piVar5 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar5[1] < 6) {
    FUN_142e54290(0x90,piVar5[1],6);
  }
  *piVar5 = 1;
  *(undefined1 *)((longlong)local_478[0] + 6) = 0;
  if (piVar5[1] + 1 < 7) {
    FUN_142e54290(0x9c,6);
  }
  piVar5[2] = 6;
  FUN_14275f2c0(lVar2,local_478);
  lVar2 = DAT_143ad00b0;
  cVar3 = FUN_140a04280(*(undefined4 *)(DAT_143ad00b0 + 0xbd8));
  if (cVar3 != '\0') {
    iVar1 = *(int *)(lVar2 + 0xbe0);
    iVar4 = FUN_142cb9550(DAT_143aa84a0);
    if (*(int *)((longlong)iVar1 * 0x330 + 0x2338 + lVar2) == iVar4) {
      FUN_1406ed520(local_468,0x3c6);
      FUN_1415d01c0(local_468);
      FUN_1406ed610(local_468);
    }
  }
  return;
}



//===========================================================
// FUN_14275e9d0 @ 14275e9d0   (573 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14275e9d0(longlong param_1,longlong *param_2)

{
  longlong *plVar1;
  int *piVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  char cVar6;
  int iVar7;
  int iVar8;
  longlong *plVar9;
  longlong lVar10;
  longlong lVar11;
  undefined4 *puVar12;
  undefined1 auStack_4d8 [40];
  longlong *local_4b0;
  longlong *local_4a8;
  longlong *local_4a0;
  undefined1 local_498 [1104];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_4d8;
  local_4b0 = (longlong *)*param_2;
  iVar8 = *(int *)((longlong)local_4b0 + 0xc);
  local_4a0 = param_2;
  if (iVar8 == 1) {
    lVar11 = (longlong)*(int *)(param_1 + 0xbe0) * 0x330;
    puVar12 = (undefined4 *)(lVar11 + 0x2638 + param_1);
    uVar3 = puVar12[1];
    uVar4 = puVar12[2];
    uVar5 = puVar12[3];
    *(undefined4 *)(param_1 + 0xa30) = *puVar12;
    *(undefined4 *)(param_1 + 0xa34) = uVar3;
    *(undefined4 *)(param_1 + 0xa38) = uVar4;
    *(undefined4 *)(param_1 + 0xa3c) = uVar5;
    *(undefined8 *)(param_1 + 0xa40) = *(undefined8 *)(lVar11 + 0x2648 + param_1);
    local_4b0 = param_2;
    FUN_142759f20();
  }
  else if (iVar8 == 2) {
    plVar9 = (longlong *)0x0;
    if (param_2[1] != 0) {
      LOCK();
      piVar2 = (int *)(param_2[1] + 8);
      *piVar2 = *piVar2 + 1;
      UNLOCK();
      plVar9 = (longlong *)param_2[1];
    }
    local_4a8 = plVar9;
    if (*(char *)(local_4b0 + 0xb) != '\0') {
      FUN_14275ed60(param_1,local_4b0[3]);
    }
    if (plVar9 != (longlong *)0x0) {
      LOCK();
      plVar1 = plVar9 + 1;
      lVar11 = *plVar1;
      *(int *)plVar1 = (int)*plVar1 + -1;
      UNLOCK();
      if ((int)lVar11 == 1) {
        (**(code **)*plVar9)(plVar9);
        LOCK();
        piVar2 = (int *)((longlong)plVar9 + 0xc);
        iVar8 = *piVar2;
        *piVar2 = *piVar2 + -1;
        UNLOCK();
        if (iVar8 == 1) {
          (**(code **)(*plVar9 + 8))(plVar9);
        }
      }
    }
  }
  else {
    local_4b0 = param_2;
    if (iVar8 == 3) {
      for (lVar11 = param_1 + 9000; local_4b0 = param_2, lVar11 != param_1 + 0x2988;
          lVar11 = lVar11 + 0x330) {
        cVar6 = FUN_140a0a740(lVar11);
        lVar10 = 0x698;
        if (cVar6 != '\0') {
          lVar10 = 0x308;
        }
        iVar8 = 0;
        puVar12 = (undefined4 *)(lVar11 + 0x18);
        do {
          FUN_1427567f0(lVar10 + param_1,param_1 + 0x2e0,iVar8,*puVar12);
          iVar8 = iVar8 + 1;
          puVar12 = puVar12 + 3;
        } while (iVar8 < 2);
        FUN_142761970(param_1,lVar11);
        param_2 = local_4b0;
      }
      cVar6 = FUN_140a04280(*(undefined4 *)(param_1 + 0xbd8));
      if ((cVar6 != '\0') &&
         (iVar8 = *(int *)(param_1 + 0xbe0), iVar7 = FUN_142cb9550(DAT_143aa84a0),
         *(int *)((longlong)iVar8 * 0x330 + 0x2338 + param_1) == iVar7)) {
        FUN_1406ed520(local_498,0x3c6);
        FUN_1415d01c0(local_498);
        FUN_1406ed610(local_498);
      }
    }
  }
  plVar9 = (longlong *)param_2[1];
  if (plVar9 != (longlong *)0x0) {
    LOCK();
    plVar1 = plVar9 + 1;
    lVar11 = *plVar1;
    *(int *)plVar1 = (int)*plVar1 + -1;
    UNLOCK();
    if ((int)lVar11 == 1) {
      (**(code **)*plVar9)(plVar9);
      LOCK();
      piVar2 = (int *)((longlong)plVar9 + 0xc);
      iVar8 = *piVar2;
      *piVar2 = *piVar2 + -1;
      UNLOCK();
      if (iVar8 == 1) {
        (**(code **)(*plVar9 + 8))(plVar9);
      }
    }
  }
  return;
}



//===========================================================
// FUN_142762df0 @ 142762df0   (1 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142762df0(void)

{
  undefined1 auStack_488 [32];
  undefined1 auStack_468 [1104];
  ulonglong uStack_18;
  
  uStack_18 = DAT_143a8b908 ^ (ulonglong)auStack_488;
  FUN_1406ed520(auStack_468,0x3c6);
  FUN_1415d01c0(auStack_468);
  FUN_1406ed610(auStack_468);
  return;
}



//===========================================================
// FUN_1402f7010 @ 1402f7010   (131 bytes)
//===========================================================

uint FUN_1402f7010(undefined2 param_1,longlong param_2)

{
  byte bVar1;
  byte bVar2;
  longlong lVar3;
  byte *pbVar4;
  uint uVar5;
  undefined2 local_res8 [4];
  
  uVar5 = 0xbaadf00d;
  pbVar4 = (byte *)local_res8;
  lVar3 = 2;
  local_res8[0] = param_1;
  do {
    bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
    bVar1 = *pbVar4;
    pbVar4[param_2 - (longlong)local_res8] = bVar2;
    pbVar4[(param_2 + 2) - (longlong)local_res8] = bVar2 ^ bVar1;
    pbVar4 = pbVar4 + 1;
    uVar5 = ((uVar5 ^ bVar2) >> 5 | (uVar5 ^ bVar2) << 0x1b) + (uint)(bVar2 ^ bVar1);
    lVar3 = lVar3 + -1;
  } while (lVar3 != 0);
  return uVar5;
}



//===========================================================
// FUN_1402f70a0 @ 1402f70a0   (131 bytes)
//===========================================================

uint FUN_1402f70a0(undefined2 param_1,longlong param_2)

{
  byte bVar1;
  byte bVar2;
  longlong lVar3;
  byte *pbVar4;
  uint uVar5;
  undefined2 local_res8 [4];
  
  uVar5 = 0xbaadf00d;
  pbVar4 = (byte *)local_res8;
  lVar3 = 2;
  local_res8[0] = param_1;
  do {
    bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
    bVar1 = *pbVar4;
    pbVar4[param_2 - (longlong)local_res8] = bVar2;
    pbVar4[(param_2 + 2) - (longlong)local_res8] = bVar2 ^ bVar1;
    pbVar4 = pbVar4 + 1;
    uVar5 = ((uVar5 ^ bVar2) >> 5 | (uVar5 ^ bVar2) << 0x1b) + (uint)(bVar2 ^ bVar1);
    lVar3 = lVar3 + -1;
  } while (lVar3 != 0);
  return uVar5;
}



//===========================================================
// FUN_1402f7130 @ 1402f7130   (60 bytes)
//===========================================================

int FUN_1402f7130(uint param_1,uint *param_2)

{
  uint uVar1;
  uint uVar2;
  
  uVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *param_2 = uVar1;
  uVar2 = (uVar1 ^ param_1) >> 5 | (uVar1 ^ param_1) << 0x1b;
  param_2[1] = uVar2;
  return ((uVar1 ^ 0xbaadf00d) >> 5 | (uVar1 ^ 0xbaadf00d) << 0x1b) + uVar2;
}



//===========================================================
// FUN_1402f7170 @ 1402f7170   (130 bytes)
//===========================================================

uint FUN_1402f7170(undefined8 param_1,longlong param_2)

{
  uint uVar1;
  uint uVar2;
  longlong lVar3;
  uint *puVar4;
  uint uVar5;
  undefined8 local_res8;
  
  uVar5 = 0xbaadf00d;
  lVar3 = 2;
  puVar4 = (uint *)&local_res8;
  local_res8 = param_1;
  do {
    uVar1 = FUN_1407386b0(&DAT_143ac1ab0);
    *(uint *)((param_2 - (longlong)&local_res8) + (longlong)puVar4) = uVar1;
    uVar2 = (uVar1 ^ *puVar4) >> 5 | (uVar1 ^ *puVar4) << 0x1b;
    uVar5 = ((uVar5 ^ uVar1) >> 5 | (uVar5 ^ uVar1) << 0x1b) + uVar2;
    *(uint *)((param_2 - (longlong)&local_res8) + 4 + (longlong)(puVar4 + 1)) = uVar2;
    lVar3 = lVar3 + -1;
    puVar4 = puVar4 + 1;
  } while (lVar3 != 0);
  return uVar5;
}



//===========================================================
// FUN_1402f72a0 @ 1402f72a0   (612 bytes)
//===========================================================

longlong * FUN_1402f72a0(float *param_1,longlong *param_2,byte *param_3,undefined4 *param_4)

{
  float *pfVar1;
  int iVar2;
  longlong *plVar3;
  longlong *plVar4;
  longlong *plVar5;
  longlong lVar6;
  longlong *plVar7;
  ulonglong uVar8;
  float fVar9;
  
  uVar8 = (((((ulonglong)*param_3 ^ 0xcbf29ce484222325) * 0x100000001b3 ^ (ulonglong)param_3[1]) *
            0x100000001b3 ^ (ulonglong)param_3[2]) * 0x100000001b3 ^ (ulonglong)param_3[3]) *
          0x100000001b3;
  plVar5 = *(longlong **)
            (*(longlong *)(param_1 + 6) + 8 + (uVar8 & *(ulonglong *)(param_1 + 0xc)) * 0x10);
  pfVar1 = param_1 + 2;
  plVar7 = *(longlong **)pfVar1;
  if (plVar5 != plVar7) {
    iVar2 = (int)plVar5[2];
    plVar7 = plVar5;
    while( true ) {
      if (*(int *)param_3 == iVar2) {
        *param_2 = (longlong)plVar7;
        *(undefined1 *)(param_2 + 1) = 0;
        return param_2;
      }
      if (plVar7 == *(longlong **)
                     (*(longlong *)(param_1 + 6) + (uVar8 & *(ulonglong *)(param_1 + 0xc)) * 0x10))
      break;
      plVar7 = (longlong *)plVar7[1];
      iVar2 = (int)plVar7[2];
    }
  }
  if (*(longlong *)(param_1 + 4) == 0xaaaaaaaaaaaaaaa) {
                    /* WARNING: Subroutine does not return */
    FUN_142ed3068("unordered_map/set too long");
  }
  plVar5 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x18,param_3,param_4,pfVar1,0);
  *(undefined4 *)(plVar5 + 2) = *(undefined4 *)param_3;
  *(undefined4 *)((longlong)plVar5 + 0x14) = *param_4;
  lVar6 = *(longlong *)(param_1 + 4);
  fVar9 = (float)(lVar6 + 1) / (float)*(ulonglong *)(param_1 + 0xe);
  if (*param_1 <= fVar9 && fVar9 != *param_1) {
    FUN_140301b00(param_1);
    plVar3 = *(longlong **)
              (*(longlong *)(param_1 + 6) + 8 + (uVar8 & *(ulonglong *)(param_1 + 0xc)) * 0x10);
    plVar7 = *(longlong **)pfVar1;
    if (plVar3 != plVar7) {
      iVar2 = (int)plVar3[2];
      plVar7 = plVar3;
      while ((int)plVar5[2] != iVar2) {
        if (plVar7 == *(longlong **)
                       (*(longlong *)(param_1 + 6) + (uVar8 & *(ulonglong *)(param_1 + 0xc)) * 0x10)
           ) goto LAB_1402f747b;
        plVar7 = (longlong *)plVar7[1];
        iVar2 = (int)plVar7[2];
      }
      plVar7 = (longlong *)*plVar7;
    }
LAB_1402f747b:
    lVar6 = *(longlong *)(param_1 + 4);
  }
  plVar3 = (longlong *)plVar7[1];
  *(longlong *)(param_1 + 4) = lVar6 + 1;
  *plVar5 = (longlong)plVar7;
  plVar5[1] = (longlong)plVar3;
  *plVar3 = (longlong)plVar5;
  plVar7[1] = (longlong)plVar5;
  lVar6 = *(longlong *)(param_1 + 6);
  uVar8 = uVar8 & *(ulonglong *)(param_1 + 0xc);
  plVar4 = *(longlong **)(lVar6 + uVar8 * 0x10);
  if (plVar4 == *(longlong **)pfVar1) {
    *(longlong **)(lVar6 + uVar8 * 0x10) = plVar5;
  }
  else {
    if (plVar4 == plVar7) {
      *(longlong **)(lVar6 + uVar8 * 0x10) = plVar5;
      goto LAB_1402f74eb;
    }
    if (*(longlong **)(lVar6 + 8 + uVar8 * 0x10) != plVar3) goto LAB_1402f74eb;
  }
  *(longlong **)(lVar6 + 8 + uVar8 * 0x10) = plVar5;
LAB_1402f74eb:
  *param_2 = (longlong)plVar5;
  *(undefined1 *)(param_2 + 1) = 1;
  return param_2;
}


