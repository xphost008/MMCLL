package mmcll.api.account;

import mmcll.api.annotation.PublicApi;

import java.net.URI;
import java.time.Instant;
import java.util.Map;

/**
 * Authenticated session used for launching.
 */
@PublicApi
public sealed interface AccountSession permits OfflineSession, MicrosoftSession, ThirdPartySession, YggdrasilSession {

    AccountProfile profile();

    /**
     * Access token used by Minecraft to authenticate. May be empty for offline.
     */
    default String accessToken() {
        return "";
    }

    /**
     * Optional refresh token.
     */
    default String refreshToken() {
        return "";
    }

    /**
     * When token expires, if known.
     */
    default Instant expiresAt() {
        return null;
    }

    /**
     * The server base URI for third-party/Yggdrasil accounts, if applicable.
     */
    default URI server() {
        return null;
    }

    /**
     * Launch argument --userType value. Common values: legacy, msa, mojang.
     */
    default String userType() {
        return switch (type()) {
            case OFFLINE -> "legacy";
            case MICROSOFT -> "msa";
            case THIRD_PARTY -> "msa";
            case YGGDRASIL -> "mojang";
        };
    }

    AccountType type();

    /**
     * Optional base64 metadata for authlib-injector/third-party launch parameters.
     */
    default String base64Metadata() {
        return "";
    }

    /**
     * Optional user properties JSON-like map used by some versions/launchers.
     */
    default Map<String, ?> userProperties() {
        return Map.of();
    }

}
