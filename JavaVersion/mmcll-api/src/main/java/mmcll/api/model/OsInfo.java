package mmcll.api.model;

import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

/**
 * Information about current runtime OS/Arch used by MMCLL.
 */
@PublicApi
public record OsInfo(
        @NonNull OsFamily family,
        @NonNull Arch arch
) {

    /**
     * Detects OS and architecture from current JVM system properties.
     */
    public static @NonNull OsInfo detect() {
        return new OsInfo(OsFamily.detect(), Arch.detect());
    }

    /**
     * Mojang rules OS string: windows/linux/osx.
     */
    public @NonNull String mojangOsName() {
        return family.mojangName();
    }

    /**
     * Java classpath separator: ';' on Windows, ':' otherwise.
     */
    public @NonNull String classpathSeparator() {
        return family == OsFamily.WINDOWS ? ";" : ":";
    }

    /**
     * Common arch placeholder used by many classifiers: 64/32.
     */
    public @NonNull String archBits() {
        return switch (arch) {
            case X86_64, ARM_64 -> "64";
            case X86_32, ARM_32 -> "32";
            default -> "";
        };
    }
}
