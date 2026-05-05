package mmcll.api.download;

import mmcll.api.annotation.PublicApi;

import java.nio.file.Path;
import java.util.concurrent.CompletionStage;

/**
 * Download-related operations.
 */
@PublicApi
public interface DownloadService {

    /**
     * Download a single file.
     */
    CompletionStage<Path> downloadSingle(
            DownloadRequest request,
            ProgressListener progress,
            CancellationToken cancellation
    );

    /**
     * Ensure a full Minecraft version environment is ready locally.
     */
    CompletionStage<DownloadReport> ensureVersionReady(
            EnsureRequest request,
            ProgressListener progress,
            CancellationToken cancellation
    );

}
