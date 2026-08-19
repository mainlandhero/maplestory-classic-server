
//===========================================================
// FUN_14030b560 @ 14030b560   (386 bytes)
//===========================================================

void FUN_14030b560(undefined8 *param_1,longlong param_2)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  char cVar3;
  int iVar4;
  undefined8 *puVar5;
  longlong *plVar6;
  longlong *plVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  undefined8 *puVar10;
  longlong local_res10;
  longlong local_res18;
  undefined1 local_res20 [8];
  undefined1 local_28 [16];
  
  if (*(longlong *)(param_2 + 8) != 0) {
    iVar4 = FUN_14019a5d0(*(longlong *)(param_2 + 8) + 0x20);
    if (iVar4 - 0x195460U < 10000) {
      plVar7 = *(longlong **)(param_2 + 8);
      if (plVar7 == (longlong *)0x0) {
        FUN_142e52ed0(0x431,0);
        plVar7 = *(longlong **)(param_2 + 8);
      }
      puVar5 = (undefined8 *)(**(code **)(*plVar7 + 0x330))();
      if (puVar5 != (undefined8 *)0x0) {
        plVar7 = (longlong *)*param_1;
        plVar6 = *(longlong **)(param_2 + 8);
        if (plVar6 == (longlong *)0x0) {
          FUN_142e52ed0(0x431,0);
          plVar6 = *(longlong **)(param_2 + 8);
        }
        plVar6 = (longlong *)(**(code **)(*plVar6 + 0x80))(plVar6,local_res20);
        puVar1 = (undefined8 *)*plVar7;
        puVar10 = puVar1;
        if (*(char *)((longlong)puVar1[1] + 0x19) == '\0') {
          puVar8 = (undefined8 *)puVar1[1];
          do {
            if ((longlong)puVar8[4] < *plVar6) {
              puVar9 = (undefined8 *)puVar8[2];
            }
            else {
              puVar9 = (undefined8 *)*puVar8;
              puVar10 = puVar8;
            }
            puVar8 = puVar9;
          } while (*(char *)((longlong)puVar9 + 0x19) == '\0');
        }
        if ((*(char *)((longlong)puVar10 + 0x19) != '\0') || (*plVar6 < (longlong)puVar10[4])) {
          puVar10 = puVar1;
        }
        if (puVar10 != *(undefined8 **)*param_1) {
          puVar1 = puVar10 + 5;
          cVar3 = FUN_14030b260(puVar5,puVar1);
          if (cVar3 != '\0') {
            plVar7 = (longlong *)FUN_1401a19e0(param_2);
            (**(code **)(*plVar7 + 0x80))(plVar7,local_28);
            FUN_14030cbc0(puVar1,&local_res18);
            FUN_14030cbc0(puVar5,&local_res10);
            if (local_res10 != 0) {
              FUN_14019f2c0(local_res10 + -0x10);
            }
            if (local_res18 != 0) {
              FUN_14019f2c0(local_res18 + -0x10);
            }
            uVar2 = puVar10[6];
            *puVar5 = *puVar1;
            puVar5[1] = uVar2;
            uVar2 = puVar10[8];
            puVar5[2] = puVar10[7];
            puVar5[3] = uVar2;
            puVar5[4] = puVar10[9];
            *(undefined1 *)(puVar5 + 5) = *(undefined1 *)(puVar10 + 10);
          }
        }
      }
    }
  }
  return;
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
// FUN_14030ca50 @ 14030ca50   (354 bytes)
//===========================================================

void FUN_14030ca50(longlong param_1,longlong param_2)

{
  longlong *plVar1;
  undefined8 *puVar2;
  int iVar3;
  longlong lVar4;
  longlong local_res10;
  
  if (*(longlong **)(param_2 + 8) == (longlong *)0x0) {
    return;
  }
  iVar3 = (**(code **)(**(longlong **)(param_2 + 8) + 0x88))();
  if (iVar3 == 1) {
    local_res10 = *(longlong *)(param_2 + 8);
    if (local_res10 == 0) {
      return;
    }
    lVar4 = *(longlong *)(local_res10 + 8);
    if (lVar4 == 1) {
      *(undefined8 *)(param_2 + 8) = 0;
      FUN_14030c520(param_1,&local_res10);
      return;
    }
  }
  else if (iVar3 == 2) {
    local_res10 = *(longlong *)(param_2 + 8);
    if (local_res10 == 0) {
      return;
    }
    lVar4 = *(longlong *)(local_res10 + 8);
    if (lVar4 == 1) {
      *(undefined8 *)(param_2 + 8) = 0;
      FUN_14030c360(param_1 + 0x970,&local_res10);
      return;
    }
  }
  else {
    if (iVar3 != 3) {
      return;
    }
    local_res10 = *(longlong *)(param_2 + 8);
    if (local_res10 == 0) {
      return;
    }
    lVar4 = *(longlong *)(local_res10 + 8);
    if (lVar4 == 1) {
      *(undefined8 *)(param_2 + 8) = 0;
      FUN_14030c6e0(param_1 + 0x12e0,&local_res10);
      return;
    }
  }
  if (0xffffe < lVar4 - 1U) {
    FUN_142e541f0(0x31e);
  }
  LOCK();
  plVar1 = (longlong *)(local_res10 + 8);
  lVar4 = *plVar1;
  *plVar1 = *plVar1 + -1;
  UNLOCK();
  if (((int)lVar4 == 1) && (puVar2 = *(undefined8 **)(param_2 + 8), puVar2 != (undefined8 *)0x0)) {
    (**(code **)*puVar2)(puVar2,1);
  }
  *(undefined8 *)(param_2 + 8) = 0;
  return;
}



//===========================================================
// FUN_1401abd80 @ 1401abd80   (106 bytes)
//===========================================================

void FUN_1401abd80(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 *puVar3;
  
  lVar2 = *(longlong *)(param_1 + 8);
  if (lVar2 != 0) {
    if (0xffffe < *(longlong *)(lVar2 + 8) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + 8);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (((int)lVar2 == 1) && (puVar3 = *(undefined8 **)(param_1 + 8), puVar3 != (undefined8 *)0x0))
    {
      (**(code **)*puVar3)(puVar3,1);
    }
    *(undefined8 *)(param_1 + 8) = 0;
  }
  return;
}


