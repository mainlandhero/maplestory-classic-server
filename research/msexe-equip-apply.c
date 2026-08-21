
//===========================================================
// FUN_1402e3770 @ 1402e3770   (366 bytes)
//===========================================================

longlong FUN_1402e3770(longlong param_1,int param_2,uint param_3)

{
  int iVar1;
  longlong lVar2;
  int iVar3;
  
  if (param_2 - 1U < 6) {
    if (param_2 == 1) {
      if ((int)param_3 < 0) {
        iVar3 = -param_3;
        iVar1 = FUN_140302620(iVar3);
        if (iVar1 != 0) {
          lVar2 = FUN_1402de170(param_1 + 0x5a8,iVar3);
          return lVar2;
        }
        if (0x1e < param_3 + 0x1f) {
          return 0;
        }
        return (longlong)iVar3 * 0x10 + 0x1a8 + param_1;
      }
    }
    else if ((param_2 == 6) && ((int)param_3 < 0)) {
      if ((0xd < -param_3 - 0x4b0) && (0x32 < -param_3 - 0x708)) {
        if (0x1e < param_3 + 0x83) {
          return 0;
        }
        return param_1 + 0x3a8 + (longlong)(int)(-100 - param_3) * 0x10;
      }
      lVar2 = FUN_1402de170(param_1 + 0x5a8,-param_3);
      return lVar2;
    }
    if (0 < (int)param_3) {
      param_1 = param_1 + (longlong)param_2 * 8;
      lVar2 = *(longlong *)(param_1 + 0x5d0);
      if ((lVar2 != 0) && ((int)param_3 <= *(int *)(lVar2 + -8) + -1)) {
        if (*(uint *)(lVar2 + -8) <= param_3) {
          FUN_142e54290(0xc6,param_3);
          lVar2 = *(longlong *)(param_1 + 0x5d0);
        }
        return (longlong)(int)param_3 * 0x10 + lVar2;
      }
    }
  }
  return 0;
}



//===========================================================
// FUN_140302620 @ 140302620   (42 bytes)
//===========================================================

undefined8 FUN_140302620(int param_1)

{
  if (((0x1f < param_1 - 3000U) && (0x1f < param_1 - 0xc1cU)) && (0x1f < param_1 - 0xc80U)) {
    return 0;
  }
  return 1;
}



//===========================================================
// FUN_1401e8780 @ 1401e8780   (142 bytes)
//===========================================================

longlong FUN_1401e8780(longlong param_1,longlong param_2)

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
    if (0xfffff < *(ulonglong *)(lVar1 + 8)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar1 + 8) = *(longlong *)(lVar1 + 8) + 1;
    UNLOCK();
  }
  FUN_1401abd80(param_1);
  *(undefined8 *)(param_1 + 8) = *(undefined8 *)(param_2 + 8);
  return param_1;
}



//===========================================================
// FUN_1401abd80 @ 1401abd80   (106 bytes)
//===========================================================

void FUN_1401abd80(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 *puVar3;
  
  lVar2 = *(longlong *)(param_1 + 8);
  if (lVar2 != 0) {
    if (0xffffe < *(longlong *)(lVar2 + 8) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + 8);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (((int)lVar2 == 1) && (puVar3 = *(undefined8 **)(param_1 + 8), puVar3 != (undefined8 *)0x0))
    {
      (**(code **)*puVar3)(puVar3,1);
    }
    *(undefined8 *)(param_1 + 8) = 0;
  }
  return;
}



//===========================================================
// FUN_140232590 @ 140232590   (68 bytes)
//===========================================================

longlong FUN_140232590(longlong param_1,longlong param_2)

{
  longlong lVar1;
  
  lVar1 = *(longlong *)(param_2 + 8);
  *(longlong *)(param_1 + 8) = lVar1;
  if (lVar1 != 0) {
    if (0xfffff < *(ulonglong *)(lVar1 + 8)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar1 + 8) = *(longlong *)(lVar1 + 8) + 1;
    UNLOCK();
  }
  return param_1;
}



//===========================================================
// FUN_1402de550 @ 1402de550   (355 bytes)
//===========================================================

void FUN_1402de550(longlong param_1,longlong param_2,int param_3)

{
  int iVar1;
  int iVar2;
  longlong lVar3;
  ulonglong uVar4;
  undefined8 uVar5;
  uint uVar6;
  ulonglong uVar7;
  longlong lVar8;
  undefined1 local_38 [8];
  longlong local_30;
  
  if ((((param_3 - 0x4b0U < 0xe) || (param_3 - 0x708U < 0x33)) || (param_3 - 3000U < 0x20)) ||
     ((param_3 - 0xc1cU < 0x20 || (param_3 - 0xc80U < 0x20)))) {
    uVar4 = 0;
    uVar7 = uVar4;
    do {
      uVar6 = (uint)uVar7;
      if (((uVar4 < 5) && ((int)(&DAT_14327dd50)[uVar4] <= param_3)) &&
         (param_3 < (int)(&DAT_14327dd68)[uVar4])) {
        if (uVar6 != 0xffffffff) {
          lVar8 = (longlong)(int)uVar6;
          iVar2 = (&DAT_14327dd50)[lVar8];
          lVar3 = *(longlong *)(param_2 + 8);
          local_30 = lVar3;
          if (lVar3 != 0) {
            if (0xfffff < *(ulonglong *)(lVar3 + 8)) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            *(longlong *)(lVar3 + 8) = *(longlong *)(lVar3 + 8) + 1;
            UNLOCK();
          }
          if (uVar6 < 5) {
            iVar1 = (&DAT_14327dd50)[lVar8] + (param_3 - iVar2);
            if ((iVar1 < (int)(&DAT_14327dd50)[lVar8]) || ((int)(&DAT_14327dd68)[lVar8] <= iVar1)) {
              FUN_1401abd80(local_38);
            }
            else {
              uVar5 = FUN_1402f1620(param_1 + lVar8 * 8,param_3 - iVar2);
              FUN_1401e8780(uVar5,local_38);
              FUN_1401abd80(local_38);
            }
          }
          else {
            FUN_1401abd80(local_38);
          }
          FUN_1401abd80(param_2);
          return;
        }
        break;
      }
      uVar7 = (ulonglong)(uVar6 + 1);
      uVar4 = uVar4 + 1;
    } while ((longlong)uVar4 < 5);
  }
  FUN_1401abd80(param_2);
  return;
}



//===========================================================
// FUN_1402f1620 @ 1402f1620   (88 bytes)
//===========================================================

longlong FUN_1402f1620(longlong *param_1,uint param_2)

{
  undefined4 uVar1;
  uint uVar2;
  longlong lVar3;
  
  lVar3 = *param_1;
  uVar1 = 0;
  uVar2 = 0;
  if (lVar3 != 0) {
    uVar2 = *(uint *)(lVar3 + -8);
  }
  if (((int)param_2 < 0) || (uVar2 <= param_2)) {
    if (lVar3 != 0) {
      uVar1 = *(undefined4 *)(lVar3 + -8);
    }
    FUN_142e54290(0xbc,param_2,uVar1);
    lVar3 = *param_1;
  }
  return (longlong)(int)param_2 * 0x10 + lVar3;
}



//===========================================================
// FUN_142da62e0 @ 142da62e0   (1302 bytes)
//===========================================================

void FUN_142da62e0(undefined8 param_1,longlong param_2,int param_3)

{
  longlong *plVar1;
  int iVar2;
  undefined8 uVar3;
  bool bVar4;
  bool bVar5;
  char *pcVar6;
  longlong lVar7;
  int iVar8;
  longlong lVar9;
  undefined8 *puVar10;
  longlong *plVar11;
  undefined4 *puVar12;
  int iVar13;
  int *piVar14;
  int *piVar15;
  int *piVar16;
  int *piVar17;
  longlong lVar18;
  int iVar19;
  int iVar20;
  char *local_res20;
  longlong local_78;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  undefined1 local_50 [8];
  longlong local_48;
  
  if (param_2 == 0) {
    return;
  }
  bVar5 = false;
  bVar4 = false;
  iVar8 = FUN_14038f960(DAT_143aa8328);
  if (iVar8 == 0) {
    return;
  }
  FUN_140391940(DAT_143aa8328,local_50,param_2);
  lVar18 = local_48;
  if ((local_48 != 0) && (plVar11 = *(longlong **)(local_48 + 0x48), plVar11 != (longlong *)0x0)) {
    plVar1 = plVar11 + *(uint *)(local_48 + 0x50);
    for (; plVar11 < plVar1; plVar11 = plVar11 + 1) {
      lVar9 = *plVar11;
      if (*plVar11 != 0) goto LAB_142da6386;
    }
  }
  goto LAB_142da674a;
LAB_142da6386:
  do {
    piVar15 = (int *)0x0;
    piVar16 = (int *)0xffffffffffffffff;
    if (lVar18 == 0) {
      FUN_142e52ed0(0x431);
      lVar18 = local_48;
    }
    lVar7 = *(longlong *)(lVar9 + 8);
    if (*(longlong *)(lVar9 + 8) == 0) {
      local_78 = 0;
      for (plVar11 = (longlong *)
                     (*(longlong *)(lVar18 + 0x48) +
                     ((ulonglong)(longlong)*(int *)(lVar9 + 0x10) %
                      (ulonglong)*(uint *)(lVar18 + 0x50) + 1) * 8);
          (lVar7 = local_78,
          plVar11 < (longlong *)
                    (*(longlong *)(lVar18 + 0x48) + (ulonglong)*(uint *)(lVar18 + 0x50) * 8) &&
          (lVar7 = *plVar11, *plVar11 == 0)); plVar11 = plVar11 + 1) {
      }
    }
    local_78 = lVar7;
    iVar8 = *(int *)(lVar9 + 0x10);
    if ((iVar8 != 0) && (*(int *)(lVar9 + 0x14) != 0)) {
      lVar9 = FUN_1407b2910(DAT_143aa84d0);
      lVar18 = local_48;
      if ((*(longlong *)(lVar9 + 8) != 0) && ((iVar8 != 0x4c4bfda && (iVar8 != 0x4c4c10e)))) {
        local_res20 = (char *)0x0;
        uVar3 = *(undefined8 *)(*(longlong *)(lVar9 + 8) + 0x10);
        if (param_3 == 0) {
          puVar10 = (undefined8 *)FUN_1408a9e40(&local_68,0x1288);
          bVar4 = true;
        }
        else {
          puVar10 = (undefined8 *)FUN_1408a9e40(&local_60,0x1287);
          bVar5 = true;
        }
        FUN_14019ba10(&local_res20,*puVar10,uVar3);
        if ((bVar4) && (bVar4 = false, local_68 != 0)) {
          FUN_14019f2c0(local_68 + -0x10);
        }
        if (bVar5) {
          bVar5 = false;
          if (local_60 != 0) {
            FUN_14019f2c0(local_60 + -0x10);
            bVar5 = false;
          }
        }
        plVar11 = (longlong *)FUN_1408a9e40(&local_58,0x1289);
        pcVar6 = local_res20;
        lVar18 = *plVar11;
        if (lVar18 != 0) {
          iVar8 = *(int *)(lVar18 + -8);
          piVar14 = (int *)(longlong)iVar8;
          if (iVar8 != 0) {
            iVar20 = 0;
            piVar17 = piVar15;
            if (local_res20 == (char *)0x0) goto LAB_142da6650;
            if (*local_res20 != '\0') {
              iVar2 = *(int *)(local_res20 + -8);
              for (iVar13 = *(int *)(local_res20 + -0xc); iVar13 < iVar2 + iVar8;
                  iVar13 = iVar13 * 2) {
              }
              piVar15 = (int *)(local_res20 + -0x10);
              if (piVar15 == (int *)0x0) {
LAB_142da6551:
                if (iVar20 < iVar13) {
                  iVar20 = iVar13;
                }
                puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar20 + 0x11));
                puVar12[1] = iVar20;
                *puVar12 = 0xffffffff;
                local_res20 = (char *)(puVar12 + 4);
                if (piVar15 == (int *)0x0) {
                  puVar12[2] = 0;
                  *local_res20 = '\0';
                }
                else {
                  iVar13 = *(int *)(pcVar6 + -8) + 1;
                  iVar19 = iVar20 + 1;
                  if (iVar19 < iVar13) {
                    FUN_142e54290(0x5c,iVar13,iVar19);
                    iVar13 = iVar19;
                  }
                  FUN_142ef7ba0(local_res20,pcVar6,(longlong)iVar13);
                  puVar12[2] = *(undefined4 *)(pcVar6 + -8);
                  local_res20[iVar20] = '\0';
                  FUN_14019f2c0(piVar15);
                }
              }
              else {
                if ((1 < *piVar15) || (*(int *)(local_res20 + -0xc) < iVar13)) {
                  iVar20 = *(int *)(local_res20 + -8);
                  goto LAB_142da6551;
                }
                if (*piVar15 != 1) {
                  FUN_142e52dd0(0x74);
                }
                *piVar15 = -1;
              }
              if (local_res20 == (char *)0x0) {
                iVar20 = 0;
              }
              else {
                iVar20 = *(int *)(local_res20 + -8);
              }
              FUN_142ef7ba0(local_res20 + iVar20,lVar18,piVar14);
              FUN_14019c870(&local_res20,iVar2 + iVar8);
              goto LAB_142da6701;
            }
            if ((local_res20 == (char *)0x0) ||
               (piVar17 = (int *)(local_res20 + -0x10), piVar17 == (int *)0x0)) {
LAB_142da6650:
              if (iVar20 < iVar8) {
                iVar20 = iVar8;
              }
              puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar20 + 0x11));
              puVar12[1] = iVar20;
              *puVar12 = 0xffffffff;
              local_res20 = (char *)(puVar12 + 4);
              puVar12[2] = 0;
              *local_res20 = '\0';
              if (piVar17 != (int *)0x0) {
                FUN_14019f2c0(piVar17);
              }
            }
            else {
              if ((1 < *piVar17) || (*(int *)(local_res20 + -0xc) < iVar8)) {
                iVar20 = *(int *)(local_res20 + -8);
                goto LAB_142da6650;
              }
              if (*piVar17 != 1) {
                FUN_142e52dd0(0x74);
              }
              *piVar17 = -1;
            }
            FUN_142ef7ba0(local_res20,lVar18,piVar14);
            pcVar6 = local_res20;
            if (*(int *)(local_res20 + -0x10) != -1) {
              FUN_142e52dd0(0x8b);
            }
            if ((iVar8 == -1) || (iVar8 <= *(int *)(pcVar6 + -0xc))) {
              pcVar6[-0x10] = '\x01';
              pcVar6[-0xf] = '\0';
              pcVar6[-0xe] = '\0';
              pcVar6[-0xd] = '\0';
              if (iVar8 != -1) goto LAB_142da66da;
              if (pcVar6 != (char *)0x0) {
                do {
                  piVar16 = (int *)((longlong)piVar16 + 1);
                  piVar15 = piVar16;
                } while (pcVar6[(longlong)piVar16] != '\0');
              }
            }
            else {
              FUN_142e54290(0x90,*(int *)(pcVar6 + -0xc),iVar8);
              pcVar6[-0x10] = '\x01';
              pcVar6[-0xf] = '\0';
              pcVar6[-0xe] = '\0';
              pcVar6[-0xd] = '\0';
LAB_142da66da:
              *(char *)((longlong)piVar14 + (longlong)local_res20) = '\0';
              piVar15 = piVar14;
            }
            iVar8 = (int)piVar15;
            if ((iVar8 < 0) || (*(int *)(pcVar6 + -0xc) + 1 <= iVar8)) {
              FUN_142e54290(0x9c,(ulonglong)piVar15 & 0xffffffff);
            }
            *(int *)(pcVar6 + -8) = iVar8;
          }
        }
LAB_142da6701:
        if (local_58 != 0) {
          FUN_14019f2c0(local_58 + -0x10);
        }
        FUN_1415eca30(&local_res20);
        lVar18 = local_48;
        if (local_res20 != (char *)0x0) {
          FUN_14019f2c0(local_res20 + -0x10);
          lVar18 = local_48;
        }
      }
    }
    lVar9 = local_78;
  } while (local_78 != 0);
LAB_142da674a:
  if (lVar18 != 0) {
    puVar10 = (undefined8 *)(lVar18 + -0x28);
    if (0xffffe < *(longlong *)(lVar18 + -0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar11 = (longlong *)(lVar18 + -0x20);
    lVar18 = *plVar11;
    *plVar11 = *plVar11 + -1;
    UNLOCK();
    if ((int)lVar18 == 1) {
      if ((local_48 != 0) && (*(longlong *)(local_48 + -0x10) != 0)) {
        LOCK();
        *(undefined8 *)(*(longlong *)(local_48 + -0x10) + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(local_48 + -0x10) + 4) != 0);
      }
      if (puVar10 != (undefined8 *)0x0) {
        (**(code **)*puVar10)(puVar10,1);
      }
    }
  }
  return;
}



//===========================================================
// FUN_1402536d0 @ 1402536d0   (15 bytes)
//===========================================================

bool FUN_1402536d0(int param_1)

{
  return param_1 - 0x708U < 0x33;
}



//===========================================================
// FUN_140193080 @ 140193080   (45 bytes)
//===========================================================

longlong * FUN_140193080(longlong *param_1)

{
  int iVar1;
  
  if (param_1 != (longlong *)0x0) {
    iVar1 = (**(code **)(*param_1 + 0x88))();
    if (iVar1 == 1) {
      return param_1;
    }
  }
  return (longlong *)0x0;
}



//===========================================================
// FUN_142d9bfd0 @ 142d9bfd0   (1124 bytes)
//===========================================================

void FUN_142d9bfd0(longlong *param_1,undefined4 param_2)

{
  longlong lVar1;
  longlong *plVar2;
  undefined8 *puVar3;
  longlong *plVar4;
  char cVar5;
  int iVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  int iVar10;
  longlong lVar11;
  longlong lVar12;
  undefined8 *puVar13;
  longlong lVar14;
  undefined8 *puVar15;
  undefined8 *puVar16;
  longlong *plVar17;
  uint uVar18;
  undefined8 uVar19;
  longlong lVar20;
  ulonglong uVar21;
  int local_res18;
  undefined8 *local_78;
  undefined8 local_70;
  undefined8 local_68;
  longlong local_60;
  undefined1 local_50 [8];
  longlong local_48;
  
  lVar11 = FUN_14209ee40();
  if (((*(longlong *)(lVar11 + 8) == 0) || (cVar5 = FUN_14209f830(), cVar5 == '\0')) &&
     (cVar5 = FUN_141b1f960(DAT_143abea80,1), cVar5 == '\0')) {
    if (DAT_143aa8518 == 0) {
      uVar19 = 0;
    }
    else {
      lVar12 = FUN_1428f74d0(DAT_143aa8518,local_50);
      lVar11 = local_48;
      uVar19 = *(undefined8 *)(lVar12 + 8);
      if (local_48 != 0) {
        puVar16 = (undefined8 *)(local_48 + -0x28);
        if (0xffffe < *(longlong *)(local_48 + -0x20) - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar17 = (longlong *)(lVar11 + -0x20);
        lVar11 = *plVar17;
        *plVar17 = *plVar17 + -1;
        UNLOCK();
        if ((int)lVar11 == 1) {
          if ((local_48 != 0) && (*(longlong *)(local_48 + -0x10) != 0)) {
            LOCK();
            *(undefined8 *)(*(longlong *)(local_48 + -0x10) + 8) = 0;
            UNLOCK();
            do {
            } while (*(int *)(*(longlong *)(local_48 + -0x10) + 4) != 0);
          }
          if (puVar16 != (undefined8 *)0x0) {
            (**(code **)*puVar16)(puVar16,1);
          }
        }
        local_48 = 0;
      }
    }
    lVar11 = DAT_143aa9d98;
    local_res18 = 0;
    local_78 = (undefined8 *)0x0;
    iVar6 = FUN_140719570(DAT_143aa9d98,param_2,&local_78);
    if (iVar6 != 0) {
      lVar12 = FUN_142cbe730(param_1);
      local_60 = lVar12;
      local_68 = (**(code **)(*param_1 + 0x30))(param_1);
      local_70 = FUN_142cbec90(param_1);
      plVar17 = *(longlong **)*local_78;
      cVar5 = *(char *)((longlong)plVar17 + 0x19);
      while (cVar5 == '\0') {
        iVar6 = *(int *)((longlong)plVar17 + 0x1c);
        uVar21 = (ulonglong)iVar6;
        if ((*(longlong *)(lVar12 + 0x1273) != 0) &&
           (lVar14 = *(longlong *)
                      (*(longlong *)(lVar12 + 0x1273) +
                      (uVar21 % (ulonglong)*(uint *)(lVar12 + 0x127b)) * 8), lVar14 != 0)) {
LAB_142d9c180:
          if (*(int *)(lVar14 + 0x10) != iVar6) goto code_r0x000142d9c186;
          puVar16 = *(undefined8 **)(lVar11 + 0x3b0);
          cVar5 = *(char *)((longlong)puVar16[1] + 0x19);
          puVar3 = puVar16;
          puVar15 = (undefined8 *)puVar16[1];
          while (cVar5 == '\0') {
            if (*(int *)((longlong)puVar15 + 0x1c) < iVar6) {
              puVar13 = (undefined8 *)puVar15[2];
              puVar15 = puVar3;
            }
            else {
              puVar13 = (undefined8 *)*puVar15;
            }
            puVar3 = puVar15;
            puVar15 = puVar13;
            cVar5 = *(char *)((longlong)puVar13 + 0x19);
          }
          if (((*(char *)((longlong)puVar3 + 0x19) == '\0') &&
              (*(int *)((longlong)puVar3 + 0x1c) <= iVar6)) && (puVar3 != puVar16)) {
            uVar7 = FUN_142cb85d0(param_1);
            uVar8 = FUN_142cb85b0(param_1);
            uVar9 = FUN_142cafb20(param_1);
            lVar14 = FUN_14070fa90(lVar11,iVar6);
            iVar10 = FUN_140711d70(lVar11,iVar6,*(undefined4 *)(lVar14 + 0x2c),uVar9,lVar12,local_68
                                   ,local_70,uVar8,uVar7,uVar19);
            lVar14 = DAT_143ad20e8;
            uVar18 = 0;
            lVar20 = 0;
            if (iVar10 == 0) {
              lVar12 = local_60;
              local_res18 = iVar6;
              if (DAT_143ad20e8 == 0) goto LAB_142d9c3c0;
              lVar12 = *(longlong *)(DAT_143ad20e8 + 0x390);
              if (lVar12 != 0) {
                for (lVar1 = *(longlong *)
                              (lVar12 + (uVar21 % (ulonglong)*(uint *)(DAT_143ad20e8 + 0x398)) * 8);
                    lVar1 != 0; lVar1 = *(longlong *)(lVar1 + 8)) {
                  if (*(int *)(lVar1 + 0x10) == iVar6) {
                    if (lVar1 != -0x14) {
                      uVar21 = uVar21 % (ulonglong)*(uint *)(DAT_143ad20e8 + 0x398);
                      puVar16 = *(undefined8 **)(lVar12 + uVar21 * 8);
                      if (puVar16 != (undefined8 *)0x0) {
                        puVar3 = puVar16;
                        puVar15 = (undefined8 *)puVar16[1];
                        if (*(int *)(puVar16 + 2) == iVar6) {
                          *(undefined8 **)(lVar12 + uVar21 * 8) = (undefined8 *)puVar16[1];
LAB_142d9c309:
                          (**(code **)*puVar16)();
                        }
                        else {
                          do {
                            puVar16 = puVar15;
                            puVar13 = puVar3;
                            if (puVar16 == (undefined8 *)0x0) goto LAB_142d9c320;
                            puVar3 = puVar16;
                            puVar15 = (undefined8 *)puVar16[1];
                          } while (*(int *)(puVar16 + 2) != iVar6);
                          puVar13[1] = (undefined8 *)puVar16[1];
                          if (puVar16 != (undefined8 *)0x0) goto LAB_142d9c309;
                        }
                        *(int *)(lVar14 + 0x39c) = *(int *)(lVar14 + 0x39c) + -1;
                      }
                    }
                    break;
                  }
                }
              }
LAB_142d9c320:
              FUN_1424efd70(DAT_143ad20e8);
            }
            lVar14 = DAT_143ad20e8;
            lVar12 = local_60;
            if (DAT_143ad20e8 != 0) {
              lVar12 = *(longlong *)(DAT_143ad20e8 + 0x2e0);
              for (; (lVar12 != 0 && (uVar18 < *(uint *)(lVar12 + -8))); uVar18 = uVar18 + 1) {
                if ((int)uVar18 < 0) {
                  FUN_142e54290(0xbc);
                  lVar12 = *(longlong *)(lVar14 + 0x2e0);
                }
                if (*(int *)(lVar20 + lVar12) == iVar6) {
                  FUN_1424efd70(DAT_143ad20e8);
                  lVar12 = local_60;
                  goto LAB_142d9c3c0;
                }
                lVar20 = lVar20 + 4;
              }
              if ((iVar10 != 0) || (lVar12 = local_60, local_res18 == iVar6)) {
                FUN_1424efe50(DAT_143ad20e8,iVar6,1);
                lVar12 = local_60;
              }
            }
          }
        }
LAB_142d9c3c0:
        plVar2 = (longlong *)plVar17[2];
        if (*(char *)((longlong)plVar2 + 0x19) == '\0') {
          cVar5 = *(char *)(*plVar2 + 0x19);
          plVar17 = plVar2;
          plVar2 = (longlong *)*plVar2;
          while (cVar5 == '\0') {
            cVar5 = *(char *)(*plVar2 + 0x19);
            plVar17 = plVar2;
            plVar2 = (longlong *)*plVar2;
          }
        }
        else {
          cVar5 = *(char *)(plVar17[1] + 0x19);
          plVar4 = (longlong *)plVar17[1];
          plVar2 = plVar17;
          while ((plVar17 = plVar4, cVar5 == '\0' && (plVar2 == (longlong *)plVar17[2]))) {
            cVar5 = *(char *)(plVar17[1] + 0x19);
            plVar4 = (longlong *)plVar17[1];
            plVar2 = plVar17;
          }
        }
        cVar5 = *(char *)((longlong)plVar17 + 0x19);
      }
    }
  }
  return;
code_r0x000142d9c186:
  lVar14 = *(longlong *)(lVar14 + 8);
  if (lVar14 == 0) goto LAB_142d9c3c0;
  goto LAB_142d9c180;
}



//===========================================================
// FUN_142ce53e0 @ 142ce53e0   (493 bytes)
//===========================================================

ulonglong FUN_142ce53e0(longlong param_1,int param_2,undefined4 param_3)

{
  char cVar1;
  int iVar2;
  longlong lVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  uint uVar6;
  uint uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  longlong local_res20;
  
  cVar1 = FUN_141b1f960(DAT_143abea80,1);
  if ((cVar1 == '\0') &&
     ((lVar3 = FUN_14209ee40(), *(longlong *)(lVar3 + 8) == 0 ||
      (cVar1 = FUN_14209f830(), cVar1 == '\0')))) {
    uVar4 = DAT_143aa9d98;
    uVar11 = 0;
    local_res20 = 0;
    uVar5 = uVar11;
    if ((((0 < param_2) && (iVar2 = FUN_1407194c0(DAT_143aa9d98,param_2,&local_res20), iVar2 != 0))
        && (local_res20 != 0)) && (*(int *)(local_res20 + -8) != 0)) {
      while ((local_res20 != 0 && (uVar7 = (uint)uVar5, uVar7 < *(uint *)(local_res20 + -8)))) {
        if ((int)uVar7 < 0) {
          FUN_142e54290(0xbc,uVar5);
        }
        iVar2 = FUN_1407159c0(uVar4);
        if (iVar2 != 0) {
          uVar6 = 0;
          if (local_res20 != 0) {
            uVar6 = *(uint *)(local_res20 + -8);
          }
          if (((int)uVar7 < 0) || (uVar6 <= uVar7)) {
            uVar8 = uVar11;
            if (local_res20 != 0) {
              uVar8 = (ulonglong)*(uint *)(local_res20 + -8);
            }
            FUN_142e54290(0xbc,uVar5,uVar8);
          }
          FUN_141d1fe90(param_1 + 0x2ea0);
        }
        uVar5 = (ulonglong)(uVar7 + 1);
      }
      uVar4 = FUN_1408f6690();
      uVar7 = 0;
      if (local_res20 != 0) {
        uVar7 = *(uint *)(local_res20 + -8);
      }
      uVar8 = uVar11;
      uVar9 = uVar11;
      uVar5 = 1;
      if (0 < (int)uVar7) {
        do {
          uVar6 = 0;
          if (local_res20 != 0) {
            uVar6 = *(uint *)(local_res20 + -8);
          }
          if (uVar6 <= (uint)uVar9) {
            uVar10 = uVar11;
            if (local_res20 != 0) {
              uVar10 = (ulonglong)*(uint *)(local_res20 + -8);
            }
            FUN_142e54290(0xbc,uVar9,uVar10);
          }
          FUN_142ce5f80(param_1,*(undefined4 *)(local_res20 + uVar8 * 4),param_3,uVar4,0);
          uVar8 = uVar8 + 1;
          uVar9 = (ulonglong)((uint)uVar9 + 1);
        } while ((longlong)uVar8 < (longlong)(int)uVar7);
      }
    }
    if (local_res20 != 0) {
      thunk_FUN_140205820(local_res20 + -8,0);
    }
  }
  else {
    uVar5 = 0;
  }
  return uVar5;
}



//===========================================================
// FUN_142d9b200 @ 142d9b200   (2262 bytes)
//===========================================================

void FUN_142d9b200(longlong *param_1,int param_2,int param_3)

{
  longlong *plVar1;
  undefined4 **ppuVar2;
  undefined4 *puVar3;
  char cVar4;
  int iVar5;
  int iVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  int iVar10;
  longlong lVar11;
  undefined1 *puVar12;
  int **ppiVar13;
  undefined8 uVar14;
  int *piVar15;
  undefined4 *puVar16;
  undefined4 *puVar17;
  undefined8 *puVar18;
  uint uVar19;
  undefined4 *puVar20;
  bool bVar21;
  int local_118;
  int *local_108;
  uint local_100;
  int *local_f8;
  int **local_f0;
  undefined8 local_e8;
  longlong local_e0;
  longlong local_d8;
  longlong local_d0;
  longlong local_c8;
  undefined8 local_c0;
  undefined4 *local_b8;
  undefined4 *local_b0;
  longlong local_a8;
  longlong local_a0;
  undefined4 *local_98;
  undefined8 local_90;
  undefined1 local_88 [8];
  longlong local_80;
  undefined1 local_78 [8];
  longlong local_70;
  undefined1 local_68 [8];
  undefined4 *local_60;
  undefined8 local_58;
  undefined8 local_50;
  
  puVar20 = (undefined4 *)0x0;
  lVar11 = FUN_14209ee40();
  if (((*(longlong *)(lVar11 + 8) == 0) || (cVar4 = FUN_14209f830(), cVar4 == '\0')) &&
     (cVar4 = FUN_141b1f960(DAT_143abea80,1), cVar4 == '\0')) {
    bVar21 = DAT_143aa8518 == 0;
    if (bVar21) {
      local_80 = 0;
      puVar12 = local_88;
    }
    else {
      puVar12 = (undefined1 *)FUN_1428f74d0(DAT_143aa8518,local_78);
    }
    lVar11 = local_80;
    local_c0 = *(undefined8 *)(puVar12 + 8);
    if ((bVar21) && (local_80 != 0)) {
      puVar18 = (undefined8 *)(local_80 + -0x28);
      if (0xffffe < *(longlong *)(local_80 + -0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar1 = (longlong *)(lVar11 + -0x20);
      lVar11 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if ((int)lVar11 == 1) {
        if (*(longlong *)(local_80 + -0x10) != 0) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_80 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_80 + -0x10) + 4) != 0);
        }
        if (puVar18 != (undefined8 *)0x0) {
          (**(code **)*puVar18)(puVar18,1);
        }
      }
    }
    lVar11 = local_70;
    if ((!bVar21) && (local_70 != 0)) {
      puVar18 = (undefined8 *)(local_70 + -0x28);
      if (0xffffe < *(longlong *)(local_70 + -0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar1 = (longlong *)(lVar11 + -0x20);
      lVar11 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if ((int)lVar11 == 1) {
        if ((local_70 != 0) && (*(longlong *)(local_70 + -0x10) != 0)) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_70 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_70 + -0x10) + 4) != 0);
        }
        if (puVar18 != (undefined8 *)0x0) {
          (**(code **)*puVar18)(puVar18,1);
        }
      }
      local_70 = 0;
    }
    uVar14 = DAT_143aa9d98;
    local_118 = 0;
    local_e0 = 0;
    local_e8 = DAT_143aa9d98;
    iVar5 = FUN_140719410(DAT_143aa9d98,param_2,&local_e0);
    if (iVar5 != 0) {
      ppiVar13 = (int **)FUN_142cbe730(param_1);
      local_f0 = ppiVar13;
      local_c8 = (**(code **)(*param_1 + 0x30))(param_1);
      local_90 = FUN_142cbec90(param_1);
      puVar16 = puVar20;
      puVar17 = puVar20;
LAB_142d9b40c:
      while ((local_100 = (uint)puVar16, local_98 = puVar17, local_e0 != 0 &&
             (local_100 < *(uint *)(local_e0 + -8)))) {
        if ((int)local_100 < 0) {
          FUN_142e54290(0xbc);
        }
        iVar5 = *(int *)((longlong)puVar17 + local_e0);
        if ((*(longlong *)((longlong)ppiVar13 + 0x1273) != 0) &&
           (lVar11 = *(longlong *)
                      (*(longlong *)((longlong)ppiVar13 + 0x1273) +
                      ((ulonglong)(longlong)iVar5 %
                      (ulonglong)*(uint *)((longlong)ppiVar13 + 0x127b)) * 8), lVar11 != 0)) {
LAB_142d9b471:
          if (*(int *)(lVar11 + 0x10) != iVar5) goto code_r0x000142d9b477;
          lVar11 = FUN_14070fa90(uVar14);
          ppuVar2 = (undefined4 **)(lVar11 + 0xe0);
          local_b0 = (undefined4 *)0x0;
          if (((&local_b0 != ppuVar2) && (*ppuVar2 != (undefined4 *)0x0)) &&
             (uVar19 = (*ppuVar2)[-2], uVar19 != 0)) {
            lVar11 = FUN_14019b780(&DAT_143ad68a0,(ulonglong)uVar19 * 0x20 + 8);
            local_b0 = (undefined4 *)(lVar11 + 8);
            if (lVar11 == 0) {
              local_b0 = puVar20;
            }
            *(ulonglong *)(local_b0 + -2) = (ulonglong)uVar19;
            puVar17 = *ppuVar2;
            puVar16 = puVar17 + (ulonglong)uVar19 * 8;
            puVar3 = local_b0;
            for (; puVar17 < puVar16; puVar17 = puVar17 + 8) {
              uVar7 = puVar17[1];
              uVar8 = puVar17[2];
              uVar9 = puVar17[3];
              *puVar3 = *puVar17;
              puVar3[1] = uVar7;
              puVar3[2] = uVar8;
              puVar3[3] = uVar9;
              uVar14 = *(undefined8 *)(puVar17 + 6);
              *(undefined8 *)(puVar3 + 4) = *(undefined8 *)(puVar17 + 4);
              *(undefined8 *)(puVar3 + 6) = uVar14;
              puVar3 = puVar3 + 8;
            }
          }
          puVar16 = local_b0;
          piVar15 = local_b0 + 4;
          puVar17 = puVar20;
          while ((local_f8 = piVar15, puVar16 != (undefined4 *)0x0 &&
                 (uVar19 = (uint)puVar17, uVar19 < (uint)puVar16[-2]))) {
            if ((int)uVar19 < 0) {
              FUN_142e54290(0xbc);
            }
            if (param_2 == piVar15[-4]) {
              iVar6 = FUN_1403ebc50(local_f0,param_2,0xf);
              uVar7 = FUN_142cb85d0(param_1);
              uVar8 = FUN_142cb85b0(param_1);
              uVar9 = FUN_142cafb20(param_1);
              lVar11 = FUN_14070fa90(local_e8,iVar5);
              ppiVar13 = local_f0;
              iVar10 = FUN_140711d70(local_e8,iVar5,*(undefined4 *)(lVar11 + 0x2c),uVar9,local_f0,
                                     local_c8,local_90,uVar8,uVar7,local_c0);
              bVar21 = iVar10 == 0;
              if (((int)uVar19 < 0) || ((uint)puVar16[-2] <= uVar19)) {
                FUN_142e54290(0xbc);
              }
              piVar15 = local_f8;
              iVar10 = *local_f8;
              if (iVar10 == 0) {
                if (((int)uVar19 < 0) || ((uint)puVar16[-2] <= uVar19)) {
                  FUN_142e54290(0xbc);
                  iVar10 = *piVar15;
                }
                if (param_3 <= iVar10) goto LAB_142d9b680;
LAB_142d9b641:
                if (!bVar21) goto LAB_142d9b6cf;
                local_118 = iVar5;
                if (DAT_143ad20e8 != 0) {
                  FUN_142dc0830();
                  FUN_1424efd70(DAT_143ad20e8);
                  goto LAB_142d9b6cf;
                }
LAB_142d9b6fd:
                if (param_3 != iVar6) goto LAB_142d9b703;
              }
              else {
LAB_142d9b680:
                if (((int)uVar19 < 0) || ((uint)puVar16[-2] <= uVar19)) {
                  FUN_142e54290(0xbc);
                  iVar10 = *piVar15;
                }
                if (param_3 < iVar10) {
                  if (((int)uVar19 < 0) || ((uint)puVar16[-2] <= uVar19)) {
                    FUN_142e54290(0xbc);
                    iVar10 = *piVar15;
                  }
                  if (iVar10 <= iVar6) goto LAB_142d9b641;
                }
LAB_142d9b6cf:
                if (DAT_143ad20e8 == 0) goto LAB_142d9b6fd;
                iVar10 = FUN_142507380();
                if (iVar10 != 0) {
                  FUN_1424efd70(DAT_143ad20e8);
                  goto LAB_142d9b6fd;
                }
                if (iVar6 <= param_3) goto LAB_142d9b6fd;
                if ((!bVar21) || (local_118 == iVar5)) {
                  FUN_1424efe50(DAT_143ad20e8,iVar5,1);
                }
LAB_142d9b703:
                if (((int)uVar19 < 0) || ((uint)puVar16[-2] <= uVar19)) {
                  FUN_142e54290(0xbc);
                }
                if (param_3 < *piVar15) {
                  lVar11 = FUN_1411fe870();
                  if ((lVar11 == 0) || (cVar4 = FUN_1411fe780(lVar11,iVar5), cVar4 == '\0')) {
                    local_108 = (int *)0x0;
                    piVar15 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
                    piVar15[1] = 0;
                    *piVar15 = -1;
                    local_108 = piVar15 + 4;
                    piVar15[2] = 0;
                    *(undefined1 *)local_108 = 0;
                    if (*piVar15 != -1) {
                      FUN_142e52dd0(0x8b);
                    }
                    if (piVar15[1] < 0) {
                      FUN_142e54290(0x90,piVar15[1],0);
                    }
                    *piVar15 = 1;
                    *(undefined1 *)local_108 = 0;
                    if (piVar15[1] + 1 < 1) {
                      FUN_142e54290(0x9c,0);
                    }
                    piVar15[2] = 0;
                    ppiVar13 = &local_108;
                    FUN_142d934f0(param_1,iVar5,param_2,0,ppiVar13);
                    piVar15 = local_f8;
                  }
                  else {
                    local_58 = 0;
                    local_a8 = 0;
                    uVar14 = FUN_14019ba10(&local_a8,&DAT_143274298,iVar6);
                    local_d0 = 0;
                    FUN_14019a260(&local_d0,uVar14);
                    if (local_a8 != 0) {
                      FUN_14019f2c0(local_a8 + -0x10);
                    }
                    local_50 = 0;
                    local_a0 = 0;
                    uVar14 = FUN_14019ba10(&local_a0,&DAT_143274298,param_3);
                    local_d8 = 0;
                    FUN_14019a260(&local_d8,uVar14);
                    if (local_a0 != 0) {
                      FUN_14019f2c0(local_a0 + -0x10);
                    }
                    ppiVar13 = (int **)((ulonglong)ppiVar13 & 0xffffffffffffff00);
                    FUN_142d93610(param_1,iVar5,&local_d8,&local_d0,ppiVar13);
                    if (local_d8 != 0) {
                      FUN_14019f2c0(local_d8 + -0x10);
                    }
                    piVar15 = local_f8;
                    if (local_d0 != 0) {
                      FUN_14019f2c0(local_d0 + -0x10);
                      piVar15 = local_f8;
                    }
                  }
                }
              }
              if ((bVar21) && (local_118 == iVar5)) {
                FUN_142d490d0(param_1,iVar5,0,1,(ulonglong)ppiVar13 & 0xffffffff00000000);
              }
            }
            piVar15 = piVar15 + 8;
            puVar17 = (undefined4 *)(ulonglong)(uVar19 + 1);
          }
          ppiVar13 = local_f0;
          uVar14 = local_e8;
          if (puVar16 != (undefined4 *)0x0) {
            thunk_FUN_140205820(puVar16 + -2);
            ppiVar13 = local_f0;
            uVar14 = local_e8;
          }
        }
        puVar17 = local_98 + 1;
        puVar16 = (undefined4 *)(ulonglong)(local_100 + 1);
      }
      if ((local_118 != 0) && (iVar5 = FUN_1407140b0(uVar14,local_118), iVar5 == 0)) {
        local_c8 = FUN_14019b780(&DAT_143ad68a0,0x370);
        puVar16 = puVar20;
        if (local_c8 != 0) {
          puVar16 = (undefined4 *)FUN_141808b90(local_c8);
        }
        puVar17 = puVar16 + 6;
        if (puVar16 == (undefined4 *)0x0) {
          puVar17 = puVar20;
        }
        if (puVar17 == (undefined4 *)0x0) {
          local_b8 = (undefined4 *)0x0;
        }
        else {
          local_b8 = puVar17 + -6;
          if (local_b8 != (undefined4 *)0x0) {
            if (0xfffff < *(ulonglong *)(puVar17 + 2)) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            *(longlong *)(puVar17 + 2) = *(longlong *)(puVar17 + 2) + 1;
            UNLOCK();
          }
        }
        puVar20 = local_b8;
        if (local_b8 == (undefined4 *)0x0) {
          FUN_142e52ed0(0x431,0);
        }
        FUN_14180d7f0(puVar20,local_118);
        local_60 = puVar20;
        if (puVar20 != (undefined4 *)0x0) {
          if (0xfffff < *(ulonglong *)(puVar20 + 8)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(puVar20 + 8) = *(longlong *)(puVar20 + 8) + 1;
          UNLOCK();
          puVar20 = local_b8;
        }
        FUN_142d97880(param_1,local_68);
        if (puVar20 != (undefined4 *)0x0) {
          if (0xffffe < *(longlong *)(puVar20 + 8) - 1U) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar1 = (longlong *)(puVar20 + 8);
          lVar11 = *plVar1;
          *plVar1 = *plVar1 + -1;
          UNLOCK();
          if (((int)lVar11 == 1) &&
             (puVar18 = (undefined8 *)(local_b8 + 6), puVar18 != (undefined8 *)0x0)) {
            (**(code **)*puVar18)(puVar18,1);
          }
        }
      }
    }
    if (local_e0 != 0) {
      thunk_FUN_140205820(local_e0 + -8,0);
    }
  }
  return;
code_r0x000142d9b477:
  lVar11 = *(longlong *)(lVar11 + 8);
  if (lVar11 == 0) goto code_r0x000142d9b480;
  goto LAB_142d9b471;
code_r0x000142d9b480:
  puVar16 = (undefined4 *)(ulonglong)(local_100 + 1);
  puVar17 = local_98 + 1;
  goto LAB_142d9b40c;
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
// FUN_1428a7f00 @ 1428a7f00   (64 bytes)
//===========================================================

void FUN_1428a7f00(longlong param_1,undefined1 param_2)

{
  int iVar1;
  longlong *plVar2;
  
  iVar1 = FUN_14279c310();
  if (iVar1 == 0) {
    plVar2 = (longlong *)(**(code **)(*(longlong *)(param_1 + 8) + 0x50))(param_1 + 8);
    (**(code **)(*plVar2 + 0x110))(plVar2,param_2);
  }
  return;
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
// FUN_14036f510 @ 14036f510   (172 bytes)
//===========================================================

void FUN_14036f510(void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  void *pvVar4;
  undefined8 *puVar5;
  int *piVar6;
  
  pvVar4 = Self;
  lVar3 = DAT_143aa8598;
  plVar1 = (longlong *)(DAT_143aa8598 + 0x18);
  LOCK();
  lVar2 = *plVar1;
  if (lVar2 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  if (lVar2 == 0) {
LAB_14036f579:
    *(undefined4 *)(lVar3 + 0x20) = 1;
  }
  else if ((void *)*plVar1 == pvVar4) {
    *(int *)(lVar3 + 0x20) = *(int *)(lVar3 + 0x20) + 1;
  }
  else {
    while( true ) {
      pvVar4 = Self;
      LOCK();
      lVar2 = *plVar1;
      if (lVar2 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar2 == 0) goto LAB_14036f579;
      if ((void *)*plVar1 == pvVar4) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar3 + 0x20) = *(int *)(lVar3 + 0x20) + 1;
  }
  piVar6 = (int *)(lVar3 + 0x20);
  puVar5 = *(undefined8 **)(lVar3 + 0x28);
  if (puVar5 == (undefined8 *)0x0) {
    puVar5 = (undefined8 *)FUN_14019d3c0(0x18,0x10);
    *(undefined8 **)(lVar3 + 0x28) = puVar5;
  }
  *(undefined8 *)(lVar3 + 0x28) = *puVar5;
  *piVar6 = *piVar6 + -1;
  if (*piVar6 == 0) {
    *plVar1 = 0;
  }
  return;
}


