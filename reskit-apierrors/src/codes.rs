

#[derive(Debug)]
pub enum BuiltinAPIErrorMeta {
    /// Success 请求成功
    /// 
    /// Mapping:
    /// - google api style guide: `google.rpc.Code.OK`
    /// - http status code: 200 OK
    ///
    /// Description:
    /// Not an error; returned on success
    Success = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "0", 
        "Successful.", 
        StatusCode::Ok).with_pvlost(PVLost::Successful),
    
    /// Unknown 未知的服务端错误，通常是服务端bug
    /// 
    /// Mapping:
    /// - google api style guide: `google.rpc.Code.UNKNOWN`
    /// - http status code: 500 Internal Server Error
    /// 
    /// Description:
    /// Unknown error.  For example, this error may be returned when
    /// a `Status` value received from another address space belongs to
    /// an error space that is not known in this address space.  Also
    /// errors raised by APIs that do not return enough error information
    /// may be converted to this error.
    Unknown = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "1", 
        "Unexpected error.", 
        StatusCode::InternalServerError),

    /// Internal 未知的服务端错误，通常是服务端bug
    /// 
    /// Mapping:
    /// - google api style guide: `google.rpc.Code.INTERNAL`
    /// - http status code: 500 Internal Server Error
    ///
    /// Internal 服务内部错误。通常是服务端bug.
    /// `google.rpc.Code.INTERNAL`
    /// Internal errors.  This means that some invariants expected by the
    /// underlying system have been broken.  This error code is reserved
    /// for serious errors.
    Internal = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "2", 
        "Failure.", 
        StatusCode::InternalServerError),

    /// Parameters 未知的服务端错误，通常是服务端bug
    /// 
    /// Mapping:
    /// - google api style guide: `google.rpc.Code.INVALID_ARGUMENT`
    /// - http status code: 400 Bad Request
    ///
    /// Parameters 客户端指定了无效的参数。 查看错误消息和错误详情以获取更多信息
    /// `google.rpc.Code.INVALID_ARGUMENT`
    /// The client specified an invalid argument.  Note that this differs
    /// from `FAILED_PRECONDITION`.  `INVALID_ARGUMENT` indicates arguments
    /// that are problematic regardless of the state of the system
    /// (e.g., a malformed file name).
    Parameters = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "3", 
        "Params error.", 
        StatusCode::StatusBadRequest),

    /// Signature 由于缺少，无效或过期的OAuth令牌，请求未通过身份验证
    ///
    /// Mapping:
    /// - `google.rpc.Code.UNAUTHENTICATED`
    /// - http status code: 401 Unauthorized
    ///
    /// The request does not have valid authentication credentials for the
    /// operation.
    Signature = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "4", 
        "Signature verification failed.", 
        StatusCode::StatusUnauthorized),

    /// LicenseExpired 账号过期
    ///
    /// Mapping:
    /// - http status code: 401 Unauthorized
    LicenseExpired = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "5", 
        "License has expired.", 
        StatusCode::StatusForbidden),

    /// NotImplemented API方法未被服务端实现
    ///
    /// Mapping:
    /// - `google.rpc.Code.UNIMPLEMENTED` = 12
    /// - http status code: 501 Not Implemented
    ///
    /// The operation is not implemented or is not supported/enabled in this
    /// service.
    NotImplemented = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "6", 
        "Not Implemented.", 
        StatusCode::StatusNotImplemented),

    /// NotFound 没有找到指定的资源，或者请求被未公开的原因（例如白名单）拒绝
    ///
    /// Mapping:
    /// - `googole.rpc.Code.NOT_FOUND`: Some requested entity (e.g., file or directory) was not found.
    /// - http status code: 404 Not Found
    ///
    /// Note to server developers: if a request is denied for an entire class
    /// of users, such as gradual feature rollout or undocumented whitelist,
    /// `NOT_FOUND` may be used. If a request is denied for some users within
    /// a class of users, such as user-based access control, `PERMISSION_DENIED`
    /// must be used.
    NotFound = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "7", 
        "Not found.", 
        StatusCode::StatusNotFound),

    /// MultiFound 服务端发现冲突的记录
    ///
    /// HTTP Mapping: 500 Internal Server Error
    MultiFound = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "8", 
        "Multi-found.", 
        StatusCode::StatusInternalServerError),

    /// HTTPBodyEmpty Deprecated, 兼容旧服务
    ///
    /// HTTP Mapping: 400 Bad Request
    HTTPBodyEmpty = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "9", 
        "HTTP body empty.", 
        StatusCode::StatusBadRequest),

    /// XMLSyntax Deprecated, 兼容旧服务
    ///
    /// HTTP Mapping: 400 Bad Request
    ErrXMLSyntax = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "10", 
        "XML format error.", 
        StatusCode::StatusBadRequest),

    /// RequestMethod 请求方法不支持
    ///
    /// HTTP Mapping: 405 Method Not Allowed
    RequestMethod = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "11", 
        "Request method not supported.", 
        StatusCode::StatusMethodNotAllowed),

    /// NoLogin 由于缺少，无效或过期的OAuth令牌，请求未通过身份验证
    /// 
    /// Google API Style Code Mapping: `google.rpc.Code.UNAUTHENTICATED`
    /// The request does not have valid authentication credentials for the
    /// operation.
    ///
    /// HTTP Mapping: 401 Unauthorized
    NoLogin = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "12", 
        "Not login.", 
        StatusCode::StatusUnauthorized),

    /// PermissionDenied 客户端没有足够的权限。 发生这种情况的原因可能是OAuth令牌没有正确的作用域，客户端没有权限，或者API尚未为客户端项目启用。
    /// 
    /// Google API Style Code Mapping: `google.rpc.Code.PERMISSION_DENIED`
    /// The caller does not have permission to execute the specified
    /// operation. `PERMISSION_DENIED` must not be used for rejections
    /// caused by exhausting some resource (use `RESOURCE_EXHAUSTED`
    /// instead for those errors). `PERMISSION_DENIED` must not be
    /// used if the caller can not be identified (use `UNAUTHENTICATED`
    /// instead for those errors). This error code does not imply the
    /// request is valid or the requested entity exists or satisfies
    /// other pre-conditions.
    ///
    /// HTTP Mapping: 403 Forbidden
    PermissionDenied = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "13", 
        "Permission Denied.", 
        StatusCode::StatusForbidden),

    /// StorageFull 服务器存储已满, Deprecated
    ///
    /// HTTP Mapping: 500 Internal Server Error
    StorageFull = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "14", 
        "Storage is full.", 
        StatusCode::StatusInternalServerError),

    /// DataSourceFailure 上游错误
    ///
    /// HTTP Mapping: 500 Internal Server Error
    DataSourceFailure = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "15", 
        "Data source request failure.", 
        StatusCode::StatusInternalServerError).with_pvlost(PVLost::LocalError),

    /// TooHighRate 资源配额不足或达到速率限制。 客户应该查找google.rpc.QuotaFailure错误详细信息以获取更多信息。
    ///
    /// Google API Style Code Mapping: `google.rpc.Code.RESOURCE_EXHAUSTED`
    /// Some resource has been exhausted, perhaps a per-user quota, or
    /// perhaps the entire file system is out of space.
    ///
    /// HTTP Mapping: 429 Too Many Requests
    TooHighRate = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "16", 
        "Too high rate.", 
        StatusCode::StatusTooManyRequests),

    /// FailedPrecondition 请求无法在当前系统状态下执行，例如删除非空目录。
    ///
    /// Google API Style Code Mapping: `google.rpc.Code.FAILED_PRECONDITION`
    /// The operation was rejected because the system is not in a state
    /// required for the operation's execution.  For example, the directory
    /// to be deleted is non-empty, an rmdir operation is applied to
    /// a non-directory, etc.
    ///
    /// Service implementors can use the following guidelines to decide
    /// between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`:
    ///  (a) Use `UNAVAILABLE` if the client can retry just the failing call.
    ///  (b) Use `ABORTED` if the client should retry at a higher level
    ///      (e.g., when a client-specified test-and-set fails, indicating the
    ///      client should restart a read-modify-write sequence).
    ///  (c) Use `FAILED_PRECONDITION` if the client should not retry until
    ///      the system state has been explicitly fixed.  E.g., if an "rmdir"
    ///      fails because the directory is non-empty, `FAILED_PRECONDITION`
    ///      should be returned since the client should not retry unless
    ///      the files are deleted from the directory.
    ///
    /// HTTP Mapping: 400 Bad Request
    FailedPrecondition = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "20", 
        "Failed precondition, do not retry.", 
        StatusCode::StatusBadRequest),

    /// OutOfRange 客户端指定了一个无效范围。
    ///
    /// Google API Style Code Mapping: `google.rpc.Code.OUT_OF_RANGE`
    /// The operation was attempted past the valid range.  E.g., seeking or
    /// reading past end-of-file.
    ///
    /// Unlike `INVALID_ARGUMENT`, this error indicates a problem that may
    /// be fixed if the system state changes. For example, a 32-bit file
    /// system will generate `INVALID_ARGUMENT` if asked to read at an
    /// offset that is not in the range [0,2^32-1], but it will generate
    /// `OUT_OF_RANGE` if asked to read from an offset past the current
    /// file size.
    ///
    /// There is a fair bit of overlap between `FAILED_PRECONDITION` and
    /// `OUT_OF_RANGE`.  We recommend using `OUT_OF_RANGE` (the more specific
    /// error) when it applies so that callers who are iterating through
    /// a space can easily look for an `OUT_OF_RANGE` error to detect when
    /// they are done.
    ///
    /// HTTP Mapping: 400 Bad Request
    OutOfRange = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "21", 
        "Out of range.", 
        StatusCode::StatusBadRequest),

    /// AlreadyExists 客户试图创建一个已存在的资源。
    ///
    /// Google API Style Code Mapping: `google.rpc.Code.ALREADY_EXISTS`
    /// The entity that a client attempted to create (e.g., file or directory)
    /// already exists.
    ///
    /// HTTP Mapping: 409 Conflict
    ErrAlreadyExists = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "22", 
        "Already exists.", 
        StatusCode::StatusConflict),

    /// Aborted 并发冲突，如读 - 修改 - 写冲突。
    ///
    /// Google API Style Code Mapping: `google.rpc.Code.ABORTED`
    /// The operation was aborted, typically due to a concurrency issue such as
    /// a sequencer check failure or transaction abort.
    ///
    /// See the guidelines above for deciding between `FAILED_PRECONDITION`,
    /// `ABORTED`, and `UNAVAILABLE`.
    ///
    /// HTTP Mapping: 409 Conflict
    Aborted = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "23", 
        "Aborted, retry whole transaction.", 
        StatusCode::StatusConflict),

    /// Cancelled 请求被客户端取消
    ///
    /// Google API Style Code Mapping: `google.rpc.Code.CANCELLED`
    /// The operation was cancelled, typically by the caller.
    ///
    /// HTTP Mapping: 499 Client Closed Request
    Cancelled = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "24", 
        "Request cancelled by client.", 
        StatusCode::StatusBadRequest), // FIXME: no 499!

    /// DeadlineExceeded 请求超时。
    ///
    /// Google API Style Code Mapping: `google.rpc.Code.DEADLINE_EXCEEDED`
    /// The deadline expired before the operation could complete. For operations
    /// that change the state of the system, this error may be returned
    /// even if the operation has completed successfully.  For example, a
    /// successful response from a server could have been delayed long
    /// enough for the deadline to expire.
    ///
    /// HTTP Mapping: 504 Gateway Timeout
    DeadlineExceeded = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "25", 
        "Timeout.", 
        StatusCode::StatusGatewayTimeout).with_pvlost(1),

    /// Unavailable 服务不可用。通常是服务端宕机。通常由网关返回
    ///
    ///	Google API Style Code Mapping: `google.rpc.Code.UNAVAILABLE`
    /// The service is currently unavailable.  This is most likely a
    /// transient condition, which can be corrected by retrying with
    /// a backoff. Note that it is not always safe to retry
    /// non-idempotent operations.
    ///
    /// See the guidelines above for deciding between `FAILED_PRECONDITION`,
    /// `ABORTED`, and `UNAVAILABLE`.
    ///
    /// HTTP Mapping: 503 Service Unavailable
    Unavailable = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "26", 
        "Service unavailable.", 
        StatusCode::StatusServiceUnavailable).with_pvlost(1),

    /// DataLoss 不可恢复的数据丢失或数据损坏。 客户端应该向用户报告错误。
    /// 
    /// Google API Style Code Mapping: `google.rpc.Code.DATA_LOSS`
    /// Unrecoverable data loss or corruption.
    ///
    /// HTTP Mapping: 500 Internal Server Error
    ErrDataLoss = NewAPIErrorClass(APPName, "27", "Data loss.", http.StatusInternalServerError)

}