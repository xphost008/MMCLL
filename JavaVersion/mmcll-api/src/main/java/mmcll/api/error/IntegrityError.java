package mmcll.api.error;

import jakarta.validation.constraints.NotBlank;
import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

import java.nio.file.Path;

/**
 * File integrity error, e.g. sha1 mismatch.
 */
@PublicApi
public record IntegrityError(
        int code,
        @NonNull @NotBlank String message,
        Throwable cause,
        Path file,
        String expectedSha1,
        String actualSha1
) implements MmcllError {

    public IntegrityError {
        if (message.isBlank()) {
            throw new IllegalArgumentException("message must not be blank");
        }
    }

    public static @NonNull IntegrityError sha1Mismatch(Path file, String expected, String actual) {
        return new IntegrityError(
                3001,
                "SHA-1 mismatch for file: " + file,
                null,
                file,
                expected,
                actual
        );
    }

}
