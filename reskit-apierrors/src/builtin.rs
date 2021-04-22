use std::fmt::Debug;

use crate::{AsAPIErrorMeta, APIErrorMeta};

/// Builtin defines the builtin api error metas
#[derive(Clone, Copy, Debug, PartialEq, AsAPIErrorMeta)] // TODO: impl APIErrorMetaEnum derive macro！
pub enum Builtin {
    /**
    Successful 请求成功

    Mapping:
    - google api style guide: `google.rpc.Code.OK`
    - http status code: 200 OK

    Description:
    Not an error; returned on success
    */
    #[apierrormeta(system="", code="0", message="Successful.", status_code=200, pvlost=0)]
    Successful,

    /**
    Unknown 未知的服务端错误，通常是服务端bug

    Mapping:
    - google api style guide: `google.rpc.Code.UNKNOWN`
    - http status code: 500 Internal Server Error

    Description:
    Unknown error.  For example, this error may be returned when
    a `Status` value received from another address space belongs to
    an error space that is not known in this address space.  Also
    errors raised by APIs that do not return enough error information
    may be converted to this error.
    */
    #[apierrormeta(system="", code="1", message="Unknown error.", status_code=500)]
    Unknown,



    /**
    InvalidArgument 未知的服务端错误，通常是服务端bug

    Mapping:
    - google api style guide: `google.rpc.Code.INVALID_ARGUMENT`
    - http status code: 400 Bad Request

    Parameters 客户端指定了无效的参数。 查看错误消息和错误详情以获取更多信息
    `google.rpc.Code.INVALID_ARGUMENT`
    The client specified an invalid argument.  Note that this differs
    from `FAILED_PRECONDITION`.  `INVALID_ARGUMENT` indicates arguments
    that are problematic regardless of the state of the system
    (e.g., a malformed file name).
    */
    //#[apierrormeta(system="", code="3", message="Invalid argument.", status_code=400)]
    //InvalidArgument,

    /**
    Unauthorized 由于缺少，无效或过期的OAuth令牌，请求未通过身份验证

    Mapping:
    - `google.rpc.Code.UNAUTHENTICATED`
    - http status code: 401 Unauthorized

    The request does not have valid authentication credentials for the
    operation.
    */
    //#[apierrormeta(system="", code="4", message="Authentication failed.", status_code=401)]
    //Unauthorized,

    /**
    NotImplemented API方法未被服务端实现

    Mapping:
    - `google.rpc.Code.UNIMPLEMENTED` = 12
    - http status code: 501 Not Implemented

    The operation is not implemented or is not supported/enabled in this
    service.
    */
    //#[apierrormeta(system="", code="6", message="Not Implemented.", status_code=501)]
    //NotImplemented,

    /**
    NotFound 没有找到指定的资源，或者请求被未公开的原因（例如白名单）拒绝

    Mapping:
    - `googole.rpc.Code.NOT_FOUND`: Some requested entity (e.g., file or directory) was not found.
    - http status code: 404 Not Found

    Note to server developers: if a request is denied for an entire class
    of users, such as gradual feature rollout or undocumented whitelist,
    `NOT_FOUND` may be used. If a request is denied for some users within
    a class of users, such as user-based access control, `PERMISSION_DENIED`
    must be used.
    */
    //#[apierrormeta(system="", code="7", message="Not found.", status_code=404)]
    //NotFound,

    /**
    PermissionDenied 客户端没有足够的权限。 发生这种情况的原因可能是OAuth令牌没有正确的作用域，客户端没有权限，或者API尚未为客户端项目启用。

    Mapping:
    - `google.rpc.Code.PERMISSION_DENIED`
    - http status code: 403 Forbidden

    The caller does not have permission to execute the specified
    operation. `PERMISSION_DENIED` must not be used for rejections
    caused by exhausting some resource (use `RESOURCE_EXHAUSTED`
    instead for those errors). `PERMISSION_DENIED` must not be
    used if the caller can not be identified (use `UNAUTHENTICATED`
    instead for those errors). This error code does not imply the
    request is valid or the requested entity exists or satisfies
    other pre-conditions.
    */
    //#[apierrormeta(system="", code="13", message="Permission Denied.", status_code=403)]
    //PermissionDenied,


    /**
    DataSourceFailure 上游错误

    Mapping:
    - http status code: 500 Internal Server Error
    */
    //#[apierrormeta(system="", code="13", message="Data source request failure.", status_code=500, pvlost=1)]
    //DataSourceFailure,

    /**
    ResourceExhausted 资源配额不足或达到速率限制。 客户应该查找google.rpc.QuotaFailure错误详细信息以获取更多信息。

    Mapping:
    - `google.rpc.Code.RESOURCE_EXHAUSTED`
    - http status code: 429 Too Many Requests

    Some resource has been exhausted, perhaps a per-user quota, or
    perhaps the entire file system is out of space.
    */
    //#[apierrormeta(system="", code="16", message="Data source request failure.", status_code=429)]
    //ResourceExhausted,

    /**
    FailedPrecondition 请求无法在当前系统状态下执行，例如删除非空目录。

    Mapping:
    -`google.rpc.Code.FAILED_PRECONDITION`
    - http status code: 400 Bad Request

    The operation was rejected because the system is not in a state
    required for the operation's execution.  For example, the directory
    to be deleted is non-empty, an rmdir operation is applied to
    a non-directory, etc.

    Service implementors can use the following guidelines to decide
    between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`:
     (a) Use `UNAVAILABLE` if the client can retry just the failing call.
     (b) Use `ABORTED` if the client should retry at a higher level
         (e.g., when a client-specified test-and-set fails, indicating the
         client should restart a read-modify-write sequence).
     (c) Use `FAILED_PRECONDITION` if the client should not retry until
         the system state has been explicitly fixed.  E.g., if an "rmdir"
         fails because the directory is non-empty, `FAILED_PRECONDITION`
         should be returned since the client should not retry unless
         the files are deleted from the directory.
    */
    //#[apierrormeta(system="", code="20", message="Failed precondition, do not retry.", status_code=400)]
    //FailedPrecondition,

    /**
    OutOfRange 客户端指定了一个无效范围。

    Mapping:
    - `google.rpc.Code.OUT_OF_RANGE`
    - http status code: 400 Bad Request

    The operation was attempted past the valid range.  E.g., seeking or
    reading past end-of-file.

    Unlike `INVALID_ARGUMENT`, this error indicates a problem that may
    be fixed if the system state changes. For example, a 32-bit file
    system will generate `INVALID_ARGUMENT` if asked to read at an
    offset that is not in the range [0,2^32-1], but it will generate
    `OUT_OF_RANGE` if asked to read from an offset past the current
    file size.

    There is a fair bit of overlap between `FAILED_PRECONDITION` and
    `OUT_OF_RANGE`.  We recommend using `OUT_OF_RANGE` (the more specific
    error) when it applies so that callers who are iterating through
    a space can easily look for an `OUT_OF_RANGE` error to detect when
    they are done.
    */
    //#[apierrormeta(system="", code="21", message="Out of range.", status_code=400)]
    //OutOfRange,

    /**
    AlreadyExists 客户试图创建一个已存在的资源。

    Mapping:
    - `google.rpc.Code.ALREADY_EXISTS`
    - http status code: 409 Conflict

    The entity that a client attempted to create (e.g., file or directory)
    already exists.
    */
    //#[apierrormeta(system="", code="22", message="Already exists.", status_code=409)]
    //AlreadyExists,

    /**
    Aborted 并发冲突，如读 - 修改 - 写冲突。

    Mapping:
    - `google.rpc.Code.ABORTED`
    - http status code: 409 Conflict

    The operation was aborted, typically due to a concurrency issue such as
    a sequencer check failure or transaction abort.

    See the guidelines above for deciding between `FAILED_PRECONDITION`,
    `ABORTED`, and `UNAVAILABLE`.
    */
    //#[apierrormeta(system="", code="23", message="Aborted, retry whole transaction.", status_code=409)]
    //Aborted,

    /**
    Cancelled 请求被客户端取消

    Mapping:
    - `google.rpc.Code.CANCELLED`
    - http status code: 499 Client Closed Request

    The operation was cancelled, typically by the caller.
    */
    //#[apierrormeta(system="", code="24", message="Request cancelled by client.", status_code=400)] // FIXME: 499
    //Cancelled,

    /**
    DeadlineExceeded 请求超时。

    Mapping:
    - `google.rpc.Code.DEADLINE_EXCEEDED`
    - http status code: 504 Gateway Timeout

    The deadline expired before the operation could complete. For operations
    that change the state of the system, this error may be returned
    even if the operation has completed successfully.  For example, a
    successful response from a server could have been delayed long
    enough for the deadline to expire.
    */
    //#[apierrormeta(system="", code="25", message="Timeout.", status_code=504, pvlost=1)]
    //DeadlineExceeded,

    /**
    Unavailable 服务不可用。通常是服务端宕机。通常由网关返回

    Mapping:
    	- `google.rpc.Code.UNAVAILABLE`
    - http status code: 503 Service Unavailable

    The service is currently unavailable.  This is most likely a
    transient condition, which can be corrected by retrying with
    a backoff. Note that it is not always safe to retry
    non-idempotent operations.

    See the guidelines above for deciding between `FAILED_PRECONDITION`,
    `ABORTED`, and `UNAVAILABLE`.
    */
    //#[apierrormeta(system="", code="26", message="Service unavailable.", status_code=503, pvlost=1)]
    //Unavailable,

    /**
    DataLoss 不可恢复的数据丢失或数据损坏。 客户端应该向用户报告错误。

    Mapping:
    - `google.rpc.Code.DATA_LOSS`
    - http status code: 500 Internal Server Error

    Unrecoverable data loss or corruption.
    */
    //#[apierrormeta(system="", code="27", message="Data loss.", status_code=503)]
    //DataLoss,

    /**
    Internal 未知的服务端错误，通常是服务端bug

    Mapping:
    - google api style guide: `google.rpc.Code.INTERNAL`
    - http status code: 500 Internal Server Error

    Internal 服务内部错误。通常是服务端bug.
    `google.rpc.Code.INTERNAL`
    Internal errors.  This means that some invariants expected by the
    underlying system have been broken.  This error code is reserved
    for serious errors.
    */
    #[apierrormeta(system="", code="2", message="Internal server error.", status_code=500)]
    Internal,
}

// impl APIErrorMeta for Builtin { // FIXME: derive
//     fn system(&self) -> &str {
//         match self {
//             Self::Successful => "",
//             Self::Unknown => "",
//             Self::Internal => "",
//         }
//     }

//     fn code(&self) -> &str {
//         match self {
//             Self::Successful => "0",
//             Self::Unknown => "1",
//             Self::Internal => "2",
//         }
//     }

//     fn message(&self) -> &str {
//         match self {
//             Self::Successful => "Successful.",
//             Self::Unknown => "Unknown error.",
//             Self::Internal => "Failure.",
//         }
//     }

//     fn status_code(&self) -> StatusCode {
//         match self {
//             Self::Successful => StatusCode::Ok,
//             Self::Unknown => StatusCode::InternalServerError,
//             Self::Internal => StatusCode::InternalServerError,
//         }
//     }

//     #[cfg(feature = "pvlost")]
//     fn pvlost(&self) -> PVLost {
//         match self {
//             Self::Successful => PVLost::Successful,
//             //Self::DataSourceFailure => PVLost::RemoteError,
//             _ => PVLost::LocalError,
//         }
//     }
// }

// impl APIErrorMetas for Builtin { // FIXME: derive
//     fn api_error_metas() -> Vec<&'static dyn APIErrorMeta> {
//         vec![
//             &Self::Successful,
//             &Self::Unknown,
//             &Self::Internal,
//         ]
//     }
// }

// impl Display for Builtin {
//     #[cfg(not(feature = "pvlost"))]
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "{}:{}:{}:{}", self.status_code(), self.system(), self.code(), self.message())  
//     }

//     #[cfg(feature = "pvlost")]
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "{}:{}:{}:{}:{}", self.status_code(), self.system(), self.code(), self.message(), self.pvlost() as u8)
//     }
// }


#[cfg(test)]
mod tests {
    use crate::{Builtin, APIErrorMeta};

    #[test]
    fn test_meta() {
        assert_eq!(Builtin::Successful.message(), "Successful.");
        assert_eq!(Builtin::Unknown.message(), "Unknown error.");
    }

    #[test]
    fn test_lifetime() {
        fn enum_as_static(meta: &'static dyn APIErrorMeta) -> &'static dyn APIErrorMeta {
            meta
        }
        let meta = enum_as_static(&Builtin::Unknown);
        assert_eq!(meta.code(), "1");
        assert_eq!(meta.system(), "");
    }
}
