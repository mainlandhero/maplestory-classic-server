
//===========================================================
// FUN_1403e8c40 @ 1403e8c40   (242 bytes)
//===========================================================

ulonglong FUN_1403e8c40(longlong *param_1)

{
  undefined8 uVar1;
  int iVar2;
  undefined4 uVar3;
  ulonglong in_RAX;
  ulonglong uVar4;
  
  uVar1 = DAT_143aa8328;
  if (param_1 == (longlong *)0x0) goto LAB_1403e8d20;
  iVar2 = FUN_1401b0340(param_1 + 4);
  if ((iVar2 - 1000000U < 1000000) || (iVar2 - 6000000U < 1000000)) {
    in_RAX = FUN_140388c60(uVar1,iVar2);
    if (in_RAX == 0) {
      iVar2 = 0;
    }
    else {
      iVar2 = *(int *)(in_RAX + 0x1a8);
    }
LAB_1403e8cb8:
    if ((iVar2 == 1) || (iVar2 == 3)) goto LAB_1403e8d20;
  }
  else {
    in_RAX = FUN_14039b100(uVar1,iVar2);
    if (in_RAX != 0) {
      iVar2 = *(int *)(in_RAX + 0x48);
      goto LAB_1403e8cb8;
    }
    iVar2 = 0;
  }
  uVar3 = FUN_1401b0340(param_1 + 4);
  in_RAX = FUN_140389c10(DAT_143aa8328,uVar3);
  if ((int)in_RAX == 0) {
    if (iVar2 == 0) {
      iVar2 = (**(code **)(*param_1 + 0x28))(param_1);
      if (iVar2 == 0) {
        in_RAX = FUN_14038aae0(DAT_143aa8328,uVar3);
        if ((int)in_RAX != 0) goto LAB_1403e8d20;
      }
    }
                    /* WARNING: Could not recover jumptable at 0x0001403e8d19. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    uVar4 = (**(code **)(*param_1 + 0x200))(param_1);
    return uVar4;
  }
LAB_1403e8d20:
  return in_RAX & 0xffffffffffffff00;
}



//===========================================================
// FUN_14038c560 @ 14038c560   (241 bytes)
//===========================================================

undefined8 FUN_14038c560(undefined8 param_1,longlong *param_2,longlong *param_3)

{
  int iVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  longlong *local_res10;
  longlong *local_res20;
  
  if (param_2 != (longlong *)0x0) {
    iVar1 = (**(code **)(*param_2 + 0x50))(param_2);
    if (iVar1 != 0) {
      iVar1 = FUN_14019a5d0(param_2 + 4);
      if ((iVar1 - 1000000U < 1000000) || (iVar1 - 6000000U < 1000000)) {
        uVar2 = FUN_14019a5d0(param_2 + 4);
        FUN_14039f600(param_1,&local_res20,uVar2,0);
        if (local_res20 == (longlong *)0x0) {
          uVar3 = 0;
        }
        else {
          local_res10 = local_res20;
          (**(code **)(*local_res20 + 8))(local_res20);
          iVar1 = FUN_140910eb0(&local_res10,L"reqLevel",0);
          uVar3 = 1;
          if (iVar1 < 1) {
            iVar1 = 1;
          }
          else {
            iVar1 = iVar1 * 10;
          }
          *param_3 = (longlong)iVar1;
        }
        if (local_res20 != (longlong *)0x0) {
          (**(code **)(*local_res20 + 0x10))(local_res20);
        }
        return uVar3;
      }
    }
  }
  return 0;
}



//===========================================================
// FUN_14038c0f0 @ 14038c0f0   (116 bytes)
//===========================================================

undefined8 FUN_14038c0f0(undefined8 param_1,undefined4 param_2)

{
  undefined *puVar1;
  int iVar2;
  undefined8 uVar3;
  longlong *local_res18;
  longlong *local_res20;
  
  FUN_14039f600(param_1,&local_res20,param_2,0);
  puVar1 = PTR_u_accountSharable_143a45ba8;
  if (local_res20 != (longlong *)0x0) {
    local_res18 = local_res20;
    (**(code **)(*local_res20 + 8))(local_res20);
    iVar2 = FUN_140910eb0(&local_res18,puVar1,0);
    if (iVar2 != 0) {
      uVar3 = 1;
      goto LAB_14038c148;
    }
  }
  uVar3 = 0;
LAB_14038c148:
  if (local_res20 != (longlong *)0x0) {
    (**(code **)(*local_res20 + 0x10))(local_res20);
  }
  return uVar3;
}



//===========================================================
// FUN_14038c3c0 @ 14038c3c0   (46 bytes)
//===========================================================

undefined8 FUN_14038c3c0(undefined8 param_1,longlong param_2)

{
  undefined4 uVar1;
  undefined8 uVar2;
  
  if (param_2 == 0) {
    return 0;
  }
  uVar1 = FUN_1401b0340(param_2 + 0x20);
  uVar2 = FUN_14038c340(param_1,uVar1);
  return uVar2;
}



//===========================================================
// FUN_1403e8d40 @ 1403e8d40   (183 bytes)
//===========================================================

undefined4 FUN_1403e8d40(longlong param_1)

{
  undefined8 uVar1;
  char cVar2;
  int iVar3;
  undefined4 uVar4;
  
  cVar2 = FUN_140841850();
  uVar1 = DAT_143aa8328;
  if (((cVar2 == '\0') && (param_1 != 0)) &&
     (iVar3 = FUN_14038c1f0(DAT_143aa8328,param_1), iVar3 == 0)) {
    uVar4 = FUN_1401b0340(param_1 + 0x20);
    iVar3 = FUN_14038c340(uVar1,uVar4);
    if (iVar3 == 0) {
      iVar3 = FUN_14038be30(uVar1,uVar4);
      if (iVar3 == 0) {
        iVar3 = FUN_14038ce90(uVar1,uVar4);
        if (iVar3 != 0) {
          return 0;
        }
        iVar3 = FUN_14038aae0(uVar1,uVar4);
        if (iVar3 != 0) {
          return 0;
        }
      }
      cVar2 = FUN_1401b0050(param_1 + 0x1a6,*(undefined4 *)(param_1 + 0x1aa));
      if (cVar2 == -1) {
        return 1;
      }
    }
  }
  return 0;
}


