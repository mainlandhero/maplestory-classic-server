
//===========================================================
// FUN_141b0ef00 @ 141b0ef00   (2397 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x000141b0f15b) */

undefined4 FUN_141b0ef00(longlong param_1,undefined8 param_2,undefined8 param_3)

{
  undefined4 uVar1;
  int iVar2;
  undefined8 uVar3;
  undefined1 *puVar4;
  longlong lVar5;
  int *piVar6;
  undefined1 *puVar7;
  undefined1 auStack_1b8 [32];
  undefined1 uStack_198;
  undefined1 uStack_197;
  undefined1 auStack_196 [2];
  undefined4 uStack_194;
  uint uStack_190;
  int iStack_18c;
  longlong lStack_188;
  int iStack_180;
  int iStack_17c;
  int iStack_178;
  int iStack_174;
  undefined4 uStack_170;
  longlong *plStack_168;
  longlong *plStack_160;
  longlong *plStack_158;
  longlong *plStack_150;
  undefined4 uStack_148;
  longlong lStack_140;
  longlong lStack_138;
  undefined8 uStack_130;
  longlong lStack_128;
  longlong lStack_120;
  longlong lStack_118;
  longlong lStack_110;
  longlong lStack_108;
  longlong lStack_100;
  code *pcStack_f8;
  longlong lStack_f0;
  int *piStack_e8;
  longlong lStack_e0;
  int *piStack_d8;
  longlong lStack_d0;
  int *piStack_c8;
  longlong lStack_c0;
  int *piStack_b8;
  undefined1 auStack_b0 [16];
  undefined1 auStack_a0 [16];
  undefined1 auStack_90 [16];
  undefined1 auStack_80 [16];
  undefined8 uStack_70;
  undefined1 auStack_68 [16];
  undefined1 auStack_58 [16];
  undefined1 auStack_48 [16];
  int iStack_38;
  int iStack_34;
  int iStack_30;
  int iStack_2c;
  ulonglong uStack_28;
  
  uStack_28 = DAT_143a8b908 ^ (ulonglong)auStack_1b8;
  FUN_142e10d00(auStack_196);
  uVar3 = (*DAT_1432622d8)(0,0,0,0);
  *(undefined8 *)(param_1 + 0x28) = uVar3;
  if (*(longlong *)(param_1 + 0x28) == 0) {
    uVar1 = (*DAT_143262838)();
    FUN_141b1c680(param_3,uVar1);
  }
  FUN_141b1ca00(param_3);
  FUN_141b10900();
  piVar6 = &iStack_38;
  for (lVar5 = 0x10; lVar5 != 0; lVar5 = lVar5 + -1) {
    *(undefined1 *)piVar6 = 0;
    piVar6 = (int *)((longlong)piVar6 + 1);
  }
  uStack_194 = 100;
  do {
    while( true ) {
      if ((1 < *(int *)(param_1 + 0x30)) ||
         (uStack_148 = (*DAT_1432622d0)(*(undefined8 *)(param_1 + 0x28),uStack_194),
         *(int *)(param_1 + 0x20) != 0)) goto LAB_141b0f74d;
      if ((iStack_38 != 0) && (((iStack_34 != 0 && (iStack_30 != 0)) && (iStack_2c != 0)))) {
        plStack_158 = (longlong *)(param_1 + 0x40);
        uStack_197 = *plStack_158 == *(longlong *)(param_1 + 0x48);
        uStack_190 = (uint)(byte)uStack_197;
        if (!(bool)uStack_197) {
          plStack_150 = (longlong *)(param_1 + 0x40);
          lStack_140 = *(longlong *)(param_1 + 0x48);
          for (lStack_188 = *plStack_150; lStack_188 != lStack_140; lStack_188 = lStack_188 + 0x10)
          {
            lStack_138 = lStack_188;
            FUN_141b1e1f0(lStack_188);
          }
          FUN_141b1e110(param_1 + 0x40);
        }
        *(undefined4 *)(param_1 + 0x30) = 2;
        iVar2 = FUN_14090d160(0x8e,1);
        if (iVar2 == 1) {
          FUN_142c4f490(DAT_143ac1898);
          uStack_130 = DAT_143ac87a0;
          uVar3 = FUN_1415f1c50(DAT_143ac87a0);
          FUN_1413f4690(uVar3);
        }
        puVar4 = (undefined1 *)FUN_1408f66e0(auStack_58);
        puVar7 = auStack_48;
        for (lVar5 = 0x10; lVar5 != 0; lVar5 = lVar5 + -1) {
          *puVar7 = *puVar4;
          puVar4 = puVar4 + 1;
          puVar7 = puVar7 + 1;
        }
        puVar4 = auStack_48;
        puVar7 = auStack_68;
        for (lVar5 = 0x10; lVar5 != 0; lVar5 = lVar5 + -1) {
          *puVar7 = *puVar4;
          puVar4 = puVar4 + 1;
          puVar7 = puVar7 + 1;
        }
        FUN_141b0eb80(auStack_68);
      }
      if (*(int *)(param_1 + 0x1c) != 1) break;
      if ((iStack_38 == 0) && ((*(uint *)(param_1 + 0x34) & 1) != 0)) {
        lStack_128 = DAT_143ac1898;
        iVar2 = FUN_142c4b720(DAT_143ac1898);
        if (iVar2 != 0) goto LAB_141b0f74d;
        iStack_180 = (*DAT_143262db0)();
        *(undefined4 *)(param_1 + 0x38) = 1;
        piStack_e8 = &iStack_38;
        lStack_f0 = param_1;
        FUN_141b193d0(auStack_80,1,&lStack_f0);
        piStack_d8 = &iStack_38;
        lStack_e0 = param_1;
        FUN_141b19520(auStack_90,1,&lStack_e0);
        FUN_141b0f930(param_1);
        piStack_c8 = &iStack_38;
        lStack_d0 = param_1;
        FUN_141b19010(auStack_a0,1,&lStack_d0);
        piStack_b8 = &iStack_38;
        lStack_c0 = param_1;
        FUN_141b19280(auStack_b0,1,&lStack_c0);
        FUN_141b1e1f0(auStack_a0);
        FUN_141b1e1f0(auStack_b0);
        FUN_141b1e1f0(auStack_90);
        FUN_141b1e1f0(auStack_80);
        *(undefined4 *)(param_1 + 0x38) = 0;
        iVar2 = (*DAT_143262db0)();
        FUN_141b0ebb0(0,iVar2 - iStack_180);
        uStack_194 = 200;
        FUN_1403f5a90(auStack_b0);
        FUN_1403f5a90(auStack_a0);
        FUN_1403f5a90(auStack_90);
        FUN_1403f5a90(auStack_80);
      }
    }
    if (((*(int *)(param_1 + 0x20) == 0) && (iStack_38 == 0)) &&
       ((*(uint *)(param_1 + 0x34) & 1) != 0)) {
      lStack_120 = DAT_143ac1898;
      iVar2 = FUN_142c4b720(DAT_143ac1898);
      if (iVar2 != 0) {
LAB_141b0f74d:
        if ((*(int *)(param_1 + 0x20) != 0) && (uStack_198 = DAT_143ac1898 != 0, (bool)uStack_198))
        {
          lStack_100 = DAT_143ac1898;
          FUN_142c50300(DAT_143ac1898,1);
        }
        plStack_160 = (longlong *)FUN_141b1e230();
        plStack_168 = (longlong *)uStack_70;
        if (*plStack_160 != 0) {
          plStack_168 = (longlong *)*plStack_160;
          pcStack_f8 = *(code **)(*plStack_168 + 0x60);
          (*pcStack_f8)(plStack_168,0);
          FUN_140934030();
          FUN_1406f5ba0(&DAT_143adddf8);
          FUN_142e159c0();
          iVar2 = (*DAT_143262db0)();
          DAT_143addd60 = iVar2 - DAT_143addd60;
          FUN_142c50b90(2);
          FUN_141b10b20();
          uStack_170 = 0;
          FUN_142e10ca0(auStack_196);
          return uStack_170;
        }
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      iStack_17c = (*DAT_143262db0)();
      *(undefined4 *)(param_1 + 0x38) = 1;
      FUN_141b0f930(param_1);
      FUN_141b0f940(param_1);
      *(undefined4 *)(param_1 + 0x38) = 0;
      iStack_38 = 1;
      iVar2 = (*DAT_143262db0)();
      FUN_141b0ebb0(0,iVar2 - iStack_17c);
    }
    if (((*(int *)(param_1 + 0x20) == 0) && (iStack_2c == 0)) &&
       ((*(uint *)(param_1 + 0x34) & 8) != 0)) {
      lStack_118 = DAT_143ac1898;
      iVar2 = FUN_142c4b720(DAT_143ac1898);
      if (iVar2 != 0) goto LAB_141b0f74d;
      iStack_178 = (*DAT_143262db0)();
      *(undefined4 *)(param_1 + 0x38) = 1;
      FUN_141b10490(param_1);
      *(undefined4 *)(param_1 + 0x38) = 0;
      iVar2 = (*DAT_143262db0)();
      FUN_141b0ebb0(3,iVar2 - iStack_178);
      iStack_2c = 1;
      if (iStack_38 != 0) {
        *(undefined4 *)(param_1 + 0x30) = 1;
      }
    }
    if ((*(int *)(param_1 + 0x20) == 0) && (*(int *)(param_1 + 0x30) == 1)) {
      if ((*(int *)(param_1 + 0x20) == 0) &&
         ((iStack_34 == 0 && ((*(uint *)(param_1 + 0x34) & 2) != 0)))) {
        lStack_110 = DAT_143ac1898;
        iVar2 = FUN_142c4b720(DAT_143ac1898);
        if (iVar2 != 0) goto LAB_141b0f74d;
        iStack_18c = (*DAT_143262db0)();
        *(undefined4 *)(param_1 + 0x38) = 1;
        FUN_141b0fc40(param_1);
        *(undefined4 *)(param_1 + 0x38) = 0;
        iVar2 = (*DAT_143262db0)();
        FUN_141b0ebb0(1,iVar2 - iStack_18c);
        iStack_18c = (*DAT_143262db0)();
        iStack_34 = 1;
      }
      if (((*(int *)(param_1 + 0x20) == 0) && (iStack_30 == 0)) &&
         ((*(uint *)(param_1 + 0x34) & 4) != 0)) {
        lStack_108 = DAT_143ac1898;
        iVar2 = FUN_142c4b720(DAT_143ac1898);
        if (iVar2 != 0) goto LAB_141b0f74d;
        iStack_174 = (*DAT_143262db0)();
        *(undefined4 *)(param_1 + 0x38) = 1;
        FUN_141b10350(param_1,0);
        *(undefined4 *)(param_1 + 0x38) = 0;
        iVar2 = (*DAT_143262db0)();
        FUN_141b0ebb0(2,iVar2 - iStack_174);
        iStack_30 = 1;
      }
    }
    uStack_194 = 200;
  } while( true );
}


