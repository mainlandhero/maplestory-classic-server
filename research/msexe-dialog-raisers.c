
//===========================================================
// FUN_1415e0fb0 @ 1415e0fb0   (166 bytes)
//===========================================================

void FUN_1415e0fb0(undefined8 param_1,undefined4 param_2,undefined4 param_3,undefined8 param_4,
                  undefined8 param_5,undefined8 param_6)

{
  longlong lVar1;
  longlong local_20;
  undefined8 local_18 [2];
  
  local_18[0] = FUN_1418039d0(param_3);
  local_20 = 0;
  FUN_1415e1540(&local_20,local_18,param_4,param_5,param_6);
  lVar1 = local_20;
  FUN_141804870(param_1,param_2,param_3,local_20);
  if (lVar1 != 0) {
    FUN_14019f2c0(lVar1 + -0x10);
  }
  return;
}



//===========================================================
// FUN_1415e0e30 @ 1415e0e30   (126 bytes)
//===========================================================

void FUN_1415e0e30(undefined8 param_1,undefined4 param_2,undefined4 param_3,undefined8 param_4,
                  undefined8 param_5)

{
  undefined8 *puVar1;
  undefined8 local_18;
  longlong local_10;
  
  local_18 = FUN_1418039d0(param_3);
  puVar1 = (undefined8 *)FUN_1415e0600(&local_10,&local_18,param_4,param_5);
  FUN_141804870(param_1,param_2,param_3,*puVar1);
  if (local_10 != 0) {
    FUN_14019f2c0(local_10 + -0x10);
  }
  return;
}



//===========================================================
// FUN_140cc2350 @ 140cc2350   (105 bytes)
//===========================================================

void FUN_140cc2350(undefined8 param_1,undefined4 param_2,undefined4 param_3)

{
  undefined8 *puVar1;
  undefined8 local_res20;
  longlong local_18 [2];
  
  local_res20 = FUN_1418039d0(param_3);
  puVar1 = (undefined8 *)FUN_140cc21e0(local_18,&local_res20);
  FUN_141804870(param_1,param_2,param_3,*puVar1);
  if (local_18[0] != 0) {
    FUN_14019f2c0(local_18[0] + -0x10);
  }
  return;
}



//===========================================================
// FUN_1415e3ba0 @ 1415e3ba0   (92 bytes)
//===========================================================

ulonglong FUN_1415e3ba0(longlong *param_1,longlong param_2)

{
  char cVar1;
  char cVar2;
  longlong lVar3;
  ulonglong in_RAX;
  char *pcVar4;
  ulonglong uVar5;
  
  lVar3 = *param_1;
  if ((lVar3 != 0) && (param_2 != 0)) {
    uVar5 = 0xffffffffffffffff;
    do {
      uVar5 = uVar5 + 1;
    } while (*(char *)(param_2 + uVar5) != '\0');
    if (uVar5 <= (ulonglong)(longlong)*(int *)(lVar3 + -8)) {
      pcVar4 = (char *)(((longlong)*(int *)(lVar3 + -8) - uVar5) + lVar3);
      param_2 = param_2 - (longlong)pcVar4;
      do {
        cVar1 = *pcVar4;
        cVar2 = pcVar4[param_2];
        if (cVar1 != cVar2) break;
        pcVar4 = pcVar4 + 1;
      } while (cVar2 != '\0');
      return CONCAT71((int7)((ulonglong)pcVar4 >> 8),cVar1 == cVar2);
    }
  }
  return in_RAX & 0xffffffffffffff00;
}


