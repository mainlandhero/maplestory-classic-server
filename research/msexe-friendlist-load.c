
//===========================================================
// FUN_142deb010 @ 142deb010   (711 bytes)
//===========================================================

void FUN_142deb010(longlong *param_1,undefined8 param_2)

{
  undefined1 auVar1 [16];
  uint uVar2;
  longlong lVar3;
  int iVar4;
  uint uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  undefined4 uVar9;
  ulonglong uVar10;
  longlong lVar11;
  
  uVar2 = FUN_1406e8c20();
  lVar3 = *param_1;
  uVar8 = (ulonglong)uVar2;
  if (lVar3 == 0) {
    uVar10 = 0;
  }
  else {
    uVar10 = (ulonglong)*(uint *)(lVar3 + -8);
  }
  if ((uint)uVar10 < uVar2) {
    if (lVar3 == 0) {
      uVar5 = 0;
    }
    else {
      uVar7 = *(ulonglong *)(lVar3 + -0x10);
      uVar6 = ~uVar7;
      if (-1 < (longlong)uVar7) {
        uVar6 = uVar7;
      }
      auVar1._8_8_ = 0;
      auVar1._0_8_ = uVar6 - 8;
      lVar11 = SUB168(ZEXT816(0x8e6527af1373f071) * auVar1,8);
      uVar5 = (uint)(((uVar6 - 8) - lVar11 >> 1) + lVar11 >> 8);
    }
    lVar11 = uVar8 * 0x149;
    if (uVar5 < uVar2) {
      lVar3 = FUN_14019b780(&DAT_143ad68a0,lVar11 + 8);
      if (lVar3 == 0) {
        lVar3 = 0;
      }
      else {
        lVar3 = lVar3 + 8;
      }
      FUN_142df3110(lVar3,lVar11 + lVar3);
      if (*param_1 != 0) {
        thunk_FUN_140205820(*param_1 + -8,0);
      }
      *param_1 = lVar3;
    }
    else {
      FUN_142df3110(uVar10 * 0x149 + lVar3,lVar11 + lVar3);
      lVar3 = *param_1;
    }
  }
  if (lVar3 != 0) {
    *(ulonglong *)(lVar3 + -8) = uVar8;
  }
  lVar3 = param_1[1];
  if (lVar3 == 0) {
    uVar5 = 0;
  }
  else {
    uVar5 = *(uint *)(lVar3 + -8);
  }
  if (uVar5 < uVar2) {
    if (lVar3 == 0) {
      uVar5 = 0;
    }
    else {
      uVar10 = *(ulonglong *)(lVar3 + -0x10);
      uVar7 = ~uVar10;
      if (-1 < (longlong)uVar10) {
        uVar7 = uVar10;
      }
      uVar5 = (uint)(uVar7 - 8 >> 2);
    }
    if (uVar5 < uVar2) {
      lVar3 = FUN_14019b780(&DAT_143ad68a0,uVar8 * 4 + 8);
      if (lVar3 == 0) {
        lVar3 = 0;
      }
      else {
        lVar3 = lVar3 + 8;
      }
      if (param_1[1] != 0) {
        thunk_FUN_140205820(param_1[1] + -8,0);
      }
      param_1[1] = lVar3;
    }
  }
  if (lVar3 != 0) {
    *(ulonglong *)(lVar3 + -8) = uVar8;
  }
  lVar3 = param_1[2];
  if (lVar3 == 0) {
    uVar5 = 0;
  }
  else {
    uVar5 = *(uint *)(lVar3 + -8);
  }
  if (uVar5 < uVar2) {
    if (lVar3 == 0) {
      uVar5 = 0;
    }
    else {
      uVar10 = *(ulonglong *)(lVar3 + -0x10);
      uVar7 = ~uVar10;
      if (-1 < (longlong)uVar10) {
        uVar7 = uVar10;
      }
      uVar5 = (uint)(uVar7 - 8 >> 2);
    }
    if (uVar5 < uVar2) {
      lVar3 = FUN_14019b780(&DAT_143ad68a0,uVar8 * 4 + 8);
      if (lVar3 == 0) {
        lVar3 = 0;
      }
      else {
        lVar3 = lVar3 + 8;
      }
      if (param_1[2] != 0) {
        thunk_FUN_140205820(param_1[2] + -8,0);
      }
      param_1[2] = lVar3;
    }
  }
  if (lVar3 != 0) {
    *(ulonglong *)(lVar3 + -8) = uVar8;
  }
  if ((int)uVar2 < 1) {
    return;
  }
  lVar3 = *param_1;
  if (lVar3 == 0) {
    FUN_142e52d50(0xd0,1);
    lVar3 = *param_1;
    if (lVar3 != 0) goto LAB_142deb222;
  }
  else {
LAB_142deb222:
    if (*(int *)(lVar3 + -8) != 0) goto LAB_142deb23a;
  }
  FUN_142e54290(0xbc,0,0);
  lVar3 = *param_1;
LAB_142deb23a:
  FUN_1406e9170(param_2,lVar3,uVar2 * 0x149);
  uVar2 = 0;
  while( true ) {
    FUN_142deaf60();
    lVar3 = *param_1;
    if (lVar3 == 0) {
      iVar4 = 0;
    }
    else {
      iVar4 = *(int *)(lVar3 + -8);
    }
    if (iVar4 <= (int)uVar2) break;
    if (lVar3 == 0) {
      uVar5 = 0;
    }
    else {
      uVar5 = *(uint *)(lVar3 + -8);
    }
    if (((int)uVar2 < 0) || (uVar5 <= uVar2)) {
      if (lVar3 == 0) {
        uVar9 = 0;
      }
      else {
        uVar9 = *(undefined4 *)(lVar3 + -8);
      }
      FUN_142e54290(0xbc,uVar2,uVar9);
    }
    FUN_142deb770(param_1,uVar2);
    uVar2 = uVar2 + 1;
  }
  FUN_142debd10(param_1);
  return;
}



//===========================================================
// FUN_142deb620 @ 142deb620   (322 bytes)
//===========================================================

void FUN_142deb620(longlong *param_1,undefined8 param_2,int param_3)

{
  char cVar1;
  int iVar2;
  undefined4 uVar3;
  uint uVar4;
  int iVar5;
  longlong lVar6;
  uint uVar7;
  uint uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  
  iVar2 = FUN_1406e8c20(param_2);
  uVar3 = FUN_1406e8c20(param_2);
  cVar1 = FUN_1406e8ae0(param_2);
  uVar11 = 0;
  if (cVar1 == '\0') {
    iVar5 = FUN_142deaf60(param_1);
    uVar9 = uVar11;
    uVar10 = uVar11;
    if (iVar5 != 0) {
      return;
    }
    do {
      uVar4 = (uint)uVar9;
      FUN_142deaf60(param_1);
      lVar6 = *param_1;
      uVar7 = 0;
      uVar8 = uVar7;
      if (lVar6 != 0) {
        uVar8 = *(uint *)(lVar6 + -8);
      }
      if ((int)uVar8 <= (int)uVar4) {
        return;
      }
      if (iVar2 != 0) {
        if (lVar6 != 0) {
          uVar7 = *(uint *)(lVar6 + -8);
        }
        if (((int)uVar4 < 0) || (uVar7 <= uVar4)) {
          uVar12 = uVar11;
          if (lVar6 != 0) {
            uVar12 = (ulonglong)*(uint *)(lVar6 + -8);
          }
          FUN_142e54290(0xbc,uVar9,uVar12);
          lVar6 = *param_1;
        }
        if (*(int *)(lVar6 + uVar10) == iVar2) goto LAB_142deb67f;
      }
      uVar9 = (ulonglong)(uVar4 + 1);
      uVar10 = uVar10 + 0x149;
    } while( true );
  }
  uVar4 = FUN_142debab0(param_1,0,uVar3);
LAB_142deb67f:
  if ((int)uVar4 < 0) {
    return;
  }
  lVar6 = *param_1;
  if (lVar6 != 0) {
    if (uVar4 < *(uint *)(lVar6 + -8)) goto LAB_142deb6a6;
    uVar11 = (ulonglong)*(uint *)(lVar6 + -8);
  }
  FUN_142e54290(0xbc,uVar4,uVar11);
  lVar6 = *param_1;
LAB_142deb6a6:
  FUN_1402d4870((longlong)(int)uVar4 * 0x149 + lVar6,param_2);
  if (param_3 != 0) {
    FUN_142deb770(param_1,uVar4,cVar1);
  }
  FUN_1415aafa0(0x13);
  return;
}



//===========================================================
// FUN_142deea90 @ 142deea90   (3733 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142deea90(longlong param_1)

{
  longlong *plVar1;
  char cVar2;
  char cVar3;
  int iVar4;
  undefined8 *puVar5;
  int iVar6;
  uint uVar7;
  undefined4 uVar8;
  longlong *plVar9;
  int *piVar10;
  undefined8 *puVar11;
  int *piVar12;
  int *piVar13;
  undefined8 *puVar14;
  int *piVar15;
  char *pcVar16;
  int *piVar17;
  undefined8 *puVar18;
  longlong *plVar19;
  longlong *plVar20;
  longlong *plVar21;
  int iVar22;
  char *pcVar23;
  ulonglong uVar24;
  longlong lVar25;
  longlong lVar26;
  char *pcVar27;
  longlong lVar28;
  longlong *plVar29;
  uint uVar30;
  longlong *plVar31;
  undefined1 auStack_548 [32];
  int local_528;
  int *local_518;
  uint uStack_510;
  undefined4 uStack_50c;
  longlong *local_4f8;
  longlong *local_4f0;
  longlong *plStack_4e8;
  longlong *local_4e0;
  uint local_4d8;
  longlong *local_4d0;
  undefined8 local_4c8;
  int *local_4b8;
  uint uStack_4b0;
  undefined4 uStack_4ac;
  longlong *local_4a8;
  undefined8 local_4a0;
  longlong *local_498;
  undefined8 local_490;
  undefined1 local_488 [1104];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_548;
  plVar21 = *(longlong **)(param_1 + 0x23c8);
  if ((plVar21 != (longlong *)0x0) && (0 < (int)plVar21[4])) {
    plVar1 = plVar21 + 3;
    plVar9 = plVar21;
    if (plVar21 == (longlong *)0x0) {
      local_4f8 = plVar21;
      FUN_142e52ed0(0x431,0);
      plVar9 = *(longlong **)(param_1 + 0x23c8);
    }
    local_4f0 = (longlong *)0x0;
    plStack_4e8 = (longlong *)0x0;
    local_4e0 = (longlong *)0x0;
    local_4d8 = *(uint *)(plVar21 + 4);
    plVar19 = *(longlong **)*plVar1;
    plVar20 = (longlong *)0x0;
    cVar2 = *(char *)((longlong)plVar19 + 0x19);
    iVar6 = local_4d8 - 1;
    local_4f8 = plVar9;
    plVar31 = local_4f0;
    pcVar23 = DAT_143aa8360;
    while (local_528 = iVar6, local_4f0 = plVar31, cVar2 == '\0') {
      pcVar27 = pcVar23;
      if (PTR_s_Default_Group_143a45268 != (char *)0x0) {
        pcVar27 = PTR_s_Default_Group_143a45268;
      }
      pcVar16 = pcVar23;
      if ((char *)plVar19[5] != (char *)0x0) {
        pcVar16 = (char *)plVar19[5];
      }
      lVar25 = (longlong)pcVar27 - (longlong)pcVar16;
      do {
        cVar2 = *pcVar16;
        cVar3 = pcVar16[lVar25];
        if (cVar2 != cVar3) break;
        pcVar16 = pcVar16 + 1;
      } while (cVar3 != '\0');
      if (cVar2 != cVar3) {
        uVar30 = 0;
        iVar6 = FUN_142df2e10(plVar9);
        plVar31 = local_4f8;
        if (iVar6 != 0) {
          lVar25 = *local_4f8;
          do {
            if (lVar25 == 0) {
              uVar7 = 0;
            }
            else {
              uVar7 = *(uint *)(lVar25 + -8);
            }
            if (((int)uVar30 < 0) || (uVar7 <= uVar30)) {
              if (lVar25 == 0) {
                uVar8 = 0;
              }
              else {
                uVar8 = *(undefined4 *)(lVar25 + -8);
              }
              FUN_142e54290(0xbc,uVar30,uVar8);
              lVar25 = *plVar31;
              pcVar23 = DAT_143aa8360;
            }
            pcVar16 = (char *)((longlong)(int)uVar30 * 0x149 + 0x16 + lVar25);
            pcVar27 = pcVar23;
            if (pcVar16 != (char *)0x0) {
              pcVar27 = pcVar16;
            }
            pcVar16 = pcVar23;
            if ((char *)plVar19[5] != (char *)0x0) {
              pcVar16 = (char *)plVar19[5];
            }
            lVar26 = (longlong)pcVar27 - (longlong)pcVar16;
            do {
              cVar2 = *pcVar16;
              cVar3 = pcVar16[lVar26];
              if (cVar2 != cVar3) break;
              pcVar16 = pcVar16 + 1;
            } while (cVar3 != '\0');
            plVar9 = local_4f8;
            plVar20 = plStack_4e8;
            if (cVar2 == cVar3) goto LAB_142deecab;
            uVar30 = uVar30 + 1;
            uVar7 = FUN_142df2e10(plVar31);
            plVar9 = local_4f8;
            plVar20 = plStack_4e8;
          } while (uVar30 < uVar7);
        }
        if (plVar20 == local_4e0) {
          FUN_1401ba060(&local_4f0,plVar20,plVar19 + 5);
          pcVar23 = DAT_143aa8360;
          plVar20 = plStack_4e8;
        }
        else {
          *plVar20 = 0;
          FUN_14019a260(plVar20,plVar19 + 5);
          plStack_4e8 = plStack_4e8 + 1;
          pcVar23 = DAT_143aa8360;
          plVar20 = plStack_4e8;
        }
      }
LAB_142deecab:
      plVar31 = (longlong *)plVar19[2];
      if (*(char *)((longlong)plVar31 + 0x19) == '\0') {
        cVar2 = *(char *)(*plVar31 + 0x19);
        plVar19 = plVar31;
        plVar31 = (longlong *)*plVar31;
        while (cVar2 == '\0') {
          cVar2 = *(char *)(*plVar31 + 0x19);
          plVar19 = plVar31;
          plVar31 = (longlong *)*plVar31;
        }
      }
      else {
        cVar2 = *(char *)(plVar19[1] + 0x19);
        plVar29 = (longlong *)plVar19[1];
        plVar31 = plVar19;
        while ((plVar19 = plVar29, cVar2 == '\0' && (plVar31 == (longlong *)plVar19[2]))) {
          cVar2 = *(char *)(plVar19[1] + 0x19);
          plVar29 = (longlong *)plVar19[1];
          plVar31 = plVar19;
        }
      }
      iVar6 = local_528;
      plVar31 = local_4f0;
      cVar2 = *(char *)((longlong)plVar19 + 0x19);
    }
    plVar9 = local_4f8;
    plVar19 = plVar31;
    plVar29 = plVar20;
    if (plVar31 != plVar20) {
LAB_142deed20:
      plVar9 = *(longlong **)*plVar1;
      if (*(char *)((longlong)plVar9 + 0x19) == '\0') {
        do {
          pcVar27 = pcVar23;
          if ((char *)*plVar31 != (char *)0x0) {
            pcVar27 = (char *)*plVar31;
          }
          pcVar16 = pcVar23;
          if ((char *)plVar9[5] != (char *)0x0) {
            pcVar16 = (char *)plVar9[5];
          }
          lVar25 = (longlong)pcVar27 - (longlong)pcVar16;
          do {
            cVar2 = *pcVar16;
            cVar3 = pcVar16[lVar25];
            if (cVar2 != cVar3) break;
            pcVar16 = pcVar16 + 1;
          } while (cVar3 != '\0');
          if (cVar2 == cVar3) {
            uVar30 = *(uint *)(plVar9 + 4);
            if ((-1 < (int)uVar30) && ((int)uVar30 <= iVar6)) {
              uVar7 = uVar30;
              if ((int)uVar30 < iVar6) goto LAB_142deee00;
              goto LAB_142deefe4;
            }
            break;
          }
          plVar19 = (longlong *)plVar9[2];
          if (*(char *)((longlong)plVar19 + 0x19) == '\0') {
            cVar2 = *(char *)(*plVar19 + 0x19);
            plVar9 = plVar19;
            plVar19 = (longlong *)*plVar19;
            while (cVar2 == '\0') {
              cVar2 = *(char *)(*plVar19 + 0x19);
              plVar9 = plVar19;
              plVar19 = (longlong *)*plVar19;
            }
          }
          else {
            cVar2 = *(char *)(plVar9[1] + 0x19);
            plVar29 = (longlong *)plVar9[1];
            plVar19 = plVar9;
            while ((plVar9 = plVar29, cVar2 == '\0' && (plVar19 == (longlong *)plVar9[2]))) {
              cVar2 = *(char *)(plVar9[1] + 0x19);
              plVar29 = (longlong *)plVar9[1];
              plVar19 = plVar9;
            }
          }
        } while (*(char *)((longlong)plVar9 + 0x19) == '\0');
      }
      goto LAB_142def198;
    }
joined_r0x000142def1b2:
    for (; local_4f8 = plVar9, plVar19 != plVar29; plVar19 = plVar19 + 1) {
      if (*plVar19 != 0) {
        FUN_14019f2c0(*plVar19 + -0x10);
      }
      pcVar23 = DAT_143aa8360;
      plVar9 = local_4f8;
    }
    uVar30 = 0;
    plStack_4e8 = local_4f0;
    iVar6 = FUN_142df2e10(plVar9);
    if (iVar6 != 0) {
      do {
        uVar8 = 0;
        lVar25 = *plVar9;
        uVar7 = 0;
        if (lVar25 != 0) {
          uVar7 = *(uint *)(lVar25 + -8);
        }
        if (((int)uVar30 < 0) || (uVar7 <= uVar30)) {
          if (lVar25 != 0) {
            uVar8 = *(undefined4 *)(lVar25 + -8);
          }
          FUN_142e54290(0xbc,uVar30,uVar8);
          lVar25 = *plVar9;
          pcVar23 = DAT_143aa8360;
        }
        lVar26 = (longlong)(int)uVar30 * 0x149;
        if ((undefined *)(lVar26 + 0x16 + lVar25) != PTR_s_Default_Group_143a45268) {
          plVar19 = *(longlong **)*plVar1;
          cVar2 = *(char *)((longlong)plVar19 + 0x19);
          plVar20 = local_4f0;
          plVar31 = plStack_4e8;
          while (local_4f0 = plVar20, plStack_4e8 = plVar31, cVar2 == '\0') {
            if (lVar25 == 0) {
              uVar7 = 0;
            }
            else {
              uVar7 = *(uint *)(lVar25 + -8);
            }
            if (((int)uVar30 < 0) || (uVar7 <= uVar30)) {
              if (lVar25 == 0) {
                uVar8 = 0;
              }
              else {
                uVar8 = *(undefined4 *)(lVar25 + -8);
              }
              FUN_142e54290(0xbc,uVar30,uVar8);
              lVar25 = *plVar9;
              pcVar23 = DAT_143aa8360;
            }
            pcVar16 = (char *)(lVar25 + 0x16 + lVar26);
            pcVar27 = pcVar23;
            if (pcVar16 != (char *)0x0) {
              pcVar27 = pcVar16;
            }
            pcVar16 = pcVar23;
            if ((char *)plVar19[5] != (char *)0x0) {
              pcVar16 = (char *)plVar19[5];
            }
            lVar28 = (longlong)pcVar27 - (longlong)pcVar16;
            do {
              cVar2 = *pcVar16;
              cVar3 = pcVar16[lVar28];
              if (cVar2 != cVar3) break;
              pcVar16 = pcVar16 + 1;
            } while (cVar3 != '\0');
            if (cVar2 == cVar3) goto LAB_142def55e;
            plVar20 = (longlong *)plVar19[2];
            if (*(char *)((longlong)plVar20 + 0x19) == '\0') {
              cVar2 = *(char *)(*plVar20 + 0x19);
              plVar19 = plVar20;
              plVar20 = (longlong *)*plVar20;
              while (cVar2 == '\0') {
                cVar2 = *(char *)(*plVar20 + 0x19);
                plVar19 = plVar20;
                plVar20 = (longlong *)*plVar20;
              }
            }
            else {
              cVar2 = *(char *)(plVar19[1] + 0x19);
              plVar31 = (longlong *)plVar19[1];
              plVar20 = plVar19;
              while ((plVar19 = plVar31, cVar2 == '\0' && (plVar20 == (longlong *)plVar19[2]))) {
                cVar2 = *(char *)(plVar19[1] + 0x19);
                plVar31 = (longlong *)plVar19[1];
                plVar20 = plVar19;
              }
            }
            plVar20 = local_4f0;
            plVar31 = plStack_4e8;
            cVar2 = *(char *)((longlong)plVar19 + 0x19);
          }
          if (plVar20 != plVar31) {
            lVar25 = *plVar9;
            do {
              if (lVar25 == 0) {
                uVar7 = 0;
              }
              else {
                uVar7 = *(uint *)(lVar25 + -8);
              }
              if (((int)uVar30 < 0) || (uVar7 <= uVar30)) {
                uVar8 = 0;
                if (lVar25 != 0) {
                  uVar8 = *(undefined4 *)(lVar25 + -8);
                }
                FUN_142e54290(0xbc,uVar30,uVar8);
                lVar25 = *plVar9;
                pcVar23 = DAT_143aa8360;
              }
              pcVar16 = (char *)(lVar26 + 0x16 + lVar25);
              pcVar27 = pcVar23;
              if (pcVar16 != (char *)0x0) {
                pcVar27 = pcVar16;
              }
              pcVar16 = pcVar23;
              if ((char *)*plVar20 != (char *)0x0) {
                pcVar16 = (char *)*plVar20;
              }
              lVar28 = (longlong)pcVar27 - (longlong)pcVar16;
              do {
                cVar2 = *pcVar16;
                cVar3 = pcVar16[lVar28];
                if (cVar2 != cVar3) break;
                pcVar16 = pcVar16 + 1;
              } while (cVar3 != '\0');
              if (cVar2 == cVar3) goto LAB_142def55e;
              plVar20 = plVar20 + 1;
            } while (plVar20 != plVar31);
          }
          lVar25 = *plVar9;
          if (lVar25 == 0) {
            uVar7 = 0;
          }
          else {
            uVar7 = *(uint *)(lVar25 + -8);
          }
          if (((int)uVar30 < 0) || (uVar7 <= uVar30)) {
            if (lVar25 == 0) {
              uVar8 = 0;
            }
            else {
              uVar8 = *(undefined4 *)(lVar25 + -8);
            }
            FUN_142e54290(0xbc,uVar30,uVar8);
            lVar25 = *plVar9;
          }
          plVar19 = (longlong *)0x0;
          local_4f8 = (longlong *)0x0;
          lVar25 = lVar26 + 0x16 + lVar25;
          if (lVar25 != 0) {
            uVar24 = 0xffffffffffffffff;
            do {
              uVar24 = uVar24 + 1;
            } while (*(char *)(lVar25 + uVar24) != '\0');
            iVar22 = (int)uVar24;
            iVar6 = 0;
            if (0 < iVar22) {
              iVar6 = iVar22;
            }
            piVar13 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
            piVar13[1] = iVar6;
            *piVar13 = -1;
            plVar19 = (longlong *)(piVar13 + 4);
            piVar13[2] = 0;
            *(undefined1 *)plVar19 = 0;
            local_4f8 = plVar19;
            FUN_142ef7ba0(plVar19,lVar25,(longlong)iVar22);
            if (*piVar13 != -1) {
              FUN_142e52dd0();
            }
            if ((iVar22 == -1) || (iVar22 <= piVar13[1])) {
              *piVar13 = 1;
              if (iVar22 != -1) goto LAB_142def4be;
              if (plVar19 == (longlong *)0x0) {
                uVar24 = 0;
              }
              else {
                uVar24 = 0xffffffffffffffff;
                do {
                  uVar24 = uVar24 + 1;
                } while (*(char *)((longlong)plVar19 + uVar24) != '\0');
              }
            }
            else {
              FUN_142e54290(0x90,piVar13[1],uVar24 & 0xffffffff);
              *piVar13 = 1;
LAB_142def4be:
              *(undefined1 *)((longlong)iVar22 + (longlong)plVar19) = 0;
            }
            iVar6 = (int)uVar24;
            if ((iVar6 < 0) || (piVar13[1] + 1 <= iVar6)) {
              FUN_142e54290(0x9c,uVar24 & 0xffffffff);
            }
            piVar13[2] = iVar6;
          }
          if (plStack_4e8 == local_4e0) {
            FUN_1401d63a0(&local_4f0,plStack_4e8);
            plVar19 = local_4f8;
          }
          else {
            *plStack_4e8 = 0;
            *plStack_4e8 = (longlong)plVar19;
            plStack_4e8 = plStack_4e8 + 1;
            plVar19 = (longlong *)0x0;
          }
          pcVar23 = DAT_143aa8360;
          if (plVar19 != (longlong *)0x0) {
            FUN_14019f2c0(plVar19 + -2);
            pcVar23 = DAT_143aa8360;
          }
        }
LAB_142def55e:
        uVar30 = uVar30 + 1;
        uVar7 = FUN_142df2e10(plVar9);
      } while (uVar30 < uVar7);
    }
    plVar19 = plStack_4e8;
    iVar22 = 0;
    iVar6 = local_528;
    for (plVar9 = local_4f0; plVar9 != plVar19; plVar9 = plVar9 + 1) {
      iVar6 = iVar6 + 1;
      piVar13 = (int *)*plVar1;
      piVar12 = *(int **)(piVar13 + 2);
      uStack_510 = 0;
      cVar2 = *(char *)((longlong)piVar12 + 0x19);
      piVar15 = piVar13;
      local_518 = piVar12;
      while (piVar10 = piVar12, cVar2 == '\0') {
        if (iVar6 <= piVar10[8]) {
          piVar12 = *(int **)piVar10;
          piVar15 = piVar10;
        }
        else {
          piVar12 = *(int **)(piVar10 + 4);
        }
        uStack_510 = (uint)(iVar6 <= piVar10[8]);
        cVar2 = *(char *)((longlong)piVar12 + 0x19);
        local_518 = piVar10;
      }
      if ((*(char *)((longlong)piVar15 + 0x19) != '\0') || (iVar6 < piVar15[8])) {
        if (plVar21[4] == 0x555555555555555) {
                    /* WARNING: Subroutine does not return */
          FUN_14019f9d0();
        }
        local_4c8 = 0;
        local_4d0 = plVar1;
        puVar14 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x30);
        *(int *)(puVar14 + 4) = iVar6;
        puVar14[5] = 0;
        *puVar14 = piVar13;
        puVar14[1] = piVar13;
        puVar14[2] = piVar13;
        *(undefined2 *)(puVar14 + 3) = 0;
        local_4c8 = 0;
        piVar15 = (int *)FUN_140301600(plVar1,&local_518,puVar14);
      }
      FUN_14019a260(piVar15 + 10,plVar9);
    }
    FUN_1406ed520(local_488,0x193);
    FUN_1406ed840(local_488,0x14);
    iVar4 = (int)plVar21[4];
    uVar8 = FUN_1406ed640(local_488);
    FUN_1406ed9d0(local_488,0);
    uVar30 = local_4d8;
    if ((int)local_4d8 < iVar4) {
      do {
        puVar14 = (undefined8 *)*plVar1;
        cVar2 = *(char *)((longlong)puVar14[1] + 0x19);
        puVar5 = puVar14;
        puVar18 = (undefined8 *)puVar14[1];
        while (cVar2 == '\0') {
          if (*(int *)(puVar18 + 4) < (int)uVar30) {
            puVar11 = (undefined8 *)puVar18[2];
            puVar18 = puVar5;
          }
          else {
            puVar11 = (undefined8 *)*puVar18;
          }
          puVar5 = puVar18;
          puVar18 = puVar11;
          cVar2 = *(char *)((longlong)puVar11 + 0x19);
        }
        if (((*(char *)((longlong)puVar5 + 0x19) == '\0') && (*(int *)(puVar5 + 4) <= (int)uVar30))
           && (puVar5 != puVar14)) {
          FUN_1406ed840(local_488,uVar30 & 0xff);
          piVar13 = (int *)*plVar1;
          piVar12 = *(int **)(piVar13 + 2);
          uStack_510 = 0;
          cVar2 = *(char *)((longlong)piVar12 + 0x19);
          piVar15 = piVar13;
          local_518 = piVar12;
          while (piVar10 = piVar12, cVar2 == '\0') {
            if ((int)uVar30 <= piVar10[8]) {
              piVar12 = *(int **)piVar10;
              piVar15 = piVar10;
            }
            else {
              piVar12 = *(int **)(piVar10 + 4);
            }
            uStack_510 = (uint)((int)uVar30 <= piVar10[8]);
            cVar2 = *(char *)((longlong)piVar12 + 0x19);
            local_518 = piVar10;
          }
          if ((*(char *)((longlong)piVar15 + 0x19) != '\0') || ((int)uVar30 < piVar15[8])) {
            if (plVar21[4] == 0x555555555555555) {
                    /* WARNING: Subroutine does not return */
              FUN_14019f9d0();
            }
            local_4c8 = 0;
            local_4d0 = plVar1;
            puVar14 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x30);
            *(uint *)(puVar14 + 4) = uVar30;
            puVar14[5] = 0;
            *puVar14 = piVar13;
            puVar14[1] = piVar13;
            puVar14[2] = piVar13;
            *(undefined2 *)(puVar14 + 3) = 0;
            local_4c8 = 0;
            piVar15 = (int *)FUN_140301600(plVar1,&local_518,puVar14);
          }
          FUN_1406edc80(local_488,piVar15 + 10);
          iVar22 = iVar22 + 1;
        }
        uVar30 = uVar30 + 1;
      } while ((int)uVar30 < iVar4);
      if (iVar22 != 0) {
        FUN_1406eda30(local_488,iVar22,uVar8);
        FUN_1415d01c0(local_488);
      }
    }
    while (iVar6 = iVar6 + 1, iVar6 < iVar4) {
      puVar14 = (undefined8 *)*plVar1;
      cVar2 = *(char *)((longlong)puVar14[1] + 0x19);
      puVar5 = puVar14;
      puVar18 = (undefined8 *)puVar14[1];
      while (cVar2 == '\0') {
        if (*(int *)(puVar18 + 4) < iVar6) {
          puVar11 = (undefined8 *)puVar18[2];
          puVar18 = puVar5;
        }
        else {
          puVar11 = (undefined8 *)*puVar18;
        }
        puVar5 = puVar18;
        puVar18 = puVar11;
        cVar2 = *(char *)((longlong)puVar11 + 0x19);
      }
      if (((*(char *)((longlong)puVar5 + 0x19) == '\0') && (*(int *)(puVar5 + 4) <= iVar6)) &&
         (puVar5 != puVar14)) {
        lVar25 = FUN_14287b6e0(plVar1);
        if (*(longlong *)(lVar25 + 0x28) != 0) {
          FUN_14019f2c0(*(longlong *)(lVar25 + 0x28) + -0x10);
        }
        thunk_FUN_140205820(lVar25,0x30);
      }
    }
    uVar8 = FUN_1406ed610(local_488);
    plVar1 = plStack_4e8;
    plVar21 = local_4f0;
    if (local_4f0 != (longlong *)0x0) {
      for (; plVar21 != plVar1; plVar21 = plVar21 + 1) {
        if (*plVar21 != 0) {
          uVar8 = FUN_14019f2c0(*plVar21 + -0x10);
        }
      }
      uVar24 = (longlong)local_4e0 - (longlong)local_4f0 & 0xfffffffffffffff8;
      plVar21 = local_4f0;
      if (0xfff < uVar24) {
        plVar21 = (longlong *)local_4f0[-1];
        if (0x1f < (ulonglong)((longlong)local_4f0 + (-8 - (longlong)plVar21))) {
                    /* WARNING: Subroutine does not return */
          FUN_142f04804(uVar8,uVar24 + 0x27);
        }
      }
      thunk_FUN_140205820(plVar21);
    }
  }
  return;
LAB_142deee00:
  do {
    piVar13 = (int *)*plVar1;
    piVar12 = *(int **)(piVar13 + 2);
    cVar3 = *(char *)((longlong)piVar12 + 0x19);
    piVar15 = piVar13;
    piVar10 = piVar12;
    cVar2 = cVar3;
    while (cVar2 == '\0') {
      if (piVar10[8] < (int)(uVar7 + 1)) {
        piVar17 = *(int **)(piVar10 + 4);
        piVar10 = piVar15;
      }
      else {
        piVar17 = *(int **)piVar10;
      }
      piVar15 = piVar10;
      piVar10 = piVar17;
      cVar2 = *(char *)((longlong)piVar17 + 0x19);
    }
    if (((*(char *)((longlong)piVar15 + 0x19) == '\0') && (piVar15[8] <= (int)(uVar7 + 1))) &&
       (piVar15 != piVar13)) {
      iVar6 = uVar7 + 1;
      uStack_510 = 0;
      piVar15 = piVar13;
      local_518 = piVar12;
      while (piVar10 = piVar12, cVar3 == '\0') {
        if (iVar6 <= piVar10[8]) {
          piVar12 = *(int **)piVar10;
          piVar15 = piVar10;
        }
        else {
          piVar12 = *(int **)(piVar10 + 4);
        }
        uStack_510 = (uint)(iVar6 <= piVar10[8]);
        cVar3 = *(char *)((longlong)piVar12 + 0x19);
        local_518 = piVar10;
      }
      if ((*(char *)((longlong)piVar15 + 0x19) != '\0') || (iVar6 < piVar15[8])) {
        if (plVar21[4] == 0x555555555555555) goto LAB_142def912;
        local_4a0 = 0;
        local_4a8 = plVar1;
        puVar14 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x30);
        *(int *)(puVar14 + 4) = iVar6;
        puVar14[5] = 0;
        *puVar14 = piVar13;
        puVar14[1] = piVar13;
        puVar14[2] = piVar13;
        *(undefined2 *)(puVar14 + 3) = 0;
        local_4a0 = 0;
        local_4b8 = local_518;
        uStack_4b0 = uStack_510;
        uStack_4ac = uStack_50c;
        piVar15 = (int *)FUN_140301600(plVar1,&local_4b8,puVar14);
      }
      piVar13 = (int *)*plVar1;
      piVar12 = *(int **)(piVar13 + 2);
      uStack_510 = 0;
      cVar2 = *(char *)((longlong)piVar12 + 0x19);
      piVar10 = piVar13;
      local_518 = piVar12;
      while (piVar17 = piVar12, cVar2 == '\0') {
        if ((int)uVar7 <= piVar17[8]) {
          piVar12 = *(int **)piVar17;
          piVar10 = piVar17;
        }
        else {
          piVar12 = *(int **)(piVar17 + 4);
        }
        uStack_510 = (uint)((int)uVar7 <= piVar17[8]);
        cVar2 = *(char *)((longlong)piVar12 + 0x19);
        local_518 = piVar17;
      }
      if ((*(char *)((longlong)piVar10 + 0x19) != '\0') || ((int)uVar7 < piVar10[8])) {
        if (plVar21[4] == 0x555555555555555) {
LAB_142def912:
                    /* WARNING: Subroutine does not return */
          FUN_14019f9d0();
        }
        local_490 = 0;
        local_498 = plVar1;
        puVar14 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x30);
        *(uint *)(puVar14 + 4) = uVar7;
        puVar14[5] = 0;
        *puVar14 = piVar13;
        puVar14[1] = piVar13;
        puVar14[2] = piVar13;
        *(undefined2 *)(puVar14 + 3) = 0;
        local_490 = 0;
        local_4b8 = local_518;
        uStack_4b0 = uStack_510;
        uStack_4ac = uStack_50c;
        piVar10 = (int *)FUN_140301600(plVar1,&local_4b8,puVar14);
      }
      FUN_14019a260(piVar10 + 10,piVar15 + 10);
      iVar6 = local_528;
    }
    uVar7 = uVar7 + 1;
    pcVar23 = DAT_143aa8360;
  } while ((int)uVar7 < iVar6);
LAB_142deefe4:
  puVar14 = (undefined8 *)*plVar1;
  cVar2 = *(char *)((longlong)puVar14[1] + 0x19);
  puVar5 = puVar14;
  puVar18 = (undefined8 *)puVar14[1];
  while (cVar2 == '\0') {
    if (*(int *)(puVar18 + 4) < iVar6) {
      puVar11 = (undefined8 *)puVar18[2];
      puVar18 = puVar5;
    }
    else {
      puVar11 = (undefined8 *)*puVar18;
    }
    puVar5 = puVar18;
    puVar18 = puVar11;
    cVar2 = *(char *)((longlong)puVar11 + 0x19);
  }
  if (((*(char *)((longlong)puVar5 + 0x19) == '\0') && (*(int *)(puVar5 + 4) <= iVar6)) &&
     (puVar5 != puVar14)) {
    piVar12 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar12[1] = 0;
    *piVar12 = -1;
    piVar13 = piVar12 + 4;
    piVar12[2] = 0;
    *(undefined1 *)piVar13 = 0;
    local_4b8 = piVar13;
    if (*piVar12 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar12[1] < 0) {
      FUN_142e54290(0x90,piVar12[1],0);
    }
    *piVar12 = 1;
    *(undefined1 *)piVar13 = 0;
    if (piVar12[1] + 1 < 1) {
      FUN_142e54290(0x9c);
    }
    piVar12[2] = 0;
    piVar12 = (int *)*plVar1;
    piVar15 = *(int **)(piVar12 + 2);
    uStack_510 = 0;
    cVar2 = *(char *)((longlong)piVar15 + 0x19);
    piVar10 = piVar12;
    local_518 = piVar15;
    while (piVar17 = piVar15, cVar2 == '\0') {
      if (local_528 <= piVar17[8]) {
        piVar15 = *(int **)piVar17;
        piVar10 = piVar17;
      }
      else {
        piVar15 = *(int **)(piVar17 + 4);
      }
      uStack_510 = (uint)(local_528 <= piVar17[8]);
      cVar2 = *(char *)((longlong)piVar15 + 0x19);
      local_518 = piVar17;
    }
    if ((*(char *)((longlong)piVar10 + 0x19) != '\0') || (local_528 < piVar10[8])) {
      if (plVar21[4] == 0x555555555555555) {
                    /* WARNING: Subroutine does not return */
        FUN_14019f9d0();
      }
      local_4c8 = 0;
      local_4d0 = plVar1;
      puVar14 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x30);
      *(int *)(puVar14 + 4) = local_528;
      puVar14[5] = 0;
      *puVar14 = piVar12;
      puVar14[1] = piVar12;
      puVar14[2] = piVar12;
      *(undefined2 *)(puVar14 + 3) = 0;
      local_4c8 = 0;
      piVar10 = (int *)FUN_140301600(plVar1,&local_518,puVar14);
    }
    if (*(longlong *)(piVar10 + 10) != 0) {
      FUN_14019f2c0(*(longlong *)(piVar10 + 10) + -0x10);
    }
    *(int **)(piVar10 + 10) = piVar13;
    iVar6 = local_528 + -1;
    pcVar23 = DAT_143aa8360;
    local_528 = iVar6;
  }
  if ((int)uVar30 < (int)local_4d8) {
    local_4d8 = uVar30;
  }
LAB_142def198:
  plVar31 = plVar31 + 1;
  plVar9 = local_4f8;
  plVar19 = local_4f0;
  plVar29 = plStack_4e8;
  if (plVar31 == plVar20) goto joined_r0x000142def1b2;
  goto LAB_142deed20;
}



//===========================================================
// FUN_142debd10 @ 142debd10   (1719 bytes)
//===========================================================

void FUN_142debd10(longlong *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  byte bVar1;
  uint uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined8 uVar5;
  undefined8 *puVar6;
  longlong lVar7;
  int *piVar8;
  byte *pbVar9;
  longlong lVar10;
  undefined8 *puVar11;
  uint uVar12;
  undefined8 *puVar13;
  undefined8 *puVar14;
  undefined8 *puVar15;
  longlong lVar16;
  longlong lVar17;
  byte *pbVar18;
  int iVar19;
  int iVar20;
  longlong lVar21;
  ulonglong uVar22;
  undefined4 uVar23;
  byte *pbVar24;
  uint uVar25;
  byte *local_res20;
  byte *pbVar26;
  undefined8 local_198 [43];
  
  uVar25 = 0;
  lVar16 = 0;
  lVar17 = *param_1;
  do {
    if ((lVar17 == 0) || (lVar7 = lVar16, uVar2 = uVar25, *(uint *)(lVar17 + -8) <= uVar25)) {
      return;
    }
    for (; (lVar17 = *param_1, lVar17 != 0 && (uVar2 < *(uint *)(lVar17 + -8))); uVar2 = uVar2 + 1)
    {
      if (uVar2 != uVar25) {
        if (((int)uVar25 < 0) || (*(uint *)(lVar17 + -8) <= uVar25)) {
          FUN_142e54290(0xbc,uVar25);
          lVar17 = *param_1;
        }
        lVar21 = lVar16 * 0x149;
        if ((byte)(*(char *)(lVar21 + 0x11 + lVar17) - 5U) < 4) {
          if (lVar17 == 0) {
            uVar12 = 0;
          }
          else {
            uVar12 = *(uint *)(lVar17 + -8);
          }
          if (((int)uVar25 < 0) || (uVar12 <= uVar25)) {
            if (lVar17 == 0) {
              uVar23 = 0;
            }
            else {
              uVar23 = *(undefined4 *)(lVar17 + -8);
            }
            FUN_142e54290(0xbc,uVar25,uVar23);
            lVar17 = *param_1;
          }
          lVar17 = lVar17 + 0x2c;
        }
        else {
          if (lVar17 == 0) {
            uVar12 = 0;
          }
          else {
            uVar12 = *(uint *)(lVar17 + -8);
          }
          if (((int)uVar25 < 0) || (uVar12 <= uVar25)) {
            if (lVar17 == 0) {
              uVar23 = 0;
            }
            else {
              uVar23 = *(undefined4 *)(lVar17 + -8);
            }
            FUN_142e54290(0xbc,uVar25,uVar23);
            lVar17 = *param_1;
          }
          lVar17 = lVar17 + 4;
        }
        lVar17 = lVar17 + (longlong)(int)uVar25 * 0x149;
        local_res20 = (byte *)0x0;
        if (lVar17 != 0) {
          uVar22 = 0xffffffffffffffff;
          do {
            uVar22 = uVar22 + 1;
          } while (*(char *)(lVar17 + uVar22) != '\0');
          iVar19 = (int)uVar22;
          iVar20 = 0;
          if (0 < iVar19) {
            iVar20 = iVar19;
          }
          piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar20 + 0x11));
          piVar8[1] = iVar20;
          *piVar8 = -1;
          local_res20 = (byte *)(piVar8 + 4);
          piVar8[2] = 0;
          *local_res20 = 0;
          FUN_142ef7ba0(local_res20,lVar17,(longlong)iVar19);
          if (*piVar8 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar19 == -1) || (iVar19 <= piVar8[1])) {
            *piVar8 = 1;
            if (iVar19 != -1) goto LAB_142debed8;
            if (local_res20 == (byte *)0x0) {
              uVar22 = 0;
            }
            else {
              uVar22 = 0xffffffffffffffff;
              do {
                uVar22 = uVar22 + 1;
              } while (local_res20[uVar22] != 0);
            }
          }
          else {
            FUN_142e54290(0x90,piVar8[1],uVar22 & 0xffffffff);
            *piVar8 = 1;
LAB_142debed8:
            local_res20[iVar19] = 0;
          }
          iVar20 = (int)uVar22;
          if ((iVar20 < 0) || (piVar8[1] + 1 <= iVar20)) {
            FUN_142e54290(0x9c,uVar22 & 0xffffffff);
          }
          piVar8[2] = iVar20;
        }
        lVar17 = *param_1;
        if (lVar17 == 0) {
          uVar12 = 0;
        }
        else {
          uVar12 = *(uint *)(lVar17 + -8);
        }
        if (((int)uVar25 < 0) || (uVar12 <= uVar25)) {
          if (lVar17 == 0) {
            uVar23 = 0;
          }
          else {
            uVar23 = *(undefined4 *)(lVar17 + -8);
          }
          FUN_142e54290(0xbc,uVar25,uVar23);
          lVar17 = *param_1;
        }
        if ((byte)(*(char *)(lVar21 + 0x11 + lVar17) - 5U) < 4) {
          if (lVar17 == 0) {
            uVar12 = 0;
          }
          else {
            uVar12 = *(uint *)(lVar17 + -8);
          }
          if (((int)uVar2 < 0) || (uVar12 <= uVar2)) {
            if (lVar17 == 0) {
              uVar23 = 0;
            }
            else {
              uVar23 = *(undefined4 *)(lVar17 + -8);
            }
            FUN_142e54290(0xbc,uVar2,uVar23);
            lVar17 = *param_1;
          }
          lVar17 = lVar17 + 0x2c;
        }
        else {
          if (lVar17 == 0) {
            uVar12 = 0;
          }
          else {
            uVar12 = *(uint *)(lVar17 + -8);
          }
          if (((int)uVar2 < 0) || (uVar12 <= uVar2)) {
            if (lVar17 == 0) {
              uVar23 = 0;
            }
            else {
              uVar23 = *(undefined4 *)(lVar17 + -8);
            }
            FUN_142e54290(0xbc,uVar2,uVar23);
            lVar17 = *param_1;
          }
          lVar17 = lVar17 + 4;
        }
        lVar17 = lVar17 + (longlong)(int)uVar2 * 0x149;
        pbVar18 = (byte *)0x0;
        pbVar26 = (byte *)0x0;
        if (lVar17 != 0) {
          uVar22 = 0xffffffffffffffff;
          do {
            uVar22 = uVar22 + 1;
          } while (*(char *)(uVar22 + lVar17) != '\0');
          iVar19 = (int)uVar22;
          iVar20 = 0;
          if (0 < iVar19) {
            iVar20 = iVar19;
          }
          piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar20 + 0x11));
          piVar8[1] = iVar20;
          *piVar8 = -1;
          pbVar18 = (byte *)(piVar8 + 4);
          piVar8[2] = 0;
          *pbVar18 = 0;
          pbVar26 = pbVar18;
          FUN_142ef7ba0(pbVar18,lVar17,(longlong)iVar19,param_4,pbVar18);
          if (*piVar8 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar19 == -1) || (iVar19 <= piVar8[1])) {
            *piVar8 = 1;
            if (iVar19 != -1) goto LAB_142dec099;
            if (pbVar18 == (byte *)0x0) {
              uVar22 = 0;
            }
            else {
              uVar22 = 0xffffffffffffffff;
              do {
                uVar22 = uVar22 + 1;
              } while (pbVar18[uVar22] != 0);
            }
          }
          else {
            FUN_142e54290(0x90,piVar8[1],uVar22 & 0xffffffff);
            *piVar8 = 1;
LAB_142dec099:
            pbVar18[iVar19] = 0;
          }
          iVar20 = (int)uVar22;
          if ((iVar20 < 0) || (piVar8[1] + 1 <= iVar20)) {
            FUN_142e54290(0x9c,uVar22 & 0xffffffff);
          }
          piVar8[2] = iVar20;
        }
        pbVar24 = DAT_143aa8360;
        if (pbVar18 != (byte *)0x0) {
          pbVar24 = pbVar18;
        }
        pbVar9 = DAT_143aa8360;
        if (local_res20 != (byte *)0x0) {
          pbVar9 = local_res20;
        }
        lVar17 = (longlong)pbVar24 - (longlong)pbVar9;
        do {
          bVar1 = *pbVar9;
          uVar12 = (uint)pbVar9[lVar17];
          if (bVar1 != uVar12) break;
          pbVar9 = pbVar9 + 1;
        } while (uVar12 != 0);
        if (0 < (int)(bVar1 - uVar12)) {
          lVar17 = *param_1;
          if (lVar17 == 0) {
            uVar12 = 0;
          }
          else {
            uVar12 = *(uint *)(lVar17 + -8);
          }
          if (((int)uVar2 < 0) || (uVar12 <= uVar2)) {
            if (lVar17 == 0) {
              uVar23 = 0;
            }
            else {
              uVar23 = *(undefined4 *)(lVar17 + -8);
            }
            FUN_142e54290(0xbc,uVar2,uVar23,param_4,pbVar26);
            lVar17 = *param_1;
          }
          puVar15 = (undefined8 *)(lVar7 * 0x149 + lVar17);
          if (lVar17 == 0) {
            uVar12 = 0;
          }
          else {
            uVar12 = *(uint *)(lVar17 + -8);
          }
          if (((int)uVar25 < 0) || (uVar12 <= uVar25)) {
            if (lVar17 == 0) {
              uVar23 = 0;
            }
            else {
              uVar23 = *(undefined4 *)(lVar17 + -8);
            }
            FUN_142e54290(0xbc,uVar25,uVar23,param_4,pbVar26);
            lVar17 = *param_1;
          }
          lVar10 = 2;
          puVar6 = (undefined8 *)(lVar17 + lVar21);
          puVar11 = local_198;
          do {
            puVar14 = puVar11;
            puVar13 = puVar6;
            uVar5 = puVar13[1];
            *puVar14 = *puVar13;
            puVar14[1] = uVar5;
            uVar5 = puVar13[3];
            puVar14[2] = puVar13[2];
            puVar14[3] = uVar5;
            uVar5 = puVar13[5];
            puVar14[4] = puVar13[4];
            puVar14[5] = uVar5;
            uVar5 = puVar13[7];
            puVar14[6] = puVar13[6];
            puVar14[7] = uVar5;
            uVar5 = puVar13[9];
            puVar14[8] = puVar13[8];
            puVar14[9] = uVar5;
            uVar5 = puVar13[0xb];
            puVar14[10] = puVar13[10];
            puVar14[0xb] = uVar5;
            uVar5 = puVar13[0xd];
            puVar14[0xc] = puVar13[0xc];
            puVar14[0xd] = uVar5;
            uVar5 = puVar13[0xf];
            puVar14[0xe] = puVar13[0xe];
            puVar14[0xf] = uVar5;
            lVar10 = lVar10 + -1;
            puVar6 = puVar13 + 0x10;
            puVar11 = puVar14 + 0x10;
          } while (lVar10 != 0);
          uVar5 = puVar13[0x11];
          puVar14[0x10] = puVar13[0x10];
          puVar14[0x11] = uVar5;
          uVar5 = puVar13[0x13];
          puVar14[0x12] = puVar13[0x12];
          puVar14[0x13] = uVar5;
          uVar5 = puVar13[0x15];
          puVar14[0x14] = puVar13[0x14];
          puVar14[0x15] = uVar5;
          uVar5 = puVar13[0x17];
          puVar14[0x16] = puVar13[0x16];
          puVar14[0x17] = uVar5;
          puVar14[0x18] = puVar13[0x18];
          *(undefined1 *)(puVar14 + 0x19) = *(undefined1 *)(puVar13 + 0x19);
          lVar10 = 2;
          puVar6 = (undefined8 *)(lVar17 + lVar21);
          puVar11 = puVar15;
          do {
            puVar14 = puVar11;
            puVar13 = puVar6;
            uVar5 = puVar14[1];
            *puVar13 = *puVar14;
            puVar13[1] = uVar5;
            uVar5 = puVar14[3];
            puVar13[2] = puVar14[2];
            puVar13[3] = uVar5;
            uVar5 = puVar14[5];
            puVar13[4] = puVar14[4];
            puVar13[5] = uVar5;
            uVar5 = puVar14[7];
            puVar13[6] = puVar14[6];
            puVar13[7] = uVar5;
            uVar5 = puVar14[9];
            puVar13[8] = puVar14[8];
            puVar13[9] = uVar5;
            uVar5 = puVar14[0xb];
            puVar13[10] = puVar14[10];
            puVar13[0xb] = uVar5;
            uVar5 = puVar14[0xd];
            puVar13[0xc] = puVar14[0xc];
            puVar13[0xd] = uVar5;
            uVar5 = puVar14[0xf];
            puVar13[0xe] = puVar14[0xe];
            puVar13[0xf] = uVar5;
            lVar10 = lVar10 + -1;
            puVar6 = puVar13 + 0x10;
            puVar11 = puVar14 + 0x10;
          } while (lVar10 != 0);
          uVar5 = puVar14[0x11];
          puVar13[0x10] = puVar14[0x10];
          puVar13[0x11] = uVar5;
          uVar5 = puVar14[0x13];
          puVar13[0x12] = puVar14[0x12];
          puVar13[0x13] = uVar5;
          uVar5 = puVar14[0x15];
          puVar13[0x14] = puVar14[0x14];
          puVar13[0x15] = uVar5;
          uVar5 = puVar14[0x17];
          puVar13[0x16] = puVar14[0x16];
          puVar13[0x17] = uVar5;
          puVar13[0x18] = puVar14[0x18];
          *(undefined1 *)(puVar13 + 0x19) = *(undefined1 *)(puVar14 + 0x19);
          lVar17 = 2;
          puVar6 = local_198;
          do {
            puVar13 = puVar15;
            puVar11 = puVar6;
            uVar5 = puVar11[1];
            *puVar13 = *puVar11;
            puVar13[1] = uVar5;
            uVar5 = puVar11[3];
            puVar13[2] = puVar11[2];
            puVar13[3] = uVar5;
            uVar5 = puVar11[5];
            puVar13[4] = puVar11[4];
            puVar13[5] = uVar5;
            uVar5 = puVar11[7];
            puVar13[6] = puVar11[6];
            puVar13[7] = uVar5;
            uVar5 = puVar11[9];
            puVar13[8] = puVar11[8];
            puVar13[9] = uVar5;
            uVar5 = puVar11[0xb];
            puVar13[10] = puVar11[10];
            puVar13[0xb] = uVar5;
            uVar5 = puVar11[0xd];
            puVar13[0xc] = puVar11[0xc];
            puVar13[0xd] = uVar5;
            uVar5 = puVar11[0xf];
            puVar13[0xe] = puVar11[0xe];
            puVar13[0xf] = uVar5;
            lVar17 = lVar17 + -1;
            puVar6 = puVar11 + 0x10;
            puVar15 = puVar13 + 0x10;
          } while (lVar17 != 0);
          uVar5 = puVar11[0x11];
          puVar13[0x10] = puVar11[0x10];
          puVar13[0x11] = uVar5;
          uVar5 = puVar11[0x13];
          puVar13[0x12] = puVar11[0x12];
          puVar13[0x13] = uVar5;
          uVar23 = *(undefined4 *)((longlong)puVar11 + 0xa4);
          uVar3 = *(undefined4 *)(puVar11 + 0x15);
          uVar4 = *(undefined4 *)((longlong)puVar11 + 0xac);
          *(undefined4 *)(puVar13 + 0x14) = *(undefined4 *)(puVar11 + 0x14);
          *(undefined4 *)((longlong)puVar13 + 0xa4) = uVar23;
          *(undefined4 *)(puVar13 + 0x15) = uVar3;
          *(undefined4 *)((longlong)puVar13 + 0xac) = uVar4;
          uVar5 = puVar11[0x17];
          puVar13[0x16] = puVar11[0x16];
          puVar13[0x17] = uVar5;
          puVar13[0x18] = puVar11[0x18];
          *(undefined1 *)(puVar13 + 0x19) = *(undefined1 *)(puVar11 + 0x19);
        }
        if (pbVar18 != (byte *)0x0) {
          FUN_14019f2c0(pbVar18 + -0x10);
        }
        if (local_res20 != (byte *)0x0) {
          FUN_14019f2c0(local_res20 + -0x10);
        }
      }
      lVar7 = lVar7 + 1;
    }
    uVar25 = uVar25 + 1;
    lVar16 = lVar16 + 1;
  } while( true );
}


