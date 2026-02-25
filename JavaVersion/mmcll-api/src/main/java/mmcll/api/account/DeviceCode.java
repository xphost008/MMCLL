package mmcll.api.account;

import jakarta.validation.constraints.NotBlank;
import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

import java.net.URI;
import java.time.Duration;
import java.time.Instant;

/**
 * OAuth 2.0 Device Authorization Grant response.
 */
@PublicApi
public record DeviceCode(
        @NonNull URI verificationUri,
        URI verificationUriComplete,
        @NonNull @NotBlank String userCode,
        @NonNull @NotBlank String deviceCode,
        @NonNull Duration expiresIn,
        @NonNull Duration interval,
        @NonNull Instant issuedAt,
        String message
) {

    public DeviceCode {
        if (userCode.isBlank()) throw new IllegalArgumentException("userCode must not be blank");
        if (deviceCode.isBlank()) throw new IllegalArgumentException("deviceCode must not be blank");
    }

    /**
     * Returns {@code true} if now is after issuedAt+expiresIn.
     */
    public boolean isExpired(@NonNull Instant now) {
        return now.isAfter(issuedAt.plus(expiresIn));
    }

    public Instant expiresAt() {
        return issuedAt.plus(expiresIn);
    }

}
