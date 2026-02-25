package mmcll.api.download;

import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

import java.nio.file.Path;
import java.time.Duration;
import java.util.List;

/**
 * Result of ensure/download operations.
 */
@PublicApi
public record DownloadReport(
        int totalTasks,
        int succeededTasks,
        int skippedTasks,
        int failedTasks,
        long downloadedBytes,
        @NonNull Duration duration,
        @NonNull List<Path> changedFiles
) {

    public DownloadReport {
        changedFiles = List.copyOf(changedFiles);
    }

    public static @NonNull DownloadReport empty() {
        return new DownloadReport(0, 0, 0, 0, 0, Duration.ZERO, List.of());
    }

}
