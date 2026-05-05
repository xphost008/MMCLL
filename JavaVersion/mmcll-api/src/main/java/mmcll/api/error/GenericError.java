package mmcll.api.error;

import jakarta.validation.constraints.NotBlank;
import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

/**
 * Fallback error type.
 */
@PublicApi
public record GenericError(
        int code,
        @NonNull @NotBlank String message,
        Throwable cause
) implements MmcllError {

    public GenericError(int code, String message) {
        this(code, message, null);
    }

    public GenericError {
        if (message.isBlank()) {
            throw new IllegalArgumentException("message must not be blank");
        }
    }

}
