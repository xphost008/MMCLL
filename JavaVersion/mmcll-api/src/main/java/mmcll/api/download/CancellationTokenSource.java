package mmcll.api.download;

import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

import java.util.concurrent.atomic.AtomicBoolean;

/**
 * A mutable cancellation token source.
 * <p>
 * You can pass {@link #token()} into long-running operations and call {@link #cancel()} to request cancellation.
 */
@PublicApi
public final class CancellationTokenSource {
    private final AtomicBoolean cancelled = new AtomicBoolean(false);

    /**
     * Requests cancellation.
     */
    public void cancel() {
        cancelled.set(true);
    }

    public boolean isCancelled() {
        return cancelled.get();
    }

    /**
     * Creates a token that is cancelled if either the source is cancelled or the provided token is cancelled.
     */
    public @NonNull CancellationToken linkedToken(@NonNull CancellationToken other) {
        CancellationToken self = token();
        return () -> self.isCancelled() || other.isCancelled();
    }

    /**
     * Token view for this source.
     */
    public @NonNull CancellationToken token() {
        return cancelled::get;
    }

}
