use core::fmt::Display;

use crate::bindings::{self};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawTwzError(bindings::twz_error);

// TODO: From category, to/from errorkind, etc

impl RawTwzError {
    pub fn category(&self) -> ErrorCategory {
        let cat = (self.0 & bindings::ERROR_CATEGORY_MASK) >> bindings::ERROR_CATEGORY_SHIFT;
        match cat as u16 {
            bindings::GENERIC_ERROR => ErrorCategory::Generic,
            bindings::ARGUMENT_ERROR => ErrorCategory::Argument,
            bindings::RESOURCE_ERROR => ErrorCategory::Resource,
            bindings::NAMING_ERROR => ErrorCategory::Naming,
            bindings::OBJECT_ERROR => ErrorCategory::Object,
            bindings::IO_ERROR => ErrorCategory::Io,
            _ => ErrorCategory::Uncategorized,
        }
    }

    pub fn code(&self) -> u16 {
        ((self.0 & bindings::ERROR_CODE_MASK) >> bindings::ERROR_CODE_SHIFT) as u16
    }

    pub fn from_parts(cat: u16, code: u16) -> Self {
        let cat = ((cat as u64) << bindings::ERROR_CATEGORY_SHIFT) & bindings::ERROR_CATEGORY_MASK;
        let code = ((code as u64) << bindings::ERROR_CODE_SHIFT) & bindings::ERROR_CODE_MASK;
        Self(cat | code)
    }

    pub fn error(&self) -> TwzError {
        match self.category() {
            ErrorCategory::Uncategorized => TwzError::Uncategorized(self.code()),
            ErrorCategory::Generic => GenericError::twz_error_from_code(self.code()),
            ErrorCategory::Argument => ArgumentError::twz_error_from_code(self.code()),
            ErrorCategory::Resource => ResourceError::twz_error_from_code(self.code()),
            ErrorCategory::Naming => NamingError::twz_error_from_code(self.code()),
            ErrorCategory::Object => ObjectError::twz_error_from_code(self.code()),
            ErrorCategory::Io => IoError::twz_error_from_code(self.code()),
        }
    }

    pub fn new(raw: bindings::twz_error) -> Self {
        Self(raw)
    }

    pub fn is_success(&self) -> bool {
        self.code() == bindings::SUCCESS
    }

    pub fn raw(&self) -> bindings::twz_error {
        self.0
    }

    pub fn success() -> Self {
        Self(bindings::SUCCESS as u64)
    }

    pub fn result(&self) -> Result<(), TwzError> {
        if self.is_success() {
            Ok(())
        } else {
            Err(self.error())
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TwzError {
    Uncategorized(u16),
    Generic(GenericError),
    Argument(ArgumentError),
    Resource(ResourceError),
    Object(ObjectError),
    Naming(NamingError),
    Io(IoError),
}

impl TwzError {
    pub const NOT_SUPPORTED: Self = Self::Generic(GenericError::NotSupported);
    pub const TIMED_OUT: Self = Self::Generic(GenericError::TimedOut);
    pub const WOULD_BLOCK: Self = Self::Generic(GenericError::WouldBlock);
    pub const INVALID_ARGUMENT: Self = Self::Argument(ArgumentError::InvalidArgument);

    pub fn category(&self) -> ErrorCategory {
        match self {
            TwzError::Uncategorized(_) => ErrorCategory::Uncategorized,
            TwzError::Generic(_) => ErrorCategory::Generic,
            TwzError::Argument(_) => ErrorCategory::Argument,
            TwzError::Resource(_) => ErrorCategory::Resource,
            TwzError::Object(_) => ErrorCategory::Object,
            TwzError::Io(_) => ErrorCategory::Io,
            TwzError::Naming(_) => ErrorCategory::Naming,
        }
    }

    pub fn raw(&self) -> bindings::twz_error {
        let cat = self.category().raw();
        let code = self.code();
        RawTwzError::from_parts(cat, code).raw()
    }

    pub fn code(&self) -> u16 {
        match self {
            TwzError::Uncategorized(code) => *code,
            TwzError::Generic(generic_error) => generic_error.code(),
            TwzError::Argument(argument_error) => argument_error.code(),
            TwzError::Resource(resource_error) => resource_error.code(),
            TwzError::Object(object_error) => object_error.code(),
            TwzError::Io(io_error) => io_error.code(),
            TwzError::Naming(naming_error) => naming_error.code(),
        }
    }
}

impl Display for TwzError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TwzError::Uncategorized(code) => write!(f, "uncategorized error: {}", code),
            TwzError::Generic(generic_error) => write!(f, "generic error: {}", generic_error),
            TwzError::Argument(argument_error) => write!(f, "argument error: {}", argument_error),
            TwzError::Resource(resource_error) => write!(f, "resource error: {}", resource_error),
            TwzError::Object(object_error) => write!(f, "object error: {}", object_error),
            TwzError::Io(io_error) => write!(f, "I/O error: {}", io_error),
            TwzError::Naming(naming_error) => write!(f, "naming error: {}", naming_error),
        }
    }
}

impl core::error::Error for TwzError {}

impl Into<u64> for TwzError {
    fn into(self) -> u64 {
        self.raw()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum ErrorCategory {
    Uncategorized = bindings::UNCATEGORIZED_ERROR,
    Generic = bindings::GENERIC_ERROR,
    Argument = bindings::ARGUMENT_ERROR,
    Resource = bindings::RESOURCE_ERROR,
    Naming = bindings::NAMING_ERROR,
    Object = bindings::OBJECT_ERROR,
    Io = bindings::IO_ERROR,
}

impl Display for ErrorCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ErrorCategory::Uncategorized => write!(f, "uncategorized"),
            ErrorCategory::Generic => write!(f, "generic"),
            ErrorCategory::Argument => write!(f, "argument"),
            ErrorCategory::Resource => write!(f, "resource"),
            ErrorCategory::Naming => write!(f, "naming"),
            ErrorCategory::Object => write!(f, "object"),
            ErrorCategory::Io => write!(f, "I/O"),
        }
    }
}

impl core::error::Error for ErrorCategory {}

impl ErrorCategory {
    pub fn raw(&self) -> u16 {
        *self as u16
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum GenericError {
    NotSupported = bindings::NOT_SUPPORTED,
    Internal = bindings::INTERNAL,
    WouldBlock = bindings::WOULD_BLOCK,
    TimedOut = bindings::TIMED_OUT,
    AccessDenied = bindings::ACCESS_DENIED,
    NoSuchOperation = bindings::NO_SUCH_OPERATION,
}

impl GenericError {
    fn twz_error_from_code(code: u16) -> TwzError {
        match code {
            bindings::NOT_SUPPORTED => TwzError::Generic(GenericError::NotSupported),
            bindings::INTERNAL => TwzError::Generic(GenericError::Internal),
            bindings::WOULD_BLOCK => TwzError::Generic(GenericError::WouldBlock),
            bindings::TIMED_OUT => TwzError::Generic(GenericError::TimedOut),
            bindings::NO_SUCH_OPERATION => TwzError::Generic(GenericError::NoSuchOperation),
            _ => TwzError::Uncategorized(code),
        }
    }

    fn code(&self) -> u16 {
        *self as u16
    }
}

impl Display for GenericError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GenericError::NotSupported => write!(f, "not supported"),
            GenericError::Internal => write!(f, "internal"),
            GenericError::WouldBlock => write!(f, "would block"),
            GenericError::TimedOut => write!(f, "timed out"),
            GenericError::AccessDenied => write!(f, "access denied"),
            GenericError::NoSuchOperation => write!(f, "no such operation"),
        }
    }
}

impl core::error::Error for GenericError {}

impl From<GenericError> for TwzError {
    fn from(value: GenericError) -> Self {
        Self::Generic(value)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum ArgumentError {
    InvalidArgument = bindings::INVALID_ARGUMENT,
    WrongType = bindings::WRONG_TYPE,
    InvalidAddress = bindings::INVALID_ADDRESS,
    BadHandle = bindings::BAD_HANDLE,
}

impl ArgumentError {
    fn twz_error_from_code(code: u16) -> TwzError {
        match code {
            bindings::INVALID_ARGUMENT => TwzError::Argument(ArgumentError::InvalidArgument),
            bindings::WRONG_TYPE => TwzError::Argument(ArgumentError::WrongType),
            bindings::INVALID_ADDRESS => TwzError::Argument(ArgumentError::InvalidAddress),
            bindings::BAD_HANDLE => TwzError::Argument(ArgumentError::BadHandle),
            _ => TwzError::Uncategorized(code),
        }
    }

    fn code(&self) -> u16 {
        *self as u16
    }
}

impl Display for ArgumentError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ArgumentError::InvalidArgument => write!(f, "invalid argument"),
            ArgumentError::WrongType => write!(f, "wrong type"),
            ArgumentError::InvalidAddress => write!(f, "invalid address"),
            ArgumentError::BadHandle => write!(f, "bad handle"),
        }
    }
}

impl core::error::Error for ArgumentError {}

impl From<ArgumentError> for TwzError {
    fn from(value: ArgumentError) -> Self {
        Self::Argument(value)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum ResourceError {
    OutOfMemory = bindings::OUT_OF_MEMORY,
    OutOfResources = bindings::OUT_OF_RESOURCES,
    OutOfNames = bindings::OUT_OF_NAMES,
    Unavailable = bindings::UNAVAILABLE,
}

impl ResourceError {
    fn twz_error_from_code(code: u16) -> TwzError {
        match code {
            bindings::OUT_OF_MEMORY => TwzError::Resource(ResourceError::OutOfMemory),
            bindings::OUT_OF_RESOURCES => TwzError::Resource(ResourceError::OutOfResources),
            bindings::OUT_OF_NAMES => TwzError::Resource(ResourceError::OutOfNames),
            bindings::UNAVAILABLE => TwzError::Resource(ResourceError::Unavailable),
            _ => TwzError::Uncategorized(code),
        }
    }

    fn code(&self) -> u16 {
        *self as u16
    }
}

impl Display for ResourceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ResourceError::OutOfMemory => write!(f, "out of memory"),
            ResourceError::OutOfResources => write!(f, "out of resources"),
            ResourceError::OutOfNames => write!(f, "out of names"),
            ResourceError::Unavailable => write!(f, "unavailable"),
        }
    }
}

impl core::error::Error for ResourceError {}

impl From<ResourceError> for TwzError {
    fn from(value: ResourceError) -> Self {
        Self::Resource(value)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum ObjectError {
    MapFailed = bindings::MAP_FAILED,
    NotMapped = bindings::NOT_MAPPED,
    InvalidFote = bindings::INVALID_FOTE,
    InvalidPtr = bindings::INVALID_PTR,
    InvalidMeta = bindings::INVALID_META,
    BaseTypeMismatch = bindings::BASETYPE_MISMATCH,
    NoSuchObject = bindings::NO_SUCH_OBJECT,
}

impl ObjectError {
    fn twz_error_from_code(code: u16) -> TwzError {
        match code {
            bindings::MAP_FAILED => TwzError::Object(ObjectError::MapFailed),
            bindings::NOT_MAPPED => TwzError::Object(ObjectError::NotMapped),
            bindings::INVALID_FOTE => TwzError::Object(ObjectError::InvalidFote),
            bindings::INVALID_PTR => TwzError::Object(ObjectError::InvalidPtr),
            bindings::INVALID_META => TwzError::Object(ObjectError::InvalidMeta),
            bindings::BASETYPE_MISMATCH => TwzError::Object(ObjectError::BaseTypeMismatch),
            bindings::NO_SUCH_OBJECT => TwzError::Object(ObjectError::NoSuchObject),
            _ => TwzError::Uncategorized(code),
        }
    }

    fn code(&self) -> u16 {
        *self as u16
    }
}

impl Display for ObjectError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ObjectError::MapFailed => write!(f, "mapping failed"),
            ObjectError::NotMapped => write!(f, "not mapped"),
            ObjectError::InvalidFote => write!(f, "invalid FOT entry"),
            ObjectError::InvalidPtr => write!(f, "invalid pointer"),
            ObjectError::InvalidMeta => write!(f, "invalid metadata"),
            ObjectError::BaseTypeMismatch => write!(f, "base type mismatch"),
            ObjectError::NoSuchObject => write!(f, "no such object"),
        }
    }
}

impl core::error::Error for ObjectError {}

impl From<ObjectError> for TwzError {
    fn from(value: ObjectError) -> Self {
        Self::Object(value)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum IoError {
    Other = bindings::OTHER_IO_ERROR,
    DataLoss = bindings::DATA_LOSS,
    DeviceError = bindings::DEVICE_ERROR,
    SeekFailed = bindings::SEEK_FAILED,
}

impl IoError {
    fn twz_error_from_code(code: u16) -> TwzError {
        match code {
            bindings::OTHER_IO_ERROR => TwzError::Io(IoError::Other),
            bindings::DATA_LOSS => TwzError::Io(IoError::DataLoss),
            bindings::DEVICE_ERROR => TwzError::Io(IoError::DeviceError),
            bindings::SEEK_FAILED => TwzError::Io(IoError::SeekFailed),
            _ => TwzError::Uncategorized(code),
        }
    }

    fn code(&self) -> u16 {
        *self as u16
    }
}

impl Display for IoError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IoError::Other => write!(f, "other I/O error"),
            IoError::DataLoss => write!(f, "data loss"),
            IoError::DeviceError => write!(f, "device error"),
            IoError::SeekFailed => write!(f, "seek failed"),
        }
    }
}

impl core::error::Error for IoError {}

impl From<IoError> for TwzError {
    fn from(value: IoError) -> Self {
        Self::Io(value)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum NamingError {
    NotFound = bindings::NOT_FOUND,
    AlreadyExists = bindings::ALREADY_EXISTS,
    WrongNameKind = bindings::WRONG_NAME_KIND,
    AlreadyBound = bindings::ALREADY_BOUND,
    LinkLoop = bindings::LINK_LOOP,
    NotEmpty = bindings::NOT_EMPTY,
}

impl NamingError {
    fn twz_error_from_code(code: u16) -> TwzError {
        match code {
            bindings::NOT_FOUND => TwzError::Naming(NamingError::NotFound),
            bindings::ALREADY_EXISTS => TwzError::Naming(NamingError::AlreadyExists),
            bindings::WRONG_NAME_KIND => TwzError::Naming(NamingError::WrongNameKind),
            bindings::ALREADY_BOUND => TwzError::Naming(NamingError::AlreadyBound),
            bindings::LINK_LOOP => TwzError::Naming(NamingError::LinkLoop),
            bindings::NOT_EMPTY => TwzError::Naming(NamingError::NotEmpty),
            _ => TwzError::Uncategorized(code),
        }
    }

    fn code(&self) -> u16 {
        *self as u16
    }
}

impl Display for NamingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NamingError::NotFound => write!(f, "not found"),
            NamingError::AlreadyExists => write!(f, "already exists"),
            NamingError::WrongNameKind => write!(f, "wrong name kind"),
            NamingError::AlreadyBound => write!(f, "already bound"),
            NamingError::LinkLoop => write!(f, "link loop"),
            NamingError::NotEmpty => write!(f, "not empty"),
        }
    }
}

impl core::error::Error for NamingError {}

impl From<NamingError> for TwzError {
    fn from(value: NamingError) -> Self {
        Self::Naming(value)
    }
}

impl From<core::alloc::AllocError> for TwzError {
    fn from(_value: core::alloc::AllocError) -> Self {
        ResourceError::OutOfMemory.into()
    }
}
