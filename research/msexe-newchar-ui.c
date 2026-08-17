
//===========================================================
// FUN_141122420 @ 141122420   (1580 bytes)
//===========================================================

void FUN_141122420(longlong *param_1,undefined4 param_2)

{
  longlong *plVar1;
  longlong lVar2;
  char cVar3;
  undefined8 uVar4;
  undefined8 *puVar5;
  undefined1 local_res8 [8];
  undefined1 local_18 [8];
  longlong local_10;
  
  if (param_1[0x47] == 0) {
    return;
  }
  cVar3 = FUN_141b3faf0();
  if (cVar3 != '\0') {
    return;
  }
  plVar1 = param_1 + 0x48;
  cVar3 = FUN_142aa1a20(plVar1,&DAT_14336f5f0,param_2);
  if (cVar3 != '\0') {
    FUN_141ad7ec0(plVar1,local_18,L"edit_name");
    if (local_10 != 0) {
      lVar2 = param_1[0x47];
      uVar4 = FUN_141126e80(local_10,local_res8);
      cVar3 = FUN_141b28950(lVar2,uVar4);
      if (cVar3 != '\0') {
        FUN_141b3fb10(param_1[0x47]);
      }
    }
    lVar2 = local_10;
    if (local_10 != 0) {
      if (0xffffe < *(longlong *)(local_10 + 0x20) - 1U) {
        FUN_142e541f0(0x31e);
      }
      LOCK();
      plVar1 = (longlong *)(lVar2 + 0x20);
      lVar2 = *plVar1;
      *plVar1 = *plVar1 + -1;
      UNLOCK();
      if ((int)lVar2 == 1) {
        puVar5 = (undefined8 *)(local_10 + 0x18);
        if (local_10 == 0) {
          puVar5 = (undefined8 *)0x0;
        }
        if (puVar5 != (undefined8 *)0x0) {
          (**(code **)*puVar5)(puVar5,1);
        }
      }
    }
    return;
  }
  cVar3 = FUN_142aa1a20(plVar1,L"cancel",param_2);
  if (cVar3 == '\0') {
    cVar3 = FUN_142aa1a20(plVar1,L"gender_prev",param_2);
    if ((cVar3 == '\0') && (cVar3 = FUN_142aa1a20(plVar1,L"gender_next",param_2), cVar3 == '\0')) {
      cVar3 = FUN_142aa1a20(plVar1,L"face_prev",param_2);
      if (cVar3 == '\0') {
        cVar3 = FUN_142aa1a20(plVar1,L"face_next",param_2);
        if (cVar3 == '\0') {
          cVar3 = FUN_142aa1a20(plVar1,L"hairstyle_prev",param_2);
          if (cVar3 == '\0') {
            cVar3 = FUN_142aa1a20(plVar1,L"hairstyle_next",param_2);
            if (cVar3 == '\0') {
              cVar3 = FUN_142aa1a20(plVar1,L"haircolor_prev",param_2);
              if (cVar3 == '\0') {
                cVar3 = FUN_142aa1a20(plVar1,L"haircolor_next",param_2);
                if (cVar3 == '\0') {
                  cVar3 = FUN_142aa1a20(plVar1,L"skincolor_prev",param_2);
                  if (cVar3 == '\0') {
                    cVar3 = FUN_142aa1a20(plVar1,L"skincolor_next",param_2);
                    if (cVar3 == '\0') {
                      cVar3 = FUN_142aa1a20(plVar1,L"top_prev",param_2);
                      if (cVar3 == '\0') {
                        cVar3 = FUN_142aa1a20(plVar1,L"top_next",param_2);
                        if (cVar3 == '\0') {
                          cVar3 = FUN_142aa1a20(plVar1,L"bottom_prev",param_2);
                          if (cVar3 == '\0') {
                            cVar3 = FUN_142aa1a20(plVar1,L"bottom_next",param_2);
                            if (cVar3 == '\0') {
                              cVar3 = FUN_142aa1a20(plVar1,L"shoes_prev",param_2);
                              if (cVar3 == '\0') {
                                cVar3 = FUN_142aa1a20(plVar1,L"shoes_next",param_2);
                                if (cVar3 == '\0') {
                                  cVar3 = FUN_142aa1a20(plVar1,L"weapon_prev",param_2);
                                  if (cVar3 == '\0') {
                                    cVar3 = FUN_142aa1a20(plVar1,L"weapon_next",param_2);
                                    if (cVar3 == '\0') {
                                      cVar3 = FUN_142aa1a20(plVar1,L"str_prev",param_2);
                                      if (cVar3 == '\0') {
                                        cVar3 = FUN_142aa1a20(plVar1,L"str_next",param_2);
                                        if (cVar3 == '\0') {
                                          cVar3 = FUN_142aa1a20(plVar1,L"dex_prev",param_2);
                                          if (cVar3 != '\0') {
                                            FUN_141b3df70(param_1[0x47],1,0xffffffffffffffff);
                                            goto LAB_141122a24;
                                          }
                                          cVar3 = FUN_142aa1a20(plVar1,L"dex_next",param_2);
                                          if (cVar3 != '\0') {
                                            FUN_141b3df70(param_1[0x47],1,1);
                                            goto LAB_141122a24;
                                          }
                                          cVar3 = FUN_142aa1a20(plVar1,L"int_prev",param_2);
                                          if (cVar3 != '\0') {
                                            FUN_141b3df70(param_1[0x47],2,0xffffffffffffffff);
                                            goto LAB_141122a24;
                                          }
                                          cVar3 = FUN_142aa1a20(plVar1,L"int_next",param_2);
                                          if (cVar3 == '\0') {
                                            cVar3 = FUN_142aa1a20(plVar1,L"luk_prev",param_2);
                                            if (cVar3 != '\0') {
                                              FUN_141b3df70(param_1[0x47],3,0xffffffffffffffff);
                                              goto LAB_141122a24;
                                            }
                                            cVar3 = FUN_142aa1a20(plVar1,L"luk_next",param_2);
                                            if (cVar3 == '\0') {
                                              cVar3 = FUN_142aa1a20(plVar1,L"check_name",param_2);
                                              if (cVar3 != '\0') {
                                                FUN_141ad7ec0(plVar1,local_18,L"edit_name");
                                                if (local_10 != 0) {
                                                  lVar2 = param_1[0x47];
                                                  uVar4 = FUN_141126e80(local_10,local_res8);
                                                  FUN_141b28950(lVar2,uVar4);
                                                }
                                                FUN_140d835a0(local_18);
                                                return;
                                              }
                                              goto LAB_141122a24;
                                            }
                                            uVar4 = 3;
                                          }
                                          else {
                                            uVar4 = 2;
                                          }
                                        }
                                        else {
                                          uVar4 = 0;
                                        }
                                        FUN_141b3df70(param_1[0x47],uVar4,1);
                                      }
                                      else {
                                        FUN_141b3df70(param_1[0x47],0,0xffffffffffffffff);
                                      }
                                    }
                                    else {
                                      FUN_141125340(param_1,6,1);
                                    }
                                  }
                                  else {
                                    FUN_141125340(param_1,6,0xffffffffffffffff);
                                  }
                                }
                                else {
                                  FUN_141125340(param_1,5,1);
                                }
                              }
                              else {
                                FUN_141125340(param_1,5,0xffffffffffffffff);
                              }
                            }
                            else {
                              FUN_141125340(param_1,4,1);
                            }
                          }
                          else {
                            FUN_141125340(param_1,4,0xffffffffffffffff);
                          }
                        }
                        else {
                          FUN_141125340(param_1,3,1);
                        }
                      }
                      else {
                        FUN_141125340(param_1,3,0xffffffffffffffff);
                      }
                    }
                    else {
                      FUN_141125580(param_1,1);
                    }
                  }
                  else {
                    FUN_141125580(param_1,0xffffffffffffffff);
                  }
                }
                else {
                  FUN_1411254b0(param_1,1);
                }
              }
              else {
                FUN_1411254b0(param_1,0xffffffffffffffff);
              }
            }
            else {
              FUN_141125340(param_1,2,1);
            }
          }
          else {
            FUN_141125340(param_1,2,0xffffffffffffffff);
          }
        }
        else {
          FUN_141125340(param_1,1,1);
        }
      }
      else {
        FUN_141125340(param_1,1,0xffffffffffffffff);
      }
    }
    else {
      FUN_1411252d0(param_1);
    }
  }
  else {
    FUN_141b3f050(param_1[0x47],4,0x14a);
  }
LAB_141122a24:
  (**(code **)(*param_1 + 0x90))(param_1,0);
  FUN_142bf5d70(param_1,param_2);
  return;
}



//===========================================================
// FUN_141177a10 @ 141177a10   (249 bytes)
//===========================================================

void FUN_141177a10(longlong param_1,undefined4 param_2)

{
  char cVar1;
  
  if ((*(longlong *)(param_1 + 0x248) != 0) && (cVar1 = FUN_141b3faf0(), cVar1 == '\0')) {
    cVar1 = FUN_142aa1a20(param_1 + 0x250,L"select",param_2);
    if (cVar1 != '\0') {
      FUN_141179410(param_1);
      return;
    }
    cVar1 = FUN_142aa1a20(param_1 + 0x250,&DAT_1433881d0,param_2);
    if (cVar1 == '\0') {
      cVar1 = FUN_142aa1a20(param_1 + 0x250,L"delete",param_2);
      if (cVar1 == '\0') {
        FUN_142bf5d70(param_1,param_2);
      }
      else if (*(longlong *)(param_1 + 0x248) != 0) {
        FUN_141b28750();
        return;
      }
    }
    else {
      cVar1 = FUN_140c9e3f0();
      if ((cVar1 != '\0') && (*(longlong *)(param_1 + 0x248) != 0)) {
        FUN_141b282d0();
        return;
      }
    }
  }
  return;
}


