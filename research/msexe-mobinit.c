
//===========================================================
// FUN_141cc9410 @ 141cc9410   (139 bytes)
//===========================================================

void FUN_141cc9410(longlong param_1,undefined8 param_2)

{
  longlong lVar1;
  
  FUN_141d22c10(param_1 + 0xa18);
  lVar1 = FUN_141d0ed80(0);
  if (*(longlong *)(lVar1 + 8) != 0) {
    FUN_142e541f0(0x2fe);
  }
  *(undefined8 *)(lVar1 + 8) = 1;
  *(longlong *)(param_1 + 0xa20) = lVar1 + 0x28;
  if (lVar1 + 0x28 == 0) {
    FUN_142e52ed0(0x431,0);
  }
  FUN_14085acd0(*(undefined8 *)(param_1 + 0xa20),param_2);
  return;
}



//===========================================================
// FUN_141c76190 @ 141c76190   (1025 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141c76190(longlong *param_1,undefined8 param_2,undefined4 param_3)

{
  ulonglong uVar1;
  longlong lVar2;
  longlong *plVar3;
  undefined4 uVar4;
  longlong lVar5;
  undefined8 uVar6;
  longlong lVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  uint uVar11;
  undefined1 auStack_d8 [32];
  undefined4 local_b8;
  longlong *local_a8;
  longlong *local_a0;
  undefined1 local_98 [8];
  longlong **local_90;
  longlong **local_88;
  uint local_78 [6];
  undefined8 local_60;
  undefined8 uStack_58;
  uint local_50;
  longlong local_48 [2];
  longlong local_38;
  ulonglong local_30;
  
  local_30 = DAT_143a8b908 ^ (ulonglong)auStack_d8;
  lVar7 = param_1[0x79];
  if (lVar7 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar7 = param_1[0x79];
  }
  local_78[0] = 0xffffffff;
  local_78[1] = 0xffffffff;
  local_78[2] = 0xffffffff;
  local_78[3] = 0xffffffff;
  local_78[4] = 0xffffffff;
  FUN_140473cd0(lVar7,local_48,local_78);
  lVar7 = param_1[0x79];
  if (lVar7 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar7 = param_1[0x79];
  }
  lVar2 = param_1[0x144];
  lVar5 = FUN_141892840();
  if (lVar5 == 0) {
    uVar4 = 100;
  }
  else {
    uVar6 = FUN_141892840();
    uVar4 = FUN_141866780(uVar6);
  }
  FUN_14046ad90(lVar7,param_1[0x75],uVar4,lVar2);
  local_60 = 0;
  uStack_58 = 0;
  local_50 = 0;
  FUN_1406e9170(param_2,&local_60,0x14);
  lVar7 = param_1[0x79];
  if (lVar7 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar7 = param_1[0x79];
  }
  local_b8 = FUN_1429e3ef0();
  FUN_14046fba0(lVar7,local_48,&local_60,param_2);
  if (local_48[0] != 0) {
    uVar1 = (local_38 - local_48[0] >> 2) * 4;
    if (0xfff < uVar1) {
      if (0x1f < (local_48[0] - *(longlong *)(local_48[0] + -8)) - 8U) {
                    /* WARNING: Subroutine does not return */
        FUN_142f04804(*(longlong *)(local_48[0] + -8),uVar1 + 0x27);
      }
    }
    thunk_FUN_140205820();
  }
  *(undefined4 *)(param_1 + 0xc4) = param_3;
  FUN_141c9c210(param_1,0);
  local_78[4] = local_50 & DAT_143a421f8;
  local_78[1] = local_60._4_4_ & DAT_143a421ec;
  local_78[0] = (uint)local_60 & DAT_143a421e8;
  local_78[3] = uStack_58._4_4_ & DAT_143a421f4;
  local_78[2] = (uint)uStack_58 & DAT_143a421f0;
  lVar7 = 0;
  do {
    if (local_78[lVar7] != 0) goto LAB_141c763ca;
    lVar7 = lVar7 + 1;
  } while (lVar7 < 4);
  if (local_78[4] == 0) {
    local_78[4] = local_50 & DAT_143a42120;
    local_78[1] = local_60._4_4_ & DAT_143a42114;
    local_78[0] = (uint)local_60 & DAT_143a42110;
    local_78[3] = uStack_58._4_4_ & DAT_143a4211c;
    local_78[2] = (uint)uStack_58 & DAT_143a42118;
    lVar7 = 0;
    do {
      if (local_78[lVar7] != 0) goto LAB_141c763ca;
      lVar7 = lVar7 + 1;
    } while (lVar7 < 4);
    if (local_78[4] == 0) goto LAB_141c763f0;
  }
LAB_141c763ca:
  FUN_141c5ae70(param_1);
LAB_141c763f0:
  local_78[4] = local_50 & DAT_143ac0dc8;
  local_78[1] = DAT_143ac0dbc & local_60._4_4_;
  local_78[0] = DAT_143ac0db8 & (uint)local_60;
  local_78[3] = uStack_58._4_4_ & DAT_143ac0dc4;
  local_78[2] = (uint)uStack_58 & DAT_143ac0dc0;
  lVar7 = 0;
  do {
    if (local_78[lVar7] != 0) goto LAB_141c76445;
    lVar7 = lVar7 + 1;
  } while (lVar7 < 4);
  uVar10 = uStack_58._4_4_;
  uVar9 = (uint)uStack_58;
  uVar8 = local_60._4_4_;
  uVar11 = (uint)local_60;
  if (local_78[4] != 0) {
LAB_141c76445:
    FUN_142902480(DAT_143aa8518,CONCAT71((uint7)(uint3)(((uint)uStack_58 & DAT_143ac0dc0) >> 8),1));
    uVar10 = uStack_58._4_4_;
    uVar9 = (uint)uStack_58;
    uVar8 = local_60._4_4_;
    uVar11 = (uint)local_60;
  }
  local_78[4] = local_50 & DAT_143ac0f00;
  local_78[1] = DAT_143ac0ef4 & uVar8;
  local_78[0] = DAT_143ac0ef0 & uVar11;
  local_78[3] = DAT_143ac0efc & uVar10;
  local_78[2] = DAT_143ac0ef8 & uVar9;
  lVar7 = 0;
  do {
    if (local_78[lVar7] != 0) goto LAB_141c764c9;
    lVar7 = lVar7 + 1;
  } while (lVar7 < 4);
  if (local_78[4] != 0) {
LAB_141c764c9:
    if ((param_1[0x79] != 0) && (*(int *)(param_1[0x79] + 0x2f8) != 0)) {
      plVar3 = (longlong *)param_1[0x57];
      if (plVar3 != (longlong *)0x0) {
        local_90 = &local_a8;
        local_a8 = plVar3;
        (**(code **)(*plVar3 + 8))();
        local_88 = &local_a0;
        local_a0 = (longlong *)param_1[0xc2];
        if (local_a0 != (longlong *)0x0) {
          (**(code **)(*local_a0 + 8))();
        }
        uVar6 = FUN_141ce16c0(param_1,local_98);
        FUN_141c77860(param_1,uVar6,&local_a0,&local_a8);
      }
      local_b8 = 0;
      FUN_141c54430(param_1,0,0,0);
    }
  }
  (**(code **)(*param_1 + 0xe0))(param_1,1,&local_60);
  return;
}


