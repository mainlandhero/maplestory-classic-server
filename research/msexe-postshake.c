
//===========================================================
// FUN_1415d5aa0 @ 1415d5aa0   (141 bytes)
//===========================================================

void FUN_1415d5aa0(longlong param_1)

{
  char cVar1;
  
  FUN_1406e9510(param_1 + 0xb0);
  FUN_1415e41b0(param_1 + 0x70);
  FUN_1415e41b0(param_1 + 0x88);
  while( true ) {
    cVar1 = FUN_1415e56e0(param_1 + 0x108);
    if (cVar1 != '\0') break;
    FUN_1415e5a40(param_1 + 0x108);
  }
  *(undefined4 *)(param_1 + 0x130) = 0;
  FUN_1415d9cd0(param_1);
  return;
}



//===========================================================
// FUN_1415d5c20 @ 1415d5c20   (289 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1415d5c20(undefined8 param_1)

{
  undefined1 uVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined1 auStack_498 [32];
  undefined1 local_478;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  FUN_1406ed520(local_468,0x71);
  FUN_1406ed840(local_468,1);
  FUN_1406ed9d0(local_468,1);
  FUN_1406ed9d0(local_468,100);
  FUN_1406ed840(local_468,0);
  uVar2 = FUN_140caa510();
  uVar1 = FUN_142cb8610(uVar2);
  FUN_1406ed840(local_468,uVar1);
  lVar3 = FUN_140caa510();
  FUN_1406ed840(local_468,*(undefined1 *)(lVar3 + 0x2520));
  lVar3 = FUN_140caa510();
  FUN_1406ed840(local_468,*(undefined1 *)(lVar3 + 0x2524));
  lVar3 = FUN_140caa510();
  FUN_1406ed840(local_468,*(undefined1 *)(lVar3 + 0x2528));
  local_478 = FUN_1415dcc90();
  FUN_1406ed840(local_468,local_478);
  FUN_1415dc6f0(local_468);
  FUN_1415d3990(param_1,local_468);
  FUN_140c92fd0(2);
  FUN_1406ed610(local_468);
  return;
}



//===========================================================
// FUN_1415d3990 @ 1415d3990   (831 bytes)
//===========================================================

/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1415d3990(longlong param_1,undefined8 param_2)

{
  char cVar1;
  uint uVar2;
  undefined1 auStack_788 [32];
  undefined4 *local_768;
  undefined4 *local_760;
  undefined4 *local_758;
  int local_6f4 [15];
  undefined4 local_6b8;
  undefined4 local_6b4;
  undefined4 local_6b0;
  undefined4 local_6ac;
  undefined4 local_6a8;
  undefined4 local_6a4;
  undefined4 local_6a0;
  undefined4 local_69c;
  undefined4 local_698 [43];
  undefined4 local_5ec;
  undefined4 local_5e8;
  undefined4 local_5e4;
  undefined4 local_5e0 [20];
  undefined8 local_590;
  undefined8 local_588;
  undefined8 local_580;
  undefined1 local_578 [1360];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_788;
  if (param_1 == 0) {
    local_5ec = FUN_1406ed670(param_2);
    local_5e8 = FUN_1406ed830(param_2);
    local_5e4 = 2;
    local_5e0[0] = 0x446;
    local_760 = &local_5ec;
    local_768 = &local_5e8;
    FUN_1415df4f0(&DAT_1432731ec,&DAT_143271f04,local_5e0,&local_5e4);
    return;
  }
  local_6f4[0] = FUN_14029e2f0(*(undefined4 *)(param_1 + 0xc));
  uVar2 = FUN_1406ed660(param_2);
  if (local_6f4[0] - 100U < uVar2) {
    local_590 = FUN_1406eea10(param_2,local_578,0x100);
    local_588 = local_590;
    local_580 = local_590;
    local_6b8 = FUN_1406ed660(param_2);
    local_6b4 = FUN_1406ed670(param_2);
    local_6b0 = FUN_1406ed830(param_2);
    local_758 = (undefined4 *)local_580;
    local_760 = &local_6b8;
    local_768 = &local_6b4;
    FUN_1415de6c0(1,param_1 + 0xc,local_6f4,&local_6b0);
    FUN_140199470(local_578);
  }
  FUN_1406ed660(param_2);
  if (*(char *)(param_1 + 8) == '\0') {
    local_6ac = FUN_1406ed670(param_2);
    local_6a8 = FUN_1406ed830(param_2);
    local_6a4 = 3;
    local_6a0 = 0x454;
    local_768 = (undefined4 *)(param_1 + 0xc);
    local_758 = &local_6ac;
    local_760 = &local_6a8;
    FUN_1415dfb60(&DAT_1432731ec,&DAT_143271f04,&local_6a0,&local_6a4);
    cVar1 = FUN_14090d340(0x219);
    if (cVar1 != '\0') {
      local_69c = 4;
      local_698[0] = 0x456;
      FUN_1415de620(0x13,&DAT_143271f04,local_698,&local_69c);
      cVar1 = FUN_14090d240(0x218);
      if (cVar1 != '\0') {
        while (*(ulonglong *)((longlong)Self + 0x10) < *(ulonglong *)((longlong)Self + 8)) {
          *(longlong *)((longlong)Self + 8) = *(longlong *)((longlong)Self + 8) + -4;
          **(undefined4 **)((longlong)Self + 8) = 0;
        }
      }
    }
  }
                    /* WARNING: Bad instruction - Truncating control flow here */
  halt_baddata();
}



//===========================================================
// FUN_1415d5b40 @ 1415d5b40   (214 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1415d5b40(longlong param_1)

{
  char cVar1;
  undefined1 auStack_498 [32];
  undefined8 local_478;
  undefined8 local_470;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  *(undefined1 *)(param_1 + 8) = 1;
  FUN_1406ed520(local_468,0x70);
  FUN_1406ed840(local_468,2);
  FUN_1406ed9d0(local_468,100);
  cVar1 = FUN_140739170(5);
  if (cVar1 == '\0') {
    local_470 = FUN_140caa4f0();
    FUN_140c7b230(local_470,local_468);
  }
  else {
    local_478 = FUN_140caa4f0();
    FUN_140c7b560(local_478,local_468);
  }
  FUN_1415d3990(param_1,local_468);
  FUN_140c92fd0(0);
  FUN_1406ed610(local_468);
  return;
}



//===========================================================
// FUN_1415e3de0 @ 1415e3de0   (68 bytes)
//===========================================================

void FUN_1415e3de0(undefined8 *param_1,undefined8 param_2,undefined4 param_3)

{
  int iVar1;
  undefined4 uVar2;
  undefined4 local_res18 [4];
  undefined1 local_18 [24];
  
  local_res18[0] = param_3;
  iVar1 = (*DAT_143262e50)(*param_1,param_2,local_res18);
  if (iVar1 != -1) {
    return;
  }
  uVar2 = (*DAT_143262e10)();
  FUN_1401bb8b0(local_18,uVar2);
                    /* WARNING: Subroutine does not return */
  _CxxThrowException(local_18,(ThrowInfo *)&DAT_143a3b118);
}


