
//===========================================================
// FUN_141183150 @ 141183150   (1 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141183150(void)

{
  undefined1 auStack_498 [32];
  undefined4 auStack_478 [4];
  undefined1 auStack_468 [1104];
  ulonglong uStack_18;
  
  uStack_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  if (-1 < DAT_143a872a0) {
    FUN_1406ed520(auStack_468,0x1fd);
    auStack_478[0] = 1;
    FUN_1406ede20(auStack_468,auStack_478,4);
    FUN_1406ed9d0(auStack_468,DAT_143a872a0);
    FUN_1415d01c0(auStack_468);
    if (DAT_143aa84a0 != 0) {
      FUN_142cc4430(DAT_143aa84a0,1);
    }
    FUN_1406ed610(auStack_468);
  }
  return;
}



//===========================================================
// FUN_141183200 @ 141183200   (1 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

undefined8 FUN_141183200(longlong *param_1)

{
  int iVar1;
  longlong lVar2;
  undefined1 auStack_498 [32];
  undefined4 auStack_478 [2];
  longlong *plStack_470;
  undefined1 auStack_468 [1104];
  ulonglong uStack_18;
  
  uStack_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  plStack_470 = param_1;
  if ((DAT_143a872a0 < 0) && (DAT_143aca618 == 0)) {
    lVar2 = *param_1;
  }
  else {
    iVar1 = FUN_1415c0460(&DAT_143aca848,param_1,1);
    if ((iVar1 != 0) &&
       ((DAT_143aa84a0 != 0 && (iVar1 = FUN_142cc42d0(DAT_143aa84a0,500,0), iVar1 != 0)))) {
      FUN_1406ed520(auStack_468,0x1fd);
      auStack_478[0] = 3;
      FUN_1406ede20(auStack_468,auStack_478,4);
      FUN_1406edc80(auStack_468,param_1);
      FUN_1415d01c0(auStack_468);
      FUN_1406ed610(auStack_468);
      if (*param_1 != 0) {
        FUN_14019f2c0(*param_1 + -0x10);
      }
      return 1;
    }
    lVar2 = *param_1;
  }
  if (lVar2 != 0) {
    FUN_14019f2c0(lVar2 + -0x10);
  }
  return 0;
}



//===========================================================
// FUN_141e95720 @ 141e95720   (1224 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141e95720(longlong param_1,undefined4 param_2)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  undefined1 auStack_4d8 [32];
  undefined4 local_4b8;
  undefined4 local_4b0;
  undefined4 local_4a8;
  undefined4 local_4a0;
  undefined4 local_498;
  undefined4 local_490;
  undefined4 local_488;
  undefined4 local_478;
  undefined4 uStack_474;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4d8;
  switch(param_2) {
  case 0x3e9:
    iVar1 = FUN_141c3f8a0(param_1);
    if (0 < iVar1) {
      FUN_1406ed520(local_468,0x17e);
      local_478 = 0x1c;
      FUN_1406ede20(local_468,&local_478,4);
      FUN_1415d01c0(local_468);
      FUN_1406ed610(local_468);
    }
    break;
  case 0x3ea:
    if ((*(int *)(param_1 + 0x1760) == 0) && (*(int *)(param_1 + 0x175c) == 0)) {
      uVar2 = FUN_1408a9e40(&local_478,0x1f9);
      local_488 = 0;
      local_490 = 0;
      local_498 = 3;
      local_4a0 = 0;
      local_4a8 = 0;
      local_4b0 = 0xffffffff;
      local_4b8 = 0;
      iVar1 = FUN_142a269c0(uVar2,0,param_1 + 0x240,1);
      if (iVar1 == 6) {
        FUN_1406ed520(local_468,0x17e);
        local_478 = 0x11;
        FUN_1406ede20(local_468,&local_478,4);
        FUN_1415d01c0(local_468);
        *(undefined4 *)(param_1 + 0x175c) = 1;
        *(undefined4 *)(param_1 + 0x1760) = 1;
        FUN_1406ed610(local_468);
      }
    }
    break;
  case 0x3eb:
    if (*(int *)(param_1 + 0x1758) == 0) {
      uVar2 = FUN_1408a9e40(&local_478,0x1f6);
      local_488 = 0;
      local_490 = 0;
      local_498 = 3;
      local_4a0 = 0;
      local_4a8 = 0;
      local_4b0 = 0xffffffff;
      local_4b8 = 0;
      iVar1 = FUN_142a269c0(uVar2,0,param_1 + 0x240,1);
      if ((iVar1 == 6) && (*(int *)(param_1 + 0x1b00) == 1)) {
        FUN_1406ed520(local_468,0x17e);
        local_478 = 0x13;
        FUN_1406ede20(local_468,&local_478,4);
        FUN_1415d01c0(local_468);
        *(undefined4 *)(param_1 + 0x1758) = 1;
        FUN_1406ed610(local_468);
      }
    }
    break;
  case 0x3ec:
    FUN_141e9c770(param_1);
    break;
  default:
    FUN_14177fb00(param_1);
    break;
  case 0x3ee:
    FUN_141e9c320(param_1);
    break;
  case 0x3ef:
    FUN_141c417f0(param_1);
    break;
  case 0x3f0:
    if (*(int *)(param_1 + 0x1b08) == 0) {
      FUN_1406ed520(local_468,0x17e);
      local_478 = 0x19;
      FUN_1406ede20(local_468,&local_478,4);
      FUN_1415d01c0(local_468);
    }
    else {
      FUN_1406ed520(local_468,0x17e);
      local_478 = 0x1a;
      FUN_1406ede20(local_468,&local_478,4);
      FUN_1415d01c0(local_468);
    }
    FUN_1406ed610(local_468);
    puVar3 = (undefined8 *)FUN_1408a9d20(&local_478,0x5d9);
    FUN_1429f1f50(*puVar3,100,0);
    if (CONCAT44(uStack_474,local_478) != 0) {
      FUN_1401bebb0(CONCAT44(uStack_474,local_478) + -0x10);
    }
    break;
  case 0x3f1:
    uVar2 = FUN_1408a9e40(&local_478,0x1f7);
    local_488 = 0;
    local_490 = 0;
    local_498 = 3;
    local_4a0 = 0;
    local_4a8 = 0;
    local_4b0 = 0xffffffff;
    local_4b8 = 0;
    iVar1 = FUN_142a269c0(uVar2,0,param_1 + 0x240,1);
    if (iVar1 == 6) {
      FUN_1406ed520(local_468,0x17e);
      local_478 = 0x1b;
      FUN_1406ede20(local_468,&local_478,4);
      FUN_1415d01c0(local_468);
      FUN_1406ed610(local_468);
    }
    break;
  case 0x3f2:
    if (0 < *(int *)(param_1 + 0x1b38)) {
      if (((*(int *)(param_1 + 0x1764) == 0) && (*(int *)(param_1 + 0x1768) == 0)) &&
         (*(int *)(param_1 + 0x176c) == 0)) {
        uVar2 = FUN_1408a9e40(&local_478,0x1fd);
        local_488 = 0;
        local_490 = 0;
        local_498 = 3;
        local_4a0 = 0;
        local_4a8 = 0;
        local_4b0 = 0xffffffff;
        local_4b8 = 0;
        iVar1 = FUN_142a269c0(uVar2,0,param_1 + 0x240,1);
        if (iVar1 == 6) {
          FUN_1406ed520(local_468,0x17e);
          local_478 = 0x15;
          FUN_1406ede20(local_468,&local_478,4);
          FUN_1415d01c0(local_468);
          *(undefined4 *)(param_1 + 0x1764) = 1;
          *(undefined4 *)(param_1 + 0x1768) = 1;
          FUN_1406ed610(local_468);
        }
      }
      if (*(int *)(param_1 + 0x176c) == 1) {
        uVar2 = FUN_1408a9d20(&local_478,0x1fb);
        FUN_141c3fdd0(param_1,uVar2,2);
      }
    }
  }
  return;
}



//===========================================================
// FUN_141e9c0f0 @ 141e9c0f0   (1 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141e9c0f0(longlong param_1)

{
  int iVar1;
  undefined8 uVar2;
  undefined1 auStack_4d8 [32];
  undefined4 uStack_4b8;
  undefined4 uStack_4b0;
  undefined4 uStack_4a8;
  undefined4 uStack_4a0;
  undefined4 uStack_498;
  undefined4 uStack_490;
  undefined4 uStack_488;
  undefined4 auStack_478 [4];
  undefined1 auStack_468 [1104];
  ulonglong uStack_18;
  
  uStack_18 = DAT_143a8b908 ^ (ulonglong)auStack_4d8;
  if (0 < *(int *)(param_1 + 0x1b38)) {
    if (((*(int *)(param_1 + 0x1764) == 0) && (*(int *)(param_1 + 0x1768) == 0)) &&
       (*(int *)(param_1 + 0x176c) == 0)) {
      uVar2 = FUN_1408a9e40(auStack_478,0x1fd);
      uStack_488 = 0;
      uStack_490 = 0;
      uStack_498 = 3;
      uStack_4a0 = 0;
      uStack_4a8 = 0;
      uStack_4b0 = 0xffffffff;
      uStack_4b8 = 0;
      iVar1 = FUN_142a269c0(uVar2,0,param_1 + 0x240,1);
      if (iVar1 == 6) {
        FUN_1406ed520(auStack_468,0x17e);
        auStack_478[0] = 0x15;
        FUN_1406ede20(auStack_468,auStack_478,4);
        FUN_1415d01c0(auStack_468);
        *(undefined4 *)(param_1 + 0x1764) = 1;
        *(undefined4 *)(param_1 + 0x1768) = 1;
        FUN_1406ed610(auStack_468);
      }
    }
    if (*(int *)(param_1 + 0x176c) == 1) {
      uVar2 = FUN_1408a9d20(auStack_478,0x1fb);
      FUN_141c3fdd0(param_1,uVar2,2);
    }
  }
  return;
}


