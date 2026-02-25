package mmcll.api.account;

import mmcll.api.annotation.PublicApi;
import mmcll.api.download.CancellationToken;

import java.net.URI;
import java.util.concurrent.CompletionStage;

/**
 * Account login/validation API.
 */
@PublicApi
public interface AccountService {

    /**
     * Create offline session.
     */
    default OfflineSession offline(String name) {
        return OfflineSession.ofName(name);
    }

    /**
     * Begin Microsoft device code login flow.
     */
    DeviceCode beginMicrosoftDeviceCode(String clientId);

    /**
     * Poll until Microsoft login completes or fails.
     */
    CompletionStage<MicrosoftSession> pollMicrosoftDeviceCode(DeviceCode deviceCode, CancellationToken cancellation);

    /**
     * Begin third-party OAuth device code flow.
     */
    DeviceCode beginThirdPartyDeviceCode(URI deviceEndpoint, String clientId, String scope);

    /**
     * Poll until third-party device login completes or fails.
     */
    CompletionStage<ThirdPartySession> pollThirdPartyDeviceCode(DeviceCode deviceCode, URI tokenEndpoint, CancellationToken cancellation);

    /**
     * Login to a Yggdrasil auth server via username/password.
     */
    CompletionStage<YggdrasilSession> loginYggdrasil(URI server, String username, String password, CancellationToken cancellation);

    /**
     * Validate a session (best-effort).
     */
    CompletionStage<Boolean> validate(AccountSession session);

}
