
//===========================================================
// FUN_141b054f0 @ 141b054f0   (16675 bytes)
//===========================================================

/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Function: __chkstk replaced with injection: alloca_probe */
/* WARNING: Removing unreachable block (ram,0x000141b05974) */

undefined4
FUN_141b054f0(longlong *param_1,IUnknown *param_2,longlong *param_3,longlong *param_4,int param_5,
             int param_6,short param_7,undefined1 param_8,short param_9,undefined1 param_10,
             int param_11,int param_12,int param_13,char param_14)

{
  char *pcVar1;
  char *pcVar2;
  code *pcVar3;
  bool bVar4;
  short *psVar5;
  undefined1 uVar6;
  char cVar7;
  int iVar8;
  uint uVar9;
  int iVar10;
  undefined4 uVar11;
  int iVar12;
  int *piVar13;
  undefined8 *puVar14;
  IUnknown *pIVar15;
  longlong *plVar16;
  longlong *plVar17;
  undefined8 uVar18;
  undefined2 *puVar19;
  IUnknown *pIVar20;
  IUnknown *pIVar21;
  longlong *plVar22;
  undefined8 *puVar23;
  longlong lVar24;
  ulonglong uVar25;
  longlong lVar26;
  short *psVar27;
  undefined *puVar28;
  undefined8 *puVar29;
  uint uVar30;
  uint uVar31;
  IUnknown *pIVar32;
  IUnknown *pIVar33;
  undefined1 *puVar34;
  IUnknown *pIVar35;
  IUnknown *pIVar36;
  undefined *puStack_310;
  undefined1 auStack_308 [32];
  longlong *local_2e8;
  longlong *local_2e0;
  int *local_2d8;
  longlong *local_2d0;
  uint *local_2c8;
  undefined8 *local_2c0;
  undefined8 *local_2b8;
  undefined8 *local_2b0;
  int local_2a8;
  char local_2a0;
  undefined8 local_288;
  longlong *plStack_280;
  undefined8 local_278;
  IUnknown *local_270;
  IUnknown *local_268 [2];
  undefined8 local_258;
  longlong *plStack_250;
  undefined8 local_248;
  IUnknown *local_238;
  undefined8 local_230;
  undefined4 uStack_228;
  undefined4 uStack_224;
  undefined8 local_220;
  short local_218;
  undefined2 uStack_216;
  undefined4 uStack_214;
  longlong *plStack_210;
  undefined8 local_208;
  int local_200;
  uint local_1fc;
  IUnknown *local_1f8;
  IUnknown *local_1f0;
  IUnknown *local_1e8;
  longlong *plStack_1e0;
  undefined8 local_1d8;
  undefined8 local_1c8;
  longlong *plStack_1c0;
  undefined8 local_1b8;
  undefined8 local_1a8;
  longlong *local_1a0;
  IUnknown *local_198;
  longlong *local_190;
  IUnknown *local_188;
  int local_180;
  int local_17c;
  short *local_178;
  IUnknown *local_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 local_148;
  undefined8 local_140;
  undefined8 local_138;
  undefined4 local_130;
  IUnknown *local_128;
  IUnknown *local_120;
  IUnknown *local_118;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  IUnknown *local_e8;
  undefined8 local_e0;
  undefined8 *local_d8;
  undefined8 uStack_d0;
  undefined **local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 local_b0;
  undefined4 local_a8;
  uint local_a4;
  undefined4 local_a0;
  undefined4 local_9c;
  undefined4 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  ulonglong local_48;
  
  puVar34 = auStack_308;
  local_48 = DAT_143a8b908 ^ (ulonglong)&local_288;
  local_190 = param_1;
  local_188 = param_2;
  if (*param_3 == 0) {
    param_4 = (longlong *)*param_4;
  }
  else {
    puStack_310 = (undefined *)0x141b0555e;
    iVar8 = FUN_141b0dff0(param_1,param_5);
    local_180 = iVar8;
    if (iVar8 < 1) {
LAB_141b055f4:
      pIVar36 = (IUnknown *)0x0;
      if ((param_5 == 1000) || (param_5 == 0x3f2)) {
        local_118 = (IUnknown *)CONCAT44(local_118._4_4_,1);
        if (param_6 != 1) goto LAB_141b0561d;
      }
      else {
LAB_141b0561d:
        local_118 = (IUnknown *)((ulonglong)local_118 & 0xffffffff00000000);
      }
      local_120 = (IUnknown *)0x0;
      pIVar33 = pIVar36;
      if (param_2 != (IUnknown *)0x0) {
        pIVar35 = (IUnknown *)0xffffffffffffffff;
        do {
          pIVar35 = pIVar35 + 1;
        } while (param_2[(longlong)pIVar35] != (IUnknown)0x0);
        iVar10 = (int)pIVar35;
        iVar8 = 0;
        if (0 < iVar10) {
          iVar8 = iVar10;
        }
        puStack_310 = (undefined *)0x141b05662;
        piVar13 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar8 + 0x11));
        piVar13[1] = iVar8;
        *piVar13 = -1;
        pIVar33 = (IUnknown *)(piVar13 + 4);
        piVar13[2] = 0;
        *pIVar33 = (IUnknown)0x0;
        puStack_310 = (undefined *)0x141b0568d;
        local_120 = pIVar33;
        FUN_142ef7ba0(pIVar33,param_2,(longlong)iVar10);
        if (*piVar13 != -1) {
          puStack_310 = (undefined *)0x141b056a0;
          FUN_142e52dd0(0x8b);
        }
        if ((iVar10 == -1) || (iVar10 <= piVar13[1])) {
          *piVar13 = 1;
          if (iVar10 != -1) goto LAB_141b056ca;
          pIVar35 = pIVar36;
          if (pIVar33 != (IUnknown *)0x0) {
            pIVar35 = (IUnknown *)0xffffffffffffffff;
            do {
              pIVar35 = pIVar35 + 1;
            } while (pIVar33[(longlong)pIVar35] != (IUnknown)0x0);
          }
        }
        else {
          puStack_310 = (undefined *)0x141b056c2;
          FUN_142e54290(0x90,piVar13[1],(ulonglong)pIVar35 & 0xffffffff);
          *piVar13 = 1;
LAB_141b056ca:
          pIVar33[iVar10] = (IUnknown)0x0;
        }
        iVar8 = (int)pIVar35;
        if ((iVar8 < 0) || (piVar13[1] + 1 <= iVar8)) {
          puStack_310 = (undefined *)0x141b056ea;
          FUN_142e54290(0x9c,(ulonglong)pIVar35 & 0xffffffff);
        }
        piVar13[2] = iVar8;
      }
      pIVar35 = pIVar33;
      if (param_5 == 0x3ef) {
        puStack_310 = (undefined *)0x141b05711;
        puVar14 = (undefined8 *)FUN_1408a9e40(&local_1a0,0xc2a);
        pcVar2 = (char *)*puVar14;
        if (pIVar33 == (IUnknown *)0x0) {
          pIVar35 = pIVar36;
          if ((pcVar2 == (char *)0x0) || (*pcVar2 == '\0')) {
LAB_141b0579c:
            puStack_310 = (undefined *)0x141b057b5;
            uVar18 = FUN_14019ce60(&local_120,&local_178,0,pIVar35);
            lVar24 = -1;
            do {
              pcVar1 = &DAT_1434b2af2 + lVar24;
              lVar24 = lVar24 + 1;
            } while (*pcVar1 != '\0');
            puStack_310 = (undefined *)0x141b057da;
            FUN_1401abc80(uVar18,local_268);
            pIVar15 = pIVar36;
            if (pcVar2 != (char *)0x0) {
              pIVar15 = (IUnknown *)0xffffffffffffffff;
              do {
                pIVar15 = pIVar15 + 1;
              } while (pcVar2[(longlong)pIVar15] != '\0');
            }
            puStack_310 = (undefined *)0x141b05812;
            plVar16 = (longlong *)
                      FUN_14019ce60(&local_120,&local_198,(int)pIVar35 + (int)pIVar15,0xffffffff);
            lVar24 = *plVar16;
            pIVar35 = pIVar36;
            if (lVar24 != 0) {
              pIVar35 = (IUnknown *)(ulonglong)*(uint *)(lVar24 + -8);
            }
            puStack_310 = (undefined *)0x141b05831;
            FUN_1401abc80(local_268,&local_238,lVar24,pIVar35);
            if (local_198 != (IUnknown *)0x0) {
              puStack_310 = (undefined *)0x141b05847;
              FUN_14019f2c0(local_198 + -0x10);
            }
            if (local_268[0] != (IUnknown *)0x0) {
              puStack_310 = (undefined *)0x141b0585a;
              FUN_14019f2c0(local_268[0] + -0x10);
            }
            if (local_178 != (short *)0x0) {
              puStack_310 = (undefined *)0x141b05870;
              FUN_14019f2c0(local_178 + -8);
            }
            local_188 = local_238;
            pIVar35 = local_238;
          }
          else {
            local_188 = (IUnknown *)0x0;
            local_238 = (IUnknown *)0x0;
          }
        }
        else {
          if ((pcVar2 == (char *)0x0) || (*pcVar2 == '\0')) {
            uVar9 = *(uint *)(pIVar33 + -8);
LAB_141b05793:
            pIVar35 = (IUnknown *)(ulonglong)uVar9;
            if (-1 < (int)uVar9) goto LAB_141b0579c;
          }
          else {
            puStack_310 = (undefined *)0x141b0577e;
            lVar24 = FUN_142ef6a98(pIVar33,pcVar2);
            if (lVar24 != 0) {
              uVar9 = (int)lVar24 - (int)pIVar33;
              goto LAB_141b05793;
            }
          }
          pIVar15 = (IUnknown *)0xffffffffffffffff;
          do {
            pIVar15 = pIVar15 + 1;
          } while (pIVar33[(longlong)pIVar15] != (IUnknown)0x0);
          iVar10 = (int)pIVar15;
          iVar8 = 0;
          if (0 < iVar10) {
            iVar8 = iVar10;
          }
          puStack_310 = (undefined *)0x141b058b3;
          piVar13 = (int *)FUN_14019b600(&DAT_143ad6a30,(longlong)(iVar8 + 0x11));
          piVar13[1] = iVar8;
          *piVar13 = -1;
          pIVar35 = (IUnknown *)(piVar13 + 4);
          piVar13[2] = 0;
          *pIVar35 = (IUnknown)0x0;
          puStack_310 = (undefined *)0x141b058e4;
          local_238 = pIVar35;
          local_188 = pIVar35;
          FUN_142ef7ba0(pIVar35,pIVar33,(longlong)iVar10);
          if (*piVar13 != -1) {
            puStack_310 = (undefined *)0x141b058f8;
            FUN_142e52dd0(0x8b);
          }
          if ((iVar10 == -1) || (iVar10 <= piVar13[1])) {
            *piVar13 = 1;
            if (iVar10 != -1) goto LAB_141b05924;
            pIVar15 = pIVar36;
            if (pIVar35 != (IUnknown *)0x0) {
              pIVar15 = (IUnknown *)0xffffffffffffffff;
              do {
                pIVar15 = pIVar15 + 1;
              } while (pIVar35[(longlong)pIVar15] != (IUnknown)0x0);
            }
          }
          else {
            puStack_310 = (undefined *)0x141b0591b;
            FUN_142e54290(0x90,piVar13[1],(ulonglong)pIVar15 & 0xffffffff);
            *piVar13 = 1;
LAB_141b05924:
            pIVar35[iVar10] = (IUnknown)0x0;
          }
          iVar8 = (int)pIVar15;
          if ((iVar8 < 0) || (piVar13[1] + 1 <= iVar8)) {
            puStack_310 = (undefined *)0x141b05946;
            FUN_142e54290(0x9c,(ulonglong)pIVar15 & 0xffffffff);
          }
          piVar13[2] = iVar8;
        }
        if (pIVar33 != (IUnknown *)0x0) {
          puStack_310 = (undefined *)0x141b0595f;
          FUN_14019f2c0(pIVar33 + -0x10);
        }
        local_120 = pIVar35;
        if (local_1a0 != (longlong *)0x0) {
          puStack_310 = (undefined *)0x141b05993;
          FUN_14019f2c0(local_1a0 + -2);
        }
      }
      local_1f0 = (IUnknown *)0x0;
      local_e0._4_4_ = 0;
      local_d8 = (undefined8 *)0x0;
      uStack_d0 = 0;
      local_17c = 0;
      local_1fc = 0;
      local_148 = (IUnknown *)((ulonglong)local_148._4_4_ << 0x20);
      local_1a8 = (undefined2 *)((ulonglong)local_1a8._4_4_ << 0x20);
      local_200 = 0;
      local_138 = (IUnknown *)((ulonglong)local_138._4_4_ << 0x20);
      local_140 = (ulonglong)local_140._4_4_ << 0x20;
      local_178 = (short *)0x0;
      puVar28 = PTR_u_UI_NameTag_img__d_143a46b98;
      if (((param_5 == 1000) || (param_5 == 0x3f2)) ||
         (puVar28 = PTR_u_UI_NameTag_img_pet__d_143a46ba0, param_5 == 0x3eb)) {
        puStack_310 = (undefined *)0x141b05ada;
        FUN_1401c21c0(&local_178,puVar28,param_6);
      }
      else if (param_5 == 0x3ef) {
        puStack_310 = (undefined *)0x141b05a5c;
        puVar14 = (undefined8 *)FUN_1408a9d20(local_268,0x594);
        puStack_310 = (undefined *)0x141b05a6f;
        FUN_1401c21c0(&local_178,*puVar14,param_6);
        if (local_268[0] != (IUnknown *)0x0) {
          puStack_310 = (undefined *)0x141b05a82;
          FUN_1401bebb0(local_268[0] + -0x10);
        }
      }
      else if (param_5 == 0x3f1) {
        puStack_310 = (undefined *)0x141b05a9b;
        puVar14 = (undefined8 *)FUN_1408a9d20(local_268,0x595);
        puStack_310 = (undefined *)0x141b05aae;
        FUN_1401c21c0(&local_178,*puVar14,param_6);
        if (local_268[0] != (IUnknown *)0x0) {
          puStack_310 = (undefined *)0x141b05ac1;
          FUN_1401bebb0(local_268[0] + -0x10);
        }
      }
      psVar5 = local_178;
      pIVar33 = DAT_143add058;
      local_e8 = (IUnknown *)0x0;
      local_130 = 2;
      if ((local_178 == (short *)0x0) || (*local_178 == 0)) {
LAB_141b06043:
        pIVar33 = (IUnknown *)0x0;
        local_1f8 = (IUnknown *)0x0;
        if (DAT_143ad48a0 == (code *)0x0) {
          iVar8 = -0x7ffbfe10;
LAB_141b092c7:
          puStack_310 = (undefined *)0x141b092db;
          FUN_1401a59c0(&local_108,iVar8,0,0);
                    /* WARNING: Subroutine does not return */
          puStack_310 = &UNK_141b092ee;
          _CxxThrowException(&local_108,(ThrowInfo *)&DAT_143a3b0c0);
        }
        puStack_310 = (undefined *)0x141b06074;
        iVar8 = (*DAT_143ad48a0)(PTR_u_Canvas_Font_143a47cc0,&DAT_143297250,&local_1f8,0);
        pIVar15 = local_1f8;
        if (iVar8 < 0) goto LAB_141b092c7;
        local_138 = (IUnknown *)CONCAT44(local_138._4_4_,0xa0000000);
        switch(param_5) {
        case 0x3e9:
          if (((local_190 == (longlong *)0x0) || (local_190[0x33] == 0)) ||
             (pIVar32 = *(IUnknown **)(local_190[0x33] + 0x18), pIVar32 == (IUnknown *)0x0)) {
            if (local_1f8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              puStack_310 = &UNK_141b0933a;
              FUN_142ef3ac0(0x80004003);
            }
            puStack_310 = (undefined *)0x141b0611e;
            (*DAT_143262a20)(&local_1e8);
            if (DAT_143a8b8d8 == 8) {
              if ((short)local_1e8 == 8) {
                local_1e8 = (IUnknown *)((ulonglong)local_1e8._2_6_ << 0x10);
                if (plStack_1e0 != (longlong *)0x0) {
                  puStack_310 = (undefined *)0x141b0614f;
                  (*DAT_143ad5990)((longlong)plStack_1e0 + -4);
                }
              }
              else {
                puStack_310 = (undefined *)0x141b0615e;
                iVar8 = (*DAT_143262a18)(&local_1e8);
                if (iVar8 < 0) goto LAB_141b09328;
              }
              local_1e8 = (IUnknown *)CONCAT62(local_1e8._2_6_,8);
              if (DAT_143a8b8e0 == 0) {
                puStack_310 = (undefined *)0x141b06185;
                plStack_1e0 = (longlong *)FUN_1401a5fa0(0,0);
              }
              else {
                puStack_310 = (undefined *)0x141b06198;
                plStack_1e0 = (longlong *)
                              FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
              }
            }
            else {
              if (((short)local_1e8 == 8) &&
                 (local_1e8 = (IUnknown *)((ulonglong)local_1e8._2_6_ << 0x10),
                 plStack_1e0 != (longlong *)0x0)) {
                puStack_310 = (undefined *)0x141b061c8;
                (*DAT_143ad5990)((longlong)plStack_1e0 + -4);
              }
              puStack_310 = (undefined *)0x141b061dc;
              iVar8 = (*DAT_143262a28)(&local_1e8,&DAT_143a8b8d8);
              if (iVar8 < 0) {
LAB_141b09328:
                    /* WARNING: Subroutine does not return */
                puStack_310 = &UNK_141b0932f;
                FUN_142ef3ac0(iVar8);
              }
            }
            puStack_310 = (undefined *)0x141b061f3;
            (*DAT_143262a20)(&local_218);
            if (DAT_143a8b8d8 == 8) {
              if (local_218 == 8) {
                local_218 = 0;
                if (plStack_210 != (longlong *)0x0) {
                  puStack_310 = (undefined *)0x141b0621b;
                  (*DAT_143ad5990)((longlong)plStack_210 + -4);
                }
              }
              else {
                puStack_310 = (undefined *)0x141b06227;
                iVar8 = (*DAT_143262a18)(&local_218);
                if (iVar8 < 0) goto LAB_141b092ef;
              }
              local_218 = 8;
              pIVar32 = pIVar33;
              if (DAT_143a8b8e0 != 0) {
                pIVar32 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
              }
              puStack_310 = (undefined *)0x141b0624d;
              plStack_210 = (longlong *)FUN_1401a5fa0(DAT_143a8b8e0,pIVar32);
            }
            else {
              if ((local_218 == 8) && (local_218 = 0, plStack_210 != (longlong *)0x0)) {
                puStack_310 = (undefined *)0x141b06298;
                (*DAT_143ad5990)((longlong)plStack_210 + -4);
              }
              puStack_310 = (undefined *)0x141b062a9;
              iVar8 = (*DAT_143262a28)(&local_218,&DAT_143a8b8d8);
              if (iVar8 < 0) {
LAB_141b092ef:
                    /* WARNING: Subroutine does not return */
                puStack_310 = &UNK_141b092f6;
                FUN_142ef3ac0(iVar8);
              }
            }
            puStack_310 = (undefined *)0x141b0625f;
            pIVar21 = (IUnknown *)FUN_1408a9d80(local_268,0x1800);
            pIVar32 = pIVar33;
            if (*(IUnknown **)pIVar21 != (IUnknown *)0x0) {
              pIVar32 = *(IUnknown **)*(IUnknown **)pIVar21;
            }
            local_168 = local_1e8;
            uStack_160 = plStack_1e0;
            local_158 = local_1d8;
            local_258 = (IUnknown *)CONCAT44(uStack_214,CONCAT22(uStack_216,local_218));
            plStack_250 = plStack_210;
            local_248 = local_208;
            local_2e0 = &local_168;
            local_2e8 = &local_258;
            puStack_310 = (undefined *)0x141b06310;
            local_270 = pIVar21;
            iVar8 = (**(code **)(*(longlong *)pIVar15 + 0x18))(pIVar15,pIVar32,0xc,0xffffff00);
            if (iVar8 < 0) {
              puStack_310 = (undefined *)0x141b06325;
              _com_issue_errorex(iVar8,pIVar15,(_GUID *)&DAT_143297250);
            }
            puStack_310 = (undefined *)0x141b0632e;
            thunk_FUN_1401be120(pIVar21);
            if (local_218 == 8) {
              local_218 = 0;
              if (plStack_210 != (longlong *)0x0) {
                puStack_310 = (undefined *)0x141b0634d;
                (*DAT_143ad5990)((longlong)plStack_210 + -4);
              }
            }
            else {
              puStack_310 = (undefined *)0x141b06359;
              (*DAT_143262a18)(&local_218);
            }
            if ((short)local_1e8 == 8) {
              local_1e8 = (IUnknown *)((ulonglong)local_1e8 & 0xffffffffffff0000);
              pIVar15 = local_1f8;
              if (plStack_1e0 != (longlong *)0x0) {
                puStack_310 = (undefined *)0x141b06381;
                (*DAT_143ad5990)((longlong)plStack_1e0 + -4);
                pIVar15 = local_1f8;
              }
            }
            else {
              puStack_310 = (undefined *)0x141b06390;
              (*DAT_143262a18)(&local_1e8);
              pIVar15 = local_1f8;
            }
          }
          else if (local_1f8 != pIVar32) {
            puStack_310 = (undefined *)0x141b060e1;
            (**(code **)(*(longlong *)pIVar32 + 8))(pIVar32);
            pIVar15 = pIVar32;
            if (local_1f8 != (IUnknown *)0x0) {
              lVar24 = *(longlong *)local_1f8;
              puStack_310 = (undefined *)0x141b060fb;
              local_1f8 = pIVar32;
              (**(code **)(lVar24 + 0x10))();
              pIVar15 = local_1f8;
            }
          }
          break;
        case 0x3ea:
          if (((local_190 == (longlong *)0x0) || (local_190[0x33] == 0)) ||
             (pIVar32 = *(IUnknown **)(local_190[0x33] + 0x20), pIVar32 == (IUnknown *)0x0)) {
            if (local_1f8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              puStack_310 = &UNK_141b09355;
              FUN_142ef3ac0(0x80004003);
            }
            puStack_310 = (undefined *)0x141b06407;
            (*DAT_143262a20)(&local_230);
            puStack_310 = (undefined *)0x141b06417;
            iVar8 = FUN_14023c4c0(&local_230,&DAT_143a8b8d8);
            if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
              puStack_310 = &UNK_141b09342;
              FUN_142ef3ac0(iVar8);
            }
            puStack_310 = (undefined *)0x141b06429;
            (*DAT_143262a20)(&local_288);
            puStack_310 = (undefined *)0x141b06439;
            iVar8 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
            if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
              puStack_310 = &UNK_141b0934a;
              FUN_142ef3ac0(iVar8);
            }
            puStack_310 = (undefined *)0x141b0644f;
            uVar18 = FUN_1408a9d80(local_268,0x1800);
            local_2e0 = &local_230;
            puStack_310 = (undefined *)0x141b06478;
            local_2e8 = &local_288;
            FUN_140ee60f0(pIVar15,uVar18,0xd,0xffffff00);
            if ((short)local_288 == 8) {
              local_288._0_2_ = 0;
              if (plStack_280 != (longlong *)0x0) {
                puStack_310 = (undefined *)0x141b06497;
                (*DAT_143ad5990)((longlong)plStack_280 + -4);
              }
            }
            else {
              puStack_310 = (undefined *)0x141b064a3;
              (*DAT_143262a18)(&local_288);
            }
            if ((short)local_230 == 8) {
              local_230._0_2_ = 0;
              pIVar15 = local_1f8;
              if (CONCAT44(uStack_224,uStack_228) != 0) {
                puStack_310 = (undefined *)0x141b064c2;
                (*DAT_143ad5990)(CONCAT44(uStack_224,uStack_228) + -4);
                pIVar15 = local_1f8;
              }
            }
            else {
              puStack_310 = (undefined *)0x141b064ce;
              (*DAT_143262a18)(&local_230);
              pIVar15 = local_1f8;
            }
          }
          else if (local_1f8 != pIVar32) {
            puStack_310 = (undefined *)0x141b063cd;
            (**(code **)(*(longlong *)pIVar32 + 8))(pIVar32);
            pIVar15 = pIVar32;
            if (local_1f8 != (IUnknown *)0x0) {
              lVar24 = *(longlong *)local_1f8;
              puStack_310 = (undefined *)0x141b063e7;
              local_1f8 = pIVar32;
              (**(code **)(lVar24 + 0x10))();
              pIVar15 = local_1f8;
            }
          }
          break;
        default:
          if ((int)local_118 == 0) {
            puStack_310 = (undefined *)0x141b06955;
            puVar14 = (undefined8 *)FUN_1429fbeb0(local_268,0);
            pIVar32 = local_1f8;
            pIVar15 = (IUnknown *)*puVar14;
            pIVar21 = local_1f8;
            if ((local_1f8 != pIVar15) &&
               (*puVar14 = 0, pIVar21 = pIVar15, local_1f8 != (IUnknown *)0x0)) {
              lVar24 = *(longlong *)local_1f8;
              puStack_310 = (undefined *)0x141b0697d;
              local_1f8 = pIVar15;
              (**(code **)(lVar24 + 0x10))(pIVar32);
              pIVar21 = local_1f8;
            }
            local_1f8 = pIVar21;
            pIVar15 = local_1f8;
            if (local_268[0] != (IUnknown *)0x0) {
              puStack_310 = (undefined *)0x141b0698d;
              (**(code **)(*(longlong *)local_268[0] + 0x10))();
              pIVar15 = local_1f8;
            }
          }
          else {
            if (local_1f8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              puStack_310 = &UNK_141b093b4;
              FUN_142ef3ac0(0x80004003);
            }
            puStack_310 = (undefined *)0x141b06887;
            (*DAT_143262a20)(&local_288);
            puStack_310 = (undefined *)0x141b06897;
            iVar8 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
            if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
              puStack_310 = &UNK_141b093a9;
              FUN_142ef3ac0(iVar8);
            }
            puStack_310 = (undefined *)0x141b068af;
            FUN_140ed5680(&local_258,PTR_DAT_143a47cd0);
            puStack_310 = (undefined *)0x141b068be;
            uVar18 = FUN_1408a9d80(local_268,0x1800);
            local_2e8 = &local_258;
            puStack_310 = (undefined *)0x141b068e7;
            local_2e0 = &local_288;
            FUN_140ee60f0(pIVar15,uVar18,0xc,0xff000000);
            if ((short)local_258 == 8) {
              local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
              if (plStack_250 != (longlong *)0x0) {
                puStack_310 = (undefined *)0x141b06906;
                (*DAT_143ad5990)((longlong)plStack_250 + -4);
              }
            }
            else {
              puStack_310 = (undefined *)0x141b06912;
              (*DAT_143262a18)(&local_258);
            }
            if ((short)local_288 == 8) {
              local_288._0_2_ = 0;
              if (plStack_280 != (longlong *)0x0) {
                puStack_310 = (undefined *)0x141b06931;
                (*DAT_143ad5990)((longlong)plStack_280 + -4);
              }
            }
            else {
              puStack_310 = (undefined *)0x141b0693d;
              (*DAT_143262a18)(&local_288);
            }
            local_138 = (IUnknown *)CONCAT44(local_138._4_4_,0xff2c80ad);
            pIVar15 = local_1f8;
          }
          break;
        case 0x3ec:
        case 0x3ed:
          if (local_1f8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
            puStack_310 = &UNK_141b09368;
            FUN_142ef3ac0(0x80004003);
          }
          puStack_310 = (undefined *)0x141b064ee;
          (*DAT_143262a20)(&local_288);
          puStack_310 = (undefined *)0x141b064fe;
          iVar8 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
          if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
            puStack_310 = &UNK_141b0935d;
            FUN_142ef3ac0(iVar8);
          }
          puStack_310 = (undefined *)0x141b06516;
          FUN_140ed5680(&local_258,PTR_DAT_143a47cc8);
          puStack_310 = (undefined *)0x141b06525;
          uVar18 = FUN_1408a9d80(local_268,0x1800);
          local_2e8 = &local_258;
          puStack_310 = (undefined *)0x141b0654e;
          local_2e0 = &local_288;
          FUN_140ee60f0(pIVar15,uVar18,0xd,0xffffffff);
          if ((short)local_258 == 8) {
            local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
            if (plStack_250 != (longlong *)0x0) {
              puStack_310 = (undefined *)0x141b0656d;
              (*DAT_143ad5990)((longlong)plStack_250 + -4);
            }
          }
          else {
            puStack_310 = (undefined *)0x141b06579;
            (*DAT_143262a18)(&local_258);
          }
          if ((short)local_288 == 8) {
            local_288._0_2_ = 0;
            pIVar15 = local_1f8;
            if (plStack_280 != (longlong *)0x0) {
              puStack_310 = (undefined *)0x141b06598;
              (*DAT_143ad5990)((longlong)plStack_280 + -4);
              pIVar15 = local_1f8;
            }
          }
          else {
            puStack_310 = (undefined *)0x141b065a4;
            (*DAT_143262a18)(&local_288);
            pIVar15 = local_1f8;
          }
          break;
        case 0x3ee:
          if (local_1f8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
            puStack_310 = &UNK_141b0937b;
            FUN_142ef3ac0(0x80004003);
          }
          puStack_310 = (undefined *)0x141b065c4;
          (*DAT_143262a20)(&local_288);
          puStack_310 = (undefined *)0x141b065d4;
          iVar8 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
          if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
            puStack_310 = &UNK_141b09370;
            FUN_142ef3ac0(iVar8);
          }
          puStack_310 = (undefined *)0x141b065ec;
          FUN_140ed5680(&local_258,PTR_DAT_143a47cc8);
          puStack_310 = (undefined *)0x141b065fb;
          uVar18 = FUN_1408a9d80(local_268,0x1800);
          local_2e8 = &local_258;
          puStack_310 = (undefined *)0x141b06624;
          local_2e0 = &local_288;
          FUN_140ee60f0(pIVar15,uVar18,0xd,0xffffffff);
          if ((short)local_258 == 8) {
            local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
            if (plStack_250 != (longlong *)0x0) {
              puStack_310 = (undefined *)0x141b06643;
              (*DAT_143ad5990)((longlong)plStack_250 + -4);
            }
          }
          else {
            puStack_310 = (undefined *)0x141b0664f;
            (*DAT_143262a18)(&local_258);
          }
          if ((short)local_288 == 8) {
            local_288._0_2_ = 0;
            pIVar15 = local_1f8;
            if (plStack_280 != (longlong *)0x0) {
              puStack_310 = (undefined *)0x141b0666e;
              (*DAT_143ad5990)((longlong)plStack_280 + -4);
              pIVar15 = local_1f8;
            }
          }
          else {
            puStack_310 = (undefined *)0x141b0667a;
            (*DAT_143262a18)(&local_288);
            pIVar15 = local_1f8;
          }
          break;
        case 0x3f0:
          if (param_6 == 0) {
            if (local_1f8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              puStack_310 = &UNK_141b0938e;
              FUN_142ef3ac0(0x80004003);
            }
            puStack_310 = (undefined *)0x141b066a3;
            (*DAT_143262a20)(&local_288);
            puStack_310 = (undefined *)0x141b066b3;
            iVar8 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
            if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
              puStack_310 = &UNK_141b09383;
              FUN_142ef3ac0(iVar8);
            }
            puStack_310 = (undefined *)0x141b066cb;
            FUN_140ed5680(&local_258,PTR_DAT_143a47cc8);
            puStack_310 = (undefined *)0x141b066da;
            uVar18 = FUN_1408a9d80(local_268,0x1800);
            local_2e8 = &local_258;
            puStack_310 = (undefined *)0x141b06701;
            local_2e0 = &local_288;
            FUN_140ee60f0(pIVar15,uVar18,0xd,0xffff5533);
            if ((short)local_258 == 8) {
              local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
              if (plStack_250 != (longlong *)0x0) {
                puStack_310 = (undefined *)0x141b06720;
                (*DAT_143ad5990)((longlong)plStack_250 + -4);
              }
            }
            else {
              puStack_310 = (undefined *)0x141b0672c;
              (*DAT_143262a18)(&local_258);
            }
          }
          else {
            if (param_6 != 1) {
              puStack_310 = (undefined *)0x141b06822;
              puVar14 = (undefined8 *)FUN_1429fbeb0(local_268,0);
              pIVar32 = local_1f8;
              pIVar15 = (IUnknown *)*puVar14;
              pIVar21 = local_1f8;
              if ((local_1f8 != pIVar15) &&
                 (*puVar14 = 0, pIVar21 = pIVar15, local_1f8 != (IUnknown *)0x0)) {
                lVar24 = *(longlong *)local_1f8;
                puStack_310 = (undefined *)0x141b0684a;
                local_1f8 = pIVar15;
                (**(code **)(lVar24 + 0x10))(pIVar32);
                pIVar21 = local_1f8;
              }
              local_1f8 = pIVar21;
              pIVar15 = local_1f8;
              if (local_268[0] != (IUnknown *)0x0) {
                puStack_310 = (undefined *)0x141b0685a;
                (**(code **)(*(longlong *)local_268[0] + 0x10))();
                pIVar15 = local_1f8;
              }
              break;
            }
            if (local_1f8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              puStack_310 = &UNK_141b093a1;
              FUN_142ef3ac0(0x80004003);
            }
            puStack_310 = (undefined *)0x141b0677c;
            (*DAT_143262a20)(&local_288);
            puStack_310 = (undefined *)0x141b0678c;
            iVar8 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
            if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
              puStack_310 = &UNK_141b09396;
              FUN_142ef3ac0(iVar8);
            }
            puStack_310 = (undefined *)0x141b067a4;
            FUN_140ed5680(&local_258,PTR_DAT_143a47cc8);
            puStack_310 = (undefined *)0x141b067b3;
            uVar18 = FUN_1408a9d80(local_268,0x1800);
            local_2e8 = &local_258;
            puStack_310 = (undefined *)0x141b067da;
            local_2e0 = &local_288;
            FUN_140ee60f0(pIVar15,uVar18,0xd,0xff4488ff);
            if ((short)local_258 == 8) {
              local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
              if (plStack_250 != (longlong *)0x0) {
                puStack_310 = (undefined *)0x141b067f9;
                (*DAT_143ad5990)((longlong)plStack_250 + -4);
              }
            }
            else {
              puStack_310 = (undefined *)0x141b06805;
              (*DAT_143262a18)(&local_258);
            }
          }
          if ((short)local_288 == 8) {
            local_288._0_2_ = 0;
            pIVar15 = local_1f8;
            if (plStack_280 != (longlong *)0x0) {
              puStack_310 = (undefined *)0x141b06753;
              (*DAT_143ad5990)((longlong)plStack_280 + -4);
              pIVar15 = local_1f8;
            }
          }
          else {
            puStack_310 = (undefined *)0x141b06815;
            (*DAT_143262a18)(&local_288);
            pIVar15 = local_1f8;
          }
        }
        local_1f8 = pIVar15;
        pIVar15 = local_1f8;
        if (local_1f8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          puStack_310 = (undefined *)0x141b09301;
          FUN_142ef3ac0(0x80004003);
        }
        puStack_310 = (undefined *)0x141b069a8;
        (*DAT_143262a20)(&local_288);
        puStack_310 = (undefined *)0x141b069b8;
        iVar8 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
        pIVar32 = local_188;
        if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
          puStack_310 = (undefined *)0x141b093bc;
          FUN_142ef3ac0(iVar8);
        }
        puStack_310 = (undefined *)0x141b069d3;
        uVar18 = FUN_1401a5780(local_268,local_188);
        puStack_310 = (undefined *)0x141b069e2;
        local_1fc = FUN_140ee5fb0(pIVar15,uVar18,&local_288);
        if ((short)local_288 == 8) {
          local_288._0_2_ = 0;
          if (plStack_280 != (longlong *)0x0) {
            puStack_310 = (undefined *)0x141b06a06;
            (*DAT_143ad5990)((longlong)plStack_280 + -4);
          }
        }
        else {
          puStack_310 = (undefined *)0x141b06a12;
          (*DAT_143262a18)(&local_288);
        }
        local_140 = CONCAT44(local_140._4_4_,0xff);
        local_268[0] = (IUnknown *)0x0;
        pIVar15 = pIVar33;
        if (param_5 == 0x3ec) {
          if (param_7 == 0) {
            if (param_9 == 0) {
              if (param_11 == 0) goto LAB_141b06b96;
            }
            else {
LAB_141b06a78:
              if (param_11 == 0) goto LAB_141b06b30;
            }
            puStack_310 = (undefined *)0x141b06a98;
            puVar14 = (undefined8 *)FUN_142cc0be0(DAT_143aa84a0,&local_238);
            pIVar33 = (IUnknown *)*puVar14;
            if (pIVar33 != (IUnknown *)0x0) {
              *puVar14 = 0;
              pIVar15 = pIVar33;
              local_268[0] = pIVar33;
            }
            if (local_238 != (IUnknown *)0x0) {
              puStack_310 = (undefined *)0x141b06aba;
              (**(code **)(*(longlong *)local_238 + 0x10))();
            }
            local_140 = CONCAT44(local_140._4_4_,0xffffffff);
          }
          else {
            if ((param_7 + 0xe000U & 0xdfff) != 0) goto LAB_141b06a78;
            local_2e0 = (longlong *)CONCAT71(local_2e0._1_7_,param_8);
            local_2e8 = (longlong *)CONCAT62(local_2e8._2_6_,param_7);
            puStack_310 = (undefined *)0x141b06af6;
            puVar14 = (undefined8 *)FUN_142dde270(DAT_143aa84a0,&local_238,param_9,param_10);
            pIVar33 = (IUnknown *)*puVar14;
            if (pIVar33 != (IUnknown *)0x0) {
              *puVar14 = 0;
              pIVar15 = pIVar33;
              local_268[0] = pIVar33;
            }
            if (local_238 != (IUnknown *)0x0) {
              puStack_310 = (undefined *)0x141b06b18;
              (**(code **)(*(longlong *)local_238 + 0x10))();
            }
          }
LAB_141b06b7f:
          if (pIVar15 != (IUnknown *)0x0) {
            local_200 = 0x12;
          }
        }
        else if (((param_5 - 0x3edU & 0xfffffffc) == 0) &&
                (pIVar15 = (IUnknown *)0x0, param_5 != 0x3ef)) {
LAB_141b06b30:
          local_2e0 = (longlong *)CONCAT71(local_2e0._1_7_,param_8);
          local_2e8 = (longlong *)CONCAT62(local_2e8._2_6_,param_7);
          puStack_310 = (undefined *)0x141b06b5c;
          puVar14 = (undefined8 *)FUN_142dde270(DAT_143aa84a0,&local_238,param_9,param_10);
          pIVar21 = (IUnknown *)*puVar14;
          pIVar15 = pIVar33;
          if (pIVar21 != (IUnknown *)0x0) {
            *puVar14 = 0;
            pIVar15 = pIVar21;
            local_268[0] = pIVar21;
          }
          if (local_238 != (IUnknown *)0x0) {
            puStack_310 = (undefined *)0x141b06b7e;
            (**(code **)(*(longlong *)local_238 + 0x10))();
          }
          goto LAB_141b06b7f;
        }
LAB_141b06b96:
        local_1a0 = (longlong *)0x0;
        puVar34 = auStack_308;
        if (param_5 == 1000) {
          if (param_14 == '\0') {
            puVar34 = auStack_308;
            if (-1 < param_13) {
              local_238 = (IUnknown *)0x0;
              puStack_310 = (undefined *)0x141b071e4;
              FUN_14019ba10(&local_238,"Effect/BasicEff.img/FamilyEmblem/%d",param_13);
              if (param_13 - 3U < 4) {
                puStack_310 = (undefined *)0x141b071fc;
                cVar7 = (**(code **)(*local_190 + 0x20))(local_190);
                uVar18 = 0x4d;
                if (cVar7 != '\0') {
                  uVar18 = 0x46;
                }
                puStack_310 = (undefined *)0x141b0720d;
                FUN_141b0e890(&local_238,uVar18);
              }
              pIVar33 = DAT_143add058;
              if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
                puStack_310 = (undefined *)0x141b09464;
                FUN_142ef3ac0(0x80004003);
              }
              puStack_310 = (undefined *)0x141b0722a;
              (*DAT_143262a20)(&local_1c8);
              puStack_310 = (undefined *)0x141b0723d;
              iVar8 = FUN_14023c4c0(&local_1c8,&DAT_143a8b8d8);
              if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
                puStack_310 = (undefined *)0x141b09418;
                FUN_142ef3ac0(iVar8);
              }
              puStack_310 = (undefined *)0x141b0724f;
              (*DAT_143262a20)(&local_258);
              puStack_310 = (undefined *)0x141b0725f;
              iVar8 = FUN_14023c4c0(&local_258,&DAT_143a8b8d8);
              pIVar21 = local_238;
              if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
                puStack_310 = (undefined *)0x141b09420;
                FUN_142ef3ac0(iVar8);
              }
              if (local_238 == (IUnknown *)0x0) {
                iVar8 = 2;
              }
              else {
                local_2e0 = (longlong *)((ulonglong)local_2e0 & 0xffffffff00000000);
                local_2e8 = (longlong *)0x0;
                puStack_310 = (undefined *)0x141b07294;
                iVar8 = (*DAT_1432627f8)(0xfde9,0,local_238,0xffffffff);
                iVar8 = iVar8 * 2;
              }
              uVar25 = (longlong)iVar8 + 0xf;
              if (uVar25 <= (ulonglong)(longlong)iVar8) {
                uVar25 = 0xffffffffffffff0;
              }
              puStack_310 = (undefined *)0x141b072b7;
              lVar24 = -(uVar25 & 0xfffffffffffffff0);
              pIVar32 = (IUnknown *)((longlong)&local_288 + lVar24);
              local_270 = pIVar32;
              if (pIVar21 == (IUnknown *)0x0) {
                if (pIVar32 != (IUnknown *)0x0) {
                  *(undefined2 *)pIVar32 = 0;
                }
              }
              else {
                *(undefined4 *)((longlong)&local_2e0 + lVar24) = 0x100000;
                *(IUnknown **)((longlong)&local_2e8 + lVar24) = pIVar32;
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b072fa;
                (*DAT_1432627f8)(0xfde9,0,pIVar21,0xffffffff);
                pIVar32 = local_270;
              }
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b0730d;
              uVar18 = FUN_1401a5890(&local_1e8,pIVar32);
              *(undefined8 **)((longlong)&local_2e8 + lVar24) = &local_1c8;
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b0732f;
              uVar18 = FUN_140403ec0(pIVar33,&local_168,uVar18,&local_258);
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b0733f;
              uVar18 = FUN_1409339d0(&local_128,uVar18);
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b0734c;
              FUN_1403ee040(&local_270,uVar18);
              if (local_128 != (IUnknown *)0x0) {
                pcVar3 = *(code **)(*(longlong *)local_128 + 0x10);
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b0735f;
                (*pcVar3)();
              }
              if ((short)local_168 == 8) {
                local_168 = (IUnknown *)((ulonglong)local_168 & 0xffffffffffff0000);
                if (uStack_160 != (longlong *)0x0) {
                  lVar26 = (longlong)uStack_160 + -4;
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b07387;
                  (*DAT_143ad5990)(lVar26);
                }
              }
              else {
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b07396;
                (*DAT_143262a18)(&local_168);
              }
              if ((short)local_258 == 8) {
                local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
                if (plStack_250 != (longlong *)0x0) {
                  lVar26 = (longlong)plStack_250 + -4;
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b073b5;
                  (*DAT_143ad5990)(lVar26);
                }
              }
              else {
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b073c1;
                (*DAT_143262a18)(&local_258);
              }
              if ((short)local_1c8 == 8) {
                local_1c8 = (IUnknown *)((ulonglong)local_1c8 & 0xffffffffffff0000);
                if (plStack_1c0 != (longlong *)0x0) {
                  lVar26 = (longlong)plStack_1c0 + -4;
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b073e9;
                  (*DAT_143ad5990)(lVar26);
                }
              }
              else {
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b073f8;
                (*DAT_143262a18)(&local_1c8);
              }
              pIVar33 = local_270;
              if (local_270 != (IUnknown *)0x0) {
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b0741c;
                FUN_140295e30(PTR_u_Canvas_143a479d8,&local_1a0,0);
                plVar16 = local_1a0;
                if (local_1a0 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b09459;
                  FUN_142ef3ac0(0x80004003);
                }
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b07436;
                (*DAT_143262a20)(&local_218);
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b07446;
                iVar8 = FUN_14023c4c0(&local_218,&DAT_143a8b8d8);
                if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b09428;
                  FUN_142ef3ac0(iVar8);
                }
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b07458;
                (*DAT_143262a20)(&local_230);
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b07468;
                iVar8 = FUN_14023c4c0(&local_230,&DAT_143a8b8d8);
                if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b09430;
                  FUN_142ef3ac0(iVar8);
                }
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b0747a;
                (*DAT_143262a20)(&local_288);
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b0748a;
                iVar8 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
                if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b09438;
                  FUN_142ef3ac0(iVar8);
                }
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b0749a;
                uVar11 = FUN_140404240(pIVar33);
                local_1a8 = (undefined2 *)CONCAT44(local_1a8._4_4_,uVar11);
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b074a8;
                uVar11 = FUN_140404280(pIVar33);
                *(short **)((longlong)&local_2e0 + lVar24) = &local_218;
                *(undefined8 **)((longlong)&local_2e8 + lVar24) = &local_230;
                uVar25 = (ulonglong)local_1a8 & 0xffffffff;
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b074cf;
                FUN_140cf0040(plVar16,uVar11,uVar25,&local_288);
                if ((short)local_288 == 8) {
                  local_288._0_2_ = 0;
                  if (plStack_280 != (longlong *)0x0) {
                    lVar26 = (longlong)plStack_280 + -4;
                    *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b074f0;
                    (*DAT_143ad5990)(lVar26);
                  }
                }
                else {
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b074fc;
                  (*DAT_143262a18)(&local_288);
                }
                if ((short)local_230 == 8) {
                  local_230._0_2_ = 0;
                  lVar26 = CONCAT44(uStack_224,uStack_228);
                  if (lVar26 != 0) {
                    *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b0751c;
                    (*DAT_143ad5990)(lVar26 + -4);
                  }
                }
                else {
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b07528;
                  (*DAT_143262a18)(&local_230);
                }
                if (local_218 == 8) {
                  local_218 = 0;
                  if (plStack_210 != (longlong *)0x0) {
                    lVar26 = (longlong)plStack_210 + -4;
                    *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b07547;
                    (*DAT_143ad5990)(lVar26);
                  }
                }
                else {
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b07553;
                  (*DAT_143262a18)(&local_218);
                }
                plVar16 = local_1a0;
                if (local_1a0 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
                  *(undefined **)(auStack_308 + lVar24 + -8) = &UNK_141b0944e;
                  FUN_142ef3ac0(0x80004003);
                }
                local_288._0_2_ = 3;
                plStack_280 = (longlong *)CONCAT44(plStack_280._4_4_,0xff);
                *(undefined8 **)((longlong)&local_2e8 + lVar24) = &local_288;
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b0758a;
                FUN_140ceffd0(plVar16,0,0,pIVar33);
                if ((short)local_288 == 8) {
                  local_288._0_2_ = 0;
                  if (plStack_280 != (longlong *)0x0) {
                    lVar26 = (longlong)plStack_280 + -4;
                    *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b075a9;
                    (*DAT_143ad5990)(lVar26);
                  }
                }
                else {
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b075b5;
                  (*DAT_143262a18)(&local_288);
                }
                if (local_1a0 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
                  *(undefined **)(auStack_308 + lVar24 + -8) = &UNK_141b09443;
                  FUN_142ef3ac0(0x80004003);
                }
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b075cb;
                local_200 = FUN_140404280();
                local_200 = local_200 + 2;
              }
              if (pIVar33 != (IUnknown *)0x0) {
                pcVar3 = *(code **)(*(IUnknown **)pIVar33 + 0x10);
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b075e2;
                (*pcVar3)(pIVar33);
              }
              puVar34 = auStack_308 + lVar24;
              pIVar32 = local_188;
              if (pIVar21 != (IUnknown *)0x0) {
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b075f1;
                FUN_14019f2c0(pIVar21 + -0x10);
                puVar34 = auStack_308 + lVar24;
                pIVar32 = local_188;
              }
            }
          }
          else {
            local_238 = (IUnknown *)0x0;
            puStack_310 = (undefined *)0x141b06bcf;
            FUN_14019ba10(&local_238,"Effect/BasicEff.img/ColdHotEmblem/%d",param_14);
            pIVar33 = DAT_143add058;
            if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              puStack_310 = (undefined *)0x141b09410;
              FUN_142ef3ac0(0x80004003);
            }
            puStack_310 = (undefined *)0x141b06be9;
            (*DAT_143262a20)(&local_230);
            puStack_310 = (undefined *)0x141b06bf9;
            iVar8 = FUN_14023c4c0(&local_230,&DAT_143a8b8d8);
            if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
              puStack_310 = (undefined *)0x141b093c4;
              FUN_142ef3ac0(iVar8);
            }
            puStack_310 = (undefined *)0x141b06c0b;
            (*DAT_143262a20)(&local_288);
            puStack_310 = (undefined *)0x141b06c1b;
            iVar8 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
            pIVar21 = local_238;
            if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
              puStack_310 = (undefined *)0x141b093cc;
              FUN_142ef3ac0(iVar8);
            }
            if (local_238 == (IUnknown *)0x0) {
              iVar8 = 2;
            }
            else {
              local_2e0 = (longlong *)((ulonglong)local_2e0 & 0xffffffff00000000);
              local_2e8 = (longlong *)0x0;
              puStack_310 = (undefined *)0x141b06c50;
              iVar8 = (*DAT_1432627f8)(0xfde9,0,local_238,0xffffffff);
              iVar8 = iVar8 * 2;
            }
            uVar25 = (longlong)iVar8 + 0xf;
            if (uVar25 <= (ulonglong)(longlong)iVar8) {
              uVar25 = 0xffffffffffffff0;
            }
            puStack_310 = (undefined *)0x141b06c73;
            lVar24 = -(uVar25 & 0xfffffffffffffff0);
            puVar19 = (undefined2 *)((longlong)&local_288 + lVar24);
            local_1a8 = puVar19;
            if (pIVar21 == (IUnknown *)0x0) {
              if (puVar19 != (undefined2 *)0x0) {
                *puVar19 = 0;
              }
            }
            else {
              *(undefined4 *)((longlong)&local_2e0 + lVar24) = 0x100000;
              *(undefined2 **)((longlong)&local_2e8 + lVar24) = puVar19;
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06cb9;
              (*DAT_1432627f8)(0xfde9,0,pIVar21,0xffffffff);
              puVar19 = local_1a8;
            }
            *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06ccf;
            uVar18 = FUN_1401a5890(&local_128,puVar19);
            *(undefined8 **)((longlong)&local_2e8 + lVar24) = &local_230;
            *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06cee;
            uVar18 = FUN_140403ec0(pIVar33,&local_168,uVar18,&local_288);
            *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06cfe;
            uVar18 = FUN_1409339d0(&local_1e8,uVar18);
            *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06d0b;
            FUN_1403ee040(&local_270,uVar18);
            if (local_1e8 != (IUnknown *)0x0) {
              pcVar3 = *(code **)(*(longlong *)local_1e8 + 0x10);
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06d1e;
              (*pcVar3)();
            }
            if ((short)local_168 == 8) {
              local_168 = (IUnknown *)((ulonglong)local_168 & 0xffffffffffff0000);
              if (uStack_160 != (longlong *)0x0) {
                lVar26 = (longlong)uStack_160 + -4;
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06d46;
                (*DAT_143ad5990)(lVar26);
              }
            }
            else {
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06d55;
              (*DAT_143262a18)(&local_168);
            }
            if ((short)local_288 == 8) {
              local_288._0_2_ = 0;
              if (plStack_280 != (longlong *)0x0) {
                lVar26 = (longlong)plStack_280 + -4;
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06d74;
                (*DAT_143ad5990)(lVar26);
              }
            }
            else {
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06d80;
              (*DAT_143262a18)(&local_288);
            }
            if ((short)local_230 == 8) {
              local_230._0_2_ = 0;
              lVar26 = CONCAT44(uStack_224,uStack_228);
              if (lVar26 != 0) {
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06d9f;
                (*DAT_143ad5990)(lVar26 + -4);
              }
            }
            else {
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06dab;
              (*DAT_143262a18)(&local_230);
            }
            pIVar33 = local_270;
            if (local_270 != (IUnknown *)0x0) {
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06dcf;
              FUN_140295e30(PTR_u_Canvas_143a479d8,&local_1a0,0);
              plVar16 = local_1a0;
              if (local_1a0 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b09405;
                FUN_142ef3ac0(0x80004003);
              }
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06de9;
              (*DAT_143262a20)(&local_258);
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06df9;
              iVar8 = FUN_14023c4c0(&local_258,&DAT_143a8b8d8);
              if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b093d4;
                FUN_142ef3ac0(iVar8);
              }
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06e0e;
              (*DAT_143262a20)(&local_1c8);
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06e21;
              iVar8 = FUN_14023c4c0(&local_1c8,&DAT_143a8b8d8);
              if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b093dc;
                FUN_142ef3ac0(iVar8);
              }
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06e33;
              (*DAT_143262a20)(&local_218);
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06e43;
              iVar8 = FUN_14023c4c0(&local_218,&DAT_143a8b8d8);
              if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b093e4;
                FUN_142ef3ac0(iVar8);
              }
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06e53;
              uVar11 = FUN_140404240(pIVar33);
              local_1a8 = (undefined2 *)CONCAT44(local_1a8._4_4_,uVar11);
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06e61;
              uVar11 = FUN_140404280(pIVar33);
              *(undefined8 **)((longlong)&local_2e0 + lVar24) = &local_258;
              *(undefined8 **)((longlong)&local_2e8 + lVar24) = &local_1c8;
              uVar25 = (ulonglong)local_1a8 & 0xffffffff;
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06e8b;
              FUN_140cf0040(plVar16,uVar11,uVar25,&local_218);
              if (local_218 == 8) {
                local_218 = 0;
                if (plStack_210 != (longlong *)0x0) {
                  lVar26 = (longlong)plStack_210 + -4;
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06eac;
                  (*DAT_143ad5990)(lVar26);
                }
              }
              else {
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06eb8;
                (*DAT_143262a18)(&local_218);
              }
              if ((short)local_1c8 == 8) {
                local_1c8 = (IUnknown *)((ulonglong)local_1c8 & 0xffffffffffff0000);
                if (plStack_1c0 != (longlong *)0x0) {
                  lVar26 = (longlong)plStack_1c0 + -4;
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06ee1;
                  (*DAT_143ad5990)(lVar26);
                }
              }
              else {
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06ef0;
                (*DAT_143262a18)(&local_1c8);
              }
              if ((short)local_258 == 8) {
                local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
                if (plStack_250 != (longlong *)0x0) {
                  lVar26 = (longlong)plStack_250 + -4;
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06f0f;
                  (*DAT_143ad5990)(lVar26);
                }
              }
              else {
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06f1b;
                (*DAT_143262a18)(&local_258);
              }
              plVar16 = local_1a0;
              if (local_1a0 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
                *(undefined **)(auStack_308 + lVar24 + -8) = &UNK_141b093fa;
                FUN_142ef3ac0(0x80004003);
              }
              local_258 = (IUnknown *)CONCAT62(local_258._2_6_,3);
              plStack_250 = (longlong *)CONCAT44(plStack_250._4_4_,0xff);
              *(undefined8 **)((longlong)&local_2e8 + lVar24) = &local_258;
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06f52;
              FUN_140ceffd0(plVar16,0,0,pIVar33);
              if ((short)local_258 == 8) {
                local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
                if (plStack_250 != (longlong *)0x0) {
                  lVar26 = (longlong)plStack_250 + -4;
                  *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06f71;
                  (*DAT_143ad5990)(lVar26);
                }
              }
              else {
                *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06f7d;
                (*DAT_143262a18)(&local_258);
              }
              if (local_1a0 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
                *(undefined **)(auStack_308 + lVar24 + -8) = &UNK_141b093ef;
                FUN_142ef3ac0(0x80004003);
              }
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06f93;
              local_200 = FUN_140404280();
              local_200 = local_200 + 2;
            }
            if (pIVar33 != (IUnknown *)0x0) {
              pcVar3 = *(code **)(*(IUnknown **)pIVar33 + 0x10);
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06faa;
              (*pcVar3)(pIVar33);
            }
            puVar34 = auStack_308 + lVar24;
            pIVar32 = local_188;
            if (pIVar21 != (IUnknown *)0x0) {
              *(undefined8 *)(auStack_308 + lVar24 + -8) = 0x141b06fb9;
              FUN_14019f2c0(pIVar21 + -0x10);
              puVar34 = auStack_308 + lVar24;
              pIVar32 = local_188;
            }
          }
        }
        pIVar33 = (IUnknown *)0x0;
        local_170 = (IUnknown *)0x0;
        local_198 = (IUnknown *)0x0;
        local_238 = local_1f8;
        if (local_1f8 != (IUnknown *)0x0) {
          pcVar3 = *(code **)(*(longlong *)local_1f8 + 8);
          *(undefined8 *)(puVar34 + -8) = 0x141b06fe5;
          (*pcVar3)();
        }
        *(undefined4 *)(puVar34 + 0x20) = 0;
        *(undefined8 *)(puVar34 + -8) = 0x141b07000;
        FUN_142a5f0e0(&local_198,&local_238,0,0);
        if (local_1f8 == (IUnknown *)0x0) {
LAB_141b092a1:
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b092ab;
          FUN_142ef3ac0(0x80004003);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b07015;
        uVar11 = FUN_141b0e9a0();
        if (local_1f8 == (IUnknown *)0x0) goto LAB_141b092a1;
        *(undefined8 *)(puVar34 + -8) = 0x141b0702c;
        local_a0 = FUN_141b0e9a0();
        local_b0 = 0;
        local_c0 = 0;
        local_b8 = 0;
        local_c8 = &PTR_FUN_14336e7a8;
        local_90 = 0;
        local_88 = 0;
        uStack_80 = 0;
        local_78 = 0;
        uStack_70 = 0;
        local_68 = 0;
        uStack_60 = 0;
        local_58 = 0;
        uStack_50 = 0;
        local_a4 = local_1fc;
        local_a8 = 0;
        local_98 = 0;
        local_9c = uVar11;
        *(undefined8 *)(puVar34 + -8) = 0x141b070b2;
        uVar18 = FUN_140196ed0(&local_270,pIVar32,0xffffffff);
        *(undefined8 *)(puVar34 + 0x70) = 0;
        *(undefined4 *)(puVar34 + 0x68) = 0;
        puVar34[0x60] = 0;
        puVar34[0x58] = 0;
        puVar34[0x50] = 0;
        puVar34[0x48] = 0;
        *(undefined4 *)(puVar34 + 0x40) = 0;
        *(undefined4 *)(puVar34 + 0x38) = 1;
        *(undefined4 *)(puVar34 + 0x30) = 0;
        *(undefined4 *)(puVar34 + 0x28) = 0;
        *(undefined4 *)(puVar34 + 0x20) = 0;
        *(undefined8 *)(puVar34 + -8) = 0x141b07104;
        FUN_142a45580(&local_c8,uVar18,&local_170,&local_198);
        local_1fc = 0;
        if (local_170 != (IUnknown *)0x0) {
          pIVar32 = local_170;
          if (*(int *)(local_170 + -8) != 0) {
            do {
              pIVar21 = local_1f8;
              if (local_1f8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
                *(undefined **)(puVar34 + -8) = &UNK_141b09477;
                FUN_142ef3ac0(0x80004003);
              }
              *(undefined8 *)(puVar34 + -8) = 0x141b0714a;
              (*DAT_143262a20)(&local_258);
              *(undefined8 *)(puVar34 + -8) = 0x141b0715a;
              iVar8 = FUN_14023c4c0(&local_258,&DAT_143a8b8d8);
              if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
                *(undefined **)(puVar34 + -8) = &UNK_141b0946c;
                FUN_142ef3ac0(iVar8);
              }
              lVar24 = *(longlong *)(pIVar32 + 0x20);
              *(undefined8 *)(puVar34 + -8) = 0x141b0716f;
              uVar18 = FUN_1401a5780(&local_270,lVar24);
              *(undefined8 *)(puVar34 + -8) = 0x141b0717e;
              uVar9 = FUN_140ee5fb0(pIVar21,uVar18,&local_258);
              if (local_1fc < uVar9) {
                local_1fc = uVar9;
              }
              if ((short)local_258 == 8) {
                local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
                if (plStack_250 != (longlong *)0x0) {
                  lVar24 = (longlong)plStack_250 + -4;
                  *(undefined8 *)(puVar34 + -8) = 0x141b071b9;
                  (*DAT_143ad5990)(lVar24);
                }
              }
              else {
                *(undefined8 *)(puVar34 + -8) = 0x141b07601;
                (*DAT_143262a18)(&local_258);
              }
            } while ((pIVar32 < local_170 + *(longlong *)(local_170 + -8) * 0x58 + -0x58) &&
                    (pIVar32 = pIVar32 + 0x58, pIVar32 != (IUnknown *)0x0));
          }
          if ((local_170 != (IUnknown *)0x0) && (uVar9 = *(uint *)(local_170 + -8), uVar9 != 0)) {
            uVar30 = uVar9 - 1;
            if (uVar9 <= uVar30) {
              *(undefined8 *)(puVar34 + -8) = 0x141b0764d;
              FUN_142e54290(0xbc,uVar30);
            }
            pIVar32 = local_170;
            if (local_170 == (IUnknown *)0x0) {
              uVar31 = 0xffffffff;
              pIVar21 = pIVar33;
LAB_141b07679:
              *(undefined8 *)(puVar34 + -8) = 0x141b07685;
              FUN_142e54290(0xbc,uVar31,pIVar21);
            }
            else {
              uVar9 = *(uint *)(local_170 + -8);
              pIVar21 = (IUnknown *)(ulonglong)uVar9;
              uVar31 = uVar9 - 1;
              if (uVar9 <= uVar31) goto LAB_141b07679;
            }
            local_148 = (IUnknown *)
                        CONCAT44(local_148._4_4_,
                                 *(int *)(local_170 + (ulonglong)uVar31 * 0x58 + 0x3c) +
                                 *(int *)(pIVar32 + (ulonglong)uVar30 * 0x58 + 0x34));
          }
        }
        uVar9 = local_200 + local_1fc + 4;
        local_148 = (IUnknown *)CONCAT44(local_148._4_4_,(int)local_148 + 1);
        local_1fc = uVar9;
        *(undefined8 *)(puVar34 + -8) = 0x141b076d2;
        FUN_140295e30(PTR_u_Canvas_143a479d8,&local_1f0,0);
        pIVar32 = local_1f0;
        if (local_1f0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          *(undefined **)(puVar34 + -8) = &UNK_141b09677;
          FUN_142ef3ac0(0x80004003);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b076ec;
        (*DAT_143262a20)(&local_230);
        *(undefined8 *)(puVar34 + -8) = 0x141b076fc;
        iVar8 = FUN_14023c4c0(&local_230,&DAT_143a8b8d8);
        if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b0947f;
          FUN_142ef3ac0(iVar8);
        }
        local_1e8 = (IUnknown *)CONCAT62(local_1e8._2_6_,0x16);
        plStack_1e0 = (longlong *)CONCAT44(plStack_1e0._4_4_,2);
        *(undefined8 *)(puVar34 + -8) = 0x141b07724;
        (*DAT_143262a20)(&local_218);
        *(undefined8 *)(puVar34 + -8) = 0x141b07734;
        iVar8 = FUN_14023c4c0(&local_218,&DAT_143a8b8d8);
        if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b09487;
          FUN_142ef3ac0(iVar8);
        }
        lVar24 = *(longlong *)pIVar32;
        local_168 = (IUnknown *)CONCAT44(local_230._4_4_,CONCAT22(local_230._2_2_,(short)local_230))
        ;
        uStack_160 = (longlong *)CONCAT44(uStack_224,uStack_228);
        local_158 = local_220;
        local_258 = local_1e8;
        plStack_250 = plStack_1e0;
        local_248 = local_1d8;
        local_1c8 = (IUnknown *)CONCAT44(uStack_214,CONCAT22(uStack_216,local_218));
        plStack_1c0 = plStack_210;
        local_1b8 = local_208;
        *(undefined8 **)(puVar34 + 0x28) = &local_168;
        *(undefined8 **)(puVar34 + 0x20) = &local_258;
        uVar25 = (ulonglong)local_148 & 0xffffffff;
        pcVar3 = *(code **)(lVar24 + 0x68);
        *(undefined8 *)(puVar34 + -8) = 0x141b077b5;
        iVar8 = (*pcVar3)(pIVar32,uVar9,uVar25,&local_1c8);
        if (iVar8 < 0) {
          *(undefined8 *)(puVar34 + -8) = 0x141b077ca;
          _com_issue_errorex(iVar8,pIVar32,(_GUID *)&DAT_14327ac98);
        }
        if (local_218 == 8) {
          local_218 = 0;
          if (plStack_210 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_210 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b077e9;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b077f5;
          (*DAT_143262a18)(&local_218);
        }
        if ((short)local_1e8 == 8) {
          local_1e8 = (IUnknown *)((ulonglong)local_1e8 & 0xffffffffffff0000);
          if (plStack_1e0 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_1e0 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b0781d;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b0782c;
          (*DAT_143262a18)(&local_1e8);
        }
        if ((short)local_230 == 8) {
          local_230._0_2_ = 0;
          lVar24 = CONCAT44(uStack_224,uStack_228);
          if (lVar24 != 0) {
            *(undefined8 *)(puVar34 + -8) = 0x141b0784b;
            (*DAT_143ad5990)(lVar24 + -4);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b07857;
          (*DAT_143262a18)(&local_230);
        }
        pIVar32 = local_1f0;
        if (local_1f0 == (IUnknown *)0x0) {
LAB_141b09662:
                    /* WARNING: Subroutine does not return */
          *(undefined **)(puVar34 + -8) = &UNK_141b0966c;
          FUN_142ef3ac0(0x80004003);
        }
        *(int *)(puVar34 + 0x28) = (int)local_138;
        *(int *)(puVar34 + 0x20) = (int)local_148;
        iVar8 = local_200;
        *(undefined8 *)(puVar34 + -8) = 0x141b0788d;
        FUN_1411ec450(pIVar32,iVar8,0,uVar9);
        pIVar32 = local_1f0;
        if (pIVar15 != (IUnknown *)0x0) {
          if (local_1f0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
            *(undefined **)(puVar34 + -8) = &UNK_141b09492;
            FUN_142ef3ac0(0x80004003);
          }
          local_230._0_2_ = 3;
          uStack_228 = (int)local_140;
          lVar24 = *(longlong *)local_1f0;
          local_168 = (IUnknown *)CONCAT44(local_230._4_4_,CONCAT22(local_230._2_2_,3));
          uStack_160 = (longlong *)CONCAT44(uStack_224,(int)local_140);
          local_158 = local_220;
          *(undefined8 **)(puVar34 + 0x20) = &local_168;
          pcVar3 = *(code **)(lVar24 + 0x128);
          *(undefined8 *)(puVar34 + -8) = 0x141b078f0;
          iVar8 = (*pcVar3)(pIVar32,0,0,pIVar15);
          if (iVar8 < 0) {
            *(undefined8 *)(puVar34 + -8) = 0x141b07905;
            _com_issue_errorex(iVar8,pIVar32,(_GUID *)&DAT_14327ac98);
          }
          if ((short)local_230 == 8) {
            local_230._0_2_ = 0;
            lVar24 = CONCAT44(uStack_224,uStack_228);
            if (lVar24 != 0) {
              *(undefined8 *)(puVar34 + -8) = 0x141b07924;
              (*DAT_143ad5990)(lVar24 + -4);
            }
          }
          else {
            *(undefined8 *)(puVar34 + -8) = 0x141b07930;
            (*DAT_143262a18)(&local_230);
          }
        }
        pIVar32 = local_1f0;
        if (local_1a0 != (longlong *)0x0) {
          if (local_1f0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
            *(undefined8 *)(puVar34 + -8) = 0x141b0949d;
            FUN_142ef3ac0(0x80004003);
          }
          local_258 = (IUnknown *)CONCAT62(local_258._2_6_,3);
          plStack_250 = (longlong *)CONCAT44(plStack_250._4_4_,0xff);
          *(undefined8 **)(puVar34 + 0x20) = &local_258;
          *(undefined8 *)(puVar34 + -8) = 0x141b07970;
          FUN_140ceffd0(pIVar32,0,0);
          if ((short)local_258 == 8) {
            local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
            if (plStack_250 != (longlong *)0x0) {
              lVar24 = (longlong)plStack_250 + -4;
              *(undefined8 *)(puVar34 + -8) = 0x141b0798f;
              (*DAT_143ad5990)(lVar24);
            }
          }
          else {
            *(undefined8 *)(puVar34 + -8) = 0x141b0799b;
            (*DAT_143262a18)(&local_258);
          }
        }
        pIVar32 = local_1f0;
        if (local_1f0 == (IUnknown *)0x0) goto LAB_141b09662;
        lVar24 = *(longlong *)local_1f0;
        *(undefined4 *)(puVar34 + 0x28) = 0xffffff;
        *(undefined4 *)(puVar34 + 0x20) = 1;
        iVar8 = local_200;
        pcVar3 = *(code **)(lVar24 + 0x170);
        *(undefined8 *)(puVar34 + -8) = 0x141b079d7;
        iVar8 = (*pcVar3)(pIVar32,iVar8,0,1);
        if (iVar8 < 0) {
          *(undefined8 *)(puVar34 + -8) = 0x141b079ec;
          _com_issue_errorex(iVar8,pIVar32,(_GUID *)&DAT_14327ac98);
        }
        pIVar32 = local_1f0;
        if (local_1f0 == (IUnknown *)0x0) goto LAB_141b09662;
        iVar10 = (int)local_148 + -1;
        lVar24 = *(longlong *)local_1f0;
        *(undefined4 *)(puVar34 + 0x28) = 0xffffff;
        *(undefined4 *)(puVar34 + 0x20) = 1;
        iVar8 = local_200;
        pcVar3 = *(code **)(lVar24 + 0x170);
        *(undefined8 *)(puVar34 + -8) = 0x141b07a2e;
        iVar8 = (*pcVar3)(pIVar32,iVar8,iVar10,1);
        if (iVar8 < 0) {
          *(undefined8 *)(puVar34 + -8) = 0x141b07a43;
          _com_issue_errorex(iVar8,pIVar32,(_GUID *)&DAT_14327ac98);
        }
        pIVar32 = local_1f0;
        if (local_1f0 == (IUnknown *)0x0) goto LAB_141b09662;
        lVar24 = *(longlong *)local_1f0;
        *(undefined4 *)(puVar34 + 0x28) = 0xffffff;
        *(undefined4 *)(puVar34 + 0x20) = 1;
        pcVar3 = *(code **)(lVar24 + 0x170);
        *(undefined8 *)(puVar34 + -8) = 0x141b07a7b;
        iVar8 = (*pcVar3)(pIVar32,uVar9 - 1,0,1);
        if (iVar8 < 0) {
          *(undefined8 *)(puVar34 + -8) = 0x141b07a90;
          _com_issue_errorex(iVar8,pIVar32,(_GUID *)&DAT_14327ac98);
        }
        pIVar32 = local_1f0;
        if (local_1f0 == (IUnknown *)0x0) goto LAB_141b09662;
        lVar24 = *(longlong *)local_1f0;
        *(undefined4 *)(puVar34 + 0x28) = 0xffffff;
        *(undefined4 *)(puVar34 + 0x20) = 1;
        iVar8 = (int)local_148 + -1;
        pcVar3 = *(code **)(lVar24 + 0x170);
        *(undefined8 *)(puVar34 + -8) = 0x141b07acf;
        iVar8 = (*pcVar3)(pIVar32,uVar9 - 1,iVar8,1);
        if (iVar8 < 0) {
          *(undefined8 *)(puVar34 + -8) = 0x141b07ae4;
          _com_issue_errorex(iVar8,pIVar32,(_GUID *)&DAT_14327ac98);
        }
        pIVar32 = local_1f0;
        if ((int)local_118 != 0) {
          if (local_1f0 == (IUnknown *)0x0) goto LAB_141b09662;
          iVar10 = (int)local_148 + -2;
          iVar12 = local_1fc - 2;
          iVar8 = local_200 + 1;
          *(undefined4 *)(puVar34 + 0x28) = 0xffffff;
          *(int *)(puVar34 + 0x20) = iVar10;
          *(undefined8 *)(puVar34 + -8) = 0x141b07b31;
          FUN_1411ec450(pIVar32,iVar8,1,iVar12);
          pIVar32 = local_1f0;
          if (local_1f0 == (IUnknown *)0x0) goto LAB_141b09662;
          *(undefined4 *)(puVar34 + 0x28) = 0xb2ffffff;
          *(int *)(puVar34 + 0x20) = iVar10;
          iVar8 = local_200 + 1;
          *(undefined8 *)(puVar34 + -8) = 0x141b07b63;
          FUN_1411ec450(pIVar32,iVar8,1,iVar12);
        }
        local_238 = local_170;
        if ((local_170 != (IUnknown *)0x0) && (pIVar32 = local_170, *(int *)(local_170 + -8) != 0))
        {
          do {
            pIVar21 = local_1f8;
            local_270 = local_1f8;
            local_238 = pIVar32;
            if (local_1f8 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              *(undefined8 *)(puVar34 + -8) = 0x141b094e9;
              FUN_142ef3ac0(0x80004003);
            }
            *(undefined8 *)(puVar34 + -8) = 0x141b07bae;
            (*DAT_143262a20)(&local_230);
            *(undefined8 *)(puVar34 + -8) = 0x141b07bbe;
            iVar8 = FUN_14023c4c0(&local_230,&DAT_143a8b8d8);
            if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
              *(undefined **)(puVar34 + -8) = &UNK_141b094de;
              FUN_142ef3ac0(iVar8);
            }
            local_1a8 = *(undefined2 **)(pIVar32 + 0x20);
            *(undefined8 *)(puVar34 + -8) = 0x141b07be2;
            pIVar20 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
            lVar24 = (longlong)local_1a8;
            pIVar32 = pIVar33;
            local_118 = pIVar20;
            if (pIVar20 != (IUnknown *)0x0) {
              *(longlong *)(pIVar20 + 8) = 0;
              *(int *)(pIVar20 + 0x10) = 1;
              pIVar32 = pIVar20;
              if (local_1a8 == (undefined2 *)0x0) {
                *(longlong *)pIVar20 = 0;
              }
              else {
                *(undefined4 *)(puVar34 + 0x28) = 0;
                *(undefined8 *)(puVar34 + 0x20) = 0;
                *(undefined8 *)(puVar34 + -8) = 0x141b07c33;
                iVar8 = (*DAT_1432627f8)(0xfde9,0,lVar24,0xffffffff);
                iVar8 = (int)((ulonglong)(longlong)(iVar8 * 2) >> 1);
                *(undefined8 *)(puVar34 + -8) = 0x141b07c45;
                pIVar21 = (IUnknown *)FUN_1401a5fa0(0,iVar8 + -1);
                local_1e8 = pIVar21;
                *(int *)(puVar34 + 0x28) = iVar8;
                *(IUnknown **)(puVar34 + 0x20) = pIVar21;
                lVar24 = (longlong)local_1a8;
                *(undefined8 *)(puVar34 + -8) = 0x141b07c6f;
                (*DAT_1432627f8)(0xfde9,0,lVar24,0xffffffff);
                *(IUnknown **)pIVar20 = local_1e8;
                pIVar21 = local_270;
              }
            }
            local_188 = pIVar32;
            if (pIVar32 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              *(undefined8 *)(puVar34 + -8) = 0x141b094d6;
              FUN_142ef3ac0(0x8007000e);
            }
            local_270 = (IUnknown *)&local_188;
            local_140 = local_140 & 0xffffffff00000000;
            local_168 = (IUnknown *)
                        CONCAT44(local_230._4_4_,CONCAT22(local_230._2_2_,(short)local_230));
            uStack_160 = (longlong *)CONCAT44(uStack_224,uStack_228);
            local_158 = local_220;
            lVar24 = *(longlong *)pIVar32;
            pcVar3 = *(code **)(*(longlong *)pIVar21 + 0xb0);
            *(undefined8 *)(puVar34 + -8) = 0x141b07cd8;
            iVar8 = (*pcVar3)(pIVar21,lVar24,&local_168,&local_140);
            if (iVar8 < 0) {
              *(undefined8 *)(puVar34 + -8) = 0x141b07ced;
              _com_issue_errorex(iVar8,pIVar21,(_GUID *)&DAT_143297250);
            }
            iVar8 = (int)local_140;
            *(undefined8 *)(puVar34 + -8) = 0x141b07cff;
            thunk_FUN_1401be120(&local_188);
            if ((short)local_230 == 8) {
              local_230._0_2_ = 0;
              lVar24 = CONCAT44(uStack_224,uStack_228);
              if (lVar24 != 0) {
                *(undefined8 *)(puVar34 + -8) = 0x141b07d1e;
                (*DAT_143ad5990)(lVar24 + -4);
              }
            }
            else {
              *(undefined8 *)(puVar34 + -8) = 0x141b07d2a;
              (*DAT_143262a18)(&local_230);
            }
            local_1a8 = (undefined2 *)CONCAT44(local_1a8._4_4_,(int)local_1fc / 2 - iVar8 / 2);
            local_128 = local_1f0;
            if (local_1f0 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              *(undefined8 *)(puVar34 + -8) = 0x141b094cb;
              FUN_142ef3ac0(0x80004003);
            }
            *(undefined8 *)(puVar34 + -8) = 0x141b07d68;
            (*DAT_143262a20)(&local_288);
            *(undefined8 *)(puVar34 + -8) = 0x141b07d78;
            iVar8 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
            if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
              *(undefined **)(puVar34 + -8) = &UNK_141b094c0;
              FUN_142ef3ac0(iVar8);
            }
            *(undefined8 *)(puVar34 + -8) = 0x141b07d8a;
            (*DAT_143262a20)(&local_218);
            *(undefined8 *)(puVar34 + -8) = 0x141b07d9a;
            iVar8 = FUN_14023c4c0(&local_218,&DAT_143a8b8d8);
            if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
              *(undefined8 *)(puVar34 + -8) = 0x141b094b8;
              FUN_142ef3ac0(iVar8);
            }
            *(undefined8 *)(puVar34 + -8) = 0x141b07dac;
            (*DAT_143262a20)(&local_230);
            *(undefined8 *)(puVar34 + -8) = 0x141b07dbc;
            iVar8 = FUN_14023c4c0(&local_230,&DAT_143a8b8d8);
            if (iVar8 < 0) {
                    /* WARNING: Subroutine does not return */
              *(undefined8 *)(puVar34 + -8) = 0x141b094b0;
              FUN_142ef3ac0(iVar8);
            }
            pIVar32 = *(IUnknown **)(local_238 + 0x20);
            local_140 = *(ulonglong *)(local_238 + 0x18);
            local_270 = pIVar32;
            *(undefined8 *)(puVar34 + -8) = 0x141b07dec;
            pIVar20 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
            pIVar21 = pIVar33;
            local_148 = pIVar20;
            if (pIVar20 != (IUnknown *)0x0) {
              *(longlong *)(pIVar20 + 8) = 0;
              *(int *)(pIVar20 + 0x10) = 1;
              pIVar21 = pIVar20;
              if (pIVar32 == (IUnknown *)0x0) {
                *(longlong *)pIVar20 = 0;
              }
              else {
                *(undefined4 *)(puVar34 + 0x28) = 0;
                *(undefined8 *)(puVar34 + 0x20) = 0;
                *(undefined8 *)(puVar34 + -8) = 0x141b07e32;
                iVar8 = (*DAT_1432627f8)(0xfde9,0,pIVar32,0xffffffff);
                iVar8 = (int)((ulonglong)(longlong)(iVar8 * 2) >> 1);
                *(undefined8 *)(puVar34 + -8) = 0x141b07e44;
                pIVar32 = (IUnknown *)FUN_1401a5fa0(0,iVar8 + -1);
                local_1e8 = pIVar32;
                *(int *)(puVar34 + 0x28) = iVar8;
                *(IUnknown **)(puVar34 + 0x20) = pIVar32;
                pIVar32 = local_270;
                *(undefined8 *)(puVar34 + -8) = 0x141b07e6b;
                (*DAT_1432627f8)(0xfde9,0,pIVar32,0xffffffff);
                *(IUnknown **)pIVar20 = local_1e8;
              }
            }
            pIVar32 = local_128;
            local_138 = pIVar21;
            if (pIVar21 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
              *(undefined8 *)(puVar34 + -8) = 0x141b094a8;
              FUN_142ef3ac0(0x8007000e);
            }
            local_270 = (IUnknown *)&local_138;
            iVar10 = (int)local_1a8 + local_200;
            local_1a8 = (undefined2 *)((ulonglong)local_1a8 & 0xffffffff00000000);
            lVar24 = *(longlong *)local_128;
            local_258 = (IUnknown *)
                        CONCAT44(local_288._4_4_,CONCAT22(local_288._2_2_,(short)local_288));
            plStack_250 = plStack_280;
            local_248 = local_278;
            local_1c8 = (IUnknown *)CONCAT44(uStack_214,CONCAT22(uStack_216,local_218));
            plStack_1c0 = plStack_210;
            local_1b8 = local_208;
            local_108 = CONCAT44(local_230._4_4_,CONCAT22(local_230._2_2_,(short)local_230));
            uStack_100 = (longlong *)CONCAT44(uStack_224,uStack_228);
            local_f8 = local_220;
            *(undefined8 **)(puVar34 + 0x40) = &local_1a8;
            *(undefined8 **)(puVar34 + 0x38) = &local_258;
            *(undefined8 **)(puVar34 + 0x30) = &local_1c8;
            *(undefined8 **)(puVar34 + 0x28) = &local_108;
            *(ulonglong *)(puVar34 + 0x20) = local_140;
            lVar26 = *(longlong *)pIVar21;
            iVar8 = *(int *)(local_238 + 0x34);
            pcVar3 = *(code **)(lVar24 + 0x1a8);
            *(undefined8 *)(puVar34 + -8) = 0x141b07f43;
            iVar8 = (*pcVar3)(pIVar32,iVar10,iVar8,lVar26);
            if (iVar8 < 0) {
              *(undefined8 *)(puVar34 + -8) = 0x141b07f58;
              _com_issue_errorex(iVar8,pIVar32,(_GUID *)&DAT_14327ac98);
            }
            *(undefined8 *)(puVar34 + -8) = 0x141b07f65;
            thunk_FUN_1401be120(&local_138);
            if ((short)local_230 == 8) {
              local_230._0_2_ = 0;
              lVar24 = CONCAT44(uStack_224,uStack_228);
              if (lVar24 != 0) {
                *(undefined8 *)(puVar34 + -8) = 0x141b07f84;
                (*DAT_143ad5990)(lVar24 + -4);
              }
            }
            else {
              *(undefined8 *)(puVar34 + -8) = 0x141b07f90;
              (*DAT_143262a18)(&local_230);
            }
            if (local_218 == 8) {
              local_218 = 0;
              if (plStack_210 != (longlong *)0x0) {
                lVar24 = (longlong)plStack_210 + -4;
                *(undefined8 *)(puVar34 + -8) = 0x141b07faf;
                (*DAT_143ad5990)(lVar24);
              }
            }
            else {
              *(undefined8 *)(puVar34 + -8) = 0x141b07fbb;
              (*DAT_143262a18)(&local_218);
            }
            if ((short)local_288 == 8) {
              local_288._0_2_ = 0;
              if (plStack_280 != (longlong *)0x0) {
                lVar24 = (longlong)plStack_280 + -4;
                *(undefined8 *)(puVar34 + -8) = 0x141b07fda;
                (*DAT_143ad5990)(lVar24);
              }
            }
            else {
              *(undefined8 *)(puVar34 + -8) = 0x141b07fe6;
              (*DAT_143262a18)(&local_288);
            }
          } while ((local_238 < local_170 + *(longlong *)(local_170 + -8) * 0x58 + -0x58) &&
                  (local_238 = local_238 + 0x58, pIVar32 = local_238, local_238 != (IUnknown *)0x0))
          ;
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b08020;
        FUN_140da11f0(&local_c8);
        if (local_198 != (IUnknown *)0x0) {
          pIVar33 = local_198 + *(longlong *)(local_198 + -8) * 8;
          for (pIVar32 = local_198; pIVar32 < pIVar33; pIVar32 = pIVar32 + 8) {
            if (*(longlong **)pIVar32 != (longlong *)0x0) {
              pcVar3 = *(code **)(**(longlong **)pIVar32 + 0x10);
              *(undefined8 *)(puVar34 + -8) = 0x141b0804e;
              (*pcVar3)();
            }
          }
          pIVar33 = local_198 + -8;
          *(undefined8 *)(puVar34 + -8) = 0x141b0806a;
          thunk_FUN_140205820(pIVar33,0);
          local_198 = (IUnknown *)0x0;
        }
        if (local_170 != (IUnknown *)0x0) {
          if (local_170 < local_170 + *(longlong *)(local_170 + -8) * 0x58) {
            pIVar33 = local_170 + 0x20;
            lVar24 = (ulonglong)
                     (local_170 + *(longlong *)(local_170 + -8) * 0x58 + (-1 - (longlong)local_170))
                     / 0x58 + 1;
            do {
              if (*(longlong **)(pIVar33 + 8) != (longlong *)0x0) {
                pcVar3 = *(code **)(**(longlong **)(pIVar33 + 8) + 0x10);
                *(undefined8 *)(puVar34 + -8) = 0x141b080bf;
                (*pcVar3)();
              }
              lVar26 = *(longlong *)pIVar33;
              if (lVar26 != 0) {
                *(undefined8 *)(puVar34 + -8) = 0x141b080d1;
                FUN_14019f2c0(lVar26 + -0x10);
              }
              if (*(longlong **)(pIVar33 + -8) != (longlong *)0x0) {
                pcVar3 = *(code **)(**(longlong **)(pIVar33 + -8) + 0x10);
                *(undefined8 *)(puVar34 + -8) = 0x141b080e1;
                (*pcVar3)();
              }
              pIVar33 = pIVar33 + 0x58;
              lVar24 = lVar24 + -1;
            } while (lVar24 != 0);
          }
          pIVar33 = local_170 + -8;
          *(undefined8 *)(puVar34 + -8) = 0x141b080fe;
          thunk_FUN_140205820(pIVar33,0);
          local_170 = (IUnknown *)0x0;
        }
        if (local_1a0 != (longlong *)0x0) {
          pcVar3 = *(code **)(*local_1a0 + 0x10);
          *(undefined8 *)(puVar34 + -8) = 0x141b08117;
          (*pcVar3)();
        }
        if (pIVar15 != (IUnknown *)0x0) {
          pcVar3 = *(code **)(*(longlong *)pIVar15 + 0x10);
          *(undefined8 *)(puVar34 + -8) = 0x141b08126;
          (*pcVar3)(pIVar15);
        }
        plVar16 = local_190;
        iVar8 = local_200;
        if (local_1f8 != (IUnknown *)0x0) {
          pcVar3 = *(code **)(*(longlong *)local_1f8 + 0x10);
          *(undefined8 *)(puVar34 + -8) = 0x141b08139;
          (*pcVar3)();
          plVar16 = local_190;
          iVar8 = local_200;
        }
      }
      else {
        local_238 = DAT_143add058;
        if (DAT_143add058 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          puStack_310 = &UNK_141b09327;
          FUN_142ef3ac0(0x80004003);
        }
        puStack_310 = (undefined *)0x141b05b28;
        (*DAT_143262a20)(&local_218);
        if (DAT_143a8b8d8 == 8) {
          if (local_218 == 8) {
            local_218 = 0;
            if (plStack_210 != (longlong *)0x0) {
              puStack_310 = (undefined *)0x141b05b50;
              (*DAT_143ad5990)((longlong)plStack_210 + -4);
            }
          }
          else {
            puStack_310 = (undefined *)0x141b05b5c;
            iVar8 = (*DAT_143262a18)(&local_218);
            if (iVar8 < 0) goto LAB_141b09302;
          }
          local_218 = 8;
          if (DAT_143a8b8e0 == 0) {
            puStack_310 = (undefined *)0x141b05b80;
            plStack_210 = (longlong *)FUN_1401a5fa0(0,0);
          }
          else {
            puStack_310 = (undefined *)0x141b05b90;
            plStack_210 = (longlong *)
                          FUN_1401a5fa0(DAT_143a8b8e0,*(uint *)(DAT_143a8b8e0 + -4) >> 1);
          }
        }
        else {
          if ((local_218 == 8) && (local_218 = 0, plStack_210 != (longlong *)0x0)) {
            puStack_310 = (undefined *)0x141b05bb4;
            (*DAT_143ad5990)((longlong)plStack_210 + -4);
          }
          puStack_310 = (undefined *)0x141b05bc5;
          iVar8 = (*DAT_143262a28)(&local_218,&DAT_143a8b8d8);
          if (iVar8 < 0) {
LAB_141b09302:
                    /* WARNING: Subroutine does not return */
            puStack_310 = (undefined *)0x141b09309;
            FUN_142ef3ac0(iVar8);
          }
        }
        puStack_310 = (undefined *)0x141b05bdf;
        (*DAT_143262a20)(&local_1e8);
        if (DAT_143a8b8d8 == 8) {
          if ((short)local_1e8 == 8) {
            local_1e8 = (IUnknown *)((ulonglong)local_1e8._2_6_ << 0x10);
            if (plStack_1e0 != (longlong *)0x0) {
              puStack_310 = (undefined *)0x141b05c14;
              (*DAT_143ad5990)((longlong)plStack_1e0 + -4);
            }
          }
          else {
            puStack_310 = (undefined *)0x141b05c23;
            iVar8 = (*DAT_143262a18)(&local_1e8);
            if (iVar8 < 0) goto LAB_141b092ba;
          }
          local_1e8 = (IUnknown *)CONCAT62(local_1e8._2_6_,8);
          pIVar15 = pIVar36;
          if (DAT_143a8b8e0 != 0) {
            pIVar15 = (IUnknown *)(ulonglong)(*(uint *)(DAT_143a8b8e0 + -4) >> 1);
          }
          puStack_310 = (undefined *)0x141b05c4c;
          plStack_1e0 = (longlong *)FUN_1401a5fa0(DAT_143a8b8e0,pIVar15);
        }
        else {
          if (((short)local_1e8 == 8) &&
             (local_1e8 = (IUnknown *)((ulonglong)local_1e8._2_6_ << 0x10),
             plStack_1e0 != (longlong *)0x0)) {
            puStack_310 = (undefined *)0x141b05cea;
            (*DAT_143ad5990)((longlong)plStack_1e0 + -4);
          }
          puStack_310 = (undefined *)0x141b05cfe;
          iVar8 = (*DAT_143262a28)(&local_1e8,&DAT_143a8b8d8);
          if (iVar8 < 0) {
LAB_141b092ba:
                    /* WARNING: Subroutine does not return */
            puStack_310 = (undefined *)0x141b092c1;
            FUN_142ef3ac0(iVar8);
          }
        }
        puStack_310 = (undefined *)0x141b05c64;
        pIVar15 = (IUnknown *)FUN_14019b780(&DAT_143ad68a0,0x18);
        local_198 = pIVar36;
        local_270 = pIVar15;
        if (pIVar15 != (IUnknown *)0x0) {
          *(longlong *)(pIVar15 + 8) = 0;
          *(int *)(pIVar15 + 0x10) = 1;
          lVar24 = -1;
          do {
            lVar24 = lVar24 + 1;
          } while (psVar5[lVar24] != 0);
          local_268[0] = (IUnknown *)(ulonglong)((int)lVar24 + 1);
          puStack_310 = (undefined *)0x141b05cb5;
          piVar13 = (int *)(*DAT_143ad5980)((longlong)local_268[0] * 2 + 4);
          if (piVar13 == (int *)0x0) {
            *(longlong *)pIVar15 = 0;
LAB_141b092af:
                    /* WARNING: Subroutine does not return */
            puStack_310 = (undefined *)0x141b092b9;
            FUN_142ef3ac0(0x8007000e);
          }
          *piVar13 = (int)lVar24 * 2;
          piVar13 = piVar13 + 1;
          puStack_310 = (undefined *)0x141b05d27;
          FUN_142ef7ba0(piVar13,psVar5,(longlong)local_268[0] * 2);
          *(int **)pIVar15 = piVar13;
          pIVar33 = local_238;
          local_198 = pIVar15;
          if (piVar13 == (int *)0x0) goto LAB_141b092af;
        }
        if (local_198 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          puStack_310 = (undefined *)0x141b09314;
          FUN_142ef3ac0(0x8007000e);
        }
        local_270 = (IUnknown *)&local_198;
        puStack_310 = (undefined *)0x141b05d68;
        (*DAT_143262a20)(&local_258);
        pIVar15 = pIVar36;
        if (local_198 != (IUnknown *)0x0) {
          pIVar15 = *(IUnknown **)local_198;
        }
        local_168 = (IUnknown *)CONCAT44(uStack_214,CONCAT22(uStack_216,local_218));
        uStack_160 = plStack_210;
        local_158 = local_208;
        local_1c8 = local_1e8;
        plStack_1c0 = plStack_1e0;
        local_1b8 = local_1d8;
        local_2e8 = &local_258;
        puStack_310 = (undefined *)0x141b05dd9;
        iVar8 = (**(code **)(*(longlong *)pIVar33 + 0x48))(pIVar33,pIVar15,&local_1c8,&local_168);
        if (iVar8 < 0) {
          puStack_310 = (undefined *)0x141b05dee;
          _com_issue_errorex(iVar8,pIVar33,(_GUID *)&DAT_1432743e8);
        }
        local_288._0_2_ = (short)local_258;
        local_288._2_2_ = (undefined2)((ulonglong)local_258 >> 0x10);
        local_288._4_4_ = local_258._4_4_;
        plStack_280 = plStack_250;
        local_278 = local_248;
        local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
        puStack_310 = (undefined *)0x141b05e10;
        thunk_FUN_1401be120(&local_198);
        puStack_310 = (undefined *)0x141b05e21;
        plVar16 = (longlong *)FUN_1409339d0(&local_1a0,&local_288);
        plVar16 = (longlong *)*plVar16;
        if (plVar16 == (longlong *)0x0) {
          iVar8 = -0x7fffbffe;
          pIVar33 = pIVar36;
        }
        else {
          puStack_310 = (undefined *)0x141b05e33;
          (**(code **)(*plVar16 + 8))(plVar16);
          local_268[0] = (IUnknown *)0x0;
          puStack_310 = (undefined *)0x141b05e52;
          iVar8 = (**(code **)*plVar16)(plVar16,&DAT_143272478,local_268);
          pIVar33 = (IUnknown *)0x0;
          if (-1 < iVar8) {
            pIVar33 = local_268[0];
          }
        }
        if (plVar16 != (longlong *)0x0) {
          puStack_310 = (undefined *)0x141b05e70;
          (**(code **)(*plVar16 + 0x10))(plVar16);
        }
        if (((iVar8 + 0x80000000U & 0x80000000) == 0) && (iVar8 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
          puStack_310 = (undefined *)0x141b0931c;
          FUN_142ef3ac0(iVar8);
        }
        if (pIVar33 != (IUnknown *)0x0) {
          pIVar36 = pIVar33;
        }
        local_e8 = pIVar36;
        if (local_1a0 != (longlong *)0x0) {
          puStack_310 = (undefined *)0x141b05ea9;
          (**(code **)(*local_1a0 + 0x10))();
        }
        if ((short)local_288 == 8) {
          local_288._0_2_ = 0;
          if (plStack_280 != (longlong *)0x0) {
            puStack_310 = (undefined *)0x141b05eca;
            (*DAT_143ad5990)((longlong)plStack_280 + -4);
          }
        }
        else {
          puStack_310 = (undefined *)0x141b05ed6;
          (*DAT_143262a18)(&local_288);
        }
        if ((short)local_1e8 == 8) {
          local_1e8 = (IUnknown *)((ulonglong)local_1e8 & 0xffffffffffff0000);
          if (plStack_1e0 != (longlong *)0x0) {
            puStack_310 = (undefined *)0x141b05eff;
            (*DAT_143ad5990)((longlong)plStack_1e0 + -4);
          }
        }
        else {
          puStack_310 = (undefined *)0x141b05f0e;
          (*DAT_143262a18)(&local_1e8);
        }
        if (local_218 == 8) {
          local_218 = 0;
          if (plStack_210 != (longlong *)0x0) {
            puStack_310 = (undefined *)0x141b05f2d;
            (*DAT_143ad5990)((longlong)plStack_210 + -4);
          }
        }
        else {
          puStack_310 = (undefined *)0x141b05f39;
          (*DAT_143262a18)(&local_218);
        }
        plVar16 = local_190;
        if (pIVar36 == (IUnknown *)0x0) goto LAB_141b06043;
        puStack_310 = (undefined *)0x141b05f53;
        uVar6 = (**(code **)(*local_190 + 0x20))(local_190);
        puStack_310 = (undefined *)0x141b05f64;
        local_268[0] = pIVar36;
        (**(code **)(*(longlong *)pIVar36 + 8))(pIVar36);
        local_2d0 = plVar16 + 0x1f;
        local_2a0 = param_14;
        local_2a8 = param_13;
        local_2b0 = &local_140;
        local_2b8 = &local_138;
        local_2c0 = &local_1a8;
        local_2c8 = &local_1fc;
        local_2d8 = &local_17c;
        local_2e0 = &local_e0;
        local_2e8 = (longlong *)CONCAT71(local_2e8._1_7_,uVar6);
        puStack_310 = (undefined *)0x141b05fed;
        plVar17 = (longlong *)FUN_141b027c0(&local_238,local_188,local_268,param_5);
        pIVar15 = local_1f0;
        pIVar33 = (IUnknown *)*plVar17;
        pIVar32 = local_1f0;
        if ((local_1f0 != pIVar33) &&
           (*plVar17 = 0, pIVar32 = pIVar33, local_1f0 != (IUnknown *)0x0)) {
          lVar24 = *(longlong *)local_1f0;
          puStack_310 = (undefined *)0x141b06015;
          local_1f0 = pIVar33;
          (**(code **)(lVar24 + 0x10))(pIVar15);
          pIVar32 = local_1f0;
        }
        local_1f0 = pIVar32;
        if (local_238 != (IUnknown *)0x0) {
          puStack_310 = (undefined *)0x141b0602b;
          (**(code **)(*(longlong *)local_238 + 0x10))();
        }
        puVar34 = auStack_308;
        iVar8 = (int)local_138;
      }
      pIVar33 = local_1f0;
      puVar14 = (undefined8 *)0x0;
      if (local_1f0 == (IUnknown *)0x0) goto LAB_141b09657;
      iVar10 = local_1fc - iVar8;
      pcVar3 = *(code **)(*(longlong *)local_1f0 + 0x100);
      *(undefined8 *)(puVar34 + -8) = 0x141b08173;
      iVar8 = (*pcVar3)(pIVar33,iVar8 + iVar10 / 2);
      if (iVar8 < 0) {
        *(undefined8 *)(puVar34 + -8) = 0x141b08188;
        _com_issue_errorex(iVar8,pIVar33,(_GUID *)&DAT_14327ac98);
      }
      pIVar33 = local_1f0;
      if (local_1f0 == (IUnknown *)0x0) goto LAB_141b09657;
      pcVar3 = *(code **)(*(longlong *)local_1f0 + 0x110);
      *(undefined8 *)(puVar34 + -8) = 0x141b081a9;
      iVar8 = (*pcVar3)(pIVar33,0xfffffffe);
      if (iVar8 < 0) {
        *(undefined8 *)(puVar34 + -8) = 0x141b081be;
        _com_issue_errorex(iVar8,pIVar33,(_GUID *)&DAT_14327ac98);
      }
      iVar8 = local_180;
      if (param_5 == 0x3f1) {
        local_130 = 0xfffffffe;
        bVar4 = false;
        if (*(char *)((longlong)local_190 + 0xd1) == '\0') {
          bVar4 = true;
        }
      }
      else {
        if ((param_5 == 0x3f2) || (param_5 == 0x3eb)) {
          local_130 = 0x2325;
LAB_141b0837e:
          bVar4 = false;
          if (param_5 == 0x3ef) {
            if ((char)local_190[0x1a] == '\0') {
              bVar4 = true;
            }
            goto LAB_141b08256;
          }
        }
        else {
          if ((param_5 != 1000) || (plVar16 == (longlong *)0x0)) goto LAB_141b0837e;
          pcVar3 = *(code **)(*plVar16 + 0xd0);
          *(undefined8 *)(puVar34 + -8) = 0x141b08234;
          cVar7 = (*pcVar3)(plVar16,0);
          if (cVar7 == '\0') {
            pcVar3 = *(code **)(*plVar16 + 0x58);
            *(undefined8 *)(puVar34 + -8) = 0x141b08241;
            iVar10 = (*pcVar3)(plVar16);
            if (iVar10 == 0) goto LAB_141b0837e;
          }
          local_130 = 0x2325;
        }
        bVar4 = false;
      }
LAB_141b08256:
      lVar24 = DAT_143add050;
      if (DAT_143add050 == 0) {
                    /* WARNING: Subroutine does not return */
        *(undefined **)(puVar34 + -8) = &UNK_141b09656;
        FUN_142ef3ac0(0x80004003);
      }
      *(undefined8 *)(puVar34 + -8) = 0x141b08270;
      (*DAT_143262a20)(&local_230);
      *(undefined8 *)(puVar34 + -8) = 0x141b08280;
      iVar10 = FUN_14023c4c0(&local_230,&DAT_143a8b8d8);
      if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
        *(undefined8 *)(puVar34 + -8) = 0x141b094f1;
        FUN_142ef3ac0(iVar10);
      }
      *(undefined8 *)(puVar34 + -8) = 0x141b08292;
      (*DAT_143262a20)(&local_288);
      *(undefined8 *)(puVar34 + -8) = 0x141b082a2;
      iVar10 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
      if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
        *(undefined8 *)(puVar34 + -8) = 0x141b094f9;
        FUN_142ef3ac0(iVar10);
      }
      *(undefined8 *)(puVar34 + -8) = 0x141b082b7;
      (*DAT_143262a20)(&local_1c8);
      *(undefined8 *)(puVar34 + -8) = 0x141b082ca;
      iVar10 = FUN_14023c4c0(&local_1c8,&DAT_143a8b8d8);
      if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
        *(undefined8 *)(puVar34 + -8) = 0x141b09501;
        FUN_142ef3ac0(iVar10);
      }
      local_258 = (IUnknown *)CONCAT62(local_258._2_6_,3);
      plStack_250 = (longlong *)((ulonglong)plStack_250 & 0xffffffff00000000);
      *(undefined8 **)(puVar34 + 0x50) = &local_230;
      *(undefined8 **)(puVar34 + 0x48) = &local_288;
      *(undefined8 **)(puVar34 + 0x40) = &local_1c8;
      *(undefined8 **)(puVar34 + 0x38) = &local_258;
      *(undefined4 *)(puVar34 + 0x30) = 0;
      *(undefined4 *)(puVar34 + 0x28) = 0;
      *(undefined4 *)(puVar34 + 0x20) = 0;
      *(undefined8 *)(puVar34 + -8) = 0x141b08323;
      plVar22 = (longlong *)FUN_140d8d300(lVar24,&local_270,0,0);
      plVar17 = (longlong *)plVar16[(longlong)iVar8 + 9];
      if (plVar17 != (longlong *)*plVar22) {
        plVar16[(longlong)iVar8 + 9] = *plVar22;
        *plVar22 = 0;
        if (plVar17 != (longlong *)0x0) {
          pcVar3 = *(code **)(*plVar17 + 0x10);
          *(undefined8 *)(puVar34 + -8) = 0x141b08342;
          (*pcVar3)();
        }
      }
      if (local_270 != (IUnknown *)0x0) {
        pcVar3 = *(code **)(*(IUnknown **)local_270 + 0x10);
        *(undefined8 *)(puVar34 + -8) = 0x141b08352;
        (*pcVar3)();
      }
      if ((short)local_258 == 8) {
        local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
        if (plStack_250 != (longlong *)0x0) {
          lVar24 = (longlong)plStack_250 + -4;
          *(undefined8 *)(puVar34 + -8) = 0x141b08371;
          (*DAT_143ad5990)(lVar24);
        }
      }
      else {
        *(undefined8 *)(puVar34 + -8) = 0x141b083b3;
        (*DAT_143262a18)(&local_258);
      }
      if ((short)local_1c8 == 8) {
        local_1c8 = (IUnknown *)((ulonglong)local_1c8 & 0xffffffffffff0000);
        if (plStack_1c0 != (longlong *)0x0) {
          lVar24 = (longlong)plStack_1c0 + -4;
          *(undefined8 *)(puVar34 + -8) = 0x141b083db;
          (*DAT_143ad5990)(lVar24);
        }
      }
      else {
        *(undefined8 *)(puVar34 + -8) = 0x141b083ea;
        (*DAT_143262a18)(&local_1c8);
      }
      if ((short)local_288 == 8) {
        local_288._0_2_ = 0;
        if (plStack_280 != (longlong *)0x0) {
          lVar24 = (longlong)plStack_280 + -4;
          *(undefined8 *)(puVar34 + -8) = 0x141b08409;
          (*DAT_143ad5990)(lVar24);
        }
      }
      else {
        *(undefined8 *)(puVar34 + -8) = 0x141b08415;
        (*DAT_143262a18)(&local_288);
      }
      if ((short)local_230 == 8) {
        local_230._0_2_ = 0;
        lVar24 = CONCAT44(uStack_224,uStack_228);
        if (lVar24 != 0) {
          *(undefined8 *)(puVar34 + -8) = 0x141b08434;
          (*DAT_143ad5990)(lVar24 + -4);
        }
      }
      else {
        *(undefined8 *)(puVar34 + -8) = 0x141b08440;
        (*DAT_143262a18)(&local_230);
      }
      pIVar33 = (IUnknown *)plVar16[(longlong)iVar8 + 9];
      if (pIVar33 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        *(undefined **)(puVar34 + -8) = &UNK_141b0964b;
        FUN_142ef3ac0(0x80004003);
      }
      plStack_280 = (longlong *)*param_3;
      local_288._0_2_ = 0xd;
      if (plStack_280 != (longlong *)0x0) {
        pcVar3 = *(code **)(*plStack_280 + 8);
        *(undefined8 *)(puVar34 + -8) = 0x141b08470;
        (*pcVar3)();
      }
      local_108 = CONCAT44(local_288._4_4_,CONCAT22(local_288._2_2_,(short)local_288));
      uStack_100 = plStack_280;
      local_f8 = local_278;
      pcVar3 = *(code **)(*(longlong *)pIVar33 + 0x238);
      *(undefined8 *)(puVar34 + -8) = 0x141b0849c;
      iVar10 = (*pcVar3)(pIVar33,&local_108);
      if (iVar10 < 0) {
        *(undefined8 *)(puVar34 + -8) = 0x141b084b1;
        _com_issue_errorex(iVar10,pIVar33,(_GUID *)&DAT_14327fcb0);
      }
      if ((short)local_288 == 8) {
        local_288._0_2_ = 0;
        if (plStack_280 != (longlong *)0x0) {
          lVar24 = (longlong)plStack_280 + -4;
          *(undefined8 *)(puVar34 + -8) = 0x141b084d0;
          (*DAT_143ad5990)(lVar24);
        }
      }
      else {
        *(undefined8 *)(puVar34 + -8) = 0x141b084dc;
        (*DAT_143262a18)(&local_288);
      }
      uVar11 = local_130;
      pIVar33 = (IUnknown *)plVar16[(longlong)iVar8 + 9];
      if (pIVar33 == (IUnknown *)0x0) goto LAB_141b09657;
      pcVar3 = *(code **)(*(longlong *)pIVar33 + 0x198);
      *(undefined8 *)(puVar34 + -8) = 0x141b084fc;
      iVar10 = (*pcVar3)(pIVar33,uVar11);
      if (iVar10 < 0) {
        *(undefined8 *)(puVar34 + -8) = 0x141b08511;
        _com_issue_errorex(iVar10,pIVar33,(_GUID *)&DAT_14327fcb0);
      }
      pIVar33 = (IUnknown *)plVar16[(longlong)iVar8 + 9];
      if (pIVar33 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
        *(undefined **)(puVar34 + -8) = &UNK_141b09640;
        FUN_142ef3ac0(0x80004003);
      }
      plStack_280 = (longlong *)*param_4;
      local_288._0_2_ = 0xd;
      if (plStack_280 != (longlong *)0x0) {
        pcVar3 = *(code **)(*plStack_280 + 8);
        *(undefined8 *)(puVar34 + -8) = 0x141b08540;
        (*pcVar3)();
      }
      local_108 = CONCAT44(local_288._4_4_,CONCAT22(local_288._2_2_,(short)local_288));
      uStack_100 = plStack_280;
      local_f8 = local_278;
      pcVar3 = *(code **)(*(longlong *)pIVar33 + 200);
      *(undefined8 *)(puVar34 + -8) = 0x141b0856c;
      iVar10 = (*pcVar3)(pIVar33,&local_108);
      if (iVar10 < 0) {
        *(undefined8 *)(puVar34 + -8) = 0x141b08581;
        _com_issue_errorex(iVar10,pIVar33,(_GUID *)&DAT_143273488);
      }
      if ((short)local_288 == 8) {
        local_288._0_2_ = 0;
        if (plStack_280 != (longlong *)0x0) {
          lVar24 = (longlong)plStack_280 + -4;
          *(undefined8 *)(puVar34 + -8) = 0x141b085a0;
          (*DAT_143ad5990)(lVar24);
        }
      }
      else {
        *(undefined8 *)(puVar34 + -8) = 0x141b085ac;
        (*DAT_143262a18)(&local_288);
      }
      pIVar33 = (IUnknown *)plVar16[(longlong)iVar8 + 9];
      if (pIVar33 == (IUnknown *)0x0) goto LAB_141b09657;
      puVar29 = (undefined8 *)0xffffffff;
      if (bVar4) {
        puVar29 = puVar14;
      }
      pcVar3 = *(code **)(*(longlong *)pIVar33 + 0x200);
      *(undefined8 *)(puVar34 + -8) = 0x141b085d1;
      iVar10 = (*pcVar3)(pIVar33,puVar29);
      if (iVar10 < 0) {
        *(undefined8 *)(puVar34 + -8) = 0x141b085e6;
        _com_issue_errorex(iVar10,pIVar33,(_GUID *)&DAT_14327fcb0);
      }
      puVar29 = local_d8;
      puVar23 = local_d8;
      if (local_e0._4_4_ == 0) {
        lVar24 = plVar16[(longlong)iVar8 + 9];
        if (lVar24 == 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined **)(puVar34 + -8) = &UNK_141b09635;
          FUN_142ef3ac0(0x80004003);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b089ba;
        (*DAT_143262a20)(&local_218);
        *(undefined8 *)(puVar34 + -8) = 0x141b089ca;
        iVar10 = FUN_14023c4c0(&local_218,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b09562;
          FUN_142ef3ac0(iVar10);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b089dc;
        (*DAT_143262a20)(&local_230);
        *(undefined8 *)(puVar34 + -8) = 0x141b089ec;
        iVar10 = FUN_14023c4c0(&local_230,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b0956a;
          FUN_142ef3ac0(iVar10);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b089fe;
        (*DAT_143262a20)(&local_288);
        *(undefined8 *)(puVar34 + -8) = 0x141b08a0e;
        iVar10 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b09572;
          FUN_142ef3ac0(iVar10);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b08a23;
        (*DAT_143262a20)(&local_1c8);
        *(undefined8 *)(puVar34 + -8) = 0x141b08a36;
        iVar10 = FUN_14023c4c0(&local_1c8,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b0957a;
          FUN_142ef3ac0(iVar10);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b08a48;
        (*DAT_143262a20)(&local_258);
        *(undefined8 *)(puVar34 + -8) = 0x141b08a58;
        iVar10 = FUN_14023c4c0(&local_258,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b09582;
          FUN_142ef3ac0(iVar10);
        }
        *(short **)(puVar34 + 0x38) = &local_218;
        *(undefined8 **)(puVar34 + 0x30) = &local_230;
        *(undefined8 **)(puVar34 + 0x28) = &local_288;
        *(undefined8 **)(puVar34 + 0x20) = &local_1c8;
        pIVar33 = local_1f0;
        *(undefined8 *)(puVar34 + -8) = 0x141b08aa1;
        FUN_140d5ec50(lVar24,&local_168,pIVar33,&local_258);
        if ((short)local_168 == 8) {
          local_168 = (IUnknown *)((ulonglong)local_168 & 0xffffffffffff0000);
          if (uStack_160 != (longlong *)0x0) {
            lVar24 = (longlong)uStack_160 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b08ac9;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b08ad8;
          (*DAT_143262a18)(&local_168);
        }
        if ((short)local_258 == 8) {
          local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
          if (plStack_250 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_250 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b08af7;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b08b03;
          (*DAT_143262a18)(&local_258);
        }
        if ((short)local_1c8 == 8) {
          local_1c8 = (IUnknown *)((ulonglong)local_1c8 & 0xffffffffffff0000);
          if (plStack_1c0 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_1c0 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b08b2b;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b08b3a;
          (*DAT_143262a18)(&local_1c8);
        }
        if ((short)local_288 == 8) {
          local_288._0_2_ = 0;
          if (plStack_280 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_280 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b08b59;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b08b65;
          (*DAT_143262a18)(&local_288);
        }
        if ((short)local_230 == 8) {
          local_230._0_2_ = 0;
          lVar24 = CONCAT44(uStack_224,uStack_228);
          if (lVar24 != 0) {
            *(undefined8 *)(puVar34 + -8) = 0x141b08b84;
            (*DAT_143ad5990)(lVar24 + -4);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b08b90;
          (*DAT_143262a18)(&local_230);
        }
        if (local_218 == 8) {
          local_218 = 0;
          plVar17 = plStack_210;
          goto LAB_141b08ba0;
        }
        psVar27 = &local_218;
LAB_141b08bb5:
        *(undefined8 *)(puVar34 + -8) = 0x141b08bbb;
        (*DAT_143262a18)(psVar27);
      }
      else {
        while (puVar23 != (undefined8 *)0x0) {
          plStack_1e0 = (longlong *)0x0;
          local_268[0] = (IUnknown *)*puVar23;
          if (local_268[0] != (IUnknown *)0x0) {
            pcVar3 = *(code **)(*(longlong *)local_268[0] + 8);
            *(undefined8 *)(puVar34 + -8) = 0x141b08629;
            (*pcVar3)();
          }
          local_238 = (IUnknown *)plVar16[(longlong)iVar8 + 9];
          if (local_238 != (IUnknown *)0x0) {
            pcVar3 = *(code **)(*(longlong *)local_238 + 8);
            *(undefined8 *)(puVar34 + -8) = 0x141b0863d;
            (*pcVar3)();
          }
          *(undefined4 *)(puVar34 + 0x28) = 0;
          *(IUnknown ***)(puVar34 + 0x20) = &local_1e8;
          *(undefined8 *)(puVar34 + -8) = 0x141b08661;
          FUN_140dc9820(&local_238,local_268,0,0);
          uVar25 = puVar23[-4];
          if ((uVar25 != 0) && (uVar25 < 0x10001)) {
            *(undefined8 *)(puVar34 + -8) = 0x141b08680;
            FUN_142e52ed0(0x33e);
            uVar25 = puVar23[-4];
          }
          puVar23 = puVar14;
          if (uVar25 != 0) {
            puVar23 = (undefined8 *)(uVar25 + 0x28);
          }
        }
        local_270 = (IUnknown *)plVar16[(longlong)iVar8 + 9];
        if (local_270 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          *(undefined **)(puVar34 + -8) = &UNK_141b0955a;
          FUN_142ef3ac0(0x80004003);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b086b6;
        (*DAT_143262a20)(&local_218);
        *(undefined8 *)(puVar34 + -8) = 0x141b086c6;
        iVar10 = FUN_14023c4c0(&local_218,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b09509;
          FUN_142ef3ac0(iVar10);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b086d8;
        (*DAT_143262a20)(&local_230);
        *(undefined8 *)(puVar34 + -8) = 0x141b086e8;
        iVar10 = FUN_14023c4c0(&local_230,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b09511;
          FUN_142ef3ac0(iVar10);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b086fa;
        (*DAT_143262a20)(&local_288);
        *(undefined8 *)(puVar34 + -8) = 0x141b0870a;
        iVar10 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b09519;
          FUN_142ef3ac0(iVar10);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b0871f;
        (*DAT_143262a20)(&local_1c8);
        *(undefined8 *)(puVar34 + -8) = 0x141b08732;
        iVar10 = FUN_14023c4c0(&local_1c8,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b09521;
          FUN_142ef3ac0(iVar10);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b08744;
        (*DAT_143262a20)(&local_258);
        *(undefined8 *)(puVar34 + -8) = 0x141b08754;
        iVar10 = FUN_14023c4c0(&local_258,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b09529;
          FUN_142ef3ac0(iVar10);
        }
        if (plVar16[(longlong)iVar8 + 9] == 0) {
LAB_141b09545:
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b0954f;
          FUN_142ef3ac0(0x80004003);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b0876e;
        iVar10 = FUN_1401cc540();
        iVar10 = iVar10 + local_17c;
        if (plVar16[(longlong)iVar8 + 9] == 0) goto LAB_141b09545;
        *(undefined8 *)(puVar34 + -8) = 0x141b08788;
        uVar11 = FUN_1401cc500();
        *(short **)(puVar34 + 0x38) = &local_218;
        *(undefined8 **)(puVar34 + 0x30) = &local_230;
        *(undefined8 **)(puVar34 + 0x28) = &local_288;
        *(undefined8 **)(puVar34 + 0x20) = &local_1c8;
        pIVar33 = local_270;
        *(undefined8 *)(puVar34 + -8) = 0x141b087c1;
        FUN_140458ff0(pIVar33,uVar11,iVar10,&local_258);
        if ((short)local_258 == 8) {
          local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
          if (plStack_250 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_250 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b087e0;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b087ec;
          (*DAT_143262a18)(&local_258);
        }
        if ((short)local_1c8 == 8) {
          local_1c8 = (IUnknown *)((ulonglong)local_1c8 & 0xffffffffffff0000);
          if (plStack_1c0 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_1c0 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b08814;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b08823;
          (*DAT_143262a18)(&local_1c8);
        }
        if ((short)local_288 == 8) {
          local_288._0_2_ = 0;
          if (plStack_280 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_280 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b08842;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b0884e;
          (*DAT_143262a18)(&local_288);
        }
        if ((short)local_230 == 8) {
          local_230._0_2_ = 0;
          lVar24 = CONCAT44(uStack_224,uStack_228);
          if (lVar24 != 0) {
            *(undefined8 *)(puVar34 + -8) = 0x141b0886d;
            (*DAT_143ad5990)(lVar24 + -4);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b08879;
          (*DAT_143262a18)(&local_230);
        }
        if (local_218 == 8) {
          local_218 = 0;
          if (plStack_210 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_210 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b08898;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b088a4;
          (*DAT_143262a18)(&local_218);
        }
        pIVar33 = (IUnknown *)plVar16[(longlong)iVar8 + 9];
        if (pIVar33 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b09544;
          FUN_142ef3ac0(0x80004003);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b088bc;
        (*DAT_143262a20)(&local_230);
        *(undefined8 *)(puVar34 + -8) = 0x141b088cc;
        iVar10 = FUN_14023c4c0(&local_230,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b09531;
          FUN_142ef3ac0(iVar10);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b088de;
        (*DAT_143262a20)(&local_288);
        *(undefined8 *)(puVar34 + -8) = 0x141b088ee;
        iVar10 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b09539;
          FUN_142ef3ac0(iVar10);
        }
        local_108 = CONCAT44(local_230._4_4_,CONCAT22(local_230._2_2_,(short)local_230));
        uStack_100 = (longlong *)CONCAT44(uStack_224,uStack_228);
        local_f8 = local_220;
        local_168 = (IUnknown *)CONCAT44(local_288._4_4_,CONCAT22(local_288._2_2_,(short)local_288))
        ;
        uStack_160 = plStack_280;
        local_158 = local_278;
        pcVar3 = *(code **)(*(longlong *)pIVar33 + 0x280);
        *(undefined8 *)(puVar34 + -8) = 0x141b08945;
        iVar10 = (*pcVar3)(pIVar33,0x20,&local_168,&local_108);
        if (iVar10 < 0) {
          *(undefined8 *)(puVar34 + -8) = 0x141b0895a;
          _com_issue_errorex(iVar10,pIVar33,(_GUID *)&DAT_14327fcb0);
        }
        if ((short)local_288 == 8) {
          local_288._0_2_ = 0;
          if (plStack_280 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_280 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b08979;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b08985;
          (*DAT_143262a18)(&local_288);
        }
        if ((short)local_230 != 8) {
          psVar27 = (short *)&local_230;
          goto LAB_141b08bb5;
        }
        local_230._0_2_ = 0;
        plVar17 = (longlong *)CONCAT44(uStack_224,uStack_228);
LAB_141b08ba0:
        if (plVar17 != (longlong *)0x0) {
          *(undefined8 *)(puVar34 + -8) = 0x141b08baf;
          (*DAT_143ad5990)((longlong)plVar17 + -4);
        }
      }
      plVar22 = local_190;
      plVar17 = plStack_280;
      if (param_5 == 0x3f1) {
        pIVar33 = (IUnknown *)plVar16[(longlong)iVar8 + 9];
        if (pIVar33 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
          *(undefined **)(puVar34 + -8) = &UNK_141b095d6;
          FUN_142ef3ac0(0x80004003);
        }
        local_288._0_2_ = 3;
        plStack_280 = (longlong *)((ulonglong)plStack_280 & 0xffffffff00000000);
        local_268[0] = (IUnknown *)0x0;
        local_108 = CONCAT44(local_288._4_4_,CONCAT22(local_288._2_2_,3));
        uStack_100 = (longlong *)((ulonglong)plVar17 & 0xffffffff00000000);
        local_f8 = local_278;
        pcVar3 = *(code **)(*(longlong *)pIVar33 + 0x240);
        *(undefined8 *)(puVar34 + -8) = 0x141b08c18;
        iVar10 = (*pcVar3)(pIVar33,&local_108,local_268);
        if (iVar10 < 0) {
          *(undefined8 *)(puVar34 + -8) = 0x141b08c2d;
          _com_issue_errorex(iVar10,pIVar33,(_GUID *)&DAT_14327fcb0);
        }
        pIVar33 = local_268[0];
        local_1e8 = local_268[0];
        if ((local_268[0] == (IUnknown *)0x0) ||
           (local_270 = (IUnknown *)plVar16[(longlong)iVar8 + 9], local_270 == (IUnknown *)0x0)) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b095cb;
          FUN_142ef3ac0(0x80004003);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b08c5a;
        iVar10 = FUN_140404240(pIVar33);
        pIVar33 = local_270;
        *(undefined8 *)(puVar34 + -8) = 0x141b08c70;
        iVar12 = FUN_1401cc540(pIVar33);
        local_1a8 = (undefined2 *)CONCAT44(local_1a8._4_4_,iVar12 + ((-10 - param_12) - iVar10));
        pcVar3 = *(code **)(*(longlong *)local_268[0] + 0x10);
        *(undefined8 *)(puVar34 + -8) = 0x141b08c82;
        (*pcVar3)();
        if ((short)local_288 == 8) {
          local_288._0_2_ = 0;
          if (plStack_280 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_280 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b08ca1;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b08cad;
          (*DAT_143262a18)(&local_288);
        }
        lVar24 = plVar16[(longlong)iVar8 + 9];
        if (lVar24 == 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b095c0;
          FUN_142ef3ac0(0x80004003);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b08cc5;
        (*DAT_143262a20)(&local_218);
        *(undefined8 *)(puVar34 + -8) = 0x141b08cd5;
        iVar10 = FUN_14023c4c0(&local_218,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b0958a;
          FUN_142ef3ac0(iVar10);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b08ce7;
        (*DAT_143262a20)(&local_230);
        *(undefined8 *)(puVar34 + -8) = 0x141b08cf7;
        iVar10 = FUN_14023c4c0(&local_230,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b09592;
          FUN_142ef3ac0(iVar10);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b08d09;
        (*DAT_143262a20)(&local_288);
        *(undefined8 *)(puVar34 + -8) = 0x141b08d19;
        iVar10 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b0959a;
          FUN_142ef3ac0(iVar10);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b08d2e;
        (*DAT_143262a20)(&local_1c8);
        *(undefined8 *)(puVar34 + -8) = 0x141b08d41;
        iVar10 = FUN_14023c4c0(&local_1c8,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b095a2;
          FUN_142ef3ac0(iVar10);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b08d53;
        (*DAT_143262a20)(&local_258);
        *(undefined8 *)(puVar34 + -8) = 0x141b08d63;
        iVar10 = FUN_14023c4c0(&local_258,&DAT_143a8b8d8);
        if (iVar10 < 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b095aa;
          FUN_142ef3ac0(iVar10);
        }
        if (plVar16[(longlong)iVar8 + 9] == 0) {
                    /* WARNING: Subroutine does not return */
          *(undefined8 *)(puVar34 + -8) = 0x141b095b5;
          FUN_142ef3ac0(0x80004003);
        }
        *(undefined8 *)(puVar34 + -8) = 0x141b08d7d;
        uVar11 = FUN_1401cc500();
        *(short **)(puVar34 + 0x38) = &local_218;
        *(undefined8 **)(puVar34 + 0x30) = &local_230;
        *(undefined8 **)(puVar34 + 0x28) = &local_288;
        *(undefined8 **)(puVar34 + 0x20) = &local_1c8;
        uVar25 = (ulonglong)local_1a8 & 0xffffffff;
        *(undefined8 *)(puVar34 + -8) = 0x141b08db9;
        FUN_140458ff0(lVar24,uVar11,uVar25,&local_258);
        if ((short)local_258 == 8) {
          local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
          if (plStack_250 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_250 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b08dd8;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b08de4;
          (*DAT_143262a18)(&local_258);
        }
        if ((short)local_1c8 == 8) {
          local_1c8 = (IUnknown *)((ulonglong)local_1c8 & 0xffffffffffff0000);
          if (plStack_1c0 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_1c0 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b08e0c;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b08e1b;
          (*DAT_143262a18)(&local_1c8);
        }
        if ((short)local_288 == 8) {
          local_288._0_2_ = 0;
          if (plStack_280 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_280 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b08e3a;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b08e46;
          (*DAT_143262a18)(&local_288);
        }
        if ((short)local_230 == 8) {
          local_230._0_2_ = 0;
          lVar24 = CONCAT44(uStack_224,uStack_228);
          if (lVar24 != 0) {
            *(undefined8 *)(puVar34 + -8) = 0x141b08e65;
            (*DAT_143ad5990)(lVar24 + -4);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b08e71;
          (*DAT_143262a18)(&local_230);
        }
LAB_141b0926a:
        if (local_218 == 8) {
          local_218 = 0;
          if (plStack_210 != (longlong *)0x0) {
            lVar24 = (longlong)plStack_210 + -4;
            *(undefined8 *)(puVar34 + -8) = 0x141b09288;
            (*DAT_143ad5990)(lVar24);
          }
        }
        else {
          *(undefined8 *)(puVar34 + -8) = 0x141b09294;
          (*DAT_143262a18)(&local_218);
        }
      }
      else {
        iVar10 = local_180 + -1;
        lVar24 = (longlong)iVar10;
        if (-1 < iVar10) {
          plVar17 = local_190 + lVar24 + 9;
          do {
            if (*plVar17 != 0) {
              lVar24 = local_190[(longlong)iVar10 + 9];
              if (lVar24 == 0) {
                    /* WARNING: Subroutine does not return */
                *(undefined **)(puVar34 + -8) = &UNK_141b0962a;
                FUN_142ef3ac0(0x80004003);
              }
              local_258 = (IUnknown *)CONCAT62(local_258._2_6_,3);
              plStack_250 = (longlong *)((ulonglong)plStack_250 & 0xffffffff00000000);
              *(undefined8 *)(puVar34 + -8) = 0x141b08fca;
              puVar23 = (undefined8 *)FUN_140ee7c90(lVar24,&local_1e8,&local_258);
              local_270 = (IUnknown *)*puVar23;
              if ((local_270 == (IUnknown *)0x0) || (plVar22[(longlong)iVar10 + 9] == 0)) {
                    /* WARNING: Subroutine does not return */
                *(undefined8 *)(puVar34 + -8) = 0x141b0961f;
                FUN_142ef3ac0(0x80004003);
              }
              *(undefined8 *)(puVar34 + -8) = 0x141b08fed;
              iVar10 = FUN_1401cc540();
              pIVar33 = local_270;
              *(undefined8 *)(puVar34 + -8) = 0x141b08ff8;
              local_17c = FUN_140404240(pIVar33);
              local_17c = iVar10 + 1 + local_17c;
              if (local_1e8 != (IUnknown *)0x0) {
                pcVar3 = *(code **)(*(longlong *)local_1e8 + 0x10);
                *(undefined8 *)(puVar34 + -8) = 0x141b09015;
                (*pcVar3)();
              }
              if ((short)local_258 == 8) {
                local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
                if (plStack_250 != (longlong *)0x0) {
                  lVar24 = (longlong)plStack_250 + -4;
                  *(undefined8 *)(puVar34 + -8) = 0x141b09034;
                  (*DAT_143ad5990)(lVar24);
                }
              }
              else {
                *(undefined8 *)(puVar34 + -8) = 0x141b09040;
                (*DAT_143262a18)(&local_258);
              }
              pIVar33 = (IUnknown *)local_190[9];
              if (pIVar33 == (IUnknown *)0x0) goto LAB_141b09657;
              local_180 = 0;
              pcVar3 = *(code **)(*(longlong *)pIVar33 + 0x2a8);
              *(undefined8 *)(puVar34 + -8) = 0x141b0906e;
              iVar10 = (*pcVar3)(pIVar33,&local_180);
              if (iVar10 < 0) {
                *(undefined8 *)(puVar34 + -8) = 0x141b09083;
                _com_issue_errorex(iVar10,pIVar33,(_GUID *)&DAT_14327fcb0);
              }
              iVar10 = local_17c;
              if (1 < local_180) {
                iVar10 = local_17c + (int)local_190[0x1f];
                *(undefined4 *)(local_190 + 0x1f) = 0;
              }
              local_270 = (IUnknown *)plVar16[(longlong)iVar8 + 9];
              if (local_270 == (IUnknown *)0x0) {
                    /* WARNING: Subroutine does not return */
                *(undefined8 *)(puVar34 + -8) = 0x141b09614;
                FUN_142ef3ac0(0x80004003);
              }
              *(undefined8 *)(puVar34 + -8) = 0x141b090c0;
              (*DAT_143262a20)(&local_218);
              *(undefined8 *)(puVar34 + -8) = 0x141b090d0;
              iVar12 = FUN_14023c4c0(&local_218,&DAT_143a8b8d8);
              if (iVar12 < 0) {
                    /* WARNING: Subroutine does not return */
                *(undefined8 *)(puVar34 + -8) = 0x141b095de;
                FUN_142ef3ac0(iVar12);
              }
              *(undefined8 *)(puVar34 + -8) = 0x141b090e2;
              (*DAT_143262a20)(&local_230);
              *(undefined8 *)(puVar34 + -8) = 0x141b090f2;
              iVar12 = FUN_14023c4c0(&local_230,&DAT_143a8b8d8);
              if (iVar12 < 0) {
                    /* WARNING: Subroutine does not return */
                *(undefined8 *)(puVar34 + -8) = 0x141b095e6;
                FUN_142ef3ac0(iVar12);
              }
              *(undefined8 *)(puVar34 + -8) = 0x141b09104;
              (*DAT_143262a20)(&local_288);
              *(undefined8 *)(puVar34 + -8) = 0x141b09114;
              iVar12 = FUN_14023c4c0(&local_288,&DAT_143a8b8d8);
              if (iVar12 < 0) {
                    /* WARNING: Subroutine does not return */
                *(undefined8 *)(puVar34 + -8) = 0x141b095ee;
                FUN_142ef3ac0(iVar12);
              }
              *(undefined8 *)(puVar34 + -8) = 0x141b09129;
              (*DAT_143262a20)(&local_1c8);
              *(undefined8 *)(puVar34 + -8) = 0x141b0913c;
              iVar12 = FUN_14023c4c0(&local_1c8,&DAT_143a8b8d8);
              if (iVar12 < 0) {
                    /* WARNING: Subroutine does not return */
                *(undefined8 *)(puVar34 + -8) = 0x141b095f6;
                FUN_142ef3ac0(iVar12);
              }
              *(undefined8 *)(puVar34 + -8) = 0x141b0914e;
              (*DAT_143262a20)(&local_258);
              *(undefined8 *)(puVar34 + -8) = 0x141b0915e;
              iVar12 = FUN_14023c4c0(&local_258,&DAT_143a8b8d8);
              if (iVar12 < 0) {
                    /* WARNING: Subroutine does not return */
                *(undefined8 *)(puVar34 + -8) = 0x141b095fe;
                FUN_142ef3ac0(iVar12);
              }
              if (plVar16[(longlong)iVar8 + 9] == 0) {
                    /* WARNING: Subroutine does not return */
                *(undefined8 *)(puVar34 + -8) = 0x141b09609;
                FUN_142ef3ac0(0x80004003);
              }
              *(undefined8 *)(puVar34 + -8) = 0x141b09178;
              uVar11 = FUN_1401cc500();
              *(short **)(puVar34 + 0x38) = &local_218;
              *(undefined8 **)(puVar34 + 0x30) = &local_230;
              *(undefined8 **)(puVar34 + 0x28) = &local_288;
              *(undefined8 **)(puVar34 + 0x20) = &local_1c8;
              pIVar33 = local_270;
              *(undefined8 *)(puVar34 + -8) = 0x141b091b1;
              FUN_140458ff0(pIVar33,uVar11,iVar10,&local_258);
              if ((short)local_258 == 8) {
                local_258 = (IUnknown *)((ulonglong)local_258 & 0xffffffffffff0000);
                if (plStack_250 != (longlong *)0x0) {
                  lVar24 = (longlong)plStack_250 + -4;
                  *(undefined8 *)(puVar34 + -8) = 0x141b091d0;
                  (*DAT_143ad5990)(lVar24);
                }
              }
              else {
                *(undefined8 *)(puVar34 + -8) = 0x141b091dc;
                (*DAT_143262a18)(&local_258);
              }
              if ((short)local_1c8 == 8) {
                local_1c8 = (IUnknown *)((ulonglong)local_1c8 & 0xffffffffffff0000);
                if (plStack_1c0 != (longlong *)0x0) {
                  lVar24 = (longlong)plStack_1c0 + -4;
                  *(undefined8 *)(puVar34 + -8) = 0x141b09204;
                  (*DAT_143ad5990)(lVar24);
                }
              }
              else {
                *(undefined8 *)(puVar34 + -8) = 0x141b09213;
                (*DAT_143262a18)(&local_1c8);
              }
              if ((short)local_288 == 8) {
                local_288._0_2_ = 0;
                if (plStack_280 != (longlong *)0x0) {
                  lVar24 = (longlong)plStack_280 + -4;
                  *(undefined8 *)(puVar34 + -8) = 0x141b09232;
                  (*DAT_143ad5990)(lVar24);
                }
              }
              else {
                *(undefined8 *)(puVar34 + -8) = 0x141b0923e;
                (*DAT_143262a18)(&local_288);
              }
              if ((short)local_230 == 8) {
                local_230._0_2_ = 0;
                lVar24 = CONCAT44(uStack_224,uStack_228);
                if (lVar24 != 0) {
                  *(undefined8 *)(puVar34 + -8) = 0x141b0925d;
                  (*DAT_143ad5990)(lVar24 + -4);
                }
              }
              else {
                *(undefined8 *)(puVar34 + -8) = 0x141b09269;
                (*DAT_143262a18)(&local_230);
              }
              goto LAB_141b0926a;
            }
            iVar10 = iVar10 + -1;
            plVar17 = plVar17 + -1;
            lVar24 = lVar24 + -1;
          } while (-1 < lVar24);
        }
      }
      plVar16 = local_190;
      if (DAT_143aa84a0 != 0) {
        *(undefined8 *)(puVar34 + -8) = 0x141b08ebb;
        cVar7 = FUN_142d0f320();
        if (cVar7 != '\0') {
          pcVar3 = *(code **)(*plVar16 + 0x18);
          *(undefined8 *)(puVar34 + -8) = 0x141b08ecd;
          (*pcVar3)(plVar16,0,0);
        }
      }
      if (local_1f0 == (IUnknown *)0x0) {
LAB_141b09657:
                    /* WARNING: Subroutine does not return */
        *(undefined8 *)(puVar34 + -8) = 0x141b09661;
        FUN_142ef3ac0(0x80004003);
      }
      *(undefined8 *)(puVar34 + -8) = 0x141b08ee2;
      uVar11 = FUN_140404240();
      if (pIVar36 != (IUnknown *)0x0) {
        pcVar3 = *(code **)(*(longlong *)pIVar36 + 0x10);
        *(undefined8 *)(puVar34 + -8) = 0x141b08ef4;
        (*pcVar3)(pIVar36);
      }
      if (psVar5 != (short *)0x0) {
        *(undefined8 *)(puVar34 + -8) = 0x141b08f03;
        FUN_1401bebb0(psVar5 + -8);
      }
      while (puVar29 != (undefined8 *)0x0) {
        puVar23 = puVar29 + -5;
        *(undefined8 *)(puVar34 + -8) = 0x141b08f1c;
        plVar16 = (longlong *)FUN_14030e750(puVar23);
        puVar29 = puVar14;
        if (*plVar16 != 0) {
          puVar29 = (undefined8 *)(*plVar16 + 0x28);
        }
        if (puVar23 != (undefined8 *)0x0) {
          pcVar3 = *(code **)*puVar23;
          *(undefined8 *)(puVar34 + -8) = 0x141b08f3f;
          (*pcVar3)(puVar23,1);
        }
      }
      if (local_1f0 != (IUnknown *)0x0) {
        pcVar3 = *(code **)(*(longlong *)local_1f0 + 0x10);
        *(undefined8 *)(puVar34 + -8) = 0x141b08f56;
        (*pcVar3)();
      }
      if (pIVar35 != (IUnknown *)0x0) {
        *(undefined8 *)(puVar34 + -8) = 0x141b08f65;
        FUN_14019f2c0(pIVar35 + -0x10);
      }
      if ((longlong *)*param_3 != (longlong *)0x0) {
        pcVar3 = *(code **)(*(longlong *)*param_3 + 0x10);
        *(undefined8 *)(puVar34 + -8) = 0x141b08f7b;
        (*pcVar3)();
      }
      if ((longlong *)*param_4 != (longlong *)0x0) {
        pcVar3 = *(code **)(*(longlong *)*param_4 + 0x10);
        *(undefined8 *)(puVar34 + -8) = 0x141b08f91;
        (*pcVar3)();
      }
      goto LAB_141b055cb;
    }
    if (param_2 != (IUnknown *)0x0) {
      lVar24 = -1;
      do {
        lVar24 = lVar24 + 1;
      } while (param_2[lVar24] != (IUnknown)0x0);
      if (lVar24 != 0) goto LAB_141b055f4;
    }
    if ((longlong *)param_1[(longlong)iVar8 + 9] != (longlong *)0x0) {
      puStack_310 = (undefined *)0x141b055a4;
      (**(code **)(*(longlong *)param_1[(longlong)iVar8 + 9] + 0x10))();
    }
    param_1[(longlong)iVar8 + 9] = 0;
    if ((longlong *)*param_3 != (longlong *)0x0) {
      puStack_310 = (undefined *)0x141b055b9;
      (**(code **)(*(longlong *)*param_3 + 0x10))();
    }
    param_4 = (longlong *)*param_4;
  }
  if (param_4 != (longlong *)0x0) {
    puStack_310 = (undefined *)0x141b055c8;
    (**(code **)(*param_4 + 0x10))();
  }
  uVar11 = 0;
LAB_141b055cb:
  *(undefined8 *)(puVar34 + -8) = 0x141b055da;
  return uVar11;
}



//===========================================================
// FUN_141ece680 @ 141ece680   (135 bytes)
//===========================================================

longlong * FUN_141ece680(longlong *param_1,longlong param_2)

{
  int iVar1;
  longlong lVar2;
  longlong local_res8;
  
  *param_1 = 0;
  if (param_2 == 0) {
    iVar1 = -0x7fffbffe;
  }
  else {
    local_res8 = 0;
    iVar1 = (*(code *)**(undefined8 **)(param_2 + 0x20))
                      ((undefined8 *)(param_2 + 0x20),&DAT_143273488,&local_res8);
    lVar2 = 0;
    if (-1 < iVar1) {
      lVar2 = local_res8;
    }
    if ((longlong *)*param_1 != (longlong *)0x0) {
      (**(code **)(*(longlong *)*param_1 + 0x10))();
    }
    *param_1 = lVar2;
  }
  if (((iVar1 + 0x80000000U & 0x80000000) == 0) && (iVar1 != -0x7fffbffe)) {
                    /* WARNING: Subroutine does not return */
    FUN_142ef3ac0(iVar1);
  }
  return param_1;
}


