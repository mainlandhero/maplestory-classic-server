
//===========================================================
// FUN_1415d36c0 @ 1415d36c0   (709 bytes)
//===========================================================

void FUN_1415d36c0(longlong param_1)

{
  short sVar1;
  ushort uVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  int local_6c;
  undefined1 local_68;
  int local_60 [2];
  undefined8 local_58;
  longlong local_50;
  longlong local_48;
  longlong local_40;
  undefined1 local_38 [56];
  
  if (*(int *)(param_1 + 0x48) == 0) {
    local_6c = 2;
  }
  else {
    local_6c = 1;
  }
  if ((local_6c == 1) || (local_6c == 2)) {
    local_68 = 1;
  }
  else {
    local_68 = 0;
  }
  iVar3 = FUN_1415e3c40(param_1 + 0x70);
  if (iVar3 != 0) {
    uVar7 = FUN_1420a3640(&DAT_143ad53e0);
    *(undefined8 *)(param_1 + 0xa0) = uVar7;
    if (0 < *(longlong *)(param_1 + 0xa8)) {
      local_50 = *(longlong *)(param_1 + 0xa0) - *(longlong *)(param_1 + 0xa8);
      FUN_142e132a0(local_50,iVar3);
    }
  }
  do {
    iVar4 = FUN_1415e3ff0(param_1 + 0x70);
    if (iVar4 != 0) {
      if (iVar3 != 0) {
        FUN_142e12da0();
        uVar7 = FUN_1420a3640(&DAT_143ad53e0);
        *(undefined8 *)(param_1 + 0xa8) = uVar7;
      }
      return;
    }
    local_58 = FUN_1415e3c90(param_1 + 0x70);
    local_48 = param_1 + 0xb0;
    iVar4 = FUN_1406e9530(local_48,local_58,local_60,local_68);
    iVar5 = FUN_1415e2d00(local_58,0);
    if (iVar5 != 0) {
      FUN_1415e4670(param_1 + 0x70);
    }
    if ((0 < iVar4) && (local_60[0] < 1)) {
      local_40 = param_1 + 0xb0;
      sVar1 = FUN_1406e97e0(local_40,*(undefined4 *)(param_1 + 0xec));
      if (sVar1 != -2) {
        FUN_1415d33c0(param_1,0,0);
        return;
      }
      uVar2 = FUN_1406e9810(param_1 + 0xb0);
      if (0x20000 < uVar2) {
        FUN_1415d33c0(param_1,0,0);
        return;
      }
    }
    if (iVar4 == 2) {
      FUN_1406e88d0(local_38,param_1 + 0xb0);
      FUN_1406e99e0(local_38,*(undefined4 *)(param_1 + 0xec),local_6c);
      uVar6 = FUN_140c78630(param_1 + 0xec,4,0);
      *(undefined4 *)(param_1 + 0xec) = uVar6;
      FUN_1415d60e0(param_1,local_38);
      FUN_1406e89e0(local_38);
    }
  } while( true );
}


