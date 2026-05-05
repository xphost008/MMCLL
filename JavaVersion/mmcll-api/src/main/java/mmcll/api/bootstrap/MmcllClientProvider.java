package mmcll.api.bootstrap;

import mmcll.api.MmcllClient;
import mmcll.api.annotation.PublicApi;

/**
 * Provider for {@link MmcllClient}.
 * <p>
 * Implementations are typically registered via {@code META-INF/services/mmcll.api.bootstrap.MmcllClientProvider}.
 */
@PublicApi
public interface MmcllClientProvider {

    /**
     * Provider name (unique).
     */
    String name();

    /**
     * Provider priority. Higher priority is preferred.
     */
    default int priority() {
        return 0;
    }

    /**
     * Whether this provider supports the given config.
     */
    default boolean supports(MmcllClientConfig config) {
        return true;
    }

    /**
     * Creates a client.
     */
    MmcllClient create(MmcllClientConfig config);

}
