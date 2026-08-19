
//===========================================================
// FUN_14030ddb0 @ 14030ddb0   (1424 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14030ddb0(int *param_1,longlong param_2)

{
  int iVar1;
  undefined8 *puVar2;
  longlong lVar3;
  longlong *plVar4;
  undefined8 *puVar5;
  undefined8 *puVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  int iVar9;
  longlong lVar10;
  longlong *plVar11;
  ulonglong uVar12;
  undefined1 auStack_4c8 [32];
  longlong local_4a8;
  undefined **local_498 [3];
  int *local_480;
  longlong local_470;
  undefined8 local_44b;
  undefined1 local_443;
  undefined1 local_442;
  undefined1 local_441;
  undefined1 local_440;
  undefined1 local_43f;
  undefined1 local_43e;
  undefined1 local_43d;
  undefined1 local_43c;
  undefined1 local_43b;
  undefined1 local_43a;
  undefined1 local_439;
  undefined1 local_438;
  undefined1 local_437;
  undefined8 local_436 [46];
  undefined1 local_2c6 [32];
  undefined8 local_2a6;
  undefined8 local_29e;
  undefined4 local_296;
  undefined4 local_292;
  undefined4 local_28e;
  undefined4 local_28a;
  undefined1 local_286 [32];
  undefined8 local_266;
  undefined4 local_25e;
  undefined4 local_25a;
  undefined8 local_256;
  undefined8 uStack_24e;
  undefined8 local_246;
  undefined8 uStack_23e;
  undefined8 local_236;
  undefined1 local_22e;
  undefined8 local_22d;
  undefined8 uStack_225;
  undefined8 local_21d;
  undefined8 uStack_215;
  undefined8 local_20d;
  undefined8 uStack_205;
  undefined8 local_1fd;
  undefined8 uStack_1f5;
  undefined8 local_1ed;
  undefined8 uStack_1e5;
  undefined8 local_1dd;
  undefined8 uStack_1d5;
  undefined8 local_1cd;
  undefined8 uStack_1c5;
  undefined8 local_1bd;
  undefined8 uStack_1b5;
  undefined8 local_1ad;
  undefined8 uStack_1a5;
  undefined8 local_19d;
  undefined8 uStack_195;
  undefined8 local_18d;
  undefined8 local_185;
  undefined8 uStack_17d;
  undefined8 local_175;
  undefined8 uStack_16d;
  undefined8 local_165;
  undefined8 uStack_15d;
  undefined8 local_155;
  undefined8 uStack_14d;
  undefined8 local_145;
  undefined8 uStack_13d;
  undefined8 local_135;
  undefined8 uStack_12d;
  undefined8 local_125;
  undefined8 uStack_11d;
  undefined8 local_115;
  undefined8 uStack_10d;
  undefined8 local_105;
  undefined8 uStack_fd;
  undefined8 local_f5;
  undefined4 local_ed;
  undefined8 local_e9;
  undefined8 uStack_e1;
  undefined8 local_d9;
  undefined8 uStack_d1;
  undefined8 local_c9;
  undefined8 uStack_c1;
  undefined8 local_b9;
  undefined8 uStack_b1;
  undefined8 local_a9;
  undefined8 uStack_a1;
  undefined8 local_99;
  undefined8 uStack_91;
  undefined8 local_89;
  undefined8 uStack_81;
  undefined8 local_79;
  undefined8 uStack_71;
  undefined8 local_69;
  undefined8 uStack_61;
  undefined4 local_59;
  undefined4 uStack_55;
  undefined4 uStack_51;
  undefined4 uStack_4d;
  undefined4 local_49;
  undefined4 uStack_45;
  undefined4 uStack_41;
  undefined4 uStack_3d;
  undefined8 local_39;
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_4c8;
  FUN_14030c930();
  if (*param_1 == 0) {
    local_4a8 = FUN_14019b780(&DAT_143ad68a0,0x467);
    if (local_4a8 == 0) {
      uVar7 = 0;
    }
    else {
      uVar7 = FUN_1402f7da0(local_4a8);
    }
    FUN_1402fa3f0(param_2,uVar7);
    return;
  }
  FUN_14030c930(param_1);
  iVar1 = *param_1;
  iVar9 = 0;
  if ((iVar1 + -1 < 0) || (299 < (longlong)iVar1 - 1U)) {
    plVar4 = *(longlong **)(param_1 + 0x25a);
    if (plVar4 == (longlong *)0x0) {
      uVar12 = 0;
    }
    else {
      uVar12 = plVar4[1];
      if (uVar12 != 0) {
        for (puVar2 = *(undefined8 **)*plVar4; puVar2 != (undefined8 *)*plVar4;
            puVar2 = (undefined8 *)*puVar2) {
          plVar11 = puVar2 + 2;
          if (iVar9 == iVar1 + -0x12d) goto LAB_14030de76;
          iVar9 = iVar9 + 1;
        }
      }
    }
    FUN_142e54290(0x608,iVar1 + -1,uVar12 & 0xffffffff);
    plVar11 = (longlong *)(param_1 + 2);
  }
  else {
    plVar11 = (longlong *)(param_1 + ((longlong)iVar1 - 1U) * 2 + 2);
  }
LAB_14030de76:
  lVar3 = *plVar11;
  FUN_14030c930(param_1);
  iVar1 = *param_1;
  if (iVar1 == 0) {
    uVar7 = 0x638;
  }
  else {
    if ((0 < iVar1) && ((longlong)iVar1 - 1U < 300)) {
      (param_1 + ((longlong)iVar1 - 1U) * 2 + 2)[0] = 0;
      (param_1 + ((longlong)iVar1 - 1U) * 2 + 2)[1] = 0;
      *param_1 = *param_1 + -1;
      FUN_14030c930(param_1);
      goto LAB_14030df0b;
    }
    plVar4 = *(longlong **)(param_1 + 0x25a);
    if (plVar4 == (longlong *)0x0) {
      uVar7 = 0x64f;
    }
    else {
      if (plVar4[1] != 0) {
        plVar11 = *(longlong **)(*plVar4 + 8);
        lVar10 = *plVar11;
        plVar4[1] = plVar4[1] + -1;
        *(longlong *)plVar11[1] = lVar10;
        *(longlong *)(lVar10 + 8) = plVar11[1];
        thunk_FUN_140205820(plVar11,0x18);
        *param_1 = *param_1 + -1;
        FUN_14030c930(param_1);
        goto LAB_14030df0b;
      }
      uVar7 = 0x649;
    }
  }
  FUN_142e52dd0(uVar7);
LAB_14030df0b:
  FUN_1401abd80(param_2);
  *(longlong *)(param_2 + 8) = lVar3;
  FUN_1402f7da0(local_498);
  FUN_14030a7e0(lVar3,local_498);
  *(undefined8 *)(lVar3 + 0x4d) = local_44b;
  *(undefined1 *)(lVar3 + 0x55) = local_443;
  *(undefined1 *)(lVar3 + 0x56) = local_442;
  *(undefined1 *)(lVar3 + 0x57) = local_441;
  *(undefined1 *)(lVar3 + 0x58) = local_440;
  *(undefined1 *)(lVar3 + 0x59) = local_43f;
  *(undefined1 *)(lVar3 + 0x5a) = local_43e;
  *(undefined1 *)(lVar3 + 0x5b) = local_43d;
  *(undefined1 *)(lVar3 + 0x5c) = local_43c;
  *(undefined1 *)(lVar3 + 0x5d) = local_43b;
  *(undefined1 *)(lVar3 + 0x5e) = local_43a;
  *(undefined1 *)(lVar3 + 0x5f) = local_439;
  *(undefined1 *)(lVar3 + 0x60) = local_438;
  *(undefined1 *)(lVar3 + 0x61) = local_437;
  lVar10 = 2;
  puVar2 = local_436;
  puVar5 = (undefined8 *)(lVar3 + 0x62);
  do {
    puVar8 = puVar5;
    puVar6 = puVar2;
    uVar7 = puVar6[1];
    *puVar8 = *puVar6;
    puVar8[1] = uVar7;
    uVar7 = puVar6[3];
    puVar8[2] = puVar6[2];
    puVar8[3] = uVar7;
    uVar7 = puVar6[5];
    puVar8[4] = puVar6[4];
    puVar8[5] = uVar7;
    uVar7 = puVar6[7];
    puVar8[6] = puVar6[6];
    puVar8[7] = uVar7;
    uVar7 = puVar6[9];
    puVar8[8] = puVar6[8];
    puVar8[9] = uVar7;
    uVar7 = puVar6[0xb];
    puVar8[10] = puVar6[10];
    puVar8[0xb] = uVar7;
    uVar7 = puVar6[0xd];
    puVar8[0xc] = puVar6[0xc];
    puVar8[0xd] = uVar7;
    uVar7 = puVar6[0xf];
    puVar8[0xe] = puVar6[0xe];
    puVar8[0xf] = uVar7;
    lVar10 = lVar10 + -1;
    puVar2 = puVar6 + 0x10;
    puVar5 = puVar8 + 0x10;
  } while (lVar10 != 0);
  uVar7 = puVar6[0x11];
  puVar8[0x10] = puVar6[0x10];
  puVar8[0x11] = uVar7;
  uVar7 = puVar6[0x13];
  puVar8[0x12] = puVar6[0x12];
  puVar8[0x13] = uVar7;
  uVar7 = puVar6[0x15];
  puVar8[0x14] = puVar6[0x14];
  puVar8[0x15] = uVar7;
  uVar7 = puVar6[0x17];
  puVar8[0x16] = puVar6[0x16];
  puVar8[0x17] = uVar7;
  uVar7 = puVar6[0x19];
  puVar8[0x18] = puVar6[0x18];
  puVar8[0x19] = uVar7;
  uVar7 = puVar6[0x1b];
  puVar8[0x1a] = puVar6[0x1a];
  puVar8[0x1b] = uVar7;
  uVar7 = puVar6[0x1d];
  puVar8[0x1c] = puVar6[0x1c];
  puVar8[0x1d] = uVar7;
  *(undefined8 *)(lVar3 + 0x1f2) = local_2a6;
  *(undefined8 *)(lVar3 + 0x1fa) = local_29e;
  *(undefined4 *)(lVar3 + 0x202) = local_296;
  *(undefined4 *)(lVar3 + 0x206) = local_292;
  *(undefined4 *)(lVar3 + 0x20a) = local_28e;
  *(undefined4 *)(lVar3 + 0x20e) = local_28a;
  *(undefined8 *)(lVar3 + 0x232) = local_266;
  *(undefined4 *)(lVar3 + 0x23a) = local_25e;
  *(undefined4 *)(lVar3 + 0x23e) = local_25a;
  *(undefined8 *)(lVar3 + 0x242) = local_256;
  *(undefined8 *)(lVar3 + 0x24a) = uStack_24e;
  *(undefined8 *)(lVar3 + 0x252) = local_246;
  *(undefined8 *)(lVar3 + 0x25a) = uStack_23e;
  *(undefined8 *)(lVar3 + 0x262) = local_236;
  *(undefined1 *)(lVar3 + 0x26a) = local_22e;
  *(undefined8 *)(lVar3 + 0x26b) = local_22d;
  *(undefined8 *)(lVar3 + 0x273) = uStack_225;
  *(undefined8 *)(lVar3 + 0x27b) = local_21d;
  *(undefined8 *)(lVar3 + 0x283) = uStack_215;
  *(undefined8 *)(lVar3 + 0x28b) = local_20d;
  *(undefined8 *)(lVar3 + 0x293) = uStack_205;
  *(undefined8 *)(lVar3 + 0x29b) = local_1fd;
  *(undefined8 *)(lVar3 + 0x2a3) = uStack_1f5;
  *(undefined8 *)(lVar3 + 0x2ab) = local_1ed;
  *(undefined8 *)(lVar3 + 0x2b3) = uStack_1e5;
  *(undefined8 *)(lVar3 + 699) = local_1dd;
  *(undefined8 *)(lVar3 + 0x2c3) = uStack_1d5;
  *(undefined8 *)(lVar3 + 0x2cb) = local_1cd;
  *(undefined8 *)(lVar3 + 0x2d3) = uStack_1c5;
  *(undefined8 *)(lVar3 + 0x2db) = local_1bd;
  *(undefined8 *)(lVar3 + 0x2e3) = uStack_1b5;
  *(undefined8 *)(lVar3 + 0x2eb) = local_1ad;
  *(undefined8 *)(lVar3 + 0x2f3) = uStack_1a5;
  *(undefined8 *)(lVar3 + 0x2fb) = local_19d;
  *(undefined8 *)(lVar3 + 0x303) = uStack_195;
  *(undefined8 *)(lVar3 + 0x30b) = local_18d;
  *(undefined8 *)(lVar3 + 0x313) = local_185;
  *(undefined8 *)(lVar3 + 0x31b) = uStack_17d;
  *(undefined8 *)(lVar3 + 0x323) = local_175;
  *(undefined8 *)(lVar3 + 0x32b) = uStack_16d;
  *(undefined8 *)(lVar3 + 0x333) = local_165;
  *(undefined8 *)(lVar3 + 0x33b) = uStack_15d;
  *(undefined8 *)(lVar3 + 0x343) = local_155;
  *(undefined8 *)(lVar3 + 0x34b) = uStack_14d;
  *(undefined8 *)(lVar3 + 0x353) = local_145;
  *(undefined8 *)(lVar3 + 0x35b) = uStack_13d;
  *(undefined8 *)(lVar3 + 0x363) = local_135;
  *(undefined8 *)(lVar3 + 0x36b) = uStack_12d;
  *(undefined8 *)(lVar3 + 0x373) = local_125;
  *(undefined8 *)(lVar3 + 0x37b) = uStack_11d;
  *(undefined8 *)(lVar3 + 899) = local_115;
  *(undefined8 *)(lVar3 + 0x38b) = uStack_10d;
  *(undefined8 *)(lVar3 + 0x393) = local_105;
  *(undefined8 *)(lVar3 + 0x39b) = uStack_fd;
  *(undefined8 *)(lVar3 + 0x3a3) = local_f5;
  *(undefined4 *)(lVar3 + 0x3ab) = local_ed;
  *(undefined8 *)(lVar3 + 0x3af) = local_e9;
  *(undefined8 *)(lVar3 + 0x3b7) = uStack_e1;
  *(undefined8 *)(lVar3 + 0x3bf) = local_d9;
  *(undefined8 *)(lVar3 + 0x3c7) = uStack_d1;
  *(undefined8 *)(lVar3 + 0x3cf) = local_c9;
  *(undefined8 *)(lVar3 + 0x3d7) = uStack_c1;
  *(undefined8 *)(lVar3 + 0x3df) = local_b9;
  *(undefined8 *)(lVar3 + 999) = uStack_b1;
  *(undefined8 *)(lVar3 + 0x3ef) = local_a9;
  *(undefined8 *)(lVar3 + 0x3f7) = uStack_a1;
  *(undefined8 *)(lVar3 + 0x3ff) = local_99;
  *(undefined8 *)(lVar3 + 0x407) = uStack_91;
  *(undefined8 *)(lVar3 + 0x40f) = local_89;
  *(undefined8 *)(lVar3 + 0x417) = uStack_81;
  *(undefined8 *)(lVar3 + 0x41f) = local_79;
  *(undefined8 *)(lVar3 + 0x427) = uStack_71;
  *(undefined8 *)(lVar3 + 0x42f) = local_69;
  *(undefined8 *)(lVar3 + 0x437) = uStack_61;
  *(undefined4 *)(lVar3 + 0x43f) = local_59;
  *(undefined4 *)(lVar3 + 0x443) = uStack_55;
  *(undefined4 *)(lVar3 + 0x447) = uStack_51;
  *(undefined4 *)(lVar3 + 1099) = uStack_4d;
  *(undefined4 *)(lVar3 + 0x44f) = local_49;
  *(undefined4 *)(lVar3 + 0x453) = uStack_45;
  *(undefined4 *)(lVar3 + 0x457) = uStack_41;
  *(undefined4 *)(lVar3 + 0x45b) = uStack_3d;
  *(undefined8 *)(lVar3 + 0x45f) = local_39;
  FUN_1401d5120(local_286);
  FUN_1401d5120(local_2c6);
  if (local_470 != 0) {
    thunk_FUN_140205820(local_470,0xc);
  }
  local_498[0] = &PTR_FUN_143273970;
  if (local_480 != (int *)0x0) {
    LOCK();
    local_480[2] = 0;
    local_480[3] = 0;
    UNLOCK();
    do {
    } while (local_480[1] != 0);
    if (local_480 != (int *)0x0) {
      LOCK();
      iVar1 = *local_480;
      *local_480 = *local_480 + -1;
      UNLOCK();
      if (iVar1 == 1) {
        thunk_FUN_140205820(local_480,0x10);
      }
    }
  }
  return;
}



//===========================================================
// FUN_14030db00 @ 14030db00   (684 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14030db00(int *param_1,longlong param_2)

{
  int iVar1;
  undefined8 *puVar2;
  longlong lVar3;
  longlong *plVar4;
  longlong lVar5;
  longlong *plVar6;
  undefined8 uVar7;
  int iVar8;
  ulonglong uVar9;
  undefined1 auStack_d8 [32];
  longlong local_b8;
  undefined **local_a8 [3];
  int *local_90;
  longlong local_80;
  undefined8 local_5b;
  undefined8 local_53;
  undefined8 local_4b;
  undefined8 local_43;
  undefined1 local_3b;
  undefined1 local_3a;
  undefined1 local_39;
  undefined1 local_38;
  undefined1 local_37;
  undefined1 local_36;
  undefined1 local_35;
  undefined1 local_34;
  undefined1 local_33;
  undefined1 local_32;
  undefined1 local_31;
  undefined1 local_30;
  undefined1 local_2f;
  undefined4 local_2e;
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_d8;
  FUN_14030c8a0();
  if (*param_1 == 0) {
    local_b8 = FUN_14019b780(&DAT_143ad68a0,0x7e);
    if (local_b8 == 0) {
      uVar7 = 0;
    }
    else {
      uVar7 = FUN_1402f7cd0(local_b8);
    }
    FUN_1402fa3f0(param_2,uVar7);
    return;
  }
  FUN_14030c8a0(param_1);
  iVar1 = *param_1;
  iVar8 = 0;
  if ((iVar1 + -1 < 0) || (299 < (longlong)iVar1 - 1U)) {
    plVar4 = *(longlong **)(param_1 + 0x25a);
    if (plVar4 == (longlong *)0x0) {
      uVar9 = 0;
    }
    else {
      uVar9 = plVar4[1];
      if (uVar9 != 0) {
        for (puVar2 = *(undefined8 **)*plVar4; puVar2 != (undefined8 *)*plVar4;
            puVar2 = (undefined8 *)*puVar2) {
          plVar6 = puVar2 + 2;
          if (iVar8 == iVar1 + -0x12d) goto LAB_14030dbc0;
          iVar8 = iVar8 + 1;
        }
      }
    }
    FUN_142e54290(0x608,iVar1 + -1,uVar9 & 0xffffffff);
    plVar6 = (longlong *)(param_1 + 2);
  }
  else {
    plVar6 = (longlong *)(param_1 + ((longlong)iVar1 - 1U) * 2 + 2);
  }
LAB_14030dbc0:
  lVar3 = *plVar6;
  FUN_14030c8a0(param_1);
  iVar1 = *param_1;
  if (iVar1 == 0) {
    uVar7 = 0x638;
  }
  else {
    if ((0 < iVar1) && ((longlong)iVar1 - 1U < 300)) {
      (param_1 + ((longlong)iVar1 - 1U) * 2 + 2)[0] = 0;
      (param_1 + ((longlong)iVar1 - 1U) * 2 + 2)[1] = 0;
      *param_1 = *param_1 + -1;
      FUN_14030c8a0(param_1);
      goto LAB_14030dc55;
    }
    plVar4 = *(longlong **)(param_1 + 0x25a);
    if (plVar4 == (longlong *)0x0) {
      uVar7 = 0x64f;
    }
    else {
      if (plVar4[1] != 0) {
        plVar6 = *(longlong **)(*plVar4 + 8);
        lVar5 = *plVar6;
        plVar4[1] = plVar4[1] + -1;
        *(longlong *)plVar6[1] = lVar5;
        *(longlong *)(lVar5 + 8) = plVar6[1];
        thunk_FUN_140205820(plVar6,0x18);
        *param_1 = *param_1 + -1;
        FUN_14030c8a0(param_1);
        goto LAB_14030dc55;
      }
      uVar7 = 0x649;
    }
  }
  FUN_142e52dd0(uVar7);
LAB_14030dc55:
  FUN_1401abd80(param_2);
  *(longlong *)(param_2 + 8) = lVar3;
  FUN_1402f7cd0(local_a8);
  FUN_14030a7e0(lVar3,local_a8);
  *(undefined8 *)(lVar3 + 0x4d) = local_5b;
  *(undefined8 *)(lVar3 + 0x55) = local_53;
  *(undefined8 *)(lVar3 + 0x5d) = local_4b;
  *(undefined8 *)(lVar3 + 0x65) = local_43;
  *(undefined1 *)(lVar3 + 0x6d) = local_3b;
  *(undefined1 *)(lVar3 + 0x6e) = local_3a;
  *(undefined1 *)(lVar3 + 0x6f) = local_39;
  *(undefined1 *)(lVar3 + 0x70) = local_38;
  *(undefined1 *)(lVar3 + 0x71) = local_37;
  *(undefined1 *)(lVar3 + 0x72) = local_36;
  *(undefined1 *)(lVar3 + 0x73) = local_35;
  *(undefined1 *)(lVar3 + 0x74) = local_34;
  *(undefined1 *)(lVar3 + 0x75) = local_33;
  *(undefined1 *)(lVar3 + 0x76) = local_32;
  *(undefined1 *)(lVar3 + 0x77) = local_31;
  *(undefined1 *)(lVar3 + 0x78) = local_30;
  *(undefined1 *)(lVar3 + 0x79) = local_2f;
  *(undefined4 *)(lVar3 + 0x7a) = local_2e;
  if (local_80 != 0) {
    thunk_FUN_140205820(local_80,0xc);
  }
  local_a8[0] = &PTR_FUN_143273970;
  if (local_90 != (int *)0x0) {
    LOCK();
    local_90[2] = 0;
    local_90[3] = 0;
    UNLOCK();
    do {
    } while (local_90[1] != 0);
    if (local_90 != (int *)0x0) {
      LOCK();
      iVar1 = *local_90;
      *local_90 = *local_90 + -1;
      UNLOCK();
      if (iVar1 == 1) {
        thunk_FUN_140205820(local_90,0x10);
      }
    }
  }
  return;
}



//===========================================================
// FUN_14030e340 @ 14030e340   (829 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14030e340(int *param_1,longlong param_2)

{
  int iVar1;
  undefined8 *puVar2;
  longlong lVar3;
  longlong *plVar4;
  longlong lVar5;
  longlong *plVar6;
  undefined8 uVar7;
  int iVar8;
  ulonglong uVar9;
  undefined1 auStack_128 [32];
  longlong local_108;
  undefined **local_f8 [3];
  int *local_e0;
  longlong local_d0;
  undefined1 local_ab;
  undefined1 local_aa;
  undefined1 local_a9;
  undefined1 local_a8;
  undefined1 local_a7;
  undefined1 local_a6;
  undefined1 local_a5;
  undefined1 local_a4;
  undefined1 local_a3;
  undefined1 local_a2;
  undefined1 local_a1;
  undefined1 local_a0;
  undefined1 local_9f;
  undefined8 local_9e;
  undefined8 local_96;
  undefined8 local_8e;
  undefined8 local_86;
  undefined8 local_7e;
  undefined8 local_76;
  undefined8 local_6e;
  undefined4 local_66;
  undefined8 local_62;
  undefined8 local_5a;
  undefined8 local_52;
  undefined4 local_4a;
  undefined8 local_46;
  undefined8 local_3e;
  undefined8 local_36;
  undefined4 local_2e;
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_128;
  FUN_14030c9c0();
  if (*param_1 == 0) {
    local_108 = FUN_14019b780(&DAT_143ad68a0,0xce);
    if (local_108 == 0) {
      uVar7 = 0;
    }
    else {
      uVar7 = FUN_1402f8b40(local_108);
    }
    FUN_1402fa3f0(param_2,uVar7);
    return;
  }
  FUN_14030c9c0(param_1);
  iVar1 = *param_1;
  iVar8 = 0;
  if ((iVar1 + -1 < 0) || (299 < (longlong)iVar1 - 1U)) {
    plVar4 = *(longlong **)(param_1 + 0x25a);
    if (plVar4 == (longlong *)0x0) {
      uVar9 = 0;
    }
    else {
      uVar9 = plVar4[1];
      if (uVar9 != 0) {
        for (puVar2 = *(undefined8 **)*plVar4; puVar2 != (undefined8 *)*plVar4;
            puVar2 = (undefined8 *)*puVar2) {
          plVar6 = puVar2 + 2;
          if (iVar8 == iVar1 + -0x12d) goto LAB_14030e400;
          iVar8 = iVar8 + 1;
        }
      }
    }
    FUN_142e54290(0x608,iVar1 + -1,uVar9 & 0xffffffff);
    plVar6 = (longlong *)(param_1 + 2);
  }
  else {
    plVar6 = (longlong *)(param_1 + ((longlong)iVar1 - 1U) * 2 + 2);
  }
LAB_14030e400:
  lVar3 = *plVar6;
  FUN_14030c9c0(param_1);
  iVar1 = *param_1;
  if (iVar1 == 0) {
    uVar7 = 0x638;
  }
  else {
    if ((0 < iVar1) && ((longlong)iVar1 - 1U < 300)) {
      (param_1 + ((longlong)iVar1 - 1U) * 2 + 2)[0] = 0;
      (param_1 + ((longlong)iVar1 - 1U) * 2 + 2)[1] = 0;
      *param_1 = *param_1 + -1;
      FUN_14030c9c0(param_1);
      goto LAB_14030e495;
    }
    plVar4 = *(longlong **)(param_1 + 0x25a);
    if (plVar4 == (longlong *)0x0) {
      uVar7 = 0x64f;
    }
    else {
      if (plVar4[1] != 0) {
        plVar6 = *(longlong **)(*plVar4 + 8);
        lVar5 = *plVar6;
        plVar4[1] = plVar4[1] + -1;
        *(longlong *)plVar6[1] = lVar5;
        *(longlong *)(lVar5 + 8) = plVar6[1];
        thunk_FUN_140205820(plVar6,0x18);
        *param_1 = *param_1 + -1;
        FUN_14030c9c0(param_1);
        goto LAB_14030e495;
      }
      uVar7 = 0x649;
    }
  }
  FUN_142e52dd0(uVar7);
LAB_14030e495:
  FUN_1401abd80(param_2);
  *(longlong *)(param_2 + 8) = lVar3;
  FUN_1402f8b40(local_f8);
  FUN_14030a7e0(lVar3,local_f8);
  *(undefined1 *)(lVar3 + 0x4d) = local_ab;
  *(undefined1 *)(lVar3 + 0x4e) = local_aa;
  *(undefined1 *)(lVar3 + 0x4f) = local_a9;
  *(undefined1 *)(lVar3 + 0x50) = local_a8;
  *(undefined1 *)(lVar3 + 0x51) = local_a7;
  *(undefined1 *)(lVar3 + 0x52) = local_a6;
  *(undefined1 *)(lVar3 + 0x53) = local_a5;
  *(undefined1 *)(lVar3 + 0x54) = local_a4;
  *(undefined1 *)(lVar3 + 0x55) = local_a3;
  *(undefined1 *)(lVar3 + 0x56) = local_a2;
  *(undefined1 *)(lVar3 + 0x57) = local_a1;
  *(undefined1 *)(lVar3 + 0x58) = local_a0;
  *(undefined1 *)(lVar3 + 0x59) = local_9f;
  *(undefined8 *)(lVar3 + 0x5a) = local_9e;
  *(undefined8 *)(lVar3 + 0x62) = local_96;
  *(undefined8 *)(lVar3 + 0x6a) = local_8e;
  *(undefined8 *)(lVar3 + 0x72) = local_86;
  *(undefined8 *)(lVar3 + 0x7a) = local_7e;
  *(undefined8 *)(lVar3 + 0x82) = local_76;
  *(undefined8 *)(lVar3 + 0x8a) = local_6e;
  *(undefined4 *)(lVar3 + 0x92) = local_66;
  *(undefined8 *)(lVar3 + 0x96) = local_62;
  *(undefined8 *)(lVar3 + 0x9e) = local_5a;
  *(undefined8 *)(lVar3 + 0xa6) = local_52;
  *(undefined4 *)(lVar3 + 0xae) = local_4a;
  *(undefined8 *)(lVar3 + 0xb2) = local_46;
  *(undefined8 *)(lVar3 + 0xba) = local_3e;
  *(undefined8 *)(lVar3 + 0xc2) = local_36;
  *(undefined4 *)(lVar3 + 0xca) = local_2e;
  if (local_d0 != 0) {
    thunk_FUN_140205820(local_d0,0xc);
  }
  local_f8[0] = &PTR_FUN_143273970;
  if (local_e0 != (int *)0x0) {
    LOCK();
    local_e0[2] = 0;
    local_e0[3] = 0;
    UNLOCK();
    do {
    } while (local_e0[1] != 0);
    if (local_e0 != (int *)0x0) {
      LOCK();
      iVar1 = *local_e0;
      *local_e0 = *local_e0 + -1;
      UNLOCK();
      if (iVar1 == 1) {
        thunk_FUN_140205820(local_e0,0x10);
      }
    }
  }
  return;
}



//===========================================================
// FUN_1403023d0 @ 1403023d0   (210 bytes)
//===========================================================

undefined8 FUN_1403023d0(undefined8 param_1,undefined4 param_2)

{
  switch(param_2) {
  case 1:
    FUN_140302860(param_1,&DAT_143abdb20);
    return param_1;
  case 2:
    FUN_140302860(param_1,&DAT_143abdab0);
    return param_1;
  case 3:
    FUN_140302860(param_1,&DAT_143abda40);
    return param_1;
  case 4:
    FUN_140302860(param_1,&DAT_143abd9d0);
    return param_1;
  case 5:
    FUN_140302860(param_1,&DAT_143abd960);
    return param_1;
  case 6:
    FUN_140302860(param_1,&DAT_143abd8f0);
    return param_1;
  default:
    FUN_140302860(param_1,&DAT_143abd880);
    return param_1;
  }
}



//===========================================================
// FUN_140253980 @ 140253980   (739 bytes)
//===========================================================

bool FUN_140253980(int param_1,uint param_2,int param_3,char param_4)

{
  int iVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  bool bVar5;
  
  iVar1 = FUN_1402531f0();
  if ((((iVar1 == 0) && (iVar1 = FUN_140416760(param_1), iVar1 == 0)) &&
      (iVar1 = FUN_140416820(param_1), iVar1 == 0)) &&
     (((iVar1 = FUN_140253130(param_1), param_3 != 2 && (iVar1 != 2)) && (iVar1 != param_3)))) {
    return false;
  }
  uVar2 = param_2 - 0x708;
  iVar1 = 0;
  if (uVar2 < 0x33) {
    uVar3 = 0;
    iVar4 = iVar1;
    do {
      if ((iVar1 <= (int)uVar2) && ((int)uVar2 < iVar1 + 0x11)) {
        uVar3 = param_2 + iVar4 * -0x11 + -0x708;
        break;
      }
      iVar4 = iVar4 + 1;
      iVar1 = iVar1 + 0x11;
    } while (iVar1 < 0x33);
  }
  else if (((param_2 - 3000 < 0x20) || (param_2 - 0xc1c < 0x20)) ||
          (uVar3 = param_2, param_2 - 0xc80 < 0x20)) {
    iVar1 = (int)((ulonglong)((longlong)(int)param_2 * -0x51eb851f) >> 0x20);
    uVar3 = param_2 + ((iVar1 >> 5) - (iVar1 >> 0x1f)) * 100;
  }
  switch(param_1 / 10000) {
  case 100:
    if (uVar3 == 1) {
      return true;
    }
    bVar5 = uVar3 == 0x4b0;
    break;
  case 0x65:
    if (uVar3 == 2) {
      return true;
    }
    bVar5 = uVar3 == 0x4b2;
    break;
  case 0x66:
    if (uVar3 == 3) {
      return true;
    }
    bVar5 = uVar3 == 0x4b8;
    break;
  case 0x67:
    if (uVar3 == 4) {
      return true;
    }
    bVar5 = uVar3 == 0x4b9;
    break;
  case 0x68:
  case 0x69:
    if (uVar3 == 5) {
      return true;
    }
    bVar5 = uVar3 == 0x4b3;
    break;
  case 0x6a:
    if (uVar3 == 6) {
      return true;
    }
    bVar5 = uVar3 == 0x4b4;
    break;
  case 0x6b:
    if (uVar3 == 7) {
      return true;
    }
    bVar5 = uVar3 == 0x4b5;
    break;
  case 0x6c:
    if (uVar3 == 8) {
      return true;
    }
    bVar5 = uVar3 == 0x4b6;
    break;
  case 0x6e:
    if (uVar3 == 9) {
      return true;
    }
    bVar5 = uVar3 == 0x4b1;
    break;
  case 0x6f:
    if ((uVar3 < 0x11) && ((0x1b000U >> (uVar3 & 0x1f) & 1) != 0)) {
      return true;
    }
    if (uVar3 - 0x4ba < 4) {
      return true;
    }
    return false;
  case 0x70:
    if (uVar3 == 0x11) {
      return true;
    }
    bVar5 = uVar3 == 0x1f;
    break;
  case 0x71:
    return uVar3 == 0x16;
  case 0x72:
    return uVar3 == 0x15;
  case 0x73:
    return uVar3 == 0x17;
  case 0x74:
    return uVar3 == 0x1a;
  default:
    iVar1 = FUN_140255180(param_1);
    if (((iVar1 == 0) && (99999 < param_1 - 1600000U)) && (param_1 / 10000 != 0xaa)) {
      return false;
    }
    if (uVar3 == 0xb) {
      return true;
    }
    bVar5 = uVar3 == 0x4b7;
    break;
  case 0x76:
    return uVar3 == 0x1d;
  case 0x77:
    return uVar3 == 0x1e;
  case 0x9c:
    if (param_4 != '\0') {
      return uVar3 - 10 < 2;
    }
  case 0x6d:
  case 0x86:
  case 0x87:
    return uVar3 == 10;
  case 0xa6:
    return uVar3 == 0x1b;
  case 0xa7:
    return (uVar3 - 0x1c & 0xfffffffd) == 0;
  case 0xb4:
    if (uVar3 == 0xe) {
      return true;
    }
    if (uVar3 - 0x18 < 2) {
      return true;
    }
    return false;
  case 0xbe:
    return uVar3 == 0x12;
  case 0xbf:
    return uVar3 == 0x13;
  case 0xc0:
    return uVar3 == 0x14;
  }
  if (bVar5) {
    return true;
  }
  return false;
}



//===========================================================
// FUN_140f80140 @ 140f80140   (57 bytes)
//===========================================================

void FUN_140f80140(longlong param_1,undefined8 param_2,undefined4 param_3)

{
  FUN_1402eeb50(param_1 + 0x30);
  FUN_140f80200(param_1,param_3,0,0,0);
  return;
}


