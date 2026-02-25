package mmcll.api;

import mmcll.api.account.AccountService;
import mmcll.api.annotation.PublicApi;
import mmcll.api.download.DownloadService;
import mmcll.api.launcher.LauncherService;

/**
 * Main entry point for MMCLL library.
 */
@PublicApi
public interface MmcllClient extends AutoCloseable {

    AccountService accounts();

    DownloadService downloads();

    LauncherService launcher();

    /**
     * Close and release resources.
     * <p>
     * Default implementations should be idempotent.
     */
    @Override
    default void close() {
        // no-op by default
    }

}
