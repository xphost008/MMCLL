package mmcll.api.account;

import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

/**
 * Offline (cracked) account session.
 */
@PublicApi
public record OfflineSession(@NonNull AccountProfile profile) implements AccountSession {

    public static @NonNull OfflineSession ofName(String name) {
        return new OfflineSession(AccountProfile.offline(name));
    }

    @Override
    public AccountType type() {
        return AccountType.OFFLINE;
    }

}
