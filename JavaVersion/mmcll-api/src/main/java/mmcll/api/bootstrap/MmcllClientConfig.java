package mmcll.api.bootstrap;

import lombok.NonNull;
import mmcll.api.annotation.PublicApi;
import mmcll.api.model.HttpOptions;
import mmcll.api.model.OsInfo;

import java.util.Collections;
import java.util.LinkedHashMap;
import java.util.Map;

/**
 * Configuration passed to {@link MmcllClientProvider}.
 */
@PublicApi
public record MmcllClientConfig(
        @NonNull OsInfo os,
        @NonNull HttpOptions http,
        @NonNull Map<String, Object> attributes
) {

    public MmcllClientConfig {
        attributes = Collections.unmodifiableMap(new LinkedHashMap<>(attributes));
    }

    public static @NonNull MmcllClientConfig defaults() {
        return new MmcllClientConfig(OsInfo.detect(), HttpOptions.defaults(), Map.of());
    }

    public @NonNull MmcllClientConfig withAttribute(String key, Object value) {
        var map = new LinkedHashMap<>(attributes);
        map.put(key, value);
        return new MmcllClientConfig(os, http, map);
    }

    public Object attribute(String key) {
        return attributes.get(key);
    }

}
