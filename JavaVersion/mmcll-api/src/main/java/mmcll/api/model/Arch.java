package mmcll.api.model;

import mmcll.api.annotation.PublicApi;

/**
 * CPU architecture classification used for selecting Minecraft native classifiers.
 */
@PublicApi
public enum Arch {
    X86_32,
    X86_64,
    ARM_32,
    ARM_64,
    UNKNOWN;

    /**
     * Tries to classify from typical {@code os.arch} values.
     */
    public static Arch detect() {
        String arch = System.getProperty("os.arch", "").toLowerCase();
        return fromOsArch(arch);
    }

    public static Arch fromOsArch(String osArch) {
        if (osArch == null) return UNKNOWN;
        String a = osArch.toLowerCase();
        // x86
        if (a.equals("x86") || a.equals("i386") || a.equals("i486") || a.equals("i586") || a.equals("i686")) {
            return X86_32;
        }
        if (a.equals("x86_64") || a.equals("amd64")) {
            return X86_64;
        }
        // arm
        if (a.startsWith("aarch64") || a.equals("arm64")) {
            return ARM_64;
        }
        if (a.startsWith("arm") || a.equals("armv7") || a.equals("armv7l")) {
            return ARM_32;
        }
        return UNKNOWN;
    }
}
