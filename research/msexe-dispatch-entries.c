
//===========================================================
// FUN_14177b7e0 @ 14177b7e0   (4118 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14177b7e0(undefined8 param_1,undefined4 param_2,undefined8 param_3)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  IUnknown *pIVar7;
  IUnknown *pIVar8;
  int iVar9;
  uint uVar10;
  uint uVar11;
  int iVar12;
  undefined4 uVar13;
  int iVar14;
  longlong lVar15;
  undefined8 uVar16;
  longlong lVar17;
  int *piVar18;
  int iVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  int iVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  double dVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  undefined1 auStack_2b8 [32];
  undefined8 *local_298;
  undefined8 local_290;
  undefined1 *local_288;
  undefined1 *local_280;
  undefined8 *local_278;
  undefined4 local_270;
  undefined4 local_268;
  undefined8 local_260;
  undefined4 local_258;
  undefined4 local_250;
  undefined8 *local_248;
  undefined8 local_238;
  undefined8 local_230;
  undefined8 local_228;
  IUnknown *local_220;
  undefined8 local_218;
  undefined8 local_210;
  IUnknown *local_208;
  short local_200 [4];
  longlong lStack_1f8;
  undefined8 local_1f0;
  short local_1e8 [4];
  longlong lStack_1e0;
  undefined8 local_1d8;
  short local_1d0 [4];
  longlong lStack_1c8;
  undefined8 local_1c0;
  short local_1b8 [4];
  longlong lStack_1b0;
  undefined8 local_1a8;
  undefined8 *local_1a0;
  undefined1 local_198 [8];
  longlong lStack_190;
  undefined8 local_188;
  undefined1 local_178 [8];
  longlong lStack_170;
  undefined8 local_168;
  undefined1 local_158 [8];
  longlong lStack_150;
  undefined8 local_148;
  undefined1 local_138 [8];
  longlong lStack_130;
  undefined8 local_128;
  ulonglong local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 local_f8;
  int iStack_f0;
  undefined4 uStack_ec;
  undefined8 local_e8;
  undefined1 local_e0 [16];
  ulonglong local_d0;
  
  local_d0 = DAT_143a8b908 ^ (ulonglong)auStack_2b8;
  switch(param_2) {
  case 0x5d5:
    if (DAT_143ace9d0 != '\0') {
      FUN_1406e9050(param_3,&local_230);
      FUN_1406e9170(param_3,&local_f8,0x10);
      uVar16 = FUN_1429fbeb0(&local_220,0xc1);
      local_290 = (undefined1 *)((ulonglong)local_290._4_4_ << 0x20);
      local_298 = (undefined8 *)CONCAT44(local_298._4_4_,0xa0ff00ff);
      FUN_140e722e0(&local_f8,&local_230,0,uVar16);
      if (local_230 != (IUnknown *)0x0) {
        FUN_14019f2c0(local_230 + -0x10);
      }
    }
    break;
  case 0x5d6:
    if ((DAT_143ace9d0 != '\0') && (uVar11 = FUN_1406e8c20(param_3), 0 < (int)uVar11)) {
      uVar21 = (ulonglong)uVar11;
      do {
        FUN_1406e9170(param_3,&local_f8,0x10);
        FUN_1406e9050(param_3,&local_230);
        uVar16 = FUN_1429fbeb0(&local_220,0xc1);
        local_290 = (undefined1 *)((ulonglong)local_290 & 0xffffffff00000000);
        local_298 = (undefined8 *)CONCAT44(local_298._4_4_,0xa0ff00ff);
        FUN_140e722e0(&local_f8,&local_230,0,uVar16);
        if (local_230 != (IUnknown *)0x0) {
          FUN_14019f2c0(local_230 + -0x10);
        }
        uVar21 = uVar21 - 1;
      } while (uVar21 != 0);
    }
    break;
  case 0x5d7:
    if ((DAT_143ace9d0 != '\0') &&
       (uVar10 = FUN_1406e8c20(param_3), uVar11 = DAT_1434b9710, fVar6 = DAT_1434b9680,
       fVar5 = DAT_1434b9614, fVar4 = DAT_1434b95fc, fVar3 = DAT_14329cf10, fVar2 = DAT_14329cf0c,
       fVar1 = DAT_14329cf08, 0 < (int)uVar10)) {
      uVar21 = (ulonglong)uVar10;
      do {
        FUN_1406e9170(param_3,&local_210,8);
        FUN_1406e9170(param_3,&local_218,8);
        iVar12 = FUN_1406e8c20(param_3);
        local_290 = (undefined1 *)CONCAT44(local_290._4_4_,10);
        local_298 = (undefined8 *)CONCAT44(local_298._4_4_,2000);
        FUN_140e75870(local_210,local_218,0xa0ff0000,0);
        if (0 < iVar12) {
          iVar22 = (int)local_218;
          iVar19 = (int)local_210;
          fVar23 = (float)(iVar22 - iVar19);
          iVar14 = local_218._4_4_;
          iVar9 = local_210._4_4_;
          fVar23 = (float)(local_218._4_4_ - local_210._4_4_) *
                   (float)(local_218._4_4_ - local_210._4_4_) + fVar23 * fVar23;
          if (fVar23 != 0.0) {
            local_238 = (IUnknown *)
                        CONCAT44(local_238._4_4_,
                                 ((uint)fVar23 >> 1 & 0x3f800000) +
                                 *(int *)(&DAT_143a418c0 +
                                         (ulonglong)(byte)((uint)fVar23 >> 0x10) * 4));
          }
          dVar26 = (double)FUN_142f25b68();
          dVar26 = (dVar26 * DAT_143289c28) / _DAT_143ace9f0;
          fVar28 = ((float)(dVar26 + DAT_143376c88) * fVar4) / DAT_1434b9660;
          fVar29 = fVar28 + DAT_1434b95b8;
          fVar23 = fVar29;
          if (fVar4 < fVar29) {
            fVar23 = fVar29 + fVar6;
          }
          fVar24 = fVar23 * fVar2 * fVar23;
          if (0.0 <= fVar23) {
            fVar24 = fVar23 * fVar3 - fVar24;
          }
          else {
            fVar24 = fVar24 + fVar23 * fVar3;
          }
          fVar23 = fVar24;
          if (fVar24 < 0.0) {
            fVar23 = (float)((uint)fVar24 ^ uVar11);
          }
          fVar30 = (float)iVar12;
          if (DAT_1434b967c <= fVar28) {
            fVar27 = fVar28;
            if (fVar4 < fVar28) {
              fVar27 = fVar28 + fVar6;
            }
          }
          else {
            fVar27 = fVar28 + fVar5;
          }
          fVar25 = fVar27 * fVar2 * fVar27;
          if (0.0 <= fVar27) {
            fVar25 = fVar27 * fVar3 - fVar25;
          }
          else {
            fVar25 = fVar25 + fVar27 * fVar3;
          }
          fVar27 = fVar25;
          if (fVar25 < 0.0) {
            fVar27 = (float)((uint)fVar25 ^ uVar11);
          }
          local_230 = (IUnknown *)
                      CONCAT44((int)(((fVar27 * fVar25 - fVar25) * fVar1 + fVar25) * fVar30 +
                                    (float)iVar9),
                               (int)(((fVar23 * fVar24 - fVar24) * fVar1 + fVar24) * fVar30 +
                                    (float)iVar19));
          if (fVar4 < fVar29) {
            fVar29 = fVar29 + fVar6;
          }
          fVar23 = fVar29 * fVar2 * fVar29;
          if (0.0 <= fVar29) {
            fVar23 = fVar29 * fVar3 - fVar23;
          }
          else {
            fVar23 = fVar23 + fVar29 * fVar3;
          }
          fVar29 = fVar23;
          if (fVar23 < 0.0) {
            fVar29 = (float)((uint)fVar23 ^ uVar11);
          }
          if (DAT_1434b967c <= fVar28) {
            if (fVar4 < fVar28) {
              fVar28 = fVar28 + fVar6;
            }
          }
          else {
            fVar28 = fVar28 + fVar5;
          }
          fVar24 = fVar28 * fVar2 * fVar28;
          if (0.0 <= fVar28) {
            fVar24 = fVar28 * fVar3 - fVar24;
          }
          else {
            fVar24 = fVar24 + fVar28 * fVar3;
          }
          fVar28 = fVar24;
          if (fVar24 < 0.0) {
            fVar28 = (float)((uint)fVar24 ^ uVar11);
          }
          local_228 = (IUnknown *)
                      CONCAT44((int)(((fVar28 * fVar24 - fVar24) * fVar1 + fVar24) * fVar30 +
                                    (float)iVar14),
                               (int)(((fVar29 * fVar23 - fVar23) * fVar1 + fVar23) * fVar30 +
                                    (float)iVar22));
          local_290._0_4_ = 10;
          local_298._0_4_ = 2000;
          FUN_140e75870(local_230,local_228,0xa000ff00,0);
          fVar28 = ((float)(dVar26 - DAT_143376c88) * fVar4) / DAT_1434b9660;
          fVar29 = fVar28 + DAT_1434b95b8;
          fVar23 = fVar29;
          if (fVar4 < fVar29) {
            fVar23 = fVar29 + fVar6;
          }
          fVar24 = fVar23 * fVar2 * fVar23;
          if (0.0 <= fVar23) {
            fVar24 = fVar23 * fVar3 - fVar24;
          }
          else {
            fVar24 = fVar24 + fVar23 * fVar3;
          }
          fVar23 = fVar24;
          if (fVar24 < 0.0) {
            fVar23 = (float)((uint)fVar24 ^ uVar11);
          }
          if (DAT_1434b967c <= fVar28) {
            fVar27 = fVar28;
            if (fVar4 < fVar28) {
              fVar27 = fVar28 + fVar6;
            }
          }
          else {
            fVar27 = fVar28 + fVar5;
          }
          fVar25 = fVar27 * fVar2 * fVar27;
          if (0.0 <= fVar27) {
            fVar25 = fVar27 * fVar3 - fVar25;
          }
          else {
            fVar25 = fVar25 + fVar27 * fVar3;
          }
          fVar27 = fVar25;
          if (fVar25 < 0.0) {
            fVar27 = (float)((uint)fVar25 ^ uVar11);
          }
          local_230 = (IUnknown *)
                      CONCAT44((int)(((fVar27 * fVar25 - fVar25) * fVar1 + fVar25) * fVar30 +
                                    (float)local_210._4_4_),
                               (int)(((fVar23 * fVar24 - fVar24) * fVar1 + fVar24) * fVar30 +
                                    (float)(int)local_210));
          if (fVar4 < fVar29) {
            fVar29 = fVar29 + fVar6;
          }
          fVar23 = fVar29 * fVar2 * fVar29;
          if (0.0 <= fVar29) {
            fVar23 = fVar29 * fVar3 - fVar23;
          }
          else {
            fVar23 = fVar23 + fVar29 * fVar3;
          }
          fVar29 = fVar23;
          if (fVar23 < 0.0) {
            fVar29 = (float)((uint)fVar23 ^ uVar11);
          }
          if (DAT_1434b967c <= fVar28) {
            if (fVar4 < fVar28) {
              fVar28 = fVar28 + fVar6;
            }
          }
          else {
            fVar28 = fVar28 + fVar5;
          }
          fVar24 = fVar28 * fVar2 * fVar28;
          if (0.0 <= fVar28) {
            fVar24 = fVar28 * fVar3 - fVar24;
          }
          else {
            fVar24 = fVar24 + fVar28 * fVar3;
          }
          fVar28 = fVar24;
          if (fVar24 < 0.0) {
            fVar28 = (float)((uint)fVar24 ^ uVar11);
          }
          local_228 = (IUnknown *)
                      CONCAT44((int)(((fVar28 * fVar24 - fVar24) * fVar1 + fVar24) * fVar30 +
                                    (float)local_218._4_4_),
                               (int)(((fVar29 * fVar23 - fVar23) * fVar1 + fVar23) * fVar30 +
                                    (float)(int)local_218));
          local_290 = (undefined1 *)CONCAT44(local_290._4_4_,10);
          local_298 = (undefined8 *)CONCAT44(local_298._4_4_,2000);
          FUN_140e75870(local_230,local_228,0xa000ff00,0);
        }
        uVar21 = uVar21 - 1;
      } while (uVar21 != 0);
    }
    break;
  case 0x5d8:
    FUN_1406e9050(param_3,&local_230);
    FUN_1406e9170(param_3,&local_f8,0x10);
    uVar16 = FUN_1429fbeb0(&local_220,0xc1);
    local_290 = (undefined1 *)((ulonglong)local_290._4_4_ << 0x20);
    local_298 = (undefined8 *)CONCAT44(local_298._4_4_,0xa0ff00ff);
    FUN_140e722e0(&local_f8,&local_230,0,uVar16);
    local_220 = local_230;
    goto LAB_14177c76b;
  case 0x5d9:
    iVar12 = FUN_1406e8c20(param_3);
    uVar11 = FUN_1406e8c20(param_3);
    if (0 < (int)uVar11) {
      uVar21 = (ulonglong)uVar11;
      do {
        FUN_1406e9170(param_3,&local_f8,0x10);
        local_238 = (IUnknown *)0x0;
        if (iVar12 == 0) {
          FUN_14019ba10(&local_238,"HiddenRect(%d,%d,%d,%d)",(ulonglong)local_f8 & 0xffffffff,
                        local_f8._4_4_);
          uVar16 = FUN_1429fbeb0(&local_208,0xc0);
          local_298 = (undefined8 *)CONCAT44(local_298._4_4_,0xa08080ff);
LAB_14177c097:
          local_290 = (undefined1 *)((ulonglong)local_290._4_4_ << 0x20);
          FUN_140e722e0(&local_f8,&local_238,0,uVar16);
        }
        else if (iVar12 == 1) {
          FUN_14019ba10(&local_238,"Check!!",(ulonglong)local_f8 & 0xffffffff,local_f8._4_4_);
          uVar16 = FUN_1429fbeb0(&local_220,0xc0);
          local_298 = (undefined8 *)CONCAT44(local_298._4_4_,0xa080ff00);
          goto LAB_14177c097;
        }
        if (local_238 != (IUnknown *)0x0) {
          FUN_14019f2c0(local_238 + -0x10);
        }
        uVar21 = uVar21 - 1;
      } while (uVar21 != 0);
    }
    break;
  case 0x5da:
    local_238 = (IUnknown *)0x0;
    piVar18 = (int *)FUN_14019b600(&DAT_143ad6a30,0x29);
    piVar18[1] = 0x18;
    *piVar18 = -1;
    local_238 = (IUnknown *)(piVar18 + 4);
    piVar18[2] = 0;
    *local_238 = (IUnknown)0x0;
    uVar16 = s__________________________1433d2e18._8_8_;
    *(undefined8 *)local_238 = s__________________________1433d2e18._0_8_;
    *(undefined8 *)(piVar18 + 6) = uVar16;
    *(undefined8 *)(piVar18 + 8) = s__________________________1433d2e18._16_8_;
    if (*piVar18 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar18[1] < 0x18) {
      FUN_142e54290(0x90,piVar18[1],0x18);
    }
    *piVar18 = 1;
    local_238[0x18] = (IUnknown)0x0;
    if (piVar18[1] + 1 < 0x19) {
      FUN_142e54290(0x9c,0x18);
    }
    piVar18[2] = 0x18;
    FUN_1415eca30(&local_238,10);
    uVar11 = FUN_1406e8c20(param_3);
    local_220 = local_238;
    if (0 < (int)uVar11) {
      uVar21 = (ulonglong)uVar11;
      do {
        FUN_1406e9050(param_3,&local_230);
        FUN_1415eca30(&local_230,10);
        if (local_230 != (IUnknown *)0x0) {
          FUN_14019f2c0(local_230 + -0x10);
        }
        uVar21 = uVar21 - 1;
        local_220 = local_238;
      } while (uVar21 != 0);
    }
    goto LAB_14177c76b;
  case 0x5db:
    iVar12 = FUN_1406e8c20(param_3);
    FUN_1406e9050(param_3,&local_218);
    uVar13 = FUN_1406e8c20(param_3);
    if (iVar12 == 0) {
      uVar21 = 0;
      local_230 = (IUnknown *)0x0;
      uVar16 = FUN_14019ba10(&local_230,"Script load fail. An error occurred at %s %d line",
                             local_218,uVar13);
      local_228 = (IUnknown *)0x0;
      FUN_14019a260(&local_228,uVar16);
      if (local_230 != (IUnknown *)0x0) {
        FUN_14019f2c0(local_230 + -0x10);
      }
      lVar15 = (longlong)local_228;
      if (local_228 != (IUnknown *)0x0) {
        uVar21 = 0xffffffffffffffff;
        do {
          uVar21 = uVar21 + 1;
        } while (*(char *)((longlong)local_228 + uVar21) != '\0');
      }
      local_288 = (undefined1 *)0x0;
      local_290 = (undefined1 *)((ulonglong)local_290._4_4_ << 0x20);
      local_298 = (undefined8 *)CONCAT44(local_298._4_4_,3);
      lVar17 = (*DAT_143262328)("\\\\.\\pipe\\mypipe",0xc0000000,0,0);
      if (lVar17 != -1) {
        local_298 = (undefined8 *)0x0;
        (*DAT_143262338)(lVar17,lVar15,uVar21 & 0xffffffff,&local_238);
        (*DAT_143262880)(lVar17);
      }
      local_220 = local_218;
      if (lVar15 != 0) {
        FUN_14019f2c0(lVar15 + -0x10);
        local_220 = local_218;
      }
    }
    else {
      uVar21 = 0xffffffffffffffff;
      do {
        uVar20 = uVar21 + 1;
        lVar15 = uVar21 + 1;
        uVar21 = uVar20;
      } while ("Success"[lVar15] != '\0');
      local_288 = (undefined1 *)0x0;
      local_290 = (undefined1 *)((ulonglong)local_290 & 0xffffffff00000000);
      local_298 = (undefined8 *)CONCAT44(local_298._4_4_,3);
      lVar15 = (*DAT_143262328)("\\\\.\\pipe\\mypipe",0xc0000000,0,0);
      local_220 = local_218;
      if (lVar15 != -1) {
        local_298 = (undefined8 *)0x0;
        (*DAT_143262338)(lVar15,"Success",uVar20 & 0xffffffff,&local_238);
        (*DAT_143262880)(lVar15);
        local_220 = local_218;
      }
    }
    goto LAB_14177c76b;
  case 0x5dc:
    FUN_1406e9050(param_3,&local_220);
    FUN_1406e9170(param_3,local_e0,0x10);
    iVar12 = FUN_1406e8c20(param_3);
    uVar13 = FUN_1406e8c20(param_3);
    local_218 = (IUnknown *)0x0;
    local_290 = (undefined1 *)CONCAT44(local_290._4_4_,uVar13);
    local_298 = &local_218;
    FUN_140e76620(&local_230,local_e0,&local_220,0);
    pIVar7 = local_230;
    if ((local_230 != (IUnknown *)0x0) && (DAT_143abfdf0 != 0)) {
      local_228 = (IUnknown *)0x0;
      iVar14 = (**(code **)(*(longlong *)local_230 + 0x208))(local_230,&local_228);
      if (iVar14 < 0) {
        _com_issue_errorex(iVar14,pIVar7,(_GUID *)&DAT_14327fcb0);
      }
      pIVar7 = local_228;
      local_208 = local_228;
      if (local_228 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      FUN_1404a7700(local_1b8,&DAT_143a8b8d8);
      FUN_1404a7700(local_1d0,&DAT_143a8b8d8);
      FUN_1404a7700(local_1e8,&DAT_143a8b8d8);
      FUN_1404a7700(local_200,&DAT_143a8b8d8);
      pIVar8 = local_230;
      if (local_230 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      local_238 = (IUnknown *)((ulonglong)local_238._4_4_ << 0x20);
      iVar14 = (**(code **)(*(longlong *)local_230 + 0xb0))(local_230,&local_238);
      if (iVar14 < 0) {
        _com_issue_errorex(iVar14,pIVar8,(_GUID *)&DAT_143273488);
      }
      iStack_f0 = (int)local_238 + iVar12;
      local_f8 = (undefined8 *)CONCAT62(local_f8._2_6_,3);
      lStack_190 = lStack_1b0;
      local_188 = local_1a8;
      lStack_170 = lStack_1c8;
      local_168 = local_1c0;
      lStack_150 = lStack_1e0;
      local_148 = local_1d8;
      lStack_130 = lStack_1f8;
      local_128 = local_1f0;
      uStack_110 = CONCAT44(uStack_ec,iStack_f0);
      local_118 = (ulonglong)local_f8;
      local_108 = local_e8;
      local_280 = local_198;
      local_288 = local_178;
      local_290 = local_158;
      local_298 = (undefined8 *)local_138;
      iVar14 = (**(code **)(*(longlong *)pIVar7 + 0x140))(pIVar7,0,0,&local_118);
      if (iVar14 < 0) {
        _com_issue_errorex(iVar14,pIVar7,(_GUID *)&DAT_143273488);
      }
      if ((short)local_f8 == 8) {
        local_f8 = (undefined8 *)((ulonglong)local_f8 & 0xffffffffffff0000);
        if (CONCAT44(uStack_ec,iStack_f0) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_ec,iStack_f0) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_f8);
      }
      if (local_200[0] == 8) {
        local_200[0] = 0;
        if (lStack_1f8 != 0) {
          (*DAT_143ad5990)(lStack_1f8 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_200);
      }
      if (local_1e8[0] == 8) {
        local_1e8[0] = 0;
        if (lStack_1e0 != 0) {
          (*DAT_143ad5990)(lStack_1e0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_1e8);
      }
      if (local_1d0[0] == 8) {
        local_1d0[0] = 0;
        if (lStack_1c8 != 0) {
          (*DAT_143ad5990)(lStack_1c8 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_1d0);
      }
      if (local_1b8[0] == 8) {
        local_1b8[0] = 0;
        if (lStack_1b0 != 0) {
          (*DAT_143ad5990)(lStack_1b0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_1b8);
      }
      (**(code **)(*(longlong *)pIVar7 + 0x10))(pIVar7);
      lVar15 = DAT_143abfdf0;
      local_1a0 = &local_210;
      local_210 = 0;
      local_f8 = &local_228;
      local_228 = (IUnknown *)0x0;
      FUN_1401bc8a0(&local_228,&DAT_143278568,0xffffffff);
      local_238 = (IUnknown *)0x0;
      piVar18 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
      piVar18[1] = 0;
      *piVar18 = -1;
      local_238 = (IUnknown *)(piVar18 + 4);
      piVar18[2] = 0;
      *local_238 = (IUnknown)0x0;
      if (*piVar18 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar18[1] < 0) {
        FUN_142e54290(0x90,piVar18[1],0);
      }
      *piVar18 = 1;
      *local_238 = (IUnknown)0x0;
      if (piVar18[1] + 1 < 1) {
        FUN_142e54290(0x9c,0);
      }
      piVar18[2] = 0;
      local_208 = local_230;
      if (local_230 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_230 + 8))();
      }
      local_248 = &local_210;
      local_250 = 0;
      local_258 = 0;
      local_260 = 0;
      local_268 = 0;
      local_270 = 0;
      local_278 = &local_228;
      local_280 = (undefined1 *)((ulonglong)local_280 & 0xffffffff00000000);
      local_288 = (undefined1 *)((ulonglong)local_288 & 0xffffffff00000000);
      local_290 = (undefined1 *)((ulonglong)local_290 & 0xffffffff00000000);
      local_298 = (undefined8 *)((ulonglong)local_298 & 0xffffffff00000000);
      FUN_140dd7ab0(lVar15,&local_208,iVar12,&local_238);
    }
    if (local_230 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_230 + 0x10))(local_230);
    }
LAB_14177c76b:
    if (local_220 != (IUnknown *)0x0) {
      FUN_14019f2c0(local_220 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_142279c50 @ 142279c50   (619 bytes)
//===========================================================

void FUN_142279c50(longlong param_1,undefined4 param_2,undefined8 param_3)

{
  char *pcVar1;
  undefined4 uVar2;
  char *local_res20;
  undefined4 local_18;
  undefined4 uStack_14;
  undefined4 local_10;
  
  switch(param_2) {
  case 0x5c5:
    if (DAT_143acf410 != 0) {
      FUN_14271ef80(DAT_143acf410,param_3);
      return;
    }
    break;
  case 0x5c7:
    FUN_1402a0170(param_3);
    return;
  case 0x5c8:
    FUN_14207e0f0(param_3);
    return;
  case 0x5c9:
    uVar2 = FUN_1406e8c20(param_3);
    local_18 = FUN_1406e8c20(param_3);
    uStack_14 = FUN_1406e8c20(param_3);
    local_10 = FUN_1406e8c20(param_3);
    FUN_142cb1020(DAT_143aa84a0,0x45c,0xffffffff,0,0);
    if (DAT_143ad7740 != 0) {
      FUN_1422e6050(DAT_143ad7740,uVar2,&local_18);
      return;
    }
    break;
  case 0x5cb:
    if (DAT_143ad7748 != 0) {
      FUN_142495590(DAT_143ad7748,param_3);
      return;
    }
    break;
  case 0x5cc:
    FUN_1406e8c20(param_3);
    FUN_1406e9050(param_3,&local_res20);
    FUN_1406e9050(param_3,&local_18);
    pcVar1 = (char *)CONCAT44(uStack_14,local_18);
    if ((((local_res20 == (char *)0x0) || (*local_res20 == '\0')) || (pcVar1 == (char *)0x0)) ||
       (*pcVar1 == '\0')) {
      if (pcVar1 != (char *)0x0) {
        FUN_14019f2c0(pcVar1 + -0x10);
      }
    }
    else {
      FUN_14019f2c0(pcVar1 + -0x10);
    }
    if (local_res20 != (char *)0x0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
    return;
  case 0x5cd:
    FUN_142279fc0(param_1,param_3);
    return;
  case 0x5ce:
    if (DAT_143acc2f8 != 0) {
      FUN_14139b7e0(DAT_143acc2f8,param_3);
      return;
    }
    break;
  case 0x5cf:
    if (DAT_143acbe70 == 0) {
      local_res20 = (char *)FUN_14019b780(&DAT_143ad68a0,0x2f8);
      if (local_res20 != (char *)0x0) {
        FUN_14135aef0(local_res20);
      }
      if (DAT_143acbe70 == 0) {
        return;
      }
    }
    FUN_14135ba80(DAT_143acbe70,param_3);
    return;
  case 0x5d0:
    FUN_140caf700(param_3);
    return;
  case 0x5d1:
    FUN_14118f950(param_3);
    return;
  case 0x5d2:
    FUN_1406e9170(param_3,param_1 + 8,8);
    uVar2 = FUN_1406e8c20(param_3);
    *(undefined4 *)(param_1 + 0x10) = uVar2;
  }
  return;
}


