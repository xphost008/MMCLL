package mmcll.api.error;

import jakarta.validation.constraints.NotBlank;
import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

import java.nio.file.Path;

/**
 * JSON parsing/validation/merging error.
 */
@PublicApi
public record JsonError(
        int code,
        @NonNull @NotBlank String message,
        Throwable cause,
        Path source
) implements MmcllError {

    public JsonError {
        if (message.isBlank()) {
            throw new IllegalArgumentException("message must not be blank");
        }
    }

    public static @NonNull JsonError of(
            int code,
            String message,
            Path source,
            Throwable cause
    ) {
        return new JsonError(code, message, cause, source);
    }

    public static @NonNull JsonError of(
            int code,
            String message,
            Path source
    ) {
        return new JsonError(code, message, null, source);
    }

}
