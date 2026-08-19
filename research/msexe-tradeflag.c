
//===========================================================
// FUN_14038cf10 @ 14038cf10   (132 bytes)
//===========================================================

undefined8 FUN_14038cf10(undefined8 param_1,longlong *param_2)

{
  char cVar1;
  int iVar2;
  undefined4 uVar3;
  
  iVar2 = (**(code **)(*param_2 + 0x28))(param_2);
  if (iVar2 == 0) {
    uVar3 = FUN_1401b0340(param_2 + 4);
    iVar2 = FUN_14038ce90(param_1,uVar3);
    if (iVar2 != 0) {
      return 1;
    }
    cVar1 = (**(code **)(*param_2 + 0x200))(param_2);
    if (cVar1 != '\0') {
      uVar3 = FUN_1401b0340(param_2 + 4);
      iVar2 = FUN_14038aae0(param_1,uVar3);
      if (iVar2 == 0) {
        return 1;
      }
    }
  }
  return 0;
}



//===========================================================
// FUN_140389c10 @ 140389c10   (83 bytes)
//===========================================================

undefined4 FUN_140389c10(undefined8 param_1,int param_2)

{
  longlong lVar1;
  
  if (param_2 - 5000000U < 1000000) {
    return 1;
  }
  if ((param_2 - 1000000U < 1000000) || (param_2 - 6000000U < 1000000)) {
    lVar1 = FUN_140388c60();
  }
  else {
    lVar1 = FUN_14039b100();
  }
  if (lVar1 != 0) {
    return *(undefined4 *)(lVar1 + 0x18);
  }
  return 0;
}



//===========================================================
// FUN_14038ab40 @ 14038ab40   (127 bytes)
//===========================================================

undefined8 FUN_14038ab40(undefined8 param_1,longlong *param_2)

{
  char cVar1;
  int iVar2;
  undefined4 uVar3;
  undefined8 uVar4;
  
  if (param_2 == (longlong *)0x0) {
    return 0;
  }
  iVar2 = (**(code **)(*param_2 + 0x28))(param_2);
  if (iVar2 != 0) {
    return 1;
  }
  cVar1 = (**(code **)(*param_2 + 0x2f8))(param_2);
  if (cVar1 != '\0') {
    FUN_1401b0340(param_2 + 4);
  }
  uVar3 = FUN_1401b0340(param_2 + 4);
  uVar4 = FUN_14038aae0(param_1,uVar3);
  return uVar4;
}



//===========================================================
// FUN_14038ac80 @ 14038ac80   (324 bytes)
//===========================================================

bool FUN_14038ac80(undefined8 param_1,int param_2)

{
  char cVar1;
  int iVar2;
  longlong lVar3;
  bool bVar4;
  longlong *local_res18;
  longlong *local_res20;
  
  cVar1 = FUN_140841850();
  if (cVar1 != '\0') {
    if (param_2 - 5000000U < 1000000) {
      return true;
    }
    if ((param_2 - 1000000U < 1000000) || (param_2 - 6000000U < 1000000)) {
      lVar3 = FUN_140388c60(param_1,param_2);
    }
    else {
      lVar3 = FUN_14039b100(param_1,param_2);
    }
    if ((lVar3 != 0) && (*(int *)(lVar3 + 0x18) != 0)) {
      return true;
    }
  }
  if (param_2 < 0x4e6e21) {
    if (param_2 != 0x4e6e20) {
      switch(param_2) {
      case 0x4cc070:
      case 0x4cc071:
      case 0x4cc072:
      case 0x4cc073:
      case 0x4cc074:
      case 0x4cc075:
      case 0x4cc078:
      case 0x4cc079:
      case 0x4cc07a:
      case 0x4cc07b:
        break;
      default:
        goto switchD_14038ad14_caseD_4cc076;
      }
    }
  }
  else {
    if (param_2 < 0x4e6e25) {
      if (param_2 == 0x4e6e24) {
        return true;
      }
      if (param_2 == 0x4e6e21) {
        return true;
      }
      if (param_2 == 0x4e6e22) {
        return true;
      }
      bVar4 = param_2 == 0x4e6e23;
    }
    else {
      if (param_2 == 0x4fcdb0) {
        return true;
      }
      bVar4 = param_2 == 0x4fcdb3;
    }
    if (!bVar4) {
switchD_14038ad14_caseD_4cc076:
      FUN_14039f600(param_1,&local_res20,param_2,0);
      if (local_res20 == (longlong *)0x0) {
        bVar4 = false;
      }
      else {
        local_res18 = local_res20;
        (**(code **)(*local_res20 + 8))(local_res20);
        iVar2 = FUN_140910eb0(&local_res18,L"cashTradeBlock",0);
        bVar4 = iVar2 != 0;
        (**(code **)(*local_res20 + 0x10))(local_res20);
      }
      return bVar4;
    }
  }
  return true;
}



//===========================================================
// FUN_1402fdb20 @ 1402fdb20   (122 bytes)
//===========================================================

undefined8 FUN_1402fdb20(longlong *param_1)

{
  char cVar1;
  int iVar2;
  ulonglong uVar3;
  
  cVar1 = FUN_140841850();
  if (cVar1 == '\0') {
    uVar3 = FUN_1401ab420((longlong)param_1 + 0x10a,*(undefined4 *)((longlong)param_1 + 0x10e));
    if ((uVar3 & 0x10) != 0) {
      iVar2 = FUN_1401b0340(param_1 + 4);
      if (((iVar2 / 1000000 == 1) && (param_1[7] != 0)) || (param_1[7] == 0)) {
        cVar1 = (**(code **)(*param_1 + 0x2f8))(param_1);
        if (cVar1 == '\0') {
          return 1;
        }
      }
    }
  }
  return 0;
}



//===========================================================
// FUN_1402fd3d0 @ 1402fd3d0   (35 bytes)
//===========================================================

uint FUN_1402fd3d0(longlong param_1)

{
  uint uVar1;
  
  uVar1 = FUN_1401ab420(param_1 + 0x10a,*(undefined4 *)(param_1 + 0x10e));
  return (uVar1 & 0x1000) >> 0xc;
}



//===========================================================
// FUN_1402fdcc0 @ 1402fdcc0   (30 bytes)
//===========================================================

uint FUN_1402fdcc0(longlong param_1)

{
  uint uVar1;
  
  uVar1 = FUN_1401ab420(param_1 + 0x10a,*(undefined4 *)(param_1 + 0x10e));
  return uVar1 & 1;
}



//===========================================================
// FUN_14038a960 @ 14038a960   (75 bytes)
//===========================================================

undefined4 FUN_14038a960(undefined8 param_1,int param_2)

{
  longlong lVar1;
  
  if (999999 < param_2 - 5000000U) {
    if ((param_2 - 1000000U < 1000000) || (param_2 - 6000000U < 1000000)) {
      lVar1 = FUN_140388c60();
    }
    else {
      lVar1 = FUN_14039b100();
    }
    if (lVar1 != 0) {
      return *(undefined4 *)(lVar1 + 0x2c);
    }
  }
  return 0;
}



//===========================================================
// FUN_14038c1f0 @ 14038c1f0   (175 bytes)
//===========================================================

undefined8 FUN_14038c1f0(undefined8 param_1,longlong *param_2)

{
  char cVar1;
  undefined4 uVar2;
  int iVar3;
  
  if (param_2 == (longlong *)0x0) {
    return 0;
  }
  uVar2 = FUN_1401b0340(param_2 + 4);
  iVar3 = FUN_14038c0f0(param_1,uVar2);
  if ((iVar3 == 0) && (iVar3 = (**(code **)(*param_2 + 0x68))(param_2), iVar3 == 0)) {
    uVar2 = FUN_1401b0340(param_2 + 4);
    iVar3 = FUN_14038c170(param_1,uVar2);
    if ((iVar3 != 0) && (iVar3 = (**(code **)(*param_2 + 0x30))(param_2), iVar3 != 0)) {
      return 1;
    }
    cVar1 = (**(code **)(*param_2 + 0x2f8))(param_2);
    if (cVar1 == '\0') {
      return 0;
    }
    FUN_1401b0340(param_2 + 4);
  }
  return 1;
}



//===========================================================
// FUN_1402fb7e0 @ 1402fb7e0   (258 bytes)
//===========================================================

void FUN_1402fb7e0(longlong param_1)

{
  undefined4 uVar1;
  
  uVar1 = FUN_1402f7010(0,param_1);
  *(undefined4 *)(param_1 + 4) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 8);
  *(undefined4 *)(param_1 + 0xc) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x10);
  *(undefined4 *)(param_1 + 0x14) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x18);
  *(undefined4 *)(param_1 + 0x1c) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x20);
  *(undefined4 *)(param_1 + 0x24) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x28);
  *(undefined4 *)(param_1 + 0x2c) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x30);
  *(undefined4 *)(param_1 + 0x34) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x38);
  *(undefined4 *)(param_1 + 0x3c) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x40);
  *(undefined4 *)(param_1 + 0x44) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x48);
  *(undefined4 *)(param_1 + 0x4c) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x50);
  *(undefined4 *)(param_1 + 0x54) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x58);
  *(undefined4 *)(param_1 + 0x5c) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x60);
  *(undefined4 *)(param_1 + 100) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x68);
  *(undefined4 *)(param_1 + 0x6c) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x70);
  *(undefined4 *)(param_1 + 0x74) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x78);
  *(undefined4 *)(param_1 + 0x7c) = uVar1;
  uVar1 = FUN_1402f7010(0,param_1 + 0x80);
  *(undefined4 *)(param_1 + 0x84) = uVar1;
  return;
}


