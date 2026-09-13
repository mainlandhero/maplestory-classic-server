
//===========================================================
// FUN_1428a01a0 @ 1428a01a0   (2420 bytes)
//===========================================================

void FUN_1428a01a0(undefined8 param_1,undefined8 param_2)

{
  char cVar1;
  char cVar2;
  short sVar3;
  int iVar4;
  undefined4 uVar5;
  uint uVar6;
  ulonglong uVar7;
  longlong lVar8;
  undefined8 uVar9;
  ulonglong uVar10;
  longlong lVar11;
  longlong *plVar12;
  int *piVar13;
  undefined8 *puVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  undefined8 uVar17;
  bool bVar18;
  longlong local_res18;
  int *local_res20;
  code *pcVar19;
  undefined8 local_b8;
  longlong local_b0;
  int *local_a8;
  int *local_a0;
  undefined8 local_98;
  undefined1 *local_90;
  int **local_88;
  longlong *local_78;
  int *local_70;
  undefined1 local_68 [16];
  undefined1 local_58 [8];
  undefined8 local_50;
  longlong local_48;
  
  lVar11 = DAT_143aa8328;
  if (DAT_143aa8328 == 0) {
    return;
  }
  if (DAT_143aa84a0 == 0) {
    return;
  }
  iVar4 = FUN_1406e8c20(param_2);
  if (iVar4 != 0) {
    return;
  }
  cVar1 = FUN_1406e8ae0(param_2);
  if (cVar1 != '\0') {
    cVar1 = FUN_1406e8ae0(param_2);
    local_res18 = FUN_14019b780(&DAT_143ad68a0,0x668);
    uVar16 = 0;
    uVar7 = uVar16;
    if (local_res18 != 0) {
      uVar7 = FUN_141eb62e0(local_res18);
    }
    if (uVar7 == 0) {
      return;
    }
    cVar2 = FUN_141eb9760(uVar7,param_1,0,param_2);
    if (cVar2 == '\0') {
      return;
    }
    FUN_1427707e0(param_1,0,uVar7);
    lVar8 = FUN_141ebdad0(uVar7);
    FUN_141ecde00(uVar7);
    if (((cVar1 == '\0') && (lVar8 != 0)) &&
       (sVar3 = FUN_1401ab420(lVar8 + 0x62,*(undefined4 *)(lVar8 + 0x66)), sVar3 == 0)) {
      uVar5 = FUN_14019a5d0(lVar8 + 0x20);
      iVar4 = FUN_14038a300(lVar11,uVar5);
      if (iVar4 == 0) {
        uVar5 = FUN_14019a5d0(lVar8 + 0x20);
        iVar4 = FUN_14038a400(lVar11,uVar5);
        if (iVar4 == 0) {
          uVar5 = FUN_14019a5d0(lVar8 + 0x20);
          FUN_140389c70(lVar11,uVar5,0);
          uVar5 = FUN_14038a5b0(lVar11,lVar8);
          *(undefined4 *)(DAT_143aa84a0 + 0x1e4) = 1;
          pcVar19 = FUN_140199470;
          _eh_vector_constructor_iterator_(local_68,8,2,FUN_1402bc5e0,FUN_140199470);
          uVar17 = 0xdb;
          uVar7 = uVar16;
          switch(uVar5) {
          case 1:
          case 2:
          case 3:
            uVar17 = 0x1f20de;
            uVar9 = FUN_1408a9e40(&local_res18,0x1a7);
            FUN_140319ad0(local_68,uVar9);
            if (local_res18 != 0) {
              FUN_14019f2c0(local_res18 + -0x10);
            }
            break;
          case 4:
          case 5:
          case 6:
            uVar17 = 0x1f20de;
            uVar9 = FUN_1408a9e40(&local_res18,0x1a8);
            FUN_140319ad0(local_68,uVar9);
            if (local_res18 != 0) {
              FUN_14019f2c0(local_res18 + -0x10);
            }
            break;
          default:
            uVar9 = FUN_1408a9e40(&local_res18,0x1a6);
            FUN_140319ad0(local_68,uVar9);
            if (local_res18 != 0) {
              FUN_14019f2c0(local_res18 + -0x10);
            }
          }
          while ((int)uVar7 == 0) {
            local_48 = FUN_14019b780(&DAT_143ad68a0,2000);
            uVar10 = uVar16;
            if (local_48 != 0) {
              uVar10 = FUN_142a57d30(local_48,0,0,0);
            }
            uVar15 = uVar10 + 0x18;
            if (uVar10 == 0) {
              uVar15 = uVar16;
            }
            if (uVar15 == 0) {
              local_78 = (longlong *)0x0;
            }
            else {
              local_78 = (longlong *)(uVar15 - 0x18);
              if (local_78 != (longlong *)0x0) {
                if (0xfffff < *(ulonglong *)(uVar15 + 8)) {
                  FUN_142e541f0(0x30f);
                }
                LOCK();
                *(longlong *)(uVar15 + 8) = *(longlong *)(uVar15 + 8) + 1;
                UNLOCK();
              }
            }
            plVar12 = local_78;
            if (local_78 == (longlong *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            local_90 = local_58;
            local_50 = 0;
            local_88 = &local_70;
            local_70 = (int *)0x0;
            local_98 = 0;
            FUN_14019a260(&local_98,local_68 + uVar7 * 8);
            pcVar19 = (code *)((ulonglong)pcVar19 & 0xffffffff00000000);
            FUN_142a61900(plVar12,0,uVar17,&local_98,pcVar19,0,&local_70,local_58,0,0);
            if (plVar12 == (longlong *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            FUN_142a62bf0(plVar12,0);
            if (plVar12 == (longlong *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            FUN_142a5ee30(plVar12);
            if (plVar12 == (longlong *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            iVar4 = (**(code **)(*plVar12 + 0x130))(plVar12);
            if (iVar4 == 0x2000) {
              if (0xffffe < plVar12[4] - 1U) {
                FUN_142e541f0(0x31e);
              }
              LOCK();
              plVar12 = plVar12 + 4;
              lVar11 = *plVar12;
              *plVar12 = *plVar12 + -1;
              UNLOCK();
              if (((int)lVar11 == 1) && (plVar12 = local_78 + 3, plVar12 != (longlong *)0x0)) {
                (**(code **)*plVar12)(plVar12,1);
              }
              uVar6 = 0xffffffff;
            }
            else {
              if (iVar4 != 0x2001) {
                if (plVar12 != (longlong *)0x0) {
                  if (0xffffe < plVar12[4] - 1U) {
                    FUN_142e541f0(0x31e);
                  }
                  LOCK();
                  plVar12 = plVar12 + 4;
                  lVar11 = *plVar12;
                  *plVar12 = *plVar12 + -1;
                  UNLOCK();
                  if (((int)lVar11 == 1) && (plVar12 = local_78 + 3, plVar12 != (longlong *)0x0)) {
                    (**(code **)*plVar12)(plVar12,1);
                  }
                }
                break;
              }
              if (0xffffe < plVar12[4] - 1U) {
                FUN_142e541f0(0x31e);
              }
              LOCK();
              plVar12 = plVar12 + 4;
              lVar11 = *plVar12;
              *plVar12 = *plVar12 + -1;
              UNLOCK();
              if (((int)lVar11 == 1) && (plVar12 = local_78 + 3, plVar12 != (longlong *)0x0)) {
                (**(code **)*plVar12)(plVar12,1);
              }
              uVar6 = 1;
            }
            local_78 = (longlong *)0x0;
            uVar7 = (ulonglong)uVar6;
            if ((int)uVar6 < 0) break;
          }
          _eh_vector_destructor_iterator_(local_68,8,2,FUN_140199470);
        }
      }
    }
    goto LAB_1428a0ad0;
  }
  FUN_1408a9e40(&local_b0,0x21);
  lVar11 = FUN_1427703d0(param_1,0);
  if (lVar11 == 0) {
    if (local_b0 == 0) {
      return;
    }
    FUN_14019f2c0(local_b0 + -0x10);
    return;
  }
  uVar17 = FUN_141ebda20(lVar11);
  FUN_14019a260(&local_b0,uVar17);
  bVar18 = false;
  uVar17 = FUN_1427703d0(param_1,0);
  lVar8 = FUN_141ebdad0(uVar17);
  lVar11 = DAT_143aa8328;
  if (lVar8 != 0) {
    uVar5 = FUN_14019a5d0(lVar8 + 0x20);
    iVar4 = FUN_14038a400(lVar11,uVar5);
    bVar18 = iVar4 != 0;
    FUN_14019a5d0(lVar8 + 0x20);
  }
  FUN_1427707e0(param_1,0,0);
  local_res18 = 0;
  cVar1 = FUN_1406e8ae0(param_2);
  if (cVar1 == '\x01') {
    local_88 = &local_res20;
    local_res20 = (int *)0x0;
    piVar13 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar13[1] = 0;
    *piVar13 = -1;
    local_res20 = piVar13 + 4;
    piVar13[2] = 0;
    *(undefined1 *)local_res20 = 0;
    if (*piVar13 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar13[1] < 0) {
      FUN_142e54290(0x90,piVar13[1],0);
    }
    *piVar13 = 1;
    *(undefined1 *)local_res20 = 0;
    if (piVar13[1] + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    piVar13[2] = 0;
    local_b8 = 0;
    FUN_14019a260(&local_b8,&local_b0);
    puVar14 = (undefined8 *)FUN_1408e56b0(&local_a8,&local_b8,&local_res20);
    uVar17 = *puVar14;
    puVar14 = (undefined8 *)FUN_1408a9e40(&local_a0,0x1b0);
    FUN_14019ba10(&local_res18,*puVar14,uVar17);
    if (local_a0 != (int *)0x0) {
      FUN_14019f2c0(local_a0 + -4);
    }
LAB_1428a0a7e:
    if (local_a8 != (int *)0x0) {
      FUN_14019f2c0(local_a8 + -4);
    }
  }
  else {
    if (cVar1 != '\x02') {
      if (cVar1 == '\x03') {
        plVar12 = (longlong *)FUN_1408a9e40(&local_res20,0x1b3);
        if (local_res18 != 0) {
          FUN_14019f2c0(local_res18 + -0x10);
        }
        local_res18 = *plVar12;
        *plVar12 = 0;
        local_a8 = local_res20;
      }
      else if (cVar1 == '\x04') {
        plVar12 = (longlong *)FUN_1408a9e40(&local_res20,0x1b4);
        if (local_res18 != 0) {
          FUN_14019f2c0(local_res18 + -0x10);
        }
        local_res18 = *plVar12;
        *plVar12 = 0;
        local_a8 = local_res20;
      }
      else {
        if (cVar1 != '\x05') goto LAB_1428a0a8d;
        plVar12 = (longlong *)FUN_1408a9e40(&local_res20,0x1b5);
        if (local_res18 != 0) {
          FUN_14019f2c0(local_res18 + -0x10);
        }
        local_res18 = *plVar12;
        *plVar12 = 0;
        local_a8 = local_res20;
      }
      goto LAB_1428a0a7e;
    }
    local_88 = &local_res20;
    local_res20 = (int *)0x0;
    piVar13 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar13[1] = 0;
    local_res20 = piVar13 + 4;
    *piVar13 = -1;
    piVar13[2] = 0;
    *(undefined1 *)local_res20 = 0;
    if (bVar18) {
      if (*piVar13 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar13[1] < 0) {
        FUN_142e54290(0x90,piVar13[1],0);
      }
      *piVar13 = 1;
      *(undefined1 *)local_res20 = 0;
      if (piVar13[1] + 1 < 1) {
        FUN_142e54290(0x9c,0);
      }
      piVar13[2] = 0;
      local_b8 = 0;
      FUN_14019a260(&local_b8,&local_b0);
      puVar14 = (undefined8 *)FUN_1408e56b0(&local_a0,&local_b8,&local_res20);
      uVar17 = *puVar14;
      puVar14 = (undefined8 *)FUN_1408a9e40(&local_a8,0x1b2);
      FUN_14019ba10(&local_res18,*puVar14,uVar17);
      piVar13 = local_a0;
      if (local_a8 != (int *)0x0) {
        FUN_14019f2c0(local_a8 + -4);
        piVar13 = local_a0;
      }
    }
    else {
      if (*piVar13 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar13[1] < 0) {
        FUN_142e54290(0x90,piVar13[1],0);
      }
      *piVar13 = 1;
      *(undefined1 *)local_res20 = 0;
      if (piVar13[1] + 1 < 1) {
        FUN_142e54290(0x9c,0);
      }
      piVar13[2] = 0;
      local_b8 = 0;
      FUN_14019a260(&local_b8,&local_b0);
      puVar14 = (undefined8 *)FUN_1408e56b0(&local_a8,&local_b8,&local_res20);
      uVar17 = *puVar14;
      puVar14 = (undefined8 *)FUN_1408a9e40(&local_a0,0x1b1);
      FUN_14019ba10(&local_res18,*puVar14,uVar17);
      piVar13 = local_a8;
      if (local_a0 != (int *)0x0) {
        FUN_14019f2c0(local_a0 + -4);
        piVar13 = local_a8;
      }
    }
    if (piVar13 != (int *)0x0) {
      FUN_14019f2c0(piVar13 + -4);
    }
  }
LAB_1428a0a8d:
  if ((local_res18 != 0) && (*(int *)(local_res18 + -8) != 0)) {
    FUN_1415eca30(&local_res18,0xb);
  }
  if (local_res18 != 0) {
    FUN_14019f2c0(local_res18 + -0x10);
  }
  if (local_b0 != 0) {
    FUN_14019f2c0(local_b0 + -0x10);
  }
LAB_1428a0ad0:
  FUN_1428d7650(param_1);
  FUN_1428d76f0(param_1);
  if (DAT_143acda08 != 0) {
    FUN_1414bba10();
  }
  FUN_14271bfa0(0);
  return;
}



//===========================================================
// FUN_1429d6150 @ 1429d6150   (167 bytes)
//===========================================================

void FUN_1429d6150(undefined8 param_1,undefined8 param_2)

{
  char cVar1;
  int iVar2;
  longlong lVar3;
  
  iVar2 = FUN_1406e8c20(param_2);
  if (iVar2 == 0) {
    cVar1 = FUN_1406e8ae0(param_2);
    if (cVar1 == '\0') {
      lVar3 = 0;
    }
    else {
      FUN_1406e8ae0(param_2);
      lVar3 = FUN_14019b780(&DAT_143ad68a0,0x668);
      if (lVar3 == 0) {
        lVar3 = 0;
      }
      else {
        lVar3 = FUN_141eb62e0(lVar3);
      }
      if (lVar3 == 0) {
        return;
      }
      cVar1 = FUN_141eb9760(lVar3,param_1,0,param_2);
      if (cVar1 == '\0') {
        return;
      }
    }
    FUN_1427707e0(param_1,0,lVar3);
  }
  return;
}



//===========================================================
// FUN_141eb9760 @ 141eb9760   (10998 bytes)
//===========================================================

/* WARNING: Heritage AFTER dead removal. Example location: s0x00000018 : 0x000141ebc11f */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

undefined8 FUN_141eb9760(longlong *param_1,longlong param_2,undefined4 param_3,undefined8 param_4)

{
  longlong *plVar1;
  ushort uVar2;
  longlong *plVar3;
  code *pcVar4;
  longlong *plVar5;
  IUnknown *pIVar6;
  undefined4 *puVar7;
  undefined1 uVar8;
  byte bVar9;
  char cVar10;
  short sVar11;
  undefined2 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined8 *puVar17;
  longlong *plVar18;
  longlong lVar19;
  longlong lVar20;
  undefined8 uVar21;
  undefined4 *puVar22;
  byte bVar23;
  int iVar24;
  byte *pbVar25;
  longlong *plVar26;
  uint uVar27;
  byte bVar28;
  uint uVar29;
  longlong *local_res8;
  undefined8 local_res10;
  undefined4 local_res18;
  undefined8 local_res20;
  undefined8 in_stack_fffffffffffffe28;
  undefined4 uVar31;
  uint *puVar30;
  undefined8 *puVar32;
  undefined8 *puVar33;
  longlong **pplVar34;
  longlong local_198;
  short local_190;
  undefined6 uStack_18e;
  longlong lStack_188;
  undefined8 local_180;
  short local_178;
  undefined6 uStack_176;
  longlong lStack_170;
  undefined8 local_168;
  short local_160;
  undefined6 uStack_15e;
  longlong lStack_158;
  undefined8 local_150;
  longlong *local_148;
  longlong *local_140;
  longlong *local_138;
  uint local_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined8 local_120;
  uint local_118;
  undefined4 uStack_114;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 local_100;
  undefined1 local_f8 [16];
  undefined8 local_e8;
  longlong lStack_e0;
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
  
  uVar31 = (undefined4)((ulonglong)in_stack_fffffffffffffe28 >> 0x20);
  iVar24 = (int)param_1[0x27] + 1;
  *(int *)(param_1 + 0x27) = iVar24;
  local_res8 = param_1;
  local_res10 = param_2;
  local_res18 = param_3;
  local_res20 = param_4;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)param_1[0x28];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    param_1[0x28] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(param_1[0x28] + 4) = uVar8;
  lVar20 = param_1[0x28];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar27 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  uVar29 = uVar27;
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(param_1[0x28] + 8);
    *(ushort *)(param_1[0x28] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(param_1[0x28] + 8);
    *(ushort *)(param_1[0x28] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar9 != 0) {
      bVar28 = bVar9;
    }
    bVar9 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar28 ^ bVar9;
    bVar9 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
    uVar2 = *(ushort *)(param_1[0x28] + 8);
    *(ushort *)(param_1[0x28] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar9 != 0) {
      bVar28 = bVar9;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
    uVar2 = *(ushort *)(param_1[0x28] + 8);
    *(ushort *)(param_1[0x28] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  param_1[0x24] = local_res10;
  uVar13 = FUN_14276df20(local_res10);
  uVar21 = local_res20;
  *(undefined4 *)(param_1 + 0x25) = uVar13;
  if (((longlong *)param_1[0x24] != (longlong *)0x0) &&
     (iVar24 = (**(code **)(*(longlong *)param_1[0x24] + 0x58))(), iVar24 != 0)) {
    return 0;
  }
  uVar13 = FUN_1406e8c20(uVar21);
  local_res10 = CONCAT44(local_res10._4_4_,uVar13);
  plVar18 = (longlong *)FUN_1406e9050(uVar21,&local_148);
  plVar5 = param_1 + 0x26;
  if (*plVar5 != 0) {
    FUN_14019f2c0(*plVar5 + -0x10);
    *plVar5 = 0;
  }
  *plVar5 = *plVar18;
  *plVar18 = 0;
  if (local_148 != (longlong *)0x0) {
    FUN_14019f2c0(local_148 + -2);
  }
  plVar18 = (longlong *)FUN_14040dab0(plVar5,&local_198,&DAT_143295a74,&DAT_143275b98);
  if (*plVar5 != 0) {
    FUN_14019f2c0(*plVar5 + -0x10);
    *plVar5 = 0;
  }
  *plVar5 = *plVar18;
  *plVar18 = 0;
  if (local_198 != 0) {
    FUN_14019f2c0(local_198 + -0x10);
  }
  FUN_1406e9170(uVar21,param_1 + 0x2a,8);
  local_198 = FUN_141ebdad0(param_1);
  if (local_198 != 0) {
    sVar11 = FUN_1401ab420(local_198 + 0x62,*(undefined4 *)(local_198 + 0x66));
    local_res18 = (uint)sVar11;
    iVar24 = (int)param_1[0x2b] + 1;
    *(int *)(param_1 + 0x2b) = iVar24;
    if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
      puVar33 = (undefined8 *)param_1[0x2c];
      puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      param_1[0x2c] = (longlong)puVar17;
      *puVar17 = *puVar33;
      *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
      thunk_FUN_140205820(puVar33,0xc);
    }
    uVar8 = FUN_142f04924();
    *(undefined1 *)(param_1[0x2c] + 4) = uVar8;
    lVar20 = param_1[0x2c];
    bVar28 = *(byte *)(lVar20 + 4);
    *(undefined2 *)(lVar20 + 8) = 0x9a65;
    pbVar25 = (byte *)(lVar20 + 2);
    uVar29 = uVar27;
    do {
      if (bVar28 == 0) {
        bVar28 = 0x2a;
      }
      bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
      pbVar25[-2] = bVar28 ^ bVar9;
      bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2c] + 8);
      *(ushort *)(param_1[0x2c] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      bVar9 = 0x2a;
      if (bVar28 != 0) {
        bVar9 = bVar28;
      }
      bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
      pbVar25[-1] = bVar9 ^ bVar28;
      bVar9 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2c] + 8);
      *(ushort *)(param_1[0x2c] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 - lVar20];
      *pbVar25 = bVar28 ^ bVar9;
      bVar9 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2c] + 8);
      *(ushort *)(param_1[0x2c] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
      pbVar25[1] = bVar28 ^ bVar9;
      bVar28 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2c] + 8);
      *(ushort *)(param_1[0x2c] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      uVar29 = uVar29 + 4;
      pbVar25 = pbVar25 + 4;
    } while (uVar29 < 4);
    bVar28 = FUN_1401b0050(local_198 + 0x6a,*(undefined4 *)(local_198 + 0x6e));
    local_res18 = (uint)bVar28;
    iVar24 = (int)param_1[0x2e] + 1;
    *(int *)(param_1 + 0x2e) = iVar24;
    if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
      puVar33 = (undefined8 *)param_1[0x2f];
      puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      param_1[0x2f] = (longlong)puVar17;
      *puVar17 = *puVar33;
      *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
      thunk_FUN_140205820(puVar33,0xc);
    }
    uVar8 = FUN_142f04924();
    *(undefined1 *)(param_1[0x2f] + 4) = uVar8;
    lVar20 = param_1[0x2f];
    bVar28 = *(byte *)(lVar20 + 4);
    *(undefined2 *)(lVar20 + 8) = 0x9a65;
    pbVar25 = (byte *)(lVar20 + 2);
    uVar29 = uVar27;
    do {
      if (bVar28 == 0) {
        bVar28 = 0x2a;
      }
      bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
      pbVar25[-2] = bVar28 ^ bVar9;
      bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2f] + 8);
      *(ushort *)(param_1[0x2f] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      bVar9 = 0x2a;
      if (bVar28 != 0) {
        bVar9 = bVar28;
      }
      bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
      pbVar25[-1] = bVar9 ^ bVar28;
      bVar9 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2f] + 8);
      *(ushort *)(param_1[0x2f] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 - lVar20];
      *pbVar25 = bVar28 ^ bVar9;
      bVar9 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2f] + 8);
      *(ushort *)(param_1[0x2f] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
      pbVar25[1] = bVar28 ^ bVar9;
      bVar28 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x2f] + 8);
      *(ushort *)(param_1[0x2f] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      uVar29 = uVar29 + 4;
      pbVar25 = pbVar25 + 4;
    } while (uVar29 < 4);
    sVar11 = FUN_1401ab420(local_198 + 0x72,*(undefined4 *)(local_198 + 0x76));
    local_res18 = (uint)sVar11;
    iVar24 = (int)param_1[0x31] + 1;
    *(int *)(param_1 + 0x31) = iVar24;
    if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
      puVar33 = (undefined8 *)param_1[0x32];
      puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      param_1[0x32] = (longlong)puVar17;
      *puVar17 = *puVar33;
      *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
      thunk_FUN_140205820(puVar33,0xc);
    }
    uVar8 = FUN_142f04924();
    *(undefined1 *)(param_1[0x32] + 4) = uVar8;
    lVar20 = param_1[0x32];
    bVar28 = *(byte *)(lVar20 + 4);
    *(undefined2 *)(lVar20 + 8) = 0x9a65;
    pbVar25 = (byte *)(lVar20 + 2);
    uVar29 = uVar27;
    do {
      if (bVar28 == 0) {
        bVar28 = 0x2a;
      }
      bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
      pbVar25[-2] = bVar28 ^ bVar9;
      bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
      uVar2 = *(ushort *)(param_1[0x32] + 8);
      *(ushort *)(param_1[0x32] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      bVar9 = 0x2a;
      if (bVar28 != 0) {
        bVar9 = bVar28;
      }
      bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
      pbVar25[-1] = bVar9 ^ bVar28;
      bVar9 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x32] + 8);
      *(ushort *)(param_1[0x32] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 - lVar20];
      *pbVar25 = bVar28 ^ bVar9;
      bVar9 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x32] + 8);
      *(ushort *)(param_1[0x32] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
      pbVar25[1] = bVar28 ^ bVar9;
      bVar28 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x32] + 8);
      *(ushort *)(param_1[0x32] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      uVar29 = uVar29 + 4;
      pbVar25 = pbVar25 + 4;
    } while (uVar29 < 4);
    local_res18 = FUN_1401ba9d0(local_198 + 0xa6,*(undefined4 *)(local_198 + 0xae));
    iVar24 = (int)param_1[0x43] + 1;
    *(int *)(param_1 + 0x43) = iVar24;
    if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
      puVar33 = (undefined8 *)param_1[0x44];
      puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      param_1[0x44] = (longlong)puVar17;
      *puVar17 = *puVar33;
      *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
      thunk_FUN_140205820(puVar33,0xc);
    }
    uVar8 = FUN_142f04924();
    *(undefined1 *)(param_1[0x44] + 4) = uVar8;
    lVar20 = param_1[0x44];
    bVar28 = *(byte *)(lVar20 + 4);
    *(undefined2 *)(lVar20 + 8) = 0x9a65;
    pbVar25 = (byte *)(lVar20 + 2);
    uVar29 = uVar27;
    do {
      if (bVar28 == 0) {
        bVar28 = 0x2a;
      }
      bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
      pbVar25[-2] = bVar28 ^ bVar9;
      bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
      uVar2 = *(ushort *)(param_1[0x44] + 8);
      *(ushort *)(param_1[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      bVar9 = 0x2a;
      if (bVar28 != 0) {
        bVar9 = bVar28;
      }
      bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
      pbVar25[-1] = bVar9 ^ bVar28;
      bVar9 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x44] + 8);
      *(ushort *)(param_1[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 - lVar20];
      *pbVar25 = bVar28 ^ bVar9;
      bVar9 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x44] + 8);
      *(ushort *)(param_1[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
      bVar28 = 0x2a;
      if (bVar9 != 0) {
        bVar28 = bVar9;
      }
      bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
      pbVar25[1] = bVar28 ^ bVar9;
      bVar28 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x44] + 8);
      *(ushort *)(param_1[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      uVar29 = uVar29 + 4;
      pbVar25 = pbVar25 + 4;
    } while (uVar29 < 4);
    uVar12 = FUN_1401ab420(local_198 + 0xb2,*(undefined4 *)(local_198 + 0xb6));
    uVar21 = local_res20;
    local_res18 = CONCAT22(local_res18._2_2_,uVar12);
    iVar24 = (int)param_1[0x46] + 1;
    *(int *)(param_1 + 0x46) = iVar24;
    if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
      puVar33 = (undefined8 *)param_1[0x47];
      puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,8);
      param_1[0x47] = (longlong)puVar17;
      *puVar17 = *puVar33;
      thunk_FUN_140205820(puVar33,8);
    }
    uVar8 = FUN_142f04924();
    *(undefined1 *)(param_1[0x47] + 2) = uVar8;
    lVar20 = param_1[0x47];
    bVar28 = *(byte *)(lVar20 + 2);
    *(undefined2 *)(lVar20 + 6) = 0x9a65;
    pbVar25 = (byte *)&local_res18;
    uVar29 = uVar27;
    do {
      if (bVar28 == 0) {
        bVar28 = 0x2a;
      }
      bVar9 = *pbVar25;
      pbVar25[lVar20 - (longlong)&local_res18] = bVar28 ^ bVar9;
      bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
      uVar2 = *(ushort *)(param_1[0x47] + 6);
      *(ushort *)(param_1[0x47] + 6) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      bVar9 = 0x2a;
      if (bVar28 != 0) {
        bVar9 = bVar28;
      }
      bVar28 = pbVar25[1];
      pbVar25[(lVar20 - (longlong)&local_res18) + 1] = bVar9 ^ bVar28;
      bVar28 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
      uVar2 = *(ushort *)(param_1[0x47] + 6);
      *(ushort *)(param_1[0x47] + 6) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
      uVar29 = uVar29 + 2;
      pbVar25 = pbVar25 + 2;
    } while (uVar29 < 2);
  }
  FUN_141ec20f0(param_1);
  sVar11 = FUN_1406e8b80(uVar21);
  local_res18 = (uint)sVar11;
  iVar24 = (int)param_1[0xb3] + 1;
  *(int *)(param_1 + 0xb3) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)param_1[0xb4];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    param_1[0xb4] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(param_1[0xb4] + 4) = uVar8;
  lVar20 = param_1[0xb4];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  pbVar25 = (byte *)(lVar20 + 2);
  uVar29 = uVar27;
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb4] + 8);
    *(ushort *)(param_1[0xb4] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb4] + 8);
    *(ushort *)(param_1[0xb4] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar9 != 0) {
      bVar28 = bVar9;
    }
    bVar9 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar28 ^ bVar9;
    bVar9 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb4] + 8);
    *(ushort *)(param_1[0xb4] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar9 != 0) {
      bVar28 = bVar9;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb4] + 8);
    *(ushort *)(param_1[0xb4] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  sVar11 = FUN_1406e8b80(local_res20);
  local_res18 = (uint)sVar11;
  plVar5 = param_1 + 0xb0;
  iVar24 = (int)*plVar5 + 1;
  *(int *)plVar5 = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)param_1[0xb1];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    param_1[0xb1] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  plVar18 = local_res8;
  *(undefined1 *)(param_1[0xb1] + 4) = uVar8;
  lVar20 = param_1[0xb1];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  pbVar25 = (byte *)(lVar20 + 2);
  uVar29 = uVar27;
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb1] + 8);
    *(ushort *)(param_1[0xb1] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb1] + 8);
    *(ushort *)(param_1[0xb1] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar9 != 0) {
      bVar28 = bVar9;
    }
    bVar9 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar28 ^ bVar9;
    bVar9 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb1] + 8);
    *(ushort *)(param_1[0xb1] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar9 != 0) {
      bVar28 = bVar9;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = (bVar28 ^ bVar9) + bVar28 + 0x2a;
    uVar2 = *(ushort *)(param_1[0xb1] + 8);
    *(ushort *)(param_1[0xb1] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  FUN_1409d3c60(local_res8 + 0xaa,plVar5);
  bVar28 = FUN_1406e8ae0(local_res20);
  local_res18 = (uint)bVar28;
  iVar24 = (int)plVar18[0x5b] + 1;
  *(int *)(plVar18 + 0x5b) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar18[0x5c];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar18[0x5c] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar18[0x5c] + 4) = uVar8;
  lVar20 = plVar18[0x5c];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar18[0x5c] + 8);
    *(ushort *)(plVar18[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar28 = (bVar9 ^ bVar28) + bVar9 + 0x2a;
    uVar2 = *(ushort *)(plVar18[0x5c] + 8);
    *(ushort *)(plVar18[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar18[0x5c] + 8);
    *(ushort *)(plVar18[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar9 != 0) {
      bVar28 = bVar9;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar18[0x5c] + 8);
    *(ushort *)(plVar18[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar21 = DAT_143ac18d8;
    uVar27 = uVar27 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar27 < 4);
  sVar11 = FUN_1406e8b80(local_res20);
  local_100 = FUN_142df6c50(uVar21,(int)sVar11);
  local_res18 = FUN_1406e8c20(local_res20);
  plVar5 = local_res8;
  iVar24 = (int)local_res8[0x43] + 1;
  *(int *)(local_res8 + 0x43) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)local_res8[0x44];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar5[0x44] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  uVar21 = local_res20;
  *(undefined1 *)(plVar5[0x44] + 4) = uVar8;
  lVar20 = plVar5[0x44];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x44] + 8);
    *(ushort *)(plVar5[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x44] + 8);
    *(ushort *)(plVar5[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar23 = 0x2a;
    if (bVar9 != 0) {
      bVar23 = bVar9;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar23 ^ bVar28;
    bVar23 = bVar23 + (bVar23 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x44] + 8);
    *(ushort *)(plVar5[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar23 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar23 != 0) {
      bVar28 = bVar23;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x44] + 8);
    *(ushort *)(plVar5[0x44] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  uVar13 = FUN_1406e8c20(local_res20);
  local_198 = CONCAT44(local_198._4_4_,uVar13);
  uVar12 = FUN_1406e8b80(uVar21);
  local_res18 = CONCAT22(local_res18._2_2_,uVar12);
  iVar24 = (int)plVar5[0x49] + 1;
  *(int *)(plVar5 + 0x49) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar5[0x4a];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,8);
    plVar5[0x4a] = (longlong)puVar17;
    *puVar17 = *puVar33;
    thunk_FUN_140205820(puVar33,8);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar5[0x4a] + 2) = uVar8;
  lVar20 = plVar5[0x4a];
  bVar28 = *(byte *)(lVar20 + 2);
  *(undefined2 *)(lVar20 + 6) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)&local_res18;
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = *pbVar25;
    pbVar25[lVar20 - (longlong)&local_res18] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x4a] + 6);
    *(ushort *)(plVar5[0x4a] + 6) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[1];
    pbVar25[(lVar20 - (longlong)&local_res18) + 1] = bVar9 ^ bVar28;
    bVar28 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x4a] + 6);
    *(ushort *)(plVar5[0x4a] + 6) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 2;
    pbVar25 = pbVar25 + 2;
  } while (uVar29 < 2);
  uVar12 = FUN_1406e8b80(uVar21);
  local_res18 = CONCAT22(local_res18._2_2_,uVar12);
  iVar24 = (int)plVar5[0x46] + 1;
  *(int *)(plVar5 + 0x46) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar5[0x47];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,8);
    plVar5[0x47] = (longlong)puVar17;
    *puVar17 = *puVar33;
    thunk_FUN_140205820(puVar33,8);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar5[0x47] + 2) = uVar8;
  lVar20 = plVar5[0x47];
  bVar28 = *(byte *)(lVar20 + 2);
  *(undefined2 *)(lVar20 + 6) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)&local_res18;
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = *pbVar25;
    pbVar25[lVar20 - (longlong)&local_res18] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x47] + 6);
    *(ushort *)(plVar5[0x47] + 6) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[1];
    pbVar25[(lVar20 - (longlong)&local_res18) + 1] = bVar9 ^ bVar28;
    bVar28 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x47] + 6);
    *(ushort *)(plVar5[0x47] + 6) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 2;
    pbVar25 = pbVar25 + 2;
  } while (uVar29 < 2);
  bVar28 = FUN_1406e8ae0(uVar21);
  local_res18 = (uint)bVar28;
  iVar24 = (int)plVar5[0x4f] + 1;
  *(int *)(plVar5 + 0x4f) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar5[0x50];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar5[0x50] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar5[0x50] + 4) = uVar8;
  lVar20 = plVar5[0x50];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x50] + 8);
    *(ushort *)(plVar5[0x50] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x50] + 8);
    *(ushort *)(plVar5[0x50] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar23 = 0x2a;
    if (bVar9 != 0) {
      bVar23 = bVar9;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar23 ^ bVar28;
    bVar23 = bVar23 + (bVar23 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x50] + 8);
    *(ushort *)(plVar5[0x50] + 8) = (uVar2 >> 0xd) + (ushort)bVar23 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar23 != 0) {
      bVar28 = bVar23;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x50] + 8);
    *(ushort *)(plVar5[0x50] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  bVar28 = FUN_1406e8ae0(local_res20);
  local_res18 = (uint)bVar28;
  iVar24 = (int)plVar5[0x52] + 1;
  *(int *)(plVar5 + 0x52) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar5[0x53];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar5[0x53] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar5[0x53] + 4) = uVar8;
  lVar20 = plVar5[0x53];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x53] + 8);
    *(ushort *)(plVar5[0x53] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x53] + 8);
    *(ushort *)(plVar5[0x53] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar23 = 0x2a;
    if (bVar9 != 0) {
      bVar23 = bVar9;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar23 ^ bVar28;
    bVar23 = bVar23 + (bVar23 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x53] + 8);
    *(ushort *)(plVar5[0x53] + 8) = (uVar2 >> 0xd) + (ushort)bVar23 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar23 != 0) {
      bVar28 = bVar23;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x53] + 8);
    *(ushort *)(plVar5[0x53] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  local_res18 = 0;
  iVar24 = (int)plVar5[0x4c] + 1;
  *(int *)(plVar5 + 0x4c) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar5[0x4d];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar5[0x4d] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar5[0x4d] + 4) = uVar8;
  lVar20 = plVar5[0x4d];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x4d] + 8);
    *(ushort *)(plVar5[0x4d] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x4d] + 8);
    *(ushort *)(plVar5[0x4d] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar23 = 0x2a;
    if (bVar9 != 0) {
      bVar23 = bVar9;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar23 ^ bVar28;
    bVar23 = bVar23 + (bVar23 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x4d] + 8);
    *(ushort *)(plVar5[0x4d] + 8) = (uVar2 >> 0xd) + (ushort)bVar23 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar23 != 0) {
      bVar28 = bVar23;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x4d] + 8);
    *(ushort *)(plVar5[0x4d] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  local_res18 = 0;
  iVar24 = (int)plVar5[0x74] + 1;
  *(int *)(plVar5 + 0x74) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar5[0x75];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar5[0x75] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar5[0x75] + 4) = uVar8;
  lVar20 = plVar5[0x75];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x75] + 8);
    *(ushort *)(plVar5[0x75] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x75] + 8);
    *(ushort *)(plVar5[0x75] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar23 = 0x2a;
    if (bVar9 != 0) {
      bVar23 = bVar9;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar23 ^ bVar28;
    bVar23 = bVar23 + (bVar23 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x75] + 8);
    *(ushort *)(plVar5[0x75] + 8) = (uVar2 >> 0xd) + (ushort)bVar23 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar23 != 0) {
      bVar28 = bVar23;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x75] + 8);
    *(ushort *)(plVar5[0x75] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  plVar18 = (longlong *)FUN_142b5a560();
  local_148 = plVar18;
  if (plVar18 == (longlong *)0x0) {
    iVar24 = -0x7fffbffe;
    plVar26 = (longlong *)0x0;
  }
  else {
    local_140 = (longlong *)0x0;
    iVar24 = (**(code **)plVar18[4])(plVar18 + 4,&DAT_143273488,&local_140);
    plVar26 = (longlong *)0x0;
    if (-1 < iVar24) {
      plVar26 = local_140;
    }
  }
  plVar1 = plVar5 + 0x5b;
  if (((iVar24 + 0x80000000U & 0x80000000) == 0) && (iVar24 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0();
  }
  plVar3 = (longlong *)plVar5[0x23];
  if (plVar3 != plVar26) {
    plVar5[0x23] = (longlong)plVar26;
    plVar26 = (longlong *)0x0;
    if (plVar3 != (longlong *)0x0) {
      (**(code **)(*plVar3 + 0x10))();
    }
  }
  if (plVar26 != (longlong *)0x0) {
    (**(code **)(*plVar26 + 0x10))(plVar26);
  }
  lVar19 = FUN_141ed3540(local_res10 & 0xffffffff);
  plVar5[0x21] = lVar19;
  lVar20 = lVar19;
  if ((int)local_198 != 0) {
    lVar19 = FUN_141ed3540();
    lVar20 = plVar5[0x21];
  }
  plVar5[0x22] = lVar19;
  if (lVar20 == 0) {
    return 0;
  }
  if (lVar19 == 0) {
    return 0;
  }
  FUN_142b599f0(plVar18,plVar5 + 1,*(undefined4 *)(lVar20 + 0x10));
  pcVar4 = *(code **)(*plVar18 + 0x118);
  uVar13 = FUN_14019a5d0(plVar1);
  uVar14 = FUN_14019a5d0(plVar5 + 0xaa);
  uVar15 = FUN_14019a5d0(plVar5 + 0xad);
  uVar16 = (**(code **)(*(longlong *)plVar5[0x24] + 0x50))();
  (*pcVar4)(local_148,uVar16,uVar15,uVar14,0,0,CONCAT44(uVar31,uVar13),local_100);
  local_res18 = FUN_1409c6d00(local_148);
  iVar24 = (int)*plVar1 + 1;
  *(int *)plVar1 = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar5[0x5c];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar5[0x5c] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  plVar18 = local_res8;
  *(undefined1 *)(plVar5[0x5c] + 4) = uVar8;
  lVar20 = plVar5[0x5c];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x5c] + 8);
    *(ushort *)(plVar5[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x5c] + 8);
    *(ushort *)(plVar5[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar23 = 0x2a;
    if (bVar9 != 0) {
      bVar23 = bVar9;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar23 ^ bVar28;
    bVar23 = bVar23 + (bVar23 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x5c] + 8);
    *(ushort *)(plVar5[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar23 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar23 != 0) {
      bVar28 = bVar23;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar5[0x5c] + 8);
    *(ushort *)(plVar5[0x5c] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    pIVar6 = DAT_143add050;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  if (DAT_143add050 == (IUnknown *)0x0) goto LAB_141ebc21e;
  local_res8 = (longlong *)0x0;
  iVar24 = (**(code **)(*(longlong *)DAT_143add050 + 0x1d8))(DAT_143add050,0,&local_res8);
  if (iVar24 < 0) {
    _com_issue_errorex(iVar24,pIVar6,(_GUID *)&DAT_14327fcd0);
  }
  plVar5 = (longlong *)plVar18[0x77];
  plVar26 = local_res8;
  if (plVar5 != local_res8) {
    plVar18[0x77] = (longlong)local_res8;
    plVar26 = (longlong *)0x0;
    if (plVar5 != (longlong *)0x0) {
      (**(code **)(*plVar5 + 0x10))();
    }
  }
  if (plVar26 != (longlong *)0x0) {
    (**(code **)(*plVar26 + 0x10))(plVar26);
  }
  pIVar6 = DAT_143add050;
  if (DAT_143add050 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  (*DAT_143262a20)(&local_160);
  if (DAT_143a8b8d8 == 8) {
    if (local_160 == 8) {
      local_160 = 0;
      if (lStack_158 != 0) {
        (*DAT_143ad5990)(lStack_158 + -4);
      }
    }
    else {
      iVar24 = (*DAT_143262a18)(&local_160);
      if (iVar24 < 0) goto LAB_141ebc23f;
    }
    local_160 = 8;
    if (DAT_143a8b8e0 == 0) {
      lStack_158 = FUN_1401a5fa0(0,0);
    }
    else {
      lStack_158 = FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
  }
  else {
    if ((local_160 == 8) && (local_160 = 0, lStack_158 != 0)) {
      (*DAT_143ad5990)(lStack_158 + -4);
    }
    iVar24 = (*DAT_143262a28)(&local_160,&DAT_143a8b8d8);
    if (iVar24 < 0) {
LAB_141ebc23f:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar24);
    }
  }
  (*DAT_143262a20)(&local_178);
  if (DAT_143a8b8d8 == 8) {
    if (local_178 == 8) {
      local_178 = 0;
      if (lStack_170 != 0) {
        (*DAT_143ad5990)(lStack_170 + -4);
      }
    }
    else {
      iVar24 = (*DAT_143262a18)(&local_178);
      if (iVar24 < 0) goto LAB_141ebc229;
    }
    local_178 = 8;
    if (DAT_143a8b8e0 == 0) {
      uVar29 = 0;
    }
    else {
      uVar29 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_170 = FUN_1401a5fa0(DAT_143a8b8e0,uVar29);
  }
  else {
    if ((local_178 == 8) && (local_178 = 0, lStack_170 != 0)) {
      (*DAT_143ad5990)(lStack_170 + -4);
    }
    iVar24 = (*DAT_143262a28)(&local_178,&DAT_143a8b8d8);
    if (iVar24 < 0) {
LAB_141ebc229:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar24);
    }
  }
  (*DAT_143262a20)(&local_190);
  if (DAT_143a8b8d8 == 8) {
    if (local_190 == 8) {
      local_190 = 0;
      if (lStack_188 != 0) {
        (*DAT_143ad5990)(lStack_188 + -4);
      }
    }
    else {
      iVar24 = (*DAT_143262a18)(&local_190);
      if (iVar24 < 0) goto LAB_141ebc231;
    }
    local_190 = 8;
    if (DAT_143a8b8e0 == 0) {
      uVar29 = 0;
    }
    else {
      uVar29 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
    }
    lStack_188 = FUN_1401a5fa0(DAT_143a8b8e0,uVar29);
  }
  else {
    if ((local_190 == 8) && (local_190 = 0, lStack_188 != 0)) {
      (*DAT_143ad5990)(lStack_188 + -4);
    }
    iVar24 = (*DAT_143262a28)(&local_190,&DAT_143a8b8d8);
    if (iVar24 < 0) {
LAB_141ebc231:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(iVar24);
    }
  }
  local_130 = CONCAT22(local_130._2_2_,3);
  uStack_128 = 0;
  local_res8 = (longlong *)0x0;
  local_e8 = CONCAT62(uStack_15e,local_160);
  lStack_e0 = lStack_158;
  local_d8 = local_150;
  local_c8 = CONCAT62(uStack_176,local_178);
  lStack_c0 = lStack_170;
  local_b8 = local_168;
  local_a8 = CONCAT62(uStack_18e,local_190);
  lStack_a0 = lStack_188;
  local_98 = local_180;
  local_88 = local_130;
  uStack_84 = uStack_12c;
  uStack_80 = 0;
  uStack_7c = uStack_124;
  local_78 = local_120;
  pplVar34 = &local_res8;
  puVar33 = &local_e8;
  puVar17 = &local_c8;
  puVar32 = &local_a8;
  puVar30 = &local_88;
  iVar24 = (**(code **)(*(longlong *)pIVar6 + 0x168))
                     (pIVar6,0,0,0,0,0,puVar30,puVar32,puVar17,puVar33,pplVar34);
  if (iVar24 < 0) {
    _com_issue_errorex(iVar24,pIVar6,(_GUID *)&DAT_14327fcd0);
  }
  plVar5 = (longlong *)plVar18[0x79];
  plVar26 = local_res8;
  if (plVar5 != local_res8) {
    plVar18[0x79] = (longlong)local_res8;
    plVar26 = (longlong *)0x0;
    if (plVar5 != (longlong *)0x0) {
      (**(code **)(*plVar5 + 0x10))();
    }
  }
  if (plVar26 != (longlong *)0x0) {
    (**(code **)(*plVar26 + 0x10))(plVar26);
  }
  if ((short)local_130 == 8) {
    local_130 = local_130 & 0xffff0000;
    if (CONCAT44(uStack_124,uStack_128) != 0) {
      (*DAT_143ad5990)(CONCAT44(uStack_124,uStack_128) + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_130);
  }
  if (local_190 == 8) {
    local_190 = 0;
    if (lStack_188 != 0) {
      (*DAT_143ad5990)(lStack_188 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_190);
  }
  if (local_178 == 8) {
    local_178 = 0;
    if (lStack_170 != 0) {
      (*DAT_143ad5990)(lStack_170 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_178);
  }
  if (local_160 == 8) {
    local_160 = 0;
    if (lStack_158 != 0) {
      (*DAT_143ad5990)(lStack_158 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_160);
  }
  pIVar6 = (IUnknown *)plVar18[0x79];
  if (pIVar6 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  uStack_110 = (longlong *)plVar18[0x77];
  local_118 = CONCAT22(local_118._2_2_,0xd);
  if (uStack_110 != (longlong *)0x0) {
    (**(code **)(*uStack_110 + 8))();
  }
  local_68 = local_118;
  uStack_64 = uStack_114;
  uStack_60 = (undefined4)uStack_110;
  uStack_5c = uStack_110._4_4_;
  local_58 = local_108;
  iVar24 = (**(code **)(*(longlong *)pIVar6 + 0x238))(pIVar6,&local_68);
  if (iVar24 < 0) {
    _com_issue_errorex(iVar24,pIVar6,(_GUID *)&DAT_14327fcb0);
  }
  if ((short)local_118 == 8) {
    local_118 = local_118 & 0xffff0000;
    if (uStack_110 != (longlong *)0x0) {
      (*DAT_143ad5990)((longlong)uStack_110 + -4);
    }
  }
  else {
    (*DAT_143262a18)(&local_118);
  }
  local_res8 = (longlong *)0x0;
  FUN_141ec2a10(plVar18,&local_res8);
  iVar24 = (**(code **)(*(longlong *)plVar18[0x24] + 0x50))();
  if ((iVar24 == 0) && (lVar20 = FUN_141892840(), lVar20 != 0)) {
    uVar21 = FUN_141892840();
    cVar10 = FUN_14183a640(uVar21);
    if (cVar10 == '\0') goto LAB_141ebbded;
  }
  else {
LAB_141ebbded:
    uVar31 = *(undefined4 *)(plVar18[0x22] + 0x18);
    uVar21 = FUN_141ece680(local_f8,local_148);
    local_138 = (longlong *)plVar18[0x79];
    if (local_138 != (longlong *)0x0) {
      (**(code **)(*local_138 + 8))();
    }
    FUN_141b054f0(plVar18,plVar18[0x26],&local_138,uVar21,0x3eb,uVar31,
                  (ulonglong)puVar30 & 0xffffffffffff0000,(ulonglong)puVar32 & 0xffffffffffffff00,
                  (ulonglong)puVar17 & 0xffffffffffff0000,(ulonglong)puVar33 & 0xffffffffffffff00,
                  (ulonglong)pplVar34 & 0xffffffff00000000,0,0xffffffff,0);
  }
  uVar8 = FUN_141ec7e90(plVar18);
  FUN_141ec7880(plVar18,uVar8,1);
  FUN_141ec87b0(plVar18);
  local_res18 = 0;
  iVar24 = (int)plVar18[0xb6] + 1;
  *(int *)(plVar18 + 0xb6) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar33 = (undefined8 *)plVar18[0xb7];
    puVar17 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    plVar18[0xb7] = (longlong)puVar17;
    *puVar17 = *puVar33;
    *(undefined4 *)(puVar17 + 1) = *(undefined4 *)(puVar33 + 1);
    thunk_FUN_140205820(puVar33,0xc);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar18[0xb7] + 4) = uVar8;
  lVar20 = plVar18[0xb7];
  bVar28 = *(byte *)(lVar20 + 4);
  *(undefined2 *)(lVar20 + 8) = 0x9a65;
  uVar29 = 0;
  pbVar25 = (byte *)(lVar20 + 2);
  do {
    if (bVar28 == 0) {
      bVar28 = 0x2a;
    }
    bVar9 = pbVar25[(longlong)&local_res10 + (6 - lVar20)];
    pbVar25[-2] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar18[0xb7] + 8);
    *(ushort *)(plVar18[0xb7] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    bVar9 = 0x2a;
    if (bVar28 != 0) {
      bVar9 = bVar28;
    }
    bVar28 = pbVar25[(longlong)&local_res10 + -lVar20 + 7];
    pbVar25[-1] = bVar9 ^ bVar28;
    bVar9 = bVar9 + (bVar9 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar18[0xb7] + 8);
    *(ushort *)(plVar18[0xb7] + 8) = (uVar2 >> 0xd) + (ushort)bVar9 | uVar2 << 3;
    bVar23 = 0x2a;
    if (bVar9 != 0) {
      bVar23 = bVar9;
    }
    bVar28 = pbVar25[(longlong)&local_res18 - lVar20];
    *pbVar25 = bVar23 ^ bVar28;
    bVar23 = bVar23 + (bVar23 ^ bVar28) + 0x2a;
    uVar2 = *(ushort *)(plVar18[0xb7] + 8);
    *(ushort *)(plVar18[0xb7] + 8) = (uVar2 >> 0xd) + (ushort)bVar23 | uVar2 << 3;
    bVar28 = 0x2a;
    if (bVar23 != 0) {
      bVar28 = bVar23;
    }
    bVar9 = pbVar25[(longlong)&local_res18 + -lVar20 + 1];
    pbVar25[1] = bVar28 ^ bVar9;
    bVar28 = bVar28 + (bVar28 ^ bVar9) + 0x2a;
    uVar2 = *(ushort *)(plVar18[0xb7] + 8);
    *(ushort *)(plVar18[0xb7] + 8) = (uVar2 >> 0xd) + (ushort)bVar28 | uVar2 << 3;
    uVar29 = uVar29 + 4;
    pbVar25 = pbVar25 + 4;
  } while (uVar29 < 4);
  local_res18 = local_res18 & 0xffffff00;
  iVar24 = (int)plVar18[0xb9] + 1;
  *(int *)(plVar18 + 0xb9) = iVar24;
  if (iVar24 == (iVar24 / 0x6f) * 0x6f) {
    puVar7 = (undefined4 *)plVar18[0xba];
    puVar22 = (undefined4 *)FUN_14019b780(&DAT_143ad68a0,6);
    plVar18[0xba] = (longlong)puVar22;
    *puVar22 = *puVar7;
    *(undefined2 *)(puVar22 + 1) = *(undefined2 *)(puVar7 + 1);
    thunk_FUN_140205820(puVar7,6);
  }
  uVar8 = FUN_142f04924();
  *(undefined1 *)(plVar18[0xba] + 1) = uVar8;
  pbVar25 = (byte *)plVar18[0xba];
  pbVar25[4] = 0x65;
  pbVar25[5] = 0x9a;
  bVar28 = pbVar25[1];
  if (pbVar25[1] == 0) {
    bVar28 = 0x2a;
  }
  *pbVar25 = bVar28 ^ (byte)local_res18;
  uVar2 = *(ushort *)(plVar18[0xba] + 4);
  *(ushort *)(plVar18[0xba] + 4) =
       (uVar2 >> 0xd) + (ushort)(byte)(bVar28 + (bVar28 ^ (byte)local_res18) + 0x2a) | uVar2 << 3;
  FUN_141ecaf00(plVar18);
  lVar20 = DAT_143ac87a0;
  if (DAT_143ac87a0 != 0) {
    if (((longlong *)plVar18[0x24] == (longlong *)0x0) ||
       (iVar24 = (**(code **)(*(longlong *)plVar18[0x24] + 0x50))(), iVar24 == 0)) {
      iVar24 = *(int *)(lVar20 + 0x5c);
      lVar20 = 1;
    }
    else {
      iVar24 = *(int *)(lVar20 + 0x58);
      lVar20 = 0;
    }
    *(int *)(plVar18 + 0x78) = (iVar24 * 0xff) / 100;
    pIVar6 = (IUnknown *)plVar18[0x77];
    if (pIVar6 == (IUnknown *)0x0) {
LAB_141ebc21e:
                    /* WARNING: Subroutine does not return */
      FUN_142ef3ac0(0x80004003);
    }
    iVar24 = (**(code **)(*(longlong *)pIVar6 + 0x68))
                       (pIVar6,*(undefined8 *)(DAT_143aa84a0 + 0x178 + lVar20 * 8));
    if (iVar24 < 0) {
      _com_issue_errorex(iVar24,pIVar6,(_GUID *)&DAT_14336fcd0);
    }
  }
  FUN_141ecf340(plVar18,plVar18 + 0xaa);
  return 1;
}



//===========================================================
// FUN_1427703d0 @ 1427703d0   (99 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x0001427703fd) */

undefined8 FUN_1427703d0(longlong param_1,int param_2)

{
  longlong lVar1;
  
  if (((param_2 == 0) && (lVar1 = *(longlong *)(param_1 + 0x11d8), lVar1 != 0)) &&
     (*(int *)(lVar1 + -8) != 0)) {
    return *(undefined8 *)(lVar1 + 8);
  }
  return 0;
}



//===========================================================
// FUN_141ec3f20 @ 141ec3f20   (117 bytes)
//===========================================================

void FUN_141ec3f20(longlong param_1,undefined8 param_2)

{
  undefined4 uVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined8 uVar4;
  
  uVar1 = 0;
  lVar3 = *(longlong *)(param_1 + 0x118) + -0x20;
  if (*(longlong *)(param_1 + 0x118) == 0) {
    lVar3 = 0;
  }
  uVar2 = FUN_14099fe20(lVar3);
  lVar3 = FUN_141892840();
  if (lVar3 != 0) {
    uVar4 = FUN_141892840();
    uVar1 = FUN_141829fd0(uVar4);
  }
  FUN_141d598b0(uVar2,param_2,0,uVar1,1,0);
  return;
}



//===========================================================
// FUN_141ec3fa0 @ 141ec3fa0   (155 bytes)
//===========================================================

void FUN_141ec3fa0(longlong param_1,undefined8 param_2)

{
  undefined8 uVar1;
  char cVar2;
  char cVar3;
  int iVar4;
  longlong local_res8;
  undefined1 local_res18 [16];
  
  if ((*(longlong *)(param_1 + 0x120) != 0) &&
     (iVar4 = FUN_140f8abc0(*(longlong *)(param_1 + 0x120) + 0x100), iVar4 != 0)) {
    return;
  }
  cVar2 = FUN_1406e8ae0(param_2);
  cVar3 = FUN_1406e8ae0(param_2);
  FUN_1406e9050(param_2,&local_res8);
  uVar1 = FUN_1401a5780(local_res18,local_res8);
  FUN_141ec6680(param_1,(int)cVar2,(int)cVar3,uVar1,0);
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  return;
}



//===========================================================
// FUN_141ec5980 @ 141ec5980   (1895 bytes)
//===========================================================

void FUN_141ec5980(longlong *param_1,undefined8 param_2)

{
  longlong *plVar1;
  longlong **pplVar2;
  ushort uVar3;
  undefined8 *puVar4;
  bool bVar5;
  longlong *plVar6;
  undefined1 uVar7;
  uint uVar8;
  undefined4 uVar9;
  undefined8 *puVar10;
  int *piVar11;
  undefined8 uVar12;
  longlong lVar13;
  undefined4 *puVar14;
  byte bVar15;
  byte bVar16;
  int iVar17;
  undefined4 uVar18;
  longlong lVar19;
  uint uVar20;
  byte *pbVar21;
  longlong *plVar22;
  longlong *plVar23;
  byte bVar24;
  wchar_t *pwVar25;
  longlong *local_res8;
  undefined2 local_res18;
  byte abStackX_1e [2];
  undefined4 local_res20;
  byte abStack_6a [2];
  undefined4 local_68;
  longlong *local_60;
  undefined1 local_58 [8];
  undefined1 local_50 [8];
  undefined1 local_48 [16];
  
  local_res8 = param_1;
  local_res18 = FUN_1406e8b80(param_2);
  iVar17 = (int)param_1[0x46] + 1;
  *(int *)(param_1 + 0x46) = iVar17;
  if (iVar17 == (iVar17 / 0x6f) * 0x6f) {
    puVar4 = (undefined8 *)param_1[0x47];
    puVar10 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,8);
    param_1[0x47] = (longlong)puVar10;
    *puVar10 = *puVar4;
    thunk_FUN_140205820(puVar4,8);
  }
  uVar7 = FUN_142f04924();
  *(undefined1 *)(param_1[0x47] + 2) = uVar7;
  lVar13 = param_1[0x47];
  bVar24 = *(byte *)(lVar13 + 2);
  *(undefined2 *)(lVar13 + 6) = 0x9a65;
  uVar20 = 0;
  pbVar21 = (byte *)&local_res18;
  do {
    if (bVar24 == 0) {
      bVar24 = 0x2a;
    }
    bVar15 = *pbVar21;
    pbVar21[lVar13 - (longlong)&local_res18] = bVar24 ^ bVar15;
    bVar24 = bVar24 + (bVar24 ^ bVar15) + 0x2a;
    uVar3 = *(ushort *)(param_1[0x47] + 6);
    *(ushort *)(param_1[0x47] + 6) = (uVar3 >> 0xd) + (ushort)bVar24 | uVar3 << 3;
    bVar15 = 0x2a;
    if (bVar24 != 0) {
      bVar15 = bVar24;
    }
    bVar24 = pbVar21[1];
    pbVar21[(lVar13 - (longlong)&local_res18) + 1] = bVar15 ^ bVar24;
    bVar24 = (bVar15 ^ bVar24) + bVar15 + 0x2a;
    uVar3 = *(ushort *)(param_1[0x47] + 6);
    *(ushort *)(param_1[0x47] + 6) = (uVar3 >> 0xd) + (ushort)bVar24 | uVar3 << 3;
    uVar20 = uVar20 + 2;
    pbVar21 = pbVar21 + 2;
  } while (uVar20 < 2);
  bVar24 = FUN_1406e8ae0(param_2);
  local_68 = (uint)bVar24;
  bVar24 = FUN_1406e8ae0(param_2);
  local_res20 = (uint)bVar24;
  iVar17 = (int)param_1[0x52] + 1;
  *(int *)(param_1 + 0x52) = iVar17;
  if (iVar17 == (iVar17 / 0x6f) * 0x6f) {
    puVar4 = (undefined8 *)param_1[0x53];
    puVar10 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    param_1[0x53] = (longlong)puVar10;
    *puVar10 = *puVar4;
    *(undefined4 *)(puVar10 + 1) = *(undefined4 *)(puVar4 + 1);
    thunk_FUN_140205820(puVar4,0xc);
  }
  uVar7 = FUN_142f04924();
  *(undefined1 *)(param_1[0x53] + 4) = uVar7;
  lVar13 = param_1[0x53];
  bVar24 = *(byte *)(lVar13 + 4);
  *(undefined2 *)(lVar13 + 8) = 0x9a65;
  uVar20 = 0;
  pbVar21 = (byte *)(lVar13 + 2);
  do {
    if (bVar24 == 0) {
      bVar24 = 0x2a;
    }
    bVar15 = pbVar21[(longlong)(abStackX_1e + -lVar13)];
    pbVar21[-2] = bVar24 ^ bVar15;
    bVar24 = bVar24 + (bVar24 ^ bVar15) + 0x2a;
    uVar3 = *(ushort *)(param_1[0x53] + 8);
    *(ushort *)(param_1[0x53] + 8) = (uVar3 >> 0xd) + (ushort)bVar24 | uVar3 << 3;
    bVar15 = 0x2a;
    if (bVar24 != 0) {
      bVar15 = bVar24;
    }
    bVar24 = pbVar21[(longlong)(abStackX_1e + -lVar13 + 1)];
    pbVar21[-1] = bVar15 ^ bVar24;
    bVar24 = (bVar15 ^ bVar24) + bVar15 + 0x2a;
    uVar3 = *(ushort *)(param_1[0x53] + 8);
    *(ushort *)(param_1[0x53] + 8) = (uVar3 >> 0xd) + (ushort)bVar24 | uVar3 << 3;
    bVar15 = 0x2a;
    if (bVar24 != 0) {
      bVar15 = bVar24;
    }
    bVar24 = pbVar21[(longlong)&local_res20 - lVar13];
    *pbVar21 = bVar15 ^ bVar24;
    bVar15 = bVar15 + (bVar15 ^ bVar24) + 0x2a;
    uVar3 = *(ushort *)(param_1[0x53] + 8);
    *(ushort *)(param_1[0x53] + 8) = (uVar3 >> 0xd) + (ushort)bVar15 | uVar3 << 3;
    bVar24 = 0x2a;
    if (bVar15 != 0) {
      bVar24 = bVar15;
    }
    bVar15 = pbVar21[(longlong)&local_res20 + -lVar13 + 1];
    pbVar21[1] = bVar24 ^ bVar15;
    bVar24 = bVar24 + (bVar24 ^ bVar15) + 0x2a;
    uVar3 = *(ushort *)(param_1[0x53] + 8);
    *(ushort *)(param_1[0x53] + 8) = (uVar3 >> 0xd) + (ushort)bVar24 | uVar3 << 3;
    uVar20 = uVar20 + 4;
    pbVar21 = pbVar21 + 4;
  } while (uVar20 < 4);
  piVar11 = (int *)param_1[0x22];
  if ((((piVar11 == (int *)0x0) && (piVar11 = (int *)param_1[0x21], piVar11 == (int *)0x0)) ||
      (9999 < *piVar11 - 5000000U)) || (999 < *piVar11 - 0x4c4f28U)) {
    bVar5 = false;
  }
  else {
    bVar5 = true;
  }
  uVar20 = 0;
  if (bVar5) {
    uVar20 = local_68;
  }
  local_60 = (longlong *)CONCAT44(local_60._4_4_,uVar20);
  iVar17 = FUN_141ec3380(param_1,0);
  if ((iVar17 != 0) && (uVar8 = FUN_1401b0340(param_1 + 0x61), 0x7fffffff < uVar8)) {
    FUN_141ec87b0(param_1);
  }
  plVar1 = param_1 + 0x4f;
  uVar8 = FUN_1409d4410(plVar1);
  if (uVar8 == uVar20) {
    return;
  }
  iVar17 = (int)*plVar1 + 1;
  *(int *)plVar1 = iVar17;
  local_68 = uVar20;
  if (iVar17 == (iVar17 / 0x6f) * 0x6f) {
    puVar4 = (undefined8 *)param_1[0x50];
    puVar10 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
    param_1[0x50] = (longlong)puVar10;
    *puVar10 = *puVar4;
    *(undefined4 *)(puVar10 + 1) = *(undefined4 *)(puVar4 + 1);
    thunk_FUN_140205820(puVar4,0xc);
  }
  uVar7 = FUN_142f04924();
  plVar6 = local_res8;
  *(undefined1 *)(param_1[0x50] + 4) = uVar7;
  lVar13 = param_1[0x50];
  bVar24 = *(byte *)(lVar13 + 4);
  *(undefined2 *)(lVar13 + 8) = 0x9a65;
  uVar20 = 0;
  pbVar21 = (byte *)(lVar13 + 2);
  do {
    if (bVar24 == 0) {
      bVar24 = 0x2a;
    }
    bVar15 = pbVar21[(longlong)(abStack_6a + -lVar13)];
    pbVar21[-2] = bVar24 ^ bVar15;
    bVar24 = bVar24 + (bVar24 ^ bVar15) + 0x2a;
    uVar3 = *(ushort *)(param_1[0x50] + 8);
    *(ushort *)(param_1[0x50] + 8) = (uVar3 >> 0xd) + (ushort)bVar24 | uVar3 << 3;
    bVar15 = 0x2a;
    if (bVar24 != 0) {
      bVar15 = bVar24;
    }
    bVar24 = pbVar21[(longlong)(abStack_6a + -lVar13 + 1)];
    pbVar21[-1] = bVar15 ^ bVar24;
    bVar15 = bVar15 + (bVar15 ^ bVar24) + 0x2a;
    uVar3 = *(ushort *)(param_1[0x50] + 8);
    *(ushort *)(param_1[0x50] + 8) = (uVar3 >> 0xd) + (ushort)bVar15 | uVar3 << 3;
    bVar16 = 0x2a;
    if (bVar15 != 0) {
      bVar16 = bVar15;
    }
    bVar24 = pbVar21[(longlong)&local_68 - lVar13];
    *pbVar21 = bVar16 ^ bVar24;
    bVar16 = bVar16 + (bVar16 ^ bVar24) + 0x2a;
    uVar3 = *(ushort *)(param_1[0x50] + 8);
    *(ushort *)(param_1[0x50] + 8) = (uVar3 >> 0xd) + (ushort)bVar16 | uVar3 << 3;
    bVar24 = 0x2a;
    if (bVar16 != 0) {
      bVar24 = bVar16;
    }
    bVar15 = pbVar21[(longlong)&local_68 + -lVar13 + 1];
    pbVar21[1] = bVar24 ^ bVar15;
    bVar24 = bVar24 + (bVar24 ^ bVar15) + 0x2a;
    uVar3 = *(ushort *)(param_1[0x50] + 8);
    *(ushort *)(param_1[0x50] + 8) = (uVar3 >> 0xd) + (ushort)bVar24 | uVar3 << 3;
    uVar20 = uVar20 + 4;
    pbVar21 = pbVar21 + 4;
  } while (uVar20 < 4);
  uVar20 = FUN_1401b0340(local_res8 + 0x61);
  plVar22 = local_60;
  if (uVar20 < 0x80000000) goto LAB_141ec60b6;
  lVar13 = plVar6[0x22];
  pwVar25 = L"transform";
  uVar12 = FUN_1401a5890(local_58,L"transform");
  uVar9 = FUN_141ed19e0(lVar13,uVar12);
  local_res8 = (longlong *)CONCAT44(local_res8._4_4_,uVar9);
  lVar13 = FUN_141ed0a40(plVar6[0x22],uVar9,(ulonglong)plVar22 & 0xffffffff);
  if (lVar13 == 0) goto LAB_141ec60b6;
  plVar22 = (longlong *)0x0;
  local_60 = (longlong *)0x0;
  if (*(longlong *)(lVar13 + 8) == 0) {
    local_res8 = (longlong *)0x0;
    plVar23 = plVar22;
  }
  else {
    iVar17 = *(int *)(*(longlong *)(lVar13 + 8) + -8);
    if (iVar17 != 0) {
      uVar20 = FUN_140d609a0(iVar17,0);
      lVar19 = *(longlong *)(lVar13 + 8);
      if (lVar19 == 0) {
        uVar8 = 0;
LAB_141ec5f8c:
        FUN_142e54290(0xc6,(ulonglong)uVar20,uVar8);
        lVar19 = *(longlong *)(lVar13 + 8);
      }
      else {
        uVar8 = *(uint *)(lVar19 + -8);
        if (uVar8 <= uVar20) goto LAB_141ec5f8c;
      }
      pplVar2 = (longlong **)(lVar19 + (ulonglong)uVar20 * 8);
      if ((&local_60 != pplVar2) &&
         (plVar22 = *pplVar2, local_60 = plVar22, plVar22 != (longlong *)0x0)) {
        LOCK();
        *(int *)(plVar22 + 2) = (int)plVar22[2] + 1;
        UNLOCK();
      }
      uVar9 = local_res8._0_4_;
    }
    plVar23 = (longlong *)0x0;
    local_res8 = plVar22;
    if (plVar22 != (longlong *)0x0) {
      LOCK();
      *(int *)(plVar22 + 2) = (int)plVar22[2] + 1;
      UNLOCK();
      plVar23 = local_60;
    }
  }
  FUN_141ec6680(plVar6,0,uVar9,&local_res8,0);
  iVar17 = FUN_142cc1e40(DAT_143aa84a0);
  if (iVar17 == 0) {
    lVar13 = (**(code **)(plVar6[1] + 0x30))(plVar6 + 1,local_50);
    uVar9 = *(undefined4 *)(lVar13 + 4);
    puVar14 = (undefined4 *)(**(code **)(plVar6[1] + 0x30))(plVar6 + 1,local_48);
    uVar9 = FUN_1429ed960(*puVar14,uVar9);
    iVar17 = FUN_1409d4840(plVar1);
    if (iVar17 != 0) {
      pwVar25 = L"transform_TA";
    }
    if ((undefined4 *)plVar6[0x22] == (undefined4 *)0x0) {
      if ((undefined4 *)plVar6[0x21] == (undefined4 *)0x0) {
        uVar18 = 0;
      }
      else {
        uVar18 = *(undefined4 *)plVar6[0x21];
      }
    }
    else {
      uVar18 = *(undefined4 *)plVar6[0x22];
    }
    FUN_1429f4e20(uVar18,pwVar25,uVar9);
  }
  plVar1 = local_60;
  if (plVar23 != (longlong *)0x0) {
    LOCK();
    plVar23 = plVar23 + 2;
    lVar13 = *plVar23;
    *(int *)plVar23 = (int)*plVar23 + -1;
    UNLOCK();
    if ((int)lVar13 == 1) {
      if (*local_60 != 0) {
        (*DAT_143ad5990)(*local_60 + -4);
        *plVar1 = 0;
      }
      if (plVar1[1] != 0) {
        FUN_14019b4e0();
        plVar1[1] = 0;
      }
      thunk_FUN_140205820(plVar1,0x18);
    }
  }
LAB_141ec60b6:
  uVar20 = FUN_1401b0340(plVar6 + 0x61);
  if (0x7fffffff < uVar20) {
    FUN_141ec87b0(plVar6);
  }
  return;
}


