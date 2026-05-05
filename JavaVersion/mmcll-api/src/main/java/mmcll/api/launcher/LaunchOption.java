package mmcll.api.launcher;

import jakarta.validation.constraints.NotBlank;
import lombok.NonNull;
import mmcll.api.annotation.PublicApi;
import org.jspecify.annotations.Nullable;

import java.nio.file.Path;
import java.util.Collections;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

/**
 * Options for planning a Minecraft launch.
 */
@PublicApi
public record LaunchOption(
        @NonNull Path javaExecutable,
        @NonNull Path mcRoot,
        @NonNull @NotBlank String versionId,
        @NonNull Path gameDir,
        int minMemoryMb,
        int maxMemoryMb,
        int windowWidth,
        int windowHeight,
        boolean checkLibraries,
        @Nullable List<String> extraJvmArgs,
        @Nullable List<String> extraGameArgs,
        @Nullable Map<String, String> environment
) {

    public LaunchOption {
        if (versionId.isBlank()) {
            throw new IllegalArgumentException("versionId must not be blank");
        }
        if (minMemoryMb <= 0) {
            throw new IllegalArgumentException("minMemoryMb must be > 0");
        }
        if (maxMemoryMb <= 0) {
            throw new IllegalArgumentException("maxMemoryMb must be > 0");
        }
        if (maxMemoryMb < minMemoryMb) {
            throw new IllegalArgumentException("maxMemoryMb must be >= minMemoryMb");
        }
        if (windowWidth < 0 || windowHeight < 0) {
            throw new IllegalArgumentException("window size must be >= 0");
        }
        if (extraJvmArgs == null) extraJvmArgs = List.of();
        if (extraGameArgs == null) extraGameArgs = List.of();
        if (environment == null) environment = Map.of();
        extraJvmArgs = List.copyOf(extraJvmArgs);
        extraGameArgs = List.copyOf(extraGameArgs);
        environment = Collections.unmodifiableMap(new LinkedHashMap<>(environment));
    }

    public static @NonNull Builder builder() {
        return new Builder();
    }

    /**
     * Builder for {@link LaunchOption}.
     */
    public static final class Builder {
        private Path javaExecutable;
        private Path mcRoot;
        private String versionId;
        private Path gameDir;
        private int minMemoryMb = 1024;
        private int maxMemoryMb = 4096;
        private int windowWidth = 854;
        private int windowHeight = 480;
        private boolean checkLibraries = true;
        private List<String> extraJvmArgs = List.of();
        private List<String> extraGameArgs = List.of();
        private Map<String, String> environment = Map.of();

        public Builder javaExecutable(@NonNull Path javaExecutable) {
            this.javaExecutable = javaExecutable;
            return this;
        }

        public Builder mcRoot(@NonNull Path mcRoot) {
            this.mcRoot = mcRoot;
            return this;
        }

        public Builder versionId(@NonNull String versionId) {
            this.versionId = versionId;
            return this;
        }

        public Builder gameDir(@NonNull Path gameDir) {
            this.gameDir = gameDir;
            return this;
        }

        public Builder minMemoryMb(int minMemoryMb) {
            this.minMemoryMb = minMemoryMb;
            return this;
        }

        public Builder maxMemoryMb(int maxMemoryMb) {
            this.maxMemoryMb = maxMemoryMb;
            return this;
        }

        public Builder window(int width, int height) {
            this.windowWidth = width;
            this.windowHeight = height;
            return this;
        }

        public Builder checkLibraries(boolean checkLibraries) {
            this.checkLibraries = checkLibraries;
            return this;
        }

        public Builder extraJvmArgs(List<String> extraJvmArgs) {
            this.extraJvmArgs = extraJvmArgs;
            return this;
        }

        public Builder extraGameArgs(List<String> extraGameArgs) {
            this.extraGameArgs = extraGameArgs;
            return this;
        }

        public Builder environment(Map<String, String> environment) {
            this.environment = environment;
            return this;
        }

        public LaunchOption build() {
            return new LaunchOption(
                    javaExecutable,
                    mcRoot,
                    versionId,
                    gameDir,
                    minMemoryMb,
                    maxMemoryMb,
                    windowWidth,
                    windowHeight,
                    checkLibraries,
                    extraJvmArgs,
                    extraGameArgs,
                    environment
            );
        }

    }

}
