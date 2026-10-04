
//===========================================================
// FUN_142a8d5e0 @ 142a8d5e0   (2166 bytes)
//===========================================================

void FUN_142a8d5e0(longlong param_1)

{
  longlong *plVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  int iVar4;
  undefined4 uVar5;
  int *piVar6;
  undefined4 *puVar7;
  longlong lVar8;
  undefined8 *puVar9;
  int iVar10;
  wchar_t *pwVar11;
  wchar_t *pwVar12;
  int iVar13;
  wchar_t *pwVar14;
  wchar_t *pwVar15;
  undefined8 *puVar16;
  undefined1 local_res8 [8];
  undefined1 local_res10 [8];
  int local_res18;
  undefined4 local_res20 [2];
  wchar_t *local_b8;
  longlong local_b0;
  undefined1 *local_a8;
  undefined1 local_a0 [8];
  longlong local_98;
  undefined1 local_90 [8];
  longlong local_88;
  undefined1 local_80 [8];
  longlong local_78;
  longlong local_70;
  wchar_t **local_68;
  undefined4 *local_60;
  undefined1 *local_58;
  
  pwVar12 = (wchar_t *)0x0;
  iVar10 = 0;
  local_b8 = (wchar_t *)0x0;
  piVar6 = (int *)FUN_1401bc720(&DAT_143ad6980,0x34);
  piVar6[1] = 0x11;
  *piVar6 = -1;
  local_b8 = (wchar_t *)(piVar6 + 4);
  iVar4 = 0;
  piVar6[2] = 0;
  *local_b8 = L'\0';
  uVar3 = u_UI_UtilDlgEx_img__14348acc0._12_4_;
  uVar2 = u_UI_UtilDlgEx_img__14348acc0._8_4_;
  uVar5 = u_UI_UtilDlgEx_img__14348acc0._4_4_;
  *(undefined4 *)local_b8 = u_UI_UtilDlgEx_img__14348acc0._0_4_;
  piVar6[5] = uVar5;
  piVar6[6] = uVar2;
  piVar6[7] = uVar3;
  uVar3 = u_UI_UtilDlgEx_img__14348acc0._28_4_;
  uVar2 = u_UI_UtilDlgEx_img__14348acc0._24_4_;
  uVar5 = u_UI_UtilDlgEx_img__14348acc0._20_4_;
  piVar6[8] = u_UI_UtilDlgEx_img__14348acc0._16_4_;
  piVar6[9] = uVar5;
  piVar6[10] = uVar2;
  piVar6[0xb] = uVar3;
  *(wchar_t *)(piVar6 + 0xc) = u_UI_UtilDlgEx_img__14348acc0[0x10];
  if (*piVar6 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar6[1] < 0x11) {
    FUN_142e54290(0x90,piVar6[1],0x11);
  }
  *piVar6 = 1;
  local_b8[0x11] = L'\0';
  if (piVar6[1] + 1 < 0x12) {
    FUN_142e54290(0x9c,0x11);
  }
  pwVar15 = local_b8;
  piVar6[2] = 0x22;
  iVar13 = *(int *)(param_1 + 0x2a8);
  if (iVar13 == 0x17) {
LAB_142a8d94f:
    pwVar11 = (wchar_t *)0xffffffffffffffff;
    do {
      pwVar11 = (wchar_t *)((longlong)pwVar11 + 1);
    } while (L"UtilDlgEx_MixHair"[(longlong)pwVar11] != L'\0');
    iVar13 = (int)pwVar11;
    if (iVar13 == 0) goto LAB_142a8db92;
    pwVar14 = pwVar12;
    if (local_b8 == (wchar_t *)0x0) goto LAB_142a8dada;
    if (*local_b8 != L'\0') {
      iVar13 = (int)((ulonglong)(longlong)*(int *)(local_b8 + -4) >> 1) + iVar13;
      for (iVar4 = *(int *)(local_b8 + -6); iVar4 < iVar13; iVar4 = iVar4 * 2) {
      }
      pwVar12 = local_b8 + -8;
      if (pwVar12 == (wchar_t *)0x0) {
LAB_142a8d9d9:
        if (iVar10 < iVar4) {
          iVar10 = iVar4;
        }
        puVar7 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar10 * 2 + 0x12));
        puVar7[1] = iVar10;
        *puVar7 = 0xffffffff;
        local_b8 = (wchar_t *)(puVar7 + 4);
        if (pwVar12 == (wchar_t *)0x0) {
          puVar7[2] = 0;
          *local_b8 = L'\0';
        }
        else {
          iVar4 = (*(uint *)(pwVar15 + -4) & 0xfffffffe) + 2;
          local_res18 = iVar10 * 2 + 2;
          if (local_res18 < iVar4) {
            FUN_142e54290(0x5c,iVar4,local_res18);
            iVar4 = local_res18;
          }
          FUN_142ef7ba0(local_b8,pwVar15,(longlong)iVar4);
          puVar7[2] = *(int *)(pwVar15 + -4);
          local_b8[iVar10] = L'\0';
          FUN_1401bebb0(pwVar12);
        }
      }
      else {
        if ((1 < *(int *)pwVar12) || (*(int *)(local_b8 + -6) < iVar4)) {
          iVar10 = (int)((ulonglong)(longlong)*(int *)(local_b8 + -4) >> 1);
          goto LAB_142a8d9d9;
        }
        if (*(int *)pwVar12 != 1) {
          FUN_142e52dd0(0x74);
        }
        pwVar12[0] = L'\xffff';
        pwVar12[1] = L'\xffff';
      }
      pwVar12 = L"UtilDlgEx_MixHair";
      if (local_b8 == (wchar_t *)0x0) goto LAB_142a8d826;
      iVar10 = (int)((ulonglong)(longlong)*(int *)(local_b8 + -4) >> 1);
      goto LAB_142a8d829;
    }
    if ((local_b8 == (wchar_t *)0x0) || (pwVar14 = local_b8 + -8, pwVar14 == (wchar_t *)0x0)) {
LAB_142a8dada:
      if (iVar4 < iVar13) {
        iVar4 = iVar13;
      }
      puVar7 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar4 * 2 + 0x12));
      puVar7[1] = iVar4;
      *puVar7 = 0xffffffff;
      local_b8 = (wchar_t *)(puVar7 + 4);
      puVar7[2] = 0;
      *local_b8 = L'\0';
      if (pwVar14 != (wchar_t *)0x0) {
        FUN_1401bebb0(pwVar14);
      }
    }
    else {
      if ((1 < *(int *)pwVar14) || (*(int *)(local_b8 + -6) < iVar13)) {
        iVar4 = (int)((ulonglong)(longlong)*(int *)(local_b8 + -4) >> 1);
        goto LAB_142a8dada;
      }
      if (*(int *)pwVar14 != 1) {
        FUN_142e52dd0(0x74);
      }
      pwVar14[0] = L'\xffff';
      pwVar14[1] = L'\xffff';
    }
    FUN_142ef7ba0(local_b8,L"UtilDlgEx_MixHair",(longlong)iVar13 * 2);
    pwVar15 = local_b8;
    if (*(int *)(local_b8 + -8) != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar13 == -1) || (iVar13 <= *(int *)(pwVar15 + -6))) {
      pwVar15[-8] = L'\x01';
      pwVar15[-7] = L'\0';
      if (iVar13 != -1) goto LAB_142a8db64;
      if (pwVar15 != (wchar_t *)0x0) {
        pwVar12 = (wchar_t *)0xffffffffffffffff;
        do {
          pwVar12 = (wchar_t *)((longlong)pwVar12 + 1);
        } while (pwVar15[(longlong)pwVar12] != L'\0');
      }
    }
    else {
      FUN_142e54290(0x90,*(int *)(pwVar15 + -6),(ulonglong)pwVar11 & 0xffffffff);
      pwVar15[-8] = L'\x01';
      pwVar15[-7] = L'\0';
LAB_142a8db64:
      local_b8[iVar13] = L'\0';
      pwVar12 = pwVar11;
    }
LAB_142a8db70:
    iVar10 = (int)pwVar12;
    if ((iVar10 < 0) || (*(int *)(pwVar15 + -6) + 1 <= iVar10)) {
      FUN_142e54290(0x9c,(ulonglong)pwVar12 & 0xffffffff);
    }
    *(int *)(pwVar15 + -4) = iVar10 * 2;
  }
  else {
    if (iVar13 != 0x19) {
      if (iVar13 == 0x1c) goto LAB_142a8d94f;
      if (iVar13 != 0x1d) goto LAB_142a8de2f;
    }
    pwVar11 = (wchar_t *)0xffffffffffffffff;
    do {
      pwVar11 = (wchar_t *)((longlong)pwVar11 + 1);
    } while (L"UtilDlgEx_MixLens"[(longlong)pwVar11] != L'\0');
    iVar13 = (int)pwVar11;
    if (iVar13 != 0) {
      pwVar14 = pwVar12;
      if (local_b8 == (wchar_t *)0x0) goto LAB_142a8d88b;
      if (*local_b8 != L'\0') {
        iVar13 = (int)((ulonglong)(longlong)*(int *)(local_b8 + -4) >> 1) + iVar13;
        for (iVar4 = *(int *)(local_b8 + -6); iVar4 < iVar13; iVar4 = iVar4 * 2) {
        }
        pwVar12 = local_b8 + -8;
        if (pwVar12 == (wchar_t *)0x0) {
LAB_142a8d769:
          if (iVar10 < iVar4) {
            iVar10 = iVar4;
          }
          puVar7 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar10 * 2 + 0x12));
          puVar7[1] = iVar10;
          *puVar7 = 0xffffffff;
          local_b8 = (wchar_t *)(puVar7 + 4);
          if (pwVar12 == (wchar_t *)0x0) {
            puVar7[2] = 0;
            *local_b8 = L'\0';
          }
          else {
            iVar4 = (*(uint *)(pwVar15 + -4) & 0xfffffffe) + 2;
            local_res18 = iVar10 * 2 + 2;
            if (local_res18 < iVar4) {
              FUN_142e54290(0x5c,iVar4,local_res18);
              iVar4 = local_res18;
            }
            FUN_142ef7ba0(local_b8,pwVar15,(longlong)iVar4);
            puVar7[2] = *(int *)(pwVar15 + -4);
            local_b8[iVar10] = L'\0';
            FUN_1401bebb0(pwVar12);
          }
        }
        else {
          if ((1 < *(int *)pwVar12) || (*(int *)(local_b8 + -6) < iVar4)) {
            iVar10 = (int)((ulonglong)(longlong)*(int *)(local_b8 + -4) >> 1);
            goto LAB_142a8d769;
          }
          if (*(int *)pwVar12 != 1) {
            FUN_142e52dd0(0x74);
          }
          pwVar12[0] = L'\xffff';
          pwVar12[1] = L'\xffff';
        }
        pwVar12 = L"UtilDlgEx_MixLens";
        if (local_b8 == (wchar_t *)0x0) {
LAB_142a8d826:
          iVar10 = 0;
        }
        else {
          iVar10 = (int)((ulonglong)(longlong)*(int *)(local_b8 + -4) >> 1);
        }
LAB_142a8d829:
        FUN_142ef7ba0(local_b8 + iVar10,pwVar12,(longlong)(int)pwVar11 * 2);
        FUN_1401bd8a0(&local_b8,iVar13);
        goto LAB_142a8db92;
      }
      if ((local_b8 == (wchar_t *)0x0) || (pwVar14 = local_b8 + -8, pwVar14 == (wchar_t *)0x0)) {
LAB_142a8d88b:
        if (iVar4 < iVar13) {
          iVar4 = iVar13;
        }
        puVar7 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar4 * 2 + 0x12));
        puVar7[1] = iVar4;
        *puVar7 = 0xffffffff;
        local_b8 = (wchar_t *)(puVar7 + 4);
        puVar7[2] = 0;
        *local_b8 = L'\0';
        if (pwVar14 != (wchar_t *)0x0) {
          FUN_1401bebb0(pwVar14);
        }
      }
      else {
        if ((1 < *(int *)pwVar14) || (*(int *)(local_b8 + -6) < iVar13)) {
          iVar4 = (int)((ulonglong)(longlong)*(int *)(local_b8 + -4) >> 1);
          goto LAB_142a8d88b;
        }
        if (*(int *)pwVar14 != 1) {
          FUN_142e52dd0(0x74);
        }
        pwVar14[0] = L'\xffff';
        pwVar14[1] = L'\xffff';
      }
      FUN_142ef7ba0(local_b8,L"UtilDlgEx_MixLens",(longlong)iVar13 * 2);
      pwVar15 = local_b8;
      if (*(int *)(local_b8 + -8) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar13 == -1) || (iVar13 <= *(int *)(pwVar15 + -6))) {
        pwVar15[-8] = L'\x01';
        pwVar15[-7] = L'\0';
        if (iVar13 == -1) {
          if (pwVar15 != (wchar_t *)0x0) {
            pwVar12 = (wchar_t *)0xffffffffffffffff;
            do {
              pwVar12 = (wchar_t *)((longlong)pwVar12 + 1);
            } while (pwVar15[(longlong)pwVar12] != L'\0');
          }
          goto LAB_142a8db70;
        }
      }
      else {
        FUN_142e54290(0x90,*(int *)(pwVar15 + -6),(ulonglong)pwVar11 & 0xffffffff);
        pwVar15[-8] = L'\x01';
        pwVar15[-7] = L'\0';
      }
      local_b8[iVar13] = L'\0';
      pwVar12 = pwVar11;
      goto LAB_142a8db70;
    }
  }
LAB_142a8db92:
  puVar16 = (undefined8 *)0x0;
  iVar10 = *(int *)(param_1 + 0x2a8);
  local_res10[0] = iVar10 - 0x1cU < 2;
  if ((iVar10 == 0x17) || (iVar10 == 0x1c)) {
    local_res8[0] = 1;
  }
  else {
    local_res8[0] = 0;
  }
  local_res20[0] = 7;
  local_68 = &local_b8;
  local_60 = local_res20;
  local_58 = local_res8;
  local_a8 = local_res10;
  local_b0 = param_1;
  local_70 = param_1;
  FUN_141ac3370(*(undefined8 *)(param_1 + 0x6c8),local_b8,0,0,0,1,0,0);
  iVar10 = *(int *)(param_1 + 0x2a8);
  if ((((iVar10 == 0x17) || (iVar10 == 0x19)) || (iVar10 == 0x1c)) || (iVar10 == 0x1d)) {
    FUN_142a92890(&local_70,0,param_1 + 0x4b0);
    FUN_142a92890(&local_70,1,param_1 + 0x4c8);
    FUN_142a91c70(param_1 + 0x418,0);
    FUN_142a93870(&local_b0,param_1 + 0x418,0x3ed,0x108);
  }
  lVar8 = FUN_141ad46a0(*(undefined8 *)(param_1 + 0x6c8),local_a0,1);
  FUN_140ff2290(param_1 + 0x5f0,*(undefined8 *)(lVar8 + 8));
  lVar8 = local_98;
  if (local_98 != 0) {
    if (0xffffe < *(longlong *)(local_98 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar8 + 0x20);
    lVar8 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar8 == 1) {
      puVar9 = (undefined8 *)(local_98 + 0x18);
      if (local_98 == 0) {
        puVar9 = puVar16;
      }
      if (puVar9 != (undefined8 *)0x0) {
        (**(code **)*puVar9)(puVar9,1);
      }
    }
  }
  lVar8 = FUN_141ad46a0(*(undefined8 *)(param_1 + 0x6c8),local_90,0x2002);
  FUN_140ff2290(param_1 + 0x640,*(undefined8 *)(lVar8 + 8));
  lVar8 = local_88;
  if (local_88 != 0) {
    if (0xffffe < *(longlong *)(local_88 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar8 + 0x20);
    lVar8 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar8 == 1) {
      puVar9 = (undefined8 *)(local_88 + 0x18);
      if (local_88 == 0) {
        puVar9 = puVar16;
      }
      if (puVar9 != (undefined8 *)0x0) {
        (**(code **)*puVar9)(puVar9,1);
      }
    }
  }
  lVar8 = FUN_141ad46a0(*(undefined8 *)(param_1 + 0x6c8),local_80,0x2003);
  FUN_140ff2290(param_1 + 0x650,*(undefined8 *)(lVar8 + 8));
  lVar8 = local_78;
  if (local_78 != 0) {
    if (0xffffe < *(longlong *)(local_78 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar8 + 0x20);
    lVar8 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar8 == 1) {
      puVar9 = (undefined8 *)(local_78 + 0x18);
      if (local_78 == 0) {
        puVar9 = puVar16;
      }
      if (puVar9 != (undefined8 *)0x0) {
        (**(code **)*puVar9)(puVar9,1);
      }
    }
  }
  FUN_142a8b300(param_1);
  FUN_142a8ff80(param_1,param_1 + 0x418);
  uVar5 = (*DAT_143262db0)();
  *(undefined4 *)(param_1 + 0x558) = uVar5;
LAB_142a8de2f:
  if (local_b8 != (wchar_t *)0x0) {
    FUN_1401bebb0(local_b8 + -8);
  }
  return;
}



//===========================================================
// FUN_142a91f30 @ 142a91f30   (766 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000142a92143) */

void FUN_142a91f30(longlong param_1,int param_2,int param_3,undefined8 *param_4,longlong *param_5)

{
  undefined4 uVar1;
  undefined4 uVar2;
  uint *puVar3;
  longlong *plVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  undefined4 uVar8;
  undefined8 uVar9;
  undefined4 *puVar10;
  longlong lVar11;
  longlong lVar12;
  uint uVar13;
  undefined4 local_res8 [2];
  
  puVar3 = (uint *)*param_4;
  uVar13 = 0;
  uVar7 = uVar13;
  if ((puVar3 != (uint *)0x0) && (puVar3[-2] != 0)) {
    uVar7 = *puVar3;
  }
  if (0 < param_2) {
    iVar5 = FUN_140417c70(param_2);
    if (iVar5 != 0) {
      uVar6 = 100;
      goto LAB_142a91fcb;
    }
    uVar6 = FUN_1401a8170(param_2);
    if (uVar6 != 0) goto LAB_142a91fcb;
  }
  uVar6 = uVar13;
  if (0 < (int)uVar7) {
    iVar5 = FUN_1402538e0(uVar7);
    if (iVar5 == 0) {
      iVar5 = FUN_1402538a0(uVar7);
      if (iVar5 == 0) {
        iVar5 = FUN_140253930(uVar7);
        uVar6 = (uint)(iVar5 != 0);
      }
      else {
        uVar6 = 0xb;
      }
    }
    else {
      uVar6 = 0x15;
    }
  }
LAB_142a91fcb:
  *(uint *)(param_1 + 0x70) = uVar6;
  plVar4 = (longlong *)param_5[7];
  if (plVar4 != (longlong *)0x0) {
    (**(code **)(*plVar4 + 0x10))(plVar4,param_1);
  }
  if (*(longlong *)(param_1 + 0x10) != 0) {
    *(undefined4 *)(param_1 + 200) = 0;
    switch(param_3) {
    case 0x17:
    case 0x19:
    case 0x1c:
    case 0x1d:
      puVar10 = (undefined4 *)*param_4;
      if ((puVar10 != (undefined4 *)0x0) && (1 < (uint)puVar10[-2])) {
        uVar8 = *puVar10;
        if (param_3 - 0x1cU < 2) {
          uVar8 = 0x32;
        }
        uVar2 = puVar10[1];
        puVar10 = (undefined4 *)FUN_1401abb40(param_1 + 0xd0,0xffffffff);
        *puVar10 = uVar2;
        FUN_142a94840(param_1 + 0x78);
        uVar7 = FUN_1407386b0(&DAT_143ac1ab0);
        lVar12 = *(longlong *)(param_1 + 0x80);
        if (lVar12 == 0) {
          FUN_142e52ed0(0x428,0);
          lVar12 = *(longlong *)(param_1 + 0x80);
        }
        FUN_14041a250(lVar12,uVar7 & 7,(uVar7 & 7) + 1 & 0x80000007,uVar8);
      }
      break;
    case 0x18:
    case 0x1a:
      puVar10 = (undefined4 *)*param_4;
      if ((puVar10 != (undefined4 *)0x0) && (1 < (uint)puVar10[-2])) {
        uVar8 = *puVar10;
        uVar2 = puVar10[1];
        puVar10 = (undefined4 *)FUN_1401abb40(param_1 + 0xd0,0xffffffff);
        *puVar10 = uVar2;
        FUN_1402ca630(param_1 + 0x18);
        if (*(longlong *)(param_1 + 0x10) != 0) {
          FUN_1410b3850(param_1 + 0x18,local_res8);
          lVar12 = *(longlong *)(param_1 + 0x10);
          if (lVar12 == 0) {
            FUN_142e52ed0(0x428,0);
            lVar12 = *(longlong *)(param_1 + 0x10);
          }
          lVar11 = *(longlong *)(param_1 + 0x20);
          if (lVar11 == 0) {
            FUN_142e52ed0(0x428,0);
            lVar11 = *(longlong *)(param_1 + 0x20);
          }
          FUN_1402eeb50(lVar11,lVar12);
        }
        if (*(longlong *)(param_1 + 0x20) != 0) {
          uVar1 = *(undefined4 *)(param_1 + 0x70);
          uVar9 = FUN_142a927d0(param_1 + 0x18);
          FUN_1401a72a0(uVar9,uVar1,uVar8);
        }
        FUN_142a94840(param_1 + 0x78);
        uVar9 = FUN_142a92800(param_1 + 0x78);
        FUN_14041a360(uVar2,uVar9);
      }
      break;
    default:
      FUN_14022de90(param_1 + 0xd0,param_4);
    }
    iVar5 = *(int *)(param_1 + 0x70);
    if (((iVar5 == 0xd) || (iVar5 == 0x17)) && (*(longlong *)(param_1 + 0x10) != 0)) {
      local_res8[0] = 0;
      FUN_1401a7220(*(longlong *)(param_1 + 0x10),iVar5,local_res8);
      puVar10 = *(undefined4 **)(param_1 + 0xd0);
      if ((puVar10 != (undefined4 *)0x0) && (puVar10[-2] != 0)) {
        do {
          uVar8 = FUN_1401a8660(*(undefined4 *)(param_1 + 0x70),local_res8[0],*puVar10);
          *puVar10 = uVar8;
          if ((undefined4 *)
              (*(longlong *)(param_1 + 0xd0) + *(longlong *)(*(longlong *)(param_1 + 0xd0) + -8) * 4
              + -4) <= puVar10) break;
          puVar10 = puVar10 + 1;
        } while (puVar10 != (undefined4 *)0x0);
      }
    }
    FUN_142a91ad0(param_1);
  }
  plVar4 = (longlong *)param_5[7];
  if (plVar4 != (longlong *)0x0) {
    (**(code **)(*plVar4 + 0x20))(plVar4,plVar4 != param_5);
    param_5[7] = 0;
  }
  return;
}



//===========================================================
// FUN_142a8fe70 @ 142a8fe70   (253 bytes)
//===========================================================

void FUN_142a8fe70(longlong param_1)

{
  longlong lVar1;
  longlong lVar2;
  
  *(undefined1 *)(param_1 + 0x418) = 0xff;
  FUN_1402ca630(param_1 + 0x420);
  FUN_1402ca630(param_1 + 0x430);
  if (*(longlong **)(param_1 + 0x440) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x440) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x440) = 0;
  FUN_140d948a0(param_1 + 0x4a0);
  lVar1 = *(longlong *)(param_1 + 0x4b8);
  lVar2 = *(longlong *)(param_1 + 0x4b0);
  if (lVar2 != lVar1) {
    do {
      FUN_140cbb020(lVar2);
      lVar2 = lVar2 + 0x10;
    } while (lVar2 != lVar1);
    lVar2 = *(longlong *)(param_1 + 0x4b0);
  }
  *(longlong *)(param_1 + 0x4b8) = lVar2;
  lVar1 = *(longlong *)(param_1 + 0x4d0);
  lVar2 = *(longlong *)(param_1 + 0x4c8);
  if (lVar2 != lVar1) {
    do {
      FUN_140cbb020(lVar2);
      lVar2 = lVar2 + 0x10;
    } while (lVar2 != lVar1);
    lVar2 = *(longlong *)(param_1 + 0x4c8);
  }
  *(longlong *)(param_1 + 0x4d0) = lVar2;
  *(undefined4 *)(param_1 + 0x4e0) = 0;
  if (*(longlong *)(param_1 + 0x4e8) != 0) {
    thunk_FUN_140205820(*(longlong *)(param_1 + 0x4e8) + -8,0);
    *(undefined8 *)(param_1 + 0x4e8) = 0;
  }
  return;
}


