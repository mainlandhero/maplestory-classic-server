
//===========================================================
// FUN_1403e18a0 @ 1403e18a0   (4284 bytes)
//===========================================================

longlong * FUN_1403e18a0(longlong *param_1,int param_2,undefined8 param_3,undefined8 param_4)

{
  short *psVar1;
  undefined *puVar2;
  undefined *puVar3;
  undefined *puVar4;
  undefined *puVar5;
  undefined *puVar6;
  undefined *puVar7;
  undefined *puVar8;
  undefined *puVar9;
  undefined *puVar10;
  undefined *puVar11;
  undefined *puVar12;
  undefined *puVar13;
  undefined *puVar14;
  int iVar15;
  int *piVar16;
  undefined8 uVar17;
  longlong lVar18;
  int *piVar19;
  ulonglong uVar20;
  int iVar21;
  ulonglong uVar22;
  int *local_res18 [2];
  
  piVar19 = (int *)0x0;
  *param_1 = 0;
  puVar14 = PTR_u_Character_Ring__08d_img_143a46fe0;
  puVar13 = PTR_u_Character_Cape__08d_img_143a46fd8;
  puVar12 = PTR_u_Character_Shield__08d_img_143a46fd0;
  puVar11 = PTR_u_Character_Glove__08d_img_143a46fc8;
  puVar10 = PTR_u_Character_Shoes__08d_img_143a46fc0;
  puVar9 = PTR_u_Character_Pants__08d_img_143a46fb8;
  puVar8 = PTR_u_Character_Longcoat__08d_img_143a46fb0;
  puVar7 = PTR_u_Character_Coat__08d_img_143a46fa8;
  puVar6 = PTR_u_Character_Accessory__08d_img_143a46fa0;
  puVar5 = PTR_u_Character_Cap__08d_img_143a46f98;
  puVar4 = PTR_u_Character_Hair__08d_img_143a46f90;
  puVar3 = PTR_u_Character_Face__08d_img_143a46f88;
  puVar2 = PTR_u_Character__08d_img_143a46f80;
  iVar15 = 0;
  iVar21 = 0;
  switch(param_2 / 10000) {
  case 0:
  case 1:
    local_res18[0] = (int *)0x0;
    if (PTR_u_Character__08d_img_143a46f80 != (undefined *)0x0) {
      uVar20 = 0xffffffffffffffff;
      uVar22 = 0xffffffffffffffff;
      do {
        uVar22 = uVar22 + 1;
      } while (*(short *)(PTR_u_Character__08d_img_143a46f80 + uVar22 * 2) != 0);
      iVar21 = (int)uVar22;
      iVar15 = 0;
      if (0 < iVar21) {
        iVar15 = iVar21;
      }
      piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar15 * 2 + 0x12));
      piVar16[1] = iVar15;
      *piVar16 = -1;
      piVar19 = piVar16 + 4;
      piVar16[2] = 0;
      *(undefined2 *)piVar19 = 0;
      local_res18[0] = piVar19;
      FUN_142ef7ba0(piVar19,puVar2,(longlong)iVar21 * 2);
      if (*piVar16 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar21 == -1) || (iVar21 <= piVar16[1])) {
        *piVar16 = 1;
        if (iVar21 != -1) goto LAB_1403e19b7;
        if (piVar19 == (int *)0x0) {
          uVar22 = 0;
        }
        else {
          do {
            uVar20 = uVar20 + 1;
          } while (*(short *)((longlong)piVar19 + uVar20 * 2) != 0);
          uVar22 = uVar20 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar16[1],uVar22 & 0xffffffff);
        *piVar16 = 1;
LAB_1403e19b7:
        *(undefined2 *)((longlong)iVar21 * 2 + (longlong)piVar19) = 0;
      }
      iVar15 = (int)uVar22;
      if ((iVar15 < 0) || (piVar16[1] + 1 <= iVar15)) {
        FUN_142e54290(0x9c,uVar22 & 0xffffffff);
      }
      piVar16[2] = iVar15 * 2;
    }
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)piVar19;
    break;
  case 2:
  case 5:
    local_res18[0] = (int *)0x0;
    if (PTR_u_Character_Face__08d_img_143a46f88 != (undefined *)0x0) {
      uVar20 = 0xffffffffffffffff;
      uVar22 = 0xffffffffffffffff;
      do {
        uVar22 = uVar22 + 1;
      } while (*(short *)(PTR_u_Character_Face__08d_img_143a46f88 + uVar22 * 2) != 0);
      iVar21 = (int)uVar22;
      if (0 < iVar21) {
        iVar15 = iVar21;
      }
      piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar15 * 2 + 0x12));
      piVar16[1] = iVar15;
      *piVar16 = -1;
      piVar19 = piVar16 + 4;
      piVar16[2] = 0;
      *(undefined2 *)piVar19 = 0;
      local_res18[0] = piVar19;
      FUN_142ef7ba0(piVar19,puVar3,(longlong)iVar21 * 2);
      if (*piVar16 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar21 == -1) || (iVar21 <= piVar16[1])) {
        *piVar16 = 1;
        if (iVar21 != -1) goto LAB_1403e1ae6;
        if (piVar19 == (int *)0x0) {
          uVar22 = 0;
        }
        else {
          do {
            uVar20 = uVar20 + 1;
          } while (*(short *)((longlong)piVar19 + uVar20 * 2) != 0);
          uVar22 = uVar20 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar16[1],uVar22 & 0xffffffff);
        *piVar16 = 1;
LAB_1403e1ae6:
        *(undefined2 *)((longlong)iVar21 * 2 + (longlong)piVar19) = 0;
      }
      iVar15 = (int)uVar22;
      if ((iVar15 < 0) || (piVar16[1] + 1 <= iVar15)) {
        FUN_142e54290(0x9c,uVar22 & 0xffffffff);
      }
      piVar16[2] = iVar15 * 2;
    }
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)piVar19;
    break;
  case 3:
  case 4:
  case 6:
    local_res18[0] = (int *)0x0;
    if (PTR_u_Character_Hair__08d_img_143a46f90 != (undefined *)0x0) {
      uVar20 = 0xffffffffffffffff;
      uVar22 = 0xffffffffffffffff;
      do {
        uVar22 = uVar22 + 1;
      } while (*(short *)(PTR_u_Character_Hair__08d_img_143a46f90 + uVar22 * 2) != 0);
      iVar21 = (int)uVar22;
      if (0 < iVar21) {
        iVar15 = iVar21;
      }
      piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar15 * 2 + 0x12));
      piVar16[1] = iVar15;
      *piVar16 = -1;
      piVar19 = piVar16 + 4;
      piVar16[2] = 0;
      *(undefined2 *)piVar19 = 0;
      local_res18[0] = piVar19;
      FUN_142ef7ba0(piVar19,puVar4,(longlong)iVar21 * 2);
      if (*piVar16 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar21 == -1) || (iVar21 <= piVar16[1])) {
        *piVar16 = 1;
        if (iVar21 != -1) goto LAB_1403e1c06;
        if (piVar19 == (int *)0x0) {
          uVar22 = 0;
        }
        else {
          do {
            uVar20 = uVar20 + 1;
          } while (*(short *)((longlong)piVar19 + uVar20 * 2) != 0);
          uVar22 = uVar20 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar16[1],uVar22 & 0xffffffff);
        *piVar16 = 1;
LAB_1403e1c06:
        *(undefined2 *)((longlong)iVar21 * 2 + (longlong)piVar19) = 0;
      }
      iVar15 = (int)uVar22;
      if ((iVar15 < 0) || (piVar16[1] + 1 <= iVar15)) {
        FUN_142e54290(0x9c,uVar22 & 0xffffffff);
      }
      piVar16[2] = iVar15 * 2;
    }
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)piVar19;
    break;
  default:
    iVar15 = FUN_140255180(param_2);
    if ((((iVar15 == 0) && (99999 < param_2 - 1600000U)) && (param_2 / 10000 != 0xaa)) &&
       (iVar15 = FUN_140416360(param_2), iVar15 == 0)) goto LAB_1403e292b;
    FUN_1403edf80(local_res18,PTR_u_Character_Weapon__08d_img_143a46ff0,0xffffffffffffffff);
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)local_res18[0];
    break;
  case 100:
    local_res18[0] = (int *)0x0;
    if (PTR_u_Character_Cap__08d_img_143a46f98 != (undefined *)0x0) {
      uVar20 = 0xffffffffffffffff;
      uVar22 = 0xffffffffffffffff;
      do {
        uVar22 = uVar22 + 1;
      } while (*(short *)(PTR_u_Character_Cap__08d_img_143a46f98 + uVar22 * 2) != 0);
      iVar21 = (int)uVar22;
      if (0 < iVar21) {
        iVar15 = iVar21;
      }
      piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar15 * 2 + 0x12));
      piVar16[1] = iVar15;
      *piVar16 = -1;
      piVar19 = piVar16 + 4;
      piVar16[2] = 0;
      *(undefined2 *)piVar19 = 0;
      local_res18[0] = piVar19;
      FUN_142ef7ba0(piVar19,puVar5,(longlong)iVar21 * 2);
      if (*piVar16 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar21 == -1) || (iVar21 <= piVar16[1])) {
        *piVar16 = 1;
        if (iVar21 != -1) goto LAB_1403e1d26;
        if (piVar19 == (int *)0x0) {
          uVar22 = 0;
        }
        else {
          do {
            uVar20 = uVar20 + 1;
          } while (*(short *)((longlong)piVar19 + uVar20 * 2) != 0);
          uVar22 = uVar20 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar16[1],uVar22 & 0xffffffff);
        *piVar16 = 1;
LAB_1403e1d26:
        *(undefined2 *)((longlong)iVar21 * 2 + (longlong)piVar19) = 0;
      }
      iVar15 = (int)uVar22;
      if ((iVar15 < 0) || (piVar16[1] + 1 <= iVar15)) {
        FUN_142e54290(0x9c,uVar22 & 0xffffffff);
      }
      piVar16[2] = iVar15 * 2;
    }
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)piVar19;
    break;
  case 0x65:
  case 0x66:
  case 0x67:
  case 0x70:
  case 0x71:
  case 0x72:
  case 0x73:
  case 0x74:
  case 0x76:
  case 0x77:
    local_res18[0] = (int *)0x0;
    if (PTR_u_Character_Accessory__08d_img_143a46fa0 != (undefined *)0x0) {
      uVar20 = 0xffffffffffffffff;
      uVar22 = 0xffffffffffffffff;
      do {
        uVar22 = uVar22 + 1;
      } while (*(short *)(PTR_u_Character_Accessory__08d_img_143a46fa0 + uVar22 * 2) != 0);
      iVar21 = (int)uVar22;
      if (0 < iVar21) {
        iVar15 = iVar21;
      }
      piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar15 * 2 + 0x12));
      piVar16[1] = iVar15;
      *piVar16 = -1;
      piVar19 = piVar16 + 4;
      piVar16[2] = 0;
      *(undefined2 *)piVar19 = 0;
      local_res18[0] = piVar19;
      FUN_142ef7ba0(piVar19,puVar6,(longlong)iVar21 * 2);
      if (*piVar16 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar21 == -1) || (iVar21 <= piVar16[1])) {
        *piVar16 = 1;
        if (iVar21 != -1) goto LAB_1403e1e46;
        if (piVar19 == (int *)0x0) {
          uVar22 = 0;
        }
        else {
          do {
            uVar20 = uVar20 + 1;
          } while (*(short *)((longlong)piVar19 + uVar20 * 2) != 0);
          uVar22 = uVar20 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar16[1],uVar22 & 0xffffffff);
        *piVar16 = 1;
LAB_1403e1e46:
        *(undefined2 *)((longlong)iVar21 * 2 + (longlong)piVar19) = 0;
      }
      iVar15 = (int)uVar22;
      if ((iVar15 < 0) || (piVar16[1] + 1 <= iVar15)) {
        FUN_142e54290(0x9c,uVar22 & 0xffffffff);
      }
      piVar16[2] = iVar15 * 2;
    }
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)piVar19;
    break;
  case 0x68:
    local_res18[0] = (int *)0x0;
    if (PTR_u_Character_Coat__08d_img_143a46fa8 != (undefined *)0x0) {
      uVar20 = 0xffffffffffffffff;
      uVar22 = 0xffffffffffffffff;
      do {
        uVar22 = uVar22 + 1;
      } while (*(short *)(PTR_u_Character_Coat__08d_img_143a46fa8 + uVar22 * 2) != 0);
      iVar21 = (int)uVar22;
      if (0 < iVar21) {
        iVar15 = iVar21;
      }
      piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar15 * 2 + 0x12));
      piVar16[1] = iVar15;
      *piVar16 = -1;
      piVar19 = piVar16 + 4;
      piVar16[2] = 0;
      *(undefined2 *)piVar19 = 0;
      local_res18[0] = piVar19;
      FUN_142ef7ba0(piVar19,puVar7,(longlong)iVar21 * 2);
      if (*piVar16 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar21 == -1) || (iVar21 <= piVar16[1])) {
        *piVar16 = 1;
        if (iVar21 != -1) goto LAB_1403e1f66;
        if (piVar19 == (int *)0x0) {
          uVar22 = 0;
        }
        else {
          do {
            uVar20 = uVar20 + 1;
          } while (*(short *)((longlong)piVar19 + uVar20 * 2) != 0);
          uVar22 = uVar20 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar16[1],uVar22 & 0xffffffff);
        *piVar16 = 1;
LAB_1403e1f66:
        *(undefined2 *)((longlong)iVar21 * 2 + (longlong)piVar19) = 0;
      }
      iVar15 = (int)uVar22;
      if ((iVar15 < 0) || (piVar16[1] + 1 <= iVar15)) {
        FUN_142e54290(0x9c,uVar22 & 0xffffffff);
      }
      piVar16[2] = iVar15 * 2;
    }
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)piVar19;
    break;
  case 0x69:
    local_res18[0] = (int *)0x0;
    if (PTR_u_Character_Longcoat__08d_img_143a46fb0 != (undefined *)0x0) {
      uVar20 = 0xffffffffffffffff;
      uVar22 = 0xffffffffffffffff;
      do {
        uVar22 = uVar22 + 1;
      } while (*(short *)(PTR_u_Character_Longcoat__08d_img_143a46fb0 + uVar22 * 2) != 0);
      iVar21 = (int)uVar22;
      if (0 < iVar21) {
        iVar15 = iVar21;
      }
      piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar15 * 2 + 0x12));
      piVar16[1] = iVar15;
      *piVar16 = -1;
      piVar19 = piVar16 + 4;
      piVar16[2] = 0;
      *(undefined2 *)piVar19 = 0;
      local_res18[0] = piVar19;
      FUN_142ef7ba0(piVar19,puVar8,(longlong)iVar21 * 2);
      if (*piVar16 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar21 == -1) || (iVar21 <= piVar16[1])) {
        *piVar16 = 1;
        if (iVar21 != -1) goto LAB_1403e2086;
        if (piVar19 == (int *)0x0) {
          uVar22 = 0;
        }
        else {
          do {
            uVar20 = uVar20 + 1;
          } while (*(short *)((longlong)piVar19 + uVar20 * 2) != 0);
          uVar22 = uVar20 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar16[1],uVar22 & 0xffffffff);
        *piVar16 = 1;
LAB_1403e2086:
        *(undefined2 *)((longlong)iVar21 * 2 + (longlong)piVar19) = 0;
      }
      iVar15 = (int)uVar22;
      if ((iVar15 < 0) || (piVar16[1] + 1 <= iVar15)) {
        FUN_142e54290(0x9c,uVar22 & 0xffffffff);
      }
      piVar16[2] = iVar15 * 2;
    }
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)piVar19;
    break;
  case 0x6a:
    local_res18[0] = (int *)0x0;
    if (PTR_u_Character_Pants__08d_img_143a46fb8 != (undefined *)0x0) {
      uVar20 = 0xffffffffffffffff;
      uVar22 = 0xffffffffffffffff;
      do {
        uVar22 = uVar22 + 1;
      } while (*(short *)(PTR_u_Character_Pants__08d_img_143a46fb8 + uVar22 * 2) != 0);
      iVar21 = (int)uVar22;
      if (0 < iVar21) {
        iVar15 = iVar21;
      }
      piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar15 * 2 + 0x12));
      piVar16[1] = iVar15;
      *piVar16 = -1;
      piVar19 = piVar16 + 4;
      piVar16[2] = 0;
      *(undefined2 *)piVar19 = 0;
      local_res18[0] = piVar19;
      FUN_142ef7ba0(piVar19,puVar9,(longlong)iVar21 * 2);
      if (*piVar16 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar21 == -1) || (iVar21 <= piVar16[1])) {
        *piVar16 = 1;
        if (iVar21 != -1) goto LAB_1403e21a6;
        if (piVar19 == (int *)0x0) {
          uVar22 = 0;
        }
        else {
          do {
            uVar20 = uVar20 + 1;
          } while (*(short *)((longlong)piVar19 + uVar20 * 2) != 0);
          uVar22 = uVar20 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar16[1],uVar22 & 0xffffffff);
        *piVar16 = 1;
LAB_1403e21a6:
        *(undefined2 *)((longlong)iVar21 * 2 + (longlong)piVar19) = 0;
      }
      iVar15 = (int)uVar22;
      if ((iVar15 < 0) || (piVar16[1] + 1 <= iVar15)) {
        FUN_142e54290(0x9c,uVar22 & 0xffffffff);
      }
      piVar16[2] = iVar15 * 2;
    }
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)piVar19;
    break;
  case 0x6b:
    local_res18[0] = (int *)0x0;
    if (PTR_u_Character_Shoes__08d_img_143a46fc0 != (undefined *)0x0) {
      uVar20 = 0xffffffffffffffff;
      uVar22 = 0xffffffffffffffff;
      do {
        uVar22 = uVar22 + 1;
      } while (*(short *)(PTR_u_Character_Shoes__08d_img_143a46fc0 + uVar22 * 2) != 0);
      iVar21 = (int)uVar22;
      if (0 < iVar21) {
        iVar15 = iVar21;
      }
      piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar15 * 2 + 0x12));
      piVar16[1] = iVar15;
      *piVar16 = -1;
      piVar19 = piVar16 + 4;
      piVar16[2] = 0;
      *(undefined2 *)piVar19 = 0;
      local_res18[0] = piVar19;
      FUN_142ef7ba0(piVar19,puVar10,(longlong)iVar21 * 2);
      if (*piVar16 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar21 == -1) || (iVar21 <= piVar16[1])) {
        *piVar16 = 1;
        if (iVar21 != -1) goto LAB_1403e22c6;
        if (piVar19 == (int *)0x0) {
          uVar22 = 0;
        }
        else {
          do {
            uVar20 = uVar20 + 1;
          } while (*(short *)((longlong)piVar19 + uVar20 * 2) != 0);
          uVar22 = uVar20 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar16[1],uVar22 & 0xffffffff);
        *piVar16 = 1;
LAB_1403e22c6:
        *(undefined2 *)((longlong)iVar21 * 2 + (longlong)piVar19) = 0;
      }
      iVar15 = (int)uVar22;
      if ((iVar15 < 0) || (piVar16[1] + 1 <= iVar15)) {
        FUN_142e54290(0x9c,uVar22 & 0xffffffff);
      }
      piVar16[2] = iVar15 * 2;
    }
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)piVar19;
    break;
  case 0x6c:
    local_res18[0] = (int *)0x0;
    if (PTR_u_Character_Glove__08d_img_143a46fc8 != (undefined *)0x0) {
      uVar20 = 0xffffffffffffffff;
      uVar22 = 0xffffffffffffffff;
      do {
        uVar22 = uVar22 + 1;
      } while (*(short *)(PTR_u_Character_Glove__08d_img_143a46fc8 + uVar22 * 2) != 0);
      iVar15 = (int)uVar22;
      if (0 < iVar15) {
        iVar21 = iVar15;
      }
      piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar21 * 2 + 0x12));
      piVar16[1] = iVar21;
      *piVar16 = -1;
      piVar19 = piVar16 + 4;
      piVar16[2] = 0;
      *(undefined2 *)piVar19 = 0;
      local_res18[0] = piVar19;
      FUN_142ef7ba0(piVar19,puVar11,(longlong)iVar15 * 2);
      if (*piVar16 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar15 == -1) || (iVar15 <= piVar16[1])) {
        *piVar16 = 1;
        if (iVar15 != -1) goto LAB_1403e23e6;
        if (piVar19 == (int *)0x0) {
          uVar22 = 0;
        }
        else {
          do {
            uVar20 = uVar20 + 1;
          } while (*(short *)((longlong)piVar19 + uVar20 * 2) != 0);
          uVar22 = uVar20 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar16[1],uVar22 & 0xffffffff);
        *piVar16 = 1;
LAB_1403e23e6:
        *(undefined2 *)((longlong)iVar15 * 2 + (longlong)piVar19) = 0;
      }
      iVar15 = (int)uVar22;
      if ((iVar15 < 0) || (piVar16[1] + 1 <= iVar15)) {
        FUN_142e54290(0x9c,uVar22 & 0xffffffff);
      }
      piVar16[2] = iVar15 * 2;
    }
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)piVar19;
    break;
  case 0x6d:
    local_res18[0] = (int *)0x0;
    if (PTR_u_Character_Shield__08d_img_143a46fd0 != (undefined *)0x0) {
      uVar20 = 0xffffffffffffffff;
      uVar22 = 0xffffffffffffffff;
      do {
        uVar22 = uVar22 + 1;
      } while (*(short *)(PTR_u_Character_Shield__08d_img_143a46fd0 + uVar22 * 2) != 0);
      iVar15 = (int)uVar22;
      if (0 < iVar15) {
        iVar21 = iVar15;
      }
      piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar21 * 2 + 0x12));
      piVar16[1] = iVar21;
      *piVar16 = -1;
      piVar19 = piVar16 + 4;
      piVar16[2] = 0;
      *(undefined2 *)piVar19 = 0;
      local_res18[0] = piVar19;
      FUN_142ef7ba0(piVar19,puVar12,(longlong)iVar15 * 2);
      if (*piVar16 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar15 == -1) || (iVar15 <= piVar16[1])) {
        *piVar16 = 1;
        if (iVar15 != -1) goto LAB_1403e2506;
        if (piVar19 == (int *)0x0) {
          uVar22 = 0;
        }
        else {
          do {
            uVar20 = uVar20 + 1;
          } while (*(short *)((longlong)piVar19 + uVar20 * 2) != 0);
          uVar22 = uVar20 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar16[1],uVar22 & 0xffffffff);
        *piVar16 = 1;
LAB_1403e2506:
        *(undefined2 *)((longlong)iVar15 * 2 + (longlong)piVar19) = 0;
      }
      iVar15 = (int)uVar22;
      if ((iVar15 < 0) || (piVar16[1] + 1 <= iVar15)) {
        FUN_142e54290(0x9c,uVar22 & 0xffffffff);
      }
      piVar16[2] = iVar15 * 2;
    }
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)piVar19;
    break;
  case 0x6e:
    local_res18[0] = (int *)0x0;
    if (PTR_u_Character_Cape__08d_img_143a46fd8 != (undefined *)0x0) {
      uVar20 = 0xffffffffffffffff;
      uVar22 = 0xffffffffffffffff;
      do {
        uVar22 = uVar22 + 1;
      } while (*(short *)(PTR_u_Character_Cape__08d_img_143a46fd8 + uVar22 * 2) != 0);
      iVar15 = (int)uVar22;
      if (0 < iVar15) {
        iVar21 = iVar15;
      }
      piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar21 * 2 + 0x12));
      piVar16[1] = iVar21;
      *piVar16 = -1;
      piVar19 = piVar16 + 4;
      piVar16[2] = 0;
      *(undefined2 *)piVar19 = 0;
      local_res18[0] = piVar19;
      FUN_142ef7ba0(piVar19,puVar13,(longlong)iVar15 * 2);
      if (*piVar16 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar15 == -1) || (iVar15 <= piVar16[1])) {
        *piVar16 = 1;
        if (iVar15 != -1) goto LAB_1403e2626;
        if (piVar19 == (int *)0x0) {
          uVar22 = 0;
        }
        else {
          do {
            uVar20 = uVar20 + 1;
          } while (*(short *)((longlong)piVar19 + uVar20 * 2) != 0);
          uVar22 = uVar20 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar16[1],uVar22 & 0xffffffff);
        *piVar16 = 1;
LAB_1403e2626:
        *(undefined2 *)((longlong)iVar15 * 2 + (longlong)piVar19) = 0;
      }
      iVar15 = (int)uVar22;
      if ((iVar15 < 0) || (piVar16[1] + 1 <= iVar15)) {
        FUN_142e54290(0x9c,uVar22 & 0xffffffff);
      }
      piVar16[2] = iVar15 * 2;
    }
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)piVar19;
    break;
  case 0x6f:
    local_res18[0] = (int *)0x0;
    if (PTR_u_Character_Ring__08d_img_143a46fe0 != (undefined *)0x0) {
      uVar20 = 0xffffffffffffffff;
      uVar22 = 0xffffffffffffffff;
      do {
        uVar22 = uVar22 + 1;
      } while (*(short *)(PTR_u_Character_Ring__08d_img_143a46fe0 + uVar22 * 2) != 0);
      iVar15 = (int)uVar22;
      if (0 < iVar15) {
        iVar21 = iVar15;
      }
      piVar16 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar21 * 2 + 0x12));
      piVar16[1] = iVar21;
      *piVar16 = -1;
      piVar19 = piVar16 + 4;
      piVar16[2] = 0;
      *(undefined2 *)piVar19 = 0;
      local_res18[0] = piVar19;
      FUN_142ef7ba0(piVar19,puVar14,(longlong)iVar15 * 2);
      if (*piVar16 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar15 == -1) || (iVar15 <= piVar16[1])) {
        *piVar16 = 1;
        if (iVar15 != -1) goto LAB_1403e2746;
        if (piVar19 == (int *)0x0) {
          uVar22 = 0;
        }
        else {
          do {
            uVar20 = uVar20 + 1;
          } while (*(short *)((longlong)piVar19 + uVar20 * 2) != 0);
          uVar22 = uVar20 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar16[1],uVar22 & 0xffffffff);
        *piVar16 = 1;
LAB_1403e2746:
        *(undefined2 *)((longlong)iVar15 * 2 + (longlong)piVar19) = 0;
      }
      iVar15 = (int)uVar22;
      if ((iVar15 < 0) || (piVar16[1] + 1 <= iVar15)) {
        FUN_142e54290(0x9c,uVar22 & 0xffffffff);
      }
      piVar16[2] = iVar15 * 2;
    }
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)piVar19;
    break;
  case 0xa6:
  case 0xa7:
    uVar17 = FUN_1408a9ee0(0x6b6);
    FUN_1403edf80(local_res18,uVar17,0xffffffffffffffff);
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)local_res18[0];
    break;
  case 0xa8:
    uVar17 = FUN_1408a9ee0(0x6b7);
    FUN_1403edf80(local_res18,uVar17,0xffffffffffffffff);
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)local_res18[0];
    break;
  case 0xb4:
    FUN_1403edf80(local_res18,PTR_u_Character_PetEquip__08d_img_143a46fe8,0xffffffffffffffff,param_4
                  ,1);
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)local_res18[0];
    break;
  case 0xbe:
  case 0xbf:
  case 0xc1:
  case 0xc6:
    uVar17 = FUN_1408a9ee0(0x6b4);
    FUN_1403edf80(local_res18,uVar17,0xffffffffffffffff);
    if (*param_1 != 0) {
      FUN_1401bebb0(*param_1 + -0x10);
      *param_1 = 0;
    }
    lVar18 = *param_1;
    *param_1 = (longlong)local_res18[0];
  }
  if (lVar18 != 0) {
    FUN_1401bebb0(lVar18 + -0x10);
  }
LAB_1403e292b:
  psVar1 = (short *)*param_1;
  if ((psVar1 != (short *)0x0) && (*psVar1 != 0)) {
    FUN_1401c21c0(param_1,psVar1,param_2);
  }
  return param_1;
}



//===========================================================
// FUN_1403e2a80 @ 1403e2a80   (134 bytes)
//===========================================================

undefined8 * FUN_1403e2a80(undefined8 *param_1,undefined8 param_2,ushort param_3)

{
  undefined8 uVar1;
  undefined8 *puVar2;
  longlong local_res20;
  undefined4 uVar3;
  
  FUN_1403e18a0();
  uVar3 = 1;
  if ((ushort)(param_3 - 1) < 6) {
    uVar1 = *param_1;
    puVar2 = (undefined8 *)FUN_1408a9d20(&local_res20,0x6b8);
    FUN_1401c21c0(param_1,*puVar2,uVar1,param_3 - 1,uVar3);
    if (local_res20 != 0) {
      FUN_1401bebb0(local_res20 + -0x10);
    }
  }
  return param_1;
}



//===========================================================
// FUN_140ce6090 @ 140ce6090   (3665 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000140ce6416) */
/* WARNING: Removing unreachable block (ram,0x000140ce625c) */
/* WARNING: Removing unreachable block (ram,0x000140ce6334) */

void FUN_140ce6090(int param_1)

{
  char cVar1;
  undefined8 *puVar2;
  longlong *plVar3;
  ulonglong uVar4;
  IUnknown *pIVar5;
  longlong *plVar6;
  longlong lVar7;
  int iVar8;
  undefined4 uVar9;
  int *piVar10;
  int *piVar11;
  longlong *plVar12;
  int *piVar13;
  undefined8 uVar14;
  undefined8 *puVar15;
  undefined8 *puVar16;
  longlong lVar17;
  ulonglong *puVar18;
  undefined8 *puVar19;
  int iVar20;
  uint uVar21;
  int *piVar22;
  int iVar23;
  ulonglong local_res10;
  uint local_res18 [2];
  uint local_res20 [2];
  longlong *local_188;
  longlong local_180;
  int *local_178;
  IUnknown *local_170;
  longlong *local_168;
  longlong *local_160;
  longlong *local_158;
  int *local_150;
  int *local_148;
  longlong local_140;
  longlong *local_138;
  short *local_130;
  short *local_128;
  longlong *local_120;
  longlong *local_118;
  longlong *local_110;
  longlong *local_108;
  longlong *local_100;
  longlong local_f8;
  longlong local_f0;
  longlong local_e8;
  longlong local_e0;
  short *local_d8;
  short local_d0 [4];
  longlong *local_c8;
  short local_b8 [4];
  longlong local_b0;
  longlong local_98;
  longlong local_90;
  longlong **local_88;
  undefined8 *local_80;
  longlong **local_78;
  undefined8 *local_70;
  longlong **local_68;
  undefined8 *local_60;
  longlong **local_58;
  undefined8 *local_50;
  
  if ((999999 < param_1 - 1000000U) && (999999 < param_1 - 6000000U)) {
    return;
  }
  FUN_1403e18a0(&local_d8,param_1);
  if ((local_d8 == (short *)0x0) || (*local_d8 == 0)) goto LAB_140ce6e73;
  FUN_14090ead0(&local_120,local_d8);
  if (local_120 != (longlong *)0x0) {
    FUN_140253130(param_1);
    iVar23 = 0;
    local_178 = (int *)0x0;
    piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar10[1] = 0;
    *piVar10 = -1;
    local_178 = piVar10 + 4;
    piVar10[2] = 0;
    *(undefined1 *)local_178 = 0;
    if (*piVar10 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar10[1] < 0) {
      FUN_142e54290(0x90,piVar10[1],0);
    }
    *piVar10 = 1;
    *(undefined1 *)local_178 = 0;
    if (piVar10[1] + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    piVar10[2] = 0;
    if (param_1 - 0x100590U < 10000) {
      piVar11 = (int *)FUN_14019b600(&DAT_143ad6a30,0x15);
      piVar11[1] = 4;
      *piVar11 = -1;
      piVar10 = piVar11 + 4;
      piVar11[2] = 0;
      *(undefined1 *)piVar10 = 0;
      *piVar10 = DAT_14336b94c;
      local_150 = piVar10;
      if (*piVar11 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar11[1] < 4) {
        FUN_142e54290(0x90,piVar11[1],4);
      }
      *piVar11 = 1;
      *(undefined1 *)(piVar11 + 5) = 0;
      if (piVar11[1] + 1 < 5) {
        FUN_142e54290(0x9c,4);
      }
      piVar11[2] = 4;
      if (local_178 != (int *)0x0) {
        FUN_14019f2c0(local_178 + -4);
      }
      local_res10 = CONCAT44(local_res10._4_4_,0x101325);
      local_178 = piVar10;
LAB_140ce642a:
      if (DAT_143ac0188 != 0) {
        local_188 = (longlong *)0x0;
        local_180 = 0;
        local_188 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x18);
        *local_188 = (longlong)local_188;
        local_188[1] = (longlong)local_188;
        do {
          piVar11 = (int *)0x0;
          plVar12 = (longlong *)FUN_1402b2b70(&local_100,iVar23);
          local_148 = (int *)0x0;
          piVar10 = piVar11;
          if (((longlong *)*plVar12 != (longlong *)0x0) &&
             (lVar17 = *(longlong *)*plVar12, lVar17 != 0)) {
            piVar22 = (int *)0xffffffffffffffff;
            do {
              piVar22 = (int *)((longlong)piVar22 + 1);
            } while (*(short *)(lVar17 + (longlong)piVar22 * 2) != 0);
            iVar8 = (int)piVar22;
            iVar20 = 0;
            if (0 < iVar8) {
              iVar20 = iVar8;
            }
            piVar13 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar20 * 2 + 0x12));
            piVar13[1] = iVar20;
            *piVar13 = -1;
            piVar10 = piVar13 + 4;
            piVar13[2] = 0;
            *(short *)piVar10 = 0;
            local_148 = piVar10;
            FUN_142ef7ba0(piVar10,lVar17,(longlong)iVar8 * 2);
            if (*piVar13 != -1) {
              FUN_142e52dd0();
            }
            if ((iVar8 == -1) || (iVar8 <= piVar13[1])) {
              *piVar13 = 1;
              if (iVar8 != -1) goto LAB_140ce6536;
              piVar22 = piVar11;
              if (piVar10 != (int *)0x0) {
                piVar22 = (int *)0xffffffffffffffff;
                do {
                  piVar22 = (int *)((longlong)piVar22 + 1);
                } while (*(short *)((longlong)piVar10 + (longlong)piVar22 * 2) != 0);
              }
            }
            else {
              FUN_142e54290(0x90,piVar13[1],(ulonglong)piVar22 & 0xffffffff);
              *piVar13 = 1;
LAB_140ce6536:
              *(short *)((longlong)iVar8 * 2 + (longlong)piVar10) = 0;
            }
            iVar20 = (int)piVar22;
            if ((iVar20 < 0) || (piVar13[1] + 1 <= iVar20)) {
              FUN_142e54290(0x9c,(ulonglong)piVar22 & 0xffffffff);
            }
            piVar13[2] = iVar20 * 2;
          }
          plVar12 = local_100;
          if (local_100 != (longlong *)0x0) {
            LOCK();
            plVar3 = local_100 + 2;
            lVar17 = *plVar3;
            *(int *)plVar3 = (int)*plVar3 + -1;
            UNLOCK();
            if (((int)lVar17 == 1) && (local_100 != (longlong *)0x0)) {
              if (*local_100 != 0) {
                (*DAT_143ad5990)(*local_100 + -4);
                *plVar12 = 0;
              }
              if (plVar12[1] != 0) {
                FUN_14019b4e0();
                plVar12[1] = 0;
              }
              thunk_FUN_140205820(plVar12,0x18);
            }
            local_100 = (longlong *)0x0;
            piVar10 = local_148;
          }
          if ((piVar10 == (int *)0x0) || ((short)*piVar10 == 0)) {
            if (piVar10 != (int *)0x0) {
              FUN_1401bebb0();
            }
          }
          else {
            local_118 = local_120;
            if (local_120 != (longlong *)0x0) {
              (**(code **)(*local_120 + 8))();
            }
            FUN_14090f200(&local_160,&local_118,piVar10);
            if (local_160 == (longlong *)0x0) {
              local_f8 = 0;
              uVar14 = FUN_1401c21c0(&local_f8,&DAT_14336b958,piVar10);
              plVar12 = local_188;
              if (local_180 == 0xaaaaaaaaaaaaaaa) {
                    /* WARNING: Subroutine does not return */
                FUN_142ed3068("list too long");
              }
              local_88 = &local_188;
              local_80 = (undefined8 *)0x0;
              puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
              puVar15[2] = 0;
              local_80 = puVar15;
              FUN_1401c1fb0(puVar15 + 2,uVar14);
              local_180 = local_180 + 1;
              puVar2 = (undefined8 *)plVar12[1];
              *puVar15 = plVar12;
              puVar15[1] = puVar2;
              local_80 = (undefined8 *)0x0;
              plVar12[1] = (longlong)puVar15;
              *puVar2 = puVar15;
              if (local_f8 != 0) {
                FUN_1401bebb0(local_f8 + -0x10);
              }
              if (local_160 != (longlong *)0x0) {
                (**(code **)(*local_160 + 0x10))();
              }
              FUN_1401bebb0();
            }
            else {
              puVar2 = *(undefined8 **)(DAT_143ac0188 + 800);
              cVar1 = *(char *)((longlong)puVar2[1] + 0x19);
              puVar15 = puVar2;
              puVar19 = (undefined8 *)puVar2[1];
              while (cVar1 == '\0') {
                if (*(int *)((longlong)puVar19 + 0x1c) < iVar23) {
                  puVar16 = (undefined8 *)puVar19[2];
                  puVar19 = puVar15;
                }
                else {
                  puVar16 = (undefined8 *)*puVar19;
                }
                puVar15 = puVar19;
                puVar19 = puVar16;
                cVar1 = *(char *)((longlong)puVar16 + 0x19);
              }
              if ((((*(char *)((longlong)puVar15 + 0x19) == '\0') &&
                   (*(int *)((longlong)puVar15 + 0x1c) <= iVar23)) && (puVar15 != puVar2)) &&
                 (iVar20 = *(int *)(puVar15 + 4), 0 < iVar20)) {
                do {
                  local_110 = local_160;
                  if (local_160 != (longlong *)0x0) {
                    (**(code **)(*local_160 + 8))();
                  }
                  FUN_14090ede0(&local_168,&local_110,piVar11);
                  if (local_168 == (longlong *)0x0) {
                    local_f0 = 0;
                    uVar14 = FUN_1401c21c0(&local_f0,L"/%s/%d",piVar10,piVar11);
                    plVar12 = local_188;
                    if (local_180 == 0xaaaaaaaaaaaaaaa) {
                    /* WARNING: Subroutine does not return */
                      FUN_142ed3068("list too long");
                    }
                    local_78 = &local_188;
                    local_70 = (undefined8 *)0x0;
                    puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
                    puVar15[2] = 0;
                    local_70 = puVar15;
                    FUN_1401c1fb0(puVar15 + 2,uVar14);
                    local_180 = local_180 + 1;
                    puVar2 = (undefined8 *)plVar12[1];
                    *puVar15 = plVar12;
                    puVar15[1] = puVar2;
                    local_70 = (undefined8 *)0x0;
                    plVar12[1] = (longlong)puVar15;
                    *puVar2 = puVar15;
                    if (local_f0 != 0) {
                      FUN_1401bebb0(local_f0 + -0x10);
                    }
                  }
                  else {
                    local_108 = local_168;
                    (**(code **)(*local_168 + 8))();
                    FUN_14090f580(&local_170,&local_108,&local_178);
                    pIVar5 = local_170;
                    if (local_170 == (IUnknown *)0x0) {
                      local_e8 = 0;
                      uVar14 = FUN_1401c21c0(&local_e8,L"/%s/%d",piVar10,piVar11);
                      plVar12 = local_188;
                      if (local_180 == 0xaaaaaaaaaaaaaaa) {
                    /* WARNING: Subroutine does not return */
                        FUN_142ed3068("list too long");
                      }
                      local_68 = &local_188;
                      local_60 = (undefined8 *)0x0;
                      puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
                      puVar15[2] = 0;
                      local_60 = puVar15;
                      FUN_1401c1fb0(puVar15 + 2,uVar14);
                      local_180 = local_180 + 1;
                      puVar2 = (undefined8 *)plVar12[1];
                      *puVar15 = plVar12;
                      puVar15[1] = puVar2;
                      local_60 = (undefined8 *)0x0;
                      plVar12[1] = (longlong)puVar15;
                      *puVar2 = puVar15;
                      if (local_e8 != 0) {
                        FUN_1401bebb0(local_e8 + -0x10);
                      }
                      if (local_170 != (IUnknown *)0x0) {
                        (**(code **)(*(longlong *)local_170 + 0x10))();
                      }
                    }
                    else {
                      local_res18[0] = 0;
                      iVar8 = (**(code **)(*(longlong *)local_170 + 0xa0))(local_170,local_res18);
                      if (iVar8 < 0) {
                        _com_issue_errorex(iVar8,pIVar5,(_GUID *)&DAT_14327ac98);
                      }
                      pIVar5 = local_170;
                      if (local_res18[0] < 2) {
                        if (local_170 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
                          FUN_142ef3ac0(0x80004003);
                        }
                        local_res20[0] = 0;
                        iVar8 = (**(code **)(*(longlong *)local_170 + 0x98))(local_170,local_res20);
                        if (iVar8 < 0) {
                          _com_issue_errorex(iVar8,pIVar5,(_GUID *)&DAT_14327ac98);
                        }
                        if (local_res20[0] < 2) {
                          local_e0 = 0;
                          uVar14 = FUN_1401c21c0(&local_e0,L"/%s/%d",piVar10,piVar11);
                          plVar12 = local_188;
                          if (local_180 == 0xaaaaaaaaaaaaaaa) {
                    /* WARNING: Subroutine does not return */
                            FUN_142ed3068("list too long");
                          }
                          local_58 = &local_188;
                          local_50 = (undefined8 *)0x0;
                          puVar15 = (undefined8 *)FUN_14019b780(&DAT_143ad68a0,0x18);
                          puVar15[2] = 0;
                          local_50 = puVar15;
                          FUN_1401c1fb0(puVar15 + 2,uVar14);
                          local_180 = local_180 + 1;
                          puVar2 = (undefined8 *)plVar12[1];
                          *puVar15 = plVar12;
                          puVar15[1] = puVar2;
                          local_50 = (undefined8 *)0x0;
                          plVar12[1] = (longlong)puVar15;
                          *puVar2 = puVar15;
                          if (local_e0 != 0) {
                            FUN_1401bebb0(local_e0 + -0x10);
                          }
                          if (local_170 != (IUnknown *)0x0) {
                            (**(code **)(*(longlong *)local_170 + 0x10))();
                          }
                          goto LAB_140ce6a46;
                        }
                      }
                      if (local_170 != (IUnknown *)0x0) {
                        (**(code **)(*(longlong *)local_170 + 0x10))();
                      }
                    }
                  }
LAB_140ce6a46:
                  if (local_168 != (longlong *)0x0) {
                    (**(code **)(*local_168 + 0x10))();
                  }
                  uVar21 = (int)piVar11 + 1;
                  piVar11 = (int *)(ulonglong)uVar21;
                } while ((int)uVar21 < iVar20);
              }
              if (local_160 != (longlong *)0x0) {
                (**(code **)(*local_160 + 0x10))();
              }
              FUN_1401bebb0();
            }
          }
          iVar23 = iVar23 + 1;
        } while (iVar23 < 0x21);
        if (local_180 != 0) {
          FUN_1403e18a0(&local_128,param_1);
          if ((local_128 != (short *)0x0) && (*local_128 != 0)) {
            FUN_1403e18a0(&local_130,local_res10 & 0xffffffff);
            plVar12 = local_188;
            if ((local_130 != (short *)0x0) && (*local_130 != 0)) {
              for (plVar3 = (longlong *)*local_188; plVar3 != plVar12; plVar3 = (longlong *)*plVar3)
              {
                lVar17 = plVar3[2];
                if (lVar17 == 0) {
                  uVar9 = 0;
                }
                else {
                  uVar9 = (undefined4)((ulonglong)(longlong)*(int *)(lVar17 + -8) >> 1);
                }
                FUN_14040ea40(&local_128,&local_res10,lVar17,uVar9);
                uVar4 = local_res10;
                if ((local_res10 == 0) || (lVar17 = FUN_142ef8a18(local_res10,0x2f), lVar17 == 0)) {
                  iVar23 = -1;
                }
                else {
                  iVar23 = (int)((longlong)(lVar17 - uVar4) >> 1);
                }
                FUN_1404ac7b0(&local_res10,&local_140,iVar23 + 1,0xffffffff);
                if ((uVar4 == 0) || (lVar17 = FUN_142ef8a18(uVar4,0x2f), lVar17 == 0)) {
                  uVar9 = 0xffffffff;
                }
                else {
                  uVar9 = (undefined4)((longlong)(lVar17 - uVar4) >> 1);
                }
                puVar18 = (ulonglong *)FUN_1404ac7b0(&local_res10,&local_98,0,uVar9);
                if (uVar4 != 0) {
                  FUN_1401bebb0(uVar4 - 0x10);
                }
                uVar4 = *puVar18;
                *puVar18 = 0;
                local_res10 = uVar4;
                if (local_98 != 0) {
                  FUN_1401bebb0(local_98 + -0x10);
                }
                FUN_14090ead0(&local_158,uVar4);
                if (local_158 == (longlong *)0x0) {
                  if (local_140 != 0) {
                    FUN_1401bebb0(local_140 + -0x10);
                  }
                }
                else {
                  lVar17 = plVar3[2];
                  if (lVar17 == 0) {
                    uVar9 = 0;
                  }
                  else {
                    uVar9 = (undefined4)((ulonglong)(longlong)*(int *)(lVar17 + -8) >> 1);
                  }
                  FUN_14040ea40(&local_130,&local_90,lVar17,uVar9);
                  lVar17 = local_90;
                  FUN_14090ead0(&local_138,local_90);
                  plVar6 = local_158;
                  if (local_138 == (longlong *)0x0) {
                    if (lVar17 != 0) {
                      FUN_1401bebb0(lVar17 + -0x10);
                    }
                    if (local_158 != (longlong *)0x0) {
                      (**(code **)(*local_158 + 0x10))();
                    }
                    if (local_140 != 0) {
                      FUN_1401bebb0(local_140 + -0x10);
                    }
                  }
                  else {
                    if (local_158 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
                      FUN_142ef3ac0(0x80004003);
                    }
                    (*DAT_143262a20)(local_b8);
                    iVar23 = FUN_14023c4c0(local_b8,&DAT_143a8b8d8);
                    if (iVar23 < 0) {
                    /* WARNING: Subroutine does not return */
                      FUN_142ef3ac0(iVar23);
                    }
                    local_d0[0] = 0xd;
                    local_c8 = local_138;
                    if (local_138 != (longlong *)0x0) {
                      (**(code **)(*local_138 + 8))();
                    }
                    lVar7 = local_140;
                    uVar14 = FUN_1401a5890(&local_150,local_140);
                    FUN_140ced4b0(plVar6,uVar14,local_d0,local_b8);
                    if (local_d0[0] == 8) {
                      local_d0[0] = 0;
                      if (local_c8 != (longlong *)0x0) {
                        (*DAT_143ad5990)((longlong)local_c8 + -4);
                      }
                    }
                    else {
                      (*DAT_143262a18)(local_d0);
                    }
                    if (local_b8[0] == 8) {
                      local_b8[0] = 0;
                      if (local_b0 != 0) {
                        (*DAT_143ad5990)(local_b0 + -4);
                      }
                    }
                    else {
                      (*DAT_143262a18)(local_b8);
                    }
                    if (local_138 != (longlong *)0x0) {
                      (**(code **)(*local_138 + 0x10))();
                    }
                    if (lVar17 != 0) {
                      FUN_1401bebb0(lVar17 + -0x10);
                    }
                    if (local_158 != (longlong *)0x0) {
                      (**(code **)(*local_158 + 0x10))();
                    }
                    if (lVar7 != 0) {
                      FUN_1401bebb0(lVar7 + -0x10);
                    }
                  }
                }
                if (uVar4 != 0) {
                  FUN_1401bebb0(uVar4 - 0x10);
                }
              }
            }
            if (local_130 != (short *)0x0) {
              FUN_1401bebb0(local_130 + -8);
            }
          }
          if (local_128 != (short *)0x0) {
            FUN_1401bebb0();
          }
        }
        *(undefined8 *)local_188[1] = 0;
        puVar2 = (undefined8 *)*local_188;
        while (puVar2 != (undefined8 *)0x0) {
          puVar15 = (undefined8 *)*puVar2;
          if (puVar2[2] != 0) {
            FUN_1401bebb0(puVar2[2] + -0x10);
          }
          thunk_FUN_140205820(puVar2,0x18);
          puVar2 = puVar15;
        }
        thunk_FUN_140205820(local_188,0x18);
      }
    }
    else {
      if (param_1 - 0xfde80U < 10000) {
        piVar11 = (int *)FUN_14019b600(&DAT_143ad6a30,0x15);
        piVar11[1] = 4;
        *piVar11 = -1;
        piVar10 = piVar11 + 4;
        piVar11[2] = 0;
        *(undefined1 *)piVar10 = 0;
        *piVar10 = DAT_14336b94c;
        local_150 = piVar10;
        if (*piVar11 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar11[1] < 4) {
          FUN_142e54290(0x90,piVar11[1],4);
        }
        *piVar11 = 1;
        *(undefined1 *)(piVar11 + 5) = 0;
        if (piVar11[1] + 1 < 5) {
          FUN_142e54290(0x9c,4);
        }
        piVar11[2] = 4;
        if (local_178 != (int *)0x0) {
          FUN_14019f2c0(local_178 + -4);
        }
        local_res10 = CONCAT44(local_res10._4_4_,0xfe7df);
        local_178 = piVar10;
        goto LAB_140ce642a;
      }
      if (param_1 - 0x102ca0U < 10000) {
        piVar11 = (int *)FUN_14019b600(&DAT_143ad6a30,0x16);
        piVar11[1] = 5;
        *piVar11 = -1;
        piVar10 = piVar11 + 4;
        piVar11[2] = 0;
        *(undefined1 *)piVar10 = 0;
        *piVar10 = s_pants_1432797d8._0_4_;
        *(char *)(piVar11 + 5) = s_pants_1432797d8[4];
        local_150 = piVar10;
        if (*piVar11 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar11[1] < 5) {
          FUN_142e54290(0x90,piVar11[1],5);
        }
        *piVar11 = 1;
        *(undefined1 *)((longlong)piVar11 + 0x15) = 0;
        if (piVar11[1] + 1 < 6) {
          FUN_142e54290(0x9c,5);
        }
        piVar11[2] = 5;
        if (local_178 != (int *)0x0) {
          FUN_14019f2c0(local_178 + -4);
        }
        local_res10 = CONCAT44(local_res10._4_4_,0x103577);
        local_178 = piVar10;
        goto LAB_140ce642a;
      }
    }
    if (local_178 != (int *)0x0) {
      FUN_14019f2c0(local_178 + -4);
    }
  }
  if (local_120 != (longlong *)0x0) {
    (**(code **)(*local_120 + 0x10))();
  }
LAB_140ce6e73:
  if (local_d8 != (short *)0x0) {
    FUN_1401bebb0(local_d8 + -8);
  }
  return;
}


