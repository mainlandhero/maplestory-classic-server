
//===========================================================
// FUN_142d59e20 @ 142d59e20   (2688 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000142d5a70e) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_142d59e20(longlong param_1,undefined8 param_2)

{
  char cVar1;
  byte bVar2;
  byte bVar3;
  int iVar4;
  int iVar5;
  longlong lVar6;
  int *piVar7;
  undefined8 *puVar8;
  longlong *plVar9;
  int *piVar10;
  int *piVar11;
  int *piVar12;
  undefined8 uVar13;
  int *piVar14;
  ulonglong uVar15;
  uint uVar16;
  int iVar17;
  longlong lVar18;
  ulonglong uVar19;
  uint local_res18;
  ulonglong uVar20;
  undefined8 in_stack_ffffffffffffff00;
  undefined4 uVar21;
  int *local_f8;
  undefined8 local_f0;
  ulonglong local_e8;
  longlong local_e0;
  longlong local_d8;
  longlong local_d0;
  longlong local_c8;
  int *local_c0;
  longlong local_b8;
  longlong local_b0;
  longlong local_a8;
  longlong local_a0;
  longlong local_98;
  longlong local_90;
  longlong local_88;
  int *local_80;
  int *local_78;
  int *local_70;
  int local_68 [2];
  longlong local_60;
  uint local_58;
  int *local_50;
  ulonglong local_48;
  uint local_40;
  undefined4 local_3c;
  
  uVar21 = (undefined4)((ulonglong)in_stack_ffffffffffffff00 >> 0x20);
  cVar1 = FUN_141b1f960(DAT_143abea80,1);
  if (cVar1 != '\0') {
    return;
  }
  lVar6 = FUN_142cbe730(param_1);
  if (lVar6 == 0) {
    return;
  }
  iVar4 = thunk_FUN_1406e8c20(param_2);
  bVar2 = FUN_1406e8ae0(param_2);
  uVar16 = (uint)bVar2;
  piVar7 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
  piVar7[1] = 0;
  uVar15 = 0xffffffffffffffff;
  *piVar7 = -1;
  piVar14 = piVar7 + 4;
  piVar7[2] = 0;
  *(undefined1 *)piVar14 = 0;
  local_c0 = piVar14;
  if (*piVar7 != -1) {
    FUN_142e52dd0();
  }
  if (piVar7[1] < 0) {
    FUN_142e54290(0x90,piVar7[1],0);
  }
  *piVar7 = 1;
  *(undefined1 *)piVar14 = 0;
  if (piVar7[1] + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  piVar7[2] = 0;
  local_f0 = DAT_143495a88;
  local_res18 = 0;
  if (uVar16 == 1) {
    puVar8 = (undefined8 *)FUN_1406e9050(param_2,&local_e8);
    FUN_14019f2c0(piVar7);
    piVar14 = (int *)*puVar8;
    *puVar8 = 0;
    local_c0 = piVar14;
    if (local_e8 != 0) {
      FUN_14019f2c0(local_e8 - 0x10);
    }
  }
  else if (bVar2 == 2) {
    FUN_1406e9170(param_2,&local_f0,8);
  }
  else if (bVar2 == 0) {
    bVar3 = FUN_1406e8ae0(param_2);
    local_res18 = (uint)bVar3;
  }
  plVar9 = (longlong *)FUN_141892840();
  if ((plVar9 == (longlong *)0x0) ||
     (((iVar5 = FUN_14182e410(plVar9), iVar5 == 0 &&
       (iVar5 = FUN_141829f70(plVar9), 299 < iVar5 + 0xc4cfb7a0U)) && (iVar5 != 0x39d22088)))) {
    iVar5 = FUN_142dc5990(iVar4);
    if (iVar5 == 0) {
      lVar6 = FUN_14209ee40();
      if (*(longlong *)(lVar6 + 8) != 0) {
        uVar13 = FUN_14209ee40();
        lVar6 = FUN_140edd8d0(uVar13);
        iVar5 = (**(code **)(*(longlong *)(lVar6 + 8) + 0xd0))
                          ((longlong *)(lVar6 + 8),&PTR_PTR_143a870f8);
        if (iVar5 != 0) goto LAB_142d5a5f1;
      }
      lVar6 = FUN_1410ad270();
      if (lVar6 == 0) {
        local_b8 = 0;
        uVar13 = FUN_14019ba10(&local_b8,"%010d",iVar4);
        local_c8 = 0;
        FUN_14019a260(&local_c8,uVar13);
        if (local_b8 != 0) {
          FUN_14019f2c0(local_b8 + -0x10);
        }
        local_b0 = 0;
        uVar13 = FUN_14019ba10(&local_b0,&DAT_143274298,bVar2);
        local_f8 = (int *)0x0;
        FUN_14019a260(&local_f8,uVar13);
        if (local_b0 != 0) {
          FUN_14019f2c0(local_b0 + -0x10);
        }
        local_a8 = 0;
        uVar13 = FUN_14019ba10(&local_a8,&DAT_143274298,local_f0._4_4_);
        local_e0 = 0;
        FUN_14019a260(&local_e0,uVar13);
        if (local_a8 != 0) {
          FUN_14019f2c0(local_a8 + -0x10);
        }
        local_a0 = 0;
        uVar13 = FUN_14019ba10(&local_a0,&DAT_143274298,local_f0 & 0xffffffff);
        local_d8 = 0;
        FUN_14019a260(&local_d8,uVar13);
        if (local_a0 != 0) {
          FUN_14019f2c0(local_a0 + -0x10);
        }
        piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,0x16);
        piVar10[1] = 5;
        *piVar10 = -1;
        piVar7 = piVar10 + 4;
        piVar10[2] = 0;
        *(undefined1 *)piVar7 = 0;
        *piVar7 = s_false_1434b2c20._0_4_;
        *(char *)(piVar10 + 5) = s_false_1434b2c20[4];
        local_78 = piVar7;
        if (*piVar10 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar10[1] < 5) {
          FUN_142e54290(0x90,piVar10[1],5);
        }
        *piVar10 = 1;
        *(undefined1 *)((longlong)piVar10 + 0x15) = 0;
        if (piVar10[1] + 1 < 6) {
          FUN_142e54290(0x9c,5);
        }
        piVar10[2] = 5;
        if (local_res18 != 0) {
          piVar12 = (int *)FUN_14019b600(&DAT_143ad6a30,0x15);
          piVar12[1] = 4;
          *piVar12 = -1;
          piVar7 = piVar12 + 4;
          piVar12[2] = 0;
          *(undefined1 *)piVar7 = 0;
          *piVar7 = _DAT_1434b2be8;
          local_70 = piVar7;
          if (*piVar12 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if (piVar12[1] < 4) {
            FUN_142e54290(0x90,piVar12[1],4);
          }
          *piVar12 = 1;
          *(undefined1 *)(piVar12 + 5) = 0;
          if (piVar12[1] + 1 < 5) {
            FUN_142e54290(0x9c,4);
          }
          piVar12[2] = 4;
          FUN_14019f2c0(piVar10);
          local_78 = piVar7;
        }
        piVar10 = local_f8;
        FUN_1401abc80(&local_c8,&local_88,local_f8);
        lVar6 = local_e0;
        FUN_1401abc80(&local_88,&local_90,local_e0);
        lVar18 = local_d8;
        FUN_1401abc80(&local_90,&local_98,local_d8);
        FUN_1401abc80(&local_98,&local_d0,piVar7);
        if (local_98 != 0) {
          FUN_14019f2c0(local_98 + -0x10);
        }
        if (local_90 != 0) {
          FUN_14019f2c0(local_90 + -0x10);
        }
        if (local_88 != 0) {
          FUN_14019f2c0(local_88 + -0x10);
        }
        FUN_142dbc730(local_68);
        if (local_50 == (int *)0x0) {
          iVar5 = 0;
          piVar12 = (int *)0x0;
        }
        else {
          iVar5 = local_50[-2];
          piVar12 = local_50;
        }
        if (piVar14 == (int *)0x0) {
          iVar17 = 0;
          piVar11 = (int *)0x0;
        }
        else {
          iVar17 = piVar14[-2];
          piVar11 = piVar14;
        }
        local_80 = piVar11;
        local_68[0] = iVar4;
        local_58 = uVar16;
        if (((iVar5 == iVar17) && (iVar5 != 0)) && (piVar12 != (int *)0x0)) {
          if (piVar11 == (int *)0x0) goto LAB_142d5a507;
          iVar4 = memcmp(piVar12,piVar11,(longlong)iVar5);
          piVar12 = local_50;
          if (iVar4 != 0) goto LAB_142d5a37d;
        }
        else {
LAB_142d5a37d:
          if ((piVar11 == (int *)0x0) || (piVar11 + -4 == (int *)0x0)) {
LAB_142d5a507:
            piVar12 = local_50;
            if (local_50 != (int *)0x0) {
              FUN_14019f2c0(local_50 + -4);
              local_50 = (int *)0x0;
              piVar12 = local_50;
            }
          }
          else {
            iVar4 = piVar11[-4];
            if (iVar4 != -1) {
              if (iVar4 < 1) {
                FUN_142e52dd0(0xd2);
              }
              LOCK();
              piVar11[-4] = piVar11[-4] + 1;
              UNLOCK();
              piVar7 = local_78;
              piVar10 = local_f8;
              piVar14 = local_c0;
              lVar18 = local_d8;
              lVar6 = local_e0;
              piVar12 = piVar11;
              if (local_50 != (int *)0x0) {
                FUN_14019f2c0(local_50 + -4);
                piVar7 = local_78;
                piVar10 = local_f8;
                piVar14 = local_c0;
                lVar18 = local_d8;
                lVar6 = local_e0;
              }
              goto LAB_142d5a51d;
            }
            FUN_142e52d50(0xcb,0xffffff01);
            local_e8 = 0xffffffffffffffff;
            do {
              local_e8 = local_e8 + 1;
            } while (*(char *)(local_e8 + (longlong)piVar11) != '\0');
            iVar4 = 0;
            if (0 < (int)local_e8) {
              iVar4 = (int)local_e8;
            }
            piVar11 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
            uVar20 = local_e8;
            piVar11[1] = iVar4;
            *piVar11 = -1;
            piVar12 = piVar11 + 4;
            piVar11[2] = 0;
            *(undefined1 *)piVar12 = 0;
            iVar4 = (int)local_e8;
            local_e8 = (ulonglong)iVar4;
            FUN_142ef7ba0(piVar12,local_80,local_e8);
            if (*piVar11 != -1) {
              FUN_142e52dd0(0x8b);
            }
            if ((iVar4 == -1) || (iVar4 <= piVar11[1])) {
              *piVar11 = 1;
              if (iVar4 != -1) goto LAB_142d5a44d;
              if (piVar12 == (int *)0x0) {
                iVar4 = 0;
              }
              else {
                do {
                  uVar15 = uVar15 + 1;
                } while (*(char *)((longlong)piVar12 + uVar15) != '\0');
                iVar4 = (int)uVar15;
              }
            }
            else {
              FUN_142e54290(0x90,piVar11[1],uVar20 & 0xffffffff);
              *piVar11 = 1;
LAB_142d5a44d:
              *(undefined1 *)(local_e8 + (longlong)piVar12) = 0;
            }
            if ((iVar4 < 0) || (piVar11[1] + 1 <= iVar4)) {
              FUN_142e54290(0x9c,iVar4);
            }
            piVar11[2] = iVar4;
            if (local_50 != (int *)0x0) {
              FUN_14019f2c0(local_50 + -4);
            }
          }
        }
LAB_142d5a51d:
        local_50 = piVar12;
        local_48 = local_f0;
        local_40 = local_res18;
        local_3c = 0;
        FUN_142dc1000(param_1 + 0x3770,&local_d0,local_68);
        uVar13 = FUN_140369710(param_1 + 0x3788);
        FUN_14019a260(uVar13,&local_d0);
        if (local_50 != (int *)0x0) {
          FUN_14019f2c0(local_50 + -4);
        }
        if (local_60 != 0) {
          FUN_14019f2c0(local_60 + -0x10);
        }
        if (local_d0 != 0) {
          FUN_14019f2c0(local_d0 + -0x10);
        }
        if (piVar7 != (int *)0x0) {
          FUN_14019f2c0(piVar7 + -4);
        }
        if (lVar18 != 0) {
          FUN_14019f2c0(lVar18 + -0x10);
        }
        if (lVar6 != 0) {
          FUN_14019f2c0(lVar6 + -0x10);
        }
        if (piVar10 != (int *)0x0) {
          FUN_14019f2c0(piVar10 + -4);
        }
        if (local_c8 != 0) {
          FUN_14019f2c0(local_c8 + -0x10);
        }
        goto LAB_142d5a876;
      }
    }
LAB_142d5a5f1:
    FUN_142dad250(DAT_143aa84a0);
    if (plVar9 != (longlong *)0x0) goto LAB_142d5a602;
  }
  else {
    FUN_142dad250(DAT_143aa84a0);
LAB_142d5a602:
    (**(code **)(*plVar9 + 0x60))(plVar9);
  }
  uVar20 = local_f0;
  local_f8 = (int *)0x0;
  piVar10 = piVar14;
  piVar7 = local_f8;
  if ((piVar14 != (int *)0x0) && (piVar12 = piVar14 + -4, piVar12 != (int *)0x0)) {
    if (*piVar12 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar19 = 0xffffffffffffffff;
      do {
        uVar19 = uVar19 + 1;
      } while (*(char *)((longlong)piVar14 + uVar19) != '\0');
      iVar17 = (int)uVar19;
      iVar5 = 0;
      if (0 < iVar17) {
        iVar5 = iVar17;
      }
      piVar12 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar5 + 0x11));
      piVar12[1] = iVar5;
      *piVar12 = -1;
      piVar7 = piVar12 + 4;
      piVar12[2] = 0;
      *(undefined1 *)piVar7 = 0;
      local_70 = piVar7;
      FUN_142ef7ba0(piVar7,piVar14,(longlong)iVar17);
      if (*piVar12 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar17 == -1) || (iVar17 <= piVar12[1])) {
        *piVar12 = 1;
        if (iVar17 != -1) goto LAB_142d5a6c7;
        if (piVar7 == (int *)0x0) {
          uVar19 = 0;
        }
        else {
          do {
            uVar15 = uVar15 + 1;
          } while (*(char *)((longlong)piVar7 + uVar15) != '\0');
          uVar19 = uVar15 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar12[1],uVar19 & 0xffffffff);
        *piVar12 = 1;
LAB_142d5a6c7:
        *(undefined1 *)((longlong)iVar17 + (longlong)piVar7) = 0;
      }
      iVar5 = (int)uVar19;
      if ((iVar5 < 0) || (piVar12[1] + 1 <= iVar5)) {
        FUN_142e54290(0x9c,uVar19 & 0xffffffff);
      }
      piVar12[2] = iVar5;
      if (local_f8 != (int *)0x0) {
        FUN_14019f2c0();
      }
    }
    else {
      if (*piVar12 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar12 = *piVar12 + 1;
      UNLOCK();
      piVar10 = local_c0;
      piVar7 = piVar14;
      if (local_f8 != (int *)0x0) {
        FUN_14019f2c0(local_f8 + -4);
        piVar10 = local_c0;
      }
    }
  }
  local_f8 = piVar7;
  piVar14 = piVar10;
  FUN_142d5b750(param_1,bVar2,iVar4,&local_f8,uVar20,CONCAT44(uVar21,local_res18));
  FUN_142ce6750(param_1,iVar4);
  uVar13 = FUN_1408f6690();
  uVar20 = uVar20 & 0xffffffff00000000;
  FUN_142ce5f80(param_1,iVar4,0,uVar13,uVar20);
  FUN_142ce6a00(param_1,iVar4);
  FUN_1413f19c0(iVar4);
  iVar5 = FUN_142dc5b20(iVar4);
  if (iVar5 != 0) goto LAB_142d5a876;
  if ((iVar4 == 0x4222) && (DAT_143aa8518 != 0)) {
    uVar20 = uVar20 & 0xffffffff00000000;
    FUN_1428f4eb0(DAT_143aa8518,0xc71,1,0,uVar20);
    FUN_142ce5e60(param_1);
LAB_142d5a842:
    FUN_142cbefd0(param_1,0,0,0,uVar20 & 0xffffffff00000000,0);
  }
  else {
    FUN_142ce5e60(param_1);
    if ((iVar4 != 0x413c) && (((iVar4 != 0x188b4 && (iVar4 != 0x7a687)) && (iVar4 != 0x7a714))))
    goto LAB_142d5a842;
  }
  FUN_142d5a8b0(param_1,iVar4,bVar2);
  FUN_142dc54e0();
  FUN_14112c630(0);
LAB_142d5a876:
  if (piVar14 != (int *)0x0) {
    FUN_14019f2c0(piVar14 + -4);
  }
  return;
}



//===========================================================
// FUN_142dad250 @ 142dad250   (1935 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000142dad7a0) */

void FUN_142dad250(longlong param_1)

{
  char *pcVar1;
  char cVar2;
  char *_Buf1;
  bool bVar3;
  bool bVar4;
  bool bVar5;
  bool bVar6;
  int iVar7;
  undefined8 uVar8;
  int *piVar9;
  int *piVar10;
  uint uVar11;
  longlong lVar12;
  char *_Buf2;
  longlong lVar13;
  int *piVar14;
  longlong lVar15;
  int iVar16;
  ulonglong uVar17;
  ulonglong in_stack_ffffffffffffff58;
  undefined8 in_stack_ffffffffffffff60;
  undefined4 uVar18;
  int *local_98;
  char *local_90;
  int *local_88;
  longlong local_80;
  int local_78;
  int *local_70;
  undefined4 local_68;
  int *local_60;
  ulonglong local_58;
  undefined8 local_50;
  
  if (*(int *)(param_1 + 0x377c) != 0) {
    bVar3 = false;
    bVar5 = false;
    bVar6 = false;
    bVar4 = false;
    uVar8 = FUN_1408f6690();
    lVar12 = *(longlong *)(param_1 + 0x3790);
    if (lVar12 != 0) {
      do {
        uVar17 = *(ulonglong *)(lVar12 + -0x20);
        if ((uVar17 != 0) && (uVar17 < 0x10001)) {
          FUN_142e52ed0(0x33e);
          uVar17 = *(ulonglong *)(lVar12 + -0x20);
        }
        lVar15 = 0;
        if (uVar17 != 0) {
          lVar15 = uVar17 + 0x28;
        }
        local_90 = (char *)0x0;
        local_80 = lVar15;
        FUN_14019a260(&local_90,lVar12);
        local_78 = 0;
        local_70 = (int *)0x0;
        piVar9 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
        piVar9[1] = 0;
        *piVar9 = -1;
        local_70 = piVar9 + 4;
        piVar9[2] = 0;
        *(undefined1 *)local_70 = 0;
        if (*piVar9 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar9[1] < 0) {
          FUN_142e54290(0x90,piVar9[1],0);
        }
        *piVar9 = 1;
        *(undefined1 *)local_70 = 0;
        if (piVar9[1] + 1 < 1) {
          FUN_142e54290(0x9c,0);
        }
        piVar9[2] = 0;
        local_68 = 0;
        local_60 = (int *)0x0;
        piVar9 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
        piVar9[1] = 0;
        *piVar9 = -1;
        local_60 = piVar9 + 4;
        piVar9[2] = 0;
        *(undefined1 *)local_60 = 0;
        if (*piVar9 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar9[1] < 0) {
          FUN_142e54290(0x90,piVar9[1],0);
        }
        *piVar9 = 1;
        *(undefined1 *)local_60 = 0;
        if (piVar9[1] + 1 < 1) {
          FUN_142e54290(0x9c,0);
        }
        _Buf2 = local_90;
        piVar9[2] = 0;
        local_58 = 0;
        local_50 = 0;
        lVar12 = lVar15;
        if (*(longlong *)(param_1 + 0x3770) != 0) {
          uVar11 = 0x1505;
          if (local_90 != (char *)0x0) {
            cVar2 = *local_90;
            pcVar1 = local_90;
            while (cVar2 != '\0') {
              uVar11 = uVar11 * 0x21 + (int)cVar2;
              pcVar1 = pcVar1 + 1;
              cVar2 = *pcVar1;
            }
          }
          lVar13 = *(longlong *)
                    (*(longlong *)(param_1 + 0x3770) +
                    ((ulonglong)uVar11 % (ulonglong)*(uint *)(param_1 + 0x3778)) * 8);
          if (lVar13 != 0) {
            pcVar1 = local_90 + -0x10;
            do {
              _Buf1 = *(char **)(lVar13 + 0x10);
              if (_Buf1 == _Buf2) {
LAB_142dad5f5:
                local_78 = *(int *)(lVar13 + 0x18);
                FUN_14019a260(&local_70,lVar13 + 0x20);
                local_68 = *(undefined4 *)(lVar13 + 0x28);
                FUN_14019a260(&local_60,lVar13 + 0x30);
                local_58 = *(ulonglong *)(lVar13 + 0x38);
                local_50 = *(undefined8 *)(lVar13 + 0x40);
                FUN_142dc2500(param_1 + 0x3770,&local_90);
                piVar9 = local_60;
                uVar18 = (undefined4)((ulonglong)in_stack_ffffffffffffff60 >> 0x20);
                if (local_50._4_4_ == 0) {
                  bVar5 = true;
                  if (((local_78 != 0x413c) && (local_78 != 0x188b4)) &&
                     ((local_78 != 0x7a687 && (bVar4 = bVar3, local_78 != 0x7a714)))) {
                    bVar4 = true;
                    bVar3 = bVar4;
                  }
                  local_98 = (int *)0x0;
                  piVar14 = local_98;
                  if ((local_60 != (int *)0x0) && (piVar10 = local_60 + -4, piVar10 != (int *)0x0))
                  {
                    if (*piVar10 == -1) {
                      FUN_142e52d50(0xcb,1);
                      piVar9 = local_60;
                      piVar14 = (int *)0x0;
                      local_88 = (int *)0x0;
                      if (local_60 != (int *)0x0) {
                        uVar17 = 0xffffffffffffffff;
                        do {
                          uVar17 = uVar17 + 1;
                        } while (*(char *)((longlong)local_60 + uVar17) != '\0');
                        iVar16 = (int)uVar17;
                        iVar7 = 0;
                        if (0 < iVar16) {
                          iVar7 = iVar16;
                        }
                        piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar7 + 0x11));
                        piVar10[1] = iVar7;
                        *piVar10 = -1;
                        piVar14 = piVar10 + 4;
                        piVar10[2] = 0;
                        *(undefined1 *)piVar14 = 0;
                        local_88 = piVar14;
                        FUN_142ef7ba0(piVar14,piVar9,(longlong)iVar16);
                        if (*piVar10 != -1) {
                          FUN_142e52dd0(0x8b);
                        }
                        if ((iVar16 == -1) || (iVar16 <= piVar10[1])) {
                          *piVar10 = 1;
                          if (iVar16 != -1) goto LAB_142dad753;
                          if (piVar14 == (int *)0x0) {
                            uVar17 = 0;
                          }
                          else {
                            uVar17 = 0xffffffffffffffff;
                            do {
                              uVar17 = uVar17 + 1;
                            } while (*(char *)((longlong)piVar14 + uVar17) != '\0');
                          }
                        }
                        else {
                          FUN_142e54290(0x90,piVar10[1],uVar17 & 0xffffffff);
                          *piVar10 = 1;
LAB_142dad753:
                          *(undefined1 *)((longlong)piVar14 + (longlong)iVar16) = 0;
                        }
                        iVar7 = (int)uVar17;
                        if ((iVar7 < 0) || (piVar10[1] + 1 <= iVar7)) {
                          FUN_142e54290(0x9c,uVar17 & 0xffffffff);
                        }
                        piVar10[2] = iVar7;
                        lVar15 = local_80;
                      }
                      bVar3 = bVar4;
                      if (local_98 != (int *)0x0) {
                        FUN_14019f2c0();
                      }
                    }
                    else {
                      if (*piVar10 < 1) {
                        FUN_142e52dd0(0xd2);
                      }
                      LOCK();
                      *piVar10 = *piVar10 + 1;
                      UNLOCK();
                      if (local_98 != (int *)0x0) {
                        FUN_14019f2c0(local_98 + -4);
                      }
                      local_98 = piVar9;
                      _Buf2 = local_90;
                      piVar14 = local_98;
                    }
                  }
                  local_98 = piVar14;
                  in_stack_ffffffffffffff60 = CONCAT44(uVar18,(undefined4)local_50);
                  in_stack_ffffffffffffff58 = local_58;
                  FUN_142d5b750(param_1,local_68,local_78,&local_98,local_58,
                                in_stack_ffffffffffffff60);
                  FUN_142ce6750(param_1,local_78);
                  in_stack_ffffffffffffff58 = in_stack_ffffffffffffff58 & 0xffffffff00000000;
                  FUN_142ce5f80(param_1,local_78,0,uVar8,in_stack_ffffffffffffff58);
                  FUN_142ce6a00(param_1,local_78);
                  FUN_1413f19c0(local_78);
                  iVar7 = FUN_142dc5b20(local_78);
                  if (iVar7 == 0) {
                    FUN_142d5a8b0(param_1,local_78);
LAB_142dad9aa:
                    if (local_60 != (int *)0x0) {
                      FUN_14019f2c0(local_60 + -4);
                    }
                    if (local_70 != (int *)0x0) {
                      FUN_14019f2c0(local_70 + -4);
                    }
                    lVar12 = lVar15;
                    if (_Buf2 != (char *)0x0) {
                      FUN_14019f2c0(pcVar1);
                    }
                    goto LAB_142dad4ff;
                  }
                  if (local_60 != (int *)0x0) {
                    FUN_14019f2c0(local_60 + -4);
                  }
                  if (local_70 != (int *)0x0) {
                    FUN_14019f2c0(local_70 + -4);
                  }
                  lVar12 = lVar15;
                  if (_Buf2 != (char *)0x0) {
                    FUN_14019f2c0(pcVar1);
                  }
                }
                else {
                  bVar6 = true;
                  if ((((local_78 != 0x413c) && (local_78 != 0x188b4)) && (local_78 != 0x7a687)) &&
                     (bVar4 = bVar3, local_78 != 0x7a714)) {
                    bVar3 = true;
                    bVar4 = bVar3;
                  }
                  FUN_142d5aa10(param_1);
                  if (DAT_143ace8a8 != 0) {
                    FUN_141f0d620(DAT_143ace8a8,local_78);
                  }
                  FUN_1413f0ce0(local_78);
                  FUN_142d5c630(param_1,local_78);
                  iVar7 = FUN_142dc5b20(local_78);
                  if (iVar7 == 0) {
                    if ((((local_78 == 0x401e) || (local_78 == 0x7a2b7)) || (local_78 == 0x7a2b8))
                       && (DAT_143aa8518 != 0)) {
                      in_stack_ffffffffffffff58 = in_stack_ffffffffffffff58 & 0xffffffff00000000;
                      FUN_1428f4eb0(DAT_143aa8518,0xe5a,0,0,in_stack_ffffffffffffff58);
                    }
                    goto LAB_142dad9aa;
                  }
                  if (local_60 != (int *)0x0) {
                    FUN_14019f2c0(local_60 + -4);
                  }
                  if (local_70 != (int *)0x0) {
                    FUN_14019f2c0(local_70 + -4);
                  }
                  if (_Buf2 != (char *)0x0) {
                    FUN_14019f2c0(pcVar1);
                  }
                }
                goto LAB_142dad4ff;
              }
              if (_Buf1 == (char *)0x0) {
                iVar7 = 0;
              }
              else {
                iVar7 = *(int *)(_Buf1 + -8);
              }
              if (_Buf2 == (char *)0x0) {
                iVar16 = 0;
              }
              else {
                iVar16 = *(int *)(_Buf2 + -8);
              }
              if (iVar7 == iVar16) {
                if (iVar7 != 0) {
                  if (_Buf1 == (char *)0x0) {
                    iVar7 = 0;
                  }
                  else {
                    iVar7 = *(int *)(_Buf1 + -8);
                  }
                  iVar7 = memcmp(_Buf1,_Buf2,(longlong)iVar7);
                  if (iVar7 != 0) goto LAB_142dad4c1;
                }
                goto LAB_142dad5f5;
              }
LAB_142dad4c1:
              lVar13 = *(longlong *)(lVar13 + 8);
            } while (lVar13 != 0);
          }
        }
        if (local_60 != (int *)0x0) {
          FUN_14019f2c0(local_60 + -4);
        }
        if (local_70 != (int *)0x0) {
          FUN_14019f2c0(local_70 + -4);
        }
        if (_Buf2 != (char *)0x0) {
          FUN_14019f2c0(_Buf2 + -0x10);
        }
LAB_142dad4ff:
      } while (lVar12 != 0);
      if (bVar3) {
        FUN_142cbefd0(param_1,0,0,0,in_stack_ffffffffffffff58 & 0xffffffff00000000,0);
      }
      if (bVar5) {
        FUN_142ce5e60(param_1);
        FUN_142dc54e0();
        FUN_14112c630(0);
      }
      if (bVar6) {
        if ((DAT_143aa84a0 == 0) ||
           ((iVar7 = FUN_142cb8550(), iVar7 == 0 &&
            ((DAT_143aa84a0 == 0 ||
             ((iVar7 = FUN_142cb8570(), iVar7 == 0 &&
              ((DAT_143aa84a0 == 0 || (iVar7 = FUN_142cb8580(), iVar7 == 0)))))))))) {
          FUN_142dc5400();
        }
        FUN_14112c630(1);
        if (DAT_143ad00e8 != 0) {
          FUN_142221c80();
        }
        if (*(int *)(param_1 + 0x348c) != 0) {
          *(undefined4 *)(param_1 + 0x348c) = 0;
          *(undefined1 *)(param_1 + 0x3490) = 0;
        }
      }
    }
    FUN_142d33150(param_1 + 0x3770);
  }
  FUN_140369e40(param_1 + 0x3788);
  return;
}



//===========================================================
// FUN_142dc5990 @ 142dc5990   (368 bytes)
//===========================================================

undefined8 FUN_142dc5990(uint param_1)

{
  uint uVar1;
  char cVar2;
  
  if (((0x11 < param_1 - 0x4169) &&
      (((int)param_1 < 1 ||
       ((((uVar1 = param_1 / 100, uVar1 != 0x147 && (uVar1 != 0x148)) && (uVar1 != 0x14a)) &&
        ((param_1 != 0x1964 && (param_1 != 0x1ecc)))))))) &&
     (cVar2 = FUN_14108dbc0(), cVar2 == '\0')) {
    if ((int)param_1 < 0x188d2) {
      if (param_1 == 0x188d1) {
        return 1;
      }
      if ((int)param_1 < 0x69a1) {
        if (param_1 == 0x69a0) {
          return 1;
        }
        if ((int)param_1 < 0x1e1c) {
          if (param_1 == 0x1e1b) {
            return 1;
          }
          if (param_1 == 0x60b) {
            return 1;
          }
          if (param_1 == 0x1c7d) {
            return 1;
          }
          if (param_1 == 0x1c7e) {
            return 1;
          }
        }
        else {
          if (param_1 == 0x4a4b) {
            return 1;
          }
          if (param_1 == 0x550a) {
            return 1;
          }
          if (param_1 == 0x699f) {
            return 1;
          }
        }
      }
      else if ((int)param_1 < 0xa411) {
        if (param_1 == 42000) {
          return 1;
        }
        if (param_1 == 0x69a3) {
          return 1;
        }
        if (param_1 == 0x81bd) {
          return 1;
        }
        if (param_1 == 0x81be) {
          return 1;
        }
      }
      else {
        if (param_1 == 0x187cd) {
          return 1;
        }
        if (param_1 == 0x188b4) {
          return 1;
        }
      }
    }
    else if ((int)param_1 < 0x7a715) {
      if (param_1 == 0x7a714) {
        return 1;
      }
      if ((int)param_1 < 0x18adb) {
        if (param_1 == 0x18ada) {
          return 1;
        }
        if (param_1 == 0x18a6c) {
          return 1;
        }
        if (param_1 == 0x18ad8) {
          return 1;
        }
        if (param_1 == 0x18ad9) {
          return 1;
        }
      }
      else {
        if (param_1 == 0x7a535) {
          return 1;
        }
        if (param_1 == 0x7a687) {
          return 1;
        }
      }
    }
    else {
      switch(param_1) {
      case 0x7a8aa:
      case 0x7a8ab:
      case 0x7a8ac:
      case 0x7a8ad:
      case 0x7a8ae:
      case 0x7a8af:
        goto switchD_142dc5afe_caseD_7a8aa;
      }
    }
    if (((param_1 != 0x4222) && (param_1 != 0x7a2b7)) && (param_1 != 0x7a2b8)) {
      return 0;
    }
  }
switchD_142dc5afe_caseD_7a8aa:
  return 1;
}



//===========================================================
// FUN_140edd8d0 @ 140edd8d0   (40 bytes)
//===========================================================

longlong FUN_140edd8d0(longlong param_1)

{
  longlong lVar1;
  
  lVar1 = *(longlong *)(param_1 + 8);
  if (lVar1 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar1 = *(longlong *)(param_1 + 8);
  }
  return lVar1;
}



//===========================================================
// FUN_1410ad270 @ 1410ad270   (107 bytes)
//===========================================================

longlong FUN_1410ad270(void)

{
  int iVar1;
  longlong lVar2;
  longlong lVar3;
  
  lVar2 = FUN_14209ee40();
  lVar2 = *(longlong *)(lVar2 + 8);
  if (lVar2 != 0) {
    iVar1 = (**(code **)(*(longlong *)(lVar2 + 8) + 0xd0))(lVar2 + 8,&PTR_PTR_143a86b90);
    if (iVar1 != 0) {
      iVar1 = (**(code **)(*(longlong *)(lVar2 + 8) + 0xd0))(lVar2 + 8,&PTR_PTR_143a86b90);
      lVar3 = 0;
      if (iVar1 != 0) {
        lVar3 = lVar2;
      }
      return lVar3;
    }
  }
  return 0;
}



//===========================================================
// FUN_141892840 @ 141892840   (66 bytes)
//===========================================================

longlong FUN_141892840(void)

{
  int iVar1;
  longlong lVar2;
  
  lVar2 = FUN_14209ee40();
  lVar2 = *(longlong *)(lVar2 + 8);
  if (lVar2 != 0) {
    iVar1 = (**(code **)(*(longlong *)(lVar2 + 8) + 0xd0))(lVar2 + 8,&PTR_PTR_143a87ec8);
    if (iVar1 != 0) {
      return lVar2;
    }
  }
  return 0;
}


