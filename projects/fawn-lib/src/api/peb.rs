
#![allow(warnings)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
use std::arch::asm;
use std::os::raw::c_void;
//import this type
use super::super::types::*;

#[repr(C)]
pub struct UnicodeString {
    length: USHORT,
    MaximumLength: USHORT,
    Buffer: USHORT_PTR, // PWSTR is a pointer to a wide string, which is represented as *mut u16 in Rust
}
#[repr(C)]
pub struct LIST_ENTRY {
    Flink: *mut LIST_ENTRY,
    Blink: *mut LIST_ENTRY,
}
#[repr(C)]
pub struct RTL_DRIVE_LETTER_CURDIR {
    Flags: USHORT,
    Length: USHORT,
    TimeStamp: ULONG,
    DosPath: UnicodeString,
}
#[repr(C)]
pub struct RTL_USER_PROCESS_PARAMETERS {
    MaximumLength: ULONG,
    Length: ULONG,
    Flags: ULONG,
    DebugFlags: ULONG,
    ConsoleHandle: PVOID,
    ConsoleFlags: ULONG,
    StdInputHandle: HANDLE,
    StdOutputHandle: HANDLE,
    StdErrorHandle: HANDLE,
    CurrentDirectoryPath: UnicodeString,
    CurrentDirectoryHandle: HANDLE,
    DllPath: UnicodeString,
    ImagePathName: UnicodeString,
    CommandLine: UnicodeString,
    Environment: PVOID,
    StartingPositionLeft: ULONG,
    StartingPositionTop: ULONG,
    Width: ULONG,
    Height: ULONG,
    CharWidth: ULONG,
    CharHeight: ULONG,
    ConsoleTextAttributes: ULONG,
    WindowFlags: ULONG,
    ShowWindowFlags: ULONG,
    WindowTitle: UnicodeString,
    DesktopName: UnicodeString,
    ShellInfo: UnicodeString,
    RuntimeData: UnicodeString,
    DLCurrentDirectory: [RTL_DRIVE_LETTER_CURDIR; 0x20],
}
#[repr(C)]
pub struct PEB_LDR_DATA {
    Length: ULONG,
    Initialized: BOOLEAN,
    SsHandle: PVOID,
    InLoadOrderModuleList: LIST_ENTRY,
    InMemoryOrderModuleList: LIST_ENTRY,
    InInitializationOrderModuleList: LIST_ENTRY,
}
#[repr(C)]
pub struct PEB_FREE_BLOCK {
    Next: *mut PEB_FREE_BLOCK,
    Size: ULONG,
}
#[repr(C)]
pub struct PEB {
    InheritedAddressSpace: bool,
    ReadImageFileExecOptions: bool,
    BeingDebugged: bool,
    Spare: bool,
    Mutant: PVOID,
    ImageBaseAddress: PVOID,
    LoaderData: *mut PEB_LDR_DATA_PTR,
    ProcessParameters: RTL_USER_PROCESS_PARAMETERS_PTR,
    SubSystemData: PVOID,
    ProcessHeap: PVOID,
    FastPebLock: PVOID,
    FastPebLockRoutine: *mut PEBLOCKROUTINE_PTR,
    FastPebUnlockRoutine: *mut PEBLOCKROUTINE_PTR,
    EnvironmentUpdateCount: ULONG,
    KernelCallbackTable: PVOID_PTR,
    EventLogSection: PVOID,
    EventLog: PVOID,
    FreeList: *mut PEB_FREE_BLOCK_PTR,
    TlsExpansionCounter: ULONG,
    TlsBitmap: PVOID,
    TlsBitmapBits: [ULONG; 2],
    ReadOnlySharedMemoryBase: PVOID,
    ReadOnlySharedMemoryHeap: PVOID,
    ReadOnlyStaticServerData: PVOID,
    AnsiCodePageData: PVOID,
    OemCodePageData: PVOID,
    UnicodeCaseTableData: PVOID,
    NumberOfProcessors: ULONG,
    NtGlobalFlag: ULONG,
    Spare2: [u8; 4],
    CriticalSectionTimeout: i64,
    HeapSegmentReserve: ULONG,
    HeapSegmentCommit: ULONG,
    HeapDeCommitTotalFreeThreshold: ULONG,
    HeapDeCommitFreeBlockThreshold: ULONG,
    NumberOfHeaps: ULONG,
    MaximumNumberOfHeaps: ULONG,
    ProcessHeaps: PVOID_PTR,
    GdiSharedHandleTable: PVOID,
    ProcessStarterHelper: PVOID,
    GdiDCAttributeList: PVOID,
    LoaderLock: PVOID,
    OSMajorVersion: ULONG,
    OSMinorVersion: ULONG,
    OSBuildNumber: ULONG,
    OSPlatformId: ULONG,
    ImageSubSystem: ULONG,
    ImageSubSystemMajorVersion: ULONG,
    ImageSubSystemMinorVersion: ULONG,
    GdiHandleBuffer: [ULONG; 34],
    PostProcessInitRoutine: ULONG,
    TlsExpansionBitmap: ULONG,
    TlsExpansionBitmapBits: [u8; 128],
    SessionId: ULONG,
}

//_PTR is pointer to the struct
pub type PEB_PTR = *mut PEB;
pub type RTL_DRIVE_LETTER_CURDIR_PTR = *mut RTL_DRIVE_LETTER_CURDIR;
pub type RTL_USER_PROCESS_PARAMETERS_PTR = *mut RTL_USER_PROCESS_PARAMETERS;
pub type PUNICODE_STRING = *mut UnicodeString;
pub type LIST_ENTRY_PTR = *mut LIST_ENTRY;
pub type RLIST_ENTRY_PTR = *mut LIST_ENTRY;
pub type PEB_LDR_DATA_PTR = *mut PEB_LDR_DATA;
pub type PEB_FREE_BLOCK_PTR = *mut PEB_FREE_BLOCK;
pub type PEBLOCKROUTINE_PTR = unsafe fn(*mut c_void);


//this Code crash ON linux
macro_rules! PEB_READ_ASM {
    ($peb:expr) => {
        unsafe {
            asm!("mov rax, qword ptr gs:[0x60]", out("rax") $peb);
        }
    };
}
pub fn InheritedAddressSpace() -> bool {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).InheritedAddressSpace }
}

pub fn ReadImageFileExecOptions() -> bool {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ReadImageFileExecOptions }
}
pub fn BeingDebugged() -> bool {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).BeingDebugged }
}
pub fn Spare() -> bool {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).Spare }
}
pub fn Mutant() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).Mutant }
}
pub fn ImageBaseAddress() -> usize {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ImageBaseAddress as usize }
}
pub fn LoaderData() -> *mut PEB_LDR_DATA_PTR {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).LoaderData }
}
pub fn ProcessParameters() -> RTL_USER_PROCESS_PARAMETERS_PTR {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ProcessParameters }
}
pub fn SubSystemData() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).SubSystemData }
}
pub fn ProcessHeap() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ProcessHeap }
}
pub fn FastPebLock()-> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).FastPebLock }
}
pub fn FastPebLockRoutine() -> *mut PEBLOCKROUTINE_PTR{
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).FastPebLockRoutine }

}
pub fn FastPebUnlockRoutine() -> *mut PEBLOCKROUTINE_PTR {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).FastPebUnlockRoutine }
}
pub fn EnvironmentUpdateCount() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).EnvironmentUpdateCount }
}
pub fn KernelCallbackTable() -> PVOID_PTR {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).KernelCallbackTable }
}
pub fn EventLogSection() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).EventLogSection }
}
pub fn EventLog() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).EventLog }
}
pub fn FreeList() -> *mut PEB_FREE_BLOCK_PTR {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).FreeList }
}
pub fn TlsExpansionCounter() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).TlsExpansionCounter }
}
pub fn TlsBitmap() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).TlsBitmap }
}
pub fn TlsBitmapBits() -> [u32; 2] {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).TlsBitmapBits }
}
pub fn ReadOnlySharedMemoryBase() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ReadOnlySharedMemoryBase }
}
pub fn ReadOnlySharedMemoryHeap() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ReadOnlySharedMemoryHeap }
}
pub fn ReadOnlyStaticServerData() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ReadOnlyStaticServerData }
}
pub fn AnsiCodePageData() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).AnsiCodePageData }
}
pub fn OemCodePageData() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).OemCodePageData }
}
pub fn UnicodeCaseTableData() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).UnicodeCaseTableData }
}
pub fn NumberOfProcessors() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).NumberOfProcessors }
}
pub fn NtGlobalFlag() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).NtGlobalFlag }
}
pub fn Spare2() -> [u8; 4] {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).Spare2 }
}
pub fn CriticalSectionTimeout() -> i64 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).CriticalSectionTimeout }
}
pub fn HeapSegmentReserve() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).HeapSegmentReserve }
}
pub fn HeapSegmentCommit() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).HeapSegmentCommit }
}
pub fn HeapDeCommitTotalFreeThreshold() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).HeapDeCommitTotalFreeThreshold }
}
pub fn HeapDeCommitFreeBlockThreshold() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).HeapDeCommitFreeBlockThreshold }
}
pub fn NumberOfHeaps() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).NumberOfHeaps }
}
pub fn MaximumNumberOfHeaps() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).MaximumNumberOfHeaps }
}
pub fn ProcessHeaps() -> PVOID_PTR {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ProcessHeaps }
}
pub fn GdiSharedHandleTable() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).GdiSharedHandleTable }
}
pub fn ProcessStarterHelper() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ProcessStarterHelper }
}
pub fn GdiDCAttributeList() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).GdiDCAttributeList }
}
pub fn LoaderLock() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).LoaderLock }
}
pub fn OSMajorVersion() -> u32 {
    let mut peb: PEB_PTR;

    PEB_READ_ASM!(peb);
    unsafe { (*peb).OSMajorVersion }
}
pub fn OSMinorVersion() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).OSMinorVersion }
}
pub fn OSBuildNumber() -> u32 {
    let mut peb: PEB_PTR;
    //unsafe {
    //    asm!("mov rax, qword ptr gs:[0x60]", out("rax") peb);
    //}
    PEB_READ_ASM!(peb);
    unsafe { (*peb).OSBuildNumber }
}
pub fn OSPlatformId() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).OSPlatformId }
}
pub fn ImageSubSystem() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ImageSubSystem }
}
pub fn ImageSubSystemMajorVersion() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ImageSubSystemMajorVersion }
}
pub fn ImageSubSystemMinorVersion() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ImageSubSystemMinorVersion }
}
fn GdiHandleBuffer(){}
pub fn PostProcessInitRoutine() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).PostProcessInitRoutine }
}
pub fn TlsExpansionBitmap() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).TlsExpansionBitmap }
}
pub fn TlsExpansionBitmapBits() -> [u8; 128] {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).TlsExpansionBitmapBits }
}
pub fn SessionId() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).SessionId }
}