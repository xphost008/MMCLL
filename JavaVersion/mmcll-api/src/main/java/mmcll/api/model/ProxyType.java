package mmcll.api.model;

import mmcll.api.annotation.PublicApi;

/**
 * Proxy type hint for HTTP requests.
 */
@PublicApi
public enum ProxyType {
    /**
     * No proxy.
     */
    NONE,
    /**
     * System proxy selector.
     */
    SYSTEM,
    /**
     * HTTP proxy.
     */
    HTTP,
    /**
     * SOCKS proxy (typically SOCKS5).
     */
    SOCKS;
}
