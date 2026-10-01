
//===========================================================
// FUN_1402d4870 @ 1402d4870   (20 bytes)
//===========================================================

void FUN_1402d4870(undefined8 param_1,undefined8 param_2)

{
  FUN_1406e9170(param_2,param_1,0x149);
  return;
}



//===========================================================
// FUN_142deaf60 @ 142deaf60   (162 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 FUN_142deaf60(longlong *param_1)

{
  int iVar1;
  int iVar2;
  undefined1 auStack_488 [32];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_488;
  iVar1 = 0;
  if (*param_1 == 0) {
    iVar2 = 0;
  }
  else {
    iVar2 = *(int *)(*param_1 + -8);
  }
  if (param_1[1] != 0) {
    iVar1 = *(int *)(param_1[1] + -8);
  }
  if (iVar2 == iVar1) {
    return 0;
  }
  FUN_1406ed520(local_468,0x193);
  FUN_1406ed840(local_468,0);
  FUN_1415d01c0(local_468);
  FUN_1406ed610(local_468);
  return 1;
}



//===========================================================
// FUN_142d2e9e0 @ 142d2e9e0   (40 bytes)
//===========================================================

longlong FUN_142d2e9e0(longlong param_1)

{
  longlong lVar1;
  
  lVar1 = *(longlong *)(param_1 + 8);
  if (lVar1 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar1 = *(longlong *)(param_1 + 8);
  }
  return lVar1;
}



//===========================================================
// FUN_142df2e20 @ 142df2e20   (439 bytes)
//===========================================================

longlong FUN_142df2e20(ulonglong *param_1,uint param_2)

{
  undefined1 auVar1 [16];
  undefined1 auVar2 [16];
  ulonglong uVar3;
  longlong lVar4;
  undefined4 *puVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  uint uVar8;
  uint uVar9;
  ulonglong uVar10;
  
  uVar7 = *param_1;
  uVar10 = 0;
  uVar9 = 0;
  if (uVar7 != 0) {
    uVar9 = *(uint *)(uVar7 - 8);
  }
  if (param_2 == 0xffffffff) {
    param_2 = uVar9;
  }
  if (uVar7 != 0) {
    uVar6 = *(ulonglong *)(uVar7 - 0x10);
    uVar3 = ~uVar6;
    if (-1 < (longlong)uVar6) {
      uVar3 = uVar6;
    }
    auVar1._8_8_ = 0;
    auVar1._0_8_ = uVar3 - 8;
    lVar4 = SUB168(ZEXT816(0x8e6527af1373f071) * auVar1,8);
    if (uVar9 < (uint)(((uVar3 - 8) - lVar4 >> 1) + lVar4 >> 8)) goto LAB_142df2f43;
  }
  uVar8 = 1;
  if (uVar9 != 0) {
    uVar8 = uVar9 * 2;
  }
  uVar6 = uVar10;
  if (uVar7 != 0) {
    uVar6 = *(ulonglong *)(uVar7 - 0x10);
    uVar3 = ~uVar6;
    if (-1 < (longlong)uVar6) {
      uVar3 = uVar6;
    }
    auVar2._8_8_ = 0;
    auVar2._0_8_ = uVar3 - 8;
    lVar4 = SUB168(ZEXT816(0x8e6527af1373f071) * auVar2,8);
    uVar6 = ((uVar3 - 8) - lVar4 >> 1) + lVar4 >> 8;
  }
  if ((uint)uVar6 != uVar8) {
    uVar6 = uVar10;
    if (uVar7 != 0) {
      uVar6 = (ulonglong)*(uint *)(uVar7 - 8);
    }
    lVar4 = FUN_14019b780(&DAT_143ad68a0,(ulonglong)uVar8 * 0x149 + 8);
    uVar7 = lVar4 + 8;
    if (lVar4 == 0) {
      uVar7 = uVar10;
    }
    if (*param_1 != 0) {
      FUN_142ef7ba0(uVar7,*param_1,uVar6 * 0x149);
      thunk_FUN_140205820(*param_1 - 8,0);
    }
    *param_1 = uVar7;
    *(ulonglong *)(uVar7 - 8) = uVar6;
  }
LAB_142df2f43:
  *(longlong *)(*param_1 - 8) = *(longlong *)(*param_1 - 8) + 1;
  lVar4 = (longlong)(int)param_2 * 0x149;
  FUN_142ef7ba0(*param_1 + lVar4 + 0x149,*param_1 + lVar4,(ulonglong)(uVar9 - param_2) * 0x149);
  puVar5 = (undefined4 *)(*param_1 + lVar4);
  *puVar5 = 0;
  *(undefined8 *)(puVar5 + 1) = 0;
  puVar5[3] = 0;
  *(undefined2 *)(puVar5 + 4) = 0;
  *(undefined4 *)((longlong)puVar5 + 0x12) = 0xffffffff;
  *(undefined8 *)((longlong)puVar5 + 0x16) = 0;
  *(undefined8 *)((longlong)puVar5 + 0x1e) = 0;
  *(undefined2 *)((longlong)puVar5 + 0x26) = 0;
  puVar5[10] = 0;
  FUN_142ef8250(puVar5 + 0xb,0,0x10d);
  *(undefined8 *)((longlong)puVar5 + 0x139) = 0;
  *(undefined8 *)((longlong)puVar5 + 0x141) = 0;
  return *param_1 + lVar4;
}



//===========================================================
// FUN_142decf30 @ 142decf30   (556 bytes)
//===========================================================

undefined8
FUN_142decf30(longlong param_1,int param_2,int param_3,int param_4,undefined4 *param_5,char param_6)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 *puVar7;
  int iVar8;
  longlong lVar9;
  undefined8 *puVar10;
  undefined8 uVar11;
  uint uVar12;
  undefined8 *puVar13;
  undefined8 *puVar14;
  uint uVar15;
  longlong lVar16;
  longlong *plVar17;
  undefined4 uVar18;
  undefined8 local_178 [2];
  char cStack_167;
  
  *param_5 = 0;
  if (param_6 == '\0') {
    if ((param_4 == 0) || (param_2 == 0)) {
LAB_142ded11d:
      if ((DAT_143ac87a0 != 0) && (*(int *)(DAT_143ac87a0 + 0x160) == 0)) goto LAB_142ded132;
    }
    else {
      plVar17 = *(longlong **)(param_1 + 0x23c8);
      if (plVar17 == (longlong *)0x0) {
        FUN_142e52ed0(0x431,0);
        plVar17 = *(longlong **)(param_1 + 0x23c8);
      }
      iVar8 = FUN_142deaf60(plVar17);
      if (iVar8 != 0) goto LAB_142ded11d;
      uVar15 = 0;
      lVar16 = 0;
      while( true ) {
        FUN_142deaf60();
        lVar9 = *plVar17;
        if (lVar9 == 0) {
          iVar8 = 0;
        }
        else {
          iVar8 = *(int *)(lVar9 + -8);
        }
        if (iVar8 <= (int)uVar15) goto LAB_142ded11d;
        if (lVar9 == 0) {
          uVar12 = 0;
        }
        else {
          uVar12 = *(uint *)(lVar9 + -8);
        }
        if (((int)uVar15 < 0) || (uVar12 <= uVar15)) {
          if (lVar9 == 0) {
            uVar18 = 0;
          }
          else {
            uVar18 = *(undefined4 *)(lVar9 + -8);
          }
          FUN_142e54290(0xbc,uVar15,uVar18);
          lVar9 = *plVar17;
        }
        if (*(int *)(lVar16 + lVar9) == param_2) break;
        uVar15 = uVar15 + 1;
        lVar16 = lVar16 + 0x149;
      }
      if ((int)uVar15 < 0) goto LAB_142ded11d;
      lVar16 = *(longlong *)(param_1 + 0x23c8);
      if (lVar16 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar16 = *(longlong *)(param_1 + 0x23c8);
      }
      puVar10 = (undefined8 *)FUN_142d241a0(lVar16,uVar15);
      lVar16 = 2;
      puVar7 = local_178;
      do {
        puVar14 = puVar7;
        puVar13 = puVar10;
        uVar11 = puVar13[1];
        uVar4 = puVar13[2];
        uVar5 = puVar13[3];
        *puVar14 = *puVar13;
        puVar14[1] = uVar11;
        uVar11 = puVar13[4];
        uVar6 = puVar13[5];
        puVar14[2] = uVar4;
        puVar14[3] = uVar5;
        uVar4 = puVar13[6];
        uVar5 = puVar13[7];
        puVar14[4] = uVar11;
        puVar14[5] = uVar6;
        uVar11 = puVar13[8];
        uVar6 = puVar13[9];
        puVar14[6] = uVar4;
        puVar14[7] = uVar5;
        uVar4 = puVar13[10];
        uVar5 = puVar13[0xb];
        puVar14[8] = uVar11;
        puVar14[9] = uVar6;
        uVar11 = puVar13[0xc];
        uVar6 = puVar13[0xd];
        puVar14[10] = uVar4;
        puVar14[0xb] = uVar5;
        uVar4 = puVar13[0xe];
        uVar5 = puVar13[0xf];
        puVar14[0xc] = uVar11;
        puVar14[0xd] = uVar6;
        puVar14[0xe] = uVar4;
        puVar14[0xf] = uVar5;
        lVar16 = lVar16 + -1;
        puVar10 = puVar13 + 0x10;
        puVar7 = puVar14 + 0x10;
      } while (lVar16 != 0);
      uVar4 = puVar13[0x11];
      uVar11 = puVar13[0x18];
      uVar5 = puVar13[0x12];
      uVar6 = puVar13[0x13];
      puVar14[0x10] = puVar13[0x10];
      puVar14[0x11] = uVar4;
      uVar18 = *(undefined4 *)(puVar13 + 0x14);
      uVar1 = *(undefined4 *)((longlong)puVar13 + 0xa4);
      uVar2 = *(undefined4 *)(puVar13 + 0x15);
      uVar3 = *(undefined4 *)((longlong)puVar13 + 0xac);
      puVar14[0x12] = uVar5;
      puVar14[0x13] = uVar6;
      uVar4 = puVar13[0x16];
      uVar5 = puVar13[0x17];
      *(undefined4 *)(puVar14 + 0x14) = uVar18;
      *(undefined4 *)((longlong)puVar14 + 0xa4) = uVar1;
      *(undefined4 *)(puVar14 + 0x15) = uVar2;
      *(undefined4 *)((longlong)puVar14 + 0xac) = uVar3;
      puVar14[0x16] = uVar4;
      puVar14[0x17] = uVar5;
      puVar14[0x18] = uVar11;
      *(undefined1 *)(puVar14 + 0x19) = *(undefined1 *)(puVar13 + 0x19);
      if ((cStack_167 != '\0') && (1 < (byte)(cStack_167 - 3U))) goto LAB_142ded11d;
      *param_5 = 1;
    }
    uVar11 = 1;
  }
  else {
LAB_142ded132:
    lVar16 = FUN_141892840();
    if (lVar16 != 0) {
      uVar11 = FUN_141892840();
      if (param_4 != 0) {
        param_2 = param_3;
      }
      FUN_1418297d0(uVar11,param_2,param_4,1);
    }
    uVar11 = 0;
  }
  return uVar11;
}



//===========================================================
// FUN_1415fe510 @ 1415fe510   (57 bytes)
//===========================================================

undefined8 FUN_1415fe510(longlong param_1,undefined8 param_2,int param_3)

{
  int iVar1;
  longlong lVar2;
  
  if (param_3 == 0) {
    lVar2 = param_1 + 0x2a0;
  }
  else {
    if (param_3 != 1) {
      return 0;
    }
    lVar2 = param_1 + 0x2a8;
  }
  iVar1 = FUN_1415fe590(param_1,param_2,lVar2);
  if (iVar1 < 0) {
    return 0;
  }
  return 1;
}



//===========================================================
// FUN_142dec8f0 @ 142dec8f0   (1589 bytes)
//===========================================================

void FUN_142dec8f0(longlong param_1)

{
  undefined4 uVar1;
  char cVar2;
  uint uVar3;
  int iVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined4 *puVar7;
  longlong *plVar8;
  undefined4 *puVar9;
  ulonglong uVar10;
  longlong lVar11;
  ulonglong uVar12;
  uint uVar13;
  int iVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  uint uVar17;
  uint local_res10;
  undefined4 local_res18 [2];
  undefined4 *local_res20;
  undefined4 *in_stack_ffffffffffffff38;
  undefined4 *local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  ulonglong local_90;
  longlong local_80;
  longlong local_70;
  longlong local_60;
  longlong local_50;
  longlong local_48;
  longlong local_40;
  
  uVar10 = 0;
  local_a0 = 0;
  cVar2 = FUN_142d98230(param_1,0xe,&local_a0,0);
  if (cVar2 == '\0') {
    local_98 = 0;
    cVar2 = FUN_142d98230(param_1,0xf,&local_98,0);
    if ((cVar2 == '\0') && (*(int *)(param_1 + 0x3488) != 0)) {
      local_res10 = 0;
      uVar12 = uVar10;
      while( true ) {
        uVar15 = 0;
        uVar17 = (uint)uVar12;
        plVar8 = *(longlong **)(param_1 + 0x23c8);
        local_90 = uVar10;
        if (plVar8 == (longlong *)0x0) {
          FUN_142e52ed0(0x431,0);
          plVar8 = *(longlong **)(param_1 + 0x23c8);
        }
        FUN_142deaf60(plVar8);
        uVar13 = 0;
        uVar3 = uVar13;
        if (*plVar8 != 0) {
          uVar3 = *(uint *)(*plVar8 + -8);
        }
        if ((int)uVar3 <= (int)uVar17) break;
        plVar8 = *(longlong **)(param_1 + 0x23c8);
        if (plVar8 == (longlong *)0x0) {
          FUN_142e52ed0(0x431,0);
          plVar8 = *(longlong **)(param_1 + 0x23c8);
        }
        lVar5 = *plVar8;
        if (lVar5 != 0) {
          uVar13 = *(uint *)(lVar5 + -8);
        }
        if (((int)uVar17 < 0) || (uVar13 <= uVar17)) {
          uVar16 = uVar15;
          if (lVar5 != 0) {
            uVar16 = (ulonglong)*(uint *)(lVar5 + -8);
          }
          FUN_142e54290(0xbc,uVar12,uVar16);
          lVar5 = *plVar8;
        }
        puVar9 = (undefined4 *)(uVar10 * 0x149 + lVar5);
        local_res18[0] = 0;
        iVar4 = FUN_142d01050(param_1);
        if (*(char *)((longlong)puVar9 + 0x11) == '\x01') {
          uVar1 = *puVar9;
          if ((iVar4 == 0) && ((DAT_143ac87a0 == 0 || (*(int *)(DAT_143ac87a0 + 0x160) != 0)))) {
            local_48 = FUN_14019b780(&DAT_143ad68a0,0x370);
            if (local_48 == 0) {
              lVar5 = 0;
            }
            else {
              lVar5 = FUN_141808b90(local_48);
            }
            lVar11 = lVar5 + 0x18;
            if (lVar5 == 0) {
              lVar11 = 0;
            }
            if (lVar11 == 0) {
              local_80 = 0;
            }
            else {
              local_80 = lVar11 + -0x18;
              if (local_80 != 0) {
                if (0xfffff < *(ulonglong *)(lVar11 + 8)) {
                  FUN_142e541f0(0x30f);
                }
                LOCK();
                *(longlong *)(lVar11 + 8) = *(longlong *)(lVar11 + 8) + 1;
                UNLOCK();
              }
            }
            lVar5 = local_80;
            if (local_80 == 0) {
              FUN_142e52ed0(0x431,0);
            }
            uVar1 = *puVar9;
            puVar9 = puVar9 + 1;
            local_res20 = (undefined4 *)0x0;
            if (puVar9 != (undefined4 *)0x0) {
              uVar10 = 0xffffffffffffffff;
              do {
                uVar10 = uVar10 + 1;
              } while (*(char *)((longlong)puVar9 + uVar10) != '\0');
              iVar14 = (int)uVar10;
              iVar4 = 0;
              if (0 < iVar14) {
                iVar4 = iVar14;
              }
              puVar7 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
              puVar7[1] = iVar4;
              *puVar7 = 0xffffffff;
              local_res20 = puVar7 + 4;
              puVar7[2] = 0;
              *(undefined1 *)local_res20 = 0;
              FUN_142ef7ba0(local_res20,puVar9,(longlong)iVar14);
              puVar9 = local_res20;
              if (local_res20[-4] != -1) {
                FUN_142e52dd0();
              }
              if ((iVar14 == -1) || (iVar14 <= (int)puVar9[-3])) {
                puVar9[-4] = 1;
                if (iVar14 != -1) goto LAB_142decb98;
                if (puVar9 == (undefined4 *)0x0) {
                  uVar10 = 0;
                }
                else {
                  uVar10 = 0xffffffffffffffff;
                  do {
                    uVar10 = uVar10 + 1;
                  } while (*(char *)((longlong)puVar9 + uVar10) != '\0');
                }
              }
              else {
                FUN_142e54290(0x90,puVar9[-3],uVar10 & 0xffffffff);
                puVar9[-4] = 1;
LAB_142decb98:
                *(undefined1 *)((longlong)iVar14 + (longlong)local_res20) = 0;
              }
              iVar4 = (int)uVar10;
              if ((iVar4 < 0) || (puVar9[-3] + 1 <= iVar4)) {
                FUN_142e54290(0x9c,uVar10 & 0xffffffff);
              }
              puVar9[-2] = iVar4;
              uVar17 = local_res10;
            }
            in_stack_ffffffffffffff38 =
                 (undefined4 *)((ulonglong)in_stack_ffffffffffffff38 & 0xffffffff00000000);
            FUN_14180e3d0(lVar5,&local_res20,uVar1,0,in_stack_ffffffffffffff38,0);
            local_60 = lVar5;
            if (lVar5 != 0) {
              if (0xfffff < *(ulonglong *)(lVar5 + 0x20)) {
                FUN_142e541f0(0x30f);
              }
              LOCK();
              *(longlong *)(lVar5 + 0x20) = *(longlong *)(lVar5 + 0x20) + 1;
              UNLOCK();
              lVar5 = local_80;
            }
            FUN_142d97880(param_1);
            uVar10 = local_90;
            if (lVar5 != 0) {
              if (0xffffe < *(longlong *)(lVar5 + 0x20) - 1U) {
                FUN_142e541f0(0x31e);
              }
              LOCK();
              plVar8 = (longlong *)(lVar5 + 0x20);
              lVar5 = *plVar8;
              *plVar8 = *plVar8 + -1;
              UNLOCK();
              if (((int)lVar5 == 1) && ((undefined8 *)(local_80 + 0x18) != (undefined8 *)0x0)) {
                (*(code *)**(undefined8 **)(local_80 + 0x18))();
              }
              local_80 = 0;
              uVar10 = local_90;
            }
            goto LAB_142decec6;
          }
          lVar5 = FUN_141892840();
          if (lVar5 == 0) goto LAB_142decec6;
          uVar6 = FUN_141892840();
          FUN_1418297d0(uVar6,uVar1,0);
          local_res10 = uVar17 + 1;
          uVar12 = (ulonglong)local_res10;
          uVar10 = uVar10 + 1;
        }
        else {
          if (*(char *)((longlong)puVar9 + 0x11) == '\x06') {
            in_stack_ffffffffffffff38 = local_res18;
            iVar4 = FUN_142decf30(param_1,*puVar9,puVar9[10],1,in_stack_ffffffffffffff38,iVar4 != 0)
            ;
            if (iVar4 != 0) {
              local_40 = FUN_14019b780(&DAT_143ad68a0,0x370);
              uVar10 = uVar15;
              if (local_40 != 0) {
                uVar10 = FUN_141808b90(local_40);
              }
              uVar12 = uVar10 + 0x18;
              if (uVar10 == 0) {
                uVar12 = uVar15;
              }
              if (uVar12 == 0) {
                local_70 = 0;
              }
              else {
                local_70 = uVar12 - 0x18;
                if (local_70 != 0) {
                  if (0xfffff < *(ulonglong *)(uVar12 + 8)) {
                    FUN_142e541f0(0x30f);
                  }
                  LOCK();
                  *(longlong *)(uVar12 + 8) = *(longlong *)(uVar12 + 8) + 1;
                  UNLOCK();
                }
              }
              lVar5 = local_70;
              if (local_70 == 0) {
                FUN_142e52ed0(0x431,0);
              }
              uVar1 = puVar9[10];
              puVar9 = puVar9 + 0xb;
              local_a8 = (undefined4 *)0x0;
              if (puVar9 != (undefined4 *)0x0) {
                uVar10 = 0xffffffffffffffff;
                do {
                  uVar10 = uVar10 + 1;
                } while (*(char *)((longlong)puVar9 + uVar10) != '\0');
                iVar4 = (int)uVar10;
                if (0 < iVar4) {
                  uVar15 = uVar10 & 0xffffffff;
                }
                puVar7 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)((int)uVar15 + 0x11));
                puVar7[1] = (int)uVar15;
                *puVar7 = 0xffffffff;
                local_a8 = puVar7 + 4;
                puVar7[2] = 0;
                *(undefined1 *)local_a8 = 0;
                FUN_142ef7ba0(local_a8,puVar9,(longlong)iVar4);
                puVar9 = local_a8;
                if (local_a8[-4] != -1) {
                  FUN_142e52dd0(0x8b);
                }
                if ((iVar4 == -1) || (iVar4 <= (int)puVar9[-3])) {
                  puVar9[-4] = 1;
                  if (iVar4 != -1) goto LAB_142decdf2;
                  if (puVar9 == (undefined4 *)0x0) {
                    uVar10 = 0;
                  }
                  else {
                    uVar10 = 0xffffffffffffffff;
                    do {
                      uVar10 = uVar10 + 1;
                    } while (*(char *)((longlong)puVar9 + uVar10) != '\0');
                  }
                }
                else {
                  FUN_142e54290(0x90,puVar9[-3],uVar10 & 0xffffffff);
                  puVar9[-4] = 1;
LAB_142decdf2:
                  *(undefined1 *)((longlong)iVar4 + (longlong)local_a8) = 0;
                }
                iVar4 = (int)uVar10;
                if ((iVar4 < 0) || (puVar9[-3] + 1 <= iVar4)) {
                  FUN_142e54290(0x9c,uVar10 & 0xffffffff);
                }
                puVar9[-2] = iVar4;
                uVar17 = local_res10;
              }
              in_stack_ffffffffffffff38 =
                   (undefined4 *)((ulonglong)in_stack_ffffffffffffff38 & 0xffffffff00000000);
              FUN_14180e8d0(lVar5,&local_a8,uVar1,0,in_stack_ffffffffffffff38,0,local_res18[0]);
              local_50 = lVar5;
              if (lVar5 != 0) {
                if (0xfffff < *(ulonglong *)(lVar5 + 0x20)) {
                  FUN_142e541f0(0x30f);
                }
                LOCK();
                *(longlong *)(lVar5 + 0x20) = *(longlong *)(lVar5 + 0x20) + 1;
                UNLOCK();
                lVar5 = local_70;
              }
              FUN_142d97880(param_1);
              uVar10 = local_90;
              if (lVar5 != 0) {
                if (0xffffe < *(longlong *)(lVar5 + 0x20) - 1U) {
                  FUN_142e541f0(0x31e);
                }
                LOCK();
                plVar8 = (longlong *)(lVar5 + 0x20);
                lVar5 = *plVar8;
                *plVar8 = *plVar8 + -1;
                UNLOCK();
                if (((int)lVar5 == 1) && ((undefined8 *)(local_70 + 0x18) != (undefined8 *)0x0)) {
                  (*(code *)**(undefined8 **)(local_70 + 0x18))();
                }
                local_70 = 0;
                uVar10 = local_90;
              }
            }
          }
LAB_142decec6:
          local_res10 = uVar17 + 1;
          uVar12 = (ulonglong)local_res10;
          uVar10 = uVar10 + 1;
        }
      }
    }
  }
  return;
}


