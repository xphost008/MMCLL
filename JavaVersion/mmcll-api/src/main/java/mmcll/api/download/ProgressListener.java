package mmcll.api.download;

import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

/**
 * Listener for progress events.
 */
@PublicApi
@FunctionalInterface
public interface ProgressListener {
    static @NonNull ProgressListener noop() {
        return e -> {
        };
    }

    void onProgress(ProgressEvent event);

}
