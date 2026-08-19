
//===========================================================
// FUN_1403fb0d0 @ 1403fb0d0   (2773 bytes)
//===========================================================

undefined4 * FUN_1403fb0d0(undefined4 *param_1,undefined4 *param_2)

{
  longlong *plVar1;
  longlong *plVar2;
  undefined4 *puVar3;
  uint uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 *puVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  undefined4 *puVar12;
  
  *param_1 = *param_2;
  if (param_1 + 2 != param_2 + 2) {
    FUN_1402a5e70(param_1 + 2,**(undefined8 **)(param_2 + 2));
  }
  param_1[6] = param_2[6];
  param_1[7] = param_2[7];
  param_1[8] = param_2[8];
  param_1[9] = param_2[9];
  param_1[10] = param_2[10];
  param_1[0xb] = param_2[0xb];
  param_1[0xc] = param_2[0xc];
  param_1[0xe] = param_2[0xe];
  param_1[0xf] = param_2[0xf];
  param_1[0x10] = param_2[0x10];
  FUN_14019a260(param_1 + 0x12,param_2 + 0x12);
  plVar1 = (longlong *)(param_1 + 0x14);
  if (plVar1 != (longlong *)(param_2 + 0x14)) {
    FUN_1401be120(plVar1);
    lVar10 = *(longlong *)(param_2 + 0x14);
    *plVar1 = lVar10;
    if (lVar10 != 0) {
      LOCK();
      *(int *)(lVar10 + 0x10) = *(int *)(lVar10 + 0x10) + 1;
      UNLOCK();
    }
  }
  param_1[0x16] = param_2[0x16];
  param_1[0x17] = param_2[0x17];
  param_1[0x18] = param_2[0x18];
  param_1[0x19] = param_2[0x19];
  param_1[0x1a] = param_2[0x1a];
  param_1[0x1b] = param_2[0x1b];
  param_1[0x1c] = param_2[0x1c];
  param_1[0x1d] = param_2[0x1d];
  param_1[0x1e] = param_2[0x1e];
  FUN_1402326c0(param_1 + 0x20,param_2 + 0x20);
  param_1[0x26] = param_2[0x26];
  FUN_14019a260(param_1 + 0x28,param_2 + 0x28);
  param_1[0x2a] = param_2[0x2a];
  *(undefined8 *)(param_1 + 0x2c) = *(undefined8 *)(param_2 + 0x2c);
  *(undefined1 *)(param_1 + 0x2e) = *(undefined1 *)(param_2 + 0x2e);
  *(undefined2 *)((longlong)param_1 + 0xba) = *(undefined2 *)((longlong)param_2 + 0xba);
  *(undefined2 *)(param_1 + 0x2f) = *(undefined2 *)(param_2 + 0x2f);
  *(undefined2 *)((longlong)param_1 + 0xbe) = *(undefined2 *)((longlong)param_2 + 0xbe);
  *(undefined2 *)(param_1 + 0x30) = *(undefined2 *)(param_2 + 0x30);
  *(undefined2 *)((longlong)param_1 + 0xc2) = *(undefined2 *)((longlong)param_2 + 0xc2);
  *(undefined2 *)(param_1 + 0x31) = *(undefined2 *)(param_2 + 0x31);
  *(undefined2 *)((longlong)param_1 + 0xc6) = *(undefined2 *)((longlong)param_2 + 0xc6);
  *(undefined2 *)(param_1 + 0x32) = *(undefined2 *)(param_2 + 0x32);
  *(undefined2 *)((longlong)param_1 + 0xca) = *(undefined2 *)((longlong)param_2 + 0xca);
  *(undefined2 *)(param_1 + 0x33) = *(undefined2 *)(param_2 + 0x33);
  *(undefined2 *)((longlong)param_1 + 0xce) = *(undefined2 *)((longlong)param_2 + 0xce);
  *(undefined2 *)(param_1 + 0x34) = *(undefined2 *)(param_2 + 0x34);
  *(undefined2 *)((longlong)param_1 + 0xd2) = *(undefined2 *)((longlong)param_2 + 0xd2);
  *(undefined2 *)(param_1 + 0x35) = *(undefined2 *)(param_2 + 0x35);
  *(undefined2 *)((longlong)param_1 + 0xd6) = *(undefined2 *)((longlong)param_2 + 0xd6);
  *(undefined2 *)(param_1 + 0x36) = *(undefined2 *)(param_2 + 0x36);
  *(undefined2 *)((longlong)param_1 + 0xda) = *(undefined2 *)((longlong)param_2 + 0xda);
  *(undefined2 *)(param_1 + 0x37) = *(undefined2 *)(param_2 + 0x37);
  *(undefined2 *)((longlong)param_1 + 0xde) = *(undefined2 *)((longlong)param_2 + 0xde);
  *(undefined2 *)(param_1 + 0x38) = *(undefined2 *)(param_2 + 0x38);
  *(undefined2 *)((longlong)param_1 + 0xe2) = *(undefined2 *)((longlong)param_2 + 0xe2);
  param_1[0x39] = param_2[0x39];
  *(undefined2 *)(param_1 + 0x3a) = *(undefined2 *)(param_2 + 0x3a);
  *(undefined2 *)((longlong)param_1 + 0xea) = *(undefined2 *)((longlong)param_2 + 0xea);
  *(undefined2 *)(param_1 + 0x3b) = *(undefined2 *)(param_2 + 0x3b);
  *(undefined2 *)((longlong)param_1 + 0xee) = *(undefined2 *)((longlong)param_2 + 0xee);
  *(undefined2 *)(param_1 + 0x3c) = *(undefined2 *)(param_2 + 0x3c);
  *(undefined2 *)((longlong)param_1 + 0xf2) = *(undefined2 *)((longlong)param_2 + 0xf2);
  *(undefined2 *)(param_1 + 0x3d) = *(undefined2 *)(param_2 + 0x3d);
  *(undefined2 *)((longlong)param_1 + 0xf6) = *(undefined2 *)((longlong)param_2 + 0xf6);
  *(undefined2 *)(param_1 + 0x3e) = *(undefined2 *)(param_2 + 0x3e);
  *(undefined2 *)((longlong)param_1 + 0xfa) = *(undefined2 *)((longlong)param_2 + 0xfa);
  *(undefined2 *)(param_1 + 0x3f) = *(undefined2 *)(param_2 + 0x3f);
  *(undefined2 *)((longlong)param_1 + 0xfe) = *(undefined2 *)((longlong)param_2 + 0xfe);
  *(undefined2 *)(param_1 + 0x40) = *(undefined2 *)(param_2 + 0x40);
  *(undefined2 *)((longlong)param_1 + 0x102) = *(undefined2 *)((longlong)param_2 + 0x102);
  *(undefined2 *)(param_1 + 0x41) = *(undefined2 *)(param_2 + 0x41);
  *(undefined2 *)((longlong)param_1 + 0x106) = *(undefined2 *)((longlong)param_2 + 0x106);
  *(undefined1 *)(param_1 + 0x42) = *(undefined1 *)(param_2 + 0x42);
  *(undefined1 *)((longlong)param_1 + 0x109) = *(undefined1 *)((longlong)param_2 + 0x109);
  *(undefined1 *)((longlong)param_1 + 0x10a) = *(undefined1 *)((longlong)param_2 + 0x10a);
  *(undefined1 *)((longlong)param_1 + 0x10b) = *(undefined1 *)((longlong)param_2 + 0x10b);
  param_1[0x43] = param_2[0x43];
  *(undefined1 *)(param_1 + 0x44) = *(undefined1 *)(param_2 + 0x44);
  *(undefined1 *)((longlong)param_1 + 0x111) = *(undefined1 *)((longlong)param_2 + 0x111);
  *(undefined1 *)((longlong)param_1 + 0x112) = *(undefined1 *)((longlong)param_2 + 0x112);
  *(undefined1 *)((longlong)param_1 + 0x113) = *(undefined1 *)((longlong)param_2 + 0x113);
  *(undefined1 *)(param_1 + 0x45) = *(undefined1 *)(param_2 + 0x45);
  *(undefined1 *)((longlong)param_1 + 0x115) = *(undefined1 *)((longlong)param_2 + 0x115);
  *(undefined1 *)((longlong)param_1 + 0x116) = *(undefined1 *)((longlong)param_2 + 0x116);
  *(undefined1 *)((longlong)param_1 + 0x117) = *(undefined1 *)((longlong)param_2 + 0x117);
  *(undefined1 *)(param_1 + 0x46) = *(undefined1 *)(param_2 + 0x46);
  *(undefined1 *)((longlong)param_1 + 0x119) = *(undefined1 *)((longlong)param_2 + 0x119);
  *(undefined1 *)((longlong)param_1 + 0x11a) = *(undefined1 *)((longlong)param_2 + 0x11a);
  *(undefined1 *)((longlong)param_1 + 0x11b) = *(undefined1 *)((longlong)param_2 + 0x11b);
  *(undefined1 *)(param_1 + 0x47) = *(undefined1 *)(param_2 + 0x47);
  *(undefined1 *)((longlong)param_1 + 0x11d) = *(undefined1 *)((longlong)param_2 + 0x11d);
  param_1[0x48] = param_2[0x48];
  param_1[0x49] = param_2[0x49];
  param_1[0x4a] = param_2[0x4a];
  param_1[0x4b] = param_2[0x4b];
  *(undefined1 *)(param_1 + 0x4c) = *(undefined1 *)(param_2 + 0x4c);
  param_1[0x4d] = param_2[0x4d];
  param_1[0x4e] = param_2[0x4e];
  *(undefined8 *)(param_1 + 0x50) = *(undefined8 *)(param_2 + 0x50);
  *(undefined8 *)(param_1 + 0x52) = *(undefined8 *)(param_2 + 0x52);
  param_1[0x54] = param_2[0x54];
  param_1[0x55] = param_2[0x55];
  param_1[0x56] = param_2[0x56];
  param_1[0x57] = param_2[0x57];
  param_1[0x58] = param_2[0x58];
  param_1[0x59] = param_2[0x59];
  param_1[0x5a] = param_2[0x5a];
  param_1[0x5b] = param_2[0x5b];
  *(undefined2 *)(param_1 + 0x5c) = *(undefined2 *)(param_2 + 0x5c);
  *(undefined1 *)((longlong)param_1 + 0x172) = *(undefined1 *)((longlong)param_2 + 0x172);
  *(undefined1 *)((longlong)param_1 + 0x173) = *(undefined1 *)((longlong)param_2 + 0x173);
  param_1[0x5d] = param_2[0x5d];
  param_1[0x5e] = param_2[0x5e];
  param_1[0x5f] = param_2[0x5f];
  param_1[0x60] = param_2[0x60];
  param_1[0x61] = param_2[0x61];
  *(undefined1 *)(param_1 + 0x62) = *(undefined1 *)(param_2 + 0x62);
  param_1[99] = param_2[99];
  param_1[100] = param_2[100];
  param_1[0x65] = param_2[0x65];
  *(undefined2 *)(param_1 + 0x66) = *(undefined2 *)(param_2 + 0x66);
  *(undefined2 *)((longlong)param_1 + 0x19a) = *(undefined2 *)((longlong)param_2 + 0x19a);
  param_1[0x67] = param_2[0x67];
  param_1[0x68] = param_2[0x68];
  *(undefined1 *)(param_1 + 0x69) = *(undefined1 *)(param_2 + 0x69);
  param_1[0x6a] = param_2[0x6a];
  param_1[0x6b] = param_2[0x6b];
  param_1[0x6c] = param_2[0x6c];
  param_1[0x6d] = param_2[0x6d];
  param_1[0x6e] = param_2[0x6e];
  param_1[0x6f] = param_2[0x6f];
  param_1[0x70] = param_2[0x70];
  param_1[0x71] = param_2[0x71];
  param_1[0x72] = param_2[0x72];
  *(undefined8 *)(param_1 + 0x74) = *(undefined8 *)(param_2 + 0x74);
  *(undefined1 *)(param_1 + 0x76) = *(undefined1 *)(param_2 + 0x76);
  *(undefined1 *)((longlong)param_1 + 0x1d9) = *(undefined1 *)((longlong)param_2 + 0x1d9);
  *(undefined1 *)((longlong)param_1 + 0x1da) = *(undefined1 *)((longlong)param_2 + 0x1da);
  param_1[0x77] = param_2[0x77];
  *(undefined1 *)(param_1 + 0x78) = *(undefined1 *)(param_2 + 0x78);
  *(undefined1 *)((longlong)param_1 + 0x1e1) = *(undefined1 *)((longlong)param_2 + 0x1e1);
  FUN_1402326c0(param_1 + 0x7a,param_2 + 0x7a);
  FUN_1403fa820(param_1 + 0x80,param_2 + 0x80);
  param_1[0x84] = param_2[0x84];
  plVar1 = (longlong *)(param_1 + 0x88);
  param_1[0x85] = param_2[0x85];
  param_1[0x86] = param_2[0x86];
  if (plVar1 != (longlong *)(param_2 + 0x88)) {
    lVar10 = *(longlong *)(param_2 + 0x88);
    lVar9 = *plVar1;
    lVar11 = *(longlong *)(param_2 + 0x8a) - lVar10;
    if ((ulonglong)(*(longlong *)(param_1 + 0x8c) - lVar9 >> 3) < (ulonglong)(lVar11 >> 3)) {
      FUN_14040fcb0(plVar1);
      lVar9 = *plVar1;
    }
    FUN_142ef7ba0(lVar9,lVar10,lVar11);
    *(longlong *)(param_1 + 0x8a) = lVar9 + lVar11;
  }
  param_1[0x8e] = param_2[0x8e];
  plVar1 = (longlong *)(param_1 + 0xaa);
  param_1[0x8f] = param_2[0x8f];
  param_1[0x90] = param_2[0x90];
  param_1[0x91] = param_2[0x91];
  *(undefined1 *)(param_1 + 0x92) = *(undefined1 *)(param_2 + 0x92);
  *(undefined1 *)((longlong)param_1 + 0x249) = *(undefined1 *)((longlong)param_2 + 0x249);
  param_1[0x93] = param_2[0x93];
  *(undefined1 *)(param_1 + 0x94) = *(undefined1 *)(param_2 + 0x94);
  param_1[0x95] = param_2[0x95];
  param_1[0x96] = param_2[0x96];
  param_1[0x97] = param_2[0x97];
  param_1[0x98] = param_2[0x98];
  param_1[0x99] = param_2[0x99];
  param_1[0x9a] = param_2[0x9a];
  param_1[0x9b] = param_2[0x9b];
  param_1[0x9c] = param_2[0x9c];
  param_1[0x9d] = param_2[0x9d];
  param_1[0x9e] = param_2[0x9e];
  param_1[0x9f] = param_2[0x9f];
  uVar5 = param_2[0xa1];
  uVar6 = param_2[0xa2];
  uVar7 = param_2[0xa3];
  param_1[0xa0] = param_2[0xa0];
  param_1[0xa1] = uVar5;
  param_1[0xa2] = uVar6;
  param_1[0xa3] = uVar7;
  *(undefined8 *)(param_1 + 0xa4) = *(undefined8 *)(param_2 + 0xa4);
  param_1[0xa6] = param_2[0xa6];
  param_1[0xa7] = param_2[0xa7];
  param_1[0xa8] = param_2[0xa8];
  if (plVar1 != (longlong *)(param_2 + 0xaa)) {
    FUN_1401be120(plVar1);
    lVar10 = *(longlong *)(param_2 + 0xaa);
    *plVar1 = lVar10;
    if (lVar10 != 0) {
      LOCK();
      *(int *)(lVar10 + 0x10) = *(int *)(lVar10 + 0x10) + 1;
      UNLOCK();
    }
  }
  param_1[0xac] = param_2[0xac];
  *(undefined1 *)(param_1 + 0xad) = *(undefined1 *)(param_2 + 0xad);
  *(undefined1 *)((longlong)param_1 + 0x2b5) = *(undefined1 *)((longlong)param_2 + 0x2b5);
  FUN_14019a260(param_1 + 0xae,param_2 + 0xae);
  FUN_14019a260(param_1 + 0xb0,param_2 + 0xb0);
  FUN_1403f9cb0(param_1 + 0xb2,param_2 + 0xb2);
  FUN_14036f770(param_1 + 0xb8,param_2 + 0xb8);
  FUN_1403f9b00(param_1 + 0xbe,param_2 + 0xbe);
  FUN_1403f9b00(param_1 + 0xc4,param_2 + 0xc4);
  plVar1 = (longlong *)(param_2 + 0xce);
  param_1[0xca] = param_2[0xca];
  plVar2 = (longlong *)(param_1 + 0xce);
  param_1[0xcb] = param_2[0xcb];
  param_1[0xcc] = param_2[0xcc];
  if (plVar2 != plVar1) {
    lVar10 = 0;
    if (*plVar2 != 0) {
      thunk_FUN_140205820(*plVar2 + -8,0);
      *plVar2 = 0;
    }
    if (*plVar1 != 0) {
      uVar4 = *(uint *)(*plVar1 + -8);
      if (uVar4 != 0) {
        lVar9 = FUN_14019b780(&DAT_143ad68a0,(ulonglong)uVar4 * 8 + 8);
        if (lVar9 != 0) {
          lVar10 = lVar9 + 8;
        }
        *plVar2 = lVar10;
        *(ulonglong *)(lVar10 + -8) = (ulonglong)uVar4;
        puVar12 = (undefined4 *)*plVar1;
        puVar3 = puVar12 + (ulonglong)uVar4 * 2;
        puVar8 = (undefined4 *)*plVar2;
        for (; puVar12 < puVar3; puVar12 = puVar12 + 2) {
          *puVar8 = *puVar12;
          puVar8[1] = puVar12[1];
          puVar8 = puVar8 + 2;
        }
      }
    }
  }
  param_1[0xd0] = param_2[0xd0];
  param_1[0xd1] = param_2[0xd1];
  param_1[0xd2] = param_2[0xd2];
  param_1[0xd3] = param_2[0xd3];
  param_1[0xd4] = param_2[0xd4];
  param_1[0xd5] = param_2[0xd5];
  *(undefined8 *)(param_1 + 0xd6) = *(undefined8 *)(param_2 + 0xd6);
  param_1[0xd8] = param_2[0xd8];
  param_1[0xd9] = param_2[0xd9];
  FUN_14022de90(param_1 + 0xda,param_2 + 0xda);
  FUN_1403f9fb0(param_1 + 0xdc,param_2 + 0xdc);
  FUN_1403faee0(param_1 + 0xe0,param_2 + 0xe0);
  param_1[0xe4] = param_2[0xe4];
  param_1[0xe5] = param_2[0xe5];
  *(undefined1 *)(param_1 + 0xe6) = *(undefined1 *)(param_2 + 0xe6);
  return param_1;
}



//===========================================================
// FUN_14038d3c0 @ 14038d3c0   (1397 bytes)
//===========================================================

undefined1 FUN_14038d3c0(undefined8 param_1,longlong param_2,char param_3)

{
  longlong lVar1;
  undefined1 extraout_AL;
  undefined1 extraout_AL_00;
  byte bVar2;
  undefined1 uVar3;
  undefined1 extraout_AL_01;
  char cVar4;
  undefined1 extraout_AL_02;
  undefined1 extraout_AL_03;
  undefined1 extraout_AL_04;
  undefined1 extraout_AH;
  undefined1 extraout_AH_00;
  undefined1 extraout_AH_01;
  undefined1 extraout_AH_02;
  undefined1 extraout_AH_03;
  undefined1 extraout_AH_04;
  undefined1 extraout_AH_05;
  undefined1 extraout_AH_06;
  undefined1 extraout_AH_07;
  undefined1 extraout_AH_08;
  undefined1 extraout_AH_09;
  undefined1 extraout_AH_10;
  undefined1 extraout_AH_11;
  undefined1 extraout_AH_12;
  undefined1 extraout_AH_13;
  undefined1 extraout_AH_14;
  undefined1 extraout_AH_15;
  undefined1 extraout_AH_16;
  undefined1 extraout_AH_17;
  undefined1 extraout_AH_18;
  undefined1 extraout_AH_19;
  undefined1 extraout_AH_20;
  undefined1 extraout_AH_21;
  undefined1 extraout_AH_22;
  undefined1 extraout_AH_23;
  undefined1 extraout_AH_24;
  undefined1 extraout_AH_25;
  undefined1 extraout_AH_26;
  undefined1 extraout_AH_27;
  undefined1 extraout_AH_28;
  undefined1 extraout_AH_29;
  undefined1 extraout_AH_30;
  undefined1 extraout_AH_31;
  undefined1 extraout_AH_32;
  undefined1 extraout_AH_33;
  undefined1 extraout_AH_34;
  undefined1 extraout_AH_35;
  undefined1 extraout_AH_36;
  undefined2 extraout_var;
  undefined2 extraout_var_00;
  undefined2 extraout_var_01;
  undefined2 extraout_var_02;
  undefined2 extraout_var_03;
  undefined2 extraout_var_04;
  undefined4 extraout_var_05;
  undefined4 extraout_var_06;
  
  if (param_2 != 0) {
    FUN_1401b0340(param_2 + 0x20);
    FUN_140388c60(param_1,CONCAT22(extraout_var,CONCAT11(extraout_AH,extraout_AL)));
    lVar1 = CONCAT44(extraout_var_05,
                     CONCAT22(extraout_var_00,CONCAT11(extraout_AH_00,extraout_AL_00)));
    if ((CONCAT44(extraout_var_05,CONCAT22(extraout_var_00,CONCAT11(extraout_AH_00,extraout_AL_00)))
        & CONCAT44(extraout_var_05,CONCAT22(extraout_var_00,CONCAT11(extraout_AH_00,extraout_AL_00))
                  )) != 0) {
      bVar2 = FUN_1401b0050(param_2 + 0xfa,*(undefined4 *)(param_2 + 0xfe));
      if (param_3 == '\0') {
        if ((((bVar2 <= *(byte *)(lVar1 + 0xb8)) &&
             (uVar3 = FUN_1401ab420(param_2 + 0x62,*(undefined4 *)(param_2 + 0x66)),
             CONCAT11(extraout_AH_18,uVar3) == *(short *)(lVar1 + 0xba) ||
             CONCAT11(extraout_AH_18,uVar3) < *(short *)(lVar1 + 0xba))) &&
            (uVar3 = FUN_1401ab420(param_2 + 0x6a,*(undefined4 *)(param_2 + 0x6e)),
            CONCAT11(extraout_AH_19,uVar3) == *(short *)(lVar1 + 0xbc) ||
            CONCAT11(extraout_AH_19,uVar3) < *(short *)(lVar1 + 0xbc))) &&
           (((((uVar3 = FUN_1401ab420(param_2 + 0x72,*(undefined4 *)(param_2 + 0x76)),
               CONCAT11(extraout_AH_20,uVar3) == *(short *)(lVar1 + 0xbe) ||
               CONCAT11(extraout_AH_20,uVar3) < *(short *)(lVar1 + 0xbe) &&
               (uVar3 = FUN_1401ab420(param_2 + 0x7a,*(undefined4 *)(param_2 + 0x7e)),
               CONCAT11(extraout_AH_21,uVar3) == *(short *)(lVar1 + 0xc0) ||
               CONCAT11(extraout_AH_21,uVar3) < *(short *)(lVar1 + 0xc0))) &&
              ((uVar3 = FUN_1401ab420(param_2 + 0x82,*(undefined4 *)(param_2 + 0x86)),
               CONCAT11(extraout_AH_22,uVar3) == *(short *)(lVar1 + 0xc2) ||
               CONCAT11(extraout_AH_22,uVar3) < *(short *)(lVar1 + 0xc2) &&
               ((uVar3 = FUN_1401ab420(param_2 + 0x8a,*(undefined4 *)(param_2 + 0x8e)),
                CONCAT11(extraout_AH_23,uVar3) == *(short *)(lVar1 + 0xc4) ||
                CONCAT11(extraout_AH_23,uVar3) < *(short *)(lVar1 + 0xc4) &&
                (uVar3 = FUN_1401ab420(param_2 + 0xa2,*(undefined4 *)(param_2 + 0xa6)),
                CONCAT11(extraout_AH_24,uVar3) == *(short *)(lVar1 + 0xcc) ||
                CONCAT11(extraout_AH_24,uVar3) < *(short *)(lVar1 + 0xcc))))))) &&
             (uVar3 = FUN_1401ab420(param_2 + 0xaa,*(undefined4 *)(param_2 + 0xae)),
             CONCAT11(extraout_AH_25,uVar3) == *(short *)(lVar1 + 0xce) ||
             CONCAT11(extraout_AH_25,uVar3) < *(short *)(lVar1 + 0xce))) &&
            (((((uVar3 = FUN_1401ab420(param_2 + 0xb2,*(undefined4 *)(param_2 + 0xb6)),
                CONCAT11(extraout_AH_26,uVar3) == *(short *)(lVar1 + 0xd0) ||
                CONCAT11(extraout_AH_26,uVar3) < *(short *)(lVar1 + 0xd0) &&
                (uVar3 = FUN_1401ab420(param_2 + 0xba,*(undefined4 *)(param_2 + 0xbe)),
                CONCAT11(extraout_AH_27,uVar3) == *(short *)(lVar1 + 0xd2) ||
                CONCAT11(extraout_AH_27,uVar3) < *(short *)(lVar1 + 0xd2))) &&
               (uVar3 = FUN_1401ab420(param_2 + 0xc2,*(undefined4 *)(param_2 + 0xc6)),
               CONCAT11(extraout_AH_28,uVar3) == *(short *)(lVar1 + 0xd4) ||
               CONCAT11(extraout_AH_28,uVar3) < *(short *)(lVar1 + 0xd4))) &&
              ((uVar3 = FUN_1401ab420(param_2 + 0xca,*(undefined4 *)(param_2 + 0xce)),
               CONCAT11(extraout_AH_29,uVar3) == *(short *)(lVar1 + 0xd6) ||
               CONCAT11(extraout_AH_29,uVar3) < *(short *)(lVar1 + 0xd6) &&
               (uVar3 = FUN_1401ab420(param_2 + 0xd2,*(undefined4 *)(param_2 + 0xd6)),
               CONCAT11(extraout_AH_30,uVar3) == *(short *)(lVar1 + 0xd8) ||
               CONCAT11(extraout_AH_30,uVar3) < *(short *)(lVar1 + 0xd8))))) &&
             ((uVar3 = FUN_1401ab420(param_2 + 0xda,*(undefined4 *)(param_2 + 0xde)),
              CONCAT11(extraout_AH_31,uVar3) == *(short *)(lVar1 + 0xda) ||
              CONCAT11(extraout_AH_31,uVar3) < *(short *)(lVar1 + 0xda) &&
              ((uVar3 = FUN_1401ab420(param_2 + 0x92,*(undefined4 *)(param_2 + 0x96)),
               CONCAT11(extraout_AH_32,uVar3) == *(short *)(lVar1 + 0xdc) ||
               CONCAT11(extraout_AH_32,uVar3) < *(short *)(lVar1 + 0xdc) &&
               (uVar3 = FUN_1401ab420(param_2 + 0x9a,*(undefined4 *)(param_2 + 0x9e)),
               CONCAT11(extraout_AH_33,uVar3) == *(short *)(lVar1 + 0xde) ||
               CONCAT11(extraout_AH_33,uVar3) < *(short *)(lVar1 + 0xde))))))))))) {
          FUN_14038d180(DAT_143aa8328,param_2);
          if (((CONCAT22(extraout_var_02,CONCAT11(extraout_AH_34,extraout_AL_02)) &
               CONCAT22(extraout_var_02,CONCAT11(extraout_AH_34,extraout_AL_02))) != 0) &&
             (bVar2 = FUN_1401b0050(param_2 + 0x11a,*(undefined4 *)(param_2 + 0x11e)), 1 < bVar2)) {
            return 1;
          }
          cVar4 = FUN_1401b0050(param_2 + 0x112,*(undefined4 *)(param_2 + 0x116));
          if (((((cVar4 == '\0') &&
                (FUN_1401a1790(param_2 + 0x122,*(undefined4 *)(param_2 + 0x132)),
                (longlong)
                (CONCAT44(extraout_var_06,
                          CONCAT22(extraout_var_03,CONCAT11(extraout_AH_35,extraout_AL_03))) &
                CONCAT44(extraout_var_06,
                         CONCAT22(extraout_var_03,CONCAT11(extraout_AH_35,extraout_AL_03)))) < 1))
               && (FUN_1401ba9d0(param_2 + 0x146,*(undefined4 *)(param_2 + 0x14e)),
                  (int)(CONCAT22(extraout_var_04,CONCAT11(extraout_AH_36,extraout_AL_04)) &
                       CONCAT22(extraout_var_04,CONCAT11(extraout_AH_36,extraout_AL_04))) < 1)) &&
              ((bVar2 = FUN_1401b0050(param_2 + 0x186,*(undefined4 *)(param_2 + 0x18a)),
               bVar2 <= *(byte *)(lVar1 + 0x119) &&
               (bVar2 = FUN_1401b0050(param_2 + 0x18e,*(undefined4 *)(param_2 + 0x192)),
               bVar2 <= *(byte *)(lVar1 + 0x11a))))) &&
             ((bVar2 = FUN_1401b0050(param_2 + 0x196,*(undefined4 *)(param_2 + 0x19a)),
              bVar2 <= *(byte *)(lVar1 + 0x11b) &&
              (bVar2 = FUN_1401b0050(param_2 + 0x19e,*(undefined4 *)(param_2 + 0x1a2)),
              bVar2 <= *(byte *)(lVar1 + 0x11c))))) {
            return 0;
          }
        }
      }
      else if ((((bVar2 < *(byte *)(lVar1 + 0xb8)) &&
                (uVar3 = FUN_1401ab420(param_2 + 0x62,*(undefined4 *)(param_2 + 0x66)),
                CONCAT11(extraout_AH_01,uVar3) < *(short *)(lVar1 + 0xba))) &&
               ((uVar3 = FUN_1401ab420(param_2 + 0x6a,*(undefined4 *)(param_2 + 0x6e)),
                CONCAT11(extraout_AH_02,uVar3) < *(short *)(lVar1 + 0xbc) &&
                (((uVar3 = FUN_1401ab420(param_2 + 0x72,*(undefined4 *)(param_2 + 0x76)),
                  CONCAT11(extraout_AH_03,uVar3) < *(short *)(lVar1 + 0xbe) &&
                  (uVar3 = FUN_1401ab420(param_2 + 0x7a,*(undefined4 *)(param_2 + 0x7e)),
                  CONCAT11(extraout_AH_04,uVar3) < *(short *)(lVar1 + 0xc0))) &&
                 (uVar3 = FUN_1401ab420(param_2 + 0x82,*(undefined4 *)(param_2 + 0x86)),
                 CONCAT11(extraout_AH_05,uVar3) < *(short *)(lVar1 + 0xc2))))))) &&
              (((uVar3 = FUN_1401ab420(param_2 + 0x8a,*(undefined4 *)(param_2 + 0x8e)),
                CONCAT11(extraout_AH_06,uVar3) < *(short *)(lVar1 + 0xc4) &&
                (uVar3 = FUN_1401ab420(param_2 + 0xa2,*(undefined4 *)(param_2 + 0xa6)),
                CONCAT11(extraout_AH_07,uVar3) < *(short *)(lVar1 + 0xcc))) &&
               ((uVar3 = FUN_1401ab420(param_2 + 0xaa,*(undefined4 *)(param_2 + 0xae)),
                CONCAT11(extraout_AH_08,uVar3) < *(short *)(lVar1 + 0xce) &&
                (((uVar3 = FUN_1401ab420(param_2 + 0xb2,*(undefined4 *)(param_2 + 0xb6)),
                  CONCAT11(extraout_AH_09,uVar3) < *(short *)(lVar1 + 0xd0) &&
                  (uVar3 = FUN_1401ab420(param_2 + 0xba,*(undefined4 *)(param_2 + 0xbe)),
                  CONCAT11(extraout_AH_10,uVar3) < *(short *)(lVar1 + 0xd2))) &&
                 ((uVar3 = FUN_1401ab420(param_2 + 0xc2,*(undefined4 *)(param_2 + 0xc6)),
                  CONCAT11(extraout_AH_11,uVar3) < *(short *)(lVar1 + 0xd4) &&
                  ((((uVar3 = FUN_1401ab420(param_2 + 0xca,*(undefined4 *)(param_2 + 0xce)),
                     CONCAT11(extraout_AH_12,uVar3) < *(short *)(lVar1 + 0xd6) &&
                     (uVar3 = FUN_1401ab420(param_2 + 0xd2,*(undefined4 *)(param_2 + 0xd6)),
                     CONCAT11(extraout_AH_13,uVar3) < *(short *)(lVar1 + 0xd8))) &&
                    (uVar3 = FUN_1401ab420(param_2 + 0xda,*(undefined4 *)(param_2 + 0xde)),
                    CONCAT11(extraout_AH_14,uVar3) < *(short *)(lVar1 + 0xda))) &&
                   ((uVar3 = FUN_1401ab420(param_2 + 0x92,*(undefined4 *)(param_2 + 0x96)),
                    CONCAT11(extraout_AH_15,uVar3) < *(short *)(lVar1 + 0xdc) &&
                    (uVar3 = FUN_1401ab420(param_2 + 0x9a,*(undefined4 *)(param_2 + 0x9e)),
                    CONCAT11(extraout_AH_16,uVar3) < *(short *)(lVar1 + 0xde))))))))))))))) {
        FUN_14038d180(DAT_143aa8328,param_2);
        if (((CONCAT22(extraout_var_01,CONCAT11(extraout_AH_17,extraout_AL_01)) &
             CONCAT22(extraout_var_01,CONCAT11(extraout_AH_17,extraout_AL_01))) != 0) &&
           (cVar4 = FUN_1401b0050(param_2 + 0x11a,*(undefined4 *)(param_2 + 0x11e)), cVar4 != '\0'))
        {
          return 1;
        }
        FUN_1401b0050(param_2 + 0x112,*(undefined4 *)(param_2 + 0x116));
        return 1;
      }
      return 1;
    }
  }
  return 0;
}



//===========================================================
// FUN_1426e4340 @ 1426e4340   (1869 bytes)
//===========================================================

void FUN_1426e4340(longlong param_1,int param_2,uint param_3)

{
  char cVar1;
  byte bVar2;
  short sVar3;
  undefined2 uVar4;
  int iVar5;
  undefined4 uVar6;
  int *piVar7;
  longlong lVar8;
  longlong lVar9;
  int iVar10;
  byte bVar11;
  ulonglong uVar12;
  longlong *local_res20;
  int *local_88;
  longlong **local_80;
  longlong *local_78;
  longlong *local_70;
  longlong *local_68;
  undefined1 local_60 [8];
  longlong *local_58;
  longlong *local_50;
  longlong *local_48;
  longlong *local_40;
  longlong *local_38;
  longlong *local_30;
  
  uVar12 = (ulonglong)param_3;
  if ((((*(longlong *)(param_1 + 8) != 0) && (param_2 != 0)) && (param_3 != 0)) &&
     (FUN_14039e630(DAT_143aa8328,&local_78,param_2), local_78 != (longlong *)0x0)) {
    local_88 = (int *)0x0;
    piVar7 = (int *)FUN_14019b600(&DAT_143ad6a30,0x15);
    piVar7[1] = 4;
    *piVar7 = -1;
    local_88 = piVar7 + 4;
    piVar7[2] = 0;
    *(undefined1 *)local_88 = 0;
    *local_88 = DAT_143273538;
    if (*piVar7 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar7[1] < 4) {
      FUN_142e54290(0x90,piVar7[1],4);
    }
    *piVar7 = 1;
    *(undefined1 *)(local_88 + 1) = 0;
    if (piVar7[1] + 1 < 5) {
      FUN_142e54290(0x9c,4);
    }
    piVar7[2] = 4;
    local_68 = local_78;
    if (local_78 != (longlong *)0x0) {
      (**(code **)(*local_78 + 8))();
    }
    FUN_14090f030(&local_res20,&local_68,&local_88);
    if (local_88 != (int *)0x0) {
      FUN_14019f2c0(local_88 + -4);
    }
    if (local_res20 != (longlong *)0x0) {
      if (0 < (int)param_3) {
        do {
          lVar8 = *(longlong *)(param_1 + 8);
          if (lVar8 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar8 = *(longlong *)(param_1 + 8);
          }
          local_80 = (longlong **)local_60;
          lVar9 = lVar8;
          if (local_res20 != (longlong *)0x0) {
            (**(code **)(*local_res20 + 8))();
            lVar9 = *(longlong *)(param_1 + 8);
          }
          if (lVar9 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar9 = *(longlong *)(param_1 + 8);
          }
          sVar3 = FUN_1401ab420(lVar9 + 0x62,*(undefined4 *)(lVar9 + 0x66));
          iVar5 = FUN_140910ca0(local_60,"incSTR",0);
          iVar10 = 0;
          if (0 < iVar5 + sVar3) {
            iVar10 = iVar5 + sVar3;
          }
          uVar4 = 999;
          if (iVar10 < 999) {
            uVar4 = (short)iVar10;
          }
          uVar6 = FUN_1402f7010(uVar4,lVar8 + 0x62);
          *(undefined4 *)(lVar8 + 0x66) = uVar6;
          lVar8 = *(longlong *)(param_1 + 8);
          if (lVar8 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar8 = *(longlong *)(param_1 + 8);
          }
          local_80 = &local_58;
          local_58 = local_res20;
          lVar9 = lVar8;
          if (local_res20 != (longlong *)0x0) {
            (**(code **)(*local_res20 + 8))();
            lVar9 = *(longlong *)(param_1 + 8);
          }
          if (lVar9 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar9 = *(longlong *)(param_1 + 8);
          }
          sVar3 = FUN_1401ab420(lVar9 + 0x6a,*(undefined4 *)(lVar9 + 0x6e));
          iVar5 = FUN_140910ca0(&local_58,&DAT_1432726f4,0);
          iVar10 = 0;
          if (0 < iVar5 + sVar3) {
            iVar10 = iVar5 + sVar3;
          }
          uVar4 = 999;
          if (iVar10 < 999) {
            uVar4 = (short)iVar10;
          }
          uVar6 = FUN_1402f7010(uVar4,lVar8 + 0x6a);
          *(undefined4 *)(lVar8 + 0x6e) = uVar6;
          lVar8 = *(longlong *)(param_1 + 8);
          if (lVar8 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar8 = *(longlong *)(param_1 + 8);
          }
          local_80 = &local_50;
          local_50 = local_res20;
          lVar9 = lVar8;
          if (local_res20 != (longlong *)0x0) {
            (**(code **)(*local_res20 + 8))();
            lVar9 = *(longlong *)(param_1 + 8);
          }
          if (lVar9 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar9 = *(longlong *)(param_1 + 8);
          }
          sVar3 = FUN_1401ab420(lVar9 + 0x72,*(undefined4 *)(lVar9 + 0x76));
          iVar5 = FUN_140910ca0(&local_50,"incINT",0);
          iVar10 = 0;
          if (0 < iVar5 + sVar3) {
            iVar10 = iVar5 + sVar3;
          }
          uVar4 = 999;
          if (iVar10 < 999) {
            uVar4 = (short)iVar10;
          }
          uVar6 = FUN_1402f7010(uVar4,lVar8 + 0x72);
          *(undefined4 *)(lVar8 + 0x76) = uVar6;
          lVar8 = *(longlong *)(param_1 + 8);
          if (lVar8 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar8 = *(longlong *)(param_1 + 8);
          }
          local_80 = &local_48;
          local_48 = local_res20;
          lVar9 = lVar8;
          if (local_res20 != (longlong *)0x0) {
            (**(code **)(*local_res20 + 8))();
            lVar9 = *(longlong *)(param_1 + 8);
          }
          if (lVar9 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar9 = *(longlong *)(param_1 + 8);
          }
          sVar3 = FUN_1401ab420(lVar9 + 0x7a,*(undefined4 *)(lVar9 + 0x7e));
          iVar5 = FUN_140910ca0(&local_48,&DAT_143272704,0);
          iVar10 = 0;
          if (0 < iVar5 + sVar3) {
            iVar10 = iVar5 + sVar3;
          }
          uVar4 = 999;
          if (iVar10 < 999) {
            uVar4 = (short)iVar10;
          }
          uVar6 = FUN_1402f7010(uVar4,lVar8 + 0x7a);
          *(undefined4 *)(lVar8 + 0x7e) = uVar6;
          lVar8 = *(longlong *)(param_1 + 8);
          if (lVar8 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar8 = *(longlong *)(param_1 + 8);
          }
          local_80 = &local_40;
          local_40 = local_res20;
          lVar9 = lVar8;
          if (local_res20 != (longlong *)0x0) {
            (**(code **)(*local_res20 + 8))();
            lVar9 = *(longlong *)(param_1 + 8);
          }
          if (lVar9 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar9 = *(longlong *)(param_1 + 8);
          }
          sVar3 = FUN_1401ab420(lVar9 + 0xa2,*(undefined4 *)(lVar9 + 0xa6));
          iVar5 = FUN_140910ca0(&local_40,&DAT_14327270c,0);
          iVar10 = 0;
          if (0 < iVar5 + sVar3) {
            iVar10 = iVar5 + sVar3;
          }
          uVar4 = 1999;
          if (iVar10 < 1999) {
            uVar4 = (short)iVar10;
          }
          uVar6 = FUN_1402f7010(uVar4,lVar8 + 0xa2);
          *(undefined4 *)(lVar8 + 0xa6) = uVar6;
          lVar8 = *(longlong *)(param_1 + 8);
          if (lVar8 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar8 = *(longlong *)(param_1 + 8);
          }
          local_80 = &local_38;
          local_38 = local_res20;
          lVar9 = lVar8;
          if (local_res20 != (longlong *)0x0) {
            (**(code **)(*local_res20 + 8))();
            lVar9 = *(longlong *)(param_1 + 8);
          }
          if (lVar9 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar9 = *(longlong *)(param_1 + 8);
          }
          sVar3 = FUN_1401ab420(lVar9 + 0xaa,*(undefined4 *)(lVar9 + 0xae));
          iVar5 = FUN_140910ca0(&local_38,&DAT_143272714,0);
          iVar10 = 0;
          if (0 < iVar5 + sVar3) {
            iVar10 = iVar5 + sVar3;
          }
          uVar4 = 1999;
          if (iVar10 < 1999) {
            uVar4 = (short)iVar10;
          }
          uVar6 = FUN_1402f7010(uVar4,lVar8 + 0xaa);
          *(undefined4 *)(lVar8 + 0xae) = uVar6;
          lVar8 = *(longlong *)(param_1 + 8);
          if (lVar8 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar8 = *(longlong *)(param_1 + 8);
          }
          local_80 = &local_30;
          local_30 = local_res20;
          lVar9 = lVar8;
          if (local_res20 != (longlong *)0x0) {
            (**(code **)(*local_res20 + 8))();
            lVar9 = *(longlong *)(param_1 + 8);
          }
          if (lVar9 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar9 = *(longlong *)(param_1 + 8);
          }
          sVar3 = FUN_1401ab420(lVar9 + 0x82,*(undefined4 *)(lVar9 + 0x86));
          iVar5 = FUN_140910ca0(&local_30,&DAT_143272844,0);
          iVar10 = 0;
          if (0 < iVar5 + sVar3) {
            iVar10 = iVar5 + sVar3;
          }
          uVar4 = 30000;
          if (iVar10 < 30000) {
            uVar4 = (short)iVar10;
          }
          uVar6 = FUN_1402f7010(uVar4,lVar8 + 0x82);
          *(undefined4 *)(lVar8 + 0x86) = uVar6;
          lVar8 = *(longlong *)(param_1 + 8);
          if (lVar8 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar8 = *(longlong *)(param_1 + 8);
          }
          local_80 = &local_70;
          local_70 = local_res20;
          lVar9 = lVar8;
          if (local_res20 != (longlong *)0x0) {
            (**(code **)(*local_res20 + 8))();
            lVar9 = *(longlong *)(param_1 + 8);
          }
          if (lVar9 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar9 = *(longlong *)(param_1 + 8);
          }
          sVar3 = FUN_1401ab420(lVar9 + 0x8a,*(undefined4 *)(lVar9 + 0x8e));
          iVar5 = FUN_140910ca0(&local_70,&DAT_14327284c,0);
          iVar10 = 0;
          if (0 < iVar5 + sVar3) {
            iVar10 = iVar5 + sVar3;
          }
          uVar4 = 30000;
          if (iVar10 < 30000) {
            uVar4 = (short)iVar10;
          }
          uVar6 = FUN_1402f7010(uVar4,lVar8 + 0x8a);
          *(undefined4 *)(lVar8 + 0x8e) = uVar6;
          lVar8 = *(longlong *)(param_1 + 8);
          if (lVar8 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar8 = *(longlong *)(param_1 + 8);
          }
          cVar1 = FUN_1401b0050(lVar8 + 0x102,*(undefined4 *)(lVar8 + 0x106));
          bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
          *(byte *)(lVar8 + 0x102) = bVar2;
          bVar11 = cVar1 + 1U ^ bVar2;
          *(byte *)(lVar8 + 0x103) = bVar11;
          *(uint *)(lVar8 + 0x106) =
               ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)bVar11;
          lVar8 = *(longlong *)(param_1 + 8);
          if (lVar8 == 0) {
            FUN_142e52ed0(0x431,0);
            lVar8 = *(longlong *)(param_1 + 8);
          }
          cVar1 = FUN_1401b0050(lVar8 + 0xfa);
          bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
          *(byte *)(lVar8 + 0xfa) = bVar2;
          bVar11 = cVar1 - 1U ^ bVar2;
          *(byte *)(lVar8 + 0xfb) = bVar11;
          *(uint *)(lVar8 + 0xfe) =
               ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)bVar11;
          uVar12 = uVar12 - 1;
        } while (uVar12 != 0);
      }
      if (local_res20 != (longlong *)0x0) {
        (**(code **)(*local_res20 + 0x10))();
      }
    }
    if (local_78 != (longlong *)0x0) {
      (**(code **)(*local_78 + 0x10))();
    }
  }
  FUN_1402325e0(param_1);
  return;
}



//===========================================================
// FUN_1417e9fc0 @ 1417e9fc0   (1192 bytes)
//===========================================================

/* WARNING: Type propagation algorithm not settling */

undefined8 FUN_1417e9fc0(longlong *param_1,longlong *param_2)

{
  bool bVar1;
  IUnknown *pIVar2;
  char cVar3;
  byte bVar4;
  int iVar5;
  int iVar6;
  undefined4 uVar7;
  longlong *plVar8;
  longlong *plVar9;
  longlong lVar10;
  longlong **pplVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  bool bVar15;
  longlong *local_res8;
  int local_res18 [2];
  longlong *local_res20;
  int local_78;
  IUnknown *local_70;
  uint local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  uint local_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined8 local_40;
  
  if (param_1 == (longlong *)0x0) {
    return 0;
  }
  if (param_2 == (longlong *)0x0) {
    return 0;
  }
  local_res8 = param_1;
  iVar5 = FUN_14019a5d0(param_1 + 4);
  plVar8 = (longlong *)FUN_140192f00(param_2);
  iVar6 = (**(code **)(*param_2 + 0x10))(param_2);
  if (iVar6 != 0) {
    uVar12 = 0x96c;
LAB_1417ea018:
    uVar12 = FUN_1408a9e40(&local_res8,uVar12);
    FUN_142a26280(uVar12,0,0,1,0,0,0,0,0,0);
    return 0;
  }
  cVar3 = (**(code **)(*param_2 + 0x310))(param_2);
  if (cVar3 != '\0') {
    uVar12 = 0x973;
    goto LAB_1417ea018;
  }
  if (((((iVar5 - 0x4d54e0U < 1000) && (99 < iVar5 % 1000 - 300U)) || (iVar5 == 0x269ad3)) ||
      ((iVar5 == 0x26a2a1 || (iVar5 == 0x26a689)))) &&
     (iVar6 = FUN_14019a5d0(plVar8 + 4), 99999 < iVar6 - 1800000U)) {
    uVar12 = 0xf09;
    goto LAB_1417ea018;
  }
  FUN_14039f600(DAT_143aa8328,&local_70,iVar5,0);
  pIVar2 = local_70;
  if (local_70 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  plVar9 = (longlong *)FUN_1408a9d80(local_res18,0x634);
  local_res20 = plVar9;
  (*DAT_143262a20)(&local_50);
  uVar14 = 0;
  uVar12 = uVar14;
  if ((undefined8 *)*plVar9 != (undefined8 *)0x0) {
    uVar12 = *(undefined8 *)*plVar9;
  }
  iVar6 = (**(code **)(*(longlong *)pIVar2 + 0x28))(pIVar2,uVar12,&local_50);
  if (iVar6 < 0) {
    _com_issue_errorex(iVar6,pIVar2,(_GUID *)&DAT_143272478);
  }
  local_68 = local_50;
  uStack_64 = uStack_4c;
  uStack_60 = uStack_48;
  uStack_5c = uStack_44;
  local_58 = local_40;
  local_50 = local_50 & 0xffff0000;
  thunk_FUN_1401be120(plVar9);
  iVar6 = FUN_14022ee40(&local_68,0);
  if ((short)local_68 == 8) {
    local_68 = local_68 & 0xffff0000;
    if (CONCAT44(uStack_5c,uStack_60) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_5c,uStack_60) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_68);
  }
  uVar12 = DAT_143aa8328;
  uVar7 = FUN_14019a5d0(plVar8 + 4);
  lVar10 = FUN_140388c60(uVar12,uVar7);
  bVar1 = false;
  uVar12 = uVar14;
  if (lVar10 == 0) goto LAB_1417ea447;
  if ((iVar6 == 0) ||
     (bVar4 = FUN_1401b0050((longlong)plVar8 + 0x16e,*(undefined4 *)((longlong)plVar8 + 0x172)),
     (int)((uint)bVar4 + *(int *)(lVar10 + 0x78)) <= iVar6)) {
    iVar6 = FUN_140417ed0(iVar5);
    bVar15 = iVar6 == 0x3d;
    iVar6 = FUN_1417ea650(iVar5);
    uVar13 = 1;
    if ((iVar6 != 0) ||
       (local_res20 = (longlong *)((ulonglong)local_res20 & 0xffffffff00000000), bVar15)) {
      local_res20 = (longlong *)CONCAT44(local_res20._4_4_,1);
    }
    iVar6 = FUN_1417ea760(iVar5);
    if ((iVar6 != 0) || (bVar15)) {
      local_res18[0] = FUN_1417ea7a0(iVar5);
      local_78 = FUN_1417ea7e0(iVar5);
      bVar1 = true;
      iVar6 = local_res18[0];
      if ((!bVar15) ||
         ((iVar5 = (**(code **)(*plVar8 + 0x58))(plVar8), iVar5 == 0 &&
          (iVar5 = (**(code **)(*plVar8 + 0x60))(plVar8), iVar6 = local_res18[0], iVar5 == 0))))
      goto LAB_1417ea2bf;
      uVar12 = 0xf0f;
      pplVar11 = &local_res8;
      goto LAB_1417ea418;
    }
    iVar6 = FUN_1417ea7a0(iVar5);
    local_78 = FUN_1417ea7e0(iVar5);
LAB_1417ea2bf:
    if (((int)local_res20 != 0) && (iVar5 = FUN_1417e9ab0(local_res8,param_2), iVar5 == 0))
    goto LAB_1417ea447;
    if (bVar1) {
      iVar5 = (**(code **)(*plVar8 + 0x60))(plVar8);
      if (iVar5 == 0) {
        cVar3 = FUN_1401b0050((longlong)plVar8 + 0xfa,*(undefined4 *)((longlong)plVar8 + 0xfe));
        if (cVar3 != '\0') goto LAB_1417ea328;
        uVar12 = 0xf10;
        pplVar11 = (longlong **)local_res18;
      }
      else {
        uVar12 = 0xf0e;
        pplVar11 = &local_res8;
      }
    }
    else {
LAB_1417ea328:
      if (iVar6 == 0) {
LAB_1417ea399:
        uVar12 = uVar13;
        if (local_78 == 0) goto LAB_1417ea447;
        iVar5 = (**(code **)(*plVar8 + 0x78))(plVar8);
        if (iVar5 == 0) {
          iVar5 = FUN_14019a5d0(plVar8 + 4);
          if ((99999 < iVar5 - 1800000U) ||
             (cVar3 = FUN_1401b0050((longlong)plVar8 + 0xfa,*(undefined4 *)((longlong)plVar8 + 0xfe)
                                   ), cVar3 != '\0')) {
            uVar13 = DAT_143aa8328;
            uVar7 = FUN_14019a5d0(plVar8 + 4);
            lVar10 = FUN_140388c60(uVar13,uVar7);
            if ((lVar10 == 0) || (*(char *)(lVar10 + 0xb8) != '\0')) goto LAB_1417ea447;
            goto LAB_1417ea40f;
          }
          pplVar11 = (longlong **)local_res18;
LAB_1417ea413:
          uVar12 = 0xb26;
        }
        else {
          uVar12 = 0xf0d;
          pplVar11 = &local_res8;
        }
      }
      else {
        iVar5 = (**(code **)(*plVar8 + 0x70))(plVar8);
        if (iVar5 == 0) {
          cVar3 = FUN_1401b0050((longlong)plVar8 + 0xfa,*(undefined4 *)((longlong)plVar8 + 0xfe));
          uVar12 = DAT_143aa8328;
          if (cVar3 != '\0') {
            uVar7 = FUN_14019a5d0(plVar8 + 4);
            lVar10 = FUN_140388c60(uVar12,uVar7);
            if ((lVar10 == 0) || (*(char *)(lVar10 + 0xb8) != '\0')) goto LAB_1417ea399;
LAB_1417ea40f:
            pplVar11 = &local_res20;
            goto LAB_1417ea413;
          }
          uVar12 = 0xf10;
          pplVar11 = (longlong **)local_res18;
        }
        else {
          uVar12 = 0xf0d;
          pplVar11 = &local_res8;
        }
      }
    }
LAB_1417ea418:
    uVar12 = FUN_1408a9e40(pplVar11,uVar12);
  }
  else {
    uVar12 = FUN_1408a9e40(&local_res8,0x1007);
  }
  FUN_142a26280(uVar12,0,0,1,0,0,0,0,0,0);
  uVar12 = uVar14;
LAB_1417ea447:
  if (local_70 != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_70 + 0x10))();
    return uVar12;
  }
  return uVar12;
}


