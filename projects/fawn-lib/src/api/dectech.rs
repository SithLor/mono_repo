


pub fn has_java() -> bool {
    use std::env;
    let java_home: Result<String, env::VarError> = env::var("JAVA_HOME");
    return java_home.is_ok();
}

pub fn is_wine(mode:i32) -> bool {
    let mode: i32 = mode;
    if mode == 1 {
        use std::env;
        return env::var("WINELOADERNOEXEC").is_ok();
    } else {

        use windows::Win32::System::LibraryLoader::GetModuleHandleW;
        use windows::Win32::System::LibraryLoader::GetProcAddress;
        use windows::Win32::Foundation::HMODULE;
        use windows::core::Error;
        use windows::core::w;
        use windows::core::s;
        // Find The module handle of kernel32.dll
        let e: Result<HMODULE, Error> = unsafe {
            GetModuleHandleW(w!("kernel32.dll"))
        }; 
        // Check if the module handle is valid
        let h: HMODULE = e.unwrap();

        let wine_get_unix_file_name = unsafe {
            GetProcAddress(h, s!("wine_get_unix_file_name"))
        };
        return wine_get_unix_file_name.is_some();
    }
}
//constexpr u32 intel_ecx = 0x6c65746e;
//constexpr u32 amd_ecx = 0x69746e65;
pub fn is_amd() -> bool {
    const __MD_ECX: u32 = 0x69746e65;

    let cpuid: raw_cpuid::CpuId<raw_cpuid::CpuIdReaderNative> = raw_cpuid::CpuId::new();
    if let Some(vendor) = cpuid.get_vendor_info() {
        return vendor.as_str() == "AuthenticAMD";
    }

    false
}
pub fn is_intel() -> bool {
    const __INTEL_ECX: u32 = 0x6c65746e;

    let cpuid: raw_cpuid::CpuId<raw_cpuid::CpuIdReaderNative> = raw_cpuid::CpuId::new();
    if let Some(vendor) = cpuid.get_vendor_info() {
        return vendor.as_str() == "GenuineIntel";
    }

    false
}
pub fn is_virtual_env() -> bool {
    let cpuid = raw_cpuid::CpuId::new();
    if let Some(vendor) = cpuid.get_vendor_info() {
        let vendor_id = vendor.as_str();
        match vendor_id {
            "TCGTCGTCGTCG" | " KVMKVMKVM  " | "VMwareVMware" | "VBoxVBoxVBox" |
            "XenVMMXenVMM" | "Microsoft Hv" | " prl hyperv " | " lrpepyh vr " |
            "bhyve bhyve " | " QNXQVMBSQG " => return true,
            _ => return false,
        }
    }

    false
}
