package mmcll.api.error;

import jakarta.validation.constraints.NotBlank;
import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

import java.nio.file.Path;

/**
 * Launch planning / launch execution error.
 */
@PublicApi
public record LaunchError(
        int code,
        @NonNull @NotBlank String message,
        Throwable cause,
        String versionId,
        Path mcRoot
) implements MmcllError {

    public LaunchError {
        if (message.isBlank()) {
            throw new IllegalArgumentException("message must not be blank");
        }
    }

    public static @NonNull LaunchError of(
            int code,
            String message,
            String versionId,
            Path mcRoot,
            Throwable cause
    ) {
        return new LaunchError(code, message, cause, versionId, mcRoot);
    }
}
