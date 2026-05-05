package mmcll.api.download;

import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

/**
 * Cooperative cancellation token.
 * <p>
 * Implementations should be thread-safe.
 */
@PublicApi
@FunctionalInterface
public interface CancellationToken {
    /**
     * Convenience no-op token.
     */
    static @NonNull CancellationToken none() {
        return () -> false;
    }

    /**
     * Returns {@code true} if cancellation was requested.
     */
    boolean isCancelled();

}
