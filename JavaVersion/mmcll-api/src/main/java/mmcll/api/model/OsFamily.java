package mmcll.api.model;

import mmcll.api.annotation.PublicApi;

/**
 * Operating system family used for rules evaluation and natives selection.
 */
@PublicApi
public enum OsFamily {
    WINDOWS,
    LINUX,
    MACOS,
    UNKNOWN;

    public static OsFamily detect() {
        String os = System.getProperty("os.name", "").toLowerCase();
        if (os.contains("win")) return WINDOWS;
        if (os.contains("mac") || os.contains("darwin")) return MACOS;
        if (os.contains("nux") || os.contains("linux")) return LINUX;
        return UNKNOWN;
    }

    /**
     * Mojang-style OS name used in version rules: windows/linux/osx.
     */
    public String mojangName() {
        return switch (this) {
            case WINDOWS -> "windows";
            case LINUX -> "linux";
            case MACOS -> "osx";
            case UNKNOWN -> "unknown";
        };
    }
}
