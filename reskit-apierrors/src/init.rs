use std::sync::RwLock;

use http_types::{StatusCode};

use crate::{PVLost, APIErrorClass, Errorspace};

lazy_static! {
    pub static ref DEFAULT_ERRORSPACE_NAME: &'static str = "";
    pub static ref BUILTIN_APP_NAME: &'static str = "";

    /**
    ERR_SUCCESS 请求成功

    Mapping:
    - google api style guide: `google.rpc.Code.OK`
    - http status code: 200 OK

    Description:
    Not an error; returned on success
    */
    pub static ref ERR_SUCCESS: APIErrorClass = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "0", 
        "Successful.", 
        StatusCode::Ok).with_pvlost(PVLost::Successful);

    /**
    ERR_UNKNOWN 未知的服务端错误，通常是服务端bug

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
    pub static ref ERR_UNKNOWN: APIErrorClass = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "1", 
        "Unexpected error.", 
        StatusCode::InternalServerError);

    /**
    ERR_INTERNAL 未知的服务端错误，通常是服务端bug

    Mapping:
    - google api style guide: `google.rpc.Code.INTERNAL`
    - http status code: 500 Internal Server Error

    Internal 服务内部错误。通常是服务端bug.
    `google.rpc.Code.INTERNAL`
    Internal errors.  This means that some invariants expected by the
    underlying system have been broken.  This error code is reserved
    for serious errors.
    */
    pub static ref ERR_INTERNAL: APIErrorClass = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "2", 
        "Failure.", 
        StatusCode::InternalServerError);

    /**
    ERR_PARAMETERS 未知的服务端错误，通常是服务端bug

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
    pub static ref ERR_PARAMETERS: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "3", 
        "Params error.", 
        StatusCode::BadRequest);

    /**
    ERR_SIGNATURE 由于缺少，无效或过期的OAuth令牌，请求未通过身份验证

    Mapping:
    - `google.rpc.Code.UNAUTHENTICATED`
    - http status code: 401 Unauthorized

    The request does not have valid authentication credentials for the
    operation.
    */
    pub static ref ERR_SIGNATURE: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "4", 
        "Signature verification failed.", 
        StatusCode::Unauthorized);

    /**
    ERR_LICENSE_EXPIRED 账号过期

    Mapping:
    - http status code: 401 Unauthorized
    */
    pub static ref ERR_LICENSE_EXPIRED: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "5", 
        "License has expired.", 
        StatusCode::Forbidden);

    /**
    ERR_NOT_IMPLEMENTED API方法未被服务端实现

    Mapping:
    - `google.rpc.Code.UNIMPLEMENTED` = 12
    - http status code: 501 Not Implemented

    The operation is not implemented or is not supported/enabled in this
    service.
    */
    pub static ref ERR_NOT_IMPLEMENTED: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "6", 
        "Not Implemented.", 
        StatusCode::NotImplemented);

    /**
    ERR_NOT_FOUND 没有找到指定的资源，或者请求被未公开的原因（例如白名单）拒绝

    Mapping:
    - `googole.rpc.Code.NOT_FOUND`: Some requested entity (e.g., file or directory) was not found.
    - http status code: 404 Not Found

    Note to server developers: if a request is denied for an entire class
    of users, such as gradual feature rollout or undocumented whitelist,
    `NOT_FOUND` may be used. If a request is denied for some users within
    a class of users, such as user-based access control, `PERMISSION_DENIED`
    must be used.
    */
    pub static ref ERR_NOT_FOUND: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "7", 
        "Not found.", 
        StatusCode::NotFound);

    /**
    ERR_MULTI_FOUND 服务端发现冲突的记录

    Mapping:
    - http status code: 500 Internal Server Error
    */
    pub static ref ERR_MULTI_FOUND: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "8", 
        "Multi-found.", 
        StatusCode::InternalServerError);

    /**
    ERR_HTTP_BODY_EMPTY Deprecated, 兼容旧服务

    Mapping:
    - http status code: 400 Bad Request 
    */
    pub static ref ERR_HTTP_BODY_EMPTY: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "9", 
        "HTTP body empty.", 
        StatusCode::BadRequest);

    /**
    ERR_XML_SYNTAX Deprecated, 兼容旧服务

    Mapping:
    - http status code: 400 Bad Request 
    */
    pub static ref ERR_XML_SYNTAX: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "10", 
        "XML format error.", 
        StatusCode::BadRequest);

    /**
    ERR_REQUEST_METHOD 请求方法不支持

    Mapping:
    - http status code: 405 Method Not Allowed
    */
    pub static ref ERR_REQUEST_METHOD: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "11", 
        "Request method not supported.", 
        StatusCode::MethodNotAllowed);

    /**
    ERR_NO_LOGIN 由于缺少，无效或过期的OAuth令牌，请求未通过身份验证

    Mapping:
    - `google.rpc.Code.UNAUTHENTICATED`
    - http status code: 401 Unauthorized

    The request does not have valid authentication credentials for the
    operation.
    */
    pub static ref ERR_NO_LOGIN: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "12", 
        "Not login.", 
        StatusCode::Unauthorized);

    /**
    ERR_PERMISSION_DENIED 客户端没有足够的权限。 发生这种情况的原因可能是OAuth令牌没有正确的作用域，客户端没有权限，或者API尚未为客户端项目启用。

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
    pub static ref ERR_PERMISSION_DENIED: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "13", 
        "Permission Denied.", 
        StatusCode::Forbidden);

    /**
    ERR_STORAGE_FULL 服务器存储已满, Deprecated

    Mapping:
    - http status code: 500 Internal Server Error
    */
    pub static ref ERR_STORAGE_FULL: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "14", 
        "Storage is full.", 
        StatusCode::InternalServerError);

    /**
    ERR_DATA_SOURCE_FAILURE 上游错误

    Mapping:
    - http status code: 500 Internal Server Error
    */
    pub static ref ERR_DATA_SOURCE_FAILURE: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "15", 
        "Data source request failure.", 
        StatusCode::InternalServerError).with_pvlost(PVLost::RemoteError);

    /**
    ERR_TOO_HIGH_RATE 资源配额不足或达到速率限制。 客户应该查找google.rpc.QuotaFailure错误详细信息以获取更多信息。

    Mapping:
    - `google.rpc.Code.RESOURCE_EXHAUSTED`
    - http status code: 429 Too Many Requests

    Some resource has been exhausted, perhaps a per-user quota, or
    perhaps the entire file system is out of space.
    */
    pub static ref ERR_TOO_HIGH_RATE: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "16", 
        "Too high rate.", 
        StatusCode::TooManyRequests);

    /**
    ERR_FAILED_PRECONDITION 请求无法在当前系统状态下执行，例如删除非空目录。

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
    pub static ref ERR_FAILED_PRECONDITION: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "20", 
        "Failed precondition, do not retry.", 
        StatusCode::BadRequest);

    /**
    ERR_OUT_OF_RANGE 客户端指定了一个无效范围。

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
    pub static ref ERR_OUT_OF_RANGE: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "21", 
        "Out of range.", 
        StatusCode::BadRequest);

    /**
    ERR_ALREADY_EXISTS 客户试图创建一个已存在的资源。

    Mapping:
    - `google.rpc.Code.ALREADY_EXISTS`
    - http status code: 409 Conflict

    The entity that a client attempted to create (e.g., file or directory)
    already exists.
    */
    pub static ref ERR_ALREADY_EXISTS: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "22", 
        "Already exists.", 
        StatusCode::Conflict);

    /**
    ERR_ABORTED 并发冲突，如读 - 修改 - 写冲突。

    Mapping:
    - `google.rpc.Code.ABORTED`
    - http status code: 409 Conflict

    The operation was aborted, typically due to a concurrency issue such as
    a sequencer check failure or transaction abort.

    See the guidelines above for deciding between `FAILED_PRECONDITION`,
    `ABORTED`, and `UNAVAILABLE`.
    */
    pub static ref ERR_ABORTED: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "23", 
        "Aborted, retry whole transaction.", 
        StatusCode::Conflict);

    /**
    ERR_CANCELLED 请求被客户端取消

    Mapping:
    - `google.rpc.Code.CANCELLED`
    - http status code: 499 Client Closed Request

    The operation was cancelled, typically by the caller.
    */
    pub static ref ERR_CANCELLED: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "24", 
        "Request cancelled by client.", 
        StatusCode::BadRequest); // FIXME: no 499!

    /**
    ERR_DEADLINE_EXCEEDED 请求超时。

    Mapping:
    - `google.rpc.Code.DEADLINE_EXCEEDED`
    - http status code: 504 Gateway Timeout

    The deadline expired before the operation could complete. For operations
    that change the state of the system, this error may be returned
    even if the operation has completed successfully.  For example, a
    successful response from a server could have been delayed long
    enough for the deadline to expire.
    */
    pub static ref ERR_DEADLINE_EXCEEDED: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "25", 
        "Timeout.", 
        StatusCode::GatewayTimeout).with_pvlost(PVLost::RemoteError);

    /**
    ERR_UNAVAILABLE 服务不可用。通常是服务端宕机。通常由网关返回

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
    pub static ref ERR_UNAVAILABLE: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "26", 
        "Service unavailable.", 
        StatusCode::ServiceUnavailable).with_pvlost(PVLost::RemoteError);

    /**
    ERR_DATA_LOSS 不可恢复的数据丢失或数据损坏。 客户端应该向用户报告错误。

    Mapping:
    - `google.rpc.Code.DATA_LOSS`
    - http status code: 500 Internal Server Error

    Unrecoverable data loss or corruption.
    */
    pub static ref ERR_DATA_LOSS: APIErrorClass  = APIErrorClass::new(
        *BUILTIN_APP_NAME, 
        "27", 
        "Data loss.", 
        StatusCode::InternalServerError);

    pub static ref BUILTIN_API_ERROR_CLASSES: Vec<&'static APIErrorClass> = vec![
        &*ERR_SUCCESS,
        &*ERR_UNKNOWN,
        &*ERR_INTERNAL,
        &*ERR_PARAMETERS,
        &*ERR_SIGNATURE,
        &*ERR_LICENSE_EXPIRED,
        &*ERR_NOT_IMPLEMENTED,
        &*ERR_NOT_FOUND,
        &*ERR_MULTI_FOUND,
        &*ERR_HTTP_BODY_EMPTY,
        &*ERR_XML_SYNTAX,
        &*ERR_REQUEST_METHOD,
        &*ERR_NO_LOGIN,
        &*ERR_PERMISSION_DENIED,
        &*ERR_STORAGE_FULL,
        &*ERR_DATA_SOURCE_FAILURE,
        &*ERR_TOO_HIGH_RATE,
        &*ERR_FAILED_PRECONDITION,
        &*ERR_OUT_OF_RANGE,
        &*ERR_ALREADY_EXISTS,
        &*ERR_ABORTED,
        &*ERR_CANCELLED,
        &*ERR_DEADLINE_EXCEEDED,
        &*ERR_UNAVAILABLE,
        &*ERR_DATA_LOSS,
    ];

    pub static ref DEFAULT_ERRORSPACE: RwLock<Errorspace<'static>> = RwLock::new(Errorspace::new());
}

#[cfg(test)]
mod test {
    use http_types::{StatusCode};
    use crate::{APIErrorMeta, BUILTIN_API_ERROR_CLASSES, DEFAULT_ERRORSPACE};
    #[test]
    fn test_init() {
        let code = BUILTIN_API_ERROR_CLASSES.get(0).unwrap().code();
        assert_eq!(code, "0");
        let space = DEFAULT_ERRORSPACE.read().unwrap();
        let err = space.get_api_error_class("", "2").unwrap();
        assert_eq!(err.status_code(), StatusCode::InternalServerError);
        assert_eq!(err.code(), "2");
        assert_eq!(err.system(), "");
        assert_eq!(err.message(), "Failure.");
    }
}
