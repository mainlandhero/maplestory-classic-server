
//===========================================================
// FUN_142c94bd0 @ 142c94bd0   (4167 bytes)
//===========================================================

void FUN_142c94bd0(longlong param_1,longlong *param_2)

{
  int *piVar1;
  longlong lVar2;
  bool bVar3;
  int iVar4;
  char *pcVar5;
  undefined8 *puVar6;
  undefined8 uVar7;
  int *piVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  char *pcVar11;
  char *pcVar12;
  ulonglong uVar13;
  int iVar14;
  char *local_res18;
  undefined8 local_res20;
  longlong local_48;
  int *local_40;
  
  FUN_1401d1140(param_2,&DAT_14349302c);
  uVar13 = 0;
  local_res20 = 0;
  FUN_14019a260(&local_res20,param_2);
  FUN_142ca0710(&local_res18,&local_res20,0);
  if ((local_res18 == (char *)0x0) || (*local_res18 == '\0')) {
    FUN_1429e4fa0("http://maplestory.nexon.net/micro-site/20701",0,0);
    local_40 = (int *)FUN_1418039d0(0x22000009);
    puVar6 = (undefined8 *)FUN_140cc21e0(&local_48,&local_40);
    FUN_141804870(&DAT_143271f04,0x1a4,0x22000009,*puVar6);
    if (local_48 != 0) {
      FUN_14019f2c0(local_48 + -0x10);
    }
  }
  else {
    pcVar5 = (char *)FUN_14019bd40(&local_res18,0,1);
    _strupr(pcVar5);
    iVar14 = 0;
    uVar10 = uVar13;
    if (local_res18 != (char *)0x0) {
      uVar10 = (ulonglong)*(uint *)(local_res18 + -8);
    }
    FUN_14019c870(&local_res18,uVar10);
    pcVar5 = DAT_143aa8360;
    pcVar11 = DAT_143aa8360;
    if (local_res18 != (char *)0x0) {
      pcVar11 = local_res18;
    }
    iVar4 = strcmp(pcVar11,"WEBSTART");
    if (iVar4 == 0) {
      local_48 = 0;
      FUN_14019a260(&local_48,param_2);
      puVar6 = (undefined8 *)FUN_142ca0710(&local_40,&local_48,1);
      if (((char *)*puVar6 == (char *)0x0) || (*(char *)*puVar6 == '\0')) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      if (local_40 != (int *)0x0) {
        FUN_14019f2c0(local_40 + -4);
      }
      if (bVar3) {
        iVar14 = 4;
        do {
          local_48 = 0;
          FUN_14019a260(&local_48,param_2);
          puVar6 = (undefined8 *)FUN_142ca0710(&local_40,&local_48,iVar14);
          lVar2 = *(longlong *)(uVar13 + 0x90 + param_1);
          if (lVar2 != 0) {
            FUN_14019f2c0(lVar2 + -0x10);
            *(undefined8 *)(uVar13 + 0x90 + param_1) = 0;
          }
          *(undefined8 *)(uVar13 + 0x90 + param_1) = *puVar6;
          *puVar6 = 0;
          if (local_40 != (int *)0x0) {
            FUN_14019f2c0(local_40 + -4);
          }
          iVar14 = iVar14 + 1;
          uVar13 = uVar13 + 8;
        } while ((longlong)uVar13 < 0x30);
        *(undefined4 *)(param_1 + 0x38) = 3;
      }
    }
    else {
      uVar10 = uVar13;
      pcVar11 = pcVar5;
      if (local_res18 != (char *)0x0) {
        pcVar11 = local_res18;
      }
      do {
        uVar9 = uVar10 + 1;
        if (pcVar11[uVar10] != (&DAT_14349303c)[uVar10]) {
          pcVar11 = pcVar5;
          if (local_res18 != (char *)0x0) {
            pcVar11 = local_res18;
          }
          iVar4 = strcmp(pcVar11,"-NXLDEBUG");
          if (iVar4 != 0) {
            uVar10 = uVar13;
            pcVar11 = pcVar5;
            if (local_res18 != (char *)0x0) {
              pcVar11 = local_res18;
            }
            goto LAB_142c95120;
          }
          local_48 = 0;
          FUN_14019a260(&local_48,param_2);
          puVar6 = (undefined8 *)FUN_142ca0710(&local_40,&local_48,1);
          if (((char *)*puVar6 == (char *)0x0) || (*(char *)*puVar6 == '\0')) {
            bVar3 = false;
          }
          else {
            bVar3 = true;
          }
          if (local_40 != (int *)0x0) {
            FUN_14019f2c0(local_40 + -4);
          }
          if (bVar3) {
            local_48 = 0;
            FUN_14019a260(&local_48,param_2);
            uVar7 = FUN_142ca0710(&local_40,&local_48,1);
            FUN_140319ad0(param_1 + 0x18,uVar7);
            if (local_40 != (int *)0x0) {
              FUN_14019f2c0(local_40 + -4);
            }
            local_48 = 0;
            FUN_14019a260(&local_48,param_2);
            uVar7 = FUN_142ca0710(&local_40,&local_48,2);
            FUN_140319ad0(param_1 + 0x20,uVar7);
            if (local_40 != (int *)0x0) {
              FUN_14019f2c0(local_40 + -4);
            }
            iVar14 = 3;
            do {
              local_48 = 0;
              FUN_14019a260(&local_48,param_2);
              puVar6 = (undefined8 *)FUN_142ca0710(&local_40,&local_48,iVar14);
              lVar2 = *(longlong *)(uVar13 + 0x90 + param_1);
              if (lVar2 != 0) {
                FUN_14019f2c0(lVar2 + -0x10);
                *(undefined8 *)(uVar13 + 0x90 + param_1) = 0;
              }
              *(undefined8 *)(uVar13 + 0x90 + param_1) = *puVar6;
              *puVar6 = 0;
              if (local_40 != (int *)0x0) {
                FUN_14019f2c0(local_40 + -4);
              }
              iVar14 = iVar14 + 1;
              uVar13 = uVar13 + 8;
            } while ((longlong)uVar13 < 0x30);
            *(undefined4 *)(param_1 + 0x38) = 5;
          }
          goto LAB_142c95be0;
        }
        uVar10 = uVar9;
      } while (uVar9 != 5);
      local_48 = 0;
      FUN_14019a260(&local_48,param_2);
      puVar6 = (undefined8 *)FUN_142ca0710(&local_40,&local_48,1);
      if (((char *)*puVar6 == (char *)0x0) || (*(char *)*puVar6 == '\0')) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      if (local_40 != (int *)0x0) {
        FUN_14019f2c0(local_40 + -4);
      }
      if (bVar3) {
        local_48 = 0;
        FUN_14019a260(&local_48,param_2);
        uVar7 = FUN_142ca0710(&local_40,&local_48,1);
        FUN_140319ad0(param_1 + 0x80,uVar7);
        if (local_40 != (int *)0x0) {
          FUN_14019f2c0(local_40 + -4);
        }
        local_48 = 0;
        FUN_14019a260(&local_48,param_2);
        uVar7 = FUN_142ca0710(&local_40,&local_48,2);
        FUN_140319ad0(param_1 + 0xc0,uVar7);
        if (local_40 != (int *)0x0) {
          FUN_14019f2c0(local_40 + -4);
        }
        local_48 = 0;
        FUN_14019a260(&local_48,param_2);
        uVar7 = FUN_142ca0710(&local_40,&local_48,3);
        FUN_140319ad0(param_1 + 0x18,uVar7);
        if (local_40 != (int *)0x0) {
          FUN_14019f2c0(local_40 + -4);
        }
        local_48 = 0;
        FUN_14019a260(&local_48,param_2);
        uVar7 = FUN_142ca0710(&local_40,&local_48,4);
        FUN_140319ad0(param_1 + 0x20,uVar7);
        if (local_40 != (int *)0x0) {
          FUN_14019f2c0(local_40 + -4);
        }
        iVar14 = 5;
        do {
          local_48 = 0;
          FUN_14019a260(&local_48,param_2);
          puVar6 = (undefined8 *)FUN_142ca0710(&local_40,&local_48,iVar14);
          lVar2 = *(longlong *)(uVar13 + 0x90 + param_1);
          if (lVar2 != 0) {
            FUN_14019f2c0(lVar2 + -0x10);
            *(undefined8 *)(uVar13 + 0x90 + param_1) = 0;
          }
          *(undefined8 *)(uVar13 + 0x90 + param_1) = *puVar6;
          *puVar6 = 0;
          if (local_40 != (int *)0x0) {
            FUN_14019f2c0(local_40 + -4);
          }
          iVar14 = iVar14 + 1;
          uVar13 = uVar13 + 8;
        } while ((longlong)uVar13 < 0x30);
        *(undefined4 *)(param_1 + 0x38) = 5;
      }
    }
  }
  goto LAB_142c95be0;
  while (uVar10 = uVar9, uVar9 != 8) {
LAB_142c95120:
    uVar9 = uVar10 + 1;
    if (pcVar11[uVar10] != (&DAT_143493058)[uVar10]) {
      pcVar11 = pcVar5;
      if (local_res18 != (char *)0x0) {
        pcVar11 = local_res18;
      }
      iVar4 = strcmp(pcVar11,"STEAMSTART");
      if (iVar4 == 0) {
        local_48 = 0;
        FUN_14019a260(&local_48,param_2);
        puVar6 = (undefined8 *)FUN_142ca0710(&local_40,&local_48,1);
        if (((char *)*puVar6 == (char *)0x0) || (*(char *)*puVar6 == '\0')) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        if (local_40 != (int *)0x0) {
          FUN_14019f2c0(local_40 + -4);
        }
        if (!bVar3) goto LAB_142c95be0;
        local_48 = 0;
        FUN_14019a260(&local_48,param_2);
        uVar7 = FUN_142ca0710(&local_40,&local_48,1);
        FUN_140319ad0(param_1 + 0x80,uVar7);
        if (local_40 != (int *)0x0) {
          FUN_14019f2c0(local_40 + -4);
        }
        local_48 = 0;
        FUN_14019a260(&local_48,param_2);
        uVar7 = FUN_142ca0710(&local_40,&local_48,2);
        FUN_140319ad0(param_1 + 0xc0,uVar7);
        if (local_40 != (int *)0x0) {
          FUN_14019f2c0(local_40 + -4);
        }
        local_48 = 0;
        FUN_14019a260(&local_48,param_2);
        uVar7 = FUN_142ca0710(&local_40,&local_48,3);
        FUN_140319ad0(param_1 + 0x88,uVar7);
        if (local_40 != (int *)0x0) {
          FUN_14019f2c0(local_40 + -4);
        }
        FUN_142c9f2d0(param_1 + 0x88);
        pcVar5 = *(char **)(param_1 + 0x88);
        uVar10 = uVar13;
        pcVar11 = DAT_143aa8360;
        if (pcVar5 != (char *)0x0) {
          pcVar11 = pcVar5;
        }
        goto LAB_142c954c0;
      }
      pcVar11 = pcVar5;
      if (local_res18 != (char *)0x0) {
        pcVar11 = local_res18;
      }
      iVar14 = strcmp(pcVar11,"GAMELAUNCHING");
      if (iVar14 != 0) {
        uVar10 = uVar13;
        if (local_res18 != (char *)0x0) {
          pcVar5 = local_res18;
        }
        goto LAB_142c958c0;
      }
      piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,0x13);
      piVar8[1] = 2;
      *piVar8 = -1;
      piVar1 = piVar8 + 4;
      piVar8[2] = 0;
      *(undefined1 *)piVar1 = 0;
      *(undefined2 *)piVar1 = DAT_143493080;
      local_40 = piVar1;
      if (*piVar8 != -1) {
        FUN_142e52dd0(0x8b);
      }
      if (piVar8[1] < 2) {
        FUN_142e54290(0x90,piVar8[1],2);
      }
      *piVar8 = 1;
      *(undefined1 *)((longlong)piVar8 + 0x12) = 0;
      if (piVar8[1] + 1 < 3) {
        FUN_142e54290(0x9c,2);
      }
      piVar8[2] = 2;
      if (*(longlong *)(param_1 + 0xc0) != 0) {
        FUN_14019f2c0(*(longlong *)(param_1 + 0xc0) + -0x10);
      }
      *(int **)(param_1 + 0xc0) = piVar1;
      local_48 = 0;
      FUN_14019a260(&local_48,param_2);
      uVar7 = FUN_142ca0710(&local_40,&local_48,1);
      FUN_140319ad0(param_1 + 0x18,uVar7);
      if (local_40 != (int *)0x0) {
        FUN_14019f2c0(local_40 + -4);
      }
      local_48 = 0;
      FUN_14019a260(&local_48,param_2);
      uVar7 = FUN_142ca0710(&local_40,&local_48,2);
      FUN_140319ad0(param_1 + 0x20,uVar7);
      if (local_40 != (int *)0x0) {
        FUN_14019f2c0(local_40 + -4);
      }
      pcVar11 = DAT_143aa8360;
      pcVar5 = *(char **)(param_1 + 0x18);
      pcVar12 = DAT_143aa8360;
      if (pcVar5 != (char *)0x0) {
        pcVar12 = pcVar5;
      }
      iVar14 = strcmp(pcVar12,"10.9.2.131");
      if (iVar14 != 0) {
        pcVar12 = pcVar11;
        if (pcVar5 != (char *)0x0) {
          pcVar12 = pcVar5;
        }
        iVar14 = strcmp(pcVar12,"10.9.2.132");
        if (iVar14 != 0) {
          pcVar12 = pcVar11;
          if (pcVar5 != (char *)0x0) {
            pcVar12 = pcVar5;
          }
          iVar14 = strcmp(pcVar12,"10.9.2.133");
          if (iVar14 != 0) {
            pcVar12 = pcVar11;
            if (pcVar5 != (char *)0x0) {
              pcVar12 = pcVar5;
            }
            iVar14 = strcmp(pcVar12,"44.234.166.161");
            if (iVar14 != 0) {
              pcVar12 = pcVar11;
              if (pcVar5 != (char *)0x0) {
                pcVar12 = pcVar5;
              }
              iVar14 = strcmp(pcVar12,"44.234.167.163");
              if (iVar14 != 0) {
                if (pcVar5 != (char *)0x0) {
                  pcVar11 = pcVar5;
                }
                iVar14 = strcmp(pcVar11,"44.234.163.43");
                if (iVar14 != 0) {
                  FUN_1429e4fa0("http://maplestory.nexon.net/micro-site/20701",0,0);
                  uVar7 = 0x16d;
                  goto LAB_142c95b71;
                }
              }
            }
          }
        }
      }
      iVar14 = 3;
      do {
        local_48 = 0;
        FUN_14019a260(&local_48,param_2);
        puVar6 = (undefined8 *)FUN_142ca0710(&local_40,&local_48,iVar14);
        lVar2 = *(longlong *)(uVar13 + 0x90 + param_1);
        if (lVar2 != 0) {
          FUN_14019f2c0(lVar2 + -0x10);
          *(undefined8 *)(uVar13 + 0x90 + param_1) = 0;
        }
        *(undefined8 *)(uVar13 + 0x90 + param_1) = *puVar6;
        *puVar6 = 0;
        if (local_40 != (int *)0x0) {
          FUN_14019f2c0(local_40 + -4);
        }
        iVar14 = iVar14 + 1;
        uVar13 = uVar13 + 8;
      } while ((longlong)uVar13 < 0x30);
      *(undefined4 *)(param_1 + 0x38) = 2;
      goto LAB_142c95be0;
    }
  }
  local_48 = 0;
  FUN_14019a260(&local_48,param_2);
  puVar6 = (undefined8 *)FUN_142ca0710(&local_40,&local_48,1);
  if (((char *)*puVar6 == (char *)0x0) || (*(char *)*puVar6 == '\0')) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  if (local_40 != (int *)0x0) {
    FUN_14019f2c0(local_40 + -4);
  }
  if (bVar3) {
    local_48 = 0;
    FUN_14019a260(&local_48,param_2);
    uVar7 = FUN_142ca0710(&local_40,&local_48,1);
    FUN_140319ad0(param_1 + 0x80,uVar7);
    if (local_40 != (int *)0x0) {
      FUN_14019f2c0(local_40 + -4);
    }
    local_48 = 0;
    FUN_14019a260(&local_48,param_2);
    uVar7 = FUN_142ca0710(&local_40,&local_48,2);
    FUN_140319ad0(param_1 + 0xc0,uVar7);
    if (local_40 != (int *)0x0) {
      FUN_14019f2c0(local_40 + -4);
    }
    local_48 = 0;
    FUN_14019a260(&local_48,param_2);
    uVar7 = FUN_142ca0710(&local_40,&local_48,3);
    FUN_140319ad0(param_1 + 0x18,uVar7);
    if (local_40 != (int *)0x0) {
      FUN_14019f2c0(local_40 + -4);
    }
    local_48 = 0;
    FUN_14019a260(&local_48,param_2);
    uVar7 = FUN_142ca0710(&local_40,&local_48,4);
    FUN_140319ad0(param_1 + 0x20,uVar7);
    if (local_40 != (int *)0x0) {
      FUN_14019f2c0(local_40 + -4);
    }
    iVar14 = 5;
    do {
      local_48 = 0;
      FUN_14019a260(&local_48,param_2);
      puVar6 = (undefined8 *)FUN_142ca0710(&local_40,&local_48,iVar14);
      lVar2 = *(longlong *)(uVar13 + 0x90 + param_1);
      if (lVar2 != 0) {
        FUN_14019f2c0(lVar2 + -0x10);
        *(undefined8 *)(uVar13 + 0x90 + param_1) = 0;
      }
      *(undefined8 *)(uVar13 + 0x90 + param_1) = *puVar6;
      *puVar6 = 0;
      if (local_40 != (int *)0x0) {
        FUN_14019f2c0(local_40 + -4);
      }
      iVar14 = iVar14 + 1;
      uVar13 = uVar13 + 8;
    } while ((longlong)uVar13 < 0x30);
    *(undefined4 *)(param_1 + 0x38) = 5;
    *(undefined4 *)(param_1 + 200) = 1;
    DAT_143a88df8 = 0;
  }
  goto LAB_142c95be0;
  while (uVar10 = uVar9, uVar9 != 4) {
LAB_142c954c0:
    uVar9 = uVar10 + 1;
    if (pcVar11[uVar10] != (&DAT_14349306c)[uVar10]) {
      pcVar11 = DAT_143aa8360;
      if (pcVar5 != (char *)0x0) {
        pcVar11 = pcVar5;
      }
      if ((*pcVar11 != '-') || (pcVar11[1] != '\0')) goto LAB_142c954ee;
      break;
    }
  }
  iVar14 = 1;
LAB_142c954ee:
  local_48 = 0;
  FUN_14019a260(&local_48,param_2);
  uVar7 = FUN_142ca0710(&local_40,&local_48,iVar14 + 3);
  FUN_140319ad0(param_1 + 0x18,uVar7);
  if (local_40 != (int *)0x0) {
    FUN_14019f2c0(local_40 + -4);
  }
  local_48 = 0;
  FUN_14019a260(&local_48,param_2);
  uVar7 = FUN_142ca0710(&local_40,&local_48,iVar14 + 4);
  FUN_140319ad0(param_1 + 0x20,uVar7);
  if (local_40 != (int *)0x0) {
    FUN_14019f2c0(local_40 + -4);
  }
  iVar14 = iVar14 + 5;
  do {
    local_48 = 0;
    FUN_14019a260(&local_48,param_2);
    puVar6 = (undefined8 *)FUN_142ca0710(&local_40,&local_48,iVar14);
    lVar2 = *(longlong *)(uVar13 + 0x90 + param_1);
    if (lVar2 != 0) {
      FUN_14019f2c0(lVar2 + -0x10);
      *(undefined8 *)(uVar13 + 0x90 + param_1) = 0;
    }
    *(undefined8 *)(uVar13 + 0x90 + param_1) = *puVar6;
    *puVar6 = 0;
    if (local_40 != (int *)0x0) {
      FUN_14019f2c0(local_40 + -4);
    }
    iVar14 = iVar14 + 1;
    uVar13 = uVar13 + 8;
  } while ((longlong)uVar13 < 0x30);
  *(undefined4 *)(param_1 + 0x38) = 4;
  goto LAB_142c95be0;
  while (uVar10 = uVar9 + 1, uVar9 + 1 != 7) {
LAB_142c958c0:
    uVar9 = uVar10;
    if (pcVar5[uVar9] != "IPPORT"[uVar9]) {
      FUN_1429e4fa0("http://maplestory.nexon.net/micro-site/20701",0,0);
      uVar7 = 0x195;
      goto LAB_142c95b71;
    }
  }
  piVar8 = (int *)FUN_14019b600(&DAT_143ad6a30,(int)uVar9 + 0xd);
  piVar8[1] = 2;
  *piVar8 = -1;
  piVar1 = piVar8 + 4;
  piVar8[2] = 0;
  *(undefined1 *)piVar1 = 0;
  *(undefined2 *)piVar1 = DAT_143493080;
  local_40 = piVar1;
  if (*piVar8 != -1) {
    FUN_142e52dd0(0x8b);
  }
  if (piVar8[1] < 2) {
    FUN_142e54290(0x90,piVar8[1],2);
  }
  *piVar8 = 1;
  *(undefined1 *)((longlong)piVar8 + 0x12) = 0;
  if (piVar8[1] + 1 < 3) {
    FUN_142e54290(0x9c,2);
  }
  piVar8[2] = 2;
  if (*(longlong *)(param_1 + 0xc0) != 0) {
    FUN_14019f2c0(*(longlong *)(param_1 + 0xc0) + -0x10);
  }
  *(int **)(param_1 + 0xc0) = piVar1;
  local_48 = 0;
  FUN_14019a260(&local_48,param_2);
  uVar7 = FUN_142ca0710(&local_40,&local_48,1);
  FUN_140319ad0(param_1 + 0x18,uVar7);
  if (local_40 != (int *)0x0) {
    FUN_14019f2c0(local_40 + -4);
  }
  local_48 = 0;
  FUN_14019a260(&local_48,param_2);
  uVar7 = FUN_142ca0710(&local_40,&local_48,2);
  FUN_140319ad0(param_1 + 0x20,uVar7);
  if (local_40 != (int *)0x0) {
    FUN_14019f2c0(local_40 + -4);
  }
  pcVar11 = DAT_143aa8360;
  pcVar5 = *(char **)(param_1 + 0x18);
  pcVar12 = DAT_143aa8360;
  if (pcVar5 != (char *)0x0) {
    pcVar12 = pcVar5;
  }
  iVar14 = strcmp(pcVar12,"10.9.2.131");
  if (iVar14 != 0) {
    pcVar12 = pcVar11;
    if (pcVar5 != (char *)0x0) {
      pcVar12 = pcVar5;
    }
    iVar14 = strcmp(pcVar12,"10.9.2.132");
    if (iVar14 != 0) {
      pcVar12 = pcVar11;
      if (pcVar5 != (char *)0x0) {
        pcVar12 = pcVar5;
      }
      iVar14 = strcmp(pcVar12,"10.9.2.133");
      if (iVar14 != 0) {
        pcVar12 = pcVar11;
        if (pcVar5 != (char *)0x0) {
          pcVar12 = pcVar5;
        }
        iVar14 = strcmp(pcVar12,"44.234.166.161");
        if (iVar14 != 0) {
          pcVar12 = pcVar11;
          if (pcVar5 != (char *)0x0) {
            pcVar12 = pcVar5;
          }
          iVar14 = strcmp(pcVar12,"44.234.167.163");
          if (iVar14 != 0) {
            if (pcVar5 != (char *)0x0) {
              pcVar11 = pcVar5;
            }
            iVar14 = strcmp(pcVar11,"44.234.163.43");
            if (iVar14 != 0) {
              FUN_1429e4fa0("http://maplestory.nexon.net/micro-site/20701",0,0);
              uVar7 = 0x18a;
LAB_142c95b71:
              FUN_140cc2350(&DAT_143271f04,uVar7,0x22000009);
              goto LAB_142c95be0;
            }
          }
        }
      }
    }
  }
  iVar14 = 3;
  do {
    local_48 = 0;
    FUN_14019a260(&local_48,param_2);
    puVar6 = (undefined8 *)FUN_142ca0710(&local_40,&local_48,iVar14);
    lVar2 = *(longlong *)(uVar13 + 0x90 + param_1);
    if (lVar2 != 0) {
      FUN_14019f2c0(lVar2 + -0x10);
      *(undefined8 *)(uVar13 + 0x90 + param_1) = 0;
    }
    *(undefined8 *)(uVar13 + 0x90 + param_1) = *puVar6;
    *puVar6 = 0;
    if (local_40 != (int *)0x0) {
      FUN_14019f2c0(local_40 + -4);
    }
    iVar14 = iVar14 + 1;
    uVar13 = uVar13 + 8;
  } while ((longlong)uVar13 < 0x30);
  *(undefined4 *)(param_1 + 0x38) = 2;
LAB_142c95be0:
  if (local_res18 != (char *)0x0) {
    FUN_14019f2c0(local_res18 + -0x10);
  }
  if (*param_2 != 0) {
    FUN_14019f2c0(*param_2 + -0x10);
  }
  return;
}


