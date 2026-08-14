
//===========================================================
// FUN_1406e8330 @ 1406e8330   (58 bytes)
//===========================================================

undefined8 FUN_1406e8330(undefined2 *param_1,undefined2 *param_2,uint param_3)

{
  undefined1 local_18 [24];
  
  if (1 < param_3) {
    *param_1 = *param_2;
    return 2;
  }
  FUN_1401bb8b0(local_18,0x26);
                    /* WARNING: Subroutine does not return */
  _CxxThrowException(local_18,(ThrowInfo *)&DAT_143a3b118);
}



//===========================================================
// FUN_1406e84d0 @ 1406e84d0   (124 bytes)
//===========================================================

uint FUN_1406e84d0(undefined8 param_1,ushort *param_2,uint param_3)

{
  uint uVar1;
  undefined1 local_18 [16];
  
  if (param_3 < 2) {
    FUN_1401bb8b0(local_18,0x26);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(local_18,(ThrowInfo *)&DAT_143a3b118);
  }
  uVar1 = *param_2 + 2;
  if (uVar1 <= param_3) {
    FUN_1401d66b0(param_1,param_2 + 1,*param_2);
    return uVar1;
  }
  FUN_1401bb8b0(local_18,0x26);
                    /* WARNING: Subroutine does not return */
  _CxxThrowException(local_18,(ThrowInfo *)&DAT_143a3b118);
}


