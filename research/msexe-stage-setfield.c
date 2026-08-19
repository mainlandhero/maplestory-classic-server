
//===========================================================
// FUN_142097f80 @ 142097f80   (11726 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142097f80(longlong param_1,undefined8 param_2)

{
  char *pcVar1;
  char cVar2;
  undefined1 uVar3;
  byte bVar4;
  byte bVar5;
  ushort uVar6;
  int iVar7;
  undefined4 uVar8;
  int iVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  uint uVar12;
  int iVar13;
  int *piVar14;
  undefined8 *puVar15;
  longlong lVar16;
  undefined8 uVar17;
  undefined1 *puVar18;
  undefined8 *puVar19;
  IUnknown *pIVar20;
  undefined8 uVar21;
  short *psVar22;
  IUnknown *pIVar23;
  longlong lVar24;
  undefined4 *puVar25;
  longlong lVar26;
  longlong *plVar27;
  ulonglong uVar28;
  ulonglong uVar29;
  IUnknown *pIVar30;
  ulonglong uVar31;
  uint uVar32;
  ulonglong uVar33;
  undefined2 *puVar34;
  byte *pbVar35;
  IUnknown *pIVar36;
  IUnknown *pIVar37;
  char *pcVar38;
  longlong *plVar39;
  undefined2 *puVar40;
  uint uVar41;
  IUnknown *pIVar42;
  longlong *plVar43;
  IUnknown *pIVar44;
  longlong *plVar45;
  IUnknown *pIVar46;
  undefined1 auStack_4a8 [32];
  int local_488;
  undefined4 local_480;
  longlong *local_478;
  undefined8 **local_470;
  undefined4 local_468;
  undefined4 local_460;
  undefined1 local_458;
  char local_457;
  char local_456;
  undefined8 local_450;
  uint local_448;
  IUnknown *local_440;
  IUnknown *local_438;
  undefined8 **local_430;
  char local_428;
  ulonglong local_420;
  int local_418;
  longlong *local_410;
  uint local_408;
  longlong local_400;
  longlong local_3f8;
  undefined8 local_3f0;
  undefined4 local_3e8;
  undefined8 local_3e0;
  IUnknown *local_3d8;
  undefined1 local_3d0 [8];
  ulonglong local_3c8;
  int *local_3c0;
  int *local_3b8;
  undefined8 local_3b0;
  ulonglong local_3a8;
  int local_3a0 [2];
  IUnknown *local_398;
  IUnknown *local_390;
  undefined8 *local_388;
  undefined8 local_380;
  undefined1 local_378 [8];
  ulonglong local_370;
  char local_368;
  undefined4 local_364;
  IUnknown *local_358;
  undefined4 local_350;
  undefined4 uStack_34c;
  undefined8 *local_348;
  int local_340;
  longlong local_338;
  undefined8 local_330;
  undefined8 local_328;
  undefined8 local_320;
  uint local_318;
  int local_310 [2];
  undefined4 local_308 [2];
  longlong local_300;
  IUnknown *pIStack_2f8;
  IUnknown *local_2f0;
  IUnknown *local_2e8;
  longlong *local_2e0;
  longlong local_2d8;
  longlong local_2d0;
  longlong local_2c8;
  longlong local_2c0;
  longlong local_2b8;
  longlong local_2b0;
  undefined1 local_2a8 [8];
  undefined1 local_2a0 [8];
  uint local_298;
  uint uStack_294;
  longlong local_290;
  longlong local_288;
  longlong local_280;
  longlong local_278;
  longlong local_270;
  longlong local_268;
  longlong local_260;
  longlong local_258;
  undefined8 local_250;
  undefined8 local_248;
  uint local_240;
  undefined4 uStack_23c;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined8 local_230;
  longlong local_228;
  char *local_220;
  char *local_218;
  char *local_210;
  undefined8 local_208;
  longlong local_200;
  undefined1 local_1f8 [8];
  undefined8 local_1f0;
  longlong local_1e8;
  longlong local_1e0;
  undefined8 local_1d8;
  undefined8 local_1d0;
  longlong local_1c8;
  longlong local_1c0;
  longlong local_1b8;
  undefined8 local_1b0;
  longlong local_1a8;
  undefined1 local_1a0 [16];
  IUnknown *local_190;
  IUnknown *local_180;
  undefined1 local_178 [8];
  longlong local_170;
  longlong local_168;
  uint local_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined8 local_150;
  short local_148 [4];
  longlong local_140;
  short local_130 [4];
  longlong local_128;
  undefined1 local_118 [8];
  longlong local_110;
  undefined8 *local_108;
  undefined8 local_100;
  undefined8 *local_f8;
  undefined8 local_f0;
  undefined1 local_e8 [8];
  longlong local_e0;
  ushort local_d8;
  ushort local_d6;
  ushort local_d2;
  ushort local_d0;
  ushort local_ce;
  ushort local_c8;
  ushort local_c6;
  ushort local_c2;
  ushort local_c0;
  ushort local_be;
  undefined1 local_b8 [112];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  local_3f8 = param_1;
  local_3e0 = param_2;
  FUN_1406e9170(param_2,&local_1f0,8);
  FUN_1408f67d0(local_1f0);
  lVar24 = DAT_143aa84a0;
  local_400 = DAT_143aa84a0;
  if (DAT_143aa84a0 == 0) {
    return;
  }
  cVar2 = FUN_142cfb500(DAT_143aa84a0);
  if (cVar2 != '\0') {
    return;
  }
  iVar7 = FUN_142c50c50();
  if (iVar7 == 2) {
    FUN_142c4f190(DAT_143ac1898);
  }
  FUN_142c50b90(3);
  FUN_142c4f660(DAT_143ac1898,10000);
  FUN_142c4f690(DAT_143ac1898,10000);
  FUN_142d01da0(lVar24,1000,1000,1);
  iVar7 = FUN_142cb9260(lVar24);
  uVar8 = FUN_1406e8c20(param_2);
  FUN_142cb91e0(lVar24,uVar8);
  uVar3 = FUN_1406e8ae0(param_2);
  FUN_142cb9210(lVar24,uVar3);
  uVar8 = FUN_1406e8c20(param_2);
  FUN_142cd87b0(lVar24,uVar8);
  iVar9 = FUN_142cb9260(lVar24);
  if (iVar7 != iVar9) {
    local_308[0] = FUN_142cb9260(lVar24);
    FUN_1408b0600(&local_260,"Ch\ra n\tn\re\nl",&DAT_14337b3c8);
    FUN_1420a1d40(&local_268,&local_260,local_308);
    FUN_142e0f240(local_268);
    if (local_268 != 0) {
      FUN_14019f2c0(local_268 + -0x10);
    }
    if (local_260 != 0) {
      FUN_14019f2c0(local_260 + -0x10);
    }
  }
  FUN_142d98060(lVar24,9);
  local_457 = FUN_1406e8ae0(param_2);
  if (local_457 == '\x01') {
    FUN_142d16ef0(lVar24);
  }
  FUN_1406e8c20(param_2);
  local_418 = FUN_1406e8c20(param_2);
  uVar8 = FUN_1406e8c20(param_2);
  local_438 = (IUnknown *)CONCAT44(local_438._4_4_,uVar8);
  bVar4 = FUN_1406e8ae0(param_2);
  local_408 = (uint)bVar4;
  uVar6 = FUN_1406e8b80(param_2);
  local_3f0 = CONCAT44(local_3f0._4_4_,(uint)uVar6);
  piVar14 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
  plVar27 = (longlong *)0x0;
  piVar14[1] = 0;
  *piVar14 = -1;
  local_3b8 = piVar14 + 4;
  piVar14[2] = 0;
  *(undefined1 *)local_3b8 = 0;
  if (*piVar14 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar14[1] < 0) {
    FUN_142e54290(0x90,piVar14[1],0);
  }
  *piVar14 = 1;
  *(undefined1 *)local_3b8 = 0;
  if (piVar14[1] + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  piVar14[2] = 0;
  local_410 = (longlong *)0x0;
  plVar43 = plVar27;
  if (uVar6 != 0) {
    puVar15 = (undefined8 *)FUN_1406e9050(param_2,&local_1a8);
    FUN_14019f2c0(piVar14);
    local_3b8 = (int *)*puVar15;
    *puVar15 = 0;
    if (local_1a8 != 0) {
      FUN_14019f2c0(local_1a8 + -0x10);
    }
    uVar33 = local_3f0 & 0xffffffff;
    if ((int)local_3f0 != 0) {
      do {
        local_3a8 = uVar33;
        uVar31 = local_3a8;
        FUN_1406e9050(param_2,&local_258);
        uVar41 = 0;
        if (plVar43 == (longlong *)0x0) {
LAB_1420982a3:
          uVar33 = 1;
          if (plVar43 == (longlong *)0x0) {
            uVar33 = 1;
            plVar45 = plVar27;
          }
          else {
LAB_1420982ba:
            uVar28 = plVar43[-2];
            uVar29 = ~uVar28;
            if (-1 < (longlong)uVar28) {
              uVar29 = uVar28;
            }
            if ((int)(uVar29 - 8 >> 3) == (int)uVar33) goto LAB_142098339;
            if (plVar43 == (longlong *)0x0) {
              plVar45 = (longlong *)0x0;
            }
            else {
              plVar45 = (longlong *)(ulonglong)*(uint *)(plVar43 + -1);
            }
          }
          lVar16 = FUN_14019b780(&DAT_143ad68a0,uVar33 * 8 + 8);
          plVar39 = (longlong *)(lVar16 + 8);
          if (lVar16 == 0) {
            plVar39 = plVar27;
          }
          if (plVar43 != (longlong *)0x0) {
            FUN_142ef7ba0(plVar39,plVar43,(longlong)plVar45 << 3);
            thunk_FUN_140205820(plVar43 + -1,0);
          }
          plVar39[-1] = (longlong)plVar45;
          uVar31 = local_3a8;
          plVar43 = plVar39;
          local_410 = plVar39;
        }
        else {
          uVar41 = *(uint *)(plVar43 + -1);
          uVar33 = plVar43[-2];
          uVar28 = ~uVar33;
          if (-1 < (longlong)uVar33) {
            uVar28 = uVar33;
          }
          if ((uint)(uVar28 - 8 >> 3) <= uVar41) {
            if (uVar41 == 0) goto LAB_1420982a3;
            uVar33 = (ulonglong)(uVar41 * 2);
            goto LAB_1420982ba;
          }
        }
LAB_142098339:
        plVar43[-1] = plVar43[-1] + 1;
        plVar43[(int)uVar41] = 0;
        FUN_14019a260(plVar43 + (int)uVar41,&local_258);
        if (local_258 != 0) {
          FUN_14019f2c0(local_258 + -0x10);
        }
        uVar33 = uVar31 - 1;
      } while (uVar31 - 1 != 0);
      local_3a8 = 0;
    }
  }
  local_3c8 = 0;
  local_450 = (char *)((ulonglong)local_450._4_4_ << 0x20);
  local_456 = '\0';
  local_448 = FUN_142d01d50(lVar24);
  if (local_408 == 0) {
    cVar2 = FUN_1406e8ae0(param_2);
    if (((DAT_143aa8518 != (IUnknown *)0x0) &&
        (iVar7 = FUN_140f810b0(DAT_143aa8518 + 0x100), iVar7 != 0)) || (cVar2 != '\0')) {
      FUN_142d62e30(lVar24);
      FUN_142cdeca0(lVar24,0);
    }
    puVar18 = (undefined1 *)FUN_142cbe6e0(lVar24,local_e8);
    if (local_3d0 == puVar18) {
      FUN_142e52d50(0x45c,1);
    }
    lVar16 = *(longlong *)(puVar18 + 8);
    if (lVar16 != 0) {
      if (0xfffff < *(ulonglong *)(lVar16 + -0x20)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar16 + -0x20) = *(longlong *)(lVar16 + -0x20) + 1;
      UNLOCK();
      plVar43 = local_410;
    }
    lVar16 = local_e0;
    local_420 = *(ulonglong *)(puVar18 + 8);
    local_3c8 = local_420;
    if (local_e0 != 0) {
      puVar15 = (undefined8 *)(local_e0 + -0x28);
      if (0xffffe < *(longlong *)(local_e0 + -0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar43 = (longlong *)(lVar16 + -0x20);
      lVar16 = *plVar43;
      *plVar43 = *plVar43 + -1;
      UNLOCK();
      plVar43 = local_410;
      if ((int)lVar16 == 1) {
        if ((local_e0 != 0) && (*(longlong *)(local_e0 + -0x10) != 0)) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_e0 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_e0 + -0x10) + 4) != 0);
        }
        if (puVar15 != (undefined8 *)0x0) {
          (**(code **)*puVar15)(puVar15,1);
          plVar43 = local_410;
        }
      }
    }
    uVar33 = local_3c8;
    local_420 = local_3c8;
    if (local_3c8 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    piVar14 = (int *)(uVar33 + 0xf3);
    uVar8 = FUN_1402fa540(piVar14);
    FUN_142d01d40(lVar24,uVar8);
    local_3e8 = FUN_1406e8c20(param_2);
    if (uVar33 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    iVar7 = *piVar14 + 1;
    *piVar14 = iVar7;
    if (iVar7 == (iVar7 / 0x6f) * 0x6f) {
      puVar15 = *(undefined8 **)(uVar33 + 0xfb);
      puVar19 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      *(undefined8 **)(uVar33 + 0xfb) = puVar19;
      *puVar19 = *puVar15;
      *(undefined4 *)(puVar19 + 1) = *(undefined4 *)(puVar15 + 1);
      thunk_FUN_140205820(puVar15,0xc);
    }
    uVar3 = FUN_142f04924();
    uVar17 = local_3e0;
    *(undefined1 *)(*(longlong *)(uVar33 + 0xfb) + 4) = uVar3;
    lVar24 = *(longlong *)(uVar33 + 0xfb);
    bVar4 = *(byte *)(lVar24 + 4);
    *(undefined2 *)(lVar24 + 8) = 0x9a65;
    pbVar35 = (byte *)(lVar24 + 2);
    do {
      if (bVar4 == 0) {
        bVar4 = 0x2a;
      }
      bVar5 = pbVar35[(longlong)&local_3f0 + (6 - lVar24)];
      pbVar35[-2] = bVar4 ^ bVar5;
      bVar4 = bVar4 + (bVar4 ^ bVar5) + 0x2a;
      uVar6 = *(ushort *)(*(longlong *)(uVar33 + 0xfb) + 8);
      *(ushort *)(*(longlong *)(uVar33 + 0xfb) + 8) = (uVar6 >> 0xd) + (ushort)bVar4 | uVar6 << 3;
      bVar5 = 0x2a;
      if (bVar4 != 0) {
        bVar5 = bVar4;
      }
      bVar4 = pbVar35[(longlong)&local_3f0 + -lVar24 + 7];
      pbVar35[-1] = bVar5 ^ bVar4;
      bVar5 = (bVar5 ^ bVar4) + bVar5 + 0x2a;
      uVar6 = *(ushort *)(*(longlong *)(uVar33 + 0xfb) + 8);
      *(ushort *)(*(longlong *)(uVar33 + 0xfb) + 8) = (uVar6 >> 0xd) + (ushort)bVar5 | uVar6 << 3;
      bVar4 = 0x2a;
      if (bVar5 != 0) {
        bVar4 = bVar5;
      }
      bVar5 = pbVar35[(longlong)&local_3e8 - lVar24];
      *pbVar35 = bVar4 ^ bVar5;
      bVar5 = (bVar4 ^ bVar5) + bVar4 + 0x2a;
      uVar6 = *(ushort *)(*(longlong *)(uVar33 + 0xfb) + 8);
      *(ushort *)(*(longlong *)(uVar33 + 0xfb) + 8) = (uVar6 >> 0xd) + (ushort)bVar5 | uVar6 << 3;
      bVar4 = 0x2a;
      if (bVar5 != 0) {
        bVar4 = bVar5;
      }
      bVar5 = pbVar35[(longlong)&local_3e8 + -lVar24 + 1];
      pbVar35[1] = bVar4 ^ bVar5;
      bVar4 = (bVar4 ^ bVar5) + bVar4 + 0x2a;
      uVar6 = *(ushort *)(*(longlong *)(uVar33 + 0xfb) + 8);
      *(ushort *)(*(longlong *)(uVar33 + 0xfb) + 8) = (uVar6 >> 0xd) + (ushort)bVar4 | uVar6 << 3;
      uVar41 = (int)plVar27 + 4;
      plVar27 = (longlong *)(ulonglong)uVar41;
      pbVar35 = pbVar35 + 4;
    } while (uVar41 < 4);
    uVar3 = FUN_1406e8ae0(local_3e0);
    lVar24 = local_400;
    uVar33 = local_420;
    if (local_420 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    *(undefined1 *)(uVar33 + 0x10b) = uVar3;
    uVar41 = FUN_1406e8c20(uVar17);
    uVar12 = FUN_1407386b0(&DAT_143ac1ab0);
    *(uint *)(uVar33 + 0x5b) = uVar12;
    uVar41 = (uVar12 ^ uVar41) >> 5 | (uVar12 ^ uVar41) << 0x1b;
    *(uint *)(uVar33 + 0x5f) = uVar41;
    *(uint *)(uVar33 + 99) = ((uVar12 ^ 0xbaadf00d) >> 5 | (uVar12 ^ 0xbaadf00d) << 0x1b) + uVar41;
  }
  else {
    uVar8 = FUN_1406e8c20();
    uVar10 = FUN_1406e8c20(param_2);
    uVar11 = FUN_1406e8c20(local_3e0);
    uVar17 = FUN_142cbef90(lVar24);
    FUN_14025ea90(uVar17,uVar8,uVar10,uVar11);
    FUN_142ce5170(lVar24,uVar8,uVar10,uVar11);
    uVar33 = FUN_1420a3080(0);
    local_420 = uVar33;
    local_3c8 = uVar33;
    if (uVar33 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    uVar17 = local_3e0;
    local_488 = 0;
    FUN_140304b20(uVar33,local_b8,local_3e0,0);
    cVar2 = FUN_1406e8ae0(uVar17);
    if (cVar2 != '\0') {
      cVar2 = FUN_1406e8ae0(uVar17);
      if (cVar2 == '\0') {
        local_450 = (char *)((ulonglong)local_450 & 0xffffffff00000000);
      }
      else {
        uVar8 = FUN_1406e8c20(uVar17);
        local_450 = (char *)CONCAT44(local_450._4_4_,uVar8);
      }
      FUN_1406e9170(uVar17,&local_320,8);
      uVar3 = FUN_1406e8ae0(uVar17);
      local_250 = DAT_14342b7f0;
      FUN_1406e9170(uVar17,&local_250,8);
      local_248 = DAT_14342b7f0;
      FUN_1406e9170(uVar17,&local_248,8);
      uVar8 = FUN_1406e8c20(uVar17);
      if (DAT_143aa84a0 != 0) {
        FUN_142d0e420(DAT_143aa84a0,(ulonglong)local_450 & 0xffffffff);
        local_456 = FUN_142d0e4d0(DAT_143aa84a0,local_320);
        FUN_142d0e570(DAT_143aa84a0,uVar3,local_250);
        FUN_142d1d050(DAT_143aa84a0,local_248,uVar8);
      }
      FUN_142cc0160(lVar24);
    }
    if (DAT_143ac92c8 != 0) {
      FUN_140fd5630();
    }
  }
  pIVar42 = (IUnknown *)0x0;
  if (uVar33 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  piVar14 = (int *)(uVar33 + 0xf3);
  local_440 = (IUnknown *)piVar14;
  iVar7 = FUN_1402fa540(piVar14);
  if (local_448 != iVar7) {
    if (uVar33 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    uVar8 = FUN_1402fa540(piVar14);
    FUN_1403999e0(DAT_143aa8328,&local_2c8,uVar8,PTR_s_mapName_143a49020);
    FUN_1408b0600(&local_2d0,"\tM\rA\nP",&DAT_14337b3c8);
    FUN_1420a1ff0(&local_2d8,&local_2d0,&local_2c8);
    FUN_142e0f240(local_2d8);
    if (local_2d8 != 0) {
      FUN_14019f2c0(local_2d8 + -0x10);
    }
    if (local_2d0 != 0) {
      FUN_14019f2c0(local_2d0 + -0x10);
    }
    if (local_2c8 != 0) {
      FUN_14019f2c0(local_2c8 + -0x10);
    }
  }
  cVar2 = FUN_1406e8ae0(uVar17);
  if (cVar2 == '\0') {
    FUN_142da05c0(lVar24,&local_350);
  }
  else {
    local_350 = FUN_1406e8c20(uVar17);
    uStack_34c = FUN_1406e8c20(uVar17);
    FUN_142da05a0(lVar24,CONCAT44(uStack_34c,local_350));
  }
  local_3c0 = (int *)0x0;
  piVar14 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
  iVar7 = 0;
  piVar14[1] = 0;
  *piVar14 = -1;
  local_3c0 = piVar14 + 4;
  piVar14[2] = 0;
  *(undefined1 *)local_3c0 = 0;
  if (*piVar14 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar14[1] < 0) {
    FUN_142e54290(0x90,piVar14[1],0);
  }
  *piVar14 = 1;
  *(undefined1 *)local_3c0 = 0;
  if (piVar14[1] + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  piVar14[2] = 0;
  local_470 = (undefined8 **)CONCAT71(local_470._1_7_,local_408 != 0);
  local_478 = (longlong *)CONCAT44(local_478._4_4_,1);
  local_480 = 1;
  local_488 = 0;
  FUN_142d97e30(lVar24,0x18,&local_3c0);
  bVar4 = FUN_1406e8ae0(uVar17);
  if ((DAT_143ac7fdc == 1) && ((bVar4 >> 1 & 1) == 0)) {
    DAT_143ac7fdc = 2;
  }
  FUN_142cc3d90(lVar24,bVar4 & 1);
  cVar2 = FUN_1406e8ae0(uVar17);
  lVar16 = local_3f8;
  if ((cVar2 != '\0') || (*(char *)(local_3f8 + 0x49) != '\0')) {
    *(undefined1 *)(local_3f8 + 0x49) = 0;
    if (*(char *)(DAT_143ac3568 + 0x54) == '\0') {
      cVar2 = FUN_140e604f0(0x5dc);
    }
    else {
      *(undefined1 *)(DAT_143ac3568 + 0x54) = 0;
      cVar2 = FUN_14128f480(DAT_143ac3568);
    }
    if (cVar2 != '\0') {
      *(undefined1 *)(lVar16 + 0x48) = 0;
    }
  }
  if (uVar33 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  uVar8 = FUN_1402fa540(local_440);
  local_340 = FUN_141817d10(uVar8);
  if (local_340 != 0) {
    FUN_1415aafa0(7);
  }
  if (DAT_143ad4848 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  iVar9 = (**(code **)(*(longlong *)(DAT_143ad4848 + 8) + 0xd0))
                    ((longlong *)(DAT_143ad4848 + 8),&PTR_PTR_143a88588);
  if (iVar9 == 0) {
    local_430 = (undefined8 **)FUN_14019b780(&DAT_143ad68a0,0x68);
    pIVar20 = pIVar42;
    if (local_430 != (undefined8 **)0x0) {
      pIVar20 = (IUnknown *)FUN_141a3ca80(local_430);
    }
    FUN_14209ee50(pIVar20,0);
  }
  local_370 = 0;
  local_358 = (IUnknown *)0x0;
  uVar31 = 0;
  if (uVar33 != 0) {
    if (0xfffff < *(ulonglong *)(uVar33 - 0x20)) {
      FUN_142e541f0(0x30f);
    }
    uVar31 = local_370;
    LOCK();
    *(longlong *)(uVar33 - 0x20) = *(longlong *)(uVar33 - 0x20) + 1;
    UNLOCK();
    if (local_370 != 0) {
      puVar15 = (undefined8 *)(local_370 + -0x28);
      if (0xffffe < *(longlong *)(local_370 + -0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar27 = (longlong *)(uVar31 + -0x20);
      lVar16 = *plVar27;
      *plVar27 = *plVar27 + -1;
      UNLOCK();
      if ((int)lVar16 == 1) {
        if ((local_370 != 0) && (*(longlong *)(local_370 + -0x10) != 0)) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_370 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_370 + -0x10) + 4) != 0);
        }
        if (puVar15 != (undefined8 *)0x0) {
          (**(code **)*puVar15)(puVar15,1);
        }
      }
    }
    local_420 = local_3c8;
    uVar31 = local_3c8;
    plVar43 = local_410;
  }
  local_368 = local_457;
  local_370 = uVar31;
  local_364 = FUN_1406e8c20(uVar17);
  cVar2 = FUN_1406e8ae0(uVar17);
  pIVar20 = local_358;
  if (cVar2 == '\0') {
    if (local_358 != (IUnknown *)0x0) {
      if (0xffffe < *(longlong *)(local_358 + 8) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      pIVar20 = pIVar20 + 8;
      lVar16 = *(longlong *)pIVar20;
      *(longlong *)pIVar20 = *(longlong *)pIVar20 + -1;
      UNLOCK();
      if (((int)lVar16 == 1) && (local_358 != (IUnknown *)0x0)) {
        (*(code *)**(undefined8 **)local_358)(local_358,1);
      }
      local_358 = (IUnknown *)0x0;
      local_420 = local_3c8;
      uVar31 = local_3c8;
      plVar43 = local_410;
    }
  }
  else {
    if (local_358 != (IUnknown *)0x0) {
      if (0xffffe < *(longlong *)(local_358 + 8) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      pIVar20 = pIVar20 + 8;
      lVar16 = *(longlong *)pIVar20;
      *(longlong *)pIVar20 = *(longlong *)pIVar20 + -1;
      UNLOCK();
      if (((int)lVar16 == 1) && (local_358 != (IUnknown *)0x0)) {
        (*(code *)**(undefined8 **)local_358)(local_358,1);
      }
      local_358 = (IUnknown *)0x0;
      local_420 = local_3c8;
      uVar31 = local_3c8;
      plVar43 = local_410;
    }
    local_430 = (undefined8 **)FUN_14019b780(&DAT_143ad68a0,0x38);
    pIVar20 = pIVar42;
    if (local_430 != (undefined8 **)0x0) {
      pIVar20 = (IUnknown *)FUN_1418c5750(local_430);
    }
    if (*(longlong *)(pIVar20 + 8) != 0) {
      FUN_142e541f0(0x2fe);
    }
    *(longlong *)(pIVar20 + 8) = 1;
    local_358 = pIVar20;
    FUN_1418c5820(pIVar20,uVar17);
  }
  bVar4 = FUN_1406e8ae0(uVar17);
  local_318 = (uint)bVar4;
  local_3a0[0] = 0;
  if (uVar31 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  pIVar20 = local_440;
  iVar9 = FUN_1402fa540(local_440);
  if (iVar9 != 0) {
    if (uVar31 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    iVar9 = FUN_1402fa540(pIVar20);
    if (iVar9 != 999999999) {
      if (uVar31 == 0) {
        FUN_142e52ed0(0x431,0);
      }
      uVar8 = FUN_1402fa540(pIVar20);
      iVar9 = FUN_142d065d0(lVar24,uVar8,local_3a0);
      FUN_142d06570(lVar24,iVar9 != 0);
    }
  }
  if (uVar31 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  uVar8 = FUN_1402fa540(pIVar20);
  lVar16 = FUN_141816220(local_118,uVar8);
  FUN_14209ee50(*(undefined8 *)(lVar16 + 8),local_378);
  lVar16 = local_110;
  if (local_110 != 0) {
    if (0xffffe < *(longlong *)(local_110 + 0x28) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar27 = (longlong *)(lVar16 + 0x28);
    lVar16 = *plVar27;
    *plVar27 = *plVar27 + -1;
    UNLOCK();
    if ((int)lVar16 == 1) {
      pIVar20 = (IUnknown *)(local_110 + 0x20);
      if (local_110 == 0) {
        pIVar20 = pIVar42;
      }
      if (pIVar20 != (IUnknown *)0x0) {
        (*(code *)**(undefined8 **)pIVar20)(pIVar20,1);
      }
    }
    local_420 = local_3c8;
    plVar43 = local_410;
  }
  FUN_142d06570(lVar24,0);
  FUN_142cc4430(lVar24,0);
  if (local_3a0[0] != 0) {
    local_2c0 = 0;
    uVar21 = FUN_1401c21c0(&local_2c0,L"FIELD%d");
    local_2b8 = 0;
    FUN_1401c1fb0(&local_2b8,uVar21);
    if (local_2c0 != 0) {
      FUN_1401bebb0(local_2c0 + -0x10);
    }
    pIVar20 = DAT_143add058;
    local_3d8 = DAT_143add058;
    if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
    lVar16 = local_2b8;
    local_430 = (undefined8 **)puVar15;
    if (puVar15 == (undefined8 *)0x0) {
      local_348 = (undefined8 *)0x0;
    }
    else {
      puVar15[1] = 0;
      *(undefined4 *)(puVar15 + 2) = 1;
      local_348 = puVar15;
      if (local_2b8 == 0) {
        *puVar15 = 0;
      }
      else {
        lVar26 = -1;
        do {
          lVar26 = lVar26 + 1;
        } while (*(short *)(local_2b8 + lVar26 * 2) != 0);
        local_3b0 = (ulonglong)((int)lVar26 + 1);
        piVar14 = (int *)(*DAT_143ad5980)(local_3b0 * 2 + 4);
        if (piVar14 == (int *)0x0) {
          *puVar15 = 0;
LAB_14209ad22:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(0x8007000e);
        }
        *piVar14 = (int)lVar26 * 2;
        local_440 = (IUnknown *)(piVar14 + 1);
        FUN_142ef7ba0(local_440,lVar16,local_3b0 * 2);
        *puVar15 = local_440;
        pIVar20 = local_3d8;
        if (local_440 == (IUnknown *)0x0) goto LAB_14209ad22;
      }
    }
    if (local_348 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x8007000e);
    }
    local_430 = &local_348;
    iVar9 = (**(code **)(*(longlong *)pIVar20 + 0xa0))(pIVar20,*local_348);
    if (iVar9 < 0) {
      _com_issue_errorex(iVar9,pIVar20,(_GUID *)&DAT_1432743e8);
    }
    FUN_1401be120(&local_348);
    if (lVar16 != 0) {
      FUN_1401bebb0(lVar16 + -0x10);
    }
  }
  *(undefined1 *)(local_3f8 + 0x48) = 1;
  lVar16 = FUN_141892840();
  if (lVar16 != 0) {
    uVar21 = FUN_141892840();
    cVar2 = FUN_141830050(uVar21);
    if ((cVar2 != '\0') && (bVar4 = FUN_1406e8ae0(uVar17), bVar4 != 0)) {
      uVar33 = (ulonglong)bVar4;
      do {
        FUN_1406e9050(uVar17,&local_2b0);
        uVar21 = FUN_141892840();
        FUN_141b7ec60(uVar21,local_2b0,0xffffffff);
        if (local_2b0 != 0) {
          FUN_14019f2c0(local_2b0 + -0x10);
        }
        uVar33 = uVar33 - 1;
      } while (uVar33 != 0);
    }
  }
  pIVar20 = DAT_143aa8518;
  local_440 = DAT_143aa8518;
  FUN_1427bacc0(DAT_143aa8518,0);
  FUN_142843830(pIVar20);
  FUN_142903440(pIVar20);
  lVar16 = FUN_141892840();
  if (lVar16 != 0) {
    uVar21 = FUN_141892840();
    iVar9 = FUN_14182efa0(uVar21);
    uVar21 = DAT_143abfdf0;
    uVar8 = DAT_143370270;
    if (iVar9 != 0) {
      uVar17 = FUN_1409397b0(local_440 + 8,local_1a0);
      local_480 = uVar8;
      local_488 = iVar9;
      FUN_140e0b300(uVar21,uVar17,0,0);
      pIVar20 = local_440;
      uVar17 = local_3e0;
    }
    uVar21 = FUN_141892840();
    FUN_14187e880(uVar21,local_418,(ulonglong)local_438 & 0xffffffff);
  }
  FUN_142834df0(pIVar20,uVar17);
  FUN_142835840(pIVar20,uVar17);
  FUN_1428358a0(pIVar20,uVar17);
  cVar2 = FUN_1406e8ae0(uVar17);
  if (cVar2 != '\0') {
    uVar8 = FUN_1406e8c20(uVar17);
    FUN_142d96f80(lVar24,uVar8);
    uVar8 = FUN_1406e8c20(uVar17);
    FUN_142d96f90(lVar24,uVar8);
    uVar8 = FUN_1406e8c20(uVar17);
    FUN_142d96fa0(lVar24,uVar8);
    uVar8 = FUN_1406e8c20(uVar17);
    FUN_142d96fb0(lVar24,uVar8);
    uVar8 = FUN_1406e8c20(uVar17);
    FUN_142d96fc0(lVar24,uVar8);
    uVar21 = FUN_1406e9050(uVar17,local_178);
    FUN_142d96fd0(lVar24,uVar21);
  }
  FUN_1427bdd40(pIVar20,uVar17);
  bVar4 = FUN_1406e8ae0(uVar17);
  local_448 = (uint)bVar4;
  local_3d8 = (IUnknown *)0x0;
  local_398 = (IUnknown *)0x0;
  local_390 = (IUnknown *)0x0;
  local_3b0 = 0;
  pIVar37 = pIVar42;
  pIVar44 = pIVar42;
  pIVar46 = pIVar42;
  if (local_448 != 0) {
    puVar15 = (undefined8 *)FUN_1406e9050(uVar17,&local_168);
    pIVar37 = (IUnknown *)*puVar15;
    *puVar15 = 0;
    local_3d8 = pIVar37;
    if (local_168 != 0) {
      FUN_14019f2c0(local_168 + -0x10);
    }
    puVar15 = (undefined8 *)FUN_1406e9050(uVar17,&local_170);
    pIVar46 = (IUnknown *)*puVar15;
    *puVar15 = 0;
    local_398 = pIVar46;
    if (local_170 != 0) {
      FUN_14019f2c0(local_170 + -0x10);
    }
    puVar15 = (undefined8 *)FUN_1406e9050(uVar17,&local_228);
    pIVar44 = (IUnknown *)*puVar15;
    *puVar15 = 0;
    local_390 = pIVar44;
    if (local_228 != 0) {
      FUN_14019f2c0(local_228 + -0x10);
    }
    uVar8 = FUN_1406e8c20(uVar17);
    local_3b0 = CONCAT44(local_3b0._4_4_,uVar8);
    uVar8 = FUN_1406e8c20(uVar17);
    local_3b0 = CONCAT44(uVar8,(undefined4)local_3b0);
  }
  pIVar36 = pIVar20 + 0x53c8;
  lVar24 = *(longlong *)pIVar36;
  FUN_1401ba380(pIVar36,pIVar36,*(undefined8 *)(lVar24 + 8));
  *(longlong *)(lVar24 + 8) = lVar24;
  *(longlong *)lVar24 = lVar24;
  *(longlong *)(lVar24 + 0x10) = lVar24;
  *(longlong *)(pIVar20 + 0x53d0) = 0;
  FUN_1420a1b20(local_3e0,pIVar36);
  local_458 = 1;
  iVar9 = FUN_141892a90();
  if (iVar9 + 0xec549980U < 1000000) {
    uVar8 = FUN_141892a90();
    FUN_141815360(&local_2e8,uVar8);
    pIVar20 = local_2e8;
    if (local_2e8 != (IUnknown *)0x0) {
      FUN_1401bb8d0(&local_388,L"onUserEnter");
      local_430 = &local_388;
      (*DAT_143262a20)(&local_160);
      pIVar36 = pIVar42;
      if (local_388 != (undefined8 *)0x0) {
        pIVar36 = (IUnknown *)*local_388;
      }
      iVar9 = (**(code **)(*(longlong *)pIVar20 + 0x28))(pIVar20,pIVar36,&local_160);
      if (iVar9 < 0) {
        _com_issue_errorex(iVar9,pIVar20,(_GUID *)&DAT_143272478);
      }
      local_240 = local_160;
      uStack_23c = uStack_15c;
      uStack_238 = uStack_158;
      uStack_234 = uStack_154;
      local_230 = local_150;
      local_160 = local_160 & 0xffff0000;
      FUN_1401be120(&local_388);
      puVar40 = &DAT_143278568;
      puVar34 = &DAT_143278568;
      if ((short)local_240 == 8) {
        puVar34 = (undefined2 *)CONCAT44(uStack_234,uStack_238);
      }
      FUN_14022d860(&local_210,puVar34,0xffffffff);
      if ((short)local_240 == 8) {
        local_240 = local_240 & 0xffff0000;
        if (CONCAT44(uStack_234,uStack_238) != 0) {
          (*DAT_143ad5990)(CONCAT44(uStack_234,uStack_238) + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_240);
      }
      local_438 = local_2e8;
      if (local_2e8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      FUN_1401bb8d0(local_2a8,L"onFirstUserEnter");
      psVar22 = (short *)FUN_1401e4330(local_438,local_148,local_2a8);
      if (*psVar22 == 8) {
        puVar34 = *(undefined2 **)(psVar22 + 4);
      }
      else {
        puVar34 = &DAT_143278568;
      }
      FUN_14022d860(&local_220,puVar34,0xffffffff);
      if (local_148[0] == 8) {
        local_148[0] = 0;
        if (local_140 != 0) {
          (*DAT_143ad5990)(local_140 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_148);
      }
      local_438 = local_2e8;
      if (local_2e8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      FUN_1401bb8d0(local_2a0,L"fieldScript");
      psVar22 = (short *)FUN_1401e4330(local_438,local_130,local_2a0);
      if (*psVar22 == 8) {
        puVar40 = *(undefined2 **)(psVar22 + 4);
      }
      FUN_14022d860(&local_218,puVar40,0xffffffff);
      if (local_130[0] == 8) {
        local_130[0] = 0;
        if (local_128 != 0) {
          (*DAT_143ad5990)(local_128 + -4);
        }
      }
      else {
        (*DAT_143262a18)(local_130);
      }
      if ((((local_210 == (char *)0x0) || (*local_210 == '\0')) &&
          ((local_220 == (char *)0x0 || (*local_220 == '\0')))) &&
         ((local_218 == (char *)0x0 || (*local_218 == '\0')))) {
        local_458 = 1;
      }
      else {
        local_458 = 0;
      }
      if (local_218 != (char *)0x0) {
        FUN_14019f2c0(local_218 + -0x10);
      }
      if (local_220 != (char *)0x0) {
        FUN_14019f2c0(local_220 + -0x10);
      }
      if (local_210 != (char *)0x0) {
        FUN_14019f2c0(local_210 + -0x10);
      }
    }
    if (local_2e0 != (longlong *)0x0) {
      (**(code **)(*local_2e0 + 0x10))();
    }
    if (local_2e8 != (IUnknown *)0x0) {
      (**(code **)(*(longlong *)local_2e8 + 0x10))(local_2e8);
    }
  }
  lVar24 = local_400;
  uVar33 = local_420;
  if (local_408 != 0) {
    FUN_142ce4700(local_400);
    uVar33 = local_420;
    if (local_420 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    puVar15 = *(undefined8 **)(uVar33 + 0x1111);
    if ((puVar15 != (undefined8 *)0x0) &&
       (puVar19 = puVar15 + *(uint *)(uVar33 + 0x1119), puVar15 < puVar19)) {
      do {
        local_438 = (IUnknown *)*puVar15;
        if (local_438 != (IUnknown *)0x0) goto LAB_142099690;
        puVar15 = puVar15 + 1;
      } while (puVar15 < puVar19);
      local_438 = (IUnknown *)0x0;
    }
    goto LAB_14209971f;
  }
LAB_14209973a:
  FUN_142936cf0(local_440,1);
  iVar9 = (**(code **)(*(longlong *)(local_3f8 + 8) + 0xd0))
                    ((longlong *)(local_3f8 + 8),&PTR_PTR_143a886e8);
  if (iVar9 == 0) {
    iVar7 = (**(code **)(*(longlong *)(local_3f8 + 8) + 0xd0))
                      ((longlong *)(local_3f8 + 8),&PTR_PTR_143a87ec8);
    lVar16 = local_400;
    if (iVar7 != 0) {
      iVar7 = FUN_141829f70(local_3f8);
      if ((iVar7 == 0) && (lVar16 = FUN_141892840(), lVar16 != 0)) {
        if (DAT_143ad4848 == 0) {
          FUN_142e52ed0(0x431,0);
        }
        iVar7 = (**(code **)(*(longlong *)(DAT_143ad4848 + 8) + 0xd0))
                          ((longlong *)(DAT_143ad4848 + 8),&PTR_PTR_143a87ec8);
        if (iVar7 != 0) {
          uVar17 = FUN_141892840();
          iVar7 = FUN_141829f70(uVar17);
          if (iVar7 == 10000) {
            if ((((local_448 != 0) && (pIVar37 != (IUnknown *)0x0)) && (*pIVar37 != (IUnknown)0x0))
               && (cVar2 = FUN_1422bc140(local_458,uVar33), cVar2 != '\0')) {
              FUN_1420a1a40(&local_3d8,&local_398,&local_390,&local_3b0);
            }
            local_440 = DAT_143ac87a0;
            local_448 = FUN_1408fd350();
            local_456 = FUN_142e5fdd0(uVar33);
            if (((local_440 != (IUnknown *)0x0) &&
                (uVar41 = FUN_1415f64c0(local_440), uVar41 != local_448)) && (local_456 != '\0')) {
              local_488 = 0;
              FUN_142cb1020(lVar24,0x489,0xffffffffffffffff);
              FUN_1415f6130(local_440,local_448);
            }
          }
        }
      }
      iVar7 = FUN_142cc4050(lVar24);
      lVar16 = local_400;
      if (iVar7 != 0) {
        local_3f8 = FUN_1408f6690();
        (*DAT_1432625b0)(&local_3f8,&local_c8);
        iVar7 = ((uint)local_c8 * 100 + (uint)local_c6) * 100 + (uint)local_c2;
        if (((iVar7 < 0x132ddd6) ||
            (((iVar7 == 0x132ddd6 && ((uint)local_c0 * 100 + (uint)local_be < 0x597)) &&
             (DAT_143ad4858 == 0)))) &&
           (local_430 = (undefined8 **)FUN_14019b780(&DAT_143ad68a0,0x248),
           local_430 != (undefined8 **)0x0)) {
          FUN_1422b6910(local_430);
        }
        iVar7 = FUN_142cc1cf0(lVar24);
        if ((((iVar7 != 0) || (iVar7 = FUN_142cc1e30(lVar24), iVar7 != 0)) ||
            (cVar2 = FUN_142cf1c00(lVar24), cVar2 == '\0')) &&
           ((DAT_143ad4858 != 0 && (FUN_142bf3f70(), DAT_143ad4858 != 0)))) {
          (*(code *)**(undefined8 **)(DAT_143ad4858 + 8))((undefined8 *)(DAT_143ad4858 + 8),1);
        }
        FUN_142cc4030(lVar24);
        lVar16 = local_400;
      }
    }
    goto LAB_14209a9de;
  }
  cVar2 = FUN_142cf1c20(lVar24);
  if ((cVar2 == '\0') && (iVar9 = FUN_142cc1cf0(lVar24), iVar9 == 0)) {
    FUN_142cc1e30(lVar24);
  }
  cVar2 = FUN_142cf1c20(lVar24);
  if ((cVar2 == '\0') && (iVar9 = FUN_142cc1cf0(lVar24), iVar9 == 0)) {
    iVar9 = FUN_142cc1e30(lVar24);
    local_428 = '\x01';
    if (iVar9 != 0) goto LAB_1420997ae;
  }
  else {
LAB_1420997ae:
    local_428 = '\0';
  }
  if (uVar33 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  if (DAT_143ac9d60 == 0) {
LAB_1420997dd:
    local_457 = '\0';
  }
  else {
    cVar2 = FUN_141098c80();
    local_457 = '\x01';
    if (cVar2 == '\0') goto LAB_1420997dd;
  }
  iVar9 = FUN_1401ba9d0(uVar33 + 0x27,*(undefined4 *)(uVar33 + 0x2f));
  if (iVar9 == 1) {
    local_310[0] = *(int *)(uVar33 + 0xab);
    uVar41 = *(uint *)(uVar33 + 0xa3);
    local_298 = (uVar41 << 5 | uVar41 >> 0x1b) ^ *(uint *)(uVar33 + 0x9b);
    uVar32 = *(uint *)(uVar33 + 0x9b) ^ 0xbaadf00d;
    uVar12 = *(uint *)(uVar33 + 0xa7);
    uStack_294 = (uVar12 << 5 | uVar12 >> 0x1b) ^ *(uint *)(uVar33 + 0x9f);
    uVar41 = (uVar32 >> 5 | uVar32 << 0x1b) + uVar41 ^ *(uint *)(uVar33 + 0x9f);
    iVar9 = (uVar41 >> 5 | uVar41 << 0x1b) + uVar12;
    local_3e0 = CONCAT44(local_3e0._4_4_,iVar9);
    if (iVar9 != local_310[0]) {
      local_208 = FUN_1418039d0(5);
      puVar15 = (undefined8 *)FUN_1401a0ed0(&local_200,&local_208,&local_3e0,local_310);
      FUN_141804970(&DAT_143271f04,0x53,5,*puVar15);
      if (local_200 != 0) {
        FUN_14019f2c0(local_200 + -0x10);
      }
    }
    if (((CONCAT44(uStack_294,local_298) == 0) && (local_428 != '\0')) && (local_457 == '\0')) {
      local_430 = (undefined8 **)FUN_14019b780(&DAT_143ad68a0,2000);
      pIVar20 = pIVar42;
      if (local_430 != (undefined8 **)0x0) {
        pIVar20 = (IUnknown *)FUN_142a57d30(local_430,0,0,0);
      }
      pIVar37 = pIVar20 + 0x18;
      if (pIVar20 == (IUnknown *)0x0) {
        pIVar37 = pIVar42;
      }
      if (pIVar37 == (IUnknown *)0x0) {
        local_190 = (IUnknown *)0x0;
      }
      else {
        local_190 = pIVar37 + -0x18;
        if (local_190 != (IUnknown *)0x0) {
          if (0xfffff < *(ulonglong *)(pIVar37 + 8)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(pIVar37 + 8) = *(longlong *)(pIVar37 + 8) + 1;
          UNLOCK();
        }
      }
      pIVar20 = local_190;
      if (local_190 == (IUnknown *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      local_430 = &local_108;
      local_100 = 0;
      local_440 = (IUnknown *)&local_290;
      local_290 = 0;
      uVar17 = FUN_140196ed0(local_1f8,PTR_s__fUI_UtilDlgEx_img_UtilDlgEx_not_143a44be0,0xffffffff);
      local_460 = 0;
      local_468 = 0;
      local_470 = &local_108;
      local_478 = &local_290;
      local_480 = 0;
      local_488 = 0;
      FUN_142a61900(pIVar20,0,0x897b50,uVar17);
      if (pIVar20 == (IUnknown *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_142a62bf0(pIVar20,0,0);
      if (pIVar20 == (IUnknown *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_142a5ee30(pIVar20);
      if (pIVar20 == (IUnknown *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      (**(code **)(*(longlong *)pIVar20 + 0x130))(pIVar20);
      if (0xffffe < *(longlong *)(pIVar20 + 0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      pIVar20 = pIVar20 + 0x20;
      lVar16 = *(longlong *)pIVar20;
      *(longlong *)pIVar20 = *(longlong *)pIVar20 + -1;
      UNLOCK();
      pIVar37 = local_3d8;
      uVar33 = local_3c8;
      plVar43 = local_410;
      pIVar44 = local_390;
      pIVar46 = local_398;
      if (((int)lVar16 == 1) && (pIVar20 = local_190 + 0x18, pIVar20 != (IUnknown *)0x0)) {
        (*(code *)**(undefined8 **)pIVar20)(pIVar20,1);
        pIVar37 = local_3d8;
        uVar33 = local_3c8;
        plVar43 = local_410;
        pIVar44 = local_390;
        pIVar46 = local_398;
      }
    }
  }
  lVar16 = FUN_141892840();
  if (lVar16 != 0) {
    if (DAT_143ad4848 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    iVar9 = (**(code **)(*(longlong *)(DAT_143ad4848 + 8) + 0xd0))
                      ((longlong *)(DAT_143ad4848 + 8),&PTR_PTR_143a87ec8);
    if (iVar9 != 0) {
      uVar17 = FUN_141892840();
      iVar9 = FUN_141829f70(uVar17);
      if (((iVar9 != 0) && (iVar9 = FUN_142cc1cf0(lVar24), iVar9 == 0)) &&
         ((iVar9 = FUN_142cc1e30(lVar24), iVar9 == 0 &&
          ((cVar2 = FUN_142cf1c00(lVar24), cVar2 != '\0' && (*(int *)(DAT_143ac87a0 + 0x274) != 0)))
          ))) {
        local_300 = 0;
        pIStack_2f8 = (IUnknown *)0x0;
        local_440 = (IUnknown *)0x0;
        local_2f0 = (IUnknown *)0x0;
        pIVar20 = *(IUnknown **)(DAT_143ac87a0 + 0x230);
        pIVar36 = *(IUnknown **)(DAT_143ac87a0 + 0x228);
        pIVar30 = (IUnknown *)0x0;
        pIVar23 = pIVar42;
        local_438 = pIVar20;
        if (pIVar36 != pIVar20) {
          do {
            if (*(int *)pIVar36 != 0x780) {
              if (pIVar30 == pIVar23) {
                FUN_1420a22a0(&local_300,pIVar30,pIVar36);
                local_440 = local_2f0;
                pIVar23 = local_2f0;
                pIVar30 = pIStack_2f8;
                pIVar20 = local_438;
              }
              else {
                *(longlong *)pIVar30 = *(longlong *)pIVar36;
                pIStack_2f8 = pIVar30 + 8;
                pIVar23 = local_440;
                pIVar30 = pIStack_2f8;
              }
            }
            pIVar36 = pIVar36 + 8;
          } while (pIVar36 != pIVar20);
        }
        lVar24 = local_300;
        local_430 = (undefined8 **)((longlong)pIVar30 - local_300 >> 3);
        pIVar20 = local_440;
        if ((undefined8 **)0x1 < local_430) {
          local_418 = *(int *)(pIVar30 + -8);
          local_420 = CONCAT44(local_420._4_4_,*(undefined4 *)(pIVar30 + -4));
          iVar9 = FUN_1410a3d70();
          uVar41 = 0;
          if (iVar9 == 2) {
            uVar41 = FUN_142c4eaa0(DAT_143ac1898);
          }
          local_438 = (IUnknown *)CONCAT44(local_438._4_4_,uVar41);
          if (DAT_143add050 == 0) {
LAB_14209ad14:
                    /* WARNING: Subroutine does not return */
            FUN_142ef3ac0(0x80004003);
          }
          iVar9 = FUN_1420a2f30(DAT_143add050,uVar41,local_418,local_420 & 0xffffffff);
          if (iVar9 == 1) {
            local_418 = *(int *)(lVar24 + -0x10 + (longlong)local_430 * 8);
            uVar8 = *(undefined4 *)(lVar24 + -0xc + (longlong)local_430 * 8);
            local_420 = CONCAT44(local_420._4_4_,uVar8);
            if (DAT_143add050 == 0) goto LAB_14209ad14;
            iVar9 = FUN_1420a2f30(DAT_143add050,(ulonglong)local_438 & 0xffffffff,local_418,uVar8);
          }
          pIVar20 = local_440;
          if ((iVar9 == 0) &&
             (cVar2 = FUN_1410a3fe0(local_418,local_420 & 0xffffffff), pIVar20 = local_440,
             cVar2 == '\0')) {
            local_338 = 0;
            puVar15 = (undefined8 *)FUN_1408a9e40(&local_1e8,0x546);
            iVar9 = local_418;
            uVar33 = local_420;
            FUN_14019ba10(&local_338,*puVar15,local_418,local_420 & 0xffffffff);
            if (local_1e8 != 0) {
              FUN_14019f2c0(local_1e8 + -0x10);
            }
            local_430 = (undefined8 **)FUN_14019b780(&DAT_143ad68a0,2000);
            pIVar20 = pIVar42;
            if (local_430 != (undefined8 **)0x0) {
              pIVar20 = (IUnknown *)FUN_142a57d30(local_430,0,0,0);
            }
            pIVar37 = pIVar20 + 0x18;
            if (pIVar20 == (IUnknown *)0x0) {
              pIVar37 = pIVar42;
            }
            if (pIVar37 == (IUnknown *)0x0) {
              local_180 = (IUnknown *)0x0;
            }
            else {
              local_180 = pIVar37 + -0x18;
              if (local_180 != (IUnknown *)0x0) {
                if (0xfffff < *(ulonglong *)(pIVar37 + 8)) {
                  FUN_142e541f0(0x30f);
                }
                LOCK();
                *(longlong *)(pIVar37 + 8) = *(longlong *)(pIVar37 + 8) + 1;
                UNLOCK();
              }
            }
            pIVar42 = local_180;
            if (local_180 == (IUnknown *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            FUN_142a62ba0(pIVar42,1);
            if (pIVar42 == (IUnknown *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            local_430 = &local_f8;
            local_f0 = 0;
            local_440 = (IUnknown *)&local_288;
            local_288 = 0;
            local_380 = 0;
            FUN_14019a260(&local_380,&local_338);
            local_460 = 0;
            local_468 = 0;
            local_470 = &local_f8;
            local_478 = &local_288;
            local_480 = 1;
            local_488 = 0;
            FUN_142a61900(pIVar42,1,0x897b50,&local_380);
            if (pIVar42 == (IUnknown *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            FUN_142a62cb0(pIVar42);
            if (pIVar42 == (IUnknown *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            FUN_142a5ee30(pIVar42);
            if (pIVar42 == (IUnknown *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            iVar13 = (**(code **)(*(longlong *)pIVar42 + 0x130))(pIVar42);
            *(undefined4 *)(DAT_143ac87a0 + 0x270) = 5;
            if (iVar13 == 7) {
              FUN_1415f3f30(DAT_143ac87a0);
              FUN_1415ffe70(DAT_143ac87a0);
            }
            else {
              FUN_1410a4d20(iVar9,uVar33 & 0xffffffff,0);
              FUN_142da3670(DAT_143aa84a0);
            }
            if (0xffffe < *(longlong *)(pIVar42 + 0x20) - 1U) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            pIVar42 = pIVar42 + 0x20;
            lVar24 = *(longlong *)pIVar42;
            *(longlong *)pIVar42 = *(longlong *)pIVar42 + -1;
            UNLOCK();
            if (((int)lVar24 == 1) && (pIVar42 = local_180 + 0x18, pIVar42 != (IUnknown *)0x0)) {
              (*(code *)**(undefined8 **)pIVar42)(pIVar42,1);
            }
            lVar24 = local_300;
            pIVar37 = local_3d8;
            uVar33 = local_3c8;
            pIVar20 = local_2f0;
            plVar43 = local_410;
            pIVar44 = local_390;
            pIVar46 = local_398;
            if (local_338 != 0) {
              FUN_14019f2c0(local_338 + -0x10);
              lVar24 = local_300;
              pIVar37 = local_3d8;
              uVar33 = local_3c8;
              pIVar20 = local_2f0;
              plVar43 = local_410;
              pIVar44 = local_390;
              pIVar46 = local_398;
            }
          }
        }
        *(undefined4 *)(DAT_143ac87a0 + 0x274) = 0;
        if (lVar24 != 0) {
          FUN_1420a3390(&local_300,lVar24,(longlong)pIVar20 - lVar24 >> 3);
        }
      }
    }
  }
  if ((int)local_450 != 0) {
    iVar13 = (int)local_450 / 0x3c;
    local_280 = 0;
    puVar15 = (undefined8 *)FUN_1408a9e40(&local_1e0,0x926);
    iVar9 = iVar13 + 1;
    if ((int)local_450 + iVar13 * -0x3c < 1) {
      iVar9 = iVar13;
    }
    uVar17 = FUN_14019ba10(&local_280,*puVar15,iVar9);
    local_330 = 0;
    FUN_14019a260(&local_330,uVar17);
    local_460 = 0;
    local_468 = 0;
    local_470 = (undefined8 **)((ulonglong)local_470 & 0xffffffff00000000);
    local_478 = (longlong *)((ulonglong)local_478 & 0xffffffff00000000);
    local_480 = 0;
    local_488 = 0;
    FUN_142a26280(&local_330,0,0,1);
    if (local_1e0 != 0) {
      FUN_14019f2c0(local_1e0 + -0x10);
    }
    if (local_280 != 0) {
      FUN_14019f2c0(local_280 + -0x10);
    }
  }
  if (local_456 != '\0') {
    local_1d8 = FUN_1408f6690();
    local_1d0 = FUN_1408f63b0(&local_1d8,0x16d);
    cVar2 = FUN_1408fc990(&local_1d0,&local_320);
    if (cVar2 == '\0') {
      local_270 = 0;
      puVar15 = (undefined8 *)FUN_1408f7bb0(&local_1b8,&local_320,0x18);
      uVar17 = *puVar15;
      puVar15 = (undefined8 *)FUN_1408a9e40(&local_1c0,0x927);
      uVar17 = FUN_14019ba10(&local_270,*puVar15,uVar17);
      local_3a8 = 0;
      FUN_14019a260(&local_3a8,uVar17);
      local_460 = 0;
      local_468 = 0;
      local_470 = (undefined8 **)((ulonglong)local_470 & 0xffffffff00000000);
      local_478 = (longlong *)((ulonglong)local_478 & 0xffffffff00000000);
      local_480 = 0;
      local_488 = 0;
      FUN_142a26280(&local_3a8,0,0,1);
      if (local_1c0 != 0) {
        FUN_14019f2c0(local_1c0 + -0x10);
      }
      lVar24 = local_270;
      if (local_1b8 != 0) {
        FUN_14019f2c0(local_1b8 + -0x10);
        lVar24 = local_270;
      }
    }
    else {
      local_278 = 0;
      puVar15 = (undefined8 *)FUN_1408a9e40(&local_1c8,0x928);
      uVar17 = FUN_14019ba10(&local_278,*puVar15);
      local_328 = 0;
      FUN_14019a260(&local_328,uVar17);
      local_460 = 0;
      local_468 = 0;
      local_470 = (undefined8 **)((ulonglong)local_470 & 0xffffffff00000000);
      local_478 = (longlong *)((ulonglong)local_478 & 0xffffffff00000000);
      local_480 = 0;
      local_488 = 0;
      FUN_142a26280(&local_328,0,0,1);
      lVar24 = local_278;
      if (local_1c8 != 0) {
        FUN_14019f2c0(local_1c8 + -0x10);
        lVar24 = local_278;
      }
    }
    if (lVar24 != 0) {
      FUN_14019f2c0(lVar24 + -0x10);
    }
  }
  lVar24 = FUN_141892840();
  if (lVar24 != 0) {
    if (DAT_143ad4848 == 0) {
      FUN_142e52ed0(0x431,0);
    }
    iVar9 = (**(code **)(*(longlong *)(DAT_143ad4848 + 8) + 0xd0))
                      ((longlong *)(DAT_143ad4848 + 8),&PTR_PTR_143a87ec8);
    if (iVar9 != 0) {
      uVar17 = FUN_141892840();
      iVar9 = FUN_141829f70(uVar17);
      if (iVar9 != 0) {
        uVar17 = FUN_141892840();
        iVar9 = FUN_141829fd0(uVar17);
        if ((((iVar9 != 0xda) && (local_448 != 0)) && (pIVar37 != (IUnknown *)0x0)) &&
           ((*pIVar37 != (IUnknown)0x0 && (cVar2 = FUN_1422bc140(local_458,uVar33), cVar2 != '\0')))
           ) {
          FUN_1420a1a40(&local_3d8,&local_398,&local_390,&local_3b0);
        }
      }
    }
  }
  lVar16 = local_400;
  iVar9 = FUN_142cb8ad0(local_400);
  if ((iVar9 != 0) && (DAT_143aa84a0 != 0)) {
    FUN_142cb8b80();
  }
  iVar9 = FUN_142cc4050(lVar16);
  if (iVar9 != 0) {
    local_1b0 = FUN_1408f6690();
    (*DAT_1432625b0)(&local_1b0,&local_d8);
    iVar9 = ((uint)local_d8 * 100 + (uint)local_d6) * 100 + (uint)local_d2;
    if (((iVar9 < 0x132ddd6) ||
        (((iVar9 == 0x132ddd6 && ((uint)local_d0 * 100 + (uint)local_ce < 0x597)) &&
         (DAT_143ad4858 == 0)))) &&
       (local_430 = (undefined8 **)FUN_14019b780(&DAT_143ad68a0,0x248),
       local_430 != (undefined8 **)0x0)) {
      FUN_1422b6910(local_430);
    }
    iVar9 = FUN_142cc1cf0(lVar16);
    if ((((iVar9 != 0) || (iVar9 = FUN_142cc1e30(lVar16), iVar9 != 0)) ||
        (cVar2 = FUN_142cf1c00(lVar16), cVar2 == '\0')) &&
       ((DAT_143ad4858 != 0 && (FUN_142bf3f70(), DAT_143ad4858 != 0)))) {
      (*(code *)**(undefined8 **)(DAT_143ad4858 + 8))((undefined8 *)(DAT_143ad4858 + 8),1);
    }
    FUN_142cc4030(lVar16);
  }
  if ((int)local_3f0 != 0) {
    local_450 = (char *)0x0;
    FUN_14019bd40(&local_450,0,0);
    pcVar38 = local_450;
    if (*(int *)(local_450 + -0x10) != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (*(int *)(pcVar38 + -0xc) < 0) {
      FUN_142e54290(0x90,*(int *)(pcVar38 + -0xc),0);
    }
    pcVar38[-0x10] = '\x01';
    pcVar38[-0xf] = '\0';
    pcVar38[-0xe] = '\0';
    pcVar38[-0xd] = '\0';
    *pcVar38 = '\0';
    if (*(int *)(pcVar38 + -0xc) + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    pcVar38[-8] = '\0';
    pcVar38[-7] = '\0';
    pcVar38[-6] = '\0';
    pcVar38[-5] = '\0';
    if (local_3b8 != (int *)0x0) {
      uVar41 = local_3b8[-2];
      local_438 = (IUnknown *)CONCAT44(local_438._4_4_,uVar41);
      if (uVar41 != 0) {
        if (*pcVar38 == '\0') {
          uVar17 = FUN_14019bd40(&local_450,uVar41,0);
          FUN_142ef7ba0(uVar17,local_3b8,(longlong)(int)uVar41);
          FUN_14019c870(&local_450,uVar41);
          pcVar38 = local_450;
        }
        else {
          for (iVar9 = *(int *)(pcVar38 + -0xc); iVar9 < (int)uVar41; iVar9 = iVar9 * 2) {
          }
          local_448 = uVar41;
          lVar24 = FUN_14019bd40(&local_450,iVar9,1);
          pcVar38 = local_450;
          iVar9 = iVar7;
          if (local_450 != (char *)0x0) {
            iVar9 = *(int *)(local_450 + -8);
          }
          FUN_142ef7ba0(iVar9 + lVar24,local_3b8,(longlong)(int)local_438);
          FUN_14019c870(&local_450,local_448);
        }
      }
    }
    uVar31 = 0xffffffffffffffff;
    do {
      local_440 = (IUnknown *)(uVar31 + 1);
      pcVar1 = &DAT_143271d01 + uVar31;
      uVar31 = (ulonglong)local_440;
    } while (*pcVar1 != '\0');
    iVar9 = (int)local_440;
    if (iVar9 != 0) {
      if ((pcVar38 == (char *)0x0) || (*pcVar38 == '\0')) {
        uVar17 = FUN_14019bd40(&local_450,(ulonglong)local_440 & 0xffffffff,0);
        FUN_142ef7ba0(uVar17,&DAT_143271d00,(longlong)iVar9);
        FUN_14019c870(&local_450,local_440);
        pcVar38 = local_450;
      }
      else {
        local_448 = *(int *)(pcVar38 + -8) + iVar9;
        for (iVar9 = *(int *)(pcVar38 + -0xc); iVar9 < (int)local_448; iVar9 = iVar9 * 2) {
        }
        lVar24 = FUN_14019bd40(&local_450,iVar9,1);
        pcVar38 = local_450;
        iVar9 = iVar7;
        if (local_450 != (char *)0x0) {
          iVar9 = *(int *)(local_450 + -8);
        }
        FUN_142ef7ba0(iVar9 + lVar24,&DAT_143271d00,(longlong)(int)local_440);
        FUN_14019c870(&local_450,local_448);
      }
    }
    local_420 = local_420 & 0xffffffff00000000;
    if ((int)local_3f0 != 0) {
      local_440 = (IUnknown *)0x0;
      do {
        if ((pcVar38 == (char *)0x0) || (*pcVar38 == '\0')) {
          puVar25 = (undefined4 *)FUN_14019bd40(&local_450,4);
          *puVar25 = 0xa0d0a0d;
          uVar41 = 4;
        }
        else {
          local_448 = *(int *)(pcVar38 + -8) + 4;
          for (iVar9 = *(int *)(pcVar38 + -0xc); iVar9 < (int)local_448; iVar9 = iVar9 * 2) {
          }
          puVar25 = (undefined4 *)FUN_14019bd40(&local_450);
          uVar41 = local_448;
          if (local_450 == (char *)0x0) {
            *puVar25 = 0xa0d0a0d;
          }
          else {
            *(undefined4 *)((longlong)*(int *)(local_450 + -8) + (longlong)puVar25) = 0xa0d0a0d;
          }
        }
        pcVar38 = local_450;
        FUN_14019c870(&local_450,uVar41);
        uVar41 = 0;
        if (plVar43 != (longlong *)0x0) {
          uVar41 = *(uint *)(plVar43 + -1);
        }
        if (uVar41 <= (uint)local_420) {
          FUN_142e54290(0xbc,local_420 & 0xffffffff);
        }
        local_438 = (IUnknown *)plVar43[(longlong)local_440];
        if ((local_438 != (IUnknown *)0x0) &&
           (iVar9 = *(int *)(local_438 + -8), local_418 = iVar9, iVar9 != 0)) {
          if ((pcVar38 == (char *)0x0) || (*pcVar38 == '\0')) {
            uVar17 = FUN_14019bd40(&local_450,iVar9,0);
            FUN_142ef7ba0(uVar17,local_438);
            FUN_14019c870(&local_450,iVar9);
            pcVar38 = local_450;
          }
          else {
            local_448 = *(int *)(pcVar38 + -8) + iVar9;
            for (iVar9 = *(int *)(pcVar38 + -0xc); iVar9 < (int)local_448; iVar9 = iVar9 * 2) {
            }
            lVar24 = FUN_14019bd40(&local_450,iVar9,1);
            pcVar38 = local_450;
            iVar9 = iVar7;
            if (local_450 != (char *)0x0) {
              iVar9 = *(int *)(local_450 + -8);
            }
            FUN_142ef7ba0(iVar9 + lVar24,local_438);
            FUN_14019c870(&local_450,local_448);
          }
        }
        local_420 = CONCAT44(local_420._4_4_,(uint)local_420 + 1);
        local_440 = (IUnknown *)((longlong)local_440 + 1);
      } while ((longlong)local_440 < (longlong)(int)local_3f0);
    }
    local_440 = DAT_143ad4850;
    if ((DAT_143ad4850 == (IUnknown *)0x0) &&
       (local_430 = (undefined8 **)FUN_14019b780(&DAT_143ad68a0,0x318),
       local_430 != (undefined8 **)0x0)) {
      FUN_142319280(local_430);
    }
    local_3f0 = 0;
    local_440 = DAT_143ad4850;
    FUN_14019a260(&local_3f0,&local_450);
    FUN_14231aef0(local_440,&local_3f0);
    FUN_14231b0a0(DAT_143ad4850);
    FUN_14231b0e0(DAT_143ad4850);
    FUN_142c0bf50(DAT_143abfdf8,local_3f8 + 8,0);
    lVar16 = local_400;
    if (pcVar38 != (char *)0x0) {
      FUN_14019f2c0(pcVar38 + -0x10);
      lVar16 = local_400;
    }
  }
LAB_14209a9de:
  uVar17 = FUN_142cbecc0(lVar16);
  FUN_142122400(uVar17,0);
  uVar17 = FUN_142cbecc0(lVar16);
  FUN_1421223c0(uVar17,0);
  if (local_408 != 0) {
    FUN_142d9e1c0(lVar16,1);
  }
  if (local_318 == 0) {
    FUN_142d9c9f0(lVar16);
  }
  if ((DAT_143acf088 != 0) && (lVar26 = FUN_141892840(), lVar24 = DAT_143acf088, lVar26 != 0)) {
    uVar17 = FUN_141892840();
    uVar8 = FUN_14182e9b0(uVar17);
    FUN_14246e7a0(lVar24,uVar8);
    lVar16 = local_400;
  }
  FUN_142defbc0(lVar16);
  FUN_142defc50(lVar16);
  FUN_142d16c50(lVar16);
  if ((DAT_143ad4868 != 0) && (FUN_142bf3f70(), DAT_143ad4868 != 0)) {
    (*(code *)**(undefined8 **)(DAT_143ad4868 + 8))((undefined8 *)(DAT_143ad4868 + 8),1);
  }
  if ((DAT_143ad4870 != 0) && (FUN_142bf3f70(), DAT_143ad4870 != 0)) {
    (*(code *)**(undefined8 **)(DAT_143ad4870 + 8))((undefined8 *)(DAT_143ad4870 + 8),1);
  }
  if ((DAT_143ad4878 != 0) && (FUN_142bf3f70(), DAT_143ad4878 != 0)) {
    (*(code *)**(undefined8 **)(DAT_143ad4878 + 8))((undefined8 *)(DAT_143ad4878 + 8),1);
  }
  if ((DAT_143aa84a0 != 0) &&
     ((iVar7 = FUN_142cb8550(), iVar7 != 0 ||
      ((DAT_143aa84a0 != 0 &&
       ((iVar7 = FUN_142cb8570(), iVar7 != 0 ||
        ((DAT_143aa84a0 != 0 && (iVar7 = FUN_142cb8580(), iVar7 != 0)))))))))) {
    FUN_142cb4530(lVar16,0x534,0);
    FUN_142cb4530(lVar16,0x489,0);
  }
  lVar24 = FUN_141892840();
  if (lVar24 != 0) {
    plVar27 = (longlong *)FUN_141892840();
    (**(code **)(*plVar27 + 0x148))(plVar27);
  }
  if (local_340 == 0) {
    FUN_1415aafa0(3);
  }
  FUN_142a27fe0();
  FUN_140c909f0(0x1ee);
  if (pIVar44 != (IUnknown *)0x0) {
    FUN_14019f2c0(pIVar44 + -0x10);
  }
  if (pIVar46 != (IUnknown *)0x0) {
    FUN_14019f2c0(pIVar46 + -0x10);
  }
  if (pIVar37 != (IUnknown *)0x0) {
    FUN_14019f2c0(pIVar37 + -0x10);
  }
  if (local_358 != (IUnknown *)0x0) {
    iVar7 = FUN_14022eb80();
    if ((iVar7 == 0) && (local_358 != (IUnknown *)0x0)) {
      (*(code *)**(undefined8 **)local_358)(local_358,1);
    }
    local_358 = (IUnknown *)0x0;
  }
  if (local_370 != 0) {
    puVar15 = (undefined8 *)(local_370 - 0x28);
    iVar7 = FUN_14022eb80(puVar15);
    if (iVar7 == 0) {
      if ((local_370 != 0) && (*(longlong *)(local_370 - 0x10) != 0)) {
        LOCK();
        *(undefined8 *)(*(longlong *)(local_370 - 0x10) + 8) = 0;
        UNLOCK();
        do {
          uVar33 = local_3c8;
          plVar43 = local_410;
        } while (*(int *)(*(longlong *)(local_370 - 0x10) + 4) != 0);
      }
      if (puVar15 != (undefined8 *)0x0) {
        (**(code **)*puVar15)(puVar15,1);
      }
    }
    local_370 = 0;
  }
  if (uVar33 != 0) {
    puVar15 = (undefined8 *)(uVar33 - 0x28);
    iVar7 = FUN_14022eb80(puVar15);
    if (iVar7 == 0) {
      if (*(longlong *)(uVar33 - 0x10) != 0) {
        LOCK();
        *(undefined8 *)(*(longlong *)(uVar33 - 0x10) + 8) = 0;
        UNLOCK();
        do {
          plVar43 = local_410;
        } while (*(int *)(*(longlong *)(uVar33 - 0x10) + 4) != 0);
      }
      if (puVar15 != (undefined8 *)0x0) {
        (**(code **)*puVar15)(puVar15,1);
      }
    }
  }
  if (plVar43 != (longlong *)0x0) {
    plVar27 = plVar43 + -1;
    plVar45 = plVar43 + *plVar27;
    for (; plVar43 < plVar45; plVar43 = plVar43 + 1) {
      if (*plVar43 != 0) {
        FUN_14019f2c0(*plVar43 + -0x10);
      }
    }
    thunk_FUN_140205820(plVar27,0);
  }
  if (local_3b8 != (int *)0x0) {
    FUN_14019f2c0(local_3b8 + -4);
  }
  return;
LAB_142099690:
  do {
    pIVar20 = local_438;
    if (uVar33 == 0) {
      FUN_142e52ed0(0x431);
      pIVar20 = local_438;
    }
    pIVar36 = *(IUnknown **)(pIVar20 + 8);
    if (*(IUnknown **)(pIVar20 + 8) == (IUnknown *)0x0) {
      local_438 = (IUnknown *)0x0;
      for (puVar15 = (undefined8 *)
                     (*(longlong *)(uVar33 + 0x1111) +
                     ((ulonglong)(longlong)*(int *)(pIVar20 + 0x10) %
                      (ulonglong)*(uint *)(uVar33 + 0x1119) + 1) * 8);
          (pIVar36 = local_438,
          puVar15 < (undefined8 *)
                    (*(longlong *)(uVar33 + 0x1111) + (ulonglong)*(uint *)(uVar33 + 0x1119) * 8) &&
          (pIVar36 = (IUnknown *)*puVar15, (IUnknown *)*puVar15 == (IUnknown *)0x0));
          puVar15 = puVar15 + 1) {
      }
    }
    local_438 = pIVar36;
    FUN_142ce4870(lVar24,*(int *)(pIVar20 + 0x10),*(int *)(pIVar20 + 0x14) * 1000);
  } while (local_438 != (IUnknown *)0x0);
LAB_14209971f:
  FUN_142d9cd90(lVar24,1);
  FUN_140d2d500(lVar24);
  goto LAB_14209973a;
}



//===========================================================
// FUN_14209b070 @ 14209b070   (772 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14209b070(undefined8 param_1,undefined8 param_2)

{
  longlong *plVar1;
  undefined8 uVar2;
  int iVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  int *piVar8;
  undefined1 auStack_138 [32];
  undefined4 local_118;
  int **local_110;
  undefined4 local_108;
  undefined4 local_100;
  undefined4 local_f8;
  undefined4 local_f0;
  undefined4 local_e8;
  undefined1 local_e0;
  int *local_d8;
  undefined4 local_d0 [2];
  undefined8 local_c8;
  undefined1 local_c0 [8];
  longlong local_b8;
  longlong local_a8;
  undefined1 local_98 [112];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_138;
  FUN_1406e9170(param_2,&local_c8,8);
  FUN_1408f67d0(local_c8);
  uVar2 = DAT_143aa84a0;
  lVar4 = FUN_1420a3080(0);
  local_a8 = lVar4;
  if (lVar4 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  uVar7 = 0;
  local_118 = 0;
  FUN_140304b20(lVar4,local_98,param_2,0);
  lVar5 = FUN_142cbe730(DAT_143aa84a0);
  uVar6 = *(undefined8 *)(lVar5 + 0x126b);
  if (lVar4 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  *(undefined8 *)(lVar4 + 0x126b) = uVar6;
  uVar6 = DAT_143aa84a0;
  local_b8 = lVar4;
  if (0xfffff < *(ulonglong *)(lVar4 + -0x20)) {
    FUN_142e541f0(0x30f);
  }
  LOCK();
  *(longlong *)(lVar4 + -0x20) = *(longlong *)(lVar4 + -0x20) + 1;
  UNLOCK();
  FUN_142cbe8f0(uVar6,local_c0,0);
  FUN_1415aafa0(5);
  if (DAT_143ad4848 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  iVar3 = (**(code **)(*(longlong *)(DAT_143ad4848 + 8) + 0xd0))
                    ((longlong *)(DAT_143ad4848 + 8),&PTR_PTR_143a88588);
  if (iVar3 == 0) {
    local_d8 = (int *)FUN_14019b780(&DAT_143ad68a0,0x68);
    uVar6 = uVar7;
    if (local_d8 != (int *)0x0) {
      uVar6 = FUN_141a3ca80(local_d8);
    }
    FUN_14209ee50(uVar6,0);
  }
  local_d0[0] = 0;
  local_d8 = (int *)FUN_14019b780(&DAT_143ad68a0,0x70);
  if (local_d8 != (int *)0x0) {
    uVar7 = FUN_140f33b70(local_d8,param_2);
  }
  FUN_14209ee50(uVar7,local_d0);
  local_d8 = (int *)0x0;
  piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,0x12);
  piVar8[1] = 0;
  *piVar8 = -1;
  local_d8 = piVar8 + 4;
  piVar8[2] = 0;
  *(undefined2 *)local_d8 = 0;
  if (*piVar8 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar8[1] < 0) {
    FUN_142e54290(0x90,piVar8[1],0);
  }
  *piVar8 = 1;
  *(undefined2 *)local_d8 = 0;
  if (piVar8[1] + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  piVar8[2] = 0;
  local_e0 = 0;
  local_e8 = 0;
  local_f0 = 0;
  local_f8 = 0x20;
  local_100 = 0;
  local_108 = 0;
  local_110 = &local_d8;
  local_118 = 0;
  FUN_142daff60(uVar2,3,0,1);
  FUN_142c50b90(5);
  lVar4 = local_a8;
  if (0xffffe < *(longlong *)(local_a8 + -0x20) - 1U) {
    FUN_142e541f0(0x31e);
  }
  LOCK();
  plVar1 = (longlong *)(lVar4 + -0x20);
  lVar5 = *plVar1;
  *plVar1 = *plVar1 + -1;
  UNLOCK();
  if ((int)lVar5 == 1) {
    if (*(longlong *)(local_a8 + -0x10) != 0) {
      LOCK();
      *(undefined8 *)(*(longlong *)(local_a8 + -0x10) + 8) = 0;
      UNLOCK();
      do {
      } while (*(int *)(*(longlong *)(local_a8 + -0x10) + 4) != 0);
    }
    (*(code *)**(undefined8 **)(lVar4 + -0x28))(lVar4 + -0x28,1);
  }
  return;
}



//===========================================================
// FUN_14209b380 @ 14209b380   (756 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14209b380(undefined8 param_1,undefined8 param_2)

{
  longlong *plVar1;
  undefined8 uVar2;
  int iVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  int *piVar8;
  undefined1 auStack_138 [32];
  undefined4 local_118;
  int **local_110;
  undefined4 local_108;
  undefined4 local_100;
  undefined4 local_f8;
  undefined4 local_f0;
  undefined4 local_e8;
  undefined1 local_e0;
  int *local_d8;
  undefined4 local_d0 [2];
  undefined8 local_c8;
  undefined1 local_c0 [8];
  longlong local_b8;
  longlong local_a8;
  undefined1 local_98 [112];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_138;
  FUN_1406e9170(param_2,&local_c8,8);
  FUN_1408f67d0(local_c8);
  uVar2 = DAT_143aa84a0;
  lVar4 = FUN_142cbe730(DAT_143aa84a0);
  lVar5 = FUN_1420a3080(0);
  local_a8 = lVar5;
  if (lVar5 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  uVar7 = 0;
  local_118 = 0;
  FUN_140304b20(lVar5,local_98,param_2,0);
  uVar6 = *(undefined8 *)(lVar4 + 0x126b);
  if (lVar5 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  *(undefined8 *)(lVar5 + 0x126b) = uVar6;
  local_b8 = lVar5;
  if (0xfffff < *(ulonglong *)(lVar5 + -0x20)) {
    FUN_142e541f0(0x30f);
  }
  LOCK();
  *(longlong *)(lVar5 + -0x20) = *(longlong *)(lVar5 + -0x20) + 1;
  UNLOCK();
  FUN_142cbe8f0(uVar2,local_c0,0);
  FUN_1415aafa0(6);
  if (DAT_143ad4848 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  iVar3 = (**(code **)(*(longlong *)(DAT_143ad4848 + 8) + 0xd0))
                    ((longlong *)(DAT_143ad4848 + 8),&PTR_PTR_143a88588);
  if (iVar3 == 0) {
    local_d8 = (int *)FUN_14019b780(&DAT_143ad68a0,0x68);
    uVar6 = uVar7;
    if (local_d8 != (int *)0x0) {
      uVar6 = FUN_141a3ca80(local_d8);
    }
    FUN_14209ee50(uVar6,0);
  }
  local_d0[0] = 0;
  local_d8 = (int *)FUN_14019b780(&DAT_143ad68a0,0x68);
  if (local_d8 != (int *)0x0) {
    uVar7 = FUN_1411daf30(local_d8,param_2);
  }
  FUN_14209ee50(uVar7,local_d0);
  local_d8 = (int *)0x0;
  piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,0x12);
  piVar8[1] = 0;
  *piVar8 = -1;
  local_d8 = piVar8 + 4;
  piVar8[2] = 0;
  *(undefined2 *)local_d8 = 0;
  if (*piVar8 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar8[1] < 0) {
    FUN_142e54290(0x90,piVar8[1],0);
  }
  *piVar8 = 1;
  *(undefined2 *)local_d8 = 0;
  if (piVar8[1] + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  piVar8[2] = 0;
  local_e0 = 0;
  local_e8 = 0;
  local_f0 = 0;
  local_f8 = 0x20;
  local_100 = 0;
  local_108 = 0;
  local_110 = &local_d8;
  local_118 = 0;
  FUN_142daff60(uVar2,3,0,1);
  FUN_142c50b90(6);
  lVar4 = local_a8;
  if (0xffffe < *(longlong *)(local_a8 + -0x20) - 1U) {
    FUN_142e541f0(0x31e);
  }
  LOCK();
  plVar1 = (longlong *)(lVar4 + -0x20);
  lVar5 = *plVar1;
  *plVar1 = *plVar1 + -1;
  UNLOCK();
  if ((int)lVar5 == 1) {
    if (*(longlong *)(local_a8 + -0x10) != 0) {
      LOCK();
      *(undefined8 *)(*(longlong *)(local_a8 + -0x10) + 8) = 0;
      UNLOCK();
      do {
      } while (*(int *)(*(longlong *)(local_a8 + -0x10) + 4) != 0);
    }
    (*(code *)**(undefined8 **)(lVar4 + -0x28))(lVar4 + -0x28,1);
  }
  return;
}



//===========================================================
// FUN_14209ad60 @ 14209ad60   (774 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14209ad60(undefined8 param_1,undefined8 param_2)

{
  longlong *plVar1;
  undefined8 uVar2;
  int iVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  int *piVar8;
  undefined1 auStack_138 [32];
  undefined4 local_118;
  int **local_110;
  undefined4 local_108;
  undefined4 local_100;
  undefined4 local_f8;
  undefined4 local_f0;
  undefined4 local_e8;
  undefined1 local_e0;
  int *local_d8;
  undefined4 local_d0 [2];
  undefined8 local_c8;
  undefined1 local_c0 [8];
  longlong local_b8;
  longlong local_a8;
  undefined1 local_98 [112];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_138;
  FUN_1406e9170(param_2,&local_c8,8);
  FUN_1408f67d0(local_c8);
  uVar2 = DAT_143aa84a0;
  lVar4 = FUN_1420a3080(0);
  local_a8 = lVar4;
  if (lVar4 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  uVar7 = 0;
  local_118 = 0;
  FUN_140304b20(lVar4,local_98,param_2,0);
  lVar5 = FUN_142cbe730(DAT_143aa84a0);
  uVar6 = *(undefined8 *)(lVar5 + 0x126b);
  if (lVar4 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  *(undefined8 *)(lVar4 + 0x126b) = uVar6;
  uVar6 = DAT_143aa84a0;
  local_b8 = lVar4;
  if (0xfffff < *(ulonglong *)(lVar4 + -0x20)) {
    FUN_142e541f0(0x30f);
  }
  LOCK();
  *(longlong *)(lVar4 + -0x20) = *(longlong *)(lVar4 + -0x20) + 1;
  UNLOCK();
  FUN_142cbe8f0(uVar6,local_c0,0);
  FUN_1415aafa0(4);
  if (DAT_143ad4848 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  iVar3 = (**(code **)(*(longlong *)(DAT_143ad4848 + 8) + 0xd0))
                    ((longlong *)(DAT_143ad4848 + 8),&PTR_PTR_143a88588);
  if (iVar3 == 0) {
    local_d8 = (int *)FUN_14019b780(&DAT_143ad68a0,0x68);
    uVar6 = uVar7;
    if (local_d8 != (int *)0x0) {
      uVar6 = FUN_141a3ca80(local_d8);
    }
    FUN_14209ee50(uVar6,0);
  }
  local_d0[0] = FUN_142cc42c0(DAT_143aa84a0);
  local_d8 = (int *)FUN_14019b780(&DAT_143ad68a0,0x1a0);
  if (local_d8 != (int *)0x0) {
    uVar7 = FUN_140d71cb0(local_d8,param_2);
  }
  FUN_14209ee50(uVar7,local_d0);
  local_d8 = (int *)0x0;
  piVar8 = (int *)FUN_1401bc720(&DAT_143ad6980,0x12);
  piVar8[1] = 0;
  *piVar8 = -1;
  local_d8 = piVar8 + 4;
  piVar8[2] = 0;
  *(undefined2 *)local_d8 = 0;
  if (*piVar8 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar8[1] < 0) {
    FUN_142e54290(0x90,piVar8[1],0);
  }
  *piVar8 = 1;
  *(undefined2 *)local_d8 = 0;
  if (piVar8[1] + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  piVar8[2] = 0;
  local_e0 = 0;
  local_e8 = 0;
  local_f0 = 0;
  local_f8 = 0x20;
  local_100 = 0;
  local_108 = 0;
  local_110 = &local_d8;
  local_118 = 0;
  FUN_142daff60(uVar2,3,0,1);
  FUN_142c50b90(4);
  lVar4 = local_a8;
  if (0xffffe < *(longlong *)(local_a8 + -0x20) - 1U) {
    FUN_142e541f0(0x31e);
  }
  LOCK();
  plVar1 = (longlong *)(lVar4 + -0x20);
  lVar5 = *plVar1;
  *plVar1 = *plVar1 + -1;
  UNLOCK();
  if ((int)lVar5 == 1) {
    if (*(longlong *)(local_a8 + -0x10) != 0) {
      LOCK();
      *(undefined8 *)(*(longlong *)(local_a8 + -0x10) + 8) = 0;
      UNLOCK();
      do {
      } while (*(int *)(*(longlong *)(local_a8 + -0x10) + 4) != 0);
    }
    (*(code *)**(undefined8 **)(lVar4 + -0x28))(lVar4 + -0x28,1);
  }
  return;
}


