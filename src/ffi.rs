use libc::c_void;
use winapi::{ DWORD, BOOL, BYTE, XINPUT_CAPABILITIES, ERROR_DEVICE_NOT_CONNECTED, XINPUT_VIBRATION,
XINPUT_FLAG_GAMEPAD, XINPUT_BATTERY_INFORMATION, XINPUT_KEYSTROKE, XINPUT_STATE, LPWSTR, UINT, GUID };

#[link(name="Xinput")]
extern {
    pub fn XInputEnable(enable : BOOL) -> c_void;
    pub fn XInputGetAudioDeviceIds(dwUserIndex : DWORD, pRender : LPWSTR, pRenderCount :  *mut UINT, pCaptureDeviceId : LPWSTR, pCaptureCount : *mut UINT ) -> DWORD;
    pub fn XInputGetBatteryInformation(dwUserIndex : DWORD, devType : BYTE, pBatteryInformation : *mut XINPUT_BATTERY_INFORMATION ) -> DWORD;
    pub fn XInputGetCapabilities( dwUserIndex : DWORD, dwFlags : DWORD, pCapabilities : *mut XINPUT_CAPABILITIES) -> DWORD;
    pub fn XInputGetDSoundAudioDeviceGuids( dwUserIndex : DWORD, pDSoundRenderGuid : *mut GUID, pSoundCaptureGuid : *mut GUID) -> DWORD;
    pub fn XInputGetKeystroke(dwUserIndex : DWORD, devType : BYTE, pBatteryInformation : *mut XINPUT_KEYSTROKE ) -> DWORD;
    pub fn XInputGetState( dwUserIndex : DWORD, pState : *mut XINPUT_STATE) -> DWORD;
    pub fn XInputSetState( dwUserIndex : DWORD, pVibration : *mut XINPUT_VIBRATION ) -> DWORD;
    //Not exported as a name: AFAIK apps use DLL ordinals to get this function
    //fn XInputPowerOffController( dwUserIndex : DWORD ) -> DWORD;
}