package mmcll.api.account;

import jakarta.validation.constraints.NotBlank;
import lombok.NonNull;
import mmcll.api.annotation.PublicApi;
import org.jspecify.annotations.Nullable;

import java.net.URI;
import java.time.Instant;
import java.util.Map;

/**
 * Yggdrasil authentication session (classic launcher-like auth server).
 */
@PublicApi
public record YggdrasilSession(
        @NonNull AccountProfile profile,
        @NonNull URI server,
        @NonNull @NotBlank String accessToken,
        @Nullable String clientToken,
        Instant expiresAt,
        @Nullable Map<String, ?> userProperties
) implements AccountSession {

    public YggdrasilSession {
        if (accessToken.isBlank()) {
            throw new IllegalArgumentException("accessToken must not be blank");
        }
        if (clientToken == null) clientToken = "";
        if (userProperties == null) userProperties = Map.of();
    }

    @Override
    public AccountType type() {
        return AccountType.YGGDRASIL;
    }

}
