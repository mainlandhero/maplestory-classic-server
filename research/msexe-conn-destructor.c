
//===========================================================
// FUN_142c42f30 @ 142c42f30   (2225 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x000142c43215) */

undefined4
FUN_142c42f30(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined4 param_4)

{
  char cVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  undefined8 *puVar4;
  undefined4 *puVar5;
  undefined8 local_res18;
  undefined4 local_res20;
  undefined1 auStack_f08 [32];
  undefined4 local_ee8;
  char local_eb8;
  char local_eb7;
  undefined4 local_eb4;
  int local_ea8;
  int local_ea0;
  int local_e88;
  undefined4 local_e84;
  undefined4 local_e80 [2];
  int local_e78;
  int local_e74;
  undefined4 local_e64;
  undefined4 local_e60;
  int local_e5c;
  int local_e58;
  undefined1 local_e08 [8];
  undefined1 local_e00 [8];
  undefined1 local_df8 [8];
  undefined8 local_df0;
  undefined8 local_de8;
  code *local_de0;
  longlong local_d78;
  code *local_d70;
  undefined1 *local_9f0;
  undefined8 local_9e8;
  undefined1 *local_9e0;
  undefined8 local_9d8;
  undefined8 local_9d0;
  undefined1 *local_9c8;
  undefined8 local_9c0;
  undefined8 local_9b8;
  undefined1 *local_9b0;
  undefined8 local_9a8;
  undefined8 local_9a0;
  undefined1 *local_998;
  undefined8 local_990;
  undefined8 local_988;
  undefined8 local_980;
  undefined8 local_978;
  undefined8 local_970;
  undefined1 *local_968;
  undefined8 local_960;
  undefined8 local_958;
  undefined8 local_950;
  undefined8 local_948;
  undefined1 local_940 [8];
  undefined8 local_938;
  undefined1 local_850 [8];
  undefined1 local_848 [8];
  undefined1 local_840 [24];
  undefined8 local_828;
  undefined1 local_820 [264];
  undefined1 local_718 [48];
  undefined1 local_6e8 [48];
  undefined1 local_6b8 [48];
  undefined1 local_688 [64];
  undefined1 local_648 [64];
  undefined1 local_608 [64];
  undefined1 local_5c8 [208];
  undefined1 local_4f8 [1248];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_f08;
  local_res18 = param_3;
  local_res20 = param_4;
  DAT_143addd60 = (*DAT_143262db0)();
  DAT_143addd64 = (*DAT_143262db0)();
  FUN_142c50b90(0);
  FUN_142e138f0(&local_res18);
  DAT_143addd58 = param_1;
  uVar2 = (*DAT_143262db0)();
  FUN_141b0ebe0(uVar2);
  FUN_140194910();
  FUN_140c903d0();
  FUN_142e14950();
  FUN_140c91850();
  FUN_141a82d10();
  FUN_142e5bb40();
  FUN_142e5c4c0();
  FUN_140c93200();
  FUN_140c93370();
  thunk_FUN_142e1a0c0(0);
  thunk_FUN_142e1a0c0(1);
  thunk_FUN_142e1a0c0(9);
  FUN_142e0eec0(0);
  FUN_142e5c4d0();
  uVar3 = FUN_142c4ad20();
  FUN_142c43ce0(uVar3,0);
  FUN_142c43cd0(FUN_142c4adc0);
  FUN_142c43d10(1);
  FUN_142c43d20(1);
  cVar1 = FUN_140c9b240();
  if (cVar1 == '\0') {
    local_e84 = 0;
  }
  else {
    cVar1 = FUN_142e149f0();
    if (cVar1 == '\0') {
      local_e84 = 0;
    }
    else {
      FUN_142e137f0();
      cVar1 = FUN_142e14a00();
      if (cVar1 == '\0') {
        local_e84 = 0;
      }
      else {
        local_ea0 = 0;
        local_df0 = local_res18;
        puVar4 = (undefined8 *)FUN_142f2a138();
        local_de8 = *puVar4;
        puVar5 = (undefined4 *)FUN_142f2a130();
        local_e60 = *puVar5;
        FUN_142c926e0(local_5c8,local_e60,local_de8,local_df0);
        local_eb8 = FUN_142e14bc0(local_5c8);
        FUN_142e13bd0();
        FUN_140c91850();
        FUN_142c43970();
        FUN_142c43930();
        local_828 = FUN_142c43db0(local_4f8,local_5c8);
        FUN_142c4f6c0(local_4f8,local_eb8);
        local_e5c = FUN_142c4a810(local_4f8);
        local_e58 = local_e5c;
        if ((local_e5c == 5) && (local_eb7 = FUN_140d90d70(), local_eb7 == '\0')) {
          FUN_141804a70(local_820,0x23000001);
                    /* WARNING: Subroutine does not return */
          _CxxThrowException(local_820,(ThrowInfo *)&DAT_143a3bf90);
        }
        if (DAT_143ac7ed4 == '\0') {
          local_e84 = 0;
          FUN_142c44350(local_4f8);
          do {
            local_de0 = DAT_143ad58b0;
            local_ee8 = 1;
            local_e88 = (*DAT_143ad58b0)(local_718,0,0,0);
          } while (local_e88 != 0);
          FUN_142c43a00(local_5c8);
        }
        else {
          FUN_142c446a0(local_4f8);
          local_eb4 = 0;
          local_e80[0] = 0;
          FUN_142c45e50(local_4f8,local_6b8,local_e80);
          local_e78 = FUN_142c4a990(local_4f8);
          if ((local_e78 == 0) || (local_d78 = FUN_1415e3cc0(), *(int *)(local_d78 + 0x220) == 0)) {
            local_ea8 = 0;
          }
          else {
            local_ea8 = 1;
          }
          local_ea0 = local_ea8;
          FUN_142c44350(local_4f8);
          do {
            local_d70 = DAT_143ad58b0;
            local_ee8 = 1;
            local_e74 = (*DAT_143ad58b0)(local_6e8,0,0,0);
          } while (local_e74 != 0);
          (*DAT_143262108)(0);
          local_9f0 = local_688;
          local_9e8 = FUN_142c439b0(local_9f0,0);
          FUN_142c43b20(local_9e8);
          local_9e0 = local_648;
          local_9d8 = FUN_142c439b0(local_9e0,0);
          FUN_142c43c40(local_9d8);
          FUN_142c4b730();
          if ((local_eb8 != '\0') && (cVar1 = FUN_14057e4c0(), cVar1 != '\0')) {
            local_9d0 = FUN_1404c6160();
            FUN_140196ed0(local_e08,"GC:UnInitializePCOM",0xffffffff);
            FUN_1404c7800(local_9d0,0x16,local_e08);
            FUN_140199470(local_e08);
          }
          FUN_142e137f0();
          if (local_ea0 != 0) {
            local_970 = FUN_140ca2660();
            local_9c8 = local_850;
            local_9c0 = FUN_140196ed0(local_9c8,&DAT_1434b2af1,0xffffffff);
            local_9b0 = local_848;
            local_9b8 = local_9c0;
            local_988 = local_9c0;
            local_9a8 = FUN_140196ed0(local_9b0,&DAT_1434b2af1,0xffffffff);
            local_998 = local_840;
            local_9a0 = local_9a8;
            local_980 = local_9a8;
            local_990 = FUN_1408a9e40(local_998,0x545);
            local_978 = local_990;
            FUN_141d5c830(local_970,local_990,local_980,local_988);
          }
          FUN_140caa4f0();
          local_968 = local_608;
          local_960 = FUN_142c439b0(local_968,0);
          FUN_142c43bb0(local_960);
          if ((local_eb8 != '\0') && (cVar1 = FUN_14057e4c0(), cVar1 != '\0')) {
            local_958 = FUN_1404c6160();
            FUN_140196ed0(local_e00,"GC:DestroySecurityClient",0xffffffff);
            FUN_1404c7800(local_958,0x17,local_e00);
            FUN_140199470(local_e00);
          }
          cVar1 = FUN_140d90de0();
          if (cVar1 != '\0') {
            FUN_140d90db0();
          }
          local_950 = FUN_1408a9e40(local_940,0xb24);
          local_948 = local_950;
          uVar3 = FUN_140c2f770(local_950);
          FUN_1429e4dd0(uVar3);
          FUN_140199470(local_940);
          thunk_FUN_142bf0700();
          (*DAT_143263030)();
          if (local_eb8 != '\0') {
            cVar1 = FUN_14057e4c0();
            if (cVar1 != '\0') {
              local_938 = FUN_1404c6160();
              FUN_140196ed0(local_df8,"GC:GameEnd",0xffffffff);
              FUN_1404c7800(local_938,99,local_df8);
              FUN_140199470(local_df8);
            }
            cVar1 = FUN_14057e4c0();
            if (cVar1 != '\0') {
              uVar3 = FUN_1404c6160();
              FUN_1406e74e0(uVar3);
            }
          }
          local_e64 = 0;
          FUN_142c43a00(local_5c8);
          local_e84 = local_e64;
        }
      }
    }
  }
  return local_e84;
}


