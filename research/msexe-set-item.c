// The client's SetItem (2026-10-04): which invType a worn position is filed under. Type 1
// takes worn slots 1..31 (charData+0x1a8), type 6 takes 101..131 (charData+0x3a8); type 1 at
// -114 returns 0 and drops the item. Its getter twin FUN_1402e3770 is in msexe-equip-apply.c.
// net::inventory::worn_slot_tab.

//===========================================================
// FUN_1402e4c20 @ 1402e4c20   (480 bytes)
//===========================================================

undefined8 FUN_1402e4c20(longlong param_1,int param_2,int param_3,undefined8 param_4)

{
  longlong lVar1;
  int iVar2;
  undefined8 uVar3;
  longlong *plVar4;
  int iVar5;
  undefined1 local_18 [16];
  
  if (param_2 - 1U < 6) {
    if (param_2 == 1) {
      if (param_3 < 0) {
        iVar5 = -param_3;
        iVar2 = FUN_140302620(iVar5);
        if (iVar2 != 0) {
          uVar3 = FUN_140232590(local_18,param_4);
          FUN_1402de550(param_1 + 0x5a8,uVar3,iVar5);
          FUN_1401abd80(param_4);
          return 1;
        }
        if (0x1e < param_3 + 0x1fU) {
          FUN_1401abd80(param_4);
          return 0;
        }
        FUN_1401e8780(param_1 + 0x1a8 + (longlong)iVar5 * 0x10,param_4);
        FUN_1401abd80(param_4);
        return 1;
      }
    }
    else if ((param_2 == 6) && (param_3 < 0)) {
      if ((0xd < -param_3 - 0x4b0U) && (0x32 < -param_3 - 0x708U)) {
        if (0x1e < param_3 + 0x83U) {
          FUN_1401abd80(param_4);
          return 0;
        }
        FUN_1401e8780((longlong)(-100 - param_3) * 0x10 + 0x3a8 + param_1,param_4);
        FUN_1401abd80(param_4);
        return 1;
      }
      uVar3 = FUN_140232590(local_18,param_4);
      FUN_1402de550(param_1 + 0x5a8,uVar3,-param_3);
      FUN_1401abd80(param_4);
      return 1;
    }
    if (0 < param_3) {
      plVar4 = (longlong *)((longlong)param_2 * 8 + 0x5d0 + param_1);
      lVar1 = *plVar4;
      if ((lVar1 != 0) && (param_3 <= *(int *)(lVar1 + -8) + -1)) {
        uVar3 = FUN_1402f1620(plVar4,param_3);
        FUN_1401e8780(uVar3,param_4);
        FUN_1401abd80(param_4);
        return 1;
      }
    }
    FUN_1401abd80(param_4);
  }
  else {
    FUN_1401abd80(param_4);
  }
  return 0;
}


