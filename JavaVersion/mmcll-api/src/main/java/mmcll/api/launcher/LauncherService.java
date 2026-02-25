package mmcll.api.launcher;

import lombok.NonNull;
import mmcll.api.account.AccountSession;
import mmcll.api.annotation.PublicApi;

import java.io.IOException;

/**
 * Minecraft launch planning/execution.
 */
@PublicApi
public interface LauncherService {

    /**
     * Build a richer launch result.
     */
    default LaunchResult planLaunchDetailed(LaunchOption option, AccountSession session) {
        LaunchPlan plan = planLaunch(option, session);
        return new LaunchResult(plan, null, null, null);
    }

    /**
     * Build a launch plan (java executable + arguments + env + working dir).
     */
    LaunchPlan planLaunch(LaunchOption option, AccountSession session);

    /**
     * Start a process from a planned launch.
     */
    default Process launch(@NonNull LaunchPlan plan) throws IOException {
        return plan.start();
    }
}
