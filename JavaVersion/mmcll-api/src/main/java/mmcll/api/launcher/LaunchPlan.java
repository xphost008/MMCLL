package mmcll.api.launcher;

import lombok.NonNull;
import mmcll.api.annotation.PublicApi;

import java.io.IOException;
import java.nio.file.Path;
import java.util.*;

/**
 * A planned launch command.
 * <p>
 * The plan is immutable and can be executed using {@link #start()}.
 */
@PublicApi
public record LaunchPlan(
        @NonNull Path javaExecutable,
        @NonNull List<String> arguments,
        @NonNull Map<String, String> environment,
        @NonNull Path workingDirectory
) {

    public LaunchPlan {
        arguments = List.copyOf(arguments);
        environment = Collections.unmodifiableMap(new LinkedHashMap<>(environment));
    }

    /**
     * Starts the process using {@link ProcessBuilder}.
     */
    public @NonNull Process start() throws IOException {
        ProcessBuilder pb = new ProcessBuilder(command());
        pb.directory(workingDirectory.toFile());
        if (!environment.isEmpty()) {
            pb.environment().putAll(environment);
        }
        return pb.start();
    }

    /**
     * Full command list including java executable.
     */
    public @NonNull List<String> command() {
        List<String> cmd = new ArrayList<>(arguments.size() + 1);
        cmd.add(javaExecutable.toString());
        cmd.addAll(arguments);
        return Collections.unmodifiableList(cmd);
    }

}
