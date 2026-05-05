package mmcll.api.account;

import jakarta.validation.constraints.NotBlank;
import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

import java.nio.charset.StandardCharsets;
import java.util.UUID;

/**
 * Basic user profile.
 */
@PublicApi
public record AccountProfile(
        @NonNull @NotBlank String name,
        @NonNull UUID uuid
) {

    public AccountProfile {
        if (name.isBlank()) {
            throw new IllegalArgumentException("name must not be blank");
        }
    }

    /**
     * Offline UUID used by many launchers: {@code UUID.nameUUIDFromBytes(("OfflinePlayer:" + name).getBytes(UTF_8))}.
     */
    public static @NonNull AccountProfile offline(@NonNull String name) {
        UUID uuid = UUID.nameUUIDFromBytes(("OfflinePlayer:" + name).getBytes(StandardCharsets.UTF_8));
        return new AccountProfile(name, uuid);
    }

    /**
     * UUID string without dashes, commonly used by Minecraft placeholders.
     */
    public @NonNull String uuidUndashed() {
        return uuid.toString().replace("-", "");
    }

}
