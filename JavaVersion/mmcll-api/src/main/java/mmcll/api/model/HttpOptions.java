package mmcll.api.model;

import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

import java.time.Duration;
import java.util.Collections;
import java.util.LinkedHashMap;
import java.util.Map;

/**
 * HTTP options (timeouts, headers, cookies, proxy) used by MMCLL services.
 */
@PublicApi
public record HttpOptions(
        @NonNull Duration timeout,
        @NonNull Map<String, String> headers,
        @NonNull Map<String, String> cookies,
        @NonNull ProxyConfig proxy
) {

    public HttpOptions {
        headers = Collections.unmodifiableMap(new LinkedHashMap<>(headers));
        cookies = Collections.unmodifiableMap(new LinkedHashMap<>(cookies));
    }

    public static @NonNull HttpOptions defaults() {
        return new HttpOptions(Duration.ofSeconds(30), Map.of(), Map.of(), ProxyConfig.none());
    }

    public @NonNull HttpOptions withHeader(String key, String value) {
        var h = new LinkedHashMap<>(headers);
        h.put(key, value);
        return new HttpOptions(timeout, h, cookies, proxy);
    }

    public @NonNull HttpOptions withCookie(String key, String value) {
        var c = new LinkedHashMap<>(cookies);
        c.put(key, value);
        return new HttpOptions(timeout, headers, c, proxy);
    }

    public @NonNull HttpOptions withProxy(ProxyConfig proxy) {
        return new HttpOptions(timeout, headers, cookies, proxy);
    }

    public @NonNull HttpOptions withTimeout(Duration timeout) {
        return new HttpOptions(timeout, headers, cookies, proxy);
    }
}
