
//===========================================================
// FUN_1414bc3f0 @ 1414bc3f0   (5661 bytes)
//===========================================================

void FUN_1414bc3f0(longlong param_1)

{
  longlong *plVar1;
  undefined8 uVar2;
  int iVar3;
  longlong lVar4;
  ulonglong uVar5;
  longlong *plVar6;
  longlong *plVar7;
  IUnknown *pIVar8;
  longlong *plVar9;
  longlong *local_res10;
  longlong *local_res18;
  longlong *local_res20;
  undefined1 *puVar10;
  ulonglong in_stack_fffffffffffffb40;
  short local_488;
  undefined6 uStack_486;
  longlong lStack_480;
  undefined8 local_478;
  short local_470;
  undefined6 uStack_46e;
  longlong lStack_468;
  undefined8 local_460;
  short local_458;
  undefined6 uStack_456;
  longlong lStack_450;
  undefined8 local_448;
  undefined4 local_440;
  undefined4 uStack_43c;
  undefined8 uStack_438;
  undefined8 local_430;
  short local_428;
  undefined6 uStack_426;
  longlong lStack_420;
  undefined8 local_418;
  short local_410;
  undefined6 uStack_40e;
  longlong lStack_408;
  undefined8 local_400;
  short local_3f8;
  undefined6 uStack_3f6;
  longlong lStack_3f0;
  undefined8 local_3e8;
  short local_3e0;
  undefined6 uStack_3de;
  longlong lStack_3d8;
  undefined8 local_3d0;
  short local_3c8;
  undefined6 uStack_3c6;
  longlong lStack_3c0;
  undefined8 local_3b8;
  short local_3b0;
  undefined6 uStack_3ae;
  longlong lStack_3a8;
  undefined8 local_3a0;
  short local_398;
  undefined6 uStack_396;
  longlong lStack_390;
  undefined8 local_388;
  short local_380;
  undefined6 uStack_37e;
  longlong lStack_378;
  undefined8 local_370;
  undefined4 local_368;
  undefined4 uStack_364;
  undefined8 uStack_360;
  undefined8 local_358;
  short local_350;
  undefined6 uStack_34e;
  longlong lStack_348;
  undefined8 local_340;
  uint local_338;
  undefined4 uStack_334;
  undefined4 uStack_330;
  undefined4 uStack_32c;
  undefined8 local_328;
  uint local_320;
  undefined4 uStack_31c;
  undefined8 uStack_318;
  undefined8 local_310;
  uint local_308;
  undefined4 uStack_304;
  undefined8 uStack_300;
  undefined8 local_2f8;
  uint local_2f0;
  undefined4 uStack_2ec;
  undefined4 uStack_2e8;
  undefined4 uStack_2e4;
  undefined8 local_2e0;
  undefined1 local_2d8 [4];
  undefined4 local_2d4;
  longlong *local_2d0;
  undefined8 uStack_2c8;
  uint local_2c0;
  undefined4 uStack_2bc;
  undefined4 uStack_2b8;
  undefined4 uStack_2b4;
  undefined8 local_2b0;
  undefined1 local_2a8 [8];
  longlong *local_2a0;
  uint local_298;
  undefined4 uStack_294;
  undefined4 uStack_290;
  undefined4 uStack_28c;
  undefined8 local_288;
  undefined8 local_278;
  longlong lStack_270;
  undefined8 local_268;
  undefined8 local_258;
  longlong lStack_250;
  undefined8 local_248;
  undefined8 local_238;
  longlong lStack_230;
  undefined8 local_228;
  uint local_218;
  undefined4 uStack_214;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  undefined8 local_208;
  uint local_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined8 local_1e8;
  undefined8 local_1d8;
  longlong lStack_1d0;
  undefined8 local_1c8;
  undefined8 local_1b8;
  longlong lStack_1b0;
  undefined8 local_1a8;
  undefined8 local_198;
  longlong lStack_190;
  undefined8 local_188;
  undefined8 local_178;
  longlong lStack_170;
  undefined8 local_168;
  uint local_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined8 local_148;
  uint local_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined8 local_128;
  undefined8 local_118;
  longlong lStack_110;
  undefined8 local_108;
  undefined8 local_f8;
  longlong lStack_f0;
  undefined8 local_e8;
  undefined8 local_d8;
  longlong lStack_d0;
  undefined8 local_c8;
  undefined8 local_b8;
  longlong lStack_b0;
  undefined8 local_a8;
  uint local_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 local_88;
  undefined8 local_78;
  longlong lStack_70;
  undefined8 local_68;
  uint local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  
  FUN_1414be0e0(param_1,local_2a8);
  plVar1 = local_2a0;
  if ((local_2a0 == (longlong *)0x0) || (*local_2a0 == 0)) {
    if (*(longlong **)(param_1 + 0x318) != (longlong *)0x0) {
      (**(code **)(**(longlong **)(param_1 + 0x318) + 0x10))();
    }
    *(undefined8 *)(param_1 + 0x318) = 0;
LAB_1414bd8d5:
    if (plVar1 != (longlong *)0x0) {
      plVar9 = plVar1 + -5;
      if (0xffffe < plVar1[-4] - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar1 = plVar1 + -4;
      lVar4 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if ((int)lVar4 == 1) {
        if (local_2a0[-2] != 0) {
          LOCK();
          *(undefined8 *)(local_2a0[-2] + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(local_2a0[-2] + 4) != 0);
        }
        if (plVar9 != (longlong *)0x0) {
          (**(code **)*plVar9)(plVar9,1);
        }
      }
    }
    return;
  }
  lVar4 = FUN_141ed3540((int)local_2a0[6]);
  plVar9 = (longlong *)0x0;
  local_2d4 = 0;
  local_2d0 = (longlong *)0x0;
  uStack_2c8 = 0;
  if (lVar4 == 0) {
    lVar4 = *plVar1;
  }
  in_stack_fffffffffffffb40 = in_stack_fffffffffffffb40 & 0xffffffff00000000;
  puVar10 = local_2d8;
  FUN_140cd8da0(DAT_143ac0188,lVar4,1,(int)plVar1[3],puVar10,in_stack_fffffffffffffb40);
  pIVar8 = DAT_143add050;
  if (DAT_143add050 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_458);
  if (DAT_143a8b8d8 == 8) {
    if (local_458 == 8) {
      local_458 = 0;
      if (lStack_450 != 0) {
        (*DAT_143ad5990)(lStack_450 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_458);
      if (iVar3 < 0) goto LAB_1414bd966;
    }
    local_458 = 8;
    plVar6 = plVar9;
    if (DAT_143a8b8e0 != 0) {
      plVar6 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    lStack_450 = FUN_1401a5fa0(DAT_143a8b8e0,plVar6);
  }
  else {
    if ((local_458 == 8) && (local_458 = 0, lStack_450 != 0)) {
      (*DAT_143ad5990)(lStack_450 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_458,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_1414bd966:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  (*DAT_143262a20)(&local_470);
  if (DAT_143a8b8d8 == 8) {
    if (local_470 == 8) {
      local_470 = 0;
      if (lStack_468 != 0) {
        (*DAT_143ad5990)(lStack_468 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_470);
      if (iVar3 < 0) goto LAB_1414bd96e;
    }
    local_470 = 8;
    plVar6 = plVar9;
    if (DAT_143a8b8e0 != 0) {
      plVar6 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    lStack_468 = FUN_1401a5fa0(DAT_143a8b8e0,plVar6);
  }
  else {
    if ((local_470 == 8) && (local_470 = 0, lStack_468 != 0)) {
      (*DAT_143ad5990)(lStack_468 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_470,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_1414bd96e:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  (*DAT_143262a20)(&local_488);
  if (DAT_143a8b8d8 == 8) {
    if (local_488 == 8) {
      local_488 = 0;
      if (lStack_480 != 0) {
        (*DAT_143ad5990)(lStack_480 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_488);
      if (iVar3 < 0) goto LAB_1414bd976;
    }
    local_488 = 8;
    plVar6 = plVar9;
    if (DAT_143a8b8e0 != 0) {
      plVar6 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    lStack_480 = FUN_1401a5fa0(DAT_143a8b8e0,plVar6);
  }
  else {
    if ((local_488 == 8) && (local_488 = 0, lStack_480 != 0)) {
      (*DAT_143ad5990)(lStack_480 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_488,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_1414bd976:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  local_338 = CONCAT22(local_338._2_2_,3);
  uStack_330 = 0;
  local_res10 = (longlong *)0x0;
  local_278 = CONCAT62(uStack_456,local_458);
  lStack_270 = lStack_450;
  local_268 = local_448;
  local_258 = CONCAT62(uStack_46e,local_470);
  lStack_250 = lStack_468;
  local_248 = local_460;
  local_238 = CONCAT62(uStack_486,local_488);
  lStack_230 = lStack_480;
  local_228 = local_478;
  local_218 = local_338;
  uStack_214 = uStack_334;
  uStack_210 = 0;
  uStack_20c = uStack_32c;
  local_208 = local_328;
  iVar3 = (**(code **)(*(longlong *)pIVar8 + 0x168))
                    (pIVar8,0,0,0,(ulonglong)puVar10 & 0xffffffff00000000,
                     in_stack_fffffffffffffb40 & 0xffffffff00000000,&local_218,&local_238,&local_258
                     ,&local_278,&local_res10);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar8,(_GUID *)&DAT_14327fcd0);
  }
  plVar6 = *(longlong **)(param_1 + 0x318);
  plVar7 = local_res10;
  if (plVar6 != local_res10) {
    *(longlong **)(param_1 + 0x318) = local_res10;
    plVar7 = plVar9;
    if (plVar6 != (longlong *)0x0) {
      (**(code **)(*plVar6 + 0x10))();
      plVar7 = (longlong *)0x0;
    }
  }
  if (plVar7 != (longlong *)0x0) {
    (**(code **)(*plVar7 + 0x10))(plVar7);
  }
  if ((short)local_338 == 8) {
    local_338 = local_338 & 0xffff0000;
    if (CONCAT44(uStack_32c,uStack_330) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_32c,uStack_330) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_338);
  }
  if (local_488 == 8) {
    local_488 = 0;
    if (lStack_480 != 0) {
      (*DAT_143ad5990)(lStack_480 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_488);
  }
  if (local_470 == 8) {
    local_470 = 0;
    if (lStack_468 != 0) {
      (*DAT_143ad5990)(lStack_468 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_470);
  }
  if (local_458 == 8) {
    local_458 = 0;
    if (lStack_450 != 0) {
      (*DAT_143ad5990)(lStack_450 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_458);
  }
  pIVar8 = *(IUnknown **)(param_1 + 0x318);
  if (pIVar8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  uStack_318 = (longlong *)FUN_142bf6010(param_1,&local_res18);
  uStack_318 = (longlong *)*uStack_318;
  local_320 = CONCAT22(local_320._2_2_,0xd);
  if (uStack_318 != (longlong *)0x0) {
    (**(code **)(*uStack_318 + 8))();
  }
  local_1f8 = local_320;
  uStack_1f4 = uStack_31c;
  uStack_1f0 = (undefined4)uStack_318;
  uStack_1ec = uStack_318._4_4_;
  local_1e8 = local_310;
  iVar3 = (**(code **)(*(longlong *)pIVar8 + 200))(pIVar8,&local_1f8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar8,(_GUID *)&DAT_143273488);
  }
  if ((short)local_320 == 8) {
    local_320 = local_320 & 0xffff0000;
    if (uStack_318 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)uStack_318 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_320);
  }
  if (local_res18 != (longlong *)0x0) {
    (**(code **)(*local_res18 + 0x10))();
  }
  pIVar8 = *(IUnknown **)(param_1 + 0x318);
  if (pIVar8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_3e0);
  if (DAT_143a8b8d8 == 8) {
    if (local_3e0 == 8) {
      local_3e0 = 0;
      if (lStack_3d8 != 0) {
        (*DAT_143ad5990)(lStack_3d8 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_3e0);
      if (iVar3 < 0) goto LAB_1414bd97e;
    }
    local_3e0 = 8;
    plVar6 = plVar9;
    if (DAT_143a8b8e0 != 0) {
      plVar6 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    lStack_3d8 = FUN_1401a5fa0(DAT_143a8b8e0,plVar6);
  }
  else {
    if ((local_3e0 == 8) && (local_3e0 = 0, lStack_3d8 != 0)) {
      (*DAT_143ad5990)(lStack_3d8 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_3e0,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_1414bd97e:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  (*DAT_143262a20)(&local_3f8);
  if (DAT_143a8b8d8 == 8) {
    if (local_3f8 == 8) {
      local_3f8 = 0;
      if (lStack_3f0 != 0) {
        (*DAT_143ad5990)(lStack_3f0 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_3f8);
      if (iVar3 < 0) goto LAB_1414bd986;
    }
    local_3f8 = 8;
    plVar6 = plVar9;
    if (DAT_143a8b8e0 != 0) {
      plVar6 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    lStack_3f0 = FUN_1401a5fa0(DAT_143a8b8e0,plVar6);
  }
  else {
    if ((local_3f8 == 8) && (local_3f8 = 0, lStack_3f0 != 0)) {
      (*DAT_143ad5990)(lStack_3f0 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_3f8,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_1414bd986:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  (*DAT_143262a20)(&local_410);
  if (DAT_143a8b8d8 == 8) {
    if (local_410 == 8) {
      local_410 = 0;
      if (lStack_408 != 0) {
        (*DAT_143ad5990)(lStack_408 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_410);
      if (iVar3 < 0) goto LAB_1414bd98e;
    }
    local_410 = 8;
    plVar6 = plVar9;
    if (DAT_143a8b8e0 != 0) {
      plVar6 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    lStack_408 = FUN_1401a5fa0(DAT_143a8b8e0,plVar6);
  }
  else {
    if ((local_410 == 8) && (local_410 = 0, lStack_408 != 0)) {
      (*DAT_143ad5990)(lStack_408 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_410,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_1414bd98e:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  (*DAT_143262a20)(&local_428);
  if (DAT_143a8b8d8 == 8) {
    if (local_428 == 8) {
      local_428 = 0;
      if (lStack_420 != 0) {
        (*DAT_143ad5990)(lStack_420 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_428);
      if (iVar3 < 0) goto LAB_1414bd996;
    }
    local_428 = 8;
    plVar6 = plVar9;
    if (DAT_143a8b8e0 != 0) {
      plVar6 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    lStack_420 = FUN_1401a5fa0(DAT_143a8b8e0,plVar6);
  }
  else {
    if ((local_428 == 8) && (local_428 = 0, lStack_420 != 0)) {
      (*DAT_143ad5990)(lStack_420 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_428,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_1414bd996:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  (*DAT_143262a20)(&local_440);
  if (DAT_143a8b8d8 == 8) {
    if ((short)local_440 == 8) {
      local_440 = (uint)local_440._2_2_ << 0x10;
      if (uStack_438 != 0) {
        (*DAT_143ad5990)(uStack_438 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_440);
      if (iVar3 < 0) goto LAB_1414bd99e;
    }
    local_440 = CONCAT22(local_440._2_2_,8);
    plVar6 = plVar9;
    if (DAT_143a8b8e0 != 0) {
      plVar6 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    uStack_438 = FUN_1401a5fa0(DAT_143a8b8e0,plVar6);
  }
  else {
    if (((short)local_440 == 8) && (local_440 = (uint)local_440._2_2_ << 0x10, uStack_438 != 0)) {
      (*DAT_143ad5990)(uStack_438 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_440,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_1414bd99e:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  local_1d8 = CONCAT62(uStack_3de,local_3e0);
  lStack_1d0 = lStack_3d8;
  local_1c8 = local_3d0;
  local_1b8 = CONCAT62(uStack_3f6,local_3f8);
  lStack_1b0 = lStack_3f0;
  local_1a8 = local_3e8;
  local_198 = CONCAT62(uStack_40e,local_410);
  lStack_190 = lStack_408;
  local_188 = local_400;
  local_178 = CONCAT62(uStack_426,local_428);
  lStack_170 = lStack_420;
  local_168 = local_418;
  local_158 = local_440;
  uStack_154 = uStack_43c;
  uStack_150 = (undefined4)uStack_438;
  uStack_14c = uStack_438._4_4_;
  local_148 = local_430;
  iVar3 = (**(code **)(*(longlong *)pIVar8 + 0x140))
                    (pIVar8,0x3b,0x78,&local_158,&local_178,&local_198,&local_1b8,&local_1d8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar8,(_GUID *)&DAT_143273488);
  }
  if ((short)local_440 == 8) {
    local_440 = local_440 & 0xffff0000;
    if (uStack_438 != 0) {
      (*DAT_143ad5990)(uStack_438 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_440);
  }
  if (local_428 == 8) {
    local_428 = 0;
    if (lStack_420 != 0) {
      (*DAT_143ad5990)(lStack_420 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_428);
  }
  if (local_410 == 8) {
    local_410 = 0;
    if (lStack_408 != 0) {
      (*DAT_143ad5990)(lStack_408 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_410);
  }
  if (local_3f8 == 8) {
    local_3f8 = 0;
    if (lStack_3f0 != 0) {
      (*DAT_143ad5990)(lStack_3f0 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_3f8);
  }
  if (local_3e0 == 8) {
    local_3e0 = 0;
    if (lStack_3d8 != 0) {
      (*DAT_143ad5990)(lStack_3d8 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_3e0);
  }
  pIVar8 = *(IUnknown **)(param_1 + 0x318);
  if (pIVar8 != (IUnknown *)0x0) {
    iVar3 = (**(code **)(*(longlong *)pIVar8 + 0x200))(pIVar8,0xffffffff);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar8,(_GUID *)&DAT_14327fcb0);
    }
    pIVar8 = *(IUnknown **)(param_1 + 0x318);
    if (pIVar8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    uStack_300 = (longlong *)FUN_142bf6010(param_1,&local_res20);
    uStack_300 = (longlong *)*uStack_300;
    local_308 = CONCAT22(local_308._2_2_,0xd);
    if (uStack_300 != (longlong *)0x0) {
      (**(code **)(*uStack_300 + 8))();
    }
    local_138 = local_308;
    uStack_134 = uStack_304;
    uStack_130 = (undefined4)uStack_300;
    uStack_12c = uStack_300._4_4_;
    local_128 = local_2f8;
    iVar3 = (**(code **)(*(longlong *)pIVar8 + 0x238))(pIVar8,&local_138);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar8,(_GUID *)&DAT_14327fcb0);
    }
    if ((short)local_308 == 8) {
      local_308 = local_308 & 0xffff0000;
      if (uStack_300 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)uStack_300 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_308);
    }
    if (local_res20 != (longlong *)0x0) {
      (**(code **)(*local_res20 + 0x10))();
    }
    pIVar8 = *(IUnknown **)(param_1 + 0x318);
    if (pIVar8 != (IUnknown *)0x0) {
      iVar3 = (**(code **)(*(longlong *)pIVar8 + 0x198))(pIVar8,1);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar8,(_GUID *)&DAT_14327fcb0);
      }
      pIVar8 = *(IUnknown **)(param_1 + 0x318);
      if (pIVar8 != (IUnknown *)0x0) {
        iVar3 = (**(code **)(*(longlong *)pIVar8 + 0x1e0))(pIVar8,1);
        plVar6 = local_2d0;
        if (iVar3 < 0) {
          _com_issue_errorex(iVar3,pIVar8,(_GUID *)&DAT_14327fcb0);
          plVar6 = local_2d0;
        }
        while (plVar6 != (longlong *)0x0) {
          uVar5 = plVar6[-4];
          if ((uVar5 != 0) && (uVar5 < 0x10001)) {
            FUN_142e52ed0(0x33e);
            uVar5 = plVar6[-4];
          }
          plVar7 = plVar9;
          if (uVar5 != 0) {
            plVar7 = (longlong *)(uVar5 + 0x28);
          }
          lVar4 = plVar6[1];
          pIVar8 = *(IUnknown **)(param_1 + 0x318);
          if (pIVar8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          (*DAT_143262a20)(&local_380);
          if (DAT_143a8b8d8 == 8) {
            if (local_380 == 8) {
              local_380 = 0;
              if (lStack_378 != 0) {
                (*DAT_143ad5990)(lStack_378 + -4);
              }
            }
            else {
              iVar3 = (*DAT_143262a18)(&local_380);
              if (iVar3 < 0) goto LAB_1414bd9ce;
            }
            local_380 = 8;
            if (DAT_143a8b8e0 == 0) {
              lStack_378 = FUN_1401a5fa0(0,0);
            }
            else {
              lStack_378 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
            }
          }
          else {
            if ((local_380 == 8) && (local_380 = 0, lStack_378 != 0)) {
              (*DAT_143ad5990)(lStack_378 + -4);
            }
            iVar3 = (*DAT_143262a28)(&local_380,&DAT_143a8b8d8);
            if (iVar3 < 0) {
LAB_1414bd9ce:
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(iVar3);
            }
          }
          (*DAT_143262a20)(&local_398);
          if (DAT_143a8b8d8 == 8) {
            if (local_398 == 8) {
              local_398 = 0;
              if (lStack_390 != 0) {
                (*DAT_143ad5990)(lStack_390 + -4);
              }
            }
            else {
              iVar3 = (*DAT_143262a18)(&local_398);
              if (iVar3 < 0) goto LAB_1414bd9c6;
            }
            local_398 = 8;
            if (DAT_143a8b8e0 == 0) {
              lStack_390 = FUN_1401a5fa0(0,0);
            }
            else {
              lStack_390 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
            }
          }
          else {
            if ((local_398 == 8) && (local_398 = 0, lStack_390 != 0)) {
              (*DAT_143ad5990)(lStack_390 + -4);
            }
            iVar3 = (*DAT_143262a28)(&local_398,&DAT_143a8b8d8);
            if (iVar3 < 0) {
LAB_1414bd9c6:
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(iVar3);
            }
          }
          (*DAT_143262a20)(&local_3b0);
          if (DAT_143a8b8d8 == 8) {
            if (local_3b0 == 8) {
              local_3b0 = 0;
              if (lStack_3a8 != 0) {
                (*DAT_143ad5990)(lStack_3a8 + -4);
              }
            }
            else {
              iVar3 = (*DAT_143262a18)(&local_3b0);
              if (iVar3 < 0) goto LAB_1414bd9be;
            }
            local_3b0 = 8;
            if (DAT_143a8b8e0 == 0) {
              lStack_3a8 = FUN_1401a5fa0(0,0);
            }
            else {
              lStack_3a8 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
            }
          }
          else {
            if ((local_3b0 == 8) && (local_3b0 = 0, lStack_3a8 != 0)) {
              (*DAT_143ad5990)(lStack_3a8 + -4);
            }
            iVar3 = (*DAT_143262a28)(&local_3b0,&DAT_143a8b8d8);
            if (iVar3 < 0) {
LAB_1414bd9be:
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(iVar3);
            }
          }
          (*DAT_143262a20)(&local_3c8);
          if (DAT_143a8b8d8 == 8) {
            if (local_3c8 == 8) {
              local_3c8 = 0;
              if (lStack_3c0 != 0) {
                (*DAT_143ad5990)(lStack_3c0 + -4);
              }
            }
            else {
              iVar3 = (*DAT_143262a18)(&local_3c8);
              if (iVar3 < 0) goto LAB_1414bd9b6;
            }
            local_3c8 = 8;
            if (DAT_143a8b8e0 == 0) {
              lStack_3c0 = FUN_1401a5fa0(0,0);
            }
            else {
              lStack_3c0 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
            }
          }
          else {
            if ((local_3c8 == 8) && (local_3c8 = 0, lStack_3c0 != 0)) {
              (*DAT_143ad5990)(lStack_3c0 + -4);
            }
            iVar3 = (*DAT_143262a28)(&local_3c8,&DAT_143a8b8d8);
            if (iVar3 < 0) {
LAB_1414bd9b6:
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(iVar3);
            }
          }
          uStack_2e8 = *(undefined4 *)(lVar4 + 0x58);
          local_2f0 = CONCAT22(local_2f0._2_2_,3);
          uVar2 = *(undefined8 *)(lVar4 + 0x20);
          (*DAT_143262a20)(&local_298);
          local_118 = CONCAT62(uStack_37e,local_380);
          lStack_110 = lStack_378;
          local_108 = local_370;
          local_f8 = CONCAT62(uStack_396,local_398);
          lStack_f0 = lStack_390;
          local_e8 = local_388;
          local_d8 = CONCAT62(uStack_3ae,local_3b0);
          lStack_d0 = lStack_3a8;
          local_c8 = local_3a0;
          local_b8 = CONCAT62(uStack_3c6,local_3c8);
          lStack_b0 = lStack_3c0;
          local_a8 = local_3b8;
          local_98 = local_2f0;
          uStack_94 = uStack_2ec;
          uStack_90 = uStack_2e8;
          uStack_8c = uStack_2e4;
          local_88 = local_2e0;
          iVar3 = (**(code **)(*(longlong *)pIVar8 + 600))
                            (pIVar8,uVar2,&local_98,&local_b8,&local_d8,&local_f8,&local_118,
                             &local_298);
          if (iVar3 < 0) {
            _com_issue_errorex(iVar3,pIVar8,(_GUID *)&DAT_14327fcb0);
          }
          local_2c0 = local_298;
          uStack_2bc = uStack_294;
          uStack_2b8 = uStack_290;
          uStack_2b4 = uStack_28c;
          local_2b0 = local_288;
          if ((short)local_298 == 8) {
            local_2c0 = local_298 & 0xffff0000;
            if (CONCAT44(uStack_28c,uStack_290) != 0) {
              (*DAT_143ad5990)(CONCAT44(uStack_28c,uStack_290) + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_2c0);
          }
          if ((short)local_2f0 == 8) {
            local_2f0 = local_2f0 & 0xffff0000;
            if (CONCAT44(uStack_2e4,uStack_2e8) != 0) {
              (*DAT_143ad5990)(CONCAT44(uStack_2e4,uStack_2e8) + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_2f0);
          }
          if (local_3c8 == 8) {
            local_3c8 = 0;
            if (lStack_3c0 != 0) {
              (*DAT_143ad5990)(lStack_3c0 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_3c8);
          }
          if (local_3b0 == 8) {
            local_3b0 = 0;
            if (lStack_3a8 != 0) {
              (*DAT_143ad5990)(lStack_3a8 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_3b0);
          }
          if (local_398 == 8) {
            local_398 = 0;
            if (lStack_390 != 0) {
              (*DAT_143ad5990)(lStack_390 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_398);
          }
          plVar6 = plVar7;
          if (local_380 == 8) {
            local_380 = 0;
            if (lStack_378 != 0) {
              (*DAT_143ad5990)(lStack_378 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_380);
          }
        }
        pIVar8 = *(IUnknown **)(param_1 + 0x318);
        if (pIVar8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        (*DAT_143262a20)(&local_350);
        if (DAT_143a8b8d8 == 8) {
          if (local_350 == 8) {
            local_350 = 0;
            if (lStack_348 != 0) {
              (*DAT_143ad5990)(lStack_348 + -4);
            }
          }
          else {
            iVar3 = (*DAT_143262a18)(&local_350);
            if (iVar3 < 0) goto LAB_1414bd9a6;
          }
          local_350 = 8;
          plVar6 = plVar9;
          if (DAT_143a8b8e0 != 0) {
            plVar6 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
          }
          lStack_348 = FUN_1401a5fa0(DAT_143a8b8e0,plVar6);
        }
        else {
          if ((local_350 == 8) && (local_350 = 0, lStack_348 != 0)) {
            (*DAT_143ad5990)(lStack_348 + -4);
          }
          iVar3 = (*DAT_143262a28)(&local_350,&DAT_143a8b8d8);
          if (iVar3 < 0) {
LAB_1414bd9a6:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar3);
          }
        }
        (*DAT_143262a20)(&local_368);
        if (DAT_143a8b8d8 == 8) {
          if ((short)local_368 == 8) {
            local_368 = (uint)local_368._2_2_ << 0x10;
            if (uStack_360 != 0) {
              (*DAT_143ad5990)(uStack_360 + -4);
            }
          }
          else {
            iVar3 = (*DAT_143262a18)(&local_368);
            if (iVar3 < 0) goto LAB_1414bd9ae;
          }
          local_368 = CONCAT22(local_368._2_2_,8);
          plVar6 = plVar9;
          if (DAT_143a8b8e0 != 0) {
            plVar6 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
          }
          uStack_360 = FUN_1401a5fa0(DAT_143a8b8e0,plVar6);
        }
        else {
          if (((short)local_368 == 8) &&
             (local_368 = (uint)local_368._2_2_ << 0x10, uStack_360 != 0)) {
            (*DAT_143ad5990)(uStack_360 + -4);
          }
          iVar3 = (*DAT_143262a28)(&local_368,&DAT_143a8b8d8);
          if (iVar3 < 0) {
LAB_1414bd9ae:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar3);
          }
        }
        local_78 = CONCAT62(uStack_34e,local_350);
        lStack_70 = lStack_348;
        local_68 = local_340;
        local_58 = local_368;
        uStack_54 = uStack_364;
        uStack_50 = (undefined4)uStack_360;
        uStack_4c = uStack_360._4_4_;
        local_48 = local_358;
        iVar3 = (**(code **)(*(longlong *)pIVar8 + 0x280))(pIVar8,0x20,&local_58,&local_78);
        if (iVar3 < 0) {
          _com_issue_errorex(iVar3,pIVar8,(_GUID *)&DAT_14327fcb0);
        }
        if ((short)local_368 == 8) {
          local_368 = local_368 & 0xffff0000;
          if (uStack_360 != 0) {
            (*DAT_143ad5990)(uStack_360 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_368);
        }
        if (local_350 == 8) {
          local_350 = 0;
          if (lStack_348 != 0) {
            (*DAT_143ad5990)(lStack_348 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_350);
        }
        pIVar8 = *(IUnknown **)(param_1 + 0x318);
        if ((int)plVar1[4] < 0) {
          if (pIVar8 == (IUnknown *)0x0) {
LAB_1414bd9e1:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          iVar3 = (**(code **)(*(longlong *)pIVar8 + 0x318))(pIVar8,0x40);
        }
        else {
          if (pIVar8 == (IUnknown *)0x0) goto LAB_1414bda02;
          iVar3 = (**(code **)(*(longlong *)pIVar8 + 0x310))(pIVar8,0x40);
          if (iVar3 < 0) {
            _com_issue_errorex(iVar3,pIVar8,(_GUID *)&DAT_14327fcb0);
          }
          pIVar8 = *(IUnknown **)(param_1 + 0x318);
          if (pIVar8 == (IUnknown *)0x0) goto LAB_1414bd9e1;
          iVar3 = (**(code **)(*(longlong *)pIVar8 + 0x330))(pIVar8,3,(int)plVar1[4]);
        }
        plVar6 = local_2d0;
        if (iVar3 < 0) {
          _com_issue_errorex(iVar3,pIVar8,(_GUID *)&DAT_14327fcb0);
          plVar6 = local_2d0;
        }
        while (plVar7 = plVar6, plVar7 != (longlong *)0x0) {
          uVar5 = plVar7[-4];
          if ((uVar5 != 0) && (uVar5 < 0x10001)) {
            FUN_142e52ed0(0x33e);
            uVar5 = plVar7[-4];
          }
          plVar6 = plVar9;
          if (uVar5 != 0) {
            plVar6 = (longlong *)(uVar5 + 0x28);
          }
          if ((plVar7 != (longlong *)0x0) && (plVar7 = plVar7 + -5, plVar7 != (longlong *)0x0)) {
            (**(code **)*plVar7)(plVar7,1);
          }
        }
        uStack_2c8 = 0;
        local_2d0 = (longlong *)0x0;
        local_2d4 = 0;
        goto LAB_1414bd8d5;
      }
    }
  }
LAB_1414bda02:
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x80004003);
}


