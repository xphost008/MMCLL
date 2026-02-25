package mmcll.api.account;

import jakarta.validation.constraints.NotBlank;
import lombok.NonNull;
import mmcll.api.annotation.PublicApi;
import org.jspecify.annotations.Nullable;

import java.time.Instant;
import java.util.Map;

/**
 * Microsoft account session.
 */
@PublicApi
public record MicrosoftSession(
        @NonNull AccountProfile profile,
        @NonNull @NotBlank String accessToken,
        String refreshToken,
        Instant expiresAt,
        @Nullable String xuid,
        @Nullable Map<String, ?> userProperties
) implements AccountSession {

    public MicrosoftSession {
        if (accessToken.isBlank()) {
            throw new IllegalArgumentException("accessToken must not be blank");
        }
        if (refreshToken == null) refreshToken = "";
        if (userProperties == null) userProperties = Map.of();
    }

    @Override
    public AccountType type() {
        return AccountType.MICROSOFT;
    }

    @Override
    public String base64Metadata() {
        // Not always required for Microsoft sessions.
        return "";
    }

}
