package mmcll.api.download;

import lombok.NonNull;
import mmcll.api.annotation.PublicApi;
import mmcll.api.model.HttpOptions;

import java.nio.file.Path;

/**
 * Request to ensure a Minecraft version is fully prepared locally (version json/jar, libraries, natives, assets).
 */
@PublicApi
public record EnsureRequest(
        @NonNull Path mcRoot,
        @NonNull String versionId,
        boolean includeAssets,
        int maxConcurrency,
        @NonNull HttpOptions httpOptions
) {
    public EnsureRequest {
        if (maxConcurrency <= 0) maxConcurrency = 32;
    }

    public static @NonNull EnsureRequest of(Path mcRoot, String versionId) {
        return new EnsureRequest(
                mcRoot,
                versionId,
                true,
                32,
                HttpOptions.defaults()
        );
    }

}
