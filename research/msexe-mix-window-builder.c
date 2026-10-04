
//===========================================================
// FUN_142a8ff80 @ 142a8ff80   (6965 bytes)
//===========================================================

void FUN_142a8ff80(IUnknown *param_1,longlong param_2)

{
  IUnknown *pIVar1;
  undefined4 uVar2;
  IUnknown *pIVar3;
  IUnknown *pIVar4;
  char cVar5;
  short sVar6;
  uint uVar7;
  undefined4 uVar8;
  uint uVar9;
  int iVar10;
  int *piVar11;
  int *piVar12;
  longlong *plVar13;
  undefined8 *puVar14;
  undefined8 uVar15;
  longlong lVar16;
  int iVar17;
  IUnknown *pIVar18;
  uint uVar19;
  longlong lVar20;
  int *piVar21;
  wchar_t *pwVar22;
  uint uVar23;
  char *pcVar24;
  uint uVar25;
  ulonglong uVar26;
  int iVar27;
  undefined8 local_res8;
  undefined4 local_res10 [2];
  undefined4 local_res18 [2];
  IUnknown *local_res20;
  undefined8 in_stack_fffffffffffffd58;
  undefined4 uVar28;
  undefined8 in_stack_fffffffffffffd60;
  undefined4 uVar30;
  uint *puVar29;
  undefined8 in_stack_fffffffffffffd68;
  undefined4 uVar32;
  undefined8 uVar31;
  IUnknown *local_278;
  ulonglong local_270;
  IUnknown *local_268;
  IUnknown *local_260;
  IUnknown *local_258;
  undefined8 local_250;
  IUnknown *local_248;
  IUnknown *local_240;
  longlong *local_238;
  undefined4 local_230;
  undefined4 uStack_22c;
  undefined8 uStack_228;
  undefined8 local_220;
  undefined4 local_218;
  undefined4 uStack_214;
  undefined8 uStack_210;
  undefined8 local_208;
  undefined4 local_200;
  undefined4 uStack_1fc;
  undefined8 uStack_1f8;
  undefined8 local_1f0;
  short local_1e8;
  undefined6 uStack_1e6;
  longlong lStack_1e0;
  undefined8 local_1d8;
  short local_1d0;
  undefined6 uStack_1ce;
  longlong lStack_1c8;
  undefined8 local_1c0;
  undefined4 local_1b8;
  undefined4 uStack_1b4;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  IUnknown *local_1a0;
  IUnknown *local_198;
  IUnknown *local_190;
  undefined4 local_188 [2];
  longlong *local_180;
  longlong local_178;
  longlong local_170;
  longlong *local_168;
  IUnknown *local_160;
  undefined1 local_158 [8];
  longlong local_150;
  longlong local_148;
  IUnknown *local_140;
  undefined4 *local_138;
  undefined4 *local_130;
  undefined4 *local_128;
  undefined1 local_120 [8];
  longlong local_118;
  uint local_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined8 local_f8;
  uint local_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined8 local_d8;
  undefined8 local_c8;
  longlong lStack_c0;
  undefined8 local_b8;
  undefined8 local_a8;
  longlong lStack_a0;
  undefined8 local_98;
  uint local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 local_78;
  uint local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  
  uVar32 = (undefined4)((ulonglong)in_stack_fffffffffffffd68 >> 0x20);
  uVar30 = (undefined4)((ulonglong)in_stack_fffffffffffffd60 >> 0x20);
  uVar28 = (undefined4)((ulonglong)in_stack_fffffffffffffd58 >> 0x20);
  lVar20 = *(longlong *)(param_2 + 0x10);
  local_res8 = param_1;
  local_150 = lVar20;
  if (lVar20 != 0) {
    if (0xfffff < *(ulonglong *)(lVar20 + 8)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar20 + 8) = *(longlong *)(lVar20 + 8) + 1;
    UNLOCK();
  }
  lVar20 = *(longlong *)(param_2 + 0x20);
  local_118 = lVar20;
  if (lVar20 != 0) {
    if (0xfffff < *(ulonglong *)(lVar20 + 8)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar20 + 8) = *(longlong *)(lVar20 + 8) + 1;
    UNLOCK();
  }
  local_188[0] = 1;
  local_res18[0] = 0xbd;
  local_res10[0] = 0xca;
  iVar10 = *(int *)(param_1 + 0x2a8);
  if (iVar10 == 0x12) {
    local_res18[0] = 0x96;
    local_res10[0] = 0xd0;
  }
  else if (iVar10 == 0x13) {
    local_res10[0] = 0xc6;
  }
  else {
    if (iVar10 == 0x14) {
      FUN_14022d860(&local_270,L"UI/UtilDlgEx.img/UtilDlgEx_Avatar/avatarPos",0xffffffff);
      FUN_14090faf0(&local_res20,&local_270);
      if (local_270 != 0) {
        FUN_14019f2c0(local_270 - 0x10);
      }
      if (local_res20 != (IUnknown *)0x0) {
        local_res18[0] = FUN_140319f60();
        if (local_res20 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        local_res10[0] = FUN_140319fa0();
      }
    }
    else if (iVar10 == 0x17) {
      FUN_14022d860(&local_270,L"UI/UtilDlgEx.img/UtilDlgEx_MixHair/avatarPos",0xffffffff);
      FUN_14090faf0(&local_res20,&local_270);
      if (local_270 != 0) {
        FUN_14019f2c0(local_270 - 0x10);
      }
      if (local_res20 != (IUnknown *)0x0) {
        local_res18[0] = FUN_140319f60();
        if (local_res20 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        local_res10[0] = FUN_140319fa0();
      }
    }
    else {
      if (iVar10 != 0x19) goto LAB_142a901e1;
      FUN_14022d860(&local_270,L"UI/UtilDlgEx.img/UtilDlgEx_MixLens/avatarPos",0xffffffff);
      FUN_14090faf0(&local_res20,&local_270);
      if (local_270 != 0) {
        FUN_14019f2c0(local_270 - 0x10);
      }
      if (local_res20 != (IUnknown *)0x0) {
        local_res18[0] = FUN_140319f60();
        if (local_res20 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        local_res10[0] = FUN_140319fa0();
      }
    }
    if (local_res20 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_res20 + 0x10))();
    }
  }
LAB_142a901e1:
  local_138 = local_188;
  local_130 = local_res18;
  local_128 = local_res10;
  lVar20 = *(longlong *)(param_2 + 0x10);
  piVar12 = (int *)0x0;
  uVar19 = 0;
  uVar25 = 0;
  local_140 = param_1;
  if (lVar20 != 0) {
    uVar23 = *(uint *)(param_2 + 200);
    if (-1 < (int)uVar23) {
      lVar16 = *(longlong *)(param_2 + 0xd0);
      uVar7 = uVar25;
      if (lVar16 != 0) {
        uVar7 = *(uint *)(lVar16 + -8);
      }
      if ((int)uVar23 < (int)uVar7) {
        uVar7 = uVar25;
        if (lVar16 != 0) {
          uVar7 = *(uint *)(lVar16 + -8);
        }
        if (uVar7 <= uVar23) {
          piVar21 = piVar12;
          if (lVar16 != 0) {
            piVar21 = (int *)(ulonglong)*(uint *)(lVar16 + -8);
          }
          FUN_142e54290(0xbc,uVar23,piVar21);
          lVar16 = *(longlong *)(param_2 + 0xd0);
        }
        FUN_1401a72a0(lVar20,*(undefined4 *)(param_2 + 0x70),
                      *(undefined4 *)(lVar16 + (longlong)(int)uVar23 * 4));
      }
    }
  }
  iVar10 = *(int *)(param_1 + 0x2a8);
  if (iVar10 == 0x12) {
    FUN_142a92de0(&local_140,param_2 + 0x50,local_158,0);
    if (((*(longlong *)(param_2 + 0x58) != 0) && (*(longlong *)(param_1 + 0x408) != 0)) &&
       (cVar5 = FUN_140db5600(), cVar5 != '\0')) {
      lVar20 = *(longlong *)(param_2 + 0x58);
      if (lVar20 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar20 = *(longlong *)(param_2 + 0x58);
      }
      lVar16 = *(longlong *)(param_1 + 0x408);
      if (lVar16 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar16 = *(longlong *)(param_1 + 0x408);
      }
      uVar8 = FUN_140db5690(lVar16);
      FUN_140f89ba0(lVar20,uVar8,0);
    }
  }
  else if ((iVar10 - 0x18U & 0xfffffffd) == 0) {
    local_res20 = (IUnknown *)0x0;
    pcVar24 = "UtilDlgEx_MixLens_New";
    if (iVar10 == 0x18) {
      pcVar24 = "UtilDlgEx_MixHair_New";
    }
    uVar15 = FUN_14019ba10(&local_res20,"UI/UtilDlgEx.img/%s/",pcVar24);
    local_270 = 0;
    FUN_14019a260(&local_270,uVar15);
    if (local_res20 != (IUnknown *)0x0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
    lVar20 = -1;
    do {
      lVar16 = lVar20 + 1;
      lVar20 = lVar20 + 1;
    } while ("avatarPosBefore"[lVar16] != '\0');
    FUN_1401abc80(&local_270,&local_1a0);
    FUN_14090faf0(&local_268,&local_1a0);
    pIVar1 = local_268;
    if (local_268 != (IUnknown *)0x0) {
      local_res20 = (IUnknown *)((ulonglong)local_res20 & 0xffffffff00000000);
      iVar10 = (**(code **)(*(longlong *)local_268 + 0x40))(local_268,&local_res20);
      if (iVar10 < 0) {
        _com_issue_errorex(iVar10,pIVar1,(_GUID *)&DAT_143273780);
      }
      pIVar1 = local_268;
      local_res18[0] = (uint)local_res20;
      if (local_268 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      local_res20 = (IUnknown *)((ulonglong)local_res20 & 0xffffffff00000000);
      iVar10 = (**(code **)(*(longlong *)local_268 + 0x50))(local_268,&local_res20);
      if (iVar10 < 0) {
        _com_issue_errorex(iVar10,pIVar1,(_GUID *)&DAT_143273780);
      }
      local_res10[0] = (uint)local_res20;
    }
    FUN_142a92de0(&local_140,param_2 + 0x60,local_120,0);
    lVar20 = -1;
    do {
      lVar16 = lVar20 + 1;
      lVar20 = lVar20 + 1;
    } while ("avatarPosAfter"[lVar16] != '\0');
    FUN_1401abc80(&local_270,&local_168);
    plVar13 = (longlong *)FUN_14090faf0(&local_198,&local_168);
    pIVar3 = local_268;
    pIVar1 = (IUnknown *)*plVar13;
    pIVar18 = local_268;
    if ((local_268 != pIVar1) && (*plVar13 = 0, pIVar18 = pIVar1, local_268 != (IUnknown *)0x0)) {
      lVar20 = *(longlong *)local_268;
      local_268 = pIVar1;
      (**(code **)(lVar20 + 0x10))(pIVar3);
      pIVar18 = local_268;
    }
    local_268 = pIVar18;
    if (local_198 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_198 + 0x10))();
    }
    pIVar1 = local_268;
    if (local_268 != (IUnknown *)0x0) {
      local_res20 = (IUnknown *)((ulonglong)local_res20 & 0xffffffff00000000);
      iVar10 = (**(code **)(*(longlong *)local_268 + 0x40))(local_268,&local_res20);
      if (iVar10 < 0) {
        _com_issue_errorex(iVar10,pIVar1,(_GUID *)&DAT_143273780);
      }
      pIVar1 = local_268;
      local_res18[0] = (uint)local_res20;
      if (local_268 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      local_res20 = (IUnknown *)((ulonglong)local_res20 & 0xffffffff00000000);
      iVar10 = (**(code **)(*(longlong *)local_268 + 0x50))(local_268,&local_res20);
      if (iVar10 < 0) {
        _com_issue_errorex(iVar10,pIVar1,(_GUID *)&DAT_143273780);
      }
      local_res10[0] = (uint)local_res20;
    }
    FUN_142a92de0(&local_140,param_2 + 0x50,local_158,0);
    if (local_168 != (longlong *)0x0) {
      FUN_14019f2c0(local_168 + -2);
    }
    if (local_268 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_268 + 0x10))();
    }
    if (local_1a0 != (IUnknown *)0x0) {
      FUN_14019f2c0(local_1a0 + -0x10);
    }
    if (local_270 != 0) {
      FUN_14019f2c0(local_270 - 0x10);
    }
  }
  else {
    FUN_142a92de0(&local_140,param_2 + 0x50,local_158,0);
  }
  pIVar1 = *(IUnknown **)(param_2 + 0x28);
  local_198 = pIVar1;
  if (pIVar1 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)pIVar1 + 8))(pIVar1);
    (*DAT_143262a20)(&local_230);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_230 == 8) {
        local_230 = (uint)local_230._2_2_ << 0x10;
        if (uStack_228 != (int *)0x0) {
          (*DAT_143ad5990)(uStack_228 + -1);
        }
      }
      else {
        iVar10 = (*DAT_143262a18)(&local_230);
        if (iVar10 < 0) goto LAB_142a91a5a;
      }
      lVar20 = DAT_143a8b8e0;
      local_230 = CONCAT22(local_230._2_2_,8);
      piVar21 = piVar12;
      if (DAT_143a8b8e0 != 0) {
        piVar21 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
      }
      piVar11 = (int *)(*DAT_143ad5980)((ulonglong)((int)piVar21 + 1) * 2 + 4);
      param_1 = local_res8;
      uStack_228 = piVar12;
      if (piVar11 != (int *)0x0) {
        *piVar11 = (int)piVar21 * 2;
        piVar11 = piVar11 + 1;
        if (lVar20 != 0) {
          FUN_142ef7ba0(piVar11,lVar20,(longlong)piVar21 * 2);
        }
        *(undefined2 *)((longlong)piVar11 + (longlong)piVar21 * 2) = 0;
        param_1 = local_res8;
        uStack_228 = piVar11;
      }
    }
    else {
      if (((short)local_230 == 8) &&
         (local_230 = (uint)local_230._2_2_ << 0x10, uStack_228 != (int *)0x0)) {
        (*DAT_143ad5990)(uStack_228 + -1);
      }
      iVar10 = (*DAT_143262a28)(&local_230,&DAT_143a8b8d8);
      if (iVar10 < 0) {
LAB_142a91a5a:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar10);
      }
    }
    local_res20 = (IUnknown *)0x0;
    local_108 = local_230;
    uStack_104 = uStack_22c;
    uStack_100 = (undefined4)uStack_228;
    uStack_fc = uStack_228._4_4_;
    local_f8 = local_220;
    iVar10 = (**(code **)(*(longlong *)pIVar1 + 0x240))(pIVar1,&local_108);
    if (iVar10 < 0) {
      _com_issue_errorex(iVar10,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
    pIVar3 = local_res20;
    local_1a0 = local_res20;
    if ((short)local_230 == 8) {
      local_230 = local_230 & 0xffff0000;
      if (uStack_228 != (int *)0x0) {
        (*DAT_143ad5990)(uStack_228 + -1);
      }
    }
    else {
      (*DAT_143262a18)(&local_230);
    }
    if (pIVar3 == (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar1 + 0x10))(pIVar1);
    }
    else {
      local_res20 = (IUnknown *)((ulonglong)local_res20 & 0xffffffff00000000);
      iVar10 = (**(code **)(*(longlong *)pIVar3 + 0xa0))(pIVar3,&local_res20);
      if (iVar10 < 0) {
        _com_issue_errorex(iVar10,pIVar3,(_GUID *)&DAT_14327ac98);
      }
      local_270 = local_270 & 0xffffffff00000000;
      iVar10 = (**(code **)(*(longlong *)pIVar3 + 0x98))(pIVar3,&local_270);
      if (iVar10 < 0) {
        _com_issue_errorex(iVar10,pIVar3,(_GUID *)&DAT_14327ac98);
      }
      uVar15 = CONCAT44(uVar30,0xffffff);
      iVar10 = (**(code **)(*(longlong *)pIVar3 + 0x170))
                         (pIVar3,0,0,local_270 & 0xffffffff,CONCAT44(uVar28,(uint)local_res20),
                          uVar15);
      uVar30 = (undefined4)((ulonglong)uVar15 >> 0x20);
      if (iVar10 < 0) {
        _com_issue_errorex(iVar10,pIVar3,(_GUID *)&DAT_14327ac98);
      }
      if (*(int *)(param_1 + 0x410) == 0) {
        FUN_1429fbeb0(&local_260,0x26);
        if (local_260 == (IUnknown *)0x0) goto LAB_142a91327;
        piVar12 = (int *)FUN_14108c8e0(*(undefined4 *)(param_2 + 200));
        pIVar18 = local_260;
        if (*piVar12 != 0) {
          piVar21 = piVar12 + 3;
          local_278 = (IUnknown *)0x0;
          if (piVar21 != (int *)0x0) {
            uVar26 = 0xffffffffffffffff;
            do {
              uVar26 = uVar26 + 1;
            } while (*(char *)((longlong)piVar21 + uVar26) != '\0');
            iVar10 = (int)uVar26;
            uVar15 = FUN_14019bd40(&local_278,uVar26 & 0xffffffff,0);
            FUN_142ef7ba0(uVar15,piVar21,(longlong)iVar10);
            pIVar18 = local_278;
            if (*(int *)(local_278 + -0x10) != -1) {
              FUN_142e52dd0();
            }
            if ((iVar10 == -1) || (iVar10 <= *(int *)(pIVar18 + -0xc))) {
              *(undefined4 *)(pIVar18 + -0x10) = 1;
              if (iVar10 != -1) goto LAB_142a9116d;
              if (pIVar18 == (IUnknown *)0x0) {
                uVar26 = 0;
              }
              else {
                uVar26 = 0xffffffffffffffff;
                do {
                  uVar26 = uVar26 + 1;
                } while (pIVar18[uVar26] != (IUnknown)0x0);
              }
            }
            else {
              FUN_142e54290(0x90,*(int *)(pIVar18 + -0xc),uVar26 & 0xffffffff);
              *(undefined4 *)(pIVar18 + -0x10) = 1;
LAB_142a9116d:
              local_278[iVar10] = (IUnknown)0x0;
            }
            iVar10 = (int)uVar26;
            if ((iVar10 < 0) || (*(int *)(pIVar18 + -0xc) + 1 <= iVar10)) {
              FUN_142e54290(0x9c,uVar26 & 0xffffffff);
            }
            *(int *)(pIVar18 + -8) = iVar10;
          }
          pIVar4 = local_260;
          pIVar18 = local_res8;
          iVar10 = FUN_142bf7d70(local_res8);
          local_250 = pIVar3;
          (**(code **)(*(longlong *)pIVar3 + 8))(pIVar3);
          uVar31 = CONCAT44(uVar32,0xff);
          uVar15 = CONCAT44(uVar30,1);
          FUN_142a0ff80(&local_250,iVar10 / 2,4,&local_278,pIVar4,uVar15,uVar31);
          uVar28 = (undefined4)((ulonglong)uVar15 >> 0x20);
          uVar30 = (undefined4)((ulonglong)uVar31 >> 0x20);
          iVar10 = piVar12[0x43];
          sVar6 = FUN_1401d72c0(piVar12);
          uVar15 = FUN_1402b0250((int)sVar6,(int)(short)iVar10);
          local_res20 = (IUnknown *)CONCAT44(local_res20._4_4_,*(int *)((longlong)piVar12 + 0x2f));
          uVar19 = *(uint *)((longlong)piVar12 + 0x27);
          uVar25 = *(uint *)((longlong)piVar12 + 0x2b);
          uVar23 = uVar19 ^ 0xbaadf00d;
          iVar10 = (uVar23 >> 5 | uVar23 << 0x1b) + uVar25;
          local_268 = (IUnknown *)CONCAT44(local_268._4_4_,iVar10);
          if (iVar10 != *(int *)((longlong)piVar12 + 0x2f)) {
            local_258 = (IUnknown *)FUN_1418039d0(5);
            puVar14 = (undefined8 *)FUN_1401a0ed0(&local_190,&local_258,&local_268,&local_res20);
            FUN_141804970(&DAT_143271f04,0x53,5,*puVar14);
            if (local_190 != (IUnknown *)0x0) {
              FUN_14019f2c0(local_190 + -0x10);
            }
          }
          FUN_14019ba10(&local_278,"Lv.%d %s",(uVar25 << 5 | uVar25 >> 0x1b) ^ uVar19,uVar15);
          pIVar4 = local_260;
          iVar10 = FUN_142bf7d70(pIVar18);
          local_248 = pIVar3;
          (**(code **)(*(longlong *)pIVar3 + 8))(pIVar3);
          uVar31 = CONCAT44(uVar30,0xff);
          uVar15 = CONCAT44(uVar28,1);
          FUN_142a0ff80(&local_248,iVar10 / 2,0x11,&local_278,pIVar4,uVar15,uVar31);
          uVar32 = (undefined4)((ulonglong)uVar31 >> 0x20);
          uVar30 = (undefined4)((ulonglong)uVar15 >> 0x20);
          pIVar18 = local_260;
          if (local_278 != (IUnknown *)0x0) {
            FUN_14019f2c0(local_278 + -0x10);
            pIVar18 = local_260;
          }
        }
LAB_142a9130c:
        param_1 = local_res8;
        if (pIVar18 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)pIVar18 + 0x10))();
          param_1 = local_res8;
        }
      }
      else if ((*(int *)(param_1 + 0x410) == 1) &&
              (FUN_1429fbeb0(&local_278,0x26), local_278 != (IUnknown *)0x0)) {
        uVar23 = *(uint *)(param_2 + 200);
        if (-1 < (int)uVar23) {
          lVar20 = *(longlong *)(param_2 + 0xd0);
          uVar9 = 0;
          uVar7 = uVar9;
          if (lVar20 != 0) {
            uVar7 = *(uint *)(lVar20 + -8);
          }
          if ((int)uVar23 < (int)uVar7) {
            if (lVar20 != 0) {
              uVar9 = *(uint *)(lVar20 + -8);
            }
            if (uVar9 <= uVar23) {
              FUN_142e54290(0xbc,uVar23);
              lVar20 = *(longlong *)(param_2 + 0xd0);
            }
            pIVar18 = local_278;
            if (*(int *)(param_2 + 0x70) == 0) goto LAB_142a9130c;
            if (*(int *)(param_2 + 0x70) == 100) {
              FUN_1402edb30(&local_258);
              (*DAT_143262a20)(&local_1d0);
              if (DAT_143a8b8d8 == 8) {
                if (local_1d0 == 8) {
                  local_1d0 = 0;
                  if (lStack_1c8 != 0) {
                    (*DAT_143ad5990)(lStack_1c8 + -4);
                  }
                }
                else {
                  iVar10 = (*DAT_143262a18)(&local_1d0);
                  if (iVar10 < 0) goto LAB_142a91a62;
                }
                local_1d0 = 8;
                piVar21 = piVar12;
                if (DAT_143a8b8e0 != 0) {
                  piVar21 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
                }
                lStack_1c8 = FUN_1401a5fa0(DAT_143a8b8e0,piVar21);
              }
              else {
                if ((local_1d0 == 8) && (local_1d0 = 0, lStack_1c8 != 0)) {
                  (*DAT_143ad5990)(lStack_1c8 + -4);
                }
                iVar10 = (*DAT_143262a28)(&local_1d0,&DAT_143a8b8d8);
                if (iVar10 < 0) {
LAB_142a91a62:
                    /* WARNING: Subroutine does not return */
                  FUN_142ef3ac0(iVar10);
                }
              }
              (*DAT_143262a20)(&local_1e8);
              if (DAT_143a8b8d8 == 8) {
                if (local_1e8 == 8) {
                  local_1e8 = 0;
                  if (lStack_1e0 != 0) {
                    (*DAT_143ad5990)(lStack_1e0 + -4);
                  }
                }
                else {
                  iVar10 = (*DAT_143262a18)(&local_1e8);
                  if (iVar10 < 0) goto LAB_142a91a6a;
                }
                local_1e8 = 8;
                piVar21 = piVar12;
                if (DAT_143a8b8e0 != 0) {
                  piVar21 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
                }
                lStack_1e0 = FUN_1401a5fa0(DAT_143a8b8e0,piVar21);
              }
              else {
                if ((local_1e8 == 8) && (local_1e8 = 0, lStack_1e0 != 0)) {
                  (*DAT_143ad5990)(lStack_1e0 + -4);
                }
                iVar10 = (*DAT_143262a28)(&local_1e8,&DAT_143a8b8d8);
                if (iVar10 < 0) {
LAB_142a91a6a:
                    /* WARNING: Subroutine does not return */
                  FUN_142ef3ac0(iVar10);
                }
              }
              (*DAT_143262a20)(&local_200);
              if (DAT_143a8b8d8 == 8) {
                if ((short)local_200 == 8) {
                  local_200 = (uint)local_200._2_2_ << 0x10;
                  if (uStack_1f8 != 0) {
                    (*DAT_143ad5990)(uStack_1f8 + -4);
                  }
                }
                else {
                  iVar10 = (*DAT_143262a18)(&local_200);
                  if (iVar10 < 0) goto LAB_142a91a72;
                }
                local_200 = CONCAT22(local_200._2_2_,8);
                if (DAT_143a8b8e0 != 0) {
                  piVar12 = (int *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
                }
                uStack_1f8 = FUN_1401a5fa0(DAT_143a8b8e0,piVar12);
              }
              else {
                if (((short)local_200 == 8) &&
                   (local_200 = (uint)local_200._2_2_ << 0x10, uStack_1f8 != 0)) {
                  (*DAT_143ad5990)(uStack_1f8 + -4);
                }
                iVar10 = (*DAT_143262a28)(&local_200,&DAT_143a8b8d8);
                if (iVar10 < 0) {
LAB_142a91a72:
                    /* WARNING: Subroutine does not return */
                  FUN_142ef3ac0(iVar10);
                }
              }
              pIVar18 = local_278;
              local_190 = (IUnknown *)&local_250;
              plVar13 = (longlong *)FUN_1401a5780(&local_250,local_258);
              pIVar4 = local_278;
              local_168 = plVar13;
              if (local_278 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x80004003);
              }
              (*DAT_143262a20)();
              if (DAT_143a8b8d8 == 8) {
                if ((short)local_218 == 8) {
                  local_218 = (uint)local_218._2_2_ << 0x10;
                  if (uStack_210 != 0) {
                    (*DAT_143ad5990)(uStack_210 + -4);
                  }
                }
                else {
                  iVar10 = (*DAT_143262a18)(&local_218);
                  if (iVar10 < 0) goto LAB_142a91a7a;
                }
                local_218 = CONCAT22(local_218._2_2_,8);
                if (DAT_143a8b8e0 == 0) {
                  uVar19 = 0;
                }
                else {
                  uVar19 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
                }
                uStack_210 = FUN_1401a5fa0(DAT_143a8b8e0,uVar19);
              }
              else {
                if (((short)local_218 == 8) &&
                   (local_218 = (uint)local_218._2_2_ << 0x10, uStack_210 != 0)) {
                  (*DAT_143ad5990)(uStack_210 + -4);
                }
                iVar10 = (*DAT_143262a28)(&local_218,&DAT_143a8b8d8);
                if (iVar10 < 0) {
LAB_142a91a7a:
                    /* WARNING: Subroutine does not return */
                  FUN_142ef3ac0(iVar10);
                }
              }
              local_260 = (IUnknown *)FUN_1401a5780(&local_248,local_258);
              uVar15 = 0;
              local_res20 = (IUnknown *)((ulonglong)local_res20 & 0xffffffff00000000);
              if (*(undefined8 **)local_260 != (undefined8 *)0x0) {
                uVar15 = **(undefined8 **)local_260;
              }
              local_e8 = local_218;
              uStack_e4 = uStack_214;
              uStack_e0 = (undefined4)uStack_210;
              uStack_dc = uStack_210._4_4_;
              local_d8 = local_208;
              iVar10 = (**(code **)(*(longlong *)pIVar4 + 0xb0))
                                 (pIVar4,uVar15,&local_e8,&local_res20);
              if (iVar10 < 0) {
                _com_issue_errorex(iVar10,pIVar4,(_GUID *)&DAT_143297250);
              }
              uVar19 = (uint)local_res20;
              thunk_FUN_1401be120();
              uVar15 = 0;
              local_res20 = (IUnknown *)((ulonglong)local_res20 & 0xffffffff00000000);
              if ((undefined8 *)*plVar13 != (undefined8 *)0x0) {
                uVar15 = *(undefined8 *)*plVar13;
              }
              local_c8 = CONCAT62(uStack_1ce,local_1d0);
              lStack_c0 = lStack_1c8;
              local_b8 = local_1c0;
              local_a8 = CONCAT62(uStack_1e6,local_1e8);
              lStack_a0 = lStack_1e0;
              local_98 = local_1d8;
              local_88 = local_200;
              uStack_84 = uStack_1fc;
              uStack_80 = (undefined4)uStack_1f8;
              uStack_7c = uStack_1f8._4_4_;
              local_78 = local_1f0;
              puVar14 = &local_a8;
              puVar29 = &local_88;
              iVar10 = (**(code **)(*(longlong *)pIVar3 + 0x1a8))
                                 (pIVar3,0x56 - (uVar19 >> 1),4,uVar15,pIVar18,puVar29,puVar14,
                                  &local_c8,&local_res20);
              uVar32 = (undefined4)((ulonglong)puVar14 >> 0x20);
              uVar30 = (undefined4)((ulonglong)puVar29 >> 0x20);
              if (iVar10 < 0) {
                _com_issue_errorex(iVar10,pIVar3,(_GUID *)&DAT_14327ac98);
              }
              thunk_FUN_1401be120(plVar13);
              if ((short)local_218 == 8) {
                local_218 = local_218 & 0xffff0000;
                if (uStack_210 != 0) {
                  (*DAT_143ad5990)(uStack_210 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_218);
              }
              if ((short)local_200 == 8) {
                local_200 = local_200 & 0xffff0000;
                if (uStack_1f8 != 0) {
                  (*DAT_143ad5990)(uStack_1f8 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_200);
              }
              if (local_1e8 == 8) {
                local_1e8 = 0;
                if (lStack_1e0 != 0) {
                  (*DAT_143ad5990)(lStack_1e0 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_1e8);
              }
              if (local_1d0 == 8) {
                local_1d0 = 0;
                if (lStack_1c8 != 0) {
                  (*DAT_143ad5990)(lStack_1c8 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_1d0);
              }
            }
            else {
              FUN_140393990(DAT_143aa8328,&local_258,
                            *(undefined4 *)(lVar20 + (longlong)(int)uVar23 * 4),1);
              local_260 = (IUnknown *)0x0;
              FUN_14019a260(&local_260,&local_258);
              local_res20 = (IUnknown *)((ulonglong)local_res20 & 0xffffffff00000000);
              iVar10 = (**(code **)(*(longlong *)pIVar3 + 0x98))(pIVar3,&local_res20);
              if (iVar10 < 0) {
                _com_issue_errorex(iVar10,pIVar3,(_GUID *)&DAT_14327ac98);
              }
              iVar10 = (uint)local_res20;
              local_248 = local_278;
              if (local_278 != (IUnknown *)0x0) {
                (**(code **)(*(longlong *)local_278 + 8))();
              }
              FUN_1429eb300(&local_260,&local_248,iVar10 + -8);
              pIVar18 = local_278;
              local_res20 = (IUnknown *)((ulonglong)local_res20 & 0xffffffff00000000);
              iVar10 = (**(code **)(*(longlong *)pIVar3 + 0x98))(pIVar3,&local_res20);
              if (iVar10 < 0) {
                _com_issue_errorex(iVar10,pIVar3,(_GUID *)&DAT_14327ac98);
              }
              uVar23 = (uint)local_res20 >> 1;
              local_250 = pIVar3;
              (**(code **)(*(longlong *)pIVar3 + 8))(pIVar3);
              uVar31 = CONCAT44(uVar32,0xff);
              uVar15 = CONCAT44(uVar30,1);
              FUN_142a0ff80(&local_250,uVar23,2,&local_260,pIVar18,uVar15,uVar31);
              uVar32 = (undefined4)((ulonglong)uVar31 >> 0x20);
              uVar30 = (undefined4)((ulonglong)uVar15 >> 0x20);
              if (local_260 == local_258) {
LAB_142a90ae8:
                piVar12 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
                piVar12[1] = 0;
                *piVar12 = -1;
                pIVar18 = (IUnknown *)(piVar12 + 4);
                piVar12[2] = 0;
                *pIVar18 = (IUnknown)0x0;
                local_res20 = pIVar18;
                if (*piVar12 != -1) {
                  FUN_142e52dd0(0x8b);
                }
                if (piVar12[1] < 0) {
                  FUN_142e54290(0x90,piVar12[1],0);
                }
                *piVar12 = 1;
                *pIVar18 = (IUnknown)0x0;
                if (piVar12[1] + 1 < 1) {
                  FUN_142e54290(0x9c,0);
                }
                piVar12[2] = 0;
                if (*(longlong *)(param_2 + 0x40) != 0) {
                  FUN_14019f2c0(*(longlong *)(param_2 + 0x40) + -0x10);
                }
                *(IUnknown **)(param_2 + 0x40) = pIVar18;
              }
              else {
                uVar23 = uVar25;
                if (local_260 != (IUnknown *)0x0) {
                  uVar23 = *(uint *)(local_260 + -8);
                }
                if (local_258 != (IUnknown *)0x0) {
                  uVar19 = *(uint *)(local_258 + -8);
                }
                if (uVar23 == uVar19) {
                  if (uVar23 != 0) {
                    if (local_260 != (IUnknown *)0x0) {
                      uVar25 = *(uint *)(local_260 + -8);
                    }
                    iVar10 = memcmp(local_260,local_258,(longlong)(int)uVar25);
                    if (iVar10 != 0) goto LAB_142a90a10;
                  }
                  goto LAB_142a90ae8;
                }
LAB_142a90a10:
                FUN_14019a260(param_2 + 0x40,&local_258);
                local_res20 = (IUnknown *)((ulonglong)local_res20 & 0xffffffff00000000);
                iVar10 = (**(code **)(*(longlong *)pIVar1 + 0xd0))(pIVar1,&local_res20);
                if (iVar10 < 0) {
                  _com_issue_errorex(iVar10,pIVar1,(_GUID *)&DAT_143273488);
                }
                *(uint *)(param_2 + 0x30) = (uint)local_res20 + 4;
                local_res20 = (IUnknown *)((ulonglong)local_res20 & 0xffffffff00000000);
                iVar10 = (**(code **)(*(longlong *)pIVar1 + 0xe0))(pIVar1,&local_res20);
                if (iVar10 < 0) {
                  _com_issue_errorex(iVar10,pIVar1,(_GUID *)&DAT_143273488);
                }
                *(uint *)(param_2 + 0x34) = (uint)local_res20 + 4;
                local_res20 = (IUnknown *)((ulonglong)local_res20 & 0xffffffff00000000);
                iVar10 = (**(code **)(*(longlong *)pIVar1 + 0x1a0))(pIVar1,&local_res20);
                if (iVar10 < 0) {
                  _com_issue_errorex(iVar10,pIVar1,(_GUID *)&DAT_14327fcb0);
                }
                *(uint *)(param_2 + 0x38) = *(int *)(param_2 + 0x30) + (uint)local_res20 + -8;
                *(int *)(param_2 + 0x3c) = *(int *)(param_2 + 0x34) + 0xe;
              }
              if (local_260 != (IUnknown *)0x0) {
                FUN_14019f2c0(local_260 + -0x10);
              }
            }
            pIVar18 = local_278;
            if (local_258 != (IUnknown *)0x0) {
              FUN_14019f2c0(local_258 + -0x10);
              pIVar18 = local_278;
            }
            goto LAB_142a9130c;
          }
        }
        (**(code **)(*(longlong *)local_278 + 0x10))();
      }
LAB_142a91327:
      (**(code **)(*(longlong *)pIVar3 + 0x10))(pIVar3);
      (**(code **)(*(longlong *)pIVar1 + 0x10))(pIVar1);
    }
  }
  pIVar1 = *(IUnknown **)(param_2 + 0x48);
  local_190 = pIVar1;
  if (pIVar1 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)pIVar1 + 8))(pIVar1);
    (*DAT_143262a20)(&local_1b8);
    if (DAT_143a8b8d8 == 8) {
      if ((short)local_1b8 == 8) {
        local_1b8 = (uint)local_1b8._2_2_ << 0x10;
        if (uStack_1b0 != 0) {
          (*DAT_143ad5990)(uStack_1b0 + -4);
        }
      }
      else {
        iVar10 = (*DAT_143262a18)(&local_1b8);
        if (iVar10 < 0) goto LAB_142a91a82;
      }
      local_1b8 = CONCAT22(local_1b8._2_2_,8);
      uVar19 = 0;
      if (DAT_143a8b8e0 != 0) {
        uVar19 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      uStack_1b0 = FUN_1401a5fa0(DAT_143a8b8e0,uVar19);
    }
    else {
      if (((short)local_1b8 == 8) && (local_1b8 = (uint)local_1b8._2_2_ << 0x10, uStack_1b0 != 0)) {
        (*DAT_143ad5990)(uStack_1b0 + -4);
      }
      iVar10 = (*DAT_143262a28)(&local_1b8,&DAT_143a8b8d8);
      if (iVar10 < 0) {
LAB_142a91a82:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar10);
      }
    }
    local_res8 = (IUnknown *)0x0;
    local_68 = local_1b8;
    uStack_64 = uStack_1b4;
    uStack_60 = (undefined4)uStack_1b0;
    uStack_5c = uStack_1b0._4_4_;
    local_58 = local_1a8;
    iVar10 = (**(code **)(*(longlong *)pIVar1 + 0x240))(pIVar1,&local_68,&local_res8);
    if (iVar10 < 0) {
      _com_issue_errorex(iVar10,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
    pIVar3 = local_res8;
    local_198 = local_res8;
    if ((short)local_1b8 == 8) {
      local_1b8 = local_1b8 & 0xffff0000;
      if (uStack_1b0 != 0) {
        (*DAT_143ad5990)(uStack_1b0 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_1b8);
    }
    if (pIVar3 == (IUnknown *)0x0) {
      (**(code **)(*(longlong *)pIVar1 + 0x10))(pIVar1);
    }
    else {
      local_res8 = (IUnknown *)((ulonglong)local_res8 & 0xffffffff00000000);
      iVar10 = (**(code **)(*(longlong *)pIVar3 + 0xa0))(pIVar3,&local_res8);
      if (iVar10 < 0) {
        _com_issue_errorex(iVar10,pIVar3,(_GUID *)&DAT_14327ac98);
      }
      local_res20 = (IUnknown *)((ulonglong)local_res20 & 0xffffffff00000000);
      iVar10 = (**(code **)(*(longlong *)pIVar3 + 0x98))(pIVar3,&local_res20);
      if (iVar10 < 0) {
        _com_issue_errorex(iVar10,pIVar3,(_GUID *)&DAT_14327ac98);
      }
      uVar15 = CONCAT44(uVar30,0xffffff);
      iVar10 = (**(code **)(*(longlong *)pIVar3 + 0x170))
                         (pIVar3,0,0,(ulonglong)local_res20 & 0xffffffff,
                          (ulonglong)local_res8 & 0xffffffff,uVar15);
      uVar28 = (undefined4)((ulonglong)uVar15 >> 0x20);
      if (iVar10 < 0) {
        _com_issue_errorex(iVar10,pIVar3,(_GUID *)&DAT_14327ac98);
      }
      iVar10 = *(int *)(param_1 + 0x2a8);
      if (((((iVar10 == 0x17) || (iVar10 == 0x19)) || (iVar10 == 0x1c)) || (iVar10 == 0x1d)) &&
         (FUN_1429fbeb0(&local_238,0xaa), local_238 != (longlong *)0x0)) {
        lVar20 = *(longlong *)(param_2 + 0x80);
        if (lVar20 == 0) {
          (**(code **)(*local_238 + 0x10))();
        }
        else {
          iVar10 = FUN_14041a270(lVar20);
          if (iVar10 != 0) {
            local_res8 = (IUnknown *)((ulonglong)local_res8 & 0xffffffff00000000);
            local_res20 = (IUnknown *)((ulonglong)local_res20 & 0xffffffff00000000);
            local_268 = (IUnknown *)((ulonglong)local_268 & 0xffffffff00000000);
            local_270 = local_270 & 0xffffffff00000000;
            if ((*(int *)(param_1 + 0x2a8) == 0x17) ||
               (pwVar22 = L"UtilDlgEx_MixLens", *(int *)(param_1 + 0x2a8) == 0x1c)) {
              pwVar22 = L"UtilDlgEx_MixHair";
            }
            FUN_1403edf80(&local_148,pwVar22,0xffffffff);
            local_278 = (IUnknown *)0x0;
            pwVar22 = (wchar_t *)FUN_1401bd080(&local_278,0x11);
            pIVar18 = local_278;
            uVar2 = u_UI_UtilDlgEx_img__14348acc0._12_4_;
            uVar8 = u_UI_UtilDlgEx_img__14348acc0._8_4_;
            uVar30 = u_UI_UtilDlgEx_img__14348acc0._4_4_;
            *(undefined4 *)pwVar22 = u_UI_UtilDlgEx_img__14348acc0._0_4_;
            *(undefined4 *)(pwVar22 + 2) = uVar30;
            *(undefined4 *)(pwVar22 + 4) = uVar8;
            *(undefined4 *)(pwVar22 + 6) = uVar2;
            uVar2 = u_UI_UtilDlgEx_img__14348acc0._28_4_;
            uVar8 = u_UI_UtilDlgEx_img__14348acc0._24_4_;
            uVar30 = u_UI_UtilDlgEx_img__14348acc0._20_4_;
            *(undefined4 *)(pwVar22 + 8) = u_UI_UtilDlgEx_img__14348acc0._16_4_;
            *(undefined4 *)(pwVar22 + 10) = uVar30;
            *(undefined4 *)(pwVar22 + 0xc) = uVar8;
            *(undefined4 *)(pwVar22 + 0xe) = uVar2;
            pwVar22[0x10] = u_UI_UtilDlgEx_img__14348acc0[0x10];
            if (*(int *)(local_278 + -0x10) != -1) {
              FUN_142e52dd0(0x8b);
            }
            if (*(int *)(pIVar18 + -0xc) < 0x11) {
              FUN_142e54290(0x90,*(int *)(pIVar18 + -0xc),0x11);
            }
            *(undefined4 *)(pIVar18 + -0x10) = 1;
            *(short *)(pIVar18 + 0x22) = 0;
            if (*(int *)(pIVar18 + -0xc) + 1 < 0x12) {
              FUN_142e54290(0x9c,0x11);
            }
            *(undefined4 *)(pIVar18 + -8) = 0x22;
            uVar19 = 0;
            if (local_148 != 0) {
              uVar26 = (ulonglong)(longlong)*(int *)(local_148 + -8) >> 1;
              iVar10 = (int)uVar26;
              if (iVar10 != 0) {
                if (*(short *)pIVar18 == 0) {
                  uVar15 = FUN_1401bd080(&local_278,uVar26 & 0xffffffff,0);
                  FUN_142ef7ba0(uVar15,local_148,(longlong)iVar10 * 2);
                  FUN_1401bd8a0(&local_278,uVar26 & 0xffffffff);
                  pIVar18 = local_278;
                }
                else {
                  iVar27 = (int)((ulonglong)(longlong)*(int *)(pIVar18 + -8) >> 1) + iVar10;
                  for (iVar17 = *(int *)(pIVar18 + -0xc); iVar17 < iVar27; iVar17 = iVar17 * 2) {
                  }
                  lVar16 = FUN_1401bd080(&local_278,iVar17,1);
                  pIVar18 = local_278;
                  if (local_278 == (IUnknown *)0x0) {
                    iVar17 = 0;
                  }
                  else {
                    iVar17 = (int)((ulonglong)(longlong)*(int *)(local_278 + -8) >> 1);
                  }
                  FUN_142ef7ba0(lVar16 + (longlong)iVar17 * 2,local_148,(longlong)iVar10 * 2);
                  FUN_1401bd8a0(&local_278,iVar27);
                }
              }
              uVar19 = (uint)local_res8;
            }
            FUN_14090ead0(&local_180,pIVar18);
            if (local_180 != (longlong *)0x0) {
              local_res8 = (IUnknown *)local_180;
              (**(code **)(*local_180 + 8))();
              FUN_14090fe90(&local_160,&local_res8,L"MixPercentBase");
              local_res20 = local_160;
              if (local_160 != (IUnknown *)0x0) {
                (**(code **)(*(longlong *)local_160 + 8))();
              }
              local_250 = (IUnknown *)FUN_142aa2fa0(&local_res20,0);
              if (local_160 != (IUnknown *)0x0) {
                (**(code **)(*(longlong *)local_160 + 0x10))();
              }
              local_res8 = (IUnknown *)local_180;
              if (local_180 != (longlong *)0x0) {
                (**(code **)(*local_180 + 8))();
              }
              FUN_14090fe90(&local_240,&local_res8,L"MixPercentAdd");
              local_res20 = local_240;
              if (local_240 != (IUnknown *)0x0) {
                (**(code **)(*(longlong *)local_240 + 8))();
              }
              local_res8 = (IUnknown *)FUN_142aa2fa0(&local_res20,0);
              if (local_240 != (IUnknown *)0x0) {
                (**(code **)(*(longlong *)local_240 + 0x10))();
              }
              local_res20 = (IUnknown *)CONCAT44(local_res20._4_4_,local_250._4_4_);
              local_270 = CONCAT44(local_270._4_4_,local_res8._4_4_);
              local_268 = (IUnknown *)CONCAT44(local_268._4_4_,(uint)local_res8);
              uVar19 = (uint)local_250;
            }
            if (local_180 != (longlong *)0x0) {
              (**(code **)(*local_180 + 0x10))();
            }
            local_res8 = (IUnknown *)0x0;
            uVar15 = FUN_14019ba10(&local_res8,&DAT_14327a9a8,100 - (uint)*(byte *)(lVar20 + 2));
            local_170 = 0;
            FUN_14019a260(&local_170,uVar15);
            if (local_res8 != (IUnknown *)0x0) {
              FUN_14019f2c0((longlong)local_res8 + -0x10);
            }
            local_res8 = (IUnknown *)0x0;
            uVar15 = FUN_14019ba10(&local_res8,&DAT_14327a9a8,*(undefined1 *)(lVar20 + 2));
            local_178 = 0;
            FUN_14019a260(&local_178,uVar15);
            if (local_res8 != (IUnknown *)0x0) {
              FUN_14019f2c0(local_res8 + -0x10);
            }
            plVar13 = local_238;
            local_240 = pIVar3;
            (**(code **)(*(longlong *)pIVar3 + 8))(pIVar3);
            uVar31 = CONCAT44(uVar32,0xff);
            uVar15 = CONCAT44(uVar28,1);
            FUN_142a0ff80(&local_240,uVar19,(ulonglong)local_res20 & 0xffffffff,&local_170,plVar13,
                          uVar15,uVar31);
            plVar13 = local_238;
            uVar28 = (undefined4)((ulonglong)uVar15 >> 0x20);
            uVar30 = (undefined4)((ulonglong)uVar31 >> 0x20);
            local_res20 = pIVar3;
            (**(code **)(*(longlong *)pIVar3 + 8))(pIVar3);
            FUN_142a0ff80(&local_res20,(ulonglong)local_268 & 0xffffffff,local_270 & 0xffffffff,
                          &local_178,plVar13,CONCAT44(uVar28,1),CONCAT44(uVar30,0xff));
            if (local_178 != 0) {
              FUN_14019f2c0(local_178 + -0x10);
            }
            if (local_170 != 0) {
              FUN_14019f2c0(local_170 + -0x10);
            }
            if (pIVar18 != (IUnknown *)0x0) {
              FUN_1401bebb0(pIVar18 + -0x10);
            }
            if (local_148 != 0) {
              FUN_1401bebb0(local_148 + -0x10);
            }
          }
          if (local_238 != (longlong *)0x0) {
            (**(code **)(*local_238 + 0x10))();
          }
        }
      }
      (**(code **)(*(longlong *)pIVar3 + 0x10))(pIVar3);
      (**(code **)(*(longlong *)pIVar1 + 0x10))(pIVar1);
    }
  }
  FUN_1402ca630(local_120);
  FUN_1402ca630(local_158);
  return;
}



//===========================================================
// FUN_142a93710 @ 142a93710   (340 bytes)
//===========================================================

void FUN_142a93710(undefined8 *param_1,longlong *param_2,int param_3,int param_4)

{
  longlong *plVar1;
  longlong lVar2;
  uint uVar3;
  undefined8 *puVar4;
  longlong lVar5;
  int iVar6;
  longlong lVar7;
  bool bVar8;
  
  iVar6 = 0;
  lVar7 = *param_2;
  if (param_2[1] - lVar7 >> 4 != 0) {
    lVar5 = 0;
    do {
      lVar7 = *(longlong *)(lVar7 + 8 + lVar5);
      if (lVar7 != 0) {
        if (0xfffff < *(ulonglong *)(lVar7 + 0x20)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar7 + 0x20) = *(longlong *)(lVar7 + 0x20) + 1;
        UNLOCK();
      }
      if (lVar7 != 0) {
        bVar8 = iVar6 == param_3;
        uVar3 = FUN_1416899e0(lVar7);
        if (bVar8 != uVar3) {
          FUN_1416899b0(lVar7,bVar8);
        }
        if (*(char *)*param_1 == '\0') {
          bVar8 = iVar6 != param_4;
        }
        (**(code **)(*(longlong *)(lVar7 + 8) + 0x70))((longlong *)(lVar7 + 8),bVar8);
        if (0xffffe < *(longlong *)(lVar7 + 0x20) - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar1 = (longlong *)(lVar7 + 0x20);
        lVar2 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if (((int)lVar2 == 1) &&
           (puVar4 = (undefined8 *)(lVar7 + 0x18), puVar4 != (undefined8 *)0x0)) {
          (**(code **)*puVar4)(puVar4,1);
        }
      }
      iVar6 = iVar6 + 1;
      lVar5 = lVar5 + 0x10;
      lVar7 = *param_2;
    } while ((ulonglong)(longlong)iVar6 < (ulonglong)(param_2[1] - lVar7 >> 4));
  }
  return;
}


