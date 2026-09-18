
//===========================================================
// FUN_1429d5290 @ 1429d5290   (888 bytes)
//===========================================================

void FUN_1429d5290(longlong param_1,undefined8 param_2)

{
  undefined8 uVar1;
  byte bVar2;
  byte bVar3;
  undefined1 uVar4;
  char cVar5;
  char cVar6;
  char cVar7;
  uint uVar8;
  undefined4 uVar9;
  int iVar10;
  int iVar11;
  int *piVar12;
  longlong lVar13;
  uint uVar14;
  int iVar15;
  int *local_res8;
  
  bVar2 = FUN_1406e8ae0(param_2);
  iVar15 = 0;
  if ((bVar2 & 1) != 0) {
    local_res8 = (int *)0x0;
    piVar12 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar12[1] = 0;
    local_res8 = piVar12 + 4;
    *piVar12 = -1;
    piVar12[2] = 0;
    *(undefined1 *)local_res8 = 0;
    if (*piVar12 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar12[1] < 0) {
      FUN_142e54290(0x90,piVar12[1],0);
    }
    *piVar12 = 1;
    *(undefined1 *)local_res8 = 0;
    if (piVar12[1] + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    piVar12[2] = 0;
    FUN_1402ee8d0(param_1 + 0x130,param_2,&local_res8,0);
    FUN_1429b9660(DAT_143ac1b90,param_1 + 0x1228);
    FUN_1429b9910(DAT_143ac1b90,param_1 + 0x1520);
    FUN_1429b9de0(DAT_143ac1b90,*(undefined4 *)(param_1 + 0x10d0));
    FUN_140db8a40(param_1);
    FUN_140f80200(param_1 + 0x100,0,0,0,0);
  }
  if ((bVar2 & 2) != 0) {
    lVar13 = *(longlong *)(param_1 + 0x4070);
    bVar3 = FUN_1406e8ae0(param_2);
    uVar8 = FUN_1407386b0(&DAT_143ac1ab0);
    *(uint *)(lVar13 + 0x28) = uVar8;
    uVar14 = (bVar3 ^ uVar8) >> 5 | (bVar3 ^ uVar8) << 0x1b;
    *(uint *)(lVar13 + 0x2c) = uVar14;
    *(uint *)(lVar13 + 0x30) = ((uVar8 ^ 0xbaadf00d) >> 5 | (uVar8 ^ 0xbaadf00d) << 0x1b) + uVar14;
  }
  if ((bVar2 & 4) != 0) {
    uVar4 = FUN_1406e8ae0(param_2);
    FUN_1427eb600(param_1,uVar4);
  }
  cVar5 = FUN_1406e8ae0(param_2);
  if (cVar5 != '\0') {
    FUN_1406e9170(param_2,param_1 + 0x1228,8);
    FUN_1406e9170(param_2,param_1 + 0x1230,8);
    uVar1 = DAT_143ac1b90;
    uVar9 = FUN_1406e8c20(param_2);
    FUN_1429b9490(uVar1,param_1 + 0x1228,param_1,uVar9);
  }
  cVar6 = FUN_1406e8ae0(param_2);
  if (cVar6 != '\0') {
    FUN_1406e9170(param_2,param_1 + 0x1520,8);
    FUN_1406e9170(param_2,param_1 + 0x1528,8);
    uVar1 = DAT_143ac1b90;
    uVar9 = FUN_1406e8c20(param_2);
    FUN_1429b9740(uVar1,param_1 + 0x1520,param_1,uVar9);
  }
  cVar7 = FUN_1406e8ae0(param_2);
  if (cVar7 == '\0') {
    *(undefined8 *)(param_1 + 0x13a8) = 0;
    *(undefined4 *)(param_1 + 0x13b0) = 0;
  }
  else {
    uVar9 = FUN_1406e8c20(param_2);
    *(undefined4 *)(param_1 + 0x13a8) = uVar9;
    uVar9 = FUN_1406e8c20(param_2);
    *(undefined4 *)(param_1 + 0x13ac) = uVar9;
    uVar9 = FUN_1406e8c20(param_2);
    *(undefined4 *)(param_1 + 0x13b0) = uVar9;
    FUN_1429b9c80(DAT_143ac1b90,*(undefined4 *)(param_1 + 0x13a8),param_1,uVar9);
  }
  if (cVar5 == '\0') {
    *(undefined8 *)(param_1 + 0x1228) = 0;
    *(undefined8 *)(param_1 + 0x1230) = 0;
  }
  if (cVar6 == '\0') {
    *(undefined8 *)(param_1 + 0x1520) = 0;
    *(undefined8 *)(param_1 + 0x1528) = 0;
  }
  uVar9 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x2e08) = uVar9;
  uVar9 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x4358) = uVar9;
  FUN_1427e90b0(param_1);
  FUN_1428336c0(param_1);
  iVar10 = FUN_142dec860(DAT_143aa84a0,*(undefined4 *)(param_1 + 0x10d0));
  if ((iVar10 != 0) && (DAT_143aca960 != 0)) {
    iVar10 = *(int *)(DAT_143aca960 + 0x340);
    do {
      lVar13 = FUN_140f80130(param_1 + 0x100);
      iVar11 = FUN_1402537f0(iVar15);
      if (*(int *)(lVar13 + 0x39 + (longlong)iVar11 * 4) - 0x10ff90U < 100) goto LAB_1429d55f3;
      iVar15 = iVar15 + 1;
    } while (iVar15 < 4);
    if (iVar10 != 0) {
LAB_1429d55f3:
      FUN_14118f5d0(DAT_143aca960);
    }
  }
  return;
}



//===========================================================
// FUN_140f80200 @ 140f80200   (1531 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000140f80499) */
/* WARNING: Removing unreachable block (ram,0x000140f80522) */

void FUN_140f80200(longlong *param_1,int param_2,undefined8 param_3,undefined1 param_4,char param_5)

{
  longlong *plVar1;
  longlong **pplVar2;
  bool bVar3;
  char cVar4;
  int iVar5;
  undefined4 uVar6;
  uint uVar7;
  longlong lVar8;
  undefined8 uVar9;
  uint uVar10;
  undefined8 *puVar11;
  int *piVar12;
  longlong *plVar13;
  longlong lVar14;
  undefined4 local_68;
  undefined4 local_64;
  undefined4 local_60;
  undefined8 local_5c;
  undefined8 uStack_54;
  longlong *local_48 [2];
  
  lVar8 = FUN_14209ee40();
  lVar8 = *(longlong *)(lVar8 + 8);
  if (lVar8 != 0) {
    if (0xfffff < *(ulonglong *)(lVar8 + 0x28)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar8 + 0x28) = *(longlong *)(lVar8 + 0x28) + 1;
    UNLOCK();
  }
  if ((lVar8 == 0) ||
     ((iVar5 = (**(code **)(*(longlong *)(lVar8 + 8) + 0xd0))(lVar8 + 8,&PTR_PTR_143a870f8),
      iVar5 == 0 &&
      (iVar5 = (**(code **)(*(longlong *)(lVar8 + 8) + 0xd0))(lVar8 + 8,&PTR_PTR_143a886e8),
      iVar5 == 0)))) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
    FUN_140c9d340(*(undefined4 *)((longlong)param_1 + 0x59),
                  *(undefined4 *)((longlong)param_1 + 0x69));
    FUN_1403eafa0(param_1 + 6);
  }
  plVar13 = param_1 + 6;
  if ((param_2 == 0) &&
     (iVar5 = memcmp((void *)((longlong)param_1 + 0x213),plVar13,0x1e3), iVar5 == 0))
  goto LAB_140f8079d;
  FUN_1402eeb50((longlong)param_1 + 0x213,plVar13);
  local_5c = 0;
  uStack_54 = 0;
  local_48[0] = (longlong *)0x0;
  local_68 = (undefined4)param_1[0x95];
  local_64 = *(undefined4 *)((longlong)param_1 + 0x4ac);
  local_60 = (undefined4)param_1[0x96];
  uVar6 = FUN_1401ba9d0(param_1 + 0x9c,(int)param_1[0x9d]);
  local_5c = CONCAT44((int)param_1[0x97],uVar6);
  uStack_54 = CONCAT44(*(undefined4 *)((longlong)param_1 + 0x4dc),
                       *(undefined4 *)((longlong)param_1 + 0x4b4));
  pplVar2 = (longlong **)(param_1 + 0x9e);
  if ((local_48 != pplVar2) && (local_48[0] = *pplVar2, local_48[0] != (longlong *)0x0)) {
    LOCK();
    *(int *)(local_48[0] + 2) = (int)local_48[0][2] + 1;
    UNLOCK();
  }
  iVar5 = (**(code **)(*param_1 + 8))(param_1);
  FUN_140f809b0(param_1,&local_68,plVar13,iVar5 != 0);
  *(undefined4 *)(param_1 + 0x95) = local_68;
  *(undefined4 *)((longlong)param_1 + 0x4ac) = local_64;
  *(undefined4 *)(param_1 + 0x96) = local_60;
  uVar7 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)(param_1 + 0x9c) = uVar7;
  uVar10 = (uVar7 ^ (uint)local_5c) >> 5 | (uVar7 ^ (uint)local_5c) << 0x1b;
  *(uint *)((longlong)param_1 + 0x4e4) = uVar10;
  *(uint *)(param_1 + 0x9d) = ((uVar7 ^ 0xbaadf00d) >> 5 | (uVar7 ^ 0xbaadf00d) << 0x1b) + uVar10;
  *(undefined4 *)(param_1 + 0x97) = local_5c._4_4_;
  *(undefined4 *)((longlong)param_1 + 0x4b4) = (undefined4)uStack_54;
  *(undefined4 *)((longlong)param_1 + 0x4dc) = uStack_54._4_4_;
  if (pplVar2 != local_48) {
    FUN_1401be120(pplVar2);
    *pplVar2 = local_48[0];
    if (local_48[0] != (longlong *)0x0) {
      LOCK();
      *(int *)(local_48[0] + 2) = (int)local_48[0][2] + 1;
      UNLOCK();
    }
  }
  plVar1 = local_48[0];
  FUN_140fafed0(param_1);
  lVar14 = 1;
  piVar12 = (int *)((longlong)param_1 + 0x6d);
  do {
    if ((0 < *piVar12) && (iVar5 = FUN_140392d30(DAT_143aa8328), iVar5 - 1U < 0x26)) {
      if (iVar5 - 1U < 0x26) goto LAB_140f8045d;
      break;
    }
    lVar14 = lVar14 + 1;
    piVar12 = piVar12 + 1;
  } while (lVar14 < 0x20);
  iVar5 = *(int *)((longlong)param_1 + 0x5ec);
LAB_140f8045d:
  *(int *)((longlong)param_1 + 0xef4) = iVar5;
  FUN_140f832e0(param_1,iVar5,0xffffffff);
  lVar14 = FUN_141892840();
  if (lVar14 != 0) {
    FUN_141829fd0(lVar14);
  }
  if ((DAT_143aa8518 != (longlong *)0x0) && (param_1 == DAT_143aa8518 + 0x20)) {
    if (DAT_143ac8fb0 != 0) {
      FUN_141f7e3d0();
    }
    if (DAT_143ac8fb8 != 0) {
      FUN_141f80e20();
    }
  }
  if (bVar3) {
    FUN_140c9d370(plVar13);
  }
  if (*(int *)((longlong)param_1 + 0x5ac) == 0) {
    if (param_5 == '\0') {
      FUN_140fbbe20(param_1,param_3,param_4);
    }
    if ((((DAT_143aa8518 != (longlong *)0x0) &&
         ((**(code **)(*DAT_143aa8518 + 0xe8))(), DAT_143aa8518 != (longlong *)0x0)) &&
        (param_1 != DAT_143aa8518 + 0x20)) ||
       ((DAT_143aa8560 == 0 || (*(int *)(DAT_143aa8560 + 0xc) == 0)))) {
      (**(code **)(*param_1 + 0xc0))(param_1,0);
      (**(code **)(*param_1 + 0x70))(param_1,*(undefined4 *)((longlong)param_1 + 0x5e4),1);
    }
    iVar5 = FUN_140f82790(param_1,0,0);
    uVar7 = *(uint *)((longlong)param_1 + 0x5e4) & 1 | iVar5 * 2;
    if ((longlong *)param_1[0x1e9] != (longlong *)0x0) {
      (**(code **)(*(longlong *)param_1[0x1e9] + 0x30))();
      *(undefined4 *)(param_1 + 0x1e7) = 0xffffffff;
      lVar14 = FUN_141892840();
      if (lVar14 != 0) {
        uVar9 = FUN_141892840();
        cVar4 = FUN_14182ffd0(uVar9);
        if (cVar4 == '\x01') goto LAB_140f805be;
      }
      FUN_140fb8070(param_1,uVar7);
      *(uint *)(param_1 + 0x1ec) = uVar7;
    }
LAB_140f805be:
    (**(code **)(*param_1 + 0x50))(param_1);
    if ((int)param_1[0x1db] != 0) {
      uVar7 = *(uint *)((longlong)param_1 + 0xedc);
      if (((int)uVar7 < 0) || (iVar5 = FUN_1402b1780(), iVar5 <= (int)uVar7)) {
        *(undefined4 *)(param_1 + 0x1db) = 0;
        *(undefined4 *)((longlong)param_1 + 0xedc) = 0xffffffff;
        (**(code **)(*param_1 + 0xc0))(param_1,0);
        (**(code **)(*param_1 + 0xa0))(param_1);
        FUN_140f8d910(param_1,6,0x78,0,0,0xffffffff,0);
      }
      else {
        *(undefined4 *)(param_1 + 0x1db) = 1;
        if (uVar7 < 2) {
          uVar7 = (uint)(*(int *)((longlong)param_1 + 0x4b4) != 1);
        }
        (**(code **)(*param_1 + 0xc0))(param_1,0);
        (**(code **)(*param_1 + 0xa0))(param_1);
        *(uint *)((longlong)param_1 + 0xedc) = uVar7;
        *(uint *)(param_1 + 0xbd) = uVar7;
        FUN_140f8d910(param_1,6,200,0,0,0xffffffff,0);
        param_1[0x1d0] = 1;
        *(undefined4 *)(param_1 + 0x1d1) = 0;
      }
    }
    if (*(int *)((longlong)param_1 + 0xee4) == 0) {
      if (*(int *)((longlong)param_1 + 0xeec) == 0) {
        cVar4 = (**(code **)(*param_1 + 0x110))(param_1);
        if (cVar4 == '\0') {
          uVar7 = *(uint *)((longlong)param_1 + 0xef4);
          if (uVar7 - 1 < 0x26) {
            *(uint *)((longlong)param_1 + 0xef4) = uVar7;
          }
          else {
            uVar7 = *(uint *)((longlong)param_1 + 0x5ec);
          }
          goto LAB_140f8073d;
        }
        (**(code **)(*param_1 + 0x118))(param_1);
      }
      else {
        uVar6 = FUN_140f82790(param_1,0,0);
        FUN_140fb6eb0(param_1,*(undefined4 *)((longlong)param_1 + 0x52c),uVar6);
      }
    }
    else {
      uVar10 = *(uint *)(param_1 + 0x1dd);
      uVar7 = uVar10;
      if (0x26 < uVar10) {
        uVar7 = *(uint *)((longlong)param_1 + 0x5ec);
      }
      *(uint *)((longlong)param_1 + 0xee4) = (uint)(uVar10 < 0x27);
      *(uint *)(param_1 + 0x1dd) = uVar7;
LAB_140f8073d:
      FUN_140f832e0(param_1,uVar7,0xffffffff);
    }
    (**(code **)(*param_1 + 0x10))(param_1);
  }
  plVar13 = local_48[0];
  if (plVar1 != (longlong *)0x0) {
    LOCK();
    plVar1 = plVar1 + 2;
    lVar14 = *plVar1;
    *(int *)plVar1 = (int)*plVar1 + -1;
    UNLOCK();
    if ((int)lVar14 == 1) {
      if (*local_48[0] != 0) {
        (*DAT_143ad5990)(*local_48[0] + -4);
        *plVar13 = 0;
      }
      if (plVar13[1] != 0) {
        FUN_14019b4e0();
        plVar13[1] = 0;
      }
      thunk_FUN_140205820(plVar13,0x18);
    }
  }
LAB_140f8079d:
  if (lVar8 != 0) {
    if (0xffffe < *(longlong *)(lVar8 + 0x28) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar13 = (longlong *)(lVar8 + 0x28);
    lVar14 = *plVar13;
    *plVar13 = *plVar13 + -1;
    UNLOCK();
    if (((int)lVar14 == 1) && (puVar11 = (undefined8 *)(lVar8 + 0x20), puVar11 != (undefined8 *)0x0)
       ) {
      (**(code **)*puVar11)(puVar11,1);
    }
  }
  return;
}



//===========================================================
// FUN_140db8a40 @ 140db8a40   (69 bytes)
//===========================================================

void FUN_140db8a40(longlong param_1)

{
  undefined1 local_18 [8];
  longlong local_10;
  
  if (param_1 != 0) {
    FUN_1427be040(param_1,local_18);
    if (((local_10 != 0) && (*(longlong *)(local_10 + 0x2b0) != 0)) &&
       (0 < *(int *)(*(longlong *)(local_10 + 0x2b0) + 0x18))) {
      FUN_140db56b0();
    }
    FUN_140ce88a0(local_18);
  }
  return;
}



//===========================================================
// FUN_1427e90b0 @ 1427e90b0   (6026 bytes)
//===========================================================

/* WARNING: Type propagation algorithm not settling */

void FUN_1427e90b0(undefined8 **param_1)

{
  longlong *plVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  longlong lVar6;
  longlong lVar7;
  int *piVar8;
  undefined8 *puVar9;
  undefined8 uVar10;
  undefined8 **ppuVar11;
  longlong *plVar12;
  IUnknown *pIVar13;
  undefined4 *puVar14;
  IUnknown *pIVar15;
  IUnknown *pIVar16;
  IUnknown *pIVar17;
  longlong lVar18;
  IUnknown *pIVar19;
  IUnknown *pIVar20;
  undefined8 **local_res8;
  int local_res10 [2];
  undefined4 *local_res18;
  IUnknown *local_res20;
  uint *puVar21;
  IUnknown *local_368;
  undefined8 *local_360;
  IUnknown *local_358;
  short local_350;
  undefined6 uStack_34e;
  longlong lStack_348;
  undefined8 local_340;
  undefined4 local_338;
  undefined4 uStack_334;
  undefined8 uStack_330;
  undefined8 local_328;
  undefined8 *local_320;
  undefined8 *local_318;
  undefined8 *local_310;
  IUnknown *local_308;
  ulonglong local_300;
  longlong local_2f8;
  undefined8 **local_2f0;
  undefined8 **local_2e8;
  short local_2e0 [4];
  longlong local_2d8;
  longlong *local_2c8;
  ulonglong local_2c0 [2];
  IUnknown *local_2b0;
  int *local_2a8;
  IUnknown *local_2a0;
  short local_298 [4];
  longlong lStack_290;
  undefined8 local_288;
  uint local_280;
  undefined4 uStack_27c;
  undefined4 uStack_278;
  undefined4 uStack_274;
  undefined8 local_270;
  short local_268 [4];
  longlong lStack_260;
  undefined8 local_258;
  uint local_250;
  undefined4 uStack_24c;
  undefined4 uStack_248;
  undefined4 uStack_244;
  undefined8 local_240;
  uint local_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined8 local_228;
  uint local_220;
  undefined4 uStack_21c;
  undefined4 uStack_218;
  undefined4 uStack_214;
  undefined8 local_210;
  uint local_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined8 local_1f8;
  uint local_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined8 local_1e0;
  undefined8 *local_1d8;
  IUnknown *local_1d0;
  longlong *local_1c8;
  longlong *local_1c0;
  longlong local_1b8;
  IUnknown *local_1b0;
  undefined1 local_1a8 [8];
  uint local_1a0;
  undefined4 uStack_19c;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined8 local_190;
  uint local_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined8 local_178;
  uint local_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined8 local_160;
  uint local_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined8 local_148;
  short local_140 [4];
  longlong local_138;
  short local_128 [4];
  longlong local_120;
  short local_110 [4];
  longlong local_108;
  undefined8 local_f8;
  longlong lStack_f0;
  undefined8 local_e8;
  uint local_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined8 local_c8;
  undefined1 local_b8 [8];
  longlong lStack_b0;
  undefined8 local_a8;
  uint local_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 local_88;
  undefined1 local_78 [8];
  longlong lStack_70;
  undefined8 local_68;
  uint local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  
  pIVar15 = (IUnknown *)0x0;
  local_res18 = (undefined4 *)((ulonglong)local_res18 & 0xffffffff00000000);
  ppuVar11 = param_1 + 0x20;
  local_res8 = param_1;
  iVar3 = FUN_140f8abc0(ppuVar11);
  if (iVar3 != 0) {
    return;
  }
  iVar3 = FUN_1403d59d0(DAT_143aa8328,(longlong)param_1 + 0x169,
                        *(undefined4 *)((longlong)param_1 + 0x15d));
  pIVar19 = (IUnknown *)0x0;
  local_368 = (IUnknown *)0x0;
  local_res10[0] = iVar3;
  if (-1 < iVar3) goto LAB_1427e9349;
  iVar3 = (*(code *)(*param_1)[10])(param_1);
  if (iVar3 == 0) {
    lVar6 = FUN_1403a91b0(DAT_143aa8328,*(undefined4 *)(param_1 + 0x5c1));
    pIVar20 = pIVar19;
    if (lVar6 != 0) {
      iVar3 = *(int *)(param_1 + 0x5c1);
      puVar9 = (undefined8 *)(lVar6 + 0x250);
      goto LAB_1427e932b;
    }
  }
  else {
    lVar6 = FUN_142cbe730(DAT_143aa84a0);
    plVar12 = *(longlong **)(lVar6 + 0xf89);
    pIVar20 = pIVar15;
    if (plVar12 != (longlong *)0x0) {
      plVar1 = plVar12 + *(uint *)(lVar6 + 0xf91);
      for (; pIVar20 = pIVar19, plVar12 < plVar1; plVar12 = plVar12 + 1) {
        lVar18 = *plVar12;
        if (lVar18 != 0) goto LAB_1427e9281;
      }
    }
  }
LAB_1427e9175:
  *(undefined8 *)((longlong)param_1 + 0x2e24) = 0;
  param_1[0x5c3] = (undefined8 *)0x0;
  *(undefined4 *)(param_1 + 0x5c4) = 1;
  *(undefined8 *)((longlong)param_1 + 0x2e2c) = 0;
  if (param_1[0x5c7] != (undefined8 *)0x0) {
    thunk_FUN_140205820(param_1[0x5c7] + -1,0);
    param_1[0x5c7] = (undefined8 *)0x0;
  }
  *(undefined4 *)(param_1 + 0x5c8) = 0;
  *(undefined4 *)((longlong)param_1 + 0x2e04) = 0;
  if (param_1[0x5c2] != (undefined8 *)0x0) {
    FUN_1401bebb0(param_1[0x5c2] + -2);
    param_1[0x5c2] = (undefined8 *)0x0;
  }
  *(undefined4 *)(param_1 + 0x5c1) = 0;
  FUN_140fabb10(ppuVar11);
  do {
    ppuVar11 = param_1 + (longlong)pIVar15 * 6 + 0x2ad;
    *(undefined4 *)ppuVar11 = 0xffffffff;
    *(undefined4 *)(ppuVar11 + 2) = 1;
    if (ppuVar11[5] != (longlong *)0x0) {
      (**(code **)(*ppuVar11[5] + 0x10))();
    }
    ppuVar11[5] = (undefined8 *)0x0;
    if (ppuVar11[3] != (longlong *)0x0) {
      (**(code **)(*ppuVar11[3] + 0x10))();
    }
    ppuVar11[3] = (undefined8 *)0x0;
    if (ppuVar11[4] != (longlong *)0x0) {
      (**(code **)(*ppuVar11[4] + 0x10))();
    }
    ppuVar11[4] = (undefined8 *)0x0;
    if (pIVar15 == (IUnknown *)0x1) {
      *(undefined4 *)(param_1 + 0x2b3) = 0;
      break;
    }
    pIVar15 = pIVar15 + 1;
  } while ((longlong)pIVar15 < 1);
  *(undefined4 *)(param_1 + 0x5c9) = 0xffffffff;
  *(undefined1 *)((longlong)param_1 + 0x2e4c) = 0;
  lVar6 = 1;
  do {
    ppuVar11 = param_1 + lVar6 * 6 + 0x2ad;
    *(undefined4 *)ppuVar11 = 0xffffffff;
    *(undefined4 *)(ppuVar11 + 2) = 1;
    if (ppuVar11[5] != (longlong *)0x0) {
      (**(code **)(*ppuVar11[5] + 0x10))();
    }
    ppuVar11[5] = (undefined8 *)0x0;
    if (ppuVar11[3] != (longlong *)0x0) {
      (**(code **)(*ppuVar11[3] + 0x10))();
    }
    ppuVar11[3] = (undefined8 *)0x0;
    if (ppuVar11[4] != (longlong *)0x0) {
      (**(code **)(*ppuVar11[4] + 0x10))();
    }
    ppuVar11[4] = (undefined8 *)0x0;
    if (lVar6 == 1) {
      *(undefined4 *)(param_1 + 0x2b3) = 0;
      break;
    }
    lVar6 = lVar6 + 1;
  } while (lVar6 < 2);
LAB_1427ea7b1:
  if (pIVar20 != (IUnknown *)0x0) {
    FUN_1401bebb0(pIVar20 + -0x10);
  }
  return;
LAB_1427e9281:
  piVar8 = (int *)(lVar18 + 0x10);
  lVar7 = FUN_1403a91b0(DAT_143aa8328);
  if ((lVar7 != 0) && (*(int *)(lVar18 + 0x14) == *(int *)(lVar7 + 0x248))) {
    puVar9 = (undefined8 *)(lVar7 + 0x250);
    if (((short *)*puVar9 != (short *)0x0) && (*(short *)*puVar9 != 0)) goto LAB_1427e92fb;
  }
  lVar18 = *(longlong *)(lVar18 + 8);
  if (lVar18 == 0) {
    plVar12 = (longlong *)
              (*(longlong *)(lVar6 + 0xf89) +
              ((ulonglong)(longlong)*piVar8 % (ulonglong)*(uint *)(lVar6 + 0xf91) + 1) * 8);
    while( true ) {
      if ((longlong *)(*(longlong *)(lVar6 + 0xf89) + (ulonglong)*(uint *)(lVar6 + 0xf91) * 8) <=
          plVar12) goto LAB_1427e9175;
      lVar18 = *plVar12;
      if (lVar18 != 0) break;
      plVar12 = plVar12 + 1;
    }
  }
  goto LAB_1427e9281;
LAB_1427e92fb:
  iVar3 = *piVar8;
LAB_1427e932b:
  local_res10[0] = iVar3;
  FUN_1401c1fb0(&local_368,puVar9);
  pIVar20 = local_368;
  if (-1 < iVar3) {
LAB_1427e9349:
    pIVar20 = local_368;
    iVar4 = FUN_140f89f40(ppuVar11);
    if ((((iVar4 == 0) && (iVar4 = FUN_140f89e20(ppuVar11), iVar4 == 0)) &&
        (iVar4 = FUN_140fb7040(ppuVar11), iVar4 == 0)) &&
       ((cVar2 = FUN_142826340(param_1), cVar2 == '\0' &&
        (cVar2 = FUN_140f80830(ppuVar11), cVar2 == '\0')))) {
      local_2f0 = param_1 + 0x2ad;
      if (*(int *)local_2f0 == iVar3) goto LAB_1427ea7b1;
      local_res20 = (IUnknown *)0x0;
      pIVar19 = pIVar20;
      if ((pIVar20 == (IUnknown *)0x0) || (*(short *)pIVar20 == 0)) {
        puVar9 = (undefined8 *)FUN_1408a9d20(&local_2f8,0x704);
        FUN_1401c21c0(&local_res20,*puVar9,iVar3);
        if (local_2f8 != 0) {
          FUN_1401bebb0(local_2f8 + -0x10);
        }
      }
      else {
        local_res10[0] = -iVar3;
        pIVar16 = pIVar20 + -0x10;
        if (pIVar16 != (IUnknown *)0x0) {
          if (*(int *)pIVar16 == -1) {
            FUN_142e52d50(0xcb,0xffffff01);
            pIVar16 = (IUnknown *)0xffffffffffffffff;
            do {
              pIVar16 = pIVar16 + 1;
            } while (*(short *)(pIVar20 + (longlong)pIVar16 * 2) != 0);
            iVar4 = (int)pIVar16;
            iVar3 = 0;
            if (0 < iVar4) {
              iVar3 = iVar4;
            }
            piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar3 * 2 + 0x12));
            piVar8[1] = iVar3;
            *piVar8 = -1;
            pIVar13 = (IUnknown *)(piVar8 + 4);
            piVar8[2] = 0;
            *(short *)pIVar13 = 0;
            local_2a0 = pIVar13;
            FUN_142ef7ba0(pIVar13,pIVar20,(longlong)iVar4 * 2);
            if (*piVar8 != -1) {
              FUN_142e52dd0(0x8b);
            }
            if ((iVar4 == -1) || (iVar4 <= piVar8[1])) {
              *piVar8 = 1;
              if (iVar4 != -1) goto LAB_1427e9497;
              pIVar16 = pIVar15;
              if (pIVar13 != (IUnknown *)0x0) {
                pIVar16 = (IUnknown *)0xffffffffffffffff;
                do {
                  pIVar16 = pIVar16 + 1;
                } while (*(short *)(pIVar13 + (longlong)pIVar16 * 2) != 0);
              }
            }
            else {
              FUN_142e54290(0x90,piVar8[1],(ulonglong)pIVar16 & 0xffffffff);
              *piVar8 = 1;
LAB_1427e9497:
              *(undefined2 *)(pIVar13 + (longlong)iVar4 * 2) = 0;
            }
            iVar3 = (int)pIVar16;
            if ((iVar3 < 0) || (piVar8[1] + 1 <= iVar3)) {
              FUN_142e54290(0x9c,(ulonglong)pIVar16 & 0xffffffff);
            }
            piVar8[2] = iVar3 * 2;
            local_res20 = pIVar13;
          }
          else {
            if (*(int *)pIVar16 < 1) {
              FUN_142e52dd0(0xd2);
            }
            LOCK();
            *(int *)pIVar16 = *(int *)pIVar16 + 1;
            UNLOCK();
            pIVar19 = local_368;
            local_res20 = pIVar20;
          }
        }
      }
      pIVar20 = pIVar19;
      pIVar19 = local_res20;
      lVar6 = -1;
      lVar18 = -1;
      do {
        lVar18 = lVar18 + 1;
      } while ((&DAT_1432ac608)[lVar18] != 0);
      FUN_14040ea40(&local_res20,&local_1d8);
      local_res18 = (undefined4 *)CONCAT44(local_res18._4_4_,2);
      if (param_1[0x5c2] != (undefined8 *)0x0) {
        FUN_1401bebb0(param_1[0x5c2] + -2);
      }
      param_1[0x5c2] = local_1d8;
      *(undefined4 *)((longlong)param_1 + 0x2e04) = 0;
      *(undefined8 *)((longlong)param_1 + 0x2e24) = 0;
      param_1[0x5c3] = (undefined8 *)0x0;
      *(undefined4 *)(param_1 + 0x5c4) = 1;
      *(undefined8 *)((longlong)param_1 + 0x2e2c) = 0;
      if (param_1[0x5c7] != (undefined8 *)0x0) {
        thunk_FUN_140205820(param_1[0x5c7] + -1,0);
        param_1[0x5c7] = (undefined8 *)0x0;
      }
      *(undefined4 *)(param_1 + 0x5c8) = 0;
      local_308 = (IUnknown *)0x0;
      local_1d0 = DAT_143add058;
      if (DAT_143add058 == (IUnknown *)0x0) {
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
          if (iVar3 < 0) goto LAB_1427ea7d3;
        }
        local_350 = 8;
        pIVar16 = pIVar15;
        if (DAT_143a8b8e0 != 0) {
          pIVar16 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        lStack_348 = FUN_1401a5fa0(DAT_143a8b8e0,pIVar16);
      }
      else {
        if ((local_350 == 8) && (local_350 = 0, lStack_348 != 0)) {
          (*DAT_143ad5990)(lStack_348 + -4);
        }
        iVar3 = (*DAT_143262a28)(&local_350,&DAT_143a8b8d8);
        if (iVar3 < 0) {
LAB_1427ea7d3:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar3);
        }
      }
      (*DAT_143262a20)(&local_338);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_338 == 8) {
          local_338 = (uint)local_338._2_2_ << 0x10;
          if (uStack_330 != 0) {
            (*DAT_143ad5990)(uStack_330 + -4);
          }
        }
        else {
          iVar3 = (*DAT_143262a18)(&local_338);
          if (iVar3 < 0) goto LAB_1427ea7db;
        }
        local_338 = CONCAT22(local_338._2_2_,8);
        pIVar16 = pIVar15;
        if (DAT_143a8b8e0 != 0) {
          pIVar16 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        uStack_330 = FUN_1401a5fa0(DAT_143a8b8e0,pIVar16);
      }
      else {
        if (((short)local_338 == 8) && (local_338 = (uint)local_338._2_2_ << 0x10, uStack_330 != 0))
        {
          (*DAT_143ad5990)(uStack_330 + -4);
        }
        iVar3 = (*DAT_143262a28)(&local_338,&DAT_143a8b8d8);
        if (iVar3 < 0) {
LAB_1427ea7db:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar3);
        }
      }
      puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
      local_2e8 = (undefined8 **)puVar9;
      if (puVar9 == (undefined8 *)0x0) {
        local_320 = (undefined8 *)0x0;
      }
      else {
        puVar9[1] = 0;
        *(undefined4 *)(puVar9 + 2) = 1;
        local_320 = puVar9;
        if (pIVar19 == (IUnknown *)0x0) {
          *puVar9 = 0;
        }
        else {
          do {
            lVar6 = lVar6 + 1;
          } while (*(short *)(pIVar19 + lVar6 * 2) != 0);
          local_300 = (ulonglong)((int)lVar6 + 1);
          piVar8 = (int *)(*DAT_143ad5980)(local_300 * 2 + 4);
          if (piVar8 == (int *)0x0) {
            *puVar9 = 0;
LAB_1427ea7e3:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x8007000e);
          }
          *piVar8 = (int)lVar6 * 2;
          local_2a8 = piVar8 + 1;
          FUN_142ef7ba0(local_2a8,pIVar19,local_300 * 2);
          *puVar9 = local_2a8;
          if (local_2a8 == (int *)0x0) goto LAB_1427ea7e3;
        }
      }
      if (local_320 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x8007000e);
      }
      local_2e8 = &local_320;
      (*DAT_143262a20)(&local_1a0);
      pIVar16 = local_1d0;
      pIVar13 = pIVar15;
      if (local_320 != (undefined8 *)0x0) {
        pIVar13 = (IUnknown *)*local_320;
      }
      local_f8 = CONCAT62(uStack_34e,local_350);
      lStack_f0 = lStack_348;
      local_e8 = local_340;
      local_d8 = local_338;
      uStack_d4 = uStack_334;
      uStack_d0 = (undefined4)uStack_330;
      uStack_cc = uStack_330._4_4_;
      local_c8 = local_328;
      puVar21 = &local_1a0;
      iVar3 = (**(code **)(*(longlong *)local_1d0 + 0x48))
                        (local_1d0,pIVar13,&local_d8,&local_f8,puVar21);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar16,(_GUID *)&DAT_1432743e8);
      }
      local_238 = local_1a0;
      uStack_234 = uStack_19c;
      uStack_230 = uStack_198;
      uStack_22c = uStack_194;
      local_228 = local_190;
      local_1a0 = local_1a0 & 0xffff0000;
      local_res18 = (undefined4 *)CONCAT44(local_res18._4_4_,10);
      FUN_1401be120(&local_320);
      uVar10 = FUN_1409339d0(&local_1c8,&local_238);
      FUN_1401a5040(&local_2b0,uVar10);
      pIVar16 = local_2b0;
      pIVar13 = pIVar15;
      if (local_2b0 != (IUnknown *)0x0) {
        local_308 = local_2b0;
        local_2b0 = (IUnknown *)0x0;
        pIVar13 = pIVar16;
      }
      if (local_2b0 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_2b0 + 0x10))();
      }
      if (local_1c8 != (longlong *)0x0) {
        (**(code **)(*local_1c8 + 0x10))();
      }
      if ((short)local_238 == 8) {
        local_238 = local_238 & 0xffff0000;
        if (CONCAT44(uStack_22c,uStack_230) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_22c,uStack_230) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_238);
      }
      if ((short)local_338 == 8) {
        local_338 = local_338 & 0xffff0000;
        if (uStack_330 != 0) {
          (*DAT_143ad5990)(uStack_330 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_338);
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
      if (pIVar13 == (IUnknown *)0x0) {
        FUN_1427bee20(param_1,0,0xffffffff);
      }
      else {
        *(int *)local_2f0 = local_res10[0];
        if (pIVar13 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        FUN_1401bb8d0(&local_318,PTR_u_action_143a47558);
        local_res8 = &local_318;
        (*DAT_143262a20)(&local_188);
        pIVar16 = pIVar15;
        if (local_318 != (undefined8 *)0x0) {
          pIVar16 = (IUnknown *)*local_318;
        }
        iVar3 = (**(code **)(*(longlong *)pIVar13 + 0x28))(pIVar13,pIVar16,&local_188);
        if (iVar3 < 0) {
          _com_issue_errorex(iVar3,pIVar13,(_GUID *)&DAT_143272478);
        }
        local_220 = local_188;
        uStack_21c = uStack_184;
        uStack_218 = uStack_180;
        uStack_214 = uStack_17c;
        local_210 = local_178;
        local_188 = local_188 & 0xffff0000;
        FUN_1401be120(&local_318);
        iVar3 = FUN_14022ee40(&local_220,0);
        *(uint *)((longlong)param_1 + 0x2e04) = (uint)(iVar3 != 0);
        if ((short)local_220 == 8) {
          local_220 = local_220 & 0xffff0000;
          if (CONCAT44(uStack_214,uStack_218) != 0) {
            (*DAT_143ad5990)(CONCAT44(uStack_214,uStack_218) + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_220);
        }
        if (*(int *)((longlong)param_1 + 0x2e04) == 0) {
          local_res8 = (undefined8 **)(local_2c0 + 1);
          local_2c0[1] = 0;
          if (param_1[0x1cb] == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          uVar5 = FUN_140d1a410();
          FUN_1401bb8d0(&local_358,pIVar19);
          ppuVar11 = local_2f0;
          puVar21 = (uint *)((ulonglong)puVar21 & 0xffffffff00000000);
          iVar3 = FUN_1427c91d0(param_1,&local_358,uVar5,local_2f0 + 1,puVar21,0xffff,local_2c0 + 1,
                                0,0xff);
          if (iVar3 == 0) {
            FUN_1427bee20(param_1,0,0xffffffff);
          }
          else {
            pIVar19 = (IUnknown *)ppuVar11[3];
            if (pIVar19 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            (*DAT_143262a20)(local_268);
            iVar3 = FUN_14023c4c0(local_268,&DAT_143a8b8d8);
            if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(iVar3);
            }
            (*DAT_143262a20)(&local_280);
            iVar3 = FUN_14023c4c0(&local_280,&DAT_143a8b8d8);
            if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(iVar3);
            }
            lStack_b0 = lStack_260;
            local_a8 = local_258;
            local_98 = local_280;
            uStack_94 = uStack_27c;
            uStack_90 = uStack_278;
            uStack_8c = uStack_274;
            local_88 = local_270;
            iVar3 = (**(code **)(*(longlong *)pIVar19 + 0x280))(pIVar19,0x20,&local_98,local_b8);
            if (iVar3 < 0) {
              _com_issue_errorex(iVar3,pIVar19,(_GUID *)&DAT_14327fcb0);
            }
            if ((short)local_280 == 8) {
              local_280 = local_280 & 0xffff0000;
              if (CONCAT44(uStack_274,uStack_278) != 0) {
                (*DAT_143ad5990)(CONCAT44(uStack_274,uStack_278) + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_280);
            }
            if (local_268[0] == 8) {
              local_268[0] = 0;
              if (lStack_260 != 0) {
                (*DAT_143ad5990)(lStack_260 + -4);
              }
            }
            else {
              (*DAT_143262a18)(local_268);
            }
          }
        }
        else {
          ppuVar11 = (undefined8 **)FUN_1408a9d80(local_1a8,0x56b);
          local_res8 = ppuVar11;
          (*DAT_143262a20)(&local_170);
          pIVar19 = pIVar15;
          if (*ppuVar11 != (undefined8 *)0x0) {
            pIVar19 = (IUnknown *)**ppuVar11;
          }
          iVar3 = (**(code **)(*(longlong *)pIVar13 + 0x28))(pIVar13,pIVar19,&local_170);
          if (iVar3 < 0) {
            _com_issue_errorex(iVar3,pIVar13,(_GUID *)&DAT_143272478);
          }
          local_208 = local_170;
          uStack_204 = uStack_16c;
          uStack_200 = uStack_168;
          uStack_1fc = uStack_164;
          local_1f8 = local_160;
          local_170 = local_170 & 0xffff0000;
          FUN_1401be120(ppuVar11);
          local_res18 = (undefined4 *)CONCAT44(local_res18._4_4_,0x3b);
          iVar3 = FUN_14022ee40(&local_208,0);
          *(uint *)((longlong)param_1 + 0x2e1c) = (uint)(iVar3 != 0);
          if ((short)local_208 == 8) {
            local_208 = local_208 & 0xffff0000;
            if (CONCAT44(uStack_1fc,uStack_200) != 0) {
              (*DAT_143ad5990)(CONCAT44(uStack_1fc,uStack_200) + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_208);
          }
          FUN_1401bb8d0(&local_310,PTR_u_tamingMob_143a464d0);
          local_res8 = &local_310;
          (*DAT_143262a20)(&local_158);
          pIVar19 = pIVar15;
          if (local_310 != (undefined8 *)0x0) {
            pIVar19 = (IUnknown *)*local_310;
          }
          iVar3 = (**(code **)(*(longlong *)pIVar13 + 0x28))(pIVar13,pIVar19,&local_158);
          if (iVar3 < 0) {
            _com_issue_errorex(iVar3,pIVar13,(_GUID *)&DAT_143272478);
          }
          local_1f0 = local_158;
          uStack_1ec = uStack_154;
          uStack_1e8 = uStack_150;
          uStack_1e4 = uStack_14c;
          local_1e0 = local_148;
          local_158 = local_158 & 0xffff0000;
          FUN_1401be120(&local_310);
          iVar3 = FUN_14022ee40(&local_1f0,1);
          *(uint *)(param_1 + 0x5c4) = (uint)(iVar3 != 0);
          if ((short)local_1f0 == 8) {
            local_1f0 = local_1f0 & 0xffff0000;
            if (CONCAT44(uStack_1e4,uStack_1e8) != 0) {
              (*DAT_143ad5990)(CONCAT44(uStack_1e4,uStack_1e8) + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_1f0);
          }
          if (((*(int *)(param_1 + 0x5c4) != 0) ||
              (iVar3 = FUN_140f8a990(param_1 + 0x20), iVar3 == 0)) ||
             (iVar3 = FUN_140f82720(param_1 + 0x20), iVar3 != 0)) {
            uVar10 = FUN_140404160(pIVar13,&local_1c0);
            FUN_14023b290(&local_2c8,uVar10);
            if (local_1c0 != (longlong *)0x0) {
              (**(code **)(*local_1c0 + 0x10))();
            }
            (*DAT_143262a20)(local_2e0);
            local_res8 = (undefined8 **)((ulonglong)local_res8 & 0xffffffff00000000);
            plVar12 = local_2c8;
            while( true ) {
              if (plVar12 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x80004003);
              }
              iVar3 = (**(code **)(*plVar12 + 0x18))(plVar12,1,local_2e0,&local_res8);
              if (iVar3 != 0) break;
              FUN_1401a5cf0(local_2c0,local_2e0);
              local_300 = local_2c0[0];
              if (local_2c0[0] != 0) {
                LOCK();
                *(int *)(local_2c0[0] + 0x10) = *(int *)(local_2c0[0] + 0x10) + 1;
                UNLOCK();
                plVar12 = local_2c8;
                pIVar13 = local_308;
                pIVar20 = local_368;
              }
              iVar3 = FUN_1402b2cf0(&local_300);
              if (-1 < iVar3) {
                piVar8 = (int *)FUN_1401abb40(param_1 + 0x5c7,0xffffffff);
                *piVar8 = iVar3;
              }
              FUN_1401be120(local_2c0);
              if (local_2e0[0] == 8) {
                local_2e0[0] = 0;
                if (local_2d8 != 0) {
                  (*DAT_143ad5990)(local_2d8 + -4);
                }
              }
              else {
                iVar3 = (*DAT_143262a18)(local_2e0);
                if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
                  FUN_142ef3ac0(iVar3);
                }
              }
            }
            FUN_1427f14e0(param_1);
            if (local_2e0[0] == 8) {
              local_2e0[0] = 0;
              if (local_2d8 != 0) {
                (*DAT_143ad5990)(local_2d8 + -4);
              }
            }
            else {
              (*DAT_143262a18)(local_2e0);
            }
            (**(code **)(*plVar12 + 0x10))(plVar12);
          }
        }
        local_358 = pIVar13;
        (**(code **)(*(longlong *)pIVar13 + 8))(pIVar13);
        uVar5 = FUN_140910eb0(&local_358,L"replacedStandAction",0);
        FUN_140fab990(param_1 + 0x20,uVar5);
      }
      local_res18 = (undefined4 *)FUN_1427beca0(param_1,1,0xffffffff);
      puVar9 = (undefined8 *)FUN_1408a9d20(&local_1b8,0x859);
      FUN_1401c21c0(&local_res20,*puVar9,local_res10[0]);
      if (local_1b8 != 0) {
        FUN_1401bebb0(local_1b8 + -0x10);
      }
      pIVar19 = local_res20;
      uVar10 = FUN_14090de10(local_140,local_res20);
      uVar10 = FUN_1409339d0(&local_2a0,uVar10);
      FUN_1401a5040(&local_1b0,uVar10);
      pIVar16 = local_1b0;
      pIVar17 = pIVar13;
      if (pIVar13 != local_1b0) {
        local_308 = local_1b0;
        pIVar16 = pIVar15;
        pIVar17 = local_1b0;
        if (pIVar13 != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)pIVar13 + 0x10))();
          pIVar16 = (IUnknown *)0x0;
        }
      }
      if (pIVar16 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)pIVar16 + 0x10))(pIVar16);
      }
      if (local_2a0 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_2a0 + 0x10))();
      }
      if (local_140[0] == 8) {
        local_140[0] = 0;
        if (local_138 != 0) {
          (*DAT_143ad5990)(local_138 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_140);
      }
      if (pIVar17 == (IUnknown *)0x0) {
        FUN_1427bee20(param_1,1,0xffffffff);
        puVar14 = local_res18;
      }
      else {
        FUN_1401bb8d0(local_res10,&DAT_14329e1d8);
        uVar10 = FUN_1401e4330(pIVar17,local_128,local_res10);
        uVar5 = FUN_14022ee40(uVar10,0xffffffff);
        *(undefined4 *)(param_1 + 0x5c9) = uVar5;
        if (local_128[0] == 8) {
          local_128[0] = 0;
          if (local_120 != 0) {
            (*DAT_143ad5990)(local_120 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_128);
        }
        FUN_1401bb8d0(&local_360,PTR_u_fixed_143a47ac8);
        uVar10 = FUN_1401e4330(pIVar17,local_110,&local_360);
        iVar3 = FUN_14022ee40(uVar10,0);
        *(bool *)((longlong)param_1 + 0x2e4c) = iVar3 != 0;
        if (local_110[0] == 8) {
          local_110[0] = 0;
          if (local_108 != 0) {
            (*DAT_143ad5990)(local_108 + -4);
          }
        }
        else {
          (*DAT_143262a18)(local_110);
        }
        puVar14 = local_res18;
        local_2e8 = &local_360;
        local_360 = (undefined8 *)0x0;
        local_res8 = (undefined8 **)CONCAT62(local_res8._2_6_,*(undefined2 *)(param_1 + 0x5c9));
        if (*(char *)((longlong)param_1 + 0x2e4c) == '\0') {
          if (param_1[0x1cb] == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          local_res10[0] = FUN_140d1a410();
        }
        else {
          local_res10[0] = 0;
        }
        FUN_1401bb8d0(&local_2f8,pIVar19);
        iVar3 = FUN_1427c91d0(param_1,&local_2f8,local_res10[0],puVar14 + 2,
                              (ulonglong)puVar21 & 0xffffffff00000000,local_res8._0_2_,&local_360,0,
                              0xff);
        if (iVar3 == 0) {
          FUN_1427bee20(param_1,1,0xffffffff);
        }
        else {
          *puVar14 = 0;
          puVar14[4] = 1;
          if (*(longlong *)(puVar14 + 6) == 0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          FUN_140dbb710(*(longlong *)(puVar14 + 6),0xffffffff);
          pIVar15 = *(IUnknown **)(puVar14 + 6);
          if (pIVar15 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          (*DAT_143262a20)(local_298);
          iVar3 = FUN_14023c4c0(local_298,&DAT_143a8b8d8);
          if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar3);
          }
          (*DAT_143262a20)(&local_250);
          iVar3 = FUN_14023c4c0(&local_250,&DAT_143a8b8d8);
          if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar3);
          }
          lStack_70 = lStack_290;
          local_68 = local_288;
          local_58 = local_250;
          uStack_54 = uStack_24c;
          uStack_50 = uStack_248;
          uStack_4c = uStack_244;
          local_48 = local_240;
          iVar3 = (**(code **)(*(longlong *)pIVar15 + 0x280))(pIVar15,0x20,&local_58,local_78);
          if (iVar3 < 0) {
            _com_issue_errorex(iVar3,pIVar15,(_GUID *)&DAT_14327fcb0);
          }
          if ((short)local_250 == 8) {
            local_250 = local_250 & 0xffff0000;
            if (CONCAT44(uStack_244,uStack_248) != 0) {
              (*DAT_143ad5990)(CONCAT44(uStack_244,uStack_248) + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_250);
          }
          if (local_298[0] == 8) {
            local_298[0] = 0;
            if (lStack_290 != 0) {
              (*DAT_143ad5990)(lStack_290 + -4);
            }
          }
          else {
            (*DAT_143262a18)(local_298);
          }
          FUN_1427ea880(param_1,0);
        }
      }
      if (param_1[0x6f7] == (undefined8 *)0x0) {
        if (*(int *)(param_1 + 0x260) != 0) {
          if (local_2f0[3] != (undefined8 *)0x0) {
            FUN_140d1a5b0(local_2f0[3],0);
          }
          if ((puVar14 != (undefined4 *)0x0) && (*(longlong *)(puVar14 + 6) != 0)) {
            FUN_140d1a5b0(*(longlong *)(puVar14 + 6),0);
          }
        }
      }
      else {
        FUN_1427c1410(param_1,0,0);
      }
      if (pIVar17 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)pIVar17 + 0x10))(pIVar17);
      }
      if (pIVar19 != (IUnknown *)0x0) {
        FUN_1401bebb0(pIVar19 + -0x10);
      }
      goto LAB_1427ea7b1;
    }
  }
  goto LAB_1427e9175;
}



//===========================================================
// FUN_1428336c0 @ 1428336c0   (1376 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1428336c0(longlong param_1)

{
  int *piVar1;
  longlong *plVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  code *pcVar5;
  void *pvVar6;
  undefined1 *puVar7;
  int iVar8;
  uint uVar9;
  longlong lVar10;
  int *piVar11;
  longlong lVar12;
  longlong *plVar13;
  ulonglong uVar14;
  undefined8 *puVar15;
  undefined8 uVar16;
  longlong *plVar17;
  int *piVar18;
  longlong lVar19;
  undefined8 *puVar20;
  uint uVar21;
  undefined8 *puVar22;
  longlong lVar23;
  longlong lVar24;
  ulonglong uVar25;
  bool bVar26;
  int local_res8 [2];
  undefined1 *local_res10;
  longlong *local_res18;
  undefined1 local_78 [8];
  int *local_70;
  undefined8 *local_68;
  ulonglong local_60;
  undefined4 local_58;
  undefined4 local_54;
  
  FUN_1429ba1f0(DAT_143ac1b90,*(undefined4 *)(param_1 + 0x10d0));
  local_res10 = (undefined1 *)(param_1 + 0x3bb0);
  FUN_14287f0c0();
  FUN_1408f6690();
  local_68 = (undefined8 *)0x0;
  local_60 = 0x1f;
  local_58 = 100;
  local_54 = 0x18;
  lVar24 = 1;
  piVar18 = (int *)(param_1 + 0x16d);
  do {
    if ((((0 < *piVar18) && (lVar10 = FUN_140388c60(DAT_143aa8328), lVar10 != 0)) &&
        (-1 < *(int *)(lVar10 + 0x2a0))) &&
       (piVar11 = (int *)FUN_1403de360(DAT_143aa8328), piVar11 != (int *)0x0)) {
      local_res8[0] = piVar11[1] + *piVar11;
      lVar10 = FUN_142876550(&local_68,local_res8);
      piVar1 = (int *)(lVar10 + 0x18);
      uVar21 = 0;
      iVar8 = FUN_1401c21f0();
      if (iVar8 != 0) {
        lVar19 = 0;
LAB_1428337b5:
        lVar12 = *(longlong *)(piVar11 + 8);
        if (lVar12 == 0) {
          uVar9 = 0;
        }
        else {
          uVar9 = *(uint *)(lVar12 + -8);
        }
        if (((int)uVar21 < 0) || (uVar9 <= uVar21)) {
          FUN_142e54290(0xc6,uVar21);
          lVar12 = *(longlong *)(piVar11 + 8);
        }
        FUN_140257e00(piVar1);
        uVar9 = 0;
        if (0 < *piVar1) {
          lVar23 = 0;
          do {
            if (((int)uVar9 < 0) || (99 < uVar9)) break;
            plVar17 = (longlong *)(lVar10 + 0x1c + lVar23);
            if ((int)*plVar17 == *(int *)(lVar12 + lVar19)) goto LAB_142833856;
            uVar9 = uVar9 + 1;
            lVar23 = lVar23 + 4;
          } while ((int)uVar9 < *piVar1);
        }
        if (*(longlong **)(lVar10 + 0x1b0) != (longlong *)0x0) {
          plVar2 = (longlong *)**(longlong **)(lVar10 + 0x1b0);
          plVar13 = (longlong *)*plVar2;
          if (plVar13 != plVar2) {
            do {
              plVar17 = plVar13 + 2;
              if ((int)*plVar17 == *(int *)(lVar12 + lVar19)) goto LAB_142833856;
              plVar13 = (longlong *)*plVar13;
            } while (plVar13 != plVar2);
          }
        }
        goto LAB_14283385b;
      }
    }
LAB_1428338cb:
    lVar24 = lVar24 + 1;
    piVar18 = piVar18 + 1;
  } while (lVar24 < 0x20);
  goto LAB_1428338df;
LAB_142833856:
  if (plVar17 == (longlong *)0x0) {
LAB_14283385b:
    lVar12 = *(longlong *)(piVar11 + 8);
    if (lVar12 == 0) {
      uVar9 = 0;
    }
    else {
      uVar9 = *(uint *)(lVar12 + -8);
    }
    if (((int)uVar21 < 0) || (uVar9 <= uVar21)) {
      FUN_142e54290(0xc6,uVar21);
      lVar12 = *(longlong *)(piVar11 + 8);
    }
    FUN_140257c50(piVar1,lVar12 + (longlong)(int)uVar21 * 4);
  }
  FUN_140257e00(piVar1);
  pvVar6 = Self;
  lVar12 = DAT_143adbcb0;
  if (*piVar1 == piVar11[3]) {
    local_70 = (int *)0x0;
    plVar17 = (longlong *)(DAT_143adbcb0 + 0x18);
    LOCK();
    lVar24 = *plVar17;
    if (lVar24 == 0) {
      *plVar17 = (longlong)Self;
    }
    UNLOCK();
    local_res18 = plVar17;
    if (lVar24 == 0) goto LAB_1428339b9;
    if ((void *)*plVar17 != pvVar6) goto LAB_142833990;
    *(int *)(lVar12 + 0x20) = *(int *)(lVar12 + 0x20) + 1;
    goto LAB_1428339c3;
  }
  uVar21 = uVar21 + 1;
  lVar19 = lVar19 + 4;
  uVar9 = FUN_1401c21f0();
  if (uVar9 <= uVar21) goto LAB_1428338cb;
  goto LAB_1428337b5;
LAB_142833990:
  pvVar6 = Self;
  LOCK();
  lVar24 = *plVar17;
  if (lVar24 == 0) {
    *plVar17 = (longlong)Self;
  }
  UNLOCK();
  if (lVar24 == 0) goto LAB_1428339b9;
  if ((void *)*plVar17 == pvVar6) goto LAB_1428339b1;
  (*DAT_143262828)(0);
  goto LAB_142833990;
LAB_1428339b9:
  *(undefined4 *)(lVar12 + 0x20) = 1;
  goto LAB_1428339c3;
LAB_1428339b1:
  *(int *)(lVar12 + 0x20) = *(int *)(lVar12 + 0x20) + 1;
LAB_1428339c3:
  piVar18 = (int *)(lVar12 + 0x20);
  puVar15 = *(undefined8 **)(lVar12 + 0x28);
  if (puVar15 == (undefined8 *)0x0) {
    puVar15 = (undefined8 *)FUN_14019d3c0(0x38,0x10);
    *(undefined8 **)(lVar12 + 0x28) = puVar15;
  }
  *(undefined8 *)(lVar12 + 0x28) = *puVar15;
  *piVar18 = *piVar18 + -1;
  if (*piVar18 == 0) {
    *plVar17 = 0;
  }
  *(undefined8 *)((longlong)puVar15 + 0x2c) = 0;
  *(undefined4 *)((longlong)puVar15 + 0x34) = 0;
  puVar15[3] = 0;
  puVar15[1] = 0;
  puVar15[2] = 0;
  *puVar15 = &PTR_FUN_143482af8;
  puVar15[4] = &PTR_LAB_143482b00;
  piVar18 = (int *)(puVar15 + 5);
  piVar18[0] = -1;
  piVar18[1] = -1;
  *(undefined4 *)(puVar15 + 6) = 1;
  *(undefined4 *)((longlong)puVar15 + 0x34) = 0xffffffff;
  if (puVar15[1] != 0) {
    FUN_142e541f0(0x2fe);
  }
  puVar15[1] = 1;
  local_70 = piVar18;
  if (piVar18 == (int *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  puVar7 = local_res10;
  iVar8 = piVar11[1];
  *piVar18 = *piVar11;
  *(int *)((longlong)puVar15 + 0x2c) = iVar8;
  if ((*(longlong *)(local_res10 + 8) - 1U < 999) || (*(longlong *)(local_res10 + 8) == -1)) {
    FUN_142e52ed0(0x447);
  }
  if (puVar7 == local_78) {
    FUN_142e52d50(0x45c,1);
  }
  if (piVar18 != (int *)0x0) {
    if (0xfffff < (ulonglong)puVar15[1]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    puVar15[1] = puVar15[1] + 1;
    UNLOCK();
  }
  FUN_14287f0c0(puVar7);
  *(int **)(puVar7 + 8) = piVar18;
  if (piVar18 != (int *)0x0) {
    if (0xffffe < puVar15[1] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar17 = puVar15 + 1;
    lVar24 = *plVar17;
    *plVar17 = *plVar17 + -1;
    UNLOCK();
    if ((int)lVar24 == 1) {
      if ((local_70 != (int *)0x0) && (puVar15[3] != 0)) {
        LOCK();
        *(undefined8 *)(puVar15[3] + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(puVar15[3] + 4) != 0);
      }
      if (puVar15 != (undefined8 *)0x0) {
        (**(code **)*puVar15)(puVar15,1);
      }
    }
  }
LAB_1428338df:
  puVar15 = local_68;
  if (local_68 == (undefined8 *)0x0) {
    return;
  }
  puVar20 = local_68 + (local_60 & 0xffffffff);
  pcVar5 = DAT_143ad5530;
  pvVar6 = Self;
  puVar22 = local_68;
  while (DAT_143ad5530 = pcVar5, Self = pvVar6, puVar22 < puVar20) {
    puVar3 = (undefined8 *)*puVar22;
    puVar22 = puVar22 + 1;
    while (pcVar5 = DAT_143ad5530, pvVar6 = Self, puVar3 != (undefined8 *)0x0) {
      puVar4 = (undefined8 *)puVar3[1];
      (**(code **)*puVar3)(puVar3,1);
      puVar3 = puVar4;
    }
  }
  uVar14 = puVar15[-1];
  if ((longlong)uVar14 < 0) {
    uVar14 = ~uVar14;
  }
  if (uVar14 < 0x21) {
    uVar25 = (ulonglong)(0x10 < uVar14);
  }
  else {
    if (uVar14 < 0x41) {
      uVar25 = 2;
      goto LAB_142833b53;
    }
    uVar25 = 0xffffffff;
    if (uVar14 < 0x81) {
      uVar25 = 3;
    }
  }
  if ((int)uVar25 < 0) {
    uVar16 = (*DAT_143ad5538)();
    (*pcVar5)(uVar16,0,puVar15 + -1);
    return;
  }
LAB_142833b53:
  lVar24 = uVar25 * 0x10;
  LOCK();
  bVar26 = *(longlong *)(&DAT_143ad68c8 + lVar24) == 0;
  if (bVar26) {
    *(void **)(&DAT_143ad68c8 + lVar24) = pvVar6;
  }
  UNLOCK();
  if (bVar26) {
LAB_142833be5:
    *(undefined4 *)(&DAT_143ad68d0 + lVar24) = 1;
  }
  else {
    if (*(void **)(&DAT_143ad68c8 + lVar24) != pvVar6) {
      while( true ) {
        pvVar6 = Self;
        LOCK();
        bVar26 = *(longlong *)(&DAT_143ad68c8 + lVar24) == 0;
        if (bVar26) {
          *(void **)(&DAT_143ad68c8 + lVar24) = Self;
        }
        UNLOCK();
        if (bVar26) goto LAB_142833be5;
        if (*(void **)(&DAT_143ad68c8 + lVar24) == pvVar6) break;
        (*DAT_143262828)(0);
      }
    }
    *(int *)(&DAT_143ad68d0 + lVar24) = *(int *)(&DAT_143ad68d0 + lVar24) + 1;
  }
  piVar18 = (int *)(&DAT_143ad68d0 + lVar24);
  *local_68 = *(undefined8 *)(&DAT_143ad6908 + uVar25 * 8);
  *(undefined8 **)(&DAT_143ad6908 + uVar25 * 8) = local_68;
  _DAT_143ad6948 = *local_68;
  *(int *)(&DAT_143ad68b4 + uVar25 * 4) = *(int *)(&DAT_143ad68b4 + uVar25 * 4) + -1;
  *piVar18 = *piVar18 + -1;
  if (*piVar18 == 0) {
    *(undefined8 *)(&DAT_143ad68c8 + lVar24) = 0;
  }
  return;
}



//===========================================================
// FUN_1427eb600 @ 1427eb600   (8026 bytes)
//===========================================================

void FUN_1427eb600(longlong param_1,int param_2)

{
  longlong lVar1;
  longlong *plVar2;
  IUnknown *pIVar3;
  char cVar4;
  int iVar5;
  uint uVar6;
  long lVar7;
  undefined8 *puVar8;
  undefined8 ******ppppppuVar9;
  undefined8 uVar10;
  undefined8 ******ppppppuVar11;
  int *piVar12;
  longlong *plVar13;
  int iVar14;
  undefined8 *****pppppuVar15;
  ulonglong uVar16;
  longlong lVar17;
  longlong lVar18;
  int iVar19;
  uint uVar20;
  undefined8 ******local_res18;
  undefined8 ******local_res20;
  undefined8 *puVar21;
  ulonglong *puVar22;
  short local_378;
  ushort uStack_376;
  undefined4 uStack_374;
  undefined8 uStack_370;
  undefined8 local_368;
  undefined8 local_358;
  undefined8 uStack_350;
  undefined8 local_348;
  short local_338;
  undefined2 uStack_336;
  undefined4 uStack_334;
  longlong lStack_330;
  undefined8 local_328;
  ulonglong local_318;
  longlong lStack_310;
  undefined8 local_308;
  short local_2f8;
  undefined2 uStack_2f6;
  undefined4 uStack_2f4;
  longlong lStack_2f0;
  undefined8 local_2e8;
  uint local_2d8;
  undefined4 uStack_2d4;
  IUnknown *pIStack_2d0;
  undefined8 local_2c8;
  IUnknown *local_2b8;
  IUnknown *local_2b0;
  IUnknown *local_2a8;
  uint local_2a0;
  IUnknown *local_298;
  int local_290;
  longlong local_288;
  longlong local_280;
  longlong local_278;
  IUnknown *local_270;
  IUnknown *local_268;
  longlong *local_260;
  longlong *local_258 [2];
  undefined4 local_248;
  undefined4 uStack_244;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  undefined8 local_238;
  undefined4 local_228;
  undefined4 uStack_224;
  undefined4 uStack_220;
  undefined4 uStack_21c;
  undefined8 local_218;
  undefined4 local_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined8 local_1f8;
  undefined4 local_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined8 local_1d8;
  undefined8 local_1c8;
  IUnknown *pIStack_1c0;
  undefined8 local_1b8;
  undefined8 local_1a8;
  longlong lStack_1a0;
  undefined8 local_198;
  ulonglong local_188;
  longlong lStack_180;
  undefined8 local_178;
  undefined8 local_168;
  longlong lStack_160;
  undefined8 local_158;
  undefined4 local_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined8 local_138;
  undefined8 local_128;
  IUnknown *pIStack_120;
  undefined8 local_118;
  undefined8 local_108;
  longlong lStack_100;
  undefined8 local_f8;
  ulonglong local_e8;
  longlong lStack_e0;
  undefined8 local_d8;
  undefined8 local_c8;
  longlong lStack_c0;
  undefined8 local_b8;
  undefined4 local_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 local_98;
  undefined4 local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 local_78;
  undefined4 local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  
  lVar1 = param_1 + 0x100;
  iVar5 = FUN_140f8abc0(lVar1);
  if (iVar5 == 0) {
    lVar18 = 0x40;
    lVar17 = -1;
    do {
      *(undefined4 *)(param_1 + 0x1568 + lVar18 * 0x30) = 0xffffffff;
      *(undefined4 *)(param_1 + 0x1578 + lVar18 * 0x30) = 1;
      plVar2 = *(longlong **)(param_1 + 0x1590 + lVar18 * 0x30);
      if (plVar2 != (longlong *)0x0) {
        (**(code **)(*plVar2 + 0x10))();
      }
      *(undefined8 *)(param_1 + 0x1590 + lVar18 * 0x30) = 0;
      plVar2 = *(longlong **)(param_1 + 0x1580 + lVar18 * 0x30);
      if (plVar2 != (longlong *)0x0) {
        (**(code **)(*plVar2 + 0x10))();
      }
      *(undefined8 *)(param_1 + 0x1580 + lVar18 * 0x30) = 0;
      plVar2 = *(longlong **)(param_1 + 0x1588 + lVar18 * 0x30);
      if (plVar2 != (longlong *)0x0) {
        (**(code **)(*plVar2 + 0x10))();
      }
      *(undefined8 *)(param_1 + 0x1588 + lVar18 * 0x30) = 0;
      if (lVar18 == 1) {
        *(undefined4 *)(param_1 + 0x1598) = 0;
      }
      lVar18 = lVar18 + 1;
    } while (lVar18 < 0x54);
    local_288 = param_1 + 0x2168;
    iVar5 = FUN_140f89f40(lVar1);
    if ((((iVar5 == 0) && (iVar5 = FUN_140f89e20(lVar1), iVar5 == 0)) &&
        (iVar5 = FUN_140fb7040(lVar1), iVar5 == 0)) &&
       ((cVar4 = FUN_140f80830(lVar1), cVar4 == '\0' && (param_2 != 0)))) {
      uVar20 = 0;
      local_280 = 0;
      local_278 = 0;
      puVar8 = (undefined8 *)FUN_1408a9d20(&local_res18,0x705);
      FUN_1401c21c0(&local_280,*puVar8,0x4df8f0);
      if (local_res18 != (undefined8 ******)0x0) {
        FUN_1401bebb0(local_res18 + -2);
      }
      puVar8 = (undefined8 *)FUN_1408a9d20(&local_res18,0x706);
      FUN_1401c21c0(&local_278,*puVar8,0x4df8f0);
      if (local_res18 != (undefined8 ******)0x0) {
        FUN_1401bebb0(local_res18 + -2);
      }
      pIVar3 = DAT_143add058;
      if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_318);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_318 == 8) {
          local_318 = (ulonglong)local_318._2_6_ << 0x10;
          if (lStack_310 != 0) {
            (*DAT_143ad5990)(lStack_310 + -4);
          }
        }
        else {
          iVar5 = (*DAT_143262a18)(&local_318);
          if (iVar5 < 0) goto LAB_1427ed392;
        }
        local_318 = CONCAT62(local_318._2_6_,8);
        uVar6 = uVar20;
        if (DAT_143a8b8e0 != 0) {
          uVar6 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
        }
        lStack_310 = FUN_1401a5fa0(DAT_143a8b8e0,uVar6);
      }
      else {
        if (((short)local_318 == 8) &&
           (local_318 = (ulonglong)local_318._2_6_ << 0x10, lStack_310 != 0)) {
          (*DAT_143ad5990)(lStack_310 + -4);
        }
        iVar5 = (*DAT_143262a28)(&local_318,&DAT_143a8b8d8);
        if (iVar5 < 0) {
LAB_1427ed392:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar5);
        }
      }
      (*DAT_143262a20)(&local_2f8);
      if (DAT_143a8b8d8 == 8) {
        if (local_2f8 == 8) {
          local_2f8 = 0;
          if (lStack_2f0 != 0) {
            (*DAT_143ad5990)(lStack_2f0 + -4);
          }
        }
        else {
          iVar5 = (*DAT_143262a18)(&local_2f8);
          if (iVar5 < 0) goto LAB_1427ed39a;
        }
        local_2f8 = 8;
        if (DAT_143a8b8e0 != 0) {
          uVar20 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
        }
        lStack_2f0 = FUN_1401a5fa0(DAT_143a8b8e0,uVar20);
      }
      else {
        if ((local_2f8 == 8) && (local_2f8 = 0, lStack_2f0 != 0)) {
          (*DAT_143ad5990)(lStack_2f0 + -4);
        }
        iVar5 = (*DAT_143262a28)(&local_2f8,&DAT_143a8b8d8);
        if (iVar5 < 0) {
LAB_1427ed39a:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar5);
        }
      }
      ppppppuVar9 = (undefined8 ******)FUN_14019b780(&DAT_143ad68a0,0x18);
      lVar1 = local_280;
      local_res20 = ppppppuVar9;
      if (ppppppuVar9 == (undefined8 ******)0x0) {
        local_res18 = (undefined8 ******)0x0;
      }
      else {
        ppppppuVar9[1] = (undefined8 *****)0x0;
        *(undefined4 *)(ppppppuVar9 + 2) = 1;
        local_res18 = ppppppuVar9;
        if (local_280 == 0) {
          *ppppppuVar9 = (undefined8 *****)0x0;
        }
        else {
          lVar18 = -1;
          do {
            lVar18 = lVar18 + 1;
          } while (*(short *)(local_280 + lVar18 * 2) != 0);
          uVar20 = (int)lVar18 + 1;
          piVar12 = (int *)(*DAT_143ad5980)((ulonglong)uVar20 * 2 + 4);
          if (piVar12 == (int *)0x0) {
            *ppppppuVar9 = (undefined8 *****)0x0;
LAB_1427ed3a7:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x8007000e);
          }
          *piVar12 = (int)lVar18 * 2;
          pppppuVar15 = (undefined8 *****)(piVar12 + 1);
          FUN_142ef7ba0(pppppuVar15,lVar1,(ulonglong)uVar20 * 2);
          *ppppppuVar9 = pppppuVar15;
          if (pppppuVar15 == (undefined8 *****)0x0) goto LAB_1427ed3a7;
        }
      }
      ppppppuVar9 = (undefined8 ******)0x0;
      if (local_res18 == (undefined8 ******)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x8007000e);
      }
      local_res20 = &local_res18;
      (*DAT_143262a20)(&local_2d8);
      ppppppuVar11 = ppppppuVar9;
      if (local_res18 != (undefined8 ******)0x0) {
        ppppppuVar11 = (undefined8 ******)*local_res18;
      }
      local_358 = local_318;
      uStack_350 = lStack_310;
      local_348 = local_308;
      local_338 = local_2f8;
      uStack_336 = uStack_2f6;
      uStack_334 = uStack_2f4;
      lStack_330 = lStack_2f0;
      local_328 = local_2e8;
      iVar5 = (**(code **)(*(longlong *)pIVar3 + 0x48))
                        (pIVar3,ppppppuVar11,&local_338,&local_358,&local_2d8);
      if (iVar5 < 0) {
        _com_issue_errorex(iVar5,pIVar3,(_GUID *)&DAT_1432743e8);
      }
      local_378 = (short)local_2d8;
      uStack_376 = (ushort)(local_2d8 >> 0x10);
      uStack_374 = uStack_2d4;
      uStack_370 = pIStack_2d0;
      local_368 = local_2c8;
      local_2d8 = (uint)uStack_376 << 0x10;
      FUN_1401be120(&local_res18);
      uVar10 = FUN_1409339d0(&local_2b0,&local_378);
      FUN_1403ee040(&local_270,uVar10);
      if (local_2b0 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_2b0 + 0x10))();
      }
      if (local_378 == 8) {
        local_378 = 0;
        if (uStack_370 != (IUnknown *)0x0) {
          (*DAT_143ad5990)((longlong)uStack_370 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_378);
      }
      if (local_2f8 == 8) {
        local_2f8 = 0;
        if (lStack_2f0 != 0) {
          (*DAT_143ad5990)(lStack_2f0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_2f8);
      }
      if ((short)local_318 == 8) {
        local_318 = local_318 & 0xffffffffffff0000;
        if (lStack_310 != 0) {
          (*DAT_143ad5990)(lStack_310 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_318);
      }
      pIVar3 = DAT_143add058;
      if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_358);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_358 == 8) {
          local_358 = (ulonglong)local_358._2_6_ << 0x10;
          if (uStack_350 != 0) {
            (*DAT_143ad5990)(uStack_350 + -4);
          }
        }
        else {
          iVar5 = (*DAT_143262a18)(&local_358);
          if (iVar5 < 0) goto LAB_1427ed3b2;
        }
        local_358 = CONCAT62(local_358._2_6_,8);
        ppppppuVar11 = ppppppuVar9;
        if (DAT_143a8b8e0 != 0) {
          ppppppuVar11 = (undefined8 ******)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        uStack_350 = FUN_1401a5fa0(DAT_143a8b8e0,ppppppuVar11);
      }
      else {
        if (((short)local_358 == 8) &&
           (local_358 = (ulonglong)local_358._2_6_ << 0x10, uStack_350 != 0)) {
          (*DAT_143ad5990)(uStack_350 + -4);
        }
        iVar5 = (*DAT_143262a28)(&local_358,&DAT_143a8b8d8);
        if (iVar5 < 0) {
LAB_1427ed3b2:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar5);
        }
      }
      (*DAT_143262a20)(&local_338);
      if (DAT_143a8b8d8 == 8) {
        if (local_338 == 8) {
          local_338 = 0;
          if (lStack_330 != 0) {
            (*DAT_143ad5990)(lStack_330 + -4);
          }
        }
        else {
          iVar5 = (*DAT_143262a18)(&local_338);
          if (iVar5 < 0) goto LAB_1427ed3ba;
        }
        local_338 = 8;
        ppppppuVar11 = ppppppuVar9;
        if (DAT_143a8b8e0 != 0) {
          ppppppuVar11 = (undefined8 ******)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
        }
        lStack_330 = FUN_1401a5fa0(DAT_143a8b8e0,ppppppuVar11);
      }
      else {
        if ((local_338 == 8) && (local_338 = 0, lStack_330 != 0)) {
          (*DAT_143ad5990)(lStack_330 + -4);
        }
        iVar5 = (*DAT_143262a28)(&local_338,&DAT_143a8b8d8);
        if (iVar5 < 0) {
LAB_1427ed3ba:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar5);
        }
      }
      ppppppuVar11 = (undefined8 ******)FUN_14019b780(&DAT_143ad68a0,0x18);
      lVar18 = local_278;
      local_res18 = ppppppuVar11;
      if (ppppppuVar11 != (undefined8 ******)0x0) {
        ppppppuVar11[1] = (undefined8 *****)0x0;
        *(undefined4 *)(ppppppuVar11 + 2) = 1;
        ppppppuVar9 = ppppppuVar11;
        if (local_278 != 0) {
          do {
            lVar17 = lVar17 + 1;
          } while (*(short *)(local_278 + lVar17 * 2) != 0);
          uVar20 = (int)lVar17 + 1;
          piVar12 = (int *)(*DAT_143ad5980)((ulonglong)uVar20 * 2 + 4);
          if (piVar12 == (int *)0x0) {
            *ppppppuVar11 = (undefined8 *****)0x0;
          }
          else {
            *piVar12 = (int)lVar17 * 2;
            pppppuVar15 = (undefined8 *****)(piVar12 + 1);
            FUN_142ef7ba0(pppppuVar15,lVar18,(ulonglong)uVar20 * 2);
            *ppppppuVar11 = pppppuVar15;
            if (pppppuVar15 != (undefined8 *****)0x0) goto LAB_1427ebd4f;
          }
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x8007000e);
        }
        *ppppppuVar11 = (undefined8 *****)0x0;
      }
LAB_1427ebd4f:
      pppppuVar15 = (undefined8 *****)0x0;
      local_res20 = ppppppuVar9;
      if (ppppppuVar9 == (undefined8 ******)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x8007000e);
      }
      local_res18 = &local_res20;
      (*DAT_143262a20)(&local_378);
      if (local_res20 != (undefined8 ******)0x0) {
        pppppuVar15 = *local_res20;
      }
      local_318 = local_358;
      lStack_310 = uStack_350;
      local_308 = local_348;
      local_2f8 = local_338;
      uStack_2f6 = uStack_336;
      uStack_2f4 = uStack_334;
      lStack_2f0 = lStack_330;
      local_2e8 = local_328;
      iVar5 = (**(code **)(*(longlong *)pIVar3 + 0x48))
                        (pIVar3,pppppuVar15,&local_2f8,&local_318,&local_378);
      if (iVar5 < 0) {
        _com_issue_errorex(iVar5,pIVar3,(_GUID *)&DAT_1432743e8);
      }
      local_2d8 = CONCAT22(uStack_376,local_378);
      uStack_2d4 = uStack_374;
      pIStack_2d0 = uStack_370;
      local_2c8 = local_368;
      local_378 = 0;
      FUN_1401be120(&local_res20);
      uVar10 = FUN_1409339d0(&local_2b8,&local_2d8);
      FUN_1403ee040(&local_268,uVar10);
      if (local_2b8 != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_2b8 + 0x10))();
      }
      if ((short)local_2d8 == 8) {
        local_2d8 = local_2d8 & 0xffff0000;
        if (pIStack_2d0 != (IUnknown *)0x0) {
          (*DAT_143ad5990)((longlong)pIStack_2d0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_2d8);
      }
      if (local_338 == 8) {
        local_338 = 0;
        if (lStack_330 != 0) {
          (*DAT_143ad5990)(lStack_330 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_338);
      }
      if ((short)local_358 == 8) {
        local_358 = local_358 & 0xffffffffffff0000;
        if (uStack_350 != 0) {
          (*DAT_143ad5990)(uStack_350 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_358);
      }
      local_290 = param_2 / 10;
      uVar20 = param_2 % 10;
      local_2a0 = local_290 + uVar20;
      local_298 = (IUnknown *)0x0;
      if (DAT_143ad48a0 == (code *)0x0) {
        iVar5 = -0x7ffbfe10;
      }
      else {
        iVar5 = (*DAT_143ad48a0)(PTR_u_Shape2D_Vector2D_143a479e8,&DAT_143273488,&local_298,0);
        pIVar3 = local_298;
        if (-1 < iVar5) {
          if (local_298 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          uStack_370 = *(IUnknown **)(param_1 + 0xe00);
          local_378 = 0xd;
          if (uStack_370 != (IUnknown *)0x0) {
            (**(code **)(*(longlong *)uStack_370 + 8))();
          }
          local_2d8 = CONCAT22(uStack_376,local_378);
          uStack_2d4 = uStack_374;
          pIStack_2d0 = uStack_370;
          local_2c8 = local_368;
          iVar5 = (**(code **)(*(longlong *)pIVar3 + 200))(pIVar3,&local_2d8);
          if (iVar5 < 0) {
            _com_issue_errorex(iVar5,pIVar3,(_GUID *)&DAT_143273488);
          }
          if (local_378 == 8) {
            local_378 = 0;
            if (uStack_370 != (IUnknown *)0x0) {
              (*DAT_143ad5990)(uStack_370 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_378);
          }
          uVar16 = 0;
          local_res20 = (undefined8 ******)((ulonglong)local_res20 & 0xffffffff00000000);
          if (local_2a0 == 0) {
LAB_1427ed336:
            if (local_298 != (IUnknown *)0x0) {
              (**(code **)(*(longlong *)local_298 + 0x10))();
            }
            if (local_268 != (IUnknown *)0x0) {
              (**(code **)(*(longlong *)local_268 + 0x10))(local_268);
            }
            if (local_270 != (IUnknown *)0x0) {
              (**(code **)(*(longlong *)local_270 + 0x10))(local_270);
            }
            if (lVar18 != 0) {
              FUN_1401bebb0(lVar18 + -0x10);
            }
            if (lVar1 == 0) {
              return;
            }
            FUN_1401bebb0(lVar1 + -0x10);
            return;
          }
          local_res18 = (undefined8 ******)CONCAT44(local_res18._4_4_,0x800401f0);
          iVar5 = -0x7ffbfe10;
          do {
            local_2a8 = (IUnknown *)0x0;
            if (DAT_143ad48a0 != (code *)0x0) {
              iVar5 = (*DAT_143ad48a0)(PTR_u_Shape2D_Vector2D_143a479e8,&DAT_143273488,&local_2a8,0)
              ;
            }
            pIVar3 = local_2a8;
            if (iVar5 < 0) {
              FUN_1401a59c0(&local_2d8,iVar5,0,0);
                    /* WARNING: Subroutine does not return */
              _CxxThrowException(&local_2d8,(ThrowInfo *)&DAT_143a3b0c0);
            }
            if (local_2a8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            local_378 = 0xd;
            uStack_370 = local_298;
            if (local_298 != (IUnknown *)0x0) {
              (**(code **)(*(longlong *)local_298 + 8))();
            }
            local_228 = CONCAT22(uStack_376,local_378);
            uStack_224 = uStack_374;
            uStack_220 = (undefined4)uStack_370;
            uStack_21c = uStack_370._4_4_;
            local_218 = local_368;
            iVar5 = (**(code **)(*(longlong *)pIVar3 + 200))(pIVar3,&local_228);
            if (iVar5 < 0) {
              _com_issue_errorex(iVar5,pIVar3,(_GUID *)&DAT_143273488);
            }
            if (local_378 == 8) {
              local_378 = 0;
              if (uStack_370 != (IUnknown *)0x0) {
                (*DAT_143ad5990)(uStack_370 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_378);
            }
            local_2b0 = (IUnknown *)0x0;
            if (DAT_143ad48a0 == (code *)0x0) {
              iVar14 = (int)local_res18;
              iVar5 = (int)local_res18;
            }
            else {
              iVar14 = (*DAT_143ad48a0)(PTR_u_Shape2D_Vector2D_143a479e8,&DAT_143273488,&local_2b0,0
                                       );
              iVar5 = (int)local_res18;
            }
            if (iVar14 < 0) {
              FUN_1401a59c0(&local_2d8,iVar14,0,0);
                    /* WARNING: Subroutine does not return */
              _CxxThrowException(&local_2d8,(ThrowInfo *)&DAT_143a3b0c0);
            }
            uVar6 = (uint)uVar16;
            if (uVar6 < uVar20) {
              iVar19 = (int)(uVar16 / 5);
              if ((int)uVar20 % 10 < 6) {
                iVar14 = (uVar6 + iVar19 * -5) * 2;
              }
              else {
                if (uVar6 + (int)(uVar16 / 10) * -10 < 5) {
                  iVar14 = (uVar6 + iVar19 * -5 + -2) * 10;
                  iVar19 = (-2 - iVar19) * 10;
                  goto LAB_1427ec233;
                }
                iVar14 = ((int)uVar20 / 5 + iVar19 * -2) * 5 + uVar6 * 2;
              }
              iVar14 = ((iVar14 - uVar20) + 1) * 5;
              iVar19 = (-2 - iVar19) * 10;
            }
            else {
              iVar14 = (((local_290 / 5) * 5 - local_290) + 1) * 7 + ((uVar6 - uVar20) % 5) * 0xf;
              iVar19 = ((uVar6 - uVar20) / 5) * -0xf + ((int)(uVar20 - 1) / 5) * -10 + -0x23;
            }
LAB_1427ec233:
            local_2b8 = (IUnknown *)0x0;
            if (DAT_143ad48a0 != (code *)0x0) {
              iVar5 = (*DAT_143ad48a0)(PTR_u_Shape2D_Vector2D_143a479e8,&DAT_143273488,&local_2b8,0)
              ;
            }
            pIVar3 = local_2b8;
            if (iVar5 < 0) {
              FUN_1401a59c0(&local_2d8,iVar5,0,0);
                    /* WARNING: Subroutine does not return */
              _CxxThrowException(&local_2d8,(ThrowInfo *)&DAT_143a3b0c0);
            }
            if (local_2b8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            (*DAT_143262a20)(&local_358);
            if (DAT_143a8b8d8 == 8) {
              if ((short)local_358 == 8) {
                local_358 = (ulonglong)local_358._2_6_ << 0x10;
                if (uStack_350 != 0) {
                  (*DAT_143ad5990)(uStack_350 + -4);
                }
              }
              else {
                iVar5 = (*DAT_143262a18)(&local_358);
                if (iVar5 < 0) goto LAB_1427ed501;
              }
              local_358 = CONCAT62(local_358._2_6_,8);
              if (DAT_143a8b8e0 == 0) {
                uStack_350 = FUN_1401a5fa0(0,0);
              }
              else {
                uStack_350 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
              }
            }
            else {
              if (((short)local_358 == 8) &&
                 (local_358 = (ulonglong)local_358._2_6_ << 0x10, uStack_350 != 0)) {
                (*DAT_143ad5990)(uStack_350 + -4);
              }
              iVar5 = (*DAT_143262a28)(&local_358,&DAT_143a8b8d8);
              if (iVar5 < 0) {
LAB_1427ed501:
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
            }
            uVar6 = FUN_1407386b0(&DAT_143ac1ab0);
            local_208 = (undefined4)local_358;
            uStack_204 = local_358._4_4_;
            uStack_200 = (undefined4)uStack_350;
            uStack_1fc = uStack_350._4_4_;
            local_1f8 = local_348;
            lVar7 = (**(code **)(*(longlong *)pIVar3 + 0x160))
                              (pIVar3,(double)(uVar6 % 0x168),&local_208);
            if (lVar7 < 0) {
              _com_issue_errorex(lVar7,pIVar3,(_GUID *)&DAT_143273488);
            }
            if ((short)local_358 == 8) {
              local_358 = local_358 & 0xffffffffffff0000;
              if (uStack_350 != 0) {
                (*DAT_143ad5990)(uStack_350 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_358);
            }
            pIVar3 = local_2b0;
            if (local_2b0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            local_378 = 0xd;
            uStack_370 = local_2b8;
            if (local_2b8 != (IUnknown *)0x0) {
              (**(code **)(*(longlong *)local_2b8 + 8))();
            }
            local_1e8 = CONCAT22(uStack_376,local_378);
            uStack_1e4 = uStack_374;
            uStack_1e0 = (undefined4)uStack_370;
            uStack_1dc = uStack_370._4_4_;
            local_1d8 = local_368;
            lVar7 = (**(code **)(*(longlong *)pIVar3 + 200))(pIVar3,&local_1e8);
            if (lVar7 < 0) {
              _com_issue_errorex(lVar7,pIVar3,(_GUID *)&DAT_143273488);
            }
            if (local_378 == 8) {
              local_378 = 0;
              if (uStack_370 != (IUnknown *)0x0) {
                (*DAT_143ad5990)(uStack_370 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_378);
            }
            pIVar3 = local_2a8;
            if (local_2a8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            (*DAT_143262a20)(&local_378);
            if (DAT_143a8b8d8 == 8) {
              if (local_378 == 8) {
                local_378 = 0;
                if (uStack_370 != (IUnknown *)0x0) {
                  (*DAT_143ad5990)(uStack_370 + -4);
                }
              }
              else {
                iVar5 = (*DAT_143262a18)(&local_378);
                if (iVar5 < 0) goto LAB_1427ed4e3;
              }
              local_378 = 8;
              if (DAT_143a8b8e0 == 0) {
                uStack_370 = (IUnknown *)FUN_1401a5fa0(0,0);
              }
              else {
                uStack_370 = (IUnknown *)
                             FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
              }
            }
            else {
              if ((local_378 == 8) && (local_378 = 0, uStack_370 != (IUnknown *)0x0)) {
                (*DAT_143ad5990)(uStack_370 + -4);
              }
              iVar5 = (*DAT_143262a28)(&local_378,&DAT_143a8b8d8);
              if (iVar5 < 0) {
LAB_1427ed4e3:
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
            }
            (*DAT_143262a20)(&local_2f8);
            if (DAT_143a8b8d8 == 8) {
              if (local_2f8 == 8) {
                local_2f8 = 0;
                if (lStack_2f0 != 0) {
                  (*DAT_143ad5990)(lStack_2f0 + -4);
                }
              }
              else {
                iVar5 = (*DAT_143262a18)(&local_2f8);
                if (iVar5 < 0) goto LAB_1427ed4db;
              }
              local_2f8 = 8;
              if (DAT_143a8b8e0 == 0) {
                lStack_2f0 = FUN_1401a5fa0(0,0);
              }
              else {
                lStack_2f0 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
              }
            }
            else {
              if ((local_2f8 == 8) && (local_2f8 = 0, lStack_2f0 != 0)) {
                (*DAT_143ad5990)(lStack_2f0 + -4);
              }
              iVar5 = (*DAT_143262a28)(&local_2f8,&DAT_143a8b8d8);
              if (iVar5 < 0) {
LAB_1427ed4db:
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
            }
            (*DAT_143262a20)(&local_318);
            if (DAT_143a8b8d8 == 8) {
              if ((short)local_318 == 8) {
                local_318 = (ulonglong)local_318._2_6_ << 0x10;
                if (lStack_310 != 0) {
                  (*DAT_143ad5990)(lStack_310 + -4);
                }
              }
              else {
                iVar5 = (*DAT_143262a18)(&local_318);
                if (iVar5 < 0) goto LAB_1427ed4d3;
              }
              local_318 = CONCAT62(local_318._2_6_,8);
              if (DAT_143a8b8e0 == 0) {
                lStack_310 = FUN_1401a5fa0(0,0);
              }
              else {
                lStack_310 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
              }
            }
            else {
              if (((short)local_318 == 8) &&
                 (local_318 = (ulonglong)local_318._2_6_ << 0x10, lStack_310 != 0)) {
                (*DAT_143ad5990)(lStack_310 + -4);
              }
              iVar5 = (*DAT_143262a28)(&local_318,&DAT_143a8b8d8);
              if (iVar5 < 0) {
LAB_1427ed4d3:
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
            }
            (*DAT_143262a20)(&local_338);
            if (DAT_143a8b8d8 == 8) {
              if (local_338 == 8) {
                local_338 = 0;
                if (lStack_330 != 0) {
                  (*DAT_143ad5990)(lStack_330 + -4);
                }
              }
              else {
                iVar5 = (*DAT_143262a18)(&local_338);
                if (iVar5 < 0) goto LAB_1427ed4cb;
              }
              local_338 = 8;
              if (DAT_143a8b8e0 == 0) {
                lStack_330 = FUN_1401a5fa0(0,0);
              }
              else {
                lStack_330 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
              }
            }
            else {
              if ((local_338 == 8) && (local_338 = 0, lStack_330 != 0)) {
                (*DAT_143ad5990)(lStack_330 + -4);
              }
              iVar5 = (*DAT_143262a28)(&local_338,&DAT_143a8b8d8);
              if (iVar5 < 0) {
LAB_1427ed4cb:
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
            }
            (*DAT_143262a20)(&local_358);
            if (DAT_143a8b8d8 == 8) {
              if ((short)local_358 == 8) {
                local_358 = (ulonglong)local_358._2_6_ << 0x10;
                if (uStack_350 != 0) {
                  (*DAT_143ad5990)(uStack_350 + -4);
                }
              }
              else {
                iVar5 = (*DAT_143262a18)(&local_358);
                if (iVar5 < 0) goto LAB_1427ed4c3;
              }
              local_358 = CONCAT62(local_358._2_6_,8);
              if (DAT_143a8b8e0 == 0) {
                uStack_350 = FUN_1401a5fa0(0,0);
              }
              else {
                uStack_350 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
              }
            }
            else {
              if (((short)local_358 == 8) &&
                 (local_358 = (ulonglong)local_358._2_6_ << 0x10, uStack_350 != 0)) {
                (*DAT_143ad5990)(uStack_350 + -4);
              }
              iVar5 = (*DAT_143262a28)(&local_358,&DAT_143a8b8d8);
              if (iVar5 < 0) {
LAB_1427ed4c3:
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
            }
            local_1c8 = CONCAT44(uStack_374,CONCAT22(uStack_376,local_378));
            pIStack_1c0 = uStack_370;
            local_1b8 = local_368;
            local_1a8 = CONCAT44(uStack_2f4,CONCAT22(uStack_2f6,local_2f8));
            lStack_1a0 = lStack_2f0;
            local_198 = local_2e8;
            local_188 = local_318;
            lStack_180 = lStack_310;
            local_178 = local_308;
            local_168 = CONCAT44(uStack_334,CONCAT22(uStack_336,local_338));
            lStack_160 = lStack_330;
            local_158 = local_328;
            local_148 = (undefined4)local_358;
            uStack_144 = local_358._4_4_;
            uStack_140 = (undefined4)uStack_350;
            uStack_13c = uStack_350._4_4_;
            local_138 = local_348;
            puVar8 = &local_168;
            iVar5 = (**(code **)(*(longlong *)pIVar3 + 0x140))
                              (pIVar3,iVar14,iVar19,&local_148,puVar8,&local_188,&local_1a8,
                               &local_1c8);
            if (iVar5 < 0) {
              _com_issue_errorex(iVar5,pIVar3,(_GUID *)&DAT_143273488);
            }
            if ((short)local_358 == 8) {
              local_358 = local_358 & 0xffffffffffff0000;
              if (uStack_350 != 0) {
                (*DAT_143ad5990)(uStack_350 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_358);
            }
            if (local_338 == 8) {
              local_338 = 0;
              if (lStack_330 != 0) {
                (*DAT_143ad5990)(lStack_330 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_338);
            }
            if ((short)local_318 == 8) {
              local_318 = local_318 & 0xffffffffffff0000;
              if (lStack_310 != 0) {
                (*DAT_143ad5990)(lStack_310 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_318);
            }
            if (local_2f8 == 8) {
              local_2f8 = 0;
              if (lStack_2f0 != 0) {
                (*DAT_143ad5990)(lStack_2f0 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_2f8);
            }
            if (local_378 == 8) {
              local_378 = 0;
              if (uStack_370 != (IUnknown *)0x0) {
                (*DAT_143ad5990)(uStack_370 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_378);
            }
            pIVar3 = local_2a8;
            if (local_2a8 == (IUnknown *)0x0) {
LAB_1427ed4b8:
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            iVar5 = (**(code **)(*(longlong *)local_2a8 + 0x150))
                              (local_2a8,local_2b0,5,5,(ulonglong)puVar8 & 0xffffffff00000000,5);
            if (iVar5 < 0) {
              _com_issue_errorex(iVar5,pIVar3,(_GUID *)&DAT_143273488);
            }
            pIVar3 = local_2b0;
            if (local_2b0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            (*DAT_143262a20)(&local_378);
            if (DAT_143a8b8d8 == 8) {
              if (local_378 == 8) {
                local_378 = 0;
                if (uStack_370 != (IUnknown *)0x0) {
                  (*DAT_143ad5990)(uStack_370 + -4);
                }
              }
              else {
                iVar5 = (*DAT_143262a18)(&local_378);
                if (iVar5 < 0) goto LAB_1427ed4a5;
              }
              local_378 = 8;
              if (DAT_143a8b8e0 == 0) {
                uStack_370 = (IUnknown *)FUN_1401a5fa0(0,0);
              }
              else {
                uStack_370 = (IUnknown *)
                             FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
              }
            }
            else {
              if ((local_378 == 8) && (local_378 = 0, uStack_370 != (IUnknown *)0x0)) {
                (*DAT_143ad5990)(uStack_370 + -4);
              }
              iVar5 = (*DAT_143262a28)(&local_378,&DAT_143a8b8d8);
              if (iVar5 < 0) {
LAB_1427ed4a5:
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
            }
            (*DAT_143262a20)(&local_2f8);
            iVar5 = FUN_14023c4c0(&local_2f8,&DAT_143a8b8d8);
            if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(iVar5);
            }
            (*DAT_143262a20)(&local_318);
            iVar5 = FUN_14023c4c0(&local_318,&DAT_143a8b8d8);
            if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(iVar5);
            }
            (*DAT_143262a20)(&local_338);
            iVar5 = FUN_14023c4c0(&local_338,&DAT_143a8b8d8);
            if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(iVar5);
            }
            (*DAT_143262a20)(&local_358);
            iVar5 = FUN_14023c4c0(&local_358,&DAT_143a8b8d8);
            if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(iVar5);
            }
            local_128 = CONCAT44(uStack_374,CONCAT22(uStack_376,local_378));
            pIStack_120 = uStack_370;
            local_118 = local_368;
            local_108 = CONCAT44(uStack_2f4,CONCAT22(uStack_2f6,local_2f8));
            lStack_100 = lStack_2f0;
            local_f8 = local_2e8;
            local_e8 = local_318;
            lStack_e0 = lStack_310;
            local_d8 = local_308;
            local_c8 = CONCAT44(uStack_334,CONCAT22(uStack_336,local_338));
            lStack_c0 = lStack_330;
            local_b8 = local_328;
            local_a8 = (undefined4)local_358;
            uStack_a4 = local_358._4_4_;
            uStack_a0 = (undefined4)uStack_350;
            uStack_9c = uStack_350._4_4_;
            local_98 = local_348;
            puVar8 = &local_108;
            puVar22 = &local_e8;
            puVar21 = &local_c8;
            iVar5 = (**(code **)(*(longlong *)pIVar3 + 0x140))
                              (pIVar3,0,0xfffffffb,&local_a8,puVar21,puVar22,puVar8,&local_128);
            if (iVar5 < 0) {
              _com_issue_errorex(iVar5,pIVar3,(_GUID *)&DAT_143273488);
            }
            if ((short)local_358 == 8) {
              local_358 = local_358 & 0xffffffffffff0000;
              if (uStack_350 != 0) {
                (*DAT_143ad5990)(uStack_350 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_358);
            }
            if (local_338 == 8) {
              local_338 = 0;
              if (lStack_330 != 0) {
                (*DAT_143ad5990)(lStack_330 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_338);
            }
            if ((short)local_318 == 8) {
              local_318 = local_318 & 0xffffffffffff0000;
              if (lStack_310 != 0) {
                (*DAT_143ad5990)(lStack_310 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_318);
            }
            if (local_2f8 == 8) {
              local_2f8 = 0;
              if (lStack_2f0 != 0) {
                (*DAT_143ad5990)(lStack_2f0 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_2f8);
            }
            if (local_378 == 8) {
              local_378 = 0;
              if (uStack_370 != (IUnknown *)0x0) {
                (*DAT_143ad5990)(uStack_370 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_378);
            }
            pIVar3 = local_2b8;
            if (local_2b8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            local_378 = 3;
            uStack_370 = (IUnknown *)CONCAT44(uStack_370._4_4_,2000);
            local_88 = CONCAT22(uStack_376,3);
            uStack_84 = uStack_374;
            uStack_80 = 2000;
            uStack_7c = uStack_370._4_4_;
            local_78 = local_368;
            iVar5 = (**(code **)(*(longlong *)local_2b8 + 0x160))(local_2b8,0);
            if (iVar5 < 0) {
              _com_issue_errorex(iVar5,pIVar3,(_GUID *)&DAT_143273488);
            }
            if (local_378 == 8) {
              local_378 = 0;
              if (uStack_370 != (IUnknown *)0x0) {
                (*DAT_143ad5990)((longlong)uStack_370 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_378);
            }
            lVar17 = DAT_143add050;
            uVar6 = (uint)local_res20;
            if ((uint)local_res20 < uVar20) {
              if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x80004003);
              }
              (*DAT_143262a20)(&local_338);
              iVar5 = FUN_14023c4c0(&local_338,&DAT_143a8b8d8);
              if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
              (*DAT_143262a20)(&local_358);
              iVar5 = FUN_14023c4c0(&local_358,&DAT_143a8b8d8);
              if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
              (*DAT_143262a20)(&local_378);
              iVar5 = FUN_14023c4c0(&local_378,&DAT_143a8b8d8);
              if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
              local_2d8 = CONCAT22(local_2d8._2_2_,0xd);
              pIStack_2d0 = local_270;
              if (local_270 != (IUnknown *)0x0) {
                (**(code **)(*(longlong *)local_270 + 8))(local_270);
              }
              plVar13 = (longlong *)
                        FUN_140d8d300(lVar17,&local_260,0,0,(ulonglong)puVar21 & 0xffffffff00000000,
                                      (ulonglong)puVar22 & 0xffffffff00000000,
                                      (ulonglong)puVar8 & 0xffffffff00000000,&local_2d8,&local_378,
                                      &local_358,&local_338);
              lVar17 = local_288;
              plVar2 = *(longlong **)(local_288 + 0x18);
              if (plVar2 != (longlong *)*plVar13) {
                *(longlong **)(local_288 + 0x18) = (longlong *)*plVar13;
                *plVar13 = 0;
                if (plVar2 != (longlong *)0x0) {
                  (**(code **)(*plVar2 + 0x10))();
                }
              }
              if (local_260 != (longlong *)0x0) {
                (**(code **)(*local_260 + 0x10))();
              }
              if ((short)local_2d8 == 8) {
                local_2d8 = local_2d8 & 0xffff0000;
                if (pIStack_2d0 != (IUnknown *)0x0) {
                  (*DAT_143ad5990)(pIStack_2d0 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_2d8);
              }
              if (local_378 == 8) {
                local_378 = 0;
                if (uStack_370 != (IUnknown *)0x0) {
                  (*DAT_143ad5990)((longlong)uStack_370 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_378);
              }
              if ((short)local_358 == 8) {
                local_358 = local_358 & 0xffffffffffff0000;
                if (uStack_350 != 0) {
                  (*DAT_143ad5990)(uStack_350 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_358);
              }
            }
            else {
              if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x80004003);
              }
              (*DAT_143262a20)(&local_338);
              iVar5 = FUN_14023c4c0(&local_338,&DAT_143a8b8d8);
              if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
              (*DAT_143262a20)(&local_358);
              iVar5 = FUN_14023c4c0(&local_358,&DAT_143a8b8d8);
              if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
              (*DAT_143262a20)(&local_378);
              iVar5 = FUN_14023c4c0(&local_378,&DAT_143a8b8d8);
              if (iVar5 < 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar5);
              }
              local_2d8 = CONCAT22(local_2d8._2_2_,0xd);
              pIStack_2d0 = local_268;
              if (local_268 != (IUnknown *)0x0) {
                (**(code **)(*(longlong *)local_268 + 8))(local_268);
              }
              plVar13 = (longlong *)
                        FUN_140d8d300(lVar17,local_258,0,0,(ulonglong)puVar21 & 0xffffffff00000000,
                                      (ulonglong)puVar22 & 0xffffffff00000000,
                                      (ulonglong)puVar8 & 0xffffffff00000000,&local_2d8,&local_378,
                                      &local_358,&local_338);
              lVar17 = local_288;
              plVar2 = *(longlong **)(local_288 + 0x18);
              if (plVar2 != (longlong *)*plVar13) {
                *(longlong **)(local_288 + 0x18) = (longlong *)*plVar13;
                *plVar13 = 0;
                if (plVar2 != (longlong *)0x0) {
                  (**(code **)(*plVar2 + 0x10))();
                }
              }
              if (local_258[0] != (longlong *)0x0) {
                (**(code **)(*local_258[0] + 0x10))();
              }
              if ((short)local_2d8 == 8) {
                local_2d8 = local_2d8 & 0xffff0000;
                if (pIStack_2d0 != (IUnknown *)0x0) {
                  (*DAT_143ad5990)(pIStack_2d0 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_2d8);
              }
              if (local_378 == 8) {
                local_378 = 0;
                if (uStack_370 != (IUnknown *)0x0) {
                  (*DAT_143ad5990)((longlong)uStack_370 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_378);
              }
              if ((short)local_358 == 8) {
                local_358 = local_358 & 0xffffffffffff0000;
                if (uStack_350 != 0) {
                  (*DAT_143ad5990)(uStack_350 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_358);
              }
            }
            if (local_338 == 8) {
              local_338 = 0;
              if (lStack_330 != 0) {
                (*DAT_143ad5990)(lStack_330 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_338);
            }
            pIVar3 = *(IUnknown **)(lVar17 + 0x18);
            if (pIVar3 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            uStack_370 = *(IUnknown **)(param_1 + 0xe58);
            local_378 = 0xd;
            if (uStack_370 != (IUnknown *)0x0) {
              (**(code **)(*(longlong *)uStack_370 + 8))();
            }
            local_68 = CONCAT22(uStack_376,local_378);
            uStack_64 = uStack_374;
            uStack_60 = (undefined4)uStack_370;
            uStack_5c = uStack_370._4_4_;
            local_58 = local_368;
            lVar7 = (**(code **)(*(longlong *)pIVar3 + 0x238))(pIVar3,&local_68);
            if (lVar7 < 0) {
              _com_issue_errorex(lVar7,pIVar3,(_GUID *)&DAT_14327fcb0);
            }
            if (local_378 == 8) {
              local_378 = 0;
              if (uStack_370 != (IUnknown *)0x0) {
                (*DAT_143ad5990)(uStack_370 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_378);
            }
            pIVar3 = *(IUnknown **)(lVar17 + 0x18);
            if (pIVar3 == (IUnknown *)0x0) goto LAB_1427ed4b8;
            iVar5 = (**(code **)(*(longlong *)pIVar3 + 0x198))(pIVar3,5);
            if (iVar5 < 0) {
              _com_issue_errorex(iVar5,pIVar3,(_GUID *)&DAT_14327fcb0);
            }
            pIVar3 = *(IUnknown **)(lVar17 + 0x18);
            if (pIVar3 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            local_378 = 0xd;
            uStack_370 = local_2a8;
            if (local_2a8 != (IUnknown *)0x0) {
              (**(code **)(*(longlong *)local_2a8 + 8))();
            }
            local_248 = CONCAT22(uStack_376,local_378);
            uStack_244 = uStack_374;
            uStack_240 = (undefined4)uStack_370;
            uStack_23c = uStack_370._4_4_;
            local_238 = local_368;
            lVar7 = (**(code **)(*(longlong *)pIVar3 + 200))(pIVar3,&local_248);
            if (lVar7 < 0) {
              _com_issue_errorex(lVar7,pIVar3,(_GUID *)&DAT_143273488);
            }
            if (local_378 == 8) {
              local_378 = 0;
              if (uStack_370 != (IUnknown *)0x0) {
                (*DAT_143ad5990)(uStack_370 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_378);
            }
            pIVar3 = *(IUnknown **)(lVar17 + 0x18);
            if (pIVar3 == (IUnknown *)0x0) goto LAB_1427ed4b8;
            iVar5 = (**(code **)(*(longlong *)pIVar3 + 0x200))(pIVar3,0xffffffff);
            if (iVar5 < 0) {
              _com_issue_errorex(iVar5,pIVar3,(_GUID *)&DAT_14327fcb0);
            }
            *(undefined4 *)(lVar17 + 0x10) = 0;
            if (local_2b8 != (IUnknown *)0x0) {
              (**(code **)(*(longlong *)local_2b8 + 0x10))();
            }
            if (local_2b0 != (IUnknown *)0x0) {
              (**(code **)(*(longlong *)local_2b0 + 0x10))();
            }
            if (local_2a8 != (IUnknown *)0x0) {
              (**(code **)(*(longlong *)local_2a8 + 0x10))();
            }
            uVar6 = uVar6 + 1;
            uVar16 = (ulonglong)uVar6;
            local_res20 = (undefined8 ******)CONCAT44(local_res20._4_4_,uVar6);
            local_288 = lVar17 + 0x30;
            if (local_2a0 <= uVar6) goto LAB_1427ed336;
            iVar5 = (int)local_res18;
          } while( true );
        }
      }
      FUN_1401a59c0(&local_248,iVar5,0,0);
                    /* WARNING: Subroutine does not return */
      _CxxThrowException(&local_248,(ThrowInfo *)&DAT_143a3b0c0);
    }
  }
  else {
    *(int *)(param_1 + 0x2d98) = param_2;
  }
  return;
}



//===========================================================
// FUN_140f98f40 @ 140f98f40   (16030 bytes)
//===========================================================

void FUN_140f98f40(longlong param_1,longlong *param_2,int param_3,int param_4,longlong *param_5,
                  int param_6)

{
  longlong lVar1;
  IUnknown *pIVar2;
  int iVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  uint uVar6;
  longlong *plVar7;
  ulonglong *puVar8;
  longlong *plVar9;
  undefined8 uVar10;
  longlong *plVar11;
  ulonglong uVar12;
  uint uVar13;
  ulonglong *puVar14;
  longlong lVar15;
  ulonglong uVar16;
  undefined8 *puVar17;
  ulonglong uVar18;
  undefined8 *puVar19;
  undefined8 *puVar20;
  ulonglong uVar21;
  short local_168;
  undefined2 uStack_166;
  undefined4 uStack_164;
  longlong *plStack_160;
  undefined8 local_158;
  short local_148;
  undefined2 uStack_146;
  undefined4 uStack_144;
  longlong *plStack_140;
  undefined8 local_138;
  short local_128;
  undefined6 uStack_126;
  longlong lStack_120;
  undefined8 local_118;
  short local_110;
  undefined6 uStack_10e;
  longlong lStack_108;
  undefined8 local_100;
  IUnknown *local_f8 [2];
  undefined8 local_e8;
  longlong *plStack_e0;
  undefined8 local_d8;
  short local_c8;
  undefined2 uStack_c6;
  undefined4 uStack_c4;
  longlong lStack_c0;
  undefined8 local_b8;
  undefined1 local_b0 [4];
  undefined1 local_ac [4];
  undefined8 local_a8;
  longlong *plStack_a0;
  undefined8 local_98;
  undefined8 local_88;
  longlong *plStack_80;
  undefined8 local_78;
  undefined8 local_68;
  longlong lStack_60;
  undefined8 local_58;
  undefined8 local_48;
  longlong lStack_40;
  undefined8 local_38;
  
  uVar6 = 0;
  if ((param_3 != 0) || (param_4 != 0)) {
    local_f8[0] = (IUnknown *)0x0;
    if (DAT_143ad48a0 == (code *)0x0) {
      iVar3 = -0x7ffbfe10;
LAB_140f9caaa:
      FUN_1401a59c0(&local_e8,iVar3,0,0);
                    /* WARNING: Subroutine does not return */
      _CxxThrowException(&local_e8,(ThrowInfo *)&DAT_143a3b0c0);
    }
    iVar3 = (*DAT_143ad48a0)(PTR_u_Shape2D_Vector2D_143a479e8,&DAT_143273488,local_f8,0);
    pIVar2 = local_f8[0];
    if (iVar3 < 0) goto LAB_140f9caaa;
    plVar11 = (longlong *)*param_2;
    if (plVar11 != (longlong *)0x0) {
      if (local_f8[0] == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      local_168 = 0xd;
      plStack_160 = plVar11;
      (**(code **)(*plVar11 + 8))();
      local_148 = local_168;
      uStack_146 = uStack_166;
      uStack_144 = uStack_164;
      plStack_140 = plStack_160;
      local_138 = local_158;
      iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_148);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
      }
      if (local_168 == 8) {
        local_168 = 0;
        if (plStack_160 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)plStack_160 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_168);
      }
    }
    pIVar2 = local_f8[0];
    if (local_f8[0] == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_168);
    if (DAT_143a8b8d8 == 8) {
      if (local_168 == 8) {
        local_168 = 0;
        if (plStack_160 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)plStack_160 + -4);
        }
      }
      else {
        iVar3 = (*DAT_143262a18)(&local_168);
        if (iVar3 < 0) goto LAB_140f9cacc;
      }
      local_168 = 8;
      uVar13 = uVar6;
      if (DAT_143a8b8e0 != 0) {
        uVar13 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      plStack_160 = (longlong *)FUN_1401a5fa0(DAT_143a8b8e0,uVar13);
    }
    else {
      if ((local_168 == 8) && (local_168 = 0, plStack_160 != (longlong *)0x0)) {
        (*DAT_143ad5990)((longlong)plStack_160 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_168,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140f9cacc:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
    }
    (*DAT_143262a20)(&local_148);
    if (DAT_143a8b8d8 == 8) {
      if (local_148 == 8) {
        local_148 = 0;
        if (plStack_140 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)plStack_140 + -4);
        }
      }
      else {
        iVar3 = (*DAT_143262a18)(&local_148);
        if (iVar3 < 0) goto LAB_140f9cad4;
      }
      local_148 = 8;
      uVar13 = uVar6;
      if (DAT_143a8b8e0 != 0) {
        uVar13 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      plStack_140 = (longlong *)FUN_1401a5fa0(DAT_143a8b8e0,uVar13);
    }
    else {
      if ((local_148 == 8) && (local_148 = 0, plStack_140 != (longlong *)0x0)) {
        (*DAT_143ad5990)((longlong)plStack_140 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_148,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140f9cad4:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
    }
    (*DAT_143262a20)(&local_110);
    if (DAT_143a8b8d8 == 8) {
      if (local_110 == 8) {
        local_110 = 0;
        if (lStack_108 != 0) {
          (*DAT_143ad5990)(lStack_108 + -4);
        }
      }
      else {
        iVar3 = (*DAT_143262a18)(&local_110);
        if (iVar3 < 0) goto LAB_140f9cadc;
      }
      local_110 = 8;
      uVar13 = uVar6;
      if (DAT_143a8b8e0 != 0) {
        uVar13 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_108 = FUN_1401a5fa0(DAT_143a8b8e0,uVar13);
    }
    else {
      if ((local_110 == 8) && (local_110 = 0, lStack_108 != 0)) {
        (*DAT_143ad5990)(lStack_108 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_110,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140f9cadc:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
    }
    (*DAT_143262a20)(&local_128);
    if (DAT_143a8b8d8 == 8) {
      if (local_128 == 8) {
        local_128 = 0;
        if (lStack_120 != 0) {
          (*DAT_143ad5990)(lStack_120 + -4);
        }
      }
      else {
        iVar3 = (*DAT_143262a18)(&local_128);
        if (iVar3 < 0) goto LAB_140f9cae4;
      }
      local_128 = 8;
      uVar13 = uVar6;
      if (DAT_143a8b8e0 != 0) {
        uVar13 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_120 = FUN_1401a5fa0(DAT_143a8b8e0,uVar13);
    }
    else {
      if ((local_128 == 8) && (local_128 = 0, lStack_120 != 0)) {
        (*DAT_143ad5990)(lStack_120 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_128,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140f9cae4:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
    }
    (*DAT_143262a20)(&local_c8);
    if (DAT_143a8b8d8 == 8) {
      if (local_c8 == 8) {
        local_c8 = 0;
        if (lStack_c0 != 0) {
          (*DAT_143ad5990)(lStack_c0 + -4);
        }
      }
      else {
        iVar3 = (*DAT_143262a18)(&local_c8);
        if (iVar3 < 0) goto LAB_140f9caec;
      }
      local_c8 = 8;
      uVar13 = uVar6;
      if (DAT_143a8b8e0 != 0) {
        uVar13 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
      }
      lStack_c0 = FUN_1401a5fa0(DAT_143a8b8e0,uVar13);
    }
    else {
      if ((local_c8 == 8) && (local_c8 = 0, lStack_c0 != 0)) {
        (*DAT_143ad5990)(lStack_c0 + -4);
      }
      iVar3 = (*DAT_143262a28)(&local_c8,&DAT_143a8b8d8);
      if (iVar3 < 0) {
LAB_140f9caec:
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
    }
    local_a8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
    plStack_a0 = plStack_160;
    local_98 = local_158;
    local_88 = CONCAT44(uStack_144,CONCAT22(uStack_146,local_148));
    plStack_80 = plStack_140;
    local_78 = local_138;
    local_68 = CONCAT62(uStack_10e,local_110);
    lStack_60 = lStack_108;
    local_58 = local_100;
    local_48 = CONCAT62(uStack_126,local_128);
    lStack_40 = lStack_120;
    local_38 = local_118;
    local_e8 = CONCAT44(uStack_c4,CONCAT22(uStack_c6,local_c8));
    plStack_e0 = (longlong *)lStack_c0;
    local_d8 = local_b8;
    iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x140))
                      (pIVar2,param_3,param_4,&local_e8,&local_48,&local_68,&local_88,&local_a8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
    }
    if (local_c8 == 8) {
      local_c8 = 0;
      if (lStack_c0 != 0) {
        (*DAT_143ad5990)(lStack_c0 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_c8);
    }
    if (local_128 == 8) {
      local_128 = 0;
      if (lStack_120 != 0) {
        (*DAT_143ad5990)(lStack_120 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_128);
    }
    if (local_110 == 8) {
      local_110 = 0;
      if (lStack_108 != 0) {
        (*DAT_143ad5990)(lStack_108 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_110);
    }
    if (local_148 == 8) {
      local_148 = 0;
      if (plStack_140 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_140 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_148);
    }
    if (local_168 == 8) {
      local_168 = 0;
      if (plStack_160 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_160 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_168);
    }
    pIVar2 = local_f8[0];
    if ((IUnknown *)*param_2 != local_f8[0]) {
      if (local_f8[0] != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_f8[0] + 8))();
      }
      plVar11 = (longlong *)*param_2;
      *param_2 = (longlong)pIVar2;
      if (plVar11 != (longlong *)0x0) {
        (**(code **)(*plVar11 + 0x10))(plVar11);
      }
    }
    if (local_f8[0] != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_f8[0] + 0x10))();
    }
  }
  plVar11 = *(longlong **)(param_1 + 0xce8);
  plVar9 = (longlong *)*param_2;
  if (plVar11 != plVar9) {
    if (plVar9 != (longlong *)0x0) {
      (**(code **)(*plVar9 + 8))(plVar9);
      plVar11 = *(longlong **)(param_1 + 0xce8);
    }
    *(longlong **)(param_1 + 0xce8) = plVar9;
    if (plVar11 != (longlong *)0x0) {
      (**(code **)(*plVar11 + 0x10))();
    }
  }
  plVar11 = (longlong *)(param_1 + 0xcf8);
  FUN_14032dc90(PTR_u_Shape2D_Vector2D_143a479e8,plVar11,0);
  pIVar2 = (IUnknown *)*plVar11;
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  plStack_160 = (longlong *)*param_2;
  local_168 = 0xd;
  if (plStack_160 != (longlong *)0x0) {
    (**(code **)(*plStack_160 + 8))();
  }
  local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
  plStack_e0 = plStack_160;
  local_d8 = local_158;
  iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  pIVar2 = (IUnknown *)*plVar11;
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_c8);
  if (DAT_143a8b8d8 == 8) {
    if (local_c8 == 8) {
      local_c8 = 0;
      if (lStack_c0 != 0) {
        (*DAT_143ad5990)(lStack_c0 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_c8);
      if (iVar3 < 0) goto LAB_140f9caf4;
    }
    local_c8 = 8;
    uVar13 = uVar6;
    if (DAT_143a8b8e0 != 0) {
      uVar13 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_c0 = FUN_1401a5fa0(DAT_143a8b8e0,uVar13);
  }
  else {
    if ((local_c8 == 8) && (local_c8 = 0, lStack_c0 != 0)) {
      (*DAT_143ad5990)(lStack_c0 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_c8,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_140f9caf4:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  (*DAT_143262a20)(&local_128);
  if (DAT_143a8b8d8 == 8) {
    if (local_128 == 8) {
      local_128 = 0;
      if (lStack_120 != 0) {
        (*DAT_143ad5990)(lStack_120 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_128);
      if (iVar3 < 0) goto LAB_140f9cafc;
    }
    local_128 = 8;
    uVar13 = uVar6;
    if (DAT_143a8b8e0 != 0) {
      uVar13 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_120 = FUN_1401a5fa0(DAT_143a8b8e0,uVar13);
  }
  else {
    if ((local_128 == 8) && (local_128 = 0, lStack_120 != 0)) {
      (*DAT_143ad5990)(lStack_120 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_128,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_140f9cafc:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  (*DAT_143262a20)(&local_110);
  if (DAT_143a8b8d8 == 8) {
    if (local_110 == 8) {
      local_110 = 0;
      if (lStack_108 != 0) {
        (*DAT_143ad5990)(lStack_108 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_110);
      if (iVar3 < 0) goto LAB_140f9cb04;
    }
    local_110 = 8;
    uVar13 = uVar6;
    if (DAT_143a8b8e0 != 0) {
      uVar13 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_108 = FUN_1401a5fa0(DAT_143a8b8e0,uVar13);
  }
  else {
    if ((local_110 == 8) && (local_110 = 0, lStack_108 != 0)) {
      (*DAT_143ad5990)(lStack_108 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_110,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_140f9cb04:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  (*DAT_143262a20)(&local_148);
  if (DAT_143a8b8d8 == 8) {
    if (local_148 == 8) {
      local_148 = 0;
      if (plStack_140 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_140 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_148);
      if (iVar3 < 0) goto LAB_140f9cb0c;
    }
    local_148 = 8;
    uVar13 = uVar6;
    if (DAT_143a8b8e0 != 0) {
      uVar13 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    plStack_140 = (longlong *)FUN_1401a5fa0(DAT_143a8b8e0,uVar13);
  }
  else {
    if ((local_148 == 8) && (local_148 = 0, plStack_140 != (longlong *)0x0)) {
      (*DAT_143ad5990)((longlong)plStack_140 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_148,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_140f9cb0c:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  (*DAT_143262a20)(&local_168);
  if (DAT_143a8b8d8 == 8) {
    if (local_168 == 8) {
      local_168 = 0;
      if (plStack_160 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_160 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_168);
      if (iVar3 < 0) goto LAB_140f9cb14;
    }
    local_168 = 8;
    uVar13 = uVar6;
    if (DAT_143a8b8e0 != 0) {
      uVar13 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    plStack_160 = (longlong *)FUN_1401a5fa0(DAT_143a8b8e0,uVar13);
  }
  else {
    if ((local_168 == 8) && (local_168 = 0, plStack_160 != (longlong *)0x0)) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_168,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_140f9cb14:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  local_e8 = CONCAT44(uStack_c4,CONCAT22(uStack_c6,local_c8));
  plStack_e0 = (longlong *)lStack_c0;
  local_d8 = local_b8;
  local_48 = CONCAT62(uStack_126,local_128);
  lStack_40 = lStack_120;
  local_38 = local_118;
  local_68 = CONCAT62(uStack_10e,local_110);
  lStack_60 = lStack_108;
  local_58 = local_100;
  local_88 = CONCAT44(uStack_144,CONCAT22(uStack_146,local_148));
  plStack_80 = plStack_140;
  local_78 = local_138;
  local_a8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
  plStack_a0 = plStack_160;
  local_98 = local_158;
  iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x140))
                    (pIVar2,0,0,&local_a8,&local_88,&local_68,&local_48,&local_e8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  if (local_148 == 8) {
    local_148 = 0;
    if (plStack_140 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_140 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_148);
  }
  if (local_110 == 8) {
    local_110 = 0;
    if (lStack_108 != 0) {
      (*DAT_143ad5990)(lStack_108 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_110);
  }
  if (local_128 == 8) {
    local_128 = 0;
    if (lStack_120 != 0) {
      (*DAT_143ad5990)(lStack_120 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_128);
  }
  if (local_c8 == 8) {
    local_c8 = 0;
    if (lStack_c0 != 0) {
      (*DAT_143ad5990)(lStack_c0 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_c8);
  }
  plVar9 = (longlong *)(param_1 + 0xd08);
  FUN_14032dc90(PTR_u_Shape2D_Vector2D_143a479e8,plVar9,0);
  pIVar2 = (IUnknown *)*plVar9;
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  plStack_160 = (longlong *)*plVar11;
  local_168 = 0xd;
  if (plStack_160 != (longlong *)0x0) {
    (**(code **)(*plStack_160 + 8))();
  }
  local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
  plStack_e0 = plStack_160;
  local_d8 = local_158;
  iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  pIVar2 = (IUnknown *)*plVar9;
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_c8);
  if (DAT_143a8b8d8 == 8) {
    if (local_c8 == 8) {
      local_c8 = 0;
      if (lStack_c0 != 0) {
        (*DAT_143ad5990)(lStack_c0 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_c8);
      if (iVar3 < 0) goto LAB_140f9cb1c;
    }
    local_c8 = 8;
    uVar13 = uVar6;
    if (DAT_143a8b8e0 != 0) {
      uVar13 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_c0 = FUN_1401a5fa0(DAT_143a8b8e0,uVar13);
  }
  else {
    if ((local_c8 == 8) && (local_c8 = 0, lStack_c0 != 0)) {
      (*DAT_143ad5990)(lStack_c0 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_c8,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_140f9cb1c:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  (*DAT_143262a20)(&local_128);
  if (DAT_143a8b8d8 == 8) {
    if (local_128 == 8) {
      local_128 = 0;
      if (lStack_120 != 0) {
        (*DAT_143ad5990)(lStack_120 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_128);
      if (iVar3 < 0) goto LAB_140f9cb24;
    }
    local_128 = 8;
    uVar13 = uVar6;
    if (DAT_143a8b8e0 != 0) {
      uVar13 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_120 = FUN_1401a5fa0(DAT_143a8b8e0,uVar13);
  }
  else {
    if ((local_128 == 8) && (local_128 = 0, lStack_120 != 0)) {
      (*DAT_143ad5990)(lStack_120 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_128,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_140f9cb24:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  (*DAT_143262a20)(&local_110);
  if (DAT_143a8b8d8 == 8) {
    if (local_110 == 8) {
      local_110 = 0;
      if (lStack_108 != 0) {
        (*DAT_143ad5990)(lStack_108 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_110);
      if (iVar3 < 0) goto LAB_140f9cb2c;
    }
    local_110 = 8;
    uVar13 = uVar6;
    if (DAT_143a8b8e0 != 0) {
      uVar13 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_108 = FUN_1401a5fa0(DAT_143a8b8e0,uVar13);
  }
  else {
    if ((local_110 == 8) && (local_110 = 0, lStack_108 != 0)) {
      (*DAT_143ad5990)(lStack_108 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_110,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_140f9cb2c:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  (*DAT_143262a20)(&local_148);
  if (DAT_143a8b8d8 == 8) {
    if (local_148 == 8) {
      local_148 = 0;
      if (plStack_140 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_140 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_148);
      if (iVar3 < 0) goto LAB_140f9cb34;
    }
    local_148 = 8;
    uVar13 = uVar6;
    if (DAT_143a8b8e0 != 0) {
      uVar13 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    plStack_140 = (longlong *)FUN_1401a5fa0(DAT_143a8b8e0,uVar13);
  }
  else {
    if ((local_148 == 8) && (local_148 = 0, plStack_140 != (longlong *)0x0)) {
      (*DAT_143ad5990)((longlong)plStack_140 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_148,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_140f9cb34:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  (*DAT_143262a20)(&local_168);
  if (DAT_143a8b8d8 == 8) {
    if (local_168 == 8) {
      local_168 = 0;
      if (plStack_160 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_160 + -4);
      }
    }
    else {
      iVar3 = (*DAT_143262a18)(&local_168);
      if (iVar3 < 0) goto LAB_140f9cb3c;
    }
    local_168 = 8;
    if (DAT_143a8b8e0 != 0) {
      uVar6 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    plStack_160 = (longlong *)FUN_1401a5fa0(DAT_143a8b8e0,uVar6);
  }
  else {
    if ((local_168 == 8) && (local_168 = 0, plStack_160 != (longlong *)0x0)) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
    iVar3 = (*DAT_143262a28)(&local_168,&DAT_143a8b8d8);
    if (iVar3 < 0) {
LAB_140f9cb3c:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
  }
  local_e8 = CONCAT44(uStack_c4,CONCAT22(uStack_c6,local_c8));
  plStack_e0 = (longlong *)lStack_c0;
  local_d8 = local_b8;
  local_48 = CONCAT62(uStack_126,local_128);
  lStack_40 = lStack_120;
  local_38 = local_118;
  local_68 = CONCAT62(uStack_10e,local_110);
  lStack_60 = lStack_108;
  local_58 = local_100;
  local_88 = CONCAT44(uStack_144,CONCAT22(uStack_146,local_148));
  plStack_80 = plStack_140;
  local_78 = local_138;
  local_a8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
  plStack_a0 = plStack_160;
  local_98 = local_158;
  puVar20 = &local_48;
  puVar19 = &local_68;
  puVar17 = &local_88;
  iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x140))
                    (pIVar2,0,0,&local_a8,puVar17,puVar19,puVar20,&local_e8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  if (local_148 == 8) {
    local_148 = 0;
    if (plStack_140 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_140 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_148);
  }
  if (local_110 == 8) {
    local_110 = 0;
    if (lStack_108 != 0) {
      (*DAT_143ad5990)(lStack_108 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_110);
  }
  if (local_128 == 8) {
    local_128 = 0;
    if (lStack_120 != 0) {
      (*DAT_143ad5990)(lStack_120 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_128);
  }
  if (local_c8 == 8) {
    local_c8 = 0;
    if (lStack_c0 != 0) {
      (*DAT_143ad5990)(lStack_c0 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_c8);
  }
  FUN_14032dc90(PTR_u_Shape2D_Vector2D_143a479e8,param_1 + 0xd00,0);
  pIVar2 = *(IUnknown **)(param_1 + 0xd00);
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  plStack_160 = (longlong *)*plVar9;
  local_168 = 0xd;
  if (plStack_160 != (longlong *)0x0) {
    (**(code **)(*plStack_160 + 8))();
  }
  local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
  plStack_e0 = plStack_160;
  local_d8 = local_158;
  iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  FUN_14032dc90(PTR_u_Shape2D_Vector2D_143a479e8,param_1 + 0xd10,0);
  pIVar2 = *(IUnknown **)(param_1 + 0xd10);
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  plStack_160 = (longlong *)*plVar9;
  local_168 = 0xd;
  if (plStack_160 != (longlong *)0x0) {
    (**(code **)(*plStack_160 + 8))();
  }
  local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
  plStack_e0 = plStack_160;
  local_d8 = local_158;
  iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  FUN_14032dc90(PTR_u_Shape2D_Vector2D_143a479e8,param_1 + 0xd18,0);
  pIVar2 = *(IUnknown **)(param_1 + 0xd18);
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  plStack_160 = (longlong *)*plVar9;
  local_168 = 0xd;
  if (plStack_160 != (longlong *)0x0) {
    (**(code **)(*plStack_160 + 8))();
  }
  local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
  plStack_e0 = plStack_160;
  local_d8 = local_158;
  iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  FUN_14032dc90(PTR_u_Shape2D_Vector2D_143a479e8,param_1 + 0xd20,0);
  pIVar2 = *(IUnknown **)(param_1 + 0xd20);
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  plStack_160 = (longlong *)*plVar9;
  local_168 = 0xd;
  if (plStack_160 != (longlong *)0x0) {
    (**(code **)(*plStack_160 + 8))();
  }
  local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
  plStack_e0 = plStack_160;
  local_d8 = local_158;
  iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  FUN_14032dc90(PTR_u_Shape2D_Vector2D_143a479e8,param_1 + 0xd28,0);
  pIVar2 = *(IUnknown **)(param_1 + 0xd28);
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  plStack_160 = (longlong *)*plVar11;
  local_168 = 0xd;
  if (plStack_160 != (longlong *)0x0) {
    (**(code **)(*plStack_160 + 8))();
  }
  local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
  plStack_e0 = plStack_160;
  local_d8 = local_158;
  iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  FUN_14032dc90(PTR_u_Shape2D_Vector2D_143a479e8,param_1 + 0xd30,0);
  pIVar2 = *(IUnknown **)(param_1 + 0xd30);
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  plStack_160 = (longlong *)*plVar11;
  local_168 = 0xd;
  if (plStack_160 != (longlong *)0x0) {
    (**(code **)(*plStack_160 + 8))();
  }
  local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
  plStack_e0 = plStack_160;
  local_d8 = local_158;
  iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  FUN_14032dc90(PTR_u_Shape2D_Vector2D_143a479e8,param_1 + 0xd38,0);
  pIVar2 = *(IUnknown **)(param_1 + 0xd38);
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  plStack_160 = (longlong *)*plVar11;
  local_168 = 0xd;
  if (plStack_160 != (longlong *)0x0) {
    (**(code **)(*plStack_160 + 8))();
  }
  local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
  plStack_e0 = plStack_160;
  local_d8 = local_158;
  iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  FUN_14032dc90(PTR_u_Shape2D_Vector2D_143a479e8,param_1 + 0xd40,0);
  pIVar2 = *(IUnknown **)(param_1 + 0xd40);
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  plStack_160 = (longlong *)*plVar11;
  local_168 = 0xd;
  if (plStack_160 != (longlong *)0x0) {
    (**(code **)(*plStack_160 + 8))();
  }
  local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
  plStack_e0 = plStack_160;
  local_d8 = local_158;
  iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  FUN_14032dc90(PTR_u_Shape2D_Vector2D_143a479e8,param_1 + 0xdf0,0);
  pIVar2 = *(IUnknown **)(param_1 + 0xdf0);
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  plStack_160 = (longlong *)*plVar11;
  local_168 = 0xd;
  if (plStack_160 != (longlong *)0x0) {
    (**(code **)(*plStack_160 + 8))();
  }
  local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
  plStack_e0 = plStack_160;
  local_d8 = local_158;
  iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  lVar15 = DAT_143add050;
  uVar4 = (undefined4)((ulonglong)puVar20 >> 0x20);
  if (*param_5 != 0) {
    if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_128);
    iVar3 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
    if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
    (*DAT_143262a20)(&local_110);
    iVar3 = FUN_14023c4c0(&local_110,&DAT_143a8b8d8);
    if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
    (*DAT_143262a20)(&local_148);
    iVar3 = FUN_14023c4c0(&local_148,&DAT_143a8b8d8);
    if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
    local_168 = 3;
    plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
    uVar10 = CONCAT44(uVar4,param_6 + -1);
    uVar18 = (ulonglong)puVar19 & 0xffffffff00000000;
    uVar16 = (ulonglong)puVar17 & 0xffffffff00000000;
    plVar7 = (longlong *)
             FUN_140d8d300(lVar15,local_f8,0,0,uVar16,uVar18,uVar10,&local_168,&local_148,&local_110
                           ,&local_128);
    uVar4 = (undefined4)((ulonglong)uVar10 >> 0x20);
    plVar11 = *(longlong **)(param_1 + 0xd98);
    if (plVar11 != (longlong *)*plVar7) {
      *(longlong **)(param_1 + 0xd98) = (longlong *)*plVar7;
      *plVar7 = 0;
      if (plVar11 != (longlong *)0x0) {
        (**(code **)(*plVar11 + 0x10))();
      }
    }
    if (local_f8[0] != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_f8[0] + 0x10))();
    }
    if (local_168 == 8) {
      local_168 = 0;
      if (plStack_160 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_160 - 4);
      }
    }
    else {
      (*DAT_143262a18)(&local_168);
    }
    if (local_148 == 8) {
      local_148 = 0;
      if (plStack_140 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_140 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_148);
    }
    if (local_110 == 8) {
      local_110 = 0;
      if (lStack_108 != 0) {
        (*DAT_143ad5990)(lStack_108 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_110);
    }
    if (local_128 == 8) {
      local_128 = 0;
      if (lStack_120 != 0) {
        (*DAT_143ad5990)(lStack_120 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_128);
    }
    pIVar2 = *(IUnknown **)(param_1 + 0xd98);
    if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    plStack_160 = (longlong *)*param_5;
    local_168 = 0xd;
    if (plStack_160 != (longlong *)0x0) {
      (**(code **)(*plStack_160 + 8))();
    }
    local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
    plStack_e0 = plStack_160;
    local_d8 = local_158;
    iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x238))(pIVar2,&local_e8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
    }
    if (local_168 == 8) {
      local_168 = 0;
      if (plStack_160 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_160 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_168);
    }
    lVar15 = DAT_143add050;
    if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_128);
    iVar3 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
    if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
    (*DAT_143262a20)(&local_110);
    iVar3 = FUN_14023c4c0(&local_110,&DAT_143a8b8d8);
    if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
    (*DAT_143262a20)(&local_148);
    iVar3 = FUN_14023c4c0(&local_148,&DAT_143a8b8d8);
    if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
    local_168 = 3;
    plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
    uVar10 = CONCAT44(uVar4,param_6);
    uVar18 = uVar18 & 0xffffffff00000000;
    uVar16 = uVar16 & 0xffffffff00000000;
    plVar7 = (longlong *)
             FUN_140d8d300(lVar15,local_f8,0,0,uVar16,uVar18,uVar10,&local_168,&local_148,&local_110
                           ,&local_128);
    uVar4 = (undefined4)((ulonglong)uVar10 >> 0x20);
    plVar11 = *(longlong **)(param_1 + 0xd60);
    if (plVar11 != (longlong *)*plVar7) {
      *(longlong **)(param_1 + 0xd60) = (longlong *)*plVar7;
      *plVar7 = 0;
      if (plVar11 != (longlong *)0x0) {
        (**(code **)(*plVar11 + 0x10))();
      }
    }
    if (local_f8[0] != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_f8[0] + 0x10))();
    }
    if (local_168 == 8) {
      local_168 = 0;
      if (plStack_160 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_160 - 4);
      }
    }
    else {
      (*DAT_143262a18)(&local_168);
    }
    if (local_148 == 8) {
      local_148 = 0;
      if (plStack_140 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_140 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_148);
    }
    if (local_110 == 8) {
      local_110 = 0;
      if (lStack_108 != 0) {
        (*DAT_143ad5990)(lStack_108 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_110);
    }
    if (local_128 == 8) {
      local_128 = 0;
      if (lStack_120 != 0) {
        (*DAT_143ad5990)(lStack_120 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_128);
    }
    pIVar2 = *(IUnknown **)(param_1 + 0xd60);
    if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    plStack_160 = (longlong *)*param_5;
    local_168 = 0xd;
    if (plStack_160 != (longlong *)0x0) {
      (**(code **)(*plStack_160 + 8))();
    }
    local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
    plStack_e0 = plStack_160;
    local_d8 = local_158;
    iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x238))(pIVar2,&local_e8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
    }
    if (local_168 == 8) {
      local_168 = 0;
      if (plStack_160 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_160 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_168);
    }
    lVar15 = DAT_143add050;
    if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_128);
    iVar3 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
    if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
    (*DAT_143262a20)(&local_110);
    iVar3 = FUN_14023c4c0(&local_110,&DAT_143a8b8d8);
    if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
    (*DAT_143262a20)(&local_148);
    iVar3 = FUN_14023c4c0(&local_148,&DAT_143a8b8d8);
    if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
    local_168 = 3;
    plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
    puVar20 = (undefined8 *)CONCAT44(uVar4,param_6 + 1);
    puVar19 = (undefined8 *)(uVar18 & 0xffffffff00000000);
    puVar17 = (undefined8 *)(uVar16 & 0xffffffff00000000);
    plVar7 = (longlong *)
             FUN_140d8d300(lVar15,local_f8,0,0,puVar17,puVar19,puVar20,&local_168,&local_148,
                           &local_110,&local_128);
    plVar11 = *(longlong **)(param_1 + 0xda0);
    if (plVar11 != (longlong *)*plVar7) {
      *(longlong **)(param_1 + 0xda0) = (longlong *)*plVar7;
      *plVar7 = 0;
      if (plVar11 != (longlong *)0x0) {
        (**(code **)(*plVar11 + 0x10))();
      }
    }
    if (local_f8[0] != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_f8[0] + 0x10))();
    }
    if (local_168 == 8) {
      local_168 = 0;
      if (plStack_160 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_160 - 4);
      }
    }
    else {
      (*DAT_143262a18)(&local_168);
    }
    if (local_148 == 8) {
      local_148 = 0;
      if (plStack_140 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_140 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_148);
    }
    if (local_110 == 8) {
      local_110 = 0;
      if (lStack_108 != 0) {
        (*DAT_143ad5990)(lStack_108 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_110);
    }
    if (local_128 == 8) {
      local_128 = 0;
      if (lStack_120 != 0) {
        (*DAT_143ad5990)(lStack_120 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_128);
    }
    pIVar2 = *(IUnknown **)(param_1 + 0xda0);
    if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    plStack_160 = (longlong *)*param_5;
    local_168 = 0xd;
    if (plStack_160 != (longlong *)0x0) {
      (**(code **)(*plStack_160 + 8))();
    }
    local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
    plStack_e0 = plStack_160;
    local_d8 = local_158;
    iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x238))(pIVar2,&local_e8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
    }
    if (local_168 == 8) {
      local_168 = 0;
      if (plStack_160 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_160 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_168);
    }
  }
  lVar15 = DAT_143add050;
  if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_128);
  iVar3 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
  if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar3);
  }
  (*DAT_143262a20)(&local_110);
  iVar3 = FUN_14023c4c0(&local_110,&DAT_143a8b8d8);
  if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar3);
  }
  uVar4 = 2;
  if (*(int *)(param_1 + 0xe58) == 100) {
    uVar4 = 0;
  }
  local_148 = 3;
  plStack_140 = (longlong *)CONCAT44(plStack_140._4_4_,uVar4);
  local_168 = 3;
  plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
  uVar21 = (ulonglong)puVar20 & 0xffffffff00000000;
  uVar18 = (ulonglong)puVar19 & 0xffffffff00000000;
  uVar16 = (ulonglong)puVar17 & 0xffffffff00000000;
  plVar7 = (longlong *)
           FUN_140d8d300(lVar15,local_f8,0,0,uVar16,uVar18,uVar21,&local_168,&local_148,&local_110,
                         &local_128);
  uVar4 = (undefined4)(uVar21 >> 0x20);
  plVar11 = *(longlong **)(param_1 + 0xd58);
  if (plVar11 != (longlong *)*plVar7) {
    *(longlong **)(param_1 + 0xd58) = (longlong *)*plVar7;
    *plVar7 = 0;
    if (plVar11 != (longlong *)0x0) {
      (**(code **)(*plVar11 + 0x10))();
    }
  }
  if (local_f8[0] != (IUnknown *)0x0) {
    (**(code **)(*(longlong *)local_f8[0] + 0x10))();
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 - 4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  if (local_148 == 8) {
    local_148 = 0;
    if (plStack_140 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_140 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_148);
  }
  if (local_110 == 8) {
    local_110 = 0;
    if (lStack_108 != 0) {
      (*DAT_143ad5990)(lStack_108 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_110);
  }
  if (local_128 == 8) {
    local_128 = 0;
    if (lStack_120 != 0) {
      (*DAT_143ad5990)(lStack_120 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_128);
  }
  pIVar2 = *(IUnknown **)(param_1 + 0xd58);
  if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  plStack_160 = (longlong *)*plVar9;
  local_168 = 0xd;
  if (plStack_160 != (longlong *)0x0) {
    (**(code **)(*plStack_160 + 8))();
  }
  local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
  plStack_e0 = plStack_160;
  local_d8 = local_158;
  iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
  if (iVar3 < 0) {
    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
  }
  if (local_168 == 8) {
    local_168 = 0;
    if (plStack_160 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)plStack_160 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_168);
  }
  pIVar2 = *(IUnknown **)(param_1 + 0xd58);
  if (pIVar2 != (IUnknown *)0x0) {
    iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x200))(pIVar2,0xffffffff);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
    }
    plVar11 = *(longlong **)(param_1 + 0xd60);
    if (plVar11 != (longlong *)0x0) {
      pIVar2 = *(IUnknown **)(param_1 + 0xd58);
      if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      local_168 = 0xd;
      plStack_160 = plVar11;
      (**(code **)(*plVar11 + 8))();
      local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
      plStack_e0 = plStack_160;
      local_d8 = local_158;
      iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x238))(pIVar2,&local_e8);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
      }
      if (local_168 == 8) {
        local_168 = 0;
        if (plStack_160 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)plStack_160 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_168);
      }
    }
    lVar15 = DAT_143add050;
    if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    (*DAT_143262a20)(&local_128);
    iVar3 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
    if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
    (*DAT_143262a20)(&local_110);
    iVar3 = FUN_14023c4c0(&local_110,&DAT_143a8b8d8);
    if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar3);
    }
    uVar5 = 2;
    if (*(int *)(param_1 + 0xe58) == 100) {
      uVar5 = 0;
    }
    local_148 = 3;
    plStack_140 = (longlong *)CONCAT44(plStack_140._4_4_,uVar5);
    local_168 = 3;
    plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
    uVar10 = CONCAT44(uVar4,2);
    uVar18 = uVar18 & 0xffffffff00000000;
    uVar16 = uVar16 & 0xffffffff00000000;
    plVar7 = (longlong *)
             FUN_140d8d300(lVar15,local_f8,0,0,uVar16,uVar18,uVar10,&local_168,&local_148,&local_110
                           ,&local_128);
    uVar4 = (undefined4)((ulonglong)uVar10 >> 0x20);
    plVar11 = *(longlong **)(param_1 + 0xda8);
    if (plVar11 != (longlong *)*plVar7) {
      *(longlong **)(param_1 + 0xda8) = (longlong *)*plVar7;
      *plVar7 = 0;
      if (plVar11 != (longlong *)0x0) {
        (**(code **)(*plVar11 + 0x10))();
      }
    }
    if (local_f8[0] != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_f8[0] + 0x10))();
    }
    if (local_168 == 8) {
      local_168 = 0;
      if (plStack_160 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_160 - 4);
      }
    }
    else {
      (*DAT_143262a18)(&local_168);
    }
    if (local_148 == 8) {
      local_148 = 0;
      if (plStack_140 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_140 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_148);
    }
    if (local_110 == 8) {
      local_110 = 0;
      if (lStack_108 != 0) {
        (*DAT_143ad5990)(lStack_108 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_110);
    }
    if (local_128 == 8) {
      local_128 = 0;
      if (lStack_120 != 0) {
        (*DAT_143ad5990)(lStack_120 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_128);
    }
    pIVar2 = *(IUnknown **)(param_1 + 0xda8);
    if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    plStack_160 = (longlong *)*plVar9;
    local_168 = 0xd;
    if (plStack_160 != (longlong *)0x0) {
      (**(code **)(*plStack_160 + 8))();
    }
    local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
    plStack_e0 = plStack_160;
    local_d8 = local_158;
    iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
    }
    if (local_168 == 8) {
      local_168 = 0;
      if (plStack_160 != (longlong *)0x0) {
        (*DAT_143ad5990)((longlong)plStack_160 + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_168);
    }
    pIVar2 = *(IUnknown **)(param_1 + 0xda8);
    if (pIVar2 != (IUnknown *)0x0) {
      iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x200))(pIVar2,0xffffffff);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
      }
      pIVar2 = *(IUnknown **)(param_1 + 0xda8);
      if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      plStack_160 = *(longlong **)(param_1 + 0xd58);
      local_168 = 0xd;
      if (plStack_160 != (longlong *)0x0) {
        (**(code **)(*plStack_160 + 8))();
      }
      local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
      plStack_e0 = plStack_160;
      local_d8 = local_158;
      iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x238))(pIVar2,&local_e8);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
      }
      if (local_168 == 8) {
        local_168 = 0;
        if (plStack_160 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)plStack_160 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_168);
      }
      lVar15 = DAT_143add050;
      if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_128);
      iVar3 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
      if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
      (*DAT_143262a20)(&local_110);
      iVar3 = FUN_14023c4c0(&local_110,&DAT_143a8b8d8);
      if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(iVar3);
      }
      uVar5 = 2;
      if (*(int *)(param_1 + 0xe58) == 100) {
        uVar5 = 0;
      }
      local_148 = 3;
      plStack_140 = (longlong *)CONCAT44(plStack_140._4_4_,uVar5);
      local_168 = 3;
      plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
      uVar10 = CONCAT44(uVar4,1);
      uVar18 = uVar18 & 0xffffffff00000000;
      uVar16 = uVar16 & 0xffffffff00000000;
      plVar7 = (longlong *)
               FUN_140d8d300(lVar15,local_f8,0,0,uVar16,uVar18,uVar10,&local_168,&local_148,
                             &local_110,&local_128);
      uVar4 = (undefined4)((ulonglong)uVar10 >> 0x20);
      plVar11 = *(longlong **)(param_1 + 0xd48);
      if (plVar11 != (longlong *)*plVar7) {
        *(longlong **)(param_1 + 0xd48) = (longlong *)*plVar7;
        *plVar7 = 0;
        if (plVar11 != (longlong *)0x0) {
          (**(code **)(*plVar11 + 0x10))();
        }
      }
      if (local_f8[0] != (IUnknown *)0x0) {
        (**(code **)(*(longlong *)local_f8[0] + 0x10))();
      }
      if (local_168 == 8) {
        local_168 = 0;
        if (plStack_160 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)plStack_160 - 4);
        }
      }
      else {
        (*DAT_143262a18)(&local_168);
      }
      if (local_148 == 8) {
        local_148 = 0;
        if (plStack_140 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)plStack_140 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_148);
      }
      if (local_110 == 8) {
        local_110 = 0;
        if (lStack_108 != 0) {
          (*DAT_143ad5990)(lStack_108 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_110);
      }
      if (local_128 == 8) {
        local_128 = 0;
        if (lStack_120 != 0) {
          (*DAT_143ad5990)(lStack_120 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_128);
      }
      pIVar2 = *(IUnknown **)(param_1 + 0xd48);
      if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      plStack_160 = *(longlong **)(param_1 + 0xd00);
      local_168 = 0xd;
      if (plStack_160 != (longlong *)0x0) {
        (**(code **)(*plStack_160 + 8))();
      }
      local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
      plStack_e0 = plStack_160;
      local_d8 = local_158;
      iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
      }
      if (local_168 == 8) {
        local_168 = 0;
        if (plStack_160 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)plStack_160 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_168);
      }
      pIVar2 = *(IUnknown **)(param_1 + 0xd48);
      if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      plStack_160 = *(longlong **)(param_1 + 0xd58);
      local_168 = 0xd;
      if (plStack_160 != (longlong *)0x0) {
        (**(code **)(*plStack_160 + 8))();
      }
      local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
      plStack_e0 = plStack_160;
      local_d8 = local_158;
      iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x238))(pIVar2,&local_e8);
      if (iVar3 < 0) {
        _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
      }
      if (local_168 == 8) {
        local_168 = 0;
        if (plStack_160 != (longlong *)0x0) {
          (*DAT_143ad5990)((longlong)plStack_160 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_168);
      }
      pIVar2 = *(IUnknown **)(param_1 + 0xd48);
      if (pIVar2 != (IUnknown *)0x0) {
        iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x200))(pIVar2,0xffffffff);
        if (iVar3 < 0) {
          _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
        }
        lVar15 = DAT_143add050;
        if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        (*DAT_143262a20)(&local_128);
        iVar3 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
        if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar3);
        }
        (*DAT_143262a20)(&local_110);
        iVar3 = FUN_14023c4c0(&local_110,&DAT_143a8b8d8);
        if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar3);
        }
        uVar5 = 2;
        if (*(int *)(param_1 + 0xe58) == 100) {
          uVar5 = 0;
        }
        local_148 = 3;
        plStack_140 = (longlong *)CONCAT44(plStack_140._4_4_,uVar5);
        local_168 = 3;
        plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
        uVar10 = CONCAT44(uVar4,2);
        uVar18 = uVar18 & 0xffffffff00000000;
        uVar16 = uVar16 & 0xffffffff00000000;
        plVar7 = (longlong *)
                 FUN_140d8d300(lVar15,local_f8,0,0,uVar16,uVar18,uVar10,&local_168,&local_148,
                               &local_110,&local_128);
        uVar4 = (undefined4)((ulonglong)uVar10 >> 0x20);
        plVar11 = *(longlong **)(param_1 + 0xd50);
        if (plVar11 != (longlong *)*plVar7) {
          *(longlong **)(param_1 + 0xd50) = (longlong *)*plVar7;
          *plVar7 = 0;
          if (plVar11 != (longlong *)0x0) {
            (**(code **)(*plVar11 + 0x10))();
          }
        }
        if (local_f8[0] != (IUnknown *)0x0) {
          (**(code **)(*(longlong *)local_f8[0] + 0x10))();
        }
        if (local_168 == 8) {
          local_168 = 0;
          if (plStack_160 != (longlong *)0x0) {
            (*DAT_143ad5990)((longlong)plStack_160 - 4);
          }
        }
        else {
          (*DAT_143262a18)(&local_168);
        }
        if (local_148 == 8) {
          local_148 = 0;
          if (plStack_140 != (longlong *)0x0) {
            (*DAT_143ad5990)((longlong)plStack_140 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_148);
        }
        if (local_110 == 8) {
          local_110 = 0;
          if (lStack_108 != 0) {
            (*DAT_143ad5990)(lStack_108 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_110);
        }
        if (local_128 == 8) {
          local_128 = 0;
          if (lStack_120 != 0) {
            (*DAT_143ad5990)(lStack_120 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_128);
        }
        pIVar2 = *(IUnknown **)(param_1 + 0xd50);
        if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        plStack_160 = (longlong *)*plVar9;
        local_168 = 0xd;
        if (plStack_160 != (longlong *)0x0) {
          (**(code **)(*plStack_160 + 8))();
        }
        local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
        plStack_e0 = plStack_160;
        local_d8 = local_158;
        iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
        if (iVar3 < 0) {
          _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
        }
        if (local_168 == 8) {
          local_168 = 0;
          if (plStack_160 != (longlong *)0x0) {
            (*DAT_143ad5990)((longlong)plStack_160 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_168);
        }
        pIVar2 = *(IUnknown **)(param_1 + 0xd50);
        if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x80004003);
        }
        plStack_160 = *(longlong **)(param_1 + 0xd58);
        local_168 = 0xd;
        if (plStack_160 != (longlong *)0x0) {
          (**(code **)(*plStack_160 + 8))();
        }
        local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
        plStack_e0 = plStack_160;
        local_d8 = local_158;
        iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x238))(pIVar2,&local_e8);
        if (iVar3 < 0) {
          _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
        }
        if (local_168 == 8) {
          local_168 = 0;
          if (plStack_160 != (longlong *)0x0) {
            (*DAT_143ad5990)((longlong)plStack_160 + -4);
          }
        }
        else {
          (*DAT_143262a18)(&local_168);
        }
        pIVar2 = *(IUnknown **)(param_1 + 0xd50);
        if (pIVar2 != (IUnknown *)0x0) {
          iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x200))(pIVar2,0xffffffff);
          if (iVar3 < 0) {
            _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
          }
          lVar15 = DAT_143add050;
          if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          (*DAT_143262a20)(&local_128);
          iVar3 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
          if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar3);
          }
          (*DAT_143262a20)(&local_110);
          iVar3 = FUN_14023c4c0(&local_110,&DAT_143a8b8d8);
          if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(iVar3);
          }
          uVar5 = 2;
          if (*(int *)(param_1 + 0xe58) == 100) {
            uVar5 = 0;
          }
          local_148 = 3;
          plStack_140 = (longlong *)CONCAT44(plStack_140._4_4_,uVar5);
          local_168 = 3;
          plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
          uVar10 = CONCAT44(uVar4,0xfffffffe);
          uVar18 = uVar18 & 0xffffffff00000000;
          uVar16 = uVar16 & 0xffffffff00000000;
          plVar7 = (longlong *)
                   FUN_140d8d300(lVar15,local_f8,0,0,uVar16,uVar18,uVar10,&local_168,&local_148,
                                 &local_110,&local_128);
          plVar11 = *(longlong **)(param_1 + 0xd68);
          if (plVar11 != (longlong *)*plVar7) {
            *(longlong **)(param_1 + 0xd68) = (longlong *)*plVar7;
            *plVar7 = 0;
            if (plVar11 != (longlong *)0x0) {
              (**(code **)(*plVar11 + 0x10))();
            }
          }
          if (local_f8[0] != (IUnknown *)0x0) {
            (**(code **)(*(longlong *)local_f8[0] + 0x10))();
          }
          if (local_168 == 8) {
            local_168 = 0;
            if (plStack_160 != (longlong *)0x0) {
              (*DAT_143ad5990)((longlong)plStack_160 - 4);
            }
          }
          else {
            (*DAT_143262a18)(&local_168);
          }
          if (local_148 == 8) {
            local_148 = 0;
            if (plStack_140 != (longlong *)0x0) {
              (*DAT_143ad5990)((longlong)plStack_140 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_148);
          }
          if (local_110 == 8) {
            local_110 = 0;
            if (lStack_108 != 0) {
              (*DAT_143ad5990)(lStack_108 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_110);
          }
          if (local_128 == 8) {
            local_128 = 0;
            if (lStack_120 != 0) {
              (*DAT_143ad5990)(lStack_120 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_128);
          }
          pIVar2 = *(IUnknown **)(param_1 + 0xd68);
          if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          plStack_160 = (longlong *)*plVar9;
          local_168 = 0xd;
          if (plStack_160 != (longlong *)0x0) {
            (**(code **)(*plStack_160 + 8))();
          }
          local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
          plStack_e0 = plStack_160;
          local_d8 = local_158;
          iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
          if (iVar3 < 0) {
            _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
          }
          if (local_168 == 8) {
            local_168 = 0;
            if (plStack_160 != (longlong *)0x0) {
              (*DAT_143ad5990)((longlong)plStack_160 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_168);
          }
          pIVar2 = *(IUnknown **)(param_1 + 0xd68);
          if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          plStack_160 = *(longlong **)(param_1 + 0xd58);
          local_168 = 0xd;
          if (plStack_160 != (longlong *)0x0) {
            (**(code **)(*plStack_160 + 8))();
          }
          local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
          plStack_e0 = plStack_160;
          local_d8 = local_158;
          iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x238))(pIVar2,&local_e8);
          if (iVar3 < 0) {
            _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
          }
          if (local_168 == 8) {
            local_168 = 0;
            if (plStack_160 != (longlong *)0x0) {
              (*DAT_143ad5990)((longlong)plStack_160 + -4);
            }
          }
          else {
            (*DAT_143262a18)(&local_168);
          }
          pIVar2 = *(IUnknown **)(param_1 + 0xd68);
          if (pIVar2 != (IUnknown *)0x0) {
            iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x200))(pIVar2,0xffffff);
            if (iVar3 < 0) {
              _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
            }
            puVar14 = (ulonglong *)(param_1 + 0x958);
            lVar15 = 2;
            do {
              uVar6 = FUN_1402b1780();
              FUN_140d8d480(puVar14);
              if (uVar6 != 0) {
                lVar1 = (ulonglong)uVar6 * 0x18;
                puVar8 = (ulonglong *)FUN_14019b780(&DAT_143ad68a0,lVar1 + 8);
                if (puVar8 == (ulonglong *)0x0) {
                  *puVar14 = 0;
                }
                else {
                  *puVar14 = (ulonglong)(puVar8 + 1);
                  if (puVar8 + 1 != (ulonglong *)0x0) {
                    *puVar8 = (ulonglong)uVar6;
                    uVar12 = *puVar14;
                    uVar21 = uVar12 + lVar1;
                    for (; uVar12 < uVar21; uVar12 = uVar12 + 0x18) {
                      *(undefined4 *)(uVar12 + 4) = 0;
                      *(undefined8 *)(uVar12 + 8) = 0;
                      *(undefined8 *)(uVar12 + 0x10) = 0;
                    }
                  }
                }
              }
              lVar1 = DAT_143add050;
              uVar4 = (undefined4)((ulonglong)uVar10 >> 0x20);
              puVar14 = puVar14 + 0x6f;
              lVar15 = lVar15 + -1;
            } while (lVar15 != 0);
            if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            (*DAT_143262a20)(&local_128);
            iVar3 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
            if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(iVar3);
            }
            (*DAT_143262a20)(&local_110);
            iVar3 = FUN_14023c4c0(&local_110,&DAT_143a8b8d8);
            if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(iVar3);
            }
            uVar5 = 2;
            if (*(int *)(param_1 + 0xe58) == 100) {
              uVar5 = 0;
            }
            local_148 = 3;
            plStack_140 = (longlong *)CONCAT44(plStack_140._4_4_,uVar5);
            local_168 = 3;
            plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
            uVar10 = CONCAT44(uVar4,0xffffffff);
            uVar18 = uVar18 & 0xffffffff00000000;
            uVar16 = uVar16 & 0xffffffff00000000;
            plVar9 = (longlong *)
                     FUN_140d8d300(lVar1,local_f8,0,0,uVar16,uVar18,uVar10,&local_168,&local_148,
                                   &local_110,&local_128);
            uVar4 = (undefined4)((ulonglong)uVar10 >> 0x20);
            plVar11 = *(longlong **)(param_1 + 0xd78);
            if (plVar11 != (longlong *)*plVar9) {
              *(longlong **)(param_1 + 0xd78) = (longlong *)*plVar9;
              *plVar9 = 0;
              if (plVar11 != (longlong *)0x0) {
                (**(code **)(*plVar11 + 0x10))();
              }
            }
            if (local_f8[0] != (IUnknown *)0x0) {
              (**(code **)(*(longlong *)local_f8[0] + 0x10))();
            }
            if (local_168 == 8) {
              local_168 = 0;
              if (plStack_160 != (longlong *)0x0) {
                (*DAT_143ad5990)((longlong)plStack_160 - 4);
              }
            }
            else {
              (*DAT_143262a18)(&local_168);
            }
            if (local_148 == 8) {
              local_148 = 0;
              if (plStack_140 != (longlong *)0x0) {
                (*DAT_143ad5990)((longlong)plStack_140 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_148);
            }
            if (local_110 == 8) {
              local_110 = 0;
              if (lStack_108 != 0) {
                (*DAT_143ad5990)(lStack_108 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_110);
            }
            if (local_128 == 8) {
              local_128 = 0;
              if (lStack_120 != 0) {
                (*DAT_143ad5990)(lStack_120 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_128);
            }
            pIVar2 = *(IUnknown **)(param_1 + 0xd78);
            if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            plStack_160 = *(longlong **)(param_1 + 0xcf8);
            local_168 = 0xd;
            if (plStack_160 != (longlong *)0x0) {
              (**(code **)(*plStack_160 + 8))();
            }
            local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
            plStack_e0 = plStack_160;
            local_d8 = local_158;
            iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
            if (iVar3 < 0) {
              _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
            }
            if (local_168 == 8) {
              local_168 = 0;
              if (plStack_160 != (longlong *)0x0) {
                (*DAT_143ad5990)((longlong)plStack_160 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_168);
            }
            pIVar2 = *(IUnknown **)(param_1 + 0xd78);
            if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              FUN_142ef3ac0(0x80004003);
            }
            plStack_160 = *(longlong **)(param_1 + 0xd58);
            local_168 = 0xd;
            if (plStack_160 != (longlong *)0x0) {
              (**(code **)(*plStack_160 + 8))();
            }
            local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
            plStack_e0 = plStack_160;
            local_d8 = local_158;
            iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x238))(pIVar2,&local_e8);
            if (iVar3 < 0) {
              _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
            }
            if (local_168 == 8) {
              local_168 = 0;
              if (plStack_160 != (longlong *)0x0) {
                (*DAT_143ad5990)((longlong)plStack_160 + -4);
              }
            }
            else {
              (*DAT_143262a18)(&local_168);
            }
            pIVar2 = *(IUnknown **)(param_1 + 0xd78);
            if (pIVar2 != (IUnknown *)0x0) {
              iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x200))(pIVar2,0xffffff);
              if (iVar3 < 0) {
                _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
              }
              lVar15 = DAT_143add050;
              if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x80004003);
              }
              (*DAT_143262a20)(&local_128);
              iVar3 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
              if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar3);
              }
              (*DAT_143262a20)(&local_110);
              iVar3 = FUN_14023c4c0(&local_110,&DAT_143a8b8d8);
              if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(iVar3);
              }
              uVar5 = 2;
              if (*(int *)(param_1 + 0xe58) == 100) {
                uVar5 = 0;
              }
              local_148 = 3;
              plStack_140 = (longlong *)CONCAT44(plStack_140._4_4_,uVar5);
              local_168 = 3;
              plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
              uVar10 = CONCAT44(uVar4,3);
              uVar18 = uVar18 & 0xffffffff00000000;
              uVar16 = uVar16 & 0xffffffff00000000;
              plVar9 = (longlong *)
                       FUN_140d8d300(lVar15,local_f8,0,0,uVar16,uVar18,uVar10,&local_168,&local_148,
                                     &local_110,&local_128);
              uVar4 = (undefined4)((ulonglong)uVar10 >> 0x20);
              plVar11 = *(longlong **)(param_1 + 0xd70);
              if (plVar11 != (longlong *)*plVar9) {
                *(longlong **)(param_1 + 0xd70) = (longlong *)*plVar9;
                *plVar9 = 0;
                if (plVar11 != (longlong *)0x0) {
                  (**(code **)(*plVar11 + 0x10))();
                }
              }
              if (local_f8[0] != (IUnknown *)0x0) {
                (**(code **)(*(longlong *)local_f8[0] + 0x10))();
              }
              if (local_168 == 8) {
                local_168 = 0;
                if (plStack_160 != (longlong *)0x0) {
                  (*DAT_143ad5990)((longlong)plStack_160 - 4);
                }
              }
              else {
                (*DAT_143262a18)(&local_168);
              }
              if (local_148 == 8) {
                local_148 = 0;
                if (plStack_140 != (longlong *)0x0) {
                  (*DAT_143ad5990)((longlong)plStack_140 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_148);
              }
              if (local_110 == 8) {
                local_110 = 0;
                if (lStack_108 != 0) {
                  (*DAT_143ad5990)(lStack_108 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_110);
              }
              if (local_128 == 8) {
                local_128 = 0;
                if (lStack_120 != 0) {
                  (*DAT_143ad5990)(lStack_120 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_128);
              }
              pIVar2 = *(IUnknown **)(param_1 + 0xd70);
              if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x80004003);
              }
              plStack_160 = *(longlong **)(param_1 + 0xcf8);
              local_168 = 0xd;
              if (plStack_160 != (longlong *)0x0) {
                (**(code **)(*plStack_160 + 8))();
              }
              local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
              plStack_e0 = plStack_160;
              local_d8 = local_158;
              iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
              if (iVar3 < 0) {
                _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
              }
              if (local_168 == 8) {
                local_168 = 0;
                if (plStack_160 != (longlong *)0x0) {
                  (*DAT_143ad5990)((longlong)plStack_160 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_168);
              }
              pIVar2 = *(IUnknown **)(param_1 + 0xd70);
              if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
                FUN_142ef3ac0(0x80004003);
              }
              plStack_160 = *(longlong **)(param_1 + 0xd58);
              local_168 = 0xd;
              if (plStack_160 != (longlong *)0x0) {
                (**(code **)(*plStack_160 + 8))();
              }
              local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
              plStack_e0 = plStack_160;
              local_d8 = local_158;
              iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x238))(pIVar2,&local_e8);
              if (iVar3 < 0) {
                _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
              }
              if (local_168 == 8) {
                local_168 = 0;
                if (plStack_160 != (longlong *)0x0) {
                  (*DAT_143ad5990)((longlong)plStack_160 + -4);
                }
              }
              else {
                (*DAT_143262a18)(&local_168);
              }
              pIVar2 = *(IUnknown **)(param_1 + 0xd70);
              if (pIVar2 != (IUnknown *)0x0) {
                iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x200))(pIVar2,0xffffff);
                if (iVar3 < 0) {
                  _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
                }
                lVar15 = DAT_143add050;
                if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
                  FUN_142ef3ac0(0x80004003);
                }
                (*DAT_143262a20)(&local_128);
                iVar3 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
                if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
                  FUN_142ef3ac0(iVar3);
                }
                (*DAT_143262a20)(&local_110);
                iVar3 = FUN_14023c4c0(&local_110,&DAT_143a8b8d8);
                if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
                  FUN_142ef3ac0(iVar3);
                }
                uVar5 = 2;
                if (*(int *)(param_1 + 0xe58) == 100) {
                  uVar5 = 0;
                }
                local_148 = 3;
                plStack_140 = (longlong *)CONCAT44(plStack_140._4_4_,uVar5);
                local_168 = 3;
                plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
                uVar21 = CONCAT44(uVar4,1);
                uVar18 = uVar18 & 0xffffffff00000000;
                uVar16 = uVar16 & 0xffffffff00000000;
                plVar9 = (longlong *)
                         FUN_140d8d300(lVar15,local_f8,0,0,uVar16,uVar18,uVar21,&local_168,
                                       &local_148,&local_110,&local_128);
                plVar11 = *(longlong **)(param_1 + 0xd90);
                if (plVar11 != (longlong *)*plVar9) {
                  *(longlong **)(param_1 + 0xd90) = (longlong *)*plVar9;
                  *plVar9 = 0;
                  if (plVar11 != (longlong *)0x0) {
                    (**(code **)(*plVar11 + 0x10))();
                  }
                }
                if (local_f8[0] != (IUnknown *)0x0) {
                  (**(code **)(*(longlong *)local_f8[0] + 0x10))();
                }
                if (local_168 == 8) {
                  local_168 = 0;
                  if (plStack_160 != (longlong *)0x0) {
                    (*DAT_143ad5990)((longlong)plStack_160 - 4);
                  }
                }
                else {
                  (*DAT_143262a18)(&local_168);
                }
                if (local_148 == 8) {
                  local_148 = 0;
                  if (plStack_140 != (longlong *)0x0) {
                    (*DAT_143ad5990)((longlong)plStack_140 + -4);
                  }
                }
                else {
                  (*DAT_143262a18)(&local_148);
                }
                if (local_110 == 8) {
                  local_110 = 0;
                  if (lStack_108 != 0) {
                    (*DAT_143ad5990)(lStack_108 + -4);
                  }
                }
                else {
                  (*DAT_143262a18)(&local_110);
                }
                if (local_128 == 8) {
                  local_128 = 0;
                  if (lStack_120 != 0) {
                    (*DAT_143ad5990)(lStack_120 + -4);
                  }
                }
                else {
                  (*DAT_143262a18)(&local_128);
                }
                pIVar2 = *(IUnknown **)(param_1 + 0xd90);
                if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
                  FUN_142ef3ac0(0x80004003);
                }
                plStack_160 = *(longlong **)(param_1 + 0xd40);
                local_168 = 0xd;
                if (plStack_160 != (longlong *)0x0) {
                  (**(code **)(*plStack_160 + 8))();
                }
                local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
                plStack_e0 = plStack_160;
                local_d8 = local_158;
                iVar3 = (**(code **)(*(longlong *)pIVar2 + 200))(pIVar2,&local_e8);
                if (iVar3 < 0) {
                  _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
                }
                if (local_168 == 8) {
                  local_168 = 0;
                  if (plStack_160 != (longlong *)0x0) {
                    (*DAT_143ad5990)((longlong)plStack_160 + -4);
                  }
                }
                else {
                  (*DAT_143262a18)(&local_168);
                }
                pIVar2 = *(IUnknown **)(param_1 + 0xd90);
                if (pIVar2 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
                  FUN_142ef3ac0(0x80004003);
                }
                plStack_160 = *(longlong **)(param_1 + 0xd78);
                local_168 = 0xd;
                if (plStack_160 != (longlong *)0x0) {
                  (**(code **)(*plStack_160 + 8))();
                }
                local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
                plStack_e0 = plStack_160;
                local_d8 = local_158;
                iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x238))(pIVar2,&local_e8);
                if (iVar3 < 0) {
                  _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
                }
                if (local_168 == 8) {
                  local_168 = 0;
                  if (plStack_160 != (longlong *)0x0) {
                    (*DAT_143ad5990)((longlong)plStack_160 + -4);
                  }
                }
                else {
                  (*DAT_143262a18)(&local_168);
                }
                pIVar2 = *(IUnknown **)(param_1 + 0xd90);
                if (pIVar2 != (IUnknown *)0x0) {
                  iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x200))(pIVar2,0xffffffff);
                  if (iVar3 < 0) {
                    _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
                  }
                  lVar15 = DAT_143add050;
                  if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
                    FUN_142ef3ac0(0x80004003);
                  }
                  (*DAT_143262a20)(&local_128);
                  iVar3 = FUN_14023c4c0(&local_128,&DAT_143a8b8d8);
                  if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
                    FUN_142ef3ac0(iVar3);
                  }
                  (*DAT_143262a20)(&local_110);
                  iVar3 = FUN_14023c4c0(&local_110,&DAT_143a8b8d8);
                  if (-1 < iVar3) {
                    (*DAT_143262a20)(&local_148);
                    iVar3 = FUN_14023c4c0(&local_148,&DAT_143a8b8d8);
                    if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
                      FUN_142ef3ac0(iVar3);
                    }
                    local_168 = 3;
                    plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
                    uVar10 = FUN_140d8d300(lVar15,local_f8,0,0,uVar16 & 0xffffffff00000000,
                                           uVar18 & 0xffffffff00000000,uVar21 & 0xffffffff00000000,
                                           &local_168,&local_148,&local_110,&local_128);
                    FUN_140cb04e0(param_1 + 0xd80,uVar10);
                    if (local_f8[0] != (IUnknown *)0x0) {
                      (**(code **)(*(longlong *)local_f8[0] + 0x10))();
                    }
                    if (local_168 == 8) {
                      local_168 = 0;
                      if (plStack_160 != (longlong *)0x0) {
                        (*DAT_143ad5990)((longlong)plStack_160 - 4);
                      }
                    }
                    else {
                      (*DAT_143262a18)(&local_168);
                    }
                    if (local_148 == 8) {
                      local_148 = 0;
                      if (plStack_140 != (longlong *)0x0) {
                        (*DAT_143ad5990)((longlong)plStack_140 + -4);
                      }
                    }
                    else {
                      (*DAT_143262a18)(&local_148);
                    }
                    if (local_110 == 8) {
                      local_110 = 0;
                      if (lStack_108 != 0) {
                        (*DAT_143ad5990)(lStack_108 + -4);
                      }
                    }
                    else {
                      (*DAT_143262a18)(&local_110);
                    }
                    if (local_128 == 8) {
                      local_128 = 0;
                      if (lStack_120 != 0) {
                        (*DAT_143ad5990)(lStack_120 + -4);
                      }
                    }
                    else {
                      (*DAT_143262a18)(&local_128);
                    }
                    pIVar2 = *(IUnknown **)(param_1 + 0xd80);
                    if (pIVar2 != (IUnknown *)0x0) {
                      plStack_160 = *(longlong **)(param_1 + 0xd58);
                      local_168 = 0xd;
                      if (plStack_160 != (longlong *)0x0) {
                        (**(code **)(*plStack_160 + 8))();
                      }
                      local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
                      plStack_e0 = plStack_160;
                      local_d8 = local_158;
                      iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x238))(pIVar2,&local_e8);
                      if (iVar3 < 0) {
                        _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_14327fcb0);
                      }
                      if (local_168 == 8) {
                        local_168 = 0;
                        if (plStack_160 != (longlong *)0x0) {
                          (*DAT_143ad5990)((longlong)plStack_160 + -4);
                        }
                      }
                      else {
                        (*DAT_143262a18)(&local_168);
                      }
                      pIVar2 = (IUnknown *)*param_2;
                      if (pIVar2 != (IUnknown *)0x0) {
                        (*DAT_143262a20)(&local_168);
                        iVar3 = FUN_14023c4c0(&local_168,&DAT_143a8b8d8);
                        if (-1 < iVar3) {
                          local_e8 = CONCAT44(uStack_164,CONCAT22(uStack_166,local_168));
                          plStack_e0 = plStack_160;
                          local_d8 = local_158;
                          iVar3 = (**(code **)(*(longlong *)pIVar2 + 0x138))
                                            (pIVar2,local_b0,local_ac,0,0,0,0,0,0,&local_e8);
                          if (iVar3 < 0) {
                            _com_issue_errorex(iVar3,pIVar2,(_GUID *)&DAT_143273488);
                          }
                          if (local_168 == 8) {
                            local_168 = 0;
                            if (plStack_160 != (longlong *)0x0) {
                              (*DAT_143ad5990)((longlong)plStack_160 + -4);
                            }
                          }
                          else {
                            (*DAT_143262a18)(&local_168);
                          }
                          uVar10 = FUN_1409d4040(param_1 + 0xe28,local_b0);
                          FUN_1409d3c60(param_1 + 0xdf8,uVar10);
                          if (*(longlong **)(param_1 + 0xdb0) != (longlong *)0x0) {
                            (**(code **)(**(longlong **)(param_1 + 0xdb0) + 0x10))();
                          }
                          *(undefined8 *)(param_1 + 0xdb0) = 0;
                          lVar15 = *(longlong *)(param_1 + 0xdc0);
                          FUN_140fbc830(param_1 + 0xdc0,param_1 + 0xdc0,*(undefined8 *)(lVar15 + 8))
                          ;
                          *(longlong *)(lVar15 + 8) = lVar15;
                          *(longlong *)lVar15 = lVar15;
                          *(longlong *)(lVar15 + 0x10) = lVar15;
                          *(undefined8 *)(param_1 + 0xdc8) = 0;
                          *(undefined8 *)(param_1 + 0x58c) = 0;
                          if ((longlong *)*param_2 != (longlong *)0x0) {
                            (**(code **)(*(longlong *)*param_2 + 0x10))();
                          }
                          if ((longlong *)*param_5 != (longlong *)0x0) {
                            (**(code **)(*(longlong *)*param_5 + 0x10))();
                          }
                          return;
                        }
                    /* WARNING: Subroutine does not return */
                        FUN_142ef3ac0(iVar3);
                      }
                    /* WARNING: Subroutine does not return */
                      FUN_142ef3ac0(0x80004003);
                    }
                    /* WARNING: Subroutine does not return */
                    FUN_142ef3ac0(0x80004003);
                  }
                    /* WARNING: Subroutine does not return */
                  FUN_142ef3ac0(iVar3);
                }
              }
            }
          }
        }
      }
    }
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x80004003);
}



//===========================================================
// FUN_1427e87e0 @ 1427e87e0   (1219 bytes)
//===========================================================

void FUN_1427e87e0(longlong *param_1,longlong param_2)

{
  longlong lVar1;
  longlong *plVar2;
  longlong *plVar3;
  char cVar4;
  char cVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  undefined8 uVar9;
  longlong lVar10;
  undefined8 uVar11;
  longlong *plVar12;
  ulonglong uVar13;
  char cVar14;
  longlong lVar15;
  bool bVar16;
  char local_res8 [8];
  undefined1 local_68 [8];
  longlong local_60;
  undefined **local_58;
  undefined8 local_50;
  undefined8 uStack_48;
  undefined8 local_40;
  undefined2 local_38;
  int local_34;
  
  local_40 = 0;
  local_50 = 0;
  uStack_48 = 0;
  local_58 = &PTR_FUN_143304508;
  cVar5 = '\0';
  local_38 = 0;
  cVar14 = '\0';
  local_34 = -0x40000000;
  if (param_2 != 0) {
    cVar5 = *(char *)(param_2 + 0x20);
    cVar14 = *(char *)(param_2 + 0x21);
    local_38 = *(undefined2 *)(param_2 + 0x20);
    local_34 = *(int *)(param_2 + 0x24);
  }
  iVar8 = local_34;
  uVar9 = (**(code **)(param_1[1] + 0x50))(param_1 + 1);
  lVar10 = FUN_141892840();
  if (lVar10 != 0) {
    uVar11 = FUN_141892840();
    cVar4 = FUN_141bc0990(uVar11);
    local_res8[0] = '\x01';
    if (cVar4 != '\0') goto LAB_1427e888a;
  }
  local_res8[0] = '\0';
LAB_1427e888a:
  iVar6 = FUN_1409c6ce0(uVar9);
  iVar7 = FUN_1409c6cc0(uVar9);
  iVar6 = (iVar7 * 3000 - iVar6) * 10 + -0x3fff8ada;
  if (cVar5 != '\0') {
    iVar6 = -0x3ffbe150;
  }
  lVar10 = (**(code **)(*param_1 + 0x48))(param_1);
  iVar7 = FUN_1401ba9d0(lVar10 + 0x13fc,*(undefined4 *)(lVar10 + 0x1404));
  cVar5 = local_res8[0];
  if (0 < iVar7) {
    iVar6 = -0x3ffbe089;
  }
  if (local_res8[0] != '\0') {
    lVar10 = (**(code **)(param_1[1] + 0x30))(param_1 + 1,local_res8);
    iVar6 = *(int *)(lVar10 + 4) * 10 + -0x3ffccbb0;
  }
  if (cVar14 != '\0') {
    iVar6 = iVar8;
  }
  if (cVar5 == '\0') {
    if ((char)param_1[0x781] == '\0') {
      iVar8 = FUN_1409c5080(uVar9);
      bVar16 = iVar8 == 0;
    }
    else if (*(char *)((longlong)param_1 + 0x3c09) == '\0') {
      if (DAT_143aa8518 == 0) {
        iVar8 = (**(code **)(*param_1 + 0x58))(param_1);
        bVar16 = iVar8 == 0;
      }
      else {
        iVar8 = FUN_1427b9a70();
        iVar7 = FUN_1427b9a70(param_1);
        bVar16 = iVar7 != iVar8;
      }
    }
    else {
      iVar8 = (**(code **)(*param_1 + 0x50))(param_1);
      bVar16 = iVar8 == 0;
    }
    iVar8 = 2;
    if (!bVar16) {
      iVar8 = 7;
    }
    iVar6 = iVar6 + iVar8;
    if (*(int *)((longlong)param_1 + 0x1264) == 0) {
      if (((int)param_1[0x24c] != 0) &&
         (plVar12 = (longlong *)FUN_1429b6c90(DAT_143ac1b90), plVar12 != (longlong *)0x0)) {
        iVar8 = FUN_1409bd0e0(uVar9);
        if (iVar8 == 0) {
          iVar8 = (**(code **)(*plVar12 + 0x50))(plVar12);
          if (iVar8 != 0) {
            uVar9 = (**(code **)(plVar12[1] + 0x50))();
            iVar8 = FUN_1409bd0e0(uVar9);
            if (iVar8 == 0) {
              iVar6 = FUN_140f81350(plVar12 + 0x20);
              iVar6 = iVar6 + -1;
            }
            else {
              iVar6 = FUN_140f81350(plVar12 + 0x20);
              iVar6 = iVar6 + 1;
            }
          }
        }
        else {
          iVar8 = (**(code **)(*param_1 + 0x50))(param_1);
          if (iVar8 == 0) {
            iVar6 = FUN_140f81350(plVar12 + 0x20);
            iVar6 = iVar6 + 1;
          }
        }
      }
    }
    else {
      plVar12 = (longlong *)FUN_1429b6c90(DAT_143ac1b90);
      if (plVar12 != (longlong *)0x0) {
        iVar8 = FUN_1409bd0e0(uVar9);
        if (iVar8 == 0) {
          iVar8 = (**(code **)(*param_1 + 0x50))(param_1);
          if ((iVar8 == 0) || (iVar8 = FUN_140f81350(plVar12 + 0x20), iVar8 <= iVar6)) {
            iVar8 = (**(code **)(*plVar12 + 0x50))(plVar12);
            if (iVar8 == 0) {
              iVar8 = (**(code **)(*param_1 + 0x50))(param_1);
              if (iVar8 == 0) {
                iVar6 = iVar6 + 1;
              }
            }
            else {
              iVar6 = iVar6 + 6;
            }
          }
          else {
            FUN_140f81230(plVar12 + 0x20,iVar6 + -1);
          }
        }
        else {
          iVar8 = (**(code **)(*param_1 + 0x50))();
          if ((iVar8 == 0) || (iVar8 = FUN_140f81350(plVar12 + 0x20), iVar6 <= iVar8)) {
            iVar8 = (**(code **)(*plVar12 + 0x50))(plVar12);
            if (iVar8 != 0) {
              iVar6 = iVar6 + 4;
            }
          }
          else {
            FUN_140f81230(plVar12 + 0x20,iVar6 + 1);
          }
        }
      }
    }
  }
  FUN_140f81230(param_1 + 0x20,iVar6);
  FUN_140f82790(param_1 + 0x20,0,0);
  lVar10 = param_1[0x240];
  while (lVar10 != 0) {
    uVar13 = *(ulonglong *)(lVar10 + -0x20);
    if ((uVar13 != 0) && (uVar13 < 0x10001)) {
      FUN_142e52ed0(0x33e);
      uVar13 = *(ulonglong *)(lVar10 + -0x20);
    }
    lVar15 = 0;
    if (uVar13 != 0) {
      lVar15 = uVar13 + 0x28;
    }
    lVar1 = *(longlong *)(lVar10 + 8);
    local_60 = lVar1;
    if (lVar1 != 0) {
      if (0xfffff < *(ulonglong *)(lVar1 + 0x18)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar1 + 0x18) = *(longlong *)(lVar1 + 0x18) + 1;
      UNLOCK();
    }
    lVar10 = lVar15;
    if (lVar1 == 0) {
      FUN_1419ef260(local_68);
    }
    else {
      FUN_1420d0630(local_60,0);
      FUN_1419ef260(local_68);
    }
  }
  iVar8 = (**(code **)(*param_1 + 0x50))(param_1);
  if ((iVar8 != 0) && (DAT_143aa8518 != 0)) {
    FUN_1428eb520();
  }
  lVar10 = FUN_141892840();
  if (lVar10 != 0) {
    uVar9 = FUN_141892840();
    cVar5 = FUN_141bc0990(uVar9);
    if ((((cVar5 != '\0') && (lVar10 = param_1[0x23b], lVar10 != 0)) && (*(int *)(lVar10 + -8) != 0)
        ) && (*(longlong *)(lVar10 + 8) != 0)) {
      thunk_FUN_141eca710(*(undefined8 *)(lVar10 + 8));
    }
  }
  plVar12 = *(longlong **)param_1[0x7ef];
  cVar5 = *(char *)((longlong)plVar12 + 0x19);
  while (cVar5 == '\0') {
    if (plVar12[5] != 0) {
      FUN_1413ed540();
    }
    plVar2 = (longlong *)plVar12[2];
    if (*(char *)((longlong)plVar2 + 0x19) == '\0') {
      cVar5 = *(char *)(*plVar2 + 0x19);
      plVar12 = plVar2;
      plVar2 = (longlong *)*plVar2;
      while (cVar5 == '\0') {
        cVar5 = *(char *)(*plVar2 + 0x19);
        plVar12 = plVar2;
        plVar2 = (longlong *)*plVar2;
      }
    }
    else {
      cVar5 = *(char *)(plVar12[1] + 0x19);
      plVar3 = (longlong *)plVar12[1];
      plVar2 = plVar12;
      while ((plVar12 = plVar3, cVar5 == '\0' && (plVar2 == (longlong *)plVar12[2]))) {
        cVar5 = *(char *)(plVar12[1] + 0x19);
        plVar3 = (longlong *)plVar12[1];
        plVar2 = plVar12;
      }
    }
    cVar5 = *(char *)((longlong)plVar12 + 0x19);
  }
  return;
}



//===========================================================
// FUN_1407f5ce0 @ 1407f5ce0   (3 bytes)
//===========================================================

undefined8 FUN_1407f5ce0(void)

{
  return 0;
}



//===========================================================
// FUN_1420f8a20 @ 1420f8a20   (2017 bytes)
//===========================================================

void FUN_1420f8a20(longlong param_1,undefined4 param_2,undefined8 param_3)

{
  char cVar1;
  longlong lVar2;
  undefined1 auVar3 [16];
  undefined8 *puVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  undefined8 *puVar8;
  longlong lVar9;
  undefined8 *puVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  longlong *plVar13;
  longlong lVar14;
  undefined8 *puVar15;
  longlong *plVar16;
  longlong local_a8;
  longlong *local_a0;
  undefined1 local_98 [8];
  longlong local_90;
  undefined1 local_88 [8];
  longlong *local_80;
  undefined1 local_78 [33];
  undefined1 local_57;
  
  uVar5 = FUN_1406e8c20(param_3);
  uVar12 = CONCAT44(0,uVar5);
  if (DAT_143ac87a0 == 0) goto LAB_1420f8aa1;
  FUN_1415ed8e0(local_78);
  if (DAT_143aa8518 == 0) {
LAB_1420f8a83:
    local_57 = 0;
  }
  else {
    uVar6 = FUN_14276df20();
    local_57 = 1;
    if (uVar6 != uVar5) goto LAB_1420f8a83;
  }
  FUN_1415f0bf0(DAT_143ac87a0,local_78);
  FUN_1415ed910(local_78);
LAB_1420f8aa1:
  uVar6 = FUN_1406e8c20(param_3);
  if ((DAT_143aa8518 != 0) && (uVar7 = FUN_14276df20(DAT_143aa8518), uVar7 != uVar5)) {
    puVar15 = *(undefined8 **)(param_1 + 0x20);
    cVar1 = *(char *)((longlong)puVar15[1] + 0x19);
    puVar4 = puVar15;
    puVar8 = (undefined8 *)puVar15[1];
    while (cVar1 == '\0') {
      if (*(uint *)((longlong)puVar8 + 0x1c) < uVar6) {
        puVar10 = (undefined8 *)puVar8[2];
        puVar8 = puVar4;
      }
      else {
        puVar10 = (undefined8 *)*puVar8;
      }
      puVar4 = puVar8;
      puVar8 = puVar10;
      cVar1 = *(char *)((longlong)puVar10 + 0x19);
    }
    if ((((*(char *)((longlong)puVar4 + 0x19) == '\0') &&
         (*(uint *)((longlong)puVar4 + 0x1c) <= uVar6)) && (puVar4 != puVar15)) &&
       (lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12), lVar9 != 0)) {
      FUN_1427973e0(lVar9,&local_a8,uVar6);
      if (local_a0 == (longlong *)0x0) {
        local_90 = 0;
        lVar14 = 0;
        if (*(longlong *)(param_1 + 8) != 0) {
          auVar3._8_8_ = 0;
          auVar3._0_8_ = uVar12;
          auVar3 = auVar3 % ZEXT416(*(uint *)(param_1 + 0x10));
          for (lVar2 = *(longlong *)(*(longlong *)(param_1 + 8) + auVar3._0_8_ * 8); lVar14 = 0,
              lVar2 != 0; lVar2 = *(longlong *)(lVar2 + 8)) {
            if (*(uint *)(lVar2 + 0x10) == uVar5) {
              if (local_98 == (undefined1 *)(lVar2 + 0x18)) {
                FUN_142e52d50(0x45c,CONCAT71(auVar3._1_7_,1));
              }
              lVar14 = *(longlong *)(lVar2 + 0x20);
              if (lVar14 != 0) {
                if (0xfffff < *(ulonglong *)(lVar14 + -0x20)) {
                  FUN_142e541f0(0x30f);
                }
                LOCK();
                *(longlong *)(lVar14 + -0x20) = *(longlong *)(lVar14 + -0x20) + 1;
                UNLOCK();
              }
              lVar14 = *(longlong *)(lVar2 + 0x20);
              local_90 = lVar14;
              if (lVar2 != -0x18) {
                if (lVar14 == 0) {
                  FUN_142e52ed0(0x431,0);
                }
                if (*(longlong **)(lVar14 + 8) != (longlong *)0x0) {
                  plVar16 = *(longlong **)(lVar14 + 8);
                  do {
                    uVar11 = plVar16[-4];
                    if ((uVar11 != 0) && (uVar11 < 0x10001)) {
                      FUN_142e52ed0(0x33e);
                      uVar11 = plVar16[-4];
                    }
                    plVar13 = (longlong *)0x0;
                    if (uVar11 != 0) {
                      plVar13 = (longlong *)(uVar11 + 0x28);
                    }
                    if (((longlong)local_a0 - 1U < 999) ||
                       (local_a0 == (longlong *)0xffffffffffffffff)) {
                      FUN_142e52ed0(0x447,local_a0);
                    }
                    if (&local_a8 == plVar16) {
                      FUN_142e52d50(0x45c,1);
                    }
                    lVar14 = plVar16[1];
                    if (lVar14 != 0) {
                      if (0xfffff < *(ulonglong *)(lVar14 + 0x18)) {
                        FUN_142e541f0(0x30f);
                      }
                      LOCK();
                      *(longlong *)(lVar14 + 0x18) = *(longlong *)(lVar14 + 0x18) + 1;
                      UNLOCK();
                    }
                    FUN_1419ef260(&local_a8);
                    local_a0 = (longlong *)plVar16[1];
                    if ((local_a0 != (longlong *)0x0) &&
                       (uVar7 = (**(code **)(*local_a0 + 8))(local_a0), plVar16 = local_a0,
                       uVar7 == uVar6)) {
                      local_80 = local_a0;
                      if (local_a0 != (longlong *)0x0) {
                        if (0xfffff < (ulonglong)local_a0[3]) {
                          FUN_142e541f0(0x30f);
                        }
                        LOCK();
                        plVar16[3] = plVar16[3] + 1;
                        UNLOCK();
                      }
                      FUN_142795c80(lVar9,local_88);
                    }
                    lVar14 = local_90;
                    plVar16 = plVar13;
                  } while (plVar13 != (longlong *)0x0);
                }
              }
              break;
            }
          }
        }
        if (lVar14 != 0) {
          puVar15 = (undefined8 *)(lVar14 + -0x28);
          if (0xffffe < *(longlong *)(lVar14 + -0x20) - 1U) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar16 = (longlong *)(lVar14 + -0x20);
          lVar9 = *plVar16;
          *plVar16 = *plVar16 + -1;
          UNLOCK();
          if ((int)lVar9 == 1) {
            if (*(longlong *)(local_90 + -0x10) != 0) {
              LOCK();
              *(undefined8 *)(*(longlong *)(local_90 + -0x10) + 8) = 0;
              UNLOCK();
              do {
              } while (*(int *)(*(longlong *)(local_90 + -0x10) + 4) != 0);
            }
            if (puVar15 != (undefined8 *)0x0) {
              (**(code **)*puVar15)(puVar15,1);
            }
          }
        }
        FUN_1419ef260(&local_a8);
      }
      else {
        FUN_1419ef260(&local_a8);
      }
    }
  }
  switch(param_2) {
  case 0x3a0:
    FUN_1420f92b0(param_1,uVar12,uVar6,param_3);
    break;
  case 0x3ab:
    FUN_1420fa150(param_1,uVar12,uVar6,param_3);
    break;
  case 0x3ac:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_142796260(lVar9,param_3,uVar6);
    }
    break;
  case 0x3ad:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_142796410(lVar9,param_3,uVar6);
    }
    break;
  case 0x3ae:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_14279b560(lVar9,param_3,uVar6);
    }
    break;
  case 0x3af:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_14279b8c0(lVar9,param_3,uVar6);
    }
    break;
  case 0x3b0:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_1427965c0(lVar9,param_3,uVar6);
    }
    break;
  case 0x3b2:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_14279b710(lVar9,param_3,uVar6);
    }
    break;
  case 0x3b3:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_142799600(lVar9,param_3,uVar6);
    }
    break;
  case 0x3b4:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_1427997b0(lVar9,param_3,uVar6);
    }
    break;
  case 0x3b5:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_142799960(lVar9,param_3,uVar6);
    }
    break;
  case 0x3b6:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_14279ba70(lVar9,param_3,uVar6);
    }
    break;
  case 0x3b7:
    if ((DAT_143aa8518 != 0) && (uVar7 = FUN_14276df20(DAT_143aa8518), uVar7 == uVar5)) {
      FUN_14279ab20(DAT_143aa8518,param_3,uVar6);
    }
    break;
  case 0x3b8:
    if ((DAT_143aa8518 != 0) && (uVar7 = FUN_14276df20(DAT_143aa8518), uVar7 == uVar5)) {
      FUN_14279acd0(DAT_143aa8518,param_3,uVar6);
    }
    break;
  case 0x3b9:
    if ((DAT_143aa8518 != 0) && (uVar7 = FUN_14276df20(DAT_143aa8518), uVar7 == uVar5)) {
      FUN_14279ae80(DAT_143aa8518,param_3,uVar6);
    }
    break;
  case 0x3ba:
    if ((DAT_143aa8518 != 0) && (uVar7 = FUN_14276df20(DAT_143aa8518), uVar7 == uVar5)) {
      FUN_14279b030(DAT_143aa8518,param_3,uVar6);
    }
    break;
  case 0x3bb:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_142799b10(lVar9,param_3,uVar6);
    }
    break;
  case 0x3bc:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_142799cc0(lVar9,param_3,uVar6);
    }
    break;
  case 0x3bd:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_142799e70(lVar9,param_3,uVar6);
    }
    break;
  case 0x3be:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_14279a020(lVar9,param_3,uVar6);
    }
    break;
  case 0x3bf:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_14279a1d0(lVar9,param_3,uVar6);
    }
    break;
  case 0x3c0:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_14279a380(lVar9,param_3,uVar6);
    }
    break;
  case 0x3c1:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_14279a530(lVar9,param_3,uVar6);
    }
    break;
  case 0x3c3:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_14279b1e0(lVar9,param_3,uVar6);
    }
    break;
  case 0x3c4:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_14279b3b0(lVar9,param_3,uVar6);
    }
    break;
  case 0x3c5:
    lVar9 = FUN_1429b6c90(DAT_143ac1b90,uVar12);
    if (lVar9 != 0) {
      FUN_142796770(lVar9,param_3,uVar6);
    }
  }
  if (DAT_143ac87a0 != 0) {
    FUN_1415f0d70();
  }
  return;
}



//===========================================================
// FUN_1420f92b0 @ 1420f92b0   (3725 bytes)
//===========================================================

void FUN_1420f92b0(longlong param_1,uint param_2,uint param_3,undefined8 param_4)

{
  int *piVar1;
  undefined1 auVar2 [16];
  undefined8 *puVar3;
  void *pvVar4;
  ulonglong uVar5;
  char cVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  int iVar9;
  uint uVar10;
  uint uVar11;
  longlong lVar12;
  longlong lVar13;
  undefined8 uVar14;
  longlong *plVar15;
  undefined8 *puVar16;
  longlong *plVar17;
  undefined8 *puVar18;
  ulonglong uVar19;
  undefined8 *puVar20;
  longlong *plVar21;
  longlong *plVar22;
  longlong *plVar23;
  bool bVar24;
  uint local_res10 [2];
  uint local_res18;
  undefined8 local_res20;
  longlong local_108;
  longlong *local_100;
  ulonglong local_f8;
  undefined1 local_f0;
  undefined1 local_ef;
  undefined1 local_ee;
  undefined1 local_ed;
  undefined1 local_ec [4];
  undefined4 local_e8;
  undefined4 local_e4;
  longlong *local_e0;
  undefined1 local_d8 [8];
  undefined8 *local_d0;
  undefined8 *local_c0;
  undefined1 local_b8 [8];
  longlong local_b0;
  undefined8 local_a8;
  uint uStack_a0;
  undefined4 uStack_9c;
  longlong *local_88;
  longlong *local_78;
  longlong *local_70;
  longlong *local_68;
  undefined8 local_60;
  undefined4 local_58;
  undefined4 uStack_54;
  uint uStack_50;
  undefined4 uStack_4c;
  
  uVar19 = CONCAT44(0,param_2);
  local_res10[0] = param_2;
  local_res18 = param_3;
  local_res20 = param_4;
  lVar12 = FUN_1429b6c90(DAT_143ac1b90);
  uVar7 = FUN_1406e8c20(param_4);
  local_f8 = CONCAT44(local_f8._4_4_,uVar7);
  uVar8 = FUN_1406e8c20(param_4);
  local_e8 = uVar8;
  uVar7 = FUN_1406e8c20(param_4);
  local_e4 = uVar7;
  lVar13 = FUN_141892840();
  if (lVar13 != 0) {
    uVar14 = FUN_141892840();
    cVar6 = FUN_141bc8c60(uVar14);
    if (((cVar6 != '\0') && (lVar12 == 0)) && (param_2 != 0)) {
      return;
    }
  }
  lVar12 = DAT_143aa84a0;
  if (((DAT_143aa84a0 != 0) && (iVar9 = FUN_142cc3d80(DAT_143aa84a0), iVar9 != 0)) &&
     ((param_2 != 0 && (uVar10 = FUN_142cb9550(lVar12), param_2 != uVar10)))) {
    return;
  }
  plVar23 = (longlong *)0x0;
  local_100 = (longlong *)0x0;
  local_b0 = 0;
  local_e0 = (longlong *)(param_1 + 8);
  plVar21 = plVar23;
  if (*local_e0 != 0) {
    auVar2._8_8_ = 0;
    auVar2._0_8_ = uVar19;
    auVar2 = auVar2 % ZEXT416(*(uint *)(param_1 + 0x10));
    for (lVar12 = *(longlong *)(*local_e0 + auVar2._0_8_ * 8); plVar21 = (longlong *)0x0,
        lVar12 != 0; lVar12 = *(longlong *)(lVar12 + 8)) {
      if (*(uint *)(lVar12 + 0x10) == param_2) {
        if (local_b8 == (undefined1 *)(lVar12 + 0x18)) {
          FUN_142e52d50(0x45c,CONCAT71(auVar2._1_7_,1));
        }
        lVar13 = *(longlong *)(lVar12 + 0x20);
        plVar21 = plVar23;
        if (lVar13 != 0) {
          if (0xfffff < *(ulonglong *)(lVar13 + -0x20)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(lVar13 + -0x20) = *(longlong *)(lVar13 + -0x20) + 1;
          UNLOCK();
          uVar19 = (ulonglong)local_res10[0];
          plVar21 = local_100;
        }
        local_b0 = *(longlong *)(lVar12 + 0x20);
        uVar7 = local_e4;
        uVar8 = local_e8;
        if (lVar12 != -0x18) {
          if (local_b0 == 0) {
            FUN_142e52ed0(0x431,0);
          }
          uVar10 = local_res18;
          plVar15 = *(longlong **)(local_b0 + 8);
          uVar7 = local_e4;
          uVar8 = local_e8;
          if (plVar15 != (longlong *)0x0) goto LAB_1420f9450;
        }
        break;
      }
    }
  }
  goto LAB_1420f9645;
  while (plVar15 = plVar22, plVar22 != (longlong *)0x0) {
LAB_1420f9450:
    uVar19 = plVar15[-4];
    if ((uVar19 != 0) && (uVar19 < 0x10001)) {
      FUN_142e52ed0(0x33e);
      uVar19 = plVar15[-4];
    }
    plVar22 = plVar23;
    if (uVar19 != 0) {
      plVar22 = (longlong *)(uVar19 + 0x28);
    }
    if (((longlong)plVar21 - 1U < 999) || (plVar21 == (longlong *)0xffffffffffffffff)) {
      FUN_142e52ed0(0x447,plVar21);
    }
    if (&local_108 == plVar15) {
      FUN_142e52d50(0x45c,1);
    }
    lVar12 = plVar15[1];
    if (lVar12 != 0) {
      if (0xfffff < *(ulonglong *)(lVar12 + 0x18)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar12 + 0x18) = *(longlong *)(lVar12 + 0x18) + 1;
      UNLOCK();
      plVar21 = local_100;
    }
    if (plVar21 != (longlong *)0x0) {
      puVar18 = (undefined8 *)plVar21[7];
      local_c0 = puVar18;
      if (puVar18 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar18[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar18[1] = puVar18[1] + 1;
        UNLOCK();
      }
      puVar18 = local_c0;
      pvVar4 = Self;
      plVar21 = local_c0 + 4;
      LOCK();
      lVar12 = *plVar21;
      if (lVar12 == 0) {
        *plVar21 = (longlong)Self;
      }
      UNLOCK();
      local_70 = plVar21;
      if (lVar12 == 0) {
LAB_1420f9561:
        *(undefined4 *)(puVar18 + 5) = 1;
      }
      else {
        if ((void *)*plVar21 != pvVar4) {
          while( true ) {
            pvVar4 = Self;
            LOCK();
            lVar12 = *plVar21;
            if (lVar12 == 0) {
              *plVar21 = (longlong)Self;
            }
            UNLOCK();
            if (lVar12 == 0) goto LAB_1420f9561;
            if ((void *)*plVar21 == pvVar4) break;
            (*DAT_143262828)(0);
          }
        }
        *(int *)(puVar18 + 5) = *(int *)(puVar18 + 5) + 1;
      }
      plVar17 = local_100;
      if (0xffffe < local_100[3] - 1U) {
        FUN_142e541f0(0x31e);
      }
      puVar16 = local_c0;
      LOCK();
      plVar17 = plVar17 + 3;
      lVar12 = *plVar17;
      *plVar17 = *plVar17 + -1;
      UNLOCK();
      if ((int)lVar12 == 1) {
        local_c0[6] = 0;
        plVar17 = local_100 + 2;
        if (plVar17 != (longlong *)0x0) {
          (**(code **)*plVar17)(plVar17,1);
        }
      }
      piVar1 = (int *)(puVar18 + 5);
      *piVar1 = *piVar1 + -1;
      if (*piVar1 == 0) {
        *plVar21 = 0;
      }
      if (puVar16 != (undefined8 *)0x0) {
        if (0xffffe < puVar16[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar21 = puVar16 + 1;
        lVar12 = *plVar21;
        *plVar21 = *plVar21 + -1;
        UNLOCK();
        if ((int)lVar12 == 1) {
          (**(code **)*local_c0)(local_c0,1);
        }
        local_c0 = (undefined8 *)0x0;
      }
    }
    plVar21 = (longlong *)plVar15[1];
    local_100 = plVar21;
    if (plVar21 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    uVar11 = (**(code **)(*plVar21 + 8))(plVar21);
    if (uVar11 == uVar10) goto LAB_1420fa0d1;
  }
  uVar19 = (ulonglong)local_res10[0];
  uVar7 = local_e4;
  uVar8 = local_e8;
LAB_1420f9645:
  uVar5 = local_f8;
  if ((int)local_f8 == 0x4ffa9e) {
    local_f8 = FUN_14019b780(&DAT_143ad68a0,0x2738);
    plVar15 = plVar23;
    if (local_f8 != 0) {
      plVar15 = (longlong *)FUN_1420ec330(local_f8,uVar19,local_res18,0x4ffa9e,uVar8,uVar7);
    }
    if (((longlong)plVar21 - 1U < 999) || (plVar21 == (longlong *)0xffffffffffffffff)) {
      FUN_142e52ed0(0x447,plVar21);
    }
    plVar22 = plVar15 + 2;
    if (plVar15 == (longlong *)0x0) {
      plVar22 = plVar23;
    }
    plVar15 = plVar23;
    if ((plVar22 != (longlong *)0x0) && (plVar15 = plVar22 + -2, plVar15 != (longlong *)0x0)) {
      if (0xfffff < (ulonglong)plVar22[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar22[1] = plVar22[1] + 1;
      UNLOCK();
      uVar19 = (ulonglong)local_res10[0];
      plVar21 = local_100;
    }
    local_100 = plVar15;
    if (plVar21 != (longlong *)0x0) {
      puVar18 = (undefined8 *)plVar21[7];
      if (puVar18 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar18[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar18[1] = puVar18[1] + 1;
        UNLOCK();
      }
      puVar16 = puVar18 + 4;
      FUN_1401d3510(puVar16,&local_ed);
      if (0xffffe < plVar21[3] - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar15 = plVar21 + 3;
      lVar12 = *plVar15;
      *plVar15 = *plVar15 + -1;
      UNLOCK();
      if ((int)lVar12 == 1) {
        puVar18[6] = 0;
        plVar21 = plVar21 + 2;
        if (plVar21 != (longlong *)0x0) {
          (**(code **)*plVar21)(plVar21,1);
        }
      }
      if (puVar16 != (undefined8 *)0x0) {
        piVar1 = (int *)(puVar18 + 5);
        *piVar1 = *piVar1 + -1;
        if (*piVar1 == 0) {
          *puVar16 = 0;
        }
      }
      if (puVar18 != (undefined8 *)0x0) {
        if (0xffffe < puVar18[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar21 = puVar18 + 1;
        lVar12 = *plVar21;
        *plVar21 = *plVar21 + -1;
        UNLOCK();
        if ((int)lVar12 == 1) {
          (**(code **)*puVar18)(puVar18,1);
        }
      }
      uVar19 = (ulonglong)local_res10[0];
    }
  }
  else if ((int)local_f8 == 0x17d78828) {
    local_f8 = FUN_14019b780(&DAT_143ad68a0,0x2738);
    plVar15 = plVar23;
    if (local_f8 != 0) {
      plVar15 = (longlong *)FUN_1420eaa70(local_f8,uVar19,local_res18,0x17d78828,uVar8,uVar7);
    }
    if (((longlong)plVar21 - 1U < 999) || (plVar21 == (longlong *)0xffffffffffffffff)) {
      FUN_142e52ed0(0x447,plVar21);
    }
    plVar22 = plVar15 + 2;
    if (plVar15 == (longlong *)0x0) {
      plVar22 = plVar23;
    }
    plVar15 = plVar23;
    if ((plVar22 != (longlong *)0x0) && (plVar15 = plVar22 + -2, plVar15 != (longlong *)0x0)) {
      if (0xfffff < (ulonglong)plVar22[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar22[1] = plVar22[1] + 1;
      UNLOCK();
      uVar19 = (ulonglong)local_res10[0];
      plVar21 = local_100;
    }
    local_100 = plVar15;
    if (plVar21 != (longlong *)0x0) {
      puVar18 = (undefined8 *)plVar21[7];
      if (puVar18 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar18[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar18[1] = puVar18[1] + 1;
        UNLOCK();
      }
      puVar16 = puVar18 + 4;
      FUN_1401d3510(puVar16,&local_ee);
      if (0xffffe < plVar21[3] - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar15 = plVar21 + 3;
      lVar12 = *plVar15;
      *plVar15 = *plVar15 + -1;
      UNLOCK();
      if ((int)lVar12 == 1) {
        puVar18[6] = 0;
        plVar21 = plVar21 + 2;
        if (plVar21 != (longlong *)0x0) {
          (**(code **)*plVar21)(plVar21,1);
        }
      }
      if (puVar16 != (undefined8 *)0x0) {
        piVar1 = (int *)(puVar18 + 5);
        *piVar1 = *piVar1 + -1;
        if (*piVar1 == 0) {
          *puVar16 = 0;
        }
      }
      if (puVar18 != (undefined8 *)0x0) {
        if (0xffffe < puVar18[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar21 = puVar18 + 1;
        lVar12 = *plVar21;
        *plVar21 = *plVar21 + -1;
        UNLOCK();
        if ((int)lVar12 == 1) {
          (**(code **)*puVar18)(puVar18,1);
        }
      }
      uVar19 = (ulonglong)local_res10[0];
    }
  }
  else if ((int)local_f8 == 0x17d78829) {
    local_f8 = FUN_14019b780(&DAT_143ad68a0,0x2748);
    plVar15 = plVar23;
    if (local_f8 != 0) {
      plVar15 = (longlong *)FUN_1420eb100(local_f8,uVar19,local_res18,0x17d78829,uVar8,uVar7);
    }
    if (((longlong)plVar21 - 1U < 999) || (plVar21 == (longlong *)0xffffffffffffffff)) {
      FUN_142e52ed0(0x447,plVar21);
    }
    plVar22 = plVar15 + 2;
    if (plVar15 == (longlong *)0x0) {
      plVar22 = plVar23;
    }
    plVar15 = plVar23;
    if ((plVar22 != (longlong *)0x0) && (plVar15 = plVar22 + -2, plVar15 != (longlong *)0x0)) {
      if (0xfffff < (ulonglong)plVar22[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar22[1] = plVar22[1] + 1;
      UNLOCK();
      uVar19 = (ulonglong)local_res10[0];
      plVar21 = local_100;
    }
    local_100 = plVar15;
    if (plVar21 != (longlong *)0x0) {
      puVar18 = (undefined8 *)plVar21[7];
      if (puVar18 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar18[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar18[1] = puVar18[1] + 1;
        UNLOCK();
      }
      puVar16 = puVar18 + 4;
      FUN_1401d3510(puVar16,&local_ef);
      if (0xffffe < plVar21[3] - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar15 = plVar21 + 3;
      lVar12 = *plVar15;
      *plVar15 = *plVar15 + -1;
      UNLOCK();
      if ((int)lVar12 == 1) {
        puVar18[6] = 0;
        plVar21 = plVar21 + 2;
        if (plVar21 != (longlong *)0x0) {
          (**(code **)*plVar21)(plVar21,1);
        }
      }
      if (puVar16 != (undefined8 *)0x0) {
        piVar1 = (int *)(puVar18 + 5);
        *piVar1 = *piVar1 + -1;
        if (*piVar1 == 0) {
          *puVar16 = 0;
        }
      }
      if (puVar18 != (undefined8 *)0x0) {
        if (0xffffe < puVar18[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar21 = puVar18 + 1;
        lVar12 = *plVar21;
        *plVar21 = *plVar21 + -1;
        UNLOCK();
        if ((int)lVar12 == 1) {
          (**(code **)*puVar18)(puVar18,1);
        }
      }
      uVar19 = (ulonglong)local_res10[0];
    }
  }
  else {
    local_f8 = FUN_14019b780(&DAT_143ad68a0,0x2728);
    plVar15 = plVar23;
    if (local_f8 != 0) {
      plVar15 = (longlong *)
                FUN_1420a9220(local_f8,uVar19,local_res18,uVar5 & 0xffffffff,uVar8,uVar7);
    }
    if (((longlong)plVar21 - 1U < 999) || (plVar21 == (longlong *)0xffffffffffffffff)) {
      FUN_142e52ed0(0x447,plVar21);
    }
    plVar22 = plVar15 + 2;
    if (plVar15 == (longlong *)0x0) {
      plVar22 = plVar23;
    }
    plVar15 = plVar23;
    if ((plVar22 != (longlong *)0x0) && (plVar15 = plVar22 + -2, plVar15 != (longlong *)0x0)) {
      if (0xfffff < (ulonglong)plVar22[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar22[1] = plVar22[1] + 1;
      UNLOCK();
      uVar19 = (ulonglong)local_res10[0];
      plVar21 = local_100;
    }
    local_100 = plVar15;
    if (plVar21 != (longlong *)0x0) {
      puVar18 = (undefined8 *)plVar21[7];
      if (puVar18 != (undefined8 *)0x0) {
        if (0xfffff < (ulonglong)puVar18[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar18[1] = puVar18[1] + 1;
        UNLOCK();
      }
      puVar16 = puVar18 + 4;
      FUN_1401d3510(puVar16,&local_f0);
      if (0xffffe < plVar21[3] - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar15 = plVar21 + 3;
      lVar12 = *plVar15;
      *plVar15 = *plVar15 + -1;
      UNLOCK();
      if ((int)lVar12 == 1) {
        puVar18[6] = 0;
        plVar21 = plVar21 + 2;
        if (plVar21 != (longlong *)0x0) {
          (**(code **)*plVar21)(plVar21,1);
        }
      }
      if (puVar16 != (undefined8 *)0x0) {
        piVar1 = (int *)(puVar18 + 5);
        *piVar1 = *piVar1 + -1;
        if (*piVar1 == 0) {
          *puVar16 = 0;
        }
      }
      if (puVar18 != (undefined8 *)0x0) {
        if (0xffffe < puVar18[1] - 1) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar21 = puVar18 + 1;
        lVar12 = *plVar21;
        *plVar21 = *plVar21 + -1;
        UNLOCK();
        if ((int)lVar12 == 1) {
          (**(code **)*puVar18)(puVar18,1);
        }
      }
      uVar19 = (ulonglong)local_res10[0];
    }
  }
  plVar21 = local_100;
  if (local_100 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
  }
  iVar9 = FUN_1420aee80(plVar21,local_res20);
  if (iVar9 != 0) {
    plVar15 = (longlong *)FUN_1429b6c90(DAT_143ac1b90);
    if (plVar15 != (longlong *)0x0) {
      local_88 = plVar21;
      if (plVar21 != (longlong *)0x0) {
        if (0xfffff < (ulonglong)plVar21[3]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        plVar21[3] = plVar21[3] + 1;
        UNLOCK();
        uVar19 = (ulonglong)local_res10[0];
        plVar21 = local_100;
      }
      FUN_142795c80(plVar15);
      iVar9 = (**(code **)(*plVar15 + 0x50))(plVar15);
      if (iVar9 != 0) {
        local_78 = plVar21;
        if (plVar21 != (longlong *)0x0) {
          if (0xfffff < (ulonglong)plVar21[3]) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          plVar21[3] = plVar21[3] + 1;
          UNLOCK();
          uVar19 = (ulonglong)local_res10[0];
          plVar21 = local_100;
        }
        FUN_1428a0e50(plVar15);
      }
    }
    if (plVar21 == (longlong *)0x0) {
      FUN_142e52ed0(0x431);
    }
    cVar6 = FUN_1407f9540(*(undefined4 *)((longlong)plVar21 + 0x30c));
    uVar10 = local_res18;
    if (cVar6 != '\0') {
      plVar22 = (longlong *)(param_1 + 0x20);
      puVar18 = (undefined8 *)*plVar22;
      puVar16 = (undefined8 *)puVar18[1];
      uStack_a0 = 0;
      cVar6 = *(char *)((longlong)puVar16 + 0x19);
      puVar20 = puVar18;
      local_a8 = puVar16;
      while (puVar3 = puVar16, cVar6 == '\0') {
        bVar24 = local_res18 <= *(uint *)((longlong)puVar3 + 0x1c);
        if (bVar24) {
          puVar16 = (undefined8 *)*puVar3;
          puVar20 = puVar3;
        }
        else {
          puVar16 = (undefined8 *)puVar3[2];
        }
        uStack_a0 = (uint)bVar24;
        cVar6 = *(char *)((longlong)puVar16 + 0x19);
        local_a8 = puVar3;
      }
      if ((*(char *)((longlong)puVar20 + 0x19) != '\0') ||
         (local_res18 < *(uint *)((longlong)puVar20 + 0x1c))) {
        if (*(longlong *)(param_1 + 0x28) == 0x7ffffffffffffff) {
                    /* WARNING: Subroutine does not return */
          FUN_14019f9d0();
        }
        local_60 = 0;
        local_68 = plVar22;
        plVar17 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x20);
        *(uint *)((longlong)plVar17 + 0x1c) = uVar10;
        *plVar17 = (longlong)puVar18;
        plVar17[1] = (longlong)puVar18;
        plVar17[2] = (longlong)puVar18;
        *(undefined2 *)(plVar17 + 3) = 0;
        local_58 = (undefined4)local_a8;
        uStack_54 = local_a8._4_4_;
        uStack_50 = uStack_a0;
        uStack_4c = uStack_9c;
        FUN_1404ace20(plVar22,&local_58,plVar17);
      }
      plVar22 = local_e0;
      local_d0 = (undefined8 *)0x0;
      if (*local_e0 != 0) {
        for (lVar12 = *(longlong *)(*local_e0 + (uVar19 % (ulonglong)*(uint *)(local_e0 + 1)) * 8);
            lVar12 != 0; lVar12 = *(longlong *)(lVar12 + 8)) {
          if (*(int *)(lVar12 + 0x10) == (int)uVar19) {
            FUN_1420fade0(local_d8,lVar12 + 0x18);
            puVar18 = local_d0;
            if (lVar12 + 0x18 != 0) goto LAB_1420f9ec8;
            if (local_d0 != (undefined8 *)0x0) {
              puVar16 = local_d0 + -5;
              iVar9 = FUN_14022eb80(puVar16);
              if (iVar9 == 0) {
                lVar12 = puVar18[-2];
                if (lVar12 != 0) {
                  LOCK();
                  *(undefined8 *)(lVar12 + 8) = 0;
                  UNLOCK();
                  do {
                    plVar21 = local_100;
                  } while (*(int *)(puVar18[-2] + 4) != 0);
                }
                if (puVar16 != (undefined8 *)0x0) {
                  (**(code **)*puVar16)(puVar16,1);
                }
              }
              local_d0 = (undefined8 *)0x0;
            }
            break;
          }
        }
      }
      lVar12 = DAT_143ad6c38;
      FUN_140309b50(&local_e0,DAT_143ad6c38 + 0x18,local_ec);
      puVar18 = *(undefined8 **)(lVar12 + 0x28);
      if (puVar18 == (undefined8 *)0x0) {
        puVar18 = (undefined8 *)FUN_14019d3c0(0x40,0x10);
        *(undefined8 **)(lVar12 + 0x28) = puVar18;
      }
      *(undefined8 *)(lVar12 + 0x28) = *puVar18;
      plVar17 = local_e0 + 1;
      *(int *)plVar17 = (int)*plVar17 + -1;
      if ((int)*plVar17 == 0) {
        *local_e0 = 0;
      }
      puVar18[5] = 0;
      puVar18[3] = 0;
      puVar18[1] = 0;
      puVar18[2] = 0;
      *puVar18 = &PTR_FUN_14342ec80;
      puVar18[4] = &PTR_LAB_14342ec88;
      *(undefined4 *)((longlong)puVar18 + 0x2c) = 0;
      puVar18[6] = 0;
      puVar18[7] = 0;
      if (puVar18[1] != 0) {
        FUN_142e541f0(0x2fe);
      }
      puVar18[1] = 1;
      local_d0 = puVar18 + 5;
      FUN_1420fb2b0(plVar22,local_res10,local_d8);
      puVar18 = puVar18 + 5;
LAB_1420f9ec8:
      if (puVar18 == (undefined8 *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      plVar22 = (longlong *)FUN_1420fb140(puVar18);
      if ((plVar22[1] - 1U < 999) || (plVar22[1] == -1)) {
        FUN_142e52ed0(0x447);
      }
      if (plVar22 == &local_108) {
        FUN_142e52d50(0x45c,1);
      }
      if (0xfffff < (ulonglong)plVar21[3]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar21[3] = plVar21[3] + 1;
      UNLOCK();
      FUN_1419ef260(plVar22);
      plVar21 = local_100;
      plVar22[1] = (longlong)local_100;
      if ((plVar15 != (longlong *)0x0) &&
         (iVar9 = (**(code **)(*plVar15 + 0x50))(plVar15), iVar9 != 0)) {
        uVar7 = (**(code **)(*plVar21 + 8))(plVar21);
        cVar6 = FUN_142859e60(plVar15,uVar7);
        if (cVar6 != '\0') {
          iVar9 = FUN_1401ba9d0(plVar21 + 0x7a,(int)plVar21[0x7b]);
          iVar9 = iVar9 >> 1;
          if (iVar9 == 1) {
            plVar23 = (longlong *)0x5;
          }
          else if (iVar9 != 2) {
            if (iVar9 == 3) {
              plVar23 = (longlong *)0x23;
            }
            else if (iVar9 == 6) {
              plVar23 = (longlong *)0x6;
            }
            else if (iVar9 == 7) {
              plVar23 = (longlong *)0x22;
            }
          }
          iVar9 = FUN_1401ba9d0((longlong)plVar21 + 0x3dc,*(undefined4 *)((longlong)plVar21 + 0x3e4)
                               );
          if (-1 < iVar9) {
            uVar10 = FUN_1401ba9d0((longlong)plVar21 + 0x3dc,
                                   *(undefined4 *)((longlong)plVar21 + 0x3e4));
            plVar23 = (longlong *)(ulonglong)uVar10;
          }
          FUN_1420c3620(plVar21,plVar23);
        }
      }
      puVar18 = local_d0;
      if (local_d0 != (undefined8 *)0x0) {
        puVar16 = local_d0 + -5;
        iVar9 = FUN_14022eb80(puVar16);
        if (iVar9 == 0) {
          lVar12 = puVar18[-2];
          if (lVar12 != 0) {
            LOCK();
            *(undefined8 *)(lVar12 + 8) = 0;
            UNLOCK();
            do {
            } while (*(int *)(puVar18[-2] + 4) != 0);
          }
          if (puVar16 != (undefined8 *)0x0) {
            (**(code **)*puVar16)(puVar16,1);
          }
        }
      }
    }
  }
LAB_1420fa0d1:
  lVar12 = local_b0;
  if (local_b0 != 0) {
    puVar18 = (undefined8 *)(local_b0 + -0x28);
    iVar9 = FUN_14022eb80(puVar18);
    if (iVar9 == 0) {
      lVar13 = *(longlong *)(lVar12 + -0x10);
      if (lVar13 != 0) {
        LOCK();
        *(undefined8 *)(lVar13 + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(lVar12 + -0x10) + 4) != 0);
      }
      if (puVar18 != (undefined8 *)0x0) {
        (**(code **)*puVar18)(puVar18,1);
      }
    }
  }
  FUN_1419ef260(&local_108);
  return;
}



//===========================================================
// FUN_1420fa150 @ 1420fa150   (813 bytes)
//===========================================================

void FUN_1420fa150(longlong param_1,uint param_2,uint param_3,undefined8 param_4)

{
  char cVar1;
  longlong lVar2;
  undefined8 *puVar3;
  longlong *plVar4;
  undefined1 auVar5 [16];
  undefined8 *puVar6;
  longlong *plVar7;
  int iVar8;
  uint uVar9;
  longlong *plVar10;
  undefined8 *puVar11;
  undefined8 uVar12;
  ulonglong uVar13;
  undefined8 *puVar14;
  longlong lVar15;
  longlong lVar16;
  undefined1 local_50 [8];
  longlong local_48;
  undefined1 local_40 [8];
  longlong *local_38;
  
  plVar10 = (longlong *)FUN_1429b6c90(DAT_143ac1b90);
  lVar15 = 0;
  iVar8 = 0;
  if (plVar10 != (longlong *)0x0) {
    iVar8 = FUN_142795fd0(plVar10,param_3,param_4);
  }
  local_48 = 0;
  if (*(longlong *)(param_1 + 8) != 0) {
    auVar5 = ZEXT416(param_2) % ZEXT416(*(uint *)(param_1 + 0x10));
    for (lVar2 = *(longlong *)(*(longlong *)(param_1 + 8) + auVar5._0_8_ * 8); lVar15 = 0,
        lVar2 != 0; lVar2 = *(longlong *)(lVar2 + 8)) {
      if (*(uint *)(lVar2 + 0x10) == param_2) {
        if (local_50 == (undefined1 *)(lVar2 + 0x18)) {
          FUN_142e52d50(0x45c,CONCAT71(auVar5._1_7_,1));
        }
        lVar15 = *(longlong *)(lVar2 + 0x20);
        if (lVar15 != 0) {
          if (0xfffff < *(ulonglong *)(lVar15 + -0x20)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(lVar15 + -0x20) = *(longlong *)(lVar15 + -0x20) + 1;
          UNLOCK();
        }
        lVar15 = *(longlong *)(lVar2 + 0x20);
        local_48 = lVar15;
        if (lVar2 != -0x18) {
          if (lVar15 == 0) {
            FUN_142e52ed0(0x431,0);
          }
          lVar2 = *(longlong *)(lVar15 + 8);
          while (lVar2 != 0) {
            uVar13 = *(ulonglong *)(lVar2 + -0x20);
            if ((uVar13 != 0) && (uVar13 < 0x10001)) {
              FUN_142e52ed0(0x33e);
              uVar13 = *(ulonglong *)(lVar2 + -0x20);
            }
            lVar16 = 0;
            if (uVar13 != 0) {
              lVar16 = uVar13 + 0x28;
            }
            plVar4 = *(longlong **)(lVar2 + 8);
            local_38 = plVar4;
            if (plVar4 != (longlong *)0x0) {
              if (0xfffff < (ulonglong)plVar4[3]) {
                FUN_142e541f0(0x30f);
              }
              LOCK();
              plVar4[3] = plVar4[3] + 1;
              UNLOCK();
              lVar15 = local_48;
            }
            plVar7 = local_38;
            if (plVar4 == (longlong *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            uVar9 = (**(code **)(*plVar7 + 8))(plVar7);
            if (uVar9 == param_3) {
              if (iVar8 == 0) {
                FUN_1420c9a80(plVar7,param_4);
              }
              FUN_1420fb540(lVar15,lVar2);
              FUN_1419ef260(local_40);
              lVar2 = lVar16;
            }
            else {
              FUN_1419ef260(local_40);
              lVar2 = lVar16;
            }
          }
        }
        break;
      }
    }
  }
  puVar3 = *(undefined8 **)(param_1 + 0x20);
  cVar1 = *(char *)((longlong)puVar3[1] + 0x19);
  puVar6 = puVar3;
  puVar11 = (undefined8 *)puVar3[1];
  while (cVar1 == '\0') {
    if (*(uint *)((longlong)puVar11 + 0x1c) < param_3) {
      puVar14 = (undefined8 *)puVar11[2];
      puVar11 = puVar6;
    }
    else {
      puVar14 = (undefined8 *)*puVar11;
    }
    puVar6 = puVar11;
    puVar11 = puVar14;
    cVar1 = *(char *)((longlong)puVar14 + 0x19);
  }
  if (((*(char *)((longlong)puVar6 + 0x19) == '\0') &&
      (*(uint *)((longlong)puVar6 + 0x1c) <= param_3)) && (puVar6 != puVar3)) {
    if (*(char *)((longlong)puVar6[2] + 0x19) == '\0') {
      plVar4 = *(longlong **)puVar6[2];
      cVar1 = *(char *)((longlong)plVar4 + 0x19);
      while (cVar1 == '\0') {
        plVar4 = (longlong *)*plVar4;
        cVar1 = *(char *)((longlong)plVar4 + 0x19);
      }
    }
    else {
      cVar1 = *(char *)((longlong)puVar6[1] + 0x19);
      puVar11 = (undefined8 *)puVar6[1];
      puVar3 = puVar6;
      while ((puVar14 = puVar11, cVar1 == '\0' && (puVar3 == (undefined8 *)puVar14[2]))) {
        cVar1 = *(char *)((longlong)puVar14[1] + 0x19);
        puVar11 = (undefined8 *)puVar14[1];
        puVar3 = puVar14;
      }
    }
    uVar12 = FUN_140dbbbc0(param_1 + 0x20,puVar6);
    thunk_FUN_140205820(uVar12,0x20);
    if ((plVar10 != (longlong *)0x0) &&
       (iVar8 = (**(code **)(*plVar10 + 0x50))(plVar10), iVar8 != 0)) {
      FUN_142859da0(plVar10,param_3);
    }
  }
  if (lVar15 != 0) {
    puVar3 = (undefined8 *)(lVar15 + -0x28);
    if (0xffffe < *(longlong *)(lVar15 + -0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar10 = (longlong *)(lVar15 + -0x20);
    lVar15 = *plVar10;
    *plVar10 = *plVar10 + -1;
    UNLOCK();
    if ((int)lVar15 == 1) {
      if (*(longlong *)(local_48 + -0x10) != 0) {
        LOCK();
        *(undefined8 *)(*(longlong *)(local_48 + -0x10) + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(local_48 + -0x10) + 4) != 0);
      }
      if (puVar3 != (undefined8 *)0x0) {
        (**(code **)*puVar3)(puVar3,1);
      }
    }
  }
  return;
}



//===========================================================
// FUN_142795fd0 @ 142795fd0   (634 bytes)
//===========================================================

undefined8 FUN_142795fd0(longlong *param_1,uint param_2,undefined8 param_3)

{
  longlong *plVar1;
  int *piVar2;
  longlong lVar3;
  undefined8 *puVar4;
  undefined1 auVar5 [12];
  undefined1 auVar6 [12];
  int iVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  ulonglong uVar10;
  longlong lVar11;
  undefined8 uVar12;
  undefined1 local_res8 [8];
  undefined1 local_48 [8];
  longlong local_40;
  undefined1 local_38 [8];
  longlong local_30;
  
  local_40 = 0;
  if (param_1[0x23c] != 0) {
    auVar5._4_8_ = 0;
    auVar5._0_4_ = *(uint *)(param_1 + 0x23d);
    auVar6._8_4_ = 0;
    auVar6._0_8_ = CONCAT44(0,param_2);
    for (lVar11 = *(longlong *)(param_1[0x23c] + SUB128(auVar6 % auVar5,0) * 8); lVar11 != 0;
        lVar11 = *(longlong *)(lVar11 + 8)) {
      if (*(uint *)(lVar11 + 0x10) == param_2) {
        if (local_48 == (undefined1 *)(lVar11 + 0x18)) {
          FUN_142e52d50(0x45c,CONCAT71(SUB127(auVar6 % auVar5,1),1));
        }
        lVar3 = *(longlong *)(lVar11 + 0x20);
        if (lVar3 != 0) {
          if (0xfffff < *(ulonglong *)(lVar3 + 0x18)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(lVar3 + 0x18) = *(longlong *)(lVar3 + 0x18) + 1;
          UNLOCK();
        }
        lVar11 = *(longlong *)(lVar11 + 0x20);
        local_40 = lVar11;
        if (lVar11 != 0) {
          FUN_1420c9a80(lVar11,param_3);
          FUN_1420fb540(param_1 + 0x23f);
          lVar3 = param_1[0x23c];
          if (lVar3 == 0) goto LAB_1427961f3;
          uVar10 = CONCAT44(0,param_2) % (ulonglong)*(uint *)(param_1 + 0x23d);
          puVar9 = *(undefined8 **)(lVar3 + uVar10 * 8);
          if (puVar9 == (undefined8 *)0x0) goto LAB_1427961f3;
          puVar4 = (undefined8 *)puVar9[1];
          if (*(uint *)(puVar9 + 2) == param_2) {
            *(undefined8 **)(lVar3 + uVar10 * 8) = puVar4;
            goto LAB_1427961e2;
          }
          puVar8 = puVar9;
          if (puVar4 != (undefined8 *)0x0) goto LAB_1427961c0;
          goto LAB_1427961f3;
        }
        break;
      }
    }
  }
  uVar12 = 0;
  lVar11 = local_40;
LAB_142796026:
  if (lVar11 != 0) {
    puVar9 = *(undefined8 **)(lVar11 + 0x38);
    if (puVar9 != (undefined8 *)0x0) {
      if (0xfffff < (ulonglong)puVar9[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      puVar9[1] = puVar9[1] + 1;
      UNLOCK();
      lVar11 = local_40;
    }
    puVar4 = puVar9 + 4;
    FUN_1401d3510(puVar4,local_res8);
    if (0xffffe < *(longlong *)(lVar11 + 0x18) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar11 + 0x18);
    lVar11 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar11 == 1) {
      puVar9[6] = 0;
      puVar8 = (undefined8 *)(local_40 + 0x10);
      if (puVar8 != (undefined8 *)0x0) {
        (**(code **)*puVar8)(puVar8,1);
      }
    }
    if (puVar4 != (undefined8 *)0x0) {
      piVar2 = (int *)(puVar9 + 5);
      *piVar2 = *piVar2 + -1;
      if (*piVar2 == 0) {
        *puVar4 = 0;
      }
    }
    if (puVar9 != (undefined8 *)0x0) {
      if (0xffffe < puVar9[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar1 = puVar9 + 1;
      lVar11 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if ((int)lVar11 == 1) {
        (**(code **)*puVar9)(puVar9,1);
      }
    }
  }
  return uVar12;
LAB_1427961c0:
  puVar9 = puVar4;
  puVar4 = (undefined8 *)puVar9[1];
  if (*(uint *)(puVar9 + 2) == param_2) {
    puVar8[1] = puVar4;
    if (puVar9 != (undefined8 *)0x0) {
LAB_1427961e2:
      (**(code **)*puVar9)(puVar9,1);
    }
    *(int *)((longlong)param_1 + 0x11ec) = *(int *)((longlong)param_1 + 0x11ec) + -1;
LAB_1427961f3:
    iVar7 = (**(code **)(*param_1 + 0x50))(param_1);
    lVar3 = DAT_143aa8518;
    if ((iVar7 != 0) && (DAT_143aa8518 != 0)) {
      local_30 = lVar11;
      if (0xfffff < *(ulonglong *)(lVar11 + 0x18)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar11 + 0x18) = *(longlong *)(lVar11 + 0x18) + 1;
      UNLOCK();
      FUN_1428a1040(lVar3,local_38);
      lVar11 = local_40;
    }
    uVar12 = 1;
    goto LAB_142796026;
  }
  puVar8 = puVar9;
  if (puVar4 == (undefined8 *)0x0) goto LAB_1427961f3;
  goto LAB_1427961c0;
}



//===========================================================
// FUN_142d012e0 @ 142d012e0   (569 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142d012e0(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  undefined4 uVar2;
  longlong lVar3;
  int *piVar4;
  undefined1 auStack_248 [32];
  int *local_228 [2];
  undefined **local_218;
  undefined8 local_210;
  undefined8 local_208;
  int *local_200;
  undefined1 local_1f8;
  undefined8 local_1f7;
  undefined8 local_1ef;
  undefined8 local_1e7;
  undefined8 local_1df;
  undefined8 uStack_1d7;
  undefined8 local_1cf;
  undefined8 uStack_1c7;
  undefined8 local_1bf;
  undefined8 uStack_1b7;
  undefined8 local_1af;
  undefined8 uStack_1a7;
  undefined8 local_19f;
  undefined8 uStack_197;
  undefined8 local_18f;
  undefined8 uStack_187;
  undefined8 local_17f;
  undefined8 uStack_177;
  undefined8 local_16f;
  undefined8 uStack_167;
  undefined8 local_df;
  undefined8 uStack_d7;
  undefined8 local_cf;
  undefined8 uStack_c7;
  undefined8 local_bf;
  undefined8 uStack_b7;
  undefined8 local_af;
  undefined8 uStack_a7;
  undefined8 local_9f;
  undefined8 uStack_97;
  undefined8 local_8f;
  undefined8 uStack_87;
  undefined8 local_7f;
  undefined8 uStack_77;
  undefined8 local_6f;
  undefined8 uStack_67;
  undefined4 local_5f;
  undefined8 local_5b;
  undefined4 local_53;
  undefined1 local_4f;
  undefined8 local_4e;
  undefined4 local_46;
  undefined8 local_42;
  undefined4 local_3a;
  undefined1 local_36;
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_248;
  uVar2 = FUN_1406e8c20(param_2);
  lVar3 = FUN_1429b6c90(DAT_143ac1b90,uVar2);
  if (lVar3 != 0) {
    local_200 = (int *)0x0;
    local_210 = 0;
    local_208 = 0;
    local_218 = &PTR_FUN_14327d968;
    local_4e = 0;
    local_1f8 = 0;
    local_1f7 = 0;
    local_1ef = 0;
    local_1e7 = 0;
    local_5b = 0;
    local_53 = 0;
    local_4f = 0;
    local_1df = 0;
    uStack_1d7 = 0;
    local_1cf = 0;
    uStack_1c7 = 0;
    local_1bf = 0;
    uStack_1b7 = 0;
    local_1af = 0;
    uStack_1a7 = 0;
    local_19f = 0;
    uStack_197 = 0;
    local_18f = 0;
    uStack_187 = 0;
    local_17f = 0;
    uStack_177 = 0;
    local_16f = 0;
    uStack_167 = 0;
    local_5f = 0;
    local_df = 0;
    uStack_d7 = 0;
    local_cf = 0;
    uStack_c7 = 0;
    local_bf = 0;
    uStack_b7 = 0;
    local_af = 0;
    uStack_a7 = 0;
    local_9f = 0;
    uStack_97 = 0;
    local_8f = 0;
    uStack_87 = 0;
    local_7f = 0;
    uStack_77 = 0;
    local_6f = 0;
    uStack_67 = 0;
    local_42 = 0;
    local_3a = 0;
    local_36 = 0;
    local_46 = 0xffffffff;
    local_228[0] = (int *)0x0;
    piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar4[1] = 0;
    *piVar4 = -1;
    local_228[0] = piVar4 + 4;
    piVar4[2] = 0;
    *(undefined1 *)local_228[0] = 0;
    if (*piVar4 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar4[1] < 0) {
      FUN_142e54290(0x90,piVar4[1],0);
    }
    *piVar4 = 1;
    *(undefined1 *)local_228[0] = 0;
    if (piVar4[1] + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    piVar4[2] = 0;
    FUN_1402ee8d0(&local_218,param_2,local_228,0);
    FUN_142797be0(lVar3,&local_218);
    local_218 = &PTR_FUN_143273970;
    if (local_200 != (int *)0x0) {
      LOCK();
      local_200[2] = 0;
      local_200[3] = 0;
      UNLOCK();
      do {
      } while (local_200[1] != 0);
      if (local_200 != (int *)0x0) {
        LOCK();
        iVar1 = *local_200;
        *local_200 = *local_200 + -1;
        UNLOCK();
        if (iVar1 == 1) {
          thunk_FUN_140205820(local_200,0x10);
        }
      }
    }
  }
  return;
}



//===========================================================
// FUN_140f80130 @ 140f80130   (5 bytes)
//===========================================================

longlong FUN_140f80130(longlong param_1)

{
  return param_1 + 0x30;
}



//===========================================================
// FUN_14118f5d0 @ 14118f5d0   (27 bytes)
//===========================================================

void FUN_14118f5d0(undefined8 param_1)

{
  FUN_142bf3f70();
  FUN_14118f040(param_1);
  return;
}



//===========================================================
// FUN_1429b9660 @ 1429b9660   (214 bytes)
//===========================================================

void FUN_1429b9660(longlong param_1,longlong *param_2)

{
  longlong lVar1;
  longlong lVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  ulonglong uVar5;
  
  lVar1 = *param_2;
  if ((lVar1 != 0) &&
     (puVar4 = *(undefined4 **)(param_1 + 0x20),
     *(undefined4 **)(param_1 + 0x20) != (undefined4 *)0x0)) {
    while( true ) {
      uVar5 = *(ulonglong *)(puVar4 + -8);
      if ((uVar5 != 0) && (uVar5 < 0x10001)) {
        FUN_142e52ed0();
        uVar5 = *(ulonglong *)(puVar4 + -8);
        lVar1 = *param_2;
      }
      puVar3 = (undefined4 *)0x0;
      if (uVar5 != 0) {
        puVar3 = (undefined4 *)(uVar5 + 0x28);
      }
      if ((*(longlong *)(puVar4 + 2) == lVar1) || (*(longlong *)(puVar4 + 4) == lVar1)) break;
      puVar4 = puVar3;
      if (puVar3 == (undefined4 *)0x0) {
        return;
      }
    }
    lVar1 = FUN_1429b6c90(param_1,*puVar4);
    lVar2 = FUN_1429b6c90(param_1,puVar4[1]);
    if ((lVar1 != 0) && (lVar2 != 0)) {
      FUN_1427b1d60(lVar1,0,lVar2,puVar4[6]);
    }
    FUN_1429bf430(param_1 + 0x18,puVar4);
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


