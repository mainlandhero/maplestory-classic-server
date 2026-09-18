
//===========================================================
// FUN_14038a9c0 @ 14038a9c0   (154 bytes)
//===========================================================

undefined8 FUN_14038a9c0(undefined8 param_1,undefined4 param_2)

{
  undefined *puVar1;
  int iVar2;
  longlong *local_res18;
  longlong *local_res20;
  
  if (DAT_143aa8328 != 0) {
    FUN_14039f600(DAT_143aa8328,&local_res20,param_2,0);
    puVar1 = PTR_u_onlyEquip_143a46240;
    if (local_res20 == (longlong *)0x0) {
      return 0;
    }
    local_res18 = local_res20;
    (**(code **)(*local_res20 + 8))(local_res20);
    iVar2 = FUN_140910eb0(&local_res18,puVar1,0);
    if (iVar2 != 0) {
      (**(code **)(*local_res20 + 0x10))(local_res20);
      return 1;
    }
    (**(code **)(*local_res20 + 0x10))(local_res20);
  }
  return 0;
}



//===========================================================
// FUN_1403e88e0 @ 1403e88e0   (518 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

ulonglong FUN_1403e88e0(longlong param_1,int param_2,undefined1 param_3,undefined8 param_4,
                       undefined8 param_5,char param_6)

{
  int iVar1;
  uint uVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  longlong *plVar5;
  longlong lVar6;
  ulonglong uVar7;
  undefined1 auStackY_88 [32];
  undefined8 *local_58;
  ulonglong local_50;
  
  local_50 = DAT_143a8b908 ^ (ulonglong)auStackY_88;
  iVar1 = FUN_1403e8af0(param_2);
  if (iVar1 == 1) {
    lVar6 = 1;
    plVar5 = (longlong *)(param_1 + 0x1c0);
    do {
      if ((*plVar5 != 0) && (iVar1 = FUN_14019a5d0(*plVar5 + 0x20), iVar1 == param_2)) {
        return 1;
      }
      lVar6 = lVar6 + 1;
      plVar5 = plVar5 + 2;
    } while (lVar6 < 0x20);
    uVar7 = FUN_1402de840(param_1 + 0x5a8,param_2,0,param_3);
  }
  else if (iVar1 == 6) {
    lVar6 = 1;
    plVar5 = (longlong *)(param_1 + 0x3c0);
    do {
      if ((*plVar5 != 0) && (iVar1 = FUN_14019a5d0(*plVar5 + 0x20), iVar1 == param_2)) {
        return 1;
      }
      lVar6 = lVar6 + 1;
      plVar5 = plVar5 + 2;
    } while (lVar6 < 0x20);
    uVar2 = FUN_1402de840(param_1 + 0x5a8,param_2,1,param_3);
    uVar7 = (ulonglong)uVar2;
    if (param_6 != '\0') {
      _eh_vector_constructor_iterator_
                (&local_58,8,1,(_func_void_void_ptr *)&LAB_1402f77e0,
                 (_func_void_void_ptr *)&LAB_1401d33e0);
      FUN_1401d3570(&local_58);
      puVar3 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x208);
      if (puVar3 == (undefined8 *)0x0) {
        local_58 = (undefined8 *)0x0;
      }
      else {
        local_58 = puVar3 + 1;
        if (local_58 != (undefined8 *)0x0) {
          *puVar3 = 0x20;
          for (puVar4 = local_58; puVar4 < puVar3 + 0x41; puVar4 = puVar4 + 2) {
            puVar4[1] = 0;
          }
        }
      }
      FUN_1402e3e60(param_1,&local_58,0xffffffff);
      if ((uVar2 != 0) || (iVar1 = FUN_1402deed0(&local_58,param_2), uVar7 = 0, iVar1 != 0)) {
        uVar7 = 1;
      }
      _eh_vector_destructor_iterator_(&local_58,8,1,(_func_void_void_ptr *)&LAB_1401d33e0);
    }
  }
  else {
    uVar7 = 0;
  }
  return uVar7;
}



//===========================================================
// FUN_14038dcf0 @ 14038dcf0   (112 bytes)
//===========================================================

undefined4 FUN_14038dcf0(undefined8 param_1,int param_2)

{
  longlong lVar1;
  
  if (param_2 - 5000000U < 10000) {
    return 100;
  }
  if (param_2 - 1800000U < 100000) {
    return 0x65;
  }
  if ((999999 < param_2 - 1000000U) && (999999 < param_2 - 6000000U)) {
    return 0xffffffff;
  }
  lVar1 = FUN_140388c60();
  if (lVar1 != 0) {
    return *(undefined4 *)(lVar1 + 0x364);
  }
  return 0;
}



//===========================================================
// FUN_140253130 @ 140253130   (168 bytes)
//===========================================================

undefined8 FUN_140253130(int param_1)

{
  int iVar1;
  
  if ((((999999 < param_1 - 1000000U) && (999999 < param_1 - 6000000U)) &&
      (9999 < param_1 - 0x26c1e0U)) ||
     (((iVar1 = FUN_1402531f0(), iVar1 != 0 || (iVar1 = FUN_140416760(param_1), iVar1 != 0)) ||
      (iVar1 = FUN_140416820(param_1), iVar1 != 0)))) {
    return 2;
  }
  iVar1 = (param_1 / 1000) % 10;
  if (iVar1 == 0) {
    return 0;
  }
  if (iVar1 != 1) {
    if (iVar1 == 5) {
      return 0;
    }
    if (iVar1 != 6) {
      return 2;
    }
  }
  return 1;
}



//===========================================================
// FUN_140398820 @ 140398820   (6 bytes)
//===========================================================

undefined8 FUN_140398820(void)

{
  return 1;
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


