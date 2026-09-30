
//===========================================================
// FUN_1408d6760 @ 1408d6760   (197 bytes)
//===========================================================

void FUN_1408d6760(longlong param_1,undefined8 param_2)

{
  undefined8 *puVar1;
  longlong local_res8;
  
  puVar1 = (undefined8 *)FUN_1406e9050(param_2,&local_res8);
  if (*(longlong *)(param_1 + 8) != 0) {
    FUN_14019f2c0();
    *(undefined8 *)(param_1 + 8) = 0;
  }
  *(undefined8 *)(param_1 + 8) = *puVar1;
  *puVar1 = 0;
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  FUN_1408dcb80(param_2,param_1 + 0x10,param_1 + 0x18,param_1 + 0x1c,param_1 + 0x20,param_1 + 0x24,
                param_1,param_1 + 0x28,param_1 + 0x30,param_1 + 0x38,param_1 + 0x40);
  return;
}



//===========================================================
// FUN_1408dcb80 @ 1408dcb80   (406 bytes)
//===========================================================

void FUN_1408dcb80(undefined8 param_1,longlong *param_2,undefined8 param_3,undefined8 param_4,
                  undefined8 param_5,undefined8 param_6,undefined8 param_7,longlong *param_8,
                  undefined8 param_9,longlong *param_10,undefined8 param_11)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong local_res10;
  
  plVar1 = (longlong *)FUN_1406e9050(param_1,&local_res10);
  if (*param_2 != 0) {
    FUN_14019f2c0();
    *param_2 = 0;
  }
  *param_2 = *plVar1;
  *plVar1 = 0;
  if (local_res10 != 0) {
    FUN_14019f2c0(local_res10 + -0x10);
  }
  FUN_1406e9170(param_1,param_3,4);
  FUN_1406e9170(param_1,param_4,4);
  FUN_1406e9170(param_1,param_5,1);
  FUN_1406e9170(param_1,param_6,4);
  FUN_1406e9170(param_1,param_7,4);
  plVar2 = (longlong *)FUN_1406e9050(param_1,&local_res10);
  plVar1 = param_8;
  if (*param_8 != 0) {
    FUN_14019f2c0();
    *plVar1 = 0;
  }
  *plVar1 = *plVar2;
  *plVar2 = 0;
  if (local_res10 != 0) {
    FUN_14019f2c0(local_res10 + -0x10);
  }
  FUN_1406e9170(param_1,param_9,4);
  plVar2 = (longlong *)FUN_1406e9050(param_1,&param_8);
  plVar1 = param_10;
  if (*param_10 != 0) {
    FUN_14019f2c0();
    *plVar1 = 0;
  }
  *plVar1 = *plVar2;
  *plVar2 = 0;
  if (param_8 != (longlong *)0x0) {
    FUN_14019f2c0(param_8 + -2);
  }
  FUN_1406e9170(param_1,param_11,4);
  return;
}


