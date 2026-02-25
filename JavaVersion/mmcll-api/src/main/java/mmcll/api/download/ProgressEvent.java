package mmcll.api.download;

import mmcll.api.annotation.PublicApi;
import org.jspecify.annotations.Nullable;

import java.net.URI;

/**
 * Progress event used by download/ensure operations.
 */
@PublicApi
public record ProgressEvent(
        @Nullable String stage,
        @Nullable String name,
        URI uri,
        long current,
        long total,
        int index,
        int totalCount
) {
    public ProgressEvent {
        if (stage == null) stage = "";
        if (name == null) name = "";
    }

    /**
     * Returns progress ratio in range [0,1] when total > 0, otherwise -1.
     */
    public double ratio() {
        if (total <= 0) return -1.0;
        double r = (double) current / (double) total;
        if (r < 0) return 0;
        if (r > 1) return 1;
        return r;
    }

}
