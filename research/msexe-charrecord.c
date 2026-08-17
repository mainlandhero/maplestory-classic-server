
//===========================================================
// FUN_14108bdf0 @ 14108bdf0   (1046 bytes)
//===========================================================

void FUN_14108bdf0(undefined8 param_1)

{
  int *piVar1;
  longlong *plVar2;
  int iVar3;
  char cVar4;
  uint uVar5;
  longlong *plVar6;
  undefined4 *puVar7;
  undefined4 *puVar8;
  undefined8 *puVar9;
  undefined4 *puVar10;
  byte bVar11;
  int iVar12;
  undefined4 uVar13;
  longlong *plVar14;
  undefined8 *puVar15;
  undefined8 *puVar16;
  longlong lVar17;
  ulonglong uVar18;
  undefined8 *puVar19;
  ulonglong uVar20;
  uint *puVar21;
  undefined1 local_res10 [16];
  longlong *local_res20;
  uint *local_98;
  longlong *local_90;
  undefined8 *local_88;
  undefined8 local_80;
  undefined4 local_78;
  undefined4 uStack_74;
  uint uStack_70;
  undefined4 uStack_6c;
  undefined4 *local_68;
  undefined4 *puStack_60;
  longlong local_58;
  undefined8 local_50;
  uint uStack_48;
  undefined4 uStack_44;
  
  local_68 = (undefined4 *)0x0;
  puStack_60 = (undefined4 *)0x0;
  local_58 = 0;
  iVar12 = FUN_1406e8c20();
  uVar20 = (ulonglong)iVar12;
  uVar18 = (longlong)puStack_60 - (longlong)local_68 >> 2;
  puVar15 = DAT_143ac9890;
  puVar7 = local_68;
  if (uVar20 < uVar18) {
    puVar8 = local_68 + uVar20;
    puStack_60 = local_68 + uVar20;
  }
  else {
    puVar8 = puStack_60;
    if (uVar18 < uVar20) {
      if ((ulonglong)(local_58 - (longlong)local_68 >> 2) < uVar20) {
        FUN_140234110(&local_68,uVar20,local_res10);
        puVar15 = DAT_143ac9890;
        puVar7 = local_68;
        puVar8 = puStack_60;
      }
      else {
        puVar8 = puStack_60 + (uVar20 - uVar18);
        FUN_142ef8250(puStack_60,0);
        puVar15 = DAT_143ac9890;
        puVar7 = local_68;
        puStack_60 = puVar8;
      }
    }
  }
  for (; puVar10 = puStack_60, DAT_143ac9890 = puVar15, puVar7 != puStack_60; puVar7 = puVar7 + 1) {
    puStack_60 = puVar8;
    uVar13 = FUN_1406e8c20(param_1);
    *puVar7 = uVar13;
    puVar15 = DAT_143ac9890;
    puVar8 = puStack_60;
    puStack_60 = puVar10;
  }
  puStack_60 = puVar8;
  FUN_14108e740(&DAT_143ac9890,&DAT_143ac9890,puVar15[1]);
  puVar15[1] = puVar15;
  *puVar15 = puVar15;
  puVar15[2] = puVar15;
  DAT_143ac9898 = 0;
  FUN_14108e130(DAT_143ac9850,DAT_143ac9858,&DAT_143ac9850);
  DAT_143ac9858 = DAT_143ac9850;
  bVar11 = FUN_1406e8ae0(param_1);
  iVar12 = 0;
  if (bVar11 != 0) {
    do {
      plVar14 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x348);
      local_res20 = plVar14;
      if (plVar14 == (longlong *)0x0) {
        plVar14 = (longlong *)0x0;
      }
      else {
        *plVar14 = 0;
        plVar14[1] = 0;
        *(undefined4 *)(plVar14 + 1) = 1;
        *(undefined4 *)((longlong)plVar14 + 0xc) = 1;
        *plVar14 = (longlong)&PTR_FUN_143379de8;
        FUN_14108e7f0(plVar14 + 2);
      }
      puVar21 = (uint *)(plVar14 + 2);
      local_98 = puVar21;
      local_90 = plVar14;
      FUN_1403094b0(puVar21);
      puVar9 = DAT_143ac9890;
      uVar5 = *puVar21;
      cVar4 = *(char *)((longlong)DAT_143ac9890[1] + 0x19);
      puVar15 = (undefined8 *)DAT_143ac9890[1];
      puVar16 = DAT_143ac9890;
      while (puVar19 = puVar15, cVar4 == '\0') {
        if (*(uint *)(puVar19 + 4) < uVar5) {
          puVar15 = (undefined8 *)puVar19[2];
          puVar19 = puVar16;
        }
        else {
          puVar15 = (undefined8 *)*puVar19;
        }
        cVar4 = *(char *)((longlong)puVar15 + 0x19);
        puVar16 = puVar19;
      }
      if (((*(char *)((longlong)puVar16 + 0x19) == '\0') && (*(uint *)(puVar16 + 4) <= uVar5)) &&
         (puVar16 != DAT_143ac9890)) {
        if (plVar14 != (longlong *)0x0) {
          LOCK();
          plVar6 = plVar14 + 1;
          lVar17 = *plVar6;
          *(int *)plVar6 = (int)*plVar6 + -1;
          UNLOCK();
          if ((int)lVar17 == 1) {
            (**(code **)*plVar14)(plVar14);
            LOCK();
            piVar1 = (int *)((longlong)plVar14 + 0xc);
            iVar3 = *piVar1;
            *piVar1 = *piVar1 + -1;
            UNLOCK();
            if (iVar3 == 1) {
              lVar17 = *plVar14;
LAB_14108c187:
              (**(code **)(lVar17 + 8))(plVar14);
            }
          }
        }
      }
      else {
        puVar15 = (undefined8 *)DAT_143ac9890[1];
        uStack_48 = 0;
        cVar4 = *(char *)((longlong)puVar15 + 0x19);
        local_50 = puVar15;
        puVar16 = DAT_143ac9890;
        while (puVar19 = puVar15, cVar4 == '\0') {
          if (uVar5 <= *(uint *)(puVar19 + 4)) {
            puVar15 = (undefined8 *)*puVar19;
            puVar16 = puVar19;
          }
          else {
            puVar15 = (undefined8 *)puVar19[2];
          }
          uStack_48 = (uint)(uVar5 <= *(uint *)(puVar19 + 4));
          cVar4 = *(char *)((longlong)puVar15 + 0x19);
          local_50 = puVar19;
        }
        if ((*(char *)((longlong)puVar16 + 0x19) != '\0') || (uVar5 < *(uint *)(puVar16 + 4))) {
          if (DAT_143ac9898 == 0x492492492492492) {
                    /* WARNING: Subroutine does not return */
            FUN_14019f9d0();
          }
          local_88 = &DAT_143ac9890;
          local_80 = 0;
          puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x38);
          *(uint *)(puVar15 + 4) = uVar5;
          puVar15[5] = 0;
          puVar15[6] = 0;
          *puVar15 = puVar9;
          puVar15[1] = puVar9;
          puVar15[2] = puVar9;
          *(undefined2 *)(puVar15 + 3) = 0;
          local_80 = 0;
          local_78 = (undefined4)local_50;
          uStack_74 = local_50._4_4_;
          uStack_70 = uStack_48;
          uStack_6c = uStack_44;
          puVar16 = (undefined8 *)FUN_1410902b0(&DAT_143ac9890,&local_78,puVar15);
        }
        if (plVar14 != (longlong *)0x0) {
          LOCK();
          *(int *)(plVar14 + 1) = (int)plVar14[1] + 1;
          UNLOCK();
          puVar21 = local_98;
        }
        puVar16[5] = puVar21;
        plVar6 = (longlong *)puVar16[6];
        puVar16[6] = plVar14;
        if (plVar6 != (longlong *)0x0) {
          LOCK();
          plVar2 = plVar6 + 1;
          lVar17 = *plVar2;
          *(int *)plVar2 = (int)*plVar2 + -1;
          UNLOCK();
          puVar21 = local_98;
          if ((int)lVar17 == 1) {
            (**(code **)*plVar6)(plVar6);
            LOCK();
            piVar1 = (int *)((longlong)plVar6 + 0xc);
            iVar3 = *piVar1;
            *piVar1 = *piVar1 + -1;
            UNLOCK();
            puVar21 = local_98;
            if (iVar3 == 1) {
              (**(code **)(*plVar6 + 8))(plVar6);
              puVar21 = local_98;
            }
          }
        }
        puVar15 = DAT_143ac9858;
        if (DAT_143ac9858 == DAT_143ac9860) {
          FUN_14108e1b0(&DAT_143ac9850,DAT_143ac9858,&local_98);
        }
        else {
          *DAT_143ac9858 = 0;
          puVar15[1] = 0;
          if (plVar14 != (longlong *)0x0) {
            LOCK();
            *(int *)(plVar14 + 1) = (int)plVar14[1] + 1;
            UNLOCK();
            puVar21 = local_98;
          }
          *puVar15 = puVar21;
          puVar15[1] = plVar14;
          DAT_143ac9858 = DAT_143ac9858 + 2;
        }
        plVar14 = local_90;
        if (local_90 != (longlong *)0x0) {
          LOCK();
          plVar6 = local_90 + 1;
          lVar17 = *plVar6;
          *(int *)plVar6 = (int)*plVar6 + -1;
          UNLOCK();
          if ((int)lVar17 == 1) {
            (**(code **)*local_90)(local_90);
            LOCK();
            piVar1 = (int *)((longlong)plVar14 + 0xc);
            iVar3 = *piVar1;
            *piVar1 = *piVar1 + -1;
            UNLOCK();
            if (iVar3 == 1) {
              lVar17 = *local_90;
              plVar14 = local_90;
              goto LAB_14108c187;
            }
          }
        }
      }
      iVar12 = iVar12 + 1;
    } while (iVar12 < (int)(uint)bVar11);
  }
  FUN_14108d510(&local_68);
  if (local_68 != (undefined4 *)0x0) {
    uVar18 = local_58 - (longlong)local_68 & 0xfffffffffffffffc;
    if (0xfff < uVar18) {
      if (0x1f < (ulonglong)((longlong)local_68 + (-8 - *(longlong *)(local_68 + -2)))) {
                    /* WARNING: Subroutine does not return */
        FUN_142f04804(*(longlong *)(local_68 + -2),uVar18 + 0x27);
      }
    }
    thunk_FUN_140205820();
  }
  return;
}



//===========================================================
// FUN_14108d290 @ 14108d290   (443 bytes)
//===========================================================

void FUN_14108d290(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  char cVar1;
  longlong *plVar2;
  longlong *plVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  int iVar6;
  uint uVar7;
  undefined8 uVar8;
  undefined8 *puVar9;
  int iVar10;
  bool bVar11;
  undefined8 local_res18;
  undefined8 local_res20;
  undefined8 *puVar12;
  undefined8 *local_58;
  uint uStack_50;
  
  puVar9 = DAT_143ac9868;
  cVar1 = *(char *)((longlong)DAT_143ac9868[1] + 0x19);
  plVar3 = (longlong *)DAT_143ac9868[1];
  while (cVar1 == '\0') {
    FUN_14108e6e0(&DAT_143ac9868,&DAT_143ac9868,plVar3[2]);
    plVar2 = (longlong *)*plVar3;
    thunk_FUN_140205820(plVar3,0x28);
    plVar3 = plVar2;
    cVar1 = *(char *)((longlong)plVar2 + 0x19);
  }
  puVar9[1] = puVar9;
  *puVar9 = puVar9;
  puVar9[2] = puVar9;
  iVar10 = 0;
  DAT_143ac9870 = 0;
  iVar6 = FUN_1406e8c20(param_1);
  if (0 < iVar6) {
    do {
      uVar7 = FUN_1406e8c20(param_1);
      local_res18 = DAT_143379de0;
      FUN_1406e9170(param_1,&local_res18,8);
      local_res20 = local_res18;
      uVar8 = FUN_1408f63b0(&local_res20,1);
      puVar5 = DAT_143ac9868;
      puVar9 = (undefined8 *)DAT_143ac9868[1];
      uStack_50 = 0;
      cVar1 = *(char *)((longlong)puVar9 + 0x19);
      local_58 = puVar9;
      puVar12 = DAT_143ac9868;
      while (puVar4 = puVar9, cVar1 == '\0') {
        bVar11 = uVar7 <= *(uint *)((longlong)puVar4 + 0x1c);
        if (bVar11) {
          puVar9 = (undefined8 *)*puVar4;
          puVar12 = puVar4;
        }
        else {
          puVar9 = (undefined8 *)puVar4[2];
        }
        uStack_50 = (uint)bVar11;
        cVar1 = *(char *)((longlong)puVar9 + 0x19);
        local_58 = puVar4;
      }
      if ((*(char *)((longlong)puVar12 + 0x19) != '\0') ||
         (uVar7 < *(uint *)((longlong)puVar12 + 0x1c))) {
        if (DAT_143ac9870 == 0x666666666666666) {
                    /* WARNING: Subroutine does not return */
          FUN_14019f9d0();
        }
        puVar12 = &DAT_143ac9868;
        puVar9 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x28);
        *(uint *)((longlong)puVar9 + 0x1c) = uVar7;
        puVar9[4] = 0;
        *puVar9 = puVar5;
        puVar9[1] = puVar5;
        puVar9[2] = puVar5;
        *(undefined2 *)(puVar9 + 3) = 0;
        puVar12 = (undefined8 *)FUN_141090030(&DAT_143ac9868,&local_58,puVar9,param_4,puVar12,0);
      }
      puVar12[4] = uVar8;
      iVar10 = iVar10 + 1;
    } while (iVar10 < iVar6);
  }
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


