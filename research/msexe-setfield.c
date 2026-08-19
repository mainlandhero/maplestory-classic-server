// Inbound game-stage opcode 0x0070 and everything its handler reads.
//
// NOTE ON THE FILE NAME: this file was opened under the working hypothesis that
// 0x0070 was SetField. It is not. FUN_142d51930 is CWvsContext::OnInventoryOperation
// (ModifyInventoryItem) - see research/msexe-setfield.md for the argument. The name is
// kept only because it is already referenced elsewhere.
//
// Contents, in dependency order:
//   FUN_142d51930   the 0x0070 handler itself
//   FUN_140303530   item-blob entry point: u8 slot type, factory, virtual Decode
//   FUN_1402cc180   the factory (1=Equip, 2=Bundle, 3=Pet, else NULL)
//   FUN_1402f7da0 / FUN_1402f7cd0 / FUN_1402f8b40   the three ctors (for the vtables)
//   FUN_140304100 / FUN_140304450 / FUN_140304550   Equip / Bundle / Pet Decode (vtbl+0x358)
//   FUN_1403035a0   GW_ItemSlotBase::RawDecode, called first by all three
//   FUN_140303b40   equip "stat block"  (optional-short block + a u32 flag word)
//   FUN_140303800   optional-short block: u32 mask + one i16 per set bit 0..16
//   FUN_1402cce00 / FUN_1402cd090 / FUN_1402cb4f0   fixed sub-blocks inside the equip decode
//   plus the non-reading helpers the handler branches on (FUN_140255650 & co).
//

//===========================================================
// FUN_142d51930 @ 142d51930   (11648 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142d51930(longlong *param_1,ulonglong param_2)

{
  int *piVar1;
  code *pcVar2;
  undefined8 *puVar3;
  bool bVar4;
  bool bVar5;
  char cVar6;
  byte bVar7;
  undefined1 uVar8;
  short sVar9;
  short sVar10;
  ushort uVar11;
  uint uVar12;
  undefined4 uVar13;
  int iVar14;
  int iVar15;
  uint uVar16;
  undefined4 uVar17;
  int iVar18;
  int iVar19;
  ulonglong *puVar20;
  longlong *plVar21;
  uint *puVar22;
  longlong lVar23;
  undefined8 uVar24;
  longlong lVar25;
  longlong *plVar26;
  undefined8 *puVar27;
  ulonglong uVar28;
  undefined8 uVar29;
  uint uVar30;
  longlong *plVar31;
  undefined4 *puVar32;
  longlong lVar33;
  int *piVar34;
  undefined4 *puVar35;
  undefined4 *puVar36;
  char *pcVar37;
  char *pcVar38;
  int iVar39;
  ulonglong uVar40;
  longlong *plVar41;
  ulonglong uVar42;
  undefined1 auStackY_488 [32];
  uint local_458;
  uint local_44c;
  longlong *local_448;
  undefined8 local_440;
  int local_438;
  uint local_434;
  longlong local_430;
  ulonglong local_428;
  longlong *local_420;
  undefined4 *local_418;
  longlong *local_410;
  int *local_408;
  uint local_400;
  uint local_3fc;
  uint local_3f8;
  char local_3f0;
  int local_3ec;
  undefined1 local_3e8 [8];
  longlong *local_3e0;
  int local_3d8;
  int local_3d4;
  undefined1 local_3d0 [8];
  undefined8 *local_3c8;
  undefined1 local_3c0 [8];
  longlong *local_3b8;
  char *local_3b0;
  ulonglong local_3a8;
  undefined4 local_3a0;
  undefined4 local_39c;
  uint local_398;
  uint local_394;
  int local_390;
  uint local_38c;
  uint local_388;
  uint local_384;
  undefined1 local_380 [8];
  undefined8 *local_378;
  char *local_370;
  int local_368;
  undefined8 local_360;
  longlong local_358;
  undefined1 local_350 [8];
  undefined8 local_348;
  undefined4 local_340;
  undefined4 local_33c;
  undefined4 local_338 [2];
  longlong local_330;
  uint local_328 [2];
  undefined1 local_320 [8];
  longlong local_318;
  longlong local_310;
  longlong local_308;
  longlong local_300;
  longlong local_2f8;
  longlong local_2f0;
  undefined1 local_2e8 [8];
  undefined8 *local_2e0;
  undefined1 local_2d8 [8];
  longlong *local_2d0;
  undefined1 local_2c8 [8];
  longlong local_2c0;
  undefined1 local_2b8 [8];
  longlong local_2b0;
  longlong local_2a8;
  longlong local_2a0;
  undefined1 local_298 [8];
  undefined8 *local_290;
  undefined1 local_288 [8];
  longlong local_280;
  undefined1 local_278 [8];
  longlong local_270;
  undefined1 local_268 [8];
  longlong local_260;
  undefined1 local_258 [8];
  longlong local_250;
  undefined1 local_248 [8];
  longlong local_240;
  undefined1 local_238 [8];
  undefined8 local_230;
  undefined1 local_228 [8];
  longlong local_220;
  undefined1 local_218 [8];
  longlong local_210;
  undefined1 local_208 [8];
  longlong local_200;
  undefined1 local_1f8 [8];
  longlong local_1f0;
  undefined1 local_1e8 [8];
  longlong *local_1e0;
  undefined1 local_1d8 [8];
  undefined8 local_1d0;
  undefined1 local_1c8 [8];
  longlong *local_1c0;
  undefined1 local_1b8 [8];
  longlong *local_1b0;
  undefined1 local_1a8 [8];
  undefined8 *local_1a0;
  undefined1 local_198 [16];
  undefined1 local_188 [16];
  undefined1 local_178 [16];
  undefined1 local_168 [16];
  undefined **local_158;
  char **local_150;
  undefined ***local_120;
  longlong local_118 [7];
  undefined8 local_e0;
  longlong local_d8 [7];
  undefined8 local_a0;
  longlong local_98 [6];
  longlong alStack_68 [6];
  
  alStack_68[5] = DAT_143a8b908 ^ (ulonglong)auStackY_488;
  local_428 = param_2;
  local_410 = param_1;
  cVar6 = FUN_1406e8ae0(param_2);
  if (cVar6 != '\0') {
    FUN_142cc4430(param_1,0);
  }
  cVar6 = FUN_1406e8ae0(param_2);
  local_3f0 = cVar6 != '\0';
  local_430 = FUN_142cbe730(param_1);
  puVar35 = (undefined4 *)0x0;
  bVar5 = false;
  local_418 = (undefined4 *)0x0;
  local_3b0 = (char *)0x0;
  local_458 = 0x1f;
  local_3a8 = 0x1f;
  local_3a0 = 100;
  local_39c = 0x18;
  local_158 = &PTR_LAB_143495df8;
  local_150 = &local_3b0;
  local_120 = &local_158;
  FUN_1403ebca0(local_430,0xffffffff,&local_158);
  local_448 = (longlong *)0x0;
  local_440 = 0x1f;
  local_438 = 100;
  local_434 = 0x18;
  local_390 = 0;
  _eh_vector_constructor_iterator_(alStack_68 + 2,8,3,FUN_140c2f230,FUN_1401a1850);
  local_398 = 0xffffffff;
  local_394 = 0xffffffff;
  iVar39 = 2;
  local_3d4 = 2;
  plVar26 = alStack_68 + 2;
  do {
    uVar12 = FUN_140255590(iVar39);
    if (*plVar26 != 0) {
      thunk_FUN_140205820(*plVar26 + -8);
      *plVar26 = 0;
    }
    if (uVar12 != 0) {
      puVar20 = (ulonglong *)FUN_14019b780(&DAT_143ad68a0);
      if (puVar20 == (ulonglong *)0x0) {
        *plVar26 = 0;
      }
      else {
        *plVar26 = (longlong)(puVar20 + 1);
        if (puVar20 + 1 != (ulonglong *)0x0) {
          *puVar20 = (ulonglong)uVar12;
        }
      }
    }
    uVar40 = 0;
    uVar42 = uVar40;
    if (0 < (int)uVar12) {
      do {
        uVar17 = 0;
        lVar23 = *plVar26;
        uVar30 = 0;
        if (lVar23 != 0) {
          uVar30 = *(uint *)(lVar23 + -8);
        }
        if (uVar30 <= (uint)uVar40) {
          if (lVar23 != 0) {
            uVar17 = *(undefined4 *)(lVar23 + -8);
          }
          FUN_142e54290(0xbc,uVar40,uVar17);
          lVar23 = *plVar26;
        }
        *(undefined4 *)(uVar42 + lVar23) = 0;
        uVar30 = (uint)uVar40 + 1;
        uVar40 = (ulonglong)uVar30;
        uVar42 = uVar42 + 4;
      } while ((int)uVar30 < (int)uVar12);
    }
    iVar39 = iVar39 + 1;
    plVar26 = plVar26 + 1;
  } while (iVar39 < 5);
  local_3d8 = 0;
  iVar39 = FUN_1406e8c20(local_428);
  local_3ec = iVar39;
  bVar7 = FUN_1406e8ae0(local_428);
  local_370 = (char *)CONCAT44(local_370._4_4_,(uint)bVar7);
  local_360 = (int *)((ulonglong)local_360._4_4_ << 0x20);
  local_368 = 0;
  local_3fc = 0;
  uVar42 = local_428;
  plVar26 = local_410;
  iVar18 = 0;
  iVar19 = 0;
  lVar23 = (longlong)iVar39;
  if (0 < iVar39) {
    do {
      local_358 = lVar23;
      plVar21 = local_448;
      bVar7 = FUN_1406e8ae0(uVar42);
      cVar6 = FUN_1406e8ae0(uVar42);
      uVar40 = (ulonglong)cVar6;
      local_3f8 = (int)cVar6;
      sVar9 = FUN_1406e8b80(uVar42);
      plVar26 = DAT_143aa8518;
      local_44c = (uint)sVar9;
      local_400 = local_44c;
      if (bVar7 < 0xd) {
        iVar19 = (int)cVar6;
        iVar18 = (int)cVar6;
        iVar14 = (int)cVar6;
        iVar15 = (int)cVar6;
        switch(bVar7) {
        case 0:
          FUN_140303530(local_3e8);
          if (local_3e0 != (longlong *)0x0) {
            if (0 < sVar9) {
              iVar18 = FUN_14019a5d0(local_3e0 + 4);
              uVar12 = local_458;
              plVar26 = plVar21;
              if (plVar21 == (longlong *)0x0) {
LAB_142d51c28:
                if ((uVar12 != 0) && ((local_458 != uVar12 || (plVar21 == (longlong *)0x0)))) {
                  puVar22 = (uint *)&DAT_1434956a0;
                  uVar42 = 0xf6;
                  do {
                    uVar40 = uVar42 >> 1;
                    if (puVar22[uVar40] < uVar12) {
                      puVar22 = puVar22 + uVar40 + 1;
                      uVar40 = uVar42 + (-1 - uVar40);
                    }
                    uVar42 = uVar40;
                  } while (0 < (longlong)uVar40);
                  uVar12 = *puVar22;
                  uVar42 = (ulonglong)local_458;
                  local_440 = CONCAT44(local_440._4_4_,uVar12);
                  if (local_438 == -1) {
                    local_434 = 0xffffffff;
                  }
                  else {
                    local_434 = uVar12 * local_438 >> 7;
                  }
                  plVar26 = (longlong *)FUN_14019b780(&DAT_143ad68a0);
                  local_448 = plVar26;
                  FUN_142ef8250(plVar26,0,(ulonglong)uVar12 * 8);
                  local_458 = uVar12;
                  plVar41 = plVar21;
                  if (plVar21 != (longlong *)0x0) {
                    while (plVar41 < plVar21 + uVar42) {
                      lVar23 = *plVar41;
                      plVar41 = plVar41 + 1;
                      while (lVar23 != 0) {
                        uVar40 = (ulonglong)(longlong)*(int *)(lVar23 + 0x10) % (ulonglong)uVar12;
                        lVar25 = *(longlong *)(lVar23 + 8);
                        *(longlong *)(lVar23 + 8) = plVar26[uVar40];
                        plVar26[uVar40] = lVar23;
                        lVar23 = lVar25;
                      }
                    }
                    FUN_14019b4e0(plVar21);
                  }
                }
              }
              else if (local_434 < local_440._4_4_) {
                uVar12 = local_458 * 2;
                goto LAB_142d51c28;
              }
              uVar42 = (ulonglong)(longlong)iVar18 % (ulonglong)local_458;
              for (lVar23 = plVar26[uVar42]; lVar23 != 0; lVar23 = *(longlong *)(lVar23 + 8)) {
                if (*(int *)(lVar23 + 0x10) == iVar18) {
                  *(uint *)(lVar23 + 0x14) = local_44c;
                  goto LAB_142d51dce;
                }
              }
              local_440 = CONCAT44(local_440._4_4_ + 1,(uint)local_440);
              local_420 = (longlong *)FUN_14036f510();
              if (local_420 == (longlong *)0x0) {
                plVar21 = (longlong *)0x0;
              }
              else {
                lVar23 = plVar26[uVar42];
                *local_420 = (longlong)&PTR_FUN_14327ecd8;
                local_420[1] = lVar23;
                local_420[2] = 0;
                *(int *)(local_420 + 2) = iVar18;
                plVar21 = local_420;
              }
              *(uint *)((longlong)plVar21 + 0x14) = local_44c;
              plVar26[uVar42] = (longlong)plVar21;
LAB_142d51dce:
              uVar40 = (ulonglong)local_3f8;
            }
            if (local_3e0 == (longlong *)0x0) {
              FUN_142e52ed0(0x431);
            }
            iVar18 = FUN_14019a5d0(local_3e0 + 4);
            if ((local_3b0 == (char *)0x0) ||
               (lVar23 = *(longlong *)
                          (local_3b0 + ((ulonglong)(longlong)iVar18 % (local_3a8 & 0xffffffff)) * 8)
               , lVar23 == 0)) {
LAB_142d51e33:
              uVar17 = 0;
            }
            else {
              do {
                if (*(int *)(lVar23 + 0x10) == iVar18) {
                  uVar17 = *(undefined4 *)(lVar23 + 0x14);
                  if ((undefined4 *)(lVar23 + 0x14) != (undefined4 *)0x0) goto LAB_142d51e3e;
                  goto LAB_142d51e33;
                }
                lVar23 = *(longlong *)(lVar23 + 8);
              } while (lVar23 != 0);
              uVar17 = 0;
            }
LAB_142d51e3e:
            if (local_3e0 == (longlong *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            FUN_14019a5d0(local_3e0 + 4);
            if (local_3e0 == (longlong *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            iVar18 = FUN_14019a5d0(local_3e0 + 4);
            if (iVar18 - 3700000U < 10000) {
              if (local_3e0 == (longlong *)0x0) {
                FUN_142e52ed0(0x431,0);
              }
              uVar30 = FUN_14019a5d0(local_3e0 + 4);
              uVar12 = local_3fc;
              local_3fc = uVar12;
              if (local_3fc == uVar30) {
                if (local_3e0 == (longlong *)0x0) {
                  FUN_142e52ed0(0x431,0);
                }
                cVar6 = (**(code **)(*local_3e0 + 0x1b0))();
                local_3fc = uVar12;
                if (cVar6 == '\0') {
                  local_3fc = 0;
                }
              }
            }
            plVar26 = local_3e0;
            local_1b0 = local_3e0;
            if (local_3e0 != (longlong *)0x0) {
              if (0xfffff < (ulonglong)local_3e0[1]) {
                FUN_142e541f0(0x30f);
              }
              LOCK();
              plVar26[1] = plVar26[1] + 1;
              UNLOCK();
              local_458 = (uint)local_440;
              puVar35 = local_418;
              local_44c = local_400;
            }
            FUN_1402e4c20(local_430,uVar40 & 0xffffffff,local_44c,local_1b8);
            if (local_3e0 == (longlong *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            uVar13 = FUN_14019a5d0(local_3e0 + 4);
            plVar26 = local_410;
            FUN_142d9b200(local_410,uVar13,uVar17);
            if (local_3e0 == (longlong *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            uVar17 = FUN_14019a5d0(local_3e0 + 4);
            FUN_142ce53e0(plVar26,uVar17,1);
            uVar24 = DAT_143aa8328;
            if (local_3e0 == (longlong *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            uVar17 = FUN_14019a5d0(local_3e0 + 4);
            iVar18 = FUN_140388e10(uVar24,uVar17);
            if (iVar18 != 0) {
              lVar23 = FUN_140192f00(local_3e0);
              local_330 = 0;
              if ((lVar23 != 0) &&
                 (iVar18 = FUN_1401ba9d0(lVar23 + 0x13a,*(undefined4 *)(lVar23 + 0x142)),
                 uVar24 = DAT_143aa8328, iVar18 == 0)) {
                uVar17 = FUN_14019a5d0(lVar23 + 0x20);
                puVar27 = (undefined8 *)FUN_140398ba0(uVar24,&local_2f8,uVar17);
                uVar24 = *puVar27;
                iVar18 = FUN_14019a5d0(lVar23 + 0x20);
                if ((iVar18 - 1500000U < 10000) || (bVar4 = false, iVar18 - 0x170a70U < 10000)) {
                  bVar4 = true;
                }
                uVar29 = 0xe3e;
                if (bVar4) {
                  uVar29 = 0xf44;
                }
                puVar27 = (undefined8 *)FUN_1408a9e40(&local_2a8,uVar29);
                uVar24 = FUN_14019ba10(&local_330,*puVar27,uVar24);
                FUN_1415eca30(uVar24,0xb);
                if (local_2a8 != 0) {
                  FUN_14019f2c0(local_2a8 + -0x10);
                }
                if (local_2f8 != 0) {
                  FUN_14019f2c0(local_2f8 + -0x10);
                }
              }
              if (local_330 != 0) {
                FUN_14019f2c0(local_330 + -0x10);
              }
            }
            if (DAT_143ac8e08 != 0) {
              FUN_142208060();
            }
            if (DAT_143ad20e8 != (longlong *)0x0) {
              (**(code **)(*DAT_143ad20e8 + 0x90))(DAT_143ad20e8,0);
            }
            plVar26 = local_3e0;
            uVar42 = local_428;
            if (local_3e0 != (longlong *)0x0) {
              if (0xffffe < local_3e0[1] - 1U) {
                FUN_142e541f0(0x31e);
              }
              LOCK();
              plVar26 = plVar26 + 1;
              lVar23 = *plVar26;
              *plVar26 = *plVar26 + -1;
              UNLOCK();
              if (((int)lVar23 == 1) && (local_3e0 != (longlong *)0x0)) {
                (**(code **)*local_3e0)(local_3e0,1);
              }
              local_3e0 = (longlong *)0x0;
              local_458 = (uint)local_440;
              uVar42 = local_428;
              puVar35 = local_418;
            }
          }
          break;
        case 1:
          sVar9 = FUN_1406e8b80(uVar42);
          uVar12 = (uint)sVar9;
          local_3f8 = uVar12;
          lVar23 = FUN_1402e3cd0(local_430,local_2e8,iVar19,local_44c);
          puVar27 = local_2e0;
          plVar26 = *(longlong **)(lVar23 + 8);
          local_420 = plVar26;
          if (local_2e0 != (undefined8 *)0x0) {
            if (0xffffe < local_2e0[1] - 1) {
              FUN_142e541f0();
            }
            LOCK();
            plVar21 = puVar27 + 1;
            lVar23 = *plVar21;
            *plVar21 = *plVar21 + -1;
            UNLOCK();
            if (((int)lVar23 == 1) && (local_2e0 != (undefined8 *)0x0)) {
              (**(code **)*local_2e0)();
            }
            local_2e0 = (undefined8 *)0x0;
            local_458 = (uint)local_440;
            local_44c = local_400;
            puVar35 = local_418;
          }
          uVar42 = local_428;
          if (plVar26 != (longlong *)0x0) {
            if (0 < (int)local_44c) {
              iVar18 = FUN_14019a5d0(plVar26 + 4);
              plVar21 = local_448;
              uVar30 = local_458;
              if (local_448 == (longlong *)0x0) {
LAB_142d522e5:
                if ((uVar30 != 0) && ((local_458 != uVar30 || (local_448 == (longlong *)0x0)))) {
                  puVar22 = (uint *)&DAT_1434956a0;
                  uVar42 = 0xf6;
                  do {
                    uVar40 = uVar42 >> 1;
                    if (puVar22[uVar40] < uVar30) {
                      puVar22 = puVar22 + uVar40 + 1;
                      uVar40 = uVar42 + (-1 - uVar40);
                    }
                    uVar42 = uVar40;
                  } while (0 < (longlong)uVar40);
                  uVar30 = *puVar22;
                  plVar41 = local_448 + local_458;
                  local_440 = CONCAT44(local_440._4_4_,uVar30);
                  if (local_438 == -1) {
                    local_434 = 0xffffffff;
                  }
                  else {
                    local_434 = local_438 * uVar30 >> 7;
                  }
                  local_448 = (longlong *)FUN_14019b780(&DAT_143ad68a0);
                  FUN_142ef8250(local_448,0,(ulonglong)uVar30 * 8);
                  plVar26 = local_420;
                  uVar12 = local_3f8;
                  local_458 = uVar30;
                  plVar31 = plVar21;
                  if (plVar21 != (longlong *)0x0) {
                    while (plVar31 < plVar41) {
                      lVar23 = *plVar31;
                      plVar31 = plVar31 + 1;
                      while (lVar23 != 0) {
                        uVar42 = (ulonglong)(longlong)*(int *)(lVar23 + 0x10) % (ulonglong)uVar30;
                        lVar25 = *(longlong *)(lVar23 + 8);
                        *(longlong *)(lVar23 + 8) = local_448[uVar42];
                        local_448[uVar42] = lVar23;
                        lVar23 = lVar25;
                      }
                    }
                    FUN_14019b4e0(plVar21);
                    uVar12 = local_3f8;
                  }
                }
              }
              else if (local_434 < local_440._4_4_) {
                uVar30 = local_458 * 2;
                goto LAB_142d522e5;
              }
              puVar20 = (ulonglong *)
                        (local_448 + (ulonglong)(longlong)iVar18 % (ulonglong)local_458);
              for (uVar42 = *puVar20; uVar42 != 0; uVar42 = *(ulonglong *)(uVar42 + 8)) {
                if (*(int *)(uVar42 + 0x10) == iVar18) {
                  *(uint *)(uVar42 + 0x14) = local_44c;
                  goto LAB_142d5247c;
                }
              }
              local_440 = CONCAT44(local_440._4_4_ + 1,(uint)local_440);
              local_420 = (longlong *)FUN_14036f510(0x18);
              if (local_420 != (longlong *)0x0) {
                uVar42 = *puVar20;
                *local_420 = (longlong)&PTR_FUN_14327ecd8;
                local_420[1] = uVar42;
                local_420[2] = 0;
                *(int *)(local_420 + 2) = iVar18;
              }
              *(uint *)((longlong)local_420 + 0x14) = local_44c;
              *puVar20 = (ulonglong)local_420;
            }
LAB_142d5247c:
            iVar18 = FUN_14019a5d0(plVar26 + 4);
            if (local_3b0 != (char *)0x0) {
              for (lVar23 = *(longlong *)
                             (local_3b0 +
                             ((ulonglong)(longlong)iVar18 % (local_3a8 & 0xffffffff)) * 8);
                  lVar23 != 0; lVar23 = *(longlong *)(lVar23 + 8)) {
                if (*(int *)(lVar23 + 0x10) == iVar18) {
                  uVar17 = *(undefined4 *)(lVar23 + 0x14);
                  if ((undefined4 *)(lVar23 + 0x14) != (undefined4 *)0x0) goto LAB_142d524c7;
                  break;
                }
              }
            }
            uVar17 = 0;
LAB_142d524c7:
            (**(code **)(*plVar26 + 0xb8))(plVar26,uVar12 & 0xffff);
            uVar13 = FUN_14019a5d0(plVar26 + 4);
            FUN_142ce53e0(local_410,uVar13,1);
            uVar13 = FUN_14019a5d0(plVar26 + 4);
            FUN_142d9b200(local_410,uVar13,uVar17);
            uVar42 = local_428;
            if (DAT_143ac8e08 != 0) {
              FUN_142208060();
              uVar42 = local_428;
            }
          }
          break;
        case 2:
          sVar10 = FUN_1406e8b80(uVar42);
          iVar18 = (int)sVar10;
          if (((iVar15 == 1) || (iVar15 == 6)) && ((sVar9 < 0 || (iVar18 < 0)))) {
            bVar5 = true;
          }
          FUN_1402e3cd0(local_430,local_3d0,iVar15,iVar18);
          FUN_1402e3cd0(local_430,local_380,iVar15,local_44c);
          if (local_3c8 != (undefined8 *)0x0) {
            uVar17 = FUN_14019a5d0(local_3c8 + 4);
            if (puVar35 == (undefined4 *)0x0) {
              uVar12 = 0;
              iVar19 = 1;
LAB_142d525db:
              if (puVar35 == (undefined4 *)0x0) {
                iVar14 = 0;
              }
              else {
                uVar42 = *(ulonglong *)(puVar35 + -4);
                uVar28 = ~uVar42;
                if (-1 < (longlong)uVar42) {
                  uVar28 = uVar42;
                }
                iVar14 = (int)(uVar28 - 8 >> 2);
              }
              puVar32 = (undefined4 *)0x0;
              if (iVar14 != iVar19) {
                puVar36 = puVar32;
                if (puVar35 != (undefined4 *)0x0) {
                  puVar36 = (undefined4 *)(ulonglong)(uint)puVar35[-2];
                }
                lVar23 = FUN_14019b780(&DAT_143ad68a0);
                if (lVar23 != 0) {
                  puVar32 = (undefined4 *)(lVar23 + 8);
                }
                if (puVar35 != (undefined4 *)0x0) {
                  FUN_142ef7ba0(puVar32,puVar35,(longlong)puVar36 << 2);
                  thunk_FUN_140205820(puVar35 + -2);
                }
                *(undefined4 **)(puVar32 + -2) = puVar36;
                puVar35 = puVar32;
                local_418 = puVar32;
              }
            }
            else {
              uVar12 = puVar35[-2];
              uVar42 = *(ulonglong *)(puVar35 + -4);
              uVar28 = ~uVar42;
              if (-1 < (longlong)uVar42) {
                uVar28 = uVar42;
              }
              if ((uint)(uVar28 - 8 >> 2) <= uVar12) {
                if (uVar12 == 0) {
                  iVar19 = 1;
                }
                else {
                  iVar19 = uVar12 * 2;
                }
                goto LAB_142d525db;
              }
            }
            *(longlong *)(puVar35 + -2) = *(longlong *)(puVar35 + -2) + 1;
            puVar35[(int)uVar12] = uVar17;
          }
          if (local_378 != (undefined8 *)0x0) {
            uVar17 = FUN_14019a5d0(local_378 + 4);
            if (puVar35 == (undefined4 *)0x0) {
              uVar12 = 0;
              iVar19 = 1;
LAB_142d526cb:
              if (puVar35 == (undefined4 *)0x0) {
                iVar14 = 0;
              }
              else {
                uVar42 = *(ulonglong *)(puVar35 + -4);
                uVar28 = ~uVar42;
                if (-1 < (longlong)uVar42) {
                  uVar28 = uVar42;
                }
                iVar14 = (int)(uVar28 - 8 >> 2);
              }
              puVar32 = (undefined4 *)0x0;
              if (iVar14 != iVar19) {
                puVar36 = puVar32;
                if (puVar35 != (undefined4 *)0x0) {
                  puVar36 = (undefined4 *)(ulonglong)(uint)puVar35[-2];
                }
                lVar23 = FUN_14019b780(&DAT_143ad68a0);
                if (lVar23 != 0) {
                  puVar32 = (undefined4 *)(lVar23 + 8);
                }
                if (puVar35 != (undefined4 *)0x0) {
                  FUN_142ef7ba0(puVar32,puVar35,(longlong)puVar36 << 2);
                  thunk_FUN_140205820(puVar35 + -2);
                }
                *(undefined4 **)(puVar32 + -2) = puVar36;
                puVar35 = puVar32;
                local_418 = puVar32;
              }
            }
            else {
              uVar12 = puVar35[-2];
              uVar42 = *(ulonglong *)(puVar35 + -4);
              uVar28 = ~uVar42;
              if (-1 < (longlong)uVar42) {
                uVar28 = uVar42;
              }
              if ((uint)(uVar28 - 8 >> 2) <= uVar12) {
                if (uVar12 == 0) {
                  iVar19 = 1;
                }
                else {
                  iVar19 = uVar12 * 2;
                }
                goto LAB_142d526cb;
              }
            }
            *(longlong *)(puVar35 + -2) = *(longlong *)(puVar35 + -2) + 1;
            puVar35[(int)uVar12] = uVar17;
          }
          if ((0 < (int)local_44c) || (0 < sVar10)) {
            if ((local_3c8 == (undefined8 *)0x0) ||
               ((iVar19 = FUN_14019a5d0(local_3c8 + 4), plVar26 = local_410,
                iVar19 != *(int *)((longlong)local_410 + 0x303c) ||
                (iVar18 != (int)local_410[0x608])))) {
              plVar26 = local_410;
              if ((local_378 == (undefined8 *)0x0) ||
                 ((iVar19 = FUN_14019a5d0(local_378 + 4),
                  iVar19 != *(int *)((longlong)plVar26 + 0x303c) ||
                  (local_44c != *(uint *)(plVar26 + 0x608))))) goto LAB_142d52bc5;
              if (local_378 == (undefined8 *)0x0) {
                FUN_142e52ed0(0x431);
              }
              iVar19 = FUN_14019a5d0(local_378 + 4);
              plVar21 = local_448;
              uVar12 = local_458;
              if (local_448 == (longlong *)0x0) {
LAB_142d52a1e:
                if ((uVar12 != 0) && ((local_458 != uVar12 || (local_448 == (longlong *)0x0)))) {
                  puVar22 = (uint *)&DAT_1434956a0;
                  uVar42 = 0xf6;
                  do {
                    uVar40 = uVar42 >> 1;
                    if (puVar22[uVar40] < uVar12) {
                      puVar22 = puVar22 + uVar40 + 1;
                      uVar40 = uVar42 + (-1 - uVar40);
                    }
                    uVar42 = uVar40;
                  } while (0 < (longlong)uVar40);
                  uVar12 = *puVar22;
                  plVar41 = local_448 + local_458;
                  local_440 = CONCAT44(local_440._4_4_,uVar12);
                  if (local_438 == -1) {
                    local_434 = 0xffffffff;
                  }
                  else {
                    local_434 = local_438 * uVar12 >> 7;
                  }
                  local_448 = (longlong *)FUN_14019b780(&DAT_143ad68a0);
                  FUN_142ef8250(local_448,0,(ulonglong)uVar12 * 8);
                  plVar26 = local_410;
                  local_458 = uVar12;
                  plVar31 = plVar21;
                  if (plVar21 != (longlong *)0x0) {
                    while (plVar31 < plVar41) {
                      lVar23 = *plVar31;
                      plVar31 = plVar31 + 1;
                      while (lVar23 != 0) {
                        uVar42 = (ulonglong)(longlong)*(int *)(lVar23 + 0x10) % (ulonglong)uVar12;
                        lVar25 = *(longlong *)(lVar23 + 8);
                        *(longlong *)(lVar23 + 8) = local_448[uVar42];
                        local_448[uVar42] = lVar23;
                        lVar23 = lVar25;
                      }
                    }
                    FUN_14019b4e0(plVar21);
                    plVar26 = local_410;
                  }
                }
              }
              else if (local_434 < local_440._4_4_) {
                uVar12 = local_458 * 2;
                goto LAB_142d52a1e;
              }
              puVar20 = (ulonglong *)
                        (local_448 + (ulonglong)(longlong)iVar19 % (ulonglong)local_458);
              for (uVar42 = *puVar20; uVar42 != 0; uVar42 = *(ulonglong *)(uVar42 + 8)) {
                if (*(int *)(uVar42 + 0x10) == iVar19) {
                  *(int *)(uVar42 + 0x14) = iVar18;
                  goto LAB_142d52bb3;
                }
              }
              local_440 = CONCAT44(local_440._4_4_ + 1,(uint)local_440);
              local_420 = (longlong *)FUN_14036f510(0x18);
              if (local_420 != (longlong *)0x0) {
                uVar42 = *puVar20;
                *local_420 = (longlong)&PTR_FUN_14327ecd8;
                local_420[1] = uVar42;
                local_420[2] = 0;
                *(int *)(local_420 + 2) = iVar19;
              }
              *(int *)((longlong)local_420 + 0x14) = iVar18;
              uVar12 = uRam0000000000000014;
            }
            else {
              if (local_3c8 == (undefined8 *)0x0) {
                FUN_142e52ed0(0x431);
              }
              iVar19 = FUN_14019a5d0(local_3c8 + 4);
              plVar21 = local_448;
              uVar12 = local_458;
              if (local_448 == (longlong *)0x0) {
LAB_142d527f3:
                if ((uVar12 != 0) && ((local_458 != uVar12 || (local_448 == (longlong *)0x0)))) {
                  puVar22 = (uint *)&DAT_1434956a0;
                  uVar42 = 0xf6;
                  do {
                    uVar40 = uVar42 >> 1;
                    if (puVar22[uVar40] < uVar12) {
                      puVar22 = puVar22 + uVar40 + 1;
                      uVar40 = uVar42 + (-1 - uVar40);
                    }
                    uVar42 = uVar40;
                  } while (0 < (longlong)uVar40);
                  uVar12 = *puVar22;
                  plVar41 = local_448 + local_458;
                  local_440 = CONCAT44(local_440._4_4_,uVar12);
                  if (local_438 == -1) {
                    local_434 = 0xffffffff;
                  }
                  else {
                    local_434 = local_438 * uVar12 >> 7;
                  }
                  local_448 = (longlong *)FUN_14019b780(&DAT_143ad68a0);
                  FUN_142ef8250(local_448,0,(ulonglong)uVar12 * 8);
                  plVar26 = local_410;
                  local_458 = uVar12;
                  plVar31 = plVar21;
                  if (plVar21 != (longlong *)0x0) {
                    while (plVar31 < plVar41) {
                      lVar23 = *plVar31;
                      plVar31 = plVar31 + 1;
                      while (lVar23 != 0) {
                        uVar42 = (ulonglong)(longlong)*(int *)(lVar23 + 0x10) % (ulonglong)uVar12;
                        lVar25 = *(longlong *)(lVar23 + 8);
                        *(longlong *)(lVar23 + 8) = local_448[uVar42];
                        local_448[uVar42] = lVar23;
                        lVar23 = lVar25;
                      }
                    }
                    FUN_14019b4e0(plVar21);
                    plVar26 = local_410;
                  }
                }
              }
              else if (local_434 < local_440._4_4_) {
                uVar12 = local_458 * 2;
                goto LAB_142d527f3;
              }
              puVar20 = (ulonglong *)
                        (local_448 + (ulonglong)(longlong)iVar19 % (ulonglong)local_458);
              for (uVar42 = *puVar20; uVar42 != 0; uVar42 = *(ulonglong *)(uVar42 + 8)) {
                if (*(int *)(uVar42 + 0x10) == iVar19) {
                  *(uint *)(uVar42 + 0x14) = local_44c;
                  goto LAB_142d52bb3;
                }
              }
              local_440 = CONCAT44(local_440._4_4_ + 1,(uint)local_440);
              local_420 = (longlong *)FUN_14036f510(0x18);
              uVar12 = local_44c;
              if (local_420 != (longlong *)0x0) {
                uVar42 = *puVar20;
                *local_420 = (longlong)&PTR_FUN_14327ecd8;
                local_420[1] = uVar42;
                local_420[2] = 0;
                *(int *)(local_420 + 2) = iVar19;
                *(uint *)((longlong)local_420 + 0x14) = local_44c;
                uVar12 = uRam0000000000000014;
              }
            }
            uRam0000000000000014 = uVar12;
            *puVar20 = (ulonglong)local_420;
LAB_142d52bb3:
            local_390 = (int)plVar26[0x608];
            uVar40 = (ulonglong)local_3f8;
          }
LAB_142d52bc5:
          puVar27 = local_3c8;
          local_1a0 = local_3c8;
          if (local_3c8 != (undefined8 *)0x0) {
            if (0xfffff < (ulonglong)local_3c8[1]) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            puVar27[1] = puVar27[1] + 1;
            UNLOCK();
            local_458 = (uint)local_440;
            puVar35 = local_418;
            local_44c = local_400;
          }
          lVar23 = local_430;
          FUN_1402e4c20(local_430,uVar40 & 0xffffffff,local_44c,local_1a8);
          puVar27 = local_378;
          local_290 = local_378;
          if (local_378 != (undefined8 *)0x0) {
            if (0xfffff < (ulonglong)local_378[1]) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            puVar27[1] = puVar27[1] + 1;
            UNLOCK();
            local_458 = (uint)local_440;
            puVar35 = local_418;
            local_44c = local_400;
          }
          FUN_1402e4c20(lVar23,uVar40 & 0xffffffff,iVar18,local_298);
          plVar26 = local_410;
          if (local_3c8 != (undefined8 *)0x0) {
            uVar17 = FUN_14019a5d0(local_3c8 + 4);
            plVar26 = local_410;
            FUN_142ce53e0(local_410,uVar17,1);
            if (bVar5) {
              if (local_3c8 == (undefined8 *)0x0) {
                FUN_142e52ed0(0x431,0);
              }
              uVar17 = FUN_14019a5d0(local_3c8 + 4);
              FUN_142d9bfd0(plVar26,uVar17);
            }
          }
          if (local_378 != (undefined8 *)0x0) {
            uVar17 = FUN_14019a5d0(local_378 + 4);
            FUN_142ce53e0(plVar26,uVar17,1);
            if (bVar5) {
              if (local_378 == (undefined8 *)0x0) {
                FUN_142e52ed0(0x431,0);
              }
              uVar17 = FUN_14019a5d0(local_378 + 4);
              FUN_142d9bfd0(plVar26,uVar17);
            }
          }
          if ((sVar10 < 0) || ((int)local_44c < 0)) {
            local_360 = (int *)CONCAT44(local_360._4_4_,1);
          }
          if ((((local_44c == 0xfffffff6) || (iVar18 == -10)) &&
              (iVar19 = (**(code **)(*plVar26 + 0xa8))(plVar26), iVar19 - 0x78U < 3)) &&
             (DAT_143aa8518 != (longlong *)0x0)) {
            FUN_1428f4eb0(DAT_143aa8518,0x410,0,0);
          }
          if (DAT_143ac8e08 != 0) {
            FUN_142208060();
          }
          if (bVar5) {
            iVar14 = -local_44c;
            iVar19 = FUN_1402536d0(iVar14);
            if (iVar19 == 0) {
              iVar15 = -iVar18;
              iVar19 = FUN_1402536d0(iVar15);
              if (iVar19 == 0) {
                if (local_378 != (undefined8 *)0x0) {
                  if (((iVar18 < 0) && (0x1f < iVar15 - 3000U)) &&
                     ((0x1f < iVar15 - 0xc1cU && (0x1f < iVar15 - 0xc80U)))) {
                    uVar17 = 1;
                  }
                  else {
                    uVar17 = 0;
                  }
                  uVar24 = FUN_140193080(local_378);
                  FUN_142da62e0(plVar26,uVar24,uVar17);
                }
                if (local_3c8 != (undefined8 *)0x0) {
                  if ((((int)local_44c < 0) && (0x1f < iVar14 - 3000U)) &&
                     ((0x1f < iVar14 - 0xc1cU && (0x1f < iVar14 - 0xc80U)))) {
                    uVar17 = 1;
                  }
                  else {
                    uVar17 = 0;
                  }
                  uVar24 = FUN_140193080(local_3c8);
                  FUN_142da62e0(plVar26,uVar24,uVar17);
                }
              }
            }
          }
          if (DAT_143ad20e8 != (longlong *)0x0) {
            (**(code **)(*DAT_143ad20e8 + 0x90))(DAT_143ad20e8,0);
          }
          puVar27 = local_378;
          if ((iVar18 < 0) || ((int)local_44c < 0)) {
            local_368 = 1;
          }
          if (local_378 != (undefined8 *)0x0) {
            if (0xffffe < local_378[1] - 1) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            plVar26 = puVar27 + 1;
            lVar23 = *plVar26;
            *plVar26 = *plVar26 + -1;
            UNLOCK();
            if (((int)lVar23 == 1) && (local_378 != (undefined8 *)0x0)) {
              (**(code **)*local_378)(local_378,1);
            }
            local_378 = (undefined8 *)0x0;
            local_458 = (uint)local_440;
            puVar35 = local_418;
          }
          puVar27 = local_3c8;
          uVar42 = local_428;
          if (local_3c8 != (undefined8 *)0x0) {
            if (0xffffe < local_3c8[1] - 1) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            plVar26 = puVar27 + 1;
            lVar23 = *plVar26;
            *plVar26 = *plVar26 + -1;
            UNLOCK();
            if (((int)lVar23 == 1) && (local_3c8 != (undefined8 *)0x0)) {
              (**(code **)*local_3c8)(local_3c8,1);
            }
            local_3c8 = (undefined8 *)0x0;
            local_458 = (uint)local_440;
            uVar42 = local_428;
            puVar35 = local_418;
          }
          break;
        case 3:
          lVar23 = FUN_1402e3cd0(local_430,local_178,iVar15,local_44c);
          plVar21 = *(longlong **)(lVar23 + 8);
          FUN_1401abd80(local_178);
          if (plVar21 != (longlong *)0x0) {
            uVar12 = FUN_14019a5d0(plVar21 + 4);
            if (local_3b0 != (char *)0x0) {
              for (lVar23 = *(longlong *)
                             (local_3b0 +
                             ((ulonglong)(longlong)(int)uVar12 % (local_3a8 & 0xffffffff)) * 8);
                  lVar23 != 0; lVar23 = *(longlong *)(lVar23 + 8)) {
                if (*(uint *)(lVar23 + 0x10) == uVar12) {
                  uVar17 = *(undefined4 *)(lVar23 + 0x14);
                  if ((undefined4 *)(lVar23 + 0x14) != (undefined4 *)0x0) goto LAB_142d52fec;
                  break;
                }
              }
            }
            uVar17 = 0;
LAB_142d52fec:
            local_328[0] = uVar12;
            if (0 < (int)local_44c) {
              FUN_1402fc930(&local_448,local_328,&local_400);
              local_458 = (uint)local_440;
            }
            plVar41 = local_410;
            if ((((plVar26 != (longlong *)0x0) &&
                 (cVar6 = (**(code **)(*plVar26 + 0x120))(plVar26), plVar41 = local_410,
                 cVar6 != '\0')) &&
                (cVar6 = (**(code **)(*plVar26 + 0xd0))(plVar26,uVar12), plVar41 = local_410,
                cVar6 != '\0')) &&
               (iVar18 = FUN_1402e9260(local_430,iVar15,uVar12,1), plVar41 = local_410, iVar18 == 1)
               ) {
              FUN_142cd3a60(local_410,0);
            }
            if (((iVar15 == 1) || (iVar15 == 6)) && ((int)local_44c < 0)) {
              bVar5 = true;
            }
            puVar22 = (uint *)FUN_1401abb40(&local_418,0xffffffffffffffff);
            *puVar22 = uVar12;
            iVar18 = (int)uVar12 / 10000;
            if (iVar18 == 0xdf) {
              if ((int)local_370 != 0) goto LAB_142d530f8;
              FUN_1402e5710(local_430,uVar12);
              uVar30 = local_3fc;
LAB_142d530dc:
              local_3fc = uVar30;
              local_348 = 0;
              FUN_1402e4c20(local_430,iVar19,local_44c,local_350);
LAB_142d5311e:
              FUN_142ce53e0(plVar41,uVar12,1);
            }
            else {
              if ((iVar18 == 0x172) &&
                 (cVar6 = (**(code **)(*plVar21 + 0x1b0))(plVar21), uVar30 = uVar12, cVar6 == '\x01'
                 )) goto LAB_142d530dc;
LAB_142d530f8:
              local_348 = 0;
              FUN_1402e4c20(local_430,iVar19,local_44c,local_350);
              if ((iVar18 != 0x77) || (local_358 == 1)) goto LAB_142d5311e;
            }
            FUN_142d9b200(plVar41,uVar12,uVar17);
            FUN_142ce71f0(plVar41,uVar12,1);
            if (DAT_143ac8e08 != 0) {
              FUN_142208060();
            }
            iVar18 = FUN_142ce7140(plVar41,uVar12,local_44c);
            uVar42 = local_428;
            puVar35 = local_418;
            if (iVar18 != 0) {
              FUN_142ce7120(plVar41);
              uVar42 = local_428;
              puVar35 = local_418;
            }
          }
          break;
        case 4:
          uVar24 = FUN_1406e8f10(uVar42);
          FUN_1402e3cd0(local_430,local_2d8,iVar19,local_44c);
          if (local_2d0 == (longlong *)0x0) {
            FUN_1401abd80(local_2d8);
          }
          else {
            (**(code **)(*local_2d0 + 0x1a0))(local_2d0,uVar24);
            FUN_1401abd80(local_2d8);
          }
          break;
        case 5:
          FUN_140303530(local_320,uVar42);
          if (local_318 == 0) {
            FUN_1401abd80(local_320);
          }
          else {
            iVar18 = FUN_140255650(local_44c,iVar14);
            lVar23 = local_318;
            if (iVar18 == 0) {
              local_270 = local_318;
              if (local_318 != 0) {
                if (0xfffff < *(ulonglong *)(local_318 + 8)) {
                  FUN_142e541f0(0x30f);
                }
                LOCK();
                plVar26 = (longlong *)(lVar23 + 8);
                *plVar26 = *plVar26 + 1;
                UNLOCK();
                local_458 = (uint)local_440;
                puVar35 = local_418;
                local_44c = local_400;
              }
              FUN_1402e4c20(local_430,iVar14,local_44c,local_278);
            }
            else {
              local_280 = local_318;
              if (local_318 != 0) {
                if (0xfffff < *(ulonglong *)(local_318 + 8)) {
                  FUN_142e541f0(0x30f);
                }
                LOCK();
                plVar26 = (longlong *)(lVar23 + 8);
                *plVar26 = *plVar26 + 1;
                UNLOCK();
                local_458 = (uint)local_440;
                puVar35 = local_418;
                local_44c = local_400;
              }
              FUN_14022f680(local_430,local_44c,local_288,iVar19);
            }
            FUN_1401abd80(local_320);
          }
          break;
        case 6:
          *(char *)(local_430 + 0x1a6) = (char)sVar9;
          break;
        case 7:
          uVar17 = FUN_1406e8c20(uVar42);
          lVar23 = local_430;
          FUN_14022f570(local_248,local_430,uVar17,iVar14);
          FUN_1402e3cd0(lVar23,local_2c8,iVar14,local_44c);
          if (local_2c0 != 0) {
            uVar13 = FUN_14019a5d0(local_2c0 + 0x20);
            iVar18 = FUN_142ce7140(local_410,uVar13,local_44c);
            if (iVar18 != 0) {
              local_3d8 = 1;
            }
          }
          lVar25 = local_240;
          local_260 = local_240;
          if (local_240 != 0) {
            if (0xfffff < *(ulonglong *)(local_240 + 8)) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            *(longlong *)(lVar25 + 8) = *(longlong *)(lVar25 + 8) + 1;
            UNLOCK();
            local_458 = (uint)local_440;
            puVar35 = local_418;
            lVar23 = local_430;
            local_44c = local_400;
          }
          FUN_1402e4c20(lVar23,iVar14,local_44c,local_268);
          lVar25 = local_2c0;
          local_250 = local_2c0;
          if (local_2c0 != 0) {
            if (0xfffff < *(ulonglong *)(local_2c0 + 8)) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            *(longlong *)(lVar25 + 8) = *(longlong *)(lVar25 + 8) + 1;
            UNLOCK();
            local_458 = (uint)local_440;
            puVar35 = local_418;
          }
          FUN_14022f680(lVar23,uVar17,local_258,iVar14);
          FUN_1401abd80(local_2c8);
          FUN_1401abd80(local_248);
          uVar42 = local_428;
          break;
        case 8:
          iVar18 = FUN_140255650(local_44c,iVar14);
          if (iVar18 != 0) {
            sVar9 = FUN_1406e8b80(uVar42);
            lVar23 = FUN_14022f570(local_168,local_430,local_44c,iVar14);
            plVar26 = *(longlong **)(lVar23 + 8);
            FUN_1401abd80(local_168);
            if (plVar26 != (longlong *)0x0) {
              uVar30 = FUN_14019a5d0(plVar26 + 4);
              uVar12 = local_3f8;
              local_384 = 0;
              local_340 = 0;
              lVar23 = (longlong)(int)local_3f8;
              FUN_140255700(local_44c,&local_384,&local_340,local_3f8);
              iVar18 = FUN_140255610(local_384,uVar12);
              uVar42 = local_428;
              if (iVar18 != 0) {
                local_398 = uVar30;
                local_394 = local_44c;
                lVar25 = FUN_140192e80(plVar26);
                uVar11 = FUN_1401b00e0(lVar25 + 0x4d);
                lVar33 = (longlong)(int)local_384;
                lVar25 = alStack_68[lVar23];
                uVar17 = 0;
                uVar12 = 0;
                if (lVar25 != 0) {
                  uVar12 = *(uint *)(lVar25 + -8);
                }
                if (((int)local_384 < 0) || (uVar12 <= local_384)) {
                  if (lVar25 != 0) {
                    uVar17 = *(undefined4 *)(lVar25 + -8);
                  }
                  FUN_142e54290(0xbc,local_384,uVar17);
                  lVar25 = alStack_68[lVar23];
                }
                piVar34 = (int *)(lVar25 + lVar33 * 4);
                *piVar34 = *piVar34 + ((int)sVar9 - (uint)uVar11);
                iVar18 = 0;
                iVar19 = FUN_14019a5d0(plVar26 + 4);
                uVar24 = DAT_143aa8328;
                if (local_3b0 != (char *)0x0) {
                  for (lVar23 = *(longlong *)
                                 (local_3b0 +
                                 ((ulonglong)(longlong)iVar19 % (local_3a8 & 0xffffffff)) * 8);
                      iVar18 = 0, lVar23 != 0; lVar23 = *(longlong *)(lVar23 + 8)) {
                    if (*(int *)(lVar23 + 0x10) == iVar19) {
                      iVar18 = *(int *)(lVar23 + 0x14);
                      break;
                    }
                  }
                }
                local_420 = local_118;
                local_e0 = 0;
                uVar17 = FUN_14019a5d0(plVar26 + 4);
                iVar19 = FUN_140388410(uVar24,local_430,uVar17,1);
                (**(code **)(*plVar26 + 0xb8))(plVar26,sVar9);
                FUN_142d9b200(local_410,uVar30,iVar18 + iVar19);
LAB_142d535c0:
                FUN_142ce53e0(local_410,uVar30,1);
                uVar42 = local_428;
              }
            }
          }
          break;
        case 9:
          iVar18 = FUN_140255650(local_44c,iVar14);
          if (iVar18 != 0) {
            lVar23 = FUN_14022f570(local_198,local_430,local_44c,iVar14);
            plVar26 = *(longlong **)(lVar23 + 8);
            FUN_1401abd80(local_198);
            if (plVar26 != (longlong *)0x0) {
              uVar30 = FUN_14019a5d0(plVar26 + 4);
              uVar12 = local_3f8;
              local_388 = 0;
              local_33c = 0;
              lVar23 = (longlong)(int)local_3f8;
              FUN_140255700(local_44c,&local_388,&local_33c,local_3f8);
              iVar18 = FUN_140255610(local_388,uVar12);
              if (iVar18 != 0) {
                lVar25 = FUN_140192e80(plVar26);
                uVar11 = FUN_1401b00e0(lVar25 + 0x4d);
                lVar33 = (longlong)(int)local_388;
                lVar25 = alStack_68[lVar23];
                uVar17 = 0;
                uVar12 = 0;
                if (lVar25 != 0) {
                  uVar12 = *(uint *)(lVar25 + -8);
                }
                if (((int)local_388 < 0) || (uVar12 <= local_388)) {
                  if (lVar25 != 0) {
                    uVar17 = *(undefined4 *)(lVar25 + -8);
                  }
                  FUN_142e54290(0xbc,local_388,uVar17);
                  lVar25 = alStack_68[lVar23];
                }
                piVar34 = (int *)(lVar25 + lVar33 * 4);
                *piVar34 = *piVar34 - (uint)uVar11;
                iVar18 = 0;
                iVar19 = FUN_14019a5d0(plVar26 + 4);
                uVar24 = DAT_143aa8328;
                if (local_3b0 != (char *)0x0) {
                  for (lVar23 = *(longlong *)
                                 (local_3b0 +
                                 ((ulonglong)(longlong)iVar19 % (local_3a8 & 0xffffffff)) * 8);
                      lVar23 != 0; lVar23 = *(longlong *)(lVar23 + 8)) {
                    if (*(int *)(lVar23 + 0x10) == iVar19) {
                      iVar18 = *(int *)(lVar23 + 0x14);
                      break;
                    }
                  }
                }
                local_420 = local_d8;
                local_a0 = 0;
                uVar17 = FUN_14019a5d0(plVar26 + 4);
                iVar19 = FUN_140388410(uVar24,local_430,uVar17,1);
                uVar12 = local_3f8;
                plVar21 = DAT_143aa8518;
                if (((local_3f8 == 3) && (DAT_143aa8518 != (longlong *)0x0)) &&
                   (cVar6 = (**(code **)(*DAT_143aa8518 + 0x120))(DAT_143aa8518), cVar6 != '\0')) {
                  pcVar2 = *(code **)(*plVar21 + 0xd0);
                  uVar17 = FUN_14019a5d0(plVar26 + 4);
                  cVar6 = (*pcVar2)(plVar21,uVar17);
                  uVar12 = local_3f8;
                  if ((cVar6 != '\0') && (iVar18 + iVar19 == 1)) {
                    FUN_142cd3a60(local_410,0);
                    uVar12 = local_3f8;
                  }
                }
                if ((uVar30 - 3700000 < 10000) &&
                   (cVar6 = (**(code **)(*plVar26 + 0x1b0))(plVar26), cVar6 == '\x01')) {
                  local_3fc = uVar30;
                }
                local_230 = 0;
                FUN_14022f680(local_430,local_44c,local_238,uVar12);
                FUN_142d9b200(local_410,uVar30,iVar18 + iVar19);
                goto LAB_142d535c0;
              }
            }
          }
          break;
        case 10:
          sVar9 = FUN_1406e8b80(uVar42);
          lVar23 = local_430;
          FUN_14022f570(local_1f8,local_430,local_44c,iVar18);
          FUN_14022f570(local_208,lVar23,(int)sVar9,iVar18);
          lVar25 = local_200;
          local_220 = local_200;
          if (local_200 != 0) {
            if (0xfffff < *(ulonglong *)(local_200 + 8)) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            *(longlong *)(lVar25 + 8) = *(longlong *)(lVar25 + 8) + 1;
            UNLOCK();
            local_458 = (uint)local_440;
            puVar35 = local_418;
            lVar23 = local_430;
          }
          FUN_14022f680(lVar23,local_44c,local_228,iVar14);
          lVar25 = local_1f0;
          local_210 = local_1f0;
          if (local_1f0 != 0) {
            if (0xfffff < *(ulonglong *)(local_1f0 + 8)) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            *(longlong *)(lVar25 + 8) = *(longlong *)(lVar25 + 8) + 1;
            UNLOCK();
            local_458 = (uint)local_440;
            puVar35 = local_418;
          }
          FUN_14022f680(lVar23,(int)sVar9,local_218,iVar14);
          FUN_1401abd80(local_208);
          FUN_1401abd80(local_1f8);
          uVar42 = local_428;
          break;
        case 0xb:
          FUN_140303530(local_3c0,uVar42);
          if (local_3b8 == (longlong *)0x0) {
            FUN_1401abd80(local_3c0);
          }
          else {
            uVar12 = FUN_14019a5d0(local_3b8 + 4);
            iVar19 = FUN_140255650(local_44c,iVar18);
            if (iVar19 == 0) {
              FUN_1401abd80(local_3c0);
            }
            else {
              uVar17 = 0;
              uVar30 = 0;
              local_38c = 0;
              local_338[0] = 0;
              FUN_140255700(local_44c,&local_38c,local_338,iVar18);
              iVar19 = FUN_140255610(local_38c,iVar18);
              if (iVar19 == 0) {
                FUN_1401abd80(local_3c0);
              }
              else {
                local_398 = uVar12;
                local_394 = local_44c;
                lVar23 = FUN_140192e80(local_3b8);
                uVar11 = FUN_1401b00e0(lVar23 + 0x4d,*(undefined4 *)(lVar23 + 0x51));
                lVar25 = (longlong)(int)local_38c;
                lVar23 = alStack_68[uVar40];
                if (lVar23 != 0) {
                  uVar30 = *(uint *)(lVar23 + -8);
                }
                if (((int)local_38c < 0) || (uVar30 <= local_38c)) {
                  if (lVar23 != 0) {
                    uVar17 = *(undefined4 *)(lVar23 + -8);
                  }
                  FUN_142e54290(0xbc,local_38c,uVar17);
                  lVar23 = alStack_68[uVar40];
                }
                piVar34 = (int *)(lVar23 + lVar25 * 4);
                *piVar34 = *piVar34 + (uint)uVar11;
                iVar19 = 0;
                if (local_3b8 == (longlong *)0x0) {
                  FUN_142e52ed0(0x431);
                }
                iVar14 = FUN_14019a5d0(local_3b8 + 4);
                uVar24 = DAT_143aa8328;
                if (local_3b0 != (char *)0x0) {
                  for (lVar23 = *(longlong *)
                                 (local_3b0 +
                                 ((ulonglong)(longlong)iVar14 % (local_3a8 & 0xffffffff)) * 8);
                      lVar23 != 0; lVar23 = *(longlong *)(lVar23 + 8)) {
                    if (*(int *)(lVar23 + 0x10) == iVar14) {
                      iVar19 = *(int *)(lVar23 + 0x14);
                      break;
                    }
                  }
                }
                local_420 = local_98;
                alStack_68[1] = 0;
                if (local_3b8 == (longlong *)0x0) {
                  FUN_142e52ed0(0x431,0);
                }
                uVar17 = FUN_14019a5d0(local_3b8 + 4);
                lVar23 = local_430;
                iVar14 = FUN_140388410(uVar24,local_430,uVar17,1);
                if (local_3b8 == (longlong *)0x0) {
                  FUN_142e52ed0(0x431,0);
                }
                iVar15 = FUN_14019a5d0(local_3b8 + 4);
                if (iVar15 - 3700000U < 10000) {
                  if (local_3b8 == (longlong *)0x0) {
                    FUN_142e52ed0(0x431,0);
                  }
                  uVar16 = FUN_14019a5d0(local_3b8 + 4);
                  uVar30 = local_3fc;
                  local_3fc = uVar30;
                  if (local_3fc == uVar16) {
                    if (local_3b8 == (longlong *)0x0) {
                      FUN_142e52ed0(0x431,0);
                    }
                    cVar6 = (**(code **)(*local_3b8 + 0x1b0))();
                    local_3fc = uVar30;
                    if (cVar6 == '\0') {
                      local_3fc = 0;
                    }
                  }
                }
                plVar26 = local_3b8;
                local_1e0 = local_3b8;
                if (local_3b8 != (longlong *)0x0) {
                  if (0xfffff < (ulonglong)local_3b8[1]) {
                    FUN_142e541f0(0x30f);
                  }
                  LOCK();
                  plVar26[1] = plVar26[1] + 1;
                  UNLOCK();
                  local_458 = (uint)local_440;
                  puVar35 = local_418;
                }
                FUN_14022f680(lVar23,local_44c,local_1e8,iVar18);
                FUN_142d9b200(local_410,uVar12,iVar19 + iVar14);
                FUN_142ce53e0(local_410,uVar12,1);
                FUN_1401abd80(local_3c0);
                uVar42 = local_428;
              }
            }
          }
          break;
        case 0xc:
          local_1d0 = 0;
          FUN_1402e5020(local_430,(int)cVar6,local_44c,local_1d8);
          plVar26 = local_410;
          iVar19 = FUN_142d9ed40(local_410,iVar18,local_44c);
          if (iVar19 != 0) {
            uVar17 = FUN_142746d20(iVar18,local_44c);
            FUN_142cb4530(plVar26,uVar17,0);
          }
        }
      }
      plVar21 = DAT_143aa8518;
      uVar12 = 0;
      iVar39 = iVar39 + -1;
      lVar23 = local_358;
    } while (0 < iVar39);
    plVar26 = local_410;
    if (bVar5) {
      puVar32 = puVar35;
      if (DAT_143aa8518 != (longlong *)0x0) {
        uVar8 = FUN_1406e8ae0(uVar42);
        FUN_1428a7f00(plVar21,uVar8);
        plVar26 = local_410;
      }
      for (; (puVar35 != (undefined4 *)0x0 && (uVar12 < (uint)puVar35[-2])); uVar12 = uVar12 + 1) {
        if ((int)uVar12 < 0) {
          FUN_142e54290(0xbc,uVar12);
        }
        FUN_142ce51b0(plVar26,*puVar32);
        puVar32 = puVar32 + 1;
      }
      FUN_142ce5e60(plVar26);
      iVar18 = (int)local_360;
      iVar39 = local_3ec;
      iVar19 = local_3d8;
    }
    else {
      iVar18 = (int)local_360;
      iVar39 = local_3ec;
      iVar19 = local_3d8;
    }
  }
  FUN_142cbefd0(plVar26,1,0,0);
  if ((iVar18 != 0) && (DAT_143aa8518 != (longlong *)0x0)) {
    FUN_1427e90b0();
    FUN_1428336c0(DAT_143aa8518);
  }
  if ((DAT_143acda08 != 0) && (0 < iVar39)) {
    FUN_1414bba10();
  }
  if (DAT_143ad8410 != 0) {
    FUN_14241fc90();
  }
  if (DAT_143ad84b8 != 0) {
    FUN_14243c950();
  }
  if (DAT_143ac8240 != 0) {
    FUN_142386c20();
  }
  lVar23 = FUN_14139de10(local_2b8,0x5e9);
  lVar25 = local_2b0;
  lVar23 = *(longlong *)(lVar23 + 8);
  if (local_2b0 != 0) {
    if (0xffffe < *(longlong *)(local_2b0 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar21 = (longlong *)(lVar25 + 0x20);
    lVar25 = *plVar21;
    *plVar21 = *plVar21 + -1;
    UNLOCK();
    if ((int)lVar25 == 1) {
      puVar27 = (undefined8 *)(local_2b0 + 0x18);
      if (local_2b0 == 0) {
        puVar27 = (undefined8 *)0x0;
      }
      if (puVar27 != (undefined8 *)0x0) {
        (**(code **)*puVar27)(puVar27,1);
      }
    }
    local_2b0 = 0;
    local_458 = (uint)local_440;
    puVar35 = local_418;
  }
  if (lVar23 != 0) {
    FUN_1411ba610(lVar23);
  }
  if (iVar19 == 0) {
    if (local_448 != (longlong *)0x0) {
      uVar42 = (ulonglong)local_458;
      plVar21 = local_448 + uVar42;
      for (plVar41 = local_448; plVar26 = local_410, local_428 = uVar42, local_420 = plVar21,
          plVar41 < plVar21; plVar41 = plVar41 + 1) {
        lVar23 = *plVar41;
        if (lVar23 != 0) goto LAB_142d53df0;
      }
    }
LAB_142d5419d:
    plVar21 = alStack_68 + 2;
    iVar39 = 2;
    do {
      uVar40 = 0;
      iVar18 = FUN_140255590(iVar39);
      uVar42 = uVar40;
      if (0 < iVar18) {
        do {
          uVar17 = 0;
          lVar23 = *plVar21;
          uVar12 = 0;
          if (lVar23 != 0) {
            uVar12 = *(uint *)(lVar23 + -8);
          }
          uVar30 = (uint)uVar40;
          if (uVar12 <= uVar30) {
            if (lVar23 != 0) {
              uVar17 = *(undefined4 *)(lVar23 + -8);
            }
            FUN_142e54290(0xbc,uVar40,uVar17);
            lVar23 = *plVar21;
          }
          lVar25 = local_430;
          if ((*(int *)(lVar23 + uVar42 * 4) != 0) &&
             (FUN_1402e4e10(local_430,&local_410,iVar39,uVar40), piVar34 = local_408,
             local_408 != (int *)0x0)) {
            iVar39 = *local_408;
            if ((iVar39 - 0x286f90U < 10000) ||
               ((iVar39 - 0x2eff40U < 10000 || (iVar39 - 0x421210U < 10000)))) {
              local_3ec = 0;
              iVar19 = FUN_14022f410(lVar25,uVar40,local_3d4,&local_3ec);
              if ((iVar19 == 0) || (local_3ec == 0)) {
                iVar39 = local_3d4;
                if (local_408 != (int *)0x0) {
                  piVar34 = local_408 + -10;
                  iVar39 = FUN_14022eb80(piVar34);
                  if (iVar39 == 0) {
                    if ((local_408 != (int *)0x0) && (*(longlong *)(local_408 + -4) != 0)) {
                      LOCK();
                      *(undefined8 *)(*(longlong *)(local_408 + -4) + 8) = 0;
                      UNLOCK();
                      do {
                      } while (*(int *)(*(longlong *)(local_408 + -4) + 4) != 0);
                    }
                    if (piVar34 != (int *)0x0) {
                      (*(code *)**(undefined8 **)piVar34)(piVar34,1);
                    }
                  }
                  local_408 = (int *)0x0;
                  iVar39 = local_3d4;
                }
              }
              else {
                lVar23 = *plVar21;
                uVar17 = 0;
                uVar16 = 0;
                uVar12 = uVar16;
                if (lVar23 != 0) {
                  uVar12 = *(uint *)(lVar23 + -8);
                }
                if (uVar12 <= uVar30) {
                  uVar13 = uVar17;
                  if (lVar23 != 0) {
                    uVar13 = *(undefined4 *)(lVar23 + -8);
                  }
                  FUN_142e54290(0xbc,uVar40,uVar13);
                  lVar23 = *plVar21;
                }
                if (*(int *)(lVar23 + uVar42 * 4) < 0) {
                  iVar19 = FUN_142ce7140(plVar26,iVar39,local_3ec);
                  if (iVar19 != 0) {
                    FUN_142ce7120(plVar26);
                    lVar23 = *plVar21;
                    goto LAB_142d545b2;
                  }
                  iVar39 = local_3d4;
                  if (local_408 == (int *)0x0) goto LAB_142d543b8;
                  piVar34 = local_408 + -10;
                  iVar39 = FUN_14022eb80(piVar34);
                  if (iVar39 == 0) {
                    if ((local_408 != (int *)0x0) && (*(longlong *)(local_408 + -4) != 0)) {
                      LOCK();
                      *(undefined8 *)(*(longlong *)(local_408 + -4) + 8) = 0;
                      UNLOCK();
                      do {
                      } while (*(int *)(*(longlong *)(local_408 + -4) + 4) != 0);
                    }
                    goto LAB_142d5439c;
                  }
                }
                else {
LAB_142d545b2:
                  if (lVar23 != 0) {
                    uVar16 = *(uint *)(lVar23 + -8);
                  }
                  if (uVar16 <= uVar30) {
                    if (lVar23 != 0) {
                      uVar17 = *(undefined4 *)(lVar23 + -8);
                    }
                    FUN_142e54290(0xbc,uVar40,uVar17);
                    lVar23 = *plVar21;
                  }
                  if (0 < *(int *)(lVar23 + uVar42 * 4)) {
                    FUN_142ce70c0(plVar26,iVar39,local_3ec);
                    FUN_142ce7100(plVar26,local_398,local_394);
                  }
                  iVar39 = local_3d4;
                  if (local_408 == (int *)0x0) goto LAB_142d543b8;
                  piVar34 = local_408 + -10;
                  iVar39 = FUN_14022eb80(piVar34);
                  if (iVar39 == 0) {
                    if ((local_408 != (int *)0x0) && (*(longlong *)(local_408 + -4) != 0)) {
                      LOCK();
                      *(undefined8 *)(*(longlong *)(local_408 + -4) + 8) = 0;
                      UNLOCK();
                      do {
                      } while (*(int *)(*(longlong *)(local_408 + -4) + 4) != 0);
                    }
LAB_142d5439c:
                    if (piVar34 != (int *)0x0) {
                      (*(code *)**(undefined8 **)piVar34)(piVar34,1);
                    }
                  }
                }
                local_408 = (int *)0x0;
                iVar39 = local_3d4;
              }
            }
            else {
              piVar1 = local_408 + -10;
              if (0xffffe < *(longlong *)(local_408 + -8) - 1U) {
                FUN_142e541f0(0x31e);
              }
              LOCK();
              plVar41 = (longlong *)(piVar34 + -8);
              lVar23 = *plVar41;
              *plVar41 = *plVar41 + -1;
              UNLOCK();
              if ((int)lVar23 == 1) {
                if ((local_408 != (int *)0x0) && (*(longlong *)(local_408 + -4) != 0)) {
                  LOCK();
                  *(undefined8 *)(*(longlong *)(local_408 + -4) + 8) = 0;
                  UNLOCK();
                  do {
                  } while (*(int *)(*(longlong *)(local_408 + -4) + 4) != 0);
                }
                if (piVar1 != (int *)0x0) {
                  (*(code *)**(undefined8 **)piVar1)(piVar1,1);
                }
              }
              local_408 = (int *)0x0;
              iVar39 = local_3d4;
            }
          }
LAB_142d543b8:
          uVar40 = (ulonglong)(uVar30 + 1);
          uVar42 = uVar42 + 1;
        } while ((longlong)uVar42 < (longlong)iVar18);
      }
      local_3d4 = iVar39 + 1;
      plVar21 = plVar21 + 1;
      iVar39 = local_3d4;
    } while (local_3d4 < 5);
    if (DAT_143ad7630 != 0) {
      FUN_142261810();
    }
    if (DAT_143aa84f8 != 0) {
      FUN_141e755d0(DAT_143aa84f8,0);
    }
    if (DAT_143acf0a8 != 0) {
      FUN_1425c4e50();
    }
    if (DAT_143aca128 != 0) {
      FUN_1410ebb80();
    }
    if (DAT_143ad79b0 != (longlong *)0x0) {
      (**(code **)(*DAT_143ad79b0 + 0x160))();
    }
    if ((local_368 != 0) && (DAT_143aa8518 != (longlong *)0x0)) {
      FUN_142935790();
    }
    if ((local_3f0 != '\0') && (DAT_143aa8518 != (longlong *)0x0)) {
      FUN_1428f4eb0(DAT_143aa8518,0x58f,1);
    }
    if (0 < (int)local_3fc) {
      iVar39 = (*DAT_143262db0)();
      *(int *)(plVar26 + 0x50d) = iVar39 + -2000;
      FUN_142d4dd10(plVar26,0,0);
    }
    puVar35 = local_418;
    local_458 = (uint)local_440;
  }
  else {
    FUN_142ce7120(plVar26);
  }
  _eh_vector_destructor_iterator_(alStack_68 + 2,8,3,FUN_1401a1850);
  plVar26 = local_448;
  if (local_448 != (longlong *)0x0) {
    plVar21 = local_448 + local_458;
    plVar41 = local_448;
    while (plVar41 < plVar21) {
      puVar27 = (undefined8 *)*plVar41;
      plVar41 = plVar41 + 1;
      while (puVar27 != (undefined8 *)0x0) {
        puVar3 = (undefined8 *)puVar27[1];
        (**(code **)*puVar27)(puVar27,1);
        puVar27 = puVar3;
      }
    }
    FUN_14019b4e0(plVar26);
  }
  if (local_3b0 != (char *)0x0) {
    pcVar37 = local_3b0 + (local_3a8 & 0xffffffff) * 8;
    pcVar38 = local_3b0;
    while (pcVar38 < pcVar37) {
      puVar27 = *(undefined8 **)pcVar38;
      pcVar38 = pcVar38 + 8;
      while (puVar27 != (undefined8 *)0x0) {
        puVar3 = (undefined8 *)puVar27[1];
        (**(code **)*puVar27)(puVar27,1);
        puVar27 = puVar3;
      }
    }
    FUN_14019b4e0(local_3b0);
    local_3b0 = (char *)0x0;
    local_3a8 = local_3a8 & 0xffffffff;
  }
  if (puVar35 != (undefined4 *)0x0) {
    thunk_FUN_140205820(puVar35 + -2,0);
  }
  return;
LAB_142d53df0:
  iVar39 = *(int *)(lVar23 + 0x14);
  piVar34 = (int *)(lVar23 + 0x10);
  iVar18 = *piVar34;
  local_360 = piVar34;
  uVar24 = FUN_142cbe730(DAT_143aa84a0);
  FUN_1403e8af0(iVar18);
  iVar19 = FUN_1402e9460(uVar24,iVar18,1);
  plVar26 = local_410;
  if ((iVar39 != 0) && (iVar18 != 0)) {
    if (local_3b0 != (char *)0x0) {
      for (lVar25 = *(longlong *)
                     (local_3b0 + ((ulonglong)(longlong)iVar18 % (local_3a8 & 0xffffffff)) * 8);
          lVar25 != 0; lVar25 = *(longlong *)(lVar25 + 8)) {
        if (*(int *)(lVar25 + 0x10) == iVar18) {
          iVar14 = *(int *)(lVar25 + 0x14);
          if ((int *)(lVar25 + 0x14) != (int *)0x0) goto LAB_142d53e75;
          break;
        }
      }
    }
    iVar14 = 0;
LAB_142d53e75:
    if (iVar14 < iVar19) {
      *(int *)((longlong)local_410 + 0x303c) = iVar18;
      *(int *)(local_410 + 0x608) = iVar39;
      FUN_142ce71f0(local_410,iVar18,0);
      iVar19 = FUN_140388e10(DAT_143aa8328);
      if ((iVar19 != 0) &&
         (iVar19 = FUN_1403e8af0(*(undefined4 *)((longlong)plVar26 + 0x303c)), iVar19 == 1)) {
        lVar25 = FUN_1402e3cd0(local_430,local_188,1,(int)plVar26[0x608]);
        plVar26 = (longlong *)FUN_140192f00(*(undefined8 *)(lVar25 + 8));
        local_1c0 = plVar26;
        if (plVar26 != (longlong *)0x0) {
          if (0xfffff < (ulonglong)plVar26[1]) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          plVar26[1] = plVar26[1] + 1;
          UNLOCK();
        }
        plVar26 = local_1c0;
        FUN_1401abd80(local_188);
        if (plVar26 != (longlong *)0x0) {
          pcVar37 = (char *)0x0;
          local_370 = (char *)0x0;
          cVar6 = (**(code **)(*plVar26 + 400))(plVar26);
          if (cVar6 == '\x01') {
            puVar27 = (undefined8 *)FUN_1408a9e40();
            pcVar37 = (char *)*puVar27;
            *puVar27 = 0;
            local_370 = pcVar37;
            if (local_2f0 != 0) {
              FUN_14019f2c0(local_2f0 + -0x10);
            }
LAB_142d54055:
            if ((pcVar37 != (char *)0x0) && (*pcVar37 != '\0')) {
              local_358 = 0;
              puVar27 = (undefined8 *)FUN_1408a9e40(&local_310,0xd93);
              uVar24 = FUN_14019ba10(&local_358,*puVar27,pcVar37);
              FUN_1415eca30(uVar24);
              if (local_310 != 0) {
                FUN_14019f2c0(local_310 + -0x10);
              }
              if (local_358 != 0) {
                FUN_14019f2c0(local_358 + -0x10);
              }
            }
          }
          else {
            if (cVar6 == '\x02') {
              puVar27 = (undefined8 *)FUN_1408a9e40();
              pcVar37 = (char *)*puVar27;
              *puVar27 = 0;
              local_370 = pcVar37;
              if (local_2a0 != 0) {
                FUN_14019f2c0(local_2a0 + -0x10);
              }
              goto LAB_142d54055;
            }
            if (cVar6 == '\x03') {
              puVar27 = (undefined8 *)FUN_1408a9e40();
              pcVar37 = (char *)*puVar27;
              *puVar27 = 0;
              local_370 = pcVar37;
              if (local_308 != 0) {
                FUN_14019f2c0(local_308 + -0x10);
              }
              goto LAB_142d54055;
            }
            if (cVar6 == '\x04') {
              puVar27 = (undefined8 *)FUN_1408a9e40(&local_300);
              pcVar37 = (char *)*puVar27;
              *puVar27 = 0;
              local_370 = pcVar37;
              if (local_300 != 0) {
                FUN_14019f2c0(local_300 + -0x10);
              }
              goto LAB_142d54055;
            }
          }
          plVar21 = local_420;
          piVar34 = local_360;
          if (pcVar37 != (char *)0x0) {
            FUN_14019f2c0(pcVar37 + -0x10);
            plVar21 = local_420;
            piVar34 = local_360;
          }
        }
        FUN_1408948c0(local_1c8);
        plVar26 = local_410;
      }
    }
    else if (((iVar19 < iVar14) && (iVar18 == *(int *)((longlong)local_410 + 0x303c))) &&
            (iVar39 == (int)local_410[0x608])) {
      FUN_142ce7120(local_410);
    }
    uVar42 = local_428;
    if ((((iVar14 != 0) && (iVar18 == *(int *)((longlong)plVar26 + 0x303c))) &&
        (local_390 == (int)plVar26[0x608])) && (iVar39 != (int)plVar26[0x608])) {
      if (iVar39 < 0) {
        FUN_142ce7120(plVar26);
        uVar42 = local_428;
      }
      else {
        *(int *)(plVar26 + 0x608) = iVar39;
        if (DAT_143ac8240 != 0) {
          FUN_142386c10();
          uVar42 = local_428;
        }
      }
    }
  }
  lVar23 = *(longlong *)(lVar23 + 8);
  if (lVar23 == 0) {
    plVar41 = local_448 + ((ulonglong)(longlong)*piVar34 % uVar42 & 0xffffffff);
    do {
      plVar41 = plVar41 + 1;
      plVar26 = local_410;
      if (plVar21 <= plVar41) goto LAB_142d5419d;
      lVar23 = *plVar41;
    } while (lVar23 == 0);
  }
  goto LAB_142d53df0;
}


//===========================================================
// FUN_140303530 @ 140303530   (101 bytes)
//===========================================================

longlong FUN_140303530(longlong param_1,undefined8 param_2)

{
  undefined1 uVar1;
  undefined1 local_18 [8];
  longlong *local_10;
  
  uVar1 = FUN_1406e8ae0(param_2);
  FUN_1402cc180(local_18,uVar1);
  if (local_10 == (longlong *)0x0) {
    *(undefined8 *)(param_1 + 8) = 0;
    return param_1;
  }
  (**(code **)(*local_10 + 0x358))(local_10,param_2);
  *(longlong **)(param_1 + 8) = local_10;
  return param_1;
}



//===========================================================
// FUN_140255650 @ 140255650   (63 bytes)
//===========================================================

undefined8 FUN_140255650(int param_1,int param_2)

{
  int iVar1;
  
  if ((param_2 - 2U < 3) && (0x2774 < param_1)) {
    if (param_2 == 2) {
      iVar1 = 0x2864;
    }
    else if (param_2 == 3) {
      iVar1 = 0x2b20;
    }
    else {
      if (param_2 != 4) {
        return 0;
      }
      iVar1 = 0x29f4;
    }
    if (param_1 <= iVar1) {
      return 1;
    }
  }
  return 0;
}



//===========================================================
// FUN_140255590 @ 140255590   (36 bytes)
//===========================================================

undefined8 FUN_140255590(int param_1)

{
  if (param_1 == 2) {
    return 3;
  }
  if (param_1 != 3) {
    if (param_1 != 4) {
      return 0;
    }
    return 7;
  }
  return 10;
}



//===========================================================
// FUN_140255700 @ 140255700   (44 bytes)
//===========================================================

void FUN_140255700(int param_1,int *param_2,int *param_3)

{
  *param_2 = (param_1 + -10000) / 100;
  *param_3 = (param_1 + -10000) % 100;
  *param_2 = *param_2 + -1;
  *param_3 = *param_3 + -1;
  return;
}



//===========================================================
// FUN_140255610 @ 140255610   (57 bytes)
//===========================================================

undefined8 FUN_140255610(int param_1,int param_2)

{
  int iVar1;
  
  if ((param_2 - 2U < 3) && (-1 < param_1)) {
    if (param_2 == 2) {
      iVar1 = 3;
    }
    else if (param_2 == 3) {
      iVar1 = 10;
    }
    else {
      if (param_2 != 4) {
        return 0;
      }
      iVar1 = 7;
    }
    if (param_1 < iVar1) {
      return 1;
    }
  }
  return 0;
}



//===========================================================
// FUN_142cc4430 @ 142cc4430   (55 bytes)
//===========================================================

void FUN_142cc4430(longlong param_1,undefined4 param_2)

{
  undefined4 uVar1;
  
  *(undefined4 *)(param_1 + 0x2330) = param_2;
  uVar1 = FUN_1429e3ef0();
  *(undefined4 *)(param_1 + 0x2334) = uVar1;
  uVar1 = FUN_1429e3ef0();
  FUN_142e54b20(uVar1);
  uVar1 = FUN_1429e3ef0();
  FUN_142e54f40(uVar1);
  return;
}



//===========================================================
// FUN_142cbe730 @ 142cbe730   (8 bytes)
//===========================================================

undefined8 FUN_142cbe730(longlong param_1)

{
  return *(undefined8 *)(param_1 + 0x2358);
}



//===========================================================
// FUN_1402e3cd0 @ 1402e3cd0   (336 bytes)
//===========================================================

longlong FUN_1402e3cd0(undefined8 param_1,longlong param_2,undefined4 param_3,undefined4 param_4)

{
  longlong *plVar1;
  uint uVar2;
  longlong lVar3;
  undefined8 *local_18;
  
  lVar3 = FUN_1402e3770(param_1,param_3,param_4,param_4,0);
  if (lVar3 == 0) {
    local_18 = (undefined8 *)0x0;
    uVar2 = 2;
  }
  else {
    local_18 = *(undefined8 **)(lVar3 + 8);
    if (local_18 != (undefined8 *)0x0) {
      if (0xfffff < (ulonglong)local_18[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      local_18[1] = local_18[1] + 1;
      UNLOCK();
    }
    uVar2 = 1;
  }
  *(undefined8 **)(param_2 + 8) = local_18;
  if (local_18 != (undefined8 *)0x0) {
    if (0xfffff < (ulonglong)local_18[1]) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    local_18[1] = local_18[1] + 1;
    UNLOCK();
  }
  if (((uVar2 & 2) != 0) && (uVar2 = uVar2 & 0xfffffffd, local_18 != (undefined8 *)0x0)) {
    if (0xffffe < local_18[1] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = local_18 + 1;
    lVar3 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar3 == 1) {
      (**(code **)*local_18)(local_18,1);
    }
  }
  if (((uVar2 & 1) != 0) && (local_18 != (undefined8 *)0x0)) {
    if (0xffffe < local_18[1] - 1) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = local_18 + 1;
    lVar3 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar3 == 1) {
      (**(code **)*local_18)(local_18,1);
    }
  }
  return param_2;
}



//===========================================================
// FUN_1402e4c20 @ 1402e4c20   (480 bytes)
//===========================================================

undefined8 FUN_1402e4c20(longlong param_1,int param_2,int param_3,undefined8 param_4)

{
  longlong lVar1;
  int iVar2;
  undefined8 uVar3;
  longlong *plVar4;
  int iVar5;
  undefined1 local_18 [16];
  
  if (param_2 - 1U < 6) {
    if (param_2 == 1) {
      if (param_3 < 0) {
        iVar5 = -param_3;
        iVar2 = FUN_140302620(iVar5);
        if (iVar2 != 0) {
          uVar3 = FUN_140232590(local_18,param_4);
          FUN_1402de550(param_1 + 0x5a8,uVar3,iVar5);
          FUN_1401abd80(param_4);
          return 1;
        }
        if (0x1e < param_3 + 0x1fU) {
          FUN_1401abd80(param_4);
          return 0;
        }
        FUN_1401e8780(param_1 + 0x1a8 + (longlong)iVar5 * 0x10,param_4);
        FUN_1401abd80(param_4);
        return 1;
      }
    }
    else if ((param_2 == 6) && (param_3 < 0)) {
      if ((0xd < -param_3 - 0x4b0U) && (0x32 < -param_3 - 0x708U)) {
        if (0x1e < param_3 + 0x83U) {
          FUN_1401abd80(param_4);
          return 0;
        }
        FUN_1401e8780((longlong)(-100 - param_3) * 0x10 + 0x3a8 + param_1,param_4);
        FUN_1401abd80(param_4);
        return 1;
      }
      uVar3 = FUN_140232590(local_18,param_4);
      FUN_1402de550(param_1 + 0x5a8,uVar3,-param_3);
      FUN_1401abd80(param_4);
      return 1;
    }
    if (0 < param_3) {
      plVar4 = (longlong *)((longlong)param_2 * 8 + 0x5d0 + param_1);
      lVar1 = *plVar4;
      if ((lVar1 != 0) && (param_3 <= *(int *)(lVar1 + -8) + -1)) {
        uVar3 = FUN_1402f1620(plVar4,param_3);
        FUN_1401e8780(uVar3,param_4);
        FUN_1401abd80(param_4);
        return 1;
      }
    }
    FUN_1401abd80(param_4);
  }
  else {
    FUN_1401abd80(param_4);
  }
  return 0;
}



//===========================================================
// FUN_14022f570 @ 14022f570   (259 bytes)
//===========================================================

longlong FUN_14022f570(longlong param_1,undefined8 param_2,undefined4 param_3,undefined4 param_4)

{
  int iVar1;
  undefined4 local_28;
  int local_24;
  undefined1 local_20 [8];
  longlong local_18;
  
  iVar1 = FUN_140255650(param_3,param_4);
  if (iVar1 != 0) {
    local_28 = 0;
    local_24 = 0;
    FUN_140255700(param_3,&local_28,&local_24,param_4);
    iVar1 = FUN_140255610(local_28,param_4);
    if (((iVar1 != 0) && (iVar1 = FUN_1402556a0(local_24), iVar1 != 0)) &&
       (iVar1 = FUN_14022f410(param_2,local_28,param_4,0), iVar1 != 0)) {
      FUN_1402e4e10(param_2,local_20,param_4,local_28);
      if (local_18 == 0) {
        *(undefined8 *)(param_1 + 8) = 0;
        FUN_1401a1880(local_20);
        return param_1;
      }
      FUN_140232590(param_1,(longlong)local_24 * 0x10 + local_18 + 4);
      FUN_1401a1880(local_20);
      return param_1;
    }
  }
  *(undefined8 *)(param_1 + 8) = 0;
  return param_1;
}



//===========================================================
// FUN_14022f680 @ 14022f680   (408 bytes)
//===========================================================

undefined8 FUN_14022f680(undefined8 param_1,undefined4 param_2,longlong param_3,undefined4 param_4)

{
  longlong lVar1;
  int iVar2;
  longlong lVar3;
  undefined4 local_28;
  int local_24;
  undefined1 local_20 [8];
  longlong local_18;
  
  iVar2 = FUN_140255650(param_2,param_4);
  if (iVar2 == 0) {
    FUN_1401abd80(param_3);
  }
  else {
    local_28 = 0;
    local_24 = 0;
    FUN_140255700(param_2,&local_28,&local_24,param_4);
    iVar2 = FUN_140255610(local_28,param_4);
    if ((iVar2 == 0) || (iVar2 = FUN_1402556a0(local_24), iVar2 == 0)) {
      FUN_1401abd80(param_3);
    }
    else {
      iVar2 = FUN_14022f410(param_1,local_28,param_4,0);
      if (iVar2 == 0) {
        FUN_1401abd80(param_3);
      }
      else {
        FUN_1402e4e10(param_1,local_20,param_4,local_28);
        if (local_18 != 0) {
          lVar3 = FUN_140232740(local_20);
          lVar3 = lVar3 + 4 + (longlong)local_24 * 0x10;
          if ((*(longlong *)(lVar3 + 8) - 1U < 999) || (*(longlong *)(lVar3 + 8) == -1)) {
            FUN_142e52ed0(0x447);
          }
          if (lVar3 == param_3) {
            FUN_142e52d50(0x45c,1);
          }
          lVar1 = *(longlong *)(param_3 + 8);
          if (lVar1 != 0) {
            if (0xfffff < *(ulonglong *)(lVar1 + 8)) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            *(longlong *)(lVar1 + 8) = *(longlong *)(lVar1 + 8) + 1;
            UNLOCK();
          }
          FUN_1401abd80(lVar3);
          *(undefined8 *)(lVar3 + 8) = *(undefined8 *)(param_3 + 8);
          FUN_1401a1880(local_20);
          FUN_1401abd80(param_3);
          return 1;
        }
        FUN_1401a1880(local_20);
        FUN_1401abd80(param_3);
      }
    }
  }
  return 0;
}



//===========================================================
// FUN_1402e5020 @ 1402e5020   (105 bytes)
//===========================================================

void FUN_1402e5020(longlong param_1,int param_2,uint param_3,undefined8 param_4)

{
  if (param_2 == 2) {
    if (2 < param_3) goto LAB_1402e507a;
    param_1 = param_1 + 0x608;
  }
  else if (param_2 == 3) {
    if (9 < param_3) goto LAB_1402e507a;
    param_1 = param_1 + 0x638;
  }
  else {
    if ((param_2 != 4) || (6 < param_3)) goto LAB_1402e507a;
    param_1 = param_1 + 0x6d8;
  }
  FUN_1402fa360(param_1 + (longlong)(int)param_3 * 0x10,param_4);
LAB_1402e507a:
  FUN_140301bb0(param_4);
  return;
}



//===========================================================
// FUN_1403ebca0 @ 1403ebca0   (1622 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1403ebca0(longlong param_1,ulonglong param_2,longlong *param_3)

{
  undefined8 *puVar1;
  longlong *plVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  longlong lVar6;
  uint uVar7;
  longlong lVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong lVar11;
  ulonglong *puVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  longlong *plVar15;
  undefined1 auStack_98 [32];
  int local_78;
  longlong local_70;
  undefined1 local_68 [8];
  longlong local_60;
  longlong local_58;
  longlong local_50;
  longlong *local_48;
  ulonglong local_40;
  
  local_40 = DAT_143a8b908 ^ (ulonglong)auStack_98;
  local_50 = DAT_143aa8328;
  local_70 = param_1;
  local_48 = param_3;
  if (DAT_143aa8328 == 0) {
    plVar15 = (longlong *)param_3[7];
    if (plVar15 == (longlong *)0x0) {
      return;
    }
    (**(code **)(*plVar15 + 0x20))(plVar15,plVar15 != param_3);
  }
  else {
    uVar14 = 0;
    if ((param_2 & 1) != 0) {
      for (plVar15 = (longlong *)(param_1 + 0x5d0); plVar15 != (longlong *)(param_1 + 0x608);
          plVar15 = plVar15 + 1) {
        uVar7 = 1;
        uVar3 = FUN_140232a80(plVar15);
        if (1 < uVar3) {
          lVar11 = 0x10;
          do {
            lVar8 = *plVar15;
            uVar3 = 0;
            if (lVar8 != 0) {
              uVar3 = *(uint *)(lVar8 + -8);
            }
            if (((int)uVar7 < 0) || (uVar3 <= uVar7)) {
              uVar10 = uVar14;
              if (lVar8 != 0) {
                uVar10 = (ulonglong)*(uint *)(lVar8 + -8);
              }
              FUN_142e54290(0xc6,uVar7,uVar10);
              lVar8 = *plVar15;
            }
            if (*(longlong *)(lVar11 + 8 + lVar8) != 0) {
              if ((longlong *)param_3[7] == (longlong *)0x0) goto LAB_1403ec2ec;
              (**(code **)(*(longlong *)param_3[7] + 0x10))();
            }
            uVar7 = uVar7 + 1;
            lVar11 = lVar11 + 0x10;
            uVar3 = FUN_140232a80(plVar15);
          } while (uVar7 < uVar3);
        }
      }
    }
    lVar11 = local_70;
    if ((param_2 & 2) != 0) {
      lVar6 = local_70 + 0x3a8;
      for (lVar8 = local_70 + 0x1a8; lVar8 != lVar6; lVar8 = lVar8 + 0x10) {
        if (*(longlong *)(lVar8 + 8) != 0) {
          if ((longlong *)param_3[7] == (longlong *)0x0) goto LAB_1403ec2ec;
          (**(code **)(*(longlong *)param_3[7] + 0x10))();
        }
      }
      puVar12 = (ulonglong *)(lVar11 + 0x5a8);
      uVar10 = uVar14;
      do {
        iVar4 = FUN_1402557d0(uVar10);
        if ((((iVar4 == 0) && (iVar4 = FUN_140255790(uVar10), iVar4 == 0)) &&
            (uVar9 = *puVar12, uVar9 != 0)) && (*(int *)(uVar9 - 8) != 0)) {
          do {
            if (*(longlong *)(uVar9 + 8) != 0) {
              if ((longlong *)param_3[7] == (longlong *)0x0) goto LAB_1403ec2ec;
              (**(code **)(*(longlong *)param_3[7] + 0x10))();
            }
          } while ((uVar9 < *(longlong *)(*puVar12 - 8) * 0x10 + -0x10 + *puVar12) &&
                  (uVar9 = uVar9 + 0x10, uVar9 != 0));
        }
        uVar3 = (int)uVar10 + 1;
        uVar10 = (ulonglong)uVar3;
        puVar12 = puVar12 + 1;
      } while ((int)uVar3 < 5);
    }
    if ((param_2 & 4) != 0) {
      puVar12 = (ulonglong *)(lVar11 + 0x5a8);
      uVar10 = uVar14;
      do {
        iVar4 = FUN_1402557d0(uVar10);
        if (((iVar4 != 0) && (iVar4 = FUN_140255790(uVar10), iVar4 == 0)) &&
           ((uVar9 = *puVar12, uVar9 != 0 && (*(int *)(uVar9 - 8) != 0)))) {
          do {
            if (*(longlong *)(uVar9 + 8) != 0) {
              if ((longlong *)param_3[7] == (longlong *)0x0) goto LAB_1403ec2ec;
              (**(code **)(*(longlong *)param_3[7] + 0x10))();
            }
          } while ((uVar9 < *(longlong *)(*puVar12 - 8) * 0x10 + -0x10 + *puVar12) &&
                  (uVar9 = uVar9 + 0x10, uVar9 != 0));
        }
        uVar3 = (int)uVar10 + 1;
        uVar10 = (ulonglong)uVar3;
        puVar12 = puVar12 + 1;
      } while ((int)uVar3 < 5);
    }
    if ((param_2 & 0x10) != 0) {
      for (lVar8 = lVar11 + 0x3a8; lVar8 != lVar11 + 0x5a8; lVar8 = lVar8 + 0x10) {
        if (*(longlong *)(lVar8 + 8) != 0) {
          if ((longlong *)param_3[7] == (longlong *)0x0) {
LAB_1403ec2ec:
                    /* WARNING: Subroutine does not return */
            FUN_142ed3024();
          }
          (**(code **)(*(longlong *)param_3[7] + 0x10))();
        }
      }
      puVar12 = (ulonglong *)(lVar11 + 0x5a8);
      uVar10 = uVar14;
      do {
        iVar4 = FUN_1402557b0(uVar10);
        if ((((iVar4 == 0) && (iVar4 = FUN_1402557d0(uVar10), iVar4 == 0)) &&
            (iVar4 = FUN_140255790(uVar10), iVar4 != 0)) &&
           ((uVar9 = *puVar12, uVar9 != 0 && (*(int *)(uVar9 - 8) != 0)))) {
          do {
            if (*(longlong *)(uVar9 + 8) != 0) {
              if ((longlong *)param_3[7] == (longlong *)0x0) goto LAB_1403ec2ec;
              (**(code **)(*(longlong *)param_3[7] + 0x10))();
            }
          } while ((uVar9 < *(longlong *)(*puVar12 - 8) * 0x10 + -0x10 + *puVar12) &&
                  (uVar9 = uVar9 + 0x10, uVar9 != 0));
        }
        uVar3 = (int)uVar10 + 1;
        uVar10 = (ulonglong)uVar3;
        puVar12 = puVar12 + 1;
      } while ((int)uVar3 < 5);
    }
    if ((param_2 & 8) != 0) {
      local_78 = 2;
      plVar15 = (longlong *)(lVar11 + 0x5e0);
      do {
        iVar4 = local_78;
        uVar7 = 1;
        uVar3 = FUN_140232a80(plVar15);
        if (1 < uVar3) {
          lVar11 = 0x10;
          local_58 = 0x10;
          do {
            lVar8 = *plVar15;
            uVar3 = 0;
            if (lVar8 != 0) {
              uVar3 = *(uint *)(lVar8 + -8);
            }
            if (((int)uVar7 < 0) || (uVar3 <= uVar7)) {
              uVar10 = uVar14;
              if (lVar8 != 0) {
                uVar10 = (ulonglong)*(uint *)(lVar8 + -8);
              }
              FUN_142e54290(0xc6,uVar7,uVar10);
              lVar8 = *plVar15;
            }
            lVar11 = lVar11 + lVar8;
            if ((*(longlong *)(lVar11 + 8) != 0) &&
               (((iVar5 = FUN_14019a5d0(*(longlong *)(lVar11 + 8) + 0x20), iVar5 - 0x286f90U < 10000
                 || (iVar5 - 0x2eff40U < 10000)) || (iVar5 - 0x421210U < 10000)))) {
              lVar8 = *(longlong *)(lVar11 + 8);
              if (lVar8 == 0) {
                FUN_142e52ed0(0x431,0);
                lVar8 = *(longlong *)(lVar11 + 8);
              }
              iVar5 = FUN_1401b0340(lVar8 + 0x20);
              iVar4 = local_78;
              if ((((iVar5 - 0x286f90U < 10000) || (iVar5 - 0x2eff40U < 10000)) ||
                  (iVar5 - 0x421210U < 10000)) && (-1 < *(int *)(lVar8 + 0x48))) {
                iVar4 = FUN_1401b0340(lVar8 + 0x20);
                iVar5 = FUN_140255590(iVar4 / 1000000);
                iVar4 = local_78;
                if (*(int *)(lVar8 + 0x48) < iVar5) {
                  lVar8 = *(longlong *)(lVar11 + 8);
                  if (lVar8 == 0) {
                    FUN_142e52ed0(0x431,0);
                    lVar8 = *(longlong *)(lVar11 + 8);
                  }
                  iVar4 = local_78;
                  iVar5 = FUN_140255610(*(undefined4 *)(lVar8 + 0x48));
                  if ((iVar5 != 0) && (lVar8 = FUN_1403a64e0(local_50), lVar8 != 0)) {
                    lVar6 = *(longlong *)(lVar11 + 8);
                    if (lVar6 == 0) {
                      FUN_142e52ed0(0x431,0);
                      lVar6 = *(longlong *)(lVar11 + 8);
                    }
                    FUN_1402e4e10(local_70,local_68,iVar4,*(undefined4 *)(lVar6 + 0x48));
                    if (local_60 != 0) {
                      lVar11 = local_60;
                      uVar10 = uVar14;
                      uVar9 = uVar14;
                      uVar13 = uVar14;
                      if (0 < *(int *)(lVar8 + 4)) {
                        do {
                          if (lVar11 == 0) {
                            FUN_142e52ed0(0x431,0);
                            lVar11 = local_60;
                          }
                          if (uVar10 < 0x28) {
                            if (lVar11 == 0) {
                              FUN_142e52ed0(0x431,0);
                              lVar11 = local_60;
                            }
                            if (*(longlong *)(uVar13 + 0xc + lVar11) != 0) {
                              if ((longlong *)param_3[7] == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
                                FUN_142ed3024();
                              }
                              (**(code **)(*(longlong *)param_3[7] + 0x10))();
                              lVar11 = local_60;
                            }
                          }
                          uVar3 = (int)uVar9 + 1;
                          uVar10 = uVar10 + 1;
                          uVar9 = (ulonglong)uVar3;
                          uVar13 = uVar13 + 0x10;
                          iVar4 = local_78;
                        } while ((int)uVar3 < *(int *)(lVar8 + 4));
                      }
                      if (lVar11 != 0) {
                        puVar1 = (undefined8 *)(lVar11 + -0x28);
                        if (0xffffe < *(longlong *)(lVar11 + -0x20) - 1U) {
                          FUN_142e541f0(0x31e);
                        }
                        LOCK();
                        plVar2 = (longlong *)(lVar11 + -0x20);
                        lVar11 = *plVar2;
                        *plVar2 = *plVar2 + -1;
                        UNLOCK();
                        if ((int)lVar11 == 1) {
                          if ((local_60 != 0) && (*(longlong *)(local_60 + -0x10) != 0)) {
                            LOCK();
                            *(undefined8 *)(*(longlong *)(local_60 + -0x10) + 8) = 0;
                            UNLOCK();
                            do {
                            } while (*(int *)(*(longlong *)(local_60 + -0x10) + 4) != 0);
                          }
                          if (puVar1 != (undefined8 *)0x0) {
                            (**(code **)*puVar1)(puVar1);
                          }
                        }
                        local_60 = 0;
                      }
                    }
                  }
                }
              }
            }
            uVar7 = uVar7 + 1;
            lVar11 = local_58 + 0x10;
            local_58 = lVar11;
            uVar3 = FUN_140232a80(plVar15);
          } while (uVar7 < uVar3);
        }
        local_78 = iVar4 + 1;
        plVar15 = plVar15 + 1;
      } while (local_78 < 5);
    }
    plVar15 = (longlong *)param_3[7];
    if (plVar15 == (longlong *)0x0) {
      return;
    }
    (**(code **)(*plVar15 + 0x20))(plVar15,plVar15 != param_3);
  }
  param_3[7] = 0;
  return;
}



//===========================================================
// FUN_1402cc180 @ 1402cc180   (229 bytes)
//===========================================================

longlong FUN_1402cc180(longlong param_1,int param_2)

{
  longlong lVar1;
  
  if (param_2 == 1) {
    lVar1 = FUN_14019b780(&DAT_143ad68a0,0x467);
    if (lVar1 == 0) {
      lVar1 = 0;
    }
    else {
      lVar1 = FUN_1402f7da0(lVar1);
    }
  }
  else if (param_2 == 2) {
    lVar1 = FUN_14019b780(&DAT_143ad68a0,0x7e);
    if (lVar1 == 0) {
      lVar1 = 0;
    }
    else {
      lVar1 = FUN_1402f7cd0(lVar1);
    }
  }
  else {
    if (param_2 != 3) {
      *(undefined8 *)(param_1 + 8) = 0;
      return param_1;
    }
    lVar1 = FUN_14019b780(&DAT_143ad68a0,0xce);
    if (lVar1 == 0) {
      lVar1 = 0;
    }
    else {
      lVar1 = FUN_1402f8b40(lVar1);
    }
  }
  *(longlong *)(param_1 + 8) = lVar1;
  if (lVar1 != 0) {
    if (0xfffff < *(ulonglong *)(lVar1 + 8)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar1 + 8) = *(longlong *)(lVar1 + 8) + 1;
    UNLOCK();
  }
  return param_1;
}



//===========================================================
// FUN_1402f7da0 @ 1402f7da0   (524 bytes)
//===========================================================

undefined8 * FUN_1402f7da0(undefined8 *param_1)

{
  byte bVar1;
  
  FUN_1402f7aa0();
  *param_1 = &PTR_FUN_14327e1d8;
  FUN_1402f7fb0((longlong)param_1 + 0x62);
  *(undefined8 *)((longlong)param_1 + 0x1ea) = 0;
  *(undefined8 *)((longlong)param_1 + 0x1da) = 0;
  *(undefined8 *)((longlong)param_1 + 0x1e2) = 0;
  *(undefined ***)((longlong)param_1 + 0x1d2) = &PTR_FUN_14327e1c8;
  *(undefined8 *)((longlong)param_1 + 0x1f2) = 0;
  *(undefined8 *)((longlong)param_1 + 0x206) = 0;
  *(undefined4 *)((longlong)param_1 + 0x20e) = 0;
  *(undefined8 *)((longlong)param_1 + 0x1fa) = DAT_14327dd80;
  *(undefined4 *)((longlong)param_1 + 0x202) = 0;
  *(undefined8 *)((longlong)param_1 + 0x22a) = 0;
  *(undefined8 *)((longlong)param_1 + 0x21a) = 0;
  *(undefined8 *)((longlong)param_1 + 0x222) = 0;
  *(undefined ***)((longlong)param_1 + 0x212) = &PTR_FUN_14327e1d0;
  *(undefined8 *)((longlong)param_1 + 0x232) = 0;
  *(undefined8 *)((longlong)param_1 + 0x23a) = 0;
  *(undefined8 *)((longlong)param_1 + 0x242) = 0;
  *(undefined8 *)((longlong)param_1 + 0x24a) = 0;
  *(undefined8 *)((longlong)param_1 + 0x252) = 0;
  *(undefined4 *)((longlong)param_1 + 0x25a) = 0;
  *(undefined1 *)((longlong)param_1 + 0x25e) = 0;
  *(undefined4 *)((longlong)param_1 + 0x25f) = 0;
  *(undefined8 *)((longlong)param_1 + 0x263) = DAT_14327dd80;
  *(undefined8 *)((longlong)param_1 + 0x242) = 0;
  *(undefined8 *)((longlong)param_1 + 0x24a) = 0;
  *(undefined1 *)((longlong)param_1 + 0x252) = 0;
  *(undefined8 *)((longlong)param_1 + 0x263) = DAT_14327dd80;
  FUN_1402f8a00((longlong)param_1 + 0x26b);
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x303) = bVar1;
  *(byte *)((longlong)param_1 + 0x304) = bVar1;
  *(uint *)((longlong)param_1 + 0x307) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x30b) = bVar1;
  *(byte *)((longlong)param_1 + 0x30c) = bVar1;
  *(uint *)((longlong)param_1 + 0x30f) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x303) = bVar1;
  *(byte *)((longlong)param_1 + 0x304) = bVar1;
  *(uint *)((longlong)param_1 + 0x307) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x30b) = bVar1;
  *(byte *)((longlong)param_1 + 0x30c) = bVar1;
  *(uint *)((longlong)param_1 + 0x30f) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  FUN_1402fb7e0((longlong)param_1 + 0x26b);
  FUN_1402f8a00((longlong)param_1 + 0x313);
  *(undefined4 *)((longlong)param_1 + 0x3ab) = 0;
  FUN_1402f8710((longlong)param_1 + 0x3af);
  *(undefined8 *)((longlong)param_1 + 0x4d) = 0;
  *(undefined1 *)((longlong)param_1 + 0x55) = 0;
  return param_1;
}



//===========================================================
// FUN_1402f7cd0 @ 1402f7cd0   (198 bytes)
//===========================================================

undefined8 * FUN_1402f7cd0(undefined8 *param_1)

{
  byte bVar1;
  undefined4 uVar2;
  
  FUN_1402f7aa0();
  *param_1 = &PTR_FUN_14327e588;
  uVar2 = FUN_1402f70a0(0,(longlong)param_1 + 0x4d);
  *(undefined4 *)((longlong)param_1 + 0x51) = uVar2;
  uVar2 = FUN_1402f7010(0,(longlong)param_1 + 0x55);
  *(undefined4 *)((longlong)param_1 + 0x59) = uVar2;
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x5d) = bVar1;
  *(byte *)((longlong)param_1 + 0x5e) = bVar1;
  *(uint *)((longlong)param_1 + 0x61) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  uVar2 = FUN_1402f70a0(0,(longlong)param_1 + 0x4d);
  *(undefined4 *)((longlong)param_1 + 0x51) = uVar2;
  uVar2 = FUN_1402f7010(0,(longlong)param_1 + 0x55);
  *(undefined4 *)((longlong)param_1 + 0x59) = uVar2;
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x5d) = bVar1;
  *(byte *)((longlong)param_1 + 0x5e) = bVar1;
  *(uint *)((longlong)param_1 + 0x61) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  *(undefined8 *)((longlong)param_1 + 0x65) = 0;
  *(undefined1 *)((longlong)param_1 + 0x6d) = 0;
  *(undefined4 *)((longlong)param_1 + 0x7a) = 0;
  return param_1;
}



//===========================================================
// FUN_1402f8b40 @ 1402f8b40   (915 bytes)
//===========================================================

undefined8 * FUN_1402f8b40(undefined8 *param_1)

{
  byte bVar1;
  undefined4 uVar2;
  uint uVar3;
  uint uVar4;
  
  FUN_1402f7aa0();
  *param_1 = &PTR_FUN_14327e910;
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x5a) = bVar1;
  *(byte *)((longlong)param_1 + 0x5b) = bVar1;
  *(uint *)((longlong)param_1 + 0x5e) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  uVar2 = FUN_1402f7010(0,(longlong)param_1 + 0x62);
  *(undefined4 *)((longlong)param_1 + 0x66) = uVar2;
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x6a) = bVar1;
  *(byte *)((longlong)param_1 + 0x6b) = bVar1;
  *(uint *)((longlong)param_1 + 0x6e) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  uVar2 = FUN_1402f7010(0,(longlong)param_1 + 0x72);
  *(undefined4 *)((longlong)param_1 + 0x76) = uVar2;
  uVar2 = FUN_1402f70a0(0,(longlong)param_1 + 0x7a);
  *(undefined4 *)((longlong)param_1 + 0x7e) = uVar2;
  uVar3 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0x8a) = uVar3;
  uVar4 = uVar3 >> 5 | uVar3 << 0x1b;
  *(uint *)((longlong)param_1 + 0x8e) = uVar4;
  *(uint *)((longlong)param_1 + 0x92) =
       ((uVar3 ^ 0xbaadf00d) >> 5 | (uVar3 ^ 0xbaadf00d) << 0x1b) + uVar4;
  uVar2 = FUN_1402f7010(0,(longlong)param_1 + 0x96);
  *(undefined4 *)((longlong)param_1 + 0x9a) = uVar2;
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x9e) = bVar1;
  *(byte *)((longlong)param_1 + 0x9f) = bVar1;
  *(uint *)((longlong)param_1 + 0xa2) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  uVar3 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0xa6) = uVar3;
  uVar4 = uVar3 >> 5 | uVar3 << 0x1b;
  *(uint *)((longlong)param_1 + 0xaa) = uVar4;
  *(uint *)((longlong)param_1 + 0xae) =
       ((uVar3 ^ 0xbaadf00d) >> 5 | (uVar3 ^ 0xbaadf00d) << 0x1b) + uVar4;
  uVar2 = FUN_1402f7010(0,(longlong)param_1 + 0xb2);
  *(undefined4 *)((longlong)param_1 + 0xb6) = uVar2;
  uVar2 = FUN_1402f7010(0,(longlong)param_1 + 0xba);
  *(undefined4 *)((longlong)param_1 + 0xbe) = uVar2;
  uVar3 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0xc2) = uVar3;
  uVar4 = uVar3 >> 5 | uVar3 << 0x1b;
  *(uint *)((longlong)param_1 + 0xc6) = uVar4;
  *(uint *)((longlong)param_1 + 0xca) =
       ((uVar3 ^ 0xbaadf00d) >> 5 | (uVar3 ^ 0xbaadf00d) << 0x1b) + uVar4;
  *(undefined1 *)((longlong)param_1 + 0x4d) = 0;
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x5a) = bVar1;
  *(byte *)((longlong)param_1 + 0x5b) = bVar1;
  *(uint *)((longlong)param_1 + 0x5e) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  uVar2 = FUN_1402f7010(0,(longlong)param_1 + 0x62);
  *(undefined4 *)((longlong)param_1 + 0x66) = uVar2;
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x6a) = bVar1;
  *(byte *)((longlong)param_1 + 0x6b) = bVar1;
  *(uint *)((longlong)param_1 + 0x6e) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  uVar2 = FUN_1402f7010(0,(longlong)param_1 + 0x72);
  *(undefined4 *)((longlong)param_1 + 0x76) = uVar2;
  uVar2 = FUN_1402f70a0(0,(longlong)param_1 + 0x7a);
  *(undefined4 *)((longlong)param_1 + 0x7e) = uVar2;
  *(undefined8 *)((longlong)param_1 + 0x82) = DAT_14327dd80;
  uVar3 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0x8a) = uVar3;
  uVar4 = uVar3 >> 5 | uVar3 << 0x1b;
  *(uint *)((longlong)param_1 + 0x8e) = uVar4;
  *(uint *)((longlong)param_1 + 0x92) =
       ((uVar3 ^ 0xbaadf00d) >> 5 | (uVar3 ^ 0xbaadf00d) << 0x1b) + uVar4;
  uVar2 = FUN_1402f7010(0,(longlong)param_1 + 0x96);
  *(undefined4 *)((longlong)param_1 + 0x9a) = uVar2;
  bVar1 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)((longlong)param_1 + 0x9e) = bVar1;
  *(byte *)((longlong)param_1 + 0x9f) = bVar1;
  *(uint *)((longlong)param_1 + 0xa2) =
       ((bVar1 ^ 0xbaadf00d) >> 5 | (bVar1 ^ 0xbaadf00d) << 0x1b) + (uint)bVar1;
  uVar3 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0xa6) = uVar3;
  uVar4 = ~uVar3 >> 5 | ~uVar3 << 0x1b;
  *(uint *)((longlong)param_1 + 0xaa) = uVar4;
  *(uint *)((longlong)param_1 + 0xae) =
       ((uVar3 ^ 0xbaadf00d) >> 5 | (uVar3 ^ 0xbaadf00d) << 0x1b) + uVar4;
  uVar2 = FUN_1402f7010(100,(longlong)param_1 + 0xb2);
  *(undefined4 *)((longlong)param_1 + 0xb6) = uVar2;
  uVar2 = FUN_1402f7010(0,(longlong)param_1 + 0xba);
  *(undefined4 *)((longlong)param_1 + 0xbe) = uVar2;
  uVar3 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)((longlong)param_1 + 0xc2) = uVar3;
  uVar4 = uVar3 >> 5 | uVar3 << 0x1b;
  *(uint *)((longlong)param_1 + 0xc6) = uVar4;
  *(uint *)((longlong)param_1 + 0xca) =
       ((uVar3 ^ 0xbaadf00d) >> 5 | (uVar3 ^ 0xbaadf00d) << 0x1b) + uVar4;
  return param_1;
}



//===========================================================
// FUN_140304100 @ 140304100   (828 bytes)
//===========================================================

void FUN_140304100(longlong param_1,undefined8 param_2)

{
  byte bVar1;
  byte bVar2;
  char cVar3;
  undefined2 uVar4;
  undefined4 uVar5;
  int iVar6;
  
  FUN_1403035a0();
  FUN_140303b40(param_1 + 0x62,param_2);
  FUN_1406e9170(param_2,param_1 + 0x55,0xd);
  *(undefined1 *)(param_1 + 0x61) = 0;
  bVar1 = FUN_1406e8ae0(param_2);
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x3af) = bVar2;
  *(byte *)(param_1 + 0x3b0) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0x3b3) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  bVar1 = FUN_1406e8ae0(param_2);
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x3b7) = bVar2;
  *(byte *)(param_1 + 0x3b8) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0x3bb) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 0x3bf);
  *(undefined4 *)(param_1 + 0x3c3) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 0x3c7);
  *(undefined4 *)(param_1 + 0x3cb) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 0x3cf);
  *(undefined4 *)(param_1 + 0x3d3) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 0x3d7);
  *(undefined4 *)(param_1 + 0x3db) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 999);
  *(undefined4 *)(param_1 + 0x3eb) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 0x3ef);
  *(undefined4 *)(param_1 + 0x3f3) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 0x3df);
  *(undefined4 *)(param_1 + 0x3e3) = uVar5;
  if (*(longlong *)(param_1 + 0x38) == 0) {
    FUN_1406e9170(param_2,param_1 + 0x4d,8);
  }
  else {
    *(undefined8 *)(param_1 + 0x4d) = 0;
  }
  FUN_1402cce00(param_2,param_1 + 0x1d2);
  FUN_1402cd090(param_2,param_1 + 0x212);
  uVar5 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x23e) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f7010(uVar4,param_1 + 0x3f7);
  *(undefined4 *)(param_1 + 0x3fb) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f7010(uVar4,param_1 + 0x3ff);
  *(undefined4 *)(param_1 + 0x403) = uVar5;
  uVar4 = FUN_1406e8b80(param_2);
  uVar5 = FUN_1402f70a0(uVar4,param_1 + 0x407);
  *(undefined4 *)(param_1 + 0x40b) = uVar5;
  iVar6 = FUN_14019a5d0(param_1 + 0x20);
  if (iVar6 / 10000 == 0xa6) {
    FUN_1402cb4f0(param_1 + 0x242,param_2);
  }
  bVar1 = FUN_1406e8ae0(param_2);
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x303) = bVar2;
  *(byte *)(param_1 + 0x304) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0x307) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  bVar1 = FUN_1406e8ae0(param_2);
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x30b) = bVar2;
  *(byte *)(param_1 + 0x30c) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0x30f) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  FUN_140303800(param_1 + 0x26b,param_2);
  cVar3 = FUN_1406e8ae0(param_2);
  if (cVar3 == '\0') {
    *(undefined4 *)(param_1 + 0x3ab) = 0;
    FUN_1402fb7e0(param_1 + 0x313);
  }
  else {
    FUN_140303800(param_1 + 0x313,param_2);
  }
  return;
}



//===========================================================
// FUN_140304450 @ 140304450   (238 bytes)
//===========================================================

void FUN_140304450(longlong param_1,undefined8 param_2)

{
  byte bVar1;
  byte bVar2;
  undefined2 uVar3;
  undefined4 uVar4;
  int iVar5;
  
  FUN_1403035a0();
  uVar3 = FUN_1406e8b80(param_2);
  uVar4 = FUN_1402f70a0(uVar3,param_1 + 0x4d);
  *(undefined4 *)(param_1 + 0x51) = uVar4;
  FUN_1406e9170(param_2,param_1 + 0x6d,0xd);
  *(undefined1 *)(param_1 + 0x79) = 0;
  uVar3 = FUN_1406e8b80(param_2);
  uVar4 = FUN_1402f7010(uVar3,param_1 + 0x55);
  *(undefined4 *)(param_1 + 0x59) = uVar4;
  bVar1 = FUN_1406e8ae0(param_2);
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x5d) = bVar2;
  *(byte *)(param_1 + 0x5e) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0x61) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  iVar5 = FUN_14019a5d0(param_1 + 0x20);
  if ((iVar5 - 0x1f95f0U < 10000) || (iVar5 - 0x238d90U < 10000)) {
    FUN_1406e9170(param_2,param_1 + 0x65,8);
  }
  else {
    *(undefined8 *)(param_1 + 0x65) = 0;
  }
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x7a) = uVar4;
  return;
}



//===========================================================
// FUN_140304550 @ 140304550   (579 bytes)
//===========================================================

void FUN_140304550(longlong param_1,undefined8 param_2)

{
  byte bVar1;
  byte bVar2;
  undefined2 uVar3;
  undefined4 uVar4;
  uint uVar5;
  uint uVar6;
  
  FUN_1403035a0();
  FUN_1406e9170(param_2,param_1 + 0x4d,0xd);
  bVar1 = FUN_1406e8ae0(param_2);
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x5a) = bVar2;
  *(byte *)(param_1 + 0x5b) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0x5e) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  uVar3 = FUN_1406e8b80(param_2);
  uVar4 = FUN_1402f7010(uVar3,param_1 + 0x62);
  *(undefined4 *)(param_1 + 0x66) = uVar4;
  bVar1 = FUN_1406e8ae0(param_2);
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x6a) = bVar2;
  *(byte *)(param_1 + 0x6b) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0x6e) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  FUN_1406e9170(param_2,param_1 + 0x82,8);
  uVar3 = FUN_1406e8b80(param_2);
  uVar4 = FUN_1402f7010(uVar3,param_1 + 0x72);
  *(undefined4 *)(param_1 + 0x76) = uVar4;
  uVar3 = FUN_1406e8b80(param_2);
  uVar4 = FUN_1402f70a0(uVar3,param_1 + 0x7a);
  *(undefined4 *)(param_1 + 0x7e) = uVar4;
  uVar5 = FUN_1406e8c20(param_2);
  uVar6 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)(param_1 + 0x8a) = uVar6;
  uVar5 = (uVar6 ^ uVar5) >> 5 | (uVar6 ^ uVar5) << 0x1b;
  *(uint *)(param_1 + 0x8e) = uVar5;
  *(uint *)(param_1 + 0x92) = ((uVar6 ^ 0xbaadf00d) >> 5 | (uVar6 ^ 0xbaadf00d) << 0x1b) + uVar5;
  uVar3 = FUN_1406e8b80(param_2);
  uVar4 = FUN_1402f7010(uVar3,param_1 + 0x96);
  *(undefined4 *)(param_1 + 0x9a) = uVar4;
  bVar1 = FUN_1406e8ae0(param_2);
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x9e) = bVar2;
  *(byte *)(param_1 + 0x9f) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0xa2) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  uVar5 = FUN_1406e8c20(param_2);
  uVar6 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)(param_1 + 0xa6) = uVar6;
  uVar5 = (uVar6 ^ uVar5) >> 5 | (uVar6 ^ uVar5) << 0x1b;
  *(uint *)(param_1 + 0xaa) = uVar5;
  *(uint *)(param_1 + 0xae) = ((uVar6 ^ 0xbaadf00d) >> 5 | (uVar6 ^ 0xbaadf00d) << 0x1b) + uVar5;
  uVar3 = FUN_1406e8b80(param_2);
  uVar4 = FUN_1402f7010(uVar3,param_1 + 0xb2);
  *(undefined4 *)(param_1 + 0xb6) = uVar4;
  uVar3 = FUN_1406e8b80(param_2);
  uVar4 = FUN_1402f7010(uVar3,param_1 + 0xba);
  *(undefined4 *)(param_1 + 0xbe) = uVar4;
  uVar5 = FUN_1406e8c20(param_2);
  uVar6 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)(param_1 + 0xc2) = uVar6;
  uVar5 = (uVar6 ^ uVar5) >> 5 | (uVar6 ^ uVar5) << 0x1b;
  *(uint *)(param_1 + 0xc6) = uVar5;
  *(uint *)(param_1 + 0xca) = ((uVar6 ^ 0xbaadf00d) >> 5 | (uVar6 ^ 0xbaadf00d) << 0x1b) + uVar5;
  return;
}



//===========================================================
// FUN_1403035a0 @ 1403035a0   (598 bytes)
//===========================================================

void FUN_1403035a0(longlong param_1,undefined8 param_2)

{
  byte *pbVar1;
  ushort uVar2;
  undefined8 *puVar3;
  byte *pbVar4;
  undefined1 uVar5;
  byte bVar6;
  char cVar7;
  undefined4 uVar8;
  undefined8 *puVar9;
  int iVar10;
  byte bVar11;
  byte *pbVar12;
  uint uVar13;
  undefined4 local_res8;
  
  local_res8 = FUN_1406e8c20(param_2);
  iVar10 = *(int *)(param_1 + 0x20) + 1;
  *(int *)(param_1 + 0x20) = iVar10;
  if (iVar10 == (iVar10 / 0x6f) * 0x6f) {
    puVar3 = *(undefined8 **)(param_1 + 0x28);
    puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    *(undefined8 **)(param_1 + 0x28) = puVar9;
    *puVar9 = *puVar3;
    *(undefined4 *)(puVar9 + 1) = *(undefined4 *)(puVar3 + 1);
    thunk_FUN_140205820(puVar3,0xc);
  }
  uVar5 = FUN_142f04924();
  uVar13 = 0;
  *(undefined1 *)(*(longlong *)(param_1 + 0x28) + 4) = uVar5;
  pbVar4 = *(byte **)(param_1 + 0x28);
  bVar11 = pbVar4[4];
  pbVar4[8] = 0x65;
  pbVar4[9] = 0x9a;
  pbVar12 = pbVar4;
  do {
    pbVar1 = pbVar12 + 4;
    if (bVar11 == 0) {
      bVar11 = 0x2a;
    }
    bVar6 = pbVar1[(longlong)(&stack0x00000004 + -(longlong)pbVar4)];
    *pbVar12 = bVar11 ^ bVar6;
    bVar11 = bVar11 + (bVar11 ^ bVar6) + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0x28) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0x28) + 8) = (uVar2 >> 0xd) + (ushort)bVar11 | uVar2 << 3;
    bVar6 = 0x2a;
    if (bVar11 != 0) {
      bVar6 = bVar11;
    }
    bVar11 = pbVar12[(longlong)&local_res8 + (1 - (longlong)pbVar4)];
    pbVar12[1] = bVar6 ^ bVar11;
    bVar6 = (bVar6 ^ bVar11) + bVar6 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0x28) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0x28) + 8) = (uVar2 >> 0xd) + (ushort)bVar6 | uVar2 << 3;
    bVar11 = 0x2a;
    if (bVar6 != 0) {
      bVar11 = bVar6;
    }
    bVar6 = pbVar1[(longlong)(&stack0x00000006 + -(longlong)pbVar4)];
    pbVar12[2] = bVar11 ^ bVar6;
    bVar6 = (bVar11 ^ bVar6) + bVar11 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0x28) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0x28) + 8) = (uVar2 >> 0xd) + (ushort)bVar6 | uVar2 << 3;
    bVar11 = 0x2a;
    if (bVar6 != 0) {
      bVar11 = bVar6;
    }
    uVar13 = uVar13 + 4;
    bVar6 = pbVar1[(longlong)(&stack0x00000007 + -(longlong)pbVar4)];
    pbVar12[3] = bVar11 ^ bVar6;
    bVar11 = (bVar11 ^ bVar6) + bVar11 + 0x2a;
    uVar2 = *(ushort *)(*(longlong *)(param_1 + 0x28) + 8);
    *(ushort *)(*(longlong *)(param_1 + 0x28) + 8) = (uVar2 >> 0xd) + (ushort)bVar11 | uVar2 << 3;
    pbVar12 = pbVar1;
  } while (uVar13 < 4);
  cVar7 = FUN_1406e8ae0(param_2);
  if (cVar7 == '\0') {
    *(undefined8 *)(param_1 + 0x38) = 0;
  }
  else {
    FUN_1406e9170(param_2,param_1 + 0x38,8);
  }
  FUN_1406e9170(param_2,param_1 + 0x40,8);
  uVar8 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x48) = uVar8;
  cVar7 = FUN_1406e8ae0(param_2);
  *(bool *)(param_1 + 0x4c) = cVar7 != '\0';
  return;
}



//===========================================================
// FUN_140303b40 @ 140303b40   (1453 bytes)
//===========================================================

void FUN_140303b40(longlong param_1,undefined8 param_2)

{
  byte bVar1;
  byte bVar2;
  undefined2 uVar3;
  undefined2 uVar4;
  uint uVar5;
  undefined4 uVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  
  FUN_140303800();
  uVar5 = FUN_1406e8c20(param_2);
  if ((uVar5 & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x98) = bVar2;
  *(byte *)(param_1 + 0x99) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x9c) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 & 2) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0xa0) = bVar2;
  uVar12 = 0;
  uVar10 = 0;
  *(byte *)(param_1 + 0xa1) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0xa4) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  uVar4 = 0;
  uVar3 = uVar4;
  if ((uVar5 & 4) != 0) {
    uVar3 = FUN_1406e8b80(param_2);
  }
  uVar6 = FUN_1402f7010(uVar3,param_1 + 0xa8);
  *(undefined4 *)(param_1 + 0xac) = uVar6;
  if ((uVar5 & 8) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0xb0) = bVar2;
  *(byte *)(param_1 + 0xb1) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0xb4) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 & 0x10) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0xb8) = bVar2;
  *(byte *)(param_1 + 0xb9) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0xbc) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  uVar11 = uVar12;
  if ((uVar5 & 0x20) != 0) {
    uVar11 = FUN_1406e8f10(param_2);
  }
  uVar6 = FUN_1402f7170(uVar11,param_1 + 0xc0);
  *(undefined4 *)(param_1 + 0xd0) = uVar6;
  uVar9 = 0xffffffff;
  if ((uVar5 & 0x40) == 0) {
    uVar7 = 0xffffffff;
  }
  else {
    uVar7 = FUN_1406e8c20(param_2);
  }
  uVar8 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)(param_1 + 0xd8) = uVar8;
  uVar7 = (uVar8 ^ uVar7) >> 5 | (uVar8 ^ uVar7) << 0x1b;
  *(uint *)(param_1 + 0xdc) = uVar7;
  *(uint *)(param_1 + 0xe0) = ((uVar8 ^ 0xbaadf00d) >> 5 | (uVar8 ^ 0xbaadf00d) << 0x1b) + uVar7;
  if ((char)uVar5 < '\0') {
    uVar7 = FUN_1406e8c20(param_2);
  }
  else {
    uVar7 = 0;
  }
  uVar8 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)(param_1 + 0xe4) = uVar8;
  uVar7 = (uVar8 ^ uVar7) >> 5 | (uVar8 ^ uVar7) << 0x1b;
  *(uint *)(param_1 + 0xe8) = uVar7;
  *(uint *)(param_1 + 0xec) = ((uVar8 ^ 0xbaadf00d) >> 5 | (uVar8 ^ 0xbaadf00d) << 0x1b) + uVar7;
  if ((uVar5 >> 8 & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0xf0) = bVar2;
  *(byte *)(param_1 + 0xf1) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0xf4) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 9 & 1) != 0) {
    uVar4 = FUN_1406e8b80(param_2);
  }
  uVar6 = FUN_1402f7010(uVar4,param_1 + 0xf8);
  *(undefined4 *)(param_1 + 0xfc) = uVar6;
  if ((uVar5 >> 10 & 1) != 0) {
    uVar9 = FUN_1406e8c20(param_2);
  }
  uVar7 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)(param_1 + 0x100) = uVar7;
  uVar9 = (uVar7 ^ uVar9) >> 5 | (uVar7 ^ uVar9) << 0x1b;
  *(uint *)(param_1 + 0x104) = uVar9;
  *(uint *)(param_1 + 0x108) = ((uVar7 ^ 0xbaadf00d) >> 5 | (uVar7 ^ 0xbaadf00d) << 0x1b) + uVar9;
  if ((uVar5 >> 0xb & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x10c) = bVar2;
  *(byte *)(param_1 + 0x10d) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x110) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 0xc & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x114) = bVar2;
  *(byte *)(param_1 + 0x115) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x118) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 0xd & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x11c) = bVar2;
  *(byte *)(param_1 + 0x11d) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x120) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 0xe & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x124) = bVar2;
  *(byte *)(param_1 + 0x125) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x128) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 0xf & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 300) = bVar2;
  *(byte *)(param_1 + 0x12d) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x130) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 0x10 & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x134) = bVar2;
  *(byte *)(param_1 + 0x135) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x138) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 0x11 & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x13c) = bVar2;
  *(byte *)(param_1 + 0x13d) = bVar1 ^ bVar2;
  *(uint *)(param_1 + 0x140) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar1 ^ bVar2);
  if ((uVar5 >> 0x12 & 1) == 0) {
    bVar1 = 0;
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
  *(byte *)(param_1 + 0x144) = bVar2;
  *(byte *)(param_1 + 0x145) = bVar2 ^ bVar1;
  *(uint *)(param_1 + 0x148) =
       ((bVar2 ^ 0xbaadf00d) >> 5 | (bVar2 ^ 0xbaadf00d) << 0x1b) + (uint)(bVar2 ^ bVar1);
  if ((uVar5 >> 0x13 & 1) != 0) {
    uVar12 = FUN_1406e8f10(param_2);
  }
  uVar6 = FUN_1402f7170(uVar12,param_1 + 0x14c);
  *(undefined4 *)(param_1 + 0x15c) = uVar6;
  if ((uVar5 >> 0x14 & 1) != 0) {
    uVar10 = FUN_1406e8c20(param_2);
  }
  uVar5 = FUN_1407386b0(&DAT_143ac1ab0);
  *(uint *)(param_1 + 0x164) = uVar5;
  uVar10 = (uVar5 ^ uVar10) >> 5 | (uVar5 ^ uVar10) << 0x1b;
  *(uint *)(param_1 + 0x168) = uVar10;
  *(uint *)(param_1 + 0x16c) = ((uVar5 ^ 0xbaadf00d) >> 5 | (uVar5 ^ 0xbaadf00d) << 0x1b) + uVar10;
  return;
}



//===========================================================
// FUN_1402cce00 @ 1402cce00   (132 bytes)
//===========================================================

void FUN_1402cce00(undefined8 param_1,longlong param_2)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  longlong lVar3;
  undefined8 local_res10;
  undefined8 local_res18;
  
  FUN_1406e9170(param_1,&local_res10,8);
  *(undefined8 *)(param_2 + 0x20) = local_res10;
  FUN_1406e9170(param_1,&local_res18,8);
  *(undefined8 *)(param_2 + 0x28) = local_res18;
  uVar1 = FUN_1406e8c20(param_1);
  *(undefined4 *)(param_2 + 0x30) = uVar1;
  lVar3 = 3;
  puVar2 = (undefined4 *)(param_2 + 0x34);
  do {
    uVar1 = FUN_1406e8c20(param_1);
    *puVar2 = uVar1;
    puVar2 = puVar2 + 1;
    lVar3 = lVar3 + -1;
  } while (lVar3 != 0);
  return;
}



//===========================================================
// FUN_1402cd090 @ 1402cd090   (63 bytes)
//===========================================================

void FUN_1402cd090(undefined8 param_1,longlong param_2)

{
  undefined4 uVar1;
  undefined8 local_res10 [3];
  
  FUN_1406e9170(param_1,local_res10,8);
  *(undefined8 *)(param_2 + 0x20) = local_res10[0];
  uVar1 = FUN_1406e8c20(param_1);
  *(undefined4 *)(param_2 + 0x28) = uVar1;
  return;
}



//===========================================================
// FUN_1402cb4f0 @ 1402cb4f0   (166 bytes)
//===========================================================

void FUN_1402cb4f0(uint *param_1,undefined8 param_2)

{
  ushort uVar1;
  uint uVar2;
  undefined4 uVar3;
  undefined8 *puVar4;
  undefined1 *puVar5;
  longlong local_res8;
  
  uVar1 = FUN_1406e8b80(param_2);
  *param_1 = (uint)uVar1;
  uVar2 = FUN_1406e8c20(param_2);
  param_1[2] = uVar2;
  uVar2 = FUN_1406e8c20(param_2);
  param_1[3] = uVar2;
  puVar4 = (undefined8 *)FUN_1406e9050(param_2,&local_res8);
  puVar5 = (undefined1 *)*puVar4;
  if (param_1 + 4 != (uint *)0x0) {
    if (puVar5 == (undefined1 *)0x0) {
      puVar5 = &DAT_1434b2af1;
    }
    (*DAT_143262858)(param_1 + 4,puVar5);
  }
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  uVar3 = FUN_1406e8c20(param_2);
  *(undefined4 *)((longlong)param_1 + 0x1d) = uVar3;
  FUN_1406e9170(param_2,(longlong)param_1 + 0x21,8);
  uVar2 = FUN_1406e8c20(param_2);
  param_1[1] = uVar2;
  return;
}



//===========================================================
// FUN_140303800 @ 140303800   (621 bytes)
//===========================================================

void FUN_140303800(longlong param_1,undefined8 param_2)

{
  undefined2 uVar1;
  undefined2 uVar2;
  undefined2 uVar3;
  uint uVar4;
  undefined4 uVar5;
  
  uVar4 = FUN_1406e8c20(param_2);
  uVar3 = 0;
  uVar2 = 0;
  uVar1 = uVar2;
  if ((uVar4 & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1);
  *(undefined4 *)(param_1 + 4) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 & 2) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 8);
  *(undefined4 *)(param_1 + 0xc) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 & 4) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x10);
  *(undefined4 *)(param_1 + 0x14) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 & 8) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x18);
  *(undefined4 *)(param_1 + 0x1c) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 & 0x10) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x20);
  *(undefined4 *)(param_1 + 0x24) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 & 0x20) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x28);
  *(undefined4 *)(param_1 + 0x2c) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 & 0x40) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x30);
  *(undefined4 *)(param_1 + 0x34) = uVar5;
  uVar1 = uVar2;
  if ((char)uVar4 < '\0') {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x38);
  *(undefined4 *)(param_1 + 0x3c) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 >> 8 & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x40);
  *(undefined4 *)(param_1 + 0x44) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 >> 9 & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x48);
  *(undefined4 *)(param_1 + 0x4c) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 >> 10 & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x50);
  *(undefined4 *)(param_1 + 0x54) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 >> 0xb & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x58);
  *(undefined4 *)(param_1 + 0x5c) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 >> 0xc & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x60);
  *(undefined4 *)(param_1 + 100) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 >> 0xd & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x68);
  *(undefined4 *)(param_1 + 0x6c) = uVar5;
  uVar1 = uVar2;
  if ((uVar4 >> 0xe & 1) != 0) {
    uVar1 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar1,param_1 + 0x70);
  *(undefined4 *)(param_1 + 0x74) = uVar5;
  if ((uVar4 >> 0xf & 1) != 0) {
    uVar2 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar2,param_1 + 0x78);
  *(undefined4 *)(param_1 + 0x7c) = uVar5;
  if ((uVar4 >> 0x10 & 1) != 0) {
    uVar3 = FUN_1406e8b80(param_2);
  }
  uVar5 = FUN_1402f7010(uVar3,param_1 + 0x80);
  *(undefined4 *)(param_1 + 0x84) = uVar5;
  return;
}



//===========================================================
// FUN_1402f70a0 @ 1402f70a0   (131 bytes)
//===========================================================

uint FUN_1402f70a0(undefined2 param_1,longlong param_2)

{
  byte bVar1;
  byte bVar2;
  longlong lVar3;
  byte *pbVar4;
  uint uVar5;
  undefined2 local_res8 [4];
  
  uVar5 = 0xbaadf00d;
  pbVar4 = (byte *)local_res8;
  lVar3 = 2;
  local_res8[0] = param_1;
  do {
    bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
    bVar1 = *pbVar4;
    pbVar4[param_2 - (longlong)local_res8] = bVar2;
    pbVar4[(param_2 + 2) - (longlong)local_res8] = bVar2 ^ bVar1;
    pbVar4 = pbVar4 + 1;
    uVar5 = ((uVar5 ^ bVar2) >> 5 | (uVar5 ^ bVar2) << 0x1b) + (uint)(bVar2 ^ bVar1);
    lVar3 = lVar3 + -1;
  } while (lVar3 != 0);
  return uVar5;
}



//===========================================================
// FUN_1402f7010 @ 1402f7010   (131 bytes)
//===========================================================

uint FUN_1402f7010(undefined2 param_1,longlong param_2)

{
  byte bVar1;
  byte bVar2;
  longlong lVar3;
  byte *pbVar4;
  uint uVar5;
  undefined2 local_res8 [4];
  
  uVar5 = 0xbaadf00d;
  pbVar4 = (byte *)local_res8;
  lVar3 = 2;
  local_res8[0] = param_1;
  do {
    bVar2 = FUN_1407386b0(&DAT_143ac1ab0);
    bVar1 = *pbVar4;
    pbVar4[param_2 - (longlong)local_res8] = bVar2;
    pbVar4[(param_2 + 2) - (longlong)local_res8] = bVar2 ^ bVar1;
    pbVar4 = pbVar4 + 1;
    uVar5 = ((uVar5 ^ bVar2) >> 5 | (uVar5 ^ bVar2) << 0x1b) + (uint)(bVar2 ^ bVar1);
    lVar3 = lVar3 + -1;
  } while (lVar3 != 0);
  return uVar5;
}


