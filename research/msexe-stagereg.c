
//===========================================================
// FUN_141127730 @ 141127730   (1846 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141127730(void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  longlong *plVar4;
  longlong *plVar5;
  longlong *plVar6;
  int *piVar7;
  longlong lVar8;
  undefined1 auStack_a8 [32];
  wchar_t *local_88 [2];
  undefined8 *local_78;
  undefined1 *local_70;
  longlong *local_68;
  wchar_t *local_60;
  longlong *local_58;
  longlong *local_50;
  longlong *local_48;
  longlong *local_40;
  undefined8 local_30;
  undefined4 local_28;
  undefined1 local_24 [4];
  ulonglong local_20;
  
  local_20 = DAT_143a8b908 ^ (ulonglong)auStack_a8;
  FUN_14090ead0(&local_60,DAT_143aca360);
  if (local_60 != (wchar_t *)0x0) {
    local_88[0] = local_60;
    (**(code **)(*(longlong *)local_60 + 8))();
    FUN_14090f200(&local_50,local_88,L"CenterPos");
    if (local_50 != (longlong *)0x0) {
      local_30 = local_60;
      if (local_60 != (wchar_t *)0x0) {
        (**(code **)(*(longlong *)local_60 + 8))();
      }
      FUN_14090f200(&local_58,&local_30,L"CameraPos");
      if (local_58 != (longlong *)0x0) {
        local_88[0] = local_60;
        if (local_60 != (wchar_t *)0x0) {
          (**(code **)(*(longlong *)local_60 + 8))();
        }
        FUN_14090f200(&local_68,local_88,L"CameraPos_svga");
        plVar6 = local_50;
        if (local_68 != (longlong *)0x0) {
          local_48 = local_50;
          if (local_50 != (longlong *)0x0) {
            (**(code **)(*local_50 + 8))(local_50);
          }
          plVar5 = local_58;
          local_40 = local_58;
          if (local_58 != (longlong *)0x0) {
            (**(code **)(*local_58 + 8))(local_58);
          }
          plVar4 = local_68;
          if (local_68 != (longlong *)0x0) {
            (**(code **)(*local_68 + 8))(local_68);
          }
          local_88[0] = (wchar_t *)0x0;
          piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,0x1c);
          piVar7[1] = 5;
          *piVar7 = -1;
          local_88[0] = (wchar_t *)(piVar7 + 4);
          piVar7[2] = 0;
          *local_88[0] = L'\0';
          *(undefined8 *)local_88[0] = u_Title_1432b1188._0_8_;
          *(wchar_t *)(piVar7 + 6) = u_Title_1432b1188[4];
          if (*piVar7 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if (piVar7[1] < 5) {
            FUN_142e54290(0x90,piVar7[1],5);
          }
          *piVar7 = 1;
          local_88[0][5] = L'\0';
          if (piVar7[1] + 1 < 6) {
            FUN_142e54290(0x9c,5);
          }
          piVar7[2] = 10;
          local_30 = (wchar_t *)CONCAT44(local_30._4_4_,1);
          FUN_141128f70(&local_48,&local_30,local_88);
          local_88[0] = (wchar_t *)0x0;
          piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,0x28);
          piVar7[1] = 0xb;
          *piVar7 = -1;
          local_88[0] = (wchar_t *)(piVar7 + 4);
          piVar7[2] = 0;
          *local_88[0] = L'\0';
          uVar3 = u_WorldSelect_1433835a8._12_4_;
          uVar2 = u_WorldSelect_1433835a8._8_4_;
          uVar1 = u_WorldSelect_1433835a8._4_4_;
          *(undefined4 *)local_88[0] = u_WorldSelect_1433835a8._0_4_;
          piVar7[5] = uVar1;
          piVar7[6] = uVar2;
          piVar7[7] = uVar3;
          piVar7[8] = u_WorldSelect_1433835a8._16_4_;
          *(wchar_t *)(piVar7 + 9) = u_WorldSelect_1433835a8[10];
          if (*piVar7 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if (piVar7[1] < 0xb) {
            FUN_142e54290(0x90,piVar7[1],0xb);
          }
          *piVar7 = 1;
          local_88[0][0xb] = L'\0';
          if (piVar7[1] + 1 < 0xc) {
            FUN_142e54290(0x9c,0xb);
          }
          piVar7[2] = 0x16;
          local_30 = (wchar_t *)CONCAT44(local_30._4_4_,2);
          FUN_141128f70(&local_48,&local_30,local_88);
          local_88[0] = (wchar_t *)0x0;
          piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,0x2a);
          piVar7[1] = 0xc;
          *piVar7 = -1;
          local_88[0] = (wchar_t *)(piVar7 + 4);
          piVar7[2] = 0;
          *local_88[0] = L'\0';
          uVar3 = u_ClassicIntro_1433835c0._12_4_;
          uVar2 = u_ClassicIntro_1433835c0._8_4_;
          uVar1 = u_ClassicIntro_1433835c0._4_4_;
          *(undefined4 *)local_88[0] = u_ClassicIntro_1433835c0._0_4_;
          piVar7[5] = uVar1;
          piVar7[6] = uVar2;
          piVar7[7] = uVar3;
          *(undefined8 *)(piVar7 + 8) = u_ClassicIntro_1433835c0._16_8_;
          if (*piVar7 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if (piVar7[1] < 0xc) {
            FUN_142e54290(0x90,piVar7[1],0xc);
          }
          *piVar7 = 1;
          local_88[0][0xc] = L'\0';
          if (piVar7[1] + 1 < 0xd) {
            FUN_142e54290(0x9c,0xc);
          }
          piVar7[2] = 0x18;
          local_30 = (wchar_t *)CONCAT44(local_30._4_4_,3);
          FUN_141128f70(&local_48,&local_30,local_88);
          local_88[0] = (wchar_t *)0x0;
          piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,0x26);
          piVar7[1] = 10;
          *piVar7 = -1;
          local_88[0] = (wchar_t *)(piVar7 + 4);
          piVar7[2] = 0;
          *local_88[0] = L'\0';
          uVar3 = u_CharSelect_1432b0f08._12_4_;
          uVar2 = u_CharSelect_1432b0f08._8_4_;
          uVar1 = u_CharSelect_1432b0f08._4_4_;
          *(undefined4 *)local_88[0] = u_CharSelect_1432b0f08._0_4_;
          piVar7[5] = uVar1;
          piVar7[6] = uVar2;
          piVar7[7] = uVar3;
          piVar7[8] = u_CharSelect_1432b0f08._16_4_;
          if (*piVar7 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if (piVar7[1] < 10) {
            FUN_142e54290(0x90,piVar7[1],10);
          }
          *piVar7 = 1;
          local_88[0][10] = L'\0';
          if (piVar7[1] + 1 < 0xb) {
            FUN_142e54290(0x9c,10);
          }
          piVar7[2] = 0x14;
          local_30 = (wchar_t *)CONCAT44(local_30._4_4_,4);
          FUN_141128f70(&local_48,&local_30,local_88);
          local_88[0] = (wchar_t *)0x0;
          piVar7 = (int *)FUN_1401bc720(&DAT_143ad6980,0x20);
          piVar7[1] = 7;
          *piVar7 = -1;
          local_88[0] = (wchar_t *)(piVar7 + 4);
          piVar7[2] = 0;
          *local_88[0] = L'\0';
          *(undefined8 *)local_88[0] = u_NewChar_1433835e0._0_8_;
          piVar7[6] = u_NewChar_1433835e0._8_4_;
          *(wchar_t *)(piVar7 + 7) = u_NewChar_1433835e0[6];
          if (*piVar7 != -1) {
            FUN_142e52dd0(0x8b);
          }
          if (piVar7[1] < 7) {
            FUN_142e54290(0x90,piVar7[1],7);
          }
          *piVar7 = 1;
          local_88[0][7] = L'\0';
          if (piVar7[1] + 1 < 8) {
            FUN_142e54290(0x9c,7);
          }
          piVar7[2] = 0xe;
          local_30._0_4_ = 5;
          FUN_141128f70(&local_48,&local_30,local_88);
          local_30 = (wchar_t *)((ulonglong)local_30._4_4_ << 0x20);
          local_88[0]._0_4_ = 1;
          local_78 = &local_30;
          local_70 = (undefined1 *)((longlong)&local_30 + 4);
          lVar8 = FUN_141128e40(&DAT_143aca3d8,local_88);
          FUN_141128dc0(lVar8 + 0x18,&local_78);
          local_30 = (wchar_t *)0x200000001;
          local_28 = 3;
          local_88[0]._0_4_ = 2;
          local_78 = &local_30;
          local_70 = local_24;
          lVar8 = FUN_141128e40(&DAT_143aca3d8,local_88);
          FUN_141128dc0(lVar8 + 0x18,&local_78);
          local_30 = (wchar_t *)0x500000004;
          local_28 = 6;
          local_88[0]._0_4_ = 3;
          local_78 = &local_30;
          local_70 = local_24;
          lVar8 = FUN_141128e40(&DAT_143aca3d8,local_88);
          FUN_141128dc0(lVar8 + 0x18,&local_78);
          local_30 = (wchar_t *)0x500000004;
          local_28 = 6;
          local_88[0]._0_4_ = 4;
          local_78 = &local_30;
          local_70 = local_24;
          lVar8 = FUN_141128e40(&DAT_143aca3d8,local_88);
          FUN_141128dc0(lVar8 + 0x18,&local_78);
          local_30 = (wchar_t *)0x500000004;
          local_28 = 6;
          local_88[0] = (wchar_t *)CONCAT44(local_88[0]._4_4_,5);
          local_78 = &local_30;
          local_70 = local_24;
          lVar8 = FUN_141128e40(&DAT_143aca3d8,local_88);
          FUN_141128dc0(lVar8 + 0x18,&local_78);
          if (plVar4 != (longlong *)0x0) {
            (**(code **)(*plVar4 + 0x10))(plVar4);
          }
          if (plVar5 != (longlong *)0x0) {
            (**(code **)(*plVar5 + 0x10))(plVar5);
          }
          if (plVar6 != (longlong *)0x0) {
            (**(code **)(*plVar6 + 0x10))(plVar6);
          }
        }
        if (local_68 != (longlong *)0x0) {
          (**(code **)(*local_68 + 0x10))(local_68);
        }
      }
      if (local_58 != (longlong *)0x0) {
        (**(code **)(*local_58 + 0x10))();
      }
    }
    if (local_50 != (longlong *)0x0) {
      (**(code **)(*local_50 + 0x10))();
    }
  }
  if (local_60 != (wchar_t *)0x0) {
    (**(code **)(*(longlong *)local_60 + 0x10))();
  }
  return;
}


