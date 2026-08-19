
//===========================================================
// FUN_14030b560 @ 14030b560   (386 bytes)
//===========================================================

void FUN_14030b560(undefined8 *param_1,longlong param_2)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  char cVar3;
  int iVar4;
  undefined8 *puVar5;
  longlong *plVar6;
  longlong *plVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  undefined8 *puVar10;
  longlong local_res10;
  longlong local_res18;
  undefined1 local_res20 [8];
  undefined1 local_28 [16];
  
  if (*(longlong *)(param_2 + 8) != 0) {
    iVar4 = FUN_14019a5d0(*(longlong *)(param_2 + 8) + 0x20);
    if (iVar4 - 0x195460U < 10000) {
      plVar7 = *(longlong **)(param_2 + 8);
      if (plVar7 == (longlong *)0x0) {
        FUN_142e52ed0(0x431,0);
        plVar7 = *(longlong **)(param_2 + 8);
      }
      puVar5 = (undefined8 *)(**(code **)(*plVar7 + 0x330))();
      if (puVar5 != (undefined8 *)0x0) {
        plVar7 = (longlong *)*param_1;
        plVar6 = *(longlong **)(param_2 + 8);
        if (plVar6 == (longlong *)0x0) {
          FUN_142e52ed0(0x431,0);
          plVar6 = *(longlong **)(param_2 + 8);
        }
        plVar6 = (longlong *)(**(code **)(*plVar6 + 0x80))(plVar6,local_res20);
        puVar1 = (undefined8 *)*plVar7;
        puVar10 = puVar1;
        if (*(char *)((longlong)puVar1[1] + 0x19) == '\0') {
          puVar8 = (undefined8 *)puVar1[1];
          do {
            if ((longlong)puVar8[4] < *plVar6) {
              puVar9 = (undefined8 *)puVar8[2];
            }
            else {
              puVar9 = (undefined8 *)*puVar8;
              puVar10 = puVar8;
            }
            puVar8 = puVar9;
          } while (*(char *)((longlong)puVar9 + 0x19) == '\0');
        }
        if ((*(char *)((longlong)puVar10 + 0x19) != '\0') || (*plVar6 < (longlong)puVar10[4])) {
          puVar10 = puVar1;
        }
        if (puVar10 != *(undefined8 **)*param_1) {
          puVar1 = puVar10 + 5;
          cVar3 = FUN_14030b260(puVar5,puVar1);
          if (cVar3 != '\0') {
            plVar7 = (longlong *)FUN_1401a19e0(param_2);
            (**(code **)(*plVar7 + 0x80))(plVar7,local_28);
            FUN_14030cbc0(puVar1,&local_res18);
            FUN_14030cbc0(puVar5,&local_res10);
            if (local_res10 != 0) {
              FUN_14019f2c0(local_res10 + -0x10);
            }
            if (local_res18 != 0) {
              FUN_14019f2c0(local_res18 + -0x10);
            }
            uVar2 = puVar10[6];
            *puVar5 = *puVar1;
            puVar5[1] = uVar2;
            uVar2 = puVar10[8];
            puVar5[2] = puVar10[7];
            puVar5[3] = uVar2;
            puVar5[4] = puVar10[9];
            *(undefined1 *)(puVar5 + 5) = *(undefined1 *)(puVar10 + 10);
          }
        }
      }
    }
  }
  return;
}



//===========================================================
// FUN_14030cbc0 @ 14030cbc0   (198 bytes)
//===========================================================

undefined8 * FUN_14030cbc0(undefined4 *param_1,undefined8 *param_2)

{
  undefined4 uVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  longlong local_res8;
  longlong local_res10;
  
  local_res10 = 0;
  uVar1 = param_1[1];
  puVar2 = (undefined8 *)FUN_1408f7bb0(&local_res8,(longlong)param_1 + 0x21,4);
  uVar3 = FUN_14019ba10(&local_res10,"%d:%d:%d:%s:%d:%s:%d",*param_1,param_1[2],param_1[3],
                        param_1 + 4,*(undefined4 *)((longlong)param_1 + 0x1d),*puVar2,uVar1);
  *param_2 = 0;
  FUN_14019a260(param_2,uVar3);
  if (local_res8 != 0) {
    FUN_14019f2c0(local_res8 + -0x10);
  }
  if (local_res10 != 0) {
    FUN_14019f2c0(local_res10 + -0x10);
  }
  return param_2;
}



//===========================================================
// FUN_14030ca50 @ 14030ca50   (354 bytes)
//===========================================================

void FUN_14030ca50(longlong param_1,longlong param_2)

{
  longlong *plVar1;
  undefined8 *puVar2;
  int iVar3;
  longlong lVar4;
  longlong local_res10;
  
  if (*(longlong **)(param_2 + 8) == (longlong *)0x0) {
    return;
  }
  iVar3 = (**(code **)(**(longlong **)(param_2 + 8) + 0x88))();
  if (iVar3 == 1) {
    local_res10 = *(longlong *)(param_2 + 8);
    if (local_res10 == 0) {
      return;
    }
    lVar4 = *(longlong *)(local_res10 + 8);
    if (lVar4 == 1) {
      *(undefined8 *)(param_2 + 8) = 0;
      FUN_14030c520(param_1,&local_res10);
      return;
    }
  }
  else if (iVar3 == 2) {
    local_res10 = *(longlong *)(param_2 + 8);
    if (local_res10 == 0) {
      return;
    }
    lVar4 = *(longlong *)(local_res10 + 8);
    if (lVar4 == 1) {
      *(undefined8 *)(param_2 + 8) = 0;
      FUN_14030c360(param_1 + 0x970,&local_res10);
      return;
    }
  }
  else {
    if (iVar3 != 3) {
      return;
    }
    local_res10 = *(longlong *)(param_2 + 8);
    if (local_res10 == 0) {
      return;
    }
    lVar4 = *(longlong *)(local_res10 + 8);
    if (lVar4 == 1) {
      *(undefined8 *)(param_2 + 8) = 0;
      FUN_14030c6e0(param_1 + 0x12e0,&local_res10);
      return;
    }
  }
  if (0xffffe < lVar4 - 1U) {
    FUN_142e541f0(0x31e);
  }
  LOCK();
  plVar1 = (longlong *)(local_res10 + 8);
  lVar4 = *plVar1;
  *plVar1 = *plVar1 + -1;
  UNLOCK();
  if (((int)lVar4 == 1) && (puVar2 = *(undefined8 **)(param_2 + 8), puVar2 != (undefined8 *)0x0)) {
    (**(code **)*puVar2)(puVar2,1);
  }
  *(undefined8 *)(param_2 + 8) = 0;
  return;
}


