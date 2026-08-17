
//===========================================================
// FUN_141b3f050 @ 141b3f050   (560 bytes)
//===========================================================

void FUN_141b3f050(longlong param_1,int param_2,int param_3)

{
  int iVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong *local_res8;
  longlong local_res20;
  ushort uStack_4e;
  
  iVar1 = *(int *)(param_1 + 0xd0);
  *(int *)(param_1 + 0x238) = param_2;
  iVar3 = FUN_1429e3ef0();
  *(undefined1 *)(param_1 + 0x240) = 1;
  *(bool *)(param_1 + 0x241) = param_3 == 0;
  *(int *)(param_1 + 0x244) = iVar3;
  *(int *)(param_1 + 0x248) = (int)((double)param_3 * DAT_14327aa58 + (double)iVar3);
  if (param_3 == 0) {
    *(undefined4 *)(param_1 + 0x23c) = 0;
    FUN_141b3f290(param_1);
    FUN_141b3e060(param_1,*(undefined4 *)(param_1 + 0x238));
    *(undefined8 *)(param_1 + 0x238) = 0;
    *(ulonglong *)(param_1 + 0x240) = (ulonglong)uStack_4e << 0x10;
    *(undefined8 *)(param_1 + 0x248) = 0;
    *(undefined8 *)(param_1 + 0x250) = 0;
    *(undefined4 *)(param_1 + 600) = 0;
  }
  else if ((((iVar1 == 3) || (iVar1 == 4)) || (iVar1 == 5)) &&
          (((param_2 == 3 || (param_2 == 4)) || (param_2 == 5)))) {
    *(undefined4 *)(param_1 + 0x23c) = 2;
    FUN_141b3e1d0(param_1,param_2,param_3);
  }
  else {
    *(undefined4 *)(param_1 + 0x23c) = 1;
    iVar4 = param_3 / 2;
    FUN_140de1ce0(DAT_143abfdf0,&local_res8,iVar4,0,param_3 - iVar4,0x27e2,0xff,0xff000000,0,0);
    if (local_res8 != (longlong *)0x0) {
      (**(code **)(*local_res8 + 0x10))();
    }
    *(int *)(param_1 + 0x244) = iVar4 + iVar3;
  }
  uVar5 = FUN_141b3b980(param_1,&local_res20,iVar1);
  uVar6 = FUN_141b3b980(param_1,&local_res8,*(undefined4 *)(param_1 + 0x238));
  cVar2 = FUN_1407330d0(uVar5,uVar6);
  if (local_res8 != (longlong *)0x0) {
    FUN_1401bebb0(local_res8 + -2);
  }
  if (local_res20 != 0) {
    FUN_1401bebb0(local_res20 + -0x10);
  }
  if ((cVar2 != '\0') && (DAT_143abfea0 != 0)) {
    FUN_142081a20(DAT_143abfea0,1,1000,1000,0,0);
  }
  return;
}


