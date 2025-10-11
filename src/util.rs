#[inline(always)]
pub(crate) fn check_cross_page(ptr: *const u8, step: usize) -> bool {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        // Check if reading 'step' bytes from 'ptr' would cross a page boundary
        // Page size is typically 4096 bytes on x86_64 Linux and macOS
        const PAGE_SIZE: usize = 4096;
        ((ptr as usize & (PAGE_SIZE - 1)) + step) > PAGE_SIZE
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        // On other platforms, always use the safe path with temporary buffer
        // to avoid potential page faults
        true
    }
}
