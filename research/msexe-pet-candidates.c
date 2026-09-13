
//===========================================================
// FUN_142799290 @ 142799290   (809 bytes)
//===========================================================

void FUN_142799290(undefined8 param_1,undefined8 param_2)

{
  ulonglong *puVar1;
  undefined8 *puVar2;
  longlong *plVar3;
  int *piVar4;
  ulonglong *puVar5;
  undefined8 *puVar6;
  byte bVar7;
  short sVar8;
  short sVar9;
  uint uVar10;
  undefined4 uVar11;
  int iVar12;
  longlong lVar13;
  uint uVar14;
  undefined8 *puVar15;
  longlong lVar16;
  uint uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  undefined8 *local_res18 [2];
  undefined8 local_58;
  undefined8 local_50;
  ulonglong *local_48;
  longlong local_38;
  
  uVar10 = FUN_1406e8c20(param_2);
  uVar17 = (int)uVar10 >> 0x10;
  sVar8 = FUN_1406e8b80(param_2);
  sVar9 = FUN_1406e8b80(param_2);
  if ((DAT_143aa84d0 != 0) && (lVar13 = FUN_1407b5420(DAT_143aa84d0,uVar10 & 0xffff), lVar13 != 0))
  {
    lVar16 = *(longlong *)(lVar13 + 8);
    uVar18 = 0;
    uVar10 = 0;
    if (lVar16 != 0) {
      uVar10 = *(uint *)(lVar16 + -8);
    }
    if (uVar17 <= uVar10) {
      uVar10 = uVar17 - 1;
      uVar14 = 0;
      if (lVar16 != 0) {
        uVar14 = *(uint *)(lVar16 + -8);
      }
      if (((int)uVar10 < 0) || (uVar14 <= uVar10)) {
        uVar19 = uVar18;
        if (lVar16 != 0) {
          uVar19 = (ulonglong)*(uint *)(lVar16 + -8);
        }
        FUN_142e54290(0xc6,uVar10,uVar19);
        lVar16 = *(longlong *)(lVar13 + 8);
      }
      lVar13 = DAT_143abfdf0;
      if (DAT_143abfdf0 != 0) {
        local_res18[0] = &local_58;
        local_58 = 0;
        local_50 = 0;
        puVar5 = *(ulonglong **)((longlong)(int)uVar17 * 0x418 + -0x310 + lVar16);
        if (puVar5 != (ulonglong *)0x0) {
          LOCK();
          *(int *)(puVar5 + 2) = (int)puVar5[2] + 1;
          UNLOCK();
          uVar18 = *puVar5;
        }
        local_48 = puVar5;
        FUN_140e16070(lVar13,uVar18,0,&local_50,(int)sVar8,(int)sVar9,&local_58,0xc00614a4,0,0xff);
        if (puVar5 != (ulonglong *)0x0) {
          LOCK();
          puVar1 = puVar5 + 2;
          uVar18 = *puVar1;
          *(int *)puVar1 = (int)*puVar1 + -1;
          UNLOCK();
          if ((int)uVar18 == 1) {
            if (*puVar5 != 0) {
              (*DAT_143ad5990)(*puVar5 - 4);
              *puVar5 = 0;
            }
            if (puVar5[1] != 0) {
              FUN_14019b4e0();
              puVar5[1] = 0;
            }
            thunk_FUN_140205820(puVar5,0x18);
          }
        }
      }
      bVar7 = FUN_1406e8ae0(param_2);
      if ((DAT_143ac1b90 != 0) && (bVar7 != 0)) {
        uVar18 = (ulonglong)bVar7;
        do {
          uVar11 = FUN_1406e8c20(param_2);
          iVar12 = FUN_1406e8c20(param_2);
          lVar13 = FUN_1429b6c90(DAT_143ac1b90,uVar11);
          if ((lVar13 == 0) || (lVar13 == -0x10)) {
            local_38 = 0;
          }
          else {
            local_38 = lVar13;
            if (0xfffff < *(ulonglong *)(lVar13 + 0x18)) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            *(longlong *)(lVar13 + 0x18) = *(longlong *)(lVar13 + 0x18) + 1;
            UNLOCK();
          }
          lVar13 = local_38;
          if (local_38 != 0) {
            FUN_142771360(local_38,-iVar12,0,0,0,0);
            puVar6 = *(undefined8 **)(lVar13 + 0x38);
            if (puVar6 != (undefined8 *)0x0) {
              if (0xfffff < (ulonglong)puVar6[1]) {
                FUN_142e541f0(0x30f);
              }
              LOCK();
              puVar6[1] = puVar6[1] + 1;
              UNLOCK();
              lVar13 = local_38;
            }
            puVar2 = puVar6 + 4;
            FUN_1401d3510(puVar2,local_res18);
            if (0xffffe < *(longlong *)(lVar13 + 0x18) - 1U) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            plVar3 = (longlong *)(lVar13 + 0x18);
            lVar13 = *plVar3;
            *plVar3 = *plVar3 + -1;
            UNLOCK();
            if ((int)lVar13 == 1) {
              puVar6[6] = 0;
              puVar15 = (undefined8 *)(local_38 + 0x10);
              if (puVar15 != (undefined8 *)0x0) {
                (**(code **)*puVar15)(puVar15,1);
              }
            }
            if (puVar2 != (undefined8 *)0x0) {
              piVar4 = (int *)(puVar6 + 5);
              *piVar4 = *piVar4 + -1;
              if (*piVar4 == 0) {
                *puVar2 = 0;
              }
            }
            if (puVar6 != (undefined8 *)0x0) {
              if (0xffffe < puVar6[1] - 1) {
                FUN_142e541f0(0x31e);
              }
              LOCK();
              plVar3 = puVar6 + 1;
              lVar13 = *plVar3;
              *plVar3 = *plVar3 + -1;
              UNLOCK();
              if ((int)lVar13 == 1) {
                (**(code **)*puVar6)(puVar6,1);
              }
            }
            local_38 = 0;
          }
          uVar18 = uVar18 - 1;
        } while (uVar18 != 0);
      }
    }
  }
  return;
}



//===========================================================
// FUN_1427995c0 @ 1427995c0   (51 bytes)
//===========================================================

void FUN_1427995c0(undefined8 param_1)

{
  int iVar1;
  
  iVar1 = FUN_1406e8c20();
  FUN_142771360(param_1,-iVar1,0,0,0,0);
  return;
}



//===========================================================
// FUN_14279a6e0 @ 14279a6e0   (1075 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14279a6e0(longlong param_1,undefined8 param_2)

{
  int iVar1;
  ulonglong uVar2;
  longlong *plVar3;
  byte bVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  int iVar7;
  uint uVar8;
  longlong lVar9;
  longlong *plVar10;
  longlong lVar11;
  longlong *plVar12;
  ulonglong *puVar13;
  uint uVar14;
  ulonglong uVar15;
  undefined1 auStack_1b8 [32];
  int local_198;
  undefined4 local_190;
  undefined8 local_188;
  longlong local_180;
  ulonglong local_178;
  undefined4 local_170;
  undefined4 local_168;
  ulonglong local_160;
  longlong **local_158;
  undefined8 *local_150;
  undefined1 local_148;
  undefined4 local_140;
  undefined4 local_138;
  ulonglong local_130;
  undefined4 local_128;
  undefined4 local_120;
  undefined4 local_118;
  undefined1 local_110;
  uint local_108;
  undefined8 local_100;
  undefined4 local_f8;
  undefined4 local_f0;
  undefined1 *local_e8;
  undefined4 local_e0;
  undefined8 local_d8;
  undefined8 local_c8;
  int local_c0;
  longlong *local_b8;
  undefined4 local_b0;
  longlong *local_a8;
  longlong local_a0;
  undefined1 local_98 [8];
  undefined1 local_90 [8];
  undefined8 local_88;
  undefined1 *local_80;
  longlong **local_78;
  undefined4 local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_1b8;
  local_c8 = param_2;
  local_a0 = param_1;
  uVar5 = FUN_1406e8c20(param_2);
  local_b0 = uVar5;
  bVar4 = FUN_1406e8ae0(param_2);
  lVar9 = FUN_1407b2910(DAT_143aa84d0,uVar5);
  lVar9 = *(longlong *)(lVar9 + 8);
  if (bVar4 != 0) {
    uVar15 = (ulonglong)bVar4;
    do {
      uVar5 = 0;
      uVar6 = FUN_1406e8c20(param_2);
      local_c0 = FUN_1406e8c20(param_2);
      plVar10 = (longlong *)FUN_141d2efc0(DAT_143abfe00,uVar6);
      if (plVar10 != (longlong *)0x0) {
        (**(code **)(*plVar10 + 0x10))(plVar10,&local_58,1);
        lVar11 = FUN_14079fe90(lVar9,1);
        plVar12 = (longlong *)(lVar11 + 0x710);
        lVar11 = *plVar12;
        if ((lVar11 == 0) || (*(int *)(lVar11 + -8) == 0)) {
          plVar12 = (longlong *)FUN_14079f670(lVar9,1);
          if (plVar12 == (longlong *)0x0) {
            plVar12 = (longlong *)(lVar9 + 0x1120);
            lVar11 = *plVar12;
          }
          else {
            lVar11 = *plVar12;
          }
          if (lVar11 != 0) goto LAB_14279a7fb;
LAB_14279a85c:
          local_b8 = (longlong *)0x0;
        }
        else {
LAB_14279a7fb:
          iVar1 = *(int *)(lVar11 + -8);
          if (iVar1 == 0) goto LAB_14279a85c;
          iVar7 = FUN_142f04924();
          uVar2 = (longlong)iVar7 % (longlong)iVar1;
          uVar14 = (uint)uVar2;
          lVar11 = *plVar12;
          uVar8 = 0;
          if (lVar11 != 0) {
            uVar8 = *(uint *)(lVar11 + -8);
          }
          if (((int)uVar14 < 0) || (uVar8 <= uVar14)) {
            if (lVar11 != 0) {
              uVar5 = *(undefined4 *)(lVar11 + -8);
            }
            FUN_142e54290(0xc6,uVar2 & 0xffffffff,uVar5);
            lVar11 = *plVar12;
          }
          local_b8 = *(longlong **)(lVar11 + (longlong)(int)uVar14 * 8);
          if (local_b8 != (longlong *)0x0) {
            LOCK();
            *(int *)(local_b8 + 2) = (int)local_b8[2] + 1;
            UNLOCK();
          }
        }
        plVar3 = local_b8;
        local_80 = local_90;
        local_88 = 0;
        local_78 = &local_a8;
        plVar12 = local_b8 + 2;
        if (local_b8 != (longlong *)0x0) {
          LOCK();
          *(int *)plVar12 = (int)*plVar12 + 1;
          UNLOCK();
        }
        local_68 = local_58;
        uStack_64 = uStack_54;
        uStack_60 = uStack_50;
        uStack_5c = uStack_4c;
        local_a8 = local_b8;
        puVar13 = (ulonglong *)FUN_141c58fd0(plVar10,local_98);
        uVar2 = *puVar13;
        uVar5 = (**(code **)(*plVar10 + 0x68))(plVar10);
        uVar6 = FUN_141c56bb0(plVar10);
        local_198 = FUN_1429e3ef0();
        param_1 = local_a0;
        local_198 = local_198 + 0x550;
        local_180 = (longlong)local_c0;
        local_d8 = 0;
        local_e0 = 0xffffffff;
        local_e8 = local_90;
        local_f0 = 0;
        local_f8 = 0;
        local_100 = 0;
        local_108 = local_108 & 0xffffff00;
        local_110 = 0;
        local_118 = 0;
        local_120 = 0;
        local_128 = 0;
        local_130 = local_130 & 0xffffffff00000000;
        local_138 = 0;
        local_140 = 0;
        local_148 = 0;
        local_150 = (undefined8 *)((ulonglong)local_150 & 0xffffffff00000000);
        local_158 = &local_a8;
        local_168 = 1;
        local_170 = 1;
        local_178 = local_178 & 0xffffffff00000000;
        local_188 = CONCAT44(local_188._4_4_,uVar5);
        local_190 = uVar6;
        local_160 = uVar2;
        FUN_141c5d190(plVar10,*(undefined4 *)(local_a0 + 0x10d0),local_b0,1);
        plVar10 = local_b8;
        param_2 = local_c8;
        if (plVar3 != (longlong *)0x0) {
          LOCK();
          lVar11 = *plVar12;
          *(int *)plVar12 = (int)*plVar12 + -1;
          UNLOCK();
          if ((int)lVar11 == 1) {
            if (*local_b8 != 0) {
              (*DAT_143ad5990)(*local_b8 + -4);
              *plVar10 = 0;
            }
            if (plVar10[1] != 0) {
              FUN_14019b4e0();
              plVar10[1] = 0;
            }
            thunk_FUN_140205820(plVar10,0x18);
            param_2 = local_c8;
          }
        }
      }
      uVar15 = uVar15 - 1;
    } while (uVar15 != 0);
  }
  local_c8 = 0;
  local_100 = 0;
  local_108 = 0;
  local_110 = 0;
  local_118 = 0;
  local_120 = 0xffffffff;
  local_128 = 0;
  local_130 = 0;
  local_138 = 0;
  local_140 = 0;
  local_148 = 0;
  local_150 = &local_c8;
  local_158 = (longlong **)((ulonglong)local_158 & 0xffffffff00000000);
  local_160 = local_160 & 0xffffffff00000000;
  local_168 = 0;
  local_170 = 0;
  local_178 = 0;
  local_180 = 0;
  local_188 = 0;
  local_190 = 0x7fffffff;
  local_198 = 0;
  FUN_1427d5b80(param_1,lVar9,1);
  return;
}



//===========================================================
// FUN_1427bdd40 @ 1427bdd40   (217 bytes)
//===========================================================

void FUN_1427bdd40(longlong param_1,undefined8 param_2)

{
  undefined4 uVar1;
  undefined8 *puVar2;
  longlong local_res8;
  
  uVar1 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x1308) = uVar1;
  puVar2 = (undefined8 *)FUN_1406e9050(param_2,&local_res8);
  if (*(longlong *)(param_1 + 0x1310) != 0) {
    FUN_14019f2c0();
    *(undefined8 *)(param_1 + 0x1310) = 0;
  }
  *(undefined8 *)(param_1 + 0x1310) = *puVar2;
  *puVar2 = 0;
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  puVar2 = (undefined8 *)FUN_1406e9050(param_2,&local_res8);
  if (*(longlong *)(param_1 + 0x1318) != 0) {
    FUN_14019f2c0();
    *(undefined8 *)(param_1 + 0x1318) = 0;
  }
  *(undefined8 *)(param_1 + 0x1318) = *puVar2;
  *puVar2 = 0;
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  return;
}



//===========================================================
// FUN_142798cf0 @ 142798cf0   (81 bytes)
//===========================================================

void FUN_142798cf0(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  
  iVar1 = FUN_1406e8c20(param_2);
  if (iVar1 != 0) {
    if (iVar1 == 1) {
      FUN_142798f10(param_1,param_2);
      return;
    }
    if (iVar1 != 2) {
      return;
    }
  }
  FUN_142798d50(param_1,param_2);
  return;
}



//===========================================================
// FUN_1427eeb40 @ 1427eeb40   (231 bytes)
//===========================================================

void FUN_1427eeb40(longlong param_1,undefined8 param_2)

{
  IUnknown *pIVar1;
  int iVar2;
  undefined4 *puVar3;
  longlong lVar4;
  
  FUN_1406e8c20(param_2);
  if (*(longlong *)(param_1 + 0x2960) != 0) {
    lVar4 = 0x6a;
    do {
      puVar3 = (undefined4 *)(param_1 + 0x1568 + lVar4 * 0x30);
      *puVar3 = 0xffffffff;
      puVar3[4] = 1;
      if (*(longlong **)(puVar3 + 10) != (longlong *)0x0) {
        (**(code **)(**(longlong **)(puVar3 + 10) + 0x10))();
      }
      *(undefined8 *)(puVar3 + 10) = 0;
      if (*(longlong **)(puVar3 + 6) != (longlong *)0x0) {
        (**(code **)(**(longlong **)(puVar3 + 6) + 0x10))();
      }
      *(undefined8 *)(puVar3 + 6) = 0;
      if (*(longlong **)(puVar3 + 8) != (longlong *)0x0) {
        (**(code **)(**(longlong **)(puVar3 + 8) + 0x10))();
      }
      *(undefined8 *)(puVar3 + 8) = 0;
      if (lVar4 == 1) {
        *(undefined4 *)(param_1 + 0x1598) = 0;
      }
      lVar4 = lVar4 + 1;
    } while (lVar4 < 0x6b);
  }
  pIVar1 = *(IUnknown **)(param_1 + 0x2930);
  if (pIVar1 != (IUnknown *)0x0) {
    iVar2 = (**(code **)(*(longlong *)pIVar1 + 0x2b8))(pIVar1,1);
    if (iVar2 < 0) {
      _com_issue_errorex(iVar2,pIVar1,(_GUID *)&DAT_14327fcb0);
    }
  }
  return;
}



//===========================================================
// FUN_1427991b0 @ 1427991b0   (215 bytes)
//===========================================================

void FUN_1427991b0(longlong *param_1,undefined8 param_2)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  
  cVar1 = FUN_1406e8ae0(param_2);
  FUN_140f8ca70(param_1 + 0x20,0,0,0);
  if (cVar1 == '\0') {
    if ((int)param_1[0x250] != -1) {
      if ((int)param_1[0x250] == 2) {
        iVar2 = (*DAT_143262db0)();
        if (iVar2 - *(int *)((longlong)param_1 + 0x1284) < 1) goto LAB_14279920f;
      }
      *(undefined4 *)(param_1 + 0x250) = 0xffffffff;
    }
LAB_14279920f:
    uVar3 = 2;
  }
  else {
    if (cVar1 != '\x02') goto LAB_14279925c;
    if ((int)param_1[0x250] != -1) {
      if ((int)param_1[0x250] == 4) {
        iVar2 = (*DAT_143262db0)();
        if (iVar2 - *(int *)((longlong)param_1 + 0x1284) < 1) goto LAB_142799245;
      }
      *(undefined4 *)(param_1 + 0x250) = 0xffffffff;
    }
LAB_142799245:
    uVar3 = 4;
  }
  FUN_140f832e0(param_1 + 0x20,uVar3,2000);
LAB_14279925c:
  iVar2 = (**(code **)(*param_1 + 0x50))(param_1);
  if (iVar2 != 0) {
    FUN_1428a5de0(DAT_143aa8518,cVar1);
  }
  return;
}



//===========================================================
// FUN_141b1e6c0 @ 141b1e6c0   (161 bytes)
//===========================================================

void FUN_141b1e6c0(undefined1 *param_1)

{
  FUN_141b1e380();
  *(undefined1 *)(DAT_143ac18a0 + 0x13c) = 0;
  FUN_141b1f970(param_1,0);
  FUN_1407ab0d0(DAT_143aa84d0);
  FUN_141b20200(param_1);
  *(uint *)(param_1 + 4) = *(uint *)(param_1 + 4) & 0xfffffffe;
  FUN_141b203d0(param_1);
  FUN_141ed1fb0();
  DAT_143ad3164 = 0;
  FUN_141b205a0(param_1);
  FUN_1403bb3b0(DAT_143aa8328);
  FUN_141b20770(param_1);
  FUN_1403cfdf0(DAT_143aa8328);
  FUN_1408460c0(param_1 + 8);
  *(undefined4 *)(param_1 + 4) = 0;
  *param_1 = 0;
  *(undefined1 *)(DAT_143ac18a0 + 0x13c) = 1;
  return;
}



//===========================================================
// FUN_141b0f940 @ 141b0f940   (750 bytes)
//===========================================================

void FUN_141b0f940(longlong param_1)

{
  IUnknown *pIVar1;
  char cVar2;
  int iVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong lVar6;
  undefined8 *local_res10;
  undefined8 **local_res18;
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
  undefined1 local_38 [32];
  
  FUN_141b1ae30(local_38);
  if (*(int *)(param_1 + 0x1c) == 1) {
    FUN_141e77a10();
    uVar4 = FUN_140caa510();
    FUN_142d487a0(uVar4);
  }
  else {
    FUN_141e77a10();
    uVar4 = FUN_140caa510();
    FUN_142d487a0(uVar4);
  }
  pIVar1 = DAT_143add068;
  if ((DAT_143add068 != (IUnknown *)0x0) && (*(int *)(param_1 + 0x18) != 0)) {
    FUN_1401bb8d0(&local_res10,L"noquest");
    local_res18 = &local_res10;
    (*DAT_143262a20)(&local_50);
    uVar4 = 0;
    if (local_res10 != (undefined8 *)0x0) {
      uVar4 = *local_res10;
    }
    iVar3 = (**(code **)(*(longlong *)pIVar1 + 0x28))(pIVar1,uVar4,&local_50);
    if (iVar3 < 0) {
      _com_issue_errorex(iVar3,pIVar1,(_GUID *)&DAT_143272478);
    }
    local_68 = local_50;
    uStack_64 = uStack_4c;
    uStack_60 = uStack_48;
    uStack_5c = uStack_44;
    local_58 = local_40;
    local_50 = local_50 & 0xffff0000;
    FUN_1401be120(&local_res10);
    iVar3 = FUN_14022ee40(&local_68,0);
    *(uint *)(param_1 + 0x18) = (uint)(iVar3 == 0);
    if ((short)local_68 == 8) {
      local_68 = local_68 & 0xffff0000;
      if (CONCAT44(uStack_5c,uStack_60) != 0) {
        (*DAT_143ad5990)(CONCAT44(uStack_5c,uStack_60) + -4);
      }
    }
    else {
      (*DAT_143262a18)(&local_68);
    }
  }
  cVar2 = FUN_141b1c7f0();
  if (cVar2 == '\0') {
    FUN_141b14ad0();
  }
  uVar4 = FUN_141b1c710();
  FUN_141780db0(uVar4);
  FUN_141b151c0();
  uVar4 = FUN_141b1c790();
  iVar3 = FUN_140439af0(uVar4);
  if (iVar3 == 0) {
    FUN_140cc2350(&DAT_143271f04,0xb8,0x22000006);
  }
  FUN_140430610();
  FUN_141b150c0();
  uVar4 = FUN_141b1c770();
  iVar3 = FUN_140378e90(uVar4);
  if (iVar3 == 0) {
    FUN_140cc2350(&DAT_143271f04,0xbd,0x22000006);
  }
  FUN_141d51d30();
  FUN_1420ffe60();
  FUN_141ed1fb0();
  FUN_140239690();
  FUN_141f47100();
  FUN_1406ff610();
  FUN_141b15690();
  FUN_141b15710();
  FUN_141b15360();
  uVar4 = FUN_141b1c7a0();
  FUN_14043e010(uVar4);
  FUN_141b145f0();
  uVar4 = FUN_141b1c6d0();
  FUN_141017fa0(uVar4);
  FUN_141b14950();
  FUN_141b14bd0();
  uVar4 = FUN_141b1c730();
  FUN_141801bc0(uVar4);
  FUN_141f6bc80();
  FUN_141b14670();
  FUN_1408ec7e0();
  FUN_141b159b0();
  uVar4 = FUN_141b1c7d0();
  FUN_140705d40(uVar4);
  FUN_141b15a30();
  uVar4 = FUN_141b1c7e0();
  FUN_140a0e4d0(uVar4);
  FUN_142107f80();
  FUN_141b15610();
  FUN_1413f0c90();
  uVar4 = FUN_141b149d0();
  FUN_140752c30(uVar4);
  uVar4 = FUN_141b15570();
  FUN_1411f5a60(uVar4);
  if (*(int *)(param_1 + 0x1c) == 1) {
    lVar5 = FUN_141b1e0d0(local_38);
    lVar6 = FUN_141b1e0e0(local_38);
    for (; lVar5 != lVar6; lVar5 = lVar5 + 0x10) {
      uVar4 = FUN_141b19bb0(lVar5);
      uVar4 = FUN_141b19bb0(uVar4);
      FUN_141b1a560(&local_68,uVar4,1);
      FUN_141b1d6c0(&local_68);
      FUN_141b1bdb0(&local_68);
    }
  }
  FUN_1403f6060(local_38);
  return;
}


