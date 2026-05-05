package mmcll.api.account;

import jakarta.validation.constraints.NotBlank;
import lombok.NonNull;
import mmcll.api.annotation.PublicApi;
import org.jspecify.annotations.Nullable;

import java.net.URI;
import java.time.Instant;
import java.util.Map;

/**
 * Third-party OAuth/Yggdrasil-like session used with authlib-injector or custom services.
 */
@PublicApi
public record ThirdPartySession(
        @NonNull AccountProfile profile,
        @NonNull URI server,
        @NonNull @NotBlank String accessToken,
        @Nullable String refreshToken,
        Instant expiresAt,
        @Nullable String base64Metadata,
        @Nullable Map<String, ?> userProperties
) implements AccountSession {

    public ThirdPartySession {
        if (accessToken.isBlank()) {
            throw new IllegalArgumentException("accessToken must not be blank");
        }
        if (refreshToken == null) refreshToken = "";
        if (base64Metadata == null) base64Metadata = "";
        if (userProperties == null) userProperties = Map.of();
    }

    @Override
    public AccountType type() {
        return AccountType.THIRD_PARTY;
    }

}
