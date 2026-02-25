package mmcll.api.error;

import jakarta.validation.constraints.NotBlank;
import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

import java.net.URI;

/**
 * Authentication / authorization error.
 */
@PublicApi
public record AuthError(
        int code,
        @NonNull @NotBlank String message,
        Throwable cause,
        URI server,
        String stage
) implements MmcllError {

    public AuthError {
        if (message.isBlank()) {
            throw new IllegalArgumentException("message must not be blank");
        }
    }

    public static AuthError of(int code, String message, String stage, URI server, Throwable cause) {
        return new AuthError(code, message, cause, server, stage);
    }

}
