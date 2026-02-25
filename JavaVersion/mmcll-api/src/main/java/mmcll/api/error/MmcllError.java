package mmcll.api.error;

import mmcll.api.annotation.PublicApi;

/**
 * Structured error object used across MMCLL public API.
 */
@PublicApi
public sealed interface MmcllError permits NetworkError, JsonError, IntegrityError, AuthError, LaunchError, GenericError {

    /**
     * Numeric error code.
     * <p>
     * Codes are intended to be stable, but may expand over time.
     */
    int code();

    /**
     * Human readable message.
     */
    String message();

    /**
     * Optional cause.
     */
    Throwable cause();
}
