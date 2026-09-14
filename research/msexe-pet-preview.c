
//===========================================================
// FUN_142719d80 @ 142719d80   (761 bytes)
//===========================================================

undefined8 *
FUN_142719d80(undefined8 *param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4)

{
  undefined4 uVar1;
  undefined4 uVar2;
  IUnknown *pIVar3;
  int iVar4;
  undefined8 uVar5;
  int *piVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 local_res20 [2];
  int *local_58;
  IUnknown *local_50;
  undefined1 local_48 [32];
  
  uVar5 = FUN_141d5d8c0(local_48,&DAT_143271f04,&DAT_143271f04,0x1b6,0xffffffff);
  FUN_142bf1bb0(param_1,uVar5);
  DAT_143acda20 = param_1;
  if (param_1 == (undefined8 *)0xfffffffffffffdcf) {
    DAT_143acda20 = (undefined8 *)0x0;
  }
  *param_1 = &PTR_FUN_14347d168;
  param_1[1] = &PTR_LAB_14347d298;
  param_1[3] = &PTR_FUN_14347d370;
  param_1[0x48] = 0;
  param_1[0x4a] = 0;
  param_1[0x4c] = 0;
  param_1[0x4e] = 0;
  param_1[0x50] = 0;
  param_1[0x52] = 0;
  FUN_142644810(param_1 + 0x53);
  *(undefined4 *)(param_1 + 0x278) = 0;
  *(undefined4 *)((longlong)param_1 + 0x13cc) = param_4;
  *(undefined4 *)(param_1 + 0x27a) = 0;
  param_1[0x27b] = 0;
  param_1[0x27c] = 0;
  param_1[0x27d] = 0;
  param_1[0x27e] = 0;
  FUN_141aa3a20(param_1 + 0x27f);
  uVar7 = 0x104;
  local_58 = (int *)0x0;
  piVar6 = (int *)FUN_14019b600(&DAT_143ad6a30,0x3d);
  piVar6[1] = 0x2c;
  *piVar6 = -1;
  local_58 = piVar6 + 4;
  piVar6[2] = 0;
  *(undefined1 *)local_58 = 0;
  uVar5 = s_UI_UIWindow2_img_UserInfo_except_14347d7e0._8_8_;
  *(undefined8 *)local_58 = s_UI_UIWindow2_img_UserInfo_except_14347d7e0._0_8_;
  *(undefined8 *)(piVar6 + 6) = uVar5;
  uVar2 = s_UI_UIWindow2_img_UserInfo_except_14347d7e0._28_4_;
  uVar1 = s_UI_UIWindow2_img_UserInfo_except_14347d7e0._24_4_;
  uVar8 = s_UI_UIWindow2_img_UserInfo_except_14347d7e0._20_4_;
  piVar6[8] = s_UI_UIWindow2_img_UserInfo_except_14347d7e0._16_4_;
  piVar6[9] = uVar8;
  piVar6[10] = uVar1;
  piVar6[0xb] = uVar2;
  *(undefined8 *)(piVar6 + 0xc) = s_UI_UIWindow2_img_UserInfo_except_14347d7e0._32_8_;
  piVar6[0xe] = s_UI_UIWindow2_img_UserInfo_except_14347d7e0._40_4_;
  if (*piVar6 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar6[1] < 0x2c) {
    FUN_142e54290(0x90,piVar6[1],0x2c);
  }
  *piVar6 = 1;
  *(undefined1 *)(local_58 + 0xb) = 0;
  if (piVar6[1] + 1 < 0x2d) {
    FUN_142e54290(0x9c);
  }
  piVar6[2] = 0x2c;
  FUN_14090f3b0(&local_50,&local_58);
  if (local_58 != (int *)0x0) {
    FUN_14019f2c0(local_58 + -4);
  }
  pIVar3 = local_50;
  uVar8 = 0xc4;
  if (local_50 != (IUnknown *)0x0) {
    local_res20[0] = 0;
    iVar4 = (**(code **)(*(longlong *)local_50 + 0x98))(local_50,local_res20);
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar3,(_GUID *)&DAT_14327ac98);
    }
    pIVar3 = local_50;
    uVar7 = local_res20[0];
    if (local_50 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res20[0] = 0;
    iVar4 = (**(code **)(*(longlong *)local_50 + 0xa0))(local_50,local_res20);
    uVar8 = local_res20[0];
    if (iVar4 < 0) {
      _com_issue_errorex(iVar4,pIVar3,(_GUID *)&DAT_14327ac98);
      uVar8 = local_res20[0];
    }
  }
  FUN_142bf2d70(param_1,param_2,param_3,uVar7,uVar8,0x271a,1,0,1,0);
  if (local_50 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_50 + 0x10))();
  }
  return param_1;
}



//===========================================================
// FUN_141d5d8c0 @ 141d5d8c0   (22 bytes)
//===========================================================

undefined8 *
FUN_141d5d8c0(undefined8 *param_1,undefined8 param_2,undefined8 param_3,undefined4 param_4,
             undefined4 param_5)

{
  *(undefined4 *)((longlong)param_1 + 0x14) = param_5;
  *param_1 = param_2;
  param_1[1] = param_3;
  *(undefined4 *)(param_1 + 2) = param_4;
  return param_1;
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
// FUN_142ef3ac0 @ 142ef3ac0   (16 bytes)
//===========================================================

void FUN_142ef3ac0(undefined8 param_1)

{
  (*(code *)PTR_FUN_1432630d8)(param_1,0);
  return;
}



//===========================================================
// FUN_142bf2d70 @ 142bf2d70   (4226 bytes)
//===========================================================

void FUN_142bf2d70(longlong *param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
                  undefined4 param_5,undefined4 param_6,uint param_7,undefined8 param_8,int param_9,
                  undefined4 param_10)

{
  ushort uVar1;
  undefined8 *puVar2;
  longlong lVar3;
  IUnknown *pIVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  undefined1 uVar8;
  byte bVar9;
  undefined8 *puVar10;
  longlong *plVar11;
  int iVar12;
  longlong lVar13;
  IUnknown *pIVar14;
  byte bVar15;
  byte *pbVar16;
  uint uVar17;
  undefined8 local_res8;
  undefined4 local_res10;
  undefined4 local_res18;
  undefined8 in_stack_fffffffffffffe58;
  ulonglong uVar18;
  undefined8 in_stack_fffffffffffffe60;
  ulonglong uVar19;
  ulonglong uVar20;
  undefined8 uVar21;
  short local_168;
  undefined6 uStack_166;
  longlong lStack_160;
  undefined8 local_158;
  short local_150;
  undefined6 uStack_14e;
  longlong lStack_148;
  undefined8 local_140;
  short local_138;
  undefined2 uStack_136;
  undefined4 uStack_134;
  undefined8 uStack_130;
  undefined8 local_128;
  short local_120;
  undefined2 uStack_11e;
  undefined4 uStack_11c;
  undefined8 uStack_118;
  undefined8 local_110;
  ulonglong local_108;
  longlong lStack_100;
  undefined8 local_f8;
  undefined8 local_e8;
  longlong *plStack_e0;
  undefined8 local_d8;
  longlong *local_c8;
  IUnknown *local_c0;
  undefined8 local_b8;
  longlong lStack_b0;
  undefined8 local_a8;
  ulonglong local_98;
  longlong lStack_90;
  undefined8 local_88;
  undefined8 local_78;
  longlong lStack_70;
  undefined8 local_68;
  undefined4 local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  
  DAT_143add840 = DAT_143add840 + 1;
  if (DAT_143add840 == 0) {
    DAT_143add840 = 1;
  }
  *(int *)(param_1 + 7) = DAT_143add840;
  uVar17 = 0;
  param_1[0x16] = 0;
  param_1[0x17] = 0;
  *(undefined4 *)(param_1 + 0x2c) = param_10;
  uVar20 = (ulonglong)param_7;
  uVar19 = CONCAT44((int)((ulonglong)in_stack_fffffffffffffe60 >> 0x20),param_6);
  uVar18 = CONCAT44((int)((ulonglong)in_stack_fffffffffffffe58 >> 0x20),param_5);
  local_res10 = param_2;
  local_res18 = param_3;
  uVar21 = param_8;
  (**(code **)(*param_1 + 0x18))();
  if (param_1[8] == 0) {
    iVar12 = (int)param_1[0x10] + 1;
    *(int *)(param_1 + 0x10) = iVar12;
    param_10 = param_4;
    if (iVar12 == (iVar12 / 0x6f) * 0x6f) {
      puVar2 = (undefined8 *)param_1[0x11];
      puVar10 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      param_1[0x11] = (longlong)puVar10;
      *puVar10 = *puVar2;
      *(undefined4 *)(puVar10 + 1) = *(undefined4 *)(puVar2 + 1);
      thunk_FUN_140205820(puVar2,0xc);
    }
    uVar8 = FUN_142f04924();
    *(undefined1 *)(param_1[0x11] + 4) = uVar8;
    pbVar16 = (byte *)param_1[0x11];
    bVar15 = pbVar16[4];
    pbVar16[8] = 0x65;
    pbVar16[9] = 0x9a;
    lVar13 = (longlong)&param_10 - (longlong)pbVar16;
    lVar5 = 1 - (longlong)pbVar16;
    lVar6 = 2 - (longlong)pbVar16;
    lVar7 = 3 - (longlong)pbVar16;
    do {
      if (bVar15 == 0) {
        bVar15 = 0x2a;
      }
      bVar9 = pbVar16[lVar13];
      *pbVar16 = bVar15 ^ bVar9;
      bVar15 = bVar15 + (bVar15 ^ bVar9) + 0x2a;
      uVar1 = *(ushort *)(param_1[0x11] + 8);
      *(ushort *)(param_1[0x11] + 8) = (uVar1 >> 0xd) + (ushort)bVar15 | uVar1 << 3;
      bVar9 = 0x2a;
      if (bVar15 != 0) {
        bVar9 = bVar15;
      }
      bVar15 = pbVar16[(longlong)&param_10 + lVar5];
      pbVar16[1] = bVar9 ^ bVar15;
      bVar9 = (bVar9 ^ bVar15) + bVar9 + 0x2a;
      uVar1 = *(ushort *)(param_1[0x11] + 8);
      *(ushort *)(param_1[0x11] + 8) = (uVar1 >> 0xd) + (ushort)bVar9 | uVar1 << 3;
      bVar15 = 0x2a;
      if (bVar9 != 0) {
        bVar15 = bVar9;
      }
      bVar9 = pbVar16[(longlong)&param_10 + lVar6];
      pbVar16[2] = bVar15 ^ bVar9;
      bVar9 = (bVar15 ^ bVar9) + bVar15 + 0x2a;
      uVar1 = *(ushort *)(param_1[0x11] + 8);
      *(ushort *)(param_1[0x11] + 8) = (uVar1 >> 0xd) + (ushort)bVar9 | uVar1 << 3;
      bVar15 = 0x2a;
      if (bVar9 != 0) {
        bVar15 = bVar9;
      }
      bVar9 = pbVar16[(longlong)&param_10 + lVar7];
      pbVar16[3] = bVar15 ^ bVar9;
      bVar15 = (bVar15 ^ bVar9) + bVar15 + 0x2a;
      lVar3 = param_1[0x11];
      *(ushort *)(lVar3 + 8) =
           (*(ushort *)(lVar3 + 8) >> 0xd) + (ushort)bVar15 | *(ushort *)(lVar3 + 8) << 3;
      uVar17 = uVar17 + 4;
      pbVar16 = pbVar16 + 4;
    } while (uVar17 < 4);
    param_10 = param_5;
    iVar12 = (int)param_1[0x13] + 1;
    *(int *)(param_1 + 0x13) = iVar12;
    if (iVar12 == (iVar12 / 0x6f) * 0x6f) {
      puVar2 = (undefined8 *)param_1[0x14];
      puVar10 = (undefined8 *)
                FUN_14019b780(&DAT_143ad68a0,0xc,lVar3,bVar15,uVar18,uVar19,uVar20,uVar21);
      param_1[0x14] = (longlong)puVar10;
      *puVar10 = *puVar2;
      *(undefined4 *)(puVar10 + 1) = *(undefined4 *)(puVar2 + 1);
      thunk_FUN_140205820(puVar2,0xc);
    }
    uVar8 = FUN_142f04924();
    *(undefined1 *)(param_1[0x14] + 4) = uVar8;
    pbVar16 = (byte *)param_1[0x14];
    bVar15 = pbVar16[4];
    pbVar16[8] = 0x65;
    pbVar16[9] = 0x9a;
    uVar17 = 0;
    lVar13 = (longlong)&param_10 - (longlong)pbVar16;
    lVar5 = 1 - (longlong)pbVar16;
    lVar6 = 2 - (longlong)pbVar16;
    lVar7 = 3 - (longlong)pbVar16;
    do {
      if (bVar15 == 0) {
        bVar15 = 0x2a;
      }
      bVar9 = pbVar16[lVar13];
      *pbVar16 = bVar15 ^ bVar9;
      bVar15 = bVar15 + (bVar15 ^ bVar9) + 0x2a;
      uVar1 = *(ushort *)(param_1[0x14] + 8);
      *(ushort *)(param_1[0x14] + 8) = (uVar1 >> 0xd) + (ushort)bVar15 | uVar1 << 3;
      bVar9 = 0x2a;
      if (bVar15 != 0) {
        bVar9 = bVar15;
      }
      bVar15 = pbVar16[(longlong)&param_10 + lVar5];
      pbVar16[1] = bVar9 ^ bVar15;
      bVar9 = (bVar9 ^ bVar15) + bVar9 + 0x2a;
      uVar1 = *(ushort *)(param_1[0x14] + 8);
      *(ushort *)(param_1[0x14] + 8) = (uVar1 >> 0xd) + (ushort)bVar9 | uVar1 << 3;
      bVar15 = 0x2a;
      if (bVar9 != 0) {
        bVar15 = bVar9;
      }
      bVar9 = pbVar16[(longlong)&param_10 + lVar6];
      pbVar16[2] = bVar15 ^ bVar9;
      bVar9 = (bVar15 ^ bVar9) + bVar15 + 0x2a;
      uVar1 = *(ushort *)(param_1[0x14] + 8);
      *(ushort *)(param_1[0x14] + 8) = (uVar1 >> 0xd) + (ushort)bVar9 | uVar1 << 3;
      bVar15 = 0x2a;
      if (bVar9 != 0) {
        bVar15 = bVar9;
      }
      bVar9 = pbVar16[(longlong)&param_10 + lVar7];
      pbVar16[3] = bVar15 ^ bVar9;
      bVar15 = (bVar15 ^ bVar9) + bVar15 + 0x2a;
      uVar1 = *(ushort *)(param_1[0x14] + 8);
      *(ushort *)(param_1[0x14] + 8) = (uVar1 >> 0xd) + (ushort)bVar15 | uVar1 << 3;
      pIVar4 = DAT_143add050;
      uVar17 = uVar17 + 4;
      pbVar16 = pbVar16 + 4;
    } while (uVar17 < 4);
    if (DAT_143add050 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_138);
    if (DAT_143a8b8d8 == 8) {
      if (local_138 == 8) {
        local_138 = 0;
        if (uStack_130 != 0) {
          (*DAT_143ad5990)(uStack_130 + -4);
        }
      }
      else {
        iVar12 = (*DAT_143262a18)(&local_138);
        if (iVar12 < 0) goto LAB_142bf3d89;
      }
      local_138 = 8;
      if (DAT_143a8b8e0 == 0) {
        uVar17 = 0;
      }
      else {
        uVar17 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      uStack_130 = FUN_1401a5fa0(DAT_143a8b8e0,uVar17);
    }
    else {
      if ((local_138 == 8) && (local_138 = 0, uStack_130 != 0)) {
        (*DAT_143ad5990)(uStack_130 + -4);
      }
      iVar12 = (*DAT_143262a28)(&local_138,&DAT_143a8b8d8);
      if (iVar12 < 0) {
LAB_142bf3d89:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar12);
      }
    }
    (*DAT_143262a20)(&local_150);
    if (DAT_143a8b8d8 == 8) {
      if (local_150 == 8) {
        local_150 = 0;
        if (lStack_148 != 0) {
          (*DAT_143ad5990)(lStack_148 + -4);
        }
      }
      else {
        iVar12 = (*DAT_143262a18)(&local_150);
        if (iVar12 < 0) goto LAB_142bf3d91;
      }
      local_150 = 8;
      if (DAT_143a8b8e0 == 0) {
        uVar17 = 0;
      }
      else {
        uVar17 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_148 = FUN_1401a5fa0(DAT_143a8b8e0,uVar17);
    }
    else {
      if ((local_150 == 8) && (local_150 = 0, lStack_148 != 0)) {
        (*DAT_143ad5990)(lStack_148 + -4);
      }
      iVar12 = (*DAT_143262a28)(&local_150,&DAT_143a8b8d8);
      if (iVar12 < 0) {
LAB_142bf3d91:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar12);
      }
    }
    (*DAT_143262a20)(&local_168);
    if (DAT_143a8b8d8 == 8) {
      if (local_168 == 8) {
        local_168 = 0;
        if (lStack_160 != 0) {
          (*DAT_143ad5990)(lStack_160 + -4);
        }
      }
      else {
        iVar12 = (*DAT_143262a18)(&local_168);
        if (iVar12 < 0) goto LAB_142bf3d99;
      }
      local_168 = 8;
      if (DAT_143a8b8e0 == 0) {
        uVar17 = 0;
      }
      else {
        uVar17 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_160 = FUN_1401a5fa0(DAT_143a8b8e0,uVar17);
    }
    else {
      if ((local_168 == 8) && (local_168 = 0, lStack_160 != 0)) {
        (*DAT_143ad5990)(lStack_160 + -4);
      }
      iVar12 = (*DAT_143262a28)(&local_168,&DAT_143a8b8d8);
      if (iVar12 < 0) {
LAB_142bf3d99:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar12);
      }
    }
    uVar17 = uStack_118._4_4_;
    local_120 = 3;
    uStack_118 = (longlong *)((ulonglong)uStack_118._4_4_ << 0x20);
    local_res8 = (IUnknown *)0x0;
    local_108 = CONCAT44(uStack_134,CONCAT22(uStack_136,local_138));
    lStack_100 = uStack_130;
    local_f8 = local_128;
    local_b8 = CONCAT62(uStack_14e,local_150);
    lStack_b0 = lStack_148;
    local_a8 = local_140;
    local_98 = CONCAT62(uStack_166,local_168);
    lStack_90 = lStack_160;
    local_88 = local_158;
    local_e8 = CONCAT44(uStack_11c,CONCAT22(uStack_11e,3));
    plStack_e0 = (longlong *)((ulonglong)uVar17 << 0x20);
    local_d8 = local_110;
    iVar12 = (**(code **)(*(longlong *)pIVar4 + 0x168))
                       (pIVar4,0,0,0,uVar18 & 0xffffffff00000000,uVar19 & 0xffffffff00000000,
                        &local_e8,&local_98,&local_b8,&local_108,&local_res8);
    if (iVar12 < 0) {
      _com_issue_errorex(iVar12,pIVar4,(_GUID *)&DAT_14327fcd0);
    }
    pIVar4 = (IUnknown *)param_1[8];
    pIVar14 = local_res8;
    if (pIVar4 != local_res8) {
      param_1[8] = (longlong)local_res8;
      pIVar14 = (IUnknown *)0x0;
      if (pIVar4 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)pIVar4 + 0x10))();
      }
    }
    if (pIVar14 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar14 + 0x10))(pIVar14);
    }
    if (local_120 == 8) {
      local_120 = 0;
      if (uStack_118 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)uStack_118 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_120);
    }
    if (local_168 == 8) {
      local_168 = 0;
      if (lStack_160 != 0) {
        (*DAT_143ad5990)(lStack_160 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_168);
    }
    if (local_150 == 8) {
      local_150 = 0;
      if (lStack_148 != 0) {
        (*DAT_143ad5990)(lStack_148 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_150);
    }
    if (local_138 == 8) {
      local_138 = 0;
      if (uStack_130 != 0) {
        (*DAT_143ad5990)(uStack_130 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_138);
    }
    if (*(uint *)(param_1 + 0x41) < 2) {
      pIVar4 = (IUnknown *)param_1[8];
      if (pIVar4 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      local_res8 = (IUnknown *)0x0;
      iVar12 = (**(code **)(*(longlong *)pIVar4 + 0x1c0))(pIVar4,&local_res8);
      if (iVar12 < 0) {
        _com_issue_errorex(iVar12,pIVar4,(_GUID *)&DAT_14327fcb0);
      }
      pIVar4 = local_res8;
      local_c0 = local_res8;
      if (local_res8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      uStack_118 = (longlong *)FUN_142c137a0(DAT_143abfdf8,&local_c8,(int)param_1[0x2c]);
      uStack_118 = (longlong *)*uStack_118;
      local_120 = 0xd;
      if (uStack_118 != (longlong *)0x0) {
        (**(code **)(*uStack_118 + 8))();
      }
      local_e8 = CONCAT44(uStack_11c,CONCAT22(uStack_11e,local_120));
      plStack_e0 = uStack_118;
      local_d8 = local_110;
      iVar12 = (**(code **)(*(longlong *)pIVar4 + 200))(pIVar4,&local_e8);
      if (iVar12 < 0) {
        _com_issue_errorex(iVar12,pIVar4,(_GUID *)&DAT_143273488);
      }
      if (local_120 == 8) {
        local_120 = 0;
        if (uStack_118 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)uStack_118 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_120);
      }
      if (local_c8 != (longlong *)0x0) {
        (**(code **)(*local_c8 + 0x10))();
      }
      (**(code **)(*(longlong *)pIVar4 + 0x10))(pIVar4);
    }
    pIVar4 = (IUnknown *)param_1[8];
    if (pIVar4 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res8 = (IUnknown *)0x0;
    iVar12 = (**(code **)(*(longlong *)pIVar4 + 0x1c0))(pIVar4,&local_res8);
    if (iVar12 < 0) {
      _com_issue_errorex(iVar12,pIVar4,(_GUID *)&DAT_14327fcb0);
    }
    pIVar4 = local_res8;
    local_c0 = local_res8;
    if (local_res8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_120);
    if (DAT_143a8b8d8 == 8) {
      if (local_120 == 8) {
        local_120 = 0;
        if (uStack_118 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)uStack_118 + -4);
        }
      }
      else {
        iVar12 = (*DAT_143262a18)(&local_120);
        if (iVar12 < 0) goto LAB_142bf3da1;
      }
      local_120 = 8;
      if (DAT_143a8b8e0 == 0) {
        uVar17 = 0;
      }
      else {
        uVar17 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      uStack_118 = (longlong *)FUN_1401a5fa0(DAT_143a8b8e0,uVar17);
    }
    else {
      if ((local_120 == 8) && (local_120 = 0, uStack_118 != (longlong *)0x0)) {
        (*DAT_143ad5990)((longlong)uStack_118 + -4);
      }
      iVar12 = (*DAT_143262a28)(&local_120,&DAT_143a8b8d8);
      if (iVar12 < 0) {
LAB_142bf3da1:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar12);
      }
    }
    (*DAT_143262a20)(&local_108);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_108 == 8) {
        local_108 = (ulonglong)local_108._2_6_ << 0x10;
        if (lStack_100 != 0) {
          (*DAT_143ad5990)(lStack_100 + -4);
        }
      }
      else {
        iVar12 = (*DAT_143262a18)(&local_108);
        if (iVar12 < 0) goto LAB_142bf3da9;
      }
      local_108 = CONCAT62(local_108._2_6_,8);
      if (DAT_143a8b8e0 == 0) {
        uVar17 = 0;
      }
      else {
        uVar17 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_100 = FUN_1401a5fa0(DAT_143a8b8e0,uVar17);
    }
    else {
      if (((short)local_108 == 8) &&
         (local_108 = (ulonglong)local_108._2_6_ << 0x10, lStack_100 != 0)) {
        (*DAT_143ad5990)(lStack_100 + -4);
      }
      iVar12 = (*DAT_143262a28)(&local_108,&DAT_143a8b8d8);
      if (iVar12 < 0) {
LAB_142bf3da9:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar12);
      }
    }
    (*DAT_143262a20)(&local_168);
    if (DAT_143a8b8d8 == 8) {
      if (local_168 == 8) {
        local_168 = 0;
        if (lStack_160 != 0) {
          (*DAT_143ad5990)(lStack_160 + -4);
        }
      }
      else {
        iVar12 = (*DAT_143262a18)(&local_168);
        if (iVar12 < 0) goto LAB_142bf3db1;
      }
      local_168 = 8;
      if (DAT_143a8b8e0 == 0) {
        uVar17 = 0;
      }
      else {
        uVar17 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_160 = FUN_1401a5fa0(DAT_143a8b8e0,uVar17);
    }
    else {
      if ((local_168 == 8) && (local_168 = 0, lStack_160 != 0)) {
        (*DAT_143ad5990)(lStack_160 + -4);
      }
      iVar12 = (*DAT_143262a28)(&local_168,&DAT_143a8b8d8);
      if (iVar12 < 0) {
LAB_142bf3db1:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar12);
      }
    }
    (*DAT_143262a20)(&local_150);
    if (DAT_143a8b8d8 == 8) {
      if (local_150 == 8) {
        local_150 = 0;
        if (lStack_148 != 0) {
          (*DAT_143ad5990)(lStack_148 + -4);
        }
      }
      else {
        iVar12 = (*DAT_143262a18)(&local_150);
        if (iVar12 < 0) goto LAB_142bf3db9;
      }
      local_150 = 8;
      if (DAT_143a8b8e0 == 0) {
        uVar17 = 0;
      }
      else {
        uVar17 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_148 = FUN_1401a5fa0(DAT_143a8b8e0,uVar17);
    }
    else {
      if ((local_150 == 8) && (local_150 = 0, lStack_148 != 0)) {
        (*DAT_143ad5990)(lStack_148 + -4);
      }
      iVar12 = (*DAT_143262a28)(&local_150,&DAT_143a8b8d8);
      if (iVar12 < 0) {
LAB_142bf3db9:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar12);
      }
    }
    (*DAT_143262a20)(&local_138);
    if (DAT_143a8b8d8 == 8) {
      if (local_138 == 8) {
        local_138 = 0;
        if (uStack_130 != 0) {
          (*DAT_143ad5990)(uStack_130 + -4);
        }
      }
      else {
        iVar12 = (*DAT_143262a18)(&local_138);
        if (iVar12 < 0) goto LAB_142bf3dc1;
      }
      local_138 = 8;
      if (DAT_143a8b8e0 == 0) {
        uVar17 = 0;
      }
      else {
        uVar17 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      uStack_130 = FUN_1401a5fa0(DAT_143a8b8e0,uVar17);
    }
    else {
      if ((local_138 == 8) && (local_138 = 0, uStack_130 != 0)) {
        (*DAT_143ad5990)(uStack_130 + -4);
      }
      iVar12 = (*DAT_143262a28)(&local_138,&DAT_143a8b8d8);
      if (iVar12 < 0) {
LAB_142bf3dc1:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar12);
      }
    }
    local_e8 = CONCAT44(uStack_11c,CONCAT22(uStack_11e,local_120));
    plStack_e0 = uStack_118;
    local_d8 = local_110;
    local_98 = local_108;
    lStack_90 = lStack_100;
    local_88 = local_f8;
    local_b8 = CONCAT62(uStack_166,local_168);
    lStack_b0 = lStack_160;
    local_a8 = local_158;
    local_78 = CONCAT62(uStack_14e,local_150);
    lStack_70 = lStack_148;
    local_68 = local_140;
    local_58 = CONCAT22(uStack_136,local_138);
    uStack_54 = uStack_134;
    uStack_50 = (undefined4)uStack_130;
    uStack_4c = uStack_130._4_4_;
    local_48 = local_128;
    iVar12 = (**(code **)(*(longlong *)pIVar4 + 0x140))
                       (pIVar4,local_res10,local_res18,&local_58,&local_78,&local_b8,&local_98,
                        &local_e8);
    if (iVar12 < 0) {
      _com_issue_errorex(iVar12,pIVar4,(_GUID *)&DAT_143273488);
    }
    if (local_138 == 8) {
      local_138 = 0;
      if (uStack_130 != 0) {
        (*DAT_143ad5990)(uStack_130 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_138);
    }
    if (local_150 == 8) {
      local_150 = 0;
      if (lStack_148 != 0) {
        (*DAT_143ad5990)(lStack_148 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_150);
    }
    if (local_168 == 8) {
      local_168 = 0;
      if (lStack_160 != 0) {
        (*DAT_143ad5990)(lStack_160 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_168);
    }
    if ((short)local_108 == 8) {
      local_108 = local_108 & 0xffffffffffff0000;
      if (lStack_100 != 0) {
        (*DAT_143ad5990)(lStack_100 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_108);
    }
    if (local_120 == 8) {
      local_120 = 0;
      if (uStack_118 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)uStack_118 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_120);
    }
    (**(code **)(*(longlong *)pIVar4 + 0x10))(pIVar4);
    pIVar4 = (IUnknown *)param_1[8];
    if (pIVar4 == (IUnknown *)0x0) goto LAB_142bf3dc9;
    iVar12 = (**(code **)(*(longlong *)pIVar4 + 0x198))(pIVar4,param_6);
    if (iVar12 < 0) {
      _com_issue_errorex(iVar12,pIVar4,(_GUID *)&DAT_14327fcb0);
    }
    pIVar4 = (IUnknown *)param_1[8];
    if (pIVar4 == (IUnknown *)0x0) goto LAB_142bf3dc9;
    iVar12 = (**(code **)(*(longlong *)pIVar4 + 0x200))(pIVar4,0xffffffff);
    if (iVar12 < 0) {
      _com_issue_errorex(iVar12,pIVar4,(_GUID *)&DAT_14327fcb0);
    }
  }
  pIVar4 = (IUnknown *)param_1[8];
  if (pIVar4 != (IUnknown *)0x0) {
    iVar12 = (**(code **)(*(longlong *)pIVar4 + 0x350))(pIVar4,1);
    if (iVar12 < 0) {
      _com_issue_errorex(iVar12,pIVar4,(_GUID *)&DAT_14327fcb0);
    }
    (**(code **)(*param_1 + 0x20))(param_1,param_8);
    (**(code **)(*param_1 + 0x90))(param_1,0);
    FUN_142c05480(param_1);
    if (param_9 != 0) {
      FUN_142c0bf50(DAT_143abfdf8,param_1 + 1,0);
    }
    if (((int)param_1[0x1a] != 0) && (DAT_143abfdf8 != 0)) {
      FUN_142c0c190(DAT_143abfdf8,&local_res8,0,1);
      plVar11 = (longlong *)
                FUN_142c12c90(DAT_143abfdf8,(ulonglong)local_res8 & 0xffffffff,local_res8._4_4_);
      if ((plVar11 != (longlong *)0x0) &&
         (iVar12 = (**(code **)(*plVar11 + 0x78))(plVar11), iVar12 != 0)) {
        (**(code **)(*plVar11 + 0x30))(plVar11,1);
      }
    }
    return;
  }
LAB_142bf3dc9:
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x80004003);
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
// FUN_142644810 @ 142644810   (1337 bytes)
//===========================================================

undefined8 * FUN_142644810(undefined8 *param_1)

{
  int *piVar1;
  int iVar2;
  char cVar3;
  longlong *plVar4;
  longlong *plVar5;
  longlong *plVar6;
  longlong lVar7;
  longlong *plVar8;
  longlong *plVar9;
  longlong *plVar10;
  longlong lVar11;
  longlong lVar12;
  undefined4 *puVar13;
  undefined8 uStack_60;
  longlong *local_48;
  ulonglong uStack_40;
  
  param_1[3] = 0;
  param_1[1] = 0;
  param_1[2] = 0;
  *param_1 = &PTR_FUN_14327fce8;
  param_1[4] = 0;
  param_1[5] = 0;
  param_1[6] = 0;
  param_1[9] = 0;
  param_1[10] = 0;
  param_1[0xb] = 0;
  param_1[0xc] = 0;
  param_1[0xd] = 0;
  param_1[0xe] = 0;
  param_1[0xf] = 0;
  param_1[0x11] = 0;
  param_1[0x12] = 0;
  lVar7 = FUN_14019b780(&DAT_143ad68a0,0x28);
  *(longlong *)lVar7 = lVar7;
  *(longlong *)(lVar7 + 8) = lVar7;
  *(longlong *)(lVar7 + 0x10) = lVar7;
  *(undefined2 *)(lVar7 + 0x18) = 0x101;
  param_1[0x11] = lVar7;
  param_1[0x13] = 0;
  lVar7 = 0x22;
  _eh_vector_constructor_iterator_
            (param_1 + 0x15,0x20,0x22,(_func_void_void_ptr *)&LAB_1426d4b80,FUN_140331880);
  _eh_vector_constructor_iterator_
            (param_1 + 0x9f,0x20,0x22,(_func_void_void_ptr *)&LAB_1426d4b80,FUN_140331880);
  plVar8 = param_1 + 0x127;
  _eh_vector_constructor_iterator_
            (plVar8,0x18,0x22,(_func_void_void_ptr *)&LAB_1426d5100,
             (_func_void_void_ptr *)&LAB_140331a20);
  _eh_vector_constructor_iterator_
            (param_1 + 400,0x20,0x22,(_func_void_void_ptr *)&LAB_1426d4b80,FUN_140331880);
  *(undefined4 *)(param_1 + 0x219) = 0;
  param_1[0x21b] = 0;
  param_1[0x21c] = 0;
  param_1[0x21d] = 0;
  param_1[0x21e] = 0;
  param_1[0x21f] = 0;
  *(undefined4 *)(param_1 + 0x220) = 0;
  param_1[0x221] = 0;
  param_1[0x222] = 0;
  param_1[0x223] = 0;
  param_1[0x224] = 0x271a;
  FUN_1426dc3a0(&DAT_143adb3b0);
  if (DAT_143adb3b8 != 0) {
    LOCK();
    *(int *)(DAT_143adb3b8 + 8) = *(int *)(DAT_143adb3b8 + 8) + 1;
    UNLOCK();
  }
  lVar12 = DAT_143adb3b8;
  param_1[5] = DAT_143adb3b0;
  plVar9 = (longlong *)param_1[6];
  param_1[6] = lVar12;
  if (plVar9 != (longlong *)0x0) {
    LOCK();
    plVar10 = plVar9 + 1;
    lVar12 = *plVar10;
    *(int *)plVar10 = (int)*plVar10 + -1;
    UNLOCK();
    if ((int)lVar12 == 1) {
      (**(code **)*plVar9)(plVar9);
      LOCK();
      piVar1 = (int *)((longlong)plVar9 + 0xc);
      iVar2 = *piVar1;
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (iVar2 == 1) {
        (**(code **)(*plVar9 + 8))(plVar9);
      }
    }
  }
  param_1[7] = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  if ((longlong *)param_1[9] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[9] + 0x10))();
  }
  param_1[9] = 0;
  if ((longlong *)param_1[10] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[10] + 0x10))();
  }
  param_1[10] = 0;
  if ((longlong *)param_1[0xd] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0xd] + 0x10))();
  }
  param_1[0xd] = 0;
  if ((longlong *)param_1[0xe] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0xe] + 0x10))();
  }
  param_1[0xe] = 0;
  if ((longlong *)param_1[0xb] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0xb] + 0x10))();
  }
  param_1[0xb] = 0;
  if ((longlong *)param_1[0xc] != (longlong *)0x0) {
    (**(code **)(*(longlong *)param_1[0xc] + 0x10))();
  }
  param_1[0xc] = 0;
  *(undefined4 *)(param_1 + 0x14) = 0;
  param_1[0x18e] = 0;
  *(undefined4 *)(param_1 + 399) = 0;
  *(undefined4 *)(param_1 + 0x218) = 0;
  *(undefined4 *)((longlong)param_1 + 0x10c4) = 0xffffffff;
  *(undefined4 *)(param_1 + 0x220) = 0;
  *(undefined1 *)(param_1 + 0x9d) = 0;
  *(undefined8 *)((longlong)param_1 + 0x4ec) = 0;
  puVar13 = (undefined4 *)((longlong)param_1 + 0xc4);
  do {
    *(undefined8 *)(puVar13 + -7) = 0;
    *puVar13 = 0;
    *(undefined8 *)(puVar13 + 0x2ef) = 0;
    puVar13[0x2f6] = 0;
    lVar12 = plVar8[1];
    lVar11 = *plVar8;
    if (lVar11 != lVar12) {
      do {
        if (*(longlong *)(lVar11 + 8) != 0) {
          FUN_14019f2c0(*(longlong *)(lVar11 + 8) + -0x10);
        }
        lVar11 = lVar11 + 0x10;
      } while (lVar11 != lVar12);
      lVar11 = *plVar8;
    }
    plVar8[1] = lVar11;
    *(undefined8 *)(puVar13 + 0x10d) = 0;
    puVar13[0x114] = 0;
    plVar8 = plVar8 + 3;
    puVar13 = puVar13 + 8;
    lVar7 = lVar7 + -1;
  } while (lVar7 != 0);
  lVar7 = param_1[0x21c];
  lVar12 = param_1[0x21d];
  if (lVar7 != lVar12) {
    do {
      uStack_60 = *(undefined8 **)(lVar7 + 8);
      if (uStack_60 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)uStack_60[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        uStack_60[1] = uStack_60[1] + 1;
        UNLOCK();
      }
      if (uStack_60 == (undefined8 *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_142645170(uStack_60);
      if (uStack_60 != (undefined8 *)0x0) {
        if (0xffffe < uStack_60[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar8 = uStack_60 + 1;
        lVar11 = *plVar8;
        *plVar8 = *plVar8 + -1;
        UNLOCK();
        if (((int)lVar11 == 1) && (uStack_60 != (undefined8 *)0x0)) {
          (**(code **)*uStack_60)(uStack_60,1);
        }
        uStack_60 = (undefined8 *)0x0;
      }
      lVar7 = lVar7 + 0x10;
    } while (lVar7 != lVar12);
    lVar7 = param_1[0x21d];
    lVar12 = param_1[0x21c];
    if (lVar12 != lVar7) {
      do {
        FUN_140336390(lVar12);
        lVar12 = lVar12 + 0x10;
      } while (lVar12 != lVar7);
      lVar12 = param_1[0x21c];
    }
    param_1[0x21d] = lVar12;
  }
  if ((DAT_143adb3c8 == (longlong *)0x0) &&
     (plVar8 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x10), plVar8 != (longlong *)0x0)) {
    *plVar8 = 0;
    plVar8[1] = 0;
    DAT_143adb3c8 = plVar8;
    *plVar8 = 0;
    plVar8[1] = 0;
    lVar7 = FUN_14019b780(&DAT_143ad68a0,0x28);
    *(longlong *)lVar7 = lVar7;
    *(longlong *)(lVar7 + 8) = lVar7;
    *(longlong *)(lVar7 + 0x10) = lVar7;
    *(undefined2 *)(lVar7 + 0x18) = 0x101;
    *plVar8 = lVar7;
  }
  plVar6 = DAT_143adb3c8;
  plVar8 = (longlong *)*DAT_143adb3c8;
  plVar9 = (longlong *)plVar8[1];
  uStack_60 = (undefined8 *)((ulonglong)uStack_60 & 0xffffffff00000000);
  cVar3 = *(char *)((longlong)plVar9 + 0x19);
  plVar10 = plVar8;
  plVar5 = plVar9;
  while (plVar4 = plVar9, cVar3 == '\0') {
    uStack_60._4_4_ = (uint)((ulonglong)uStack_60 >> 0x20);
    if ((undefined8 *)plVar4[4] < param_1) {
      uStack_60 = (undefined8 *)((ulonglong)uStack_60._4_4_ << 0x20);
      plVar9 = (longlong *)plVar4[2];
    }
    else {
      uStack_60 = (undefined8 *)CONCAT44(uStack_60._4_4_,1);
      plVar9 = (longlong *)*plVar4;
      plVar10 = plVar4;
    }
    cVar3 = *(char *)((longlong)plVar9 + 0x19);
    plVar5 = plVar4;
  }
  if ((*(char *)((longlong)plVar10 + 0x19) != '\0') || (param_1 < (undefined8 *)plVar10[4])) {
    if (DAT_143adb3c8[1] == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
      FUN_14019f9d0();
    }
    local_48 = DAT_143adb3c8;
    uStack_40 = 0;
    plVar9 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x28);
    plVar9[4] = (longlong)param_1;
    *plVar9 = (longlong)plVar8;
    plVar9[1] = (longlong)plVar8;
    plVar9[2] = (longlong)plVar8;
    *(undefined2 *)(plVar9 + 3) = 0;
    local_48 = plVar5;
    uStack_40 = (ulonglong)uStack_60;
    FUN_1426e2550(plVar6,&local_48,plVar9);
  }
  return param_1;
}



//===========================================================
// FUN_14090f3b0 @ 14090f3b0   (128 bytes)
//===========================================================

longlong * FUN_14090f3b0(longlong *param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  longlong *local_res8 [4];
  short local_28 [4];
  longlong local_20;
  
  local_res8[0] = param_1;
  uVar1 = FUN_14090df00(local_28,*param_2);
  uVar1 = FUN_1409339d0(local_res8,uVar1);
  FUN_1403ee040(param_1,uVar1);
  if (local_res8[0] != (longlong *)0x0) {
    (**(code **)(*local_res8[0] + 0x10))();
  }
  if (local_28[0] == 8) {
    local_28[0] = 0;
    if (local_20 != 0) {
      (*DAT_143ad5990)(local_20 + -4);
    }
  }
  else {
    (*DAT_143262a18)(local_28);
  }
  return param_1;
}



//===========================================================
// FUN_142bf1bb0 @ 142bf1bb0   (767 bytes)
//===========================================================

undefined8 * FUN_142bf1bb0(undefined8 *param_1)

{
  char cVar1;
  int iVar2;
  bool bVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  longlong lVar6;
  undefined8 *puVar7;
  undefined8 *puVar8;
  undefined8 *local_48;
  undefined8 uStack_40;
  undefined8 *local_38;
  uint uStack_30;
  undefined4 uStack_2c;
  
  *param_1 = &PTR_LAB_143296d40;
  FUN_141d5f4b0(param_1 + 1);
  param_1[6] = 0;
  param_1[4] = 0;
  param_1[5] = 0;
  *param_1 = &PTR_FUN_14348eee0;
  param_1[1] = &PTR_LAB_14348f010;
  param_1[3] = &PTR_FUN_14348f0e8;
  *(undefined4 *)(param_1 + 7) = 0;
  param_1[8] = 0;
  param_1[9] = 0;
  param_1[10] = 0;
  param_1[0xb] = 0;
  param_1[0xc] = 0;
  param_1[0xd] = 0x1f;
  *(undefined4 *)(param_1 + 0xe) = 100;
  *(undefined4 *)((longlong)param_1 + 0x74) = 0x18;
  *(undefined1 *)(param_1 + 0xf) = 0;
  FUN_14022dad0(param_1 + 0x10);
  FUN_14022dad0(param_1 + 0x13);
  *(undefined8 *)((longlong)param_1 + 0xc4) = 0;
  *(undefined4 *)((longlong)param_1 + 0xcc) = 1;
  param_1[0x1a] = 1;
  param_1[0x1b] = 0;
  *(undefined4 *)(param_1 + 0x1c) = 0;
  FUN_1409d2160(param_1 + 0x1d);
  *(undefined4 *)((longlong)param_1 + 0x11c) = 0;
  param_1[0x24] = 0;
  param_1[0x25] = 0;
  param_1[0x26] = 0;
  param_1[0x27] = 0;
  param_1[0x28] = 0;
  param_1[0x29] = 0x1f;
  *(undefined4 *)(param_1 + 0x2a) = 100;
  *(undefined4 *)((longlong)param_1 + 0x154) = 0x18;
  param_1[0x2b] = 0;
  *(undefined4 *)(param_1 + 0x2c) = 0;
  *(undefined4 *)((longlong)param_1 + 0x164) = 1;
  param_1[0x2d] = 0;
  param_1[0x2e] = 0;
  param_1[0x2f] = 0;
  param_1[0x30] = 0xff;
  param_1[0x31] = 0;
  param_1[0x32] = 0;
  param_1[0x33] = 0;
  param_1[0x34] = 0;
  param_1[0x35] = 0;
  param_1[0x36] = 0;
  param_1[0x37] = 0;
  param_1[0x38] = 0;
  param_1[0x39] = 0;
  param_1[0x3a] = 0;
  *(undefined1 *)(param_1 + 0x3b) = 0;
  param_1[0x3c] = 0;
  param_1[0x3d] = 0;
  param_1[0x3e] = 0;
  *(undefined4 *)(param_1 + 0x3f) = 0xff;
  param_1[0x40] = 0;
  param_1[0x41] = 1;
  *(undefined4 *)(param_1 + 0x42) = 0;
  param_1[0x43] = 0;
  param_1[0x44] = 0;
  lVar6 = FUN_14019b780(&DAT_143ad68a0,0x50);
  *(longlong *)lVar6 = lVar6;
  *(longlong *)(lVar6 + 8) = lVar6;
  param_1[0x43] = lVar6;
  *(undefined4 *)(param_1 + 0x45) = 0;
  puVar5 = DAT_143add860;
  iVar2 = *(int *)(param_1 + 2);
  puVar7 = (undefined8 *)DAT_143add860[1];
  uStack_30 = 0;
  cVar1 = *(char *)((longlong)puVar7 + 0x19);
  local_38 = puVar7;
  puVar8 = DAT_143add860;
  while (puVar4 = puVar7, cVar1 == '\0') {
    bVar3 = iVar2 <= *(int *)((longlong)puVar4 + 0x1c);
    if (bVar3) {
      puVar7 = (undefined8 *)*puVar4;
      puVar8 = puVar4;
    }
    else {
      puVar7 = (undefined8 *)puVar4[2];
    }
    uStack_30 = (uint)bVar3;
    cVar1 = *(char *)((longlong)puVar7 + 0x19);
    local_38 = puVar4;
  }
  if ((*(char *)((longlong)puVar8 + 0x19) != '\0') || (iVar2 < *(int *)((longlong)puVar8 + 0x1c))) {
    if (DAT_143add868 == 0x7ffffffffffffff) {
                    /* WARNING: Subroutine does not return */
      FUN_14019f9d0();
    }
    local_48 = &DAT_143add860;
    uStack_40 = 0;
    puVar7 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x20);
    *(int *)((longlong)puVar7 + 0x1c) = iVar2;
    *puVar7 = puVar5;
    puVar7[1] = puVar5;
    puVar7[2] = puVar5;
    *(undefined2 *)(puVar7 + 3) = 0;
    local_48 = local_38;
    uStack_40 = CONCAT44(uStack_2c,uStack_30);
    FUN_142c04780(&DAT_143add860,&local_48,puVar7);
  }
  FUN_142bfcd30(param_1);
  return param_1;
}



//===========================================================
// FUN_141aa3a20 @ 141aa3a20   (57 bytes)
//===========================================================

undefined8 * FUN_141aa3a20(undefined8 *param_1)

{
  longlong lVar1;
  undefined8 uVar2;
  
  lVar1 = FUN_14019b780(&DAT_143ad68a0,0x460);
  uVar2 = 0;
  if (lVar1 != 0) {
    uVar2 = FUN_141aef1d0(lVar1);
  }
  *param_1 = uVar2;
  return param_1;
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
// FUN_14019d350 @ 14019d350   (105 bytes)
//===========================================================

longlong * FUN_14019d350(longlong param_1,ulonglong param_2)

{
  code *pcVar1;
  undefined8 uVar2;
  longlong *plVar3;
  
  if (0xc7fffff < param_2) {
    FUN_142e541f0(0x3a);
  }
  pcVar1 = DAT_143ad5528;
  uVar2 = (*DAT_143ad5538)();
  plVar3 = (longlong *)(*pcVar1)(uVar2,0,param_1 + 8);
  if (plVar3 != (longlong *)0x0) {
    *plVar3 = param_1;
    return plVar3 + 1;
  }
  return (longlong *)0x0;
}



//===========================================================
// FUN_14019d3c0 @ 14019d3c0   (144 bytes)
//===========================================================

undefined8 * FUN_14019d3c0(longlong param_1,longlong param_2)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  
  puVar3 = (undefined8 *)FUN_14019d350((param_1 + 8) * param_2 + 8,param_1 * param_2);
  *puVar3 = 0;
  puVar1 = puVar3 + 2;
  puVar3[1] = param_1;
  param_2 = param_2 + -1;
  puVar3 = puVar1;
  if (param_2 == 0) {
    *puVar1 = 0;
    return puVar1;
  }
  do {
    puVar2 = (undefined8 *)((longlong)puVar3 + param_1 + 8);
    *puVar3 = puVar2;
    puVar2[-1] = param_1;
    param_2 = param_2 + -1;
    puVar3 = puVar2;
  } while (param_2 != 0);
  *puVar2 = 0;
  return puVar1;
}



//===========================================================
// FUN_142c05480 @ 142c05480   (1067 bytes)
//===========================================================

void FUN_142c05480(longlong param_1)

{
  IUnknown *pIVar1;
  undefined8 *puVar2;
  IUnknown *pIVar3;
  char cVar4;
  int iVar5;
  int iVar6;
  longlong *plVar7;
  longlong lVar8;
  undefined4 *puVar9;
  IUnknown *pIVar10;
  undefined8 *puVar11;
  ulonglong uVar12;
  undefined8 *puVar13;
  IUnknown *pIVar14;
  IUnknown *pIVar15;
  IUnknown *local_res8;
  longlong *local_res10;
  longlong *local_res18;
  IUnknown *local_res20;
  longlong *local_78;
  uint local_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined8 local_60;
  uint local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  
  if (param_1 != 0) {
    pIVar15 = *(IUnknown **)(param_1 + 0x40);
    if (pIVar15 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_58);
    iVar5 = (**(code **)(*(longlong *)pIVar15 + 0x230))(pIVar15,&local_58);
    if (iVar5 < 0) {
      _com_issue_errorex(iVar5,pIVar15,(_GUID *)&DAT_14327fcb0);
    }
    local_70 = local_58;
    uStack_6c = uStack_54;
    uStack_68 = uStack_50;
    uStack_64 = uStack_4c;
    local_60 = local_48;
    pIVar15 = (IUnknown *)0x0;
    local_58 = local_58 & 0xffff0000;
    plVar7 = (longlong *)FUN_1409339d0(&local_78,&local_70);
    local_res20 = (IUnknown *)0x0;
    plVar7 = (longlong *)*plVar7;
    if (plVar7 == (longlong *)0x0) {
      iVar5 = -0x7fffbffe;
    }
    else {
      (**(code **)(*plVar7 + 8))(plVar7);
      local_res8 = (IUnknown *)0x0;
      iVar5 = (**(code **)*plVar7)(plVar7,&DAT_14327fcb0,&local_res8);
      local_res20 = pIVar15;
      if (-1 < iVar5) {
        local_res20 = local_res8;
      }
    }
    pIVar3 = local_res20;
    if (plVar7 != (longlong *)0x0) {
      (**(code **)(*plVar7 + 0x10))(plVar7);
    }
    if (((iVar5 + 0x80000000U & 0x80000000) == 0) && (iVar5 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar5);
    }
    if (local_78 != (longlong *)0x0) {
      (**(code **)(*local_78 + 0x10))();
    }
    if ((short)local_70 == 8) {
      local_70 = local_70 & 0xffff0000;
      if (CONCAT44(uStack_64,uStack_68) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_64,uStack_68) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_70);
    }
    pIVar10 = *(IUnknown **)(param_1 + 0x40);
    if (pIVar10 == (IUnknown *)0x0) {
LAB_142c0587a:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    local_res8 = (IUnknown *)((ulonglong)local_res8 & 0xffffffff00000000);
    iVar5 = (**(code **)(*(longlong *)pIVar10 + 400))(pIVar10,&local_res8);
    if (iVar5 < 0) {
      _com_issue_errorex(iVar5,pIVar10,(_GUID *)&DAT_14327fcb0);
    }
    if (pIVar3 == (IUnknown *)0x0) {
      pIVar10 = DAT_143add9a8;
      iVar5 = (int)local_res8;
    }
    else {
      local_res10 = (longlong *)((ulonglong)local_res10 & 0xffffffff00000000);
      iVar5 = (**(code **)(*(longlong *)pIVar3 + 400))(pIVar3,&local_res10);
      if (iVar5 < 0) {
        _com_issue_errorex(iVar5,pIVar3,(_GUID *)&DAT_14327fcb0);
      }
      pIVar10 = DAT_143add9a8;
      iVar5 = (int)local_res10;
    }
    while (pIVar10 != (IUnknown *)0x0) {
      iVar6 = *(int *)pIVar10;
      cVar4 = FUN_141d5dd40(iVar6);
      if (cVar4 != '\0') {
        lVar8 = thunk_FUN_141d5f870(iVar6);
        cVar4 = *(char *)((longlong)DAT_143add860[1] + 0x19);
        puVar11 = (undefined8 *)DAT_143add860[1];
        puVar2 = DAT_143add860;
        while (puVar13 = puVar11, cVar4 == '\0') {
          if (*(int *)((longlong)puVar13 + 0x1c) < iVar6) {
            puVar11 = (undefined8 *)puVar13[2];
            puVar13 = puVar2;
          }
          else {
            puVar11 = (undefined8 *)*puVar13;
          }
          cVar4 = *(char *)((longlong)puVar11 + 0x19);
          puVar2 = puVar13;
        }
        if ((((*(char *)((longlong)puVar2 + 0x19) != '\0') ||
             (iVar6 < *(int *)((longlong)puVar2 + 0x1c))) || (puVar2 == DAT_143add860)) ||
           ((lVar8 == 0 || (lVar8 == 8)))) goto LAB_142c0585c;
        pIVar1 = *(IUnknown **)(lVar8 + 0x38);
        if (pIVar1 != pIVar3) {
          local_res10 = (longlong *)0x0;
          local_res18 = (longlong *)0x0;
          if (pIVar1 != (IUnknown *)0x0) {
            iVar6 = (*(code *)**(undefined8 **)pIVar1)(pIVar1,&DAT_14336c1e8,&local_res10);
            if (iVar6 < 0) {
              local_res10 = (longlong *)0x0;
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(iVar6);
            }
            (**(code **)(*local_res10 + 0x10))();
          }
          if (pIVar3 != (IUnknown *)0x0) {
            iVar6 = (*(code *)**(undefined8 **)pIVar3)(pIVar3,&DAT_14336c1e8,&local_res18);
            if (iVar6 < 0) {
              local_res18 = (longlong *)0x0;
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(iVar6);
            }
            (**(code **)(*local_res18 + 0x10))();
          }
          if (local_res10 != local_res18) {
            pIVar1 = *(IUnknown **)(lVar8 + 0x38);
            if (pIVar1 == (IUnknown *)0x0) goto LAB_142c0587a;
            local_res10 = (longlong *)((ulonglong)local_res10 & 0xffffffff00000000);
            iVar6 = (**(code **)(*(longlong *)pIVar1 + 400))(pIVar1,&local_res10);
            if (iVar6 < 0) {
              _com_issue_errorex(iVar6,pIVar1,(_GUID *)&DAT_14327fcb0);
            }
            if ((int)local_res10 <= iVar5) goto LAB_142c05755;
          }
        }
        if (pIVar10 != (IUnknown *)0x0) {
          puVar9 = (undefined4 *)FUN_142c1e2e0(&DAT_143add9a0,pIVar10);
          *puVar9 = *(undefined4 *)(param_1 + 0x10);
          goto LAB_142c05830;
        }
        break;
      }
LAB_142c05755:
      uVar12 = *(ulonglong *)(pIVar10 + -0x20);
      if ((uVar12 != 0) && (uVar12 < 0x10001)) {
        FUN_142e52ed0(0x33e);
        uVar12 = *(ulonglong *)(pIVar10 + -0x20);
      }
      pIVar10 = pIVar15;
      if (uVar12 != 0) {
        pIVar10 = (IUnknown *)(uVar12 + 0x28);
      }
    }
    pIVar10 = (IUnknown *)FUN_142624330(&DAT_143add9a0,DAT_143add9b0,0);
    pIVar1 = pIVar10;
    if (DAT_143add9b0 != (IUnknown *)0x0) {
      pIVar14 = pIVar15;
      if (pIVar10 != (IUnknown *)0x0) {
        pIVar14 = pIVar10 + -0x28;
      }
      if (DAT_143add9b0 == (IUnknown *)0x0) {
        pIVar10 = (IUnknown *)&DAT_00000008;
      }
      else {
        pIVar10 = DAT_143add9b0 + -0x20;
      }
      if (*(longlong *)pIVar10 - 1U < 0x10000) {
        FUN_142e52ed0(0x33e);
      }
      *(IUnknown **)pIVar10 = pIVar14;
      pIVar10 = pIVar15;
      pIVar1 = DAT_143add9a8;
      if (pIVar14 != (IUnknown *)0x0) {
        pIVar10 = pIVar14 + 0x28;
      }
    }
    DAT_143add9a8 = pIVar1;
    DAT_143add9b0 = pIVar10;
    *(int *)pIVar10 = *(int *)(param_1 + 0x10);
LAB_142c05830:
    FUN_142c020b0(param_1,(ulonglong)local_res8 & 0xffffffff);
    if ((DAT_143add870 != '\0') || ((DAT_143acedb0 != 0 && (*(int *)(DAT_143acedb0 + 8) != 0)))) {
      FUN_142d1d4d0();
    }
LAB_142c0585c:
    if (pIVar3 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar3 + 0x10))(pIVar3);
    }
  }
  return;
}



//===========================================================
// thunk_FUN_140205820 @ 142ef3bb8   (5 bytes)
//===========================================================

void thunk_FUN_140205820(undefined8 param_1)

{
  FUN_14019bb50(&DAT_143ad68a0,param_1);
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
// FUN_142c0c190 @ 142c0c190   (328 bytes)
//===========================================================

void FUN_142c0c190(longlong param_1,int *param_2,int param_3,int param_4)

{
  int iVar1;
  int iVar2;
  longlong lVar3;
  float fVar4;
  
  if (DAT_143ac3738 != 0) {
    FUN_141a38400();
    if (param_3 == 0) {
      iVar2 = *param_2;
      fVar4 = (float)FUN_1410a4df0();
      iVar1 = param_2[1];
      *param_2 = (int)((float)iVar2 / fVar4);
      fVar4 = (float)FUN_1410a4df0();
      param_2[1] = (int)((float)iVar1 / fVar4);
      return;
    }
    if (param_4 == 0) {
      iVar2 = *param_2;
      fVar4 = (float)FUN_1410a4df0();
      iVar1 = param_2[1];
      *param_2 = (int)((float)iVar2 / fVar4);
      fVar4 = (float)FUN_1410a4df0();
      param_2[1] = (int)((float)iVar1 / fVar4);
      if (*(longlong *)(param_1 + 0x368) == 0) goto LAB_142c0c2ce;
      iVar2 = FUN_140319f60();
      *param_2 = *param_2 + iVar2;
      lVar3 = *(longlong *)(param_1 + 0x368);
    }
    else {
      if (((DAT_143add050 == (longlong *)0x0) ||
          ((**(code **)(*DAT_143add050 + 0x2d8))(DAT_143add050,param_2),
          DAT_143add050 == (longlong *)0x0)) ||
         ((**(code **)(*DAT_143add050 + 0x2d8))(DAT_143add050,param_2 + 1),
         *(longlong *)(param_1 + 0x3b0) == 0)) goto LAB_142c0c2ce;
      iVar2 = FUN_140319f60();
      *param_2 = *param_2 + iVar2;
      lVar3 = *(longlong *)(param_1 + 0x3b0);
    }
    if (lVar3 == 0) {
LAB_142c0c2ce:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    iVar2 = FUN_140319fa0();
    param_2[1] = param_2[1] + iVar2;
  }
  return;
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
// FUN_142c0bf50 @ 142c0bf50   (237 bytes)
//===========================================================

void FUN_142c0bf50(longlong param_1,longlong *param_2,undefined1 param_3)

{
  int iVar1;
  longlong *plVar2;
  longlong *plVar3;
  
  FUN_142c0b490();
  if ((param_2 != (longlong *)0x0) && (iVar1 = (**(code **)(*param_2 + 0x78))(param_2), iVar1 == 0))
  {
    return;
  }
  if (*(longlong **)(param_1 + 0x268) == param_2) {
    return;
  }
  if ((param_2 == (longlong *)0x0) ||
     (iVar1 = (**(code **)(*param_2 + 0xd0))(param_2,&PTR_DAT_143a87db8), iVar1 == 0)) {
    plVar3 = (longlong *)0x0;
  }
  else {
    plVar3 = param_2 + -1;
    if (plVar3 != (longlong *)0x0) {
      plVar2 = (longlong *)param_2[9];
      goto LAB_142c0bfee;
    }
  }
  plVar2 = param_2 + -1;
  if ((param_2 == (longlong *)0x0) ||
     (iVar1 = (**(code **)(*param_2 + 0xd0))(param_2,&PTR_DAT_143a8b230), iVar1 == 0)) {
    plVar2 = (longlong *)0x0;
  }
LAB_142c0bfee:
  iVar1 = FUN_142c13e60(param_1,plVar2);
  if (((iVar1 != 0) && (FUN_142c13f40(param_1,param_2,param_3), plVar3 != (longlong *)0x0)) &&
     (plVar3[10] != 0)) {
    FUN_142bf7c90(plVar3[10],plVar3);
  }
  return;
}



//===========================================================
// FUN_142c137a0 @ 142c137a0   (44 bytes)
//===========================================================

longlong * FUN_142c137a0(longlong param_1,longlong *param_2,int param_3)

{
  longlong *plVar1;
  
  plVar1 = *(longlong **)(param_1 + 0x368 + (longlong)param_3 * 8);
  *param_2 = (longlong)plVar1;
  if (plVar1 != (longlong *)0x0) {
    (**(code **)(*plVar1 + 8))();
  }
  return param_2;
}



//===========================================================
// FUN_142f04924 @ 142f04924   (41 bytes)
//===========================================================

uint FUN_142f04924(void)

{
  longlong lVar1;
  uint uVar2;
  
  lVar1 = FUN_142f31784();
  uVar2 = *(int *)(lVar1 + 0x28) * 0x343fd + 0x269ec3;
  *(uint *)(lVar1 + 0x28) = uVar2;
  return uVar2 >> 0x10 & 0x7fff;
}



//===========================================================
// FUN_142c12c90 @ 142c12c90   (1486 bytes)
//===========================================================

longlong * FUN_142c12c90(longlong param_1,int param_2,int param_3)

{
  uint *puVar1;
  code *pcVar2;
  char cVar3;
  int iVar4;
  int iVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  longlong *plVar9;
  longlong *plVar10;
  ulonglong uVar11;
  longlong *plVar12;
  longlong *plVar13;
  char cVar14;
  longlong *plVar15;
  bool bVar16;
  ulonglong local_res20;
  longlong local_68;
  longlong local_60;
  longlong *local_58;
  undefined1 local_50 [4];
  undefined4 local_4c;
  longlong *local_48;
  longlong *plStack_40;
  
  lVar8 = DAT_143add878;
  if (*(char *)(DAT_143ac1898 + 0xc0) != '\0') {
    return (longlong *)0x0;
  }
  cVar14 = '\x01';
  local_res20 = CONCAT71(local_res20._1_7_,1);
  if (DAT_143add878 == 0) {
    (*DAT_143ad5850)(&local_58);
    lVar7 = FUN_141ee7690(&local_58);
    if (lVar7 != 0) goto LAB_142c12d04;
  }
  else {
    lVar6 = FUN_142c49f00();
    lVar7 = lVar8;
    if (lVar8 != lVar6) {
LAB_142c12d04:
      cVar14 = '\0';
      local_res20 = local_res20 & 0xffffffffffffff00;
      lVar8 = lVar7;
    }
  }
  FUN_142c0b490(param_1);
  if (((*(int *)(param_1 + 0x23c) != 0) &&
      (plVar13 = (longlong *)**(longlong **)(param_1 + 0x248), plVar13 != (longlong *)0x0)) &&
     (cVar3 = FUN_141d5dc50(plVar13), cVar3 != '\0')) {
    iVar4 = (**(code **)(*plVar13 + 0xd0))(plVar13,&PTR_DAT_143a8b230);
    if (((iVar4 != 0) && (plVar9 = plVar13 + -1, plVar9 != (longlong *)0x0)) &&
       ((cVar14 != '\0' || (lVar7 = FUN_142c01ed0(plVar9), lVar7 == lVar8)))) {
      plVar10 = (longlong *)0x0;
      local_res20 = 0;
      pcVar2 = *(code **)(*plVar9 + 0x68);
      iVar4 = (**(code **)(*plVar13 + 0x98))(plVar13);
      iVar5 = (**(code **)(*plVar13 + 0x90))(plVar13);
      iVar4 = (*pcVar2)(plVar9,param_2 - iVar5,param_3 - iVar4,&local_res20);
      if (iVar4 != 0) {
        FUN_142bf21d0(plVar9);
        plVar9 = (longlong *)plVar13[0x27];
        if (plVar9 != (longlong *)0x0) {
          plVar15 = plVar9 + *(uint *)(plVar13 + 0x28);
          for (; plVar9 < plVar15; plVar9 = plVar9 + 1) {
            lVar8 = *plVar9;
            if (*plVar9 != 0) goto LAB_142c12e60;
          }
        }
LAB_142c12e01:
        if ((((local_res20 != 0) &&
             (iVar4 = (**(code **)(*(longlong *)(local_res20 + 8) + 0x78))(), plVar13 = plVar10,
             iVar4 != 0)) &&
            (iVar4 = (**(code **)(*(longlong *)(local_res20 + 8) + 0x88))(), iVar4 != 0)) &&
           (local_res20 != 0)) {
          plVar13 = (longlong *)(local_res20 + 8);
          FUN_141d5dc50(plVar13);
          return plVar13;
        }
        FUN_141d5dc50(plVar13);
        return plVar13;
      }
    }
    FUN_141d5dc50(plVar13);
    return plVar13;
  }
  plVar15 = (longlong *)0x0;
  local_4c = 0;
  local_48 = (longlong *)0x0;
  plStack_40 = (longlong *)0x0;
  FUN_142c1d380(local_50,&DAT_143add9a0);
  plVar10 = plStack_40;
  plVar9 = local_48;
  plVar13 = plVar15;
  if (plStack_40 != (longlong *)0x0) {
    while( true ) {
      cVar3 = FUN_141d5dd40((int)*plVar10);
      if (cVar3 != '\0') {
        plVar9 = (longlong *)FUN_142c06bb0((int)*plVar10);
        lVar7 = FUN_142c01ed0(plVar9);
        if (cVar14 == '\0') {
          bVar16 = lVar7 == lVar8;
        }
        else {
          bVar16 = lVar7 == 0;
        }
        if (bVar16) {
          local_68 = 0;
          pcVar2 = *(code **)(*plVar9 + 0x68);
          plVar13 = plVar9 + 1;
          local_58 = plVar13;
          iVar4 = (**(code **)(*plVar13 + 0x98))(plVar13);
          iVar5 = (**(code **)(*plVar13 + 0x90))(plVar13);
          iVar4 = (*pcVar2)(plVar9,param_2 - iVar5,param_3 - iVar4,&local_68);
          if (iVar4 != 0) {
            plVar10 = (longlong *)plVar9[0x28];
            if ((plVar10 == (longlong *)0x0) ||
               (plVar12 = plVar10 + *(uint *)(plVar9 + 0x29), plVar12 <= plVar10))
            goto LAB_142c1318a;
            goto LAB_142c13040;
          }
        }
      }
      uVar11 = plVar10[-3];
      if ((uVar11 != 0) && (uVar11 < 0x10001)) {
        FUN_142e52ed0(0x330);
        uVar11 = plVar10[-3];
      }
      plVar10 = plVar15;
      if (uVar11 != 0) {
        plVar10 = (longlong *)(uVar11 + 0x28);
      }
      plVar9 = local_48;
      plVar13 = plVar15;
      if (plVar10 == (longlong *)0x0) break;
      cVar14 = (char)local_res20;
    }
  }
joined_r0x000142c131f1:
  while (plVar9 != (longlong *)0x0) {
    uVar11 = plVar9[-4];
    if ((uVar11 != 0) && (uVar11 < 0x10001)) {
      FUN_142e52ed0(0x33e);
      uVar11 = plVar9[-4];
    }
    plVar10 = plVar15;
    if (uVar11 != 0) {
      plVar10 = (longlong *)(uVar11 + 0x28);
    }
    plVar12 = plVar15;
    if (plVar9 != (longlong *)0x0) {
      plVar12 = plVar9 + -5;
    }
    plVar9 = plVar10;
    if (plVar12 != (longlong *)0x0) {
      (**(code **)*plVar12)(plVar12,1);
    }
  }
  return plVar13;
  while (plVar10 = plVar10 + 1, plVar10 < plVar12) {
LAB_142c13040:
    lVar8 = *plVar10;
    if (lVar8 != 0) goto LAB_142c13060;
  }
LAB_142c1318a:
  if (local_68 == 0) {
LAB_142c131dd:
    FUN_141d5dc50(plVar13);
    plVar9 = local_48;
  }
  else {
    iVar4 = (**(code **)(*(longlong *)(local_68 + 8) + 0x78))();
    if (((iVar4 == 0) || (iVar4 = (**(code **)(*(longlong *)(local_68 + 8) + 0x88))(), iVar4 == 0))
       || (local_68 == 0)) {
      FUN_141d5dc50(0);
      plVar9 = local_48;
      plVar13 = plVar15;
    }
    else {
      plVar13 = (longlong *)(local_68 + 8);
      FUN_141d5dc50(plVar13);
      plVar9 = local_48;
    }
  }
  goto joined_r0x000142c131f1;
LAB_142c13060:
  puVar1 = (uint *)(lVar8 + 0x10);
  local_res20 = 0;
  local_60 = 0;
  plVar13 = *(longlong **)(lVar8 + 0x18);
  if (plVar13 != (longlong *)0x0) {
    pcVar2 = *(code **)(*plVar13 + 0x148);
    iVar4 = (**(code **)(plVar13[1] + 0x98))();
    iVar5 = (**(code **)(*(longlong *)(*(longlong *)(lVar8 + 0x18) + 8) + 0x90))();
    iVar4 = (*pcVar2)(plVar13,param_2 - iVar5,param_3 - iVar4,&local_res20,&local_60);
    if (iVar4 != 0) {
      if (local_res20 == 0) {
        if (local_60 == 0) {
          uVar11 = *(ulonglong *)(lVar8 + 0x18);
          goto LAB_142c130f8;
        }
        plVar13 = (longlong *)(local_60 + 8);
      }
      else {
        iVar4 = (**(code **)(*(longlong *)(local_res20 + 8) + 0x88))();
        plVar13 = plVar15;
        uVar11 = local_res20;
        if (iVar4 != 0) {
LAB_142c130f8:
          plVar13 = plVar15;
          if (uVar11 != 0) {
            plVar13 = (longlong *)(uVar11 + 8);
          }
        }
      }
      if ((local_68 == 0) || (cVar14 = FUN_1415c5200(*(undefined8 *)(lVar8 + 0x18)), cVar14 == '\0')
         ) goto LAB_142c131dd;
      plVar10 = (longlong *)(*(longlong *)(lVar8 + 0x18) + 8);
      if (*(longlong *)(lVar8 + 0x18) == 0) {
        plVar10 = plVar15;
      }
      if (plVar10 != plVar13) goto LAB_142c131dd;
    }
  }
  lVar8 = *(longlong *)(lVar8 + 8);
  if (lVar8 == 0) {
    plVar10 = (longlong *)
              (plVar9[0x28] + ((ulonglong)*puVar1 % (ulonglong)*(uint *)(plVar9 + 0x29) + 1) * 8);
    while( true ) {
      plVar13 = local_58;
      if ((longlong *)(plVar9[0x28] + (ulonglong)*(uint *)(plVar9 + 0x29) * 8) <= plVar10)
      goto LAB_142c1318a;
      lVar8 = *plVar10;
      if (lVar8 != 0) break;
      plVar10 = plVar10 + 1;
    }
  }
  goto LAB_142c13060;
LAB_142c12e60:
  while( true ) {
    lVar7 = lVar8;
    local_68 = 0;
    local_60 = 0;
    plVar9 = *(longlong **)(lVar7 + 0x18);
    if ((plVar9 != (longlong *)0x0) &&
       (iVar4 = (**(code **)(*plVar9 + 0x148))(plVar9,param_2,param_3,&local_68,&local_60),
       iVar4 != 0)) break;
    lVar8 = *(longlong *)(lVar7 + 8);
    if (*(longlong *)(lVar7 + 8) == 0) {
      plVar9 = (longlong *)
               (plVar13[0x27] +
               ((ulonglong)*(uint *)(lVar7 + 0x10) % (ulonglong)*(uint *)(plVar13 + 0x28) + 1) * 8);
      while( true ) {
        if ((longlong *)(plVar13[0x27] + (ulonglong)*(uint *)(plVar13 + 0x28) * 8) <= plVar9)
        goto LAB_142c12e01;
        lVar8 = *plVar9;
        if (*plVar9 != 0) break;
        plVar9 = plVar9 + 1;
      }
    }
  }
  if (local_68 == 0) {
    lVar8 = local_60;
    if (local_60 == 0) {
      lVar8 = *(longlong *)(lVar7 + 0x18);
      goto LAB_142c12f05;
    }
  }
  else {
    iVar4 = (**(code **)(*(longlong *)(local_68 + 8) + 0x88))();
    lVar8 = local_68;
    if (iVar4 == 0) goto LAB_142c12f0e;
LAB_142c12f05:
    if (lVar8 == 0) goto LAB_142c12f0e;
  }
  plVar10 = (longlong *)(lVar8 + 8);
LAB_142c12f0e:
  FUN_141d5dc50(plVar10);
  return plVar10;
}



//===========================================================
// FUN_140336390 @ 140336390   (106 bytes)
//===========================================================

void FUN_140336390(longlong param_1)

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
// FUN_1426e2550 @ 1426e2550   (637 bytes)
//===========================================================

longlong * FUN_1426e2550(longlong *param_1,longlong *param_2,longlong *param_3)

{
  char cVar1;
  longlong *plVar2;
  longlong *plVar3;
  undefined8 *puVar4;
  longlong lVar5;
  longlong *plVar6;
  longlong *plVar7;
  longlong *plVar8;
  longlong *plVar9;
  longlong *plVar10;
  
  param_1[1] = param_1[1] + 1;
  plVar2 = (longlong *)*param_1;
  plVar8 = (longlong *)*param_2;
  param_3[1] = (longlong)plVar8;
  if (plVar8 == plVar2) {
    *plVar2 = (longlong)param_3;
    plVar2[1] = (longlong)param_3;
    plVar2[2] = (longlong)param_3;
    *(undefined1 *)(param_3 + 3) = 1;
    return param_3;
  }
  if ((int)param_2[1] == 0) {
    plVar8[2] = (longlong)param_3;
    if (plVar8 == (longlong *)plVar2[2]) {
      plVar2[2] = (longlong)param_3;
    }
  }
  else {
    *plVar8 = (longlong)param_3;
    if (plVar8 == (longlong *)*plVar2) {
      *plVar2 = (longlong)param_3;
    }
  }
  cVar1 = *(char *)(param_3[1] + 0x18);
  plVar8 = param_3;
  do {
    if (cVar1 != '\0') {
      *(undefined1 *)(plVar2[1] + 0x18) = 1;
      return param_3;
    }
    plVar9 = (longlong *)plVar8[1];
    plVar7 = plVar8 + 1;
    plVar10 = plVar9 + 1;
    lVar5 = *(longlong *)plVar9[1];
    if (plVar9 == (longlong *)lVar5) {
      lVar5 = ((longlong *)plVar9[1])[2];
      if (*(char *)(lVar5 + 0x18) != '\0') {
        plVar3 = (longlong *)plVar9[2];
        if (plVar8 == plVar3) {
          plVar9[2] = *plVar3;
          if (*(char *)(*plVar3 + 0x19) == '\0') {
            *(longlong **)(*plVar3 + 8) = plVar9;
          }
          plVar3[1] = *plVar10;
          if (plVar9 == (longlong *)*(longlong *)(*param_1 + 8)) {
            *(longlong **)(*param_1 + 8) = plVar3;
            *plVar3 = (longlong)plVar9;
            *plVar10 = (longlong)plVar3;
            plVar8 = plVar9;
            plVar9 = plVar3;
            plVar7 = plVar10;
          }
          else {
            plVar8 = (longlong *)*plVar10;
            if (plVar9 == (longlong *)*plVar8) {
              *plVar8 = (longlong)plVar3;
              *plVar3 = (longlong)plVar9;
              *plVar10 = (longlong)plVar3;
              plVar8 = plVar9;
              plVar9 = plVar3;
              plVar7 = plVar10;
            }
            else {
              plVar8[2] = (longlong)plVar3;
              *plVar3 = (longlong)plVar9;
              *plVar10 = (longlong)plVar3;
              plVar8 = plVar9;
              plVar9 = plVar3;
              plVar7 = plVar10;
            }
          }
        }
        *(undefined1 *)(plVar9 + 3) = 1;
        *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
        plVar7 = *(longlong **)(*plVar7 + 8);
        plVar10 = (longlong *)*plVar7;
        *plVar7 = plVar10[2];
        if (*(char *)(plVar10[2] + 0x19) == '\0') {
          *(longlong **)(plVar10[2] + 8) = plVar7;
        }
        plVar10[1] = plVar7[1];
        if (plVar7 == *(longlong **)(*param_1 + 8)) {
          *(longlong **)(*param_1 + 8) = plVar10;
          plVar10[2] = (longlong)plVar7;
        }
        else {
          plVar9 = (longlong *)plVar7[1];
          if (plVar7 == (longlong *)plVar9[2]) {
            plVar9[2] = (longlong)plVar10;
            plVar10[2] = (longlong)plVar7;
          }
          else {
            *plVar9 = (longlong)plVar10;
            plVar10[2] = (longlong)plVar7;
          }
        }
        goto LAB_1426e27a5;
      }
LAB_1426e26d7:
      *(undefined1 *)(plVar9 + 3) = 1;
      *(undefined1 *)(lVar5 + 0x18) = 1;
      *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
      plVar8 = *(longlong **)(*plVar7 + 8);
    }
    else {
      if (*(char *)(lVar5 + 0x18) == '\0') goto LAB_1426e26d7;
      plVar3 = (longlong *)*plVar9;
      plVar6 = plVar9;
      if (plVar8 == plVar3) {
        *plVar9 = plVar3[2];
        if (*(char *)(plVar3[2] + 0x19) == '\0') {
          *(longlong **)(plVar3[2] + 8) = plVar9;
        }
        plVar3[1] = *plVar10;
        if (plVar9 == (longlong *)*(longlong *)(*param_1 + 8)) {
          *(longlong **)(*param_1 + 8) = plVar3;
        }
        else {
          puVar4 = (undefined8 *)*plVar10;
          if (plVar9 == (longlong *)puVar4[2]) {
            puVar4[2] = plVar3;
          }
          else {
            *puVar4 = plVar3;
          }
        }
        plVar3[2] = (longlong)plVar9;
        *plVar10 = (longlong)plVar3;
        plVar6 = plVar3;
        plVar8 = plVar9;
        plVar7 = plVar10;
      }
      *(undefined1 *)(plVar6 + 3) = 1;
      *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
      plVar7 = *(longlong **)(*plVar7 + 8);
      plVar10 = (longlong *)plVar7[2];
      plVar7[2] = *plVar10;
      if (*(char *)(*plVar10 + 0x19) == '\0') {
        *(longlong **)(*plVar10 + 8) = plVar7;
      }
      plVar10[1] = plVar7[1];
      if (plVar7 == *(longlong **)(*param_1 + 8)) {
        *(longlong **)(*param_1 + 8) = plVar10;
      }
      else {
        puVar4 = (undefined8 *)plVar7[1];
        if (plVar7 == (longlong *)*puVar4) {
          *puVar4 = plVar10;
        }
        else {
          puVar4[2] = plVar10;
        }
      }
      *plVar10 = (longlong)plVar7;
LAB_1426e27a5:
      plVar7[1] = (longlong)plVar10;
    }
    cVar1 = *(char *)(plVar8[1] + 0x18);
  } while( true );
}



//===========================================================
// FUN_1426dc3a0 @ 1426dc3a0   (233 bytes)
//===========================================================

void FUN_1426dc3a0(undefined8 *param_1)

{
  longlong *plVar1;
  int *piVar2;
  int iVar3;
  longlong lVar4;
  longlong *plVar5;
  undefined1 uVar6;
  longlong *local_18;
  longlong *local_10;
  
  if (*(char *)(param_1 + 2) == '\0') {
    local_10 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x858);
    if (local_10 == (longlong *)0x0) {
      local_10 = (longlong *)0x0;
    }
    else {
      *local_10 = 0;
      local_10[1] = 0;
      *(undefined4 *)(local_10 + 1) = 1;
      *(undefined4 *)((longlong)local_10 + 0xc) = 1;
      *local_10 = (longlong)&PTR_FUN_143479c88;
      FUN_142ef8250(local_10 + 2,0,0x848);
      FUN_1426d4b90(local_10 + 2);
    }
    local_18 = local_10 + 2;
    FUN_1426d6260(param_1,&local_18);
    plVar5 = local_10;
    if (local_10 != (longlong *)0x0) {
      LOCK();
      plVar1 = local_10 + 1;
      lVar4 = *plVar1;
      *(int *)plVar1 = (int)*plVar1 + -1;
      UNLOCK();
      if ((int)lVar4 == 1) {
        (**(code **)*local_10)(local_10);
        LOCK();
        piVar2 = (int *)((longlong)plVar5 + 0xc);
        iVar3 = *piVar2;
        *piVar2 = *piVar2 + -1;
        UNLOCK();
        if (iVar3 == 1) {
          (**(code **)(*local_10 + 8))();
        }
      }
    }
    uVar6 = FUN_1426c7360(*param_1);
    *(undefined1 *)(param_1 + 2) = uVar6;
  }
  return;
}



//===========================================================
// FUN_142645170 @ 142645170   (630 bytes)
//===========================================================

void FUN_142645170(longlong param_1)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  undefined4 *puVar4;
  longlong *plVar5;
  longlong lVar6;
  
  *(undefined8 *)(param_1 + 0x38) = 0;
  *(undefined4 *)(param_1 + 0x40) = 0;
  if (*(longlong **)(param_1 + 0x48) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x48) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x48) = 0;
  if (*(longlong **)(param_1 + 0x50) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x50) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x50) = 0;
  if (*(longlong **)(param_1 + 0x68) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x68) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x68) = 0;
  if (*(longlong **)(param_1 + 0x70) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x70) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x70) = 0;
  if (*(longlong **)(param_1 + 0x58) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x58) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x58) = 0;
  if (*(longlong **)(param_1 + 0x60) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x60) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x60) = 0;
  *(undefined4 *)(param_1 + 0xa0) = 0;
  *(undefined8 *)(param_1 + 0xc70) = 0;
  *(undefined4 *)(param_1 + 0xc78) = 0;
  *(undefined4 *)(param_1 + 0x10c0) = 0;
  *(undefined4 *)(param_1 + 0x10c4) = 0xffffffff;
  *(undefined4 *)(param_1 + 0x1100) = 0;
  *(undefined1 *)(param_1 + 0x4e8) = 0;
  *(undefined8 *)(param_1 + 0x4ec) = 0;
  plVar5 = (longlong *)(param_1 + 0x938);
  puVar4 = (undefined4 *)(param_1 + 0xc4);
  lVar6 = 0x22;
  do {
    *(undefined8 *)(puVar4 + -7) = 0;
    *puVar4 = 0;
    *(undefined8 *)(puVar4 + 0x2ef) = 0;
    puVar4[0x2f6] = 0;
    lVar3 = plVar5[1];
    lVar2 = *plVar5;
    if (lVar2 != lVar3) {
      do {
        if (*(longlong *)(lVar2 + 8) != 0) {
          FUN_14019f2c0(*(longlong *)(lVar2 + 8) + -0x10);
        }
        lVar2 = lVar2 + 0x10;
      } while (lVar2 != lVar3);
      lVar2 = *plVar5;
    }
    plVar5[1] = lVar2;
    *(undefined8 *)(puVar4 + 0x10d) = 0;
    puVar4[0x114] = 0;
    plVar5 = plVar5 + 3;
    puVar4 = puVar4 + 8;
    lVar6 = lVar6 + -1;
  } while (lVar6 != 0);
  lVar6 = *(longlong *)(param_1 + 0x10e0);
  lVar3 = *(longlong *)(param_1 + 0x10e8);
  if (lVar6 != lVar3) {
    do {
      puVar1 = *(undefined8 **)(lVar6 + 8);
      if (puVar1 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar1[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar1[1] = puVar1[1] + 1;
        UNLOCK();
      }
      if (puVar1 == (undefined8 *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_142645170(puVar1);
      if (puVar1 != (undefined8 *)0x0) {
        if (0xffffe < puVar1[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar5 = puVar1 + 1;
        lVar2 = *plVar5;
        *plVar5 = *plVar5 + -1;
        UNLOCK();
        if (((int)lVar2 == 1) && (puVar1 != (undefined8 *)0x0)) {
          (**(code **)*puVar1)(puVar1,1);
        }
      }
      lVar6 = lVar6 + 0x10;
    } while (lVar6 != lVar3);
    lVar6 = *(longlong *)(param_1 + 0x10e8);
    lVar3 = *(longlong *)(param_1 + 0x10e0);
    if (lVar3 != lVar6) {
      do {
        FUN_140336390(lVar3);
        lVar3 = lVar3 + 0x10;
      } while (lVar3 != lVar6);
      lVar3 = *(longlong *)(param_1 + 0x10e0);
    }
    *(longlong *)(param_1 + 0x10e8) = lVar3;
  }
  return;
}



//===========================================================
// `eh_vector_constructor_iterator' @ 142ef44fc   (112 bytes)
//===========================================================

/* Library Function - Single Match
    void __cdecl `eh vector constructor iterator'(void * __ptr64,unsigned __int64,unsigned
   __int64,void (__cdecl*)(void * __ptr64),void (__cdecl*)(void * __ptr64))
   
   Libraries: Visual Studio 2017 Release, Visual Studio 2019 Release */

void __cdecl
_eh_vector_constructor_iterator_
          (void *param_1,__uint64 param_2,__uint64 param_3,_func_void_void_ptr *param_4,
          _func_void_void_ptr *param_5)

{
  __uint64 _Var1;
  
  for (_Var1 = 0; _Var1 != param_3; _Var1 = _Var1 + 1) {
    (*(code *)PTR_FUN_1432630d8)(param_1);
    param_1 = (void *)((longlong)param_1 + param_2);
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
// FUN_14019f9d0 @ 14019f9d0   (16 bytes)
//===========================================================

void FUN_14019f9d0(void)

{
                    /* WARNING: Subroutine does not return */
  FUN_142ed3068("map/set too long");
}



//===========================================================
// FUN_1403ee040 @ 1403ee040   (195 bytes)
//===========================================================

longlong * FUN_1403ee040(longlong *param_1,longlong *param_2)

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
    iVar2 = (**(code **)*param_2)(param_2,&DAT_14327ac98,&local_res8);
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
// FUN_14090df00 @ 14090df00   (422 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_14090df00(undefined8 *param_1,longlong param_2)

{
  undefined2 *puVar1;
  longlong lVar2;
  code *pcVar3;
  longlong lVar4;
  int iVar5;
  ulong *puVar6;
  ulonglong uVar7;
  undefined2 *puVar8;
  undefined8 auStack_b0 [5];
  ulonglong local_88;
  undefined4 local_80 [2];
  undefined2 local_78;
  undefined6 uStack_76;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined4 local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  undefined4 local_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined8 local_28;
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)&local_78;
  if (param_2 == 0) {
    iVar5 = 2;
  }
  else {
    local_80[0] = 0;
    local_88 = 0;
    auStack_b0[0] = 0x14090df6b;
    iVar5 = (*DAT_1432627f8)(0xfde9,0,param_2,0xffffffff);
    iVar5 = iVar5 * 2;
  }
  uVar7 = (longlong)iVar5 + 0xf;
  if (uVar7 <= (ulonglong)(longlong)iVar5) {
    uVar7 = 0xffffffffffffff0;
  }
  auStack_b0[0] = 0x14090df8e;
  lVar4 = -(uVar7 & 0xfffffffffffffff0);
  puVar1 = (undefined2 *)((longlong)&local_78 + lVar4);
  if (param_2 == 0) {
    if (puVar1 != (undefined2 *)0x0) {
      *puVar1 = 0;
      puVar8 = puVar1;
      goto LAB_14090dfdb;
    }
  }
  else {
    *(undefined4 *)((longlong)local_80 + lVar4) = 0x100000;
    *(undefined2 **)((longlong)local_80 + lVar4 + -8) = puVar1;
    *(undefined8 *)((longlong)auStack_b0 + lVar4) = 0x14090dfca;
    (*DAT_1432627f8)(0xfde9,0,param_2,0xffffffff);
  }
  puVar8 = &DAT_143278568;
  if (puVar1 != (undefined2 *)0x0) {
    puVar8 = puVar1;
  }
LAB_14090dfdb:
  *(undefined8 *)((longlong)auStack_b0 + lVar4) = 0x14090dfe5;
  (*DAT_143262a20)(&local_78);
  if (DAT_143add058 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
    *(undefined **)((longlong)auStack_b0 + lVar4) = &UNK_14090e0a6;
    FUN_142ef3ac0(0x80004003);
  }
  lVar2 = *DAT_143add058;
  *(undefined2 **)((longlong)local_80 + lVar4 + -8) = &local_78;
  local_58 = _DAT_143a8b8d8;
  uStack_54 = uRam0000000143a8b8dc;
  uStack_50 = (undefined4)DAT_143a8b8e0;
  uStack_4c = DAT_143a8b8e0._4_4_;
  local_38 = _DAT_143a8b8d8;
  uStack_34 = uRam0000000143a8b8dc;
  uStack_30 = (undefined4)DAT_143a8b8e0;
  uStack_2c = DAT_143a8b8e0._4_4_;
  local_48 = DAT_143a8b8e8;
  local_28 = DAT_143a8b8e8;
  pcVar3 = *(code **)(lVar2 + 0x48);
  *(undefined8 *)((longlong)auStack_b0 + lVar4) = 0x14090e030;
  iVar5 = (*pcVar3)(DAT_143add058,puVar8,&local_38,&local_58);
  if (iVar5 < 0) {
    local_78 = 0;
    *(undefined8 *)((longlong)auStack_b0 + lVar4) = 0x14090e043;
    (*DAT_143262a08)(0,0);
  }
  if (param_1 == (undefined8 *)0x0) {
    *(undefined8 *)((longlong)auStack_b0 + lVar4) = 0x14090e04d;
    puVar6 = __doserrno();
    *puVar6 = 0x16;
    *(undefined8 *)((longlong)auStack_b0 + lVar4) = 0x14090e058;
    FUN_142f047e4();
  }
  else {
    *param_1 = CONCAT62(uStack_76,local_78);
    param_1[1] = uStack_70;
    param_1[2] = local_68;
  }
  *(undefined8 *)((longlong)auStack_b0 + lVar4) = 0x14090e07a;
  return param_1;
}



//===========================================================
// FUN_142bfcd30 @ 142bfcd30   (1006 bytes)
//===========================================================

void FUN_142bfcd30(longlong param_1)

{
  byte *pbVar1;
  ushort uVar2;
  undefined8 *puVar3;
  byte *pbVar4;
  undefined1 uVar5;
  byte bVar6;
  undefined8 *puVar7;
  int iVar8;
  byte bVar9;
  byte *pbVar10;
  uint uVar11;
  uint uVar12;
  byte local_res8 [8];
  
  local_res8[0] = 0xff;
  local_res8[1] = 0xff;
  local_res8[2] = 0xff;
  local_res8[3] = 0xff;
  iVar8 = *(int *)(param_1 + 0x100) + 1;
  *(int *)(param_1 + 0x100) = iVar8;
  if (iVar8 == (iVar8 / 0x6f) * 0x6f) {
    puVar3 = *(undefined8 **)(param_1 + 0x108);
    puVar7 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    *(undefined8 **)(param_1 + 0x108) = puVar7;
    *puVar7 = *puVar3;
    *(undefined4 *)(puVar7 + 1) = *(undefined4 *)(puVar3 + 1);
    thunk_FUN_140205820(puVar3,0xc);
  }
  uVar5 = FUN_142f04924();
  uVar12 = 0;
  uVar11 = 0;
  *(undefined1 *)(*(longlong *)(param_1 + 0x108) + 4) = uVar5;
  pbVar4 = *(byte **)(param_1 + 0x108);
  bVar9 = pbVar4[4];
  pbVar4[8] = 0x65;
  pbVar4[9] = 0x9a;
  pbVar10 = pbVar4;
  do {
    pbVar1 = pbVar10 + 4;
    if (bVar9 == 0) {
      bVar9 = 0x2a;
    }
    bVar6 = pbVar1[(longlong)(&stack0x00000004 + -(longlong)pbVar4)];
    *pbVar10 = bVar9 ^ bVar6;
    bVar9 = bVar9 + (bVar9 ^ bVar6) + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0x108) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0x108) + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar6 = 0x2a;
    if (bVar9 != 0) {
      bVar6 = bVar9;
    }
    bVar9 = pbVar1[(longlong)(&stack0x00000005 + -(longlong)pbVar4)];
    pbVar10[1] = bVar6 ^ bVar9;
    bVar6 = (bVar6 ^ bVar9) + bVar6 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0x108) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0x108) + 8) = (uVar2 >> 0xd) + (ushort)bVar6 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar6 != 0) {
      bVar9 = bVar6;
    }
    bVar6 = pbVar10[(longlong)(local_res8 + (2 - (longlong)pbVar4))];
    pbVar10[2] = bVar9 ^ bVar6;
    bVar6 = (bVar9 ^ bVar6) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0x108) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0x108) + 8) = (uVar2 >> 0xd) + (ushort)bVar6 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar6 != 0) {
      bVar9 = bVar6;
    }
    uVar11 = uVar11 + 4;
    bVar6 = pbVar1[(longlong)(&stack0x00000007 + -(longlong)pbVar4)];
    pbVar10[3] = bVar9 ^ bVar6;
    bVar9 = (bVar9 ^ bVar6) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0x108) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0x108) + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    pbVar10 = pbVar1;
  } while (uVar11 < 4);
  iVar8 = *(int *)(param_1 + 0xe8) + 1;
  local_res8[0] = 0xff;
  local_res8[1] = 0xff;
  local_res8[2] = 0xff;
  local_res8[3] = 0xff;
  *(int *)(param_1 + 0xe8) = iVar8;
  if (iVar8 == (iVar8 / 0x6f) * 0x6f) {
    puVar3 = *(undefined8 **)(param_1 + 0xf0);
    puVar7 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    *(undefined8 **)(param_1 + 0xf0) = puVar7;
    *puVar7 = *puVar3;
    *(undefined4 *)(puVar7 + 1) = *(undefined4 *)(puVar3 + 1);
    thunk_FUN_140205820(puVar3,0xc);
  }
  uVar5 = FUN_142f04924();
  *(undefined1 *)(*(longlong *)(param_1 + 0xf0) + 4) = uVar5;
  pbVar4 = *(byte **)(param_1 + 0xf0);
  bVar9 = pbVar4[4];
  pbVar4[8] = 0x65;
  pbVar4[9] = 0x9a;
  pbVar10 = pbVar4;
  do {
    pbVar1 = pbVar10 + 4;
    if (bVar9 == 0) {
      bVar9 = 0x2a;
    }
    bVar6 = pbVar1[(longlong)(&stack0x00000004 + -(longlong)pbVar4)];
    *pbVar10 = bVar9 ^ bVar6;
    bVar9 = bVar9 + (bVar9 ^ bVar6) + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0xf0) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0xf0) + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar6 = 0x2a;
    if (bVar9 != 0) {
      bVar6 = bVar9;
    }
    bVar9 = pbVar1[(longlong)(&stack0x00000005 + -(longlong)pbVar4)];
    pbVar10[1] = bVar6 ^ bVar9;
    bVar6 = (bVar6 ^ bVar9) + bVar6 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0xf0) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0xf0) + 8) = (uVar2 >> 0xd) + (ushort)bVar6 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar6 != 0) {
      bVar9 = bVar6;
    }
    bVar6 = pbVar10[(longlong)(local_res8 + (2 - (longlong)pbVar4))];
    pbVar10[2] = bVar9 ^ bVar6;
    bVar6 = (bVar9 ^ bVar6) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0xf0) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0xf0) + 8) = (uVar2 >> 0xd) + (ushort)bVar6 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar6 != 0) {
      bVar9 = bVar6;
    }
    uVar12 = uVar12 + 4;
    bVar6 = pbVar1[(longlong)(&stack0x00000007 + -(longlong)pbVar4)];
    pbVar10[3] = bVar9 ^ bVar6;
    bVar9 = (bVar9 ^ bVar6) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0xf0) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0xf0) + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    pbVar10 = pbVar1;
  } while (uVar12 < 4);
  return;
}



//===========================================================
// FUN_14022dad0 @ 14022dad0   (575 bytes)
//===========================================================

int * FUN_14022dad0(int *param_1)

{
  byte *pbVar1;
  ushort uVar2;
  undefined8 *puVar3;
  byte *pbVar4;
  undefined1 uVar5;
  int iVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  byte bVar9;
  int iVar10;
  byte bVar11;
  byte *pbVar12;
  uint uVar13;
  byte local_res8 [8];
  
  uVar13 = 0;
  *param_1 = 0;
  uVar7 = FUN_14019b780(&DAT_143ad68a0,0xc);
  *(undefined8 *)(param_1 + 2) = uVar7;
  iVar10 = (int)param_1 + -0x3ff8;
  iVar6 = FUN_142f04924();
  param_1[1] = iVar6 + iVar10;
  iVar6 = FUN_142f04924();
  param_1[4] = iVar6 + iVar10;
  local_res8[0] = 0;
  local_res8[1] = 0;
  local_res8[2] = 0;
  local_res8[3] = 0;
  *(char *)(*(longlong *)(param_1 + 2) + 5) = (char)param_1[1];
  *(char *)(*(longlong *)(param_1 + 2) + 6) = (char)param_1[4];
  iVar6 = *param_1 + 1;
  *param_1 = iVar6;
  if (iVar6 == (iVar6 / 0x6f) * 0x6f) {
    puVar3 = *(undefined8 **)(param_1 + 2);
    puVar8 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    *(undefined8 **)(param_1 + 2) = puVar8;
    *puVar8 = *puVar3;
    *(undefined4 *)(puVar8 + 1) = *(undefined4 *)(puVar3 + 1);
    thunk_FUN_140205820(puVar3,0xc);
  }
  uVar5 = FUN_142f04924();
  *(undefined1 *)(*(longlong *)(param_1 + 2) + 4) = uVar5;
  pbVar4 = *(byte **)(param_1 + 2);
  bVar11 = pbVar4[4];
  pbVar4[8] = 0x65;
  pbVar4[9] = 0x9a;
  pbVar12 = pbVar4;
  do {
    pbVar1 = pbVar12 + 4;
    if (bVar11 == 0) {
      bVar11 = 0x2a;
    }
    bVar9 = pbVar1[(longlong)(&stack0x00000004 + -(longlong)pbVar4)];
    *pbVar12 = bVar11 ^ bVar9;
    bVar11 = bVar11 + (bVar11 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
    *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar2 >> 0xd) + (ushort)bVar11 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar11 != 0) {
      bVar9 = bVar11;
    }
    bVar11 = pbVar1[(longlong)(&stack0x00000005 + -(longlong)pbVar4)];
    pbVar12[1] = bVar9 ^ bVar11;
    bVar9 = bVar9 + (bVar9 ^ bVar11) + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
    *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar11 = 0x2a;
    if (bVar9 != 0) {
      bVar11 = bVar9;
    }
    bVar9 = pbVar12[(longlong)(local_res8 + (2 - (longlong)pbVar4))];
    pbVar12[2] = bVar11 ^ bVar9;
    bVar11 = bVar11 + (bVar11 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
    *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar2 >> 0xd) + (ushort)bVar11 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar11 != 0) {
      bVar9 = bVar11;
    }
    uVar13 = uVar13 + 4;
    bVar11 = pbVar1[(longlong)(&stack0x00000007 + -(longlong)pbVar4)];
    pbVar12[3] = bVar9 ^ bVar11;
    bVar11 = (bVar9 ^ bVar11) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
    *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar2 >> 0xd) + (ushort)bVar11 | uVar2 << 3;
    pbVar12 = pbVar1;
  } while (uVar13 < 4);
  return param_1;
}



//===========================================================
// FUN_141d5f4b0 @ 141d5f4b0   (82 bytes)
//===========================================================

undefined8 * FUN_141d5f4b0(undefined8 *param_1)

{
  int iVar1;
  longlong lVar2;
  
  *(undefined4 *)(param_1 + 1) = 0;
  *param_1 = &PTR_FUN_143409768;
  do {
    do {
      iVar1 = FUN_140738830();
    } while (iVar1 == 0);
    lVar2 = FUN_141d5f870(iVar1);
  } while (lVar2 != 0);
  FUN_141d5f6e0(iVar1,param_1);
  *(int *)(param_1 + 1) = iVar1;
  return param_1;
}



//===========================================================
// FUN_1409d2160 @ 1409d2160   (942 bytes)
//===========================================================

int * FUN_1409d2160(int *param_1)

{
  ushort uVar1;
  undefined8 *puVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  undefined1 uVar7;
  byte bVar8;
  undefined8 *puVar9;
  byte *pbVar10;
  int iVar11;
  uint uVar12;
  byte bVar13;
  uint uVar14;
  byte local_res10 [8];
  
  FUN_14022dad0();
  FUN_14022dad0(param_1 + 6);
  uVar12 = 0;
  local_res10[0] = 0;
  local_res10[1] = 0;
  local_res10[2] = 0;
  local_res10[3] = 0;
  iVar11 = param_1[6] + 1;
  param_1[6] = iVar11;
  if (iVar11 == (iVar11 / 0x6f) * 0x6f) {
    puVar2 = *(undefined8 **)(param_1 + 8);
    puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    *(undefined8 **)(param_1 + 8) = puVar9;
    *puVar9 = *puVar2;
    *(undefined4 *)(puVar9 + 1) = *(undefined4 *)(puVar2 + 1);
    thunk_FUN_140205820(puVar2,0xc);
  }
  uVar7 = FUN_142f04924();
  *(undefined1 *)(*(longlong *)(param_1 + 8) + 4) = uVar7;
  pbVar10 = *(byte **)(param_1 + 8);
  bVar13 = pbVar10[4];
  pbVar10[8] = 0x65;
  pbVar10[9] = 0x9a;
  uVar14 = 0;
  lVar3 = -(longlong)pbVar10;
  lVar4 = 1 - (longlong)pbVar10;
  lVar5 = 2 - (longlong)pbVar10;
  lVar6 = 3 - (longlong)pbVar10;
  do {
    if (bVar13 == 0) {
      bVar13 = 0x2a;
    }
    bVar8 = pbVar10[(longlong)(local_res10 + lVar3)];
    *pbVar10 = bVar13 ^ bVar8;
    bVar13 = bVar13 + (bVar13 ^ bVar8) + 0x2a;
    uVar1 = *(ushort *)(*(longlong *)(param_1 + 8) + 8);
    *(ushort *)(*(longlong *)(param_1 + 8) + 8) = (uVar1 >> 0xd) + (ushort)bVar13 | uVar1 << 3;
    bVar8 = 0x2a;
    if (bVar13 != 0) {
      bVar8 = bVar13;
    }
    bVar13 = pbVar10[(longlong)(local_res10 + lVar4)];
    pbVar10[1] = bVar8 ^ bVar13;
    bVar8 = (bVar8 ^ bVar13) + bVar8 + 0x2a;
    uVar1 = *(ushort *)(*(longlong *)(param_1 + 8) + 8);
    *(ushort *)(*(longlong *)(param_1 + 8) + 8) = (uVar1 >> 0xd) + (ushort)bVar8 | uVar1 << 3;
    bVar13 = 0x2a;
    if (bVar8 != 0) {
      bVar13 = bVar8;
    }
    bVar8 = pbVar10[(longlong)(local_res10 + lVar5)];
    pbVar10[2] = bVar13 ^ bVar8;
    bVar8 = (bVar13 ^ bVar8) + bVar13 + 0x2a;
    uVar1 = *(ushort *)(*(longlong *)(param_1 + 8) + 8);
    *(ushort *)(*(longlong *)(param_1 + 8) + 8) = (uVar1 >> 0xd) + (ushort)bVar8 | uVar1 << 3;
    bVar13 = 0x2a;
    if (bVar8 != 0) {
      bVar13 = bVar8;
    }
    bVar8 = pbVar10[(longlong)(local_res10 + lVar6)];
    pbVar10[3] = bVar13 ^ bVar8;
    bVar13 = (bVar13 ^ bVar8) + bVar13 + 0x2a;
    uVar1 = *(ushort *)(*(longlong *)(param_1 + 8) + 8);
    *(ushort *)(*(longlong *)(param_1 + 8) + 8) = (uVar1 >> 0xd) + (ushort)bVar13 | uVar1 << 3;
    uVar14 = uVar14 + 4;
    pbVar10 = pbVar10 + 4;
  } while (uVar14 < 4);
  local_res10[0] = 0;
  local_res10[1] = 0;
  local_res10[2] = 0;
  local_res10[3] = 0;
  iVar11 = *param_1 + 1;
  *param_1 = iVar11;
  if (iVar11 == (iVar11 / 0x6f) * 0x6f) {
    puVar2 = *(undefined8 **)(param_1 + 2);
    puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    *(undefined8 **)(param_1 + 2) = puVar9;
    *puVar9 = *puVar2;
    *(undefined4 *)(puVar9 + 1) = *(undefined4 *)(puVar2 + 1);
    thunk_FUN_140205820(puVar2,0xc);
  }
  uVar7 = FUN_142f04924();
  *(undefined1 *)(*(longlong *)(param_1 + 2) + 4) = uVar7;
  pbVar10 = *(byte **)(param_1 + 2);
  bVar13 = pbVar10[4];
  pbVar10[8] = 0x65;
  pbVar10[9] = 0x9a;
  lVar3 = -(longlong)pbVar10;
  lVar4 = 1 - (longlong)pbVar10;
  lVar5 = 2 - (longlong)pbVar10;
  lVar6 = 3 - (longlong)pbVar10;
  do {
    if (bVar13 == 0) {
      bVar13 = 0x2a;
    }
    bVar8 = pbVar10[(longlong)(local_res10 + lVar3)];
    *pbVar10 = bVar13 ^ bVar8;
    bVar13 = bVar13 + (bVar13 ^ bVar8) + 0x2a;
    uVar1 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
    *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar1 >> 0xd) + (ushort)bVar13 | uVar1 << 3;
    bVar8 = 0x2a;
    if (bVar13 != 0) {
      bVar8 = bVar13;
    }
    bVar13 = pbVar10[(longlong)(local_res10 + lVar4)];
    pbVar10[1] = bVar8 ^ bVar13;
    bVar8 = (bVar8 ^ bVar13) + bVar8 + 0x2a;
    uVar1 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
    *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar1 >> 0xd) + (ushort)bVar8 | uVar1 << 3;
    bVar13 = 0x2a;
    if (bVar8 != 0) {
      bVar13 = bVar8;
    }
    bVar8 = pbVar10[(longlong)(local_res10 + lVar5)];
    pbVar10[2] = bVar13 ^ bVar8;
    bVar8 = (bVar13 ^ bVar8) + bVar13 + 0x2a;
    uVar1 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
    *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar1 >> 0xd) + (ushort)bVar8 | uVar1 << 3;
    bVar13 = 0x2a;
    if (bVar8 != 0) {
      bVar13 = bVar8;
    }
    bVar8 = pbVar10[(longlong)(local_res10 + lVar6)];
    pbVar10[3] = bVar13 ^ bVar8;
    bVar13 = (bVar13 ^ bVar8) + bVar13 + 0x2a;
    uVar1 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
    *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar1 >> 0xd) + (ushort)bVar13 | uVar1 << 3;
    uVar12 = uVar12 + 4;
    pbVar10 = pbVar10 + 4;
  } while (uVar12 < 4);
  return param_1;
}



//===========================================================
// FUN_142c04780 @ 142c04780   (637 bytes)
//===========================================================

longlong * FUN_142c04780(longlong *param_1,longlong *param_2,longlong *param_3)

{
  char cVar1;
  longlong *plVar2;
  longlong *plVar3;
  undefined8 *puVar4;
  longlong lVar5;
  longlong *plVar6;
  longlong *plVar7;
  longlong *plVar8;
  longlong *plVar9;
  longlong *plVar10;
  
  param_1[1] = param_1[1] + 1;
  plVar2 = (longlong *)*param_1;
  plVar8 = (longlong *)*param_2;
  param_3[1] = (longlong)plVar8;
  if (plVar8 == plVar2) {
    *plVar2 = (longlong)param_3;
    plVar2[1] = (longlong)param_3;
    plVar2[2] = (longlong)param_3;
    *(undefined1 *)(param_3 + 3) = 1;
    return param_3;
  }
  if ((int)param_2[1] == 0) {
    plVar8[2] = (longlong)param_3;
    if (plVar8 == (longlong *)plVar2[2]) {
      plVar2[2] = (longlong)param_3;
    }
  }
  else {
    *plVar8 = (longlong)param_3;
    if (plVar8 == (longlong *)*plVar2) {
      *plVar2 = (longlong)param_3;
    }
  }
  cVar1 = *(char *)(param_3[1] + 0x18);
  plVar8 = param_3;
  do {
    if (cVar1 != '\0') {
      *(undefined1 *)(plVar2[1] + 0x18) = 1;
      return param_3;
    }
    plVar9 = (longlong *)plVar8[1];
    plVar7 = plVar8 + 1;
    plVar10 = plVar9 + 1;
    lVar5 = *(longlong *)plVar9[1];
    if (plVar9 == (longlong *)lVar5) {
      lVar5 = ((longlong *)plVar9[1])[2];
      if (*(char *)(lVar5 + 0x18) != '\0') {
        plVar3 = (longlong *)plVar9[2];
        if (plVar8 == plVar3) {
          plVar9[2] = *plVar3;
          if (*(char *)(*plVar3 + 0x19) == '\0') {
            *(longlong **)(*plVar3 + 8) = plVar9;
          }
          plVar3[1] = *plVar10;
          if (plVar9 == (longlong *)*(longlong *)(*param_1 + 8)) {
            *(longlong **)(*param_1 + 8) = plVar3;
            *plVar3 = (longlong)plVar9;
            *plVar10 = (longlong)plVar3;
            plVar8 = plVar9;
            plVar9 = plVar3;
            plVar7 = plVar10;
          }
          else {
            plVar8 = (longlong *)*plVar10;
            if (plVar9 == (longlong *)*plVar8) {
              *plVar8 = (longlong)plVar3;
              *plVar3 = (longlong)plVar9;
              *plVar10 = (longlong)plVar3;
              plVar8 = plVar9;
              plVar9 = plVar3;
              plVar7 = plVar10;
            }
            else {
              plVar8[2] = (longlong)plVar3;
              *plVar3 = (longlong)plVar9;
              *plVar10 = (longlong)plVar3;
              plVar8 = plVar9;
              plVar9 = plVar3;
              plVar7 = plVar10;
            }
          }
        }
        *(undefined1 *)(plVar9 + 3) = 1;
        *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
        plVar7 = *(longlong **)(*plVar7 + 8);
        plVar10 = (longlong *)*plVar7;
        *plVar7 = plVar10[2];
        if (*(char *)(plVar10[2] + 0x19) == '\0') {
          *(longlong **)(plVar10[2] + 8) = plVar7;
        }
        plVar10[1] = plVar7[1];
        if (plVar7 == *(longlong **)(*param_1 + 8)) {
          *(longlong **)(*param_1 + 8) = plVar10;
          plVar10[2] = (longlong)plVar7;
        }
        else {
          plVar9 = (longlong *)plVar7[1];
          if (plVar7 == (longlong *)plVar9[2]) {
            plVar9[2] = (longlong)plVar10;
            plVar10[2] = (longlong)plVar7;
          }
          else {
            *plVar9 = (longlong)plVar10;
            plVar10[2] = (longlong)plVar7;
          }
        }
        goto LAB_142c049d5;
      }
LAB_142c04907:
      *(undefined1 *)(plVar9 + 3) = 1;
      *(undefined1 *)(lVar5 + 0x18) = 1;
      *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
      plVar8 = *(longlong **)(*plVar7 + 8);
    }
    else {
      if (*(char *)(lVar5 + 0x18) == '\0') goto LAB_142c04907;
      plVar3 = (longlong *)*plVar9;
      plVar6 = plVar9;
      if (plVar8 == plVar3) {
        *plVar9 = plVar3[2];
        if (*(char *)(plVar3[2] + 0x19) == '\0') {
          *(longlong **)(plVar3[2] + 8) = plVar9;
        }
        plVar3[1] = *plVar10;
        if (plVar9 == (longlong *)*(longlong *)(*param_1 + 8)) {
          *(longlong **)(*param_1 + 8) = plVar3;
        }
        else {
          puVar4 = (undefined8 *)*plVar10;
          if (plVar9 == (longlong *)puVar4[2]) {
            puVar4[2] = plVar3;
          }
          else {
            *puVar4 = plVar3;
          }
        }
        plVar3[2] = (longlong)plVar9;
        *plVar10 = (longlong)plVar3;
        plVar6 = plVar3;
        plVar8 = plVar9;
        plVar7 = plVar10;
      }
      *(undefined1 *)(plVar6 + 3) = 1;
      *(undefined1 *)(*(longlong *)(*plVar7 + 8) + 0x18) = 0;
      plVar7 = *(longlong **)(*plVar7 + 8);
      plVar10 = (longlong *)plVar7[2];
      plVar7[2] = *plVar10;
      if (*(char *)(*plVar10 + 0x19) == '\0') {
        *(longlong **)(*plVar10 + 8) = plVar7;
      }
      plVar10[1] = plVar7[1];
      if (plVar7 == *(longlong **)(*param_1 + 8)) {
        *(longlong **)(*param_1 + 8) = plVar10;
      }
      else {
        puVar4 = (undefined8 *)plVar7[1];
        if (plVar7 == (longlong *)*puVar4) {
          *puVar4 = plVar10;
        }
        else {
          puVar4[2] = plVar10;
        }
      }
      *plVar10 = (longlong)plVar7;
LAB_142c049d5:
      plVar7[1] = (longlong)plVar10;
    }
    cVar1 = *(char *)(plVar8[1] + 0x18);
  } while( true );
}



//===========================================================
// FUN_141aef1d0 @ 141aef1d0   (2107 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000141aef91e) */
/* WARNING: Removing unreachable block (ram,0x000141aef8be) */
/* WARNING: Removing unreachable block (ram,0x000141aef8d3) */
/* WARNING: Removing unreachable block (ram,0x000141aefa06) */
/* WARNING: Removing unreachable block (ram,0x000141aef8ec) */
/* WARNING: Removing unreachable block (ram,0x000141aef8ef) */
/* WARNING: Removing unreachable block (ram,0x000141aef92b) */
/* WARNING: Removing unreachable block (ram,0x000141aef930) */

undefined8 * FUN_141aef1d0(undefined8 *param_1)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined8 *puVar4;
  
  *param_1 = 0;
  param_1[1] = 1;
  *(undefined4 *)(param_1 + 2) = 0;
  param_1[3] = 0;
  param_1[4] = 0;
  param_1[5] = 0;
  param_1[6] = 0;
  param_1[7] = 0;
  param_1[8] = 0;
  param_1[9] = 0;
  param_1[10] = 0x1f;
  *(undefined4 *)(param_1 + 0xb) = 100;
  *(undefined4 *)((longlong)param_1 + 0x5c) = 0x18;
  param_1[0xc] = 0;
  param_1[0xd] = 0x1f;
  *(undefined4 *)(param_1 + 0xe) = 100;
  *(undefined4 *)((longlong)param_1 + 0x74) = 0x18;
  param_1[0xf] = 0;
  param_1[0x10] = 0x1f;
  *(undefined4 *)(param_1 + 0x11) = 100;
  *(undefined4 *)((longlong)param_1 + 0x8c) = 0x18;
  param_1[0x12] = 0;
  param_1[0x13] = 0x1f;
  *(undefined4 *)(param_1 + 0x14) = 100;
  *(undefined4 *)((longlong)param_1 + 0xa4) = 0x18;
  param_1[0x15] = 0;
  param_1[0x16] = 0x1f;
  *(undefined4 *)(param_1 + 0x17) = 100;
  *(undefined4 *)((longlong)param_1 + 0xbc) = 0x18;
  param_1[0x18] = 0;
  param_1[0x19] = 0x1f;
  *(undefined4 *)(param_1 + 0x1a) = 100;
  *(undefined4 *)((longlong)param_1 + 0xd4) = 0x18;
  param_1[0x1b] = 0;
  param_1[0x1c] = 0x1f;
  *(undefined4 *)(param_1 + 0x1d) = 100;
  *(undefined4 *)((longlong)param_1 + 0xec) = 0x18;
  param_1[0x1e] = 0;
  param_1[0x1f] = 0x1f;
  *(undefined4 *)(param_1 + 0x20) = 100;
  *(undefined4 *)((longlong)param_1 + 0x104) = 0x18;
  param_1[0x21] = 0;
  param_1[0x22] = 0x1f;
  *(undefined4 *)(param_1 + 0x23) = 100;
  *(undefined4 *)((longlong)param_1 + 0x11c) = 0x18;
  param_1[0x24] = 0;
  param_1[0x25] = 0x1f;
  *(undefined4 *)(param_1 + 0x26) = 100;
  *(undefined4 *)((longlong)param_1 + 0x134) = 0x18;
  param_1[0x27] = 0;
  param_1[0x28] = 0x1f;
  *(undefined4 *)(param_1 + 0x29) = 100;
  *(undefined4 *)((longlong)param_1 + 0x14c) = 0x18;
  param_1[0x2a] = 0;
  param_1[0x2b] = 0x1f;
  *(undefined4 *)(param_1 + 0x2c) = 100;
  *(undefined4 *)((longlong)param_1 + 0x164) = 0x18;
  param_1[0x2d] = 0;
  param_1[0x2e] = 0x1f;
  *(undefined4 *)(param_1 + 0x2f) = 100;
  *(undefined4 *)((longlong)param_1 + 0x17c) = 0x18;
  param_1[0x30] = 0;
  param_1[0x31] = 0x1f;
  *(undefined4 *)(param_1 + 0x32) = 100;
  *(undefined4 *)((longlong)param_1 + 0x194) = 0x18;
  param_1[0x33] = 0;
  param_1[0x34] = 0x1f;
  *(undefined4 *)(param_1 + 0x35) = 100;
  *(undefined4 *)((longlong)param_1 + 0x1ac) = 0x18;
  param_1[0x36] = 0;
  param_1[0x37] = 0x1f;
  *(undefined4 *)(param_1 + 0x38) = 100;
  *(undefined4 *)((longlong)param_1 + 0x1c4) = 0x18;
  param_1[0x39] = 0;
  param_1[0x3a] = 0x1f;
  *(undefined4 *)(param_1 + 0x3b) = 100;
  *(undefined4 *)((longlong)param_1 + 0x1dc) = 0x18;
  param_1[0x3c] = 0;
  param_1[0x3d] = 0x1f;
  *(undefined4 *)(param_1 + 0x3e) = 100;
  *(undefined4 *)((longlong)param_1 + 500) = 0x18;
  param_1[0x3f] = 0;
  param_1[0x40] = 0x1f;
  *(undefined4 *)(param_1 + 0x41) = 100;
  *(undefined4 *)((longlong)param_1 + 0x20c) = 0x18;
  param_1[0x42] = 0;
  param_1[0x43] = 0x1f;
  *(undefined4 *)(param_1 + 0x44) = 100;
  *(undefined4 *)((longlong)param_1 + 0x224) = 0x18;
  param_1[0x45] = 0;
  param_1[0x46] = 0x1f;
  *(undefined4 *)(param_1 + 0x47) = 100;
  *(undefined4 *)((longlong)param_1 + 0x23c) = 0x18;
  param_1[0x48] = 0;
  param_1[0x49] = 0x1f;
  *(undefined4 *)(param_1 + 0x4a) = 100;
  *(undefined4 *)((longlong)param_1 + 0x254) = 0x18;
  param_1[0x4b] = 0;
  param_1[0x4c] = 0x1f;
  *(undefined4 *)(param_1 + 0x4d) = 100;
  *(undefined4 *)((longlong)param_1 + 0x26c) = 0x18;
  param_1[0x4e] = 0;
  param_1[0x4f] = 0x1f;
  *(undefined4 *)(param_1 + 0x50) = 100;
  *(undefined4 *)((longlong)param_1 + 0x284) = 0x18;
  param_1[0x51] = 0;
  param_1[0x52] = 0x1f;
  *(undefined4 *)(param_1 + 0x53) = 100;
  *(undefined4 *)((longlong)param_1 + 0x29c) = 0x18;
  param_1[0x54] = 0;
  param_1[0x55] = 0x1f;
  *(undefined4 *)(param_1 + 0x56) = 100;
  *(undefined4 *)((longlong)param_1 + 0x2b4) = 0x18;
  param_1[0x57] = 0;
  param_1[0x58] = 0;
  param_1[0x59] = 0;
  param_1[0x5a] = 0;
  param_1[0x5b] = 0;
  lVar3 = FUN_14019b780(&DAT_143ad68a0,0x38);
  *(longlong *)lVar3 = lVar3;
  *(longlong *)(lVar3 + 8) = lVar3;
  *(longlong *)(lVar3 + 0x10) = lVar3;
  *(undefined2 *)(lVar3 + 0x18) = 0x101;
  param_1[0x5a] = lVar3;
  param_1[0x5c] = 0;
  param_1[0x5d] = 0x1f;
  *(undefined4 *)(param_1 + 0x5e) = 100;
  *(undefined4 *)((longlong)param_1 + 0x2f4) = 0x18;
  param_1[0x5f] = 0;
  param_1[0x60] = 0x1f;
  *(undefined4 *)(param_1 + 0x61) = 100;
  *(undefined4 *)((longlong)param_1 + 0x30c) = 0x18;
  param_1[0x62] = 0;
  param_1[99] = 0x1f;
  *(undefined4 *)(param_1 + 100) = 100;
  *(undefined4 *)((longlong)param_1 + 0x324) = 0x18;
  param_1[0x65] = 0;
  param_1[0x66] = 0x1f;
  *(undefined4 *)(param_1 + 0x67) = 100;
  *(undefined4 *)((longlong)param_1 + 0x33c) = 0x18;
  param_1[0x68] = 0;
  param_1[0x69] = 0;
  lVar3 = FUN_14019b780(&DAT_143ad68a0,0x30);
  *(longlong *)lVar3 = lVar3;
  *(longlong *)(lVar3 + 8) = lVar3;
  *(longlong *)(lVar3 + 0x10) = lVar3;
  *(undefined2 *)(lVar3 + 0x18) = 0x101;
  param_1[0x68] = lVar3;
  param_1[0x6a] = 0;
  param_1[0x6b] = 0x1f;
  *(undefined4 *)(param_1 + 0x6c) = 100;
  *(undefined4 *)((longlong)param_1 + 0x364) = 0x18;
  param_1[0x6d] = 0;
  param_1[0x6e] = 0x1f;
  *(undefined4 *)(param_1 + 0x6f) = 100;
  *(undefined4 *)((longlong)param_1 + 0x37c) = 0x18;
  param_1[0x70] = 0;
  param_1[0x71] = 0x1f;
  *(undefined4 *)(param_1 + 0x72) = 100;
  *(undefined4 *)((longlong)param_1 + 0x394) = 0x18;
  param_1[0x73] = 0;
  param_1[0x74] = 0;
  lVar3 = FUN_14019b780(&DAT_143ad68a0,0x38);
  *(longlong *)lVar3 = lVar3;
  *(longlong *)(lVar3 + 8) = lVar3;
  *(longlong *)(lVar3 + 0x10) = lVar3;
  *(undefined2 *)(lVar3 + 0x18) = 0x101;
  param_1[0x73] = lVar3;
  param_1[0x75] = 0;
  param_1[0x76] = 0;
  lVar3 = FUN_14019b780(&DAT_143ad68a0,0x38);
  *(longlong *)lVar3 = lVar3;
  *(longlong *)(lVar3 + 8) = lVar3;
  *(longlong *)(lVar3 + 0x10) = lVar3;
  *(undefined2 *)(lVar3 + 0x18) = 0x101;
  param_1[0x75] = lVar3;
  param_1[0x77] = 0;
  param_1[0x78] = 0;
  lVar3 = FUN_14019b780(&DAT_143ad68a0,0x38);
  *(longlong *)lVar3 = lVar3;
  *(longlong *)(lVar3 + 8) = lVar3;
  *(longlong *)(lVar3 + 0x10) = lVar3;
  *(undefined2 *)(lVar3 + 0x18) = 0x101;
  param_1[0x77] = lVar3;
  param_1[0x79] = 0;
  param_1[0x7a] = 0;
  lVar3 = FUN_14019b780(&DAT_143ad68a0,0x38);
  *(longlong *)lVar3 = lVar3;
  *(longlong *)(lVar3 + 8) = lVar3;
  *(longlong *)(lVar3 + 0x10) = lVar3;
  *(undefined2 *)(lVar3 + 0x18) = 0x101;
  param_1[0x79] = lVar3;
  *(undefined4 *)(param_1 + 0x7b) = 0;
  param_1[0x7c] = 0;
  param_1[0x7d] = 0;
  lVar3 = FUN_14019b780(&DAT_143ad68a0,0x30);
  *(longlong *)lVar3 = lVar3;
  *(longlong *)(lVar3 + 8) = lVar3;
  param_1[0x7c] = lVar3;
  param_1[0x7e] = 0;
  param_1[0x7f] = 0;
  param_1[0x80] = 0;
  param_1[0x81] = 7;
  param_1[0x82] = 8;
  *(undefined4 *)(param_1 + 0x7b) = 0x3f800000;
  uVar2 = param_1[0x7c];
  puVar4 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x80);
  param_1[0x7e] = puVar4;
  puVar1 = puVar4 + 0x10;
  param_1[0x7f] = puVar1;
  param_1[0x80] = puVar1;
  for (; puVar4 != puVar1; puVar4 = puVar4 + 1) {
    *puVar4 = uVar2;
  }
  *(undefined2 *)(param_1 + 0x83) = 0;
  param_1[0x84] = 0;
  param_1[0x85] = 0;
  lVar3 = FUN_14019b780(&DAT_143ad68a0,0x38);
  *(longlong *)lVar3 = lVar3;
  *(longlong *)(lVar3 + 8) = lVar3;
  *(longlong *)(lVar3 + 0x10) = lVar3;
  *(undefined2 *)(lVar3 + 0x18) = 0x101;
  param_1[0x84] = lVar3;
  param_1[0x86] = 0;
  param_1[0x87] = 0;
  param_1[0x88] = 0;
  lVar3 = FUN_14019b780(&DAT_143ad68a0,0x38);
  *(longlong *)lVar3 = lVar3;
  *(longlong *)(lVar3 + 8) = lVar3;
  *(longlong *)(lVar3 + 0x10) = lVar3;
  *(undefined2 *)(lVar3 + 0x18) = 0x101;
  param_1[0x87] = lVar3;
  param_1[0x89] = 0;
  param_1[0x8a] = 0;
  lVar3 = FUN_14019b780(&DAT_143ad68a0,0x30);
  *(longlong *)lVar3 = lVar3;
  *(longlong *)(lVar3 + 8) = lVar3;
  *(longlong *)(lVar3 + 0x10) = lVar3;
  *(undefined2 *)(lVar3 + 0x18) = 0x101;
  param_1[0x89] = lVar3;
  param_1[0x8b] = 0;
  return param_1;
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
// FUN_142a1d8a0 @ 142a1d8a0   (4667 bytes)
//===========================================================

ulonglong FUN_142a1d8a0(ulonglong param_1)

{
  longlong *plVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  int *piVar4;
  undefined4 *puVar5;
  longlong lVar6;
  int iVar7;
  longlong lVar8;
  int iVar9;
  int *piVar10;
  int iVar11;
  int *piVar12;
  ulonglong uVar13;
  int *piVar14;
  longlong lVar15;
  int iVar16;
  int *piVar17;
  longlong local_res10;
  undefined4 local_res18 [2];
  undefined4 local_res20;
  undefined4 uStackX_24;
  int *local_c0;
  longlong local_b8;
  undefined4 local_b0;
  undefined4 local_ac;
  undefined4 local_a8;
  undefined4 local_a4;
  longlong local_a0;
  longlong local_98;
  longlong local_90;
  longlong local_88;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  undefined8 local_68;
  longlong local_60;
  undefined8 local_58 [3];
  
  piVar17 = (int *)0x0;
  iVar11 = 0;
  uVar2 = FUN_142c4a030(DAT_143ac1898);
  local_res10 = CONCAT44(local_res10._4_4_,uVar2);
  local_68 = FUN_1408f6690();
  local_res18[0] = (*DAT_143262db0)();
  local_res20 = FUN_142c50c50();
  local_b0 = FUN_141892a90();
  uVar3 = FUN_1408fadb0(&local_60,&local_68);
  local_ac = 100;
  FUN_142a24230(param_1,"VERSION",&local_ac,"DATETIME",uVar3,&DAT_143487774,&local_b0,"LastUseName",
                &DAT_143adc708,"State",&local_res20,"Time1",local_res18,"Time2",&local_res10);
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  plVar1 = DAT_143aa84a0;
  if (DAT_143aa84a0 == (longlong *)0x0) {
    uVar3 = FUN_142a245b0(&local_res20,param_1,&DAT_143273ad4,&DAT_143271f04,"Channel",
                          &DAT_143271f04,&DAT_143273ac0,&DAT_143271f04,&DAT_143487790,&DAT_143271f04
                         );
    FUN_140319ad0(param_1,uVar3);
    if (CONCAT44(uStackX_24,local_res20) != 0) {
      FUN_14019f2c0(CONCAT44(uStackX_24,local_res20) + -0x10);
    }
    goto LAB_142a1ea24;
  }
  local_a4 = (**(code **)(*DAT_143aa84a0 + 0xa8))(DAT_143aa84a0);
  local_58[0] = FUN_142cb9610(plVar1);
  local_a8 = FUN_142cb9260(plVar1);
  local_res20 = FUN_142cb9230(plVar1);
  local_c0 = (int *)0x0;
  FUN_1408bc980(&local_88,param_1);
  lVar8 = local_88;
  piVar10 = piVar17;
  if (local_88 != 0) {
    iVar16 = *(int *)(local_88 + -8);
    piVar12 = (int *)(longlong)iVar16;
    if (iVar16 != 0) {
      iVar7 = 0;
      if (0 < iVar16) {
        iVar7 = iVar16;
      }
      piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar7 + 0x11));
      piVar4[1] = iVar7;
      *piVar4 = -1;
      piVar10 = piVar4 + 4;
      piVar4[2] = 0;
      *(char *)piVar10 = '\0';
      local_c0 = piVar10;
      FUN_142ef7ba0(piVar10,lVar8,piVar12);
      if (*piVar4 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar16 == -1) || (iVar16 <= piVar4[1])) {
        *piVar4 = 1;
        if (iVar16 != -1) goto LAB_142a1dab0;
        piVar12 = piVar17;
        if (piVar10 != (int *)0x0) {
          piVar12 = (int *)0xffffffffffffffff;
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
          } while (*(char *)((longlong)piVar10 + (longlong)piVar12) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar4[1],iVar16);
        *piVar4 = 1;
LAB_142a1dab0:
        *(char *)((longlong)piVar12 + (longlong)piVar10) = '\0';
      }
      iVar16 = (int)piVar12;
      if ((iVar16 < 0) || (piVar4[1] + 1 <= iVar16)) {
        FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
      }
      piVar4[2] = iVar16;
    }
  }
  if (local_88 != 0) {
    FUN_14019f2c0(local_88 + -0x10);
  }
  local_80 = 0;
  uVar3 = FUN_14019ba10(&local_80,&DAT_143272338,&DAT_143273ad4);
  local_b8 = 0;
  FUN_14019a260(&local_b8,uVar3);
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  lVar8 = local_b8;
  piVar12 = piVar10;
  if (local_b8 != 0) {
    iVar16 = *(int *)(local_b8 + -8);
    piVar4 = (int *)(longlong)iVar16;
    if (iVar16 != 0) {
      piVar14 = piVar17;
      if (piVar10 == (int *)0x0) goto LAB_142a1dcdf;
      if ((char)*piVar10 != '\0') {
        iVar7 = piVar10[-2];
        for (iVar9 = piVar10[-3]; iVar9 < iVar7 + iVar16; iVar9 = iVar9 * 2) {
        }
        piVar17 = piVar10 + -4;
        if (piVar17 == (int *)0x0) {
LAB_142a1dbdf:
          if (iVar11 < iVar9) {
            iVar11 = iVar9;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
          puVar5[1] = iVar11;
          *puVar5 = 0xffffffff;
          piVar12 = puVar5 + 4;
          local_c0 = piVar12;
          if (piVar17 == (int *)0x0) {
            puVar5[2] = 0;
            *(char *)piVar12 = '\0';
          }
          else {
            iVar9 = piVar10[-2] + 1;
            if (iVar11 + 1 < iVar9) {
              FUN_142e54290(0x5c,iVar9,iVar11 + 1);
              iVar9 = iVar11 + 1;
            }
            FUN_142ef7ba0(piVar12,piVar10,(longlong)iVar9);
            puVar5[2] = piVar10[-2];
            *(char *)((longlong)iVar11 + (longlong)piVar12) = '\0';
            FUN_14019f2c0(piVar17);
          }
        }
        else {
          if ((1 < *piVar17) || (piVar10[-3] < iVar9)) {
            iVar11 = piVar10[-2];
            goto LAB_142a1dbdf;
          }
          if (*piVar17 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar17 = -1;
        }
        iVar11 = 0;
        if (piVar12 != (int *)0x0) {
          iVar11 = piVar12[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar11 + (longlong)piVar12),lVar8,piVar4);
        FUN_14019c870(&local_c0,iVar7 + iVar16);
        goto LAB_142a1dd80;
      }
      if ((piVar10 == (int *)0x0) || (piVar14 = piVar10 + -4, piVar14 == (int *)0x0)) {
LAB_142a1dcdf:
        if (iVar11 < iVar16) {
          iVar11 = iVar16;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
        puVar5[1] = iVar11;
        *puVar5 = 0xffffffff;
        piVar12 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar12 = '\0';
        local_c0 = piVar12;
        if (piVar14 != (int *)0x0) {
          FUN_14019f2c0(piVar14);
        }
      }
      else {
        if ((1 < *piVar14) || (piVar10[-3] < iVar16)) {
          iVar11 = piVar10[-2];
          goto LAB_142a1dcdf;
        }
        if (*piVar14 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar14 = -1;
      }
      FUN_142ef7ba0(piVar12,lVar8,piVar4);
      if (piVar12[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar16 == -1) || (iVar16 <= piVar12[-3])) {
        piVar12[-4] = 1;
        if (iVar16 != -1) goto LAB_142a1dd5d;
        if (piVar12 != (int *)0x0) {
          piVar17 = (int *)0xffffffffffffffff;
          do {
            piVar17 = (int *)((longlong)piVar17 + 1);
          } while (*(char *)((longlong)piVar12 + (longlong)piVar17) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar12[-3],iVar16);
        piVar12[-4] = 1;
LAB_142a1dd5d:
        *(char *)((longlong)piVar4 + (longlong)piVar12) = '\0';
        piVar17 = piVar4;
      }
      iVar11 = (int)piVar17;
      if ((iVar11 < 0) || (piVar12[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
      }
      piVar12[-2] = iVar11;
    }
  }
LAB_142a1dd80:
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  FUN_1408bc080(&local_a0,&local_res20);
  lVar8 = local_a0;
  if (local_a0 != 0) {
    iVar11 = *(int *)(local_a0 + -8);
    uVar13 = (ulonglong)iVar11;
    if (iVar11 != 0) {
      if (piVar12 == (int *)0x0) {
LAB_142a1df3d:
        piVar17 = (int *)0x0;
LAB_142a1df3f:
        iVar16 = 0;
LAB_142a1df41:
        if (iVar16 < iVar11) {
          iVar16 = iVar11;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
        puVar5[1] = iVar16;
        *puVar5 = 0xffffffff;
        piVar12 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar12 = '\0';
        local_c0 = piVar12;
        if (piVar17 != (int *)0x0) {
          FUN_14019f2c0(piVar17);
        }
      }
      else {
        if ((char)*piVar12 != '\0') {
          iVar16 = piVar12[-2];
          for (iVar7 = piVar12[-3]; iVar7 < iVar16 + iVar11; iVar7 = iVar7 * 2) {
          }
          piVar17 = piVar12 + -4;
          if (piVar17 == (int *)0x0) {
            iVar9 = 0;
LAB_142a1de4f:
            if (iVar9 < iVar7) {
              iVar9 = iVar7;
            }
            puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
            puVar5[1] = iVar9;
            *puVar5 = 0xffffffff;
            piVar10 = puVar5 + 4;
            local_c0 = piVar10;
            if (piVar17 == (int *)0x0) {
              puVar5[2] = 0;
              *(char *)piVar10 = '\0';
            }
            else {
              iVar7 = piVar12[-2] + 1;
              if (iVar9 + 1 < iVar7) {
                FUN_142e54290(0x5c,iVar7,iVar9 + 1);
                iVar7 = iVar9 + 1;
              }
              FUN_142ef7ba0(piVar10,piVar12,(longlong)iVar7);
              puVar5[2] = piVar12[-2];
              *(char *)((longlong)iVar9 + (longlong)piVar10) = '\0';
              FUN_14019f2c0(piVar17);
            }
          }
          else {
            if ((1 < *piVar17) || (piVar12[-3] < iVar7)) {
              iVar9 = piVar12[-2];
              goto LAB_142a1de4f;
            }
            if (*piVar17 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar17 = -1;
            piVar10 = piVar12;
          }
          if (piVar10 == (int *)0x0) {
            iVar7 = 0;
          }
          else {
            iVar7 = piVar10[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar7 + (longlong)piVar10),lVar8,uVar13);
          FUN_14019c870(&local_c0,iVar16 + iVar11);
          piVar12 = piVar10;
          goto LAB_142a1dfe9;
        }
        if (piVar12 == (int *)0x0) goto LAB_142a1df3d;
        piVar17 = piVar12 + -4;
        if (piVar17 == (int *)0x0) goto LAB_142a1df3f;
        if ((1 < *piVar17) || (piVar12[-3] < iVar11)) {
          iVar16 = piVar12[-2];
          goto LAB_142a1df41;
        }
        if (*piVar17 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar17 = -1;
      }
      FUN_142ef7ba0(piVar12,lVar8,uVar13);
      if (piVar12[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar11 == -1) || (iVar11 <= piVar12[-3])) {
        piVar12[-4] = 1;
        if (iVar11 != -1) goto LAB_142a1dfc2;
        if (piVar12 == (int *)0x0) {
          uVar13 = 0;
        }
        else {
          uVar13 = 0xffffffffffffffff;
          do {
            uVar13 = uVar13 + 1;
          } while (*(char *)((longlong)piVar12 + uVar13) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar12[-3],iVar11);
        piVar12[-4] = 1;
LAB_142a1dfc2:
        *(char *)(uVar13 + (longlong)piVar12) = '\0';
      }
      iVar11 = (int)uVar13;
      if ((iVar11 < 0) || (piVar12[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,uVar13 & 0xffffffff);
      }
      piVar12[-2] = iVar11;
    }
  }
LAB_142a1dfe9:
  if (local_a0 != 0) {
    FUN_14019f2c0(local_a0 + -0x10);
  }
  iVar16 = 0;
  piVar17 = (int *)0x0;
  iVar11 = 0;
  local_78 = 0;
  uVar3 = FUN_14019ba10(&local_78,&DAT_143272338,"Channel");
  local_b8 = 0;
  FUN_14019a260(&local_b8,uVar3);
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  lVar8 = local_b8;
  piVar10 = piVar12;
  if (local_b8 != 0) {
    iVar7 = *(int *)(local_b8 + -8);
    piVar4 = (int *)(longlong)iVar7;
    if (iVar7 != 0) {
      piVar14 = piVar17;
      if (piVar12 == (int *)0x0) goto LAB_142a1e1de;
      if ((char)*piVar12 != '\0') {
        iVar16 = piVar12[-2];
        for (iVar9 = piVar12[-3]; iVar9 < iVar16 + iVar7; iVar9 = iVar9 * 2) {
        }
        piVar17 = piVar12 + -4;
        if (piVar17 == (int *)0x0) {
LAB_142a1e0e9:
          if (iVar11 < iVar9) {
            iVar11 = iVar9;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
          puVar5[1] = iVar11;
          *puVar5 = 0xffffffff;
          piVar10 = puVar5 + 4;
          local_c0 = piVar10;
          if (piVar17 == (int *)0x0) {
            puVar5[2] = 0;
            *(char *)piVar10 = '\0';
          }
          else {
            iVar9 = piVar12[-2] + 1;
            if (iVar11 + 1 < iVar9) {
              FUN_142e54290(0x5c,iVar9,iVar11 + 1);
              iVar9 = iVar11 + 1;
            }
            FUN_142ef7ba0(piVar10,piVar12,(longlong)iVar9);
            puVar5[2] = piVar12[-2];
            *(char *)((longlong)iVar11 + (longlong)piVar10) = '\0';
            FUN_14019f2c0(piVar17);
          }
        }
        else {
          if ((1 < *piVar17) || (piVar12[-3] < iVar9)) {
            iVar11 = piVar12[-2];
            goto LAB_142a1e0e9;
          }
          if (*piVar17 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar17 = -1;
        }
        if (piVar10 == (int *)0x0) {
          iVar11 = 0;
        }
        else {
          iVar11 = piVar10[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar11 + (longlong)piVar10),lVar8,piVar4);
        FUN_14019c870(&local_c0,iVar16 + iVar7);
        goto LAB_142a1e282;
      }
      if ((piVar12 == (int *)0x0) || (piVar14 = piVar12 + -4, piVar14 == (int *)0x0)) {
LAB_142a1e1de:
        if (iVar16 < iVar7) {
          iVar16 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
        puVar5[1] = iVar16;
        *puVar5 = 0xffffffff;
        piVar10 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar10 = '\0';
        local_c0 = piVar10;
        if (piVar14 != (int *)0x0) {
          FUN_14019f2c0(piVar14);
        }
      }
      else {
        if ((1 < *piVar14) || (piVar12[-3] < iVar7)) {
          iVar16 = piVar12[-2];
          goto LAB_142a1e1de;
        }
        if (*piVar14 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar14 = -1;
      }
      FUN_142ef7ba0(piVar10,lVar8,piVar4);
      if (piVar10[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar7 == -1) || (iVar7 <= piVar10[-3])) {
        piVar10[-4] = 1;
        if (iVar7 != -1) goto LAB_142a1e25f;
        if (piVar10 != (int *)0x0) {
          piVar17 = (int *)0xffffffffffffffff;
          do {
            piVar17 = (int *)((longlong)piVar17 + 1);
          } while (*(char *)((longlong)piVar10 + (longlong)piVar17) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar10[-3],iVar7);
        piVar10[-4] = 1;
LAB_142a1e25f:
        *(char *)((longlong)piVar4 + (longlong)piVar10) = '\0';
        piVar17 = piVar4;
      }
      iVar11 = (int)piVar17;
      if ((iVar11 < 0) || (piVar10[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
      }
      piVar10[-2] = iVar11;
    }
  }
LAB_142a1e282:
  piVar17 = (int *)0x0;
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  FUN_1408bc080(&local_98,&local_a8);
  lVar8 = local_98;
  if (local_98 != 0) {
    iVar11 = *(int *)(local_98 + -8);
    piVar12 = (int *)(longlong)iVar11;
    if (iVar11 != 0) {
      iVar16 = 0;
      piVar4 = piVar17;
      if (piVar10 == (int *)0x0) goto LAB_142a1e443;
      if ((char)*piVar10 == '\0') {
        if ((piVar10 == (int *)0x0) || (piVar4 = piVar10 + -4, piVar4 == (int *)0x0)) {
LAB_142a1e443:
          if (iVar16 < iVar11) {
            iVar16 = iVar11;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
          puVar5[1] = iVar16;
          *puVar5 = 0xffffffff;
          piVar10 = puVar5 + 4;
          puVar5[2] = 0;
          *(char *)piVar10 = '\0';
          local_c0 = piVar10;
          if (piVar4 != (int *)0x0) {
            FUN_14019f2c0(piVar4);
          }
        }
        else {
          if ((1 < *piVar4) || (piVar10[-3] < iVar11)) {
            iVar16 = piVar10[-2];
            goto LAB_142a1e443;
          }
          if (*piVar4 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar4 = -1;
        }
        FUN_142ef7ba0(piVar10,lVar8,piVar12);
        if (piVar10[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar11 == -1) || (iVar11 <= piVar10[-3])) {
          piVar10[-4] = 1;
          if (iVar11 != -1) goto LAB_142a1e4cb;
          if (piVar10 != (int *)0x0) {
            piVar17 = (int *)0xffffffffffffffff;
            do {
              piVar17 = (int *)((longlong)piVar17 + 1);
            } while (*(char *)((longlong)piVar10 + (longlong)piVar17) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar10[-3],iVar11);
          piVar10[-4] = 1;
LAB_142a1e4cb:
          *(char *)((longlong)piVar12 + (longlong)piVar10) = '\0';
          piVar17 = piVar12;
        }
        iVar11 = (int)piVar17;
        if ((iVar11 < 0) || (piVar10[-3] + 1 <= iVar11)) {
          FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
        }
        piVar10[-2] = iVar11;
        goto LAB_142a1e4f2;
      }
      iVar16 = piVar10[-2];
      for (iVar7 = piVar10[-3]; iVar7 < iVar16 + iVar11; iVar7 = iVar7 * 2) {
      }
      piVar17 = piVar10 + -4;
      if (piVar17 == (int *)0x0) {
        iVar9 = 0;
LAB_142a1e34f:
        if (iVar9 < iVar7) {
          iVar9 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
        puVar5[1] = iVar9;
        *puVar5 = 0xffffffff;
        piVar4 = puVar5 + 4;
        local_c0 = piVar4;
        if (piVar17 == (int *)0x0) {
          puVar5[2] = 0;
          *(char *)piVar4 = '\0';
        }
        else {
          iVar7 = piVar10[-2] + 1;
          if (iVar9 + 1 < iVar7) {
            FUN_142e54290(0x5c,iVar7,iVar9 + 1);
            iVar7 = iVar9 + 1;
          }
          FUN_142ef7ba0(piVar4,piVar10,(longlong)iVar7);
          puVar5[2] = piVar10[-2];
          *(char *)((longlong)iVar9 + (longlong)piVar4) = '\0';
          FUN_14019f2c0(piVar17);
        }
      }
      else {
        if ((1 < *piVar17) || (piVar10[-3] < iVar7)) {
          iVar9 = piVar10[-2];
          goto LAB_142a1e34f;
        }
        if (*piVar17 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar17 = -1;
        piVar4 = piVar10;
      }
      if (piVar4 == (int *)0x0) {
        iVar7 = 0;
      }
      else {
        iVar7 = piVar4[-2];
      }
      FUN_142ef7ba0((char *)((longlong)iVar7 + (longlong)piVar4),lVar8,piVar12);
      FUN_14019c870(&local_c0,iVar16 + iVar11);
      piVar10 = piVar4;
    }
  }
LAB_142a1e4f2:
  if (local_98 != 0) {
    FUN_14019f2c0(local_98 + -0x10);
  }
  piVar17 = (int *)0x0;
  local_70 = 0;
  uVar3 = FUN_14019ba10(&local_70,&DAT_143272338,&DAT_143273ac0);
  local_b8 = 0;
  FUN_14019a260(&local_b8,uVar3);
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  lVar8 = local_b8;
  piVar12 = piVar10;
  if (local_b8 != 0) {
    iVar11 = *(int *)(local_b8 + -8);
    piVar4 = (int *)(longlong)iVar11;
    if (iVar11 != 0) {
      iVar16 = 0;
      piVar14 = piVar17;
      if (piVar10 == (int *)0x0) goto LAB_142a1e6fa;
      if ((char)*piVar10 != '\0') {
        iVar7 = piVar10[-2];
        for (iVar9 = piVar10[-3]; iVar9 < iVar7 + iVar11; iVar9 = iVar9 * 2) {
        }
        piVar17 = piVar10 + -4;
        if (piVar17 == (int *)0x0) {
LAB_142a1e600:
          if (iVar16 < iVar9) {
            iVar16 = iVar9;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
          puVar5[1] = iVar16;
          *puVar5 = 0xffffffff;
          piVar12 = puVar5 + 4;
          local_c0 = piVar12;
          if (piVar17 == (int *)0x0) {
            puVar5[2] = 0;
            *(char *)piVar12 = '\0';
          }
          else {
            iVar9 = piVar10[-2] + 1;
            if (iVar16 + 1 < iVar9) {
              FUN_142e54290(0x5c,iVar9,iVar16 + 1);
              iVar9 = iVar16 + 1;
            }
            FUN_142ef7ba0(piVar12,piVar10,(longlong)iVar9);
            puVar5[2] = piVar10[-2];
            *(char *)((longlong)iVar16 + (longlong)piVar12) = '\0';
            FUN_14019f2c0(piVar17);
          }
        }
        else {
          if ((1 < *piVar17) || (piVar10[-3] < iVar9)) {
            iVar16 = piVar10[-2];
            goto LAB_142a1e600;
          }
          if (*piVar17 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar17 = -1;
        }
        if (piVar12 == (int *)0x0) {
          iVar16 = 0;
        }
        else {
          iVar16 = piVar12[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar16 + (longlong)piVar12),lVar8,piVar4);
        FUN_14019c870(&local_c0,iVar7 + iVar11);
        goto LAB_142a1e7a5;
      }
      if ((piVar10 == (int *)0x0) || (piVar14 = piVar10 + -4, piVar14 == (int *)0x0)) {
LAB_142a1e6fa:
        if (iVar16 < iVar11) {
          iVar16 = iVar11;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
        puVar5[1] = iVar16;
        *puVar5 = 0xffffffff;
        piVar12 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar12 = '\0';
        local_c0 = piVar12;
        if (piVar14 != (int *)0x0) {
          FUN_14019f2c0(piVar14);
        }
      }
      else {
        if ((1 < *piVar14) || (piVar10[-3] < iVar11)) {
          iVar16 = piVar10[-2];
          goto LAB_142a1e6fa;
        }
        if (*piVar14 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar14 = -1;
      }
      FUN_142ef7ba0(piVar12,lVar8,piVar4);
      if (piVar12[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar11 == -1) || (iVar11 <= piVar12[-3])) {
        piVar12[-4] = 1;
        if (iVar11 != -1) goto LAB_142a1e782;
        piVar10 = (int *)0xffffffffffffffff;
        if (piVar12 != (int *)0x0) {
          do {
            piVar17 = (int *)((longlong)piVar10 + 1);
            piVar10 = piVar17;
          } while (*(char *)((longlong)piVar12 + (longlong)piVar17) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar12[-3],iVar11);
        piVar12[-4] = 1;
LAB_142a1e782:
        *(char *)((longlong)piVar4 + (longlong)piVar12) = '\0';
        piVar17 = piVar4;
      }
      iVar11 = (int)piVar17;
      if ((iVar11 < 0) || (piVar12[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
      }
      piVar12[-2] = iVar11;
    }
  }
LAB_142a1e7a5:
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  FUN_1408bc4c0(&local_90,local_58);
  lVar8 = local_90;
  iVar11 = 0;
  if (local_90 != 0) {
    iVar16 = *(int *)(local_90 + -8);
    lVar15 = (longlong)iVar16;
    if (iVar16 != 0) {
      if ((piVar12 == (int *)0x0) || ((char)*piVar12 == '\0')) {
        uVar3 = FUN_14019bd40(&local_c0,iVar16,0);
        FUN_142ef7ba0(uVar3,lVar8,lVar15);
      }
      else {
        iVar16 = piVar12[-2] + iVar16;
        for (iVar7 = piVar12[-3]; iVar7 < iVar16; iVar7 = iVar7 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_c0,iVar7,1);
        iVar7 = iVar11;
        if (local_c0 != (int *)0x0) {
          iVar7 = local_c0[-2];
        }
        FUN_142ef7ba0(iVar7 + lVar6,lVar8,lVar15);
      }
      FUN_14019c870(&local_c0,iVar16);
    }
  }
  if (local_90 != 0) {
    FUN_14019f2c0(local_90 + -0x10);
  }
  FUN_1401d7f50(&local_c0,&DAT_143487790,param_1 & 0xff);
  FUN_1408bc080(&local_b8,&local_a4);
  lVar8 = local_b8;
  if (local_b8 != 0) {
    iVar16 = *(int *)(local_b8 + -8);
    lVar15 = (longlong)iVar16;
    if (iVar16 != 0) {
      if ((local_c0 == (int *)0x0) || ((char)*local_c0 == '\0')) {
        uVar3 = FUN_14019bd40(&local_c0,iVar16,0);
        FUN_142ef7ba0(uVar3,lVar8,lVar15);
      }
      else {
        iVar16 = local_c0[-2] + iVar16;
        for (iVar7 = local_c0[-3]; iVar7 < iVar16; iVar7 = iVar7 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_c0,iVar7,1);
        if (local_c0 != (int *)0x0) {
          iVar11 = local_c0[-2];
        }
        FUN_142ef7ba0(iVar11 + lVar6,lVar8,lVar15);
      }
      FUN_14019c870(&local_c0,iVar16);
    }
  }
  if (local_b8 != 0) {
    FUN_14019f2c0(local_b8 + -0x10);
  }
  FUN_140319ad0(param_1,&local_c0);
  if (local_c0 != (int *)0x0) {
    FUN_14019f2c0(local_c0 + -4);
  }
LAB_142a1ea24:
  if (DAT_143ac18a0 == 0) {
    uVar3 = FUN_142a247e0(&local_res20,param_1,"Socket",&DAT_143271f04,&DAT_143271f04,&DAT_143271f04
                         );
    FUN_140319ad0(param_1,uVar3);
    lVar8 = CONCAT44(uStackX_24,local_res20);
  }
  else {
    local_res10 = 0;
    FUN_142a24da0(&local_res10,param_1,"Socket",DAT_143ac18a0 + 0x50,DAT_143ac18a0 + 0xc,
                  DAT_143ac18a0 + 0x154);
    FUN_140319ad0(param_1,&local_res10);
    lVar8 = local_res10;
  }
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  return param_1;
}



//===========================================================
// FUN_142e5d800 @ 142e5d800   (782 bytes)
//===========================================================

longlong *
FUN_142e5d800(longlong *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
             undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
             undefined8 param_9,undefined8 param_10)

{
  char *pcVar1;
  longlong lVar2;
  longlong lVar3;
  undefined4 *puVar4;
  int iVar5;
  int iVar6;
  int *piVar7;
  longlong lVar8;
  int iVar9;
  int *piVar10;
  int iVar11;
  longlong local_30;
  
  piVar7 = (int *)0x0;
  iVar5 = 0;
  *param_1 = 0;
  FUN_1408bc980(&local_30);
  lVar2 = local_30;
  iVar6 = 0;
  if (local_30 != 0) {
    iVar9 = *(int *)(local_30 + -8);
    lVar8 = (longlong)iVar9;
    if (iVar9 != 0) {
      pcVar1 = (char *)*param_1;
      piVar10 = piVar7;
      iVar11 = iVar6;
      if (pcVar1 == (char *)0x0) goto LAB_142e5d8ee;
      if (*pcVar1 == '\0') {
        piVar10 = (int *)(pcVar1 + -0x10);
        if (piVar10 == (int *)0x0) {
LAB_142e5d8ee:
          if (iVar11 < iVar9) {
            iVar11 = iVar9;
          }
          puVar4 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
          puVar4[1] = iVar11;
          *puVar4 = 0xffffffff;
          *param_1 = (longlong)(puVar4 + 4);
          puVar4[2] = 0;
          *(undefined1 *)*param_1 = 0;
          if (piVar10 != (int *)0x0) {
            FUN_14019f2c0(piVar10);
          }
        }
        else {
          if ((1 < *piVar10) || (*(int *)(pcVar1 + -0xc) < iVar9)) {
            iVar11 = *(int *)(pcVar1 + -8);
            goto LAB_142e5d8ee;
          }
          if (*piVar10 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar10 = -1;
        }
        FUN_142ef7ba0(*param_1,lVar2,lVar8);
      }
      else {
        iVar9 = *(int *)(pcVar1 + -8) + iVar9;
        for (iVar11 = *(int *)(pcVar1 + -0xc); iVar11 < iVar9; iVar11 = iVar11 * 2) {
        }
        lVar3 = FUN_14019bd40(param_1,iVar11,1);
        iVar11 = iVar5;
        if (*param_1 != 0) {
          iVar11 = *(int *)(*param_1 + -8);
        }
        FUN_142ef7ba0(iVar11 + lVar3,lVar2,lVar8);
      }
      FUN_14019c870(param_1,iVar9);
    }
  }
  if (local_30 != 0) {
    FUN_14019f2c0(local_30 + -0x10);
  }
  FUN_1408b6250(param_1,param_3,(ulonglong)param_1 & 0xff);
  FUN_1401ab0c0(param_1,param_4,(ulonglong)param_1 & 0xff);
  FUN_1408bc140(&local_30,param_5);
  lVar2 = local_30;
  if (local_30 == 0) goto LAB_142e5da9b;
  iVar9 = *(int *)(local_30 + -8);
  lVar8 = (longlong)iVar9;
  if (iVar9 == 0) goto LAB_142e5da9b;
  pcVar1 = (char *)*param_1;
  if (pcVar1 == (char *)0x0) goto LAB_142e5da3c;
  if (*pcVar1 == '\0') {
    piVar7 = (int *)(pcVar1 + -0x10);
    if (piVar7 == (int *)0x0) {
LAB_142e5da3c:
      if (iVar6 < iVar9) {
        iVar6 = iVar9;
      }
      puVar4 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
      puVar4[1] = iVar6;
      *puVar4 = 0xffffffff;
      *param_1 = (longlong)(puVar4 + 4);
      puVar4[2] = 0;
      *(undefined1 *)*param_1 = 0;
      if (piVar7 != (int *)0x0) {
        FUN_14019f2c0(piVar7);
      }
    }
    else {
      if ((1 < *piVar7) || (*(int *)(pcVar1 + -0xc) < iVar9)) {
        iVar6 = *(int *)(pcVar1 + -8);
        goto LAB_142e5da3c;
      }
      if (*piVar7 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar7 = -1;
    }
    FUN_142ef7ba0(*param_1,lVar2,lVar8);
  }
  else {
    iVar9 = *(int *)(pcVar1 + -8) + iVar9;
    for (iVar6 = *(int *)(pcVar1 + -0xc); iVar6 < iVar9; iVar6 = iVar6 * 2) {
    }
    lVar3 = FUN_14019bd40(param_1,iVar6,1);
    if (*param_1 != 0) {
      iVar5 = *(int *)(*param_1 + -8);
    }
    FUN_142ef7ba0(iVar5 + lVar3,lVar2,lVar8);
  }
  FUN_14019c870(param_1,iVar9);
LAB_142e5da9b:
  if (local_30 != 0) {
    FUN_14019f2c0(local_30 + -0x10);
  }
  FUN_1401ab0c0(param_1,param_6,(ulonglong)param_1 & 0xff);
  FUN_142e5e540(param_1,param_7,param_8,param_9,param_10);
  return param_1;
}



//===========================================================
// FUN_142a1ec10 @ 142a1ec10   (61 bytes)
//===========================================================

void FUN_142a1ec10(undefined8 param_1)

{
  undefined8 uVar1;
  undefined8 local_res10;
  undefined8 *local_res18;
  
  local_res18 = &local_res10;
  local_res10 = 0;
  FUN_14019a260(&local_res10,param_1);
  uVar1 = FUN_142e56bc0();
  FUN_142a23830(uVar1,&local_res10);
  return;
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


