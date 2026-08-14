
//===========================================================
// FUN_1415d33c0 @ 1415d33c0   (354 bytes)
//===========================================================

void FUN_1415d33c0(longlong param_1,undefined4 param_2,undefined4 param_3)

{
  char cVar1;
  byte bVar2;
  int iVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined4 local_res10 [2];
  undefined4 local_res18 [4];
  undefined8 in_stack_ffffffffffffffd8;
  undefined4 uVar6;
  
  uVar6 = (undefined4)((ulonglong)in_stack_ffffffffffffffd8 >> 0x20);
  local_res10[0] = param_2;
  local_res18[0] = param_3;
  if (*(int *)(param_1 + 0x48) == 0) {
    cVar1 = FUN_1415e4060();
    if (cVar1 != '\0') {
      uVar4 = FUN_1415e3d00();
      FUN_142c4f7e0(uVar4,4,local_res10[0],local_res18[0]);
    }
  }
  FUN_1415d35f0(param_1);
  iVar3 = FUN_1415e3c50(param_1 + 0x28);
  if (iVar3 == 0) {
    if (*(int *)(param_1 + 0x48) == 0) {
      lVar5 = FUN_140caa510();
      if (*(char *)(lVar5 + 0x20) == '\0') {
        uVar4 = FUN_140caa510();
        bVar2 = FUN_142cb95d0(uVar4);
        if ((bVar2 & 4) == 0) {
          FUN_1415e0db0(&DAT_143271f04,0x3e6,0x21000002,local_res10,local_res18);
        }
        else {
          FUN_1415d9210(param_1,&DAT_143271f04,0x3e3,local_res10[0],CONCAT44(uVar6,local_res18[0]));
        }
      }
      else {
        FUN_1415e0f30(&DAT_143271f04,0x3df,0x2200000f,local_res10,local_res18);
      }
    }
    else {
      FUN_1415e0f30(&DAT_143271f04,0x3dc,0x22000002,local_res10,local_res18);
    }
  }
  else {
    FUN_1415d10e0(param_1,0,0);
  }
  return;
}


