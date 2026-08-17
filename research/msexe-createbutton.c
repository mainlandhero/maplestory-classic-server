
//===========================================================
// FUN_141b28930 @ 141b28930   (26 bytes)
//===========================================================

void FUN_141b28930(longlong param_1)

{
  if (*(int *)(param_1 + 0x238) == 0) {
    FUN_141b3f050(param_1,4,0x14a);
    return;
  }
  return;
}



//===========================================================
// FUN_14112a570 @ 14112a570   (418 bytes)
//===========================================================

void FUN_14112a570(longlong param_1,undefined4 param_2)

{
  longlong *plVar1;
  char cVar2;
  int iVar3;
  longlong lVar4;
  undefined8 uVar5;
  undefined8 *puVar6;
  undefined1 local_18 [8];
  longlong local_10;
  
  lVar4 = FUN_14112b010();
  if (lVar4 != 0) {
    uVar5 = FUN_14112b010();
    cVar2 = FUN_141b3faf0(uVar5);
    if (cVar2 == '\0') {
      cVar2 = FUN_142aa1a20(param_1 + 0x240,L"login",param_2);
      if (cVar2 == '\0') {
        cVar2 = FUN_142aa1a20(param_1 + 0x240,L"login_saved",param_2);
        if (cVar2 == '\0') {
          cVar2 = FUN_142aa1a20(param_1 + 0x240,L"quit",param_2);
          if (cVar2 == '\0') {
            FUN_142bf5d70(param_1,param_2);
          }
          else {
            cVar2 = FUN_141b3fd10(uVar5);
            if (cVar2 != '\0') {
              FUN_141b2d4a0(uVar5,0,0);
            }
          }
        }
        else {
          FUN_141ad7170(param_1 + 0x240,local_18,L"check_saved");
          if (local_10 != 0) {
            iVar3 = FUN_141690380(local_10);
            if (local_10 == 0) {
              FUN_142e52ed0(0x431,0);
            }
            FUN_141690300(local_10,iVar3 == 0);
          }
          lVar4 = local_10;
          if (local_10 != 0) {
            if (0xffffe < *(longlong *)(local_10 + 0x20) - 1U) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            plVar1 = (longlong *)(lVar4 + 0x20);
            lVar4 = *plVar1;
            *plVar1 = *plVar1 + -1;
            UNLOCK();
            if ((int)lVar4 == 1) {
              puVar6 = (undefined8 *)(local_10 + 0x18);
              if (local_10 == 0) {
                puVar6 = (undefined8 *)0x0;
              }
              if (puVar6 != (undefined8 *)0x0) {
                (**(code **)*puVar6)(puVar6,1);
              }
            }
          }
        }
      }
      else {
        cVar2 = FUN_141b3fd10(uVar5);
        if (cVar2 == '\0') {
          FUN_141b3f050(uVar5,4,600);
        }
        else {
          FUN_141b3ff10();
        }
      }
    }
  }
  return;
}



//===========================================================
// FUN_141b46170 @ 141b46170   (330 bytes)
//===========================================================

void FUN_141b46170(undefined8 param_1,undefined4 param_2)

{
  int iVar1;
  longlong lVar2;
  char cVar3;
  longlong lVar4;
  
  lVar4 = FUN_14112b010();
  if (lVar4 == 0) {
    return;
  }
  lVar4 = FUN_14112b010();
  if (*(int *)(lVar4 + 0xd4) != 0) {
    return;
  }
  if (*(int *)(lVar4 + 0xd8) != 0) {
    return;
  }
  switch(param_2) {
  case 1000:
    FUN_141b2d290(lVar4,1);
    break;
  case 0x3e9:
  case 0x3ed:
    FUN_141b2d4a0(lVar4,0,0);
    break;
  case 0x3ea:
  case 0x3ec:
    iVar1 = *(int *)(lVar4 + 0xd0);
    if (iVar1 != 3) {
      if (iVar1 == 4) {
        cVar3 = FUN_141b3fd10(lVar4);
        if (cVar3 == '\0') {
          FUN_141b3f050(lVar4,3,600);
        }
        else {
          FUN_141b2d290(lVar4,1);
        }
      }
      else if (iVar1 == 5) {
        FUN_141b28930(lVar4);
      }
      else {
        FUN_141b2d290(lVar4,1);
      }
      break;
    }
    cVar3 = FUN_141b3fd10(lVar4);
    if (cVar3 != '\0') {
      FUN_141b2d4a0(lVar4,0,0);
      break;
    }
    goto LAB_141b4626e;
  case 0x3eb:
LAB_141b4626e:
    FUN_141b3bfd0(lVar4);
  }
  lVar2 = DAT_143abfdf8;
  if ((DAT_143abfdf8 != 0) && ((*(int *)(lVar4 + 0xd0) - 2U & 0xfffffffd) == 0)) {
    lVar4 = FUN_141b2dbe0(lVar4);
    if (lVar4 == 0) {
      lVar4 = 0;
    }
    else {
      lVar4 = lVar4 + 8;
    }
    FUN_142c0bf50(lVar2,lVar4,0);
  }
  return;
}


