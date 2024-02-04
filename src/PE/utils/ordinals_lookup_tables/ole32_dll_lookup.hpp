/* Copyright 2017 - 2024 R. Thomas
 * Copyright 2017 - 2024 Quarkslab
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#ifndef LIEF_PE_OLE32_DLL_LOOKUP_H
#define LIEF_PE_OLE32_DLL_LOOKUP_H

#include <cstdint>

namespace LIEF {
namespace PE {

inline const char* ole32_dll_lookup(uint32_t i) {
  switch(i) {
  case 0x0002: return "BindMoniker";
  case 0x0003: return "CLIPFORMAT_UserFree";
  case 0x0004: return "CLIPFORMAT_UserMarshal";
  case 0x0005: return "CLIPFORMAT_UserSize";
  case 0x0006: return "CLIPFORMAT_UserUnmarshal";
  case 0x0007: return "CLSIDFromOle1Class";
  case 0x0008: return "CLSIDFromProgID";
  case 0x0009: return "CLSIDFromProgIDEx";
  case 0x000a: return "CLSIDFromString";
  case 0x000b: return "CoAddRefServerProcess";
  case 0x000c: return "CoAllowSetForegroundWindow";
  case 0x000d: return "CoBuildVersion";
  case 0x000e: return "CoCancelCall";
  case 0x000f: return "CoCopyProxy";
  case 0x0010: return "CoCreateFreeThreadedMarshaler";
  case 0x0011: return "CoCreateGuid";
  case 0x0012: return "CoCreateInstance";
  case 0x0013: return "CoCreateInstanceEx";
  case 0x0014: return "CoCreateObjectInContext";
  case 0x0015: return "CoDeactivateObject";
  case 0x0016: return "CoDisableCallCancellation";
  case 0x0017: return "CoDisconnectObject";
  case 0x0018: return "CoDosDateTimeToFileTime";
  case 0x0019: return "CoEnableCallCancellation";
  case 0x001a: return "CoFileTimeNow";
  case 0x001b: return "CoFileTimeToDosDateTime";
  case 0x001c: return "CoFreeAllLibraries";
  case 0x001d: return "CoFreeLibrary";
  case 0x001e: return "CoFreeUnusedLibraries";
  case 0x001f: return "CoFreeUnusedLibrariesEx";
  case 0x0020: return "CoGetApartmentID";
  case 0x0021: return "CoGetCallContext";
  case 0x0022: return "CoGetCallerTID";
  case 0x0023: return "CoGetCancelObject";
  case 0x0024: return "CoGetClassObject";
  case 0x0025: return "CoGetClassVersion";
  case 0x0026: return "CoGetComCatalog";
  case 0x0027: return "CoGetContextToken";
  case 0x0028: return "CoGetCurrentLogicalThreadId";
  case 0x0029: return "CoGetCurrentProcess";
  case 0x002a: return "CoGetDefaultContext";
  case 0x002b: return "CoGetInstanceFromFile";
  case 0x002c: return "CoGetInstanceFromIStorage";
  case 0x002d: return "CoGetInterceptor";
  case 0x002e: return "CoGetInterceptorFromTypeInfo";
  case 0x002f: return "CoGetInterfaceAndReleaseStream";
  case 0x0030: return "CoGetMalloc";
  case 0x0031: return "CoGetMarshalSizeMax";
  case 0x0032: return "CoGetObject";
  case 0x0033: return "CoGetObjectContext";
  case 0x0034: return "CoGetPSClsid";
  case 0x0035: return "CoGetProcessIdentifier";
  case 0x0036: return "CoGetStandardMarshal";
  case 0x0037: return "CoGetState";
  case 0x0038: return "CoGetStdMarshalEx";
  case 0x0039: return "CoGetSystemSecurityPermissions";
  case 0x003a: return "CoGetTreatAsClass";
  case 0x003b: return "CoImpersonateClient";
  case 0x003c: return "CoInitialize";
  case 0x003d: return "CoInitializeEx";
  case 0x003e: return "CoInitializeSecurity";
  case 0x003f: return "CoInitializeWOW";
  case 0x0040: return "CoInstall";
  case 0x0041: return "CoInvalidateRemoteMachineBindings";
  case 0x0042: return "CoIsHandlerConnected";
  case 0x0043: return "CoIsOle1Class";
  case 0x0044: return "CoLoadLibrary";
  case 0x0045: return "CoLockObjectExternal";
  case 0x0046: return "CoMarshalHresult";
  case 0x0047: return "CoMarshalInterThreadInterfaceInStream";
  case 0x0048: return "CoMarshalInterface";
  case 0x0049: return "CoPopServiceDomain";
  case 0x004a: return "CoPushServiceDomain";
  case 0x004b: return "CoQueryAuthenticationServices";
  case 0x004c: return "CoQueryClientBlanket";
  case 0x004d: return "CoQueryProxyBlanket";
  case 0x004e: return "CoQueryReleaseObject";
  case 0x004f: return "CoReactivateObject";
  case 0x0050: return "CoRegisterChannelHook";
  case 0x0051: return "CoRegisterClassObject";
  case 0x0052: return "CoRegisterInitializeSpy";
  case 0x0053: return "CoRegisterMallocSpy";
  case 0x0054: return "CoRegisterMessageFilter";
  case 0x0055: return "CoRegisterPSClsid";
  case 0x0056: return "CoRegisterSurrogate";
  case 0x0057: return "CoRegisterSurrogateEx";
  case 0x0058: return "CoReleaseMarshalData";
  case 0x0059: return "CoReleaseServerProcess";
  case 0x005a: return "CoResumeClassObjects";
  case 0x005b: return "CoRetireServer";
  case 0x005c: return "CoRevertToSelf";
  case 0x005d: return "CoRevokeClassObject";
  case 0x005e: return "CoRevokeInitializeSpy";
  case 0x005f: return "CoRevokeMallocSpy";
  case 0x0060: return "CoSetCancelObject";
  case 0x0061: return "CoSetProxyBlanket";
  case 0x0062: return "CoSetState";
  case 0x0063: return "CoSuspendClassObjects";
  case 0x0064: return "CoSwitchCallContext";
  case 0x0065: return "CoTaskMemAlloc";
  case 0x0066: return "CoTaskMemFree";
  case 0x0067: return "CoTaskMemRealloc";
  case 0x0068: return "CoTestCancel";
  case 0x0069: return "CoTreatAsClass";
  case 0x006a: return "CoUninitialize";
  case 0x006b: return "CoUnloadingWOW";
  case 0x006c: return "CoUnmarshalHresult";
  case 0x006d: return "CoUnmarshalInterface";
  case 0x006e: return "CoWaitForMultipleHandles";
  case 0x006f: return "ComPs_CStdStubBuffer_AddRef";
  case 0x0070: return "ComPs_CStdStubBuffer_Connect";
  case 0x0071: return "ComPs_CStdStubBuffer_CountRefs";
  case 0x0072: return "ComPs_CStdStubBuffer_DebugServerQueryInterface";
  case 0x0073: return "ComPs_CStdStubBuffer_DebugServerRelease";
  case 0x0074: return "ComPs_CStdStubBuffer_Disconnect";
  case 0x0075: return "ComPs_CStdStubBuffer_Invoke";
  case 0x0076: return "ComPs_CStdStubBuffer_IsIIDSupported";
  case 0x0077: return "ComPs_CStdStubBuffer_QueryInterface";
  case 0x0078: return "ComPs_IUnknown_AddRef_Proxy";
  case 0x0079: return "ComPs_IUnknown_QueryInterface_Proxy";
  case 0x007a: return "ComPs_IUnknown_Release_Proxy";
  case 0x007b: return "ComPs_NdrCStdStubBuffer2_Release";
  case 0x007c: return "ComPs_NdrCStdStubBuffer_Release";
  case 0x007d: return "ComPs_NdrClientCall2";
  case 0x007e: return "ComPs_NdrClientCall2_va";
  case 0x007f: return "ComPs_NdrDllCanUnloadNow";
  case 0x0080: return "ComPs_NdrDllGetClassObject";
  case 0x0081: return "ComPs_NdrDllRegisterProxy";
  case 0x0082: return "ComPs_NdrDllUnregisterProxy";
  case 0x0083: return "ComPs_NdrStubCall2";
  case 0x0084: return "ComPs_NdrStubForwardingFunction";
  case 0x0085: return "CreateAntiMoniker";
  case 0x0086: return "CreateBindCtx";
  case 0x0087: return "CreateClassMoniker";
  case 0x0088: return "CreateDataAdviseHolder";
  case 0x0089: return "CreateDataCache";
  case 0x008a: return "CreateErrorInfo";
  case 0x008b: return "CreateFileMoniker";
  case 0x008c: return "CreateGenericComposite";
  case 0x008d: return "CreateILockBytesOnHGlobal";
  case 0x008e: return "CreateItemMoniker";
  case 0x008f: return "CreateObjrefMoniker";
  case 0x0090: return "CreateOleAdviseHolder";
  case 0x0091: return "CreatePointerMoniker";
  case 0x0092: return "CreateStdProgressIndicator";
  case 0x0093: return "CreateStreamOnHGlobal";
  case 0x0094: return "DcomChannelSetHResult";
  case 0x0095: return "DllDebugObjectRPCHook";
  case 0x0096: return "DllGetClassObject";
  case 0x0097: return "DllGetClassObjectWOW";
  case 0x0098: return "DllRegisterServer";
  case 0x0099: return "DoDragDrop";
  case 0x009a: return "EnableHookObject";
  case 0x009b: return "FmtIdToPropStgName";
  case 0x009c: return "FreePropVariantArray";
  case 0x009d: return "GetClassFile";
  case 0x009e: return "GetConvertStg";
  case 0x009f: return "GetDocumentBitStg";
  case 0x00a0: return "GetErrorInfo";
  case 0x00a1: return "GetHGlobalFromILockBytes";
  case 0x00a2: return "GetHGlobalFromStream";
  case 0x00a3: return "GetHookInterface";
  case 0x00a4: return "GetRunningObjectTable";
  case 0x00a5: return "HACCEL_UserFree";
  case 0x00a6: return "HACCEL_UserMarshal";
  case 0x00a7: return "HACCEL_UserSize";
  case 0x00a8: return "HACCEL_UserUnmarshal";
  case 0x00a9: return "HBITMAP_UserFree";
  case 0x00aa: return "HBITMAP_UserMarshal";
  case 0x00ab: return "HBITMAP_UserSize";
  case 0x00ac: return "HBITMAP_UserUnmarshal";
  case 0x00ad: return "HBRUSH_UserFree";
  case 0x00ae: return "HBRUSH_UserMarshal";
  case 0x00af: return "HBRUSH_UserSize";
  case 0x00b0: return "HBRUSH_UserUnmarshal";
  case 0x00b1: return "HDC_UserFree";
  case 0x00b2: return "HDC_UserMarshal";
  case 0x00b3: return "HDC_UserSize";
  case 0x00b4: return "HDC_UserUnmarshal";
  case 0x00b5: return "HENHMETAFILE_UserFree";
  case 0x00b6: return "HENHMETAFILE_UserMarshal";
  case 0x00b7: return "HENHMETAFILE_UserSize";
  case 0x00b8: return "HENHMETAFILE_UserUnmarshal";
  case 0x00b9: return "HGLOBAL_UserFree";
  case 0x00ba: return "HGLOBAL_UserMarshal";
  case 0x00bb: return "HGLOBAL_UserSize";
  case 0x00bc: return "HGLOBAL_UserUnmarshal";
  case 0x00bd: return "HICON_UserFree";
  case 0x00be: return "HICON_UserMarshal";
  case 0x00bf: return "HICON_UserSize";
  case 0x00c0: return "HICON_UserUnmarshal";
  case 0x00c1: return "HMENU_UserFree";
  case 0x00c2: return "HMENU_UserMarshal";
  case 0x00c3: return "HMENU_UserSize";
  case 0x00c4: return "HMENU_UserUnmarshal";
  case 0x00c5: return "HMETAFILEPICT_UserFree";
  case 0x00c6: return "HMETAFILEPICT_UserMarshal";
  case 0x00c7: return "HMETAFILEPICT_UserSize";
  case 0x00c8: return "HMETAFILEPICT_UserUnmarshal";
  case 0x00c9: return "HMETAFILE_UserFree";
  case 0x00ca: return "HMETAFILE_UserMarshal";
  case 0x00cb: return "HMETAFILE_UserSize";
  case 0x00cc: return "HMETAFILE_UserUnmarshal";
  case 0x00cd: return "HPALETTE_UserFree";
  case 0x00ce: return "HPALETTE_UserMarshal";
  case 0x00cf: return "HPALETTE_UserSize";
  case 0x00d0: return "HPALETTE_UserUnmarshal";
  case 0x00d1: return "HWND_UserFree";
  case 0x00d2: return "HWND_UserMarshal";
  case 0x00d3: return "HWND_UserSize";
  case 0x00d4: return "HWND_UserUnmarshal";
  case 0x00d5: return "HkOleRegisterObject";
  case 0x00d6: return "IIDFromString";
  case 0x00d7: return "IsAccelerator";
  case 0x00d8: return "IsEqualGUID";
  case 0x00d9: return "IsValidIid";
  case 0x00da: return "IsValidInterface";
  case 0x00db: return "IsValidPtrIn";
  case 0x00dc: return "IsValidPtrOut";
  case 0x00dd: return "MkParseDisplayName";
  case 0x00de: return "MonikerCommonPrefixWith";
  case 0x00df: return "MonikerRelativePathTo";
  case 0x00e0: return "OleBuildVersion";
  case 0x00e1: return "OleConvertIStorageToOLESTREAM";
  case 0x00e2: return "OleConvertIStorageToOLESTREAMEx";
  case 0x00e3: return "OleConvertOLESTREAMToIStorage";
  case 0x00e4: return "OleConvertOLESTREAMToIStorageEx";
  case 0x00e5: return "OleCreate";
  case 0x00e6: return "OleCreateDefaultHandler";
  case 0x00e7: return "OleCreateEmbeddingHelper";
  case 0x00e8: return "OleCreateEx";
  case 0x00e9: return "OleCreateFromData";
  case 0x00ea: return "OleCreateFromDataEx";
  case 0x00eb: return "OleCreateFromFile";
  case 0x00ec: return "OleCreateFromFileEx";
  case 0x00ed: return "OleCreateLink";
  case 0x00ee: return "OleCreateLinkEx";
  case 0x00ef: return "OleCreateLinkFromData";
  case 0x00f0: return "OleCreateLinkFromDataEx";
  case 0x00f1: return "OleCreateLinkToFile";
  case 0x00f2: return "OleCreateLinkToFileEx";
  case 0x00f3: return "OleCreateMenuDescriptor";
  case 0x00f4: return "OleCreateStaticFromData";
  case 0x00f5: return "OleDestroyMenuDescriptor";
  case 0x00f6: return "OleDoAutoConvert";
  case 0x00f7: return "OleDraw";
  case 0x00f8: return "OleDuplicateData";
  case 0x00f9: return "OleFlushClipboard";
  case 0x00fa: return "OleGetAutoConvert";
  case 0x00fb: return "OleGetClipboard";
  case 0x00fc: return "OleGetIconOfClass";
  case 0x00fd: return "OleGetIconOfFile";
  case 0x00fe: return "OleInitialize";
  case 0x00ff: return "OleInitializeWOW";
  case 0x0100: return "OleIsCurrentClipboard";
  case 0x0101: return "OleIsRunning";
  case 0x0102: return "OleLoad";
  case 0x0103: return "OleLoadFromStream";
  case 0x0104: return "OleLockRunning";
  case 0x0105: return "OleMetafilePictFromIconAndLabel";
  case 0x0106: return "OleNoteObjectVisible";
  case 0x0107: return "OleQueryCreateFromData";
  case 0x0108: return "OleQueryLinkFromData";
  case 0x0109: return "OleRegEnumFormatEtc";
  case 0x010a: return "OleRegEnumVerbs";
  case 0x010b: return "OleRegGetMiscStatus";
  case 0x010c: return "OleRegGetUserType";
  case 0x010d: return "OleRun";
  case 0x010e: return "OleSave";
  case 0x010f: return "OleSaveToStream";
  case 0x0110: return "OleSetAutoConvert";
  case 0x0111: return "OleSetClipboard";
  case 0x0112: return "OleSetContainedObject";
  case 0x0113: return "OleSetMenuDescriptor";
  case 0x0114: return "OleTranslateAccelerator";
  case 0x0115: return "OleUninitialize";
  case 0x0116: return "OpenOrCreateStream";
  case 0x0117: return "ProgIDFromCLSID";
  case 0x0118: return "PropStgNameToFmtId";
  case 0x0119: return "PropSysAllocString";
  case 0x011a: return "PropSysFreeString";
  case 0x0001: return "PropVariantChangeType";
  case 0x011b: return "PropVariantClear";
  case 0x011c: return "PropVariantCopy";
  case 0x011d: return "ReadClassStg";
  case 0x011e: return "ReadClassStm";
  case 0x011f: return "ReadFmtUserTypeStg";
  case 0x0120: return "ReadOleStg";
  case 0x0121: return "ReadStringStream";
  case 0x0122: return "RegisterDragDrop";
  case 0x0123: return "ReleaseStgMedium";
  case 0x0124: return "RevokeDragDrop";
  case 0x0125: return "SNB_UserFree";
  case 0x0126: return "SNB_UserMarshal";
  case 0x0127: return "SNB_UserSize";
  case 0x0128: return "SNB_UserUnmarshal";
  case 0x0129: return "STGMEDIUM_UserFree";
  case 0x012a: return "STGMEDIUM_UserMarshal";
  case 0x012b: return "STGMEDIUM_UserSize";
  case 0x012c: return "STGMEDIUM_UserUnmarshal";
  case 0x012d: return "SetConvertStg";
  case 0x012e: return "SetDocumentBitStg";
  case 0x012f: return "SetErrorInfo";
  case 0x0130: return "StgConvertPropertyToVariant";
  case 0x0131: return "StgConvertVariantToProperty";
  case 0x0132: return "StgCreateDocfile";
  case 0x0133: return "StgCreateDocfileOnILockBytes";
  case 0x0134: return "StgCreatePropSetStg";
  case 0x0135: return "StgCreatePropStg";
  case 0x0136: return "StgCreateStorageEx";
  case 0x0137: return "StgGetIFillLockBytesOnFile";
  case 0x0138: return "StgGetIFillLockBytesOnILockBytes";
  case 0x0139: return "StgIsStorageFile";
  case 0x013a: return "StgIsStorageILockBytes";
  case 0x013b: return "StgOpenAsyncDocfileOnIFillLockBytes";
  case 0x013c: return "StgOpenPropStg";
  case 0x013d: return "StgOpenStorage";
  case 0x013e: return "StgOpenStorageEx";
  case 0x013f: return "StgOpenStorageOnHandle";
  case 0x0140: return "StgOpenStorageOnILockBytes";
  case 0x0141: return "StgPropertyLengthAsVariant";
  case 0x0142: return "StgSetTimes";
  case 0x0143: return "StringFromCLSID";
  case 0x0144: return "StringFromGUID2";
  case 0x0145: return "StringFromIID";
  case 0x0146: return "UpdateDCOMSettings";
  case 0x0147: return "UtConvertDvtd16toDvtd32";
  case 0x0148: return "UtConvertDvtd32toDvtd16";
  case 0x0149: return "UtGetDvtd16Info";
  case 0x014a: return "UtGetDvtd32Info";
  case 0x014b: return "WdtpInterfacePointer_UserFree";
  case 0x014c: return "WdtpInterfacePointer_UserMarshal";
  case 0x014d: return "WdtpInterfacePointer_UserSize";
  case 0x014e: return "WdtpInterfacePointer_UserUnmarshal";
  case 0x014f: return "WriteClassStg";
  case 0x0150: return "WriteClassStm";
  case 0x0151: return "WriteFmtUserTypeStg";
  case 0x0152: return "WriteOleStg";
  case 0x0153: return "WriteStringStream";
  }
  return nullptr;
}


}
}

#endif

