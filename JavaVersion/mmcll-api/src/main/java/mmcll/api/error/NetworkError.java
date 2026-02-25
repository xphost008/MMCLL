package mmcll.api.error;

import jakarta.validation.constraints.NotBlank;
import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

import java.net.URI;

/**
 * Network / HTTP related error.
 */
@PublicApi
public record NetworkError(
        int code,
        @NonNull @NotBlank String message,
        Throwable cause,
        URI uri,
        String method,
        Integer httpStatus
) implements MmcllError {

    public NetworkError {
        if (message.isBlank()) {
            throw new IllegalArgumentException("message must not be blank");
        }
    }

    public static @NonNull NetworkError of(
            int code,
            String message,
            URI uri,
            String method,
            Integer httpStatus,
            Throwable cause
    ) {
        return new NetworkError(code, message, cause, uri, method, httpStatus);
    }

    public static @NonNull NetworkError of(
            int code,
            String message,
            URI uri,
            String method
    ) {
        return new NetworkError(code, message, null, uri, method, null);
    }

}
