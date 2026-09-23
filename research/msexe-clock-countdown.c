
//===========================================================
// FUN_1418564d0 @ 1418564d0   (2764 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x00014185695c) */
/* WARNING: Removing unreachable block (ram,0x000141856881) */
/* WARNING: Removing unreachable block (ram,0x000141856cfd) */

void FUN_1418564d0(longlong *param_1,undefined8 param_2)

{
  int *piVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined1 uVar4;
  undefined1 uVar5;
  undefined1 uVar6;
  char cVar7;
  uint uVar8;
  undefined4 uVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int *piVar14;
  undefined8 *puVar15;
  int *piVar16;
  undefined8 uVar17;
  int *piVar18;
  ulonglong uVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  int local_res8;
  undefined4 local_resc;
  int *local_res18;
  int *local_res20;
  int *local_a8 [2];
  undefined1 local_98 [24];
  int *local_80;
  int *local_78;
  longlong local_70;
  int local_60;
  int local_5c;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_48;
  undefined1 local_44;
  
  if (*(char *)(param_1[0x15] + 0x30c) != '\0') {
    return;
  }
  uVar4 = FUN_1406e8ae0(param_2);
  uVar17 = DAT_143aa84a0;
  switch(uVar4) {
  case 0:
    uVar8 = FUN_1406e8c20(param_2);
    if ((uVar8 != 0) &&
       (FUN_142d98870(uVar17,(uVar8 ^ (int)uVar8 >> 0x1f) - ((int)uVar8 >> 0x1f)), 0 < (int)uVar8))
    {
      return;
    }
    FUN_142d989d0(uVar17);
    return;
  case 1:
    uVar4 = FUN_1406e8ae0(param_2);
    uVar5 = FUN_1406e8ae0(param_2);
    uVar6 = FUN_1406e8ae0(param_2);
    uVar17 = FUN_1418a7dd0(param_1[0x15] + 0x220);
    FUN_1415ea5c0(uVar17,uVar4,uVar5,uVar6);
    return;
  case 2:
    FUN_1415e7c00(local_98);
    (**(code **)(*param_1 + 0x1e0))(param_1,local_98,&local_res18,&local_res8);
    uVar9 = FUN_1406e8c20(param_2);
    FUN_141839a60(param_1,(ulonglong)local_res18 & 0xffffffff,local_res8,local_98,uVar9,local_54,
                  local_50);
    if (local_70 != 0) {
      FUN_14019f2c0(local_70 + -0x10);
    }
    if (local_78 != (int *)0x0) {
      FUN_14019f2c0(local_78 + -4);
    }
    break;
  default:
    goto switchD_14185652b_caseD_3;
  case 4:
    local_res8 = FUN_1406e8c20(param_2);
    local_resc = FUN_1406e8c20(param_2);
    if ((DAT_143ace3d8 != (longlong *)0x0) &&
       ((**(code **)(*DAT_143ace3d8 + 0x168))(), DAT_143ace3d8 != (longlong *)0x0)) {
      (**(code **)DAT_143ace3d8[1])(DAT_143ace3d8 + 1,1);
    }
    if (local_res8 == -1) {
      return;
    }
    if ((DAT_143ace3d8 != (longlong *)0x0) &&
       ((**(code **)(*DAT_143ace3d8 + 0x168))(), DAT_143ace3d8 != (longlong *)0x0)) {
      (**(code **)DAT_143ace3d8[1])(DAT_143ace3d8 + 1,1);
    }
    local_res18 = (int *)FUN_14019b780(&DAT_143ad68a0,0x310);
    if (local_res18 == (int *)0x0) {
      return;
    }
    FUN_1415eb7b0(local_res18,&local_res8);
    return;
  case 5:
    cVar7 = FUN_1406e8ae0(param_2);
    FUN_1415e7c00(local_98);
    (**(code **)(*param_1 + 0x1e0))(param_1,local_98,&local_res18,&local_res8);
    uVar9 = FUN_1406e8c20(param_2);
    FUN_141839a60(param_1,(ulonglong)local_res18 & 0xffffffff,local_res8,local_98,uVar9,local_54,
                  local_50);
    uVar17 = FUN_1418a7dd0(param_1[0x15] + 0x220);
    FUN_1415ea490(uVar17,cVar7 != '\0');
    if (local_70 != 0) {
      FUN_14019f2c0(local_70 + -0x10);
    }
    if (local_78 != (int *)0x0) {
      FUN_14019f2c0(local_78 + -4);
    }
    break;
  case 6:
    FUN_1415e7c00(local_98);
    (**(code **)(*param_1 + 0x1e0))(param_1,local_98,&local_res18,&local_res8);
    local_48 = 1;
    piVar14 = (int *)FUN_14019b600(&DAT_143ad6a30,0x30);
    piVar14[1] = 0x1f;
    *piVar14 = -1;
    piVar1 = piVar14 + 4;
    piVar14[2] = 0;
    *(undefined1 *)piVar1 = 0;
    uVar3 = s_Map_Obj_etc_img_timer3_backgrnd_1433dc458._12_4_;
    uVar2 = s_Map_Obj_etc_img_timer3_backgrnd_1433dc458._8_4_;
    uVar9 = s_Map_Obj_etc_img_timer3_backgrnd_1433dc458._4_4_;
    *piVar1 = s_Map_Obj_etc_img_timer3_backgrnd_1433dc458._0_4_;
    piVar14[5] = uVar9;
    piVar14[6] = uVar2;
    piVar14[7] = uVar3;
    *(undefined8 *)(piVar14 + 8) = s_Map_Obj_etc_img_timer3_backgrnd_1433dc458._16_8_;
    piVar14[10] = s_Map_Obj_etc_img_timer3_backgrnd_1433dc458._24_4_;
    *(undefined2 *)(piVar14 + 0xb) = s_Map_Obj_etc_img_timer3_backgrnd_1433dc458._28_2_;
    *(char *)((longlong)piVar14 + 0x2e) = s_Map_Obj_etc_img_timer3_backgrnd_1433dc458[0x1e];
    local_res20 = piVar1;
    if (*piVar14 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar14[1] < 0x1f) {
      FUN_142e54290(0x90,piVar14[1],0x1f);
    }
    *piVar14 = 1;
    *(undefined1 *)((longlong)piVar14 + 0x2f) = 0;
    if (piVar14[1] + 1 < 0x20) {
      FUN_142e54290(0x9c);
    }
    piVar14[2] = 0x1f;
    if (local_80 != (int *)0x0) {
      FUN_14019f2c0(local_80 + -4);
    }
    local_80 = piVar1;
    piVar14 = (int *)FUN_14019b600(&DAT_143ad6a30,0x30);
    piVar14[1] = 0x1f;
    *piVar14 = -1;
    piVar1 = piVar14 + 4;
    piVar14[2] = 0;
    *(undefined1 *)piVar1 = 0;
    uVar3 = s_Map_Obj_etc_img_timer3_fontTime_1433dc478._12_4_;
    uVar2 = s_Map_Obj_etc_img_timer3_fontTime_1433dc478._8_4_;
    uVar9 = s_Map_Obj_etc_img_timer3_fontTime_1433dc478._4_4_;
    *piVar1 = s_Map_Obj_etc_img_timer3_fontTime_1433dc478._0_4_;
    piVar14[5] = uVar9;
    piVar14[6] = uVar2;
    piVar14[7] = uVar3;
    *(undefined8 *)(piVar14 + 8) = s_Map_Obj_etc_img_timer3_fontTime_1433dc478._16_8_;
    piVar14[10] = s_Map_Obj_etc_img_timer3_fontTime_1433dc478._24_4_;
    *(undefined2 *)(piVar14 + 0xb) = s_Map_Obj_etc_img_timer3_fontTime_1433dc478._28_2_;
    *(char *)((longlong)piVar14 + 0x2e) = s_Map_Obj_etc_img_timer3_fontTime_1433dc478[0x1e];
    local_res20 = piVar1;
    if (*piVar14 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar14[1] < 0x1f) {
      FUN_142e54290(0x90,piVar14[1],0x1f);
    }
    *piVar14 = 1;
    *(undefined1 *)((longlong)piVar14 + 0x2f) = 0;
    if (piVar14[1] + 1 < 0x20) {
      FUN_142e54290(0x9c);
    }
    piVar14[2] = 0x1f;
    if (local_78 != (int *)0x0) {
      FUN_14019f2c0(local_78 + -4);
    }
    local_60 = local_60 + -0xf;
    local_5c = local_5c + -0xf;
    local_78 = piVar1;
    uVar9 = FUN_1406e8c20(param_2);
    FUN_141839a60(param_1,(ulonglong)local_res18 & 0xffffffff,local_res8,local_98,uVar9,0x102,0x3a);
    if (local_70 != 0) {
      FUN_14019f2c0(local_70 + -0x10);
    }
    if (local_78 != (int *)0x0) {
      FUN_14019f2c0(local_78 + -4);
    }
    break;
  case 7:
    cVar7 = FUN_1406e8ae0(param_2);
    FUN_1415e7c00(local_98);
    (**(code **)(*param_1 + 0x1e0))(param_1,local_98,&local_res18,&local_res8);
    uVar9 = FUN_1406e8c20(param_2);
    FUN_141839a60(param_1,(ulonglong)local_res18 & 0xffffffff,local_res8,local_98,uVar9,local_54,
                  local_50);
    if (cVar7 != '\0') {
      uVar17 = FUN_1418a7dd0(param_1[0x15] + 0x220);
      FUN_1415ea720(uVar17);
    }
    if (local_70 != 0) {
      FUN_14019f2c0(local_70 + -0x10);
    }
    if (local_78 != (int *)0x0) {
      FUN_14019f2c0(local_78 + -4);
    }
    break;
  case 9:
    cVar7 = FUN_1406e8ae0(param_2);
    uVar8 = FUN_1406e8c20(param_2);
    if (cVar7 != '\0') {
      FUN_142d98b70(uVar17);
    }
    if ((int)uVar8 < 1) {
      return;
    }
    FUN_142d989f0(uVar17,(uVar8 ^ (int)uVar8 >> 0x1f) - ((int)uVar8 >> 0x1f));
    FUN_142d98b50(uVar17);
    return;
  case 10:
    FUN_1415e7c00(local_98);
    (**(code **)(*param_1 + 0x1e0))(param_1,local_98,&local_res18,&local_res8);
    puVar15 = (undefined8 *)FUN_1408a9e40(&local_res20,0x7a8);
    if (local_78 != (int *)0x0) {
      FUN_14019f2c0(local_78 + -4);
    }
    local_78 = (int *)*puVar15;
    *puVar15 = 0;
    if (local_res20 != (int *)0x0) {
      FUN_14019f2c0(local_res20 + -4);
    }
    FUN_1418396d0(param_1);
    cVar7 = FUN_1406e8ae0(param_2);
    if (cVar7 == '\0') {
      if (local_70 != 0) {
        FUN_14019f2c0(local_70 + -0x10);
      }
      if (local_78 != (int *)0x0) {
        FUN_14019f2c0(local_78 + -4);
      }
    }
    else {
      uVar9 = FUN_1406e8c20(param_2);
      FUN_141839a60(param_1,(ulonglong)local_res18 & 0xffffffff,local_res8,local_98,uVar9,local_54,
                    local_50);
      if (local_70 != 0) {
        FUN_14019f2c0(local_70 + -0x10);
      }
      if (local_78 != (int *)0x0) {
        FUN_14019f2c0(local_78 + -4);
      }
    }
    break;
  case 0xd:
    FUN_1415e7c00(local_98);
    (**(code **)(*param_1 + 0x1e0))(param_1,local_98,&local_res18,&local_res8);
    local_48 = 1;
    uVar9 = FUN_1406e8c20(param_2);
    cVar7 = FUN_1406e8ae0(param_2);
    local_44 = cVar7 != '\0';
    FUN_141839a60(param_1,(ulonglong)local_res18 & 0xffffffff,local_res8,local_98,uVar9,local_54,
                  local_50);
    if (local_70 != 0) {
      FUN_14019f2c0(local_70 + -0x10);
    }
    if (local_78 != (int *)0x0) {
      FUN_14019f2c0(local_78 + -4);
    }
    break;
  case 0xe:
    uVar20 = 0;
    local_res20 = (int *)0x0;
    FUN_1406e9170(param_2,&local_res8,4);
    puVar15 = (undefined8 *)FUN_1406e9050(param_2,local_a8);
    piVar1 = (int *)*puVar15;
    *puVar15 = 0;
    local_res20 = piVar1;
    if (local_a8[0] != (int *)0x0) {
      FUN_14019f2c0(local_a8[0] + -4);
    }
    local_res18 = (int *)0x0;
    piVar18 = piVar1;
    piVar14 = local_res18;
    if ((piVar1 != (int *)0x0) && (piVar16 = piVar1 + -4, piVar16 != (int *)0x0)) {
      if (*piVar16 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar21 = 0xffffffffffffffff;
        uVar19 = 0xffffffffffffffff;
        do {
          uVar19 = uVar19 + 1;
        } while (*(char *)((longlong)piVar1 + uVar19) != '\0');
        iVar11 = (int)uVar19;
        iVar10 = 0;
        if (0 < iVar11) {
          iVar10 = iVar11;
        }
        piVar16 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar10 + 0x11));
        piVar16[1] = iVar10;
        *piVar16 = -1;
        piVar14 = piVar16 + 4;
        piVar16[2] = 0;
        *(undefined1 *)piVar14 = 0;
        local_a8[0] = piVar14;
        FUN_142ef7ba0(piVar14,piVar1,(longlong)iVar11);
        if (*piVar16 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar11 == -1) || (iVar11 <= piVar16[1])) {
          *piVar16 = 1;
          if (iVar11 != -1) goto LAB_141856cbb;
          if (piVar14 != (int *)0x0) {
            do {
              uVar21 = uVar21 + 1;
            } while (*(char *)((longlong)piVar14 + uVar21) != '\0');
            uVar20 = uVar21 & 0xffffffff;
          }
        }
        else {
          FUN_142e54290(0x90,piVar16[1],uVar19 & 0xffffffff);
          *piVar16 = 1;
LAB_141856cbb:
          *(undefined1 *)((longlong)piVar14 + (longlong)iVar11) = 0;
          uVar20 = uVar19;
        }
        iVar10 = (int)uVar20;
        if ((iVar10 < 0) || (piVar16[1] + 1 <= iVar10)) {
          FUN_142e54290(0x9c,uVar20 & 0xffffffff);
        }
        piVar16[2] = iVar10;
        if (local_res18 != (int *)0x0) {
          FUN_14019f2c0(local_res18 + -4);
        }
      }
      else {
        if (*piVar16 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar16 = *piVar16 + 1;
        UNLOCK();
        piVar18 = local_res20;
        piVar14 = piVar1;
        if (local_res18 != (int *)0x0) {
          FUN_14019f2c0(local_res18 + -4);
          piVar18 = local_res20;
        }
      }
    }
    local_res18 = piVar14;
    FUN_1418570c0(param_1,0xe,local_res8,&local_res18);
    if (piVar18 == (int *)0x0) {
      return;
    }
    goto LAB_141856f7b;
  case 0x10:
    FUN_1415e7c00(local_98);
    (**(code **)(*param_1 + 0x1e0))(param_1,local_98,&local_res8,&local_res18);
    uVar9 = FUN_1406e8c20(param_2);
    FUN_141839a60(param_1,local_res8,4,local_98,uVar9,local_54,local_50);
    if (local_70 != 0) {
      FUN_14019f2c0(local_70 + -0x10);
    }
    if (local_78 != (int *)0x0) {
      FUN_14019f2c0(local_78 + -4);
    }
    break;
  case 0x11:
    FUN_1415e7c00(local_98);
    (**(code **)(*param_1 + 0x1e0))(param_1,local_98,&local_res18,&local_res8);
    iVar10 = FUN_1406e8c20(param_2);
    iVar11 = FUN_1406e8c20(param_2);
    iVar12 = FUN_1406e8c20(param_2);
    iVar13 = FUN_1406e8c20(param_2);
    FUN_141839a60(param_1,(ulonglong)local_res18 & 0xffffffff,local_res8,local_98,
                  ((iVar13 - iVar12) - iVar11) / 1000,local_54,local_50);
    if ((iVar10 != 0) && (*(longlong *)(param_1[0x15] + 0x228) != 0)) {
      uVar17 = FUN_1418a7dd0(param_1[0x15] + 0x220);
      FUN_1415ea720(uVar17);
    }
    if (local_70 != 0) {
      FUN_14019f2c0(local_70 + -0x10);
    }
    if (local_78 != (int *)0x0) {
      FUN_14019f2c0(local_78 + -4);
    }
  }
  piVar18 = local_80;
  if (local_80 != (int *)0x0) {
LAB_141856f7b:
    FUN_14019f2c0(piVar18 + -4);
  }
switchD_14185652b_caseD_3:
  return;
}



//===========================================================
// FUN_141839a60 @ 141839a60   (492 bytes)
//===========================================================

void FUN_141839a60(longlong param_1,undefined4 param_2,undefined4 param_3,undefined8 param_4,
                  undefined4 param_5,undefined4 param_6,undefined4 param_7)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 uVar3;
  longlong lVar4;
  undefined1 local_28 [8];
  longlong local_20;
  
  FUN_1418396d0();
  lVar2 = FUN_14019b780(&DAT_143ad68a0,0x2c8);
  uVar3 = 0;
  if (lVar2 != 0) {
    uVar3 = FUN_14189f7e0(lVar2);
  }
  FUN_1418a74f0(*(longlong *)(param_1 + 0xa8) + 0x220,uVar3);
  lVar2 = *(longlong *)(param_1 + 0xa8);
  lVar4 = *(longlong *)(lVar2 + 0x228);
  if (lVar4 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar4 = *(longlong *)(lVar2 + 0x228);
  }
  FUN_142bf2d70(lVar4,param_2,param_3,param_6,param_7,0xc00616fc,1,param_4,1,1);
  lVar2 = *(longlong *)(param_1 + 0xa8);
  lVar4 = *(longlong *)(lVar2 + 0x228);
  if (lVar4 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar4 = *(longlong *)(lVar2 + 0x228);
  }
  FUN_1415ea410(lVar4,param_5);
  lVar2 = *(longlong *)(param_1 + 0xa8);
  lVar4 = *(longlong *)(lVar2 + 0x228);
  if (lVar4 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar4 = *(longlong *)(lVar2 + 0x228);
  }
  FUN_1415ea650(lVar4);
  lVar2 = *(longlong *)(param_1 + 0xa8);
  plVar1 = (longlong *)(lVar2 + 0x228);
  lVar4 = *plVar1;
  if (lVar4 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar2 = *(longlong *)(param_1 + 0xa8);
    lVar4 = *plVar1;
  }
  FUN_1415eadd0(lVar4,*(undefined1 *)(lVar2 + 0x1c88));
  lVar2 = *(longlong *)(param_1 + 0xa8);
  plVar1 = (longlong *)(lVar2 + 0x228);
  lVar4 = *plVar1;
  if (lVar4 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar2 = *(longlong *)(param_1 + 0xa8);
    lVar4 = *plVar1;
  }
  FUN_1415eae10(lVar4,*(undefined1 *)(lVar2 + 0x1c89));
  lVar2 = *(longlong *)(*(longlong *)(param_1 + 0xa8) + 0x228);
  local_20 = lVar2;
  if (lVar2 != 0) {
    if (0xfffff < *(ulonglong *)(lVar2 + 0x20)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar2 + 0x20) = *(longlong *)(lVar2 + 0x20) + 1;
    UNLOCK();
  }
  FUN_141839c60(param_1,local_28);
  return;
}


