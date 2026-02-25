package mmcll.api.launcher;

import mmcll.api.annotation.PublicApi;
import org.jspecify.annotations.Nullable;

import java.nio.file.Path;
import java.util.List;

/**
 * Optional richer result of launch planning.
 */
@PublicApi
public record LaunchResult(
        LaunchPlan plan,
        String mainClass,
        Path nativesDirectory,
        @Nullable List<Path> classpathEntries
) {
    public LaunchResult {
        if (classpathEntries == null) classpathEntries = List.of();
        classpathEntries = List.copyOf(classpathEntries);
    }
}
