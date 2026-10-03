
//===========================================================
// FUN_142a8a7d0 @ 142a8a7d0   (226 bytes)
//===========================================================

undefined8 FUN_142a8a7d0(longlong param_1)

{
  int iVar1;
  undefined8 uVar2;
  undefined2 *puVar3;
  longlong lVar4;
  undefined2 local_res8;
  undefined1 local_resa;
  
  iVar1 = *(int *)(param_1 + 0x2a8);
  if ((iVar1 == 0x17) || (iVar1 == 0x19)) {
    if (*(longlong *)(param_1 + 0x498) != 0) {
      lVar4 = *(longlong *)(param_1 + 0x498);
      if (lVar4 == 0) {
        FUN_142e52ed0(0x428,0);
        lVar4 = *(longlong *)(param_1 + 0x498);
      }
      iVar1 = FUN_14041a270(lVar4);
      if (iVar1 != 0) {
        lVar4 = *(longlong *)(param_1 + 0x498);
        if (lVar4 == 0) {
          FUN_142e52ed0(0x428,0);
          lVar4 = *(longlong *)(param_1 + 0x498);
        }
        uVar2 = FUN_14041a3d0(lVar4);
        return uVar2;
      }
    }
  }
  else if (((iVar1 == 0x1c) || (iVar1 == 0x1d)) && (*(longlong *)(param_1 + 0x498) != 0)) {
    puVar3 = *(undefined2 **)(param_1 + 0x498);
    if (puVar3 == (undefined2 *)0x0) {
      FUN_142e52ed0(0x428,0);
      puVar3 = *(undefined2 **)(param_1 + 0x498);
    }
    local_res8 = *puVar3;
    local_resa = 0x32;
    iVar1 = FUN_14041a270(&local_res8);
    if (iVar1 != 0) {
      uVar2 = FUN_14041a3d0(&local_res8);
      return uVar2;
    }
  }
  return 0;
}



//===========================================================
// FUN_1401a8170 @ 1401a8170   (460 bytes)
//===========================================================

undefined8 FUN_1401a8170(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  bool bVar3;
  
  iVar1 = FUN_1403e8af0();
  if (iVar1 == 5) {
    uVar2 = FUN_140417ed0(param_1);
    switch(uVar2) {
    case 1:
      return 0x15;
    case 2:
      goto switchD_1401a82c7_caseD_2;
    case 3:
      return 1;
    default:
      bVar3 = param_1 == 0x56ac78;
      break;
    case 0x1f:
      return 0xd;
    case 0x51:
      return 0x16;
    case 0x52:
      goto switchD_1401a82c7_caseD_52;
    case 0x56:
    case 0x5f:
      return 0xe;
    case 0x58:
      return 0x17;
    case 0x59:
    case 0x5e:
      return 0x18;
    case 0x61:
      return 3;
    }
  }
  else {
    if ((param_1 - 0x2c24c8U < 1000) || (param_1 - 0x2c3080U < 1000)) {
      return 2;
    }
    if ((((param_1 - 0x2c1910U < 1000) || (param_1 - 0x2c1cf8U < 1000)) ||
        (param_1 - 0x2c28b0U < 1000)) || (param_1 - 0x2c2c98U < 1000)) {
switchD_1401a82c7_caseD_52:
      return 0xc;
    }
    if (param_1 - 0x26c1e0U < 10000) {
      return 0x16;
    }
    iVar1 = FUN_1401abe60(param_1);
    if (iVar1 != 0) {
      return 0x18;
    }
    if (param_1 == 0x25232b) {
      return 0x18;
    }
    if (0x2528f7 < param_1) {
      if (param_1 == 0x3ddfd8) {
        return 0xb;
      }
      if (param_1 == 0x3ddfd9) {
        return 0x15;
      }
      if (param_1 == 0x3ddfdc) {
        return 0xb;
      }
      if (param_1 == 0x3ddfdd) {
        return 0x15;
      }
      return 0;
    }
    if (param_1 == 0x2528f7) {
      return 0xb;
    }
    if (param_1 == 0x251928) {
      return 0x1f;
    }
    if (param_1 == 0x251c15) {
      return 0xb;
    }
    if (param_1 == 0x252642) {
      return 0xb;
    }
    if (param_1 == 0x252643) {
      return 0xb;
    }
    bVar3 = param_1 == 0x2528f6;
  }
  if (!bVar3) {
    return 0;
  }
switchD_1401a82c7_caseD_2:
  return 0xb;
}



//===========================================================
// FUN_1401a8660 @ 1401a8660   (265 bytes)
//===========================================================

ulonglong FUN_1401a8660(undefined4 param_1,int param_2,uint param_3)

{
  char cVar1;
  undefined4 uVar2;
  ulonglong uVar3;
  
  if (0 < param_2) {
    switch(param_1) {
    case 0xb:
    case 0xc:
      uVar3 = FUN_14041a5b0(param_2,param_3);
      return uVar3;
    case 0xd:
      cVar1 = FUN_140419fa0(param_3);
      if (cVar1 != '\0') {
        uVar3 = FUN_14041a780(param_2,param_3);
        return uVar3;
      }
      uVar2 = FUN_14041a0a0(param_3);
      uVar3 = FUN_14041a780(param_2,uVar2);
      return uVar3;
    case 0xe:
      uVar3 = FUN_14041aa00(param_2,param_3);
      return uVar3;
    case 0x15:
    case 0x16:
      uVar3 = FUN_14041a680(param_2,param_3);
      return uVar3;
    case 0x17:
      cVar1 = FUN_140419fb0(param_3);
      if (cVar1 != '\0') {
        uVar3 = FUN_14041a7e0(param_2,param_3);
        return uVar3;
      }
      uVar2 = FUN_14041a0f0(param_3);
      uVar3 = FUN_14041a7e0(param_2,uVar2);
      return uVar3;
    case 0x18:
      uVar3 = FUN_14041aa80(param_2,param_3);
      return uVar3;
    }
  }
  return (ulonglong)param_3;
}



//===========================================================
// FUN_1401a8500 @ 1401a8500   (163 bytes)
//===========================================================

bool FUN_1401a8500(undefined4 param_1,int param_2,int param_3)

{
  undefined1 uVar1;
  int iVar2;
  int iVar3;
  
  switch(param_1) {
  case 1:
  case 2:
    iVar2 = FUN_140419fe0(param_2);
    iVar3 = FUN_140419fe0(param_3);
    return iVar2 == iVar3;
  default:
    return param_2 == param_3;
  case 0xb:
  case 0xc:
    uVar1 = FUN_14041a130(param_2,param_3);
    return (bool)uVar1;
  case 0xd:
  case 0xe:
  case 0xf:
    uVar1 = FUN_1401a9e00(param_2,param_3);
    return (bool)uVar1;
  case 0x15:
  case 0x16:
    uVar1 = FUN_14041a1e0(param_2,param_3);
    return (bool)uVar1;
  case 0x17:
  case 0x18:
  case 0x19:
    uVar1 = FUN_1401a9eb0(param_2,param_3);
    return (bool)uVar1;
  }
}



//===========================================================
// FUN_142a8a3e0 @ 142a8a3e0   (306 bytes)
//===========================================================

void FUN_142a8a3e0(undefined8 param_1,undefined4 param_2,undefined4 param_3,longlong *param_4,
                  longlong param_5,undefined4 param_6,undefined1 param_7)

{
  undefined4 uVar1;
  int iVar2;
  undefined4 *puVar3;
  undefined4 local_68;
  undefined4 local_64;
  undefined8 local_60;
  undefined8 local_58;
  undefined1 *local_50;
  undefined8 *local_48;
  undefined1 local_40 [8];
  undefined8 local_38;
  
  local_68 = 0;
  local_64 = 0;
  iVar2 = FUN_142dcd020(param_5,param_6,&local_68,&local_64,1);
  uVar1 = local_68;
  if (iVar2 != 0) {
    puVar3 = (undefined4 *)FUN_1401abb40(param_5 + 8,0xffffffff);
    *puVar3 = uVar1;
    local_50 = local_40;
    local_38 = 0;
    local_48 = &local_58;
    local_58 = 0;
    local_60 = 0;
    FUN_14019a260(&local_60,param_4);
    FUN_142a61900(param_1,param_2,param_3,&local_60,0,0,&local_58,local_40,0,0);
    FUN_142a8a190(param_1,param_5,param_6,param_7);
  }
  if (*param_4 != 0) {
    FUN_14019f2c0(*param_4 + -0x10);
  }
  return;
}



//===========================================================
// FUN_1405be990 @ 1405be990   (998 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_1405be990(short *param_1,longlong param_2)

{
  longlong *plVar1;
  ushort uVar2;
  longlong lVar3;
  longlong lVar4;
  undefined1 auStack_88 [32];
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58 [3];
  short local_40;
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_88;
  lVar4 = 0;
  if (*param_1 != 0) {
    local_68 = 0;
    local_60 = 0;
    FUN_1405bd060(&local_68,param_1);
    uVar2 = *(ushort *)(param_2 + 2) & 0xf;
    if (uVar2 == 1) {
      lVar3 = *(longlong *)(param_2 + 0x10);
    }
    else {
      lVar3 = lVar4;
      if (uVar2 == 2) {
        lVar3 = param_2 + 0x10;
      }
    }
    FUN_1406bb8f0(lVar3,"basecolor",&local_68);
    if ((short)local_68 != 0) {
      if (((short)local_68 == 5) || ((short)local_68 == 6)) {
        if ((local_68._2_1_ & 0xf0) == 0x20) {
          plVar1 = (longlong *)FUN_1406ba900(0);
          (**(code **)(*plVar1 + 8))(plVar1,local_58[0]);
        }
      }
      else if ((short)local_68 == 7) {
        if ((local_68._2_1_ & 0xf) == 2) {
          FUN_1405bc400(local_58,0);
        }
      }
      else if ((((short)local_68 == 8) && ((local_68._2_1_ & 0xf) == 2)) &&
              (FUN_1405bdd20(local_58), local_40 != 0)) {
        FUN_1405b36b0(local_58[0]);
        plVar1 = (longlong *)FUN_1406ba900(0);
        (**(code **)(*plVar1 + 8))(plVar1,local_58[0]);
      }
    }
  }
  if (param_1[0x20] != 0) {
    local_68 = 0;
    local_60 = 0;
    FUN_1405bd060(&local_68);
    uVar2 = *(ushort *)(param_2 + 2) & 0xf;
    if (uVar2 == 1) {
      lVar3 = *(longlong *)(param_2 + 0x10);
    }
    else {
      lVar3 = lVar4;
      if (uVar2 == 2) {
        lVar3 = param_2 + 0x10;
      }
    }
    FUN_1406bb8f0(lVar3,"itemid",&local_68);
    if ((short)local_68 != 0) {
      if (((short)local_68 == 5) || ((short)local_68 == 6)) {
        if ((local_68._2_1_ & 0xf0) == 0x20) {
          plVar1 = (longlong *)FUN_1406ba900(0);
          (**(code **)(*plVar1 + 8))(plVar1,local_58[0]);
        }
      }
      else if ((short)local_68 == 7) {
        if ((local_68._2_1_ & 0xf) == 2) {
          FUN_1405bc400(local_58,0);
        }
      }
      else if ((((short)local_68 == 8) && ((local_68._2_1_ & 0xf) == 2)) &&
              (FUN_1405bdd20(local_58), local_40 != 0)) {
        FUN_1405b36b0(local_58[0]);
        plVar1 = (longlong *)FUN_1406ba900(0);
        (**(code **)(*plVar1 + 8))(plVar1,local_58[0]);
      }
    }
  }
  if (param_1[0x40] != 0) {
    local_68 = 0;
    local_60 = 0;
    FUN_1405bd060(&local_68);
    uVar2 = *(ushort *)(param_2 + 2) & 0xf;
    if (uVar2 == 1) {
      lVar3 = *(longlong *)(param_2 + 0x10);
    }
    else {
      lVar3 = lVar4;
      if (uVar2 == 2) {
        lVar3 = param_2 + 0x10;
      }
    }
    FUN_1406bb8f0(lVar3,"mixcolor",&local_68);
    if ((short)local_68 != 0) {
      if (((short)local_68 == 5) || ((short)local_68 == 6)) {
        if ((local_68._2_1_ & 0xf0) == 0x20) {
          plVar1 = (longlong *)FUN_1406ba900(0);
          (**(code **)(*plVar1 + 8))(plVar1,local_58[0]);
        }
      }
      else if ((short)local_68 == 7) {
        if ((local_68._2_1_ & 0xf) == 2) {
          FUN_1405bc400(local_58,0);
        }
      }
      else if ((((short)local_68 == 8) && ((local_68._2_1_ & 0xf) == 2)) &&
              (FUN_1405bdd20(local_58), local_40 != 0)) {
        FUN_1405b36b0(local_58[0]);
        plVar1 = (longlong *)FUN_1406ba900(0);
        (**(code **)(*plVar1 + 8))(plVar1,local_58[0]);
      }
    }
  }
  if (param_1[0x60] != 0) {
    local_68 = 0;
    local_60 = 0;
    FUN_1405bd060(&local_68);
    uVar2 = *(ushort *)(param_2 + 2) & 0xf;
    if (uVar2 == 1) {
      lVar4 = *(longlong *)(param_2 + 0x10);
    }
    else if (uVar2 == 2) {
      lVar4 = param_2 + 0x10;
    }
    FUN_1406bb8f0(lVar4,"mixcolorprob",&local_68);
    if ((short)local_68 != 0) {
      if (((short)local_68 == 5) || ((short)local_68 == 6)) {
        if ((local_68._2_1_ & 0xf0) == 0x20) {
          plVar1 = (longlong *)FUN_1406ba900(0);
          (**(code **)(*plVar1 + 8))(plVar1,local_58[0]);
        }
      }
      else if ((short)local_68 == 7) {
        if ((local_68._2_1_ & 0xf) == 2) {
          FUN_1405bc400(local_58,0);
        }
      }
      else if ((((short)local_68 == 8) && ((local_68._2_1_ & 0xf) == 2)) &&
              (FUN_1405bdd20(local_58), local_40 != 0)) {
        FUN_1405b36b0(local_58[0]);
        plVar1 = (longlong *)FUN_1406ba900(0);
        (**(code **)(*plVar1 + 8))(plVar1,local_58[0]);
      }
    }
  }
  return;
}


