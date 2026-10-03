
//===========================================================
// FUN_14041a8d0 @ 14041a8d0   (113 bytes)
//===========================================================

uint FUN_14041a8d0(uint param_1,uint param_2,uint param_3,int param_4)

{
  if (((param_2 < 8) && (param_3 < 8)) && (param_4 - 1U < 99)) {
    if (9999999 < (int)param_1) {
      param_1 = param_1 / 1000;
    }
    return (((param_2 + ((int)param_1 / 10) * 10) * 10 - param_4 / 100) + param_3) * 100 + param_4;
  }
  return param_1;
}



//===========================================================
// FUN_14041a820 @ 14041a820   (162 bytes)
//===========================================================

int FUN_14041a820(uint param_1,int param_2,int param_3,int param_4)

{
  if (9999999 < (int)param_1) {
    param_1 = param_1 / 1000;
  }
  return (((((((int)param_1 / 1000) * 10 - (int)param_1 / 100) + param_2) * 100 + param_1) * 10 -
          param_4 / 100) + param_3 % 10) * 100 + param_4;
}



//===========================================================
// FUN_140419ea0 @ 140419ea0   (10 bytes)
//===========================================================

bool FUN_140419ea0(int param_1)

{
  return 9999999 < param_1;
}



//===========================================================
// FUN_14041a520 @ 14041a520   (127 bytes)
//===========================================================

void FUN_14041a520(uint param_1,char *param_2)

{
  char cVar1;
  
  if (9999999 < (int)param_1) {
    cVar1 = (char)((param_1 % 1000) / 100);
    *param_2 = (char)(param_1 / 1000) + (char)((param_1 / 1000) / 10) * -10;
    param_2[1] = cVar1;
    param_2[2] = (char)(param_1 % 1000) + cVar1 * -100;
    return;
  }
  *param_2 = -1;
  param_2[1] = -1;
  param_2[2] = '\0';
  return;
}



//===========================================================
// FUN_14041a480 @ 14041a480   (148 bytes)
//===========================================================

void FUN_14041a480(uint param_1,char *param_2)

{
  char cVar1;
  
  if (9999999 < (int)param_1) {
    cVar1 = (char)((param_1 % 1000) / 100);
    *param_2 = (char)(param_1 / 100000) + (char)((param_1 / 100000) / 10) * -10;
    param_2[1] = cVar1;
    param_2[2] = (char)(param_1 % 1000) + cVar1 * -100;
    return;
  }
  *param_2 = -1;
  param_2[1] = -1;
  param_2[2] = '\0';
  return;
}



//===========================================================
// FUN_14041a1e0 @ 14041a1e0   (105 bytes)
//===========================================================

undefined4 FUN_14041a1e0(uint param_1,uint param_2)

{
  int iVar1;
  
  if (9999999 < (int)param_1) {
    param_1 = param_1 / 1000;
  }
  if (9999999 < (int)param_2) {
    param_2 = param_2 / 1000;
  }
  iVar1 = ((int)param_1 / 10) * 10;
  return CONCAT31((int3)((uint)iVar1 >> 8),iVar1 == ((int)param_2 / 10) * 10);
}



//===========================================================
// FUN_14041a130 @ 14041a130   (162 bytes)
//===========================================================

undefined4 FUN_14041a130(uint param_1,uint param_2)

{
  int iVar1;
  
  if (9999999 < (int)param_1) {
    param_1 = param_1 / 1000;
  }
  if (9999999 < (int)param_2) {
    param_2 = param_2 / 1000;
  }
  iVar1 = (((int)param_1 / 1000) * 10 - (int)param_1 / 100) * 100 + param_1;
  return CONCAT31((int3)((uint)iVar1 >> 8),
                  iVar1 == (((int)param_2 / 1000) * 10 - (int)param_2 / 100) * 100 + param_2);
}



//===========================================================
// FUN_14041a2a0 @ 14041a2a0   (157 bytes)
//===========================================================

undefined8 FUN_14041a2a0(byte *param_1,byte *param_2)

{
  byte bVar1;
  byte bVar2;
  int iVar3;
  int iVar4;
  undefined8 uVar5;
  
  bVar1 = *param_1;
  uVar5 = 1;
  if (((bVar1 < 10) && (param_1[1] < 10)) && ((byte)(param_1[2] - 1) < 99)) {
    iVar4 = 1;
  }
  else {
    iVar4 = 0;
  }
  bVar2 = *param_2;
  if (((bVar2 < 10) && (param_2[1] < 10)) && ((byte)(param_2[2] - 1) < 99)) {
    iVar3 = 1;
  }
  else {
    iVar3 = 0;
  }
  if (iVar4 != iVar3) {
    return 0;
  }
  if (iVar4 != 0) {
    if (bVar1 == bVar2) {
      if ((param_1[1] == param_2[1]) && (param_1[2] == param_2[2])) {
        return 1;
      }
    }
    else {
      if (bVar1 != param_2[1]) {
        return 0;
      }
      if (param_1[1] == bVar2) {
        if ((uint)param_1[2] == 100 - param_2[2]) {
          return 1;
        }
        return 0;
      }
    }
    uVar5 = 0;
  }
  return uVar5;
}



//===========================================================
// FUN_142dcd020 @ 142dcd020   (602 bytes)
//===========================================================

undefined8
FUN_142dcd020(longlong param_1,undefined4 param_2,undefined8 param_3,undefined8 param_4,char param_5
             )

{
  longlong *plVar1;
  undefined4 uVar2;
  int iVar3;
  longlong lVar4;
  longlong *plVar5;
  undefined8 uVar6;
  ulonglong in_stack_ffffffffffffff78;
  undefined1 local_58 [8];
  undefined1 local_50 [24];
  longlong *local_38;
  undefined1 local_30 [8];
  longlong local_28;
  
  if ((DAT_143aa84a0 != 0) && (lVar4 = FUN_142cbe730(), lVar4 != 0)) {
    uVar2 = FUN_1401a8170(param_2);
    if (*(char *)(param_1 + 4) == 'd') {
      if (DAT_143aa8518 != 0) {
        FUN_1427be040(DAT_143aa8518,local_30);
        if (local_28 != 0) {
          plVar5 = *(longlong **)(lVar4 + 0x360);
          local_38 = plVar5;
          if (plVar5 != (longlong *)0x0) {
            if (0xfffff < (ulonglong)plVar5[1]) {
              FUN_142e541f0(0x30f);
            }
            LOCK();
            plVar5[1] = plVar5[1] + 1;
            UNLOCK();
          }
          plVar1 = local_38;
          if ((plVar5 == (longlong *)0x0) ||
             (iVar3 = FUN_14019a5d0(local_38 + 4), 9999 < iVar3 - 0x195460U)) {
            if (param_5 != '\0') {
              uVar6 = FUN_1408a9e40(local_58,0xf5e);
              FUN_142a26280(uVar6,0,0,1,in_stack_ffffffffffffff78 & 0xffffffff00000000,0,0,0,0,0);
            }
          }
          else {
            if (plVar1 == (longlong *)0x0) {
              FUN_142e52ed0(0x431,0);
            }
            plVar5 = (longlong *)(**(code **)(*plVar1 + 0x80))(plVar1,local_50);
            if (((*plVar5 == *(longlong *)(param_1 + 0x10)) &&
                (lVar4 = (**(code **)(*plVar1 + 0x330))(plVar1), lVar4 != 0)) &&
               (iVar3 = FUN_1401a7080(lVar4,uVar2,param_3), iVar3 != 0)) {
              if (0xffffe < plVar1[1] - 1U) {
                FUN_142e541f0(0x31e);
              }
              LOCK();
              plVar1 = plVar1 + 1;
              lVar4 = *plVar1;
              *plVar1 = *plVar1 + -1;
              UNLOCK();
              if ((int)lVar4 == 1) {
                (**(code **)*local_38)(local_38,1);
              }
              FUN_140ce88a0(local_30);
              return 1;
            }
          }
          if (plVar1 != (longlong *)0x0) {
            if (0xffffe < plVar1[1] - 1U) {
              FUN_142e541f0(0x31e);
            }
            LOCK();
            plVar1 = plVar1 + 1;
            lVar4 = *plVar1;
            *plVar1 = *plVar1 + -1;
            UNLOCK();
            if ((int)lVar4 == 1) {
              (**(code **)*local_38)(local_38,1);
            }
          }
        }
        FUN_140ce88a0(local_30);
      }
    }
    else {
      iVar3 = FUN_1401a6ee0(lVar4,*(char *)(param_1 + 4),uVar2,param_3,param_4);
      if (iVar3 != 0) {
        return 1;
      }
    }
  }
  return 0;
}



//===========================================================
// FUN_142a8a190 @ 142a8a190   (586 bytes)
//===========================================================

void FUN_142a8a190(longlong param_1,longlong param_2,undefined4 param_3,uint param_4)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 *puVar6;
  undefined1 local_a8 [8];
  undefined1 local_a0 [8];
  undefined1 local_98 [8];
  longlong local_90;
  longlong local_88;
  undefined8 uStack_80;
  undefined1 *local_78;
  undefined1 *puStack_70;
  undefined **local_68;
  longlong local_60;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined1 *local_50;
  undefined1 *puStack_48;
  undefined ***local_30;
  
  FUN_142a8fe70();
  lVar3 = DAT_143aa84a0;
  if (DAT_143aa84a0 != 0) {
    *(uint *)(param_1 + 0x6bc) = param_4 & 0xff;
    *(undefined4 *)(param_1 + 0x4f0) = 1;
    *(undefined4 *)(param_1 + 0x410) = 1;
    if (*(char *)(param_2 + 4) == '\0') {
      lVar5 = FUN_142cbe6e0(lVar3,local_98);
      if ((*(longlong *)(param_1 + 0x3f8) - 1U < 999) || (*(longlong *)(param_1 + 0x3f8) == -1)) {
        FUN_142e52ed0(0x447);
      }
      if (param_1 + 0x3f0 == lVar5) {
        FUN_142e52d50(0x45c,1);
      }
      lVar2 = *(longlong *)(lVar5 + 8);
      if (lVar2 != 0) {
        if (0xfffff < *(ulonglong *)(lVar2 + -0x20)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(lVar2 + -0x20) = *(longlong *)(lVar2 + -0x20) + 1;
        UNLOCK();
      }
      FUN_1420a3270(param_1 + 0x3f0);
      lVar2 = local_90;
      *(undefined8 *)(param_1 + 0x3f8) = *(undefined8 *)(lVar5 + 8);
      if (local_90 != 0) {
        puVar6 = (undefined8 *)(local_90 + -0x28);
        if (0xffffe < *(longlong *)(local_90 + -0x20) - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar1 = (longlong *)(lVar2 + -0x20);
        lVar5 = *plVar1;
        *plVar1 = *plVar1 + -1;
        UNLOCK();
        if ((int)lVar5 == 1) {
          if ((local_90 != 0) && (*(longlong *)(local_90 + -0x10) != 0)) {
            LOCK();
            *(undefined8 *)(*(longlong *)(local_90 + -0x10) + 8) = 0;
            UNLOCK();
            do {
            } while (*(int *)(*(longlong *)(local_90 + -0x10) + 4) != 0);
          }
          if (puVar6 != (undefined8 *)0x0) {
            (**(code **)*puVar6)(puVar6,1);
          }
        }
      }
      lVar5 = *(longlong *)(param_1 + 0x3f8);
      if (lVar5 == 0) {
        return;
      }
      local_a0[0] = *(int *)(param_1 + 0x6bc) != 0;
      local_a8[0] = *(undefined1 *)(param_2 + 4);
      local_88 = lVar3;
      local_78 = local_a8;
      puStack_70 = local_a0;
      local_68 = &PTR_LAB_14348be18;
      uStack_80._0_4_ = (undefined4)lVar5;
      uStack_80._4_4_ = (undefined4)((ulonglong)lVar5 >> 0x20);
      local_60 = lVar3;
      uStack_58 = (undefined4)uStack_80;
      uStack_54 = uStack_80._4_4_;
      uStack_80 = lVar5;
      local_50 = local_78;
      puStack_48 = puStack_70;
    }
    else {
      if (*(char *)(param_2 + 4) != 'd') {
        return;
      }
      uVar4 = FUN_1427be040(DAT_143aa8518,&local_88);
      FUN_142a92740(param_1 + 0x400,uVar4);
      FUN_14287f2b0(&local_88);
      local_68 = &PTR_LAB_14348be48;
      local_60 = param_1 + 0x400;
    }
    local_30 = &local_68;
    FUN_142a91f30(param_1 + 0x418,param_3,*(undefined4 *)(param_1 + 0x2a8),param_2 + 8,&local_68);
    FUN_142a6e9f0(param_1,1);
  }
  return;
}


