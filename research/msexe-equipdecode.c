
//===========================================================
// FUN_1403095e0 @ 1403095e0   (189 bytes)
//===========================================================

longlong FUN_1403095e0(longlong param_1,undefined8 param_2,longlong param_3)

{
  longlong *plVar1;
  char cVar2;
  undefined1 local_18 [8];
  longlong *local_10;
  
  cVar2 = FUN_1406e8ae0(param_2);
  local_10 = (longlong *)0x0;
  if (cVar2 == '\x01') {
    FUN_14030ddb0(param_3,local_18);
  }
  else if (cVar2 == '\x02') {
    FUN_14030db00(param_3 + 0x970,local_18);
  }
  else {
    if (cVar2 != '\x03') goto LAB_14030965c;
    FUN_14030e340(param_3 + 0x12e0,local_18);
  }
  plVar1 = local_10;
  if (local_10 != (longlong *)0x0) {
    (**(code **)(*local_10 + 0x358))(local_10,param_2);
    *(longlong **)(param_1 + 8) = plVar1;
    return param_1;
  }
LAB_14030965c:
  *(undefined8 *)(param_1 + 8) = 0;
  return param_1;
}



//===========================================================
// FUN_1420dd920 @ 1420dd920   (21 bytes)
//===========================================================

void FUN_1420dd920(longlong param_1,undefined8 param_2)

{
  if (*(longlong *)(param_1 + 0x480) != 0) {
    FUN_140f80140(*(longlong *)(param_1 + 0x480),param_2,0);
    return;
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
// FUN_1402ee8d0 @ 1402ee8d0   (621 bytes)
//===========================================================

void FUN_1402ee8d0(longlong param_1,undefined8 param_2,longlong *param_3)

{
  undefined1 uVar1;
  byte bVar2;
  char cVar3;
  undefined4 uVar4;
  int iVar5;
  
  uVar1 = FUN_1406e8ae0(param_2);
  *(undefined1 *)(param_1 + 0x20) = uVar1;
  bVar2 = FUN_1406e8ae0(param_2);
  *(uint *)(param_1 + 0x21) = (uint)bVar2;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x25) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x29) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x1bd) = uVar4;
  *(undefined8 *)(param_1 + 0x39) = 0;
  *(undefined8 *)(param_1 + 0x41) = 0;
  *(undefined8 *)(param_1 + 0x49) = 0;
  *(undefined8 *)(param_1 + 0x51) = 0;
  *(undefined8 *)(param_1 + 0x59) = 0;
  *(undefined8 *)(param_1 + 0x61) = 0;
  *(undefined8 *)(param_1 + 0x69) = 0;
  *(undefined8 *)(param_1 + 0x71) = 0;
  *(undefined8 *)(param_1 + 0x79) = 0;
  *(undefined8 *)(param_1 + 0x81) = 0;
  *(undefined8 *)(param_1 + 0x89) = 0;
  *(undefined8 *)(param_1 + 0x91) = 0;
  *(undefined8 *)(param_1 + 0x99) = 0;
  *(undefined8 *)(param_1 + 0xa1) = 0;
  *(undefined8 *)(param_1 + 0xa9) = 0;
  *(undefined8 *)(param_1 + 0xb1) = 0;
  *(undefined8 *)(param_1 + 0xb9) = 0;
  *(undefined8 *)(param_1 + 0xc1) = 0;
  *(undefined8 *)(param_1 + 0xc9) = 0;
  *(undefined8 *)(param_1 + 0xd1) = 0;
  *(undefined8 *)(param_1 + 0xd9) = 0;
  *(undefined8 *)(param_1 + 0xe1) = 0;
  *(undefined8 *)(param_1 + 0xe9) = 0;
  *(undefined8 *)(param_1 + 0xf1) = 0;
  *(undefined8 *)(param_1 + 0xf9) = 0;
  *(undefined8 *)(param_1 + 0x101) = 0;
  *(undefined8 *)(param_1 + 0x109) = 0;
  *(undefined8 *)(param_1 + 0x111) = 0;
  *(undefined8 *)(param_1 + 0x119) = 0;
  *(undefined8 *)(param_1 + 0x121) = 0;
  *(undefined8 *)(param_1 + 0x129) = 0;
  *(undefined8 *)(param_1 + 0x131) = 0;
  FUN_1406e8ae0(param_2);
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x39) = uVar4;
  bVar2 = FUN_1406e8ae0(param_2);
  while (bVar2 != 0xff) {
    uVar4 = FUN_1406e8c20(param_2);
    if (((byte)(bVar2 - 1) < 0x1f) && (iVar5 = FUN_140253980(uVar4,bVar2,2,1), iVar5 != 0)) {
      *(undefined4 *)(param_1 + 0x39 + (ulonglong)bVar2 * 4) = uVar4;
    }
    bVar2 = FUN_1406e8ae0(param_2);
  }
  bVar2 = FUN_1406e8ae0(param_2);
  while (bVar2 != 0xff) {
    uVar4 = FUN_1406e8c20(param_2);
    if (((byte)(bVar2 - 1) < 0x1f) && (iVar5 = FUN_140253980(uVar4,bVar2,2,1), iVar5 != 0)) {
      *(undefined4 *)(param_1 + 0xb9 + (ulonglong)bVar2 * 4) = uVar4;
    }
    bVar2 = FUN_1406e8ae0(param_2);
  }
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x2d) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x31) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x35) = uVar4;
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x1c1) = uVar4;
  iVar5 = FUN_1406e8c20(param_2);
  if (iVar5 < 1) {
    iVar5 = 0;
  }
  else {
    iVar5 = iVar5 % 0x168;
  }
  *(int *)(param_1 + 0x1c5) = iVar5;
  cVar3 = FUN_1406e8ae0(param_2);
  *(bool *)(param_1 + 0x1c9) = cVar3 != '\0';
  uVar4 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x1ca) = uVar4;
  FUN_1406e9170(param_2,param_1 + 0x1b9,4);
  FUN_1406e9170(param_2,param_1 + 0x139,0x80);
  uVar4 = thunk_FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x1d2) = uVar4;
  FUN_1406e9170(param_2,param_1 + 0x1d6,0xd);
  if (*param_3 != 0) {
    FUN_14019f2c0(*param_3 + -0x10);
  }
  return;
}



//===========================================================
// FUN_140255790 @ 140255790   (18 bytes)
//===========================================================

undefined8 FUN_140255790(int param_1)

{
  if ((param_1 != 0) && (param_1 != 1)) {
    return 0;
  }
  return 1;
}



//===========================================================
// FUN_14030b6f0 @ 14030b6f0   (752 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14030b6f0(undefined8 *param_1,int param_2)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 uVar3;
  ushort uVar4;
  undefined8 uVar5;
  char *pcVar6;
  longlong lVar7;
  uint uVar8;
  uint uVar9;
  uint *puVar10;
  undefined8 *puVar11;
  int iVar12;
  undefined1 *puVar13;
  undefined4 uVar14;
  undefined1 auStack_178 [32];
  undefined1 local_158 [8];
  undefined8 *local_150;
  undefined1 local_148 [8];
  undefined8 *local_140;
  undefined1 *local_138;
  undefined1 local_128 [112];
  undefined1 local_b8 [112];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_178;
  uVar3 = *param_1;
  uVar5 = FUN_1403023d0(local_128,param_2);
  pcVar6 = (char *)FUN_1402fa9a0(uVar3,local_b8,uVar5);
  uVar8 = 0;
  iVar12 = 0;
  while (*pcVar6 == '\0') {
    uVar8 = uVar8 + 1;
    pcVar6 = pcVar6 + 1;
    if (99 < uVar8) {
      return;
    }
  }
  lVar2 = param_1[1] + (longlong)param_2 * 8;
  if (*(longlong *)(lVar2 + 0x5d0) != 0) {
    iVar12 = *(int *)(*(longlong *)(lVar2 + 0x5d0) + -8);
  }
  *(int *)param_1[2] = iVar12 + -1;
  uVar4 = FUN_1406e8b80(param_1[4]);
  *(uint *)param_1[3] = (uint)uVar4;
  iVar12 = *(int *)param_1[3];
  do {
    if (iVar12 == 0) {
      return;
    }
    uVar14 = 0;
    FUN_1403095e0(local_158,param_1[4],param_1[5]);
    puVar11 = local_150;
    puVar10 = (uint *)param_1[3];
    if ((0 < (int)*puVar10) && ((int)*puVar10 <= *(int *)param_1[2])) {
      if ((param_2 - 5U < 2) && (*(int *)param_1[6] != 0)) {
        if (local_150 == (undefined8 *)0x0) {
          FUN_142e52ed0(0x431,0);
          puVar10 = (uint *)param_1[3];
        }
        if (puVar11[7] == 0) goto LAB_14030b885;
        lVar7 = FUN_14030d840(param_1[7] + (longlong)param_2 * 8,0xffffffff);
        uVar14 = *(undefined4 *)param_1[3];
        local_140 = puVar11;
        if (0xfffff < (ulonglong)puVar11[1]) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        puVar11[1] = puVar11[1] + 1;
        UNLOCK();
        local_138 = local_148;
        FUN_1401e8780(lVar7,local_148);
        *(undefined4 *)(lVar7 + 0x10) = uVar14;
        FUN_1401abd80(local_148);
        puVar11 = local_150;
      }
      else {
LAB_14030b885:
        uVar8 = *puVar10;
        lVar7 = *(longlong *)(lVar2 + 0x5d0);
        uVar9 = 0;
        if (lVar7 != 0) {
          uVar9 = *(uint *)(lVar7 + -8);
        }
        if (((int)uVar8 < 0) || (uVar9 <= uVar8)) {
          if (lVar7 != 0) {
            uVar14 = *(undefined4 *)(lVar7 + -8);
          }
          FUN_142e54290(0xbc,uVar8,uVar14);
          lVar7 = *(longlong *)(lVar2 + 0x5d0);
        }
        puVar13 = (undefined1 *)((longlong)(int)uVar8 * 0x10 + lVar7);
        if ((*(longlong *)(puVar13 + 8) - 1U < 999) || (*(longlong *)(puVar13 + 8) == -1)) {
          FUN_142e52ed0(0x447);
        }
        if (puVar13 == local_158) {
          FUN_142e52d50(0x45c,1);
        }
        if (puVar11 != (undefined8 *)0x0) {
          if (0xfffff < (ulonglong)puVar11[1]) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          puVar11[1] = puVar11[1] + 1;
          UNLOCK();
          puVar11 = local_150;
        }
        FUN_1401abd80(puVar13);
        *(undefined8 **)(puVar13 + 8) = puVar11;
      }
      if (param_2 - 2U < 3) {
        FUN_1402e95f0(param_1[1],puVar11,0);
      }
    }
    if (puVar11 != (undefined8 *)0x0) {
      if (0xffffe < puVar11[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar1 = puVar11 + 1;
      lVar7 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if ((int)lVar7 == 1) {
        (**(code **)*local_150)(local_150,1);
      }
      local_150 = (undefined8 *)0x0;
    }
    uVar4 = FUN_1406e8b80(param_1[4]);
    *(uint *)param_1[3] = (uint)uVar4;
    iVar12 = *(int *)param_1[3];
  } while( true );
}



//===========================================================
// FUN_14030b9e0 @ 14030b9e0   (1196 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_14030b9e0(undefined8 *param_1,int param_2)

{
  longlong *plVar1;
  undefined8 uVar2;
  longlong lVar3;
  longlong *plVar4;
  bool bVar5;
  ushort uVar6;
  uint uVar7;
  int iVar8;
  uint uVar9;
  undefined8 uVar10;
  char *pcVar11;
  uint uVar12;
  longlong lVar13;
  longlong lVar14;
  undefined8 *puVar15;
  ulonglong uVar16;
  longlong lVar17;
  undefined1 *puVar18;
  undefined4 uVar19;
  longlong lVar20;
  undefined1 auStack_1a8 [32];
  int local_188;
  undefined1 local_180 [8];
  undefined8 *local_178;
  uint local_170;
  int local_16c;
  ulonglong local_168;
  undefined1 local_160 [8];
  undefined8 *local_158;
  longlong local_150;
  longlong local_148;
  longlong local_140;
  undefined1 *local_138;
  undefined1 local_128 [112];
  undefined1 local_b8 [112];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_1a8;
  uVar2 = *param_1;
  local_16c = param_2;
  uVar10 = FUN_1403023d0(local_128);
  pcVar11 = (char *)FUN_1402fa9a0(uVar2,local_b8,uVar10);
  uVar12 = 0;
  while (*pcVar11 == '\0') {
    uVar12 = uVar12 + 1;
    pcVar11 = pcVar11 + 1;
    if (99 < uVar12) {
      return;
    }
  }
  bVar5 = param_2 - 5U < 2;
  uVar12 = (uint)bVar5;
  local_170 = (uint)bVar5;
  local_188 = 0;
  local_168 = 0;
  do {
    uVar16 = local_168;
    uVar7 = FUN_140255790(local_188);
    if (uVar12 == uVar7) {
      plVar1 = (longlong *)(param_1[1] + 0x5a8 + uVar16 * 8);
      uVar7 = 0;
      iVar8 = FUN_140232a80(plVar1);
      if (iVar8 != 0) {
        lVar20 = 0;
        do {
          lVar14 = *plVar1;
          if (lVar14 == 0) {
            uVar12 = 0;
          }
          else {
            uVar12 = *(uint *)(lVar14 + -8);
          }
          if (((int)uVar7 < 0) || (uVar12 <= uVar7)) {
            if (lVar14 == 0) {
              uVar19 = 0;
            }
            else {
              uVar19 = *(undefined4 *)(lVar14 + -8);
            }
            FUN_142e54290(0xbc,uVar7,uVar19);
            lVar14 = *plVar1;
          }
          lVar3 = param_1[2];
          plVar4 = *(longlong **)(lVar20 + 8 + lVar14);
          if (plVar4 != (longlong *)0x0) {
            iVar8 = (**(code **)(*plVar4 + 0x88))();
            if (iVar8 == 1) {
              lVar17 = *(longlong *)(lVar20 + 8 + lVar14);
              if (lVar17 != 0) {
                lVar13 = *(longlong *)(lVar17 + 8);
                if (lVar13 == 1) {
                  *(undefined8 *)(lVar20 + 8 + lVar14) = 0;
                  local_140 = lVar17;
                  FUN_14030c520(lVar3,&local_140);
                }
                else {
LAB_14030bbba:
                  if (0xffffe < lVar13 - 1U) {
                    FUN_142e541f0(0x31e);
                  }
                  LOCK();
                  plVar4 = (longlong *)(lVar17 + 8);
                  lVar3 = *plVar4;
                  *plVar4 = *plVar4 + -1;
                  UNLOCK();
                  if (((int)lVar3 == 1) &&
                     (puVar15 = *(undefined8 **)(lVar20 + 8 + lVar14), puVar15 != (undefined8 *)0x0)
                     ) {
                    (**(code **)*puVar15)(puVar15,1);
                  }
                  *(undefined8 *)(lVar20 + 8 + lVar14) = 0;
                }
              }
            }
            else if (iVar8 == 2) {
              lVar17 = *(longlong *)(lVar20 + 8 + lVar14);
              if (lVar17 != 0) {
                lVar13 = *(longlong *)(lVar17 + 8);
                if (lVar13 != 1) goto LAB_14030bbba;
                *(undefined8 *)(lVar20 + 8 + lVar14) = 0;
                local_148 = lVar17;
                FUN_14030c360(lVar3 + 0x970,&local_148);
              }
            }
            else if (iVar8 == 3) {
              lVar17 = *(longlong *)(lVar20 + 8 + lVar14);
              if (lVar17 != 0) {
                lVar13 = *(longlong *)(lVar17 + 8);
                if (lVar13 != 1) goto LAB_14030bbba;
                *(undefined8 *)(lVar20 + 8 + lVar14) = 0;
                local_150 = lVar17;
                FUN_14030c6e0(lVar3 + 0x12e0,&local_150);
              }
            }
          }
          uVar7 = uVar7 + 1;
          lVar20 = lVar20 + 0x10;
          uVar9 = FUN_140232a80(plVar1);
          uVar16 = local_168;
          uVar12 = local_170;
        } while (uVar7 < uVar9);
      }
      iVar8 = (&DAT_14327dd50)[uVar16];
      uVar6 = FUN_1406e8b80(param_1[3]);
      while (uVar6 != 0) {
        uVar7 = (uint)uVar6;
        FUN_1403095e0(local_180,param_1[3]);
        puVar15 = local_178;
        if (((uVar16 < 5) && ((int)(&DAT_14327dd50)[uVar16] <= (int)uVar7)) &&
           ((int)uVar7 < (int)(&DAT_14327dd68)[uVar16])) {
          if ((uVar12 != 0) && (*(int *)param_1[4] != 0)) {
            if (local_178 == (undefined8 *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            if (puVar15[7] != 0) {
              lVar20 = FUN_14030d840(param_1[5] + (longlong)local_16c * 8,0xffffffff);
              local_158 = puVar15;
              if (0xfffff < (ulonglong)puVar15[1]) {
                FUN_142e541f0(0x30f);
              }
              LOCK();
              puVar15[1] = puVar15[1] + 1;
              UNLOCK();
              local_138 = local_160;
              FUN_1401e8780(lVar20,local_160);
              *(uint *)(lVar20 + 0x10) = -uVar7;
              FUN_1401abd80(local_160);
              puVar15 = local_178;
              uVar16 = local_168;
              goto LAB_14030bddd;
            }
          }
          uVar7 = uVar7 - iVar8;
          lVar20 = *plVar1;
          if (lVar20 == 0) {
            uVar9 = 0;
          }
          else {
            uVar9 = *(uint *)(lVar20 + -8);
          }
          if (((int)uVar7 < 0) || (uVar9 <= uVar7)) {
            if (lVar20 == 0) {
              uVar19 = 0;
            }
            else {
              uVar19 = *(undefined4 *)(lVar20 + -8);
            }
            FUN_142e54290(0xbc,uVar7,uVar19);
            lVar20 = *plVar1;
          }
          puVar18 = (undefined1 *)((longlong)(int)uVar7 * 0x10 + lVar20);
          if ((*(longlong *)(puVar18 + 8) - 1U < 999) || (*(longlong *)(puVar18 + 8) == -1)) {
            FUN_142e52ed0(0x447);
          }
          if (puVar18 == local_180) {
            FUN_142e52d50(0x45c,1);
          }
          if (puVar15 != (undefined8 *)0x0) {
            if (0xfffff < (ulonglong)puVar15[1]) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            puVar15[1] = puVar15[1] + 1;
            UNLOCK();
            puVar15 = local_178;
          }
          FUN_1401abd80(puVar18);
          *(undefined8 **)(puVar18 + 8) = puVar15;
        }
LAB_14030bddd:
        if (puVar15 != (undefined8 *)0x0) {
          if (0xffffe < puVar15[1] - 1) {
            FUN_142e541f0(0x31e);
          }
          LOCK();
          plVar4 = puVar15 + 1;
          lVar20 = *plVar4;
          *plVar4 = *plVar4 + -1;
          UNLOCK();
          if ((int)lVar20 == 1) {
            (**(code **)*local_178)(local_178,1);
          }
          local_178 = (undefined8 *)0x0;
        }
        uVar6 = FUN_1406e8b80(param_1[3]);
      }
    }
    local_188 = local_188 + 1;
    local_168 = uVar16 + 1;
    if (4 < local_188) {
      return;
    }
  } while( true );
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
// FUN_1420dd220 @ 1420dd220   (135 bytes)
//===========================================================

void FUN_1420dd220(longlong param_1,int param_2)

{
  longlong lVar1;
  
  lVar1 = *(longlong *)(param_1 + 0x480);
  if (lVar1 == 0) {
    return;
  }
  if (param_2 != 0) {
    FUN_140f813b0(lVar1,((*(int *)(param_1 + 0x4c4) * 0xaa) / 0xff) * 0x1000000 | 0xffffe1);
    lVar1 = *(longlong *)(param_1 + 0x480);
    if (lVar1 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar1 = *(longlong *)(param_1 + 0x480);
    }
    FUN_140fac110(lVar1,0x50);
    return;
  }
  FUN_140f8abf0(lVar1,0);
  return;
}



//===========================================================
// FUN_1403094b0 @ 1403094b0   (285 bytes)
//===========================================================

void FUN_1403094b0(longlong param_1,undefined8 param_2)

{
  undefined4 uVar1;
  undefined8 uVar2;
  int *piVar3;
  int *local_res8;
  
  FUN_140302e30(param_1,param_2,0);
  uVar1 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x319) = uVar1;
  uVar2 = FUN_1406e8f10(param_2);
  *(undefined8 *)(param_1 + 0x31d) = uVar2;
  uVar1 = FUN_1406e8c20(param_2);
  *(undefined4 *)(param_1 + 0x325) = uVar1;
  uVar2 = FUN_1406e8f10(param_2);
  *(undefined8 *)(param_1 + 0x329) = uVar2;
  local_res8 = (int *)0x0;
  piVar3 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
  piVar3[1] = 0;
  local_res8 = piVar3 + 4;
  *piVar3 = -1;
  piVar3[2] = 0;
  *(undefined1 *)local_res8 = 0;
  if (*piVar3 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar3[1] < 0) {
    FUN_142e54290(0x90,piVar3[1],0);
  }
  *piVar3 = 1;
  *(undefined1 *)local_res8 = 0;
  if (piVar3[1] + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  piVar3[2] = 0;
  FUN_1402ee8d0(param_1 + 0x136,param_2,&local_res8,0);
  *(undefined4 *)(param_1 + 0x308) = *(undefined4 *)(param_1 + 0x325);
  return;
}



//===========================================================
// FUN_14030b260 @ 14030b260   (764 bytes)
//===========================================================

undefined8 FUN_14030b260(int *param_1,int *param_2)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  int *piVar4;
  int *piVar5;
  undefined8 uVar6;
  int iVar7;
  int iVar8;
  int *piVar9;
  int *piVar10;
  uint uStackX_c;
  int *local_res10;
  
  piVar10 = (int *)0x0;
  iVar1 = 0;
  if ((((*param_1 == *param_2) && (param_1[3] == param_2[3])) && (param_1[2] == param_2[2])) &&
     (*(int *)((longlong)param_1 + 0x1d) == *(int *)((longlong)param_2 + 0x1d))) {
    piVar4 = param_1 + 4;
    piVar5 = (int *)0xffffffffffffffff;
    local_res10 = piVar10;
    piVar9 = piVar5;
    if (piVar4 != (int *)0x0) {
      do {
        piVar9 = (int *)((longlong)piVar9 + 1);
      } while (*(char *)((longlong)piVar4 + (longlong)piVar9) != '\0');
      iVar7 = (int)piVar9;
      iVar8 = iVar1;
      if (0 < iVar7) {
        iVar8 = iVar7;
      }
      piVar2 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar8 + 0x11));
      piVar2[1] = iVar8;
      *piVar2 = -1;
      local_res10 = piVar2 + 4;
      piVar2[2] = 0;
      *(undefined1 *)local_res10 = 0;
      FUN_142ef7ba0(local_res10,piVar4,(longlong)iVar7);
      if (*piVar2 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar7 == -1) || (iVar7 <= piVar2[1])) {
        *piVar2 = 1;
        if (iVar7 != -1) goto LAB_14030b351;
        piVar4 = piVar5;
        piVar9 = piVar10;
        if (local_res10 != (int *)0x0) {
          do {
            piVar9 = (int *)((longlong)piVar4 + 1);
            piVar4 = piVar9;
          } while (*(char *)((longlong)local_res10 + (longlong)piVar9) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar2[1],(ulonglong)piVar9 & 0xffffffff);
        *piVar2 = 1;
LAB_14030b351:
        *(undefined1 *)((longlong)local_res10 + (longlong)iVar7) = 0;
      }
      iVar8 = (int)piVar9;
      if ((iVar8 < 0) || (piVar2[1] + 1 <= iVar8)) {
        FUN_142e54290(0x9c,(ulonglong)piVar9 & 0xffffffff);
      }
      piVar2[2] = iVar8;
    }
    piVar9 = param_2 + 4;
    piVar4 = piVar10;
    piVar2 = piVar5;
    if (piVar9 != (int *)0x0) {
      do {
        piVar2 = (int *)((longlong)piVar2 + 1);
      } while (*(char *)((longlong)piVar9 + (longlong)piVar2) != '\0');
      iVar7 = (int)piVar2;
      iVar8 = 0;
      if (0 < iVar7) {
        iVar8 = iVar7;
      }
      piVar3 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar8 + 0x11));
      piVar3[1] = iVar8;
      *piVar3 = -1;
      piVar4 = piVar3 + 4;
      piVar3[2] = 0;
      *(undefined1 *)piVar4 = 0;
      FUN_142ef7ba0(piVar4,piVar9,(longlong)iVar7);
      if (*piVar3 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar7 == -1) || (iVar7 <= piVar3[1])) {
        *piVar3 = 1;
        if (iVar7 != -1) goto LAB_14030b413;
        if (piVar4 != (int *)0x0) {
          do {
            piVar5 = (int *)((longlong)piVar5 + 1);
          } while (*(char *)((longlong)piVar4 + (longlong)piVar5) != '\0');
          piVar10 = (int *)((ulonglong)piVar5 & 0xffffffff);
        }
      }
      else {
        FUN_142e54290(0x90,piVar3[1],(ulonglong)piVar2 & 0xffffffff);
        *piVar3 = 1;
LAB_14030b413:
        *(undefined1 *)((longlong)piVar4 + (longlong)iVar7) = 0;
        piVar10 = piVar2;
      }
      iVar8 = (int)piVar10;
      if ((iVar8 < 0) || (piVar3[1] + 1 <= iVar8)) {
        FUN_142e54290(0x9c,(ulonglong)piVar10 & 0xffffffff);
      }
      piVar3[2] = iVar8;
    }
    piVar10 = (int *)0x3;
    if (local_res10 != piVar4) {
      iVar8 = iVar1;
      if (local_res10 != (int *)0x0) {
        iVar8 = local_res10[-2];
      }
      iVar7 = iVar1;
      if (piVar4 != (int *)0x0) {
        iVar7 = piVar4[-2];
      }
      if (iVar8 != iVar7) goto LAB_14030b50f;
      if (iVar8 != 0) {
        if (local_res10 != (int *)0x0) {
          iVar1 = local_res10[-2];
        }
        iVar1 = memcmp(local_res10,piVar4,(longlong)iVar1);
        if (iVar1 != 0) goto LAB_14030b50f;
      }
    }
    iVar1 = (*DAT_1432627e0)((longlong)param_1 + 0x21,(longlong)param_2 + 0x21);
    if ((iVar1 == 0) && (param_1[1] == param_2[1])) {
      uVar6 = 0;
      goto LAB_14030b512;
    }
  }
  else {
    piVar4 = (int *)((ulonglong)uStackX_c << 0x20);
  }
LAB_14030b50f:
  uVar6 = 1;
LAB_14030b512:
  if ((((ulonglong)piVar10 & 2) != 0) &&
     (piVar10 = (int *)(ulonglong)((uint)piVar10 & 0xfffffffd), piVar4 != (int *)0x0)) {
    FUN_14019f2c0(piVar4 + -4);
  }
  if ((((ulonglong)piVar10 & 1) != 0) && (local_res10 != (int *)0x0)) {
    FUN_14019f2c0(local_res10 + -4);
  }
  return uVar6;
}


