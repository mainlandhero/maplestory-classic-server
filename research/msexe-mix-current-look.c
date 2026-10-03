
//===========================================================
// FUN_1401a6ee0 @ 1401a6ee0   (116 bytes)
//===========================================================

undefined8 FUN_1401a6ee0(longlong param_1,char param_2,undefined4 param_3,uint *param_4)

{
  char cVar1;
  uint uVar2;
  undefined4 local_res18 [4];
  
  local_res18[0] = param_3;
  cVar1 = FUN_14041acf0(local_res18);
  if (param_2 != '\0') {
    return 0;
  }
  if (cVar1 == '\x01') {
    uVar2 = (uint)*(byte *)(param_1 + 0x1a);
  }
  else if (cVar1 == '\x02') {
    uVar2 = *(uint *)(param_1 + 0x1f);
  }
  else {
    if (cVar1 != '\x04') {
      return 0;
    }
    uVar2 = *(uint *)(param_1 + 0x23);
  }
  *param_4 = uVar2;
  return 1;
}



//===========================================================
// FUN_1401a7080 @ 1401a7080   (123 bytes)
//===========================================================

undefined8 FUN_1401a7080(undefined4 *param_1,undefined4 param_2,undefined4 *param_3)

{
  char cVar1;
  undefined4 local_res10 [6];
  
  local_res10[0] = param_2;
  cVar1 = FUN_14041acf0(local_res10);
  if (cVar1 == '\x01') {
    *param_3 = *param_1;
    return 1;
  }
  if (cVar1 != '\x02') {
    if (cVar1 != '\x04') {
      return 0;
    }
    *param_3 = param_1[2];
    return 1;
  }
  *param_3 = param_1[3];
  return 1;
}


