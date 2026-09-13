
//===========================================================
// FUN_142d5b750 @ 142d5b750   (3785 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000142d5bfb6) */

void FUN_142d5b750(longlong *param_1,int param_2,uint param_3,longlong *param_4,undefined8 param_5,
                  int param_6)

{
  longlong *plVar1;
  char cVar2;
  int iVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  uint uVar7;
  int iVar8;
  longlong lVar9;
  undefined8 uVar10;
  longlong lVar11;
  longlong lVar12;
  int *piVar13;
  int *piVar14;
  longlong lVar15;
  uint uVar16;
  undefined8 *puVar17;
  int *piVar18;
  int *piVar19;
  longlong *plVar20;
  int *piVar21;
  ulonglong uVar22;
  bool bVar23;
  longlong local_res8;
  uint local_res18 [2];
  longlong *local_res20;
  ulonglong in_stack_fffffffffffffec8;
  ulonglong uVar24;
  undefined8 local_108;
  int *local_f8;
  int *local_f0;
  uint local_e8 [2];
  ulonglong local_e0;
  longlong local_d8;
  undefined4 local_d0;
  undefined4 local_cc;
  undefined8 local_c8;
  longlong local_c0;
  int *local_b0;
  longlong local_a0;
  int *local_98;
  longlong local_90;
  longlong local_80;
  int *local_70;
  longlong local_60;
  undefined8 local_50;
  
  lVar15 = DAT_143aa9d98;
  local_res18[0] = param_3;
  local_res20 = param_4;
  local_e0 = FUN_142cbe730();
  local_d8 = (**(code **)(*param_1 + 0x30))(param_1);
  local_c8 = FUN_142cbec90(param_1);
  if (DAT_143aa8518 == 0) {
    local_50 = 0;
    local_108 = 0;
  }
  else {
    lVar9 = FUN_1428f74d0();
    lVar11 = local_80;
    local_108 = *(undefined8 *)(lVar9 + 8);
    param_4 = local_res20;
    if (local_80 != 0) {
      puVar17 = (undefined8 *)(local_80 + -0x28);
      if (0xffffe < *(longlong *)(local_80 + -0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar20 = (longlong *)(lVar11 + -0x20);
      lVar11 = *plVar20;
      *plVar20 = *plVar20 + -1;
      UNLOCK();
      if ((int)lVar11 == 1) {
        if ((local_80 != 0) && (*(longlong *)(local_80 + -0x10) != 0)) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_80 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_80 + -0x10) + 4) != 0);
        }
        if (puVar17 != (undefined8 *)0x0) {
          (**(code **)*puVar17)(puVar17);
        }
      }
      local_80 = 0;
      param_4 = local_res20;
      param_3 = local_res18[0];
    }
  }
  piVar21 = (int *)0x0;
  if (param_2 != 0) {
    iVar3 = 0;
    if (param_2 == 1) {
      if (lVar15 == 0) {
        lVar15 = *param_4;
        goto LAB_142d5c600;
      }
      bVar23 = true;
      lVar11 = FUN_14070fa90(lVar15);
      if (lVar11 != 0) {
        uVar4 = FUN_142cb85d0(param_1);
        uVar5 = FUN_142cb85b0(param_1);
        uVar6 = FUN_142cafb20(param_1);
        lVar11 = FUN_14070fa90(lVar15,param_3);
        in_stack_fffffffffffffec8 = local_e0;
        iVar8 = FUN_140711d70(lVar15,param_3,*(undefined4 *)(lVar11 + 0x2c),uVar6,local_e0,local_d8,
                              local_c8,uVar5,uVar4,local_108);
        bVar23 = iVar8 == 0;
      }
      local_f8 = (int *)0x0;
      lVar11 = FUN_142cbe730(param_1);
      if (*(longlong *)(lVar11 + 0x1273) != 0) {
        for (lVar11 = *(longlong *)
                       (*(longlong *)(lVar11 + 0x1273) +
                       ((ulonglong)(longlong)(int)param_3 % (ulonglong)*(uint *)(lVar11 + 0x127b)) *
                       8); lVar11 != 0; lVar11 = *(longlong *)(lVar11 + 8)) {
          if (*(uint *)(lVar11 + 0x10) == param_3) {
            FUN_14019a260(&local_f8,lVar11 + 0x18);
            break;
          }
        }
      }
      uVar10 = FUN_142cbe730(param_1);
      plVar20 = local_res20;
      FUN_1402e0eb0(uVar10,param_3,local_res20);
      piVar13 = (int *)*plVar20;
      if (piVar13 != local_f8) {
        iVar8 = iVar3;
        if (piVar13 != (int *)0x0) {
          iVar8 = piVar13[-2];
        }
        piVar18 = piVar21;
        if (local_f8 != (int *)0x0) {
          piVar18 = (int *)(ulonglong)(uint)local_f8[-2];
        }
        if (iVar8 == (int)piVar18) {
          if (iVar8 != 0) {
            iVar8 = iVar3;
            if (piVar13 != (int *)0x0) {
              iVar8 = piVar13[-2];
            }
            iVar8 = memcmp(piVar13,local_f8,(longlong)iVar8);
            if (iVar8 != 0) goto LAB_142d5be67;
          }
        }
        else {
LAB_142d5be67:
          lVar11 = FUN_1411fe870();
          if ((lVar11 == 0) || (cVar2 = FUN_1411fe780(lVar11,param_3), cVar2 == '\0')) {
            piVar18 = local_f8;
            local_f0 = (int *)0x0;
            piVar13 = local_f0;
            if ((local_f8 != (int *)0x0) && (piVar19 = local_f8 + -4, piVar19 != (int *)0x0)) {
              if (*piVar19 == -1) {
                FUN_142e52d50(0xcb,0xffffff01);
                piVar18 = local_f8;
                local_98 = (int *)0x0;
                piVar13 = piVar21;
                if (local_f8 != (int *)0x0) {
                  piVar19 = (int *)0xffffffffffffffff;
                  do {
                    piVar19 = (int *)((longlong)piVar19 + 1);
                  } while (*(char *)((longlong)local_f8 + (longlong)piVar19) != '\0');
                  iVar8 = (int)piVar19;
                  if (0 < iVar8) {
                    iVar3 = iVar8;
                  }
                  piVar14 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
                  piVar14[1] = iVar3;
                  *piVar14 = -1;
                  piVar13 = piVar14 + 4;
                  piVar14[2] = 0;
                  *(undefined1 *)piVar13 = 0;
                  local_98 = piVar13;
                  FUN_142ef7ba0(piVar13,piVar18,(longlong)iVar8);
                  if (*piVar14 != -1) {
                    FUN_142e52dd0(0x8b);
                  }
                  if ((iVar8 == -1) || (iVar8 <= piVar14[1])) {
                    *piVar14 = 1;
                    if (iVar8 != -1) goto LAB_142d5bf6e;
                    if (piVar13 != (int *)0x0) {
                      piVar21 = (int *)0xffffffffffffffff;
                      do {
                        piVar21 = (int *)((longlong)piVar21 + 1);
                      } while (*(char *)((longlong)piVar13 + (longlong)piVar21) != '\0');
                    }
                  }
                  else {
                    FUN_142e54290(0x90,piVar14[1],(ulonglong)piVar19 & 0xffffffff);
                    *piVar14 = 1;
LAB_142d5bf6e:
                    *(undefined1 *)((longlong)piVar13 + (longlong)iVar8) = 0;
                    piVar21 = piVar19;
                  }
                  iVar3 = (int)piVar21;
                  if ((iVar3 < 0) || (piVar14[1] + 1 <= iVar3)) {
                    FUN_142e54290(0x9c,(ulonglong)piVar21 & 0xffffffff);
                  }
                  piVar14[2] = iVar3;
                  plVar20 = local_res20;
                }
                if (local_f0 != (int *)0x0) {
                  FUN_14019f2c0(local_f0 + -4);
                }
              }
              else {
                if (*piVar19 < 1) {
                  FUN_142e52dd0(0xd2);
                }
                LOCK();
                *piVar19 = *piVar19 + 1;
                UNLOCK();
                if (local_f0 != (int *)0x0) {
                  FUN_14019f2c0(local_f0 + -4);
                }
                local_f0 = piVar18;
                piVar13 = local_f0;
                param_3 = local_res18[0];
              }
            }
            local_f0 = piVar13;
            FUN_142d934f0(param_1,param_3,0,0,&local_f0);
          }
          else {
            FUN_142d93610(param_1,param_3,&local_f8,plVar20,
                          in_stack_fffffffffffffec8 & 0xffffffffffffff00);
          }
          FUN_140905730(param_3,&local_f8,plVar20);
        }
      }
      if (DAT_143ad20e8 != (longlong *)0x0) {
        iVar3 = FUN_142507380(DAT_143ad20e8,param_3);
        if ((iVar3 == 0) && (iVar3 = FUN_1424f0280(DAT_143ad20e8,param_3), iVar3 == 0)) {
          if (!bVar23) {
            FUN_1424efe50(DAT_143ad20e8,param_3,1);
          }
        }
        else {
          FUN_1424efd70(DAT_143ad20e8,param_3);
        }
        (**(code **)(*DAT_143ad20e8 + 0x90))(DAT_143ad20e8,0);
      }
      FUN_141d1fe90(param_1 + 0x5ca,local_res18);
      iVar3 = FUN_140715850(lVar15,param_3);
      if (iVar3 != 0) {
        local_d0 = 1;
        FUN_140726d40(param_1 + 0x5d0,local_res18,&local_d0);
      }
      if (param_3 == 0x31b4) {
        *(undefined4 *)(param_1 + 0x5d3) = 0;
        if (param_1[0x53a] != 0) {
          uVar10 = FUN_142954390(param_1 + 0x539);
LAB_142d5c145:
          FUN_142518060(uVar10,param_3,10000);
        }
      }
      else if ((param_3 == 0x29a8) &&
              (*(undefined4 *)((longlong)param_1 + 0x2e9c) = 0, param_1[0x53a] != 0)) {
        uVar10 = FUN_142954390(param_1 + 0x539);
        goto LAB_142d5c145;
      }
      lVar11 = param_1[0x53c];
      if (lVar11 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar11 = param_1[0x53c];
      }
      uVar5 = 0;
      uVar4 = uVar5;
      if (*plVar20 != 0) {
        uVar4 = FUN_142f11a94();
      }
      FUN_14252d570(lVar11,param_3,uVar4);
      if (param_3 == 0x27f3) {
        if (*plVar20 != 0) {
          uVar5 = FUN_142f11a94();
        }
        *(undefined4 *)((longlong)param_1 + 0x2fec) = uVar5;
      }
      plVar20 = *(longlong **)(local_e0 + 0x1273);
      if (plVar20 != (longlong *)0x0) {
        plVar1 = plVar20 + *(uint *)(local_e0 + 0x127b);
        for (; plVar20 < plVar1; plVar20 = plVar20 + 1) {
          lVar11 = *plVar20;
          if (*plVar20 != 0) goto LAB_142d5c200;
        }
      }
      goto LAB_142d5c4d6;
    }
    if (param_2 != 2) goto LAB_142d5c5f9;
    if ((lVar15 == 0) || (iVar3 = FUN_1402fc440(lVar15,param_3), iVar3 == 0)) {
LAB_142d5b8b4:
      uVar10 = FUN_142cbe730(param_1);
      FUN_1402e33f0(uVar10,param_3,&param_5,0);
    }
    else {
      uVar10 = FUN_142cbe730(param_1);
      cVar2 = FUN_1402e3390(uVar10,param_3,&param_5);
      if (cVar2 == '\0') goto LAB_142d5b8b4;
    }
    uVar10 = FUN_142cbe730(param_1);
    FUN_1402e0ec0(uVar10,param_3);
    FUN_141d1fe90(param_1 + 0x5e0,local_res18);
    if ((DAT_143ad20e8 != (longlong *)0x0) &&
       (iVar3 = FUN_142507380(DAT_143ad20e8,param_3), iVar3 != 0)) {
      FUN_1424f0160(DAT_143ad20e8,param_3);
    }
    FUN_142d9d560(param_1,param_3);
    lVar11 = FUN_142cbe730(param_1);
    plVar20 = *(longlong **)(lVar11 + 0x1273);
    if (plVar20 != (longlong *)0x0) {
      plVar1 = plVar20 + *(uint *)(lVar11 + 0x127b);
      for (; plVar20 < plVar1; plVar20 = plVar20 + 1) {
        lVar11 = *plVar20;
        if (*plVar20 != 0) goto LAB_142d5b967;
      }
    }
    goto LAB_142d5bc29;
  }
  lVar15 = FUN_142cbe730(param_1);
  if (*(longlong *)(lVar15 + 0x1273) != 0) {
    for (lVar15 = *(longlong *)
                   (*(longlong *)(lVar15 + 0x1273) +
                   ((ulonglong)(longlong)(int)param_3 % (ulonglong)*(uint *)(lVar15 + 0x127b)) * 8);
        lVar15 != 0; lVar15 = *(longlong *)(lVar15 + 8)) {
      if (*(uint *)(lVar15 + 0x10) == param_3) {
        uVar10 = FUN_142cbe730(param_1);
        FUN_1402e0ec0(uVar10,param_3);
        FUN_141d1fe90(param_1 + 0x5e0,local_res18);
        break;
      }
    }
  }
  uVar10 = FUN_142cbe730(param_1);
  cVar2 = FUN_1402e3340(uVar10,param_3);
  if ((cVar2 != '\0') && (param_6 != 0)) {
    uVar10 = FUN_142cbe730(param_1);
    FUN_1402e3550(uVar10,param_3);
  }
  if ((DAT_143ad20e8 != (longlong *)0x0) &&
     (iVar3 = FUN_142507380(DAT_143ad20e8,param_3), iVar3 != 0)) {
    FUN_1424f0160(DAT_143ad20e8,param_3);
  }
  FUN_142d9d560(param_1,param_3);
  lVar15 = param_1[0x53c];
  if (lVar15 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar15 = param_1[0x53c];
  }
  FUN_14252d140(lVar15,param_3);
  goto LAB_142d5c5f9;
LAB_142d5c200:
  do {
    lVar9 = *(longlong *)(lVar11 + 8);
    if (*(longlong *)(lVar11 + 8) == 0) {
      for (plVar20 = (longlong *)
                     (*(longlong *)(local_e0 + 0x1273) +
                     ((ulonglong)(longlong)*(int *)(lVar11 + 0x10) %
                      (ulonglong)*(uint *)(local_e0 + 0x127b) + 1) * 8);
          (lVar9 = 0,
          plVar20 < (longlong *)
                    (*(longlong *)(local_e0 + 0x1273) + (ulonglong)*(uint *)(local_e0 + 0x127b) * 8)
          && (lVar9 = *plVar20, lVar9 == 0)); plVar20 = plVar20 + 1) {
      }
    }
    uVar7 = *(uint *)(lVar11 + 0x10);
    uVar22 = (ulonglong)(int)uVar7;
    local_e8[0] = uVar7;
    lVar11 = FUN_14070fa90(lVar15);
    if (lVar11 != 0) {
      uVar16 = uVar7;
      if (*(uint *)(lVar11 + 0xb8) != 0) {
        uVar16 = *(uint *)(lVar11 + 0xb8);
      }
      if (uVar16 == param_3) {
        uVar4 = FUN_142cb85d0(param_1);
        uVar5 = FUN_142cb85b0(param_1);
        uVar6 = FUN_142cafb20(param_1);
        lVar11 = FUN_14070fa90(lVar15,uVar7);
        uVar24 = local_e0;
        iVar3 = FUN_140711d70(lVar15,uVar7,*(undefined4 *)(lVar11 + 0x2c),uVar6,local_e0,local_d8,
                              local_c8,uVar5,uVar4,local_108);
        if (iVar3 == 0) {
          if (param_1[0x5e0] != 0) {
            for (lVar11 = *(longlong *)
                           (param_1[0x5e0] + (uVar22 % (ulonglong)*(uint *)(param_1 + 0x5e1)) * 8);
                lVar11 != 0; lVar11 = *(longlong *)(lVar11 + 8)) {
              if (*(uint *)(lVar11 + 0x10) == uVar7) {
                if (lVar11 != -0x14) goto LAB_142d5c4c8;
                break;
              }
            }
          }
          local_cc = 1;
          FUN_140726d40(param_1 + 0x5e0,local_e8,&local_cc);
          FUN_142d490d0(param_1,uVar7,0,1,uVar24 & 0xffffffff00000000);
          iVar3 = FUN_1407140b0(lVar15);
          if (iVar3 == 0) {
            local_c0 = FUN_14019b780(&DAT_143ad68a0,0x370);
            lVar11 = 0;
            if (local_c0 != 0) {
              lVar11 = FUN_141808b90(local_c0);
            }
            lVar12 = lVar11 + 0x18;
            if (lVar11 == 0) {
              lVar12 = 0;
            }
            if (lVar12 == 0) {
              local_a0 = 0;
            }
            else {
              local_a0 = lVar12 + -0x18;
              if (local_a0 != 0) {
                if (0xfffff < *(ulonglong *)(lVar12 + 8)) {
                  FUN_142e541f0(0x30f);
                }
                LOCK();
                *(longlong *)(lVar12 + 8) = *(longlong *)(lVar12 + 8) + 1;
                UNLOCK();
                uVar22 = (ulonglong)local_e8[0];
                param_3 = local_res18[0];
              }
            }
            lVar11 = local_a0;
            if (local_a0 == 0) {
              FUN_142e52ed0(0x431,0);
            }
            FUN_14180d7f0(lVar11,uVar22 & 0xffffffff);
            local_60 = lVar11;
            if (lVar11 != 0) {
              if (0xfffff < *(ulonglong *)(lVar11 + 0x20)) {
                FUN_142e541f0(0x30f);
              }
              LOCK();
              *(longlong *)(lVar11 + 0x20) = *(longlong *)(lVar11 + 0x20) + 1;
              UNLOCK();
              lVar11 = local_a0;
              param_3 = local_res18[0];
            }
            FUN_142d97880(param_1);
            if (lVar11 != 0) {
              if (0xffffe < *(longlong *)(lVar11 + 0x20) - 1U) {
                FUN_142e541f0(0x31e);
              }
              LOCK();
              plVar20 = (longlong *)(lVar11 + 0x20);
              lVar11 = *plVar20;
              *plVar20 = *plVar20 + -1;
              UNLOCK();
              if (((int)lVar11 == 1) && ((undefined8 *)(local_a0 + 0x18) != (undefined8 *)0x0)) {
                (*(code *)**(undefined8 **)(local_a0 + 0x18))();
              }
              local_a0 = 0;
              param_3 = local_res18[0];
            }
          }
        }
      }
    }
LAB_142d5c4c8:
    lVar11 = lVar9;
  } while (lVar9 != 0);
LAB_142d5c4d6:
  lVar15 = FUN_141892840();
  if (lVar15 != 0) {
    FUN_14185d8d0(lVar15);
    FUN_14185df30(lVar15);
  }
  if (local_f8 != (int *)0x0) {
    FUN_14019f2c0(local_f8 + -4);
  }
  goto LAB_142d5c5f9;
LAB_142d5b967:
  do {
    lVar12 = FUN_142cbe730(param_1);
    lVar9 = *(longlong *)(lVar11 + 8);
    if (*(longlong *)(lVar11 + 8) == 0) {
      local_res8 = 0;
      for (plVar20 = (longlong *)
                     (*(longlong *)(lVar12 + 0x1273) +
                     ((ulonglong)(longlong)*(int *)(lVar11 + 0x10) %
                      (ulonglong)*(uint *)(lVar12 + 0x127b) + 1) * 8);
          (lVar9 = local_res8,
          plVar20 < (longlong *)
                    (*(longlong *)(lVar12 + 0x1273) + (ulonglong)*(uint *)(lVar12 + 0x127b) * 8) &&
          (lVar9 = *plVar20, *plVar20 == 0)); plVar20 = plVar20 + 1) {
      }
    }
    local_res8 = lVar9;
    uVar7 = *(uint *)(lVar11 + 0x10);
    if (param_3 != uVar7) {
      if (lVar15 == 0) break;
      lVar11 = FUN_14070fa90(lVar15);
      piVar13 = piVar21;
      piVar18 = piVar21;
      if (lVar11 != 0) {
        while( true ) {
          lVar9 = *(longlong *)(lVar11 + 0xd8);
          if ((lVar9 == 0) || (uVar16 = (uint)piVar13, *(uint *)(lVar9 + -8) <= uVar16))
          goto LAB_142d5bc20;
          if ((int)uVar16 < 0) {
            FUN_142e54290(0xc6);
            lVar9 = *(longlong *)(lVar11 + 0xd8);
          }
          if (*(uint *)(lVar9 + (longlong)piVar18) == param_3) break;
          piVar13 = (int *)(ulonglong)(uVar16 + 1);
          piVar18 = piVar18 + 3;
        }
        if (DAT_143ad20e8 != (longlong *)0x0) {
          FUN_1424efe50(DAT_143ad20e8,uVar7,1);
        }
        uVar4 = FUN_142cb85d0(param_1);
        uVar5 = FUN_142cb85b0(param_1);
        uVar6 = FUN_142cafb20(param_1);
        lVar11 = FUN_14070fa90(lVar15,uVar7);
        uVar22 = local_e0;
        iVar3 = FUN_140711d70(lVar15,uVar7,*(undefined4 *)(lVar11 + 0x2c),uVar6,local_e0,local_d8,
                              local_c8,uVar5,uVar4,local_108);
        if (iVar3 == 0) {
          FUN_142d490d0(param_1,uVar7,0,1,uVar22 & 0xffffffff00000000);
          iVar3 = FUN_1407140b0(lVar15);
          if (iVar3 == 0) {
            local_90 = FUN_14019b780(&DAT_143ad68a0,0x370);
            piVar13 = piVar21;
            if (local_90 != 0) {
              piVar13 = (int *)FUN_141808b90(local_90);
            }
            piVar18 = piVar13 + 6;
            if (piVar13 == (int *)0x0) {
              piVar18 = piVar21;
            }
            if (piVar18 == (int *)0x0) {
              local_b0 = (int *)0x0;
            }
            else {
              local_b0 = piVar18 + -6;
              if (local_b0 != (int *)0x0) {
                if (0xfffff < *(ulonglong *)(piVar18 + 2)) {
                  FUN_142e541f0(0x30f);
                }
                LOCK();
                *(longlong *)(piVar18 + 2) = *(longlong *)(piVar18 + 2) + 1;
                UNLOCK();
                param_3 = local_res18[0];
              }
            }
            piVar13 = local_b0;
            if (local_b0 == (int *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            FUN_14180d7f0(piVar13,uVar7);
            local_70 = piVar13;
            if (piVar13 != (int *)0x0) {
              if (0xfffff < *(ulonglong *)(piVar13 + 8)) {
                FUN_142e541f0(0x30f);
              }
              LOCK();
              *(longlong *)(piVar13 + 8) = *(longlong *)(piVar13 + 8) + 1;
              UNLOCK();
              piVar13 = local_b0;
              param_3 = local_res18[0];
            }
            FUN_142d97880(param_1);
            if (piVar13 != (int *)0x0) {
              if (0xffffe < *(longlong *)(piVar13 + 8) - 1U) {
                FUN_142e541f0(0x31e);
              }
              LOCK();
              plVar20 = (longlong *)(piVar13 + 8);
              lVar11 = *plVar20;
              *plVar20 = *plVar20 + -1;
              UNLOCK();
              if (((int)lVar11 == 1) && (local_b0 + 6 != (int *)0x0)) {
                (*(code *)**(undefined8 **)(local_b0 + 6))();
              }
              local_b0 = (int *)0x0;
              param_3 = local_res18[0];
            }
          }
        }
      }
    }
LAB_142d5bc20:
    lVar11 = local_res8;
  } while (local_res8 != 0);
LAB_142d5bc29:
  lVar11 = param_1[0x53c];
  if (lVar11 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar11 = param_1[0x53c];
  }
  FUN_14252d140(lVar11,param_3);
  if (((param_3 == 0x1964) || (param_3 == 0x1ecc)) && (iVar3 = FUN_142cc4060(param_1), iVar3 == 0))
  {
    FUN_142cc4070(param_1,1);
    if (DAT_143aca610 != 0) {
      FUN_1420fbb50();
    }
    FUN_141157a50();
    uVar10 = FUN_1408a9e40(&local_d8,0x11c6);
    FUN_1415eca30(uVar10,0xb);
    if (local_d8 != 0) {
      FUN_14019f2c0(local_d8 + -0x10);
    }
  }
  if (((lVar15 != 0) && (iVar3 = FUN_140715e60(lVar15,param_3), iVar3 != 0)) && (DAT_143ad8410 != 0)
     ) {
    FUN_14241f510(DAT_143ad8410,1);
  }
  uVar7 = FUN_142cc41a0(param_1);
  if (param_3 == uVar7) {
    FUN_142d49390(param_1,1);
  }
  FUN_141429810();
LAB_142d5c5f9:
  lVar15 = *local_res20;
LAB_142d5c600:
  if (lVar15 != 0) {
    FUN_14019f2c0(lVar15 + -0x10);
  }
  return;
}



//===========================================================
// FUN_142cbefd0 @ 142cbefd0   (1202 bytes)
//===========================================================

void FUN_142cbefd0(longlong *param_1,int param_2,int param_3,undefined4 param_4,undefined4 param_5,
                  undefined8 param_6)

{
  undefined4 uVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  undefined4 uVar5;
  int iVar6;
  int iVar7;
  undefined8 uVar8;
  longlong lVar9;
  undefined8 uVar10;
  longlong lVar11;
  longlong *plVar12;
  longlong *plVar13;
  undefined8 *puVar14;
  longlong lVar15;
  int iVar16;
  undefined8 in_stack_ffffffffffffff30;
  uint uVar17;
  undefined1 **ppuVar18;
  undefined4 uVar20;
  undefined8 uVar19;
  uint uVar21;
  undefined8 in_stack_ffffffffffffff60;
  uint uVar22;
  undefined1 *local_98;
  longlong local_90;
  undefined1 local_88 [8];
  undefined8 *local_80;
  undefined1 local_78 [8];
  undefined8 local_70;
  undefined1 local_68 [24];
  undefined8 local_50;
  
  uVar17 = (uint)((ulonglong)in_stack_ffffffffffffff30 >> 0x20);
  uVar22 = (uint)((ulonglong)in_stack_ffffffffffffff60 >> 0x20);
  lVar15 = param_1[0x46b];
  if (lVar15 != 0) {
    uVar5 = 0;
    local_50 = 0;
    uVar8 = (**(code **)(*param_1 + 0x30))();
    lVar9 = FUN_141892840();
    if (lVar9 != 0) {
      uVar10 = FUN_141892840();
      uVar5 = FUN_141829fd0(uVar10);
    }
    local_80 = (undefined8 *)0x0;
    local_90 = 0;
    uVar21 = 0;
    ppuVar18 = &local_98;
    FUN_14087f030(uVar5,lVar15,uVar8,param_1[0x4b2],param_1[0x4b3],param_1 + 0x4d4,param_1 + 0x4b1,
                  (ulonglong)uVar17 << 0x20,0,ppuVar18,local_88,0,0,(ulonglong)uVar22 << 0x20);
    lVar9 = local_90;
    uVar20 = (undefined4)((ulonglong)ppuVar18 >> 0x20);
    if (local_90 != 0) {
      puVar14 = (undefined8 *)(local_90 + -0x28);
      if (0xffffe < *(longlong *)(local_90 + -0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar13 = (longlong *)(lVar9 + -0x20);
      lVar9 = *plVar13;
      *plVar13 = *plVar13 + -1;
      UNLOCK();
      if ((int)lVar9 == 1) {
        if ((local_90 != 0) && (*(longlong *)(local_90 + -0x10) != 0)) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_90 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_90 + -0x10) + 4) != 0);
        }
        if (puVar14 != (undefined8 *)0x0) {
          (**(code **)*puVar14)(puVar14,1);
        }
      }
      local_90 = 0;
    }
    puVar14 = local_80;
    if (local_80 != (undefined8 *)0x0) {
      if (0xffffe < local_80[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar13 = puVar14 + 1;
      lVar9 = *plVar13;
      *plVar13 = *plVar13 + -1;
      UNLOCK();
      if (((int)lVar9 == 1) && (local_80 != (undefined8 *)0x0)) {
        (**(code **)*local_80)(local_80,1);
      }
    }
    lVar9 = param_1[0x46c];
    local_98 = local_78;
    local_70 = 0;
    uVar1 = *(undefined4 *)((longlong)param_1 + 0x3bb4);
    lVar3 = param_1[0x776];
    lVar2 = param_1[0x4b3];
    lVar11 = param_1[0x4b2];
    uVar10 = (**(code **)(*param_1 + 0x38))(param_1);
    uVar19 = CONCAT44(uVar20,uVar1);
    FUN_14085b3b0(lVar9,uVar5,lVar15,uVar8,uVar10,lVar11,lVar2,param_1 + 0x4d4,(int)lVar3,uVar19,
                  local_78,0,uVar21 & 0xffffff00,param_6);
    uVar20 = (undefined4)((ulonglong)uVar19 >> 0x20);
    uVar8 = (**(code **)(*param_1 + 0x30))(param_1);
    lVar11 = FUN_142cbe750(param_1,local_68);
    lVar9 = param_1[0x4b3];
    FUN_14087c130(uVar8,lVar15,param_1[0x46c],param_1[0x4b2],lVar9,lVar11);
    lVar2 = param_1[0x813];
    if (lVar2 != 0) {
      uVar1 = *(undefined4 *)((longlong)param_1 + 0x3bb4);
      lVar4 = param_1[0x776];
      lVar3 = param_1[0x4b3];
      lVar11 = param_1[0x4b2];
      lVar9 = (**(code **)(*param_1 + 0x38))(param_1);
      uVar8 = (**(code **)(*param_1 + 0x30))(param_1);
      FUN_1408a1da0(lVar2,uVar5,lVar15,uVar8,lVar9,lVar11,lVar3,param_1 + 0x4d4,(int)lVar4,
                    CONCAT44(uVar20,uVar1));
    }
    FUN_142d3b010(param_5);
    if (param_3 == 0) {
      iVar16 = 2;
      do {
        iVar6 = FUN_140255590(iVar16);
        lVar15 = (longlong)iVar6;
        if (0 < iVar6) {
          plVar13 = param_1 + 0x54a;
          iVar6 = 0;
          do {
            iVar7 = FUN_142d9ed40(param_1,iVar16,iVar6);
            if (iVar7 != 0) {
              if (iVar16 == 2) {
                plVar12 = (longlong *)plVar13[-6];
                if (plVar12 == (longlong *)0x0) {
                  FUN_142e52ed0(0x431,0);
                  plVar12 = (longlong *)plVar13[-6];
                }
              }
              else if (iVar16 == 3) {
                plVar12 = (longlong *)*plVar13;
                if (plVar12 == (longlong *)0x0) {
                  FUN_142e52ed0(0x431,0);
                  plVar12 = (longlong *)*plVar13;
                }
              }
              else {
                if (iVar16 != 4) goto LAB_142cbf3a0;
                plVar12 = (longlong *)plVar13[0x14];
                if (plVar12 == (longlong *)0x0) {
                  FUN_142e52ed0(0x431,0);
                  plVar12 = (longlong *)plVar13[0x14];
                }
              }
              (**(code **)(*plVar12 + 0x90))(plVar12,0);
            }
LAB_142cbf3a0:
            iVar6 = iVar6 + 1;
            plVar13 = plVar13 + 2;
            lVar15 = lVar15 + -1;
          } while (lVar15 != 0);
        }
        uVar5 = (undefined4)((ulonglong)lVar9 >> 0x20);
        uVar20 = (undefined4)((ulonglong)lVar11 >> 0x20);
        iVar16 = iVar16 + 1;
      } while (iVar16 < 5);
      FUN_142cbf740(param_1);
      FUN_142cbf9d0(param_1);
      lVar15 = DAT_143aa8518;
      if (DAT_143aa8518 != 0) {
        if (param_2 == 0) {
          plVar13 = (longlong *)(DAT_143aa9d98 + 0x630);
          lVar9 = *plVar13;
          if ((lVar9 != 0) && (*(int *)(lVar9 + -8) != 0)) {
            param_1[0x37] = 0;
            FUN_142ce5e80(param_1,plVar13,1,0,CONCAT44(uVar5,0xffffffff),CONCAT44(uVar20,param_4));
            if (param_1[0x53a] != 0) {
              FUN_142514ff0();
            }
          }
        }
        else {
          *(undefined4 *)(param_1 + 0x37) = 1;
        }
        FUN_1428e0cf0(lVar15);
        FUN_14090b2e0(param_1);
        FUN_1428e0d80(lVar15);
        FUN_1428ed470(lVar15,0xffffffff);
      }
    }
  }
  return;
}



//===========================================================
// FUN_142ce6750 @ 142ce6750   (679 bytes)
//===========================================================

void FUN_142ce6750(longlong *param_1,int param_2)

{
  undefined8 *puVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  undefined4 uVar5;
  char cVar6;
  int iVar7;
  undefined4 uVar8;
  longlong lVar9;
  undefined8 uVar10;
  longlong lVar11;
  undefined8 *puVar12;
  undefined8 *puVar13;
  undefined8 *puVar14;
  undefined8 uVar15;
  int local_res10 [2];
  undefined4 local_res18 [2];
  longlong local_res20;
  undefined1 local_60 [8];
  longlong local_58;
  undefined1 local_50 [8];
  undefined8 local_48;
  
  local_res10[0] = param_2;
  cVar6 = FUN_141b1f960(DAT_143abea80,1);
  lVar4 = DAT_143aa9d98;
  if (((cVar6 == '\0') && (DAT_143aa9d98 != 0)) &&
     (iVar7 = FUN_1407158a0(DAT_143aa9d98,param_2), iVar7 != 0)) {
    lVar9 = FUN_14070fa90(lVar4,param_2);
    if (lVar9 != 0) {
      lVar3 = param_1[0x46b];
      uVar10 = (**(code **)(*param_1 + 0x30))(param_1);
      local_res20 = param_1[0x46c];
      local_res18[0] = *(undefined4 *)(lVar9 + 0x2c);
      if (DAT_143aa8518 == 0) {
        uVar15 = 0;
        local_48 = 0;
      }
      else {
        lVar11 = FUN_1428f74d0(DAT_143aa8518,local_60);
        lVar9 = local_58;
        uVar15 = *(undefined8 *)(lVar11 + 8);
        if (local_58 != 0) {
          puVar1 = (undefined8 *)(local_58 + -0x28);
          if (0xffffe < *(longlong *)(local_58 + -0x20) - 1U) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar2 = (longlong *)(lVar9 + -0x20);
          lVar9 = *plVar2;
          *plVar2 = *plVar2 + -1;
          UNLOCK();
          if ((int)lVar9 == 1) {
            if ((local_58 != 0) && (*(longlong *)(local_58 + -0x10) != 0)) {
              LOCK();
              *(undefined8 *)(*(longlong *)(local_58 + -0x10) + 8) = 0;
              UNLOCK();
              do {
              } while (*(int *)(*(longlong *)(local_58 + -0x10) + 4) != 0);
            }
            if (puVar1 != (undefined8 *)0x0) {
              (**(code **)*puVar1)(puVar1,1);
            }
          }
          local_58 = 0;
          param_2 = local_res10[0];
        }
      }
      uVar8 = 0xffffffff;
      lVar9 = param_1[0x460];
      lVar11 = FUN_141892840();
      if (lVar11 != 0) {
        uVar8 = FUN_141829f70(lVar11);
      }
      uVar5 = local_res18[0];
      iVar7 = FUN_140711d70(lVar4,param_2,local_res18[0],uVar8,lVar3,uVar10,local_res20,
                            0 < (int)lVar9,(int)lVar9,uVar15);
      if (iVar7 == 0) {
        puVar1 = (undefined8 *)param_1[0x5db];
        cVar6 = *(char *)((longlong)puVar1[1] + 0x19);
        puVar12 = (undefined8 *)puVar1[1];
        puVar14 = puVar1;
        while (puVar13 = puVar12, cVar6 == '\0') {
          if (*(int *)((longlong)puVar13 + 0x1c) < param_2) {
            puVar12 = (undefined8 *)puVar13[2];
            puVar13 = puVar14;
          }
          else {
            puVar12 = (undefined8 *)*puVar13;
          }
          cVar6 = *(char *)((longlong)puVar12 + 0x19);
          puVar14 = puVar13;
        }
        if (((*(char *)((longlong)puVar14 + 0x19) == '\0') &&
            (*(int *)((longlong)puVar14 + 0x1c) <= param_2)) && (puVar14 != puVar1)) {
          return;
        }
        FUN_142d9ac30(param_1,param_2,uVar5,0);
        local_res18[0] = 1;
        FUN_142d28eb0(param_1 + 0x5db,local_50,local_res10,local_res18);
        return;
      }
    }
    puVar1 = (undefined8 *)param_1[0x5db];
    cVar6 = *(char *)((longlong)puVar1[1] + 0x19);
    puVar12 = puVar1;
    puVar14 = (undefined8 *)puVar1[1];
    while (cVar6 == '\0') {
      if (*(int *)((longlong)puVar14 + 0x1c) < param_2) {
        puVar13 = (undefined8 *)puVar14[2];
        puVar14 = puVar12;
      }
      else {
        puVar13 = (undefined8 *)*puVar14;
      }
      puVar12 = puVar14;
      puVar14 = puVar13;
      cVar6 = *(char *)((longlong)puVar13 + 0x19);
    }
    if (((*(char *)((longlong)puVar12 + 0x19) == '\0') &&
        (*(int *)((longlong)puVar12 + 0x1c) <= param_2)) && (puVar12 != puVar1)) {
      FUN_14029bb00(param_1 + 0x5db,local_res10);
    }
  }
  return;
}



//===========================================================
// FUN_142ce5f80 @ 142ce5f80   (1985 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000142ce61b9) */
/* WARNING: Removing unreachable block (ram,0x000142ce61cd) */
/* WARNING: Removing unreachable block (ram,0x000142ce61da) */
/* WARNING: Removing unreachable block (ram,0x000142ce61ea) */
/* WARNING: Removing unreachable block (ram,0x000142ce61fb) */
/* WARNING: Removing unreachable block (ram,0x000142ce6202) */
/* WARNING: Removing unreachable block (ram,0x000142ce620e) */
/* WARNING: Removing unreachable block (ram,0x000142ce6213) */
/* WARNING: Removing unreachable block (ram,0x000142ce6220) */

void FUN_142ce5f80(longlong *param_1,int param_2,int param_3,undefined8 *param_4,int param_5)

{
  undefined4 uVar1;
  char cVar2;
  int iVar3;
  undefined4 uVar4;
  longlong lVar5;
  longlong lVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  longlong *plVar10;
  undefined4 uVar11;
  longlong lVar12;
  int local_res10 [2];
  undefined4 local_78;
  undefined8 *local_70;
  undefined8 local_68;
  longlong local_60;
  undefined8 local_58;
  longlong local_50;
  undefined1 local_48 [8];
  longlong local_40;
  undefined8 local_30;
  
  local_res10[0] = param_2;
  cVar2 = FUN_141b1f960(DAT_143abea80,1);
  lVar12 = DAT_143aa9d98;
  if (cVar2 != '\0') {
    return;
  }
  if (DAT_143aa9d98 == 0) {
    return;
  }
  iVar3 = FUN_1407157b0(DAT_143aa9d98,param_2);
  if ((iVar3 == 0) && (iVar3 = FUN_140715a30(lVar12,param_2), iVar3 == 0)) {
    return;
  }
  iVar3 = FUN_142da3e70(param_1,param_2);
  if (iVar3 != 0) {
    return;
  }
  iVar3 = FUN_1407163a0(lVar12,param_2);
  if (iVar3 != 0) {
    return;
  }
  if ((param_1[0x787] != 0) && (cVar2 = FUN_1417fd060(param_1[0x787],param_2), cVar2 != '\0')) {
    return;
  }
  local_50 = param_1[0x46b];
  local_70 = param_4;
  local_58 = (**(code **)(*param_1 + 0x30))(param_1);
  local_60 = param_1[0x46c];
  lVar5 = FUN_141892840();
  if (lVar5 == 0) {
    local_78 = 0xffffffff;
  }
  else {
    local_78 = FUN_141829f70(lVar5);
  }
  uVar1 = *(undefined4 *)(DAT_143aa84a0 + 0x2274);
  lVar5 = FUN_14070fa40(lVar12,param_2);
  if (lVar5 == 0) {
    uVar11 = 0;
  }
  else {
    lVar5 = FUN_14070fa40(lVar12,param_2);
    uVar11 = *(undefined4 *)(lVar5 + 0x2c);
  }
  local_68 = *(undefined8 *)((longlong)param_1 + 0x238c);
  if (DAT_143aa8518 == 0) {
    local_30 = 0;
    uVar7 = 0;
  }
  else {
    lVar6 = FUN_1428f74d0(DAT_143aa8518,local_48);
    lVar5 = local_40;
    uVar7 = *(undefined8 *)(lVar6 + 8);
    if (local_40 != 0) {
      puVar8 = (undefined8 *)(local_40 + -0x28);
      if (0xffffe < *(longlong *)(local_40 + -0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar10 = (longlong *)(lVar5 + -0x20);
      lVar5 = *plVar10;
      *plVar10 = *plVar10 + -1;
      UNLOCK();
      if ((int)lVar5 == 1) {
        if ((local_40 != 0) && (*(longlong *)(local_40 + -0x10) != 0)) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_40 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_40 + -0x10) + 4) != 0);
        }
        if (puVar8 != (undefined8 *)0x0) {
          (**(code **)*puVar8)(puVar8,1);
          local_40 = 0;
          param_2 = local_res10[0];
          goto LAB_142ce6229;
        }
      }
      local_40 = 0;
      param_2 = local_res10[0];
    }
  }
LAB_142ce6229:
  puVar8 = (undefined8 *)0x0;
  iVar3 = FUN_1407159c0(lVar12);
  if (iVar3 == 0) {
    if (param_5 != 0) {
      iVar3 = (*DAT_143262db0)();
      if (iVar3 - *(int *)((longlong)param_1 + 0x363c) < 0x3e9) {
        if (param_1[0x5ca] != 0) {
          for (lVar5 = *(longlong *)
                        (param_1[0x5ca] +
                        ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(param_1 + 0x5cb)) * 8);
              lVar5 != 0; lVar5 = *(longlong *)(lVar5 + 8)) {
            if (*(int *)(lVar5 + 0x10) == param_2) {
              if (lVar5 != -0x14) goto LAB_142ce64a5;
              break;
            }
          }
        }
        if (param_1[0x5cd] == 0) {
          return;
        }
        lVar5 = *(longlong *)
                 (param_1[0x5cd] +
                 ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(param_1 + 0x5ce)) * 8);
        while( true ) {
          if (lVar5 == 0) {
            return;
          }
          if (*(int *)(lVar5 + 0x10) == param_2) break;
          lVar5 = *(longlong *)(lVar5 + 8);
        }
        if (lVar5 == -0x14) {
          return;
        }
      }
      else {
        *(int *)((longlong)param_1 + 0x363c) = iVar3;
      }
    }
LAB_142ce64a5:
    uVar4 = FUN_1413b8e00();
    iVar3 = FUN_14070fae0(lVar12,param_2,uVar11,local_50,local_58,local_60,
                          *(undefined4 *)((longlong)param_1 + 0x2f2c),local_78,uVar1,
                          0 < (int)param_1[0x460],(int)param_1[0x460],&local_68,uVar4,0,param_4,0,1,
                          uVar7);
    if (iVar3 == 0) {
      iVar3 = FUN_140715800(lVar12);
      if (iVar3 == 0) {
        if (param_3 != 0) {
          if (param_1[0x5ca] != 0) {
            for (lVar12 = *(longlong *)
                           (param_1[0x5ca] +
                           ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(param_1 + 0x5cb)) *
                           8); lVar12 != 0; lVar12 = *(longlong *)(lVar12 + 8)) {
              if (*(int *)(lVar12 + 0x10) == param_2) {
                if (lVar12 != -0x14) goto LAB_142ce6640;
                break;
              }
            }
          }
          if (param_1[0x53a] != 0) {
            uVar7 = FUN_142954390(param_1 + 0x539);
            FUN_142518060(uVar7,param_2,10000);
          }
          FUN_1428fd9c0(DAT_143aa8518,param_2);
        }
LAB_142ce6640:
        plVar10 = param_1 + 0x5ca;
        lVar12 = *plVar10;
        if (lVar12 == 0) {
          puVar8 = (undefined8 *)(ulonglong)*(uint *)(param_1 + 0x5cb);
        }
        else if (*(uint *)((longlong)param_1 + 0x2e64) < *(uint *)((longlong)param_1 + 0x2e5c)) {
          puVar8 = (undefined8 *)(ulonglong)(uint)((int)param_1[0x5cb] * 2);
        }
        if ((int)puVar8 != 0) {
          FUN_140370200(plVar10,puVar8,0);
          lVar12 = *plVar10;
        }
        plVar10 = (longlong *)
                  (lVar12 + ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(param_1 + 0x5cb)) *
                            8);
        for (lVar12 = *plVar10; lVar12 != 0; lVar12 = *(longlong *)(lVar12 + 8)) {
          if (*(int *)(lVar12 + 0x10) == param_2) {
            *(undefined4 *)(lVar12 + 0x14) = 1;
            return;
          }
        }
        *(int *)((longlong)param_1 + 0x2e5c) = *(int *)((longlong)param_1 + 0x2e5c) + 1;
        puVar8 = (undefined8 *)FUN_141d13fa0(0x18);
        if (puVar8 == (undefined8 *)0x0) {
          uRam0000000000000014 = 1;
          *plVar10 = 0;
        }
        else {
          lVar12 = *plVar10;
          *puVar8 = &PTR_FUN_1432822e8;
          puVar8[1] = lVar12;
          puVar8[2] = 0;
          *(int *)(puVar8 + 2) = param_2;
          *(undefined4 *)((longlong)puVar8 + 0x14) = 1;
          *plVar10 = (longlong)puVar8;
        }
      }
      else {
        if (param_1[0x5cd] != 0) {
          for (lVar12 = *(longlong *)
                         (param_1[0x5cd] +
                         ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(param_1 + 0x5ce)) * 8)
              ; lVar12 != 0; lVar12 = *(longlong *)(lVar12 + 8)) {
            if (*(int *)(lVar12 + 0x10) == param_2) {
              if (lVar12 != -0x14) {
                return;
              }
              break;
            }
          }
        }
        FUN_142d9ac30(param_1,param_2,uVar11,1);
        FUN_1402fce50(param_1 + 0x5cd,local_res10,&local_70);
      }
    }
    else {
      FUN_141d1fe90(param_1 + 0x5ca,local_res10);
      FUN_1402fea20(param_1 + 0x5cd,local_res10);
    }
  }
  else {
    plVar10 = param_1 + 0x5d4;
    if (*plVar10 != 0) {
      for (lVar5 = *(longlong *)
                    (*plVar10 +
                    ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(param_1 + 0x5d5)) * 8);
          lVar5 != 0; lVar5 = *(longlong *)(lVar5 + 8)) {
        if (*(int *)(lVar5 + 0x10) == param_2) {
          if (lVar5 != -0x14) {
            return;
          }
          break;
        }
      }
    }
    if ((int)param_1[0x5d7] == 0) {
      uVar4 = FUN_1413b8e00();
      iVar3 = FUN_14070fae0(lVar12,param_2,uVar11,local_50,local_58,local_60,
                            *(undefined4 *)((longlong)param_1 + 0x2f2c),local_78,uVar1,
                            0 < (int)param_1[0x460],(int)param_1[0x460],&local_68,uVar4,0,param_4,0,
                            1,uVar7);
      if (iVar3 == 0) {
        lVar12 = *plVar10;
        if (lVar12 == 0) {
          puVar9 = (undefined8 *)(ulonglong)*(uint *)(param_1 + 0x5d5);
        }
        else {
          puVar9 = puVar8;
          if (*(uint *)((longlong)param_1 + 0x2eb4) < *(uint *)((longlong)param_1 + 0x2eac)) {
            puVar9 = (undefined8 *)(ulonglong)(uint)((int)param_1[0x5d5] * 2);
          }
        }
        if ((int)puVar9 != 0) {
          FUN_140370200(plVar10,puVar9,0);
          lVar12 = *plVar10;
        }
        plVar10 = (longlong *)
                  (lVar12 + ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(param_1 + 0x5d5)) *
                            8);
        for (lVar12 = *plVar10; lVar12 != 0; lVar12 = *(longlong *)(lVar12 + 8)) {
          if (*(int *)(lVar12 + 0x10) == param_2) goto LAB_142ce63cd;
        }
        *(int *)((longlong)param_1 + 0x2eac) = *(int *)((longlong)param_1 + 0x2eac) + 1;
        local_70 = (undefined8 *)FUN_141d13fa0(0x18);
        if (local_70 != (undefined8 *)0x0) {
          lVar12 = *plVar10;
          *local_70 = &PTR_FUN_1432822e8;
          local_70[1] = lVar12;
          local_70[2] = 0;
          *(int *)(local_70 + 2) = param_2;
          puVar8 = local_70;
        }
        *plVar10 = (longlong)puVar8;
LAB_142ce63cd:
        *(undefined4 *)(param_1 + 0x5d7) = 1;
        FUN_142d9ac30(param_1,param_2,uVar11,1);
        *(undefined4 *)(param_1 + 0x5d7) = 0;
      }
    }
  }
  return;
}



//===========================================================
// FUN_142ce6a00 @ 142ce6a00   (726 bytes)
//===========================================================

void FUN_142ce6a00(longlong *param_1,int param_2)

{
  longlong *plVar1;
  undefined4 uVar2;
  longlong lVar3;
  longlong lVar4;
  char cVar5;
  int iVar6;
  undefined4 uVar7;
  longlong lVar8;
  undefined8 uVar9;
  longlong lVar10;
  longlong lVar11;
  int *piVar12;
  ulonglong uVar13;
  undefined8 *puVar14;
  undefined8 uVar15;
  undefined1 local_58 [8];
  longlong local_50;
  
  cVar5 = FUN_141b1f960(DAT_143abea80,1);
  lVar4 = DAT_143aa9d98;
  if ((((cVar5 == '\0') && (DAT_143aa9d98 != 0)) &&
      (iVar6 = FUN_140715940(DAT_143aa9d98,param_2), iVar6 != 0)) &&
     ((iVar6 = FUN_1407163a0(lVar4,param_2), iVar6 == 0 &&
      (lVar8 = FUN_14070fa90(lVar4,param_2), lVar8 != 0)))) {
    lVar8 = param_1[0x46b];
    uVar9 = (**(code **)(*param_1 + 0x30))(param_1);
    lVar3 = param_1[0x46c];
    lVar10 = FUN_14070fa90(lVar4,param_2);
    uVar2 = *(undefined4 *)(lVar10 + 0x2c);
    if (DAT_143aa8518 == 0) {
      uVar15 = 0;
    }
    else {
      lVar11 = FUN_1428f74d0(DAT_143aa8518,local_58);
      lVar10 = local_50;
      uVar15 = *(undefined8 *)(lVar11 + 8);
      if (local_50 != 0) {
        puVar14 = (undefined8 *)(local_50 + -0x28);
        if (0xffffe < *(longlong *)(local_50 + -0x20) - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar1 = (longlong *)(lVar10 + -0x20);
        lVar10 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if ((int)lVar10 == 1) {
          if ((local_50 != 0) && (*(longlong *)(local_50 + -0x10) != 0)) {
            LOCK();
            *(undefined8 *)(*(longlong *)(local_50 + -0x10) + 8) = 0;
            UNLOCK();
            do {
            } while (*(int *)(*(longlong *)(local_50 + -0x10) + 4) != 0);
          }
          if (puVar14 != (undefined8 *)0x0) {
            (**(code **)*puVar14)(puVar14,1);
          }
        }
        local_50 = 0;
      }
    }
    uVar7 = 0xffffffff;
    iVar6 = FUN_142da3e70(param_1,param_2);
    if (iVar6 == 0) {
      lVar10 = param_1[0x460];
      lVar11 = FUN_141892840();
      if (lVar11 != 0) {
        uVar7 = FUN_141829f70(lVar11);
      }
      iVar6 = FUN_140711d70(lVar4,param_2,uVar2,uVar7,lVar8,uVar9,lVar3,0 < (int)lVar10,(int)lVar10,
                            uVar15);
      if (iVar6 == 0) {
        piVar12 = (int *)param_1[0x5d9];
        while( true ) {
          if (piVar12 == (int *)0x0) {
            if (param_1[0x53a] != 0) {
              uVar9 = FUN_142954390(param_1 + 0x539);
              FUN_142518060(uVar9,param_2,10000);
            }
            piVar12 = (int *)FUN_1402bf2b0(param_1 + 0x5d8);
            *piVar12 = param_2;
            return;
          }
          if (*piVar12 == param_2) break;
          uVar13 = *(ulonglong *)(piVar12 + -8);
          if ((uVar13 != 0) && (uVar13 < 0x10001)) {
            FUN_142e52ed0(0x33e);
            uVar13 = *(ulonglong *)(piVar12 + -8);
          }
          piVar12 = (int *)0x0;
          if (uVar13 != 0) {
            piVar12 = (int *)(uVar13 + 0x28);
          }
        }
        return;
      }
    }
    piVar12 = (int *)param_1[0x5d9];
    while (piVar12 != (int *)0x0) {
      if (*piVar12 == param_2) {
        FUN_1402fe6e0(param_1 + 0x5d8,piVar12);
        return;
      }
      uVar13 = *(ulonglong *)(piVar12 + -8);
      if ((uVar13 != 0) && (uVar13 < 0x10001)) {
        FUN_142e52ed0(0x33e);
        uVar13 = *(ulonglong *)(piVar12 + -8);
      }
      piVar12 = (int *)0x0;
      if (uVar13 != 0) {
        piVar12 = (int *)(uVar13 + 0x28);
      }
    }
  }
  return;
}



//===========================================================
// FUN_142ce5e60 @ 142ce5e60   (17 bytes)
//===========================================================

void FUN_142ce5e60(longlong param_1)

{
  undefined8 *puVar1;
  longlong *plVar2;
  IUnknown *pIVar3;
  longlong lVar4;
  uint uVar5;
  bool bVar6;
  bool bVar7;
  bool bVar8;
  bool bVar9;
  undefined8 *puVar10;
  longlong *plVar11;
  char cVar12;
  char cVar13;
  undefined4 uVar14;
  int iVar15;
  int iVar16;
  int iVar17;
  uint uVar18;
  int iVar19;
  longlong lVar20;
  undefined8 uVar21;
  longlong lVar22;
  longlong *plVar23;
  undefined8 *puVar24;
  undefined4 *puVar25;
  undefined8 *puVar26;
  ulonglong uVar27;
  longlong lVar28;
  longlong lVar29;
  int *piVar30;
  int *piVar31;
  ulonglong uVar32;
  bool bVar33;
  int iStack_a4;
  int iStack_a0;
  uint uStack_98;
  longlong *aplStack_88 [2];
  undefined8 uStack_78;
  ulonglong uStack_70;
  undefined1 auStack_68 [4];
  undefined1 auStack_64 [8];
  undefined1 auStack_5c [28];
  
  lVar20 = DAT_143aa84a0;
  lVar4 = *(longlong *)(param_1 + 0x29d0);
  if (lVar4 == 0) {
    return;
  }
  FUN_142517420();
  uVar14 = FUN_141892a90();
  iVar15 = FUN_142cdfbd0(lVar20);
  if (((iVar15 == 0) || (*(char *)(lVar20 + 0x2c34) == '\0')) ||
     (cVar12 = FUN_141b1f960(DAT_143abea80,1), cVar12 != '\0')) {
    bVar8 = false;
  }
  else {
    bVar8 = true;
  }
  iVar15 = FUN_142cc1cf0(lVar20);
  lVar20 = FUN_141892840();
  if (lVar20 != 0) {
    uVar21 = FUN_141892840();
    iVar16 = FUN_14182e980(uVar21);
    if (iVar16 != 0) {
      bVar9 = true;
      goto LAB_14251508a;
    }
  }
  bVar9 = false;
LAB_14251508a:
  FUN_14031e5d0(uVar14);
  uVar21 = FUN_141892840();
  cVar12 = FUN_14031f0e0(uVar21);
  uVar27 = 0;
  uStack_98 = 0;
  uVar32 = uVar27;
  while( true ) {
    uVar18 = (uint)uVar27;
    iStack_a0 = 0;
    lVar20 = *(longlong *)(lVar4 + 0x70);
    iStack_a4 = 0;
    uStack_70 = uVar32;
    if ((lVar20 == 0) || (*(uint *)(lVar20 + -8) <= uVar18)) break;
    if ((int)uVar18 < 0) {
      FUN_142e54290(0xbc,uVar27);
      lVar20 = *(longlong *)(lVar4 + 0x70);
    }
    lVar20 = uVar32 * 0x90 + lVar20;
    if ((((*(char *)(lVar4 + 0x7c) == '\0') || (!bVar8)) || (iVar15 != 0)) || (bVar9)) {
      bVar33 = false;
    }
    else {
      bVar33 = true;
    }
    if (cVar12 != '\0') {
      bVar33 = false;
    }
    FUN_14250c500(lVar20);
    lVar29 = *(longlong *)(lVar20 + 0x18);
    if ((lVar29 != 0) && (*(uint *)(lVar29 + -8) != 0)) {
      piVar30 = (int *)(lVar29 + (ulonglong)*(uint *)(lVar29 + -8) * 4 + -4);
      while (piVar30 != (int *)0x0) {
        piVar31 = (int *)0x0;
        if (*(int **)(lVar20 + 0x18) < piVar30) {
          piVar31 = piVar30 + -1;
        }
        iVar16 = *piVar30;
        lVar29 = (longlong)iVar16;
        switch(iVar16) {
        case 6:
        case 8:
          if (DAT_143aa84a0 == 0) {
LAB_1425151c8:
            uVar18 = 0;
          }
          else {
            iVar17 = FUN_142cf38c0();
            uVar18 = (uint)(iVar17 == 1);
          }
          break;
        case 7:
          iVar17 = FUN_142cf38c0(DAT_143aa84a0);
          uVar18 = (uint)(iVar17 == 2);
          break;
        case 9:
          FUN_142cdec80(DAT_143aa84a0,auStack_68);
          uStack_78 = FUN_1408f6690();
          iVar17 = (*DAT_143ad5648)(auStack_64,&uStack_78);
          if ((-1 < iVar17) || (iVar17 = (*DAT_143ad5648)(auStack_5c,&uStack_78), iVar17 < 1))
          goto LAB_1425151c8;
          uVar18 = 1;
          break;
        case 10:
          uVar18 = (uint)(DAT_143aa84a0 != 0);
          break;
        default:
          if ((0xe < iVar16) && (iVar16 < *(int *)(lVar4 + 0x78))) {
            lVar28 = (lVar29 + -0xf) * 0x38;
            lVar22 = *(longlong *)(lVar4 + 0x40);
            puVar25 = *(undefined4 **)(lVar22 + lVar28);
            if ((*(longlong *)(lVar22 + 8 + lVar28) - (longlong)puVar25 & 0xfffffffffffffffcU) == 4)
            {
              if (*(int *)(lVar22 + 0x28 + lVar28) == 0) {
                iVar17 = FUN_140715c10(DAT_143aa9d98,*puVar25);
                if (iVar17 == 0) goto LAB_14251532e;
                puVar25 = *(undefined4 **)(lVar28 + *(longlong *)(lVar4 + 0x40));
              }
              uVar18 = FUN_1425161d0(lVar4,*puVar25);
              if (uVar18 != 0xffffffff) break;
            }
          }
LAB_14251532e:
          uVar18 = *(uint *)(*(longlong *)(lVar4 + 0x58) + 8 + lVar29 * 0x10);
          break;
        case 0xd:
          lVar22 = FUN_141892840();
          if (lVar22 == 0) {
            uVar18 = 0;
          }
          else {
            uVar21 = FUN_141892840();
            uVar18 = FUN_141829f70(uVar21);
            uVar18 = uVar18 / 10000;
            if (((uVar18 == 0x5e24) || (uVar18 == 0x5e25)) || (uVar18 == 0x5e26)) {
              if (DAT_143ad1850 == 0) {
                uVar18 = 0;
              }
              else if (*(int *)(DAT_143ad1850 + 0x50) - 1U < 2) {
                cVar13 = FUN_141a41da0();
                uVar18 = (uint)(cVar13 != '\0');
              }
              else {
                uVar18 = 0;
              }
            }
            else {
              uVar18 = 0;
            }
          }
        }
        uVar5 = 0;
        if (bVar33) {
          uVar5 = uVar18;
        }
        FUN_14250a280(*(longlong *)(lVar4 + 0x28) + lVar29 * 0x58);
        piVar30 = *(int **)(lVar4 + 0x28);
        uVar18 = piVar30[lVar29 * 0x16 + 1];
        bVar7 = false;
        if (uVar5 != uVar18) {
          piVar30[lVar29 * 0x16 + 5] = 1;
          piVar30[lVar29 * 0x16 + 1] = uVar5;
          piVar30 = *(int **)(lVar4 + 0x28);
        }
        if ((int)uVar5 < 1) {
          bVar6 = bVar7;
          if (piVar30[lVar29 * 0x16] != 0) {
            piVar30[lVar29 * 0x16 + 4] = 1;
            piVar30[lVar29 * 0x16] = 0;
            iVar16 = FUN_1429e3ef0();
            piVar30[lVar29 * 0x16 + 2] = iVar16;
            goto LAB_142515414;
          }
        }
        else {
          bVar6 = false;
          if (uVar5 != uVar18) {
            if (iVar16 == 0) {
              bVar6 = bVar7;
              if (*piVar30 != 1) {
                piVar30[4] = 1;
                *piVar30 = 1;
                iVar16 = FUN_1429e3ef0();
                piVar30[2] = iVar16 + 10000;
LAB_142515414:
                bVar6 = true;
              }
            }
            else {
              iVar16 = piVar30[lVar29 * 0x16];
              bVar6 = bVar7;
              if (iVar16 != 2) {
                piVar30[lVar29 * 0x16 + 4] = 1;
                piVar30[lVar29 * 0x16] = (iVar16 != 0) + 1;
                iVar16 = FUN_1429e3ef0();
                piVar30[lVar29 * 0x16 + 2] = iVar16 + 10000;
                goto LAB_142515414;
              }
            }
          }
        }
        if ((*(int *)(lVar29 * 0x58 + *(longlong *)(lVar4 + 0x28)) != 0) &&
           (iStack_a0 = iStack_a0 + 1, bVar6)) {
          iStack_a4 = iStack_a4 + 1;
        }
        FUN_142515860(lVar4);
        piVar30 = piVar31;
        uVar32 = uStack_70;
        uVar18 = uStack_98;
      }
    }
    if (!bVar33) goto LAB_142515708;
    if (uVar18 == 0) {
      iVar16 = *(int *)(*(longlong *)(lVar4 + 0x28) + 0x4d4);
LAB_1425156b7:
      iVar17 = *(int *)(lVar20 + 0x2c);
      iVar19 = iVar17;
      if (iVar16 != iVar17) {
        *(undefined4 *)(lVar20 + 0x3c) = 1;
        *(int *)(lVar20 + 0x2c) = iVar16;
        iVar19 = iVar16;
      }
      if (iVar19 == 0) {
LAB_1425154b2:
        iVar16 = *(int *)(lVar20 + 0x28);
        goto LAB_1425154b7;
      }
      if (iVar17 < iVar16) {
LAB_1425156db:
        iVar16 = *(int *)(lVar20 + 0x28);
LAB_1425156df:
        if (iVar16 == 2) goto LAB_142515708;
LAB_1425156e4:
        *(undefined4 *)(lVar20 + 0x38) = 1;
        *(uint *)(lVar20 + 0x28) = (iVar16 != 0) + 1;
        iVar16 = FUN_1429e3ef0();
        iVar16 = iVar16 + 10000;
        goto LAB_142515704;
      }
    }
    else {
      if (uVar18 == 1) {
        iVar16 = *(int *)(*(longlong *)(lVar4 + 0x28) + 0x3cc);
        iVar17 = *(int *)(lVar20 + 0x2c);
        bVar33 = iVar16 != iVar17;
        if (bVar33) {
          *(undefined4 *)(lVar20 + 0x3c) = 1;
          *(int *)(lVar20 + 0x2c) = iVar16;
          iVar17 = iVar16;
        }
        if (iVar17 != 0) {
          if (!bVar33) goto LAB_142515708;
          goto LAB_1425156db;
        }
        goto LAB_1425154b2;
      }
      if (uVar18 == 2) {
        iVar16 = *(int *)(*(longlong *)(lVar4 + 0x28) + 0x47c);
        goto LAB_1425156b7;
      }
      if (uVar18 != 3) {
        if (iStack_a0 != *(int *)(lVar20 + 0x2c)) {
          *(undefined4 *)(lVar20 + 0x3c) = 1;
          *(int *)(lVar20 + 0x2c) = iStack_a0;
        }
        if (iStack_a0 == 0) goto LAB_1425154b2;
        if (iStack_a4 != 0) goto LAB_1425156db;
        if (*(int *)(lVar20 + 0x28) != 0) goto LAB_142515708;
        iVar16 = *(int *)(lVar20 + 0x28);
        goto LAB_1425156e4;
      }
      iVar16 = *(int *)(*(longlong *)(lVar4 + 0x28) + 0xb4);
      if (iVar16 != *(int *)(lVar20 + 0x2c)) {
        *(undefined4 *)(lVar20 + 0x3c) = 1;
        *(int *)(lVar20 + 0x2c) = iVar16;
      }
      FUN_14251e150(aplStack_88,*(longlong *)(lVar4 + 0x58) + 0x20);
      plVar23 = (longlong *)*aplStack_88[0];
      if (*(char *)((longlong)plVar23 + 0x19) == '\0') {
        puVar1 = (undefined8 *)(*(undefined8 **)(lVar4 + 200))[1];
LAB_142515540:
        puVar10 = *(undefined8 **)(lVar4 + 200);
        puVar26 = puVar1;
        cVar13 = *(char *)((longlong)puVar1 + 0x19);
        while (cVar13 == '\0') {
          if (*(int *)((longlong)puVar26 + 0x1c) < *(int *)((longlong)plVar23 + 0x1c)) {
            puVar24 = (undefined8 *)puVar26[2];
            puVar26 = puVar10;
          }
          else {
            puVar24 = (undefined8 *)*puVar26;
          }
          puVar10 = puVar26;
          puVar26 = puVar24;
          cVar13 = *(char *)((longlong)puVar24 + 0x19);
        }
        if ((*(char *)((longlong)puVar10 + 0x19) == '\0') &&
           (*(int *)((longlong)puVar10 + 0x1c) <= *(int *)((longlong)plVar23 + 0x1c)))
        goto code_r0x00014251557c;
        cVar13 = *(char *)(aplStack_88[0][1] + 0x19);
        plVar23 = (longlong *)aplStack_88[0][1];
        while (cVar13 == '\0') {
          FUN_1401ba2c0(aplStack_88,aplStack_88,plVar23[2]);
          plVar2 = (longlong *)*plVar23;
          thunk_FUN_140205820(plVar23,0x20);
          plVar23 = plVar2;
          cVar13 = *(char *)((longlong)plVar2 + 0x19);
        }
        bVar33 = true;
        goto LAB_142515620;
      }
LAB_1425155d9:
      cVar13 = *(char *)(aplStack_88[0][1] + 0x19);
      plVar23 = (longlong *)aplStack_88[0][1];
      while (cVar13 == '\0') {
        FUN_1401ba2c0(aplStack_88,aplStack_88,plVar23[2]);
        plVar2 = (longlong *)*plVar23;
        thunk_FUN_140205820(plVar23,0x20);
        plVar23 = plVar2;
        cVar13 = *(char *)((longlong)plVar2 + 0x19);
      }
      bVar33 = false;
LAB_142515620:
      thunk_FUN_140205820(aplStack_88[0]);
      iVar16 = *(int *)(lVar20 + 0x28);
      if (*(int *)(lVar20 + 0x2c) != 0) {
        if (bVar33) goto LAB_1425156df;
        if (iVar16 == 3) goto LAB_142515708;
        *(undefined4 *)(lVar20 + 0x38) = 1;
        uVar14 = 1;
        if (iVar16 != 0) {
          uVar14 = 3;
        }
        *(undefined4 *)(lVar20 + 0x28) = uVar14;
        iVar16 = FUN_1429e3ef0();
        goto LAB_142515704;
      }
LAB_1425154b7:
      if (iVar16 != 0) {
        *(undefined4 *)(lVar20 + 0x38) = 1;
        *(undefined4 *)(lVar20 + 0x28) = 0;
        iVar16 = FUN_1429e3ef0();
LAB_142515704:
        *(int *)(lVar20 + 0x30) = iVar16;
      }
    }
LAB_142515708:
    uStack_98 = uVar18 + 1;
    uVar27 = (ulonglong)uStack_98;
    uVar32 = uVar32 + 1;
  }
  if ((*(longlong **)(lVar4 + 0x58))[1] == 0) {
    *(undefined4 *)(lVar4 + 0x20) = 0;
  }
  else {
    piVar30 = &DAT_14346b2b0;
    puVar1 = (undefined8 *)**(longlong **)(lVar4 + 0x58);
    do {
      iVar15 = *piVar30;
      puVar10 = puVar1;
      puVar26 = (undefined8 *)puVar1[1];
      cVar12 = *(char *)((longlong)puVar1[1] + 0x19);
      while (cVar12 == '\0') {
        if (*(int *)((longlong)puVar26 + 0x1c) < iVar15) {
          puVar24 = (undefined8 *)puVar26[2];
          puVar26 = puVar10;
        }
        else {
          puVar24 = (undefined8 *)*puVar26;
        }
        puVar10 = puVar26;
        puVar26 = puVar24;
        cVar12 = *(char *)((longlong)puVar24 + 0x19);
      }
      if (((*(char *)((longlong)puVar10 + 0x19) == '\0') &&
          (*(int *)((longlong)puVar10 + 0x1c) <= iVar15)) && (puVar10 != puVar1)) {
        *(int *)(lVar4 + 0x20) = iVar15;
        break;
      }
      piVar30 = piVar30 + 1;
    } while (piVar30 != (int *)&DAT_14346b31c);
  }
  pIVar3 = *(IUnknown **)(*(longlong *)(lVar4 + 0x28) + 0x448);
  if ((pIVar3 != (IUnknown *)0x0) &&
     (iVar15 = (**(code **)(*(longlong *)pIVar3 + 0x2b8))(pIVar3,0), iVar15 < 0)) {
    _com_issue_errorex(iVar15,pIVar3,(_GUID *)&DAT_14327fcb0);
  }
  return;
code_r0x00014251557c:
  plVar2 = (longlong *)plVar23[2];
  if (*(char *)((longlong)plVar2 + 0x19) == '\0') {
    cVar13 = *(char *)(*plVar2 + 0x19);
    plVar23 = plVar2;
    plVar2 = (longlong *)*plVar2;
    while (cVar13 == '\0') {
      cVar13 = *(char *)(*plVar2 + 0x19);
      plVar23 = plVar2;
      plVar2 = (longlong *)*plVar2;
    }
  }
  else {
    cVar13 = *(char *)(plVar23[1] + 0x19);
    plVar11 = (longlong *)plVar23[1];
    plVar2 = plVar23;
    while ((plVar23 = plVar11, cVar13 == '\0' && (plVar2 == (longlong *)plVar23[2]))) {
      cVar13 = *(char *)(plVar23[1] + 0x19);
      plVar11 = (longlong *)plVar23[1];
      plVar2 = plVar23;
    }
  }
  if (*(char *)((longlong)plVar23 + 0x19) != '\0') goto LAB_1425155d9;
  goto LAB_142515540;
}



//===========================================================
// FUN_142d5a8b0 @ 142d5a8b0   (284 bytes)
//===========================================================

void FUN_142d5a8b0(longlong param_1,int param_2,int param_3)

{
  char cVar1;
  int iVar2;
  
  if (((DAT_143ad7628 != (longlong *)0x0) && (iVar2 = FUN_14225f7c0(), iVar2 != 0)) &&
     (param_3 == 2)) {
    (**(code **)(*DAT_143ad7628 + 0x138))(DAT_143ad7628,1);
  }
  if (DAT_143ad8360 != 0) {
    switch(param_2) {
    case 0x3274:
    case 0x3276:
    case 0x3278:
    case 0x327a:
    case 0x327c:
    case 0x327e:
    case 0x3280:
      FUN_1424003a0();
    }
  }
  if (((DAT_143ad2010 != 0) && (cVar1 = FUN_14043e3e0(DAT_143ad2010,param_2), cVar1 != '\0')) &&
     (DAT_143ad8420 != 0)) {
    FUN_14242b660();
  }
  if (param_2 == 0x3851) {
    if (param_3 == 1) {
      FUN_142cb1020(param_1,0xd3,0xffffffff,0,0);
    }
    else {
      FUN_142cb4530(param_1,0xd3,0);
    }
  }
  if ((DAT_143adb188 != 0) && (cVar1 = FUN_1408efec0(param_2), cVar1 != '\0')) {
    FUN_14260cb40(DAT_143adb188);
  }
  if (*(int *)(param_1 + 0x348c) != 0) {
    *(undefined4 *)(param_1 + 0x348c) = 0;
    *(undefined1 *)(param_1 + 0x3490) = 0;
  }
  return;
}



//===========================================================
// FUN_142dc5b20 @ 142dc5b20   (761 bytes)
//===========================================================

undefined8 FUN_142dc5b20(int param_1)

{
  longlong *plVar1;
  char cVar2;
  int iVar3;
  undefined4 uVar4;
  longlong lVar5;
  bool bVar6;
  undefined8 *puVar7;
  bool bVar8;
  undefined1 local_18 [8];
  longlong local_10;
  
  bVar6 = false;
  if (param_1 - 0x3a52U < 10) {
    return 1;
  }
  if ((DAT_143ace690 != 0) && (iVar3 = FUN_1424fdea0(DAT_143ace690,param_1), iVar3 != 0)) {
    FUN_1424fdab0(DAT_143ace690);
    FUN_1424fdee0(DAT_143ace690,0);
    return 1;
  }
  if ((DAT_143abea88 != 0) && (cVar2 = FUN_140310780(DAT_143abea88,param_1), cVar2 != '\0')) {
    if (DAT_143acf410 == 0) {
      return 1;
    }
    FUN_14271fb70(DAT_143acf410,5);
    return 1;
  }
  if (DAT_143ad80d8 != 0) {
    FUN_14236e7c0(DAT_143ad80d8,param_1);
  }
  if ((DAT_143acd7c0 != 0) && (iVar3 = FUN_1414801d0(DAT_143acd7c0,param_1), iVar3 != 0)) {
    return 0;
  }
  if ((DAT_143ad3bb0 != 0) && (param_1 == 0x187cd)) {
    FUN_141fca480(DAT_143ad3bb0,0);
    return 1;
  }
  if (param_1 < 0x18ad9) {
    if (param_1 == 0x18ad8) {
LAB_142dc5cc7:
      FUN_141157a50();
      return 1;
    }
    if (0x988b < param_1) {
      if (param_1 == 0x988c) {
        return 1;
      }
      if (param_1 == 0x18769) {
        return 1;
      }
      bVar8 = param_1 == 0x1896c;
      goto LAB_142dc5ce3;
    }
    if (param_1 == 0x988b) {
      return 1;
    }
    if (param_1 == 0x285f) {
      return 1;
    }
    if (param_1 == 0x4102) {
      return 1;
    }
    if (param_1 == 0x411c) {
      if (DAT_143ace020 != 0) {
        FUN_142368510(DAT_143ace020,0x411c);
      }
      if (DAT_143ace028 == 0) {
        return 1;
      }
      FUN_14236b900(DAT_143ace028,0x411c);
      return 1;
    }
  }
  else {
    if (param_1 < 0x7a688) {
      if (param_1 == 0x7a687) {
        return 1;
      }
      if ((param_1 == 0x18ad9) || (param_1 == 0x18ada)) goto LAB_142dc5cc7;
      bVar8 = param_1 == 0x7a414;
    }
    else {
      if (param_1 == 0x7a714) {
        return 1;
      }
      bVar8 = param_1 == 0x7a7b8;
    }
LAB_142dc5ce3:
    if (bVar8) {
      return 1;
    }
  }
  if (DAT_143aa9d98 != 0) {
    lVar5 = FUN_142dc0900(DAT_143aa9d98,local_18,param_1);
    bVar6 = true;
    if (*(longlong *)(lVar5 + 8) != 0) {
      bVar8 = true;
      goto LAB_142dc5d16;
    }
  }
  bVar8 = false;
LAB_142dc5d16:
  lVar5 = local_10;
  if ((bVar6) && (local_10 != 0)) {
    puVar7 = (undefined8 *)(local_10 + -0x28);
    if (0xffffe < *(longlong *)(local_10 + -0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar5 + -0x20);
    lVar5 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar5 == 1) {
      if ((local_10 != 0) && (*(longlong *)(local_10 + -0x10) != 0)) {
        LOCK();
        *(undefined8 *)(*(longlong *)(local_10 + -0x10) + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(local_10 + -0x10) + 4) != 0);
      }
      if (puVar7 != (undefined8 *)0x0) {
        (**(code **)*puVar7)(puVar7,1);
      }
    }
  }
  if (!bVar8) {
    if ((DAT_143aa8518 != 0) &&
       (iVar3 = (**(code **)(*(longlong *)(DAT_143aa8518 + 0x100) + 0x20))(), iVar3 != 0)) {
      uVar4 = (**(code **)(*(longlong *)(DAT_143aa8518 + 0x100) + 0x20))();
      lVar5 = FUN_14039b100(DAT_143aa8328,uVar4);
      if ((lVar5 != 0) && (param_1 == *(int *)(lVar5 + 0x1c0))) {
        return 1;
      }
    }
    if ((DAT_143ad2020 == 0) || (cVar2 = FUN_140450470(DAT_143ad2020,param_1), cVar2 == '\0')) {
      return 0;
    }
  }
  return 1;
}


