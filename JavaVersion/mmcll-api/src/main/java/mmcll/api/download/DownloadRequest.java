package mmcll.api.download;

import lombok.NonNull;
import mmcll.api.annotation.PublicApi;
import mmcll.api.model.HttpOptions;

import java.net.URI;
import java.nio.file.Path;

/**
 * Request for a single file download.
 */
@PublicApi
public record DownloadRequest(
        @NonNull URI url,
        @NonNull Path saveTo,
        String sha1,
        Long size,
        boolean allowRange,
        int rangeChunkBytes,
        int maxConnections,
        @NonNull HttpOptions httpOptions
) {

    public DownloadRequest {
        if (rangeChunkBytes <= 0) rangeChunkBytes = 8 * 1024 * 1024;
        if (maxConnections <= 0) maxConnections = 8;
    }

    public static @NonNull DownloadRequest simple(URI url, Path saveTo) {
        return new DownloadRequest(
                url,
                saveTo,
                null,
                null,
                true,
                8 * 1024 * 1024,
                8,
                HttpOptions.defaults()
        );
    }

    public @NonNull DownloadRequest withSha1(String sha1) {
        return new DownloadRequest(
                url,
                saveTo,
                sha1,
                size,
                allowRange,
                rangeChunkBytes,
                maxConnections,
                httpOptions
        );
    }

    public @NonNull DownloadRequest withSize(Long size) {
        return new DownloadRequest(
                url,
                saveTo,
                sha1,
                size,
                allowRange,
                rangeChunkBytes,
                maxConnections,
                httpOptions
        );
    }

    public @NonNull DownloadRequest withHttpOptions(HttpOptions options) {
        return new DownloadRequest(
                url,
                saveTo,
                sha1,
                size,
                allowRange,
                rangeChunkBytes,
                maxConnections,
                options
        );
    }

}
