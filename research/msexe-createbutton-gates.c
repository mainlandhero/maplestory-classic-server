
//===========================================================
// FUN_140c9e3f0 @ 140c9e3f0   (1190 bytes)
//===========================================================

/* WARNING: Heritage AFTER dead removal. Example location: s0xffffffffffffff70 : 0x000140c9e7eb */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

byte FUN_140c9e3f0(void)

{
  longlong lVar1;
  undefined1 *puVar2;
  undefined1 *puVar3;
  byte local_a8;
  byte local_a7;
  byte local_a6 [6];
  int *local_a0;
  ushort local_98 [2];
  undefined1 local_94;
  undefined1 local_93;
  undefined1 local_92;
  byte local_90 [8];
  int local_88;
  int local_84;
  int local_80;
  int local_7c;
  undefined4 local_78 [2];
  byte *local_70;
  undefined1 *local_68;
  byte *local_60;
  byte *local_58;
  longlong local_50;
  longlong local_48;
  longlong local_40;
  ulonglong local_38;
  undefined8 local_30;
  undefined1 *local_28;
  byte *local_20;
  
  local_a0 = &DAT_143ac8168;
  local_a6[0] = *DAT_143ac8170;
  local_a8 = DAT_143ac8170[1];
  local_58 = local_a6;
  local_70 = DAT_143ac8170;
  local_98[0] = 0x9a65;
  local_88 = 0;
  while (local_88 == 0) {
    if (local_a8 == 0) {
      local_a8 = 0x2a;
    }
    local_a8 = local_a8 + 0x2a + *DAT_143ac8170;
    local_98[0] = local_98[0] << 3 | (short)((int)(uint)local_98[0] >> 0xd) + (ushort)local_a8;
    local_88 = 1;
  }
  FUN_140c78f50(0x171,0x21a3f1a4a8af17);
  if (((local_98[0] != *(ushort *)(*(longlong *)(local_a0 + 2) + 4)) ||
      ((char)local_a0[1] != *(char *)(*(longlong *)(local_a0 + 2) + 2))) ||
     ((char)local_a0[4] != *(char *)(*(longlong *)(local_a0 + 2) + 3))) {
    local_50 = *(longlong *)(local_a0 + 2) + 3;
    local_94 = (undefined1)local_a0[4];
    local_48 = *(longlong *)(local_a0 + 2) + 2;
    local_93 = (undefined1)local_a0[1];
    local_40 = *(longlong *)(local_a0 + 2) + 4;
    local_38 = (ulonglong)local_a6[0];
    local_78[0] = 2;
    FUN_140197dd0(&DAT_143271f04,0x17c,5,local_78,&local_38,&local_a8,local_98,local_40,&local_93,
                  local_48,&local_94,local_50);
  }
  local_80 = *local_a0 + 1;
  *local_a0 = local_80;
  if (local_80 % 0x37 == 0) {
    local_90[0] = local_a6[0];
    local_7c = *local_a0 + 1;
    *local_a0 = local_7c;
    if (local_7c % 0x6f == 0) {
      local_68 = *(undefined1 **)(local_a0 + 2);
      local_30 = FUN_14019a150(6);
      *(undefined8 *)(local_a0 + 2) = local_30;
      puVar2 = local_68;
      puVar3 = *(undefined1 **)(local_a0 + 2);
      for (lVar1 = 6; lVar1 != 0; lVar1 = lVar1 + -1) {
        *puVar3 = *puVar2;
        puVar2 = puVar2 + 1;
        puVar3 = puVar3 + 1;
      }
      local_28 = local_68;
      thunk_FUN_140205820(local_68,6);
    }
    local_92 = FUN_142f04924();
    *(undefined1 *)(*(longlong *)(local_a0 + 2) + 1) = local_92;
    local_a7 = *(byte *)(*(longlong *)(local_a0 + 2) + 1);
    local_20 = local_90;
    local_60 = *(byte **)(local_a0 + 2);
    *(undefined2 *)(*(longlong *)(local_a0 + 2) + 4) = 0x9a65;
    local_84 = 0;
    while (local_84 == 0) {
      if (local_a7 == 0) {
        local_a7 = 0x2a;
      }
      *local_60 = local_90[0] ^ local_a7;
      local_a7 = local_a7 + 0x2a + *local_60;
      *(ushort *)(*(longlong *)(local_a0 + 2) + 4) =
           *(short *)(*(longlong *)(local_a0 + 2) + 4) << 3 |
           (short)((int)(uint)*(ushort *)(*(longlong *)(local_a0 + 2) + 4) >> 0xd) +
           (ushort)local_a7;
      local_84 = 1;
    }
  }
  FUN_140c79130(0x182,local_a0 + 0xc0a84bdc4a);
  return local_a6[0];
}



//===========================================================
// FUN_141b3faf0 @ 141b3faf0   (11 bytes)
//===========================================================

bool FUN_141b3faf0(longlong param_1)

{
  return *(int *)(param_1 + 0x238) != 0;
}



//===========================================================
// FUN_14108c8e0 @ 14108c8e0   (63 bytes)
//===========================================================

undefined4 * FUN_14108c8e0(int param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  
  if (-1 < param_1) {
    if ((ulonglong)(longlong)param_1 < (ulonglong)(DAT_143ac9858 - DAT_143ac9850 >> 4)) {
      puVar1 = *(undefined4 **)(DAT_143ac9850 + (longlong)param_1 * 0x10);
      puVar2 = &DAT_143ac98a0;
      if (puVar1 != (undefined4 *)0x0) {
        puVar2 = puVar1;
      }
      return puVar2;
    }
  }
  return &DAT_143ac98a0;
}



//===========================================================
// FUN_141179410 @ 141179410   (190 bytes)
//===========================================================

void FUN_141179410(longlong param_1)

{
  undefined8 uVar1;
  int *piVar2;
  int *local_res8;
  
  uVar1 = *(undefined8 *)(param_1 + 0x248);
  local_res8 = (int *)0x0;
  piVar2 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
  piVar2[1] = 0;
  local_res8 = piVar2 + 4;
  *piVar2 = -1;
  piVar2[2] = 0;
  *(undefined1 *)local_res8 = 0;
  if (*piVar2 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar2[1] < 0) {
    FUN_142e54290(0x90,piVar2[1],0);
  }
  *piVar2 = 1;
  *(undefined1 *)local_res8 = 0;
  if (piVar2[1] + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  piVar2[2] = 0;
  FUN_141b28570(uVar1,&local_res8,0);
  return;
}


