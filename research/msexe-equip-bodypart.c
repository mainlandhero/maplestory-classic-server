
//===========================================================
// FUN_142d44b20 @ 142d44b20   (1146 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000142d44bc4) */

uint FUN_142d44b20(undefined8 param_1,longlong param_2,int param_3)

{
  undefined4 uVar1;
  int iVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong lVar6;
  undefined8 uVar7;
  uint *puVar8;
  uint uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  longlong lVar12;
  ulonglong uVar13;
  undefined1 uVar14;
  uint uVar15;
  uint uVar16;
  uint uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  uint *local_res10 [2];
  longlong local_res20;
  
  if (param_2 == 0) {
    return 0;
  }
  lVar12 = param_2 + 0x20;
  local_res20 = lVar12;
  uVar1 = FUN_14019a5d0(lVar12);
  iVar2 = FUN_140416820(uVar1);
  if (iVar2 != 0) {
    return 0x1c;
  }
  if (param_3 == 0) {
    lVar3 = FUN_142cbe730(param_1);
    uVar14 = *(undefined1 *)(lVar3 + 0x19);
  }
  else {
    uVar14 = 2;
  }
  uVar1 = FUN_14019a5d0(lVar12);
  FUN_140253f90(local_res10,uVar1,uVar14,param_3);
  puVar8 = local_res10[0];
  if (local_res10[0] != (uint *)0x0) {
    uVar9 = local_res10[0][-2];
    lVar3 = (longlong)(int)uVar9;
    if (uVar9 != 0) {
      if (uVar9 == 1) {
        uVar17 = *local_res10[0];
      }
      else if (param_3 == 0) {
        iVar2 = FUN_14019a5d0(lVar12);
        if (iVar2 - 1800000U < 100000) {
          uVar16 = 0;
          lVar12 = 0;
          uVar9 = uVar16;
          uVar15 = uVar16;
          do {
            uVar17 = uVar16;
            if (local_res10[0] != (uint *)0x0) {
              uVar17 = local_res10[0][-2];
            }
            if (((int)uVar15 < 0) || (uVar17 <= uVar15)) {
              uVar17 = 0;
              if (local_res10[0] != (uint *)0x0) {
                uVar17 = local_res10[0][-2];
              }
              FUN_142e54290(0xbc,uVar15,uVar17);
            }
            uVar17 = *(uint *)(lVar12 + (longlong)local_res10[0]);
            uVar4 = FUN_142cbe730(param_1);
            lVar3 = FUN_1402e51d0(uVar4,uVar17);
            uVar4 = DAT_143aa8328;
            uVar1 = FUN_14019a5d0(local_res20);
            lVar5 = FUN_140388c60(uVar4,uVar1);
            if ((lVar3 != 0) && (lVar5 != 0)) {
              uVar1 = FUN_14019a5d0(lVar3 + 0x20);
              iVar2 = FUN_1403838e0(lVar5,uVar1);
              if (iVar2 != 0) {
                iVar2 = FUN_142cbed90(param_1,uVar17);
                if ((iVar2 == 0) &&
                   (iVar2 = FUN_142cbee30(param_1,uVar17), puVar8 = local_res10[0], iVar2 == 0))
                break;
                if (uVar9 == 0) {
                  uVar9 = uVar17;
                }
              }
            }
            uVar15 = uVar15 + 1;
            lVar12 = lVar12 + 4;
            puVar8 = local_res10[0];
            uVar17 = uVar9;
          } while ((int)uVar15 < 1);
        }
        else {
          iVar2 = FUN_14019a5d0(lVar12);
          uVar9 = 0;
          uVar18 = 0;
          if (((iVar2 - 0x10eff0U < 10000) &&
              (iVar2 = FUN_14019a5d0(lVar12), iVar2 - 0x10f8ecU < 100)) && (0 < lVar3)) {
            lVar5 = 0;
            uVar11 = uVar18;
            do {
              uVar16 = (uint)uVar11;
              lVar6 = FUN_142cbe730(param_1);
              uVar15 = 0;
              if (local_res10[0] != (uint *)0x0) {
                uVar15 = local_res10[0][-2];
              }
              if (((int)uVar16 < 0) || (uVar15 <= uVar16)) {
                uVar10 = uVar18;
                if (local_res10[0] != (uint *)0x0) {
                  uVar10 = (ulonglong)local_res10[0][-2];
                }
                FUN_142e54290(0xbc,uVar11,uVar10);
              }
              lVar6 = *(longlong *)(lVar6 + 0x1b0 + (longlong)(int)local_res10[0][lVar5] * 0x10);
              if ((lVar6 != 0) && (iVar2 = FUN_14019a5d0(lVar6 + 0x20), iVar2 - 0x10f8ecU < 100))
              goto LAB_142d44f30;
              uVar11 = (ulonglong)(uVar16 + 1);
              lVar5 = lVar5 + 1;
            } while (lVar5 < lVar3);
          }
          FUN_1408f6690();
          uVar7 = FUN_142cbe730(param_1);
          uVar4 = DAT_143aa8328;
          uVar1 = FUN_1401b0340(lVar12);
          iVar2 = FUN_140389c10(uVar4,uVar1);
          if ((iVar2 != 0) || (uVar11 = uVar18, *(longlong *)(param_2 + 0x38) != 0)) {
            uVar11 = 1;
          }
          uVar10 = uVar18;
          uVar13 = uVar18;
          if (0 < lVar3) {
            do {
              uVar16 = (uint)uVar10;
              uVar15 = uVar9;
              if (local_res10[0] != (uint *)0x0) {
                uVar15 = local_res10[0][-2];
              }
              if (((int)uVar16 < 0) || (uVar15 <= uVar16)) {
                uVar19 = uVar18;
                if (local_res10[0] != (uint *)0x0) {
                  uVar19 = (ulonglong)local_res10[0][-2];
                }
                FUN_142e54290(0xbc,uVar10,uVar19);
              }
              iVar2 = FUN_140253420(local_res10[0][uVar13],uVar11);
              if (((-iVar2 - 0x4b0U < 0xe) || (-iVar2 - 0x708U < 0x33)) ||
                 (uVar15 = uVar9, iVar2 + 0x83U < 0x1f)) {
                uVar15 = 1;
              }
              if ((((uint)uVar11 == uVar15) && (lVar12 = FUN_1402e3a70(uVar7,iVar2), lVar12 != 0))
                 && (*(longlong *)(lVar12 + 8) == 0)) goto LAB_142d44f30;
              uVar13 = uVar13 + 1;
              uVar10 = (ulonglong)(uVar16 + 1);
            } while ((longlong)uVar13 < lVar3);
          }
          if ((local_res10[0] == (uint *)0x0) || (local_res10[0][-2] == 0)) {
            FUN_142e54290(0xbc,0,0);
          }
          puVar8 = local_res10[0];
          uVar17 = *local_res10[0];
        }
      }
      else {
        uVar15 = 0;
        uVar17 = uVar15;
        if (0 < (int)uVar9) {
          lVar12 = 0;
          uVar9 = uVar15;
          do {
            uVar16 = uVar15;
            if (puVar8 != (uint *)0x0) {
              uVar16 = puVar8[-2];
            }
            if (((int)uVar9 < 0) || (uVar16 <= uVar9)) {
              uVar16 = 0;
              if (puVar8 != (uint *)0x0) {
                uVar16 = puVar8[-2];
              }
              FUN_142e54290(0xbc,uVar9,uVar16);
              puVar8 = local_res10[0];
            }
            if (puVar8[lVar12] - 0x4b0 < 0xe) {
              uVar16 = uVar15;
              if (puVar8 != (uint *)0x0) {
                uVar16 = puVar8[-2];
              }
              if (((int)uVar9 < 0) || (uVar16 <= uVar9)) {
                if (puVar8 != (uint *)0x0) {
                  uVar15 = puVar8[-2];
                }
                FUN_142e54290(0xbc,uVar9,uVar15);
                puVar8 = local_res10[0];
              }
              uVar17 = puVar8[(int)uVar9];
              break;
            }
            uVar9 = uVar9 + 1;
            lVar12 = lVar12 + 1;
            uVar17 = 0;
          } while (lVar12 < lVar3);
        }
      }
      goto LAB_142d44f73;
    }
  }
  uVar17 = 0;
LAB_142d44f73:
  if (puVar8 != (uint *)0x0) {
    thunk_FUN_140205820(puVar8 + -2,0);
  }
  return uVar17;
LAB_142d44f30:
  if (local_res10[0] != (uint *)0x0) {
    uVar9 = local_res10[0][-2];
  }
  if (((int)uVar16 < 0) || (uVar9 <= uVar16)) {
    if (local_res10[0] != (uint *)0x0) {
      uVar18 = (ulonglong)local_res10[0][-2];
    }
    FUN_142e54290(0xbc,uVar16,uVar18);
  }
  puVar8 = local_res10[0];
  uVar17 = local_res10[0][(int)uVar16];
  goto LAB_142d44f73;
}



//===========================================================
// FUN_142cbee30 @ 142cbee30   (143 bytes)
//===========================================================

bool FUN_142cbee30(longlong param_1,uint param_2)

{
  longlong lVar1;
  uint uVar2;
  uint uVar3;
  undefined4 uVar4;
  
  if (-1 < (int)param_2) {
    lVar1 = *(longlong *)(param_1 + 0x2598);
    uVar4 = 0;
    uVar3 = 0;
    uVar2 = uVar3;
    if (lVar1 != 0) {
      uVar2 = *(uint *)(lVar1 + -8);
    }
    if ((int)param_2 < (int)uVar2) {
      if (lVar1 != 0) {
        uVar3 = *(uint *)(lVar1 + -8);
      }
      if (uVar3 <= param_2) {
        if (lVar1 != 0) {
          uVar4 = *(undefined4 *)(lVar1 + -8);
        }
        FUN_142e54290(0xc6,param_2,uVar4);
        lVar1 = *(longlong *)(param_1 + 0x2598);
      }
      return *(longlong *)(lVar1 + 8 + (longlong)(int)param_2 * 0x10) != 0;
    }
  }
  return false;
}



//===========================================================
// FUN_1401b0340 @ 1401b0340   (1057 bytes)
//===========================================================

ulonglong FUN_1401b0340(int *param_1)

{
  uint *puVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  undefined1 uVar5;
  ushort uVar6;
  ushort uVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  byte bVar10;
  byte bVar11;
  int iVar12;
  uint uVar13;
  byte *pbVar14;
  longlong lVar15;
  ushort uVar16;
  int iVar17;
  ushort uVar18;
  byte *pbVar19;
  uint uVar20;
  ulonglong uVar21;
  undefined1 local_res8 [8];
  undefined1 local_res10 [8];
  byte local_res18 [8];
  ushort local_res20 [4];
  undefined4 local_78;
  undefined4 local_70;
  undefined4 local_6c;
  ulonglong local_68;
  undefined8 local_60;
  longlong local_58 [3];
  
  puVar1 = *(uint **)(param_1 + 2);
  local_70 = *puVar1;
  uVar20 = 0;
  uVar13 = 0;
  local_res18[0] = (byte)puVar1[1];
  local_res20[0] = 0x9a65;
  pbVar19 = (byte *)&local_70;
  pbVar14 = (byte *)((longlong)puVar1 + 2);
  do {
    if (local_res18[0] == 0) {
      local_res18[0] = 0x2a;
    }
    bVar10 = pbVar19[(longlong)puVar1 - (longlong)&local_70];
    *pbVar19 = bVar10 ^ local_res18[0];
    bVar10 = bVar10 + local_res18[0] + 0x2a;
    uVar16 = (local_res20[0] >> 0xd) + (ushort)bVar10;
    uVar18 = local_res20[0] << 3;
    if (bVar10 == 0) {
      bVar10 = 0x2a;
    }
    bVar11 = pbVar14[-1];
    pbVar19[1] = bVar11 ^ bVar10;
    bVar11 = bVar11 + bVar10 + 0x2a;
    uVar6 = (ushort)bVar11;
    if (bVar11 == 0) {
      bVar11 = 0x2a;
    }
    local_res18[0] = *pbVar14;
    pbVar19[2] = local_res18[0] ^ bVar11;
    local_res18[0] = local_res18[0] + bVar11 + 0x2a;
    uVar7 = (ushort)local_res18[0];
    if (local_res18[0] == 0) {
      local_res18[0] = 0x2a;
    }
    bVar10 = pbVar14[1];
    pbVar19[3] = bVar10 ^ local_res18[0];
    local_res18[0] = bVar10 + 0x2a + local_res18[0];
    local_res20[0] =
         ((uVar16 | uVar18 & 0x3ff) >> 7) + (ushort)local_res18[0] |
         (((uVar18 & 0x1fff) >> 10) + uVar7 |
         (((local_res20[0] & 0x1fff) >> 10) + uVar6 | (uVar16 | uVar18) << 3) << 3) << 3;
    uVar13 = uVar13 + 4;
    pbVar19 = pbVar19 + 4;
    pbVar14 = pbVar14 + 4;
  } while (uVar13 < 4);
  FUN_140c78f50(0x56,(longlong)param_1 + 0x21a3f060fc2daf);
  uVar13 = local_70;
  lVar2 = *(longlong *)(param_1 + 2);
  uVar21 = (ulonglong)(int)local_70;
  if (((local_res20[0] != *(ushort *)(lVar2 + 8)) || ((char)param_1[1] != *(char *)(lVar2 + 5))) ||
     ((char)param_1[4] != *(char *)(lVar2 + 6))) {
    local_res8[0] = (undefined1)param_1[4];
    local_res10[0] = (undefined1)param_1[1];
    local_6c = 1;
    local_68 = uVar21;
    local_60 = FUN_1418039d0(5);
    puVar8 = (undefined8 *)
             FUN_140197ac0(local_58,&local_60,&local_6c,&local_68,local_res18,local_res20,
                           (ushort *)(lVar2 + 8),local_res10,lVar2 + 5,local_res8,lVar2 + 6);
    FUN_141804970(&DAT_143271f04,0x61,5,*puVar8);
    if (local_58[0] != 0) {
      FUN_14019f2c0(local_58[0] + -0x10);
    }
  }
  iVar12 = *param_1;
  iVar17 = iVar12 + 1;
  *param_1 = iVar17;
  if (iVar17 == (iVar17 / 0x37) * 0x37) {
    local_78 = uVar13;
    iVar12 = iVar12 + 2;
    *param_1 = iVar12;
    if (iVar12 == (iVar12 / 0x6f) * 0x6f) {
      puVar8 = *(undefined8 **)(param_1 + 2);
      puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      *(undefined8 **)(param_1 + 2) = puVar9;
      *puVar9 = *puVar8;
      *(undefined4 *)(puVar9 + 1) = *(undefined4 *)(puVar8 + 1);
      thunk_FUN_140205820(puVar8,0xc);
    }
    uVar5 = FUN_142f04924();
    *(undefined1 *)(*(longlong *)(param_1 + 2) + 4) = uVar5;
    pbVar19 = *(byte **)(param_1 + 2);
    bVar10 = pbVar19[4];
    pbVar19[8] = 0x65;
    pbVar19[9] = 0x9a;
    lVar15 = (longlong)&local_78 - (longlong)pbVar19;
    lVar2 = 1 - (longlong)pbVar19;
    lVar3 = 2 - (longlong)pbVar19;
    lVar4 = 3 - (longlong)pbVar19;
    do {
      if (bVar10 == 0) {
        bVar10 = 0x2a;
      }
      bVar11 = pbVar19[lVar15];
      *pbVar19 = bVar10 ^ bVar11;
      bVar10 = bVar10 + (bVar10 ^ bVar11) + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar10 | uVar16 << 3;
      bVar11 = 0x2a;
      if (bVar10 != 0) {
        bVar11 = bVar10;
      }
      bVar10 = pbVar19[(longlong)&local_78 + lVar2];
      pbVar19[1] = bVar11 ^ bVar10;
      bVar11 = (bVar11 ^ bVar10) + bVar11 + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar11 | uVar16 << 3;
      bVar10 = 0x2a;
      if (bVar11 != 0) {
        bVar10 = bVar11;
      }
      bVar11 = pbVar19[(longlong)&local_78 + lVar3];
      pbVar19[2] = bVar10 ^ bVar11;
      bVar11 = (bVar10 ^ bVar11) + bVar10 + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar11 | uVar16 << 3;
      bVar10 = 0x2a;
      if (bVar11 != 0) {
        bVar10 = bVar11;
      }
      bVar11 = pbVar19[(longlong)&local_78 + lVar4];
      pbVar19[3] = bVar10 ^ bVar11;
      bVar10 = (bVar10 ^ bVar11) + bVar10 + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar10 | uVar16 << 3;
      uVar20 = uVar20 + 4;
      pbVar19 = pbVar19 + 4;
    } while (uVar20 < 4);
    uVar21 = (ulonglong)local_70;
  }
  FUN_140c79130(0x67,param_1 + 0xc0a84bdc4a);
  return uVar21 & 0xffffffff;
}



//===========================================================
// FUN_142cbe730 @ 142cbe730   (8 bytes)
//===========================================================

undefined8 FUN_142cbe730(longlong param_1)

{
  return *(undefined8 *)(param_1 + 0x2358);
}



//===========================================================
// FUN_140253f90 @ 140253f90   (1084 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

longlong * FUN_140253f90(longlong *param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 *puVar3;
  int iVar4;
  undefined8 uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  undefined1 auVar8 [4];
  ulonglong uVar9;
  undefined8 *puVar10;
  uint uVar11;
  undefined8 *puVar12;
  undefined8 *puVar13;
  undefined8 *puVar14;
  bool bVar15;
  undefined1 auStack_268 [32];
  char local_248;
  char local_247;
  undefined8 *local_240;
  undefined4 local_238;
  undefined4 local_234;
  undefined8 *local_230;
  undefined8 uStack_228;
  undefined1 local_220;
  uint7 uStack_21f;
  undefined8 *local_210;
  undefined4 uStack_208;
  undefined4 uStack_204;
  longlong local_200;
  longlong *local_1f8;
  undefined1 local_1e8 [4];
  undefined8 auStack_1e4 [50];
  undefined1 local_54 [4];
  longlong *local_50;
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_268;
  local_238 = 0;
  local_1f8 = param_1;
  FUN_142ef8250(local_1e8,0,0x194);
  local_50 = (longlong *)0x0;
  FUN_1402543e0(local_1e8,param_2,param_3,param_4);
  *param_1 = 0;
  local_238 = 1;
  iVar4 = (int)_local_1e8;
  if (iVar4 == 0) {
    if ((local_50 != (longlong *)0x0) && (local_50[1] != 0)) {
      uVar5 = 0x65d;
      uVar9 = local_50[1] & 0xffffffff;
LAB_14025409c:
      FUN_142e52dd0(uVar5,uVar9);
LAB_1402540a1:
      if (*param_1 != 0) {
        uVar9 = *(ulonglong *)(*param_1 + -0x10);
        uVar6 = ~uVar9;
        if (-1 < (longlong)uVar9) {
          uVar6 = uVar9;
        }
        iVar4 = (int)(uVar6 - 8 >> 2);
        goto LAB_1402540d2;
      }
    }
  }
  else if (((int)(iVar4 - 1U) < 0) || (99 < iVar4 - 1U)) {
    if (local_50 == (longlong *)0x0) {
      uVar5 = 0x677;
      uVar9 = _local_1e8;
      goto LAB_14025409c;
    }
    if ((longlong)iVar4 != local_50[1] + 100) {
      FUN_142e54290();
      goto LAB_1402540a1;
    }
  }
  else if ((local_50 != (longlong *)0x0) && (local_50[1] != 0)) {
    FUN_142e54290(0x666,_local_1e8,local_50[1] & 0xffffffff);
    goto LAB_1402540a1;
  }
  iVar4 = 0;
LAB_1402540d2:
  auVar8 = (undefined1  [4])(int)_local_1e8;
  if (iVar4 != (int)_local_1e8) {
    if (*param_1 == 0) {
      uVar11 = 0;
    }
    else {
      uVar11 = *(uint *)(*param_1 + -8);
    }
    lVar2 = FUN_14019b780(&DAT_143ad68a0,(_local_1e8 & 0xffffffff) * 4 + 8);
    if (lVar2 == 0) {
      lVar2 = 0;
    }
    else {
      lVar2 = lVar2 + 8;
    }
    if (*param_1 != 0) {
      FUN_142ef7ba0(lVar2,*param_1,(ulonglong)uVar11 << 2);
      thunk_FUN_140205820(*param_1 + -8,0);
    }
    *param_1 = lVar2;
    *(ulonglong *)(lVar2 + -8) = (ulonglong)uVar11;
    auVar8 = local_1e8;
  }
  uStack_228 = 0;
  puVar14 = (undefined8 *)(local_1e8 + 4);
  local_220 = 0;
  uStack_208 = 0;
  uStack_204 = 0;
  local_200 = (ulonglong)uStack_21f << 8;
  puVar12 = (undefined8 *)0x0;
  if ((local_50 == (longlong *)0x0) || (local_50[1] == 0)) {
    puVar10 = (undefined8 *)(local_1e8 + (longlong)(int)auVar8 * 4 + 4);
    local_248 = '\0';
  }
  else {
    puVar10 = (undefined8 *)local_54;
    puVar12 = (undefined8 *)*local_50;
    local_248 = '\x01';
  }
  local_247 = '\0';
  puVar13 = (undefined8 *)0x0;
  local_240 = puVar13;
  local_230 = puVar14;
  local_210 = puVar14;
  do {
    plVar1 = local_50;
    if (local_247 == '\0') {
      if (local_248 == '\0') {
        bVar15 = puVar14 == puVar10;
        goto LAB_1402541d8;
      }
LAB_1402541e8:
      puVar3 = puVar14;
    }
    else {
      if (local_248 != '\0') {
        if ((puVar14 == puVar10) && (puVar13 == puVar12)) {
          bVar15 = true;
        }
        else {
          bVar15 = false;
        }
LAB_1402541d8:
        if (bVar15) {
          if (local_50 != (longlong *)0x0) {
            puVar14 = (undefined8 *)*local_50;
            *(undefined8 *)puVar14[1] = 0;
            puVar14 = (undefined8 *)*puVar14;
            while (puVar14 != (undefined8 *)0x0) {
              puVar12 = (undefined8 *)*puVar14;
              thunk_FUN_140205820(puVar14,0x18);
              puVar14 = puVar12;
            }
            thunk_FUN_140205820(*plVar1,0x18);
            thunk_FUN_140205820(plVar1,0x10);
          }
          return param_1;
        }
      }
      puVar3 = puVar13 + 2;
      if (local_247 == '\0') goto LAB_1402541e8;
    }
    local_234 = *(undefined4 *)puVar3;
    lVar2 = *param_1;
    if (lVar2 == 0) {
      uVar11 = 0;
      uVar9 = 1;
LAB_14025423c:
      if (lVar2 == 0) {
        iVar4 = 0;
      }
      else {
        uVar6 = *(ulonglong *)(lVar2 + -0x10);
        uVar7 = ~uVar6;
        if (-1 < (longlong)uVar6) {
          uVar7 = uVar6;
        }
        iVar4 = (int)(uVar7 - 8 >> 2);
      }
      puVar13 = local_240;
      if (iVar4 != (int)uVar9) {
        if (lVar2 == 0) {
          uVar6 = 0;
        }
        else {
          uVar6 = (ulonglong)*(uint *)(lVar2 + -8);
        }
        lVar2 = FUN_14019b780(&DAT_143ad68a0,uVar9 * 4 + 8);
        if (lVar2 == 0) {
          lVar2 = 0;
        }
        else {
          lVar2 = lVar2 + 8;
        }
        if (*param_1 != 0) {
          FUN_142ef7ba0(lVar2,*param_1,uVar6 << 2);
          thunk_FUN_140205820(*param_1 + -8,0);
        }
        *param_1 = lVar2;
        *(ulonglong *)(lVar2 + -8) = uVar6;
        puVar13 = local_240;
      }
    }
    else {
      uVar11 = *(uint *)(lVar2 + -8);
      uVar9 = *(ulonglong *)(lVar2 + -0x10);
      uVar6 = ~uVar9;
      if (-1 < (longlong)uVar9) {
        uVar6 = uVar9;
      }
      if ((uint)(uVar6 - 8 >> 2) <= uVar11) {
        if (uVar11 == 0) {
          uVar9 = 1;
        }
        else {
          uVar9 = (ulonglong)(uVar11 * 2);
        }
        goto LAB_14025423c;
      }
    }
    *(longlong *)(*param_1 + -8) = *(longlong *)(*param_1 + -8) + 1;
    *(undefined4 *)(*param_1 + (longlong)(int)uVar11 * 4) = local_234;
    if (puVar14 == (undefined8 *)local_54) {
      puVar13 = (undefined8 *)*puVar13;
      local_240 = puVar13;
    }
    else {
      puVar14 = (undefined8 *)((longlong)puVar14 + 4);
      if (((puVar14 == (undefined8 *)local_54) && (local_50 != (longlong *)0x0)) &&
         (local_50[1] != 0)) {
        puVar13 = *(undefined8 **)*local_50;
        local_247 = '\x01';
        local_240 = puVar13;
      }
    }
  } while( true );
}



//===========================================================
// FUN_1402e51d0 @ 1402e51d0   (194 bytes)
//===========================================================

longlong FUN_1402e51d0(undefined8 param_1,int param_2)

{
  undefined8 uVar1;
  undefined4 uVar2;
  int iVar3;
  longlong lVar4;
  bool bVar5;
  
  if (param_2 != 0xe) {
    return 0;
  }
  lVar4 = FUN_1402e5140();
  uVar1 = DAT_143aa8328;
  if (lVar4 != 0) {
    uVar2 = FUN_1401b0340(lVar4 + 0x20);
    iVar3 = FUN_14038a300(uVar1,uVar2);
    uVar1 = DAT_143aa8328;
    if (iVar3 == 0) {
      uVar2 = FUN_1401b0340(lVar4 + 0x20);
      iVar3 = FUN_14038a380(uVar1,uVar2);
      if (iVar3 == 0) {
        iVar3 = (*DAT_143ad5648)(lVar4 + 0x82,&DAT_14327dd88);
        bVar5 = -1 < iVar3;
      }
      else {
        bVar5 = false;
      }
    }
    else {
      iVar3 = FUN_1401ba9d0(lVar4 + 0x8a,*(undefined4 *)(lVar4 + 0x92));
      bVar5 = iVar3 < 1;
    }
    if (bVar5) {
      return 0;
    }
  }
  return lVar4;
}



//===========================================================
// FUN_1403838e0 @ 1403838e0   (70 bytes)
//===========================================================

undefined8 FUN_1403838e0(longlong param_1,int param_2)

{
  int *piVar1;
  
  if ((*(int *)(param_1 + 0x38) - 1800000U < 10000) && (param_2 - 5000000U < 10000)) {
    for (piVar1 = *(int **)(param_1 + 0x1e8); piVar1 != *(int **)(param_1 + 0x1f0);
        piVar1 = piVar1 + 1) {
      if (*piVar1 == param_2) {
        return 1;
      }
    }
  }
  return 0;
}



//===========================================================
// FUN_140253420 @ 140253420   (31 bytes)
//===========================================================

int FUN_140253420(int param_1,int param_2)

{
  if ((param_2 != 0) && (-param_1 + 0x1fU < 0x1f)) {
    return -100 - param_1;
  }
  return -param_1;
}



//===========================================================
// FUN_140416820 @ 140416820   (18 bytes)
//===========================================================

bool FUN_140416820(int param_1)

{
  return param_1 - 0x197b70U < 10000;
}



//===========================================================
// FUN_14019a5d0 @ 14019a5d0   (1057 bytes)
//===========================================================

ulonglong FUN_14019a5d0(int *param_1)

{
  uint *puVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  undefined1 uVar5;
  ushort uVar6;
  ushort uVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  byte bVar10;
  byte bVar11;
  int iVar12;
  uint uVar13;
  byte *pbVar14;
  longlong lVar15;
  ushort uVar16;
  int iVar17;
  ushort uVar18;
  byte *pbVar19;
  uint uVar20;
  ulonglong uVar21;
  undefined1 local_res8 [8];
  undefined1 local_res10 [8];
  byte local_res18 [8];
  ushort local_res20 [4];
  undefined4 local_78;
  undefined4 local_70;
  undefined4 local_6c;
  ulonglong local_68;
  undefined8 local_60;
  longlong local_58 [3];
  
  puVar1 = *(uint **)(param_1 + 2);
  local_70 = *puVar1;
  uVar20 = 0;
  uVar13 = 0;
  local_res18[0] = (byte)puVar1[1];
  local_res20[0] = 0x9a65;
  pbVar19 = (byte *)&local_70;
  pbVar14 = (byte *)((longlong)puVar1 + 2);
  do {
    if (local_res18[0] == 0) {
      local_res18[0] = 0x2a;
    }
    bVar10 = pbVar19[(longlong)puVar1 - (longlong)&local_70];
    *pbVar19 = bVar10 ^ local_res18[0];
    bVar10 = bVar10 + local_res18[0] + 0x2a;
    uVar16 = (local_res20[0] >> 0xd) + (ushort)bVar10;
    uVar18 = local_res20[0] << 3;
    if (bVar10 == 0) {
      bVar10 = 0x2a;
    }
    bVar11 = pbVar14[-1];
    pbVar19[1] = bVar11 ^ bVar10;
    bVar11 = bVar11 + bVar10 + 0x2a;
    uVar6 = (ushort)bVar11;
    if (bVar11 == 0) {
      bVar11 = 0x2a;
    }
    local_res18[0] = *pbVar14;
    pbVar19[2] = local_res18[0] ^ bVar11;
    local_res18[0] = local_res18[0] + bVar11 + 0x2a;
    uVar7 = (ushort)local_res18[0];
    if (local_res18[0] == 0) {
      local_res18[0] = 0x2a;
    }
    bVar10 = pbVar14[1];
    pbVar19[3] = bVar10 ^ local_res18[0];
    local_res18[0] = bVar10 + 0x2a + local_res18[0];
    local_res20[0] =
         ((uVar16 | uVar18 & 0x3ff) >> 7) + (ushort)local_res18[0] |
         (((uVar18 & 0x1fff) >> 10) + uVar7 |
         (((local_res20[0] & 0x1fff) >> 10) + uVar6 | (uVar16 | uVar18) << 3) << 3) << 3;
    uVar13 = uVar13 + 4;
    pbVar19 = pbVar19 + 4;
    pbVar14 = pbVar14 + 4;
  } while (uVar13 < 4);
  FUN_140c78f50(0x56,(longlong)param_1 + 0x21a3f060fc2daf);
  uVar13 = local_70;
  lVar2 = *(longlong *)(param_1 + 2);
  uVar21 = (ulonglong)(int)local_70;
  if (((local_res20[0] != *(ushort *)(lVar2 + 8)) || ((char)param_1[1] != *(char *)(lVar2 + 5))) ||
     ((char)param_1[4] != *(char *)(lVar2 + 6))) {
    local_res8[0] = (undefined1)param_1[4];
    local_res10[0] = (undefined1)param_1[1];
    local_6c = 1;
    local_68 = uVar21;
    local_60 = FUN_1418039d0(5);
    puVar8 = (undefined8 *)
             FUN_140197ac0(local_58,&local_60,&local_6c,&local_68,local_res18,local_res20,
                           (ushort *)(lVar2 + 8),local_res10,lVar2 + 5,local_res8,lVar2 + 6);
    FUN_141804970(&DAT_143271f04,0x61,5,*puVar8);
    if (local_58[0] != 0) {
      FUN_14019f2c0(local_58[0] + -0x10);
    }
  }
  iVar12 = *param_1;
  iVar17 = iVar12 + 1;
  *param_1 = iVar17;
  if (iVar17 == (iVar17 / 0x37) * 0x37) {
    local_78 = uVar13;
    iVar12 = iVar12 + 2;
    *param_1 = iVar12;
    if (iVar12 == (iVar12 / 0x6f) * 0x6f) {
      puVar8 = *(undefined8 **)(param_1 + 2);
      puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0xc);
      *(undefined8 **)(param_1 + 2) = puVar9;
      *puVar9 = *puVar8;
      *(undefined4 *)(puVar9 + 1) = *(undefined4 *)(puVar8 + 1);
      thunk_FUN_140205820(puVar8,0xc);
    }
    uVar5 = FUN_142f04924();
    *(undefined1 *)(*(longlong *)(param_1 + 2) + 4) = uVar5;
    pbVar19 = *(byte **)(param_1 + 2);
    bVar10 = pbVar19[4];
    pbVar19[8] = 0x65;
    pbVar19[9] = 0x9a;
    lVar15 = (longlong)&local_78 - (longlong)pbVar19;
    lVar2 = 1 - (longlong)pbVar19;
    lVar3 = 2 - (longlong)pbVar19;
    lVar4 = 3 - (longlong)pbVar19;
    do {
      if (bVar10 == 0) {
        bVar10 = 0x2a;
      }
      bVar11 = pbVar19[lVar15];
      *pbVar19 = bVar10 ^ bVar11;
      bVar10 = bVar10 + (bVar10 ^ bVar11) + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar10 | uVar16 << 3;
      bVar11 = 0x2a;
      if (bVar10 != 0) {
        bVar11 = bVar10;
      }
      bVar10 = pbVar19[(longlong)&local_78 + lVar2];
      pbVar19[1] = bVar11 ^ bVar10;
      bVar11 = (bVar11 ^ bVar10) + bVar11 + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar11 | uVar16 << 3;
      bVar10 = 0x2a;
      if (bVar11 != 0) {
        bVar10 = bVar11;
      }
      bVar11 = pbVar19[(longlong)&local_78 + lVar3];
      pbVar19[2] = bVar10 ^ bVar11;
      bVar11 = (bVar10 ^ bVar11) + bVar10 + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar11 | uVar16 << 3;
      bVar10 = 0x2a;
      if (bVar11 != 0) {
        bVar10 = bVar11;
      }
      bVar11 = pbVar19[(longlong)&local_78 + lVar4];
      pbVar19[3] = bVar10 ^ bVar11;
      bVar10 = (bVar10 ^ bVar11) + bVar10 + 0x2a;
      uVar16 = *(ushort *)(*(longlong *)(param_1 + 2) + 8);
      *(ushort *)(*(longlong *)(param_1 + 2) + 8) = (uVar16 >> 0xd) + (ushort)bVar10 | uVar16 << 3;
      uVar20 = uVar20 + 4;
      pbVar19 = pbVar19 + 4;
    } while (uVar20 < 4);
    uVar21 = (ulonglong)local_70;
  }
  FUN_140c79130(0x67,param_1 + 0xc0a84bdc4a);
  return uVar21 & 0xffffffff;
}



//===========================================================
// FUN_1402e3a70 @ 1402e3a70   (142 bytes)
//===========================================================

longlong FUN_1402e3a70(longlong param_1,int param_2)

{
  longlong lVar1;
  int iVar2;
  
  iVar2 = -param_2;
  if ((((iVar2 - 0x4b0U < 0xe) || (iVar2 - 0x708U < 0x33)) || (iVar2 - 3000U < 0x20)) ||
     ((iVar2 - 0xc1cU < 0x20 || (iVar2 - 0xc80U < 0x20)))) {
    lVar1 = FUN_1402de270(param_1 + 0x5a8);
    return lVar1;
  }
  if (param_2 + 0x83U < 0x1f) {
    return param_1 + 0x3a8 + (longlong)(-100 - param_2) * 0x10;
  }
  if (param_2 + 0x1fU < 0x1f) {
    return (longlong)iVar2 * 0x10 + 0x1a8 + param_1;
  }
  return 0;
}



//===========================================================
// FUN_1408f6690 @ 1408f6690   (73 bytes)
//===========================================================

longlong FUN_1408f6690(void)

{
  int iVar1;
  
  iVar1 = (*DAT_143262db0)();
  return (longlong)(iVar1 - DAT_143ac3128) * 10000 + ((ulonglong)DAT_143ac3124 << 0x20) +
         (ulonglong)DAT_143ac3120;
}



//===========================================================
// FUN_140388c60 @ 140388c60   (416 bytes)
//===========================================================

longlong FUN_140388c60(longlong param_1,int param_2)

{
  undefined8 *puVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  short *local_res8;
  int local_res10 [2];
  undefined1 local_28 [8];
  longlong local_20;
  
  local_res10[0] = param_2;
  if (*(longlong *)(param_1 + 0x88) != 0) {
    for (lVar4 = *(longlong *)
                  (*(longlong *)(param_1 + 0x88) +
                  ((ulonglong)(longlong)param_2 % (ulonglong)*(uint *)(param_1 + 0x90)) * 8);
        lVar4 != 0; lVar4 = *(longlong *)(lVar4 + 8)) {
      if (*(int *)(lVar4 + 0x10) == param_2) {
        if (lVar4 != -0x18) {
          FUN_1403ddaa0(param_1,param_2);
          FUN_1403ddbb0(param_1);
          return *(longlong *)(lVar4 + 0x20);
        }
        break;
      }
    }
  }
  if (param_2 == 0) {
    return 0;
  }
  FUN_1403e18a0(&local_res8,param_2);
  if ((local_res8 == (short *)0x0) || (*local_res8 == 0)) {
    lVar4 = 0;
  }
  else {
    FUN_1403b5130(param_1,local_28,param_2,local_res8);
    FUN_140406850(param_1 + 0x88,local_res10,local_28);
    if (local_20 != 0) {
      FUN_1403ddaa0(param_1,param_2);
      FUN_1403ddbb0(param_1);
    }
    lVar4 = local_20;
    if (local_20 != 0) {
      puVar1 = (undefined8 *)(local_20 + -0x28);
      if (0xffffe < *(longlong *)(local_20 + -0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar2 = (longlong *)(lVar4 + -0x20);
      lVar3 = *plVar2;
      *plVar2 = *plVar2 + -1;
      UNLOCK();
      if ((int)lVar3 == 1) {
        if ((local_20 != 0) && (*(longlong *)(local_20 + -0x10) != 0)) {
          LOCK();
          *(undefined8 *)(*(longlong *)(local_20 + -0x10) + 8) = 0;
          UNLOCK();
          do {
          } while (*(int *)(*(longlong *)(local_20 + -0x10) + 4) != 0);
        }
        if (puVar1 != (undefined8 *)0x0) {
          (**(code **)*puVar1)(puVar1,1);
        }
      }
      local_20 = 0;
    }
  }
  if (local_res8 != (short *)0x0) {
    FUN_1401bebb0(local_res8 + -8);
  }
  return lVar4;
}



//===========================================================
// FUN_142cbed90 @ 142cbed90   (143 bytes)
//===========================================================

bool FUN_142cbed90(longlong param_1,uint param_2)

{
  longlong lVar1;
  uint uVar2;
  uint uVar3;
  undefined4 uVar4;
  
  if (-1 < (int)param_2) {
    lVar1 = *(longlong *)(param_1 + 0x2590);
    uVar4 = 0;
    uVar3 = 0;
    uVar2 = uVar3;
    if (lVar1 != 0) {
      uVar2 = *(uint *)(lVar1 + -8);
    }
    if ((int)param_2 < (int)uVar2) {
      if (lVar1 != 0) {
        uVar3 = *(uint *)(lVar1 + -8);
      }
      if (uVar3 <= param_2) {
        if (lVar1 != 0) {
          uVar4 = *(undefined4 *)(lVar1 + -8);
        }
        FUN_142e54290(0xc6,param_2,uVar4);
        lVar1 = *(longlong *)(param_1 + 0x2590);
      }
      return *(longlong *)(lVar1 + 8 + (longlong)(int)param_2 * 0x10) != 0;
    }
  }
  return false;
}



//===========================================================
// thunk_FUN_140205820 @ 142ef3bb8   (5 bytes)
//===========================================================

void thunk_FUN_140205820(undefined8 param_1)

{
  FUN_14019bb50(&DAT_143ad68a0,param_1);
  return;
}



//===========================================================
// FUN_142e54290 @ 142e54290   (185 bytes)
//===========================================================

void FUN_142e54290(undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  char cVar1;
  undefined4 local_res8 [2];
  undefined4 local_res10 [2];
  undefined4 local_res18 [2];
  undefined4 local_res20 [2];
  longlong local_18 [3];
  
  local_res8[0] = param_1;
  local_res10[0] = param_2;
  local_res18[0] = param_3;
  cVar1 = FUN_142e559e0();
  if (cVar1 != '\0') {
    FUN_140194c60(local_18);
    local_res20[0] = FUN_14091a3e0(local_18);
    FUN_142e5c990("LogCallStack5",&DAT_1434997dc,local_res20,&DAT_1434997f8,local_res8,"Info1",
                  local_res10,"info2",local_res18,local_18);
    if (local_18[0] != 0) {
      FUN_14019f2c0(local_18[0] + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_140389c10 @ 140389c10   (83 bytes)
//===========================================================

undefined4 FUN_140389c10(undefined8 param_1,int param_2)

{
  longlong lVar1;
  
  if (param_2 - 5000000U < 1000000) {
    return 1;
  }
  if ((param_2 - 1000000U < 1000000) || (param_2 - 6000000U < 1000000)) {
    lVar1 = FUN_140388c60();
  }
  else {
    lVar1 = FUN_14039b100();
  }
  if (lVar1 != 0) {
    return *(undefined4 *)(lVar1 + 0x18);
  }
  return 0;
}



//===========================================================
// FUN_14019f2c0 @ 14019f2c0   (345 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14019f2c0(int *param_1)

{
  int iVar1;
  code *UNRECOVERED_JUMPTABLE;
  void *pvVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  int *piVar5;
  uint uVar6;
  longlong lVar7;
  bool bVar8;
  
  if (0x100000 < *param_1 + 1U) {
    FUN_142e52dd0(0x23);
  }
  LOCK();
  iVar1 = *param_1;
  *param_1 = *param_1 + -1;
  UNLOCK();
  if (1 < iVar1) {
    return;
  }
  if (*param_1 != 0) {
    FUN_142e52dd0(0x32);
  }
  pvVar2 = Self;
  UNRECOVERED_JUMPTABLE = DAT_143ad5530;
  uVar3 = *(ulonglong *)(param_1 + -2);
  if ((longlong)uVar3 < 0) {
    uVar3 = ~uVar3;
  }
  if (uVar3 < 0x39) {
    uVar6 = (uint)(0x28 < uVar3);
  }
  else {
    if (uVar3 < 0x59) {
      uVar6 = 2;
      goto LAB_14019f332;
    }
    uVar6 = 0xffffffff;
    if (uVar3 < 0x99) {
      uVar6 = 3;
    }
  }
  if ((int)uVar6 < 0) {
    uVar4 = (*DAT_143ad5538)();
                    /* WARNING: Could not recover jumptable at 0x00014019f3a6. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(uVar4,0,param_1 + -2);
    return;
  }
LAB_14019f332:
  uVar3 = (ulonglong)uVar6;
  lVar7 = (ulonglong)uVar6 * 0x10;
  LOCK();
  bVar8 = *(longlong *)(&DAT_143ad6a58 + lVar7) == 0;
  if (bVar8) {
    *(void **)(&DAT_143ad6a58 + lVar7) = Self;
  }
  UNLOCK();
  if (bVar8) {
LAB_14019f3d5:
    *(undefined4 *)(&DAT_143ad6a60 + lVar7) = 1;
  }
  else {
    if (*(void **)(&DAT_143ad6a58 + lVar7) != pvVar2) {
      while( true ) {
        pvVar2 = Self;
        LOCK();
        bVar8 = *(longlong *)(&DAT_143ad6a58 + lVar7) == 0;
        if (bVar8) {
          *(void **)(&DAT_143ad6a58 + lVar7) = Self;
        }
        UNLOCK();
        if (bVar8) goto LAB_14019f3d5;
        if (*(void **)(&DAT_143ad6a58 + lVar7) == pvVar2) break;
        (*DAT_143262828)(0);
      }
    }
    *(int *)(&DAT_143ad6a60 + lVar7) = *(int *)(&DAT_143ad6a60 + lVar7) + 1;
  }
  piVar5 = (int *)(&DAT_143ad6a60 + lVar7);
  *(undefined8 *)param_1 = *(undefined8 *)(&DAT_143ad6a98 + uVar3 * 8);
  *(int **)(&DAT_143ad6a98 + uVar3 * 8) = param_1;
  _DAT_143ad6ad8 = *(undefined8 *)param_1;
  *(int *)(&DAT_143ad6a44 + uVar3 * 4) = *(int *)(&DAT_143ad6a44 + uVar3 * 4) + -1;
  *piVar5 = *piVar5 + -1;
  if (*piVar5 == 0) {
    *(undefined8 *)(&DAT_143ad6a58 + lVar7) = 0;
  }
  return;
}



//===========================================================
// FUN_140c78f50 @ 140c78f50   (459 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140c78f50(undefined4 param_1,undefined8 param_2)

{
  char cVar1;
  int iVar2;
  longlong lVar3;
  int local_48;
  byte local_44;
  longlong local_40;
  int local_38;
  undefined4 local_34;
  undefined4 local_30;
  int local_2c;
  undefined1 local_28 [8];
  undefined1 local_20 [32];
  
  iVar2 = (*DAT_1432622a8)();
  if ((iVar2 + 0x10101 != DAT_143ac7f08) && (cVar1 = FUN_142e2bb90(), cVar1 == '\0')) {
    _DAT_143ac7f18 = 0x592af1c3;
    _DAT_143ac7f0c = param_1;
    _DAT_143ac7f10 = param_2;
    local_38 = (*DAT_143262db0)();
    if ((DAT_143ac7f1c == 0) ||
       (cVar1 = FUN_1408fcaa0(DAT_143ac7f1c,600000,local_38), cVar1 != '\0')) {
      DAT_143ac7f1c = local_38;
      FUN_140194c60(&local_40);
      if (local_40 != 0) {
        FUN_140caa790(&local_40);
        local_34 = 4;
        lVar3 = FUN_140cabef0(&local_40);
        local_2c = *(int *)(lVar3 + 8);
        for (local_48 = 0; local_48 < local_2c; local_48 = local_48 + 1) {
          local_44 = *(byte *)(local_40 + local_48) << ((byte)local_34 & 0x1f) |
                     *(byte *)(local_40 + local_48) >> (8 - (byte)local_34 & 0x1f);
          *(byte *)(local_48 + local_40) = local_44;
        }
      }
      local_30 = FUN_14091a3e0(&local_40);
      cVar1 = FUN_140ca2760(&DAT_143ac80c8,local_30);
      if (cVar1 == '\0') {
        FUN_140ca4f90(&DAT_143ac80c8,local_20,&local_30);
        FUN_14019a260(&DAT_143ac80d8,&local_40);
        FUN_140ca6280(local_28,0);
        FUN_140319ad0(&local_40,local_28);
        FUN_140199470(local_28);
      }
      FUN_140199470(&local_40);
    }
  }
  return;
}



//===========================================================
// FUN_1418039d0 @ 1418039d0   (546 bytes)
//===========================================================

char * FUN_1418039d0(int param_1)

{
  if (param_1 < 0x22000001) {
    if (param_1 == 0x22000000) {
      return "TERMINATE_BEGIN";
    }
    if (param_1 < 0x21000001) {
      if (param_1 == 0x21000000) {
        return "DISCONNECT_BEGIN";
      }
      if (param_1 == 0x20000000) {
        return "PATCH";
      }
    }
    else {
      switch(param_1) {
      case 0x21000001:
        return "CONNECT_TO_GAME_FAILED";
      case 0x21000002:
        return "CONNECTION_FROM_GAME_CLOSED";
      case 0x21000003:
        return "FAILED_PROTOCOL_WITH_GAME";
      case 0x21000004:
        return "FORCE_DISCONNECT";
      case 0x21000005:
        return "DISCONNECT_BY_MALICIOUS_PROCESS";
      case 0x21000006:
        return "SHUTDOWN";
      case 0x21000007:
        return "SELECTIVE_SHUTDOWN";
      case 0x21000008:
        return "PREMIUM_REMAINING_TIME_EXHAUSTED";
      case 0x21000009:
        return "IP_MAX_CONNECTED";
      case 0x2100000a:
        return "DIFFERENT_IP_NOT_ALLOWED";
      case 0x2100000b:
        return "CONNECTION_IS_UNSTABLE";
      case 0x2100000c:
        return "ACCOUNT_MACHINEID_BLOCKED";
      case 0x2100000d:
        return "NOT_APPLICABLE_PCBANG_PC";
      case 0x2100000e:
        return "DISCONNECT_END";
      }
    }
  }
  else {
    switch(param_1) {
    case 0x22000001:
      return "CONNECT_TO_LOGIN_FAILED";
    case 0x22000002:
      return "CONNECTION_FROM_LOGIN_CLOSED";
    case 0x22000003:
      return "NOT_ENOUGH_MEMORY";
    case 0x22000004:
      return "NO_DATA_PACKAGE";
    case 0x22000005:
      return "INVALID_GAME_DATA_VERSION";
    case 0x22000006:
      return "INVALID_GAME_DATA";
    case 0x22000007:
      return "INVALID_CLIENT_VERSION";
    case 0x22000008:
      return "FAILED_CRITICAL_PROTOCOL_WITH_GAME";
    case 0x22000009:
      return "WEB_LOGIN_NEEDED";
    case 0x2200000a:
      return "CLIENTCRC_FAILED";
    case 0x2200000b:
      return "ACCESS_PRTOECT_ACCOUNT";
    case 0x2200000c:
      return "CONNECT_FAILED_FOR_SERVER_INSPECTION";
    case 0x2200000d:
      return "FROM_NGU";
    case 0x2200000e:
      return "REMOTE_SHUTDOWN";
    case 0x2200000f:
      return "CLIENT_IS_SET_TO_FORCE_TERMINATE";
    case 0x22000010:
      return "TERMINATE_END";
    }
  }
  FUN_141805f00(&DAT_143acedf0,"Etc:0x%X",param_1);
  return &DAT_143acedf0;
}



//===========================================================
// FUN_140197ac0 @ 140197ac0   (773 bytes)
//===========================================================

longlong *
FUN_140197ac0(longlong *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
             undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
             undefined8 param_9,undefined8 param_10,undefined8 param_11)

{
  char *pcVar1;
  longlong lVar2;
  longlong lVar3;
  undefined4 *puVar4;
  int iVar5;
  int iVar6;
  int *piVar7;
  longlong lVar8;
  int iVar9;
  int *piVar10;
  int iVar11;
  longlong local_30;
  
  piVar7 = (int *)0x0;
  iVar5 = 0;
  *param_1 = 0;
  FUN_1408bc4c0(&local_30);
  lVar2 = local_30;
  iVar6 = 0;
  if (local_30 != 0) {
    iVar9 = *(int *)(local_30 + -8);
    lVar8 = (longlong)iVar9;
    if (iVar9 != 0) {
      pcVar1 = (char *)*param_1;
      piVar10 = piVar7;
      iVar11 = iVar6;
      if (pcVar1 == (char *)0x0) goto LAB_140197bae;
      if (*pcVar1 == '\0') {
        piVar10 = (int *)(pcVar1 + -0x10);
        if (piVar10 == (int *)0x0) {
LAB_140197bae:
          if (iVar11 < iVar9) {
            iVar11 = iVar9;
          }
          puVar4 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
          puVar4[1] = iVar11;
          *puVar4 = 0xffffffff;
          *param_1 = (longlong)(puVar4 + 4);
          puVar4[2] = 0;
          *(undefined1 *)*param_1 = 0;
          if (piVar10 != (int *)0x0) {
            FUN_14019f2c0(piVar10);
          }
        }
        else {
          if ((1 < *piVar10) || (*(int *)(pcVar1 + -0xc) < iVar9)) {
            iVar11 = *(int *)(pcVar1 + -8);
            goto LAB_140197bae;
          }
          if (*piVar10 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar10 = -1;
        }
        FUN_142ef7ba0(*param_1,lVar2,lVar8);
      }
      else {
        iVar9 = *(int *)(pcVar1 + -8) + iVar9;
        for (iVar11 = *(int *)(pcVar1 + -0xc); iVar11 < iVar9; iVar11 = iVar11 * 2) {
        }
        lVar3 = FUN_14019bd40(param_1,iVar11,1);
        iVar11 = iVar5;
        if (*param_1 != 0) {
          iVar11 = *(int *)(*param_1 + -8);
        }
        FUN_142ef7ba0(iVar11 + lVar3,lVar2,lVar8);
      }
      FUN_14019c870(param_1,iVar9);
    }
  }
  if (local_30 != 0) {
    FUN_14019f2c0(local_30 + -0x10);
  }
  FUN_1408bc020(&local_30,param_3);
  lVar2 = local_30;
  if (local_30 == 0) goto LAB_140197d39;
  iVar9 = *(int *)(local_30 + -8);
  lVar8 = (longlong)iVar9;
  if (iVar9 == 0) goto LAB_140197d39;
  pcVar1 = (char *)*param_1;
  if (pcVar1 == (char *)0x0) goto LAB_140197cda;
  if (*pcVar1 == '\0') {
    piVar7 = (int *)(pcVar1 + -0x10);
    if (piVar7 == (int *)0x0) {
LAB_140197cda:
      if (iVar6 < iVar9) {
        iVar6 = iVar9;
      }
      puVar4 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
      puVar4[1] = iVar6;
      *puVar4 = 0xffffffff;
      *param_1 = (longlong)(puVar4 + 4);
      puVar4[2] = 0;
      *(undefined1 *)*param_1 = 0;
      if (piVar7 != (int *)0x0) {
        FUN_14019f2c0(piVar7);
      }
    }
    else {
      if ((1 < *piVar7) || (*(int *)(pcVar1 + -0xc) < iVar9)) {
        iVar6 = *(int *)(pcVar1 + -8);
        goto LAB_140197cda;
      }
      if (*piVar7 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar7 = -1;
    }
    FUN_142ef7ba0(*param_1,lVar2,lVar8);
  }
  else {
    iVar9 = *(int *)(pcVar1 + -8) + iVar9;
    for (iVar6 = *(int *)(pcVar1 + -0xc); iVar6 < iVar9; iVar6 = iVar6 * 2) {
    }
    lVar3 = FUN_14019bd40(param_1,iVar6,1);
    if (*param_1 != 0) {
      iVar5 = *(int *)(*param_1 + -8);
    }
    FUN_142ef7ba0(iVar5 + lVar3,lVar2,lVar8);
  }
  FUN_14019c870(param_1,iVar9);
LAB_140197d39:
  if (local_30 != 0) {
    FUN_14019f2c0(local_30 + -0x10);
  }
  FUN_1401985a0(param_1,param_4,param_5,param_6,param_7,param_8,param_9,param_10,param_11);
  return param_1;
}



//===========================================================
// FUN_141804970 @ 141804970   (120 bytes)
//===========================================================

void FUN_141804970(undefined8 param_1,undefined4 param_2,undefined4 param_3,undefined1 *param_4)

{
  undefined8 local_res8;
  undefined4 local_res10 [2];
  undefined4 local_res18 [2];
  undefined1 *local_res20;
  undefined1 local_18 [24];
  
  local_res20 = &DAT_143271f04;
  if (param_4 != (undefined1 *)0x0) {
    local_res20 = param_4;
  }
  local_res8 = param_1;
  local_res10[0] = param_2;
  local_res18[0] = param_3;
  FUN_141804b50(0xf,"throw ZException",&local_res8,local_res10,&DAT_1433d5e74,local_res18,
                &local_res20);
  FUN_1401bb8b0(local_18,local_res18[0]);
                    /* WARNING: Subroutine does not return */
  _CxxThrowException(local_18,(ThrowInfo *)&DAT_143a3b118);
}



//===========================================================
// FUN_142f04924 @ 142f04924   (41 bytes)
//===========================================================

uint FUN_142f04924(void)

{
  longlong lVar1;
  uint uVar2;
  
  lVar1 = FUN_142f31784();
  uVar2 = *(int *)(lVar1 + 0x28) * 0x343fd + 0x269ec3;
  *(uint *)(lVar1 + 0x28) = uVar2;
  return uVar2 >> 0x10 & 0x7fff;
}



//===========================================================
// FUN_140c79130 @ 140c79130   (459 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140c79130(undefined4 param_1,undefined8 param_2)

{
  char cVar1;
  int iVar2;
  longlong lVar3;
  int local_48;
  byte local_44;
  longlong local_40;
  int local_38;
  undefined4 local_34;
  undefined4 local_30;
  int local_2c;
  undefined1 local_28 [8];
  undefined1 local_20 [32];
  
  iVar2 = (*DAT_1432622a8)();
  if ((iVar2 + 0xf010fa1 != DAT_143ac7f24) && (cVar1 = FUN_142e2bb90(), cVar1 == '\0')) {
    _DAT_143ac7f2c = 0x1aff01;
    _DAT_143ac7f28 = param_1;
    _DAT_143ac7f30 = param_2;
    local_38 = (*DAT_143262db0)();
    if ((DAT_143ac7f38 == 0) ||
       (cVar1 = FUN_1408fcaa0(DAT_143ac7f38,600000,local_38), cVar1 != '\0')) {
      DAT_143ac7f38 = local_38;
      FUN_140194c60(&local_40);
      if (local_40 != 0) {
        FUN_140caa790(&local_40);
        local_34 = 4;
        lVar3 = FUN_140cabef0(&local_40);
        local_2c = *(int *)(lVar3 + 8);
        for (local_48 = 0; local_48 < local_2c; local_48 = local_48 + 1) {
          local_44 = *(byte *)(local_40 + local_48) << ((byte)local_34 & 0x1f) |
                     *(byte *)(local_40 + local_48) >> (8 - (byte)local_34 & 0x1f);
          *(byte *)(local_48 + local_40) = local_44;
        }
      }
      local_30 = FUN_14091a3e0(&local_40);
      cVar1 = FUN_140ca2760(&DAT_143ac80e0,local_30);
      if (cVar1 == '\0') {
        FUN_140ca4f90(&DAT_143ac80e0,local_20,&local_30);
        FUN_14019a260(&DAT_143ac80f0,&local_40);
        FUN_140ca6280(local_28,0);
        FUN_140319ad0(&local_40,local_28);
        FUN_140199470(local_28);
      }
      FUN_140199470(&local_40);
    }
  }
  return;
}



//===========================================================
// FUN_14019b780 @ 14019b780   (374 bytes)
//===========================================================

void FUN_14019b780(longlong param_1,ulonglong param_2)

{
  longlong *plVar1;
  int *piVar2;
  void *pvVar3;
  longlong lVar4;
  undefined8 *puVar5;
  int iVar6;
  undefined8 uVar7;
  longlong lVar8;
  int *piVar9;
  uint uVar10;
  longlong lVar11;
  
  pvVar3 = Self;
  uVar7 = 0x80;
  if (param_2 < 0x21) {
    uVar10 = (uint)(0x10 < param_2);
LAB_14019b7de:
    if ((int)uVar10 < 0) {
      FUN_14019d350(param_2);
      return;
    }
    if (uVar10 == 0) {
      iVar6 = 0x40;
      uVar7 = 0x10;
      goto LAB_14019b825;
    }
    if (uVar10 == 1) {
      iVar6 = 0x20;
      uVar7 = 0x20;
      goto LAB_14019b825;
    }
    if (uVar10 != 2) {
      if (uVar10 == 3) {
        iVar6 = 8;
      }
      else {
        iVar6 = 0;
        uVar7 = 0;
      }
      goto LAB_14019b825;
    }
  }
  else {
    if (0x40 < param_2) {
      uVar10 = 0xffffffff;
      if (param_2 < 0x81) {
        uVar10 = 3;
      }
      goto LAB_14019b7de;
    }
    uVar10 = 2;
  }
  iVar6 = 0x10;
  uVar7 = 0x40;
LAB_14019b825:
  lVar11 = (longlong)(int)uVar10;
  lVar8 = lVar11 * 0x10 + param_1;
  plVar1 = (longlong *)(lVar8 + 0x28);
  LOCK();
  lVar4 = *plVar1;
  if (lVar4 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  if (lVar4 == 0) {
LAB_14019b889:
    *(undefined4 *)(lVar8 + 0x30) = 1;
  }
  else if ((void *)*plVar1 == pvVar3) {
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  else {
    while( true ) {
      pvVar3 = Self;
      LOCK();
      lVar4 = *plVar1;
      if (lVar4 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar4 == 0) goto LAB_14019b889;
      if ((void *)*plVar1 == pvVar3) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar8 + 0x30) = *(int *)(lVar8 + 0x30) + 1;
  }
  piVar9 = (int *)(lVar8 + 0x30);
  puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  if (puVar5 == (undefined8 *)0x0) {
    lVar4 = FUN_14019d3c0(uVar7,iVar6);
    *(undefined8 *)(lVar4 + -0x10) = *(undefined8 *)(param_1 + 0x88 + lVar11 * 8);
    *(longlong *)(param_1 + 0x88 + lVar11 * 8) = lVar4;
    *(longlong *)(param_1 + 0x68 + lVar11 * 8) = lVar4;
    piVar2 = (int *)(param_1 + 4 + lVar11 * 4);
    *piVar2 = *piVar2 + iVar6;
    puVar5 = *(undefined8 **)(param_1 + 0x68 + lVar11 * 8);
  }
  piVar2 = (int *)(param_1 + 0x14 + lVar11 * 4);
  *piVar2 = *piVar2 + 1;
  *(undefined8 *)(param_1 + 0x68 + lVar11 * 8) = *puVar5;
  *piVar9 = *piVar9 + -1;
  if (*piVar9 == 0) {
    *plVar1 = 0;
  }
  return;
}



//===========================================================
// FUN_142ef8250 @ 142ef8250   (376 bytes)
//===========================================================

undefined1 (*) [16] FUN_142ef8250(undefined1 (*param_1) [16],byte param_2,ulonglong param_3)

{
  undefined1 (*pauVar1) [16];
  undefined1 (*pauVar2) [16];
  undefined1 uVar3;
  ulonglong uVar6;
  undefined1 auVar7 [16];
  undefined2 uVar4;
  undefined4 uVar5;
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  undefined1 uVar17;
  
  uVar6 = (ulonglong)param_2 * 0x101010101010101;
  uVar3 = (undefined1)uVar6;
  uVar4 = (undefined2)uVar6;
  uVar5 = (undefined4)uVar6;
  switch(param_3) {
  case 0:
    return param_1;
  case 8:
    *(ulonglong *)*param_1 = uVar6;
    return param_1;
  case 9:
    *(ulonglong *)(param_1[-1] + param_3 + 7) = uVar6;
    param_1[-1][param_3 + 0xf] = uVar3;
    return param_1;
  case 10:
    *(ulonglong *)*param_1 = uVar6;
    *(undefined2 *)(*param_1 + 8) = uVar4;
    return param_1;
  case 0xb:
    *(ulonglong *)*param_1 = uVar6;
    *(undefined2 *)(*param_1 + 8) = uVar4;
    (*param_1)[10] = uVar3;
    return param_1;
  case 0xc:
    *(ulonglong *)(param_1[-1] + param_3 + 4) = uVar6;
  case 4:
    *(undefined4 *)(param_1[-1] + param_3 + 0xc) = uVar5;
    return param_1;
  case 0xd:
    *(ulonglong *)(param_1[-1] + param_3 + 3) = uVar6;
  case 5:
    *(undefined4 *)(param_1[-1] + param_3 + 0xb) = uVar5;
    param_1[-1][param_3 + 0xf] = uVar3;
    return param_1;
  case 0xe:
    *(ulonglong *)(param_1[-1] + param_3 + 2) = uVar6;
  case 6:
    *(undefined4 *)(param_1[-1] + param_3 + 10) = uVar5;
  case 2:
    *(undefined2 *)(param_1[-1] + param_3 + 0xe) = uVar4;
    return param_1;
  case 0xf:
    *(ulonglong *)(param_1[-1] + param_3 + 1) = uVar6;
  case 7:
    *(undefined4 *)(param_1[-1] + param_3 + 9) = uVar5;
  case 3:
    *(undefined2 *)(param_1[-1] + param_3 + 0xd) = uVar4;
  case 1:
    param_1[-1][param_3 + 0xf] = uVar3;
    return param_1;
  case 0x10:
    *(ulonglong *)*param_1 = uVar6;
    *(ulonglong *)(*param_1 + 8) = uVar6;
    return param_1;
  }
  uVar17 = (undefined1)(uVar6 >> 0x38);
  auVar16._8_6_ = 0;
  auVar16._0_8_ = uVar6;
  auVar16[0xe] = uVar17;
  auVar16[0xf] = uVar17;
  uVar17 = (undefined1)(uVar6 >> 0x30);
  auVar15._14_2_ = auVar16._14_2_;
  auVar15._8_5_ = 0;
  auVar15._0_8_ = uVar6;
  auVar15[0xd] = uVar17;
  auVar14._13_3_ = auVar15._13_3_;
  auVar14._8_4_ = 0;
  auVar14._0_8_ = uVar6;
  auVar14[0xc] = uVar17;
  uVar17 = (undefined1)(uVar6 >> 0x28);
  auVar13._12_4_ = auVar14._12_4_;
  auVar13._8_3_ = 0;
  auVar13._0_8_ = uVar6;
  auVar13[0xb] = uVar17;
  auVar12._11_5_ = auVar13._11_5_;
  auVar12._8_2_ = 0;
  auVar12._0_8_ = uVar6;
  auVar12[10] = uVar17;
  uVar17 = (undefined1)(uVar6 >> 0x20);
  auVar11._10_6_ = auVar12._10_6_;
  auVar11[8] = 0;
  auVar11._0_8_ = uVar6;
  auVar11[9] = uVar17;
  auVar10._9_7_ = auVar11._9_7_;
  auVar10[8] = uVar17;
  auVar10._0_8_ = uVar6;
  uVar17 = (undefined1)(uVar6 >> 0x18);
  auVar9._8_8_ = auVar10._8_8_;
  auVar9[7] = uVar17;
  auVar9[6] = uVar17;
  uVar17 = (undefined1)(uVar6 >> 0x10);
  auVar9[5] = uVar17;
  auVar9[4] = uVar17;
  auVar9._0_4_ = uVar5;
  uVar17 = (undefined1)(uVar6 >> 8);
  auVar8._4_12_ = auVar9._4_12_;
  auVar8[3] = uVar17;
  auVar8[2] = uVar17;
  auVar8._0_2_ = uVar4;
  auVar7._2_14_ = auVar8._2_14_;
  auVar7[1] = uVar3;
  auVar7[0] = uVar3;
  pauVar1 = param_1;
  if (0x80 < param_3) {
    if (((byte)DAT_143ae2c20 & 2) != 0) {
      for (; param_3 != 0; param_3 = param_3 - 1) {
        (*pauVar1)[0] = param_2;
        pauVar1 = (undefined1 (*) [16])(*pauVar1 + 1);
      }
      return param_1;
    }
    *param_1 = auVar7;
    pauVar1 = (undefined1 (*) [16])((ulonglong)(param_1 + 1) & 0xfffffffffffffff0);
    param_3 = (longlong)param_1 + (param_3 - (longlong)pauVar1);
    uVar6 = param_3 >> 7;
    if (uVar6 != 0) {
      if (DAT_143a8b928 < uVar6) {
        do {
          *pauVar1 = auVar7;
          pauVar1[1] = auVar7;
          pauVar2 = pauVar1 + 8;
          pauVar1[2] = auVar7;
          pauVar1[3] = auVar7;
          uVar6 = uVar6 - 1;
          pauVar1[4] = auVar7;
          pauVar1[5] = auVar7;
          pauVar1[6] = auVar7;
          pauVar1[7] = auVar7;
          pauVar1 = pauVar2;
        } while (uVar6 != 0);
        param_3 = param_3 & 0x7f;
      }
      else {
        do {
          *pauVar1 = auVar7;
          pauVar1[1] = auVar7;
          pauVar2 = pauVar1 + 8;
          pauVar1[2] = auVar7;
          pauVar1[3] = auVar7;
          uVar6 = uVar6 - 1;
          pauVar1[4] = auVar7;
          pauVar1[5] = auVar7;
          pauVar1[6] = auVar7;
          pauVar1[7] = auVar7;
          pauVar1 = pauVar2;
        } while (uVar6 != 0);
        param_3 = param_3 & 0x7f;
      }
    }
  }
  for (uVar6 = param_3 >> 4; uVar6 != 0; uVar6 = uVar6 - 1) {
    *pauVar1 = auVar7;
    pauVar1 = pauVar1 + 1;
  }
  if ((param_3 & 0xf) != 0) {
    *(undefined1 (*) [16])(pauVar1[-1] + (param_3 & 0xf)) = auVar7;
  }
  return param_1;
}



//===========================================================
// FUN_1402543e0 @ 1402543e0   (945 bytes)
//===========================================================

void FUN_1402543e0(undefined8 param_1,int param_2,int param_3,int param_4)

{
  int iVar1;
  undefined4 local_18 [4];
  
  iVar1 = FUN_1402531f0(param_2);
  if (((((iVar1 == 0) && (iVar1 = FUN_140416760(param_2), iVar1 == 0)) &&
       (iVar1 = FUN_140416820(param_2), iVar1 == 0)) &&
      ((iVar1 = FUN_140253130(param_2), param_3 != 2 && (iVar1 != 2)))) && (iVar1 != param_3)) {
    return;
  }
  switch(param_2 / 10000) {
  case 100:
    local_18[0] = 1;
    FUN_140257c50(param_1,local_18);
    if (param_4 == 0) {
      return;
    }
    local_18[0] = 0x4b0;
    break;
  case 0x65:
    local_18[0] = 2;
    FUN_140257c50(param_1,local_18);
    if (param_4 == 0) {
      return;
    }
    local_18[0] = 0x4b2;
    break;
  case 0x66:
    local_18[0] = 3;
    FUN_140257c50(param_1,local_18);
    if (param_4 == 0) {
      return;
    }
    local_18[0] = 0x4b8;
    break;
  case 0x67:
    local_18[0] = 4;
    FUN_140257c50(param_1,local_18);
    if (param_4 == 0) {
      return;
    }
    local_18[0] = 0x4b9;
    break;
  case 0x68:
  case 0x69:
    local_18[0] = 5;
    FUN_140257c50(param_1,local_18);
    if (param_4 == 0) {
      return;
    }
    local_18[0] = 0x4b3;
    break;
  case 0x6a:
    local_18[0] = 6;
    FUN_140257c50(param_1,local_18);
    if (param_4 == 0) {
      return;
    }
    local_18[0] = 0x4b4;
    break;
  case 0x6b:
    local_18[0] = 7;
    FUN_140257c50(param_1,local_18);
    if (param_4 == 0) {
      return;
    }
    local_18[0] = 0x4b5;
    break;
  case 0x6c:
    local_18[0] = 8;
    FUN_140257c50(param_1,local_18);
    if (param_4 == 0) {
      return;
    }
    local_18[0] = 0x4b6;
    break;
  case 0x6d:
  case 0x86:
  case 0x87:
  case 0x9c:
    local_18[0] = 10;
    break;
  case 0x6e:
    local_18[0] = 9;
    FUN_140257c50(param_1,local_18);
    if (param_4 == 0) {
      return;
    }
    local_18[0] = 0x4b1;
    break;
  case 0x6f:
    local_18[0] = 0xc;
    FUN_140257c50(param_1,local_18);
    local_18[0] = 0xd;
    FUN_140257c50(param_1,local_18);
    local_18[0] = 0xf;
    FUN_140257c50(param_1,local_18);
    local_18[0] = 0x10;
    FUN_140257c50(param_1,local_18);
    if (param_4 == 0) {
      return;
    }
    local_18[0] = 0x4ba;
    FUN_140257c50(param_1,local_18);
    local_18[0] = 0x4bb;
    FUN_140257c50(param_1,local_18);
    local_18[0] = 0x4bc;
    FUN_140257c50(param_1,local_18);
    local_18[0] = 0x4bd;
    break;
  case 0x70:
    local_18[0] = 0x11;
    FUN_140257c50(param_1,local_18);
    local_18[0] = 0x1f;
    break;
  case 0x71:
    local_18[0] = 0x16;
    break;
  case 0x72:
    local_18[0] = 0x15;
    break;
  case 0x73:
    local_18[0] = 0x17;
    break;
  case 0x74:
    local_18[0] = 0x1a;
    break;
  default:
    iVar1 = FUN_140255180(param_2);
    if (((iVar1 == 0) && (99999 < param_2 - 1600000U)) && (param_2 / 10000 != 0xaa)) {
      return;
    }
    local_18[0] = 0xb;
    FUN_140257c50(param_1,local_18);
    if (param_4 == 0) {
      return;
    }
    local_18[0] = 0x4b7;
    break;
  case 0x76:
    local_18[0] = 0x1d;
    break;
  case 0xa6:
    local_18[0] = 0x1b;
    break;
  case 0xa7:
    local_18[0] = 0x1c;
    FUN_140257c50(param_1,local_18);
  case 0x77:
    local_18[0] = 0x1e;
    break;
  case 0xb4:
    local_18[0] = 0xe;
    break;
  case 0xbe:
    local_18[0] = 0x12;
    break;
  case 0xbf:
    local_18[0] = 0x13;
    break;
  case 0xc0:
    local_18[0] = 0x14;
  }
  FUN_140257c50(param_1,local_18);
  return;
}



//===========================================================
// FUN_142e52dd0 @ 142e52dd0   (245 bytes)
//===========================================================

void FUN_142e52dd0(undefined4 param_1,undefined4 param_2)

{
  undefined8 uVar1;
  char cVar2;
  undefined4 local_res8 [2];
  undefined4 local_res10 [2];
  undefined4 local_res18 [2];
  longlong local_res20;
  longlong local_18;
  longlong local_10 [2];
  
  local_res8[0] = param_1;
  local_res10[0] = param_2;
  cVar2 = FUN_142e559e0();
  if (cVar2 != '\0') {
    FUN_140194c60(&local_res20);
    local_res18[0] = FUN_14091a3e0(&local_res20);
    uVar1 = FUN_142a1d8a0(local_10);
    uVar1 = FUN_142e5d800(&local_18,uVar1,"LogCallStack2",&DAT_1434997dc,local_res18,&DAT_1434997f8,
                          local_res8,"Info1",local_res10,&local_res20);
    FUN_142a1ec10(uVar1);
    if (local_18 != 0) {
      FUN_14019f2c0(local_18 + -0x10);
    }
    if (local_10[0] != 0) {
      FUN_14019f2c0(local_10[0] + -0x10);
    }
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
  }
  return;
}



//===========================================================
// __security_check_cookie @ 142ef44b0   (30 bytes)
//===========================================================

/* WARNING: This is an inlined function */

void __cdecl __security_check_cookie(uintptr_t _StackCookie)

{
  if ((_StackCookie == DAT_143a8b908) && ((short)(_StackCookie >> 0x30) == 0)) {
    return;
  }
  FUN_142ef3e44(_StackCookie);
  return;
}



//===========================================================
// FUN_142ef7ba0 @ 142ef7ba0   (1379 bytes)
//===========================================================

undefined8 * FUN_142ef7ba0(undefined8 *param_1,undefined8 *param_2,ulonglong param_3)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined1 auVar3 [32];
  undefined1 auVar4 [32];
  undefined1 auVar5 [32];
  undefined1 auVar6 [32];
  undefined1 uVar7;
  undefined2 uVar8;
  undefined4 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  undefined8 uVar22;
  undefined8 *puVar23;
  undefined1 (*pauVar24) [32];
  undefined1 (*pauVar25) [32];
  undefined8 *puVar26;
  undefined1 (*pauVar27) [32];
  undefined1 (*pauVar28) [32];
  ulonglong uVar29;
  longlong lVar30;
  ulonglong uVar31;
  undefined8 uVar32;
  undefined8 uVar33;
  
  puVar23 = param_1;
  switch(param_3) {
  case 0:
    return puVar23;
  case 1:
    *(undefined1 *)param_1 = *(undefined1 *)param_2;
    return puVar23;
  case 2:
    *(undefined2 *)param_1 = *(undefined2 *)param_2;
    return puVar23;
  case 3:
    uVar7 = *(undefined1 *)((longlong)param_2 + 2);
    *(undefined2 *)param_1 = *(undefined2 *)param_2;
    *(undefined1 *)((longlong)param_1 + 2) = uVar7;
    return puVar23;
  case 4:
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    return puVar23;
  case 5:
    uVar7 = *(undefined1 *)((longlong)param_2 + 4);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined1 *)((longlong)param_1 + 4) = uVar7;
    return puVar23;
  case 6:
    uVar8 = *(undefined2 *)((longlong)param_2 + 4);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined2 *)((longlong)param_1 + 4) = uVar8;
    return puVar23;
  case 7:
    uVar8 = *(undefined2 *)((longlong)param_2 + 4);
    uVar7 = *(undefined1 *)((longlong)param_2 + 6);
    *(undefined4 *)param_1 = *(undefined4 *)param_2;
    *(undefined2 *)((longlong)param_1 + 4) = uVar8;
    *(undefined1 *)((longlong)param_1 + 6) = uVar7;
    return puVar23;
  case 8:
    *param_1 = *param_2;
    return puVar23;
  case 9:
    uVar7 = *(undefined1 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined1 *)(param_1 + 1) = uVar7;
    return puVar23;
  case 10:
    uVar8 = *(undefined2 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined2 *)(param_1 + 1) = uVar8;
    return puVar23;
  case 0xb:
    uVar8 = *(undefined2 *)(param_2 + 1);
    uVar7 = *(undefined1 *)((longlong)param_2 + 10);
    *param_1 = *param_2;
    *(undefined2 *)(param_1 + 1) = uVar8;
    *(undefined1 *)((longlong)param_1 + 10) = uVar7;
    return puVar23;
  case 0xc:
    uVar9 = *(undefined4 *)(param_2 + 1);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    return puVar23;
  case 0xd:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar7 = *(undefined1 *)((longlong)param_2 + 0xc);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined1 *)((longlong)param_1 + 0xc) = uVar7;
    return puVar23;
  case 0xe:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar8 = *(undefined2 *)((longlong)param_2 + 0xc);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined2 *)((longlong)param_1 + 0xc) = uVar8;
    return puVar23;
  case 0xf:
    uVar9 = *(undefined4 *)(param_2 + 1);
    uVar8 = *(undefined2 *)((longlong)param_2 + 0xc);
    uVar7 = *(undefined1 *)((longlong)param_2 + 0xe);
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 1) = uVar9;
    *(undefined2 *)((longlong)param_1 + 0xc) = uVar8;
    *(undefined1 *)((longlong)param_1 + 0xe) = uVar7;
    return puVar23;
  }
  if (param_3 < 0x21) {
    uVar10 = param_2[1];
    puVar26 = (undefined8 *)((longlong)param_2 + (param_3 - 0x10));
    uVar11 = *puVar26;
    uVar32 = puVar26[1];
    *param_1 = *param_2;
    param_1[1] = uVar10;
    param_1 = (undefined8 *)((longlong)param_1 + (param_3 - 0x10));
    *param_1 = uVar11;
    param_1[1] = uVar32;
    return puVar23;
  }
  if ((param_2 < param_1) && (param_1 < (undefined8 *)((longlong)param_2 + param_3))) {
    lVar30 = (longlong)param_2 - (longlong)param_1;
    puVar23 = (undefined8 *)((longlong)param_1 + lVar30 + (param_3 - 0x10));
    uVar10 = *puVar23;
    uVar11 = puVar23[1];
    puVar26 = (undefined8 *)((longlong)param_1 + (param_3 - 0x10));
    uVar29 = param_3 - 0x10;
    puVar23 = puVar26;
    uVar32 = uVar10;
    uVar33 = uVar11;
    if (((ulonglong)puVar26 & 0xf) != 0) {
      puVar23 = (undefined8 *)((ulonglong)puVar26 & 0xfffffffffffffff0);
      uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
      uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
      *puVar26 = uVar10;
      *(undefined8 *)((longlong)param_1 + (param_3 - 8)) = uVar11;
      uVar29 = (longlong)puVar23 - (longlong)param_1;
    }
    uVar31 = uVar29 >> 7;
    if (uVar31 != 0) {
      *puVar23 = uVar32;
      puVar23[1] = uVar33;
      puVar26 = puVar23;
      while( true ) {
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x10);
        uVar10 = puVar1[1];
        puVar23 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x20);
        uVar11 = *puVar23;
        uVar32 = puVar23[1];
        puVar23 = puVar26 + -0x10;
        puVar26[-2] = *puVar1;
        puVar26[-1] = uVar10;
        puVar26[-4] = uVar11;
        puVar26[-3] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x30);
        uVar10 = puVar1[1];
        puVar2 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x40);
        uVar11 = *puVar2;
        uVar32 = puVar2[1];
        uVar31 = uVar31 - 1;
        puVar26[-6] = *puVar1;
        puVar26[-5] = uVar10;
        puVar26[-8] = uVar11;
        puVar26[-7] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x50);
        uVar10 = puVar1[1];
        puVar2 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x60);
        uVar11 = *puVar2;
        uVar32 = puVar2[1];
        puVar26[-10] = *puVar1;
        puVar26[-9] = uVar10;
        puVar26[-0xc] = uVar11;
        puVar26[-0xb] = uVar32;
        puVar1 = (undefined8 *)((longlong)puVar26 + lVar30 + -0x70);
        uVar10 = *puVar1;
        uVar11 = puVar1[1];
        uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
        uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
        if (uVar31 == 0) break;
        puVar26[-0xe] = uVar10;
        puVar26[-0xd] = uVar11;
        *puVar23 = uVar32;
        puVar26[-0xf] = uVar33;
        puVar26 = puVar23;
      }
      puVar26[-0xe] = uVar10;
      puVar26[-0xd] = uVar11;
      uVar29 = uVar29 & 0x7f;
    }
    for (uVar31 = uVar29 >> 4; uVar31 != 0; uVar31 = uVar31 - 1) {
      *puVar23 = uVar32;
      puVar23[1] = uVar33;
      puVar23 = puVar23 + -2;
      uVar32 = *(undefined8 *)((longlong)puVar23 + lVar30);
      uVar33 = ((undefined8 *)((longlong)puVar23 + lVar30))[1];
    }
    if ((uVar29 & 0xf) != 0) {
      uVar10 = param_2[1];
      *param_1 = *param_2;
      param_1[1] = uVar10;
    }
    *puVar23 = uVar32;
    puVar23[1] = uVar33;
    return param_1;
  }
  if (DAT_143a8b918 < 3) {
    if ((param_3 < 0x801) || (((byte)DAT_143ae2c20 & 2) == 0)) {
      if (0x80 < param_3) {
        lVar30 = ((ulonglong)param_1 & 0xf) - 0x10;
        param_1 = (undefined8 *)((longlong)param_1 - lVar30);
        param_2 = (undefined8 *)((longlong)param_2 - lVar30);
        param_3 = param_3 + lVar30;
        if (0x80 < param_3) {
          do {
            uVar10 = param_2[1];
            uVar11 = param_2[2];
            uVar32 = param_2[3];
            uVar33 = param_2[4];
            uVar12 = param_2[5];
            uVar13 = param_2[6];
            uVar14 = param_2[7];
            *param_1 = *param_2;
            param_1[1] = uVar10;
            param_1[2] = uVar11;
            param_1[3] = uVar32;
            param_1[4] = uVar33;
            param_1[5] = uVar12;
            param_1[6] = uVar13;
            param_1[7] = uVar14;
            uVar10 = param_2[9];
            uVar11 = param_2[10];
            uVar32 = param_2[0xb];
            uVar33 = param_2[0xc];
            uVar12 = param_2[0xd];
            uVar13 = param_2[0xe];
            uVar14 = param_2[0xf];
            param_1[8] = param_2[8];
            param_1[9] = uVar10;
            param_1[10] = uVar11;
            param_1[0xb] = uVar32;
            param_1[0xc] = uVar33;
            param_1[0xd] = uVar12;
            param_1[0xe] = uVar13;
            param_1[0xf] = uVar14;
            param_1 = param_1 + 0x10;
            param_2 = param_2 + 0x10;
            param_3 = param_3 - 0x80;
          } while (0x7f < param_3);
        }
      }
                    /* WARNING: Could not recover jumptable at 0x000142ef80b6. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      puVar23 = (undefined8 *)
                (*(code *)(IMAGE_DOS_HEADER_140000000.e_magic +
                          *(uint *)(&DAT_143c47088 + (param_3 + 0xf >> 4) * 4)))();
      return puVar23;
    }
  }
  else if (((param_3 < 0x2001) || (0x180000 < param_3)) || (((byte)DAT_143ae2c20 & 2) == 0)) {
    uVar10 = *param_2;
    uVar11 = param_2[1];
    uVar32 = param_2[2];
    uVar33 = param_2[3];
    puVar26 = (undefined8 *)((longlong)param_2 + (param_3 - 0x20));
    uVar12 = *puVar26;
    uVar13 = puVar26[1];
    uVar14 = puVar26[2];
    uVar15 = puVar26[3];
    if (0x100 < param_3) {
      lVar30 = ((ulonglong)param_1 & 0x1f) - 0x20;
      pauVar24 = (undefined1 (*) [32])((longlong)param_1 - lVar30);
      pauVar27 = (undefined1 (*) [32])((longlong)param_2 - lVar30);
      param_3 = param_3 + lVar30;
      if (0x100 < param_3) {
        if (0x180000 < param_3) {
          do {
            uVar29 = param_3;
            pauVar28 = pauVar27;
            pauVar25 = pauVar24;
            auVar3 = pauVar28[1];
            auVar4 = pauVar28[2];
            auVar5 = pauVar28[3];
            auVar6 = vmovntdq_avx(*pauVar28);
            *pauVar25 = auVar6;
            auVar3 = vmovntdq_avx(auVar3);
            pauVar25[1] = auVar3;
            auVar3 = vmovntdq_avx(auVar4);
            pauVar25[2] = auVar3;
            auVar3 = vmovntdq_avx(auVar5);
            pauVar25[3] = auVar3;
            auVar3 = pauVar28[5];
            auVar4 = pauVar28[6];
            auVar5 = pauVar28[7];
            auVar6 = vmovntdq_avx(pauVar28[4]);
            pauVar25[4] = auVar6;
            auVar3 = vmovntdq_avx(auVar3);
            pauVar25[5] = auVar3;
            auVar3 = vmovntdq_avx(auVar4);
            pauVar25[6] = auVar3;
            auVar3 = vmovntdq_avx(auVar5);
            pauVar25[7] = auVar3;
            pauVar24 = pauVar25 + 8;
            pauVar27 = pauVar28 + 8;
            param_3 = uVar29 - 0x100;
          } while (0xff < uVar29 - 0x100);
          uVar31 = uVar29 - 0xe1 & 0xffffffffffffffe0;
          switch(uVar29) {
          case 0x1e1:
          case 0x1e2:
          case 0x1e3:
          case 0x1e4:
          case 0x1e5:
          case 0x1e6:
          case 0x1e7:
          case 0x1e8:
          case 0x1e9:
          case 0x1ea:
          case 0x1eb:
          case 0x1ec:
          case 0x1ed:
          case 0x1ee:
          case 0x1ef:
          case 0x1f0:
          case 0x1f1:
          case 0x1f2:
          case 499:
          case 500:
          case 0x1f5:
          case 0x1f6:
          case 0x1f7:
          case 0x1f8:
          case 0x1f9:
          case 0x1fa:
          case 0x1fb:
          case 0x1fc:
          case 0x1fd:
          case 0x1fe:
          case 0x1ff:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(*pauVar28 + uVar31));
            *(undefined1 (*) [32])(*pauVar25 + uVar31) = auVar3;
          case 0x1c1:
          case 0x1c2:
          case 0x1c3:
          case 0x1c4:
          case 0x1c5:
          case 0x1c6:
          case 0x1c7:
          case 0x1c8:
          case 0x1c9:
          case 0x1ca:
          case 0x1cb:
          case 0x1cc:
          case 0x1cd:
          case 0x1ce:
          case 0x1cf:
          case 0x1d0:
          case 0x1d1:
          case 0x1d2:
          case 0x1d3:
          case 0x1d4:
          case 0x1d5:
          case 0x1d6:
          case 0x1d7:
          case 0x1d8:
          case 0x1d9:
          case 0x1da:
          case 0x1db:
          case 0x1dc:
          case 0x1dd:
          case 0x1de:
          case 0x1df:
          case 0x1e0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[1] + uVar31));
            *(undefined1 (*) [32])(pauVar25[1] + uVar31) = auVar3;
          case 0x1a1:
          case 0x1a2:
          case 0x1a3:
          case 0x1a4:
          case 0x1a5:
          case 0x1a6:
          case 0x1a7:
          case 0x1a8:
          case 0x1a9:
          case 0x1aa:
          case 0x1ab:
          case 0x1ac:
          case 0x1ad:
          case 0x1ae:
          case 0x1af:
          case 0x1b0:
          case 0x1b1:
          case 0x1b2:
          case 0x1b3:
          case 0x1b4:
          case 0x1b5:
          case 0x1b6:
          case 0x1b7:
          case 0x1b8:
          case 0x1b9:
          case 0x1ba:
          case 0x1bb:
          case 0x1bc:
          case 0x1bd:
          case 0x1be:
          case 0x1bf:
          case 0x1c0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[2] + uVar31));
            *(undefined1 (*) [32])(pauVar25[2] + uVar31) = auVar3;
          case 0x181:
          case 0x182:
          case 0x183:
          case 0x184:
          case 0x185:
          case 0x186:
          case 0x187:
          case 0x188:
          case 0x189:
          case 0x18a:
          case 0x18b:
          case 0x18c:
          case 0x18d:
          case 0x18e:
          case 399:
          case 400:
          case 0x191:
          case 0x192:
          case 0x193:
          case 0x194:
          case 0x195:
          case 0x196:
          case 0x197:
          case 0x198:
          case 0x199:
          case 0x19a:
          case 0x19b:
          case 0x19c:
          case 0x19d:
          case 0x19e:
          case 0x19f:
          case 0x1a0:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[3] + uVar31));
            *(undefined1 (*) [32])(pauVar25[3] + uVar31) = auVar3;
          case 0x161:
          case 0x162:
          case 0x163:
          case 0x164:
          case 0x165:
          case 0x166:
          case 0x167:
          case 0x168:
          case 0x169:
          case 0x16a:
          case 0x16b:
          case 0x16c:
          case 0x16d:
          case 0x16e:
          case 0x16f:
          case 0x170:
          case 0x171:
          case 0x172:
          case 0x173:
          case 0x174:
          case 0x175:
          case 0x176:
          case 0x177:
          case 0x178:
          case 0x179:
          case 0x17a:
          case 0x17b:
          case 0x17c:
          case 0x17d:
          case 0x17e:
          case 0x17f:
          case 0x180:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[4] + uVar31));
            *(undefined1 (*) [32])(pauVar25[4] + uVar31) = auVar3;
          case 0x141:
          case 0x142:
          case 0x143:
          case 0x144:
          case 0x145:
          case 0x146:
          case 0x147:
          case 0x148:
          case 0x149:
          case 0x14a:
          case 0x14b:
          case 0x14c:
          case 0x14d:
          case 0x14e:
          case 0x14f:
          case 0x150:
          case 0x151:
          case 0x152:
          case 0x153:
          case 0x154:
          case 0x155:
          case 0x156:
          case 0x157:
          case 0x158:
          case 0x159:
          case 0x15a:
          case 0x15b:
          case 0x15c:
          case 0x15d:
          case 0x15e:
          case 0x15f:
          case 0x160:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[5] + uVar31));
            *(undefined1 (*) [32])(pauVar25[5] + uVar31) = auVar3;
          case 0x121:
          case 0x122:
          case 0x123:
          case 0x124:
          case 0x125:
          case 0x126:
          case 0x127:
          case 0x128:
          case 0x129:
          case 0x12a:
          case 299:
          case 300:
          case 0x12d:
          case 0x12e:
          case 0x12f:
          case 0x130:
          case 0x131:
          case 0x132:
          case 0x133:
          case 0x134:
          case 0x135:
          case 0x136:
          case 0x137:
          case 0x138:
          case 0x139:
          case 0x13a:
          case 0x13b:
          case 0x13c:
          case 0x13d:
          case 0x13e:
          case 0x13f:
          case 0x140:
            auVar3 = vmovntdq_avx(*(undefined1 (*) [32])(pauVar28[6] + uVar31));
            *(undefined1 (*) [32])(pauVar25[6] + uVar31) = auVar3;
          default:
            puVar26 = (undefined8 *)(pauVar25[-1] + uVar29);
            *puVar26 = uVar12;
            puVar26[1] = uVar13;
            puVar26[2] = uVar14;
            puVar26[3] = uVar15;
          case 0x100:
            *param_1 = uVar10;
            param_1[1] = uVar11;
            param_1[2] = uVar32;
            param_1[3] = uVar33;
            return puVar23;
          }
        }
        do {
          uVar10 = *(undefined8 *)(*pauVar27 + 8);
          uVar11 = *(undefined8 *)(*pauVar27 + 0x10);
          uVar32 = *(undefined8 *)(*pauVar27 + 0x18);
          uVar33 = *(undefined8 *)pauVar27[1];
          uVar12 = *(undefined8 *)(pauVar27[1] + 8);
          uVar13 = *(undefined8 *)(pauVar27[1] + 0x10);
          uVar14 = *(undefined8 *)(pauVar27[1] + 0x18);
          uVar15 = *(undefined8 *)pauVar27[2];
          uVar16 = *(undefined8 *)(pauVar27[2] + 8);
          uVar17 = *(undefined8 *)(pauVar27[2] + 0x10);
          uVar18 = *(undefined8 *)(pauVar27[2] + 0x18);
          uVar19 = *(undefined8 *)pauVar27[3];
          uVar20 = *(undefined8 *)(pauVar27[3] + 8);
          uVar21 = *(undefined8 *)(pauVar27[3] + 0x10);
          uVar22 = *(undefined8 *)(pauVar27[3] + 0x18);
          *(undefined8 *)*pauVar24 = *(undefined8 *)*pauVar27;
          *(undefined8 *)(*pauVar24 + 8) = uVar10;
          *(undefined8 *)(*pauVar24 + 0x10) = uVar11;
          *(undefined8 *)(*pauVar24 + 0x18) = uVar32;
          *(undefined8 *)pauVar24[1] = uVar33;
          *(undefined8 *)(pauVar24[1] + 8) = uVar12;
          *(undefined8 *)(pauVar24[1] + 0x10) = uVar13;
          *(undefined8 *)(pauVar24[1] + 0x18) = uVar14;
          *(undefined8 *)pauVar24[2] = uVar15;
          *(undefined8 *)(pauVar24[2] + 8) = uVar16;
          *(undefined8 *)(pauVar24[2] + 0x10) = uVar17;
          *(undefined8 *)(pauVar24[2] + 0x18) = uVar18;
          *(undefined8 *)pauVar24[3] = uVar19;
          *(undefined8 *)(pauVar24[3] + 8) = uVar20;
          *(undefined8 *)(pauVar24[3] + 0x10) = uVar21;
          *(undefined8 *)(pauVar24[3] + 0x18) = uVar22;
          uVar10 = *(undefined8 *)(pauVar27[4] + 8);
          uVar11 = *(undefined8 *)(pauVar27[4] + 0x10);
          uVar32 = *(undefined8 *)(pauVar27[4] + 0x18);
          uVar33 = *(undefined8 *)pauVar27[5];
          uVar12 = *(undefined8 *)(pauVar27[5] + 8);
          uVar13 = *(undefined8 *)(pauVar27[5] + 0x10);
          uVar14 = *(undefined8 *)(pauVar27[5] + 0x18);
          uVar15 = *(undefined8 *)pauVar27[6];
          uVar16 = *(undefined8 *)(pauVar27[6] + 8);
          uVar17 = *(undefined8 *)(pauVar27[6] + 0x10);
          uVar18 = *(undefined8 *)(pauVar27[6] + 0x18);
          uVar19 = *(undefined8 *)pauVar27[7];
          uVar20 = *(undefined8 *)(pauVar27[7] + 8);
          uVar21 = *(undefined8 *)(pauVar27[7] + 0x10);
          uVar22 = *(undefined8 *)(pauVar27[7] + 0x18);
          *(undefined8 *)pauVar24[4] = *(undefined8 *)pauVar27[4];
          *(undefined8 *)(pauVar24[4] + 8) = uVar10;
          *(undefined8 *)(pauVar24[4] + 0x10) = uVar11;
          *(undefined8 *)(pauVar24[4] + 0x18) = uVar32;
          *(undefined8 *)pauVar24[5] = uVar33;
          *(undefined8 *)(pauVar24[5] + 8) = uVar12;
          *(undefined8 *)(pauVar24[5] + 0x10) = uVar13;
          *(undefined8 *)(pauVar24[5] + 0x18) = uVar14;
          *(undefined8 *)pauVar24[6] = uVar15;
          *(undefined8 *)(pauVar24[6] + 8) = uVar16;
          *(undefined8 *)(pauVar24[6] + 0x10) = uVar17;
          *(undefined8 *)(pauVar24[6] + 0x18) = uVar18;
          *(undefined8 *)pauVar24[7] = uVar19;
          *(undefined8 *)(pauVar24[7] + 8) = uVar20;
          *(undefined8 *)(pauVar24[7] + 0x10) = uVar21;
          *(undefined8 *)(pauVar24[7] + 0x18) = uVar22;
          pauVar24 = pauVar24 + 8;
          pauVar27 = pauVar27 + 8;
          param_3 = param_3 - 0x100;
        } while (0xff < param_3);
      }
    }
                    /* WARNING: Could not recover jumptable at 0x000142ef7e12. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    puVar23 = (undefined8 *)
              (*(code *)(IMAGE_DOS_HEADER_140000000.e_magic +
                        *(uint *)(&DAT_143c47040 + (param_3 + 0x1f >> 5) * 4)))();
    return puVar23;
  }
  for (; param_3 != 0; param_3 = param_3 - 1) {
    *(undefined1 *)param_1 = *(undefined1 *)param_2;
    param_2 = (undefined8 *)((longlong)param_2 + 1);
    param_1 = (undefined8 *)((longlong)param_1 + 1);
  }
  return puVar23;
}



//===========================================================
// FUN_1401ba9d0 @ 1401ba9d0   (140 bytes)
//===========================================================

uint FUN_1401ba9d0(uint *param_1,int param_2)

{
  uint uVar1;
  uint uVar2;
  undefined8 *puVar3;
  uint uVar4;
  int local_res8 [2];
  int local_res10 [2];
  undefined8 local_res18;
  longlong local_res20;
  
  uVar1 = *param_1;
  uVar2 = param_1[1];
  uVar4 = uVar1 ^ 0xbaadf00d;
  local_res8[0] = (uVar4 >> 5 | uVar4 << 0x1b) + param_1[1];
  if (local_res8[0] != param_2) {
    local_res10[0] = param_2;
    local_res18 = FUN_1418039d0(5);
    puVar3 = (undefined8 *)FUN_1401a0ed0(&local_res20,&local_res18,local_res8,local_res10);
    FUN_141804970(&DAT_143271f04,0x53,5,*puVar3);
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
  }
  return (uVar2 << 5 | uVar2 >> 0x1b) ^ uVar1;
}



//===========================================================
// FUN_1402e5140 @ 1402e5140   (138 bytes)
//===========================================================

undefined8 FUN_1402e5140(longlong param_1)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined4 uVar6;
  
  uVar1 = FUN_1402e52a0();
  if (uVar1 != 0) {
    lVar5 = *(longlong *)(param_1 + 0x5f8);
    uVar6 = 0;
    uVar2 = 0;
    uVar3 = uVar2;
    if (lVar5 != 0) {
      uVar3 = *(uint *)(lVar5 + -8);
    }
    if ((int)uVar1 <= (int)(uVar3 - 1)) {
      if (lVar5 != 0) {
        uVar2 = *(uint *)(lVar5 + -8);
      }
      if (((int)uVar1 < 0) || (uVar2 <= uVar1)) {
        if (lVar5 != 0) {
          uVar6 = *(undefined4 *)(lVar5 + -8);
        }
        FUN_142e54290(0xc6,uVar1,uVar6);
        lVar5 = *(longlong *)(param_1 + 0x5f8);
      }
      uVar4 = FUN_140192f80(*(undefined8 *)(lVar5 + 8 + (longlong)(int)uVar1 * 0x10));
      return uVar4;
    }
  }
  return 0;
}



//===========================================================
// FUN_14038a300 @ 14038a300   (115 bytes)
//===========================================================

bool FUN_14038a300(undefined8 param_1,undefined4 param_2)

{
  undefined *puVar1;
  int iVar2;
  bool bVar3;
  longlong *local_res18;
  longlong *local_res20;
  
  FUN_14039f600(param_1,&local_res20,param_2,0);
  puVar1 = PTR_u_limitedLife_143a46180;
  if (local_res20 == (longlong *)0x0) {
    bVar3 = false;
  }
  else {
    local_res18 = local_res20;
    (**(code **)(*local_res20 + 8))(local_res20);
    iVar2 = FUN_140910eb0(&local_res18,puVar1,0);
    bVar3 = 0 < iVar2;
    (**(code **)(*local_res20 + 0x10))(local_res20);
  }
  return bVar3;
}



//===========================================================
// FUN_14038a380 @ 14038a380   (118 bytes)
//===========================================================

bool FUN_14038a380(undefined8 param_1,undefined4 param_2)

{
  undefined *puVar1;
  int iVar2;
  bool bVar3;
  longlong *local_res18;
  longlong *local_res20;
  
  FUN_14039f600(param_1,&local_res20,param_2,0);
  puVar1 = PTR_u_life_143a46170;
  if (local_res20 == (longlong *)0x0) {
    bVar3 = false;
  }
  else {
    local_res18 = local_res20;
    (**(code **)(*local_res20 + 8))(local_res20);
    iVar2 = FUN_140910eb0(&local_res18,puVar1,1);
    bVar3 = iVar2 == 0;
    (**(code **)(*local_res20 + 0x10))(local_res20);
  }
  return bVar3;
}



//===========================================================
// FUN_1402de270 @ 1402de270   (237 bytes)
//===========================================================

longlong FUN_1402de270(longlong param_1,int param_2)

{
  longlong *plVar1;
  int iVar2;
  ulonglong uVar3;
  uint uVar4;
  longlong lVar5;
  uint uVar6;
  uint uVar7;
  
  if ((((param_2 - 0x4b0U < 0xe) || (param_2 - 0x708U < 0x33)) || (param_2 - 3000U < 0x20)) ||
     ((param_2 - 0xc1cU < 0x20 || (param_2 - 0xc80U < 0x20)))) {
    uVar6 = 0;
    uVar3 = 0;
    uVar4 = uVar6;
    do {
      if (((uVar3 < 5) && ((int)(&DAT_14327dd50)[uVar3] <= param_2)) &&
         (param_2 < (int)(&DAT_14327dd68)[uVar3])) break;
      uVar4 = uVar4 + 1;
      uVar3 = uVar3 + 1;
    } while ((longlong)uVar3 < 5);
    if (uVar4 < 5) {
      uVar7 = param_2 - (&DAT_14327dd50)[(int)uVar4];
      plVar1 = (longlong *)(param_1 + (longlong)(int)uVar4 * 8);
      if (-1 < (int)uVar7) {
        lVar5 = *plVar1;
        iVar2 = 0;
        if (lVar5 != 0) {
          iVar2 = *(int *)(lVar5 + -8);
        }
        if ((int)uVar7 < iVar2) {
          uVar4 = uVar6;
          if (lVar5 != 0) {
            uVar4 = *(uint *)(lVar5 + -8);
          }
          if (uVar4 <= uVar7) {
            if (lVar5 != 0) {
              uVar6 = *(uint *)(lVar5 + -8);
            }
            FUN_142e54290(0xbc,uVar7,uVar6);
            lVar5 = *plVar1;
          }
          return (longlong)(int)uVar7 * 0x10 + lVar5;
        }
      }
    }
  }
  return 0;
}


