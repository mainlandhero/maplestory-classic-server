
//===========================================================
// FUN_1411be0a0 @ 1411be0a0   (4541 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x0001411be26e) */
/* WARNING: Removing unreachable block (ram,0x0001411be283) */
/* WARNING: Removing unreachable block (ram,0x0001411be383) */
/* WARNING: Removing unreachable block (ram,0x0001411be47e) */

void FUN_1411be0a0(longlong *param_1)

{
  undefined4 **ppuVar1;
  undefined1 uVar2;
  byte bVar3;
  byte bVar4;
  undefined4 uVar5;
  longlong *plVar6;
  undefined4 uVar7;
  undefined *puVar8;
  longlong *plVar9;
  int iVar10;
  uint uVar11;
  undefined4 uVar12;
  longlong lVar13;
  int *piVar14;
  int *piVar15;
  int *piVar16;
  int *piVar17;
  undefined4 *puVar18;
  byte *pbVar19;
  undefined8 *puVar20;
  byte *pbVar21;
  undefined8 uVar22;
  undefined4 *puVar23;
  char cVar24;
  longlong *plVar25;
  byte *pbVar26;
  int iVar27;
  uint uVar28;
  byte *pbVar29;
  ulonglong uVar30;
  int *piVar31;
  int *piVar32;
  byte **ppbVar33;
  char cVar34;
  ulonglong uVar35;
  int *piVar36;
  undefined8 *puVar37;
  int *piVar38;
  ulonglong uVar39;
  undefined4 *puVar40;
  bool bVar41;
  undefined8 local_res10;
  undefined4 *local_res18;
  ulonglong local_res20;
  int *local_128;
  byte *local_118;
  longlong local_110;
  byte *local_108;
  uint uStack_100;
  undefined4 *local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  ulonglong local_d8;
  byte **local_d0;
  undefined8 *local_c8;
  longlong *local_c0;
  int local_b8;
  int local_b4;
  int *local_b0;
  int *local_a8;
  int *piStack_a0;
  int *local_98;
  undefined4 local_90;
  int *local_88;
  undefined4 local_80;
  undefined1 local_7c;
  undefined8 local_78;
  undefined8 uStack_70;
  
  iVar27 = 0;
  *(undefined4 *)(param_1 + 0x4e) = 0;
  local_c0 = param_1 + 0x4c;
  FUN_1411d3cc0(local_c0);
  ppbVar33 = DAT_143aa84a0;
  local_d0 = DAT_143aa84a0;
  if (DAT_143aa84a0 == (byte **)0x0) {
    return;
  }
  lVar13 = FUN_142cbe730(DAT_143aa84a0);
  *(undefined4 *)((longlong)param_1 + 0x274) = *(undefined4 *)(lVar13 + 0x118b);
  local_res18 = (undefined4 *)CONCAT44(local_res18._4_4_,0xffffffff);
  local_d8 = 0;
  local_res10 = (byte *)((ulonglong)local_res10._4_4_ << 0x20);
  iVar10 = FUN_142cc3e40(ppbVar33);
  if (iVar10 < 1) {
LAB_1411be887:
    iVar27 = 0;
  }
  else {
    do {
      piVar17 = (int *)0x0;
      piVar14 = (int *)FUN_142cc3ea0(ppbVar33,iVar27);
      iVar27 = *piVar14;
      local_res20 = CONCAT44(local_res20._4_4_,iVar27);
      if ((iVar27 != 0) || (piVar14[10] != 0)) {
        local_b0 = (int *)0x0;
        local_a8 = (int *)0x0;
        piStack_a0 = (int *)0x0;
        local_98 = (int *)0x0;
        local_90 = 0;
        local_88 = (int *)0x0;
        local_80 = 0;
        local_78 = 0;
        uStack_70 = 0;
        if ((byte)(*(char *)((longlong)piVar14 + 0x11) - 5U) < 4) {
          local_b4 = piVar14[10];
        }
        else {
          local_b4 = 0;
        }
        piVar36 = piVar14 + 1;
        piVar38 = piVar17;
        local_b8 = iVar27;
        if (piVar36 != (int *)0x0) {
          piVar31 = (int *)0xffffffffffffffff;
          do {
            piVar31 = (int *)((longlong)piVar31 + 1);
          } while (*(char *)((longlong)piVar36 + (longlong)piVar31) != '\0');
          iVar10 = (int)piVar31;
          iVar27 = 0;
          if (0 < iVar10) {
            iVar27 = iVar10;
          }
          piVar15 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar27 + 0x11));
          piVar15[1] = iVar27;
          *piVar15 = -1;
          piVar38 = piVar15 + 4;
          piVar15[2] = 0;
          *(undefined1 *)piVar38 = 0;
          FUN_142ef7ba0(piVar38,piVar36,(longlong)iVar10);
          if (*piVar15 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar10 == -1) || (iVar10 <= piVar15[1])) {
            *piVar15 = 1;
            if (iVar10 != -1) goto LAB_1411be23f;
            piVar31 = piVar17;
            if (piVar38 != (int *)0x0) {
              piVar31 = (int *)0xffffffffffffffff;
              do {
                piVar31 = (int *)((longlong)piVar31 + 1);
              } while (*(char *)((longlong)piVar38 + (longlong)piVar31) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar15[1],(ulonglong)piVar31 & 0xffffffff);
            *piVar15 = 1;
LAB_1411be23f:
            *(undefined1 *)((longlong)iVar10 + (longlong)piVar38) = 0;
          }
          iVar27 = (int)piVar31;
          if ((iVar27 < 0) || (piVar15[1] + 1 <= iVar27)) {
            FUN_142e54290(0x9c,(ulonglong)piVar31 & 0xffffffff);
          }
          piVar15[2] = iVar27;
        }
        lVar13 = (longlong)piVar14 + 0x16;
        piVar36 = piVar17;
        local_b0 = piVar38;
        if (lVar13 != 0) {
          piVar31 = (int *)0xffffffffffffffff;
          do {
            piVar31 = (int *)((longlong)piVar31 + 1);
          } while (*(char *)(lVar13 + (longlong)piVar31) != '\0');
          iVar10 = (int)piVar31;
          iVar27 = 0;
          if (0 < iVar10) {
            iVar27 = iVar10;
          }
          piVar15 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar27 + 0x11));
          piVar15[1] = iVar27;
          *piVar15 = -1;
          piVar36 = piVar15 + 4;
          piVar15[2] = 0;
          *(undefined1 *)piVar36 = 0;
          FUN_142ef7ba0(piVar36,lVar13,(longlong)iVar10);
          if (*piVar15 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar10 == -1) || (iVar10 <= piVar15[1])) {
            *piVar15 = 1;
            if (iVar10 != -1) goto LAB_1411be33b;
            piVar31 = piVar17;
            if (piVar36 != (int *)0x0) {
              piVar31 = (int *)0xffffffffffffffff;
              do {
                piVar31 = (int *)((longlong)piVar31 + 1);
              } while (*(char *)((longlong)piVar36 + (longlong)piVar31) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar15[1],(ulonglong)piVar31 & 0xffffffff);
            *piVar15 = 1;
LAB_1411be33b:
            *(undefined1 *)((longlong)iVar10 + (longlong)piVar36) = 0;
          }
          iVar27 = (int)piVar31;
          if ((iVar27 < 0) || (piVar15[1] + 1 <= iVar27)) {
            FUN_142e54290(0x9c,(ulonglong)piVar31 & 0xffffffff);
          }
          piVar15[2] = iVar27;
        }
        if (local_a8 != (int *)0x0) {
          FUN_14019f2c0(local_a8 + -4);
        }
        piVar31 = piVar14 + 0xb;
        piVar15 = piVar17;
        local_a8 = piVar36;
        if (piVar31 != (int *)0x0) {
          piVar32 = (int *)0xffffffffffffffff;
          do {
            piVar32 = (int *)((longlong)piVar32 + 1);
          } while (*(char *)((longlong)piVar31 + (longlong)piVar32) != '\0');
          iVar10 = (int)piVar32;
          iVar27 = 0;
          if (0 < iVar10) {
            iVar27 = iVar10;
          }
          piVar16 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar27 + 0x11));
          piVar16[1] = iVar27;
          *piVar16 = -1;
          piVar15 = piVar16 + 4;
          piVar16[2] = 0;
          *(undefined1 *)piVar15 = 0;
          FUN_142ef7ba0(piVar15,piVar31,(longlong)iVar10);
          if (*piVar16 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar10 == -1) || (iVar10 <= piVar16[1])) {
            *piVar16 = 1;
            if (iVar10 != -1) goto LAB_1411be438;
            if (piVar15 != (int *)0x0) {
              piVar17 = (int *)0xffffffffffffffff;
              do {
                piVar17 = (int *)((longlong)piVar17 + 1);
              } while (*(char *)((longlong)piVar15 + (longlong)piVar17) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar16[1],(ulonglong)piVar32 & 0xffffffff);
            *piVar16 = 1;
LAB_1411be438:
            *(undefined1 *)((longlong)iVar10 + (longlong)piVar15) = 0;
            piVar17 = piVar32;
          }
          iVar27 = (int)piVar17;
          if ((iVar27 < 0) || (piVar16[1] + 1 <= iVar27)) {
            FUN_142e54290(0x9c,(ulonglong)piVar17 & 0xffffffff);
          }
          piVar16[2] = iVar27;
        }
        if (piStack_a0 != (int *)0x0) {
          FUN_14019f2c0(piStack_a0 + -4);
        }
        lVar13 = (longlong)piVar14 + 0x39;
        local_128 = (int *)0x0;
        piStack_a0 = piVar15;
        if (lVar13 != 0) {
          uVar35 = 0xffffffffffffffff;
          do {
            uVar35 = uVar35 + 1;
          } while (*(char *)(uVar35 + lVar13) != '\0');
          iVar10 = (int)uVar35;
          iVar27 = 0;
          if (0 < iVar10) {
            iVar27 = iVar10;
          }
          piVar17 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar27 + 0x11));
          piVar17[1] = iVar27;
          *piVar17 = -1;
          local_128 = piVar17 + 4;
          piVar17[2] = 0;
          *(undefined1 *)local_128 = 0;
          FUN_142ef7ba0(local_128,lVar13,(longlong)iVar10);
          if (*piVar17 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar10 == -1) || (iVar10 <= piVar17[1])) {
            *piVar17 = 1;
            if (iVar10 != -1) goto LAB_1411be530;
            if (local_128 == (int *)0x0) {
              uVar35 = 0;
            }
            else {
              uVar35 = 0xffffffffffffffff;
              do {
                uVar35 = uVar35 + 1;
              } while (*(char *)((longlong)local_128 + uVar35) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar17[1],uVar35 & 0xffffffff);
            *piVar17 = 1;
LAB_1411be530:
            *(undefined1 *)((longlong)iVar10 + (longlong)local_128) = 0;
          }
          iVar27 = (int)uVar35;
          if ((iVar27 < 0) || (piVar17[1] + 1 <= iVar27)) {
            FUN_142e54290(0x9c,uVar35 & 0xffffffff);
          }
          piVar17[2] = iVar27;
        }
        uVar12 = *(undefined4 *)((longlong)piVar14 + 0x12);
        local_98 = local_128;
        local_90 = uVar12;
        lVar13 = FUN_1402b0250(*(undefined4 *)((longlong)piVar14 + 0x13d),
                               *(undefined4 *)((longlong)piVar14 + 0x141));
        piVar17 = (int *)0x0;
        if (lVar13 != 0) {
          uVar35 = 0xffffffffffffffff;
          do {
            uVar35 = uVar35 + 1;
          } while (*(char *)(uVar35 + lVar13) != '\0');
          iVar10 = (int)uVar35;
          iVar27 = 0;
          if (0 < iVar10) {
            iVar27 = iVar10;
          }
          piVar31 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar27 + 0x11));
          piVar31[1] = iVar27;
          *piVar31 = -1;
          piVar17 = piVar31 + 4;
          piVar31[2] = 0;
          *(undefined1 *)piVar17 = 0;
          local_108 = (byte *)(longlong)iVar10;
          FUN_142ef7ba0(piVar17,lVar13,local_108);
          if (*piVar31 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if ((iVar10 == -1) || (iVar10 <= piVar31[1])) {
            *piVar31 = 1;
            if (iVar10 != -1) goto LAB_1411be61b;
            if (piVar17 == (int *)0x0) {
              uVar35 = 0;
            }
            else {
              uVar35 = 0xffffffffffffffff;
              do {
                uVar35 = uVar35 + 1;
              } while (*(char *)((longlong)piVar17 + uVar35) != '\0');
            }
          }
          else {
            FUN_142e54290(0x90,piVar31[1],uVar35 & 0xffffffff);
            *piVar31 = 1;
LAB_1411be61b:
            local_108[(longlong)piVar17] = 0;
          }
          iVar27 = (int)uVar35;
          if ((iVar27 < 0) || (piVar31[1] + 1 <= iVar27)) {
            FUN_142e54290(0x9c,uVar35 & 0xffffffff);
          }
          piVar31[2] = iVar27;
        }
        uVar5 = *(undefined4 *)((longlong)piVar14 + 0x139);
        uVar2 = *(undefined1 *)((longlong)piVar14 + 0x11);
        local_78 = 0;
        uStack_70 = 0;
        local_88 = piVar17;
        local_80 = uVar5;
        local_7c = uVar2;
        puVar18 = (undefined4 *)FUN_1411d3950(&local_d8,0xffffffff);
        *puVar18 = (undefined4)local_res20;
        puVar18[1] = local_b4;
        FUN_14019a260(puVar18 + 2,&local_b0);
        FUN_14019a260(puVar18 + 4,&local_a8);
        FUN_14019a260(puVar18 + 6,&piStack_a0);
        FUN_14019a260(puVar18 + 8,&local_98);
        puVar18[10] = uVar12;
        FUN_14019a260(puVar18 + 0xc,&local_88);
        puVar18[0xe] = uVar5;
        *(undefined1 *)(puVar18 + 0xf) = uVar2;
        *(undefined8 *)(puVar18 + 0x10) = 0;
        *(undefined8 *)(puVar18 + 0x12) = 0;
        if (piVar17 != (int *)0x0) {
          FUN_14019f2c0(piVar17 + -4);
        }
        if (local_128 != (int *)0x0) {
          FUN_14019f2c0(local_128 + -4);
        }
        if (piVar15 != (int *)0x0) {
          FUN_14019f2c0(piVar15 + -4);
        }
        if (piVar36 != (int *)0x0) {
          FUN_14019f2c0(piVar36 + -4);
        }
        ppbVar33 = local_d0;
        if (piVar38 != (int *)0x0) {
          FUN_14019f2c0(piVar38 + -4);
          ppbVar33 = local_d0;
        }
      }
      iVar27 = (int)local_res10 + 1;
      local_res10 = (byte *)CONCAT44(local_res10._4_4_,iVar27);
      iVar10 = FUN_142cc3e40(ppbVar33);
    } while (iVar27 < iVar10);
    if (local_d8 == 0) goto LAB_1411be887;
    iVar27 = *(int *)(local_d8 - 8);
  }
  FUN_1411d0ef0(&local_d8,0,iVar27 + -1,&local_res10);
  local_118 = (byte *)0x0;
  local_110 = 0;
  local_118 = (byte *)FUN_14019b780(&DAT_143ad68a0,0x30);
  uVar35 = local_d8;
  *(byte **)local_118 = local_118;
  *(byte **)(local_118 + 8) = local_118;
  *(byte **)(local_118 + 0x10) = local_118;
  local_118[0x18] = 1;
  local_118[0x19] = 1;
  local_res20 = local_d8;
  puVar37 = (undefined8 *)(local_d8 + 0x10);
  uVar30 = (ulonglong)local_res18 & 0xffffffff;
  uVar39 = 0;
  while ((uVar28 = (uint)uVar39, uVar35 != 0 && (uVar28 < *(uint *)(uVar35 - 8)))) {
    if ((int)uVar28 < 0) {
      FUN_142e54290(0xbc,uVar39);
    }
    pbVar29 = local_118;
    local_108 = *(byte **)(local_118 + 8);
    uStack_100 = 0;
    pbVar21 = local_118;
    if (local_108[0x19] == 0) {
      pbVar26 = local_108;
      do {
        local_108 = pbVar26;
        pbVar26 = DAT_143aa8360;
        if ((byte *)*puVar37 != (byte *)0x0) {
          pbVar26 = (byte *)*puVar37;
        }
        pbVar19 = DAT_143aa8360;
        if (*(byte **)(local_108 + 0x20) != (byte *)0x0) {
          pbVar19 = *(byte **)(local_108 + 0x20);
        }
        lVar13 = (longlong)pbVar26 - (longlong)pbVar19;
        do {
          bVar3 = *pbVar19;
          uVar11 = (uint)pbVar19[lVar13];
          if (bVar3 != uVar11) break;
          pbVar19 = pbVar19 + 1;
        } while (uVar11 != 0);
        bVar41 = -1 < (int)(bVar3 - uVar11);
        if (bVar41) {
          pbVar26 = *(byte **)local_108;
          pbVar21 = local_108;
        }
        else {
          pbVar26 = *(byte **)(local_108 + 0x10);
        }
        uStack_100 = (uint)bVar41;
      } while (pbVar26[0x19] == 0);
    }
    if (pbVar21[0x19] == 0) {
      pbVar26 = DAT_143aa8360;
      if (*(byte **)(pbVar21 + 0x20) != (byte *)0x0) {
        pbVar26 = *(byte **)(pbVar21 + 0x20);
      }
      pbVar19 = DAT_143aa8360;
      if ((byte *)*puVar37 != (byte *)0x0) {
        pbVar19 = (byte *)*puVar37;
      }
      lVar13 = (longlong)pbVar26 - (longlong)pbVar19;
      do {
        bVar3 = *pbVar19;
        uVar11 = (uint)pbVar19[lVar13];
        if (bVar3 != uVar11) break;
        pbVar19 = pbVar19 + 1;
      } while (uVar11 != 0);
      if ((int)(bVar3 - uVar11) < 0) goto LAB_1411be9ec;
    }
    else {
LAB_1411be9ec:
      if (local_110 == 0x555555555555555) {
                    /* WARNING: Subroutine does not return */
        FUN_14019f9d0();
      }
      local_d0 = &local_118;
      local_c8 = (undefined8 *)0x0;
      puVar20 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x30);
      puVar20[4] = 0;
      local_c8 = puVar20;
      FUN_14019a260(puVar20 + 4,puVar37);
      puVar20[5] = 0;
      *puVar20 = pbVar29;
      puVar20[1] = pbVar29;
      puVar20[2] = pbVar29;
      *(undefined2 *)(puVar20 + 3) = 0;
      local_c8 = (undefined8 *)0x0;
      pbVar21 = (byte *)FUN_1411d4080(&local_118,&local_108);
    }
    uVar22 = FUN_1411d3950(pbVar21 + 0x28,0xffffffff);
    FUN_1411d2420(uVar22,puVar37 + -2);
    if ((*(char *)((longlong)puVar37 + 0x2c) != '\x01') &&
       (*(char *)((longlong)puVar37 + 0x2c) != '\x06')) {
      *(int *)(param_1 + 0x4e) = (int)param_1[0x4e] + 1;
    }
    if ((*(int *)(puVar37 + -2) == (int)param_1[0x4d]) &&
       (*(int *)((longlong)puVar37 + -0xc) == *(int *)((longlong)param_1 + 0x26c))) {
      bVar41 = true;
    }
    else {
      bVar41 = false;
    }
    puVar37 = puVar37 + 10;
    uVar30 = uVar39;
    if (!bVar41) {
      uVar30 = (ulonglong)local_res18 & 0xffffffff;
    }
    local_res18 = (undefined4 *)CONCAT44(local_res18._4_4_,(int)uVar30);
    uVar39 = (ulonglong)(uVar28 + 1);
  }
  uVar28 = (uint)uVar30;
  if (uVar28 == 0xffffffff) {
LAB_1411beb36:
    *(undefined4 *)(param_1 + 0x4d) = 0;
    uVar12 = 0;
  }
  else {
    if (uVar35 == 0) {
      uVar11 = 0;
    }
    else {
      uVar11 = *(uint *)(uVar35 - 8);
    }
    if (((int)uVar28 < 0) || (uVar11 <= uVar28)) {
      if (uVar35 == 0) {
        uVar12 = 0;
      }
      else {
        uVar12 = *(undefined4 *)(uVar35 - 8);
      }
      FUN_142e54290(0xbc,uVar30,uVar12);
    }
    puVar18 = (undefined4 *)((longlong)(int)uVar28 * 0x50 + uVar35);
    if (puVar18 == (undefined4 *)0x0) goto LAB_1411beb36;
    *(undefined4 *)(param_1 + 0x4d) = *puVar18;
    uVar12 = puVar18[1];
  }
  *(undefined4 *)((longlong)param_1 + 0x26c) = uVar12;
  if (((param_1[0x48] != 0) &&
      (plVar25 = (longlong *)(param_1[0x48] + 8),
      iVar27 = (**(code **)(*plVar25 + 0xd0))(plVar25,&PTR_DAT_143a8b230), iVar27 != 0)) &&
     (lVar13 = param_1[0x48], lVar13 != 0)) {
    FUN_1411c45b0(lVar13);
    FUN_1411c47a0(lVar13);
  }
  cVar34 = '\x01';
  if ((local_110 != 0) &&
     (plVar25 = *(longlong **)local_118, *(char *)((longlong)plVar25 + 0x19) == '\0')) {
    do {
      local_108 = (byte *)0x0;
      uStack_100 = uStack_100 & 0xffffff00;
      local_f8 = (undefined4 *)0x0;
      uStack_f0 = 0;
      local_e8 = 0;
      FUN_14019a260(&local_108,plVar25 + 4);
      uVar22 = DAT_143ac87a0;
      local_res10 = (byte *)0x0;
      FUN_14019a260(&local_res10,plVar25 + 4);
      iVar27 = FUN_1415fe800(uVar22,&local_res10);
      uStack_100 = CONCAT31(uStack_100._1_3_,iVar27 != 0);
      FUN_1411d21c0(&local_f8,plVar25 + 5);
      pbVar29 = local_108;
      uStack_f0 = 0;
      local_e8 = 0;
      pbVar21 = DAT_143aa8360;
      if (PTR_s_Default_Group_143a45268 != (undefined *)0x0) {
        pbVar21 = PTR_s_Default_Group_143a45268;
      }
      pbVar26 = DAT_143aa8360;
      if (local_108 != (byte *)0x0) {
        pbVar26 = local_108;
      }
      lVar13 = (longlong)pbVar21 - (longlong)pbVar26;
      do {
        bVar3 = *pbVar26;
        bVar4 = pbVar26[lVar13];
        if (bVar3 != bVar4) break;
        pbVar26 = pbVar26 + 1;
      } while (bVar4 != 0);
      cVar24 = '\0';
      if (bVar3 != bVar4) {
        cVar24 = cVar34;
      }
      local_res10 = (byte *)CONCAT71(local_res10._1_7_,cVar24);
      puVar23 = (undefined4 *)FUN_1411d3640(local_c0,0xffffffff);
      local_res18 = puVar23;
      FUN_14019a260(puVar23,&local_108);
      puVar18 = local_f8;
      *(undefined1 *)(puVar23 + 2) = (undefined1)uStack_100;
      ppuVar1 = (undefined4 **)(puVar23 + 4);
      cVar34 = cVar24;
      if (ppuVar1 != &local_f8) {
        puVar23 = *ppuVar1;
        if (puVar23 != (undefined4 *)0x0) {
          puVar40 = puVar23 + *(longlong *)(puVar23 + -2) * 0x14;
          if (puVar23 < puVar40) {
            do {
              FUN_1411d1e90(puVar23);
              puVar23 = puVar23 + 0x14;
            } while (puVar23 < puVar40);
            puVar23 = *ppuVar1;
          }
          thunk_FUN_140205820(puVar23 + -2,0);
          *ppuVar1 = (undefined4 *)0x0;
        }
        if (puVar18 != (undefined4 *)0x0) {
          uVar28 = puVar18[-2];
          if (uVar28 != 0) {
            puVar23 = (undefined4 *)FUN_14019b780(&DAT_143ad68a0,(ulonglong)uVar28 * 0x50 + 8);
            if (puVar23 != (undefined4 *)0x0) {
              puVar23 = puVar23 + 2;
            }
            *ppuVar1 = puVar23;
            *(ulonglong *)(puVar23 + -2) = (ulonglong)uVar28;
            ppbVar33 = (byte **)*ppuVar1;
            pbVar29 = local_108;
            for (puVar23 = puVar18; local_108 = pbVar29,
                puVar23 < puVar18 + (ulonglong)uVar28 * 0x14; puVar23 = puVar23 + 0x14) {
              *(undefined4 *)ppbVar33 = *puVar23;
              *(undefined4 *)((longlong)ppbVar33 + 4) = puVar23[1];
              ppbVar33[1] = (byte *)0x0;
              local_d0 = ppbVar33;
              FUN_14019a260(ppbVar33 + 1,puVar23 + 2);
              ppbVar33[2] = (byte *)0x0;
              FUN_14019a260(ppbVar33 + 2,puVar23 + 4);
              ppbVar33[3] = (byte *)0x0;
              FUN_14019a260(ppbVar33 + 3,puVar23 + 6);
              ppbVar33[4] = (byte *)0x0;
              FUN_14019a260(ppbVar33 + 4,puVar23 + 8);
              *(undefined4 *)(ppbVar33 + 5) = puVar23[10];
              ppbVar33[6] = (byte *)0x0;
              FUN_14019a260(ppbVar33 + 6,puVar23 + 0xc);
              *(undefined4 *)(ppbVar33 + 7) = puVar23[0xe];
              *(undefined1 *)((longlong)ppbVar33 + 0x3c) = *(undefined1 *)(puVar23 + 0xf);
              uVar12 = puVar23[0x11];
              uVar5 = puVar23[0x12];
              uVar7 = puVar23[0x13];
              *(undefined4 *)(ppbVar33 + 8) = puVar23[0x10];
              *(undefined4 *)((longlong)ppbVar33 + 0x44) = uVar12;
              *(undefined4 *)(ppbVar33 + 9) = uVar5;
              *(undefined4 *)((longlong)ppbVar33 + 0x4c) = uVar7;
              ppbVar33 = ppbVar33 + 10;
              pbVar29 = local_108;
            }
            cVar34 = (char)local_res10;
          }
        }
      }
      *(undefined8 *)(local_res18 + 6) = 0;
      *(undefined8 *)(local_res18 + 8) = 0;
      if (puVar18 != (undefined4 *)0x0) {
        plVar6 = (longlong *)(puVar18 + -2);
        puVar23 = puVar18 + *plVar6 * 0x14;
        for (; puVar18 < puVar23; puVar18 = puVar18 + 0x14) {
          FUN_1411d1e90(puVar18);
        }
        thunk_FUN_140205820(plVar6,0);
      }
      if (pbVar29 != (byte *)0x0) {
        FUN_14019f2c0(pbVar29 + -0x10);
      }
      plVar6 = (longlong *)plVar25[2];
      if (*(char *)((longlong)plVar6 + 0x19) == '\0') {
        cVar24 = *(char *)(*plVar6 + 0x19);
        plVar25 = plVar6;
        plVar6 = (longlong *)*plVar6;
        while (cVar24 == '\0') {
          cVar24 = *(char *)(*plVar6 + 0x19);
          plVar25 = plVar6;
          plVar6 = (longlong *)*plVar6;
        }
      }
      else {
        cVar24 = *(char *)(plVar25[1] + 0x19);
        plVar9 = (longlong *)plVar25[1];
        plVar6 = plVar25;
        while ((plVar25 = plVar9, cVar24 == '\0' && (plVar6 == (longlong *)plVar25[2]))) {
          cVar24 = *(char *)(plVar25[1] + 0x19);
          plVar9 = (longlong *)plVar25[1];
          plVar6 = plVar25;
        }
      }
    } while (*(char *)((longlong)plVar25 + 0x19) == '\0');
    if (cVar34 == '\0') goto LAB_1411bf18b;
  }
  puVar8 = PTR_s_Default_Group_143a45268;
  local_108 = (byte *)0x0;
  uStack_100 = uStack_100 & 0xffffff00;
  local_f8 = (undefined4 *)0x0;
  uStack_f0 = 0;
  local_e8 = 0;
  pbVar29 = (byte *)0x0;
  local_res10 = (byte *)0x0;
  if (PTR_s_Default_Group_143a45268 != (undefined *)0x0) {
    uVar35 = 0xffffffffffffffff;
    do {
      uVar35 = uVar35 + 1;
    } while (PTR_s_Default_Group_143a45268[uVar35] != '\0');
    iVar10 = (int)uVar35;
    iVar27 = 0;
    if (0 < iVar10) {
      iVar27 = iVar10;
    }
    piVar17 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar27 + 0x11));
    piVar17[1] = iVar27;
    *piVar17 = -1;
    pbVar29 = (byte *)(piVar17 + 4);
    piVar17[2] = 0;
    *pbVar29 = 0;
    local_res10 = pbVar29;
    FUN_142ef7ba0(pbVar29,puVar8,(longlong)iVar10);
    if (*piVar17 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar10 == -1) || (iVar10 <= piVar17[1])) {
      *piVar17 = 1;
      if (iVar10 != -1) goto LAB_1411bef76;
      if (pbVar29 == (byte *)0x0) {
        uVar35 = 0;
      }
      else {
        uVar35 = 0xffffffffffffffff;
        do {
          uVar35 = uVar35 + 1;
        } while (pbVar29[uVar35] != 0);
      }
    }
    else {
      FUN_142e54290(0x90,piVar17[1],uVar35 & 0xffffffff);
      *piVar17 = 1;
LAB_1411bef76:
      pbVar29[iVar10] = 0;
    }
    iVar27 = (int)uVar35;
    if ((iVar27 < 0) || (piVar17[1] + 1 <= iVar27)) {
      FUN_142e54290(0x9c,uVar35 & 0xffffffff);
    }
    piVar17[2] = iVar27;
  }
  uStack_100 = uStack_100 & 0xffffff00;
  uStack_f0 = 0;
  local_e8 = 0;
  local_108 = pbVar29;
  lVar13 = FUN_1411d3640(local_c0,0xffffffff);
  local_res10 = (byte *)lVar13;
  FUN_14019a260(lVar13,&local_108);
  puVar18 = local_f8;
  *(undefined1 *)(lVar13 + 8) = 0;
  ppuVar1 = (undefined4 **)(lVar13 + 0x10);
  if (ppuVar1 != &local_f8) {
    puVar23 = *ppuVar1;
    if (puVar23 != (undefined4 *)0x0) {
      puVar40 = puVar23 + *(longlong *)(puVar23 + -2) * 0x14;
      if (puVar23 < puVar40) {
        do {
          FUN_1411d1e90(puVar23);
          puVar23 = puVar23 + 0x14;
        } while (puVar23 < puVar40);
        puVar23 = *ppuVar1;
      }
      thunk_FUN_140205820(puVar23 + -2,0);
      *ppuVar1 = (undefined4 *)0x0;
    }
    if (puVar18 != (undefined4 *)0x0) {
      uVar28 = puVar18[-2];
      if (uVar28 != 0) {
        puVar23 = (undefined4 *)FUN_14019b780(&DAT_143ad68a0,(ulonglong)uVar28 * 0x50 + 8);
        if (puVar23 != (undefined4 *)0x0) {
          puVar23 = puVar23 + 2;
        }
        *ppuVar1 = puVar23;
        *(ulonglong *)(puVar23 + -2) = (ulonglong)uVar28;
        puVar23 = *ppuVar1;
        for (puVar40 = puVar18; puVar40 < puVar18 + (ulonglong)uVar28 * 0x14;
            puVar40 = puVar40 + 0x14) {
          *puVar23 = *puVar40;
          puVar23[1] = puVar40[1];
          *(undefined8 *)(puVar23 + 2) = 0;
          local_res18 = puVar23;
          FUN_14019a260(puVar23 + 2,puVar40 + 2);
          *(undefined8 *)(puVar23 + 4) = 0;
          FUN_14019a260(puVar23 + 4,puVar40 + 4);
          *(undefined8 *)(puVar23 + 6) = 0;
          FUN_14019a260(puVar23 + 6,puVar40 + 6);
          *(undefined8 *)(puVar23 + 8) = 0;
          FUN_14019a260(puVar23 + 8,puVar40 + 8);
          puVar23[10] = puVar40[10];
          *(undefined8 *)(puVar23 + 0xc) = 0;
          FUN_14019a260(puVar23 + 0xc,puVar40 + 0xc);
          puVar23[0xe] = puVar40[0xe];
          *(undefined1 *)(puVar23 + 0xf) = *(undefined1 *)(puVar40 + 0xf);
          uVar12 = puVar40[0x11];
          uVar5 = puVar40[0x12];
          uVar7 = puVar40[0x13];
          puVar23[0x10] = puVar40[0x10];
          puVar23[0x11] = uVar12;
          puVar23[0x12] = uVar5;
          puVar23[0x13] = uVar7;
          puVar23 = puVar23 + 0x14;
        }
      }
    }
  }
  *(undefined8 *)((longlong)local_res10 + 0x18) = 0;
  *(undefined8 *)((longlong)local_res10 + 0x20) = 0;
  if (puVar18 != (undefined4 *)0x0) {
    plVar25 = (longlong *)(puVar18 + -2);
    puVar23 = puVar18 + *plVar25 * 0x14;
    for (; puVar18 < puVar23; puVar18 = puVar18 + 0x14) {
      FUN_1411d1e90(puVar18);
    }
    thunk_FUN_140205820(plVar25,0);
  }
  if (pbVar29 != (byte *)0x0) {
    FUN_14019f2c0(pbVar29 + -0x10);
  }
LAB_1411bf18b:
  FUN_1411bf360(param_1);
  (**(code **)(*param_1 + 0x90))(param_1,0);
  FUN_1411d1460(&local_118,&local_118,*(longlong *)(local_118 + 8));
  thunk_FUN_140205820(local_118,0x30);
  if (local_res20 != 0) {
    plVar25 = (longlong *)(local_res20 - 8);
    uVar30 = *plVar25 * 0x50 + local_res20;
    for (uVar35 = local_res20; uVar35 < uVar30; uVar35 = uVar35 + 0x50) {
      FUN_1411d1e90(uVar35);
    }
    thunk_FUN_140205820(plVar25,0);
  }
  return;
}



//===========================================================
// FUN_142deda90 @ 142deda90   (2054 bytes)
//===========================================================

void FUN_142deda90(longlong *param_1,ulonglong *param_2)

{
  char cVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  longlong lVar4;
  undefined *puVar5;
  int iVar6;
  uint uVar7;
  undefined8 *puVar8;
  longlong *plVar9;
  longlong lVar10;
  int *piVar11;
  int iVar12;
  longlong *plVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  undefined8 *puVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  longlong *plVar19;
  longlong *plVar20;
  int iVar21;
  longlong *plVar22;
  longlong *plVar23;
  longlong *plVar24;
  longlong *plVar25;
  int iVar26;
  uint uVar27;
  longlong *local_res8;
  ulonglong *local_res10;
  longlong *local_res18;
  longlong *local_res20;
  
  local_res8 = param_1;
  local_res10 = param_2;
  if ((param_1[0x479] != 0) && (0 < *(int *)(param_1[0x479] + 0x20))) {
    iVar26 = 0;
    do {
      lVar10 = local_res8[0x479];
      iVar6 = 0;
      if (lVar10 != 0) {
        iVar6 = *(int *)(lVar10 + 0x20);
      }
      if (iVar6 <= iVar26) {
        return;
      }
      if (lVar10 == 0) {
LAB_142dedb4b:
        local_res18 = (longlong *)0x0;
      }
      else {
        puVar2 = *(undefined8 **)(lVar10 + 0x18);
        cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
        puVar3 = puVar2;
        puVar16 = (undefined8 *)puVar2[1];
        while (cVar1 == '\0') {
          if (*(int *)(puVar16 + 4) < iVar26) {
            puVar8 = (undefined8 *)puVar16[2];
            puVar16 = puVar3;
          }
          else {
            puVar8 = (undefined8 *)*puVar16;
          }
          puVar3 = puVar16;
          puVar16 = puVar8;
          cVar1 = *(char *)((longlong)puVar8 + 0x19);
        }
        if (((*(char *)((longlong)puVar3 + 0x19) != '\0') || (iVar26 < *(int *)(puVar3 + 4))) ||
           (puVar3 == puVar2)) goto LAB_142dedb4b;
        local_res18 = (longlong *)0x0;
        FUN_14019a260(&local_res18,puVar3 + 5);
      }
      plVar20 = local_res18;
      plVar9 = (longlong *)FUN_14022e850(param_2,0xffffffff);
      if (*plVar9 != 0) {
        FUN_14019f2c0(*plVar9 + -0x10);
      }
      *plVar9 = (longlong)plVar20;
      iVar26 = iVar26 + 1;
    } while( true );
  }
  plVar22 = (longlong *)0x0;
  iVar21 = 0;
  local_res20 = (longlong *)0x0;
  local_res18 = (longlong *)((ulonglong)local_res18 & 0xffffffff00000000);
  iVar6 = FUN_142cc3e40();
  plVar20 = (longlong *)0xffffffffffffffff;
  iVar12 = 0;
  plVar9 = plVar22;
  plVar19 = plVar22;
  iVar26 = iVar21;
  if (0 < iVar6) {
    do {
      lVar10 = FUN_142cc3ea0(param_1,plVar19);
      lVar10 = lVar10 + 0x16;
      local_res18 = (longlong *)0x0;
      plVar25 = plVar22;
      plVar23 = plVar20;
      if (lVar10 != 0) {
        do {
          plVar23 = (longlong *)((longlong)plVar23 + 1);
        } while (*(char *)(lVar10 + (longlong)plVar23) != '\0');
        iVar6 = (int)plVar23;
        iVar26 = iVar21;
        if (0 < iVar6) {
          iVar26 = iVar6;
        }
        piVar11 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar26 + 0x11));
        piVar11[1] = iVar26;
        *piVar11 = -1;
        plVar25 = (longlong *)(piVar11 + 4);
        piVar11[2] = 0;
        *(char *)plVar25 = '\0';
        local_res18 = plVar25;
        FUN_142ef7ba0(plVar25,lVar10,(longlong)iVar6);
        if (*piVar11 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar6 == -1) || (iVar6 <= piVar11[1])) {
          *piVar11 = 1;
          if (iVar6 != -1) goto LAB_142dedc53;
          plVar24 = plVar20;
          plVar23 = plVar22;
          if (plVar25 != (longlong *)0x0) {
            do {
              plVar23 = (longlong *)((longlong)plVar24 + 1);
              plVar24 = plVar23;
            } while (*(char *)((longlong)plVar25 + (longlong)plVar23) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar11[1],(ulonglong)plVar23 & 0xffffffff);
          *piVar11 = 1;
LAB_142dedc53:
          *(char *)((longlong)iVar6 + (longlong)plVar25) = '\0';
        }
        iVar26 = (int)plVar23;
        if ((iVar26 < 0) || (piVar11[1] + 1 <= iVar26)) {
          FUN_142e54290(0x9c,(ulonglong)plVar23 & 0xffffffff);
        }
        piVar11[2] = iVar26;
      }
      if (plVar9 == (longlong *)0x0) {
        uVar17 = 1;
        uVar27 = 0;
LAB_142dedce1:
        iVar26 = iVar12;
        if (plVar9 != (longlong *)0x0) {
          uVar18 = plVar9[-2];
          uVar14 = ~uVar18;
          if (-1 < (longlong)uVar18) {
            uVar14 = uVar18;
          }
          iVar26 = (int)(uVar14 - 8 >> 3);
        }
        if (iVar26 == (int)uVar17) goto LAB_142dedd74;
        plVar23 = plVar22;
        if (plVar9 != (longlong *)0x0) {
          plVar23 = (longlong *)(ulonglong)*(uint *)(plVar9 + -1);
        }
        lVar10 = FUN_14019b780(&DAT_143ad68a0,uVar17 * 8 + 8);
        plVar24 = (longlong *)(lVar10 + 8);
        if (lVar10 == 0) {
          plVar24 = plVar22;
        }
        if (plVar9 != (longlong *)0x0) {
          FUN_142ef7ba0(plVar24,plVar9,(longlong)plVar23 << 3);
          thunk_FUN_140205820(plVar9 + -1,0);
        }
        plVar24[-1] = (longlong)plVar23;
        plVar24[-1] = plVar24[-1] + 1;
        plVar23 = plVar24 + (int)uVar27;
        plVar9 = plVar24;
        local_res20 = plVar24;
      }
      else {
        uVar27 = *(uint *)(plVar9 + -1);
        uVar17 = plVar9[-2];
        uVar18 = ~uVar17;
        if (-1 < (longlong)uVar17) {
          uVar18 = uVar17;
        }
        if ((uint)(uVar18 - 8 >> 3) <= uVar27) {
          if (uVar27 == 0) {
            uVar17 = 1;
          }
          else {
            uVar17 = (ulonglong)(uVar27 * 2);
          }
          goto LAB_142dedce1;
        }
LAB_142dedd74:
        plVar9[-1] = plVar9[-1] + 1;
        plVar23 = plVar9 + (int)uVar27;
      }
      param_1 = local_res8;
      *plVar23 = 0;
      *plVar23 = (longlong)plVar25;
      uVar27 = (int)plVar19 + 1;
      iVar26 = FUN_142cc3e40(local_res8);
      plVar19 = (longlong *)(ulonglong)uVar27;
    } while ((int)uVar27 < iVar26);
    param_2 = local_res10;
    iVar26 = (int)plVar9[-1];
  }
  FUN_142df29e0(&local_res20,0,iVar26 + -1,&local_res8);
  plVar19 = (longlong *)*param_2;
  if (plVar19 != (longlong *)0x0) {
    plVar25 = plVar19 + plVar19[-1];
    if (plVar19 < plVar25) {
      do {
        if (*plVar19 != 0) {
          FUN_14019f2c0(*plVar19 + -0x10);
        }
        plVar19 = plVar19 + 1;
      } while (plVar19 < plVar25);
      plVar19 = (longlong *)*param_2;
    }
    thunk_FUN_140205820(plVar19 + -1,0);
    *param_2 = 0;
  }
  puVar5 = PTR_s_Default_Group_143a45268;
  uVar17 = 0;
  local_res8 = (longlong *)0x0;
  plVar25 = plVar20;
  plVar19 = plVar22;
  if (PTR_s_Default_Group_143a45268 != (undefined *)0x0) {
    do {
      plVar25 = (longlong *)((longlong)plVar25 + 1);
    } while (PTR_s_Default_Group_143a45268[(longlong)plVar25] != '\0');
    iVar6 = (int)plVar25;
    iVar26 = iVar12;
    if (0 < iVar6) {
      iVar26 = iVar6;
    }
    piVar11 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar26 + 0x11));
    piVar11[1] = iVar26;
    *piVar11 = -1;
    plVar19 = (longlong *)(piVar11 + 4);
    piVar11[2] = 0;
    *(char *)plVar19 = '\0';
    local_res8 = plVar19;
    FUN_142ef7ba0(plVar19,puVar5,(longlong)iVar6);
    if (*piVar11 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar6 == -1) || (iVar6 <= piVar11[1])) {
      *piVar11 = 1;
      if (iVar6 != -1) goto LAB_142dedeae;
      plVar25 = plVar22;
      if (plVar19 != (longlong *)0x0) {
        do {
          plVar20 = (longlong *)((longlong)plVar20 + 1);
        } while (*(char *)((longlong)plVar19 + (longlong)plVar20) != '\0');
        plVar25 = (longlong *)((ulonglong)plVar20 & 0xffffffff);
      }
    }
    else {
      FUN_142e54290(0x90,piVar11[1],(ulonglong)plVar25 & 0xffffffff);
      *piVar11 = 1;
LAB_142dedeae:
      *(char *)((longlong)iVar6 + (longlong)plVar19) = '\0';
    }
    iVar26 = (int)plVar25;
    if ((iVar26 < 0) || (piVar11[1] + 1 <= iVar26)) {
      FUN_142e54290(0x9c,(ulonglong)plVar25 & 0xffffffff);
    }
    piVar11[2] = iVar26;
    uVar17 = *param_2;
  }
  uVar27 = 0;
  if (uVar17 == 0) {
LAB_142dedf2d:
    uVar18 = 1;
    if (uVar17 != 0) goto LAB_142dedf3e;
  }
  else {
    uVar27 = *(uint *)(uVar17 - 8);
    uVar18 = *(ulonglong *)(uVar17 - 0x10);
    uVar14 = ~uVar18;
    if (-1 < (longlong)uVar18) {
      uVar14 = uVar18;
    }
    if (uVar27 < (uint)(uVar14 - 8 >> 3)) goto LAB_142dedfb9;
    if (uVar27 == 0) goto LAB_142dedf2d;
    uVar18 = (ulonglong)(uVar27 * 2);
LAB_142dedf3e:
    uVar14 = *(ulonglong *)(uVar17 - 0x10);
    uVar15 = ~uVar14;
    if (-1 < (longlong)uVar14) {
      uVar15 = uVar14;
    }
    iVar12 = (int)(uVar15 - 8 >> 3);
  }
  if (iVar12 != (int)uVar18) {
    plVar20 = plVar22;
    if (uVar17 != 0) {
      plVar20 = (longlong *)(ulonglong)*(uint *)(uVar17 - 8);
    }
    lVar10 = FUN_14019b780(&DAT_143ad68a0,uVar18 * 8 + 8);
    plVar25 = (longlong *)(lVar10 + 8);
    if (lVar10 == 0) {
      plVar25 = plVar22;
    }
    if (*param_2 != 0) {
      FUN_142ef7ba0(plVar25,*param_2,(longlong)plVar20 << 3);
      thunk_FUN_140205820(*param_2 - 8,0);
    }
    *param_2 = (ulonglong)plVar25;
    plVar25[-1] = (longlong)plVar20;
  }
LAB_142dedfb9:
  *(longlong *)(*param_2 - 8) = *(longlong *)(*param_2 - 8) + 1;
  *(undefined8 *)((longlong)(int)uVar27 * 8 + *param_2) = 0;
  plVar20 = (longlong *)(*param_2 + (longlong)(int)uVar27 * 8);
  lVar10 = *plVar20;
  if (lVar10 != 0) {
    FUN_14019f2c0(lVar10 + -0x10);
  }
  *plVar20 = (longlong)plVar19;
  local_res8 = (longlong *)0x0;
  plVar20 = plVar22;
  plVar19 = plVar22;
  plVar25 = plVar9;
  do {
    if ((plVar9 == (longlong *)0x0) || (uVar27 = (uint)plVar19, *(uint *)(plVar9 + -1) <= uVar27)) {
      if (plVar20 != (longlong *)0x0) {
        FUN_14019f2c0(plVar20 + -2);
      }
      if (plVar9 != (longlong *)0x0) {
        plVar20 = plVar9 + -1;
        plVar19 = plVar9 + *plVar20;
        for (; plVar9 < plVar19; plVar9 = plVar9 + 1) {
          if (*plVar9 != 0) {
            FUN_14019f2c0(*plVar9 + -0x10);
          }
        }
        thunk_FUN_140205820(plVar20,0);
      }
      return;
    }
    if ((int)uVar27 < 0) {
      FUN_142e54290(0xbc,plVar19);
    }
    plVar23 = DAT_143aa8360;
    if (PTR_s_Default_Group_143a45268 != (undefined *)0x0) {
      plVar23 = (longlong *)PTR_s_Default_Group_143a45268;
    }
    plVar24 = (longlong *)*plVar25;
    plVar13 = DAT_143aa8360;
    if (plVar24 != (longlong *)0x0) {
      plVar13 = plVar24;
    }
    lVar10 = (longlong)plVar23 - (longlong)plVar13;
    do {
      lVar4 = *plVar13;
      cVar1 = *(char *)((longlong)plVar13 + lVar10);
      if ((char)lVar4 != cVar1) break;
      plVar13 = (longlong *)((longlong)plVar13 + 1);
    } while (cVar1 != '\0');
    if ((char)lVar4 != cVar1) {
      if ((int)uVar27 < 1) {
LAB_142dee0d9:
        uVar7 = *(uint *)(plVar9 + -1);
        if ((int)uVar27 < 0) {
LAB_142dee0e8:
          FUN_142e54290(0xbc,plVar19);
        }
        else {
LAB_142dee0e4:
          if (uVar7 <= uVar27) goto LAB_142dee0e8;
        }
        uVar17 = *local_res10;
        uVar7 = 0;
        if (uVar17 == 0) {
LAB_142dee13d:
          uVar18 = 1;
          plVar20 = plVar22;
          if (uVar17 != 0) {
LAB_142dee151:
            uVar14 = *(ulonglong *)(uVar17 - 0x10);
            uVar15 = ~uVar14;
            if (-1 < (longlong)uVar14) {
              uVar15 = uVar14;
            }
            if ((int)(uVar15 - 8 >> 3) == (int)uVar18) goto LAB_142dee1de;
            if (uVar17 == 0) {
              plVar20 = (longlong *)0x0;
            }
            else {
              plVar20 = (longlong *)(ulonglong)*(uint *)(uVar17 - 8);
            }
          }
          lVar10 = FUN_14019b780(&DAT_143ad68a0,uVar18 * 8 + 8);
          plVar23 = (longlong *)(lVar10 + 8);
          if (lVar10 == 0) {
            plVar23 = plVar22;
          }
          if (*local_res10 != 0) {
            FUN_142ef7ba0(plVar23,*local_res10,(longlong)plVar20 << 3);
            thunk_FUN_140205820(*local_res10 - 8,0);
          }
          *local_res10 = (ulonglong)plVar23;
          plVar23[-1] = (longlong)plVar20;
        }
        else {
          uVar7 = *(uint *)(uVar17 - 8);
          uVar18 = *(ulonglong *)(uVar17 - 0x10);
          uVar14 = ~uVar18;
          if (-1 < (longlong)uVar18) {
            uVar14 = uVar18;
          }
          if ((uint)(uVar14 - 8 >> 3) <= uVar7) {
            if (uVar7 == 0) goto LAB_142dee13d;
            uVar18 = (ulonglong)(uVar7 * 2);
            goto LAB_142dee151;
          }
        }
LAB_142dee1de:
        *(longlong *)(*local_res10 - 8) = *(longlong *)(*local_res10 - 8) + 1;
        *(undefined8 *)((longlong)(int)uVar7 * 8 + *local_res10) = 0;
        FUN_14019a260((longlong)(int)uVar7 * 8 + *local_res10,plVar9 + (int)uVar27);
        if (((int)uVar27 < 0) || (*(uint *)(plVar9 + -1) <= uVar27)) {
          FUN_142e54290(0xbc,plVar19);
        }
        FUN_14019a260(&local_res8);
        plVar20 = local_res8;
      }
      else {
        if (*(uint *)(plVar9 + -1) <= uVar27) {
          FUN_142e54290(0xbc);
          plVar24 = (longlong *)*plVar25;
        }
        if (plVar24 != plVar20) {
          iVar26 = iVar21;
          if (plVar24 != (longlong *)0x0) {
            iVar26 = (int)plVar24[-1];
          }
          iVar6 = iVar21;
          if (plVar20 != (longlong *)0x0) {
            iVar6 = (int)plVar20[-1];
          }
          if (iVar26 != iVar6) {
            uVar7 = *(uint *)(plVar9 + -1);
            goto LAB_142dee0e4;
          }
          if (iVar26 != 0) {
            iVar26 = iVar21;
            if (plVar24 != (longlong *)0x0) {
              iVar26 = (int)plVar24[-1];
            }
            iVar26 = memcmp(plVar24,plVar20,(longlong)iVar26);
            if (iVar26 != 0) goto LAB_142dee0d9;
          }
        }
      }
    }
    plVar19 = (longlong *)(ulonglong)(uVar27 + 1);
    plVar25 = plVar25 + 1;
  } while( true );
}


