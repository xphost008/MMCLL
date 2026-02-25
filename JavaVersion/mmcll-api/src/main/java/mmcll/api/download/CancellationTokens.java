package mmcll.api.download;

import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.Future;

/**
 * Factory methods for {@link CancellationToken}.
 */
@PublicApi
public final class CancellationTokens {
    private CancellationTokens() {
    }

    public static @NonNull CancellationToken none() {
        return CancellationToken.none();
    }

    /**
     * A token that is cancelled when the provided future is cancelled.
     */
    public static @NonNull CancellationToken fromFuture(@NonNull Future<?> future) {
        return future::isCancelled;
    }

    /**
     * A token that is cancelled when the provided CompletableFuture is completed exceptionally with
     * CancellationException or cancelled via {@link CompletableFuture#cancel(boolean)}.
     */
    public static @NonNull CancellationToken fromCompletableFuture(@NonNull CompletableFuture<?> future) {
        return future::isCancelled;
    }

    /**
     * Compose tokens: cancelled if any token is cancelled.
     */
    public static @NonNull CancellationToken anyOf(
            @NonNull CancellationToken a,
            @NonNull CancellationToken b
    ) {
        return () -> a.isCancelled() || b.isCancelled();
    }

}
