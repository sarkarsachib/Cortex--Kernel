//! HAL error types and handling

/// Core error type for all HAL operations
#[derive(Debug, Clone, Copy)]
pub enum HalError {
    UnsupportedFeature,
    InitializationFailed,
    ResourceUnavailable,
    InvalidParameter,
    Timeout,
    PermissionDenied,
    DeviceNotFound,
    MemoryAllocationFailed,
    InvalidAddress,
    AlignmentError,
}

impl HalError {
    pub fn is_critical(&self) -> bool {
        matches!(self, 
            HalError::InitializationFailed |
            HalError::MemoryAllocationFailed |
            HalError::InvalidAddress)
    }
}

impl core::fmt::Display for HalError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HalError::UnsupportedFeature => write!(f, "Feature not supported on this platform"),
            HalError::InitializationFailed => write!(f, "Hardware initialization failed"),
            HalError::ResourceUnavailable => write!(f, "Required hardware resource not available"),
            HalError::InvalidParameter => write!(f, "Invalid parameter passed to HAL function"),
            HalError::Timeout => write!(f, "Operation timed out"),
            HalError::PermissionDenied => write!(f, "Permission denied for hardware operation"),
            HalError::DeviceNotFound => write!(f, "Requested device not found"),
            HalError::MemoryAllocationFailed => write!(f, "Memory allocation failed"),
            HalError::InvalidAddress => write!(f, "Invalid memory address"),
            HalError::AlignmentError => write!(f, "Memory access alignment error"),
        }
    }
}

/// Result type alias for HAL operations
pub type HalResult<T> = Result<T, HalError>;