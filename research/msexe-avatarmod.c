
//===========================================================
// FUN_142d95ab0 @ 142d95ab0   (859 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142d95ab0(undefined8 param_1,undefined8 param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  int *piVar4;
  undefined8 uVar5;
  int iVar6;
  undefined8 *puVar7;
  undefined1 auStack_258 [32];
  int *local_238;
  int *local_230;
  longlong local_228 [2];
  undefined **local_218;
  undefined8 local_210;
  undefined8 local_208;
  int *local_200;
  undefined1 local_1f8;
  undefined8 local_1f7;
  undefined8 local_1ef;
  undefined8 local_1e7;
  undefined8 local_1df;
  undefined8 uStack_1d7;
  undefined8 local_1cf;
  undefined8 uStack_1c7;
  undefined8 local_1bf;
  undefined8 uStack_1b7;
  undefined8 local_1af;
  undefined8 uStack_1a7;
  undefined8 local_19f;
  undefined8 uStack_197;
  undefined8 local_18f;
  undefined8 uStack_187;
  undefined8 local_17f;
  undefined8 uStack_177;
  undefined8 local_16f;
  undefined8 uStack_167;
  undefined8 local_df;
  undefined8 uStack_d7;
  undefined8 local_cf;
  undefined8 uStack_c7;
  undefined8 local_bf;
  undefined8 uStack_b7;
  undefined8 local_af;
  undefined8 uStack_a7;
  undefined8 local_9f;
  undefined8 uStack_97;
  undefined8 local_8f;
  undefined8 uStack_87;
  undefined8 local_7f;
  undefined8 uStack_77;
  undefined8 local_6f;
  undefined8 uStack_67;
  undefined4 local_5f;
  undefined8 local_5b;
  undefined4 local_53;
  undefined1 local_4f;
  undefined8 local_4e;
  undefined4 local_46;
  undefined8 local_42;
  undefined4 local_3a;
  undefined1 local_36;
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_258;
  iVar6 = 0;
  local_200 = (int *)0x0;
  local_210 = 0;
  local_208 = 0;
  local_218 = &PTR_FUN_14327d968;
  local_4e = 0;
  local_1f8 = 0;
  local_1f7 = 0;
  local_1ef = 0;
  local_1e7 = 0;
  local_5b = 0;
  local_53 = 0;
  local_4f = 0;
  local_1df = 0;
  uStack_1d7 = 0;
  local_1cf = 0;
  uStack_1c7 = 0;
  local_1bf = 0;
  uStack_1b7 = 0;
  local_1af = 0;
  uStack_1a7 = 0;
  local_19f = 0;
  uStack_197 = 0;
  local_18f = 0;
  uStack_187 = 0;
  local_17f = 0;
  uStack_177 = 0;
  local_16f = 0;
  uStack_167 = 0;
  local_5f = 0;
  local_df = 0;
  uStack_d7 = 0;
  local_cf = 0;
  uStack_c7 = 0;
  local_bf = 0;
  uStack_b7 = 0;
  local_af = 0;
  uStack_a7 = 0;
  local_9f = 0;
  uStack_97 = 0;
  local_8f = 0;
  uStack_87 = 0;
  local_7f = 0;
  uStack_77 = 0;
  local_6f = 0;
  uStack_67 = 0;
  local_42 = 0;
  local_3a = 0;
  local_36 = 0;
  local_46 = 0xffffffff;
  local_230 = (int *)0x0;
  piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
  piVar4[1] = 0;
  *piVar4 = -1;
  local_230 = piVar4 + 4;
  piVar4[2] = 0;
  *(undefined1 *)local_230 = 0;
  if (*piVar4 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar4[1] < 0) {
    FUN_142e54290(0x90,piVar4[1],0);
  }
  *piVar4 = 1;
  *(undefined1 *)local_230 = 0;
  if (piVar4[1] + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  piVar4[2] = 0;
  FUN_1402ee8d0(&local_218,param_2,&local_230,0);
  local_228[0] = 0;
  puVar7 = &local_1df;
  do {
    uVar5 = FUN_14019ba10(local_228,"[BP:%02d] %d",iVar6,*(undefined4 *)puVar7);
    FUN_1415eca30(uVar5,0xb);
    iVar6 = iVar6 + 1;
    puVar7 = (undefined8 *)((longlong)puVar7 + 4);
  } while (iVar6 < 0x20);
  local_238 = (int *)0x0;
  piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,0x30);
  piVar4[1] = 0x1f;
  *piVar4 = -1;
  local_238 = piVar4 + 4;
  piVar4[2] = 0;
  *(undefined1 *)local_238 = 0;
  uVar3 = s_________________________________143495c18._12_4_;
  uVar2 = s_________________________________143495c18._8_4_;
  uVar1 = s_________________________________143495c18._4_4_;
  *local_238 = s_________________________________143495c18._0_4_;
  piVar4[5] = uVar1;
  piVar4[6] = uVar2;
  piVar4[7] = uVar3;
  *(undefined8 *)(piVar4 + 8) = s_________________________________143495c18._16_8_;
  piVar4[10] = s_________________________________143495c18._24_4_;
  *(undefined2 *)(piVar4 + 0xb) = s_________________________________143495c18._28_2_;
  *(char *)((longlong)piVar4 + 0x2e) = s_________________________________143495c18[0x1e];
  if (*piVar4 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar4[1] < 0x1f) {
    FUN_142e54290(0x90,piVar4[1],0x1f);
  }
  *piVar4 = 1;
  *(undefined1 *)((longlong)local_238 + 0x1f) = 0;
  if (piVar4[1] + 1 < 0x20) {
    FUN_142e54290(0x9c);
  }
  piVar4[2] = 0x1f;
  FUN_1415eca30(&local_238,0xb);
  if (local_238 != (int *)0x0) {
    FUN_14019f2c0(local_238 + -4);
  }
  if (local_228[0] != 0) {
    FUN_14019f2c0(local_228[0] + -0x10);
  }
  local_218 = &PTR_FUN_143273970;
  if (local_200 != (int *)0x0) {
    LOCK();
    local_200[2] = 0;
    local_200[3] = 0;
    UNLOCK();
    do {
    } while (local_200[1] != 0);
    if (local_200 != (int *)0x0) {
      LOCK();
      iVar6 = *local_200;
      *local_200 = *local_200 + -1;
      UNLOCK();
      if (iVar6 == 1) {
        thunk_FUN_140205820(local_200,0x10);
      }
    }
  }
  return;
}



//===========================================================
// FUN_142da4820 @ 142da4820   (1226 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Type propagation algorithm not settling */

void FUN_142da4820(longlong param_1,undefined8 param_2)

{
  longlong *plVar1;
  longlong *plVar2;
  char cVar3;
  byte bVar4;
  int iVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined8 uVar8;
  longlong lVar9;
  int *piVar10;
  undefined1 auStack_338 [32];
  undefined8 **local_318;
  undefined8 **local_310;
  undefined8 **local_308;
  undefined8 *local_300;
  undefined4 local_2f8;
  uint local_2f0;
  int *local_2e8;
  longlong local_2e0 [2];
  undefined8 *local_2d0;
  undefined8 *local_2c8;
  undefined8 *local_2c0;
  undefined8 *local_2b8;
  undefined8 local_2b0;
  undefined8 local_2a8;
  undefined8 local_2a0;
  undefined8 local_298;
  longlong local_290;
  longlong local_288;
  longlong local_280;
  longlong local_278;
  longlong local_270;
  undefined8 **local_268;
  undefined8 local_260;
  longlong *plStack_258;
  undefined8 **local_250;
  undefined8 **local_248;
  undefined8 **local_240;
  undefined **local_238 [3];
  int *local_220;
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_338;
  uVar8 = FUN_141892840();
  cVar3 = FUN_14031ee40(uVar8);
  if (cVar3 == '\0') {
    iVar5 = FUN_142cc1cf0(param_1);
    if (iVar5 == 0) {
      iVar5 = FUN_142cc1e30(param_1);
      if (iVar5 == 0) {
        lVar9 = FUN_142cbe730(param_1);
        iVar5 = FUN_1401ba9d0(lVar9 + 0x27,*(undefined4 *)(lVar9 + 0x2f));
        if (9 < iVar5) {
          if (DAT_143ac8fd8 != 0) {
            FUN_142bf3f70();
            if (DAT_143ac8fd8 != 0) {
              (*(code *)**(undefined8 **)(DAT_143ac8fd8 + 8))((undefined8 *)(DAT_143ac8fd8 + 8),1);
            }
          }
          uVar6 = FUN_1406e8c20(param_2);
          FUN_1406e9050(param_2,&local_270);
          FUN_1406e9050(param_2,&local_278);
          FUN_1406e9050(param_2,&local_280);
          FUN_1406e9050(param_2,&local_288);
          FUN_1406e9050(param_2,&local_290);
          local_260 = 0;
          plStack_258 = (longlong *)0x0;
          FUN_142db9320(param_2,&local_260);
          uVar7 = FUN_1406e8c20(param_2);
          bVar4 = FUN_1406e8ae0(param_2);
          FUN_14042c280(local_238);
          local_2e8 = (int *)0x0;
          piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
          piVar10[1] = 0;
          *piVar10 = -1;
          local_2e8 = piVar10 + 4;
          piVar10[2] = 0;
          *(undefined1 *)local_2e8 = 0;
          if (*piVar10 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if (piVar10[1] < 0) {
            FUN_142e54290(0x90,piVar10[1],0);
          }
          *piVar10 = 1;
          *(undefined1 *)local_2e8 = 0;
          if (piVar10[1] + 1 < 1) {
            FUN_142e54290(0x9c,0);
          }
          piVar10[2] = 0;
          FUN_1402ee8d0(local_238,param_2,&local_2e8,0);
          cVar3 = FUN_1406e8ae0(param_2);
          if (cVar3 == '\0') {
            iVar5 = FUN_142ced810(DAT_143aa84a0);
            if (iVar5 != 0) {
              lVar9 = FUN_14019b780(&DAT_143ad68a0,0x4b8);
              local_2e0[0] = lVar9;
              if (lVar9 != 0) {
                local_2b8 = &local_2b0;
                local_2b0 = 0;
                FUN_14019a260(&local_2b0,&local_290);
                local_2c0 = &local_2a8;
                local_2a8 = 0;
                FUN_14019a260(&local_2a8,&local_288);
                local_2c8 = &local_2a0;
                local_2a0 = 0;
                FUN_14019a260(&local_2a0,&local_280);
                local_2d0 = &local_298;
                local_298 = 0;
                FUN_14019a260(&local_298,&local_278);
                local_2e0[1] = 0;
                FUN_14019a260(local_2e0 + 1,&local_270);
                local_2f0 = (uint)bVar4;
                local_300 = &local_2b0;
                local_308 = (undefined8 **)&local_2a8;
                local_310 = (undefined8 **)&local_2a0;
                local_318 = (undefined8 **)&local_298;
                local_2f8 = uVar7;
                FUN_140fc4610(lVar9,uVar6,local_238,local_2e0 + 1);
              }
              FUN_140fc7410(DAT_143ac8fd8);
            }
          }
          local_268 = &local_2d0;
          local_2d0 = (undefined8 *)0x0;
          FUN_14019a260(&local_2d0,&local_290);
          local_250 = &local_2c8;
          local_2c8 = (undefined8 *)0x0;
          FUN_14019a260(&local_2c8,&local_288);
          local_248 = &local_2c0;
          local_2c0 = (undefined8 *)0x0;
          FUN_14019a260(&local_2c0,&local_280);
          local_240 = &local_2b8;
          local_2b8 = (undefined8 *)0x0;
          FUN_14019a260(&local_2b8,&local_278);
          local_2e0[0] = 0;
          FUN_14019a260(local_2e0,&local_270);
          local_2f8 = CONCAT31(local_2f8._1_3_,bVar4);
          local_300 = (undefined8 *)CONCAT44(local_300._4_4_,uVar7);
          local_308 = &local_2d0;
          local_310 = &local_2c8;
          local_318 = &local_2c0;
          FUN_142da4cf0(param_1,&local_260,local_2e0,&local_2b8);
          uVar6 = (*DAT_143262db0)();
          *(undefined4 *)(param_1 + 0x3268) = uVar6;
          *(undefined4 *)(param_1 + 0x3264) = 1;
          local_238[0] = &PTR_FUN_143273970;
          if (local_220 != (int *)0x0) {
            LOCK();
            local_220[2] = 0;
            local_220[3] = 0;
            UNLOCK();
            do {
            } while (local_220[1] != 0);
            if (local_220 != (int *)0x0) {
              LOCK();
              iVar5 = *local_220;
              *local_220 = *local_220 + -1;
              UNLOCK();
              if (iVar5 == 1) {
                thunk_FUN_140205820(local_220,0x10);
              }
              local_220 = (int *)0x0;
            }
          }
          plVar2 = plStack_258;
          if (plStack_258 != (longlong *)0x0) {
            LOCK();
            plVar1 = plStack_258 + 1;
            lVar9 = *plVar1;
            *(int *)plVar1 = (int)*plVar1 + -1;
            UNLOCK();
            if ((int)lVar9 == 1) {
              (**(code **)*plStack_258)(plStack_258);
              LOCK();
              piVar10 = (int *)((longlong)plVar2 + 0xc);
              iVar5 = *piVar10;
              *piVar10 = *piVar10 + -1;
              UNLOCK();
              if (iVar5 == 1) {
                (**(code **)(*plVar2 + 8))(plVar2);
              }
            }
          }
          if (local_290 != 0) {
            FUN_14019f2c0(local_290 + -0x10);
          }
          if (local_288 != 0) {
            FUN_14019f2c0(local_288 + -0x10);
          }
          if (local_280 != 0) {
            FUN_14019f2c0(local_280 + -0x10);
          }
          if (local_278 != 0) {
            FUN_14019f2c0(local_278 + -0x10);
          }
          if (local_270 != 0) {
            FUN_14019f2c0(local_270 + -0x10);
          }
        }
      }
    }
  }
  return;
}



//===========================================================
// FUN_142d012e0 @ 142d012e0   (569 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142d012e0(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  undefined4 uVar2;
  longlong lVar3;
  int *piVar4;
  undefined1 auStack_248 [32];
  int *local_228 [2];
  undefined **local_218;
  undefined8 local_210;
  undefined8 local_208;
  int *local_200;
  undefined1 local_1f8;
  undefined8 local_1f7;
  undefined8 local_1ef;
  undefined8 local_1e7;
  undefined8 local_1df;
  undefined8 uStack_1d7;
  undefined8 local_1cf;
  undefined8 uStack_1c7;
  undefined8 local_1bf;
  undefined8 uStack_1b7;
  undefined8 local_1af;
  undefined8 uStack_1a7;
  undefined8 local_19f;
  undefined8 uStack_197;
  undefined8 local_18f;
  undefined8 uStack_187;
  undefined8 local_17f;
  undefined8 uStack_177;
  undefined8 local_16f;
  undefined8 uStack_167;
  undefined8 local_df;
  undefined8 uStack_d7;
  undefined8 local_cf;
  undefined8 uStack_c7;
  undefined8 local_bf;
  undefined8 uStack_b7;
  undefined8 local_af;
  undefined8 uStack_a7;
  undefined8 local_9f;
  undefined8 uStack_97;
  undefined8 local_8f;
  undefined8 uStack_87;
  undefined8 local_7f;
  undefined8 uStack_77;
  undefined8 local_6f;
  undefined8 uStack_67;
  undefined4 local_5f;
  undefined8 local_5b;
  undefined4 local_53;
  undefined1 local_4f;
  undefined8 local_4e;
  undefined4 local_46;
  undefined8 local_42;
  undefined4 local_3a;
  undefined1 local_36;
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_248;
  uVar2 = FUN_1406e8c20(param_2);
  lVar3 = FUN_1429b6c90(DAT_143ac1b90,uVar2);
  if (lVar3 != 0) {
    local_200 = (int *)0x0;
    local_210 = 0;
    local_208 = 0;
    local_218 = &PTR_FUN_14327d968;
    local_4e = 0;
    local_1f8 = 0;
    local_1f7 = 0;
    local_1ef = 0;
    local_1e7 = 0;
    local_5b = 0;
    local_53 = 0;
    local_4f = 0;
    local_1df = 0;
    uStack_1d7 = 0;
    local_1cf = 0;
    uStack_1c7 = 0;
    local_1bf = 0;
    uStack_1b7 = 0;
    local_1af = 0;
    uStack_1a7 = 0;
    local_19f = 0;
    uStack_197 = 0;
    local_18f = 0;
    uStack_187 = 0;
    local_17f = 0;
    uStack_177 = 0;
    local_16f = 0;
    uStack_167 = 0;
    local_5f = 0;
    local_df = 0;
    uStack_d7 = 0;
    local_cf = 0;
    uStack_c7 = 0;
    local_bf = 0;
    uStack_b7 = 0;
    local_af = 0;
    uStack_a7 = 0;
    local_9f = 0;
    uStack_97 = 0;
    local_8f = 0;
    uStack_87 = 0;
    local_7f = 0;
    uStack_77 = 0;
    local_6f = 0;
    uStack_67 = 0;
    local_42 = 0;
    local_3a = 0;
    local_36 = 0;
    local_46 = 0xffffffff;
    local_228[0] = (int *)0x0;
    piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,0x11);
    piVar4[1] = 0;
    *piVar4 = -1;
    local_228[0] = piVar4 + 4;
    piVar4[2] = 0;
    *(undefined1 *)local_228[0] = 0;
    if (*piVar4 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if (piVar4[1] < 0) {
      FUN_142e54290(0x90,piVar4[1],0);
    }
    *piVar4 = 1;
    *(undefined1 *)local_228[0] = 0;
    if (piVar4[1] + 1 < 1) {
      FUN_142e54290(0x9c,0);
    }
    piVar4[2] = 0;
    FUN_1402ee8d0(&local_218,param_2,local_228,0);
    FUN_142797be0(lVar3,&local_218);
    local_218 = &PTR_FUN_143273970;
    if (local_200 != (int *)0x0) {
      LOCK();
      local_200[2] = 0;
      local_200[3] = 0;
      UNLOCK();
      do {
      } while (local_200[1] != 0);
      if (local_200 != (int *)0x0) {
        LOCK();
        iVar1 = *local_200;
        *local_200 = *local_200 + -1;
        UNLOCK();
        if (iVar1 == 1) {
          thunk_FUN_140205820(local_200,0x10);
        }
      }
    }
  }
  return;
}


