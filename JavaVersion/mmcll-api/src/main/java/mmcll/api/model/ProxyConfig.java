package mmcll.api.model;

import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

/**
 * Proxy configuration for HTTP requests.
 */
@PublicApi
public record ProxyConfig(
        @NonNull ProxyType type,
        String host,
        int port,
        String username,
        String password
) {

    public ProxyConfig {
        if ((type == ProxyType.HTTP || type == ProxyType.SOCKS) && (host == null || host.isBlank())) {
            throw new IllegalArgumentException("host must not be blank for proxy type " + type);
        }
        if ((type == ProxyType.HTTP || type == ProxyType.SOCKS) && (port <= 0 || port > 65535)) {
            throw new IllegalArgumentException("port out of range: " + port);
        }
    }

    public static @NonNull ProxyConfig none() {
        return new ProxyConfig(ProxyType.NONE, null, 0, null, null);
    }

    public static @NonNull ProxyConfig system() {
        return new ProxyConfig(ProxyType.SYSTEM, null, 0, null, null);
    }

    public static @NonNull ProxyConfig http(String host, int port) {
        return new ProxyConfig(ProxyType.HTTP, host, port, null, null);
    }

    public static @NonNull ProxyConfig socks(String host, int port) {
        return new ProxyConfig(ProxyType.SOCKS, host, port, null, null);
    }

}
