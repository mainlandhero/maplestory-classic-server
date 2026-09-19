
//===========================================================
// FUN_142a8a190 @ 142a8a190   (586 bytes)
//===========================================================

void FUN_142a8a190(longlong param_1,longlong param_2,undefined4 param_3,uint param_4)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 *puVar6;
  undefined1 local_a8 [8];
  undefined1 local_a0 [8];
  undefined1 local_98 [8];
  longlong local_90;
  longlong local_88;
  undefined8 uStack_80;
  undefined1 *local_78;
  undefined1 *puStack_70;
  undefined **local_68;
  longlong local_60;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined1 *local_50;
  undefined1 *puStack_48;
  undefined ***local_30;
  
  FUN_142a8fe70();
  lVar3 = DAT_143aa84a0;
  if (DAT_143aa84a0 != 0) {
    *(uint *)(param_1 + 0x6bc) = param_4 & 0xff;
    *(undefined4 *)(param_1 + 0x4f0) = 1;
    *(undefined4 *)(param_1 + 0x410) = 1;
    if (*(char *)(param_2 + 4) == '\0') {
      lVar5 = FUN_142cbe6e0(lVar3,local_98);
      if ((*(longlong *)(param_1 + 0x3f8) - 1U < 999) || (*(longlong *)(param_1 + 0x3f8) == -1)) {
        FUN_142e52ed0(0x447);
      }
      if (param_1 + 0x3f0 == lVar5) {
        FUN_142e52d50(0x45c,1);
      }
      lVar2 = *(longlong *)(lVar5 + 8);
      if (lVar2 != 0) {
        if (0xfffff < *(ulonglong *)(lVar2 + -0x20)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar2 + -0x20) = *(longlong *)(lVar2 + -0x20) + 1;
        UNLOCK();
      }
      FUN_1420a3270(param_1 + 0x3f0);
      lVar2 = local_90;
      *(undefined8 *)(param_1 + 0x3f8) = *(undefined8 *)(lVar5 + 8);
      if (local_90 != 0) {
        puVar6 = (undefined8 *)(local_90 + -0x28);
        if (0xffffe < *(longlong *)(local_90 + -0x20) - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar1 = (longlong *)(lVar2 + -0x20);
        lVar5 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if ((int)lVar5 == 1) {
          if ((local_90 != 0) && (*(longlong *)(local_90 + -0x10) != 0)) {
            LOCK();
            *(undefined8 *)(*(longlong *)(local_90 + -0x10) + 8) = 0;
            UNLOCK();
            do {
            } while (*(int *)(*(longlong *)(local_90 + -0x10) + 4) != 0);
          }
          if (puVar6 != (undefined8 *)0x0) {
            (**(code **)*puVar6)(puVar6,1);
          }
        }
      }
      lVar5 = *(longlong *)(param_1 + 0x3f8);
      if (lVar5 == 0) {
        return;
      }
      local_a0[0] = *(int *)(param_1 + 0x6bc) != 0;
      local_a8[0] = *(undefined1 *)(param_2 + 4);
      local_88 = lVar3;
      local_78 = local_a8;
      puStack_70 = local_a0;
      local_68 = &PTR_LAB_14348be18;
      uStack_80._0_4_ = (undefined4)lVar5;
      uStack_80._4_4_ = (undefined4)((ulonglong)lVar5 >> 0x20);
      local_60 = lVar3;
      uStack_58 = (undefined4)uStack_80;
      uStack_54 = uStack_80._4_4_;
      uStack_80 = lVar5;
      local_50 = local_78;
      puStack_48 = puStack_70;
    }
    else {
      if (*(char *)(param_2 + 4) != 'd') {
        return;
      }
      uVar4 = FUN_1427be040(DAT_143aa8518,&local_88);
      FUN_142a92740(param_1 + 0x400,uVar4);
      FUN_14287f2b0(&local_88);
      local_68 = &PTR_LAB_14348be48;
      local_60 = param_1 + 0x400;
    }
    local_30 = &local_68;
    FUN_142a91f30(param_1 + 0x418,param_3,*(undefined4 *)(param_1 + 0x2a8),param_2 + 8,&local_68);
    FUN_142a6e9f0(param_1,1);
  }
  return;
}



//===========================================================
// FUN_14287f2b0 @ 14287f2b0   (307 bytes)
//===========================================================

void FUN_14287f2b0(longlong param_1)

{
  undefined8 *puVar1;
  int *piVar2;
  longlong lVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  longlong *plVar6;
  undefined1 local_res8 [8];
  undefined8 *local_res10;
  
  if (*(longlong *)(param_1 + 8) != 0) {
    puVar4 = *(undefined8 **)(*(longlong *)(param_1 + 8) + 0x38);
    if (puVar4 != (undefined8 *)0x0) {
      if (0xfffff < (ulonglong)puVar4[1]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      puVar4[1] = puVar4[1] + 1;
      UNLOCK();
    }
    puVar1 = puVar4 + 4;
    local_res10 = puVar1;
    FUN_1401d3510(puVar1,local_res8);
    plVar6 = (longlong *)(*(longlong *)(param_1 + 8) + 0x18);
    if (*(longlong *)(param_1 + 8) == 0) {
      plVar6 = (longlong *)&DAT_00000008;
    }
    if (0xffffe < *plVar6 - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    lVar3 = *plVar6;
    *plVar6 = *plVar6 + -1;
    UNLOCK();
    if ((int)lVar3 == 1) {
      puVar4[6] = 0;
      puVar5 = (undefined8 *)(*(longlong *)(param_1 + 8) + 0x10);
      if (*(longlong *)(param_1 + 8) == 0) {
        puVar5 = (undefined8 *)0x0;
      }
      if (puVar5 != (undefined8 *)0x0) {
        (**(code **)*puVar5)(puVar5,1);
      }
    }
    if (puVar1 != (undefined8 *)0x0) {
      piVar2 = (int *)(puVar4 + 5);
      *piVar2 = *piVar2 + -1;
      if (*piVar2 == 0) {
        *puVar1 = 0;
      }
    }
    if (puVar4 != (undefined8 *)0x0) {
      if (0xffffe < puVar4[1] - 1) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar6 = puVar4 + 1;
      lVar3 = *plVar6;
      *plVar6 = *plVar6 + -1;
      UNLOCK();
      if ((int)lVar3 == 1) {
        (**(code **)*puVar4)(puVar4,1);
      }
    }
    *(undefined8 *)(param_1 + 8) = 0;
  }
  return;
}



//===========================================================
// FUN_142e52ed0 @ 142e52ed0   (4890 bytes)
//===========================================================

void FUN_142e52ed0(undefined4 param_1,undefined8 param_2)

{
  char *pcVar1;
  longlong lVar2;
  char cVar3;
  undefined8 uVar4;
  undefined4 *puVar5;
  longlong lVar6;
  int iVar7;
  int *piVar8;
  int iVar9;
  int iVar10;
  int *piVar11;
  int *piVar12;
  int *piVar13;
  int iVar14;
  int iVar15;
  longlong lVar16;
  undefined4 local_res8 [2];
  undefined8 local_res10;
  undefined4 local_res18;
  undefined4 local_res20 [2];
  char *local_a8;
  longlong local_a0;
  longlong local_98;
  longlong local_90;
  longlong local_88;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  longlong local_50 [2];
  
  piVar13 = (int *)0x0;
  iVar9 = 0;
  local_res18 = 0;
  local_res8[0] = param_1;
  local_res10 = param_2;
  cVar3 = FUN_142e559e0();
  if (cVar3 == '\0') {
    return;
  }
  FUN_140194c60(&local_78);
  local_res20[0] = FUN_14091a3e0(&local_78);
  uVar4 = FUN_142a1d8a0(local_50);
  local_a8 = (char *)0x0;
  local_res18 = 1;
  FUN_1408bc980(&local_98,uVar4);
  lVar2 = local_98;
  pcVar1 = local_a8;
  local_res18 = 7;
  piVar12 = (int *)0xffffffffffffffff;
  if (local_98 != 0) {
    iVar10 = *(int *)(local_98 + -8);
    piVar8 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar11 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e530bb;
      if (*local_a8 == '\0') {
        if ((local_a8 == (char *)0x0) ||
           (piVar11 = (int *)(local_a8 + -0x10), piVar11 == (int *)0x0)) {
LAB_142e530bb:
          if (iVar15 < iVar10) {
            iVar15 = iVar10;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
          puVar5[1] = iVar15;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          puVar5[2] = 0;
          *local_a8 = '\0';
          if (piVar11 != (int *)0x0) {
            FUN_14019f2c0(piVar11);
          }
        }
        else {
          if ((1 < *piVar11) || (*(int *)(local_a8 + -0xc) < iVar10)) {
            iVar15 = *(int *)(local_a8 + -8);
            goto LAB_142e530bb;
          }
          if (*piVar11 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar11 = -1;
        }
        FUN_142ef7ba0(local_a8,lVar2,piVar8);
        pcVar1 = local_a8;
        if (*(int *)(local_a8 + -0x10) != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
          if (iVar10 != -1) goto LAB_142e53148;
          piVar8 = piVar13;
          if (pcVar1 != (char *)0x0) {
            do {
              piVar12 = (int *)((longlong)piVar12 + 1);
              piVar8 = piVar12;
            } while (pcVar1[(longlong)piVar12] != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
LAB_142e53148:
          *(char *)((longlong)piVar8 + (longlong)local_a8) = '\0';
        }
        iVar10 = (int)piVar8;
        if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
          FUN_142e54290(0x9c,(ulonglong)piVar8 & 0xffffffff);
        }
        *(int *)(pcVar1 + -8) = iVar10;
        goto LAB_142e53173;
      }
      iVar15 = *(int *)(local_a8 + -8);
      for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
      }
      piVar12 = (int *)(local_a8 + -0x10);
      iVar14 = iVar9;
      if (piVar12 == (int *)0x0) {
LAB_142e52fbf:
        if (iVar14 < iVar7) {
          iVar14 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
        puVar5[1] = iVar14;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        if (piVar12 == (int *)0x0) {
          puVar5[2] = 0;
          *local_a8 = '\0';
        }
        else {
          iVar7 = *(int *)(pcVar1 + -8) + 1;
          if (iVar14 + 1 < iVar7) {
            FUN_142e54290(0x5c,iVar7,iVar14 + 1);
            iVar7 = iVar14 + 1;
          }
          FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
          puVar5[2] = *(undefined4 *)(pcVar1 + -8);
          local_a8[iVar14] = '\0';
          FUN_14019f2c0(piVar12);
        }
      }
      else {
        if ((1 < *piVar12) || (*(int *)(local_a8 + -0xc) < iVar7)) {
          iVar14 = *(int *)(local_a8 + -8);
          goto LAB_142e52fbf;
        }
        if (*piVar12 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar12 = -1;
      }
      iVar7 = iVar9;
      if (local_a8 != (char *)0x0) {
        iVar7 = *(int *)(local_a8 + -8);
      }
      FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar8);
      FUN_14019c870(&local_a8,iVar15 + iVar10);
    }
  }
LAB_142e53173:
  piVar12 = (int *)0xffffffffffffffff;
  if (local_98 != 0) {
    FUN_14019f2c0(local_98 + -0x10);
  }
  local_70 = 0;
  uVar4 = FUN_14019ba10(&local_70,&DAT_143272338,"LogCallStack3");
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x27;
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  lVar2 = local_a0;
  pcVar1 = local_a8;
  local_res18 = 0x1f;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    piVar8 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar11 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e53377;
      if (*local_a8 != '\0') {
        iVar15 = *(int *)(local_a8 + -8);
        for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
        }
        piVar12 = (int *)(local_a8 + -0x10);
        iVar14 = iVar9;
        if (piVar12 == (int *)0x0) {
LAB_142e5327f:
          if (iVar14 < iVar7) {
            iVar14 = iVar7;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
          puVar5[1] = iVar14;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          if (piVar12 == (int *)0x0) {
            puVar5[2] = 0;
            *local_a8 = '\0';
          }
          else {
            iVar7 = *(int *)(pcVar1 + -8) + 1;
            if (iVar14 + 1 < iVar7) {
              FUN_142e54290(0x5c,iVar7,iVar14 + 1);
              iVar7 = iVar14 + 1;
            }
            FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
            puVar5[2] = *(undefined4 *)(pcVar1 + -8);
            local_a8[iVar14] = '\0';
            FUN_14019f2c0(piVar12);
          }
        }
        else {
          if ((1 < *piVar12) || (*(int *)(local_a8 + -0xc) < iVar7)) {
            iVar14 = *(int *)(local_a8 + -8);
            goto LAB_142e5327f;
          }
          if (*piVar12 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar12 = -1;
        }
        iVar7 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar7 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar8);
        FUN_14019c870(&local_a8,iVar15 + iVar10);
        goto LAB_142e5342f;
      }
      if ((local_a8 == (char *)0x0) || (piVar11 = (int *)(local_a8 + -0x10), piVar11 == (int *)0x0))
      {
LAB_142e53377:
        if (iVar15 < iVar10) {
          iVar15 = iVar10;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
        puVar5[1] = iVar15;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        puVar5[2] = 0;
        *local_a8 = '\0';
        if (piVar11 != (int *)0x0) {
          FUN_14019f2c0(piVar11);
        }
      }
      else {
        if ((1 < *piVar11) || (*(int *)(local_a8 + -0xc) < iVar10)) {
          iVar15 = *(int *)(local_a8 + -8);
          goto LAB_142e53377;
        }
        if (*piVar11 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar11 = -1;
      }
      FUN_142ef7ba0(local_a8,lVar2,piVar8);
      pcVar1 = local_a8;
      if (*(int *)(local_a8 + -0x10) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
        if (iVar10 != -1) goto LAB_142e53407;
        piVar8 = piVar13;
        if (pcVar1 != (char *)0x0) {
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
            piVar8 = piVar12;
          } while (pcVar1[(longlong)piVar12] != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
LAB_142e53407:
        *(char *)((longlong)piVar8 + (longlong)local_a8) = '\0';
      }
      iVar10 = (int)piVar8;
      if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
        FUN_142e54290(0x9c,(ulonglong)piVar8 & 0xffffffff);
      }
      *(int *)(pcVar1 + -8) = iVar10;
    }
  }
LAB_142e5342f:
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  local_68 = 0;
  uVar4 = FUN_14019ba10(&local_68,&DAT_143272338,&DAT_1434997dc);
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x11f;
  if (local_68 != 0) {
    FUN_14019f2c0(local_68 + -0x10);
  }
  lVar2 = local_a0;
  pcVar1 = local_a8;
  local_res18 = 0xdf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    piVar12 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar8 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e5363e;
      if (*local_a8 != '\0') {
        iVar15 = *(int *)(local_a8 + -8);
        for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
        }
        piVar8 = (int *)(local_a8 + -0x10);
        iVar14 = iVar9;
        if (piVar8 == (int *)0x0) {
LAB_142e5353f:
          if (iVar14 < iVar7) {
            iVar14 = iVar7;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
          puVar5[1] = iVar14;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          if (piVar8 == (int *)0x0) {
            puVar5[2] = 0;
            *local_a8 = '\0';
          }
          else {
            iVar7 = *(int *)(pcVar1 + -8) + 1;
            if (iVar14 + 1 < iVar7) {
              FUN_142e54290(0x5c,iVar7,iVar14 + 1);
              iVar7 = iVar14 + 1;
            }
            FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
            puVar5[2] = *(undefined4 *)(pcVar1 + -8);
            local_a8[iVar14] = '\0';
            FUN_14019f2c0(piVar8);
          }
        }
        else {
          if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar7)) {
            iVar14 = *(int *)(local_a8 + -8);
            goto LAB_142e5353f;
          }
          if (*piVar8 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar8 = -1;
        }
        iVar7 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar7 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
        FUN_14019c870(&local_a8,iVar15 + iVar10);
        goto LAB_142e536fd;
      }
      if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0)) {
LAB_142e5363e:
        if (iVar15 < iVar10) {
          iVar15 = iVar10;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
        puVar5[1] = iVar15;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        puVar5[2] = 0;
        *local_a8 = '\0';
        if (piVar8 != (int *)0x0) {
          FUN_14019f2c0(piVar8);
        }
      }
      else {
        if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
          iVar15 = *(int *)(local_a8 + -8);
          goto LAB_142e5363e;
        }
        if (*piVar8 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar8 = -1;
      }
      FUN_142ef7ba0(local_a8,lVar2,piVar12);
      pcVar1 = local_a8;
      if (*(int *)(local_a8 + -0x10) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
        if (iVar10 != -1) goto LAB_142e536d5;
        piVar12 = piVar13;
        if (pcVar1 != (char *)0x0) {
          piVar12 = (int *)0xffffffffffffffff;
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
          } while (pcVar1[(longlong)piVar12] != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
LAB_142e536d5:
        *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
      }
      iVar10 = (int)piVar12;
      if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
        FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
      }
      *(int *)(pcVar1 + -8) = iVar10;
    }
  }
LAB_142e536fd:
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  FUN_1408bc140(&local_90,local_res20);
  lVar2 = local_90;
  pcVar1 = local_a8;
  local_res18 = 0x6df;
  if (local_90 != 0) {
    iVar10 = *(int *)(local_90 + -8);
    piVar12 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar8 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e538ca;
      if (*local_a8 == '\0') {
        if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0))
        {
LAB_142e538ca:
          if (iVar15 < iVar10) {
            iVar15 = iVar10;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
          puVar5[1] = iVar15;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          puVar5[2] = 0;
          *local_a8 = '\0';
          if (piVar8 != (int *)0x0) {
            FUN_14019f2c0(piVar8);
          }
        }
        else {
          if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
            iVar15 = *(int *)(local_a8 + -8);
            goto LAB_142e538ca;
          }
          if (*piVar8 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar8 = -1;
        }
        FUN_142ef7ba0(local_a8,lVar2,piVar12);
        pcVar1 = local_a8;
        if (*(int *)(local_a8 + -0x10) != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
          if (iVar10 != -1) goto LAB_142e5395e;
          piVar12 = piVar13;
          if (pcVar1 != (char *)0x0) {
            piVar12 = (int *)0xffffffffffffffff;
            do {
              piVar12 = (int *)((longlong)piVar12 + 1);
            } while (pcVar1[(longlong)piVar12] != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
          pcVar1[-0x10] = '\x01';
          pcVar1[-0xf] = '\0';
          pcVar1[-0xe] = '\0';
          pcVar1[-0xd] = '\0';
LAB_142e5395e:
          *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
        }
        iVar10 = (int)piVar12;
        if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
          FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
        }
        *(int *)(pcVar1 + -8) = iVar10;
        goto LAB_142e53989;
      }
      iVar15 = *(int *)(local_a8 + -8);
      for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
      }
      piVar8 = (int *)(local_a8 + -0x10);
      iVar14 = iVar9;
      if (piVar8 == (int *)0x0) {
LAB_142e537cf:
        if (iVar14 < iVar7) {
          iVar14 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
        puVar5[1] = iVar14;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        if (piVar8 == (int *)0x0) {
          puVar5[2] = 0;
          *local_a8 = '\0';
        }
        else {
          iVar7 = *(int *)(pcVar1 + -8) + 1;
          if (iVar14 + 1 < iVar7) {
            FUN_142e54290(0x5c,iVar7,iVar14 + 1);
            iVar7 = iVar14 + 1;
          }
          FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
          puVar5[2] = *(undefined4 *)(pcVar1 + -8);
          local_a8[iVar14] = '\0';
          FUN_14019f2c0(piVar8);
        }
      }
      else {
        if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar7)) {
          iVar14 = *(int *)(local_a8 + -8);
          goto LAB_142e537cf;
        }
        if (*piVar8 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar8 = -1;
      }
      iVar7 = iVar9;
      if (local_a8 != (char *)0x0) {
        iVar7 = *(int *)(local_a8 + -8);
      }
      FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
      FUN_14019c870(&local_a8,iVar15 + iVar10);
    }
  }
LAB_142e53989:
  if (local_90 != 0) {
    FUN_14019f2c0(local_90 + -0x10);
  }
  local_60 = 0;
  uVar4 = FUN_14019ba10(&local_60,&DAT_143272338,&DAT_1434997f8);
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x26df;
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  lVar2 = local_a0;
  pcVar1 = local_a8;
  local_res18 = 0x1edf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    piVar12 = (int *)(longlong)iVar10;
    if (iVar10 != 0) {
      piVar8 = piVar13;
      iVar15 = iVar9;
      if (local_a8 == (char *)0x0) goto LAB_142e53b90;
      if (*local_a8 != '\0') {
        iVar15 = *(int *)(local_a8 + -8);
        for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
        }
        piVar8 = (int *)(local_a8 + -0x10);
        iVar14 = iVar9;
        if (piVar8 == (int *)0x0) {
LAB_142e53a91:
          if (iVar14 < iVar7) {
            iVar14 = iVar7;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
          puVar5[1] = iVar14;
          *puVar5 = 0xffffffff;
          local_a8 = (char *)(puVar5 + 4);
          if (piVar8 == (int *)0x0) {
            puVar5[2] = 0;
            *local_a8 = '\0';
          }
          else {
            iVar7 = *(int *)(pcVar1 + -8) + 1;
            if (iVar14 + 1 < iVar7) {
              FUN_142e54290(0x5c,iVar7,iVar14 + 1);
              iVar7 = iVar14 + 1;
            }
            FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
            puVar5[2] = *(undefined4 *)(pcVar1 + -8);
            local_a8[iVar14] = '\0';
            FUN_14019f2c0(piVar8);
          }
        }
        else {
          if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar7)) {
            iVar14 = *(int *)(local_a8 + -8);
            goto LAB_142e53a91;
          }
          if (*piVar8 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar8 = -1;
        }
        iVar7 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar7 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
        FUN_14019c870(&local_a8,iVar15 + iVar10);
        goto LAB_142e53c4f;
      }
      if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0)) {
LAB_142e53b90:
        if (iVar15 < iVar10) {
          iVar15 = iVar10;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
        puVar5[1] = iVar15;
        *puVar5 = 0xffffffff;
        local_a8 = (char *)(puVar5 + 4);
        puVar5[2] = 0;
        *local_a8 = '\0';
        if (piVar8 != (int *)0x0) {
          FUN_14019f2c0(piVar8);
        }
      }
      else {
        if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
          iVar15 = *(int *)(local_a8 + -8);
          goto LAB_142e53b90;
        }
        if (*piVar8 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar8 = -1;
      }
      FUN_142ef7ba0(local_a8,lVar2,piVar12);
      pcVar1 = local_a8;
      if (*(int *)(local_a8 + -0x10) != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
        if (iVar10 != -1) goto LAB_142e53c27;
        piVar12 = piVar13;
        if (pcVar1 != (char *)0x0) {
          piVar12 = (int *)0xffffffffffffffff;
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
          } while (pcVar1[(longlong)piVar12] != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
        pcVar1[-0x10] = '\x01';
        pcVar1[-0xf] = '\0';
        pcVar1[-0xe] = '\0';
        pcVar1[-0xd] = '\0';
LAB_142e53c27:
        *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
      }
      iVar10 = (int)piVar12;
      if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
        FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
      }
      *(int *)(pcVar1 + -8) = iVar10;
    }
  }
LAB_142e53c4f:
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  FUN_1408bc020(&local_88,local_res8);
  lVar2 = local_88;
  pcVar1 = local_a8;
  local_res18 = 0xdedf;
  if (local_88 == 0) goto LAB_142e53edb;
  iVar10 = *(int *)(local_88 + -8);
  piVar12 = (int *)(longlong)iVar10;
  if (iVar10 == 0) goto LAB_142e53edb;
  piVar8 = piVar13;
  iVar15 = iVar9;
  if (local_a8 == (char *)0x0) goto LAB_142e53e1c;
  if (*local_a8 == '\0') {
    if ((local_a8 == (char *)0x0) || (piVar8 = (int *)(local_a8 + -0x10), piVar8 == (int *)0x0)) {
LAB_142e53e1c:
      if (iVar15 < iVar10) {
        iVar15 = iVar10;
      }
      puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar15 + 0x11));
      puVar5[1] = iVar15;
      *puVar5 = 0xffffffff;
      local_a8 = (char *)(puVar5 + 4);
      puVar5[2] = 0;
      *local_a8 = '\0';
      if (piVar8 != (int *)0x0) {
        FUN_14019f2c0(piVar8);
      }
    }
    else {
      if ((1 < *piVar8) || (*(int *)(local_a8 + -0xc) < iVar10)) {
        iVar15 = *(int *)(local_a8 + -8);
        goto LAB_142e53e1c;
      }
      if (*piVar8 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar8 = -1;
    }
    piVar8 = (int *)0xffffffffffffffff;
    FUN_142ef7ba0(local_a8,lVar2,piVar12);
    pcVar1 = local_a8;
    if (*(int *)(local_a8 + -0x10) != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar10 == -1) || (iVar10 <= *(int *)(pcVar1 + -0xc))) {
      pcVar1[-0x10] = '\x01';
      pcVar1[-0xf] = '\0';
      pcVar1[-0xe] = '\0';
      pcVar1[-0xd] = '\0';
      if (iVar10 != -1) goto LAB_142e53eb0;
      if (pcVar1 != (char *)0x0) {
        do {
          piVar13 = (int *)((longlong)piVar8 + 1);
          piVar8 = piVar13;
        } while (pcVar1[(longlong)piVar13] != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),iVar10);
      pcVar1[-0x10] = '\x01';
      pcVar1[-0xf] = '\0';
      pcVar1[-0xe] = '\0';
      pcVar1[-0xd] = '\0';
LAB_142e53eb0:
      *(char *)((longlong)piVar12 + (longlong)local_a8) = '\0';
      piVar13 = piVar12;
    }
    iVar10 = (int)piVar13;
    if ((iVar10 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar10)) {
      FUN_142e54290(0x9c,(ulonglong)piVar13 & 0xffffffff);
    }
    *(int *)(pcVar1 + -8) = iVar10;
    goto LAB_142e53edb;
  }
  iVar15 = *(int *)(local_a8 + -8);
  for (iVar7 = *(int *)(local_a8 + -0xc); iVar7 < iVar15 + iVar10; iVar7 = iVar7 * 2) {
  }
  piVar13 = (int *)(local_a8 + -0x10);
  iVar14 = iVar9;
  if (piVar13 == (int *)0x0) {
LAB_142e53d21:
    if (iVar14 < iVar7) {
      iVar14 = iVar7;
    }
    puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar14 + 0x11));
    puVar5[1] = iVar14;
    *puVar5 = 0xffffffff;
    local_a8 = (char *)(puVar5 + 4);
    if (piVar13 == (int *)0x0) {
      puVar5[2] = 0;
      *local_a8 = '\0';
    }
    else {
      iVar7 = *(int *)(pcVar1 + -8) + 1;
      if (iVar14 + 1 < iVar7) {
        FUN_142e54290(0x5c,iVar7,iVar14 + 1);
        iVar7 = iVar14 + 1;
      }
      FUN_142ef7ba0(local_a8,pcVar1,(longlong)iVar7);
      puVar5[2] = *(undefined4 *)(pcVar1 + -8);
      local_a8[iVar14] = '\0';
      FUN_14019f2c0(piVar13);
    }
  }
  else {
    if ((1 < *piVar13) || (*(int *)(local_a8 + -0xc) < iVar7)) {
      iVar14 = *(int *)(local_a8 + -8);
      goto LAB_142e53d21;
    }
    if (*piVar13 != 1) {
      FUN_142e52dd0(0x74);
    }
    *piVar13 = -1;
  }
  iVar7 = iVar9;
  if (local_a8 != (char *)0x0) {
    iVar7 = *(int *)(local_a8 + -8);
  }
  FUN_142ef7ba0(local_a8 + iVar7,lVar2,piVar12);
  FUN_14019c870(&local_a8,iVar15 + iVar10);
LAB_142e53edb:
  if (local_88 != 0) {
    FUN_14019f2c0(local_88 + -0x10);
  }
  local_58 = 0;
  uVar4 = FUN_14019ba10(&local_58,&DAT_143272338,"Info1");
  local_a0 = 0;
  FUN_14019a260(&local_a0,uVar4);
  local_res18 = 0x4dedf;
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  lVar2 = local_a0;
  local_res18 = 0x3dedf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    lVar16 = (longlong)iVar10;
    if (iVar10 != 0) {
      if ((local_a8 == (char *)0x0) || (*local_a8 == '\0')) {
        uVar4 = FUN_14019bd40(&local_a8,iVar10,0);
        FUN_142ef7ba0(uVar4,lVar2,lVar16);
      }
      else {
        iVar10 = *(int *)(local_a8 + -8) + iVar10;
        for (iVar15 = *(int *)(local_a8 + -0xc); iVar15 < iVar10; iVar15 = iVar15 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_a8,iVar15,1);
        iVar15 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar15 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(iVar15 + lVar6,lVar2,lVar16);
      }
      FUN_14019c870(&local_a8,iVar10);
    }
  }
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  FUN_1408bc3a0(&local_80,&local_res10);
  lVar2 = local_80;
  local_res18 = 0x1bdedf;
  if (local_80 != 0) {
    iVar10 = *(int *)(local_80 + -8);
    lVar16 = (longlong)iVar10;
    if (iVar10 != 0) {
      if ((local_a8 == (char *)0x0) || (*local_a8 == '\0')) {
        uVar4 = FUN_14019bd40(&local_a8,iVar10,0);
        FUN_142ef7ba0(uVar4,lVar2,lVar16);
      }
      else {
        iVar10 = *(int *)(local_a8 + -8) + iVar10;
        for (iVar15 = *(int *)(local_a8 + -0xc); iVar15 < iVar10; iVar15 = iVar15 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_a8,iVar15,1);
        iVar15 = iVar9;
        if (local_a8 != (char *)0x0) {
          iVar15 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(iVar15 + lVar6,lVar2,lVar16);
      }
      FUN_14019c870(&local_a8,iVar10);
    }
  }
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  FUN_1408bc980(&local_a0,&local_78);
  lVar2 = local_a0;
  local_res18 = 0x7bdedf;
  if (local_a0 != 0) {
    iVar10 = *(int *)(local_a0 + -8);
    lVar16 = (longlong)iVar10;
    if (iVar10 != 0) {
      if ((local_a8 == (char *)0x0) || (*local_a8 == '\0')) {
        uVar4 = FUN_14019bd40(&local_a8,iVar10,0);
        FUN_142ef7ba0(uVar4,lVar2,lVar16);
      }
      else {
        iVar10 = *(int *)(local_a8 + -8) + iVar10;
        for (iVar15 = *(int *)(local_a8 + -0xc); iVar15 < iVar10; iVar15 = iVar15 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_a8,iVar15,1);
        if (local_a8 != (char *)0x0) {
          iVar9 = *(int *)(local_a8 + -8);
        }
        FUN_142ef7ba0(iVar9 + lVar6,lVar2,lVar16);
      }
      FUN_14019c870(&local_a8,iVar10);
    }
  }
  if (local_a0 != 0) {
    FUN_14019f2c0(local_a0 + -0x10);
  }
  FUN_142a1ec10(&local_a8);
  if (local_a8 != (char *)0x0) {
    FUN_14019f2c0(local_a8 + -0x10);
  }
  if (local_50[0] != 0) {
    FUN_14019f2c0(local_50[0] + -0x10);
  }
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  return;
}



//===========================================================
// FUN_1420a3270 @ 1420a3270   (145 bytes)
//===========================================================

void FUN_1420a3270(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 *puVar3;
  
  lVar2 = *(longlong *)(param_1 + 8);
  if (lVar2 != 0) {
    puVar3 = (undefined8 *)(lVar2 + -0x28);
    if (0xffffe < *(longlong *)(lVar2 + -0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + -0x20);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar2 == 1) {
      lVar2 = *(longlong *)(param_1 + 8);
      if ((lVar2 != 0) && (*(longlong *)(lVar2 + -0x10) != 0)) {
        LOCK();
        *(undefined8 *)(*(longlong *)(lVar2 + -0x10) + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(lVar2 + -0x10) + 4) != 0);
      }
      if (puVar3 != (undefined8 *)0x0) {
        (**(code **)*puVar3)(puVar3,1);
      }
    }
    *(undefined8 *)(param_1 + 8) = 0;
  }
  return;
}



//===========================================================
// FUN_142cbe6e0 @ 142cbe6e0   (71 bytes)
//===========================================================

longlong FUN_142cbe6e0(longlong param_1,longlong param_2)

{
  longlong lVar1;
  
  lVar1 = *(longlong *)(param_1 + 0x2358);
  *(longlong *)(param_2 + 8) = lVar1;
  if (lVar1 != 0) {
    if (0xfffff < *(ulonglong *)(lVar1 + -0x20)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar1 + -0x20) = *(longlong *)(lVar1 + -0x20) + 1;
    UNLOCK();
  }
  return param_2;
}



//===========================================================
// FUN_142a6e9f0 @ 142a6e9f0   (664 bytes)
//===========================================================

void FUN_142a6e9f0(longlong param_1,int param_2)

{
  char cVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  undefined4 uVar6;
  uint uVar7;
  int iVar8;
  
  iVar4 = *(int *)(param_1 + 0x2b4);
  iVar2 = FUN_142a6fae0();
  uVar7 = 0;
  if (iVar2 < iVar4) {
    iVar2 = *(int *)(param_1 + 0x2a0);
    iVar3 = FUN_142a6fae0(param_1);
    iVar8 = 0x12;
    if (iVar2 != 1 && iVar2 != 2) {
      iVar8 = 8;
    }
    if ((iVar4 - iVar3) / iVar8 < 2) goto LAB_142a6ea77;
    iVar4 = FUN_142a6fae0(param_1,(longlong)(iVar4 - iVar3) % (longlong)iVar8 & 0xffffffff);
    *(undefined4 *)(param_1 + 0x2c0) = 1;
    uVar5 = 1;
  }
  else {
LAB_142a6ea77:
    iVar2 = FUN_142a6fba0(param_1);
    if (iVar4 < iVar2) {
      iVar4 = FUN_142a6fba0(param_1);
    }
    *(undefined4 *)(param_1 + 0x2c0) = 0;
    uVar5 = uVar7;
  }
  *(int *)(param_1 + 0x2b8) = iVar4;
  if (param_2 == 0) {
    iVar4 = 0x9e;
    if (*(int *)(param_1 + 0x6b8) != 0) {
      iVar4 = 0x17;
    }
  }
  else {
    iVar4 = FUN_142a6fae0(param_1);
    *(int *)(param_1 + 0x2b8) = iVar4;
    if (iVar4 < *(int *)(param_1 + 0x2b4)) {
      iVar2 = 0x12;
      if (*(int *)(param_1 + 0x2a0) != 1 && *(int *)(param_1 + 0x2a0) != 2) {
        iVar2 = 8;
      }
      uVar5 = 1;
      if (0x7fffffff < (uint)((*(int *)(param_1 + 0x2b4) - iVar4) / iVar2)) goto LAB_142a6eaea;
    }
    else {
LAB_142a6eaea:
      uVar5 = 0;
    }
    *(uint *)(param_1 + 0x2c0) = uVar5;
    iVar4 = 0x5f;
  }
  *(uint *)(param_1 + 0x2ac) = iVar4 + (uVar5 ^ 1) * 4 + -4;
  uVar6 = 0x17;
  if (param_2 != 0) {
    uVar6 = 0x1e;
  }
  *(undefined4 *)(param_1 + 0x2b0) = uVar6;
  if (*(char *)(param_1 + 0x7a8) != '\0') {
    *(undefined4 *)(param_1 + 0x2ac) = *(undefined4 *)(param_1 + 0x7ac);
    *(undefined4 *)(param_1 + 0x2b0) = *(undefined4 *)(param_1 + 0x7b0);
  }
  uVar6 = FUN_142a6f710(param_1);
  *(undefined4 *)(param_1 + 0x2c4) = uVar6;
  if (param_2 == 0) {
    iVar4 = *(int *)(param_1 + 0x2b8) + 0x6a;
  }
  else {
    iVar4 = FUN_142a6f880(param_1);
  }
  *(int *)(param_1 + 0x2c8) = iVar4;
  if (1 < *(int *)(param_1 + 0x2a0) - 1U) {
    iVar4 = *(int *)(param_1 + 0x2a8);
    if (iVar4 == 2) {
      *(undefined4 *)(param_1 + 0x2b0) = 0x1e;
      uVar6 = FUN_142a6f880(param_1);
      *(undefined4 *)(param_1 + 0x2c8) = uVar6;
      iVar4 = *(int *)(param_1 + 0x2a8);
    }
    if (iVar4 != 0x13) {
      return;
    }
    *(undefined4 *)(param_1 + 0x2b0) = 0x1b;
    *(undefined4 *)(param_1 + 0x2ac) = 0x53;
    return;
  }
  if ((*(byte *)(param_1 + 0x6b4) & 8) == 0) {
    if (*(int *)(param_1 + 0x2a4) == 0) {
      uVar7 = 0xffffff91;
    }
    else {
      uVar7 = 0xffffff37;
      if (*(int *)(param_1 + 0x2a4) == 2) {
        uVar7 = 0xffffff4b;
      }
    }
  }
  *(undefined4 *)(param_1 + 0x2b0) = 0x12;
  *(uint *)(param_1 + 0x2ac) = uVar7 + 0x86;
  if (*(int *)(param_1 + 0x2a4) == 1) {
    *(undefined4 *)(param_1 + 0x2b0) = 0x19;
    iVar4 = uVar7 + 0xf2;
  }
  else {
    if (*(int *)(param_1 + 0x2a4) != 2) goto LAB_142a6ec1b;
    uVar6 = 0x2d;
    if (*(int *)(param_1 + 0x2a0) == 2) {
      uVar6 = 0x37;
    }
    *(undefined4 *)(param_1 + 0x2b0) = uVar6;
    cVar1 = FUN_142a11ae0();
    iVar4 = 0x101;
    if (cVar1 != '\0') {
      iVar4 = 0xf2;
    }
    iVar4 = iVar4 + uVar7;
  }
  *(int *)(param_1 + 0x2ac) = iVar4;
LAB_142a6ec1b:
  uVar6 = FUN_142a6f880(param_1);
  *(undefined4 *)(param_1 + 0x2c8) = uVar6;
  return;
}



//===========================================================
// FUN_142a8fe70 @ 142a8fe70   (253 bytes)
//===========================================================

void FUN_142a8fe70(longlong param_1)

{
  longlong lVar1;
  longlong lVar2;
  
  *(undefined1 *)(param_1 + 0x418) = 0xff;
  FUN_1402ca630(param_1 + 0x420);
  FUN_1402ca630(param_1 + 0x430);
  if (*(longlong **)(param_1 + 0x440) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x440) + 0x10))();
  }
  *(undefined8 *)(param_1 + 0x440) = 0;
  FUN_140d948a0(param_1 + 0x4a0);
  lVar1 = *(longlong *)(param_1 + 0x4b8);
  lVar2 = *(longlong *)(param_1 + 0x4b0);
  if (lVar2 != lVar1) {
    do {
      FUN_140cbb020(lVar2);
      lVar2 = lVar2 + 0x10;
    } while (lVar2 != lVar1);
    lVar2 = *(longlong *)(param_1 + 0x4b0);
  }
  *(longlong *)(param_1 + 0x4b8) = lVar2;
  lVar1 = *(longlong *)(param_1 + 0x4d0);
  lVar2 = *(longlong *)(param_1 + 0x4c8);
  if (lVar2 != lVar1) {
    do {
      FUN_140cbb020(lVar2);
      lVar2 = lVar2 + 0x10;
    } while (lVar2 != lVar1);
    lVar2 = *(longlong *)(param_1 + 0x4c8);
  }
  *(longlong *)(param_1 + 0x4d0) = lVar2;
  *(undefined4 *)(param_1 + 0x4e0) = 0;
  if (*(longlong *)(param_1 + 0x4e8) != 0) {
    thunk_FUN_140205820(*(longlong *)(param_1 + 0x4e8) + -8,0);
    *(undefined8 *)(param_1 + 0x4e8) = 0;
  }
  return;
}



//===========================================================
// FUN_1427be040 @ 1427be040   (71 bytes)
//===========================================================

longlong FUN_1427be040(longlong param_1,longlong param_2)

{
  longlong lVar1;
  
  lVar1 = *(longlong *)(param_1 + 0x13a0);
  *(longlong *)(param_2 + 8) = lVar1;
  if (lVar1 != 0) {
    if (0xfffff < *(ulonglong *)(lVar1 + 0x18)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar1 + 0x18) = *(longlong *)(lVar1 + 0x18) + 1;
    UNLOCK();
  }
  return param_2;
}



//===========================================================
// FUN_142e541f0 @ 142e541f0   (146 bytes)
//===========================================================

void FUN_142e541f0(undefined4 param_1,undefined8 param_2)

{
  char cVar1;
  undefined4 local_res8 [2];
  undefined8 local_res10;
  undefined4 local_res18 [2];
  longlong local_res20;
  
  local_res8[0] = param_1;
  local_res10 = param_2;
  cVar1 = FUN_142e559e0();
  if (cVar1 != '\0') {
    FUN_140194c60(&local_res20);
    local_res18[0] = FUN_14091a3e0(&local_res20);
    FUN_142e5cd30("LogCallStack4",&DAT_1434997dc,local_res18,&DAT_1434997f8,local_res8,"Info1",
                  &local_res10,&local_res20);
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_142a92740 @ 142a92740   (142 bytes)
//===========================================================

longlong FUN_142a92740(longlong param_1,longlong param_2)

{
  longlong lVar1;
  
  if ((*(longlong *)(param_1 + 8) - 1U < 999) || (*(longlong *)(param_1 + 8) == -1)) {
    FUN_142e52ed0(0x447);
  }
  if (param_1 == param_2) {
    FUN_142e52d50(0x45c,1);
  }
  lVar1 = *(longlong *)(param_2 + 8);
  if (lVar1 != 0) {
    if (0xfffff < *(ulonglong *)(lVar1 + 0x18)) {
      FUN_142e541f0(0x30f);
    }
    LOCK();
    *(longlong *)(lVar1 + 0x18) = *(longlong *)(lVar1 + 0x18) + 1;
    UNLOCK();
  }
  FUN_14287f2b0(param_1);
  *(undefined8 *)(param_1 + 8) = *(undefined8 *)(param_2 + 8);
  return param_1;
}



//===========================================================
// FUN_142a91f30 @ 142a91f30   (766 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000142a92143) */

void FUN_142a91f30(longlong param_1,int param_2,int param_3,undefined8 *param_4,longlong *param_5)

{
  undefined4 uVar1;
  undefined4 uVar2;
  uint *puVar3;
  longlong *plVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  undefined4 uVar8;
  undefined8 uVar9;
  undefined4 *puVar10;
  longlong lVar11;
  longlong lVar12;
  uint uVar13;
  undefined4 local_res8 [2];
  
  puVar3 = (uint *)*param_4;
  uVar13 = 0;
  uVar7 = uVar13;
  if ((puVar3 != (uint *)0x0) && (puVar3[-2] != 0)) {
    uVar7 = *puVar3;
  }
  if (0 < param_2) {
    iVar5 = FUN_140417c70(param_2);
    if (iVar5 != 0) {
      uVar6 = 100;
      goto LAB_142a91fcb;
    }
    uVar6 = FUN_1401a8170(param_2);
    if (uVar6 != 0) goto LAB_142a91fcb;
  }
  uVar6 = uVar13;
  if (0 < (int)uVar7) {
    iVar5 = FUN_1402538e0(uVar7);
    if (iVar5 == 0) {
      iVar5 = FUN_1402538a0(uVar7);
      if (iVar5 == 0) {
        iVar5 = FUN_140253930(uVar7);
        uVar6 = (uint)(iVar5 != 0);
      }
      else {
        uVar6 = 0xb;
      }
    }
    else {
      uVar6 = 0x15;
    }
  }
LAB_142a91fcb:
  *(uint *)(param_1 + 0x70) = uVar6;
  plVar4 = (longlong *)param_5[7];
  if (plVar4 != (longlong *)0x0) {
    (**(code **)(*plVar4 + 0x10))(plVar4,param_1);
  }
  if (*(longlong *)(param_1 + 0x10) != 0) {
    *(undefined4 *)(param_1 + 200) = 0;
    switch(param_3) {
    case 0x17:
    case 0x19:
    case 0x1c:
    case 0x1d:
      puVar10 = (undefined4 *)*param_4;
      if ((puVar10 != (undefined4 *)0x0) && (1 < (uint)puVar10[-2])) {
        uVar8 = *puVar10;
        if (param_3 - 0x1cU < 2) {
          uVar8 = 0x32;
        }
        uVar2 = puVar10[1];
        puVar10 = (undefined4 *)FUN_1401abb40(param_1 + 0xd0,0xffffffff);
        *puVar10 = uVar2;
        FUN_142a94840(param_1 + 0x78);
        uVar7 = FUN_1407386b0(&DAT_143ac1ab0);
        lVar12 = *(longlong *)(param_1 + 0x80);
        if (lVar12 == 0) {
          FUN_142e52ed0(0x428,0);
          lVar12 = *(longlong *)(param_1 + 0x80);
        }
        FUN_14041a250(lVar12,uVar7 & 7,(uVar7 & 7) + 1 & 0x80000007,uVar8);
      }
      break;
    case 0x18:
    case 0x1a:
      puVar10 = (undefined4 *)*param_4;
      if ((puVar10 != (undefined4 *)0x0) && (1 < (uint)puVar10[-2])) {
        uVar8 = *puVar10;
        uVar2 = puVar10[1];
        puVar10 = (undefined4 *)FUN_1401abb40(param_1 + 0xd0,0xffffffff);
        *puVar10 = uVar2;
        FUN_1402ca630(param_1 + 0x18);
        if (*(longlong *)(param_1 + 0x10) != 0) {
          FUN_1410b3850(param_1 + 0x18,local_res8);
          lVar12 = *(longlong *)(param_1 + 0x10);
          if (lVar12 == 0) {
            FUN_142e52ed0(0x428,0);
            lVar12 = *(longlong *)(param_1 + 0x10);
          }
          lVar11 = *(longlong *)(param_1 + 0x20);
          if (lVar11 == 0) {
            FUN_142e52ed0(0x428,0);
            lVar11 = *(longlong *)(param_1 + 0x20);
          }
          FUN_1402eeb50(lVar11,lVar12);
        }
        if (*(longlong *)(param_1 + 0x20) != 0) {
          uVar1 = *(undefined4 *)(param_1 + 0x70);
          uVar9 = FUN_142a927d0(param_1 + 0x18);
          FUN_1401a72a0(uVar9,uVar1,uVar8);
        }
        FUN_142a94840(param_1 + 0x78);
        uVar9 = FUN_142a92800(param_1 + 0x78);
        FUN_14041a360(uVar2,uVar9);
      }
      break;
    default:
      FUN_14022de90(param_1 + 0xd0,param_4);
    }
    iVar5 = *(int *)(param_1 + 0x70);
    if (((iVar5 == 0xd) || (iVar5 == 0x17)) && (*(longlong *)(param_1 + 0x10) != 0)) {
      local_res8[0] = 0;
      FUN_1401a7220(*(longlong *)(param_1 + 0x10),iVar5,local_res8);
      puVar10 = *(undefined4 **)(param_1 + 0xd0);
      if ((puVar10 != (undefined4 *)0x0) && (puVar10[-2] != 0)) {
        do {
          uVar8 = FUN_1401a8660(*(undefined4 *)(param_1 + 0x70),local_res8[0],*puVar10);
          *puVar10 = uVar8;
          if ((undefined4 *)
              (*(longlong *)(param_1 + 0xd0) + *(longlong *)(*(longlong *)(param_1 + 0xd0) + -8) * 4
              + -4) <= puVar10) break;
          puVar10 = puVar10 + 1;
        } while (puVar10 != (undefined4 *)0x0);
      }
    }
    FUN_142a91ad0(param_1);
  }
  plVar4 = (longlong *)param_5[7];
  if (plVar4 != (longlong *)0x0) {
    (**(code **)(*plVar4 + 0x20))(plVar4,plVar4 != param_5);
    param_5[7] = 0;
  }
  return;
}



//===========================================================
// FUN_142e52d50 @ 142e52d50   (119 bytes)
//===========================================================

void FUN_142e52d50(undefined4 param_1)

{
  char cVar1;
  undefined4 local_res8 [4];
  undefined4 local_res18 [2];
  longlong local_res20;
  
  local_res8[0] = param_1;
  cVar1 = FUN_142e559e0();
  if (cVar1 != '\0') {
    FUN_140194c60(&local_res20);
    local_res18[0] = FUN_14091a3e0(&local_res20);
    FUN_142e5d030("LogCallStack1",&DAT_1434997dc,local_res18,&DAT_1434997f8,local_res8,&local_res20)
    ;
    if (local_res20 != 0) {
      FUN_14019f2c0(local_res20 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_1401d3510 @ 1401d3510   (87 bytes)
//===========================================================

longlong FUN_1401d3510(longlong *param_1)

{
  void *pvVar1;
  longlong lVar2;
  bool bVar3;
  
  pvVar1 = Self;
  LOCK();
  lVar2 = *param_1;
  bVar3 = lVar2 == 0;
  if (bVar3) {
    *param_1 = (longlong)Self;
    lVar2 = 0;
  }
  UNLOCK();
  if (bVar3) {
LAB_1401d3561:
    *(undefined4 *)(param_1 + 1) = 1;
    return lVar2;
  }
  if ((void *)*param_1 != pvVar1) {
    while( true ) {
      pvVar1 = Self;
      LOCK();
      lVar2 = *param_1;
      bVar3 = lVar2 == 0;
      if (bVar3) {
        *param_1 = (longlong)Self;
        lVar2 = 0;
      }
      UNLOCK();
      if (bVar3) goto LAB_1401d3561;
      if ((void *)*param_1 == pvVar1) break;
      (*DAT_143262828)(0);
    }
  }
  *(int *)(param_1 + 1) = (int)param_1[1] + 1;
  return lVar2;
}



//===========================================================
// FUN_14019c870 @ 14019c870   (175 bytes)
//===========================================================

void FUN_14019c870(longlong *param_1,int param_2)

{
  longlong lVar1;
  int iVar2;
  ulonglong uVar3;
  
  lVar1 = *param_1;
  uVar3 = (ulonglong)param_2;
  if (*(int *)(lVar1 + -0x10) != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((param_2 == -1) || (param_2 <= *(int *)(lVar1 + -0xc))) {
    *(undefined4 *)(lVar1 + -0x10) = 1;
    if (param_2 != -1) goto LAB_14019c8ba;
    if (lVar1 != 0) {
      uVar3 = 0xffffffffffffffff;
      do {
        uVar3 = uVar3 + 1;
      } while (*(char *)(lVar1 + uVar3) != '\0');
      goto LAB_14019c8c1;
    }
    uVar3 = 0;
LAB_14019c8c5:
    iVar2 = (int)uVar3;
    if (iVar2 < *(int *)(lVar1 + -0xc) + 1) goto LAB_14019c8de;
  }
  else {
    FUN_142e54290(0x90,*(int *)(lVar1 + -0xc),param_2);
    *(undefined4 *)(lVar1 + -0x10) = 1;
LAB_14019c8ba:
    *(undefined1 *)(uVar3 + *param_1) = 0;
LAB_14019c8c1:
    if (-1 < (int)uVar3) goto LAB_14019c8c5;
  }
  iVar2 = (int)uVar3;
  FUN_142e54290(0x9c,uVar3 & 0xffffffff,*(undefined4 *)(lVar1 + -0xc));
LAB_14019c8de:
  *(int *)(lVar1 + -8) = iVar2;
  return;
}



//===========================================================
// FUN_1408bc3a0 @ 1408bc3a0   (90 bytes)
//===========================================================

undefined8 * FUN_1408bc3a0(undefined8 *param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  longlong local_res8 [4];
  
  local_res8[0] = 0;
  uVar1 = FUN_14019ba10(local_res8,"0x%p|",*param_2);
  *param_1 = 0;
  FUN_14019a260(param_1,uVar1);
  if (local_res8[0] != 0) {
    FUN_14019f2c0(local_res8[0] + -0x10);
  }
  return param_1;
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
// FUN_14019ba10 @ 14019ba10   (42 bytes)
//===========================================================

undefined8
FUN_14019ba10(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 local_res18;
  undefined8 local_res20;
  
  local_res18 = param_3;
  local_res20 = param_4;
  FUN_14019e480(param_1,param_2,&local_res18);
  return param_1;
}



//===========================================================
// FUN_142a1ec10 @ 142a1ec10   (61 bytes)
//===========================================================

void FUN_142a1ec10(undefined8 param_1)

{
  undefined8 uVar1;
  undefined8 local_res10;
  undefined8 *local_res18;
  
  local_res18 = &local_res10;
  local_res10 = 0;
  FUN_14019a260(&local_res10,param_1);
  uVar1 = FUN_142e56bc0();
  FUN_142a23830(uVar1,&local_res10);
  return;
}



//===========================================================
// FUN_14019b600 @ 14019b600   (375 bytes)
//===========================================================

void FUN_14019b600(longlong param_1,ulonglong param_2)

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
  uVar7 = 0x98;
  if (param_2 < 0x39) {
    uVar10 = (uint)(0x28 < param_2);
LAB_14019b65e:
    if ((int)uVar10 < 0) {
      FUN_14019d350(param_2);
      return;
    }
    if (uVar10 == 0) {
      iVar6 = 0x40;
      uVar7 = 0x28;
      goto LAB_14019b6a6;
    }
    if (uVar10 == 1) {
      iVar6 = 0x20;
      uVar7 = 0x38;
      goto LAB_14019b6a6;
    }
    if (uVar10 != 2) {
      if (uVar10 == 3) {
        iVar6 = 8;
      }
      else {
        iVar6 = 0;
        uVar7 = 0;
      }
      goto LAB_14019b6a6;
    }
  }
  else {
    if (0x58 < param_2) {
      uVar10 = 0xffffffff;
      if (param_2 < 0x99) {
        uVar10 = 3;
      }
      goto LAB_14019b65e;
    }
    uVar10 = 2;
  }
  iVar6 = 0x10;
  uVar7 = 0x58;
LAB_14019b6a6:
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
LAB_14019b709:
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
      if (lVar4 == 0) goto LAB_14019b709;
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
// FUN_1408bc140 @ 1408bc140   (90 bytes)
//===========================================================

undefined8 * FUN_1408bc140(undefined8 *param_1,undefined4 *param_2)

{
  undefined8 uVar1;
  longlong local_res8 [4];
  
  local_res8[0] = 0;
  uVar1 = FUN_14019ba10(local_res8,&DAT_1433005a0,*param_2);
  *param_1 = 0;
  FUN_14019a260(param_1,uVar1);
  if (local_res8[0] != 0) {
    FUN_14019f2c0(local_res8[0] + -0x10);
  }
  return param_1;
}



//===========================================================
// FUN_14091a3e0 @ 14091a3e0   (62 bytes)
//===========================================================

uint FUN_14091a3e0(undefined8 *param_1)

{
  byte bVar1;
  uint uVar2;
  byte *pbVar3;
  ulonglong uVar4;
  
  pbVar3 = (byte *)*param_1;
  if (pbVar3 != (byte *)0x0) {
    uVar2 = 0x811c9dc5;
    if (*(uint *)(pbVar3 + -8) != 0) {
      uVar4 = (ulonglong)*(uint *)(pbVar3 + -8);
      do {
        bVar1 = *pbVar3;
        pbVar3 = pbVar3 + 1;
        uVar2 = (bVar1 ^ uVar2) * 0x1000193;
        uVar4 = uVar4 - 1;
      } while (uVar4 != 0);
    }
    return uVar2;
  }
  return 0x811c9dc5;
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
// FUN_142e559e0 @ 142e559e0   (127 bytes)
//===========================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_142e559e0(void)

{
  char cVar1;
  int iVar2;
  undefined4 uVar3;
  
  iVar2 = FUN_14090d160(0x116,1);
  if (iVar2 != 0) {
    cVar1 = FUN_14090d340(0x11a);
    if (cVar1 != '\0') {
      _DAT_00000000 = 1;
    }
    if (DAT_143ae1514 < 3) {
      DAT_143ae1514 = DAT_143ae1514 + 1;
      return 1;
    }
    uVar3 = (*DAT_143262db0)();
    cVar1 = FUN_1408fcaa0(DAT_143ae1548,1800000,uVar3);
    if (cVar1 != '\0') {
      DAT_143ae1548 = uVar3;
      return 1;
    }
  }
  return 0;
}



//===========================================================
// FUN_1408bc980 @ 1408bc980   (208 bytes)
//===========================================================

undefined8 * FUN_1408bc980(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  undefined8 uVar2;
  int iVar3;
  char *pcVar4;
  longlong local_res10;
  
  pcVar4 = (char *)*param_2;
  if ((pcVar4 != (char *)0x0) && (0 < *(int *)(pcVar4 + -8))) {
    iVar1 = *(int *)(pcVar4 + -8);
    iVar3 = iVar1 + -1;
    if ((iVar3 < 0) || (*(int *)(pcVar4 + -8) <= iVar3)) {
      FUN_142e54290(0xcc,iVar3,*(undefined4 *)(pcVar4 + -8));
      pcVar4 = (char *)*param_2;
    }
    if (pcVar4[(longlong)iVar1 + -1] == '|') {
      *param_1 = 0;
      FUN_14019a260(param_1,param_2);
      return param_1;
    }
  }
  local_res10 = 0;
  if ((pcVar4 == (char *)0x0) || (*pcVar4 == '\0')) {
    pcVar4 = "";
  }
  uVar2 = FUN_14019ba10(&local_res10,&DAT_143272338,pcVar4);
  *param_1 = 0;
  FUN_14019a260(param_1,uVar2);
  if (local_res10 != 0) {
    FUN_14019f2c0(local_res10 + -0x10);
  }
  return param_1;
}



//===========================================================
// FUN_1408bc020 @ 1408bc020   (90 bytes)
//===========================================================

undefined8 * FUN_1408bc020(undefined8 *param_1,undefined4 *param_2)

{
  undefined8 uVar1;
  longlong local_res8 [4];
  
  local_res8[0] = 0;
  uVar1 = FUN_14019ba10(local_res8,&DAT_14330059c,*param_2);
  *param_1 = 0;
  FUN_14019a260(param_1,uVar1);
  if (local_res8[0] != 0) {
    FUN_14019f2c0(local_res8[0] + -0x10);
  }
  return param_1;
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
// FUN_14019a260 @ 14019a260   (485 bytes)
//===========================================================

longlong * FUN_14019a260(longlong *param_1,longlong *param_2)

{
  void *_Buf1;
  void *_Buf2;
  longlong lVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  int *piVar5;
  int *piVar6;
  int *piVar7;
  ulonglong uVar8;
  int iVar9;
  
  if (param_1 == param_2) {
    return param_1;
  }
  _Buf1 = (void *)*param_1;
  piVar7 = (int *)0x0;
  iVar9 = 0;
  iVar2 = iVar9;
  if (_Buf1 != (void *)0x0) {
    iVar2 = *(int *)((longlong)_Buf1 + -8);
  }
  _Buf2 = (void *)*param_2;
  iVar4 = iVar9;
  if (_Buf2 != (void *)0x0) {
    iVar4 = *(int *)((longlong)_Buf2 + -8);
  }
  if ((((iVar2 == iVar4) && (iVar2 != 0)) && (_Buf1 != (void *)0x0)) &&
     ((_Buf2 != (void *)0x0 && (iVar2 = memcmp(_Buf1,_Buf2,(longlong)iVar2), iVar2 == 0)))) {
    return param_1;
  }
  piVar5 = (int *)((longlong)_Buf2 + -0x10);
  if (_Buf2 == (void *)0x0) {
    piVar5 = piVar7;
  }
  if (piVar5 == (int *)0x0) {
    if (_Buf1 == (void *)0x0) {
      return param_1;
    }
    FUN_14019f2c0((longlong)_Buf1 + -0x10);
    *param_1 = 0;
    return param_1;
  }
  if (*piVar5 != -1) {
    if (*piVar5 < 1) {
      FUN_142e52dd0(0xd2);
    }
    LOCK();
    *piVar5 = *piVar5 + 1;
    UNLOCK();
    if (*param_1 != 0) {
      FUN_14019f2c0(*param_1 + -0x10);
    }
    *param_1 = (longlong)(piVar5 + 4);
    return param_1;
  }
  FUN_142e52d50(0xcb,0xffffff01);
  lVar1 = *param_2;
  piVar5 = piVar7;
  if (lVar1 == 0) goto LAB_14019a3c9;
  uVar8 = 0xffffffffffffffff;
  piVar6 = (int *)0xffffffffffffffff;
  do {
    piVar6 = (int *)((longlong)piVar6 + 1);
  } while (*(char *)(lVar1 + (longlong)piVar6) != '\0');
  iVar2 = (int)piVar6;
  if (0 < iVar2) {
    iVar9 = iVar2;
  }
  piVar3 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
  piVar3[1] = iVar9;
  *piVar3 = -1;
  piVar5 = piVar3 + 4;
  piVar3[2] = 0;
  *(undefined1 *)piVar5 = 0;
  FUN_142ef7ba0(piVar5,lVar1,(longlong)iVar2);
  if (*piVar3 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar2 == -1) || (iVar2 <= piVar3[1])) {
    *piVar3 = 1;
    if (iVar2 != -1) goto LAB_14019a3a6;
    if (piVar5 != (int *)0x0) {
      do {
        uVar8 = uVar8 + 1;
      } while (*(char *)((longlong)piVar5 + uVar8) != '\0');
      piVar7 = (int *)(uVar8 & 0xffffffff);
    }
  }
  else {
    FUN_142e54290(0x90,piVar3[1],(ulonglong)piVar6 & 0xffffffff);
    *piVar3 = 1;
LAB_14019a3a6:
    *(undefined1 *)((longlong)piVar5 + (longlong)iVar2) = 0;
    piVar7 = piVar6;
  }
  iVar2 = (int)piVar7;
  if ((iVar2 < 0) || (piVar3[1] + 1 <= iVar2)) {
    FUN_142e54290(0x9c,(ulonglong)piVar7 & 0xffffffff);
  }
  piVar3[2] = iVar2;
LAB_14019a3c9:
  if (*param_1 != 0) {
    FUN_14019f2c0(*param_1 + -0x10);
  }
  *param_1 = (longlong)piVar5;
  return param_1;
}



//===========================================================
// FUN_14019bd40 @ 14019bd40   (269 bytes)
//===========================================================

longlong FUN_14019bd40(longlong *param_1,int param_2,int param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  int *piVar4;
  int iVar5;
  int iVar6;
  
  piVar4 = (int *)(*param_1 + -0x10);
  if (*param_1 == 0) {
    piVar4 = (int *)0x0;
  }
  iVar6 = 0;
  if (piVar4 != (int *)0x0) {
    if ((*piVar4 < 2) && (param_2 <= piVar4[1])) {
      if (*piVar4 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar4 = -1;
      goto LAB_14019be37;
    }
    iVar6 = piVar4[2];
  }
  if (iVar6 < param_2) {
    iVar6 = param_2;
  }
  puVar1 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
  puVar1[1] = iVar6;
  *puVar1 = 0xffffffff;
  puVar2 = puVar1 + 4;
  *param_1 = (longlong)puVar2;
  if ((param_3 == 0) || (piVar4 == (int *)0x0)) {
    puVar1[2] = 0;
    *(undefined1 *)*param_1 = 0;
    if (piVar4 == (int *)0x0) goto LAB_14019be37;
  }
  else {
    iVar5 = iVar6 + 1;
    iVar3 = piVar4[2] + 1;
    if (iVar5 < iVar3) {
      FUN_142e54290(0x5c,iVar3,iVar5);
      puVar2 = (undefined4 *)*param_1;
      iVar3 = iVar5;
    }
    FUN_142ef7ba0(puVar2,piVar4 + 4,(longlong)iVar3);
    puVar1[2] = piVar4[2];
    *(undefined1 *)((longlong)iVar6 + *param_1) = 0;
  }
  FUN_14019f2c0(piVar4);
LAB_14019be37:
  return *param_1;
}



//===========================================================
// FUN_142a1d8a0 @ 142a1d8a0   (4667 bytes)
//===========================================================

ulonglong FUN_142a1d8a0(ulonglong param_1)

{
  longlong *plVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  int *piVar4;
  undefined4 *puVar5;
  longlong lVar6;
  int iVar7;
  longlong lVar8;
  int iVar9;
  int *piVar10;
  int iVar11;
  int *piVar12;
  ulonglong uVar13;
  int *piVar14;
  longlong lVar15;
  int iVar16;
  int *piVar17;
  longlong local_res10;
  undefined4 local_res18 [2];
  undefined4 local_res20;
  undefined4 uStackX_24;
  int *local_c0;
  longlong local_b8;
  undefined4 local_b0;
  undefined4 local_ac;
  undefined4 local_a8;
  undefined4 local_a4;
  longlong local_a0;
  longlong local_98;
  longlong local_90;
  longlong local_88;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  undefined8 local_68;
  longlong local_60;
  undefined8 local_58 [3];
  
  piVar17 = (int *)0x0;
  iVar11 = 0;
  uVar2 = FUN_142c4a030(DAT_143ac1898);
  local_res10 = CONCAT44(local_res10._4_4_,uVar2);
  local_68 = FUN_1408f6690();
  local_res18[0] = (*DAT_143262db0)();
  local_res20 = FUN_142c50c50();
  local_b0 = FUN_141892a90();
  uVar3 = FUN_1408fadb0(&local_60,&local_68);
  local_ac = 100;
  FUN_142a24230(param_1,"VERSION",&local_ac,"DATETIME",uVar3,&DAT_143487774,&local_b0,"LastUseName",
                &DAT_143adc708,"State",&local_res20,"Time1",local_res18,"Time2",&local_res10);
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  plVar1 = DAT_143aa84a0;
  if (DAT_143aa84a0 == (longlong *)0x0) {
    uVar3 = FUN_142a245b0(&local_res20,param_1,&DAT_143273ad4,&DAT_143271f04,"Channel",
                          &DAT_143271f04,&DAT_143273ac0,&DAT_143271f04,&DAT_143487790,&DAT_143271f04
                         );
    FUN_140319ad0(param_1,uVar3);
    if (CONCAT44(uStackX_24,local_res20) != 0) {
      FUN_14019f2c0(CONCAT44(uStackX_24,local_res20) + -0x10);
    }
    goto LAB_142a1ea24;
  }
  local_a4 = (**(code **)(*DAT_143aa84a0 + 0xa8))(DAT_143aa84a0);
  local_58[0] = FUN_142cb9610(plVar1);
  local_a8 = FUN_142cb9260(plVar1);
  local_res20 = FUN_142cb9230(plVar1);
  local_c0 = (int *)0x0;
  FUN_1408bc980(&local_88,param_1);
  lVar8 = local_88;
  piVar10 = piVar17;
  if (local_88 != 0) {
    iVar16 = *(int *)(local_88 + -8);
    piVar12 = (int *)(longlong)iVar16;
    if (iVar16 != 0) {
      iVar7 = 0;
      if (0 < iVar16) {
        iVar7 = iVar16;
      }
      piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar7 + 0x11));
      piVar4[1] = iVar7;
      *piVar4 = -1;
      piVar10 = piVar4 + 4;
      piVar4[2] = 0;
      *(char *)piVar10 = '\0';
      local_c0 = piVar10;
      FUN_142ef7ba0(piVar10,lVar8,piVar12);
      if (*piVar4 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar16 == -1) || (iVar16 <= piVar4[1])) {
        *piVar4 = 1;
        if (iVar16 != -1) goto LAB_142a1dab0;
        piVar12 = piVar17;
        if (piVar10 != (int *)0x0) {
          piVar12 = (int *)0xffffffffffffffff;
          do {
            piVar12 = (int *)((longlong)piVar12 + 1);
          } while (*(char *)((longlong)piVar10 + (longlong)piVar12) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar4[1],iVar16);
        *piVar4 = 1;
LAB_142a1dab0:
        *(char *)((longlong)piVar12 + (longlong)piVar10) = '\0';
      }
      iVar16 = (int)piVar12;
      if ((iVar16 < 0) || (piVar4[1] + 1 <= iVar16)) {
        FUN_142e54290(0x9c,(ulonglong)piVar12 & 0xffffffff);
      }
      piVar4[2] = iVar16;
    }
  }
  if (local_88 != 0) {
    FUN_14019f2c0(local_88 + -0x10);
  }
  local_80 = 0;
  uVar3 = FUN_14019ba10(&local_80,&DAT_143272338,&DAT_143273ad4);
  local_b8 = 0;
  FUN_14019a260(&local_b8,uVar3);
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  lVar8 = local_b8;
  piVar12 = piVar10;
  if (local_b8 != 0) {
    iVar16 = *(int *)(local_b8 + -8);
    piVar4 = (int *)(longlong)iVar16;
    if (iVar16 != 0) {
      piVar14 = piVar17;
      if (piVar10 == (int *)0x0) goto LAB_142a1dcdf;
      if ((char)*piVar10 != '\0') {
        iVar7 = piVar10[-2];
        for (iVar9 = piVar10[-3]; iVar9 < iVar7 + iVar16; iVar9 = iVar9 * 2) {
        }
        piVar17 = piVar10 + -4;
        if (piVar17 == (int *)0x0) {
LAB_142a1dbdf:
          if (iVar11 < iVar9) {
            iVar11 = iVar9;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
          puVar5[1] = iVar11;
          *puVar5 = 0xffffffff;
          piVar12 = puVar5 + 4;
          local_c0 = piVar12;
          if (piVar17 == (int *)0x0) {
            puVar5[2] = 0;
            *(char *)piVar12 = '\0';
          }
          else {
            iVar9 = piVar10[-2] + 1;
            if (iVar11 + 1 < iVar9) {
              FUN_142e54290(0x5c,iVar9,iVar11 + 1);
              iVar9 = iVar11 + 1;
            }
            FUN_142ef7ba0(piVar12,piVar10,(longlong)iVar9);
            puVar5[2] = piVar10[-2];
            *(char *)((longlong)iVar11 + (longlong)piVar12) = '\0';
            FUN_14019f2c0(piVar17);
          }
        }
        else {
          if ((1 < *piVar17) || (piVar10[-3] < iVar9)) {
            iVar11 = piVar10[-2];
            goto LAB_142a1dbdf;
          }
          if (*piVar17 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar17 = -1;
        }
        iVar11 = 0;
        if (piVar12 != (int *)0x0) {
          iVar11 = piVar12[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar11 + (longlong)piVar12),lVar8,piVar4);
        FUN_14019c870(&local_c0,iVar7 + iVar16);
        goto LAB_142a1dd80;
      }
      if ((piVar10 == (int *)0x0) || (piVar14 = piVar10 + -4, piVar14 == (int *)0x0)) {
LAB_142a1dcdf:
        if (iVar11 < iVar16) {
          iVar11 = iVar16;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
        puVar5[1] = iVar11;
        *puVar5 = 0xffffffff;
        piVar12 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar12 = '\0';
        local_c0 = piVar12;
        if (piVar14 != (int *)0x0) {
          FUN_14019f2c0(piVar14);
        }
      }
      else {
        if ((1 < *piVar14) || (piVar10[-3] < iVar16)) {
          iVar11 = piVar10[-2];
          goto LAB_142a1dcdf;
        }
        if (*piVar14 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar14 = -1;
      }
      FUN_142ef7ba0(piVar12,lVar8,piVar4);
      if (piVar12[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar16 == -1) || (iVar16 <= piVar12[-3])) {
        piVar12[-4] = 1;
        if (iVar16 != -1) goto LAB_142a1dd5d;
        if (piVar12 != (int *)0x0) {
          piVar17 = (int *)0xffffffffffffffff;
          do {
            piVar17 = (int *)((longlong)piVar17 + 1);
          } while (*(char *)((longlong)piVar12 + (longlong)piVar17) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar12[-3],iVar16);
        piVar12[-4] = 1;
LAB_142a1dd5d:
        *(char *)((longlong)piVar4 + (longlong)piVar12) = '\0';
        piVar17 = piVar4;
      }
      iVar11 = (int)piVar17;
      if ((iVar11 < 0) || (piVar12[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
      }
      piVar12[-2] = iVar11;
    }
  }
LAB_142a1dd80:
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  FUN_1408bc080(&local_a0,&local_res20);
  lVar8 = local_a0;
  if (local_a0 != 0) {
    iVar11 = *(int *)(local_a0 + -8);
    uVar13 = (ulonglong)iVar11;
    if (iVar11 != 0) {
      if (piVar12 == (int *)0x0) {
LAB_142a1df3d:
        piVar17 = (int *)0x0;
LAB_142a1df3f:
        iVar16 = 0;
LAB_142a1df41:
        if (iVar16 < iVar11) {
          iVar16 = iVar11;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
        puVar5[1] = iVar16;
        *puVar5 = 0xffffffff;
        piVar12 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar12 = '\0';
        local_c0 = piVar12;
        if (piVar17 != (int *)0x0) {
          FUN_14019f2c0(piVar17);
        }
      }
      else {
        if ((char)*piVar12 != '\0') {
          iVar16 = piVar12[-2];
          for (iVar7 = piVar12[-3]; iVar7 < iVar16 + iVar11; iVar7 = iVar7 * 2) {
          }
          piVar17 = piVar12 + -4;
          if (piVar17 == (int *)0x0) {
            iVar9 = 0;
LAB_142a1de4f:
            if (iVar9 < iVar7) {
              iVar9 = iVar7;
            }
            puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
            puVar5[1] = iVar9;
            *puVar5 = 0xffffffff;
            piVar10 = puVar5 + 4;
            local_c0 = piVar10;
            if (piVar17 == (int *)0x0) {
              puVar5[2] = 0;
              *(char *)piVar10 = '\0';
            }
            else {
              iVar7 = piVar12[-2] + 1;
              if (iVar9 + 1 < iVar7) {
                FUN_142e54290(0x5c,iVar7,iVar9 + 1);
                iVar7 = iVar9 + 1;
              }
              FUN_142ef7ba0(piVar10,piVar12,(longlong)iVar7);
              puVar5[2] = piVar12[-2];
              *(char *)((longlong)iVar9 + (longlong)piVar10) = '\0';
              FUN_14019f2c0(piVar17);
            }
          }
          else {
            if ((1 < *piVar17) || (piVar12[-3] < iVar7)) {
              iVar9 = piVar12[-2];
              goto LAB_142a1de4f;
            }
            if (*piVar17 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar17 = -1;
            piVar10 = piVar12;
          }
          if (piVar10 == (int *)0x0) {
            iVar7 = 0;
          }
          else {
            iVar7 = piVar10[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar7 + (longlong)piVar10),lVar8,uVar13);
          FUN_14019c870(&local_c0,iVar16 + iVar11);
          piVar12 = piVar10;
          goto LAB_142a1dfe9;
        }
        if (piVar12 == (int *)0x0) goto LAB_142a1df3d;
        piVar17 = piVar12 + -4;
        if (piVar17 == (int *)0x0) goto LAB_142a1df3f;
        if ((1 < *piVar17) || (piVar12[-3] < iVar11)) {
          iVar16 = piVar12[-2];
          goto LAB_142a1df41;
        }
        if (*piVar17 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar17 = -1;
      }
      FUN_142ef7ba0(piVar12,lVar8,uVar13);
      if (piVar12[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar11 == -1) || (iVar11 <= piVar12[-3])) {
        piVar12[-4] = 1;
        if (iVar11 != -1) goto LAB_142a1dfc2;
        if (piVar12 == (int *)0x0) {
          uVar13 = 0;
        }
        else {
          uVar13 = 0xffffffffffffffff;
          do {
            uVar13 = uVar13 + 1;
          } while (*(char *)((longlong)piVar12 + uVar13) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar12[-3],iVar11);
        piVar12[-4] = 1;
LAB_142a1dfc2:
        *(char *)(uVar13 + (longlong)piVar12) = '\0';
      }
      iVar11 = (int)uVar13;
      if ((iVar11 < 0) || (piVar12[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,uVar13 & 0xffffffff);
      }
      piVar12[-2] = iVar11;
    }
  }
LAB_142a1dfe9:
  if (local_a0 != 0) {
    FUN_14019f2c0(local_a0 + -0x10);
  }
  iVar16 = 0;
  piVar17 = (int *)0x0;
  iVar11 = 0;
  local_78 = 0;
  uVar3 = FUN_14019ba10(&local_78,&DAT_143272338,"Channel");
  local_b8 = 0;
  FUN_14019a260(&local_b8,uVar3);
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  lVar8 = local_b8;
  piVar10 = piVar12;
  if (local_b8 != 0) {
    iVar7 = *(int *)(local_b8 + -8);
    piVar4 = (int *)(longlong)iVar7;
    if (iVar7 != 0) {
      piVar14 = piVar17;
      if (piVar12 == (int *)0x0) goto LAB_142a1e1de;
      if ((char)*piVar12 != '\0') {
        iVar16 = piVar12[-2];
        for (iVar9 = piVar12[-3]; iVar9 < iVar16 + iVar7; iVar9 = iVar9 * 2) {
        }
        piVar17 = piVar12 + -4;
        if (piVar17 == (int *)0x0) {
LAB_142a1e0e9:
          if (iVar11 < iVar9) {
            iVar11 = iVar9;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar11 + 0x11));
          puVar5[1] = iVar11;
          *puVar5 = 0xffffffff;
          piVar10 = puVar5 + 4;
          local_c0 = piVar10;
          if (piVar17 == (int *)0x0) {
            puVar5[2] = 0;
            *(char *)piVar10 = '\0';
          }
          else {
            iVar9 = piVar12[-2] + 1;
            if (iVar11 + 1 < iVar9) {
              FUN_142e54290(0x5c,iVar9,iVar11 + 1);
              iVar9 = iVar11 + 1;
            }
            FUN_142ef7ba0(piVar10,piVar12,(longlong)iVar9);
            puVar5[2] = piVar12[-2];
            *(char *)((longlong)iVar11 + (longlong)piVar10) = '\0';
            FUN_14019f2c0(piVar17);
          }
        }
        else {
          if ((1 < *piVar17) || (piVar12[-3] < iVar9)) {
            iVar11 = piVar12[-2];
            goto LAB_142a1e0e9;
          }
          if (*piVar17 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar17 = -1;
        }
        if (piVar10 == (int *)0x0) {
          iVar11 = 0;
        }
        else {
          iVar11 = piVar10[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar11 + (longlong)piVar10),lVar8,piVar4);
        FUN_14019c870(&local_c0,iVar16 + iVar7);
        goto LAB_142a1e282;
      }
      if ((piVar12 == (int *)0x0) || (piVar14 = piVar12 + -4, piVar14 == (int *)0x0)) {
LAB_142a1e1de:
        if (iVar16 < iVar7) {
          iVar16 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
        puVar5[1] = iVar16;
        *puVar5 = 0xffffffff;
        piVar10 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar10 = '\0';
        local_c0 = piVar10;
        if (piVar14 != (int *)0x0) {
          FUN_14019f2c0(piVar14);
        }
      }
      else {
        if ((1 < *piVar14) || (piVar12[-3] < iVar7)) {
          iVar16 = piVar12[-2];
          goto LAB_142a1e1de;
        }
        if (*piVar14 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar14 = -1;
      }
      FUN_142ef7ba0(piVar10,lVar8,piVar4);
      if (piVar10[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar7 == -1) || (iVar7 <= piVar10[-3])) {
        piVar10[-4] = 1;
        if (iVar7 != -1) goto LAB_142a1e25f;
        if (piVar10 != (int *)0x0) {
          piVar17 = (int *)0xffffffffffffffff;
          do {
            piVar17 = (int *)((longlong)piVar17 + 1);
          } while (*(char *)((longlong)piVar10 + (longlong)piVar17) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar10[-3],iVar7);
        piVar10[-4] = 1;
LAB_142a1e25f:
        *(char *)((longlong)piVar4 + (longlong)piVar10) = '\0';
        piVar17 = piVar4;
      }
      iVar11 = (int)piVar17;
      if ((iVar11 < 0) || (piVar10[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
      }
      piVar10[-2] = iVar11;
    }
  }
LAB_142a1e282:
  piVar17 = (int *)0x0;
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  FUN_1408bc080(&local_98,&local_a8);
  lVar8 = local_98;
  if (local_98 != 0) {
    iVar11 = *(int *)(local_98 + -8);
    piVar12 = (int *)(longlong)iVar11;
    if (iVar11 != 0) {
      iVar16 = 0;
      piVar4 = piVar17;
      if (piVar10 == (int *)0x0) goto LAB_142a1e443;
      if ((char)*piVar10 == '\0') {
        if ((piVar10 == (int *)0x0) || (piVar4 = piVar10 + -4, piVar4 == (int *)0x0)) {
LAB_142a1e443:
          if (iVar16 < iVar11) {
            iVar16 = iVar11;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
          puVar5[1] = iVar16;
          *puVar5 = 0xffffffff;
          piVar10 = puVar5 + 4;
          puVar5[2] = 0;
          *(char *)piVar10 = '\0';
          local_c0 = piVar10;
          if (piVar4 != (int *)0x0) {
            FUN_14019f2c0(piVar4);
          }
        }
        else {
          if ((1 < *piVar4) || (piVar10[-3] < iVar11)) {
            iVar16 = piVar10[-2];
            goto LAB_142a1e443;
          }
          if (*piVar4 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar4 = -1;
        }
        FUN_142ef7ba0(piVar10,lVar8,piVar12);
        if (piVar10[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar11 == -1) || (iVar11 <= piVar10[-3])) {
          piVar10[-4] = 1;
          if (iVar11 != -1) goto LAB_142a1e4cb;
          if (piVar10 != (int *)0x0) {
            piVar17 = (int *)0xffffffffffffffff;
            do {
              piVar17 = (int *)((longlong)piVar17 + 1);
            } while (*(char *)((longlong)piVar10 + (longlong)piVar17) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar10[-3],iVar11);
          piVar10[-4] = 1;
LAB_142a1e4cb:
          *(char *)((longlong)piVar12 + (longlong)piVar10) = '\0';
          piVar17 = piVar12;
        }
        iVar11 = (int)piVar17;
        if ((iVar11 < 0) || (piVar10[-3] + 1 <= iVar11)) {
          FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
        }
        piVar10[-2] = iVar11;
        goto LAB_142a1e4f2;
      }
      iVar16 = piVar10[-2];
      for (iVar7 = piVar10[-3]; iVar7 < iVar16 + iVar11; iVar7 = iVar7 * 2) {
      }
      piVar17 = piVar10 + -4;
      if (piVar17 == (int *)0x0) {
        iVar9 = 0;
LAB_142a1e34f:
        if (iVar9 < iVar7) {
          iVar9 = iVar7;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar9 + 0x11));
        puVar5[1] = iVar9;
        *puVar5 = 0xffffffff;
        piVar4 = puVar5 + 4;
        local_c0 = piVar4;
        if (piVar17 == (int *)0x0) {
          puVar5[2] = 0;
          *(char *)piVar4 = '\0';
        }
        else {
          iVar7 = piVar10[-2] + 1;
          if (iVar9 + 1 < iVar7) {
            FUN_142e54290(0x5c,iVar7,iVar9 + 1);
            iVar7 = iVar9 + 1;
          }
          FUN_142ef7ba0(piVar4,piVar10,(longlong)iVar7);
          puVar5[2] = piVar10[-2];
          *(char *)((longlong)iVar9 + (longlong)piVar4) = '\0';
          FUN_14019f2c0(piVar17);
        }
      }
      else {
        if ((1 < *piVar17) || (piVar10[-3] < iVar7)) {
          iVar9 = piVar10[-2];
          goto LAB_142a1e34f;
        }
        if (*piVar17 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar17 = -1;
        piVar4 = piVar10;
      }
      if (piVar4 == (int *)0x0) {
        iVar7 = 0;
      }
      else {
        iVar7 = piVar4[-2];
      }
      FUN_142ef7ba0((char *)((longlong)iVar7 + (longlong)piVar4),lVar8,piVar12);
      FUN_14019c870(&local_c0,iVar16 + iVar11);
      piVar10 = piVar4;
    }
  }
LAB_142a1e4f2:
  if (local_98 != 0) {
    FUN_14019f2c0(local_98 + -0x10);
  }
  piVar17 = (int *)0x0;
  local_70 = 0;
  uVar3 = FUN_14019ba10(&local_70,&DAT_143272338,&DAT_143273ac0);
  local_b8 = 0;
  FUN_14019a260(&local_b8,uVar3);
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  lVar8 = local_b8;
  piVar12 = piVar10;
  if (local_b8 != 0) {
    iVar11 = *(int *)(local_b8 + -8);
    piVar4 = (int *)(longlong)iVar11;
    if (iVar11 != 0) {
      iVar16 = 0;
      piVar14 = piVar17;
      if (piVar10 == (int *)0x0) goto LAB_142a1e6fa;
      if ((char)*piVar10 != '\0') {
        iVar7 = piVar10[-2];
        for (iVar9 = piVar10[-3]; iVar9 < iVar7 + iVar11; iVar9 = iVar9 * 2) {
        }
        piVar17 = piVar10 + -4;
        if (piVar17 == (int *)0x0) {
LAB_142a1e600:
          if (iVar16 < iVar9) {
            iVar16 = iVar9;
          }
          puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
          puVar5[1] = iVar16;
          *puVar5 = 0xffffffff;
          piVar12 = puVar5 + 4;
          local_c0 = piVar12;
          if (piVar17 == (int *)0x0) {
            puVar5[2] = 0;
            *(char *)piVar12 = '\0';
          }
          else {
            iVar9 = piVar10[-2] + 1;
            if (iVar16 + 1 < iVar9) {
              FUN_142e54290(0x5c,iVar9,iVar16 + 1);
              iVar9 = iVar16 + 1;
            }
            FUN_142ef7ba0(piVar12,piVar10,(longlong)iVar9);
            puVar5[2] = piVar10[-2];
            *(char *)((longlong)iVar16 + (longlong)piVar12) = '\0';
            FUN_14019f2c0(piVar17);
          }
        }
        else {
          if ((1 < *piVar17) || (piVar10[-3] < iVar9)) {
            iVar16 = piVar10[-2];
            goto LAB_142a1e600;
          }
          if (*piVar17 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar17 = -1;
        }
        if (piVar12 == (int *)0x0) {
          iVar16 = 0;
        }
        else {
          iVar16 = piVar12[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar16 + (longlong)piVar12),lVar8,piVar4);
        FUN_14019c870(&local_c0,iVar7 + iVar11);
        goto LAB_142a1e7a5;
      }
      if ((piVar10 == (int *)0x0) || (piVar14 = piVar10 + -4, piVar14 == (int *)0x0)) {
LAB_142a1e6fa:
        if (iVar16 < iVar11) {
          iVar16 = iVar11;
        }
        puVar5 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar16 + 0x11));
        puVar5[1] = iVar16;
        *puVar5 = 0xffffffff;
        piVar12 = puVar5 + 4;
        puVar5[2] = 0;
        *(char *)piVar12 = '\0';
        local_c0 = piVar12;
        if (piVar14 != (int *)0x0) {
          FUN_14019f2c0(piVar14);
        }
      }
      else {
        if ((1 < *piVar14) || (piVar10[-3] < iVar11)) {
          iVar16 = piVar10[-2];
          goto LAB_142a1e6fa;
        }
        if (*piVar14 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar14 = -1;
      }
      FUN_142ef7ba0(piVar12,lVar8,piVar4);
      if (piVar12[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar11 == -1) || (iVar11 <= piVar12[-3])) {
        piVar12[-4] = 1;
        if (iVar11 != -1) goto LAB_142a1e782;
        piVar10 = (int *)0xffffffffffffffff;
        if (piVar12 != (int *)0x0) {
          do {
            piVar17 = (int *)((longlong)piVar10 + 1);
            piVar10 = piVar17;
          } while (*(char *)((longlong)piVar12 + (longlong)piVar17) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar12[-3],iVar11);
        piVar12[-4] = 1;
LAB_142a1e782:
        *(char *)((longlong)piVar4 + (longlong)piVar12) = '\0';
        piVar17 = piVar4;
      }
      iVar11 = (int)piVar17;
      if ((iVar11 < 0) || (piVar12[-3] + 1 <= iVar11)) {
        FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
      }
      piVar12[-2] = iVar11;
    }
  }
LAB_142a1e7a5:
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  FUN_1408bc4c0(&local_90,local_58);
  lVar8 = local_90;
  iVar11 = 0;
  if (local_90 != 0) {
    iVar16 = *(int *)(local_90 + -8);
    lVar15 = (longlong)iVar16;
    if (iVar16 != 0) {
      if ((piVar12 == (int *)0x0) || ((char)*piVar12 == '\0')) {
        uVar3 = FUN_14019bd40(&local_c0,iVar16,0);
        FUN_142ef7ba0(uVar3,lVar8,lVar15);
      }
      else {
        iVar16 = piVar12[-2] + iVar16;
        for (iVar7 = piVar12[-3]; iVar7 < iVar16; iVar7 = iVar7 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_c0,iVar7,1);
        iVar7 = iVar11;
        if (local_c0 != (int *)0x0) {
          iVar7 = local_c0[-2];
        }
        FUN_142ef7ba0(iVar7 + lVar6,lVar8,lVar15);
      }
      FUN_14019c870(&local_c0,iVar16);
    }
  }
  if (local_90 != 0) {
    FUN_14019f2c0(local_90 + -0x10);
  }
  FUN_1401d7f50(&local_c0,&DAT_143487790,param_1 & 0xff);
  FUN_1408bc080(&local_b8,&local_a4);
  lVar8 = local_b8;
  if (local_b8 != 0) {
    iVar16 = *(int *)(local_b8 + -8);
    lVar15 = (longlong)iVar16;
    if (iVar16 != 0) {
      if ((local_c0 == (int *)0x0) || ((char)*local_c0 == '\0')) {
        uVar3 = FUN_14019bd40(&local_c0,iVar16,0);
        FUN_142ef7ba0(uVar3,lVar8,lVar15);
      }
      else {
        iVar16 = local_c0[-2] + iVar16;
        for (iVar7 = local_c0[-3]; iVar7 < iVar16; iVar7 = iVar7 * 2) {
        }
        lVar6 = FUN_14019bd40(&local_c0,iVar7,1);
        if (local_c0 != (int *)0x0) {
          iVar11 = local_c0[-2];
        }
        FUN_142ef7ba0(iVar11 + lVar6,lVar8,lVar15);
      }
      FUN_14019c870(&local_c0,iVar16);
    }
  }
  if (local_b8 != 0) {
    FUN_14019f2c0(local_b8 + -0x10);
  }
  FUN_140319ad0(param_1,&local_c0);
  if (local_c0 != (int *)0x0) {
    FUN_14019f2c0(local_c0 + -4);
  }
LAB_142a1ea24:
  if (DAT_143ac18a0 == 0) {
    uVar3 = FUN_142a247e0(&local_res20,param_1,"Socket",&DAT_143271f04,&DAT_143271f04,&DAT_143271f04
                         );
    FUN_140319ad0(param_1,uVar3);
    lVar8 = CONCAT44(uStackX_24,local_res20);
  }
  else {
    local_res10 = 0;
    FUN_142a24da0(&local_res10,param_1,"Socket",DAT_143ac18a0 + 0x50,DAT_143ac18a0 + 0xc,
                  DAT_143ac18a0 + 0x154);
    FUN_140319ad0(param_1,&local_res10);
    lVar8 = local_res10;
  }
  if (lVar8 != 0) {
    FUN_14019f2c0(lVar8 + -0x10);
  }
  return param_1;
}



//===========================================================
// FUN_140194c60 @ 140194c60   (6267 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 * FUN_140194c60(undefined8 *param_1)

{
  code *pcVar1;
  undefined8 uVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  int iVar6;
  longlong *plVar7;
  int *piVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  ulonglong *puVar11;
  undefined4 *puVar12;
  longlong lVar13;
  longlong lVar14;
  char *pcVar15;
  undefined8 uVar16;
  int iVar17;
  int *piVar18;
  int *piVar19;
  int *piVar20;
  ulonglong uVar21;
  undefined8 *puVar22;
  int iVar23;
  int *piVar24;
  undefined1 auStack_aa8 [32];
  int **local_a88;
  int **local_a80;
  undefined8 local_a78;
  undefined8 local_a70;
  undefined8 local_a68;
  int *local_a58;
  longlong local_a50;
  undefined8 *local_a48;
  ulonglong local_a40;
  int local_a38;
  ulonglong local_a30;
  longlong local_a28;
  undefined8 *local_a20;
  undefined4 local_a18 [2];
  undefined8 local_a10;
  undefined8 uStack_a08;
  undefined8 local_a00;
  undefined8 uStack_9f8;
  undefined8 local_9e8;
  undefined1 local_9e0 [4];
  undefined4 local_9dc;
  longlong local_9c8;
  undefined4 local_9bc;
  undefined8 local_9b8;
  undefined4 local_9ac;
  undefined4 local_8d8;
  undefined8 local_8d4;
  undefined8 uStack_8cc;
  undefined8 local_8c4;
  undefined8 uStack_8bc;
  undefined8 local_8b4;
  undefined8 uStack_8ac;
  undefined8 local_8a4;
  undefined8 uStack_89c;
  undefined8 local_894;
  undefined8 uStack_88c;
  undefined8 local_884;
  undefined8 uStack_87c;
  undefined8 local_874;
  undefined8 uStack_86c;
  undefined8 local_864;
  undefined8 uStack_85c;
  undefined8 local_854;
  undefined8 uStack_84c;
  undefined1 local_838 [48];
  undefined4 local_808;
  undefined8 local_7a0;
  longlong local_798;
  undefined8 local_740;
  int *local_368 [34];
  undefined4 local_258 [6];
  undefined4 local_240;
  undefined1 local_23c [516];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_aa8;
  piVar24 = (int *)0x0;
  iVar3 = 0;
  local_a58 = (int *)0x0;
  local_a48 = param_1;
  local_a20 = param_1;
  if (DAT_143aa8288 == '\0') {
    local_a50 = 0;
    plVar7 = (longlong *)FUN_14019ba10(&local_a50,"Not Init\r\n");
    lVar14 = *plVar7;
    piVar18 = piVar24;
    if (lVar14 == 0) goto LAB_140194d86;
    iVar4 = *(int *)(lVar14 + -8);
    piVar20 = (int *)(longlong)iVar4;
    param_1 = local_a20;
    if (iVar4 == 0) goto LAB_140194d86;
    if (0 < iVar4) {
      iVar3 = iVar4;
    }
    piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
    piVar8[1] = iVar3;
    *piVar8 = -1;
    piVar18 = piVar8 + 4;
    piVar8[2] = 0;
    *(char *)piVar18 = '\0';
    local_a58 = piVar18;
    FUN_142ef7ba0(piVar18,lVar14,piVar20);
    if (*piVar8 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar4 == -1) || (iVar4 <= piVar8[1])) {
      *piVar8 = 1;
      if (iVar4 != -1) goto LAB_140194d5f;
      piVar20 = (int *)0xffffffffffffffff;
      if (piVar18 != (int *)0x0) {
        do {
          piVar24 = (int *)((longlong)piVar20 + 1);
          piVar20 = piVar24;
        } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,piVar8[1],iVar4);
      *piVar8 = 1;
LAB_140194d5f:
      *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
      piVar24 = piVar20;
    }
    iVar3 = (int)piVar24;
    if ((iVar3 < 0) || (piVar8[1] + 1 <= iVar3)) {
      FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
    }
    piVar8[2] = iVar3;
    param_1 = local_a20;
LAB_140194d86:
    if (local_a50 != 0) {
      FUN_14019f2c0(local_a50 + -0x10);
    }
    *param_1 = piVar18;
    return param_1;
  }
  FUN_142ef8250(local_838,0,0x4d0);
  local_808 = 0x10001f;
  (*DAT_143262840)(local_838);
  local_a50 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a50,&DAT_143271d00);
  lVar14 = *plVar7;
  piVar18 = piVar24;
  if (lVar14 != 0) {
    iVar4 = *(int *)(lVar14 + -8);
    piVar20 = (int *)(longlong)iVar4;
    piVar18 = (int *)0x0;
    if (iVar4 != 0) {
      iVar6 = 0;
      if (0 < iVar4) {
        iVar6 = iVar4;
      }
      piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar6 + 0x11));
      piVar8[1] = iVar6;
      *piVar8 = -1;
      piVar18 = piVar8 + 4;
      piVar8[2] = 0;
      *(char *)piVar18 = '\0';
      local_a58 = piVar18;
      FUN_142ef7ba0(piVar18,lVar14,piVar20);
      if (*piVar8 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar4 == -1) || (iVar4 <= piVar8[1])) {
        *piVar8 = 1;
        if (iVar4 != -1) goto LAB_140194ea0;
        piVar20 = piVar24;
        if (piVar18 != (int *)0x0) {
          piVar20 = (int *)0xffffffffffffffff;
          do {
            piVar20 = (int *)((longlong)piVar20 + 1);
          } while (*(char *)((longlong)piVar18 + (longlong)piVar20) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar8[1],iVar4);
        *piVar8 = 1;
LAB_140194ea0:
        *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
      }
      iVar4 = (int)piVar20;
      if ((iVar4 < 0) || (piVar8[1] + 1 <= iVar4)) {
        FUN_142e54290(0x9c,(ulonglong)piVar20 & 0xffffffff);
      }
      piVar8[2] = iVar4;
    }
  }
  if (local_a50 != 0) {
    FUN_14019f2c0(local_a50 + -0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Major:%d Minor:%d\r\n",1);
  lVar14 = *plVar7;
  piVar20 = piVar18;
  local_a50 = lVar14;
  if (lVar14 != 0) {
    iVar4 = *(int *)(lVar14 + -8);
    piVar8 = (int *)(longlong)iVar4;
    if (iVar4 != 0) {
      piVar19 = piVar24;
      if (piVar18 == (int *)0x0) goto LAB_14019509b;
      if ((char)*piVar18 != '\0') {
        iVar6 = piVar18[-2];
        for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
        }
        piVar24 = piVar18 + -4;
        if (piVar24 == (int *)0x0) {
LAB_140194f9f:
          if (iVar3 < iVar17) {
            iVar3 = iVar17;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
          puVar12[1] = iVar3;
          *puVar12 = 0xffffffff;
          piVar20 = puVar12 + 4;
          local_a58 = piVar20;
          if (piVar24 == (int *)0x0) {
            puVar12[2] = 0;
            *(char *)piVar20 = '\0';
            lVar14 = local_a50;
          }
          else {
            iVar17 = piVar18[-2] + 1;
            iVar23 = iVar3 + 1;
            if (iVar23 < iVar17) {
              FUN_142e54290(0x5c,iVar17,iVar23);
              iVar17 = iVar23;
            }
            FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
            puVar12[2] = piVar18[-2];
            *(char *)((longlong)iVar3 + (longlong)piVar20) = '\0';
            FUN_14019f2c0(piVar24);
            lVar14 = local_a50;
          }
        }
        else {
          if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
            iVar3 = piVar18[-2];
            goto LAB_140194f9f;
          }
          if (*piVar24 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar24 = -1;
        }
        if (piVar20 == (int *)0x0) {
          iVar3 = 0;
        }
        else {
          iVar3 = piVar20[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar20),lVar14,piVar8);
        FUN_14019c870(&local_a58,iVar6 + iVar4);
        goto LAB_140195140;
      }
      if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_14019509b:
        if (iVar3 < iVar4) {
          iVar3 = iVar4;
        }
        puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
        puVar12[1] = iVar3;
        *puVar12 = 0xffffffff;
        piVar20 = puVar12 + 4;
        puVar12[2] = 0;
        *(char *)piVar20 = '\0';
        local_a58 = piVar20;
        if (piVar19 != (int *)0x0) {
          FUN_14019f2c0(piVar19);
        }
      }
      else {
        if ((1 < *piVar19) || (piVar18[-3] < iVar4)) {
          iVar3 = piVar18[-2];
          goto LAB_14019509b;
        }
        if (*piVar19 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar19 = -1;
      }
      FUN_142ef7ba0(piVar20,lVar14,piVar8);
      if (piVar20[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar4 == -1) || (iVar4 <= piVar20[-3])) {
        piVar20[-4] = 1;
        if (iVar4 != -1) goto LAB_14019511d;
        if (piVar20 != (int *)0x0) {
          piVar24 = (int *)0xffffffffffffffff;
          do {
            piVar24 = (int *)((longlong)piVar24 + 1);
          } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar20[-3],iVar4);
        piVar20[-4] = 1;
LAB_14019511d:
        *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
        piVar24 = piVar8;
      }
      iVar3 = (int)piVar24;
      if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
        FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
      }
      piVar20[-2] = iVar3;
    }
  }
LAB_140195140:
  piVar24 = (int *)0x0;
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Call stack:\r\n");
  lVar14 = *plVar7;
  piVar18 = piVar20;
  local_a50 = lVar14;
  if (lVar14 != 0) {
    iVar3 = *(int *)(lVar14 + -8);
    piVar8 = (int *)(longlong)iVar3;
    if (iVar3 != 0) {
      iVar4 = 0;
      piVar19 = piVar24;
      if (piVar20 == (int *)0x0) goto LAB_14019530e;
      if ((char)*piVar20 != '\0') {
        iVar6 = piVar20[-2];
        for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
        }
        piVar24 = piVar20 + -4;
        if (piVar24 == (int *)0x0) {
LAB_140195212:
          if (iVar4 < iVar17) {
            iVar4 = iVar17;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
          puVar12[1] = iVar4;
          *puVar12 = 0xffffffff;
          piVar18 = puVar12 + 4;
          local_a58 = piVar18;
          if (piVar24 == (int *)0x0) {
            puVar12[2] = 0;
            *(char *)piVar18 = '\0';
            lVar14 = local_a50;
          }
          else {
            iVar17 = piVar20[-2] + 1;
            iVar23 = iVar4 + 1;
            if (iVar23 < iVar17) {
              FUN_142e54290(0x5c,iVar17,iVar23);
              iVar17 = iVar23;
            }
            FUN_142ef7ba0(piVar18,piVar20,(longlong)iVar17);
            puVar12[2] = piVar20[-2];
            *(char *)((longlong)iVar4 + (longlong)piVar18) = '\0';
            FUN_14019f2c0(piVar24);
            lVar14 = local_a50;
          }
        }
        else {
          if ((1 < *piVar24) || (piVar20[-3] < iVar17)) {
            iVar4 = piVar20[-2];
            goto LAB_140195212;
          }
          if (*piVar24 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar24 = -1;
        }
        if (piVar18 == (int *)0x0) {
          iVar4 = 0;
        }
        else {
          iVar4 = piVar18[-2];
        }
        FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar18),lVar14,piVar8);
        FUN_14019c870(&local_a58,iVar6 + iVar3);
        goto LAB_1401953b3;
      }
      if ((piVar20 == (int *)0x0) || (piVar19 = piVar20 + -4, piVar19 == (int *)0x0)) {
LAB_14019530e:
        if (iVar4 < iVar3) {
          iVar4 = iVar3;
        }
        puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
        puVar12[1] = iVar4;
        *puVar12 = 0xffffffff;
        piVar18 = puVar12 + 4;
        puVar12[2] = 0;
        *(char *)piVar18 = '\0';
        local_a58 = piVar18;
        if (piVar19 != (int *)0x0) {
          FUN_14019f2c0(piVar19);
        }
      }
      else {
        if ((1 < *piVar19) || (piVar20[-3] < iVar3)) {
          iVar4 = piVar20[-2];
          goto LAB_14019530e;
        }
        if (*piVar19 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar19 = -1;
      }
      FUN_142ef7ba0(piVar18,lVar14,piVar8);
      if (piVar18[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar3 == -1) || (iVar3 <= piVar18[-3])) {
        piVar18[-4] = 1;
        if (iVar3 != -1) goto LAB_140195390;
        if (piVar18 != (int *)0x0) {
          piVar24 = (int *)0xffffffffffffffff;
          do {
            piVar24 = (int *)((longlong)piVar24 + 1);
          } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
        }
      }
      else {
        FUN_142e54290(0x90,piVar18[-3],iVar3);
        piVar18[-4] = 1;
LAB_140195390:
        *(char *)((longlong)piVar8 + (longlong)piVar18) = '\0';
        piVar24 = piVar8;
      }
      iVar3 = (int)piVar24;
      if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
        FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
      }
      piVar18[-2] = iVar3;
    }
  }
LAB_1401953b3:
  piVar24 = (int *)0x0;
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  local_a40 = 0;
  plVar7 = (longlong *)FUN_14019ba10(&local_a40,"Address   Frame\r\n");
  lVar14 = *plVar7;
  piVar20 = piVar18;
  local_a50 = lVar14;
  if (lVar14 == 0) goto LAB_140195630;
  iVar3 = *(int *)(lVar14 + -8);
  piVar8 = (int *)(longlong)iVar3;
  if (iVar3 == 0) goto LAB_140195630;
  iVar4 = 0;
  piVar19 = piVar24;
  if (piVar18 == (int *)0x0) goto LAB_14019558b;
  if ((char)*piVar18 != '\0') {
    iVar6 = piVar18[-2];
    for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
    }
    piVar24 = piVar18 + -4;
    if (piVar24 == (int *)0x0) {
LAB_14019548f:
      if (iVar4 < iVar17) {
        iVar4 = iVar17;
      }
      puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
      puVar12[1] = iVar4;
      *puVar12 = 0xffffffff;
      piVar20 = puVar12 + 4;
      local_a58 = piVar20;
      if (piVar24 == (int *)0x0) {
        puVar12[2] = 0;
        *(char *)piVar20 = '\0';
        lVar14 = local_a50;
      }
      else {
        iVar17 = piVar18[-2] + 1;
        iVar23 = iVar4 + 1;
        if (iVar23 < iVar17) {
          FUN_142e54290(0x5c,iVar17,iVar23);
          iVar17 = iVar23;
        }
        FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
        puVar12[2] = piVar18[-2];
        *(char *)((longlong)iVar4 + (longlong)piVar20) = '\0';
        FUN_14019f2c0(piVar24);
        lVar14 = local_a50;
      }
    }
    else {
      if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
        iVar4 = piVar18[-2];
        goto LAB_14019548f;
      }
      if (*piVar24 != 1) {
        FUN_142e52dd0(0x74);
      }
      *piVar24 = -1;
    }
    if (piVar20 == (int *)0x0) {
      iVar4 = 0;
    }
    else {
      iVar4 = piVar20[-2];
    }
    FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar20),lVar14,piVar8);
    FUN_14019c870(&local_a58,iVar6 + iVar3);
    goto LAB_140195630;
  }
  if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_14019558b:
    if (iVar4 < iVar3) {
      iVar4 = iVar3;
    }
    puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
    puVar12[1] = iVar4;
    *puVar12 = 0xffffffff;
    piVar20 = puVar12 + 4;
    puVar12[2] = 0;
    *(char *)piVar20 = '\0';
    local_a58 = piVar20;
    if (piVar19 != (int *)0x0) {
      FUN_14019f2c0(piVar19);
    }
  }
  else {
    if ((1 < *piVar19) || (piVar18[-3] < iVar3)) {
      iVar4 = piVar18[-2];
      goto LAB_14019558b;
    }
    if (*piVar19 != 1) {
      FUN_142e52dd0(0x74);
    }
    *piVar19 = -1;
  }
  FUN_142ef7ba0(piVar20,lVar14,piVar8);
  if (piVar20[-4] != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((iVar3 == -1) || (iVar3 <= piVar20[-3])) {
    piVar20[-4] = 1;
    if (iVar3 != -1) goto LAB_14019560d;
    if (piVar20 != (int *)0x0) {
      piVar24 = (int *)0xffffffffffffffff;
      do {
        piVar24 = (int *)((longlong)piVar24 + 1);
      } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
    }
  }
  else {
    FUN_142e54290(0x90,piVar20[-3],iVar3);
    piVar20[-4] = 1;
LAB_14019560d:
    *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
    piVar24 = piVar8;
  }
  iVar3 = (int)piVar24;
  if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
    FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
  }
  piVar20[-2] = iVar3;
LAB_140195630:
  if (local_a40 != 0) {
    FUN_14019f2c0(local_a40 - 0x10);
  }
  FUN_142ef8250(local_9e0,0,0x100);
  uVar2 = DAT_143aa8260;
  uVar16 = DAT_143aa8258;
  pcVar1 = DAT_143aa8250;
  local_9e8 = local_740;
  local_9dc = 3;
  local_9b8 = local_7a0;
  local_9ac = 3;
  local_9c8 = local_798;
  local_9bc = 3;
  uVar9 = (*DAT_143ad5440)();
  uVar10 = (*DAT_143ad5408)();
  local_a68 = 0;
  local_a70 = uVar2;
  local_a78 = uVar16;
  local_a80 = (int **)0x0;
  local_a88 = (int **)local_838;
  iVar3 = (*pcVar1)(0x8664,uVar10,uVar9,&local_9e8);
  do {
    if ((iVar3 == 0) || (piVar24 = (int *)0x0, local_9c8 == 0)) {
      *local_a20 = piVar20;
      return local_a20;
    }
    local_a30 = 0;
    puVar11 = (ulonglong *)FUN_14019ba10(&local_a30,"%016X  %016X  ",local_9e8);
    uVar21 = *puVar11;
    piVar18 = piVar20;
    local_a40 = uVar21;
    if (uVar21 != 0) {
      iVar3 = *(int *)(uVar21 - 8);
      piVar8 = (int *)(longlong)iVar3;
      if (iVar3 != 0) {
        iVar4 = 0;
        piVar19 = piVar24;
        if (piVar20 == (int *)0x0) goto LAB_1401958b6;
        if ((char)*piVar20 != '\0') {
          iVar6 = piVar20[-2];
          for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar3; iVar17 = iVar17 * 2) {
          }
          piVar24 = piVar20 + -4;
          if (piVar24 == (int *)0x0) {
LAB_1401957c2:
            if (iVar4 < iVar17) {
              iVar4 = iVar17;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
            puVar12[1] = iVar4;
            *puVar12 = 0xffffffff;
            piVar18 = puVar12 + 4;
            local_a58 = piVar18;
            if (piVar24 == (int *)0x0) {
              puVar12[2] = 0;
              *(char *)piVar18 = '\0';
              uVar21 = local_a40;
            }
            else {
              iVar17 = piVar20[-2] + 1;
              iVar23 = iVar4 + 1;
              if (iVar23 < iVar17) {
                FUN_142e54290(0x5c,iVar17,iVar23);
                iVar17 = iVar23;
              }
              FUN_142ef7ba0(piVar18,piVar20,(longlong)iVar17);
              puVar12[2] = piVar20[-2];
              *(char *)((longlong)iVar4 + (longlong)piVar18) = '\0';
              FUN_14019f2c0(piVar24);
              uVar21 = local_a40;
            }
          }
          else {
            if ((1 < *piVar24) || (piVar20[-3] < iVar17)) {
              iVar4 = piVar20[-2];
              goto LAB_1401957c2;
            }
            if (*piVar24 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar24 = -1;
          }
          if (piVar18 == (int *)0x0) {
            iVar4 = 0;
          }
          else {
            iVar4 = piVar18[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar4 + (longlong)piVar18),uVar21,piVar8);
          FUN_14019c870(&local_a58,iVar6 + iVar3);
          goto LAB_14019595b;
        }
        if ((piVar20 == (int *)0x0) || (piVar19 = piVar20 + -4, piVar19 == (int *)0x0)) {
LAB_1401958b6:
          if (iVar4 < iVar3) {
            iVar4 = iVar3;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar4 + 0x11));
          puVar12[1] = iVar4;
          *puVar12 = 0xffffffff;
          piVar18 = puVar12 + 4;
          puVar12[2] = 0;
          *(char *)piVar18 = '\0';
          local_a58 = piVar18;
          if (piVar19 != (int *)0x0) {
            FUN_14019f2c0(piVar19);
          }
        }
        else {
          if ((1 < *piVar19) || (piVar20[-3] < iVar3)) {
            iVar4 = piVar20[-2];
            goto LAB_1401958b6;
          }
          if (*piVar19 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar19 = -1;
        }
        FUN_142ef7ba0(piVar18,uVar21,piVar8);
        if (piVar18[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar3 == -1) || (iVar3 <= piVar18[-3])) {
          piVar18[-4] = 1;
          if (iVar3 != -1) goto LAB_140195938;
          if (piVar18 != (int *)0x0) {
            piVar24 = (int *)0xffffffffffffffff;
            do {
              piVar24 = (int *)((longlong)piVar24 + 1);
            } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar18[-3],iVar3);
          piVar18[-4] = 1;
LAB_140195938:
          *(char *)((longlong)piVar8 + (longlong)piVar18) = '\0';
          piVar24 = piVar8;
        }
        iVar3 = (int)piVar24;
        if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
          FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
        }
        piVar18[-2] = iVar3;
      }
    }
LAB_14019595b:
    piVar24 = (int *)0x0;
    if (local_a30 != 0) {
      FUN_14019f2c0(local_a30 - 0x10);
    }
    local_258[0] = 0x20;
    local_240 = 0x200;
    local_a50 = 0;
    FUN_142ef8250(local_368,0,0x104);
    iVar3 = 0;
    local_a40 = local_a40 & 0xffffffff00000000;
    local_a30 = local_a30 & 0xffffffff00000000;
    local_a18[0] = 0x28;
    local_a10 = 0;
    local_a00 = 0;
    uStack_9f8 = 0;
    uStack_a08 = 0xffffffff;
    if (DAT_143aa8280 == 0) {
      local_8d8 = 0x94;
      local_8d4 = 0;
      uStack_8cc = 0;
      local_8c4 = 0;
      uStack_8bc = 0;
      local_8b4 = 0;
      uStack_8ac = 0;
      local_8a4 = 0;
      uStack_89c = 0;
      local_894 = 0;
      uStack_88c = 0;
      local_884 = 0;
      uStack_87c = 0;
      local_874 = 0;
      uStack_86c = 0;
      local_864 = 0;
      uStack_85c = 0;
      local_854 = 0;
      uStack_84c = 0;
      (*DAT_143262820)(&local_8d8);
      if (uStack_8cc._4_4_ == 2) {
        DAT_143aa8280 = (*DAT_143ad5408)();
      }
      else {
        iVar4 = (*DAT_143ad5410)();
        DAT_143aa8280 = (longlong)iVar4;
      }
      if (DAT_143aa8280 == 0) {
        local_a28 = 0;
        plVar7 = (longlong *)FUN_14019ba10(&local_a28,"m_hProcess is Null\r\n");
        puVar22 = (undefined8 *)*plVar7;
        piVar20 = piVar18;
        local_a48 = puVar22;
        if (puVar22 == (undefined8 *)0x0) goto LAB_140196462;
        iVar4 = *(int *)(puVar22 + -1);
        piVar8 = (int *)(longlong)iVar4;
        if (iVar4 == 0) goto LAB_140196462;
        piVar19 = piVar24;
        if (piVar18 == (int *)0x0) goto LAB_1401963be;
        if ((char)*piVar18 != '\0') {
          iVar6 = piVar18[-2];
          for (iVar17 = piVar18[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
          }
          piVar24 = piVar18 + -4;
          if (piVar24 == (int *)0x0) {
LAB_1401962c5:
            if (iVar3 < iVar17) {
              iVar3 = iVar17;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
            puVar12[1] = iVar3;
            *puVar12 = 0xffffffff;
            piVar20 = puVar12 + 4;
            local_a58 = piVar20;
            if (piVar24 == (int *)0x0) {
              puVar12[2] = 0;
              *(char *)piVar20 = '\0';
              puVar22 = local_a48;
            }
            else {
              iVar17 = piVar18[-2] + 1;
              iVar23 = iVar3 + 1;
              if (iVar23 < iVar17) {
                FUN_142e54290(0x5c,iVar17,iVar23);
                iVar17 = iVar23;
              }
              FUN_142ef7ba0(piVar20,piVar18,(longlong)iVar17);
              puVar12[2] = piVar18[-2];
              *(char *)((longlong)iVar3 + (longlong)piVar20) = '\0';
              FUN_14019f2c0(piVar24);
              puVar22 = local_a48;
            }
          }
          else {
            if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
              iVar3 = piVar18[-2];
              goto LAB_1401962c5;
            }
            if (*piVar24 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar24 = -1;
          }
          iVar3 = 0;
          if (piVar20 != (int *)0x0) {
            iVar3 = piVar20[-2];
          }
          FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar20),puVar22,piVar8);
          FUN_14019c870(&local_a58,iVar6 + iVar4);
          goto LAB_140196462;
        }
        if ((piVar18 == (int *)0x0) || (piVar19 = piVar18 + -4, piVar19 == (int *)0x0)) {
LAB_1401963be:
          if (iVar3 < iVar4) {
            iVar3 = iVar4;
          }
          puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
          puVar12[1] = iVar3;
          *puVar12 = 0xffffffff;
          piVar20 = puVar12 + 4;
          puVar12[2] = 0;
          *(char *)piVar20 = '\0';
          local_a58 = piVar20;
          if (piVar19 != (int *)0x0) {
            FUN_14019f2c0(piVar19);
          }
        }
        else {
          if ((1 < *piVar19) || (piVar18[-3] < iVar4)) {
            iVar3 = piVar18[-2];
            goto LAB_1401963be;
          }
          if (*piVar19 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar19 = -1;
        }
        FUN_142ef7ba0(piVar20,puVar22,piVar8);
        if (piVar20[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar4 == -1) || (iVar4 <= piVar20[-3])) {
          piVar20[-4] = 1;
          if (iVar4 == -1) {
            piVar18 = (int *)0xffffffffffffffff;
            if (piVar20 != (int *)0x0) {
              do {
                piVar24 = (int *)((longlong)piVar18 + 1);
                piVar18 = piVar24;
              } while (*(char *)((longlong)piVar20 + (longlong)piVar24) != '\0');
            }
LAB_140196443:
            iVar3 = (int)piVar24;
            if ((iVar3 < 0) || (piVar20[-3] + 1 <= iVar3)) {
              FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
            }
            piVar20[-2] = iVar3;
LAB_140196462:
            if (local_a28 != 0) {
              FUN_14019f2c0(local_a28 + -0x10);
            }
            *local_a20 = piVar20;
            return local_a20;
          }
        }
        else {
          FUN_142e54290(0x90,piVar20[-3],iVar4);
          piVar20[-4] = 1;
        }
        *(char *)((longlong)piVar8 + (longlong)piVar20) = '\0';
        piVar24 = piVar8;
        goto LAB_140196443;
      }
    }
    iVar4 = (*DAT_143aa8268)(DAT_143aa8280,local_9e8,&local_a50,local_258);
    local_a38 = iVar4;
    if (iVar4 == 0) {
      uVar5 = (*DAT_143262838)();
      local_a48 = (undefined8 *)0x0;
      plVar7 = (longlong *)FUN_14019ba10(&local_a48,"_SymGetLineFromAddr Error : %x ",uVar5);
      lVar14 = *plVar7;
      local_a28 = lVar14;
      if (lVar14 != 0) {
        iVar6 = *(int *)(lVar14 + -8);
        piVar20 = (int *)(longlong)iVar6;
        iVar4 = local_a38;
        if (iVar6 != 0) {
          piVar8 = piVar24;
          if (piVar18 == (int *)0x0) goto LAB_140195c6d;
          if ((char)*piVar18 != '\0') {
            iVar4 = piVar18[-2];
            for (iVar17 = piVar18[-3]; iVar17 < iVar4 + iVar6; iVar17 = iVar17 * 2) {
            }
            piVar24 = piVar18 + -4;
            if (piVar24 == (int *)0x0) {
LAB_140195b6f:
              if (iVar3 < iVar17) {
                iVar3 = iVar17;
              }
              puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
              puVar12[1] = iVar3;
              *puVar12 = 0xffffffff;
              piVar8 = puVar12 + 4;
              local_a58 = piVar8;
              if (piVar24 == (int *)0x0) {
                puVar12[2] = 0;
                *(char *)piVar8 = '\0';
                lVar14 = local_a28;
              }
              else {
                iVar17 = piVar18[-2] + 1;
                iVar23 = iVar3 + 1;
                if (iVar23 < iVar17) {
                  FUN_142e54290(0x5c,iVar17,iVar23);
                  iVar17 = iVar23;
                }
                FUN_142ef7ba0(piVar8,piVar18,(longlong)iVar17);
                puVar12[2] = piVar18[-2];
                *(char *)((longlong)iVar3 + (longlong)piVar8) = '\0';
                FUN_14019f2c0(piVar24);
                lVar14 = local_a28;
              }
            }
            else {
              if ((1 < *piVar24) || (piVar18[-3] < iVar17)) {
                iVar3 = piVar18[-2];
                goto LAB_140195b6f;
              }
              if (*piVar24 != 1) {
                FUN_142e52dd0(0x74);
              }
              *piVar24 = -1;
              piVar8 = piVar18;
            }
            if (piVar8 == (int *)0x0) {
              iVar3 = 0;
            }
            else {
              iVar3 = piVar8[-2];
            }
            FUN_142ef7ba0((char *)((longlong)iVar3 + (longlong)piVar8),lVar14,piVar20);
            FUN_14019c870(&local_a58,iVar4 + iVar6);
            iVar4 = local_a38;
            goto LAB_140195d1d;
          }
          if ((piVar18 == (int *)0x0) || (piVar8 = piVar18 + -4, piVar8 == (int *)0x0)) {
LAB_140195c6d:
            if (iVar3 < iVar6) {
              iVar3 = iVar6;
            }
            puVar12 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
            puVar12[1] = iVar3;
            *puVar12 = 0xffffffff;
            piVar18 = puVar12 + 4;
            puVar12[2] = 0;
            *(char *)piVar18 = '\0';
            local_a58 = piVar18;
            if (piVar8 != (int *)0x0) {
              FUN_14019f2c0(piVar8);
            }
          }
          else {
            if ((1 < *piVar8) || (piVar18[-3] < iVar6)) {
              iVar3 = piVar18[-2];
              goto LAB_140195c6d;
            }
            if (*piVar8 != 1) {
              FUN_142e52dd0(0x74);
            }
            *piVar8 = -1;
          }
          FUN_142ef7ba0(piVar18,lVar14,piVar20);
          if (piVar18[-4] != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar6 == -1) || (iVar6 <= piVar18[-3])) {
            piVar18[-4] = 1;
            if (iVar6 != -1) goto LAB_140195cf6;
            if (piVar18 != (int *)0x0) {
              piVar24 = (int *)0xffffffffffffffff;
              do {
                piVar24 = (int *)((longlong)piVar24 + 1);
              } while (*(char *)((longlong)piVar18 + (longlong)piVar24) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar18[-3],iVar6);
            piVar18[-4] = 1;
LAB_140195cf6:
            *(char *)((longlong)piVar20 + (longlong)piVar18) = '\0';
            piVar24 = piVar20;
          }
          iVar3 = (int)piVar24;
          if ((iVar3 < 0) || (piVar18[-3] + 1 <= iVar3)) {
            FUN_142e54290(0x9c,(ulonglong)piVar24 & 0xffffffff);
          }
          piVar18[-2] = iVar3;
          iVar4 = local_a38;
        }
      }
LAB_140195d1d:
      if (local_a48 != (undefined8 *)0x0) {
        FUN_14019f2c0(local_a48 + -2);
      }
    }
    else if (DAT_143aa8270 != (code *)0x0) {
      (*DAT_143aa8270)(DAT_143aa8280,local_9e8,&local_a50,local_a18);
    }
    local_a80 = &local_a58;
    local_a88 = (int **)&local_a30;
    iVar6 = FUN_140194a90(local_9e8,local_368,0x104,&local_a40);
    local_a48 = (undefined8 *)0x0;
    iVar3 = 0;
    if ((int)uStack_a08 == -1) {
      if (iVar4 == 0) {
        local_a88 = local_368;
        plVar7 = (longlong *)
                 FUN_14019ba10(&local_a48,"%04X:%08X [%s]",local_a40 & 0xffffffff,
                               local_a30 & 0xffffffff);
        lVar14 = *plVar7;
        piVar20 = local_a58;
        if (lVar14 != 0) {
          iVar4 = *(int *)(lVar14 + -8);
          if (iVar4 != 0) {
            if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
              uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
              FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar4);
              piVar20 = local_a58;
            }
            else {
              iVar17 = local_a58[-2];
              for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
              }
              lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
              piVar20 = local_a58;
              iVar23 = iVar3;
              if (local_a58 != (int *)0x0) {
                iVar23 = local_a58[-2];
              }
              FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar17 + iVar4);
            }
          }
        }
      }
      else {
        local_a88 = local_368;
        plVar7 = (longlong *)FUN_14019ba10(&local_a48,"%hs()+%X [%s]",local_23c,local_a50);
        lVar14 = *plVar7;
        piVar20 = local_a58;
        if (lVar14 != 0) {
          iVar4 = *(int *)(lVar14 + -8);
          if (iVar4 != 0) {
            if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
              uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
              FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar4);
              piVar20 = local_a58;
            }
            else {
              iVar17 = local_a58[-2];
              for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
              }
              lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
              piVar20 = local_a58;
              iVar23 = iVar3;
              if (local_a58 != (int *)0x0) {
                iVar23 = local_a58[-2];
              }
              FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
              FUN_14019c870(&local_a58,iVar17 + iVar4);
            }
          }
        }
      }
    }
    else {
      local_a80 = local_368;
      local_a88 = (int **)CONCAT44(local_a88._4_4_,(int)uStack_a08);
      plVar7 = (longlong *)FUN_14019ba10(&local_a48,"%hs() %hs(%lu) [%s]",local_23c,local_a00);
      lVar14 = *plVar7;
      piVar20 = local_a58;
      if (lVar14 != 0) {
        iVar4 = *(int *)(lVar14 + -8);
        if (iVar4 != 0) {
          if ((local_a58 == (int *)0x0) || ((char)*local_a58 == '\0')) {
            uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
            FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
            FUN_14019c870(&local_a58,iVar4);
            piVar20 = local_a58;
          }
          else {
            iVar17 = local_a58[-2];
            for (iVar23 = local_a58[-3]; iVar23 < iVar17 + iVar4; iVar23 = iVar23 * 2) {
            }
            lVar13 = FUN_14019bd40(&local_a58,iVar23,1);
            piVar20 = local_a58;
            iVar23 = iVar3;
            if (local_a58 != (int *)0x0) {
              iVar23 = local_a58[-2];
            }
            FUN_142ef7ba0(iVar23 + lVar13,lVar14,(longlong)iVar4);
            FUN_14019c870(&local_a58,iVar17 + iVar4);
          }
        }
      }
    }
    if (local_a48 != (undefined8 *)0x0) {
      FUN_14019f2c0(local_a48 + -2);
    }
    if (iVar6 == 0) {
      if ((piVar20 == (int *)0x0) || ((char)*piVar20 == '\0')) {
        pcVar15 = (char *)FUN_14019bd40(&local_a58,0xc);
        *(undefined8 *)pcVar15 = s_ErrorOccered_143272138._0_8_;
        *(undefined4 *)(pcVar15 + 8) = s_ErrorOccered_143272138._8_4_;
        FUN_14019c870(&local_a58,0xc);
        piVar20 = local_a58;
      }
      else {
        iVar4 = piVar20[-2];
        for (iVar6 = piVar20[-3]; iVar6 < iVar4 + 0xc; iVar6 = iVar6 * 2) {
        }
        lVar14 = FUN_14019bd40(&local_a58,iVar6,1);
        piVar20 = local_a58;
        iVar6 = iVar3;
        if (local_a58 != (int *)0x0) {
          iVar6 = local_a58[-2];
        }
        *(undefined8 *)(iVar6 + lVar14) = s_ErrorOccered_143272138._0_8_;
        *(undefined4 *)((longlong)iVar6 + 8 + lVar14) = s_ErrorOccered_143272138._8_4_;
        FUN_14019c870(&local_a58,iVar4 + 0xc);
      }
    }
    local_a48 = (undefined8 *)0x0;
    plVar7 = (longlong *)FUN_14019ba10(&local_a48,&DAT_143271d00);
    lVar14 = *plVar7;
    if (lVar14 != 0) {
      iVar4 = *(int *)(lVar14 + -8);
      if (iVar4 != 0) {
        if ((piVar20 == (int *)0x0) || ((char)*piVar20 == '\0')) {
          uVar16 = FUN_14019bd40(&local_a58,iVar4,0);
          FUN_142ef7ba0(uVar16,lVar14,(longlong)iVar4);
          FUN_14019c870(&local_a58,iVar4);
          piVar20 = local_a58;
        }
        else {
          iVar6 = piVar20[-2];
          for (iVar17 = piVar20[-3]; iVar17 < iVar6 + iVar4; iVar17 = iVar17 * 2) {
          }
          lVar13 = FUN_14019bd40(&local_a58,iVar17,1);
          piVar20 = local_a58;
          if (local_a58 != (int *)0x0) {
            iVar3 = local_a58[-2];
          }
          FUN_142ef7ba0(iVar3 + lVar13,lVar14,(longlong)iVar4);
          FUN_14019c870(&local_a58,iVar6 + iVar4);
        }
      }
    }
    if (local_a48 != (undefined8 *)0x0) {
      FUN_14019f2c0(local_a48 + -2);
    }
    uVar2 = DAT_143aa8260;
    uVar16 = DAT_143aa8258;
    pcVar1 = DAT_143aa8250;
    uVar9 = (*DAT_143ad5440)();
    uVar10 = (*DAT_143ad5408)();
    local_a68 = 0;
    local_a70 = uVar2;
    local_a78 = uVar16;
    local_a80 = (int **)0x0;
    local_a88 = (int **)local_838;
    iVar3 = (*pcVar1)(0x8664,uVar10,uVar9,&local_9e8);
  } while( true );
}



//===========================================================
// FUN_142a11ae0 @ 142a11ae0   (51 bytes)
//===========================================================

bool FUN_142a11ae0(void)

{
  undefined1 uVar1;
  int iVar2;
  
  iVar2 = FUN_1410a3e40();
  if (iVar2 == 1) {
    iVar2 = FUN_1410a4dc0();
    return iVar2 < 0x400;
  }
  uVar1 = FUN_1410a3fe0(800,600);
  return (bool)uVar1;
}



//===========================================================
// FUN_142a6f710 @ 142a6f710   (288 bytes)
//===========================================================

undefined8 FUN_142a6f710(longlong param_1)

{
  int iVar1;
  char cVar2;
  undefined8 uVar3;
  
  if (1 < *(int *)(param_1 + 0x2a0) - 1U) {
    switch(*(undefined4 *)(param_1 + 0x2a8)) {
    case 0:
    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
    case 7:
    case 0xb:
    case 0xc:
    case 0x10:
      uVar3 = 0x211;
      if (*(int *)(param_1 + 0x2d0) != 0) {
        uVar3 = 0x10a;
      }
      return uVar3;
    case 8:
      return 0x1a3;
    case 9:
    case 10:
    case 0xe:
    case 0xf:
      goto switchD_142a6f79f_caseD_9;
    case 0xd:
      return 0x1a8;
    case 0x11:
      return 0x211;
    case 0x12:
      return 0x11f;
    case 0x13:
    case 0x15:
      return 0x16f;
    case 0x14:
      return 0x1a7;
    case 0x16:
    case 0x17:
    case 0x18:
    case 0x19:
    case 0x1a:
    case 0x1b:
    case 0x1c:
    case 0x1d:
      return 0x1a1;
    default:
      return 0;
    }
  }
  cVar2 = FUN_142a11ae0();
  iVar1 = *(int *)(param_1 + 0x2a4);
  if (cVar2 == '\0') {
    uVar3 = 0x38e;
    if (iVar1 != 2) {
      uVar3 = 0x345;
    }
    return uVar3;
  }
  if (iVar1 == 1) {
    return 0x31d;
  }
  if (iVar1 == 2) {
    return 0x339;
  }
  return 0x2ff;
switchD_142a6f79f_caseD_9:
  if (*(longlong *)(param_1 + 0x138) == 0) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(0x80004003);
  }
  uVar3 = FUN_140404280();
  return uVar3;
}



//===========================================================
// FUN_142a6fba0 @ 142a6fba0   (104 bytes)
//===========================================================

undefined8 FUN_142a6fba0(longlong param_1)

{
  undefined8 uVar1;
  
  if (*(int *)(param_1 + 0x2a0) == 2) {
    return 0xd6;
  }
  if (*(int *)(param_1 + 0x2a0) == 1) {
    return 0x6e;
  }
  switch(*(undefined4 *)(param_1 + 0x2a8)) {
  case 0:
  case 1:
  case 2:
  case 3:
  case 4:
  case 5:
  case 6:
  case 7:
  case 0xb:
  case 0xc:
  case 0xe:
  case 0x10:
    break;
  default:
    return 0;
  case 0xd:
    return 300;
  case 0x11:
    return 0x17;
  }
  uVar1 = 0x6e;
  if (*(int *)(param_1 + 0x2d0) != 0) {
    uVar1 = 0;
  }
  return uVar1;
}



//===========================================================
// FUN_142a6f880 @ 142a6f880   (238 bytes)
//===========================================================

undefined8 FUN_142a6f880(longlong param_1)

{
  char cVar1;
  undefined8 uVar2;
  
  if (*(int *)(param_1 + 0x2a0) != 2) {
    if (*(int *)(param_1 + 0x2a0) != 1) {
      switch(*(undefined4 *)(param_1 + 0x2a8)) {
      case 2:
        return 0xc1;
      default:
        return 0;
      case 8:
        return 0x129;
      case 9:
      case 10:
      case 0xe:
      case 0xf:
        goto switchD_142a6f90a_caseD_9;
      case 0xd:
        return 0x15c;
      case 0x12:
        return 0x105;
      case 0x13:
        return 0x111;
      case 0x14:
      case 0x16:
      case 0x17:
      case 0x18:
      case 0x19:
      case 0x1a:
      case 0x1b:
      case 0x1c:
      case 0x1d:
        return 0x146;
      }
    }
    uVar2 = 0xbc;
    if (*(int *)(param_1 + 0x2a4) != 2) {
      uVar2 = 0xaa;
    }
    return uVar2;
  }
  if (*(int *)(param_1 + 0x2a4) == 2) {
    cVar1 = FUN_142a11ae0();
    uVar2 = 300;
    if (cVar1 != '\0') {
      uVar2 = 0x123;
    }
    return uVar2;
  }
  return 0x112;
switchD_142a6f90a_caseD_9:
  if (*(longlong *)(param_1 + 0x138) != 0) {
    uVar2 = FUN_140404240();
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  FUN_142ef3ac0(0x80004003);
}



//===========================================================
// FUN_142a6fae0 @ 142a6fae0   (121 bytes)
//===========================================================

undefined8 FUN_142a6fae0(longlong param_1)

{
  undefined8 uVar1;
  
  if (*(char *)(param_1 + 0x7a8) != '\0') {
    uVar1 = FUN_1409175e0(param_1 + 0x7ac);
    return uVar1;
  }
  if (*(int *)(param_1 + 0x2a0) == 2) {
    return 0xd6;
  }
  if (*(int *)(param_1 + 0x2a0) == 1) {
    return 0x6e;
  }
  switch(*(undefined4 *)(param_1 + 0x2a8)) {
  case 0:
  case 1:
  case 6:
  case 7:
  case 0x10:
    return 0xf0;
  case 2:
    return 0x69;
  case 3:
  case 4:
  case 5:
  case 0xb:
  case 0xc:
  case 0xe:
  case 0x11:
    return 0x7fffffff;
  case 8:
  case 0x12:
  case 0x13:
  case 0x15:
  case 0x16:
  case 0x17:
  case 0x18:
  case 0x19:
  case 0x1a:
  case 0x1b:
  case 0x1c:
  case 0x1d:
    return 0x46;
  default:
    return 0;
  case 0xd:
    return 300;
  }
}



//===========================================================
// FUN_140cbb020 @ 140cbb020   (15 bytes)
//===========================================================

void FUN_140cbb020(void)

{
  FUN_140cbca60();
  return;
}



//===========================================================
// FUN_1402ca630 @ 1402ca630   (106 bytes)
//===========================================================

void FUN_1402ca630(longlong param_1)

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



//===========================================================
// FUN_140d948a0 @ 140d948a0   (113 bytes)
//===========================================================

void FUN_140d948a0(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 *puVar3;
  
  lVar2 = *(longlong *)(param_1 + 8);
  if (lVar2 != 0) {
    if (0xffffe < *(longlong *)(lVar2 + 0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + 0x20);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar2 == 1) {
      puVar3 = (undefined8 *)(*(longlong *)(param_1 + 8) + 0x18);
      if (*(longlong *)(param_1 + 8) == 0) {
        puVar3 = (undefined8 *)0x0;
      }
      if (puVar3 != (undefined8 *)0x0) {
        (**(code **)*puVar3)(puVar3,1);
      }
    }
    *(undefined8 *)(param_1 + 8) = 0;
  }
  return;
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
// FUN_142e5cd30 @ 142e5cd30   (764 bytes)
//===========================================================

void FUN_142e5cd30(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                  undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8)

{
  undefined8 uVar1;
  undefined1 local_84;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  longlong local_50;
  longlong local_48;
  longlong local_40;
  longlong local_38;
  longlong local_30;
  longlong local_28;
  longlong local_20;
  
  uVar1 = FUN_142a1d8a0(&local_20);
  local_80 = 0;
  FUN_140198700(&local_80,uVar1,local_84);
  local_58 = 0;
  uVar1 = FUN_14019ba10(&local_58,&DAT_143272338,param_1);
  local_78 = 0;
  FUN_14019a260(&local_78,uVar1);
  if (local_58 != 0) {
    FUN_14019f2c0(local_58 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_78);
  if (local_78 != 0) {
    FUN_14019f2c0(local_78 + -0x10);
  }
  local_50 = 0;
  uVar1 = FUN_14019ba10(&local_50,&DAT_143272338,param_2);
  local_70 = 0;
  FUN_14019a260(&local_70,uVar1);
  if (local_50 != 0) {
    FUN_14019f2c0(local_50 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_70);
  if (local_70 != 0) {
    FUN_14019f2c0(local_70 + -0x10);
  }
  FUN_1408bc140(&local_48,param_3);
  FUN_1401a1c50(&local_80,&local_48);
  if (local_48 != 0) {
    FUN_14019f2c0(local_48 + -0x10);
  }
  local_40 = 0;
  uVar1 = FUN_14019ba10(&local_40,&DAT_143272338,param_4);
  local_68 = 0;
  FUN_14019a260(&local_68,uVar1);
  if (local_40 != 0) {
    FUN_14019f2c0(local_40 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_68);
  if (local_68 != 0) {
    FUN_14019f2c0(local_68 + -0x10);
  }
  FUN_1408bc020(&local_38,param_5);
  FUN_1401a1c50(&local_80,&local_38);
  if (local_38 != 0) {
    FUN_14019f2c0(local_38 + -0x10);
  }
  local_30 = 0;
  uVar1 = FUN_14019ba10(&local_30,&DAT_143272338,param_6);
  local_60 = 0;
  FUN_14019a260(&local_60,uVar1);
  if (local_30 != 0) {
    FUN_14019f2c0(local_30 + -0x10);
  }
  FUN_1401a1c50(&local_80,&local_60);
  if (local_60 != 0) {
    FUN_14019f2c0(local_60 + -0x10);
  }
  FUN_1408bc280(&local_28,param_7);
  FUN_1401a1c50(&local_80,&local_28);
  if (local_28 != 0) {
    FUN_14019f2c0(local_28 + -0x10);
  }
  FUN_140198700(&local_80,param_8,local_84);
  FUN_142a1ec10(&local_80);
  if (local_80 != 0) {
    FUN_14019f2c0(local_80 + -0x10);
  }
  if (local_20 != 0) {
    FUN_14019f2c0(local_20 + -0x10);
  }
  return;
}



//===========================================================
// FUN_1410b3850 @ 1410b3850   (302 bytes)
//===========================================================

longlong FUN_1410b3850(longlong param_1)

{
  undefined8 *puVar1;
  
  FUN_1402ca630();
  puVar1 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x1e3);
  if (puVar1 != (undefined8 *)0x0) {
    puVar1[3] = 0;
    puVar1[1] = 0;
    puVar1[2] = 0;
    *puVar1 = &PTR_FUN_14327d968;
    *(undefined8 *)((longlong)puVar1 + 0x1ca) = 0;
    *(undefined1 *)(puVar1 + 4) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x21) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x29) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x31) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x1bd) = 0;
    *(undefined4 *)((longlong)puVar1 + 0x1c5) = 0;
    *(undefined1 *)((longlong)puVar1 + 0x1c9) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x39) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x41) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x49) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x51) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x59) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x61) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x69) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x71) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x79) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x81) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x89) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x91) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x99) = 0;
    *(undefined8 *)((longlong)puVar1 + 0xa1) = 0;
    *(undefined8 *)((longlong)puVar1 + 0xa9) = 0;
    *(undefined8 *)((longlong)puVar1 + 0xb1) = 0;
    *(undefined4 *)((longlong)puVar1 + 0x1b9) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x139) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x141) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x149) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x151) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x159) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x161) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x169) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x171) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x179) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x181) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x189) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x191) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x199) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x1a1) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x1a9) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x1b1) = 0;
    *(undefined8 *)((longlong)puVar1 + 0x1d6) = 0;
    *(undefined4 *)((longlong)puVar1 + 0x1de) = 0;
    *(undefined1 *)((longlong)puVar1 + 0x1e2) = 0;
    *(undefined4 *)((longlong)puVar1 + 0x1d2) = 0xffffffff;
  }
  if (puVar1[1] != 0) {
    FUN_142e541f0(0x2fe);
  }
  puVar1[1] = 1;
  *(undefined8 **)(param_1 + 8) = puVar1;
  return param_1;
}



//===========================================================
// FUN_1401a8660 @ 1401a8660   (265 bytes)
//===========================================================

ulonglong FUN_1401a8660(undefined4 param_1,int param_2,uint param_3)

{
  char cVar1;
  undefined4 uVar2;
  ulonglong uVar3;
  
  if (0 < param_2) {
    switch(param_1) {
    case 0xb:
    case 0xc:
      uVar3 = FUN_14041a5b0(param_2,param_3);
      return uVar3;
    case 0xd:
      cVar1 = FUN_140419fa0(param_3);
      if (cVar1 != '\0') {
        uVar3 = FUN_14041a780(param_2,param_3);
        return uVar3;
      }
      uVar2 = FUN_14041a0a0(param_3);
      uVar3 = FUN_14041a780(param_2,uVar2);
      return uVar3;
    case 0xe:
      uVar3 = FUN_14041aa00(param_2,param_3);
      return uVar3;
    case 0x15:
    case 0x16:
      uVar3 = FUN_14041a680(param_2,param_3);
      return uVar3;
    case 0x17:
      cVar1 = FUN_140419fb0(param_3);
      if (cVar1 != '\0') {
        uVar3 = FUN_14041a7e0(param_2,param_3);
        return uVar3;
      }
      uVar2 = FUN_14041a0f0(param_3);
      uVar3 = FUN_14041a7e0(param_2,uVar2);
      return uVar3;
    case 0x18:
      uVar3 = FUN_14041aa80(param_2,param_3);
      return uVar3;
    }
  }
  return (ulonglong)param_3;
}



//===========================================================
// FUN_14041a360 @ 14041a360   (93 bytes)
//===========================================================

void FUN_14041a360(undefined4 param_1,undefined2 *param_2)

{
  int iVar1;
  
  iVar1 = FUN_1402538e0();
  if (iVar1 != 0) {
    FUN_14041a520(param_1,param_2);
    return;
  }
  iVar1 = FUN_1402538a0(param_1);
  if (iVar1 != 0) {
    FUN_14041a480(param_1,param_2);
    return;
  }
  *param_2 = 0xffff;
  *(undefined1 *)(param_2 + 1) = 0;
  return;
}



//===========================================================
// FUN_1407386b0 @ 1407386b0   (234 bytes)
//===========================================================

uint FUN_1407386b0(uint *param_1)

{
  uint *puVar1;
  uint uVar2;
  void *pvVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  bool bVar7;
  
  pvVar3 = Self;
  LOCK();
  bVar7 = *(longlong *)(param_1 + 4) == 0;
  if (bVar7) {
    *(void **)(param_1 + 4) = Self;
  }
  UNLOCK();
  if (bVar7) {
LAB_140738703:
    param_1[6] = 1;
  }
  else {
    if (*(void **)(param_1 + 4) != pvVar3) {
      while( true ) {
        pvVar3 = Self;
        LOCK();
        bVar7 = *(longlong *)(param_1 + 4) == 0;
        if (bVar7) {
          *(void **)(param_1 + 4) = Self;
        }
        UNLOCK();
        if (bVar7) goto LAB_140738703;
        if (*(void **)(param_1 + 4) == pvVar3) break;
        (*DAT_143262828)(0);
      }
    }
    param_1[6] = param_1[6] + 1;
  }
  uVar4 = param_1[9];
  uVar5 = param_1[2];
  uVar2 = *param_1;
  param_1[1] = uVar5;
  param_1[10] = uVar4;
  param_1[8] = uVar2;
  uVar6 = (uVar4 << 0xc ^ uVar4 >> 6) & 0x1fff ^ uVar4 >> 0x13 ^ uVar4 << 0xc;
  param_1[9] = uVar6;
  uVar4 = (uVar5 >> 0x17 ^ uVar5 << 4) & 0x7f ^ uVar5 >> 0x19 ^ uVar5 << 4;
  param_1[2] = uVar4;
  uVar5 = (uVar2 >> 8 ^ uVar2 << 0x11) & 0x1fffff ^ uVar2 << 0x11 ^ uVar2 >> 0xb;
  *param_1 = uVar5;
  puVar1 = param_1 + 6;
  *puVar1 = *puVar1 - 1;
  if (*puVar1 == 0) {
    param_1[4] = 0;
    param_1[5] = 0;
  }
  return uVar5 ^ uVar4 ^ uVar6;
}



//===========================================================
// FUN_1402538e0 @ 1402538e0   (60 bytes)
//===========================================================

undefined8 FUN_1402538e0(void)

{
  int iVar1;
  
  iVar1 = FUN_140419eb0();
  iVar1 = iVar1 / 10000;
  if (((iVar1 != 3) && (iVar1 != 4)) && (iVar1 != 6)) {
    return 0;
  }
  return 1;
}



//===========================================================
// FUN_142a927d0 @ 142a927d0   (40 bytes)
//===========================================================

longlong FUN_142a927d0(longlong param_1)

{
  longlong lVar1;
  
  lVar1 = *(longlong *)(param_1 + 8);
  if (lVar1 == 0) {
    FUN_142e52ed0(0x428,0);
    lVar1 = *(longlong *)(param_1 + 8);
  }
  return lVar1;
}



//===========================================================
// FUN_14022de90 @ 14022de90   (197 bytes)
//===========================================================

longlong * FUN_14022de90(longlong *param_1,longlong *param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  undefined4 *puVar3;
  longlong lVar4;
  undefined4 *puVar5;
  longlong lVar6;
  
  if (param_1 != param_2) {
    lVar6 = 0;
    if (*param_1 != 0) {
      thunk_FUN_140205820(*param_1 + -8,0);
      *param_1 = 0;
    }
    if (*param_2 != 0) {
      uVar2 = *(uint *)(*param_2 + -8);
      if (uVar2 != 0) {
        lVar4 = FUN_14019b780(&DAT_143ad68a0,(ulonglong)uVar2 * 4 + 8);
        if (lVar4 != 0) {
          lVar6 = lVar4 + 8;
        }
        *param_1 = lVar6;
        *(ulonglong *)(lVar6 + -8) = (ulonglong)uVar2;
        puVar5 = (undefined4 *)*param_2;
        puVar1 = puVar5 + uVar2;
        puVar3 = (undefined4 *)*param_1;
        for (; puVar5 < puVar1; puVar5 = puVar5 + 1) {
          *puVar3 = *puVar5;
          puVar3 = puVar3 + 1;
        }
      }
    }
    return param_1;
  }
  return param_1;
}



//===========================================================
// FUN_1401a8170 @ 1401a8170   (460 bytes)
//===========================================================

undefined8 FUN_1401a8170(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  bool bVar3;
  
  iVar1 = FUN_1403e8af0();
  if (iVar1 == 5) {
    uVar2 = FUN_140417ed0(param_1);
    switch(uVar2) {
    case 1:
      return 0x15;
    case 2:
      goto switchD_1401a82c7_caseD_2;
    case 3:
      return 1;
    default:
      bVar3 = param_1 == 0x56ac78;
      break;
    case 0x1f:
      return 0xd;
    case 0x51:
      return 0x16;
    case 0x52:
      goto switchD_1401a82c7_caseD_52;
    case 0x56:
    case 0x5f:
      return 0xe;
    case 0x58:
      return 0x17;
    case 0x59:
    case 0x5e:
      return 0x18;
    case 0x61:
      return 3;
    }
  }
  else {
    if ((param_1 - 0x2c24c8U < 1000) || (param_1 - 0x2c3080U < 1000)) {
      return 2;
    }
    if ((((param_1 - 0x2c1910U < 1000) || (param_1 - 0x2c1cf8U < 1000)) ||
        (param_1 - 0x2c28b0U < 1000)) || (param_1 - 0x2c2c98U < 1000)) {
switchD_1401a82c7_caseD_52:
      return 0xc;
    }
    if (param_1 - 0x26c1e0U < 10000) {
      return 0x16;
    }
    iVar1 = FUN_1401abe60(param_1);
    if (iVar1 != 0) {
      return 0x18;
    }
    if (param_1 == 0x25232b) {
      return 0x18;
    }
    if (0x2528f7 < param_1) {
      if (param_1 == 0x3ddfd8) {
        return 0xb;
      }
      if (param_1 == 0x3ddfd9) {
        return 0x15;
      }
      if (param_1 == 0x3ddfdc) {
        return 0xb;
      }
      if (param_1 == 0x3ddfdd) {
        return 0x15;
      }
      return 0;
    }
    if (param_1 == 0x2528f7) {
      return 0xb;
    }
    if (param_1 == 0x251928) {
      return 0x1f;
    }
    if (param_1 == 0x251c15) {
      return 0xb;
    }
    if (param_1 == 0x252642) {
      return 0xb;
    }
    if (param_1 == 0x252643) {
      return 0xb;
    }
    bVar3 = param_1 == 0x2528f6;
  }
  if (!bVar3) {
    return 0;
  }
switchD_1401a82c7_caseD_2:
  return 0xb;
}



//===========================================================
// FUN_142a92800 @ 142a92800   (40 bytes)
//===========================================================

longlong FUN_142a92800(longlong param_1)

{
  longlong lVar1;
  
  lVar1 = *(longlong *)(param_1 + 8);
  if (lVar1 == 0) {
    FUN_142e52ed0(0x428,0);
    lVar1 = *(longlong *)(param_1 + 8);
  }
  return lVar1;
}



//===========================================================
// FUN_1402eeb50 @ 1402eeb50   (527 bytes)
//===========================================================

longlong FUN_1402eeb50(longlong param_1,longlong param_2)

{
  undefined8 uVar1;
  int iVar2;
  
  *(undefined1 *)(param_1 + 0x20) = *(undefined1 *)(param_2 + 0x20);
  *(undefined4 *)(param_1 + 0x21) = *(undefined4 *)(param_2 + 0x21);
  *(undefined4 *)(param_1 + 0x25) = *(undefined4 *)(param_2 + 0x25);
  *(undefined4 *)(param_1 + 0x29) = *(undefined4 *)(param_2 + 0x29);
  *(undefined4 *)(param_1 + 0x2d) = *(undefined4 *)(param_2 + 0x2d);
  *(undefined4 *)(param_1 + 0x31) = *(undefined4 *)(param_2 + 0x31);
  *(undefined4 *)(param_1 + 0x35) = *(undefined4 *)(param_2 + 0x35);
  uVar1 = *(undefined8 *)(param_2 + 0x41);
  *(undefined8 *)(param_1 + 0x39) = *(undefined8 *)(param_2 + 0x39);
  *(undefined8 *)(param_1 + 0x41) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x51);
  *(undefined8 *)(param_1 + 0x49) = *(undefined8 *)(param_2 + 0x49);
  *(undefined8 *)(param_1 + 0x51) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x61);
  *(undefined8 *)(param_1 + 0x59) = *(undefined8 *)(param_2 + 0x59);
  *(undefined8 *)(param_1 + 0x61) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x71);
  *(undefined8 *)(param_1 + 0x69) = *(undefined8 *)(param_2 + 0x69);
  *(undefined8 *)(param_1 + 0x71) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x81);
  *(undefined8 *)(param_1 + 0x79) = *(undefined8 *)(param_2 + 0x79);
  *(undefined8 *)(param_1 + 0x81) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x91);
  *(undefined8 *)(param_1 + 0x89) = *(undefined8 *)(param_2 + 0x89);
  *(undefined8 *)(param_1 + 0x91) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0xa1);
  *(undefined8 *)(param_1 + 0x99) = *(undefined8 *)(param_2 + 0x99);
  *(undefined8 *)(param_1 + 0xa1) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0xb1);
  *(undefined8 *)(param_1 + 0xa9) = *(undefined8 *)(param_2 + 0xa9);
  *(undefined8 *)(param_1 + 0xb1) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0xc1);
  *(undefined8 *)(param_1 + 0xb9) = *(undefined8 *)(param_2 + 0xb9);
  *(undefined8 *)(param_1 + 0xc1) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0xd1);
  *(undefined8 *)(param_1 + 0xc9) = *(undefined8 *)(param_2 + 0xc9);
  *(undefined8 *)(param_1 + 0xd1) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0xe1);
  *(undefined8 *)(param_1 + 0xd9) = *(undefined8 *)(param_2 + 0xd9);
  *(undefined8 *)(param_1 + 0xe1) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0xf1);
  *(undefined8 *)(param_1 + 0xe9) = *(undefined8 *)(param_2 + 0xe9);
  *(undefined8 *)(param_1 + 0xf1) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x101);
  *(undefined8 *)(param_1 + 0xf9) = *(undefined8 *)(param_2 + 0xf9);
  *(undefined8 *)(param_1 + 0x101) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x111);
  *(undefined8 *)(param_1 + 0x109) = *(undefined8 *)(param_2 + 0x109);
  *(undefined8 *)(param_1 + 0x111) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x121);
  *(undefined8 *)(param_1 + 0x119) = *(undefined8 *)(param_2 + 0x119);
  *(undefined8 *)(param_1 + 0x121) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x131);
  *(undefined8 *)(param_1 + 0x129) = *(undefined8 *)(param_2 + 0x129);
  *(undefined8 *)(param_1 + 0x131) = uVar1;
  *(undefined4 *)(param_1 + 0x1b9) = *(undefined4 *)(param_2 + 0x1b9);
  uVar1 = *(undefined8 *)(param_2 + 0x141);
  *(undefined8 *)(param_1 + 0x139) = *(undefined8 *)(param_2 + 0x139);
  *(undefined8 *)(param_1 + 0x141) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x151);
  *(undefined8 *)(param_1 + 0x149) = *(undefined8 *)(param_2 + 0x149);
  *(undefined8 *)(param_1 + 0x151) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x161);
  *(undefined8 *)(param_1 + 0x159) = *(undefined8 *)(param_2 + 0x159);
  *(undefined8 *)(param_1 + 0x161) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x171);
  *(undefined8 *)(param_1 + 0x169) = *(undefined8 *)(param_2 + 0x169);
  *(undefined8 *)(param_1 + 0x171) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x181);
  *(undefined8 *)(param_1 + 0x179) = *(undefined8 *)(param_2 + 0x179);
  *(undefined8 *)(param_1 + 0x181) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x191);
  *(undefined8 *)(param_1 + 0x189) = *(undefined8 *)(param_2 + 0x189);
  *(undefined8 *)(param_1 + 0x191) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x1a1);
  *(undefined8 *)(param_1 + 0x199) = *(undefined8 *)(param_2 + 0x199);
  *(undefined8 *)(param_1 + 0x1a1) = uVar1;
  uVar1 = *(undefined8 *)(param_2 + 0x1b1);
  *(undefined8 *)(param_1 + 0x1a9) = *(undefined8 *)(param_2 + 0x1a9);
  *(undefined8 *)(param_1 + 0x1b1) = uVar1;
  *(undefined8 *)(param_1 + 0x1d6) = *(undefined8 *)(param_2 + 0x1d6);
  *(undefined4 *)(param_1 + 0x1de) = *(undefined4 *)(param_2 + 0x1de);
  *(undefined1 *)(param_1 + 0x1e2) = *(undefined1 *)(param_2 + 0x1e2);
  *(undefined4 *)(param_1 + 0x1bd) = *(undefined4 *)(param_2 + 0x1bd);
  *(undefined4 *)(param_1 + 0x1c1) = *(undefined4 *)(param_2 + 0x1c1);
  if (*(int *)(param_2 + 0x1c5) < 1) {
    iVar2 = 0;
  }
  else {
    iVar2 = *(int *)(param_2 + 0x1c5) % 0x168;
  }
  *(int *)(param_1 + 0x1c5) = iVar2;
  *(undefined1 *)(param_1 + 0x1c9) = *(undefined1 *)(param_2 + 0x1c9);
  *(undefined4 *)(param_1 + 0x1ca) = *(undefined4 *)(param_2 + 0x1ca);
  *(undefined4 *)(param_1 + 0x1d2) = *(undefined4 *)(param_2 + 0x1d2);
  return param_1;
}



//===========================================================
// FUN_1401abb40 @ 1401abb40   (308 bytes)
//===========================================================

longlong FUN_1401abb40(ulonglong *param_1,uint param_2)

{
  ulonglong uVar1;
  longlong lVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  uint uVar6;
  ulonglong uVar7;
  uint uVar8;
  
  uVar1 = *param_1;
  uVar7 = 0;
  uVar6 = 0;
  uVar8 = uVar6;
  if (uVar1 != 0) {
    uVar8 = *(uint *)(uVar1 - 8);
  }
  if (param_2 == 0xffffffff) {
    param_2 = uVar8;
  }
  if (uVar1 != 0) {
    uVar5 = *(ulonglong *)(uVar1 - 0x10);
    uVar3 = ~uVar5;
    if (-1 < (longlong)uVar5) {
      uVar3 = uVar5;
    }
    if (uVar8 < (uint)(uVar3 - 8 >> 2)) goto LAB_1401abc2d;
  }
  if (uVar8 == 0) {
    uVar5 = 1;
  }
  else {
    uVar5 = (ulonglong)(uVar8 * 2);
  }
  if (uVar1 != 0) {
    uVar3 = *(ulonglong *)(uVar1 - 0x10);
    uVar4 = ~uVar3;
    if (-1 < (longlong)uVar3) {
      uVar4 = uVar3;
    }
    uVar6 = (uint)(uVar4 - 8 >> 2);
  }
  if (uVar6 != (uint)uVar5) {
    uVar3 = uVar7;
    if (uVar1 != 0) {
      uVar3 = (ulonglong)*(uint *)(uVar1 - 8);
    }
    lVar2 = FUN_14019b780(&DAT_143ad68a0,uVar5 * 4 + 8);
    if (lVar2 != 0) {
      uVar7 = lVar2 + 8;
    }
    if (*param_1 != 0) {
      FUN_142ef7ba0(uVar7,*param_1,uVar3 << 2);
      thunk_FUN_140205820(*param_1 - 8,0);
    }
    *param_1 = uVar7;
    *(ulonglong *)(uVar7 - 8) = uVar3;
  }
LAB_1401abc2d:
  *(longlong *)(*param_1 - 8) = *(longlong *)(*param_1 - 8) + 1;
  lVar2 = *param_1 + (longlong)(int)param_2 * 4;
  FUN_142ef7ba0(lVar2 + 4,lVar2,(ulonglong)(uVar8 - param_2) << 2);
  return *param_1 + (longlong)(int)param_2 * 4;
}



//===========================================================
// FUN_1402538a0 @ 1402538a0   (55 bytes)
//===========================================================

undefined8 FUN_1402538a0(void)

{
  int iVar1;
  
  iVar1 = FUN_140419eb0();
  if ((iVar1 / 10000 != 2) && (iVar1 / 10000 != 5)) {
    return 0;
  }
  return 1;
}



//===========================================================
// FUN_14041a250 @ 14041a250   (11 bytes)
//===========================================================

void FUN_14041a250(undefined1 *param_1,undefined1 param_2,undefined1 param_3,undefined1 param_4)

{
  *param_1 = param_2;
  param_1[1] = param_3;
  param_1[2] = param_4;
  return;
}



//===========================================================
// FUN_1401a7220 @ 1401a7220   (121 bytes)
//===========================================================

void FUN_1401a7220(longlong param_1,int param_2,undefined4 *param_3)

{
  char cVar1;
  int local_res10 [6];
  
  if (param_2 == 100) {
    *param_3 = *(undefined4 *)(param_1 + 0x1c1);
  }
  else {
    local_res10[0] = param_2;
    cVar1 = FUN_14041acf0(local_res10);
    if (cVar1 == '\x01') {
      *param_3 = *(undefined4 *)(param_1 + 0x21);
      return;
    }
    if (cVar1 == '\x02') {
      *param_3 = *(undefined4 *)(param_1 + 0x29);
      return;
    }
    if (cVar1 == '\x04') {
      *param_3 = *(undefined4 *)(param_1 + 0x39);
      return;
    }
  }
  return;
}



//===========================================================
// FUN_1401a72a0 @ 1401a72a0   (119 bytes)
//===========================================================

void FUN_1401a72a0(longlong param_1,int param_2,undefined4 param_3)

{
  char cVar1;
  int local_res10 [6];
  
  if (param_2 == 100) {
    *(undefined4 *)(param_1 + 0x1c1) = param_3;
  }
  else {
    local_res10[0] = param_2;
    cVar1 = FUN_14041acf0(local_res10);
    if (cVar1 == '\x01') {
      *(undefined4 *)(param_1 + 0x21) = param_3;
      *(undefined4 *)(param_1 + 0x25) = 0;
      return;
    }
    if (cVar1 == '\x02') {
      *(undefined4 *)(param_1 + 0x29) = param_3;
      return;
    }
    if (cVar1 == '\x04') {
      *(undefined4 *)(param_1 + 0x39) = param_3;
      return;
    }
  }
  return;
}



//===========================================================
// FUN_140417c70 @ 140417c70   (12 bytes)
//===========================================================

bool FUN_140417c70(int param_1)

{
  return param_1 == 0x4ea8b8;
}



//===========================================================
// FUN_142a94840 @ 142a94840   (403 bytes)
//===========================================================

void FUN_142a94840(longlong param_1)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  void *pvVar4;
  undefined8 *puVar5;
  int *piVar6;
  
  lVar2 = *(longlong *)(param_1 + 8);
  if (lVar2 != 0) {
    puVar5 = (undefined8 *)(lVar2 + -0x28);
    if (0xffffe < *(longlong *)(lVar2 + -0x20) - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar1 = (longlong *)(lVar2 + -0x20);
    lVar2 = *plVar1;
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if ((int)lVar2 == 1) {
      lVar2 = *(longlong *)(param_1 + 8);
      if ((lVar2 != 0) && (*(longlong *)(lVar2 + -0x10) != 0)) {
        LOCK();
        *(undefined8 *)(*(longlong *)(lVar2 + -0x10) + 8) = 0;
        UNLOCK();
        do {
        } while (*(int *)(*(longlong *)(lVar2 + -0x10) + 4) != 0);
      }
      if (puVar5 != (undefined8 *)0x0) {
        (**(code **)*puVar5)(puVar5,1);
      }
    }
    *(undefined8 *)(param_1 + 8) = 0;
  }
  pvVar4 = Self;
  lVar3 = DAT_143add178;
  plVar1 = (longlong *)(DAT_143add178 + 0x18);
  LOCK();
  lVar2 = *plVar1;
  if (lVar2 == 0) {
    *plVar1 = (longlong)Self;
  }
  UNLOCK();
  if (lVar2 == 0) {
LAB_142a94939:
    *(undefined4 *)(lVar3 + 0x20) = 1;
  }
  else if ((void *)*plVar1 == pvVar4) {
    *(int *)(lVar3 + 0x20) = *(int *)(lVar3 + 0x20) + 1;
  }
  else {
    while( true ) {
      pvVar4 = Self;
      LOCK();
      lVar2 = *plVar1;
      if (lVar2 == 0) {
        *plVar1 = (longlong)Self;
      }
      UNLOCK();
      if (lVar2 == 0) goto LAB_142a94939;
      if ((void *)*plVar1 == pvVar4) break;
      (*DAT_143262828)(0);
    }
    *(int *)(lVar3 + 0x20) = *(int *)(lVar3 + 0x20) + 1;
  }
  piVar6 = (int *)(lVar3 + 0x20);
  puVar5 = *(undefined8 **)(lVar3 + 0x28);
  if (puVar5 == (undefined8 *)0x0) {
    puVar5 = (undefined8 *)FUN_14019d3c0(0x30,0x10);
    *(undefined8 **)(lVar3 + 0x28) = puVar5;
  }
  *(undefined8 *)(lVar3 + 0x28) = *puVar5;
  *piVar6 = *piVar6 + -1;
  if (*piVar6 == 0) {
    *plVar1 = 0;
  }
  *(undefined4 *)((longlong)puVar5 + 0x29) = 0;
  *(undefined2 *)((longlong)puVar5 + 0x2d) = 0;
  *(undefined1 *)((longlong)puVar5 + 0x2f) = 0;
  puVar5[3] = 0;
  puVar5[1] = 0;
  puVar5[2] = 0;
  *puVar5 = &PTR_LAB_14348be08;
  puVar5[4] = &PTR_LAB_14348be10;
  *(undefined2 *)(puVar5 + 5) = 0xffff;
  *(undefined1 *)((longlong)puVar5 + 0x2a) = 0;
  if (puVar5[1] != 0) {
    FUN_142e541f0(0x2fe);
  }
  puVar5[1] = 1;
  *(undefined8 **)(param_1 + 8) = puVar5 + 5;
  return;
}



//===========================================================
// FUN_142a91ad0 @ 142a91ad0   (346 bytes)
//===========================================================

void FUN_142a91ad0(longlong param_1)

{
  undefined4 uVar1;
  int iVar2;
  undefined4 *puVar3;
  longlong lVar4;
  undefined4 local_res8 [2];
  
  switch(*(undefined4 *)(param_1 + 0x70)) {
  case 0xb:
  case 0xc:
  case 0xf:
  case 0x15:
  case 0x16:
  case 0x19:
    if (*(longlong *)(param_1 + 0x10) != 0) {
      local_res8[0] = 0;
      FUN_1401a7220(*(longlong *)(param_1 + 0x10),*(undefined4 *)(param_1 + 0x70),local_res8);
      puVar3 = *(undefined4 **)(param_1 + 0xd0);
      if ((puVar3 != (undefined4 *)0x0) && (puVar3[-2] != 0)) {
        while( true ) {
          uVar1 = FUN_1401a8660(*(undefined4 *)(param_1 + 0x70),local_res8[0],*puVar3);
          *puVar3 = uVar1;
          if ((undefined4 *)
              (*(longlong *)(param_1 + 0xd0) + *(longlong *)(*(longlong *)(param_1 + 0xd0) + -8) * 4
              + -4) <= puVar3) break;
          puVar3 = puVar3 + 1;
          if (puVar3 == (undefined4 *)0x0) {
            return;
          }
        }
      }
    }
    break;
  case 0xd:
  case 0x10:
  case 0x11:
  case 0x12:
  case 0x13:
  case 0x14:
  case 0x17:
    break;
  case 0xe:
  case 0x18:
    if (((*(longlong *)(param_1 + 0x80) != 0) &&
        (puVar3 = *(undefined4 **)(param_1 + 0xd0), puVar3 != (undefined4 *)0x0)) &&
       (puVar3[-2] != 0)) {
      do {
        lVar4 = *(longlong *)(param_1 + 0x80);
        if (lVar4 == 0) {
          FUN_142e52ed0(0x428,0);
          lVar4 = *(longlong *)(param_1 + 0x80);
        }
        iVar2 = FUN_14041a270(lVar4);
        if (iVar2 == 0) {
          uVar1 = FUN_140419eb0(*puVar3);
        }
        else {
          uVar1 = FUN_14041a990();
        }
        *puVar3 = uVar1;
      } while ((puVar3 < (undefined4 *)
                         (*(longlong *)(param_1 + 0xd0) +
                          *(longlong *)(*(longlong *)(param_1 + 0xd0) + -8) * 4 + -4)) &&
              (puVar3 = puVar3 + 1, puVar3 != (undefined4 *)0x0));
    }
    break;
  default:
    goto switchD_142a91b01_default;
  }
switchD_142a91b01_default:
  return;
}



//===========================================================
// FUN_140253930 @ 140253930   (31 bytes)
//===========================================================

bool FUN_140253930(void)

{
  int iVar1;
  
  iVar1 = FUN_140419fe0();
  return iVar1 + -12000 < 12000;
}


