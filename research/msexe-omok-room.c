// Omok room and miniroom balloon - Ghidra decompile of client-patched/MapleStory.exe, 2026-10-04.
// research/omok-room-2026-10-04.md is the reading of it. Field ORDER from the listing, meaning from here.
// Functions: FUN_1427956b0 (0x0233 balloon), FUN_141594970 (balloon draw), FUN_1428b7600 (click on a
// balloon -> mode 3), FUN_141c3fcb0 (mode 8 chat send), FUN_142d1cf20, then the Omok class and the
// shared miniroom handlers.


//===========================================================
// FUN_141594970 @ 141594970   (5337 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Removing unreachable block (ram,0x000141595abc) */
/* WARNING: Removing unreachable block (ram,0x000141595dc5) */

void FUN_141594970(longlong param_1,int *param_2,longlong *param_3)

{
  undefined2 *puVar1;
  IUnknown *pIVar2;
  IUnknown *pIVar3;
  code *pcVar4;
  longlong lVar5;
  longlong lVar6;
  int iVar7;
  int iVar8;
  undefined4 uVar9;
  longlong *plVar10;
  undefined8 *puVar11;
  int *piVar12;
  undefined8 uVar13;
  undefined4 *puVar14;
  longlong *plVar15;
  undefined4 *puVar16;
  int iVar17;
  longlong lVar18;
  uint uVar19;
  undefined8 uVar20;
  IUnknown *pIVar21;
  undefined1 *puVar22;
  IUnknown *pIVar23;
  ulonglong uVar24;
  ulonglong uVar25;
  uint uVar26;
  wchar_t *pwVar27;
  undefined *puStack_170;
  undefined1 auStack_168 [32];
  longlong local_148;
  undefined4 local_140 [2];
  undefined8 local_138 [2];
  undefined8 local_128;
  IUnknown *local_120;
  longlong *local_118;
  undefined8 local_110;
  int local_108 [2];
  undefined8 local_100;
  IUnknown *local_f8;
  IUnknown *local_f0;
  longlong *local_e8;
  IUnknown *local_e0;
  int local_d8 [2];
  undefined8 local_d0;
  IUnknown *local_c8;
  longlong *local_c0;
  IUnknown *local_b8;
  IUnknown *local_b0;
  undefined4 local_a8;
  undefined4 uStack_a4;
  undefined8 uStack_a0;
  undefined8 local_98;
  IUnknown *local_90;
  IUnknown *local_88;
  longlong local_80;
  IUnknown *local_78;
  IUnknown *local_70;
  uint local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  IUnknown *local_48;
  IUnknown *local_40;
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)&local_128;
  local_110 = (IUnknown *)((ulonglong)local_110 & 0xffffffff00000000);
  puStack_170 = (undefined *)0x1415949d7;
  local_80 = param_1;
  FUN_14090ead0(&local_f8,DAT_143ace048);
  local_90 = local_f8;
  if (local_f8 != (IUnknown *)0x0) {
    puStack_170 = (undefined *)0x1415949ee;
    (**(code **)(*(longlong *)local_f8 + 8))();
  }
  puStack_170 = (undefined *)0x141594a06;
  FUN_14090f750(&local_c8,&local_90,L"backgrnd");
  pIVar2 = local_c8;
  if (local_c8 == (IUnknown *)0x0) {
LAB_141595dc0:
    if (local_f8 != (IUnknown *)0x0) {
      puStack_170 = (undefined *)0x141595dde;
      (**(code **)(*(longlong *)local_f8 + 0x10))();
    }
  }
  else {
    local_108[0] = 0;
    puStack_170 = (undefined *)0x141594a28;
    iVar7 = (**(code **)(*(longlong *)local_c8 + 0xa0))(local_c8,local_108);
    if (iVar7 < 0) {
      puStack_170 = (undefined *)0x141594a3d;
      _com_issue_errorex(iVar7,pIVar2,(_GUID *)&DAT_14327ac98);
    }
    pIVar2 = local_c8;
    *(int *)(param_1 + 0x88) = -local_108[0];
    if (local_c8 == (IUnknown *)0x0) goto LAB_141595dc0;
    local_d8[0] = 0;
    puStack_170 = (undefined *)0x141594a69;
    iVar7 = (**(code **)(*(longlong *)local_c8 + 0xf8))(local_c8,local_d8);
    if (iVar7 < 0) {
      puStack_170 = (undefined *)0x141594a7e;
      _com_issue_errorex(iVar7,pIVar2,(_GUID *)&DAT_14327ac98);
    }
    pIVar2 = local_c8;
    if (local_c8 == (IUnknown *)0x0) {
LAB_141595e24:
                    /* WARNING: Subroutine does not return */
      puStack_170 = (undefined *)0x141595e2e;
      FUN_142ef3ac0(0x80004003);
    }
    local_108[0] = 0;
    puStack_170 = (undefined *)0x141594a9f;
    iVar7 = (**(code **)(*(longlong *)local_c8 + 0x108))(local_c8,local_108);
    if (iVar7 < 0) {
      puStack_170 = (undefined *)0x141594ab4;
      _com_issue_errorex(iVar7,pIVar2,(_GUID *)&DAT_14327ac98);
    }
    pIVar2 = local_c8;
    if (local_c8 == (IUnknown *)0x0) goto LAB_141595e24;
    local_110 = (IUnknown *)((ulonglong)local_110 & 0xffffffff00000000);
    puStack_170 = (undefined *)0x141594ad5;
    iVar7 = (**(code **)(*(longlong *)local_c8 + 0x98))(local_c8,&local_110);
    if (iVar7 < 0) {
      puStack_170 = (undefined *)0x141594aea;
      _com_issue_errorex(iVar7,pIVar2,(_GUID *)&DAT_14327ac98);
    }
    pIVar2 = local_c8;
    iVar7 = local_108[0];
    *(int *)(param_1 + 0x84) = -((int)local_110 / 2);
    *(int *)(param_1 + 0x88) = *(int *)(param_1 + 0x88) - local_108[0];
    lVar18 = *param_3;
    if (local_c8 == (IUnknown *)0x0) {
LAB_141595e42:
                    /* WARNING: Subroutine does not return */
      puStack_170 = &UNK_141595e4c;
      FUN_142ef3ac0(0x80004003);
    }
    local_110 = (IUnknown *)((ulonglong)local_110 & 0xffffffff00000000);
    puStack_170 = (undefined *)0x141594b2a;
    iVar8 = (**(code **)(*(longlong *)local_c8 + 0xa0))(local_c8,&local_110);
    if (iVar8 < 0) {
      puStack_170 = (undefined *)0x141594b3f;
      _com_issue_errorex(iVar8,pIVar2,(_GUID *)&DAT_14327ac98);
    }
    pIVar2 = local_c8;
    if (local_c8 == (IUnknown *)0x0) goto LAB_141595e42;
    local_100 = (IUnknown *)((ulonglong)local_100._4_4_ << 0x20);
    puStack_170 = (undefined *)0x141594b60;
    iVar8 = (**(code **)(*(longlong *)local_c8 + 0x98))(local_c8,&local_100);
    if (iVar8 < 0) {
      puStack_170 = (undefined *)0x141594b75;
      _com_issue_errorex(iVar8,pIVar2,(_GUID *)&DAT_14327ac98);
    }
    local_140[0] = 0xc00613dc;
    local_148 = 0;
    puStack_170 = (undefined *)0x141594b95;
    plVar10 = (longlong *)
              FUN_140dcd6c0(&local_120,(ulonglong)local_100 & 0xffffffff,
                            (ulonglong)local_110 & 0xffffffff,lVar18);
    plVar15 = *(longlong **)(param_1 + 0x18);
    if (plVar15 != (longlong *)*plVar10) {
      *(longlong **)(param_1 + 0x18) = (longlong *)*plVar10;
      *plVar10 = 0;
      if (plVar15 != (longlong *)0x0) {
        puStack_170 = (undefined *)0x141594bb4;
        (**(code **)(*plVar15 + 0x10))();
      }
    }
    if (local_120 != (IUnknown *)0x0) {
      puStack_170 = (undefined *)0x141594bc4;
      (**(code **)(*(longlong *)local_120 + 0x10))();
    }
    pIVar2 = *(IUnknown **)(param_1 + 0x18);
    if (pIVar2 == (IUnknown *)0x0) {
      if (local_c8 != (IUnknown *)0x0) {
        puStack_170 = (undefined *)0x141594bdd;
        (**(code **)(*(longlong *)local_c8 + 0x10))();
      }
      if (local_f8 != (IUnknown *)0x0) {
        puStack_170 = (undefined *)0x141594bed;
        (**(code **)(*(longlong *)local_f8 + 0x10))();
      }
    }
    else {
      local_a8 = CONCAT22(local_a8._2_2_,0x16);
      uStack_5c = (undefined4)(uStack_a0 >> 0x20);
      uStack_a0 = uStack_a0 & 0xffffffff00000000;
      local_90 = (IUnknown *)0x0;
      local_68 = local_a8;
      uStack_64 = uStack_a4;
      uStack_60 = 0;
      local_58 = local_98;
      puStack_170 = (undefined *)0x141594c45;
      iVar8 = (**(code **)(*(longlong *)pIVar2 + 0x240))(pIVar2,&local_68,&local_90);
      if (iVar8 < 0) {
        puStack_170 = (undefined *)0x141594c5a;
        _com_issue_errorex(iVar8,pIVar2,(_GUID *)&DAT_14327fcb0);
      }
      pIVar2 = local_90;
      local_48 = local_90;
      local_110 = (IUnknown *)CONCAT44(local_110._4_4_,0x10);
      if ((short)local_a8 == 8) {
        local_a8 = local_a8 & 0xffff0000;
        if (uStack_a0 != 0) {
          puStack_170 = (undefined *)0x141594c98;
          (*DAT_143ad5990)(uStack_a0 - 4);
        }
      }
      else {
        puStack_170 = (undefined *)0x141594ca7;
        (*DAT_143262a18)(&local_a8);
      }
      pIVar21 = (IUnknown *)0x0;
      if (pIVar2 != (IUnknown *)0x0) {
        local_b0 = local_c8;
        if (local_c8 != (IUnknown *)0x0) {
          puStack_170 = (undefined *)0x141594ce6;
          (**(code **)(*(longlong *)local_c8 + 8))();
        }
        local_128 = pIVar2;
        puStack_170 = (undefined *)0x141594cf4;
        (**(code **)(*(longlong *)pIVar2 + 8))(pIVar2);
        local_148 = CONCAT44(local_148._4_4_,0xff);
        puStack_170 = (undefined *)0x141594d10;
        FUN_142aa1590(&local_128,&local_b0,0,0);
        local_b0 = (IUnknown *)0x0;
        pIVar23 = pIVar21;
        if (*param_2 == 3) {
          local_128 = local_f8;
          if (local_f8 != (IUnknown *)0x0) {
            puStack_170 = (undefined *)0x141594e94;
            (**(code **)(*(longlong *)local_f8 + 8))();
          }
          puStack_170 = (undefined *)0x141594ea9;
          puVar11 = (undefined8 *)FUN_14090f750(&local_120,&local_128,L"icon_omok");
          pIVar3 = (IUnknown *)*puVar11;
          if (pIVar3 != (IUnknown *)0x0) {
            *puVar11 = 0;
            pIVar23 = pIVar3;
            local_b0 = pIVar3;
          }
          if (local_120 != (IUnknown *)0x0) {
            puStack_170 = (undefined *)0x141594ecb;
            (**(code **)(*(longlong *)local_120 + 0x10))();
          }
LAB_141594ecc:
          if (pIVar23 != (IUnknown *)0x0) {
            puStack_170 = (undefined *)0x141594ede;
            local_128 = pIVar23;
            (**(code **)(*(longlong *)pIVar23 + 8))(pIVar23);
            local_100 = pIVar2;
            puStack_170 = (undefined *)0x141594eec;
            (**(code **)(*(longlong *)pIVar2 + 8))(pIVar2);
            local_148 = CONCAT44(local_148._4_4_,0xff);
            puStack_170 = (undefined *)0x141594f09;
            FUN_142aa16b0(&local_100,&local_128,local_d8[0],iVar7);
          }
        }
        else {
          if (*param_2 != 4) {
            local_128 = local_f8;
            if (local_f8 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x141594d3c;
              (**(code **)(*(longlong *)local_f8 + 8))();
            }
            puStack_170 = (undefined *)0x141594d51;
            puVar11 = (undefined8 *)FUN_14090f750(&local_120,&local_128,L"icon");
            pIVar3 = (IUnknown *)*puVar11;
            if (pIVar3 != (IUnknown *)0x0) {
              *puVar11 = 0;
              pIVar23 = pIVar3;
              local_b0 = pIVar3;
            }
            if (local_120 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x141594d73;
              (**(code **)(*(longlong *)local_120 + 0x10))();
            }
            goto LAB_141594ecc;
          }
          iVar8 = param_2[9];
          if (iVar8 == 0) {
            local_128 = local_f8;
            if (local_f8 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x141594e47;
              (**(code **)(*(longlong *)local_f8 + 8))();
            }
            puStack_170 = (undefined *)0x141594e5c;
            puVar11 = (undefined8 *)FUN_14090f750(&local_120,&local_128,L"icon_card0");
            pIVar3 = (IUnknown *)*puVar11;
            if (pIVar3 != (IUnknown *)0x0) {
              *puVar11 = 0;
              pIVar23 = pIVar3;
              local_b0 = pIVar3;
            }
            if (local_120 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x141594e7e;
              (**(code **)(*(longlong *)local_120 + 0x10))();
            }
            goto LAB_141594ecc;
          }
          if (iVar8 == 1) {
            local_128 = local_f8;
            if (local_f8 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x141594df7;
              (**(code **)(*(longlong *)local_f8 + 8))();
            }
            puStack_170 = (undefined *)0x141594e0c;
            puVar11 = (undefined8 *)FUN_14090f750(&local_120,&local_128,L"icon_card1");
            pIVar3 = (IUnknown *)*puVar11;
            if (pIVar3 != (IUnknown *)0x0) {
              *puVar11 = 0;
              pIVar23 = pIVar3;
              local_b0 = pIVar3;
            }
            if (local_120 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x141594e2e;
              (**(code **)(*(longlong *)local_120 + 0x10))();
            }
            goto LAB_141594ecc;
          }
          if (iVar8 == 2) {
            local_128 = local_f8;
            if (local_f8 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x141594da7;
              (**(code **)(*(longlong *)local_f8 + 8))();
            }
            puStack_170 = (undefined *)0x141594dbc;
            puVar11 = (undefined8 *)FUN_14090f750(&local_120,&local_128,L"icon_card2");
            pIVar3 = (IUnknown *)*puVar11;
            if (pIVar3 != (IUnknown *)0x0) {
              *puVar11 = 0;
              pIVar23 = pIVar3;
              local_b0 = pIVar3;
            }
            if (local_120 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x141594dde;
              (**(code **)(*(longlong *)local_120 + 0x10))();
            }
            goto LAB_141594ecc;
          }
        }
        uVar25 = 0xffffffffffffffff;
        if (*param_2 - 3U < 2) {
          if (param_2[6] == 0) {
            local_128 = local_f8;
            if (local_f8 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x1415950f6;
              (**(code **)(*(longlong *)local_f8 + 8))();
            }
            puStack_170 = (undefined *)0x14159510b;
            plVar15 = (longlong *)FUN_14090f750(&local_120,&local_128,L"icon_public");
            uVar19 = 0x12;
          }
          else {
            local_e0 = local_f8;
            if (local_f8 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x1415950c9;
              (**(code **)(*(longlong *)local_f8 + 8))();
            }
            puStack_170 = (undefined *)0x1415950de;
            plVar15 = (longlong *)FUN_14090f750(&local_f0,&local_e0,L"icon_private");
            uVar19 = 0x11;
          }
          local_110 = (IUnknown *)CONCAT44(local_110._4_4_,uVar19);
          pIVar21 = (IUnknown *)*plVar15;
          *plVar15 = 0;
          uVar26 = uVar19;
          local_40 = pIVar21;
          if ((uVar19 & 2) != 0) {
            uVar26 = uVar19 & 0xfffffffd;
            local_110 = (IUnknown *)(CONCAT44(local_110._4_4_,uVar19) & 0xfffffffffffffffd);
            if (local_120 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x14159513f;
              (**(code **)(*(longlong *)local_120 + 0x10))();
            }
          }
          uVar19 = uVar26;
          if ((uVar26 & 1) != 0) {
            uVar19 = uVar26 & 0xfffffffe;
            local_110 = (IUnknown *)(CONCAT44(local_110._4_4_,uVar26) & 0xfffffffffffffffe);
            if (local_f0 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x14159515d;
              (**(code **)(*(longlong *)local_f0 + 0x10))();
            }
          }
          if (param_2[10] == 0) {
            local_d0 = local_f8;
            if (local_f8 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x1415951a4;
              (**(code **)(*(longlong *)local_f8 + 8))();
            }
            puStack_170 = (undefined *)0x1415951b9;
            puVar11 = (undefined8 *)FUN_14090f750(&local_c0,&local_d0,L"status1");
            uVar19 = uVar19 | 8;
          }
          else {
            local_100 = local_f8;
            if (local_f8 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x141595179;
              (**(code **)(*(longlong *)local_f8 + 8))();
            }
            puStack_170 = (undefined *)0x14159518e;
            puVar11 = (undefined8 *)FUN_14090f750(&local_118,&local_100,L"status2");
            uVar19 = uVar19 | 4;
          }
          pIVar3 = (IUnknown *)*puVar11;
          *puVar11 = 0;
          uVar26 = uVar19;
          local_b8 = pIVar3;
          if ((uVar19 & 8) != 0) {
            uVar26 = uVar19 & 0xfffffff7;
            local_110 = (IUnknown *)(CONCAT44(local_110._4_4_,uVar19) & 0xfffffffffffffff7);
            if (local_c0 != (longlong *)0x0) {
              puStack_170 = (undefined *)0x1415951e6;
              (**(code **)(*local_c0 + 0x10))();
            }
          }
          if (((uVar26 & 4) != 0) && (local_118 != (longlong *)0x0)) {
            puStack_170 = (undefined *)0x1415951fc;
            (**(code **)(*local_118 + 0x10))();
          }
          local_e0 = pIVar21;
          if (pIVar21 != (IUnknown *)0x0) {
            puStack_170 = (undefined *)0x14159520f;
            (**(code **)(*(longlong *)pIVar21 + 8))(pIVar21);
          }
          local_128 = pIVar2;
          puStack_170 = (undefined *)0x14159521d;
          (**(code **)(*(longlong *)pIVar2 + 8))(pIVar2);
          iVar7 = local_108[0];
          local_148 = CONCAT44(local_148._4_4_,0xff);
          puStack_170 = (undefined *)0x14159523e;
          FUN_142aa16b0(&local_128,&local_e0,local_d8[0],local_108[0]);
          local_e0 = pIVar3;
          if (pIVar3 != (IUnknown *)0x0) {
            puStack_170 = (undefined *)0x141595250;
            (**(code **)(*(longlong *)pIVar3 + 8))(pIVar3);
          }
          local_128 = pIVar2;
          puStack_170 = (undefined *)0x14159525e;
          (**(code **)(*(longlong *)pIVar2 + 8))(pIVar2);
          local_148 = CONCAT44(local_148._4_4_,0xff);
          puStack_170 = (undefined *)0x14159527b;
          FUN_142aa16b0(&local_128,&local_e0,local_d8[0],iVar7);
          if (param_2[10] == 0) {
            local_110 = (IUnknown *)0x0;
            local_100 = (IUnknown *)0x0;
            puStack_170 = (undefined *)0x1415952a6;
            FUN_14019ba10(&local_110,"number/cur/%d",param_2[8]);
            puStack_170 = (undefined *)0x1415952bb;
            FUN_14019ba10(&local_100,"number/max/%d",param_2[7]);
            local_d0 = local_f8;
            if (local_f8 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x1415952ce;
              (**(code **)(*(longlong *)local_f8 + 8))();
            }
            puStack_170 = (undefined *)0x1415952e0;
            FUN_14090f580(&local_128,&local_d0,&local_110);
            local_e0 = local_f8;
            if (local_f8 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x1415952f4;
              (**(code **)(*(longlong *)local_f8 + 8))();
            }
            puStack_170 = (undefined *)0x141595306;
            FUN_14090f580(&local_120,&local_e0,&local_100);
            if (local_128 != (IUnknown *)0x0) {
              local_d0 = local_128;
              puStack_170 = (undefined *)0x14159531a;
              (**(code **)(*(longlong *)local_128 + 8))();
              local_f0 = pIVar2;
              puStack_170 = (undefined *)0x141595328;
              (**(code **)(*(longlong *)pIVar2 + 8))(pIVar2);
              local_148 = CONCAT44(local_148._4_4_,0xff);
              puStack_170 = (undefined *)0x141595348;
              FUN_142aa16b0(&local_f0,&local_d0,0x45);
            }
            if (local_120 != (IUnknown *)0x0) {
              local_f0 = local_120;
              puStack_170 = (undefined *)0x141595362;
              (**(code **)(*(longlong *)local_120 + 8))(local_120);
              local_d0 = pIVar2;
              puStack_170 = (undefined *)0x141595370;
              (**(code **)(*(longlong *)pIVar2 + 8))(pIVar2);
              local_148 = CONCAT44(local_148._4_4_,0xff);
              puStack_170 = (undefined *)0x141595390;
              FUN_142aa16b0(&local_d0,&local_f0,0x59);
            }
            if (local_120 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x1415953a6;
              (**(code **)(*(longlong *)local_120 + 0x10))(local_120);
            }
            if (local_128 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x1415953b5;
              (**(code **)(*(longlong *)local_128 + 0x10))();
            }
            if (local_100 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x1415953c8;
              FUN_14019f2c0(local_100 + -0x10);
            }
            if (local_110 != (IUnknown *)0x0) {
              puStack_170 = (undefined *)0x1415953db;
              FUN_14019f2c0(local_110 + -0x10);
            }
          }
          if (pIVar3 != (IUnknown *)0x0) {
            puStack_170 = (undefined *)0x1415953ea;
            (**(code **)(*(longlong *)pIVar3 + 0x10))(pIVar3);
          }
          if (pIVar21 != (IUnknown *)0x0) {
            puStack_170 = (undefined *)0x1415953f9;
            (**(code **)(*(longlong *)pIVar21 + 0x10))(pIVar21);
          }
        }
        else {
          pwVar27 = L"cannotEnter";
          if (param_2[1] == 1) {
            pwVar27 = L"canEnter";
          }
          uVar24 = 0xffffffffffffffff;
          do {
            uVar24 = uVar24 + 1;
          } while (pwVar27[uVar24] != L'\0');
          iVar7 = (int)uVar24;
          if (0 < iVar7) {
            pIVar21 = (IUnknown *)(uVar24 & 0xffffffff);
          }
          puStack_170 = (undefined *)0x141594f66;
          piVar12 = (int *)FUN_1401bc720(&DAT_143ad6980,(longlong)((int)pIVar21 * 2 + 0x12));
          piVar12[1] = (int)pIVar21;
          *piVar12 = -1;
          plVar15 = (longlong *)(piVar12 + 4);
          piVar12[2] = 0;
          *(undefined2 *)plVar15 = 0;
          local_120 = (IUnknown *)((longlong)iVar7 * 2);
          puStack_170 = (undefined *)0x141594f94;
          local_118 = plVar15;
          FUN_142ef7ba0(plVar15,pwVar27,local_120);
          if (*piVar12 != -1) {
            puStack_170 = (undefined *)0x141594fa6;
            FUN_142e52dd0(0x8b);
          }
          if ((iVar7 == -1) || (iVar7 <= piVar12[1])) {
            *piVar12 = 1;
            if (iVar7 != -1) goto LAB_141594fce;
            if (plVar15 == (longlong *)0x0) {
              uVar24 = 0;
            }
            else {
              uVar24 = 0xffffffffffffffff;
              do {
                uVar24 = uVar24 + 1;
              } while (*(short *)((longlong)plVar15 + uVar24 * 2) != 0);
            }
          }
          else {
            puStack_170 = (undefined *)0x141594fc7;
            FUN_142e54290(0x90,piVar12[1],uVar24 & 0xffffffff);
            *piVar12 = 1;
LAB_141594fce:
            *(undefined2 *)(local_120 + (longlong)plVar15) = 0;
          }
          iVar7 = (int)uVar24;
          if ((iVar7 < 0) || (piVar12[1] + 1 <= iVar7)) {
            puStack_170 = (undefined *)0x141594ff4;
            FUN_142e54290(0x9c,uVar24 & 0xffffffff);
          }
          piVar12[2] = iVar7 * 2;
          local_128 = local_f8;
          if (local_f8 != (IUnknown *)0x0) {
            puStack_170 = (undefined *)0x14159500d;
            (**(code **)(*(longlong *)local_f8 + 8))();
          }
          puStack_170 = (undefined *)0x14159501e;
          FUN_14090f750(&local_e0,&local_128,plVar15);
          local_100 = local_e0;
          if (local_e0 != (IUnknown *)0x0) {
            puStack_170 = (undefined *)0x141595032;
            (**(code **)(*(longlong *)local_e0 + 8))();
          }
          local_110 = pIVar2;
          puStack_170 = (undefined *)0x141595040;
          (**(code **)(*(longlong *)pIVar2 + 8))(pIVar2);
          local_148 = CONCAT44(local_148._4_4_,0xff);
          puStack_170 = (undefined *)0x14159505e;
          FUN_142aa16b0(&local_110,&local_100,local_d8[0],local_108[0]);
          if (local_e0 != (IUnknown *)0x0) {
            puStack_170 = (undefined *)0x14159506e;
            (**(code **)(*(longlong *)local_e0 + 0x10))();
          }
          puStack_170 = (undefined *)0x141595078;
          FUN_1401bebb0(piVar12);
        }
        local_120 = local_f8;
        if (local_f8 != (IUnknown *)0x0) {
          puStack_170 = (undefined *)0x14159540d;
          (**(code **)(*(longlong *)local_f8 + 8))();
        }
        puStack_170 = (undefined *)0x141595425;
        FUN_14090fe90(&local_70,&local_120,L"title_lt");
        local_f0 = local_70;
        if (local_70 != (IUnknown *)0x0) {
          puStack_170 = (undefined *)0x14159543c;
          (**(code **)(*(longlong *)local_70 + 8))();
        }
        puStack_170 = (undefined *)0x14159544c;
        uVar13 = FUN_142aa2fa0(&local_f0,0);
        local_f0 = local_f8;
        local_d0 = (IUnknown *)uVar13;
        if (local_f8 != (IUnknown *)0x0) {
          puStack_170 = (undefined *)0x141595466;
          (**(code **)(*(longlong *)local_f8 + 8))();
        }
        puStack_170 = (undefined *)0x14159547e;
        FUN_14090fe90(&local_78,&local_f0,L"title_rb");
        local_120 = local_78;
        if (local_78 != (IUnknown *)0x0) {
          puStack_170 = (undefined *)0x141595495;
          (**(code **)(*(longlong *)local_78 + 8))();
        }
        puStack_170 = (undefined *)0x1415954a2;
        iVar8 = FUN_142aa2fa0(&local_120,0);
        iVar8 = iVar8 - (int)uVar13;
        iVar7 = 2;
        puStack_170 = (undefined *)0x1415954bb;
        FUN_1429fbeb0(&local_88,2);
        *(undefined4 *)(local_80 + 0x9c) = 0;
        lVar18 = 0;
        if (*(longlong *)(param_2 + 4) != 0) {
          local_140[0] = 0;
          local_148 = 0;
          puStack_170 = (undefined *)0x1415954ee;
          iVar7 = (*DAT_1432627f8)(0xfde9,0,*(longlong *)(param_2 + 4),0xffffffff);
          iVar7 = iVar7 * 2;
          lVar18 = *(longlong *)(param_2 + 4);
        }
        uVar24 = (longlong)iVar7 + 0xf;
        if (uVar24 <= (ulonglong)(longlong)iVar7) {
          uVar24 = 0xffffffffffffff0;
        }
        puStack_170 = (undefined *)0x14159551c;
        lVar5 = -(uVar24 & 0xfffffffffffffff0);
        puVar1 = (undefined2 *)((longlong)&local_128 + lVar5);
        if (lVar18 == 0) {
          if (puVar1 == (undefined2 *)0x0) goto LAB_141595558;
          *puVar1 = 0;
          local_e8 = (longlong *)0x0;
LAB_141595565:
          local_e8 = (longlong *)0x0;
          uVar24 = 0xffffffffffffffff;
          do {
            uVar24 = uVar24 + 1;
          } while (puVar1[uVar24] != 0);
          iVar17 = (int)uVar24;
          iVar7 = 0;
          if (0 < iVar17) {
            iVar7 = iVar17;
          }
          *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x14159559c;
          puVar14 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(iVar7 * 2 + 0x12));
          puVar14[1] = iVar7;
          *puVar14 = 0xffffffff;
          plVar15 = (longlong *)(puVar14 + 4);
          puVar14[2] = 0;
          *(undefined2 *)plVar15 = 0;
          local_118 = (longlong *)((longlong)iVar17 * 2);
          local_e8 = plVar15;
          *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x1415955cf;
          FUN_142ef7ba0(plVar15,puVar1,(longlong *)((longlong)iVar17 * 2));
          plVar15 = local_e8;
          if ((int)local_e8[-2] != -1) {
            *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x1415955e5;
            FUN_142e52dd0(0x8b);
          }
          if ((iVar17 == -1) || (iVar7 = *(int *)((longlong)plVar15 + -0xc), iVar17 <= iVar7)) {
            *(undefined4 *)(plVar15 + -2) = 1;
            if (iVar17 != -1) goto LAB_14159560f;
            if (plVar15 == (longlong *)0x0) {
              uVar24 = 0;
            }
            else {
              uVar24 = 0xffffffffffffffff;
              do {
                uVar24 = uVar24 + 1;
              } while (*(short *)((longlong)plVar15 + uVar24 * 2) != 0);
            }
          }
          else {
            *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x141595608;
            FUN_142e54290(0x90,iVar7,uVar24 & 0xffffffff);
            *(undefined4 *)(plVar15 + -2) = 1;
LAB_14159560f:
            *(undefined2 *)((longlong)local_e8 + (longlong)local_118) = 0;
          }
          iVar7 = (int)uVar24;
          if ((iVar7 < 0) || (*(int *)((longlong)plVar15 + -0xc) + 1 <= iVar7)) {
            *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x14159563c;
            FUN_142e54290(0x9c,uVar24 & 0xffffffff);
          }
          *(int *)(plVar15 + -1) = iVar7 * 2;
        }
        else {
          *(undefined4 *)((longlong)local_140 + lVar5) = 0x100000;
          *(undefined2 **)((longlong)local_140 + lVar5 + -8) = puVar1;
          *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x141595558;
          (*DAT_1432627f8)(0xfde9,0,lVar18,0xffffffff);
LAB_141595558:
          local_e8 = (longlong *)0x0;
          if (puVar1 != (undefined2 *)0x0) goto LAB_141595565;
        }
        pIVar2 = local_88;
        uVar19 = 0;
        if (local_88 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x141595e41;
          FUN_142ef3ac0(0x80004003);
        }
        *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x141595663;
        (*DAT_143262a20)(&local_a8);
        if (DAT_143a8b8d8 == 8) {
          if ((short)local_a8 == 8) {
            local_a8 = (uint)local_a8._2_2_ << 0x10;
            if (uStack_a0 != 0) {
              lVar18 = uStack_a0 - 4;
              *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x141595699;
              (*DAT_143ad5990)(lVar18);
            }
          }
          else {
            *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x1415956db;
            iVar7 = (*DAT_143262a18)(&local_a8);
            if (iVar7 < 0) goto LAB_141595e2f;
          }
          local_a8 = CONCAT22(local_a8._2_2_,8);
          if (DAT_143a8b8e0 != 0) {
            uVar19 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
          }
          *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x14159570a;
          uStack_a0 = FUN_1401a5fa0(DAT_143a8b8e0,uVar19);
        }
        else {
          if (((short)local_a8 == 8) && (local_a8 = (uint)local_a8._2_2_ << 0x10, uStack_a0 != 0)) {
            lVar18 = uStack_a0 - 4;
            *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x141595779;
            (*DAT_143ad5990)(lVar18);
          }
          *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x14159578d;
          iVar7 = (*DAT_143262a28)(&local_a8,&DAT_143a8b8d8);
          if (iVar7 < 0) {
LAB_141595e2f:
                    /* WARNING: Subroutine does not return */
            *(undefined **)(auStack_168 + lVar5 + -8) = &UNK_141595e36;
            FUN_142ef3ac0(iVar7);
          }
        }
        plVar15 = (longlong *)&DAT_1432780f4;
        if (local_e8 != (longlong *)0x0) {
          plVar15 = local_e8;
        }
        *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x14159572c;
        plVar15 = (longlong *)FUN_1401a5890(&local_118,plVar15);
        uVar20 = 0;
        local_100 = (IUnknown *)((ulonglong)local_100 & 0xffffffff00000000);
        pcVar4 = *(code **)(*(longlong *)pIVar2 + 200);
        if ((undefined8 *)*plVar15 != (undefined8 *)0x0) {
          uVar20 = *(undefined8 *)*plVar15;
        }
        local_68 = local_a8;
        uStack_64 = uStack_a4;
        uStack_60 = (undefined4)uStack_a0;
        uStack_5c = uStack_a0._4_4_;
        local_58 = local_98;
        local_b8 = (IUnknown *)plVar15;
        *(undefined8 **)((longlong)local_140 + lVar5 + -8) = &local_100;
        *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x1415957d3;
        iVar7 = (*pcVar4)(pIVar2,uVar20,iVar8,&local_68);
        if (iVar7 < 0) {
          *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x1415957e8;
          _com_issue_errorex(iVar7,pIVar2,(_GUID *)&DAT_143297250);
        }
        uVar24 = (ulonglong)local_100;
        local_110 = (IUnknown *)CONCAT44(local_110._4_4_,(int)local_100);
        *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x1415957f6;
        FUN_1401be120(plVar15);
        if ((short)local_a8 == 8) {
          local_a8 = local_a8 & 0xffff0000;
          if (uStack_a0 != 0) {
            lVar18 = uStack_a0 - 4;
            *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x14159581f;
            (*DAT_143ad5990)(lVar18);
          }
        }
        else {
          *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x14159582e;
          (*DAT_143262a18)(&local_a8);
        }
        *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x141595842;
        FUN_1404ac7b0(&local_e8,&local_118,0,uVar24 & 0xffffffff);
        plVar15 = local_118;
        local_e0 = local_88;
        if (local_118 == (longlong *)0x0) {
          iVar7 = 1;
        }
        else {
          *(undefined8 *)((longlong)local_138 + lVar5 + 8) = 0;
          *(undefined8 *)((longlong)local_138 + lVar5) = 0;
          *(undefined4 *)((longlong)local_140 + lVar5) = 0;
          *(undefined8 *)((longlong)local_140 + lVar5 + -8) = 0;
          *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x141595883;
          iVar7 = (*DAT_1432627f0)(0xfde9,0,plVar15,0xffffffff);
        }
        uVar24 = (longlong)iVar7 + 0xf;
        if (uVar24 <= (ulonglong)(longlong)iVar7) {
          uVar24 = 0xffffffffffffff0;
        }
        *(undefined8 *)(auStack_168 + lVar5 + -8) = 0x1415958a4;
        lVar18 = -(uVar24 & 0xfffffffffffffff0);
        plVar10 = (longlong *)((longlong)&local_128 + lVar18 + lVar5);
        local_c0 = plVar10;
        if (plVar15 == (longlong *)0x0) {
          if (plVar10 == (longlong *)0x0) goto LAB_1415958ef;
          *(undefined1 *)plVar10 = 0;
          local_128 = (IUnknown *)0x0;
LAB_1415958fc:
          local_128 = (IUnknown *)0x0;
          uVar24 = 0xffffffffffffffff;
          do {
            uVar24 = uVar24 + 1;
          } while (*(char *)((longlong)plVar10 + uVar24) != '\0');
          iVar17 = (int)uVar24;
          iVar7 = 0;
          if (0 < iVar17) {
            iVar7 = iVar17;
          }
          *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x141595929;
          puVar16 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30);
          plVar10 = local_c0;
          puVar16[1] = iVar7;
          *puVar16 = 0xffffffff;
          puVar14 = puVar16 + 4;
          puVar16[2] = 0;
          *(undefined1 *)puVar14 = 0;
          local_128 = (IUnknown *)puVar14;
          local_f0 = (IUnknown *)(longlong)iVar17;
          *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x14159595a;
          FUN_142ef7ba0(puVar14,plVar10,(IUnknown *)(longlong)iVar17);
          puVar14 = (undefined4 *)local_128;
          if (*(int *)((longlong)local_128 + -0x10) != -1) {
            *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x141595971;
            FUN_142e52dd0(0x8b);
          }
          if ((iVar17 == -1) || (iVar7 = puVar14[-3], iVar17 <= iVar7)) {
            puVar14[-4] = 1;
            if (iVar17 != -1) goto LAB_14159599d;
            if (puVar14 == (undefined4 *)0x0) {
              uVar24 = 0;
            }
            else {
              uVar24 = 0xffffffffffffffff;
              do {
                uVar24 = uVar24 + 1;
              } while (*(char *)((longlong)puVar14 + uVar24) != '\0');
            }
          }
          else {
            *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x141595995;
            FUN_142e54290(0x90,iVar7,uVar24 & 0xffffffff);
            puVar14[-4] = 1;
LAB_14159599d:
            *(IUnknown *)((longlong)local_128 + (longlong)local_f0) = (IUnknown)0x0;
          }
          iVar7 = (int)uVar24;
          if ((iVar7 < 0) || (puVar14[-3] + 1 <= iVar7)) {
            *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x1415959c8;
            FUN_142e54290(0x9c,uVar24 & 0xffffffff);
          }
          puVar14[-2] = iVar7;
        }
        else {
          *(undefined8 *)((longlong)local_138 + lVar18 + lVar5 + 8) = 0;
          *(undefined8 *)((longlong)local_138 + lVar18 + lVar5) = 0;
          *(undefined4 *)((longlong)local_140 + lVar18 + lVar5) = 0x100000;
          *(longlong **)((longlong)local_140 + lVar18 + lVar5 + -8) = plVar10;
          *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x1415958ef;
          (*DAT_1432627f0)(0xfde9,0,plVar15,0xffffffff);
LAB_1415958ef:
          local_128 = (IUnknown *)0x0;
          if (plVar10 != (longlong *)0x0) goto LAB_1415958fc;
        }
        pIVar2 = local_90;
        local_120 = local_90;
        pcVar4 = *(code **)(*(longlong *)local_90 + 8);
        *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x1415959e1;
        (*pcVar4)(pIVar2);
        iVar7 = local_d0._4_4_ + local_108[0];
        local_100 = (IUnknown *)CONCAT44(local_100._4_4_,iVar7);
        iVar17 = local_d8[0] + (int)uVar13;
        local_108[0] = iVar17;
        *(undefined4 *)((longlong)local_138 + lVar18 + lVar5) = 0xff;
        *(undefined4 *)((longlong)local_140 + lVar18 + lVar5) = 0;
        *(IUnknown **)((longlong)local_140 + lVar18 + lVar5 + -8) = local_e0;
        *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x141595a1c;
        FUN_142a0ff80(&local_120,iVar17,iVar7,&local_128);
        if (local_128 != (IUnknown *)0x0) {
          puVar14 = (undefined4 *)((longlong)local_128 + -0x10);
          *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x141595a2f;
          FUN_14019f2c0(puVar14);
        }
        iVar7 = 0;
        if (local_e8 != (longlong *)0x0) {
          iVar7 = (int)((ulonglong)(longlong)(int)local_e8[-1] >> 1);
        }
        uVar24 = (ulonglong)local_110 & 0xffffffff;
        puVar22 = auStack_168 + lVar18 + lVar5;
        if ((int)local_110 < iVar7) {
          *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x141595a99;
          FUN_1404ac7b0(&local_e8,&local_c0,uVar24,0xffffffff);
          if (local_e8 != (longlong *)0x0) {
            plVar10 = local_e8 + -2;
            *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x141595aac;
            FUN_1401bebb0(plVar10);
          }
          local_e8 = local_c0;
          local_120 = local_88;
          if (local_88 != (IUnknown *)0x0) {
            pcVar4 = *(code **)(*(longlong *)local_88 + 8);
            *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x141595adc;
            (*pcVar4)();
          }
          *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x141595aed;
          uVar9 = FUN_1429ebdb0(&local_e8,&local_120,iVar8);
          plVar10 = local_e8;
          *(undefined4 *)(local_80 + 0x9c) = uVar9;
          local_f0 = local_88;
          if (local_e8 == (longlong *)0x0) {
            iVar7 = 1;
          }
          else {
            *(undefined8 *)((longlong)local_138 + lVar18 + lVar5 + 8) = 0;
            *(undefined8 *)((longlong)local_138 + lVar18 + lVar5) = 0;
            *(undefined4 *)((longlong)local_140 + lVar18 + lVar5) = 0;
            *(undefined8 *)((longlong)local_140 + lVar18 + lVar5 + -8) = 0;
            *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x141595b37;
            iVar7 = (*DAT_1432627f0)(0xfde9,0,plVar10,0xffffffff);
          }
          plVar10 = local_e8;
          uVar24 = (longlong)iVar7 + 0xf;
          if (uVar24 <= (ulonglong)(longlong)iVar7) {
            uVar24 = 0xffffffffffffff0;
          }
          *(undefined8 *)(auStack_168 + lVar18 + lVar5 + -8) = 0x141595b5c;
          lVar6 = -(uVar24 & 0xfffffffffffffff0);
          puVar22 = (undefined1 *)((longlong)&local_128 + lVar6 + lVar18 + lVar5);
          if (plVar10 == (longlong *)0x0) {
            if (puVar22 == (undefined1 *)0x0) goto LAB_141595ba0;
            *puVar22 = 0;
            local_128 = (IUnknown *)0x0;
LAB_141595bad:
            local_128 = (IUnknown *)0x0;
            uVar24 = 0xffffffffffffffff;
            do {
              uVar24 = uVar24 + 1;
            } while (puVar22[uVar24] != '\0');
            iVar8 = (int)uVar24;
            iVar7 = 0;
            if (0 < iVar8) {
              iVar7 = iVar8;
            }
            *(undefined8 *)(auStack_168 + lVar6 + lVar18 + lVar5 + -8) = 0x141595bd3;
            puVar16 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30);
            puVar16[1] = iVar7;
            *puVar16 = 0xffffffff;
            puVar14 = puVar16 + 4;
            puVar16[2] = 0;
            *(undefined1 *)puVar14 = 0;
            local_128 = (IUnknown *)puVar14;
            local_c0 = (longlong *)(longlong)iVar8;
            *(undefined8 *)(auStack_168 + lVar6 + lVar18 + lVar5 + -8) = 0x141595c02;
            FUN_142ef7ba0(puVar14,puVar22,(longlong *)(longlong)iVar8);
            puVar14 = (undefined4 *)local_128;
            if (*(int *)((longlong)local_128 + -0x10) != -1) {
              *(undefined8 *)(auStack_168 + lVar6 + lVar18 + lVar5 + -8) = 0x141595c18;
              FUN_142e52dd0(0x8b);
            }
            if ((iVar8 == -1) || (iVar7 = puVar14[-3], iVar8 <= iVar7)) {
              puVar14[-4] = 1;
              if (iVar8 != -1) goto LAB_141595c42;
              if (puVar14 == (undefined4 *)0x0) {
                uVar24 = 0;
              }
              else {
                do {
                  uVar25 = uVar25 + 1;
                } while (*(char *)((longlong)puVar14 + uVar25) != '\0');
                uVar24 = uVar25 & 0xffffffff;
              }
            }
            else {
              *(undefined8 *)(auStack_168 + lVar6 + lVar18 + lVar5 + -8) = 0x141595c3b;
              FUN_142e54290(0x90,iVar7,uVar24 & 0xffffffff);
              puVar14[-4] = 1;
LAB_141595c42:
              *(undefined1 *)((longlong)local_128 + (longlong)local_c0) = 0;
            }
            iVar7 = (int)uVar24;
            if ((iVar7 < 0) || (puVar14[-3] + 1 <= iVar7)) {
              *(undefined8 *)(auStack_168 + lVar6 + lVar18 + lVar5 + -8) = 0x141595c6d;
              FUN_142e54290(0x9c,uVar24 & 0xffffffff);
            }
            puVar14[-2] = iVar7;
          }
          else {
            *(undefined8 *)((longlong)local_138 + lVar6 + lVar18 + lVar5 + 8) = 0;
            *(undefined8 *)((longlong)local_138 + lVar6 + lVar18 + lVar5) = 0;
            *(undefined4 *)((longlong)local_140 + lVar6 + lVar18 + lVar5) = 0x100000;
            *(undefined1 **)((longlong)local_140 + lVar6 + lVar18 + lVar5 + -8) = puVar22;
            *(undefined8 *)(auStack_168 + lVar6 + lVar18 + lVar5 + -8) = 0x141595ba0;
            (*DAT_1432627f0)(0xfde9,0,plVar10,0xffffffff);
LAB_141595ba0:
            local_128 = (IUnknown *)0x0;
            if (puVar22 != (undefined1 *)0x0) goto LAB_141595bad;
          }
          local_120 = pIVar2;
          pcVar4 = *(code **)(*(longlong *)pIVar2 + 8);
          *(undefined8 *)(auStack_168 + lVar6 + lVar18 + lVar5 + -8) = 0x141595c81;
          (*pcVar4)(pIVar2);
          iVar8 = (int)local_100 + 10;
          *(undefined4 *)((longlong)local_138 + lVar6 + lVar18 + lVar5) = 0xff;
          *(undefined4 *)((longlong)local_140 + lVar6 + lVar18 + lVar5) = 0;
          *(IUnknown **)((longlong)local_140 + lVar6 + lVar18 + lVar5 + -8) = local_f0;
          iVar7 = local_108[0];
          *(undefined8 *)(auStack_168 + lVar6 + lVar18 + lVar5 + -8) = 0x141595caf;
          FUN_142a0ff80(&local_120,iVar7,iVar8,&local_128);
          puVar22 = auStack_168 + lVar6 + lVar18 + lVar5;
          if (local_128 != (IUnknown *)0x0) {
            puVar14 = (undefined4 *)((longlong)local_128 + -0x10);
            *(undefined8 *)(auStack_168 + lVar6 + lVar18 + lVar5 + -8) = 0x141595cc2;
            FUN_14019f2c0(puVar14);
            puVar22 = auStack_168 + lVar6 + lVar18 + lVar5;
          }
        }
        lVar18 = local_80;
        *(undefined8 *)(puVar22 + -8) = 0x141595cd7;
        FUN_1415978e0(lVar18,0x46);
        *(undefined4 *)(lVar18 + 0x94) = 1;
        if (plVar15 != (longlong *)0x0) {
          *(undefined8 *)(puVar22 + -8) = 0x141595cef;
          FUN_1401bebb0(plVar15 + -2);
        }
        if (local_e8 != (longlong *)0x0) {
          plVar15 = local_e8 + -2;
          *(undefined8 *)(puVar22 + -8) = 0x141595d02;
          FUN_1401bebb0(plVar15);
        }
        if (local_88 != (IUnknown *)0x0) {
          pcVar4 = *(code **)(*(longlong *)local_88 + 0x10);
          *(undefined8 *)(puVar22 + -8) = 0x141595d15;
          (*pcVar4)();
        }
        if (local_78 != (IUnknown *)0x0) {
          pcVar4 = *(code **)(*(longlong *)local_78 + 0x10);
          *(undefined8 *)(puVar22 + -8) = 0x141595d28;
          (*pcVar4)();
        }
        if (local_70 != (IUnknown *)0x0) {
          pcVar4 = *(code **)(*(longlong *)local_70 + 0x10);
          *(undefined8 *)(puVar22 + -8) = 0x141595d3b;
          (*pcVar4)();
        }
        if (pIVar23 != (IUnknown *)0x0) {
          pcVar4 = *(code **)(*(longlong *)pIVar23 + 0x10);
          *(undefined8 *)(puVar22 + -8) = 0x141595d4a;
          (*pcVar4)(pIVar23);
        }
        pcVar4 = *(code **)(*(longlong *)pIVar2 + 0x10);
        *(undefined8 *)(puVar22 + -8) = 0x141595d55;
        (*pcVar4)(pIVar2);
        if (local_c8 != (IUnknown *)0x0) {
          pcVar4 = *(code **)(*(longlong *)local_c8 + 0x10);
          *(undefined8 *)(puVar22 + -8) = 0x141595d65;
          (*pcVar4)();
        }
        if (local_f8 != (IUnknown *)0x0) {
          pcVar4 = *(code **)(*(longlong *)local_f8 + 0x10);
          *(undefined8 *)(puVar22 + -8) = 0x141595d75;
          (*pcVar4)();
        }
        if ((longlong *)*param_3 != (longlong *)0x0) {
          pcVar4 = *(code **)(*(longlong *)*param_3 + 0x10);
          *(undefined8 *)(puVar22 + -8) = 0x141595d8b;
          (*pcVar4)();
        }
        goto LAB_141595def;
      }
      if (local_c8 != (IUnknown *)0x0) {
        puStack_170 = (undefined *)0x141594cbd;
        (**(code **)(*(longlong *)local_c8 + 0x10))();
      }
      if (local_f8 != (IUnknown *)0x0) {
        puStack_170 = (undefined *)0x141594ccd;
        (**(code **)(*(longlong *)local_f8 + 0x10))();
      }
    }
  }
  puVar22 = auStack_168;
  if ((longlong *)*param_3 != (longlong *)0x0) {
    puStack_170 = (undefined *)0x141595dee;
    (**(code **)(*(longlong *)*param_3 + 0x10))();
    puVar22 = auStack_168;
  }
LAB_141595def:
  *(undefined8 *)(puVar22 + -8) = 0x141595dfe;
  return;
}



//===========================================================
// FUN_141c3fcb0 @ 141c3fcb0   (273 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141c3fcb0(longlong param_1,longlong *param_2,undefined4 *param_3)

{
  char cVar1;
  int iVar2;
  undefined4 uVar3;
  undefined1 auStack_4a8 [32];
  undefined4 local_488 [2];
  longlong *local_480;
  undefined1 local_478 [1104];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  *param_3 = 1;
  local_480 = param_2;
  iVar2 = FUN_1415c0460(param_1 + 0x2b0,param_2,1);
  if (iVar2 == 0) {
    *param_3 = 0;
  }
  else {
    uVar3 = (*DAT_143262db0)();
    cVar1 = FUN_1408fc970(*(undefined4 *)(param_1 + 0x4d0),200,uVar3);
    if (cVar1 == '\0') {
      *param_3 = 0;
    }
    else {
      FUN_1406ed520(local_478,0x17e);
      local_488[0] = 8;
      FUN_1406ede20(local_478,local_488,4);
      uVar3 = FUN_1429e3ef0();
      FUN_1406ed9d0(local_478,uVar3);
      FUN_1406edc80(local_478,param_2);
      FUN_1415d01c0(local_478);
      uVar3 = (*DAT_143262db0)();
      *(undefined4 *)(param_1 + 0x4d0) = uVar3;
      FUN_1406ed610(local_478);
    }
  }
  if (*param_2 != 0) {
    FUN_14019f2c0(*param_2 + -0x10);
  }
  return;
}



//===========================================================
// FUN_1428b7600 @ 1428b7600   (3743 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x0001428b7d06) */

void FUN_1428b7600(longlong *param_1)

{
  char cVar1;
  int iVar2;
  undefined4 uVar3;
  int iVar4;
  longlong lVar5;
  longlong lVar6;
  undefined8 uVar7;
  longlong *plVar8;
  undefined8 *puVar9;
  int *piVar10;
  int *piVar11;
  longlong lVar12;
  int *piVar13;
  longlong lVar14;
  longlong *plVar15;
  longlong *plVar16;
  longlong *plVar17;
  longlong *plVar18;
  undefined1 auStack_e58 [32];
  undefined4 local_e38;
  undefined4 local_e30;
  undefined8 local_e28;
  undefined8 local_e20;
  undefined4 local_e18;
  undefined4 local_e10;
  int local_e08;
  int iStack_e04;
  longlong *local_e00;
  undefined4 local_df8 [2];
  int *local_df0;
  int *local_de8;
  longlong *local_de0;
  int local_dd8 [2];
  longlong local_dd0;
  int **local_dc8;
  int *local_dc0;
  longlong *local_db8;
  undefined8 local_db0;
  int *local_da8;
  longlong *local_da0;
  undefined8 local_d98;
  undefined1 local_d90 [8];
  longlong local_d88;
  undefined1 local_d80 [8];
  longlong local_d78;
  undefined8 *local_d70;
  undefined1 local_d68 [8];
  undefined8 local_d60;
  undefined1 *local_d58;
  undefined1 local_d50 [2];
  undefined2 local_d4e;
  undefined2 local_d4a;
  int local_d40;
  int local_d3c;
  int local_d38;
  int local_d34;
  undefined8 local_d28;
  longlong *local_d20;
  undefined8 local_d18;
  undefined4 local_d10;
  longlong *local_d08;
  undefined4 local_d00;
  undefined8 local_cf8;
  undefined8 local_cf0;
  undefined8 local_ce8;
  undefined4 local_ce0;
  undefined8 local_cd0;
  undefined8 local_cc8;
  undefined8 uStack_cc0;
  undefined8 local_cb8;
  undefined8 uStack_cb0;
  undefined8 local_ca8;
  undefined8 uStack_ca0;
  undefined1 local_8d8 [1104];
  undefined1 local_488 [1104];
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_e58;
  iVar2 = FUN_140f810b0(param_1 + 0x20);
  if (iVar2 != 0) {
    return;
  }
  FUN_142c0c190(DAT_143abfdf8,&local_e08,1);
  lVar14 = DAT_143aa84a0;
  FUN_142cbe730(DAT_143aa84a0);
  if (*(int *)((longlong)param_1 + 0x561c) != 0) {
    uVar3 = (*DAT_143262db0)();
    cVar1 = FUN_1408fc980(*(undefined4 *)((longlong)param_1 + 0x561c),uVar3);
    if (cVar1 == '\0') {
      return;
    }
  }
  lVar5 = FUN_1429b60d0(DAT_143ac1b90,&local_e08);
  if (lVar5 != 0) {
    lVar14 = 0;
    local_dd0 = 0;
    iVar2 = FUN_14276f8d0(lVar5);
    if (iVar2 != 0) {
      local_de0 = (longlong *)FUN_14019b780(&DAT_143ad68a0,2000);
      lVar6 = lVar14;
      if (local_de0 != (longlong *)0x0) {
        lVar6 = FUN_142a57d30(local_de0,0,0,0);
      }
      lVar12 = lVar6 + 0x18;
      if (lVar6 == 0) {
        lVar12 = lVar14;
      }
      if (lVar12 == 0) {
        local_db8 = (longlong *)0x0;
      }
      else {
        local_db8 = (longlong *)(lVar12 + -0x18);
        if (local_db8 != (longlong *)0x0) {
          if (0xfffff < *(ulonglong *)(lVar12 + 8)) {
            FUN_142e541f0(0x30f);
          }
          LOCK();
          *(longlong *)(lVar12 + 8) = *(longlong *)(lVar12 + 8) + 1;
          UNLOCK();
        }
      }
      plVar16 = local_db8;
      if (local_db8 == (longlong *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      local_d58 = local_d68;
      local_d60 = 0;
      local_d70 = &local_d98;
      local_d98 = 0;
      uVar7 = FUN_1408a9e40(local_d80,0x204);
      local_e10 = 0;
      local_e18 = 0;
      local_e20 = local_d68;
      local_e28 = &local_d98;
      local_e30 = 0;
      local_e38 = 1;
      FUN_142a61900(plVar16,5,0,uVar7);
      if (plVar16 == (longlong *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      local_e30 = 0;
      local_e38 = 0;
      FUN_142a62e00(plVar16,0,0,0x1e);
      if (plVar16 == (longlong *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      FUN_142a5ee30(plVar16);
      if (plVar16 == (longlong *)0x0) {
        FUN_142e52ed0(0x431,0);
      }
      iVar2 = (**(code **)(*plVar16 + 0x130))(plVar16);
      if (iVar2 != 1) {
        if (0xffffe < plVar16[4] - 1U) {
          FUN_142e541f0(0x31e);
        }
        LOCK();
        plVar16 = plVar16 + 4;
        lVar14 = *plVar16;
        *plVar16 = *plVar16 + -1;
        UNLOCK();
        if (((int)lVar14 == 1) && (plVar16 = local_db8 + 3, plVar16 != (longlong *)0x0)) {
          (**(code **)*plVar16)(plVar16,1);
        }
        goto LAB_1428b7985;
      }
      plVar8 = (longlong *)FUN_142a643d0(plVar16,&local_d78);
      if (local_dd0 != 0) {
        FUN_14019f2c0(local_dd0 + -0x10);
      }
      local_dd0 = *plVar8;
      *plVar8 = 0;
      if (local_d78 != 0) {
        FUN_14019f2c0(local_d78 + -0x10);
      }
      lVar14 = 1;
      if (0xffffe < plVar16[4] - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar16 = plVar16 + 4;
      lVar6 = *plVar16;
      *plVar16 = *plVar16 + -1;
      UNLOCK();
      if (((int)lVar6 == 1) && (plVar16 = local_db8 + 3, plVar16 != (longlong *)0x0)) {
        (**(code **)*plVar16)(plVar16,1);
      }
    }
    FUN_1406ed520(local_8d8,0x17e);
    local_df8[0] = 3;
    FUN_1406ede20(local_8d8,local_df8,4);
    uVar3 = FUN_14276f790(lVar5);
    FUN_1406ed9d0(local_8d8,uVar3);
    FUN_1406ed840(local_8d8,lVar14);
    if ((int)lVar14 != 0) {
      FUN_1406edc80(local_8d8,&local_dd0);
    }
    FUN_1406ed840(local_8d8,0);
    FUN_1415d01c0(local_8d8);
    FUN_1406ed610(local_8d8);
LAB_1428b7985:
    if (local_dd0 != 0) {
      FUN_14019f2c0(local_dd0 + -0x10);
    }
    return;
  }
  lVar5 = FUN_1429b64e0(DAT_143ac1b90,&local_e08);
  if ((lVar5 != 0) && (iVar2 = FUN_142cc42d0(lVar14,500,0), iVar2 != 0)) {
    FUN_1406ed520(&local_d28,0x17f);
    local_df8[0] = 2;
    FUN_1406ede20(&local_d28,local_df8,4);
    uVar3 = FUN_14276f7a0(lVar5);
    FUN_1406ed9d0(&local_d28,uVar3);
    FUN_1415d01c0(&local_d28);
    FUN_1406ed610(&local_d28);
    return;
  }
  lVar14 = FUN_141892840();
  if ((lVar14 != 0) && (iVar2 = FUN_1418921f0(lVar14,CONCAT44(iStack_e04,local_e08)), iVar2 != 0)) {
    FUN_1406ed520(local_488,0x17f);
    local_df8[0] = 2;
    FUN_1406ede20(local_488,local_df8,4);
    FUN_1406ed9d0(local_488,iVar2);
    FUN_1415d01c0(local_488);
    FUN_1406ed610(local_488);
  }
  FUN_1406fbcc0(DAT_143acf098,local_d90,CONCAT44(iStack_e04,local_e08));
  lVar14 = local_d88;
  plVar8 = (longlong *)0x0;
  plVar16 = (longlong *)0xffffffffffffffff;
  if (local_d88 != 0) {
    iVar2 = FUN_14276df20(param_1);
    if (*(int *)(lVar14 + 8) == iVar2) {
      uVar7 = 0xed4;
LAB_1428b7b1a:
      uVar7 = FUN_1408a9e40(&local_e00,uVar7);
      local_e10 = 0;
      local_e18 = 0;
      local_e20 = (undefined1 *)((ulonglong)local_e20._4_4_ << 0x20);
      local_e28 = (undefined8 *)((ulonglong)local_e28._4_4_ << 0x20);
      local_e30 = 0;
      local_e38 = 0;
      FUN_142a26280(uVar7,0,0,1);
      goto LAB_1428b8420;
    }
    cVar1 = FUN_1423f6990();
    if (cVar1 == '\0') goto LAB_1428b8420;
    iVar2 = FUN_140f8ab50(param_1 + 0x20);
    if ((iVar2 != 0) || (cVar1 = (**(code **)(*param_1 + 0x120))(param_1), cVar1 != '\0')) {
      uVar7 = FUN_1408a9e40(&local_de0,0xea4);
      FUN_140d84b70(uVar7,0);
      goto LAB_1428b8420;
    }
    if (DAT_143aa8520 != 0) {
      uVar7 = 0x86;
      goto LAB_1428b7b1a;
    }
    iVar2 = FUN_142d0e5d0(DAT_143aa84a0);
    if (iVar2 == 0) {
      piVar11 = (int *)FUN_14019b780(&DAT_143ad68a0,0x2f0);
      local_dc0 = piVar11;
      if (piVar11 != (int *)0x0) {
        if (local_d88 == 0) {
          FUN_142e52ed0(0x431,0);
        }
        local_e38 = *(undefined4 *)(local_d88 + 8);
        FUN_1423e6960(piVar11,3,0);
      }
      local_e38 = 0;
      FUN_142cb1020(DAT_143aa84a0,0,0xffffffff,0);
      FUN_142386bc0(DAT_143ac8240,1);
      plVar16 = DAT_143ac8240 + 1;
      if (DAT_143ac8240 == (longlong *)0x0) {
        plVar16 = plVar8;
      }
      FUN_142c0bf50(DAT_143abfdf8,plVar16,0);
      iVar2 = (**(code **)(*(longlong *)(DAT_143aa8520 + 8) + 0x90))();
      iVar4 = FUN_142bf7d70(DAT_143aa8520);
      uVar3 = (**(code **)(*(longlong *)(DAT_143aa8520 + 8) + 0x98))();
      (**(code **)(*DAT_143ac8240 + 0x80))(DAT_143ac8240,iVar2 + iVar4,uVar3);
      goto LAB_1428b8420;
    }
    local_db0 = *(undefined8 *)(DAT_143aa84a0 + 0x3840);
    local_db0 = FUN_1408f63b0(&local_db0,1);
    (*DAT_1432625b0)(&local_db0,local_d50);
    local_e00 = (longlong *)0x0;
    puVar9 = (undefined8 *)FUN_1408a9e40(&local_de0,0xbb);
    uVar7 = FUN_14019ba10(&local_e00,*puVar9,local_d4e,local_d4a);
    local_df0 = (int *)0x0;
    FUN_14019a260(&local_df0,uVar7);
    if (local_de0 != (longlong *)0x0) {
      FUN_14019f2c0(local_de0 + -2);
    }
    if (local_e00 != (longlong *)0x0) {
      FUN_14019f2c0(local_e00 + -2);
    }
    piVar13 = local_df0;
    local_de8 = (int *)0x0;
    piVar11 = local_de8;
    if ((local_df0 != (int *)0x0) && (piVar10 = local_df0 + -4, piVar10 != (int *)0x0)) {
      if (*piVar10 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        plVar15 = plVar16;
        do {
          plVar15 = (longlong *)((longlong)plVar15 + 1);
        } while (*(char *)((longlong)piVar13 + (longlong)plVar15) != '\0');
        iVar4 = (int)plVar15;
        iVar2 = 0;
        if (0 < iVar4) {
          iVar2 = iVar4;
        }
        piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar2 + 0x11));
        piVar10[1] = iVar2;
        *piVar10 = -1;
        piVar11 = piVar10 + 4;
        piVar10[2] = 0;
        *(undefined1 *)piVar11 = 0;
        local_dc0 = piVar11;
        FUN_142ef7ba0(piVar11,piVar13,(longlong)iVar4);
        if (*piVar10 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar4 == -1) || (iVar4 <= piVar10[1])) {
          *piVar10 = 1;
          if (iVar4 != -1) goto LAB_1428b7cbe;
          plVar15 = plVar8;
          if (piVar11 != (int *)0x0) {
            do {
              plVar16 = (longlong *)((longlong)plVar16 + 1);
              plVar15 = plVar16;
            } while (*(char *)((longlong)piVar11 + (longlong)plVar16) != '\0');
          }
        }
        else {
          FUN_142e54290(0x90,piVar10[1],(ulonglong)plVar15 & 0xffffffff);
          *piVar10 = 1;
LAB_1428b7cbe:
          *(undefined1 *)((longlong)iVar4 + (longlong)piVar11) = 0;
        }
        iVar2 = (int)plVar15;
        if ((iVar2 < 0) || (piVar10[1] + 1 <= iVar2)) {
          FUN_142e54290(0x9c,(ulonglong)plVar15 & 0xffffffff);
        }
        piVar10[2] = iVar2;
        if (local_de8 != (int *)0x0) {
          FUN_14019f2c0(local_de8 + -4);
        }
      }
      else {
        if (*piVar10 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar10 = *piVar10 + 1;
        UNLOCK();
        if (local_de8 != (int *)0x0) {
          FUN_14019f2c0(local_de8 + -4);
        }
        local_de8 = piVar13;
        piVar13 = local_df0;
        piVar11 = local_de8;
      }
    }
    local_de8 = piVar11;
    FUN_140d84b70(&local_de8,0);
    if (piVar13 != (int *)0x0) {
      FUN_14019f2c0(piVar13 + -4);
    }
    goto LAB_1428b8420;
  }
  lVar14 = FUN_141c33440(DAT_143acf080,&local_e08);
  if (lVar14 != 0) {
    piVar11 = (int *)FUN_14019b780(&DAT_143ad68a0,0x2d8);
    local_dc0 = piVar11;
    if (piVar11 != (int *)0x0) {
      uVar3 = *(undefined4 *)(lVar14 + 0x50);
      local_dc8 = &local_df0;
      local_df0 = (int *)0x0;
      FUN_14019a260(&local_df0,lVar14 + 0x18);
      local_e00 = (longlong *)0x0;
      FUN_14019a260(&local_e00,lVar14 + 0x10);
      FUN_141c30eb0(piVar11,&local_e00,&local_df0,uVar3);
    }
    goto LAB_1428b8420;
  }
  local_dd8[0] = -1;
  plVar15 = (longlong *)FUN_1429b62d0(DAT_143ac1b90,&local_e08,local_dd8);
  iVar2 = local_dd8[0];
  if (plVar15 != (longlong *)0x0) {
    if (plVar15 == (longlong *)0xfffffffffffffff0) {
      plVar15 = (longlong *)0x0;
    }
    local_da0 = plVar15;
    if (plVar15 != (longlong *)0x0) {
      if (0xfffff < (ulonglong)plVar15[3]) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      plVar15[3] = plVar15[3] + 1;
      UNLOCK();
    }
    plVar16 = local_da0;
    local_dc8 = &local_da8;
    if (local_da0 == (longlong *)0x0) {
      FUN_142962200(&local_da8);
    }
    else if (iVar2 == 0) {
      iVar2 = (**(code **)(*local_da0 + 0x50))(local_da0);
      lVar14 = DAT_143aa84a0;
      if (iVar2 == 0) {
        if (DAT_143aa84a0 != 0) {
          uVar3 = FUN_14276df20(plVar16);
          FUN_142d1caf0(lVar14,uVar3,1);
        }
      }
      else {
        FUN_1411571c0(0x11,0);
        if (DAT_143ac8908 != 0) {
          FUN_141197a40();
          FUN_141198f20(DAT_143ac8908,1);
        }
      }
      FUN_142962200(&local_da8);
    }
    else {
      FUN_142962200(&local_da8);
    }
    goto LAB_1428b8420;
  }
  lVar14 = FUN_1429b5be0(DAT_143ac1b90,&local_e08);
  if (lVar14 != 0) {
    uVar3 = FUN_14276df20(lVar14);
    FUN_142d1caf0(DAT_143aa84a0,uVar3,0);
  }
  (**(code **)(*param_1 + 0x10))(param_1,&local_d40,1);
  if ((((local_e08 < local_d40) || (local_d38 <= local_e08)) || (iStack_e04 < local_d3c)) ||
     (local_d34 <= iStack_e04)) {
    if (param_1[0x94b] != 0) {
      plVar16 = (longlong *)FUN_142954330(param_1 + 0x94a);
      iVar2 = (**(code **)(*plVar16 + 0x10))(plVar16,&local_d40,1);
      if (((iVar2 != 0) && (local_d40 <= local_e08)) &&
         ((local_e08 < local_d38 && ((local_d3c <= iStack_e04 && (iStack_e04 < local_d34)))))) {
        FUN_1406ed520(&local_d28,0x1a1);
        lVar14 = FUN_142954330(param_1 + 0x94a);
        FUN_1406ed9d0(&local_d28,*(undefined4 *)(lVar14 + 0x30c));
        FUN_1415d01c0(&local_d28);
        FUN_1406ed610(&local_d28);
        goto LAB_1428b8420;
      }
    }
    iVar2 = FUN_1428ec060(param_1,&local_e08);
    if ((iVar2 == 0) && (param_1[0x274] != 0)) {
      uVar7 = FUN_14286e800(param_1 + 0x273);
      FUN_140da5c90(uVar7,CONCAT44(iStack_e04,local_e08));
    }
    goto LAB_1428b8420;
  }
  local_d28 = 0;
  local_d20 = (longlong *)0x0;
  local_d18 = 0;
  local_d10 = 0;
  local_d08 = (longlong *)0x0;
  local_d00 = 0;
  local_cf8 = 0;
  local_cf0 = 0;
  local_ce8 = 0;
  local_ce0 = 0;
  local_cd0 = 0;
  local_cc8 = 0;
  uStack_cc0 = 0;
  local_cb8 = 0;
  uStack_cb0 = 0;
  local_ca8 = 0;
  uStack_ca0 = 0;
  uVar3 = FUN_14276df20(param_1);
  local_d28 = CONCAT44(local_d28._4_4_,uVar3);
  lVar14 = FUN_14276df30(param_1);
  local_de0 = (longlong *)0x0;
  plVar15 = plVar8;
  plVar17 = plVar16;
  if (lVar14 != 0) {
    do {
      plVar17 = (longlong *)((longlong)plVar17 + 1);
    } while (*(char *)((longlong)plVar17 + lVar14) != '\0');
    iVar4 = (int)plVar17;
    iVar2 = 0;
    if (0 < iVar4) {
      iVar2 = iVar4;
    }
    piVar11 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar2 + 0x11));
    piVar11[1] = iVar2;
    *piVar11 = -1;
    plVar15 = (longlong *)(piVar11 + 4);
    piVar11[2] = 0;
    *(undefined1 *)plVar15 = 0;
    local_de0 = plVar15;
    FUN_142ef7ba0(plVar15,lVar14,(longlong)iVar4);
    if (*piVar11 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar4 == -1) || (iVar4 <= piVar11[1])) {
      *piVar11 = 1;
      if (iVar4 != -1) goto LAB_1428b8186;
      plVar18 = plVar16;
      plVar17 = plVar8;
      if (plVar15 != (longlong *)0x0) {
        do {
          plVar17 = (longlong *)((longlong)plVar18 + 1);
          plVar18 = plVar17;
        } while (*(char *)((longlong)plVar15 + (longlong)plVar17) != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,piVar11[1],(ulonglong)plVar17 & 0xffffffff);
      *piVar11 = 1;
LAB_1428b8186:
      *(undefined1 *)((longlong)iVar4 + (longlong)plVar15) = 0;
    }
    iVar2 = (int)plVar17;
    if ((iVar2 < 0) || (piVar11[1] + 1 <= iVar2)) {
      FUN_142e54290(0x9c,(ulonglong)plVar17 & 0xffffffff);
    }
    piVar11[2] = iVar2;
  }
  local_d20 = plVar15;
  uVar3 = (**(code **)(*param_1 + 0x40))(param_1);
  local_d18 = CONCAT44(local_d18._4_4_,uVar3);
  uVar3 = (**(code **)(*param_1 + 0xe8))(param_1);
  local_d18 = CONCAT44(uVar3,(undefined4)local_d18);
  local_d10 = FUN_14282b880(param_1);
  lVar14 = FUN_14276df40(param_1);
  local_e00 = (longlong *)0x0;
  plVar17 = plVar8;
  plVar18 = plVar16;
  if (lVar14 != 0) {
    do {
      plVar18 = (longlong *)((longlong)plVar18 + 1);
    } while (*(char *)(lVar14 + (longlong)plVar18) != '\0');
    iVar4 = (int)plVar18;
    iVar2 = 0;
    if (0 < iVar4) {
      iVar2 = iVar4;
    }
    piVar11 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar2 + 0x11));
    piVar11[1] = iVar2;
    *piVar11 = -1;
    plVar17 = (longlong *)(piVar11 + 4);
    piVar11[2] = 0;
    *(undefined1 *)plVar17 = 0;
    local_e00 = plVar17;
    FUN_142ef7ba0(plVar17,lVar14,(longlong)iVar4);
    if (*piVar11 != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((iVar4 == -1) || (iVar4 <= piVar11[1])) {
      *piVar11 = 1;
      if (iVar4 != -1) goto LAB_1428b8286;
      plVar18 = plVar8;
      if (plVar17 != (longlong *)0x0) {
        do {
          plVar16 = (longlong *)((longlong)plVar16 + 1);
          plVar18 = plVar16;
        } while (*(char *)((longlong)plVar17 + (longlong)plVar16) != '\0');
      }
    }
    else {
      FUN_142e54290(0x90,piVar11[1],(ulonglong)plVar18 & 0xffffffff);
      *piVar11 = 1;
LAB_1428b8286:
      *(undefined1 *)((longlong)plVar17 + (longlong)iVar4) = 0;
    }
    iVar2 = (int)plVar18;
    if ((iVar2 < 0) || (piVar11[1] + 1 <= iVar2)) {
      FUN_142e54290(0x9c,(ulonglong)plVar18 & 0xffffffff);
    }
    piVar11[2] = iVar2;
  }
  local_d08 = plVar17;
  FUN_1411571c0(0x11);
  if (DAT_143ac8908 != 0) {
    FUN_141197a40();
  }
  if (plVar17 != (longlong *)0x0) {
    FUN_14019f2c0(plVar17 + -2);
  }
  if (plVar15 != (longlong *)0x0) {
    FUN_14019f2c0(plVar15 + -2);
  }
LAB_1428b8420:
  lVar14 = local_d88;
  if (local_d88 == 0) {
    return;
  }
  if (0xffffe < *(longlong *)(local_d88 + 0x30) - 1U) {
    FUN_142e541f0(0x31e);
  }
  LOCK();
  plVar16 = (longlong *)(lVar14 + 0x30);
  lVar14 = *plVar16;
  *plVar16 = *plVar16 + -1;
  UNLOCK();
  if ((int)lVar14 != 1) {
    return;
  }
  plVar16 = (longlong *)(local_d88 + 0x28);
  if (local_d88 == 0) {
    plVar16 = plVar8;
  }
  if (plVar16 == (longlong *)0x0) {
    return;
  }
  (**(code **)*plVar16)(plVar16,1);
  return;
}



//===========================================================
// FUN_142d1cf20 @ 142d1cf20   (212 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142d1cf20(longlong param_1,undefined8 param_2)

{
  longlong lVar1;
  char cVar2;
  undefined1 auStack_4a8 [32];
  undefined4 local_488 [4];
  undefined1 local_478 [1104];
  ulonglong local_28;
  
  lVar1 = DAT_143aa84a0;
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  cVar2 = FUN_1406e8ae0(param_2);
  FUN_1406e8c20(param_2);
  if ((lVar1 != 0) && (cVar2 == '\0')) {
    FUN_1406ed520(local_478,0x17e);
    local_488[0] = 0;
    FUN_1406ede20(local_478,local_488,4);
    local_488[0] = 2;
    FUN_1406ede20(local_478,local_488,4);
    FUN_1406ed9d0(local_478,*(undefined4 *)(param_1 + 0x232c));
    FUN_1415d01c0(local_478);
    FUN_1406ed610(local_478);
  }
  return;
}



//===========================================================
// FUN_1427956b0 @ 1427956b0   (944 bytes)
//===========================================================

void FUN_1427956b0(longlong *param_1,undefined8 param_2)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong lVar3;
  undefined8 *puVar4;
  byte bVar5;
  undefined4 uVar6;
  int iVar7;
  undefined4 uVar8;
  longlong *plVar9;
  int *piVar10;
  undefined8 uVar11;
  int *piVar12;
  int local_res8 [2];
  undefined4 local_res18 [2];
  int *local_res20;
  longlong local_78;
  longlong local_70;
  undefined1 local_68 [8];
  longlong *local_60;
  longlong *local_58;
  longlong *local_50;
  longlong *plStack_48;
  
  FUN_1406e9170(param_2,local_res8,4);
  local_60 = param_1 + 0x223;
  *(int *)local_60 = local_res8[0];
  if (local_res8[0] == 0) {
    FUN_141596ea0(param_1[8]);
  }
  else {
    uVar6 = FUN_1406e8c20(param_2);
    *(undefined4 *)(param_1 + 0x224) = uVar6;
    FUN_1406e9170(param_2,local_res18,4);
    *(undefined4 *)((longlong)param_1 + 0x111c) = local_res18[0];
    plVar9 = (longlong *)FUN_1406e9050(param_2,&local_70);
    if (param_1[0x225] != 0) {
      FUN_14019f2c0(param_1[0x225] + -0x10);
      param_1[0x225] = 0;
    }
    param_1[0x225] = *plVar9;
    *plVar9 = 0;
    if (local_70 != 0) {
      FUN_14019f2c0(local_70 + -0x10);
    }
    uVar6 = FUN_1406e8c20(param_2);
    *(undefined4 *)((longlong)param_1 + 0x113c) = uVar6;
    bVar5 = FUN_1406e8ae0(param_2);
    *(uint *)(param_1 + 0x226) = (uint)bVar5;
    bVar5 = FUN_1406e8ae0(param_2);
    *(uint *)(param_1 + 0x227) = (uint)bVar5;
    bVar5 = FUN_1406e8ae0(param_2);
    *(uint *)((longlong)param_1 + 0x1134) = (uint)bVar5;
    bVar5 = FUN_1406e8ae0(param_2);
    *(uint *)(param_1 + 0x228) = (uint)bVar5;
    iVar7 = FUN_140f8abc0(param_1 + 0x20);
    uVar11 = DAT_143aa84a0;
    if (iVar7 == 0) {
      uVar6 = (**(code **)(*param_1 + 0x38))(param_1);
      iVar7 = FUN_142d01050(uVar11,uVar6);
      if (iVar7 == 0) {
        local_res20 = (int *)0x0;
        FUN_14019a260(&local_res20,param_1 + 0x225);
        piVar12 = local_res20;
      }
      else {
        piVar10 = (int *)FUN_14019b600(&DAT_143ad6a30,0x12);
        piVar10[1] = 1;
        *piVar10 = -1;
        piVar12 = piVar10 + 4;
        piVar10[2] = 0;
        *(undefined1 *)piVar12 = 0;
        *(undefined1 *)piVar12 = DAT_143275b98;
        local_res20 = piVar12;
        if (*piVar10 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if (piVar10[1] < 1) {
          FUN_142e54290(0x90,piVar10[1],1);
        }
        *piVar10 = 1;
        *(undefined1 *)((longlong)piVar10 + 0x11) = 0;
        if (piVar10[1] + 1 < 2) {
          FUN_142e54290(0x9c,1);
        }
        piVar10[2] = 1;
      }
      lVar3 = param_1[8];
      param_1 = param_1 + 1;
      uVar6 = (**(code **)(*param_1 + 0x40))(param_1);
      uVar11 = (**(code **)(*param_1 + 0x50))(param_1);
      uVar8 = FUN_1409c6cc0(uVar11);
      uVar11 = FUN_1409397b0(param_1,local_68);
      FUN_141594970(lVar3,local_60,uVar11,uVar8,uVar6);
      uVar11 = FUN_1408a9e40(&local_60,0x1422);
      if (piVar12 == (int *)0x0) {
        iVar7 = 0;
      }
      else {
        iVar7 = piVar12[-2];
      }
      FUN_1401abc80(uVar11,&local_78,piVar12,iVar7);
      if (local_60 != (longlong *)0x0) {
        FUN_14019f2c0(local_60 + -2);
      }
      local_50 = (longlong *)0x0;
      plStack_48 = (longlong *)0x0;
      plVar9 = (longlong *)FUN_14019b780(&DAT_143ad68a0,0x58);
      local_58 = plVar9;
      if (plVar9 == (longlong *)0x0) {
        plVar9 = (longlong *)0x0;
      }
      else {
        *plVar9 = 0;
        plVar9[1] = 0;
        *(undefined4 *)(plVar9 + 1) = 1;
        *(undefined4 *)((longlong)plVar9 + 0xc) = 1;
        *plVar9 = (longlong)&PTR_FUN_143300d78;
        FUN_1408d6690(plVar9 + 2);
      }
      plVar2 = plStack_48;
      local_50 = plVar9 + 2;
      if (plStack_48 != (longlong *)0x0) {
        LOCK();
        plVar1 = plStack_48 + 1;
        lVar3 = *plVar1;
        *(int *)plVar1 = (int)*plVar1 + -1;
        UNLOCK();
        piVar12 = local_res20;
        if ((int)lVar3 == 1) {
          puVar4 = (undefined8 *)*plStack_48;
          plStack_48 = plVar9;
          (*(code *)*puVar4)(plVar2);
          LOCK();
          piVar12 = (int *)((longlong)plVar2 + 0xc);
          iVar7 = *piVar12;
          *piVar12 = *piVar12 + -1;
          UNLOCK();
          piVar12 = local_res20;
          plVar9 = plStack_48;
          if (iVar7 == 1) {
            (**(code **)(*plVar2 + 8))(plVar2);
            piVar12 = local_res20;
            plVar9 = plStack_48;
          }
        }
      }
      plStack_48 = plVar9;
      FUN_1408d6760(local_50,param_2);
      FUN_1415ed1c0(&local_50,&local_78,0x1f);
      plVar9 = plStack_48;
      if (plStack_48 != (longlong *)0x0) {
        LOCK();
        plVar2 = plStack_48 + 1;
        lVar3 = *plVar2;
        *(int *)plVar2 = (int)*plVar2 + -1;
        UNLOCK();
        piVar12 = local_res20;
        if ((int)lVar3 == 1) {
          (**(code **)*plStack_48)(plStack_48);
          LOCK();
          piVar12 = (int *)((longlong)plVar9 + 0xc);
          iVar7 = *piVar12;
          *piVar12 = *piVar12 + -1;
          UNLOCK();
          piVar12 = local_res20;
          if (iVar7 == 1) {
            (**(code **)(*plVar9 + 8))(plVar9);
            piVar12 = local_res20;
          }
        }
      }
      if (local_78 != 0) {
        FUN_14019f2c0(local_78 + -0x10);
      }
      if (piVar12 != (int *)0x0) {
        FUN_14019f2c0(piVar12 + -4);
      }
    }
  }
  return;
}



//===========================================================
// FUN_141c3d980 @ 141c3d980   (1809 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Removing unreachable block (ram,0x000141c3de33) */
/* WARNING: Type propagation algorithm not settling */

void FUN_141c3d980(undefined8 param_1)

{
  int ****ppppiVar1;
  longlong *plVar2;
  int ****ppppiVar3;
  undefined4 uVar4;
  int iVar5;
  undefined8 *puVar6;
  int *piVar7;
  longlong lVar8;
  undefined8 uVar9;
  int ****ppppiVar10;
  int ****ppppiVar11;
  ulonglong uVar12;
  int iVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  undefined1 auStack_d8 [32];
  undefined4 local_b8;
  undefined4 local_b0;
  undefined4 local_a8;
  undefined4 local_a0;
  undefined4 local_98;
  undefined4 local_90;
  longlong local_88;
  int ****local_80;
  int ****local_78;
  int ****local_70;
  int local_68 [2];
  longlong local_60;
  longlong local_58;
  int local_50;
  int local_4c;
  undefined1 local_48 [2];
  undefined2 local_46;
  undefined2 local_42;
  ulonglong local_38;
  
  local_38 = DAT_143a8b908 ^ (ulonglong)auStack_d8;
  if (DAT_143aa8520 != 0) {
    return;
  }
  FUN_1406e9170(param_1,local_68,4);
  FUN_1406e9170(param_1,&local_50,4);
  local_4c = local_50;
  if (local_68[0] == 0) {
    FUN_1406e9170(param_1,local_68,4);
    lVar8 = 0;
    if (local_50 == 1) {
      local_78 = (int ****)FUN_14019b780(&DAT_143ad68a0,0x1678);
      if (local_78 != (int ****)0x0) {
        lVar8 = FUN_142146d90(local_78);
      }
LAB_141c3df98:
      if (lVar8 != 0) goto LAB_141c3dfeb;
    }
    else {
      if (local_50 == 3) {
        local_78 = (int ****)FUN_14019b780(&DAT_143ad68a0,0x1b40);
        if (local_78 != (int ****)0x0) {
          lVar8 = FUN_141e93230(local_78);
        }
        goto LAB_141c3df98;
      }
      if (local_50 == 4) {
        local_78 = (int ****)FUN_14019b780(&DAT_143ad68a0,0x1860);
        if (local_78 != (int ****)0x0) {
          lVar8 = FUN_141c145f0(local_78);
        }
        goto LAB_141c3df98;
      }
    }
    local_78 = (int ****)FUN_1418039d0(0x21000003);
    puVar6 = (undefined8 *)FUN_141c428f0(&local_88,&local_78,&local_4c);
    FUN_1418049f0(&DAT_143271f04,0xdf,0x21000003,*puVar6);
    if (local_88 != 0) {
      FUN_14019f2c0(local_88 + -0x10);
    }
LAB_141c3dfeb:
    *(int *)(lVar8 + 0x304) = local_50;
    *(int *)(lVar8 + 0x308) = local_68[0];
    FUN_141c3ed00(lVar8,param_1);
    plVar2 = DAT_143aa84a0;
    if (DAT_143aa84a0 == (longlong *)0x0) {
      return;
    }
    if (DAT_143aa8518 == 0) {
      return;
    }
    iVar5 = FUN_14279c310();
    if (iVar5 == 0) {
      return;
    }
    uVar9 = FUN_1408a9e40(&local_78,0xdb8);
    FUN_1415eca30(uVar9,0xb);
    if (local_78 != (int ****)0x0) {
      FUN_14019f2c0(local_78 + -2);
    }
    FUN_142d50150(plVar2,0,0,1);
    return;
  }
  uVar12 = 0;
  ppppiVar10 = (int ****)0x0;
  local_80 = (int ****)0x0;
  switch(local_68[0]) {
  case 6:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_58,0x12d6);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    local_88 = local_58;
    break;
  default:
    goto switchD_141c3da16_caseD_7;
  case 8:
    local_60 = DAT_143aa84a0[0x708];
    local_60 = FUN_1408f63b0(&local_60,1);
    (*DAT_1432625b0)(&local_60,local_48);
    local_58 = 0;
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0xbb);
    uVar9 = FUN_14019ba10(&local_58,*puVar6,local_46,local_42);
    FUN_14019a260(&local_80,uVar9);
    if (local_88 != 0) {
      FUN_14019f2c0(local_88 + -0x10);
    }
    ppppiVar10 = local_80;
    if (local_58 != 0) {
      FUN_14019f2c0(local_58 + -0x10);
      ppppiVar10 = local_80;
    }
    goto LAB_141c3dcde;
  case 0xb:
    uVar4 = (**(code **)(*DAT_143aa84a0 + 0xa8))();
    uVar9 = FUN_14025b440(7,uVar4);
    FUN_14019a260(&local_80,uVar9);
    ppppiVar10 = local_80;
    goto LAB_141c3dcde;
  case 0xd:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1786);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0xe:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0xa7);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0xf:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x177f);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x11:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x175f);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x12:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1784);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x13:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1785);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x14:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x205);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x15:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1783);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x16:
  case 0x1c:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1782);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x19:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x177e);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x1a:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1780);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x1b:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1781);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x1e:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1e0);
    local_80 = (int ****)*puVar6;
    *puVar6 = 0;
    break;
  case 0x1f:
    if ((local_50 == 3) || (local_50 == 4)) {
      puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1c6);
      ppppiVar10 = (int ****)*puVar6;
      *puVar6 = 0;
    }
    else {
      puVar6 = (undefined8 *)FUN_1408a9e40(&local_88,0x1c5);
      ppppiVar10 = (int ****)*puVar6;
      *puVar6 = 0;
    }
    local_80 = ppppiVar10;
    if (local_88 != 0) {
      FUN_14019f2c0(local_88 + -0x10);
    }
    goto LAB_141c3dcde;
  }
  ppppiVar10 = local_80;
  if (local_88 != 0) {
    FUN_14019f2c0(local_88 + -0x10);
  }
LAB_141c3dcde:
  if ((ppppiVar10 == (int ****)0x0) || (*(char *)ppppiVar10 == '\0')) {
switchD_141c3da16_caseD_7:
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_78,0x1787);
    if (ppppiVar10 != (int ****)0x0) {
      FUN_14019f2c0(ppppiVar10 + -2);
    }
    ppppiVar10 = (int ****)*puVar6;
    *puVar6 = 0;
    local_80 = ppppiVar10;
    if (local_78 != (int ****)0x0) {
      FUN_14019f2c0(local_78 + -2);
    }
  }
  ppppiVar11 = ppppiVar10;
  if ((ppppiVar10 == (int ****)0x0) || (*(char *)ppppiVar10 == '\0')) goto LAB_141c3deec;
  local_70 = (int ****)0x0;
  ppppiVar1 = ppppiVar10 + -2;
  ppppiVar3 = local_70;
  if (ppppiVar1 != (int ****)0x0) {
    if (*(int *)ppppiVar1 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      uVar15 = 0xffffffffffffffff;
      uVar14 = 0xffffffffffffffff;
      do {
        uVar14 = uVar14 + 1;
      } while (*(char *)((longlong)ppppiVar10 + uVar14) != '\0');
      iVar13 = (int)uVar14;
      iVar5 = 0;
      if (0 < iVar13) {
        iVar5 = iVar13;
      }
      piVar7 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar5 + 0x11));
      piVar7[1] = iVar5;
      *piVar7 = -1;
      ppppiVar3 = (int ****)(piVar7 + 4);
      piVar7[2] = 0;
      *(char *)ppppiVar3 = '\0';
      local_78 = ppppiVar3;
      FUN_142ef7ba0(ppppiVar3,ppppiVar10,(longlong)iVar13);
      if (*piVar7 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((iVar13 == -1) || (iVar13 <= piVar7[1])) {
        *piVar7 = 1;
        if (iVar13 != -1) goto LAB_141c3dded;
        if (ppppiVar3 != (int ****)0x0) {
          do {
            uVar15 = uVar15 + 1;
          } while (*(char *)((longlong)ppppiVar3 + uVar15) != '\0');
          uVar12 = uVar15 & 0xffffffff;
        }
      }
      else {
        FUN_142e54290(0x90,piVar7[1],uVar14 & 0xffffffff);
        *piVar7 = 1;
LAB_141c3dded:
        *(char *)((longlong)ppppiVar3 + (longlong)iVar13) = '\0';
        uVar12 = uVar14;
      }
      iVar5 = (int)uVar12;
      if ((iVar5 < 0) || (piVar7[1] + 1 <= iVar5)) {
        FUN_142e54290(0x9c,uVar12 & 0xffffffff);
      }
      piVar7[2] = iVar5;
      if (local_70 != (int ****)0x0) {
        FUN_14019f2c0(local_70 + -2);
      }
    }
    else {
      if (*(int *)ppppiVar1 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *(int *)ppppiVar1 = *(int *)ppppiVar1 + 1;
      UNLOCK();
      ppppiVar11 = local_80;
      ppppiVar3 = ppppiVar10;
      if (local_70 != (int ****)0x0) {
        FUN_14019f2c0(local_70 + -2);
        ppppiVar11 = local_80;
      }
    }
  }
  local_70 = ppppiVar3;
  local_78 = (int ****)&local_70;
  local_60 = 0;
  FUN_14019a260(&local_60,&local_70);
  local_90 = 0;
  local_98 = 0;
  local_a0 = 0;
  local_a8 = 0;
  local_b0 = 0;
  local_b8 = 0;
  FUN_142a26280(&local_60,0,0,1);
  if (local_70 != (int ****)0x0) {
    FUN_14019f2c0(local_70 + -2);
  }
LAB_141c3deec:
  if (ppppiVar11 != (int ****)0x0) {
    FUN_14019f2c0(ppppiVar11 + -2);
  }
  return;
}



//===========================================================
// FUN_141e9a5e0 @ 141e9a5e0   (2170 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000141e9ad85) */
/* WARNING: Removing unreachable block (ram,0x000141e9a736) */

void FUN_141e9a5e0(longlong *param_1,int param_2,longlong param_3,int param_4)

{
  int *piVar1;
  int *piVar2;
  int iVar3;
  int *piVar4;
  undefined4 *puVar5;
  undefined8 *puVar6;
  longlong *plVar7;
  int *piVar8;
  int *piVar9;
  int *piVar10;
  ulonglong uVar11;
  uint uVar12;
  uint uVar13;
  int iVar14;
  ulonglong uVar15;
  undefined8 in_stack_ffffffffffffff88;
  int *local_48;
  int *local_40;
  int *local_38;
  int *local_30;
  
  uVar13 = (uint)((ulonglong)in_stack_ffffffffffffff88 >> 0x20);
  piVar9 = (int *)0x0;
  uVar12 = 0;
  piVar8 = (int *)0x0;
  local_38 = (int *)0x0;
  local_48 = (int *)0x0;
  iVar3 = FUN_141c3f8b0();
  if (param_2 != iVar3) {
    FUN_14019a260(&local_38,param_3 + 8);
    piVar9 = local_38;
    local_40 = (int *)0x0;
    uVar11 = 0xffffffffffffffff;
    piVar8 = local_40;
    if ((local_38 != (int *)0x0) && (piVar4 = local_38 + -4, piVar4 != (int *)0x0)) {
      if (*piVar4 == -1) {
        FUN_142e52d50(0xcb,0xffffff01);
        uVar15 = uVar11;
        do {
          uVar15 = uVar15 + 1;
        } while (*(char *)((longlong)piVar9 + uVar15) != '\0');
        iVar14 = (int)uVar15;
        iVar3 = 0;
        if (0 < iVar14) {
          iVar3 = iVar14;
        }
        piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
        piVar4[1] = iVar3;
        *piVar4 = -1;
        piVar8 = piVar4 + 4;
        piVar4[2] = 0;
        *(undefined1 *)piVar8 = 0;
        local_30 = piVar8;
        FUN_142ef7ba0(piVar8,piVar9,(longlong)iVar14);
        if (*piVar4 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar14 == -1) || (iVar14 <= piVar4[1])) {
          *piVar4 = 1;
          if (iVar14 != -1) goto LAB_141e9a6ed;
          uVar15 = uVar11;
          if (piVar8 != (int *)0x0) {
            do {
              uVar15 = uVar15 + 1;
            } while (*(char *)((longlong)piVar8 + uVar15) != '\0');
            goto LAB_141e9a6f2;
          }
          iVar3 = 0;
        }
        else {
          FUN_142e54290(0x90,piVar4[1],uVar15 & 0xffffffff);
          *piVar4 = 1;
LAB_141e9a6ed:
          *(undefined1 *)((longlong)iVar14 + (longlong)piVar8) = 0;
LAB_141e9a6f2:
          iVar3 = (int)uVar15;
        }
        if ((iVar3 < 0) || (piVar4[1] + 1 <= iVar3)) {
          FUN_142e54290(0x9c,iVar3);
        }
        piVar4[2] = iVar3;
        if (local_40 != (int *)0x0) {
          FUN_14019f2c0(local_40 + -4);
        }
      }
      else {
        if (*piVar4 < 1) {
          FUN_142e52dd0(0xd2);
        }
        LOCK();
        *piVar4 = *piVar4 + 1;
        UNLOCK();
        if (local_40 != (int *)0x0) {
          FUN_14019f2c0(local_40 + -4);
        }
        local_40 = piVar9;
        piVar9 = local_38;
        piVar8 = local_40;
      }
    }
    local_40 = piVar8;
    piVar4 = (int *)0x0;
    FUN_141c414f0(param_1,&local_48,4,&local_40);
    piVar8 = local_48;
    local_40 = (int *)0x0;
    if (local_48 != (int *)0x0) {
      iVar3 = (*DAT_1432627f8)(0xfde9,0,local_48,0xffffffff,0,0);
      local_30 = (int *)((ulonglong)(longlong)(iVar3 * 2) >> 1);
      uVar12 = (int)local_30 - 1;
      piVar10 = piVar4;
      if ((local_40 == (int *)0x0) || (piVar10 = local_40 + -4, piVar10 == (int *)0x0)) {
LAB_141e9a82b:
        uVar13 = (uint)piVar4;
        if ((int)(uint)piVar4 < (int)uVar12) {
          uVar13 = uVar12;
        }
        puVar5 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(int)(uVar13 * 2 + 0x12));
        puVar5[1] = uVar13;
        *puVar5 = 0xffffffff;
        local_40 = puVar5 + 4;
        puVar5[2] = 0;
        *(undefined2 *)local_40 = 0;
        if (piVar10 != (int *)0x0) {
          FUN_1401bebb0(piVar10);
        }
      }
      else {
        if ((1 < *piVar10) || (local_40[-3] < (int)uVar12)) {
          piVar4 = (int *)((ulonglong)(longlong)local_40[-2] >> 1);
          goto LAB_141e9a82b;
        }
        if (*piVar10 != 1) {
          FUN_142e52dd0(0x74);
        }
        *piVar10 = -1;
      }
      (*DAT_1432627f8)(0xfde9,0,piVar8,0xffffffff,local_40,(int)local_30);
      piVar8 = local_40;
      if (local_40[-4] != -1) {
        FUN_142e52dd0(0x8b);
      }
      if ((uVar12 != 0xffffffff) && (piVar8[-3] < (int)uVar12)) {
        FUN_142e54290(0x90,piVar8[-3],(ulonglong)uVar12);
      }
      piVar8[-4] = 1;
      if (uVar12 == 0xffffffff) {
        uVar15 = 0;
        if (piVar8 != (int *)0x0) {
          do {
            uVar11 = uVar11 + 1;
            uVar15 = uVar11;
          } while (*(short *)((longlong)piVar8 + uVar11 * 2) != 0);
        }
      }
      else {
        *(undefined2 *)((longlong)local_40 + (longlong)(int)uVar12 * 2) = 0;
        uVar15 = (ulonglong)uVar12;
      }
      iVar3 = (int)uVar15;
      if ((iVar3 < 0) || (piVar8[-3] + 1 <= iVar3)) {
        FUN_142e54290(0x9c,uVar15 & 0xffffffff);
      }
      piVar8[-2] = iVar3 * 2;
    }
    FUN_141c3fdd0(param_1,&local_40,2);
    plVar7 = (longlong *)param_1[0xa8];
    if (plVar7 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
      plVar7 = (longlong *)param_1[0xa8];
    }
    (**(code **)(*plVar7 + 0x98))(plVar7,0,0);
    if ((longlong *)param_1[0xb9] != (longlong *)0x0) {
      (**(code **)(*(longlong *)param_1[0xb9] + 0x10))();
      param_1[0xb9] = 0;
    }
    if ((longlong *)param_1[0xba] != (longlong *)0x0) {
      (**(code **)(*(longlong *)param_1[0xba] + 0x10))();
      param_1[0xba] = 0;
    }
    if ((longlong *)param_1[0xbd] != (longlong *)0x0) {
      (**(code **)(*(longlong *)param_1[0xbd] + 0x10))();
      param_1[0xbd] = 0;
    }
    *(undefined4 *)(param_1 + 0x361) = 0;
    (**(code **)(*param_1 + 0x90))(param_1,0);
    goto LAB_141e9ae1a;
  }
  if ((param_4 - 1U & 0xfffffffb) == 0) {
    puVar6 = (undefined8 *)FUN_1408a9e40(&local_30,0x1eb);
    if (local_48 != (int *)0x0) {
      FUN_14019f2c0(local_48 + -4);
    }
    local_48 = (int *)*puVar6;
    *puVar6 = 0;
LAB_141e9ac43:
    if (local_30 != (int *)0x0) {
      FUN_14019f2c0(local_30 + -4);
    }
  }
  else {
    if (param_4 == 6) {
      puVar6 = (undefined8 *)FUN_1408a9e40(&local_30,0x1e8);
      if (local_48 != (int *)0x0) {
        FUN_14019f2c0(local_48 + -4);
      }
      local_48 = (int *)*puVar6;
      *puVar6 = 0;
      goto LAB_141e9ac43;
    }
    if (param_4 == 4) {
      puVar6 = (undefined8 *)FUN_1408a9e40(&local_30,0x1f2);
      if (local_48 != (int *)0x0) {
        FUN_14019f2c0(local_48 + -4);
      }
      local_48 = (int *)*puVar6;
      *puVar6 = 0;
      goto LAB_141e9ac43;
    }
    if (param_4 == 3) {
      local_40 = (int *)0x0;
      FUN_141c414f0(param_1,&local_48,0x67,&local_40);
      local_30 = local_48;
      local_40 = (int *)0x0;
      if (local_48 != (int *)0x0) {
        uVar11 = 0xffffffffffffffff;
        iVar3 = (*DAT_1432627f8)(0xfde9,0,local_48,0xffffffff,0,0);
        iVar3 = (int)((ulonglong)(longlong)(iVar3 * 2) >> 1);
        uVar13 = iVar3 - 1;
        if (local_40 == (int *)0x0) {
          piVar8 = (int *)0x0;
LAB_141e9aaf9:
          if ((int)uVar12 < (int)uVar13) {
            uVar12 = uVar13;
          }
          puVar5 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(int)(uVar12 * 2 + 0x12));
          puVar5[1] = uVar12;
          *puVar5 = 0xffffffff;
          local_40 = puVar5 + 4;
          puVar5[2] = 0;
          *(undefined2 *)local_40 = 0;
          if (piVar8 != (int *)0x0) {
            FUN_1401bebb0(piVar8);
          }
        }
        else {
          piVar8 = local_40 + -4;
          if (piVar8 == (int *)0x0) goto LAB_141e9aaf9;
          if ((1 < *piVar8) || (local_40[-3] < (int)uVar13)) {
            uVar12 = (uint)((ulonglong)(longlong)local_40[-2] >> 1);
            goto LAB_141e9aaf9;
          }
          if (*piVar8 != 1) {
            FUN_142e52dd0(0x74);
          }
          *piVar8 = -1;
        }
        (*DAT_1432627f8)(0xfde9,0,local_30,0xffffffff,local_40,iVar3);
        piVar8 = local_40;
        if (local_40[-4] != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((uVar13 != 0xffffffff) && (piVar8[-3] < (int)uVar13)) {
          FUN_142e54290(0x90,piVar8[-3],(ulonglong)uVar13);
        }
        piVar8[-4] = 1;
        if (uVar13 == 0xffffffff) {
          uVar15 = 0;
          if (piVar8 != (int *)0x0) {
            do {
              uVar11 = uVar11 + 1;
              uVar15 = uVar11;
            } while (*(short *)((longlong)piVar8 + uVar11 * 2) != 0);
          }
        }
        else {
          *(undefined2 *)((longlong)local_40 + (longlong)(int)uVar13 * 2) = 0;
          uVar15 = (ulonglong)uVar13;
        }
        iVar3 = (int)uVar15;
        if ((iVar3 < 0) || (piVar8[-3] + 1 <= iVar3)) {
          FUN_142e54290(0x9c,uVar15 & 0xffffffff);
        }
        piVar8[-2] = iVar3 * 2;
      }
      FUN_141c3fdd0(param_1,&local_40,2);
      iVar3 = FUN_1429e3ef0();
      *(int *)((longlong)param_1 + 0x1b2c) = iVar3 + 10000;
      goto LAB_141e9ae1a;
    }
  }
  (**(code **)(*param_1 + 0x138))(param_1,8);
  piVar4 = local_48;
  local_40 = (int *)0x0;
  piVar10 = piVar9;
  piVar2 = local_40;
  if ((local_48 != (int *)0x0) && (piVar1 = local_48 + -4, piVar10 = piVar8, piVar1 != (int *)0x0))
  {
    if (*piVar1 == -1) {
      FUN_142e52d50(0xcb,0xffffff01);
      piVar8 = local_48;
      local_30 = (int *)0x0;
      if (local_48 != (int *)0x0) {
        uVar11 = 0xffffffffffffffff;
        uVar15 = 0xffffffffffffffff;
        do {
          uVar15 = uVar15 + 1;
        } while (*(char *)((longlong)local_48 + uVar15) != '\0');
        iVar14 = (int)uVar15;
        iVar3 = 0;
        if (0 < iVar14) {
          iVar3 = iVar14;
        }
        piVar4 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar3 + 0x11));
        piVar4[1] = iVar3;
        *piVar4 = -1;
        piVar9 = piVar4 + 4;
        piVar4[2] = 0;
        *(undefined1 *)piVar9 = 0;
        local_30 = piVar9;
        FUN_142ef7ba0(piVar9,piVar8,(longlong)iVar14);
        if (*piVar4 != -1) {
          FUN_142e52dd0(0x8b);
        }
        if ((iVar14 == -1) || (iVar14 <= piVar4[1])) {
          *piVar4 = 1;
          if (iVar14 != -1) goto LAB_141e9ad3c;
          if (piVar9 != (int *)0x0) {
            do {
              uVar11 = uVar11 + 1;
            } while (*(char *)((longlong)piVar9 + uVar11) != '\0');
            uVar15 = uVar11 & 0xffffffff;
            goto LAB_141e9ad41;
          }
          iVar3 = 0;
        }
        else {
          FUN_142e54290(0x90,piVar4[1],uVar15 & 0xffffffff);
          *piVar4 = 1;
LAB_141e9ad3c:
          *(undefined1 *)((longlong)iVar14 + (longlong)piVar9) = 0;
LAB_141e9ad41:
          iVar3 = (int)uVar15;
        }
        if ((iVar3 < 0) || (piVar4[1] + 1 <= iVar3)) {
          FUN_142e54290(0x9c,iVar3);
        }
        piVar4[2] = iVar3;
      }
      piVar2 = piVar9;
      if (local_40 != (int *)0x0) {
        FUN_14019f2c0(local_40 + -4);
      }
    }
    else {
      if (*piVar1 < 1) {
        FUN_142e52dd0(0xd2);
      }
      LOCK();
      *piVar1 = *piVar1 + 1;
      UNLOCK();
      if (local_40 != (int *)0x0) {
        FUN_14019f2c0(local_40 + -4);
      }
      local_40 = piVar4;
      piVar10 = local_38;
      piVar2 = local_40;
    }
  }
  local_40 = piVar2;
  FUN_142a26280(&local_40,0,0,1,(ulonglong)uVar13 << 0x20,0,0,0,0,0);
  piVar9 = piVar10;
LAB_141e9ae1a:
  if (local_48 != (int *)0x0) {
    FUN_14019f2c0(local_48 + -4);
  }
  if (piVar9 != (int *)0x0) {
    FUN_14019f2c0(piVar9 + -4);
  }
  return;
}



//===========================================================
// FUN_141c3ef70 @ 141c3ef70   (627 bytes)
//===========================================================

/* WARNING: Removing unreachable block (ram,0x000141c3f172) */

void FUN_141c3ef70(longlong *param_1,undefined8 param_2)

{
  longlong lVar1;
  char cVar2;
  undefined8 *puVar3;
  longlong *plVar4;
  int local_res18 [2];
  undefined4 local_res20 [2];
  undefined8 local_a8;
  longlong local_a0;
  int local_98 [2];
  longlong local_90;
  undefined4 local_88;
  undefined1 local_80 [8];
  longlong local_78;
  undefined1 local_70 [8];
  longlong local_68;
  undefined4 local_60;
  undefined8 local_58;
  undefined4 local_50;
  longlong local_48 [3];
  undefined8 local_30;
  
  cVar2 = FUN_1406e8ae0(param_2);
  local_res18[0] = (int)cVar2;
  FUN_1406e9170(param_2,local_res20,4);
  if ((-1 < local_res18[0]) && ((ulonglong)(longlong)local_res18[0] < 8)) {
    plVar4 = param_1 + (longlong)local_res18[0] * 7 + 0x62;
    local_98[0] = (int)*plVar4;
    local_90 = 0;
    FUN_14019a260(&local_90,plVar4 + 1);
    local_88 = (undefined4)plVar4[2];
    lVar1 = plVar4[4];
    local_78 = lVar1;
    if (lVar1 != 0) {
      if (0xfffff < *(ulonglong *)(lVar1 + -0x20)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar1 + -0x20) = *(longlong *)(lVar1 + -0x20) + 1;
      UNLOCK();
    }
    lVar1 = plVar4[6];
    local_68 = lVar1;
    if (lVar1 != 0) {
      if (0xfffff < *(ulonglong *)(lVar1 + -0x20)) {
        FUN_142e541f0(0x30f);
      }
      LOCK();
      *(longlong *)(lVar1 + -0x20) = *(longlong *)(lVar1 + -0x20) + 1;
      UNLOCK();
    }
    if (local_98[0] == 0) {
      local_a8 = FUN_1418039d0(0x21000003);
      puVar3 = (undefined8 *)FUN_140ce7310(&local_a0,&local_a8,local_res18);
      FUN_1418049f0(&DAT_143271f04,0x1e8,0x21000003,*puVar3);
      if (local_a0 != 0) {
        FUN_14019f2c0(local_a0 + -0x10);
      }
    }
    local_60 = 0;
    local_58 = 0;
    local_50 = 0;
    local_48[1] = 0;
    local_30 = 0;
    plVar4 = param_1 + (longlong)local_res18[0] * 7 + 0x62;
    *(undefined4 *)plVar4 = 0;
    if (plVar4[1] != 0) {
      FUN_14019f2c0(plVar4[1] + -0x10);
    }
    local_58 = 0;
    plVar4[1] = 0;
    *(undefined4 *)(plVar4 + 2) = 0;
    if ((plVar4[4] - 1U < 999) || (plVar4[4] == -1)) {
      FUN_142e52ed0(0x447);
    }
    if (plVar4 + 3 == local_48) {
      FUN_142e52d50(0x45c,1);
    }
    FUN_140d2d420(plVar4 + 3);
    plVar4[4] = 0;
    if ((plVar4[6] - 1U < 999) || (plVar4[6] == -1)) {
      FUN_142e52ed0(0x447);
    }
    if (plVar4 + 5 == local_48 + 2) {
      FUN_142e52d50(0x45c,1);
    }
    FUN_140301d60(plVar4 + 5);
    plVar4[6] = 0;
    *(int *)(param_1 + 0x60) = (int)param_1[0x60] + -1;
    (**(code **)(*param_1 + 400))(param_1,local_res18[0],local_98,local_res20[0],param_2);
    FUN_140301d60(local_70);
    FUN_140d2d420(local_80);
    if (local_90 != 0) {
      FUN_14019f2c0(local_90 + -0x10);
    }
  }
  return;
}



//===========================================================
// FUN_141e99c40 @ 141e99c40   (1626 bytes)
//===========================================================

/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141e99c40(longlong *param_1,undefined8 param_2)

{
  undefined2 *puVar1;
  code *pcVar2;
  longlong lVar3;
  IUnknown *pIVar4;
  char cVar5;
  byte bVar6;
  int iVar7;
  longlong *plVar8;
  longlong *plVar9;
  undefined8 uVar10;
  longlong lVar11;
  ulonglong uVar12;
  longlong lVar13;
  longlong *plVar14;
  undefined *apuStack_580 [5];
  longlong lStack_558;
  undefined4 auStack_550 [2];
  longlong *plStack_548;
  undefined4 uStack_540;
  undefined4 uStack_53c;
  undefined8 uStack_538;
  undefined8 uStack_530;
  short sStack_528;
  undefined6 uStack_526;
  longlong lStack_520;
  undefined8 uStack_518;
  longlong lStack_510;
  longlong *plStack_508;
  longlong *plStack_500;
  uint uStack_4f8;
  undefined4 uStack_4f4;
  undefined4 uStack_4f0;
  undefined4 uStack_4ec;
  undefined8 uStack_4e8;
  uint uStack_4e0;
  undefined4 uStack_4dc;
  undefined4 uStack_4d8;
  undefined4 uStack_4d4;
  undefined8 uStack_4d0;
  undefined8 uStack_4c8;
  longlong lStack_4c0;
  undefined8 uStack_4b8;
  uint uStack_4a8;
  undefined4 uStack_4a4;
  undefined4 uStack_4a0;
  undefined4 uStack_49c;
  undefined8 uStack_498;
  undefined1 auStack_488 [1104];
  ulonglong uStack_38;
  
  uStack_38 = DAT_143a8b908 ^ (ulonglong)&plStack_548;
  apuStack_580[0] = (undefined *)0x141e99c7f;
  iVar7 = FUN_141c3f8b0();
  plVar14 = (longlong *)0x0;
  if (iVar7 == 0) {
    plVar8 = (longlong *)param_1[0xa4];
    if (plVar8 == (longlong *)0x0) {
      apuStack_580[0] = (undefined *)0x141e99ca2;
      FUN_142e52ed0(0x431,0);
      plVar8 = (longlong *)param_1[0xa4];
    }
    apuStack_580[0] = (undefined *)0x141e99caf;
    (**(code **)(*plVar8 + 0x18))();
    apuStack_580[0] = (undefined *)0x141e99cbb;
    FUN_1411d4600(param_1 + 0xa3);
    lVar11 = param_1[0xa2];
    if (lVar11 == 0) {
      apuStack_580[0] = (undefined *)0x141e99cd3;
      FUN_142e52ed0(0x431,0);
      lVar11 = param_1[0xa2];
    }
    apuStack_580[0] = (undefined *)0x141e99ce6;
    (**(code **)(*(longlong *)(lVar11 + 8) + 0x70))((longlong *)(lVar11 + 8),0);
    apuStack_580[0] = (undefined *)0x141e99cf7;
    FUN_1406ed520(auStack_488,0x17e);
    lStack_510 = CONCAT44(lStack_510._4_4_,10);
    apuStack_580[0] = (undefined *)0x141e99d15;
    FUN_1406ede20(auStack_488,&lStack_510,4);
    apuStack_580[0] = (undefined *)0x141e99d23;
    FUN_1406ed840(auStack_488,1);
    apuStack_580[0] = (undefined *)0x141e99d2f;
    FUN_1415d01c0(auStack_488);
    apuStack_580[0] = (undefined *)0x141e99d3c;
    FUN_1406ed610(auStack_488);
  }
  else {
    plVar8 = (longlong *)param_1[0xa2];
    if (plVar8 == (longlong *)0x0) {
      apuStack_580[0] = (undefined *)0x141e99d56;
      FUN_142e52ed0(0x431,0);
      plVar8 = (longlong *)param_1[0xa2];
    }
    apuStack_580[0] = (undefined *)0x141e99d63;
    (**(code **)(*plVar8 + 0x18))();
    apuStack_580[0] = (undefined *)0x141e99d6f;
    FUN_1411d4600(param_1 + 0xa1);
    *(undefined4 *)(param_1 + 0x361) = 0;
    plVar8 = (longlong *)param_1[0xa8];
    if (plVar8 == (longlong *)0x0) {
      apuStack_580[0] = (undefined *)0x141e99d8e;
      FUN_142e52ed0(0x431,0);
      plVar8 = (longlong *)param_1[0xa8];
    }
    apuStack_580[0] = (undefined *)0x141e99d9b;
    (**(code **)(*plVar8 + 0x18))();
    apuStack_580[0] = (undefined *)0x141e99da0;
    iVar7 = FUN_1429e3ef0();
    *(int *)((longlong)param_1 + 0x1b2c) = iVar7 + 3000;
  }
  apuStack_580[0] = (undefined *)0x141e99db3;
  cVar5 = FUN_1406e8ae0(param_2);
  while (-1 < cVar5) {
    uVar12 = (ulonglong)(uint)(int)cVar5;
    apuStack_580[0] = (undefined *)0x141e99dd5;
    FUN_141c30d00(param_1 + uVar12 * 7 + 0x67);
    lVar11 = param_1[uVar12 * 7 + 0x68];
    if (lVar11 == 0) {
      apuStack_580[0] = (undefined *)0x141e99dee;
      FUN_142e52ed0(0x431,0);
      lVar11 = param_1[uVar12 * 7 + 0x68];
    }
    apuStack_580[0] = (undefined *)0x141e99dfe;
    FUN_1402d1b50(lVar11);
    apuStack_580[0] = (undefined *)0x141e99e06;
    cVar5 = FUN_1406e8ae0(param_2);
  }
  apuStack_580[0] = (undefined *)0x141e99e19;
  plVar8 = (longlong *)FUN_1406e9050(param_2,&plStack_548);
  if (param_1[0x362] != 0) {
    apuStack_580[0] = (undefined *)0x141e99e31;
    FUN_14019f2c0(param_1[0x362] + -0x10);
    param_1[0x362] = 0;
  }
  param_1[0x362] = *plVar8;
  *plVar8 = 0;
  if (plStack_548 != (longlong *)0x0) {
    apuStack_580[0] = (undefined *)0x141e99e5a;
    FUN_14019f2c0(plStack_548 + -2);
  }
  apuStack_580[0] = (undefined *)0x141e99e63;
  bVar6 = FUN_1406e8ae0(param_2);
  *(uint *)(param_1 + 0x365) = (uint)bVar6;
  lStack_510 = 0;
  apuStack_580[0] = (undefined *)0x141e99e82;
  FUN_14019ba10(&lStack_510,PTR_s_UI_Minigame_img_Omok_stone__d_143a44358);
  pIVar4 = DAT_143add058;
  if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
    apuStack_580[0] = &UNK_141e9a28c;
    FUN_142ef3ac0(0x80004003);
  }
  apuStack_580[0] = (undefined *)0x141e99e9c;
  (*DAT_143262a20)(&sStack_528);
  if (DAT_143a8b8d8 == 8) {
    if (sStack_528 == 8) {
      sStack_528 = 0;
      if (lStack_520 != 0) {
        apuStack_580[0] = (undefined *)0x141e99ecc;
        (*DAT_143ad5990)(lStack_520 + -4);
      }
    }
    else {
      apuStack_580[0] = (undefined *)0x141e99ed8;
      iVar7 = (*DAT_143262a18)(&sStack_528);
      if (iVar7 < 0) goto LAB_141e9a28d;
    }
    sStack_528 = 8;
    plVar8 = plVar14;
    if (DAT_143a8b8e0 != 0) {
      plVar8 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    apuStack_580[0] = (undefined *)0x141e99eff;
    lStack_520 = FUN_1401a5fa0(DAT_143a8b8e0,plVar8);
  }
  else {
    if ((sStack_528 == 8) && (sStack_528 = 0, lStack_520 != 0)) {
      apuStack_580[0] = (undefined *)0x141e99f5a;
      (*DAT_143ad5990)(lStack_520 + -4);
    }
    apuStack_580[0] = (undefined *)0x141e99f6b;
    iVar7 = (*DAT_143262a28)(&sStack_528,&DAT_143a8b8d8);
    if (iVar7 < 0) {
LAB_141e9a28d:
                    /* WARNING: Subroutine does not return */
      apuStack_580[0] = &UNK_141e9a294;
      FUN_142ef3ac0(iVar7);
    }
  }
  apuStack_580[0] = (undefined *)0x141e99f0d;
  (*DAT_143262a20)(&uStack_540);
  if (DAT_143a8b8d8 == 8) {
    if ((short)uStack_540 == 8) {
      uStack_540 = (uint)uStack_540._2_2_ << 0x10;
      if (uStack_538 != 0) {
        apuStack_580[0] = (undefined *)0x141e99f3a;
        (*DAT_143ad5990)(uStack_538 + -4);
      }
    }
    else {
      apuStack_580[0] = (undefined *)0x141e99f7f;
      iVar7 = (*DAT_143262a18)(&uStack_540);
      if (iVar7 < 0) goto LAB_141e9a295;
    }
    uStack_540 = CONCAT22(uStack_540._2_2_,8);
    plVar8 = plVar14;
    if (DAT_143a8b8e0 != 0) {
      plVar8 = (longlong *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
    }
    apuStack_580[0] = (undefined *)0x141e99fa6;
    uStack_538 = FUN_1401a5fa0(DAT_143a8b8e0,plVar8);
  }
  else {
    if (((short)uStack_540 == 8) && (uStack_540 = (uint)uStack_540._2_2_ << 0x10, uStack_538 != 0))
    {
      apuStack_580[0] = (undefined *)0x141e99fd7;
      (*DAT_143ad5990)(uStack_538 + -4);
    }
    apuStack_580[0] = (undefined *)0x141e99fe8;
    iVar7 = (*DAT_143262a28)(&uStack_540,&DAT_143a8b8d8);
    if (iVar7 < 0) {
LAB_141e9a295:
                    /* WARNING: Subroutine does not return */
      apuStack_580[0] = &UNK_141e9a29c;
      FUN_142ef3ac0(iVar7);
    }
  }
  lVar11 = lStack_510;
  if (lStack_510 == 0) {
    iVar7 = 2;
  }
  else {
    auStack_550[0] = 0;
    lStack_558 = 0;
    apuStack_580[0] = (undefined *)0x141e9a012;
    iVar7 = (*DAT_1432627f8)(0xfde9,0,lStack_510,0xffffffff);
    iVar7 = iVar7 * 2;
  }
  uVar12 = (longlong)iVar7 + 0xf;
  if (uVar12 <= (ulonglong)(longlong)iVar7) {
    uVar12 = 0xffffffffffffff0;
  }
  apuStack_580[0] = (undefined *)0x141e9a035;
  lVar3 = -(uVar12 & 0xfffffffffffffff0);
  puVar1 = (undefined2 *)((longlong)&plStack_548 + lVar3);
  if (lVar11 == 0) {
    if (puVar1 != (undefined2 *)0x0) {
      *puVar1 = 0;
    }
  }
  else {
    *(undefined4 *)((longlong)auStack_550 + lVar3) = 0x100000;
    *(undefined2 **)((longlong)auStack_550 + lVar3 + -8) = puVar1;
    *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a071;
    (*DAT_1432627f8)(0xfde9,0,lVar11,0xffffffff);
  }
  *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a07d;
  plVar9 = (longlong *)FUN_1401a5890(&plStack_548,puVar1);
  plStack_508 = plVar9;
  *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a08e;
  (*DAT_143262a20)(&uStack_4e0);
  pcVar2 = *(code **)(*(longlong *)pIVar4 + 0x48);
  plVar8 = plVar14;
  if ((undefined8 *)*plVar9 != (undefined8 *)0x0) {
    plVar8 = *(longlong **)*plVar9;
  }
  uStack_4c8 = CONCAT62(uStack_526,sStack_528);
  lStack_4c0 = lStack_520;
  uStack_4b8 = uStack_518;
  uStack_4a8 = uStack_540;
  uStack_4a4 = uStack_53c;
  uStack_4a0 = (undefined4)uStack_538;
  uStack_49c = uStack_538._4_4_;
  uStack_498 = uStack_530;
  *(uint **)((longlong)auStack_550 + lVar3 + -8) = &uStack_4e0;
  *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a0f1;
  iVar7 = (*pcVar2)(pIVar4,plVar8,&uStack_4a8,&uStack_4c8);
  if (iVar7 < 0) {
    *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a106;
    _com_issue_errorex(iVar7,pIVar4,(_GUID *)&DAT_1432743e8);
  }
  uStack_4f8 = uStack_4e0;
  uStack_4f4 = uStack_4dc;
  uStack_4f0 = uStack_4d8;
  uStack_4ec = uStack_4d4;
  uStack_4e8 = uStack_4d0;
  uStack_4e0 = uStack_4e0 & 0xffff0000;
  *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a125;
  FUN_1401be120(plVar9);
  *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a133;
  uVar10 = FUN_1409339d0(&plStack_500,&uStack_4f8);
  *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a140;
  FUN_1401a5040(&plStack_508,uVar10);
  plVar8 = (longlong *)param_1[0xb1];
  plVar9 = plStack_508;
  if (plVar8 != plStack_508) {
    param_1[0xb1] = (longlong)plStack_508;
    plVar9 = plVar14;
    if (plVar8 != (longlong *)0x0) {
      pcVar2 = *(code **)(*plVar8 + 0x10);
      *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a166;
      (*pcVar2)();
      plVar9 = (longlong *)0x0;
    }
  }
  if (plVar9 != (longlong *)0x0) {
    pcVar2 = *(code **)(*plVar9 + 0x10);
    *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a175;
    (*pcVar2)(plVar9);
  }
  if (plStack_500 != (longlong *)0x0) {
    pcVar2 = *(code **)(*plStack_500 + 0x10);
    *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a185;
    (*pcVar2)();
  }
  if ((short)uStack_4f8 == 8) {
    uStack_4f8 = uStack_4f8 & 0xffff0000;
    lVar13 = CONCAT44(uStack_4ec,uStack_4f0);
    if (lVar13 != 0) {
      *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a1a5;
      (*DAT_143ad5990)(lVar13 + -4);
    }
  }
  else {
    *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a1b1;
    (*DAT_143262a18)(&uStack_4f8);
  }
  if ((short)uStack_540 == 8) {
    uStack_540 = uStack_540 & 0xffff0000;
    if (uStack_538 != 0) {
      lVar13 = uStack_538 + -4;
      *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a1d1;
      (*DAT_143ad5990)(lVar13);
    }
  }
  else {
    *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a1dd;
    (*DAT_143262a18)(&uStack_540);
  }
  if (sStack_528 == 8) {
    sStack_528 = 0;
    if (lStack_520 != 0) {
      lVar13 = lStack_520 + -4;
      *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a1fd;
      (*DAT_143ad5990)(lVar13);
    }
  }
  else {
    *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a209;
    (*DAT_143262a18)(&sStack_528);
  }
  pcVar2 = *(code **)(*param_1 + 0x90);
  *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a218;
  (*pcVar2)(param_1,0);
  plStack_548 = (longlong *)param_1[0xc4];
  if (plStack_548 != (longlong *)0x0) {
    pcVar2 = *(code **)(*plStack_548 + 8);
    *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a22e;
    (*pcVar2)();
  }
  *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a245;
  FUN_1429eb300(param_1 + 0x362,&plStack_548,0xb8);
  if (lVar11 != 0) {
    *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a254;
    FUN_14019f2c0(lVar11 + -0x10);
  }
  *(undefined8 *)((longlong)apuStack_580 + lVar3) = 0x141e9a264;
  return;
}



//===========================================================
// FUN_141e9a2b0 @ 141e9a2b0   (802 bytes)
//===========================================================

void FUN_141e9a2b0(longlong *param_1,int param_2,undefined8 param_3)

{
  undefined8 uVar1;
  int iVar2;
  longlong lVar3;
  undefined8 *puVar4;
  undefined4 *puVar5;
  longlong lVar6;
  longlong *plVar7;
  uint uVar8;
  int *piVar9;
  uint uVar11;
  undefined4 *local_res8;
  longlong local_res20;
  longlong local_48 [2];
  int *piVar10;
  
  lVar3 = (longlong)param_2;
  FUN_141c30d00(param_1 + lVar3 * 7 + 0x67);
  lVar6 = param_1[lVar3 * 7 + 0x68];
  if (lVar6 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar6 = param_1[lVar3 * 7 + 0x68];
  }
  FUN_1402d1b50(lVar6,param_3);
  piVar9 = (int *)0x0;
  uVar11 = 0;
  *(undefined4 *)(param_1 + 0x361) = 0;
  plVar7 = (longlong *)param_1[0xa8];
  if (plVar7 == (longlong *)0x0) {
    FUN_142e52ed0(0x431,0);
    plVar7 = (longlong *)param_1[0xa8];
  }
  (**(code **)(*plVar7 + 0x98))(plVar7,1);
  (**(code **)(*param_1 + 0x90))(param_1,0);
  local_res20 = 0;
  puVar4 = (undefined8 *)FUN_141c3f9f0(param_1,local_48,1);
  uVar1 = *puVar4;
  puVar4 = (undefined8 *)FUN_1408a9e40(&local_res8,0x1e3);
  FUN_14019ba10(&local_res20,*puVar4,uVar1);
  if (local_res8 != (undefined4 *)0x0) {
    FUN_14019f2c0(local_res8 + -4);
  }
  if (local_48[0] != 0) {
    FUN_14019f2c0(local_48[0] + -0x10);
  }
  lVar6 = local_res20;
  local_res8 = (undefined4 *)0x0;
  if (local_res20 == 0) goto LAB_141e9a547;
  iVar2 = (*DAT_1432627f8)(0xfde9,0,local_res20,0xffffffff,0,0);
  iVar2 = (int)((ulonglong)(longlong)(iVar2 * 2) >> 1);
  uVar8 = iVar2 - 1;
  piVar10 = piVar9;
  if ((local_res8 == (undefined4 *)0x0) || (piVar10 = local_res8 + -4, piVar10 == (int *)0x0)) {
LAB_141e9a450:
    if ((int)uVar11 < (int)uVar8) {
      uVar11 = uVar8;
    }
    puVar5 = (undefined4 *)FUN_1401bc720(&DAT_143ad6980,(longlong)(int)(uVar11 * 2 + 0x12));
    puVar5[1] = uVar11;
    *puVar5 = 0xffffffff;
    local_res8 = puVar5 + 4;
    puVar5[2] = 0;
    *(undefined2 *)local_res8 = 0;
    if (piVar10 != (int *)0x0) {
      FUN_1401bebb0(piVar10);
    }
  }
  else {
    if ((1 < *piVar10) || ((int)local_res8[-3] < (int)uVar8)) {
      uVar11 = (uint)((ulonglong)(longlong)(int)local_res8[-2] >> 1);
      goto LAB_141e9a450;
    }
    if (*piVar10 != 1) {
      FUN_142e52dd0(0x74);
    }
    *piVar10 = -1;
  }
  (*DAT_1432627f8)(0xfde9,0,lVar6,0xffffffff,local_res8,iVar2);
  puVar5 = local_res8;
  if (local_res8[-4] != -1) {
    FUN_142e52dd0(0x8b);
  }
  if ((uVar8 != 0xffffffff) && ((int)puVar5[-3] < (int)uVar8)) {
    FUN_142e54290(0x90,puVar5[-3],(int *)(ulonglong)uVar8);
  }
  puVar5[-4] = 1;
  if (uVar8 == 0xffffffff) {
    piVar10 = (int *)0xffffffffffffffff;
    if (puVar5 != (undefined4 *)0x0) {
      do {
        piVar9 = (int *)((longlong)piVar10 + 1);
        piVar10 = piVar9;
      } while (*(short *)((longlong)puVar5 + (longlong)piVar9 * 2) != 0);
    }
  }
  else {
    *(undefined2 *)((longlong)local_res8 + (longlong)(int)uVar8 * 2) = 0;
    piVar9 = (int *)(ulonglong)uVar8;
  }
  iVar2 = (int)piVar9;
  if ((iVar2 < 0) || (puVar5[-3] + 1 <= iVar2)) {
    FUN_142e54290(0x9c,(ulonglong)piVar9 & 0xffffffff);
  }
  puVar5[-2] = iVar2 * 2;
LAB_141e9a547:
  FUN_141c3fdd0(param_1,&local_res8,2);
  puVar4 = (undefined8 *)FUN_1408a9d20(&local_res8,0x5d2);
  FUN_1429f1f50(*puVar4,100,0);
  if (local_res8 != (undefined4 *)0x0) {
    FUN_1401bebb0(local_res8 + -4);
  }
  iVar2 = FUN_1429e3ef0();
  *(int *)((longlong)param_1 + 0x1b2c) = iVar2 + 3000;
  if (lVar6 != 0) {
    FUN_14019f2c0(lVar6 + -0x10);
  }
  return;
}



//===========================================================
// FUN_141c3d3e0 @ 141c3d3e0   (1021 bytes)
//===========================================================

void FUN_141c3d3e0(undefined8 param_1)

{
  longlong *plVar1;
  char cVar2;
  ushort uVar3;
  int iVar4;
  longlong *plVar5;
  undefined8 *puVar6;
  ulonglong uVar7;
  longlong *plVar8;
  uint local_res10 [2];
  longlong local_res18;
  longlong local_res20;
  
  FUN_1406e9170(param_1,local_res10,4);
  switch(local_res10[0]) {
  case 3:
    if (((DAT_143aa8520 != (longlong *)0x0) &&
        (iVar4 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a88880),
        plVar1 = DAT_143aa8520, iVar4 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
      cVar2 = FUN_1406e8ae0(param_1);
      uVar7 = (ulonglong)cVar2;
      local_res10[0] = (uint)cVar2;
      if ((-1 < cVar2) && (local_res10[0] < 8)) {
        plVar8 = plVar1 + (uVar7 + 0xe) * 7;
        if ((int)*plVar8 != 0) {
          local_res18 = FUN_1418039d0(0x21000003);
          puVar6 = (undefined8 *)FUN_140ce7310(&local_res20,&local_res18,local_res10);
          FUN_1418049f0(&DAT_143271f04,0x1c8,0x21000003,*puVar6);
          if (local_res20 != 0) {
            FUN_14019f2c0(local_res20 + -0x10);
          }
          uVar7 = (ulonglong)local_res10[0];
        }
        (**(code **)(*plVar1 + 0x1c8))(plVar1,uVar7,param_1);
        iVar4 = FUN_1406e8c20(param_1);
        *(int *)plVar8 = iVar4;
        plVar5 = (longlong *)FUN_1406e9050(param_1,&local_res18);
        if (plVar8[1] != 0) {
          FUN_14019f2c0();
          plVar8[1] = 0;
        }
        plVar8[1] = *plVar5;
        *plVar5 = 0;
        if (local_res18 != 0) {
          FUN_14019f2c0(local_res18 + -0x10);
        }
        uVar3 = FUN_1406e8b80(param_1);
        *(uint *)(plVar8 + 2) = (uint)uVar3;
        *(int *)(plVar1 + 0x60) = (int)plVar1[0x60] + 1;
        (**(code **)(*plVar1 + 0x180))(plVar1,local_res10[0],param_1);
        return;
      }
    }
    break;
  case 4:
    FUN_141c3d980(param_1);
    return;
  case 5:
    FUN_141c3e110(param_1);
    return;
  case 6:
    FUN_141c3e360(param_1);
    return;
  default:
    if (((DAT_143aa8520 != (longlong *)0x0) &&
        (iVar4 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a88880),
        iVar4 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
      (**(code **)(*DAT_143aa8520 + 0x178))(DAT_143aa8520,local_res10[0],param_1);
    }
    break;
  case 8:
    if (((DAT_143aa8520 != (longlong *)0x0) &&
        (iVar4 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a88880),
        iVar4 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
                    /* WARNING: Could not recover jumptable at 0x000141c3d60b. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (**(code **)(*DAT_143aa8520 + 0x198))(DAT_143aa8520,param_1);
      return;
    }
    break;
  case 0xb:
    if (((DAT_143aa8520 != (longlong *)0x0) &&
        (iVar4 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a88880),
        plVar1 = DAT_143aa8520, iVar4 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
      cVar2 = FUN_1406e8ae0(param_1);
      uVar7 = (ulonglong)cVar2;
      local_res10[0] = (uint)cVar2;
      if ((-1 < cVar2) && (local_res10[0] < 8)) {
        if ((int)plVar1[(uVar7 + 0xe) * 7] == 0) {
          local_res18 = FUN_1418039d0(0x21000003);
          puVar6 = (undefined8 *)FUN_140ce7310(&local_res20,&local_res18,local_res10);
          FUN_1418049f0(&DAT_143271f04,0x204,0x21000003,*puVar6);
          if (local_res20 != 0) {
            FUN_14019f2c0(local_res20 + -0x10);
          }
          uVar7 = (ulonglong)local_res10[0];
        }
        (**(code **)(*plVar1 + 0x1c8))(plVar1,uVar7,param_1);
        return;
      }
    }
    break;
  case 0xc:
    if (((DAT_143aa8520 != (longlong *)0x0) &&
        (iVar4 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a88880),
        iVar4 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
                    /* WARNING: Could not recover jumptable at 0x000141c3d740. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (**(code **)(*DAT_143aa8520 + 0x168))(DAT_143aa8520,param_1);
      return;
    }
    break;
  case 0xd:
    if (((DAT_143aa8520 != (longlong *)0x0) &&
        (iVar4 = (**(code **)(DAT_143aa8520[1] + 0xd0))(DAT_143aa8520 + 1,&PTR_PTR_143a88880),
        iVar4 != 0)) && (DAT_143aa8520 != (longlong *)0x0)) {
                    /* WARNING: Could not recover jumptable at 0x000141c3d78a. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (**(code **)(*DAT_143aa8520 + 0x1a0))(DAT_143aa8520,param_1);
      return;
    }
  }
  return;
}



//===========================================================
// FUN_141e99700 @ 141e99700   (1265 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141e99700(longlong *param_1,undefined4 param_2,undefined8 param_3)

{
  byte bVar1;
  int iVar2;
  undefined8 *puVar3;
  longlong lVar4;
  undefined8 uVar5;
  uint uVar6;
  bool bVar7;
  undefined1 auStack_4f8 [32];
  undefined4 local_4d8;
  undefined4 local_4d0;
  undefined4 local_4c8;
  undefined4 local_4c0;
  undefined4 local_4b8;
  undefined4 local_4b0;
  undefined4 local_4a8;
  uint local_498 [2];
  longlong *local_490;
  longlong local_488 [2];
  undefined1 local_478 [1104];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_4f8;
  local_498[0] = 0;
  switch(param_2) {
  case 0x11:
    FUN_1406ed520(local_478,0x17e);
    local_498[0] = 0x12;
    FUN_1406ede20(local_478,local_498,4);
    uVar5 = FUN_1408a9e40(&local_490,0x1f8);
    local_4a8 = 0;
    local_4b0 = 0;
    local_4b8 = 3;
    local_4c0 = 0;
    local_4c8 = 0;
    local_4d0 = 0xffffffff;
    local_4d8 = 0;
    iVar2 = FUN_142a269c0(uVar5,0,param_1 + 0x48,1);
    FUN_1406ed840(local_478,iVar2 == 6);
    FUN_1415d01c0(local_478);
    FUN_1406ed610(local_478);
    break;
  case 0x12:
    *(undefined4 *)((longlong)param_1 + 0x175c) = 0;
    uVar5 = 0x1fa;
    goto LAB_141e9990a;
  case 0x15:
    FUN_1406ed520(local_478,0x17e);
    local_498[0] = 0x16;
    FUN_1406ede20(local_478,local_498,4);
    uVar5 = FUN_1408a9e40(&local_490,0x1fc);
    local_4a8 = 0;
    local_4b0 = 0;
    local_4b8 = 3;
    local_4c0 = 0;
    local_4c8 = 0;
    local_4d0 = 0xffffffff;
    local_4d8 = 0;
    iVar2 = FUN_142a269c0(uVar5,0,param_1 + 0x48,1);
    FUN_1406ed840(local_478,iVar2 == 6);
    FUN_1415d01c0(local_478);
    FUN_1406ed610(local_478);
    break;
  case 0x16:
    FUN_141e9b690(param_1,param_3);
    break;
  case 0x19:
    FUN_142bf6230(param_1,&local_490);
    iVar2 = FUN_141c3f8b0(param_1);
    if (iVar2 == 0) {
      lVar4 = param_1[0xa2];
      if (lVar4 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar4 = param_1[0xa2];
      }
      (**(code **)(*(longlong *)(lVar4 + 8) + 0x70))((longlong *)(lVar4 + 8),1);
    }
    *(undefined4 *)(param_1 + 0x361) = 1;
    FUN_141e9f340(param_1);
    if (local_490 != (longlong *)0x0) {
      (**(code **)(*local_490 + 0x10))();
    }
    break;
  case 0x1a:
    iVar2 = FUN_141c3f8b0(param_1);
    if (iVar2 == 0) {
      lVar4 = param_1[0xa2];
      if (lVar4 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar4 = param_1[0xa2];
      }
      (**(code **)(*(longlong *)(lVar4 + 8) + 0x70))((longlong *)(lVar4 + 8),0);
    }
    *(undefined4 *)(param_1 + 0x361) = 0;
    FUN_141e9f340(param_1);
    break;
  case 0x1c:
    if (*(int *)((longlong)param_1 + 0x60c) != 0) {
      FUN_141ea5de0(param_1 + 0xc1);
    }
    bVar1 = FUN_1406e8ae0(param_3);
    uVar6 = FUN_141c3f8b0(param_1);
    *(uint *)((longlong)param_1 + 0x1afc) = (uint)(bVar1 != uVar6);
    *(uint *)((longlong)param_1 + 0x1af4) = (bVar1 == uVar6) + 1;
    if ((longlong *)param_1[0xb9] != (longlong *)0x0) {
      (**(code **)(*(longlong *)param_1[0xb9] + 0x10))();
    }
    param_1[0xb9] = 0;
    if ((longlong *)param_1[0xba] != (longlong *)0x0) {
      (**(code **)(*(longlong *)param_1[0xba] + 0x10))();
    }
    param_1[0xba] = 0;
    if ((longlong *)param_1[0xbc] != (longlong *)0x0) {
      (**(code **)(*(longlong *)param_1[0xbc] + 0x10))();
    }
    param_1[0xbc] = 0;
    *(undefined4 *)(param_1 + 0x363) = 30000;
    *(undefined4 *)(param_1 + 0x367) = 0;
    *(undefined4 *)(param_1 + 0x361) = 0;
    FUN_141ea0210(param_1);
    (**(code **)(*param_1 + 0x90))(param_1,0);
    *(undefined4 *)(param_1 + 0x360) = 1;
    break;
  case 0x1d:
    FUN_141e9bb10(param_1,param_3);
    break;
  case 0x1e:
    bVar1 = FUN_1406e8ae0(param_3);
    uVar6 = FUN_141c3f8b0(param_1);
    *(uint *)((longlong)param_1 + 0x1afc) = (uint)(uVar6 == bVar1);
    *(undefined4 *)(param_1 + 0x363) = 30000;
    (**(code **)(*param_1 + 0x90))(param_1,0);
    break;
  case 0x1f:
    FUN_1406e9170(param_3,param_1 + 0x366,8);
    bVar1 = FUN_1406e8ae0(param_3);
    FUN_141ea1eb0(param_1,(int)param_1[0x366],*(undefined4 *)((longlong)param_1 + 0x1b34),bVar1);
    bVar7 = *(uint *)((longlong)param_1 + 0x1af4) != (uint)bVar1;
    if (!bVar7) {
      *(int *)(param_1 + 0x367) = (int)param_1[0x367] + 1;
    }
    *(uint *)((longlong)param_1 + 0x1afc) = (uint)bVar7;
    *(undefined4 *)(param_1 + 0x2ec) = 0;
    *(undefined4 *)(param_1 + 0x2ed) = 0;
    *(undefined4 *)(param_1 + 0x363) = 30000;
    (**(code **)(*param_1 + 0x90))(param_1,0);
    if (bVar1 == 1) {
      puVar3 = (undefined8 *)FUN_1408a9d20(&local_490,0x5db);
      uVar6 = 1;
    }
    else {
      puVar3 = (undefined8 *)FUN_1408a9d20(local_488,0x5dc);
      uVar6 = 2;
    }
    local_498[0] = uVar6;
    FUN_1429f1f50(*puVar3,100,0);
    if (((uVar6 & 2) != 0) && (uVar6 = uVar6 & 0xfffffffd, local_498[0] = uVar6, local_488[0] != 0))
    {
      FUN_1401bebb0(local_488[0] + -0x10);
    }
    if (((uVar6 & 1) != 0) && (local_490 != (longlong *)0x0)) {
      FUN_1401bebb0(local_490 + -2);
    }
    break;
  case 0x20:
    FUN_1406e9170(param_3,local_498,4);
    if (local_498[0] == 0x22) {
      uVar5 = 0x206;
    }
    else {
      uVar5 = 0x207;
    }
LAB_141e9990a:
    uVar5 = FUN_1408a9d20(&local_490,uVar5);
    FUN_141c3fdd0(param_1,uVar5,2);
  }
  return;
}



//===========================================================
// FUN_141e95720 @ 141e95720   (1224 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141e95720(longlong param_1,undefined4 param_2)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  undefined1 auStack_4d8 [32];
  undefined4 local_4b8;
  undefined4 local_4b0;
  undefined4 local_4a8;
  undefined4 local_4a0;
  undefined4 local_498;
  undefined4 local_490;
  undefined4 local_488;
  undefined4 local_478;
  undefined4 uStack_474;
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4d8;
  switch(param_2) {
  case 0x3e9:
    iVar1 = FUN_141c3f8a0(param_1);
    if (0 < iVar1) {
      FUN_1406ed520(local_468,0x17e);
      local_478 = 0x1c;
      FUN_1406ede20(local_468,&local_478,4);
      FUN_1415d01c0(local_468);
      FUN_1406ed610(local_468);
    }
    break;
  case 0x3ea:
    if ((*(int *)(param_1 + 0x1760) == 0) && (*(int *)(param_1 + 0x175c) == 0)) {
      uVar2 = FUN_1408a9e40(&local_478,0x1f9);
      local_488 = 0;
      local_490 = 0;
      local_498 = 3;
      local_4a0 = 0;
      local_4a8 = 0;
      local_4b0 = 0xffffffff;
      local_4b8 = 0;
      iVar1 = FUN_142a269c0(uVar2,0,param_1 + 0x240,1);
      if (iVar1 == 6) {
        FUN_1406ed520(local_468,0x17e);
        local_478 = 0x11;
        FUN_1406ede20(local_468,&local_478,4);
        FUN_1415d01c0(local_468);
        *(undefined4 *)(param_1 + 0x175c) = 1;
        *(undefined4 *)(param_1 + 0x1760) = 1;
        FUN_1406ed610(local_468);
      }
    }
    break;
  case 0x3eb:
    if (*(int *)(param_1 + 0x1758) == 0) {
      uVar2 = FUN_1408a9e40(&local_478,0x1f6);
      local_488 = 0;
      local_490 = 0;
      local_498 = 3;
      local_4a0 = 0;
      local_4a8 = 0;
      local_4b0 = 0xffffffff;
      local_4b8 = 0;
      iVar1 = FUN_142a269c0(uVar2,0,param_1 + 0x240,1);
      if ((iVar1 == 6) && (*(int *)(param_1 + 0x1b00) == 1)) {
        FUN_1406ed520(local_468,0x17e);
        local_478 = 0x13;
        FUN_1406ede20(local_468,&local_478,4);
        FUN_1415d01c0(local_468);
        *(undefined4 *)(param_1 + 0x1758) = 1;
        FUN_1406ed610(local_468);
      }
    }
    break;
  case 0x3ec:
    FUN_141e9c770(param_1);
    break;
  default:
    FUN_14177fb00(param_1);
    break;
  case 0x3ee:
    FUN_141e9c320(param_1);
    break;
  case 0x3ef:
    FUN_141c417f0(param_1);
    break;
  case 0x3f0:
    if (*(int *)(param_1 + 0x1b08) == 0) {
      FUN_1406ed520(local_468,0x17e);
      local_478 = 0x19;
      FUN_1406ede20(local_468,&local_478,4);
      FUN_1415d01c0(local_468);
    }
    else {
      FUN_1406ed520(local_468,0x17e);
      local_478 = 0x1a;
      FUN_1406ede20(local_468,&local_478,4);
      FUN_1415d01c0(local_468);
    }
    FUN_1406ed610(local_468);
    puVar3 = (undefined8 *)FUN_1408a9d20(&local_478,0x5d9);
    FUN_1429f1f50(*puVar3,100,0);
    if (CONCAT44(uStack_474,local_478) != 0) {
      FUN_1401bebb0(CONCAT44(uStack_474,local_478) + -0x10);
    }
    break;
  case 0x3f1:
    uVar2 = FUN_1408a9e40(&local_478,0x1f7);
    local_488 = 0;
    local_490 = 0;
    local_498 = 3;
    local_4a0 = 0;
    local_4a8 = 0;
    local_4b0 = 0xffffffff;
    local_4b8 = 0;
    iVar1 = FUN_142a269c0(uVar2,0,param_1 + 0x240,1);
    if (iVar1 == 6) {
      FUN_1406ed520(local_468,0x17e);
      local_478 = 0x1b;
      FUN_1406ede20(local_468,&local_478,4);
      FUN_1415d01c0(local_468);
      FUN_1406ed610(local_468);
    }
    break;
  case 0x3f2:
    if (0 < *(int *)(param_1 + 0x1b38)) {
      if (((*(int *)(param_1 + 0x1764) == 0) && (*(int *)(param_1 + 0x1768) == 0)) &&
         (*(int *)(param_1 + 0x176c) == 0)) {
        uVar2 = FUN_1408a9e40(&local_478,0x1fd);
        local_488 = 0;
        local_490 = 0;
        local_498 = 3;
        local_4a0 = 0;
        local_4a8 = 0;
        local_4b0 = 0xffffffff;
        local_4b8 = 0;
        iVar1 = FUN_142a269c0(uVar2,0,param_1 + 0x240,1);
        if (iVar1 == 6) {
          FUN_1406ed520(local_468,0x17e);
          local_478 = 0x15;
          FUN_1406ede20(local_468,&local_478,4);
          FUN_1415d01c0(local_468);
          *(undefined4 *)(param_1 + 0x1764) = 1;
          *(undefined4 *)(param_1 + 0x1768) = 1;
          FUN_1406ed610(local_468);
        }
      }
      if (*(int *)(param_1 + 0x176c) == 1) {
        uVar2 = FUN_1408a9d20(&local_478,0x1fb);
        FUN_141c3fdd0(param_1,uVar2,2);
      }
    }
  }
  return;
}



//===========================================================
// FUN_141e9ae60 @ 141e9ae60   (1078 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141e9ae60(longlong *param_1)

{
  longlong *plVar1;
  int iVar2;
  undefined8 *puVar3;
  longlong lVar4;
  undefined4 uVar5;
  undefined8 uVar6;
  int iVar7;
  double dVar8;
  undefined1 auStack_4a8 [32];
  undefined4 local_488;
  undefined4 uStack_484;
  undefined1 local_478 [1104];
  ulonglong local_28;
  
  local_28 = DAT_143a8b908 ^ (ulonglong)auStack_4a8;
  FUN_1429e3ef0();
  if ((int)param_1[0x360] == 1) {
    lVar4 = param_1[0x363];
    iVar7 = (int)lVar4 + -0x1e;
    *(int *)(param_1 + 0x363) = iVar7;
    iVar2 = *(int *)((longlong)param_1 + 0x1b1c);
    dVar8 = floor((double)((int)lVar4 + 0x3c9) / DAT_14327ab00);
    if ((double)iVar2 != dVar8) {
      if ((*(int *)((longlong)param_1 + 0x1afc) != 0) && (iVar2 - 1U < 9)) {
        puVar3 = (undefined8 *)FUN_1408a9d20(&local_488,0x5d8);
        FUN_1429f1f50(*puVar3,100,0);
        if (CONCAT44(uStack_484,local_488) != 0) {
          FUN_1401bebb0(CONCAT44(uStack_484,local_488) + -0x10);
        }
        iVar7 = (int)param_1[0x363];
      }
      dVar8 = floor((double)(iVar7 + 999) / DAT_14327ab00);
      *(int *)((longlong)param_1 + 0x1b1c) = (int)dVar8;
      if ((int)dVar8 < 1) {
        FUN_1406ed520(local_478,0x17e);
        local_488 = 0x1e;
        FUN_1406ede20(local_478,&local_488,4);
        FUN_1415d01c0(local_478);
        *(undefined4 *)((longlong)param_1 + 0x1b1c) = 0;
        FUN_1406ed610(local_478);
      }
      (**(code **)(*param_1 + 0x90))(param_1,0);
    }
    iVar2 = FUN_141c3f8b0(param_1);
    if (iVar2 == 0) {
      lVar4 = param_1[0xa8];
      if (lVar4 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar4 = param_1[0xa8];
      }
      (**(code **)(*(longlong *)(lVar4 + 8) + 0x70))((longlong *)(lVar4 + 8),0);
      lVar4 = param_1[0xa2];
      if (lVar4 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar4 = param_1[0xa2];
      }
    }
    else {
      lVar4 = param_1[0xa4];
      if (lVar4 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar4 = param_1[0xa4];
      }
    }
    (**(code **)(*(longlong *)(lVar4 + 8) + 0x70))((longlong *)(lVar4 + 8),0);
    lVar4 = param_1[0xa0];
    if (lVar4 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar4 = param_1[0xa0];
    }
    (**(code **)(*(longlong *)(lVar4 + 8) + 0x70))((longlong *)(lVar4 + 8),1);
    lVar4 = param_1[0xaa];
    if (lVar4 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar4 = param_1[0xaa];
    }
    (**(code **)(*(longlong *)(lVar4 + 8) + 0x70))((longlong *)(lVar4 + 8),1);
    lVar4 = param_1[0x9e];
    if (lVar4 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar4 = param_1[0x9e];
    }
    uVar6 = 1;
  }
  else {
    iVar2 = FUN_141c3f8b0(param_1);
    if (iVar2 == 0) {
      lVar4 = param_1[0xa8];
      if (lVar4 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar4 = param_1[0xa8];
      }
      (**(code **)(*(longlong *)(lVar4 + 8) + 0x70))((longlong *)(lVar4 + 8),1);
      iVar2 = FUN_141c3f8a0(param_1);
      lVar4 = param_1[0xa2];
      if (iVar2 < 2) {
        if (lVar4 == 0) {
          FUN_142e52ed0(0x431,0);
          lVar4 = param_1[0xa2];
        }
        uVar5 = 0;
      }
      else {
        if (lVar4 == 0) {
          FUN_142e52ed0(0x431,0);
          lVar4 = param_1[0xa2];
        }
        uVar5 = (undefined4)param_1[0x361];
      }
    }
    else {
      lVar4 = param_1[0xa4];
      if (lVar4 == 0) {
        FUN_142e52ed0(0x431,0);
        lVar4 = param_1[0xa4];
      }
      uVar5 = 1;
    }
    (**(code **)(*(longlong *)(lVar4 + 8) + 0x70))((longlong *)(lVar4 + 8),uVar5);
    lVar4 = param_1[0xa0];
    if (lVar4 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar4 = param_1[0xa0];
    }
    (**(code **)(*(longlong *)(lVar4 + 8) + 0x70))((longlong *)(lVar4 + 8),0);
    lVar4 = param_1[0xaa];
    if (lVar4 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar4 = param_1[0xaa];
    }
    (**(code **)(*(longlong *)(lVar4 + 8) + 0x70))((longlong *)(lVar4 + 8),0);
    lVar4 = param_1[0x9e];
    if (lVar4 == 0) {
      FUN_142e52ed0(0x431,0);
      lVar4 = param_1[0x9e];
    }
    uVar6 = 0;
  }
  (**(code **)(*(longlong *)(lVar4 + 8) + 0x70))((longlong *)(lVar4 + 8),uVar6);
  if ((int)param_1[0x360] == 0) {
    if ((longlong *)param_1[0xbf] != (longlong *)0x0) {
      (**(code **)(*(longlong *)param_1[0xbf] + 0x10))();
    }
    param_1[0xbf] = 0;
    plVar1 = (longlong *)param_1[0xbe];
    if (plVar1 != (longlong *)0x0) {
      param_1[0xbe] = 0;
      (**(code **)(*plVar1 + 0x10))();
    }
  }
  iVar2 = FUN_141c3f8a0(param_1);
  if (iVar2 < 2) {
    if ((longlong *)param_1[0xbd] != (longlong *)0x0) {
      (**(code **)(*(longlong *)param_1[0xbd] + 0x10))();
    }
    param_1[0xbd] = 0;
  }
  FUN_141e9f340(param_1);
  FUN_142bf2be0(param_1);
  return;
}



//===========================================================
// FUN_141e9c770 @ 141e9c770   (568 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141e9c770(longlong *param_1)

{
  int iVar1;
  undefined8 uVar2;
  undefined1 auStack_4d8 [32];
  undefined4 local_4b8;
  undefined4 local_4b0;
  undefined4 local_4a8;
  undefined4 local_4a0;
  undefined4 local_498;
  undefined4 local_490;
  undefined4 local_488;
  undefined4 local_478 [4];
  undefined1 local_468 [1104];
  ulonglong local_18;
  
  local_18 = DAT_143a8b908 ^ (ulonglong)auStack_4d8;
  if ((int)param_1[0x360] == 1) {
    if (*(int *)((longlong)param_1 + 0x1b04) == 0) {
      uVar2 = FUN_1408a9e40(local_478,0x1ff);
      local_488 = 0;
      local_490 = 0;
      local_498 = 3;
      local_4a0 = 0;
      local_4a8 = 0;
      local_4b0 = 0xffffffff;
      local_4b8 = 0;
      iVar1 = FUN_142a269c0(uVar2,0,param_1 + 0x48,1);
      if (iVar1 != 6) {
        return;
      }
      FUN_1406ed520(local_468,0x17e);
      local_478[0] = 0x17;
      FUN_1406ede20(local_468,local_478,4);
      FUN_1415d01c0(local_468);
      *(undefined4 *)((longlong)param_1 + 0x1b04) = 1;
    }
    else {
      uVar2 = FUN_1408a9e40(local_478,0x200);
      local_488 = 0;
      local_490 = 0;
      local_498 = 3;
      local_4a0 = 0;
      local_4a8 = 0;
      local_4b0 = 0xffffffff;
      local_4b8 = 0;
      iVar1 = FUN_142a269c0(uVar2,0,param_1 + 0x48,1);
      if (iVar1 != 6) {
        return;
      }
      FUN_1406ed520(local_468,0x17e);
      local_478[0] = 0x18;
      FUN_1406ede20(local_468,local_478,4);
      FUN_1415d01c0(local_468);
      *(undefined4 *)((longlong)param_1 + 0x1b04) = 0;
    }
  }
  else {
    uVar2 = FUN_1408a9e40(local_478,0x203);
    local_488 = 0;
    local_490 = 0;
    local_498 = 3;
    local_4a0 = 0;
    local_4a8 = 0;
    local_4b0 = 0xffffffff;
    local_4b8 = 0;
    iVar1 = FUN_142a269c0(uVar2,0,param_1 + 0x48,1);
    if (iVar1 != 6) {
      return;
    }
    FUN_1406ed520(local_468,0x17e);
    local_478[0] = 0xc;
    FUN_1406ede20(local_468,local_478,4);
    FUN_1415d01c0(local_468);
    (**(code **)(*param_1 + 0x138))(param_1,2);
  }
  FUN_1406ed610(local_468);
  return;
}



//===========================================================
// FUN_141e9bb10 @ 141e9bb10   (480 bytes)
//===========================================================

void FUN_141e9bb10(longlong *param_1,undefined8 param_2)

{
  byte bVar1;
  int iVar2;
  int iVar3;
  undefined8 uVar4;
  undefined8 *puVar5;
  longlong lVar6;
  longlong local_res8;
  
  bVar1 = FUN_1406e8ae0(param_2);
  *(uint *)((longlong)param_1 + 0x1b24) = (uint)bVar1;
  if (bVar1 == 1) {
    uVar4 = FUN_1408a9d20(&local_res8,500);
    FUN_141c3fdd0(param_1,uVar4,2);
    puVar5 = (undefined8 *)FUN_1408a9d20(&local_res8,0x5d5);
    FUN_1429f1f50(*puVar5,100,0);
  }
  else {
    bVar1 = FUN_1406e8ae0(param_2);
    *(uint *)(param_1 + 0x364) = (uint)bVar1;
    iVar2 = FUN_141c3f8b0(param_1);
    iVar3 = 3 - *(int *)((longlong)param_1 + 0x1af4);
    if ((int)param_1[0x364] == iVar2) {
      iVar3 = *(int *)((longlong)param_1 + 0x1af4);
    }
    *(int *)(param_1 + 0x35f) = iVar3;
    iVar3 = FUN_141c3f8b0(param_1);
    if ((int)param_1[0x364] == iVar3) {
      uVar4 = FUN_1408a9d20(&local_res8,499);
      FUN_141c3fdd0(param_1,uVar4,2);
      *(undefined4 *)((longlong)param_1 + 0x1af4) = 2;
      puVar5 = (undefined8 *)FUN_1408a9d20(&local_res8,0x5d6);
      FUN_1429f1f50(*puVar5,100,0);
    }
    else {
      uVar4 = FUN_1408a9d20(&local_res8,0x1f5);
      FUN_141c3fdd0(param_1,uVar4,2);
      *(undefined4 *)((longlong)param_1 + 0x1af4) = 1;
      puVar5 = (undefined8 *)FUN_1408a9d20(&local_res8,0x5d7);
      FUN_1429f1f50(*puVar5,100,0);
    }
  }
  if (local_res8 != 0) {
    FUN_1401bebb0(local_res8 + -0x10);
  }
  lVar6 = param_1[0x68];
  if (lVar6 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar6 = param_1[0x68];
  }
  FUN_1402d1b50(lVar6,param_2);
  lVar6 = param_1[0x6f];
  if (lVar6 == 0) {
    FUN_142e52ed0(0x431,0);
    lVar6 = param_1[0x6f];
  }
  FUN_1402d1b50(lVar6,param_2);
  *(undefined4 *)(param_1 + 0x360) = 2;
  *(undefined4 *)(param_1 + 0x361) = 0;
  (**(code **)(*param_1 + 0x90))(param_1,0);
  FUN_141ea1d50(param_1);
  return;
}



//===========================================================
// FUN_141e9b690 @ 141e9b690   (1127 bytes)
//===========================================================

void FUN_141e9b690(longlong *param_1,undefined8 param_2)

{
  undefined4 uVar1;
  longlong *plVar2;
  IUnknown *pIVar3;
  char cVar4;
  byte bVar5;
  byte bVar6;
  int iVar7;
  uint uVar8;
  undefined8 uVar9;
  uint uVar10;
  ulonglong uVar11;
  longlong *local_res18;
  short local_b8;
  undefined2 uStack_b6;
  undefined4 uStack_b4;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined4 local_98;
  undefined4 uStack_94;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined4 local_78 [2];
  IUnknown *local_70;
  undefined8 local_68;
  longlong lStack_60;
  undefined8 local_58;
  uint local_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined8 local_38;
  
  cVar4 = FUN_1406e8ae0(param_2);
  uVar8 = 0;
  if (cVar4 == '\0') {
    uVar9 = FUN_1408a9d20(&local_res18,0x1fe);
    FUN_141c3fdd0(param_1,uVar9,2);
  }
  else {
    bVar5 = FUN_1406e8ae0(param_2);
    bVar6 = FUN_1406e8ae0(param_2);
    if (bVar5 != 0) {
      uVar11 = (ulonglong)bVar5;
      do {
        if (*(int *)((longlong)param_1 + 0x60c) != 0) {
          iVar7 = *(int *)param_1[0xc3];
          plVar2 = *(longlong **)((int *)param_1[0xc3] + 2);
          if (plVar2 != (longlong *)0x0) {
            (**(code **)(*plVar2 + 8))(plVar2);
          }
          if (iVar7 == *(int *)((longlong)param_1 + 0x1af4)) {
            *(int *)(param_1 + 0x367) = (int)param_1[0x367] + -1;
          }
          if (plVar2 != (longlong *)0x0) {
            (**(code **)(*plVar2 + 0x10))(plVar2);
          }
        }
        FUN_141ea5e80(param_1 + 0xc1,param_1[0xc3]);
        uVar11 = uVar11 - 1;
      } while (uVar11 != 0);
    }
    if (*(int *)((longlong)param_1 + 0x60c) != 0) {
      uVar1 = *(undefined4 *)param_1[0xc3];
      pIVar3 = *(IUnknown **)((undefined4 *)param_1[0xc3] + 2);
      local_78[0] = uVar1;
      local_70 = pIVar3;
      if (pIVar3 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (**(code **)(*(longlong *)pIVar3 + 8))(pIVar3);
      local_b8 = 3;
      uStack_b0 = CONCAT44(uStack_b0._4_4_,0xfffffffe);
      local_res18 = (longlong *)0x0;
      local_98 = CONCAT22(uStack_b6,3);
      uStack_94 = uStack_b4;
      uStack_90 = CONCAT44(uStack_b0._4_4_,0xfffffffe);
      local_88 = local_a8;
      iVar7 = (**(code **)(*(longlong *)pIVar3 + 0x268))(pIVar3,&local_98,&local_res18);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar3,(_GUID *)&DAT_14327fcb0);
      }
      if (local_res18 != (longlong *)0x0) {
        (**(code **)(*local_res18 + 0x10))();
      }
      if (local_b8 == 8) {
        local_b8 = 0;
        if (uStack_b0 != 0) {
          (*DAT_143ad5990)(uStack_b0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_b8);
      }
      local_res18 = (longlong *)param_1[0xb1];
      if (local_res18 != (longlong *)0x0) {
        (**(code **)(*local_res18 + 8))();
      }
      FUN_141ea4e90(param_1,&local_res18,local_78,uVar1);
      pIVar3 = local_70;
      if (local_70 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_142ef3ac0(0x80004003);
      }
      (*DAT_143262a20)(&local_b8);
      if (DAT_143a8b8d8 == 8) {
        if (local_b8 == 8) {
          local_b8 = 0;
          if (uStack_b0 != 0) {
            (*DAT_143ad5990)(uStack_b0 + -4);
          }
        }
        else {
          iVar7 = (*DAT_143262a18)(&local_b8);
          if (iVar7 < 0) goto LAB_141e9bae0;
        }
        local_b8 = 8;
        uVar10 = uVar8;
        if (DAT_143a8b8e0 != 0) {
          uVar10 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
        }
        uStack_b0 = FUN_1401a5fa0(DAT_143a8b8e0,uVar10);
      }
      else {
        if ((local_b8 == 8) && (local_b8 = 0, uStack_b0 != 0)) {
          (*DAT_143ad5990)(uStack_b0 + -4);
        }
        iVar7 = (*DAT_143262a28)(&local_b8,&DAT_143a8b8d8);
        if (iVar7 < 0) {
LAB_141e9bae0:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar7);
        }
      }
      (*DAT_143262a20)(&local_98);
      if (DAT_143a8b8d8 == 8) {
        if ((short)local_98 == 8) {
          local_98 = (uint)local_98._2_2_ << 0x10;
          if (uStack_90 != 0) {
            (*DAT_143ad5990)(uStack_90 + -4);
          }
        }
        else {
          iVar7 = (*DAT_143262a18)(&local_98);
          if (iVar7 < 0) goto LAB_141e9bae8;
        }
        local_98 = CONCAT22(local_98._2_2_,8);
        if (DAT_143a8b8e0 != 0) {
          uVar8 = *(uint *)(DAT_143a8b8e0 + -4) >> 1;
        }
        uStack_90 = FUN_1401a5fa0(DAT_143a8b8e0,uVar8);
      }
      else {
        if (((short)local_98 == 8) && (local_98 = (uint)local_98._2_2_ << 0x10, uStack_90 != 0)) {
          (*DAT_143ad5990)(uStack_90 + -4);
        }
        iVar7 = (*DAT_143262a28)(&local_98,&DAT_143a8b8d8);
        if (iVar7 < 0) {
LAB_141e9bae8:
                    /* WARNING: Subroutine does not return */
          FUN_142ef3ac0(iVar7);
        }
      }
      local_68 = CONCAT44(uStack_b4,CONCAT22(uStack_b6,local_b8));
      lStack_60 = uStack_b0;
      local_58 = local_a8;
      local_48 = local_98;
      uStack_44 = uStack_94;
      uStack_40 = (undefined4)uStack_90;
      uStack_3c = uStack_90._4_4_;
      local_38 = local_88;
      iVar7 = (**(code **)(*(longlong *)pIVar3 + 0x280))(pIVar3,0x20,&local_48,&local_68);
      if (iVar7 < 0) {
        _com_issue_errorex(iVar7,pIVar3,(_GUID *)&DAT_14327fcb0);
      }
      if ((short)local_98 == 8) {
        local_98 = local_98 & 0xffff0000;
        if (uStack_90 != 0) {
          (*DAT_143ad5990)(uStack_90 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_98);
      }
      if (local_b8 == 8) {
        local_b8 = 0;
        if (uStack_b0 != 0) {
          (*DAT_143ad5990)(uStack_b0 + -4);
        }
      }
      else {
        (*DAT_143262a18)(&local_b8);
      }
      (**(code **)(*(longlong *)pIVar3 + 0x10))(pIVar3);
    }
    uVar8 = FUN_141c3f8b0(param_1);
    if (uVar8 == bVar6) {
      *(undefined4 *)((longlong)param_1 + 0x176c) = 1;
    }
    *(uint *)((longlong)param_1 + 0x1afc) = (uint)(uVar8 == bVar6);
    *(undefined4 *)(param_1 + 0x363) = 30000;
    (**(code **)(*param_1 + 0x90))(param_1,0);
  }
  *(undefined4 *)((longlong)param_1 + 0x1764) = 0;
  return;
}



//===========================================================
// FUN_142d4abc0 @ 142d4abc0   (2202 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_142d4abc0(undefined8 param_1,undefined8 param_2,int param_3)

{
  char *pcVar1;
  int iVar2;
  int iVar3;
  undefined8 uVar4;
  int *piVar5;
  longlong lVar6;
  char *pcVar7;
  undefined4 *puVar8;
  longlong *plVar9;
  int *piVar10;
  int *piVar11;
  uint uVar12;
  char *pcVar13;
  int iVar14;
  int *piVar15;
  undefined1 auStack_538 [32];
  int *local_518;
  undefined4 local_510;
  undefined4 local_508;
  undefined4 local_500;
  undefined4 local_4f8;
  undefined4 local_4f0;
  undefined4 local_4e8;
  char *local_4d8;
  int local_4d0;
  int local_4cc;
  undefined4 local_4c8;
  undefined4 uStack_4c4;
  longlong *local_4b8;
  longlong local_4b0;
  longlong local_4a8;
  int local_4a0 [2];
  undefined1 local_498 [1104];
  ulonglong local_48;
  
  local_48 = DAT_143a8b908 ^ (ulonglong)auStack_538;
  iVar2 = FUN_1428de020(DAT_143aa8518,1);
  if (iVar2 != 0) {
    return;
  }
  iVar2 = FUN_140f89f40(DAT_143aa8518 + 0x100);
  if ((iVar2 != 0) || (iVar2 = FUN_140f89e20(DAT_143aa8518 + 0x100), iVar2 != 0)) {
    FUN_142d9f580(param_1);
    return;
  }
  if (DAT_143aa8520 != 0) {
    uVar4 = FUN_1408a9e40(&local_4c8,0x86);
    FUN_1415eca30(uVar4,0xb);
    if (CONCAT44(uStack_4c4,local_4c8) == 0) {
      return;
    }
    FUN_14019f2c0(CONCAT44(uStack_4c4,local_4c8) + -0x10);
    return;
  }
  iVar2 = FUN_14276e7e0(DAT_143aa8518);
  if (iVar2 == 0) {
    return;
  }
  uVar4 = FUN_141892840();
  iVar2 = FUN_14182e550(uVar4);
  if (iVar2 != 0) {
    uVar4 = FUN_1408a9e40(&local_4c8,0x1e1);
    local_4f0 = 0;
    local_4f8 = 0;
    local_500 = 0;
    local_508 = 0;
    local_510 = 0;
    local_518 = (int *)((ulonglong)local_518 & 0xffffffff00000000);
    FUN_142a26280(uVar4,0,0,1);
    return;
  }
  piVar15 = (int *)0x0;
  iVar2 = 0;
  local_4cc = 0;
  local_4d8 = (char *)0x0;
  local_4b0 = 0;
  local_4d0 = 0;
  local_4a0[0] = FUN_140417e60(param_3);
  if (local_4a0[0] == 0) goto LAB_142d4b3ff;
  piVar11 = (int *)0xffffffffffffffff;
  if (local_4a0[0] == 3) {
    local_4a8 = FUN_14019b780(&DAT_143ad68a0,0x328);
    piVar5 = piVar15;
    if (local_4a8 != 0) {
      piVar5 = (int *)FUN_14228e2a0(local_4a8);
    }
    piVar10 = piVar5 + 6;
    if (piVar5 == (int *)0x0) {
      piVar10 = piVar15;
    }
    if (piVar10 == (int *)0x0) {
      local_4b8 = (longlong *)0x0;
    }
    else {
      local_4b8 = (longlong *)(piVar10 + -6);
      if (local_4b8 != (longlong *)0x0) {
        if (0xfffff < *(ulonglong *)(piVar10 + 2)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(piVar10 + 2) = *(longlong *)(piVar10 + 2) + 1;
        UNLOCK();
      }
    }
    plVar9 = local_4b8;
    if (local_4b8 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    iVar3 = (**(code **)(*plVar9 + 0x130))(plVar9);
    if (iVar3 != 1) {
      if (0xffffe < plVar9[4] - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar9 = plVar9 + 4;
      lVar6 = *plVar9;
      *plVar9 = *plVar9 + -1;
      UNLOCK();
      if (((int)lVar6 == 1) && (plVar9 = local_4b8 + 3, plVar9 != (longlong *)0x0)) {
        (**(code **)*plVar9)(plVar9,1);
      }
      goto LAB_142d4b3ff;
    }
    FUN_1422907d0(plVar9,&local_4d8,&local_4b0,&local_4d0);
    if (0xffffe < plVar9[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar9 = plVar9 + 4;
    lVar6 = *plVar9;
    *plVar9 = *plVar9 + -1;
    UNLOCK();
    if (((int)lVar6 == 1) && (plVar9 = local_4b8 + 3, plVar9 != (longlong *)0x0)) {
      (**(code **)*plVar9)(plVar9,1);
    }
    local_4cc = param_3 % 100;
  }
  else if (local_4a0[0] == 4) {
    local_4a8 = FUN_14019b780(&DAT_143ad68a0,0x338);
    piVar5 = piVar15;
    if (local_4a8 != 0) {
      piVar5 = (int *)FUN_142290910(local_4a8);
    }
    piVar10 = piVar5 + 6;
    if (piVar5 == (int *)0x0) {
      piVar10 = piVar15;
    }
    if (piVar10 == (int *)0x0) {
      local_4b8 = (longlong *)0x0;
    }
    else {
      local_4b8 = (longlong *)(piVar10 + -6);
      if (local_4b8 != (longlong *)0x0) {
        if (0xfffff < *(ulonglong *)(piVar10 + 2)) {
          FUN_142e541f0(0x30f);
        }
        LOCK();
        *(longlong *)(piVar10 + 2) = *(longlong *)(piVar10 + 2) + 1;
        UNLOCK();
      }
    }
    plVar9 = local_4b8;
    if (local_4b8 == (longlong *)0x0) {
      FUN_142e52ed0(0x431,0);
    }
    iVar3 = (**(code **)(*plVar9 + 0x130))(plVar9);
    if (iVar3 != 1) {
      if (0xffffe < plVar9[4] - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar9 = plVar9 + 4;
      lVar6 = *plVar9;
      *plVar9 = *plVar9 + -1;
      UNLOCK();
      if (((int)lVar6 == 1) && (plVar9 = local_4b8 + 3, plVar9 != (longlong *)0x0)) {
        (**(code **)*plVar9)(plVar9,1);
      }
      goto LAB_142d4b3ff;
    }
    local_518 = &local_4d0;
    FUN_142293010(plVar9,&local_4d8,&local_4cc,&local_4b0);
    if (0xffffe < plVar9[4] - 1U) {
      FUN_142e541f0(0x31e);
    }
    LOCK();
    plVar9 = plVar9 + 4;
    lVar6 = *plVar9;
    *plVar9 = *plVar9 + -1;
    UNLOCK();
    if (((int)lVar6 == 1) && (plVar9 = local_4b8 + 3, plVar9 != (longlong *)0x0)) {
      (**(code **)*plVar9)(plVar9,1);
    }
  }
  if (((local_4d8 != (char *)0x0) && (*local_4d8 != '\0')) &&
     (lVar6 = FUN_142ef83e0(&DAT_143273878,(int)local_4d8[(longlong)*(int *)(local_4d8 + -8) + -1]),
     lVar6 != 0)) {
    pcVar7 = (char *)FUN_14019bd40(&local_4d8,0,1);
    iVar3 = iVar2;
    if (local_4d8 != (char *)0x0) {
      iVar3 = *(int *)(local_4d8 + -8);
    }
    pcVar13 = pcVar7 + (longlong)iVar3 + -2;
    while( true ) {
      if (pcVar13 < pcVar7) goto LAB_142d4b035;
      lVar6 = FUN_142ef83e0(&DAT_143273878,(int)*pcVar13);
      pcVar1 = local_4d8;
      if (lVar6 == 0) break;
      pcVar13 = pcVar13 + -1;
    }
    if (pcVar13 < pcVar7) goto LAB_142d4b035;
    pcVar13[1] = '\0';
    uVar12 = (int)(pcVar13 + 1) - (int)pcVar7;
    if (*(int *)(local_4d8 + -0x10) != -1) {
      FUN_142e52dd0(0x8b);
    }
    if ((uVar12 == 0xffffffff) || ((int)uVar12 <= *(int *)(pcVar1 + -0xc))) {
      pcVar1[-0x10] = '\x01';
      pcVar1[-0xf] = '\0';
      pcVar1[-0xe] = '\0';
      pcVar1[-0xd] = '\0';
      if (uVar12 == 0xffffffff) {
        piVar10 = piVar11;
        piVar5 = piVar15;
        if (pcVar1 != (char *)0x0) {
          do {
            piVar5 = (int *)((longlong)piVar10 + 1);
            piVar10 = piVar5;
          } while (pcVar1[(longlong)piVar5] != '\0');
        }
        goto LAB_142d4b19e;
      }
    }
    else {
      FUN_142e54290(0x90,*(int *)(pcVar1 + -0xc),uVar12);
      pcVar1[-0x10] = '\x01';
      pcVar1[-0xf] = '\0';
      pcVar1[-0xe] = '\0';
      pcVar1[-0xd] = '\0';
    }
    local_4d8[(int)uVar12] = '\0';
    piVar5 = (int *)(ulonglong)uVar12;
LAB_142d4b19e:
    iVar3 = (int)piVar5;
    if ((iVar3 < 0) || (*(int *)(pcVar1 + -0xc) + 1 <= iVar3)) {
      FUN_142e54290(0x9c,(ulonglong)piVar5 & 0xffffffff);
    }
    *(int *)(pcVar1 + -8) = iVar3;
  }
LAB_142d4b09e:
  FUN_1401d12a0(&local_4d8,0);
  if ((local_4d8 == (char *)0x0) || (*local_4d8 == '\0')) goto LAB_142d4b3ff;
  iVar3 = FUN_142cb8ad0(DAT_143aa84a0);
  if (iVar3 != 0) {
    uVar4 = FUN_1408a9e40(&local_4c8,0x539);
    local_4e8 = 0;
    local_4f0 = 0;
    local_4f8 = 3;
    local_500 = 0;
    local_508 = 0;
    local_510 = 0xffffffff;
    local_518 = (int *)((ulonglong)local_518 & 0xffffffff00000000);
    iVar3 = FUN_142a269c0(uVar4,0,0,1);
    if (iVar3 != 6) goto LAB_142d4b3ff;
    if (local_4d8 != (char *)0x0) {
      FUN_14019f2c0(local_4d8 + -0x10);
      local_4d8 = (char *)0x0;
    }
  }
  uVar4 = DAT_143ac2f58;
  piVar5 = piVar15;
  if ((local_4d8 == (char *)0x0) || (piVar5 = (int *)(local_4d8 + -0x10), piVar5 == (int *)0x0)) {
LAB_142d4b232:
    if (iVar2 < 0x400) {
      iVar2 = 0x400;
    }
    puVar8 = (undefined4 *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar2 + 0x11));
    puVar8[1] = iVar2;
    *puVar8 = 0xffffffff;
    local_4d8 = (char *)(puVar8 + 4);
    if (piVar5 == (int *)0x0) {
      puVar8[2] = 0;
      *local_4d8 = '\0';
    }
    else {
      iVar3 = piVar5[2] + 1;
      iVar14 = iVar2 + 1;
      if (iVar14 < iVar3) {
        FUN_142e54290(0x5c,iVar3,iVar14);
        iVar3 = iVar14;
      }
      FUN_142ef7ba0(local_4d8,piVar5 + 4,(longlong)iVar3);
      puVar8[2] = piVar5[2];
      local_4d8[iVar2] = '\0';
      FUN_14019f2c0(piVar5);
    }
  }
  else {
    if ((1 < *piVar5) || (*(int *)(local_4d8 + -0xc) < 0x400)) {
      iVar2 = *(int *)(local_4d8 + -8);
      goto LAB_142d4b232;
    }
    if (*piVar5 != 1) {
      FUN_142e52dd0(0x74);
    }
    *piVar5 = -1;
  }
  local_518 = (int *)0x0;
  iVar2 = FUN_1408bed00(uVar4,local_4d8,0,1);
  pcVar7 = local_4d8;
  if (*(int *)(local_4d8 + -0x10) != -1) {
    FUN_142e52dd0(0x8b);
  }
  pcVar7[-0x10] = '\x01';
  pcVar7[-0xf] = '\0';
  pcVar7[-0xe] = '\0';
  pcVar7[-0xd] = '\0';
  iVar3 = iRamfffffffffffffff4;
  if (pcVar7 == (char *)0x0) {
LAB_142d4b319:
    piVar11 = piVar15;
    iVar14 = (int)piVar11;
    if (iVar3 + 1 <= iVar14) goto LAB_142d4b31d;
  }
  else {
    do {
      piVar11 = (int *)((longlong)piVar11 + 1);
    } while (pcVar7[(longlong)piVar11] != '\0');
    iVar3 = *(int *)(pcVar7 + -0xc);
    piVar15 = piVar11;
    if (-1 < (int)piVar11) goto LAB_142d4b319;
LAB_142d4b31d:
    iVar14 = (int)piVar11;
    FUN_142e54290(0x9c,(ulonglong)piVar11 & 0xffffffff);
  }
  *(int *)(pcVar7 + -8) = iVar14;
  if (iVar2 == 0) {
    uVar4 = FUN_1408a9e40(&local_4a8,0xb2);
    local_4f0 = 0;
    local_4f8 = 0;
    local_500 = 0;
    local_508 = 0;
    local_510 = 0;
    local_518 = (int *)((ulonglong)local_518 & 0xffffffff00000000);
    FUN_142a26280(uVar4,0,0,1);
  }
  else {
    FUN_1406ed520(local_498,0x17e);
    local_4c8 = 0;
    FUN_1406ede20(local_498,&local_4c8,4);
    FUN_1406ede20(local_498,local_4a0,4);
    FUN_1406edc80(local_498,&local_4d8);
    FUN_1406ed840(local_498,(undefined1)local_4d0);
    if (local_4d0 != 0) {
      FUN_1406edc80(local_498,&local_4b0);
    }
    FUN_1406ed840(local_498,(undefined1)local_4cc);
    FUN_1415d01c0(local_498);
    FUN_1406ed610(local_498);
  }
LAB_142d4b3ff:
  if (local_4b0 != 0) {
    FUN_14019f2c0(local_4b0 + -0x10);
  }
  if (local_4d8 != (char *)0x0) {
    FUN_14019f2c0(local_4d8 + -0x10);
  }
  return;
LAB_142d4b035:
  pcVar7 = local_4d8;
  if (*(int *)(local_4d8 + -0x10) != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (*(int *)(pcVar7 + -0xc) < 0) {
    FUN_142e54290(0x90,*(int *)(pcVar7 + -0xc),0);
  }
  pcVar7[-0x10] = '\x01';
  pcVar7[-0xf] = '\0';
  pcVar7[-0xe] = '\0';
  pcVar7[-0xd] = '\0';
  *local_4d8 = '\0';
  if (*(int *)(pcVar7 + -0xc) + 1 < 1) {
    FUN_142e54290(0x9c,0);
  }
  pcVar7[-8] = '\0';
  pcVar7[-7] = '\0';
  pcVar7[-6] = '\0';
  pcVar7[-5] = '\0';
  if (local_4d8 != (char *)0x0) {
    FUN_14019f2c0(local_4d8 + -0x10);
    local_4d8 = (char *)0x0;
  }
  goto LAB_142d4b09e;
}



//===========================================================
// FUN_141e9bf40 @ 141e9bf40   (1 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141e9bf40(longlong param_1,undefined8 param_2)

{
  undefined1 auStack_498 [32];
  undefined4 auStack_478 [2];
  undefined8 uStack_470;
  undefined1 auStack_468 [1104];
  ulonglong uStack_18;
  
  uStack_18 = DAT_143a8b908 ^ (ulonglong)auStack_498;
  uStack_470 = param_2;
  FUN_1406ed520(auStack_468,0x17e);
  auStack_478[0] = 0x1f;
  FUN_1406ede20(auStack_468,auStack_478,4);
  FUN_1406ede20(auStack_468,&uStack_470,8);
  FUN_1406ed840(auStack_468,*(undefined1 *)(param_1 + 0x1af4));
  FUN_1415d01c0(auStack_468);
  FUN_1406ed610(auStack_468);
  return;
}



//===========================================================
// FUN_141e95590 @ 141e95590   (392 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */

void FUN_141e95590(longlong param_1,int param_2,undefined4 param_3,int param_4,int param_5)

{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  undefined1 auStack_4d8 [32];
  int iStack_4b8;
  undefined8 uStack_4a8;
  undefined8 uStack_4a0;
  undefined1 auStack_498 [1104];
  ulonglong uStack_48;
  
  uStack_48 = DAT_143a8b908 ^ (ulonglong)auStack_4d8;
  if (((param_2 == 0x202) && (*(int *)(param_1 + 0x1af4) != 0)) && (*(int *)(param_1 + 0x1af8) == 1)
     ) {
    uStack_4a8 = 0xffffffffffffffff;
    iVar4 = 0;
    iVar3 = 0x1a;
    do {
      iVar2 = 0;
      iVar1 = 0x2f;
      do {
        if (((iVar3 + -0xe <= param_4) && (param_4 < iVar3)) &&
           ((iVar1 + -0xe <= param_5 && (param_5 < iVar1)))) {
          uStack_4a8 = CONCAT44(iVar2,iVar4);
          if ((-1 < iVar4) && (-1 < iVar2)) {
            uStack_4a0 = uStack_4a8;
            FUN_1406ed520(auStack_498,0x17e);
            uStack_4a8 = CONCAT44(uStack_4a8._4_4_,0x1f);
            FUN_1406ede20(auStack_498,&uStack_4a8,4);
            FUN_1406ede20(auStack_498,&uStack_4a0,8);
            FUN_1406ed840(auStack_498,*(undefined1 *)(param_1 + 0x1aec));
            FUN_1415d01c0(auStack_498);
            FUN_1406ed610(auStack_498);
            (**(code **)(*(longlong *)(param_1 + -8) + 0x90))(param_1 + -8,0);
          }
          goto LAB_141e956e3;
        }
        iVar2 = iVar2 + 1;
        iVar1 = iVar1 + 0x18;
      } while (iVar1 < 0x197);
      iVar4 = iVar4 + 1;
      iVar3 = iVar3 + 0x18;
    } while (iVar3 < 0x182);
  }
LAB_141e956e3:
  iStack_4b8 = param_5;
  FUN_142bf2520(param_1,param_2,param_3,param_4);
  return;
}


