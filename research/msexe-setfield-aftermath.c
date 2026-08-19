
//===========================================================
// FUN_140ce89c0 @ 140ce89c0   (113 bytes)
//===========================================================

void FUN_140ce89c0(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 *puVar3;
  
  lVar2 = *(longlong *)(param_1 + 8);
  if (lVar2 != 0) {
    if (0xffffe < *(longlong *)(lVar2 + 0x28) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + 0x28);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar2 == 1) {
      puVar3 = (undefined8 *)(*(longlong *)(param_1 + 8) + 0x20);
      if (*(longlong *)(param_1 + 8) == 0) {
        puVar3 = (undefined8 *)0x0;
      }
      if (puVar3 != (undefined8 *)0x0) {
        (**(code **)*puVar3)(puVar3,1);
      }
    }
    *(undefined8 *)(param_1 + 8) = 0;
  }
  return;
}



//===========================================================
// FUN_142caa4e0 @ 142caa4e0   (7069 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x000142caa9d4) */

void FUN_142caa4e0(longlong *param_1)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong lVar3;
  IUnknown *pIVar4;
  char cVar5;
  undefined4 uVar6;
  int iVar7;
  long lVar8;
  longlong lVar9;
  undefined8 uVar10;
  int *piVar11;
  undefined8 *puVar12;
  longlong *plVar13;
  longlong *plVar14;
  longlong *plVar15;
  uint uVar16;
  longlong *plVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  int iVar20;
  ulonglong uVar21;
  undefined1 auStack_c08 [32];
  undefined8 *local_be8;
  undefined8 local_be0;
  longlong *local_bd8;
  longlong **local_bd0;
  longlong *local_bc8 [2];
  undefined4 local_bb8;
  undefined4 uStack_bb4;
  undefined8 uStack_bb0;
  undefined8 local_ba8;
  short local_ba0;
  undefined6 uStack_b9e;
  longlong lStack_b98;
  undefined8 local_b90;
  undefined4 local_b88;
  undefined4 uStack_b84;
  undefined8 uStack_b80;
  undefined8 local_b78;
  short local_b70;
  undefined6 uStack_b6e;
  longlong lStack_b68;
  undefined8 local_b60;
  undefined4 local_b58;
  undefined4 uStack_b54;
  undefined8 uStack_b50;
  undefined8 local_b48;
  short local_b40;
  undefined6 uStack_b3e;
  longlong lStack_b38;
  undefined8 local_b30;
  longlong *local_b28;
  short local_b20 [4];
  longlong lStack_b18;
  undefined8 local_b10;
  short local_b08;
  undefined6 uStack_b06;
  undefined4 uStack_b00;
  undefined4 uStack_afc;
  undefined8 local_af8;
  short local_af0;
  undefined6 uStack_aee;
  undefined4 uStack_ae8;
  undefined4 uStack_ae4;
  undefined8 local_ae0;
  short local_ad8;
  undefined6 uStack_ad6;
  undefined4 uStack_ad0;
  undefined4 uStack_acc;
  undefined8 local_ac8;
  uint local_ac0;
  undefined4 uStack_abc;
  undefined4 uStack_ab8;
  undefined4 uStack_ab4;
  undefined8 local_ab0;
  short local_aa8;
  undefined6 uStack_aa6;
  undefined4 uStack_aa0;
  undefined4 uStack_a9c;
  undefined8 local_a98;
  undefined1 local_a90 [8];
  longlong *local_a88;
  undefined1 local_a80 [8];
  longlong *local_a78;
  undefined1 local_a70 [8];
  longlong *local_a68;
  undefined1 local_a60 [8];
  undefined8 local_a58;
  longlong lStack_a50;
  undefined8 local_a48;
  undefined8 local_a38;
  undefined8 uStack_a30;
  undefined8 local_a28;
  uint local_a18;
  undefined4 uStack_a14;
  undefined4 uStack_a10;
  undefined4 uStack_a0c;
  undefined8 local_a08;
  undefined8 local_9f8;
  longlong lStack_9f0;
  undefined8 local_9e8;
  undefined8 local_9d8;
  undefined8 uStack_9d0;
  undefined8 local_9c8;
  uint local_9b8;
  undefined4 uStack_9b4;
  undefined4 uStack_9b0;
  undefined4 uStack_9ac;
  undefined8 local_9a8;
  undefined8 local_998;
  longlong lStack_990;
  undefined8 local_988;
  undefined8 local_978;
  undefined8 uStack_970;
  undefined8 local_968;
  uint local_958;
  undefined4 uStack_954;
  undefined4 uStack_950;
  undefined4 uStack_94c;
  undefined8 local_948;
  undefined8 local_938;
  longlong lStack_930;
  undefined8 local_928;
  undefined8 local_918;
  undefined8 uStack_910;
  undefined8 local_908;
  uint local_8f8;
  undefined4 uStack_8f4;
  undefined4 uStack_8f0;
  undefined4 uStack_8ec;
  undefined8 local_8e8;
  undefined1 local_8d8 [1104];
  undefined1 local_488 [1104];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_c08;
  FUN_1413b8d80();
  uVar18 = 0;
  *(undefined4 *)param_1[0x47c] = 0;
  *(undefined4 *)((longlong)param_1 + 0xec) = 1;
  FUN_142d33810(param_1 + 0x51d);
  FUN_142d39790(param_1 + 0x533);
  lVar9 = FUN_142d341c0(0);
  param_1[0x534] = lVar9;
  FUN_142d3a2d0(param_1 + 0x51f);
  lVar9 = FUN_142d347d0(0);
  param_1[0x520] = lVar9;
  FUN_142d39970(param_1 + 0x521);
  lVar9 = FUN_142d342f0(0);
  param_1[0x522] = lVar9;
  FUN_142d39530(param_1 + 0x523);
  lVar9 = FUN_142d33e10(0);
  param_1[0x524] = lVar9;
  FUN_142d39650(param_1 + 0x525);
  lVar9 = FUN_142d33f40(0);
  param_1[0x526] = lVar9;
  FUN_142d39270(param_1 + 0x527);
  lVar9 = FUN_142d33b90(0);
  param_1[0x528] = lVar9;
  FUN_142d39490(param_1 + 0x529);
  lVar9 = FUN_142d33ce0(0);
  param_1[0x52a] = lVar9;
  FUN_142d391d0(param_1 + 0x52b);
  lVar9 = FUN_142d33a60(0);
  param_1[0x52c] = lVar9;
  FUN_142d39bc0(param_1 + 0x52d);
  lVar9 = FUN_142d34670(0);
  param_1[0x52e] = lVar9;
  FUN_142d398d0(param_1 + 0x52f);
  lVar9 = FUN_142d25310(0);
  if (*(longlong *)(lVar9 + 8) != 0) {
    FUN_142e541f0(0x2fe);
  }
  *(undefined8 *)(lVar9 + 8) = 1;
  param_1[0x530] = lVar9 + 0x28;
  FUN_142d396f0(param_1 + 0x531);
  lVar9 = FUN_142d34070(0);
  param_1[0x532] = lVar9;
  FUN_142d38ea0(param_1 + 0x478);
  lVar9 = FUN_142d25080(0);
  if (*(longlong *)(lVar9 + 8) != 0) {
    FUN_142e541f0(0x2fe);
  }
  *(undefined8 *)(lVar9 + 8) = 1;
  param_1[0x479] = lVar9 + 0x28;
  FUN_142d39830(param_1 + 0x537);
  lVar9 = FUN_142d251c0(0);
  if (*(longlong *)(lVar9 + 8) != 0) {
    FUN_142e541f0(0x2fe);
  }
  *(undefined8 *)(lVar9 + 8) = 1;
  param_1[0x538] = lVar9 + 0x28;
  lVar9 = param_1[0x470];
  if (lVar9 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar9 = param_1[0x470];
  }
  FUN_14211c9d0(lVar9,0,0);
  lVar9 = param_1[0x470];
  if (lVar9 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar9 = param_1[0x470];
  }
  FUN_142122710(lVar9);
  uVar10 = (**(code **)(*param_1 + 0x30))(param_1);
  FUN_1408674e0(uVar10,0,0,0);
  *(undefined4 *)(param_1 + 0x471) = 0;
  *(undefined4 *)(param_1 + 0x519) = 0;
  FUN_1402bf500(param_1 + 0x5d8);
  FUN_1402bf500(param_1 + 0x5dd);
  FUN_14035d220(param_1 + 0x5db);
  FUN_1402fe5c0(param_1 + 0x5cd);
  FUN_1402bd6a0(param_1 + 0x5ca);
  *(undefined1 *)(param_1 + 0x5c9) = 1;
  FUN_1402bd6a0(param_1 + 0x5d0);
  *(undefined1 *)((longlong)param_1 + 0x2e49) = 1;
  FUN_1402bd6a0(param_1 + 0x5e0);
  FUN_1402ad370(param_1 + 0x5e3);
  *(undefined4 *)(param_1 + 0x5e5) = 0;
  *(undefined4 *)(param_1 + 0x466) = 0;
  uVar6 = FUN_1429e3ef0();
  *(undefined4 *)((longlong)param_1 + 0x2334) = uVar6;
  uVar6 = FUN_1429e3ef0();
  FUN_142e54b20(uVar6);
  uVar6 = FUN_1429e3ef0();
  FUN_142e54f40(uVar6);
  *(undefined1 *)(param_1 + 0x48a) = 0;
  if (param_1[0x594] != 0) {
    FUN_142bf3f70();
    FUN_1418b95b0(param_1 + 0x593);
  }
  if (param_1[0x596] != 0) {
    FUN_142bf3f70();
    FUN_1418b95b0(param_1 + 0x595);
  }
  FUN_142c4a960(DAT_143ac1898);
  FUN_1415ffb40(DAT_143ac87a0,0,0);
  FUN_1415f7910(DAT_143ac87a0,(int)param_1[1099],*(undefined4 *)((longlong)param_1 + 0x232c));
  FUN_1415f83e0(DAT_143ac87a0,(int)param_1[1099],*(undefined4 *)((longlong)param_1 + 0x232c));
  FUN_1401cf690();
  plVar17 = param_1 + 0x51b;
  if (((char *)*plVar17 == (char *)0x0) || (*(char *)*plVar17 == '\0')) {
    plVar17 = param_1 + 0x51a;
  }
  local_bc8[0] = (longlong *)0x0;
  FUN_14019a260(local_bc8,plVar17);
  plVar15 = local_bc8[0];
  local_bd8 = (longlong *)0x0;
  plVar17 = local_bd8;
  if ((local_bc8[0] != (longlong *)0x0) && (plVar1 = local_bc8[0] + -2, plVar1 != (longlong *)0x0))
  {
    if ((int)*plVar1 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar21 = 0xffffffffffffffff;
      do {
        uVar21 = uVar21 + 1;
      } while (*(char *)((longlong)plVar15 + uVar21) != '\0');
      iVar20 = (int)uVar21;
      iVar7 = 0;
      if (0 < iVar20) {
        iVar7 = iVar20;
      }
      piVar11 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar7 + 0x11));
      piVar11[1] = iVar7;
      *piVar11 = -1;
      plVar17 = (longlong *)(piVar11 + 4);
      piVar11[2] = 0;
      *(undefined1 *)plVar17 = 0;
      local_b28 = plVar17;
      FUN_142ef7ba0(plVar17,local_bc8[0],(longlong)iVar20);
      if (*piVar11 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar20 == -1) || (iVar20 <= piVar11[1])) {
        *piVar11 = 1;
        if (iVar20 != -1) goto LAB_142caa990;
        uVar21 = uVar18;
        if (plVar17 != (longlong *)0x0) {
          uVar21 = 0xffffffffffffffff;
          do {
            uVar21 = uVar21 + 1;
          } while (*(char *)((longlong)plVar17 + uVar21) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar11[1],uVar21 & 0xffffffff);
        *piVar11 = 1;
LAB_142caa990:
        *(undefined1 *)((longlong)plVar17 + (longlong)iVar20) = 0;
      }
      iVar7 = (int)uVar21;
      if ((iVar7 < 0) || (piVar11[1] + 1 <= iVar7)) {
        FUN_142e54290(0x9c,uVar21 & 0xffffffff);
      }
      piVar11[2] = iVar7;
      if (local_bd8 != (longlong *)0x0) {
        FUN_14019f2c0(local_bd8 + -2);
      }
    }
    else {
      if ((int)*plVar1 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *(int *)plVar1 = (int)*plVar1 + 1;
      UNLOCK();
      if (local_bd8 != (longlong *)0x0) {
        FUN_14019f2c0(local_bd8 + -2);
      }
      local_bd8 = plVar15;
      plVar17 = local_bd8;
    }
  }
  local_bd8 = plVar17;
  local_bd0 = &local_bd8;
  FUN_14019a260(param_1 + 0x51a,&local_bd8);
  if (local_bd8 != (longlong *)0x0) {
    FUN_14019f2c0(local_bd8 + -2);
  }
  if (DAT_143acf088 == 0) {
    local_bd0 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x630);
    if (local_bd0 != (longlong **)0x0) {
      FUN_1424655b0(local_bd0,param_1);
    }
  }
  FUN_142ce97e0(param_1,1);
  if ((DAT_143ad30d8 == 0) &&
     (local_bd0 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x28), local_bd0 != (longlong **)0x0)) {
    FUN_142571a60(local_bd0);
  }
  if ((((DAT_143ac87a0 != 0) && (iVar7 = FUN_1415febe0(), iVar7 != 0)) && (DAT_143ad20e8 == 0)) &&
     (local_bd0 = (longlong **)FUN_14019b780(&DAT_143ad68a0,1000), local_bd0 != (longlong **)0x0)) {
    FUN_1424e4490(local_bd0);
  }
  if (param_1[0x53a] == 0) {
    FUN_142d3a0e0(param_1 + 0x539);
    local_bd0 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0xd8);
    uVar21 = uVar18;
    if (local_bd0 != (longlong **)0x0) {
      uVar21 = FUN_14250ffa0(local_bd0);
    }
    if (*(longlong *)(uVar21 + 8) != 0) {
      FUN_142e541f0(0x2fe);
    }
    *(undefined8 *)(uVar21 + 8) = 1;
    param_1[0x53a] = uVar21;
  }
  FUN_1402fe4a0(&DAT_143a8a980);
  if (DAT_143ad1850 != 0) {
    FUN_141a3ccd0();
  }
  local_bd0 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x518);
  uVar21 = uVar18;
  if (local_bd0 != (longlong **)0x0) {
    uVar21 = FUN_141e28c40(local_bd0);
  }
  if ((param_1[0x540] - 1U < 999) || (param_1[0x540] == -1)) {
    FUN_142e52ed0(0x447);
  }
  uVar19 = uVar21 + 0x18;
  if (uVar21 == 0) {
    uVar19 = uVar18;
  }
  uVar21 = uVar18;
  if ((uVar19 != 0) && (uVar21 = uVar19 - 0x18, uVar21 != 0)) {
    if (0xfffff < *(ulonglong *)(uVar19 + 8)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(uVar19 + 8) = *(longlong *)(uVar19 + 8) + 1;
    UNLOCK();
  }
  lVar9 = param_1[0x540];
  param_1[0x540] = uVar21;
  if (lVar9 != 0) {
    if (0xffffe < *(longlong *)(lVar9 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar17 = (longlong *)(lVar9 + 0x20);
    lVar3 = *plVar17;
    *plVar17 = *plVar17 + -1;
    UNLOCK();
    if (((int)lVar3 == 1) && (puVar12 = (undefined8 *)(lVar9 + 0x18), puVar12 != (undefined8 *)0x0))
    {
      (**(code **)*puVar12)(puVar12,1);
    }
  }
  FUN_141f6f200(DAT_143acedb0);
  FUN_142d47e20(param_1);
  FUN_142d487a0(param_1);
  *(undefined4 *)(param_1 + 0x70d) = 1;
  iVar7 = (*DAT_143262db0)();
  *(undefined4 *)(param_1 + 0x466) = 0;
  uVar6 = FUN_1429e3ef0();
  *(undefined4 *)((longlong)param_1 + 0x2334) = uVar6;
  uVar6 = FUN_1429e3ef0();
  FUN_142e54b20(uVar6);
  uVar6 = FUN_1429e3ef0();
  FUN_142e54f40(uVar6);
  *(int *)(param_1 + 0x50b) = iVar7 + -300000;
  *(int *)(param_1 + 0x5fd) = iVar7;
  FUN_1402fe4a0(param_1 + 0x5be);
  param_1[0x5c7] = 0;
  *(undefined4 *)(param_1 + 0x5c8) = 0;
  *(undefined4 *)((longlong)param_1 + 0x3bbc) = 1;
  *(undefined1 *)(param_1 + 0x779) = 0;
  *(undefined4 *)((longlong)param_1 + 0x3bcc) = 0;
  FUN_142d16ca0(param_1);
  param_1[0x78f] = 0;
  plVar17 = (longlong *)param_1[0x790];
  param_1[0x790] = 0;
  if (plVar17 != (longlong *)0x0) {
    LOCK();
    plVar15 = plVar17 + 1;
    lVar9 = *plVar15;
    *(int *)plVar15 = (int)*plVar15 + -1;
    UNLOCK();
    if ((int)lVar9 == 1) {
      (**(code **)*plVar17)(plVar17);
      LOCK();
      piVar11 = (int *)((longlong)plVar17 + 0xc);
      iVar7 = *piVar11;
      *piVar11 = *piVar11 + -1;
      UNLOCK();
      if (iVar7 == 1) {
        (**(code **)(*plVar17 + 8))(plVar17);
      }
    }
  }
  iVar7 = FUN_1413b8e00();
  if ((((iVar7 != 0) && (DAT_143aca960 == 0)) && (cVar5 = FUN_142dec800(param_1), cVar5 != '\0')) &&
     (local_bd0 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x378), local_bd0 != (longlong **)0x0))
  {
    FUN_14118ace0(local_bd0,0);
  }
  uVar6 = FUN_1429e3ef0();
  *(undefined4 *)((longlong)param_1 + 0x3244) = uVar6;
  *(undefined4 *)(param_1 + 0x648) = 1;
  FUN_1406ed520(local_488,0x238);
  FUN_1415d01c0(local_488);
  FUN_1406ed520(local_8d8,0x24d);
  FUN_1415d01c0(local_8d8);
  uVar6 = (*DAT_143262db0)();
  *(undefined4 *)((longlong)param_1 + 0x34ec) = uVar6;
  iVar7 = FUN_1429e3ef0();
  *(int *)(param_1 + 0x6a9) = iVar7 + -600000;
  *(undefined1 *)(param_1 + 0x6aa) = 0;
  *(undefined8 *)((longlong)param_1 + 0x303c) = 0;
  *(undefined8 *)((longlong)param_1 + 0x3044) = 0;
  if ((DAT_143ac8b88 == 0) &&
     (local_bd0 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x48), local_bd0 != (longlong **)0x0)) {
    FUN_140e7b170(local_bd0);
  }
  if ((DAT_143ad7298 == 0) &&
     (local_bd0 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x118), local_bd0 != (longlong **)0x0))
  {
    FUN_14216ca60(local_bd0);
  }
  if ((DAT_143ade758 == (undefined8 *)0x0) &&
     (local_bd0 = (longlong **)FUN_14019b780(&DAT_143ad68a0,0x30), local_bd0 != (longlong **)0x0)) {
    *local_bd0 = (longlong *)0x0;
    local_bd0[1] = (longlong *)0x0;
    local_bd0[2] = (longlong *)0x0;
    local_bd0[3] = (longlong *)0x0;
    local_bd0[4] = (longlong *)0x0;
    local_bd0[5] = (longlong *)0x0;
    DAT_143ade758 = local_bd0;
    *(undefined4 *)local_bd0 = 0;
    plVar17 = (longlong *)(local_bd0 + 1);
    *plVar17 = 0;
    local_bd0[2] = (longlong *)0x0;
    local_bd0[3] = (longlong *)0x0;
    local_bd0[4] = (longlong *)0x0;
    local_bd0[5] = (longlong *)0x0;
    local_bd8 = plVar17;
    puVar12 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x10);
    puVar12[1] = 0;
    *plVar17 = (longlong)puVar12;
    *puVar12 = plVar17;
  }
  FUN_1415aafa0(9);
  FUN_1415ab890(0);
  plVar17 = param_1 + 0x689;
  lVar9 = 2;
  do {
    plVar15 = (longlong *)*plVar17;
    if (plVar15 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
      plVar15 = (longlong *)*plVar17;
    }
    (**(code **)(*plVar15 + 0x30))();
    plVar17 = plVar17 + 3;
    lVar9 = lVar9 + -1;
  } while (lVar9 != 0);
  lVar9 = param_1[0x6d5];
  cVar5 = *(char *)((longlong)*(longlong **)(lVar9 + 8) + 0x19);
  plVar17 = *(longlong **)(lVar9 + 8);
  while (cVar5 == '\0') {
    FUN_142d27330(param_1 + 0x6d5,param_1 + 0x6d5,plVar17[2]);
    plVar15 = (longlong *)*plVar17;
    thunk_FUN_140205820(plVar17,0x28);
    plVar17 = plVar15;
    cVar5 = *(char *)((longlong)plVar15 + 0x19);
  }
  *(longlong *)(lVar9 + 8) = lVar9;
  *(longlong *)lVar9 = lVar9;
  *(longlong *)(lVar9 + 0x10) = lVar9;
  param_1[0x6d6] = 0;
  FUN_141418c20();
  FUN_1411fc3d0();
  FUN_142d3ca60(param_1);
  if (DAT_143ad2038 != 0) {
    FUN_1411f6aa0();
  }
  if ((*(int *)((longlong)param_1 + 0x22ac) != 0) && ((int)param_1[0x824] == 0)) {
    piVar11 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar11[1] = 0;
    *piVar11 = -1;
    piVar11[2] = 0;
    *(undefined1 *)(piVar11 + 4) = 0;
    if (*piVar11 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar11[1] < 0) {
      FUN_142e54290(0x90,piVar11[1],0);
    }
    *piVar11 = 1;
    *(undefined1 *)(piVar11 + 4) = 0;
    if (piVar11[1] + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    piVar11[2] = 0;
    FUN_14019f2c0(piVar11);
  }
  lVar9 = DAT_143ac87a0;
  plVar17 = param_1 + 0x33;
  if (*plVar17 == 0) {
    FUN_140295e30(PTR_u_Canvas_143a479d8,plVar17,0);
    pIVar4 = (IUnknown *)*plVar17;
    if (pIVar4 != (IUnknown *)0x0) {
      (*DAT_143262a20)(&local_ba0);
      if (DAT_143a8b8d8 == 8) {
        if (local_ba0 == 8) {
          local_ba0 = 0;
          if (lStack_b98 != 0) {
            (*DAT_143ad5990)(lStack_b98 + -4);
          }
        }
        else {
          iVar7 = (*DAT_143262a18)(&local_ba0);
          if (iVar7 < 0) goto LAB_142cac014;
        }
        local_ba0 = 8;
        uVar21 = uVar18;
        if (DAT_143a8b8e0 != 0) {
          uVar21 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        lStack_b98 = FUN_1401a5fa0(DAT_143a8b8e0,uVar21);
      }
      else {
        if ((local_ba0 == 8) && (local_ba0 = 0, lStack_b98 != 0)) {
          (*DAT_143ad5990)(lStack_b98 + -4);
        }
        iVar7 = (*DAT_143262a28)(&local_ba0,&DAT_143a8b8d8);
        if (iVar7 < 0) {
LAB_142cac014:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar7);
        }
      }
      local_b08 = 0x16;
      uStack_b00 = 2;
      (*DAT_143262a20)(&local_bb8);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_bb8 == 8) {
          local_bb8 = (uint)local_bb8._2_2_ << 0x10;
          if (uStack_bb0 != 0) {
            (*DAT_143ad5990)(uStack_bb0 + -4);
          }
        }
        else {
          iVar7 = (*DAT_143262a18)(&local_bb8);
          if (iVar7 < 0) goto LAB_142cac01c;
        }
        local_bb8 = CONCAT22(local_bb8._2_2_,8);
        if (DAT_143a8b8e0 != 0) {
          uVar18 = (ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        uStack_bb0 = FUN_1401a5fa0(DAT_143a8b8e0,uVar18);
      }
      else {
        if (((short)local_bb8 == 8) && (local_bb8 = (uint)local_bb8._2_2_ << 0x10, uStack_bb0 != 0))
        {
          (*DAT_143ad5990)(uStack_bb0 + -4);
        }
        iVar7 = (*DAT_143262a28)(&local_bb8,&DAT_143a8b8d8);
        if (iVar7 < 0) {
LAB_142cac01c:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar7);
        }
      }
      local_a58 = CONCAT62(uStack_b9e,local_ba0);
      lStack_a50 = lStack_b98;
      local_a48 = local_b90;
      local_a38 = CONCAT62(uStack_b06,local_b08);
      uStack_a30 = CONCAT44(uStack_afc,uStack_b00);
      local_a28 = local_af8;
      local_a18 = local_bb8;
      uStack_a14 = uStack_bb4;
      uStack_a10 = (undefined4)uStack_bb0;
      uStack_a0c = uStack_bb0._4_4_;
      local_a08 = local_ba8;
      local_be0 = &local_a58;
      local_be8 = &local_a38;
      iVar7 = (**(code **)(*(longlong *)pIVar4 + 0x68))(pIVar4,1,1,&local_a18);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar4,(_GUID *)&DAT_14327ac98);
      }
      if ((short)local_bb8 == 8) {
        local_bb8 = local_bb8 & 0xffff0000;
        if (uStack_bb0 != 0) {
          (*DAT_143ad5990)(uStack_bb0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_bb8);
      }
      if (local_b08 == 8) {
        local_b08 = 0;
        if (CONCAT44(uStack_afc,uStack_b00) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_afc,uStack_b00) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_b08);
      }
      if (local_ba0 == 8) {
        local_ba0 = 0;
        if (lStack_b98 != 0) {
          (*DAT_143ad5990)(lStack_b98 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_ba0);
      }
      pIVar4 = (IUnknown *)*plVar17;
      if (pIVar4 == (IUnknown *)0x0) goto LAB_142cac05f;
      local_be0 = (undefined8 *)CONCAT44(local_be0._4_4_,0xffffff);
      local_be8 = (undefined8 *)CONCAT44(local_be8._4_4_,1);
      iVar7 = (**(code **)(*(longlong *)pIVar4 + 0x170))(pIVar4,0,0,1);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar4,(_GUID *)&DAT_14327ac98);
      }
      pIVar4 = (IUnknown *)*plVar17;
      if (pIVar4 == (IUnknown *)0x0) goto LAB_142cac05f;
      local_be0 = (undefined8 *)
                  (CONCAT44(local_be0._4_4_,((*(int *)(lVar9 + 0x58) * 0xff) / 100) * 0x1000000) |
                  0xffffff);
      local_be8 = (undefined8 *)CONCAT44(local_be8._4_4_,1);
      iVar7 = (**(code **)(*(longlong *)pIVar4 + 0x170))(pIVar4,0,0,1);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar4,(_GUID *)&DAT_14327ac98);
      }
    }
  }
  plVar15 = param_1 + 0x34;
  if (*plVar15 == 0) {
    FUN_140295e30(PTR_u_Canvas_143a479d8,plVar15,0);
    pIVar4 = (IUnknown *)*plVar15;
    if (pIVar4 != (IUnknown *)0x0) {
      (*DAT_143262a20)(&local_b70);
      if (DAT_143a8b8d8 == 8) {
        if (local_b70 == 8) {
          local_b70 = 0;
          if (lStack_b68 != 0) {
            (*DAT_143ad5990)(lStack_b68 + -4);
          }
        }
        else {
          iVar7 = (*DAT_143262a18)(&local_b70);
          if (iVar7 < 0) goto LAB_142cac024;
        }
        local_b70 = 8;
        if (DAT_143a8b8e0 == 0) {
          uVar16 = 0;
        }
        else {
          uVar16 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
        }
        lStack_b68 = FUN_1401a5fa0(DAT_143a8b8e0,uVar16);
      }
      else {
        if ((local_b70 == 8) && (local_b70 = 0, lStack_b68 != 0)) {
          (*DAT_143ad5990)(lStack_b68 + -4);
        }
        iVar7 = (*DAT_143262a28)(&local_b70,&DAT_143a8b8d8);
        if (iVar7 < 0) {
LAB_142cac024:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar7);
        }
      }
      local_af0 = 0x16;
      uStack_ae8 = 2;
      (*DAT_143262a20)(&local_b88);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_b88 == 8) {
          local_b88 = (uint)local_b88._2_2_ << 0x10;
          if (uStack_b80 != 0) {
            (*DAT_143ad5990)(uStack_b80 + -4);
          }
        }
        else {
          iVar7 = (*DAT_143262a18)(&local_b88);
          if (iVar7 < 0) goto LAB_142cac02c;
        }
        local_b88 = CONCAT22(local_b88._2_2_,8);
        if (DAT_143a8b8e0 == 0) {
          uVar16 = 0;
        }
        else {
          uVar16 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
        }
        uStack_b80 = FUN_1401a5fa0(DAT_143a8b8e0,uVar16);
      }
      else {
        if (((short)local_b88 == 8) && (local_b88 = (uint)local_b88._2_2_ << 0x10, uStack_b80 != 0))
        {
          (*DAT_143ad5990)(uStack_b80 + -4);
        }
        iVar7 = (*DAT_143262a28)(&local_b88,&DAT_143a8b8d8);
        if (iVar7 < 0) {
LAB_142cac02c:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar7);
        }
      }
      local_9f8 = CONCAT62(uStack_b6e,local_b70);
      lStack_9f0 = lStack_b68;
      local_9e8 = local_b60;
      local_9d8 = CONCAT62(uStack_aee,local_af0);
      uStack_9d0 = CONCAT44(uStack_ae4,uStack_ae8);
      local_9c8 = local_ae0;
      local_9b8 = local_b88;
      uStack_9b4 = uStack_b84;
      uStack_9b0 = (undefined4)uStack_b80;
      uStack_9ac = uStack_b80._4_4_;
      local_9a8 = local_b78;
      local_be0 = &local_9f8;
      local_be8 = &local_9d8;
      lVar8 = (**(code **)(*(longlong *)pIVar4 + 0x68))(pIVar4,1,1,&local_9b8);
      if (lVar8 < 0) {
        _com_issue_errorex(lVar8,pIVar4,(_GUID *)&DAT_14327ac98);
      }
      if ((short)local_b88 == 8) {
        local_b88 = local_b88 & 0xffff0000;
        if (uStack_b80 != 0) {
          (*DAT_143ad5990)(uStack_b80 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_b88);
      }
      if (local_af0 == 8) {
        local_af0 = 0;
        if (CONCAT44(uStack_ae4,uStack_ae8) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_ae4,uStack_ae8) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_af0);
      }
      if (local_b70 == 8) {
        local_b70 = 0;
        if (lStack_b68 != 0) {
          (*DAT_143ad5990)(lStack_b68 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_b70);
      }
      pIVar4 = (IUnknown *)*plVar15;
      if (pIVar4 == (IUnknown *)0x0) goto LAB_142cac05f;
      local_be0 = (undefined8 *)CONCAT44(local_be0._4_4_,0xffffff);
      local_be8 = (undefined8 *)CONCAT44(local_be8._4_4_,1);
      iVar7 = (**(code **)(*(longlong *)pIVar4 + 0x170))(pIVar4,0,0,1);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar4,(_GUID *)&DAT_14327ac98);
      }
      pIVar4 = (IUnknown *)*plVar15;
      if (pIVar4 == (IUnknown *)0x0) goto LAB_142cac05f;
      local_be0 = (undefined8 *)
                  (CONCAT44(local_be0._4_4_,((*(int *)(lVar9 + 0x5c) * 0xff) / 100) * 0x1000000) |
                  0xffffff);
      local_be8 = (undefined8 *)CONCAT44(local_be8._4_4_,1);
      iVar7 = (**(code **)(*(longlong *)pIVar4 + 0x170))(pIVar4,0,0,1);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar4,(_GUID *)&DAT_14327ac98);
      }
    }
  }
  plVar1 = param_1 + 0x35;
  if (*plVar1 == 0) {
    FUN_140295e30(PTR_u_Canvas_143a479d8,plVar1,0);
    pIVar4 = (IUnknown *)*plVar1;
    if (pIVar4 != (IUnknown *)0x0) {
      (*DAT_143262a20)(&local_b40);
      if (DAT_143a8b8d8 == 8) {
        if (local_b40 == 8) {
          local_b40 = 0;
          if (lStack_b38 != 0) {
            (*DAT_143ad5990)(lStack_b38 + -4);
          }
        }
        else {
          iVar7 = (*DAT_143262a18)(&local_b40);
          if (iVar7 < 0) goto LAB_142cac034;
        }
        local_b40 = 8;
        if (DAT_143a8b8e0 == 0) {
          uVar16 = 0;
        }
        else {
          uVar16 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
        }
        lStack_b38 = FUN_1401a5fa0(DAT_143a8b8e0,uVar16);
      }
      else {
        if ((local_b40 == 8) && (local_b40 = 0, lStack_b38 != 0)) {
          (*DAT_143ad5990)(lStack_b38 + -4);
        }
        iVar7 = (*DAT_143262a28)(&local_b40,&DAT_143a8b8d8);
        if (iVar7 < 0) {
LAB_142cac034:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar7);
        }
      }
      local_ad8 = 0x16;
      uStack_ad0 = 2;
      (*DAT_143262a20)(&local_b58);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_b58 == 8) {
          local_b58 = (uint)local_b58._2_2_ << 0x10;
          if (uStack_b50 != 0) {
            (*DAT_143ad5990)(uStack_b50 + -4);
          }
        }
        else {
          iVar7 = (*DAT_143262a18)(&local_b58);
          if (iVar7 < 0) goto LAB_142cac03c;
        }
        local_b58 = CONCAT22(local_b58._2_2_,8);
        if (DAT_143a8b8e0 == 0) {
          uVar16 = 0;
        }
        else {
          uVar16 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
        }
        uStack_b50 = FUN_1401a5fa0(DAT_143a8b8e0,uVar16);
      }
      else {
        if (((short)local_b58 == 8) && (local_b58 = (uint)local_b58._2_2_ << 0x10, uStack_b50 != 0))
        {
          (*DAT_143ad5990)(uStack_b50 + -4);
        }
        iVar7 = (*DAT_143262a28)(&local_b58,&DAT_143a8b8d8);
        if (iVar7 < 0) {
LAB_142cac03c:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar7);
        }
      }
      local_998 = CONCAT62(uStack_b3e,local_b40);
      lStack_990 = lStack_b38;
      local_988 = local_b30;
      local_978 = CONCAT62(uStack_ad6,local_ad8);
      uStack_970 = CONCAT44(uStack_acc,uStack_ad0);
      local_968 = local_ac8;
      local_958 = local_b58;
      uStack_954 = uStack_b54;
      uStack_950 = (undefined4)uStack_b50;
      uStack_94c = uStack_b50._4_4_;
      local_948 = local_b48;
      local_be0 = &local_998;
      local_be8 = &local_978;
      lVar8 = (**(code **)(*(longlong *)pIVar4 + 0x68))(pIVar4,1,1,&local_958);
      if (lVar8 < 0) {
        _com_issue_errorex(lVar8,pIVar4,(_GUID *)&DAT_14327ac98);
      }
      if ((short)local_b58 == 8) {
        local_b58 = local_b58 & 0xffff0000;
        if (uStack_b50 != 0) {
          (*DAT_143ad5990)(uStack_b50 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_b58);
      }
      if (local_ad8 == 8) {
        local_ad8 = 0;
        if (CONCAT44(uStack_acc,uStack_ad0) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_acc,uStack_ad0) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_ad8);
      }
      if (local_b40 == 8) {
        local_b40 = 0;
        if (lStack_b38 != 0) {
          (*DAT_143ad5990)(lStack_b38 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_b40);
      }
      pIVar4 = (IUnknown *)*plVar1;
      if (pIVar4 == (IUnknown *)0x0) goto LAB_142cac05f;
      local_be0 = (undefined8 *)CONCAT44(local_be0._4_4_,0xffffff);
      local_be8 = (undefined8 *)CONCAT44(local_be8._4_4_,1);
      iVar7 = (**(code **)(*(longlong *)pIVar4 + 0x170))(pIVar4,0,0,1);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar4,(_GUID *)&DAT_14327ac98);
      }
      pIVar4 = (IUnknown *)*plVar1;
      if (pIVar4 == (IUnknown *)0x0) goto LAB_142cac05f;
      local_be0 = (undefined8 *)
                  (CONCAT44(local_be0._4_4_,((*(int *)(lVar9 + 0x60) * 0xff) / 100) * 0x1000000) |
                  0xffffff);
      local_be8 = (undefined8 *)CONCAT44(local_be8._4_4_,1);
      iVar7 = (**(code **)(*(longlong *)pIVar4 + 0x170))(pIVar4,0,0,1);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar4,(_GUID *)&DAT_14327ac98);
      }
    }
  }
  plVar2 = param_1 + 0x36;
  if (*plVar2 == 0) {
    FUN_140295e30(PTR_u_Canvas_143a479d8,plVar2,0);
    pIVar4 = (IUnknown *)*plVar2;
    if (pIVar4 != (IUnknown *)0x0) {
      (*DAT_143262a20)(local_b20);
      iVar7 = FUN_14023c4c0(local_b20,&DAT_143a8b8d8);
      if (iVar7 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar7);
      }
      local_aa8 = 0x16;
      uStack_aa0 = 2;
      (*DAT_143262a20)(&local_ac0);
      iVar7 = FUN_14023c4c0(&local_ac0,&DAT_143a8b8d8);
      if (iVar7 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar7);
      }
      lStack_930 = lStack_b18;
      local_928 = local_b10;
      local_918 = CONCAT62(uStack_aa6,local_aa8);
      uStack_910 = CONCAT44(uStack_a9c,uStack_aa0);
      local_908 = local_a98;
      local_8f8 = local_ac0;
      uStack_8f4 = uStack_abc;
      uStack_8f0 = uStack_ab8;
      uStack_8ec = uStack_ab4;
      local_8e8 = local_ab0;
      local_be0 = &local_938;
      local_be8 = &local_918;
      lVar8 = (**(code **)(*(longlong *)pIVar4 + 0x68))(pIVar4,1,1,&local_8f8);
      if (lVar8 < 0) {
        _com_issue_errorex(lVar8,pIVar4,(_GUID *)&DAT_14327ac98);
      }
      if ((short)local_ac0 == 8) {
        local_ac0 = local_ac0 & 0xffff0000;
        if (CONCAT44(uStack_ab4,uStack_ab8) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_ab4,uStack_ab8) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_ac0);
      }
      if (local_aa8 == 8) {
        local_aa8 = 0;
        if (CONCAT44(uStack_a9c,uStack_aa0) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_a9c,uStack_aa0) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_aa8);
      }
      if (local_b20[0] == 8) {
        local_b20[0] = 0;
        if (lStack_b18 != 0) {
          (*DAT_143ad5990)(lStack_b18 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_b20);
      }
      pIVar4 = (IUnknown *)*plVar2;
      if (pIVar4 == (IUnknown *)0x0) {
LAB_142cac05f:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      local_be0 = (undefined8 *)CONCAT44(local_be0._4_4_,0xffffff);
      local_be8 = (undefined8 *)CONCAT44(local_be8._4_4_,1);
      iVar7 = (**(code **)(*(longlong *)pIVar4 + 0x170))(pIVar4,0,0,1);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar4,(_GUID *)&DAT_14327ac98);
      }
      pIVar4 = (IUnknown *)*plVar2;
      if (pIVar4 == (IUnknown *)0x0) goto LAB_142cac05f;
      local_be0 = (undefined8 *)
                  (CONCAT44(local_be0._4_4_,((*(int *)(lVar9 + 100) * 0xff) / 100) * 0x1000000) |
                  0xffffff);
      local_be8 = (undefined8 *)CONCAT44(local_be8._4_4_,1);
      iVar7 = (**(code **)(*(longlong *)pIVar4 + 0x170))(pIVar4,0,0,1);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar4,(_GUID *)&DAT_14327ac98);
      }
    }
  }
  lVar9 = DAT_143add050;
  if (param_1[0x2f] == 0) {
    if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    uVar10 = FUN_1401a5890(local_a90,L"alpha");
    plVar13 = (longlong *)FUN_1418ae1f0(lVar9,&local_a88,uVar10);
    plVar14 = (longlong *)param_1[0x2f];
    if (plVar14 != (longlong *)*plVar13) {
      param_1[0x2f] = *plVar13;
      *plVar13 = 0;
      if (plVar14 != (longlong *)0x0) {
        (**(code **)(*plVar14 + 0x10))();
      }
    }
    lVar9 = param_1[0x2f];
    if (local_a88 != (longlong *)0x0) {
      (**(code **)(*local_a88 + 0x10))();
    }
    if (lVar9 != 0) {
      pIVar4 = (IUnknown *)param_1[0x2f];
      if (pIVar4 == (IUnknown *)0x0) goto LAB_142cac009;
      iVar7 = (**(code **)(*(longlong *)pIVar4 + 0x38))(pIVar4,0,*plVar17);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar4,(_GUID *)&DAT_14337f168);
      }
    }
  }
  lVar9 = DAT_143add050;
  if (param_1[0x30] == 0) {
    if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    uVar10 = FUN_1401a5890(local_a80,L"alpha");
    plVar14 = (longlong *)FUN_1418ae1f0(lVar9,&local_a78,uVar10);
    plVar17 = (longlong *)param_1[0x30];
    if (plVar17 != (longlong *)*plVar14) {
      param_1[0x30] = *plVar14;
      *plVar14 = 0;
      if (plVar17 != (longlong *)0x0) {
        (**(code **)(*plVar17 + 0x10))();
      }
    }
    lVar9 = param_1[0x30];
    if (local_a78 != (longlong *)0x0) {
      (**(code **)(*local_a78 + 0x10))();
    }
    if (lVar9 != 0) {
      pIVar4 = (IUnknown *)param_1[0x30];
      if (pIVar4 == (IUnknown *)0x0) goto LAB_142cac009;
      iVar7 = (**(code **)(*(longlong *)pIVar4 + 0x38))(pIVar4,0,*plVar15);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar4,(_GUID *)&DAT_14337f168);
      }
    }
  }
  lVar9 = DAT_143add050;
  if (param_1[0x31] == 0) {
    if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    uVar10 = FUN_1401a5890(local_a70,L"alpha");
    plVar15 = (longlong *)FUN_1418ae1f0(lVar9,&local_a68,uVar10);
    plVar17 = (longlong *)param_1[0x31];
    if (plVar17 != (longlong *)*plVar15) {
      param_1[0x31] = *plVar15;
      *plVar15 = 0;
      if (plVar17 != (longlong *)0x0) {
        (**(code **)(*plVar17 + 0x10))();
      }
    }
    lVar9 = param_1[0x31];
    if (local_a68 != (longlong *)0x0) {
      (**(code **)(*local_a68 + 0x10))();
    }
    if (lVar9 != 0) {
      pIVar4 = (IUnknown *)param_1[0x31];
      if (pIVar4 == (IUnknown *)0x0) goto LAB_142cac009;
      iVar7 = (**(code **)(*(longlong *)pIVar4 + 0x38))(pIVar4,0,*plVar1);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar4,(_GUID *)&DAT_14337f168);
      }
    }
  }
  lVar9 = DAT_143add050;
  if (param_1[0x32] == 0) {
    if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    uVar10 = FUN_1401a5890(local_a60,L"alpha");
    plVar15 = (longlong *)FUN_1418ae1f0(lVar9,&local_b28,uVar10);
    plVar17 = (longlong *)param_1[0x32];
    if (plVar17 != (longlong *)*plVar15) {
      param_1[0x32] = *plVar15;
      *plVar15 = 0;
      if (plVar17 != (longlong *)0x0) {
        (**(code **)(*plVar17 + 0x10))();
      }
    }
    lVar9 = param_1[0x32];
    if (local_b28 != (longlong *)0x0) {
      (**(code **)(*local_b28 + 0x10))();
    }
    if (lVar9 != 0) {
      pIVar4 = (IUnknown *)param_1[0x32];
      if (pIVar4 == (IUnknown *)0x0) {
LAB_142cac009:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      iVar7 = (**(code **)(*(longlong *)pIVar4 + 0x38))(pIVar4,0,*plVar2);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar4,(_GUID *)&DAT_14337f168);
      }
    }
  }
  FUN_1406ed610(local_8d8);
  FUN_1406ed610(local_488);
  if (local_bc8[0] != (longlong *)0x0) {
    FUN_14019f2c0(local_bc8[0] + -2);
  }
  return;
}


