
//===========================================================
// FUN_141b2fac0 @ 141b2fac0   (1897 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000141b2ff62) */

void FUN_141b2fac0(longlong param_1,undefined8 param_2)

{
  int *piVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  wchar_t *pwVar4;
  char cVar5;
  char cVar6;
  byte bVar7;
  ushort uVar8;
  ushort uVar9;
  int iVar10;
  undefined4 uVar11;
  undefined8 *puVar12;
  int *piVar13;
  int **ppiVar14;
  longlong *plVar15;
  int *piVar16;
  longlong *plVar17;
  uint *puVar18;
  longlong lVar19;
  int *piVar20;
  int *piVar21;
  uint uVar22;
  int iVar23;
  int *piVar24;
  wchar_t *local_res18;
  int *local_res20;
  int *local_68;
  int *local_60;
  longlong local_58;
  longlong local_50;
  int **local_48 [2];
  
  FUN_142aa2810(1);
  iVar10 = FUN_142c4a810(DAT_143ac1898);
  if (iVar10 == 5) {
    FUN_141b31ff0(param_1,param_2);
    return;
  }
  local_48[0] = DAT_143aa84a0;
  cVar5 = FUN_142cf42c0(DAT_143aa84a0);
  cVar6 = FUN_1406e8ae0(param_2);
  local_res18 = (wchar_t *)CONCAT44(local_res18._4_4_,(int)cVar6);
  if (cVar6 < '\0') {
    FUN_141b3f050(param_1,2,400);
    cVar6 = FUN_1406e8ae0(param_2);
    *(bool *)(param_1 + 0x1a8) = cVar6 != '\0';
    cVar6 = FUN_1406e8ae0(param_2);
    if (cVar6 != '\0') {
      local_res18 = (wchar_t *)0x0;
      puVar12 = (undefined8 *)FUN_1408a9e40(&local_res20,3);
      FUN_14019ba10(&local_res18,*puVar12);
      if (local_res20 != (int *)0x0) {
        FUN_14019f2c0(local_res20 + -4);
      }
      pwVar4 = local_res18;
      FUN_1429e4fa0(local_res18,0,0);
      local_res20 = (int *)FUN_1418039d0(0x2200000b);
      puVar12 = (undefined8 *)FUN_140cc21e0(&local_68,&local_res20);
      FUN_141804870(&DAT_143271f04,0x905,0x2200000b,*puVar12);
      if (local_68 != (int *)0x0) {
        FUN_14019f2c0(local_68 + -4);
      }
      if (pwVar4 != (wchar_t *)0x0) {
        FUN_14019f2c0(pwVar4 + -8);
      }
    }
    if ((DAT_143ad2100 == '\0') && (*(char *)(param_1 + 0x1a8) != '\0')) {
      DAT_143ad2100 = '\x01';
      local_48[0] = &local_res20;
      local_res20 = (int *)0x0;
      local_res18 = (wchar_t *)0x0;
      piVar13 = (int *)FUN_1401bc720(&DAT_143ad6980,0x30);
      piVar13[1] = 0xf;
      *piVar13 = -1;
      local_res18 = (wchar_t *)(piVar13 + 4);
      piVar13[2] = 0;
      *local_res18 = L'\0';
      uVar3 = u_inactiveAccount_1433fdc48._12_4_;
      uVar2 = u_inactiveAccount_1433fdc48._8_4_;
      uVar11 = u_inactiveAccount_1433fdc48._4_4_;
      *(undefined4 *)local_res18 = u_inactiveAccount_1433fdc48._0_4_;
      piVar13[5] = uVar11;
      piVar13[6] = uVar2;
      piVar13[7] = uVar3;
      *(undefined8 *)(piVar13 + 8) = u_inactiveAccount_1433fdc48._16_8_;
      piVar13[10] = u_inactiveAccount_1433fdc48._24_4_;
      *(wchar_t *)(piVar13 + 0xb) = u_inactiveAccount_1433fdc48[0xe];
      if (*piVar13 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar13[1] < 0xf) {
        FUN_142e54290(0x90,piVar13[1],0xf);
      }
      *piVar13 = 1;
      local_res18[0xf] = L'\0';
      if (piVar13[1] + 1 < 0x10) {
        FUN_142e54290(0x9c,0xf);
      }
      piVar13[2] = 0x1e;
      FUN_141b4ac80(&local_res18,&local_res20,0);
      *(undefined1 *)(param_1 + 0x1a8) = 0;
      if (cVar5 == '\0') {
        lVar19 = DAT_143ad2230 + 8;
        if (DAT_143ad2230 == 0) {
          lVar19 = 0;
        }
        FUN_142c0bf50(DAT_143abfdf8,lVar19,0);
      }
    }
    cVar5 = FUN_1429e4f10(0x2800);
    if ((DAT_143ad2248 == '\0') && (DAT_143ad2248 = '\x01', cVar5 == '\0')) {
      ppiVar14 = (int **)FUN_1408a9e40(&local_res20,0x1a5);
      local_res18 = (wchar_t *)0x0;
      local_48[0] = ppiVar14;
      FUN_14019a260(&local_res18,ppiVar14);
      FUN_142a26280(&local_res18,0,0,1,0,0,0,0,0,0);
      if (*ppiVar14 != (int *)0x0) {
        FUN_14019f2c0(*ppiVar14 + -4);
      }
    }
    FUN_141b2ab80(param_1);
    return;
  }
  *(undefined1 *)(param_1 + 0x108) = 1;
  piVar13 = (int *)FUN_141b44520(param_1 + 0x100,0xffffffff);
  *piVar13 = (int)cVar6;
  local_68 = piVar13;
  plVar15 = (longlong *)FUN_1406e9050(param_2,&local_60);
  ppiVar14 = (int **)(piVar13 + 2);
  piVar21 = (int *)0x0;
  if (*ppiVar14 != (int *)0x0) {
    FUN_14019f2c0(*ppiVar14 + -4);
    *ppiVar14 = (int *)0x0;
  }
  *ppiVar14 = (int *)*plVar15;
  *plVar15 = 0;
  if (local_60 != (int *)0x0) {
    FUN_14019f2c0(local_60 + -4);
  }
  local_res20 = (int *)0x0;
  piVar20 = local_res20;
  if (((&local_res20 == ppiVar14) || (piVar1 = *ppiVar14, piVar1 == (int *)0x0)) ||
     (piVar24 = piVar1 + -4, piVar24 == (int *)0x0)) goto LAB_141b2ffc9;
  if (*piVar24 != -1) {
    if (*piVar24 < 1) {
      FUN_142e52dd0(0xd2);
    }
    LOCK();
    *piVar24 = *piVar24 + 1;
    UNLOCK();
    piVar20 = piVar1;
    if (local_res20 != (int *)0x0) {
      FUN_14019f2c0(local_res20 + -4);
    }
    goto LAB_141b2ffc9;
  }
  FUN_142e52d50(0xcb,0xffffff01);
  piVar1 = *ppiVar14;
  local_60 = (int *)0x0;
  piVar20 = piVar21;
  if (piVar1 != (int *)0x0) {
    piVar24 = (int *)0xffffffffffffffff;
    do {
      piVar24 = (int *)((longlong)piVar24 + 1);
    } while (*(char *)((longlong)piVar1 + (longlong)piVar24) != '\0');
    iVar23 = (int)piVar24;
    iVar10 = 0;
    if (0 < iVar23) {
      iVar10 = iVar23;
    }
    piVar16 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar10 + 0x11));
    piVar16[1] = iVar10;
    *piVar16 = -1;
    piVar20 = piVar16 + 4;
    piVar16[2] = 0;
    *(undefined1 *)piVar20 = 0;
    local_50 = (longlong)iVar23;
    local_60 = piVar20;
    FUN_142ef7ba0(piVar20,piVar1,local_50);
    if (*piVar16 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar23 == -1) || (iVar23 <= piVar16[1])) {
      *piVar16 = 1;
      if (iVar23 != -1) goto LAB_141b2ff19;
      piVar24 = piVar21;
      if (piVar20 != (int *)0x0) {
        piVar24 = (int *)0xffffffffffffffff;
        do {
          piVar24 = (int *)((longlong)piVar24 + 1);
        } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,piVar16[1],(ulonglong)piVar24 & 0xffffffff);
      *piVar16 = 1;
LAB_141b2ff19:
      *(undefined1 *)((longlong)piVar20 + local_50) = 0;
    }
    iVar10 = (int)piVar24;
    if ((iVar10 < 0) || (piVar16[1] + 1 <= iVar10)) {
      FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
    }
    piVar16[2] = iVar10;
  }
  if (local_res20 != (int *)0x0) {
    FUN_14019f2c0(local_res20 + -4);
  }
LAB_141b2ffc9:
  local_res20 = piVar20;
  FUN_142cf0150(local_48[0],(ulonglong)local_res18 & 0xffffffff,&local_res20);
  bVar7 = FUN_1406e8ae0(param_2);
  piVar13[4] = (uint)bVar7;
  puVar12 = (undefined8 *)FUN_1406e9050(param_2,&local_res18);
  if (*(longlong *)(piVar13 + 6) != 0) {
    FUN_14019f2c0(*(longlong *)(piVar13 + 6) + -0x10);
    piVar13[6] = 0;
    piVar13[7] = 0;
  }
  *(undefined8 *)(piVar13 + 6) = *puVar12;
  *puVar12 = 0;
  if (local_res18 != (wchar_t *)0x0) {
    FUN_14019f2c0(local_res18 + -8);
  }
  bVar7 = FUN_1406e8ae0(param_2);
  piVar13[0xc] = (uint)bVar7;
  bVar7 = FUN_1406e8ae0(param_2);
  uVar22 = (uint)bVar7;
  piVar20 = piVar13;
  if (bVar7 != 0) {
    do {
      plVar15 = (longlong *)FUN_141b443d0(piVar13 + 10,0xffffffff);
      plVar17 = (longlong *)FUN_1406e9050(param_2,local_48);
      if (*plVar15 != 0) {
        FUN_14019f2c0(*plVar15 + -0x10);
        *plVar15 = 0;
      }
      *plVar15 = *plVar17;
      *plVar17 = 0;
      if (local_48[0] != (int **)0x0) {
        FUN_14019f2c0(local_48[0] + -2);
      }
      uVar11 = FUN_1406e8c20(param_2);
      *(undefined4 *)(plVar15 + 1) = uVar11;
      bVar7 = FUN_1406e8ae0(param_2);
      *(uint *)((longlong)plVar15 + 0xc) = (uint)bVar7;
      bVar7 = FUN_1406e8ae0(param_2);
      *(uint *)(plVar15 + 2) = (uint)bVar7;
      bVar7 = FUN_1406e8ae0(param_2);
      *(uint *)((longlong)plVar15 + 0x14) = (uint)bVar7;
      bVar7 = FUN_1406e8ae0(param_2);
      *(uint *)(plVar15 + 3) = (uint)bVar7;
      uVar22 = uVar22 - 1;
      piVar20 = local_68;
    } while (0 < (int)uVar22);
  }
  uVar8 = FUN_1406e8b80(param_2);
  *(uint *)(param_1 + 0x174) = (uint)uVar8;
  FUN_141b45190(param_1 + 0x178);
  if ((*(int *)(param_1 + 0x174) != 0) && (0 < *(int *)(param_1 + 0x174))) {
    do {
      local_58 = 0;
      uVar8 = FUN_1406e8b80(param_2);
      local_60 = (int *)CONCAT44(local_60._4_4_,(uint)uVar8);
      uVar9 = FUN_1406e8b80(param_2);
      local_60 = (int *)(ulonglong)CONCAT24(uVar9,local_60._0_4_);
      plVar15 = (longlong *)FUN_1406e9050(param_2,&local_50);
      lVar19 = *plVar15;
      *plVar15 = 0;
      local_58 = lVar19;
      if (local_50 != 0) {
        FUN_14019f2c0(local_50 + -0x10);
      }
      puVar18 = (uint *)FUN_141b44280(param_1 + 0x178,0xffffffff);
      *puVar18 = (uint)uVar8;
      puVar18[1] = (uint)uVar9;
      FUN_14019a260(puVar18 + 2,&local_58);
      if (lVar19 != 0) {
        FUN_14019f2c0(lVar19 + -0x10);
      }
      uVar22 = (int)piVar21 + 1;
      piVar21 = (int *)(ulonglong)uVar22;
      piVar20 = local_68;
    } while ((int)uVar22 < *(int *)(param_1 + 0x174));
  }
  iVar10 = FUN_1406e8c20(param_2);
  piVar20[8] = iVar10;
  cVar5 = FUN_1406e8ae0(param_2);
  if (cVar5 != '\0') {
    FUN_1408e4210(piVar20 + 0xe,param_2);
  }
  if (local_res20 != (int *)0x0) {
    FUN_14019f2c0(local_res20 + -4);
  }
  return;
}


