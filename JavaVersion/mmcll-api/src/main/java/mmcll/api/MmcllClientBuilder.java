package mmcll.api;

import lombok.NonNull;
import mmcll.api.account.AccountService;
import mmcll.api.annotation.PublicApi;
import mmcll.api.bootstrap.MmcllClientConfig;
import mmcll.api.bootstrap.MmcllClientProvider;
import mmcll.api.download.DownloadService;
import mmcll.api.error.GenericError;
import mmcll.api.error.MmcllException;
import mmcll.api.launcher.LauncherService;
import mmcll.api.model.HttpOptions;
import mmcll.api.model.OsInfo;

import java.util.*;

/**
 * Builder for {@link MmcllClient}.
 * <p>
 * Typical usage:
 * <pre>
 * MmcllClient client = MmcllClientBuilder.defaultClient().build();
 * </pre>
 */
@PublicApi
public final class MmcllClientBuilder {

    private MmcllClientConfig config = MmcllClientConfig.defaults();
    private String providerName;
    private MmcllClientProvider provider;

    private AccountService accountService;
    private DownloadService downloadService;
    private LauncherService launcherService;

    private MmcllClient overrideClient;

    private MmcllClientBuilder() {
    }

    /**
     * Create a builder that will attempt to load a provider from classpath.
     */
    public static @NonNull MmcllClientBuilder defaultClient() {
        return builder();
    }

    public static @NonNull MmcllClientBuilder builder() {
        return new MmcllClientBuilder();
    }

    /**
     * Force a specific provider name (matched case-insensitively).
     */
    public MmcllClientBuilder providerName(String providerName) {
        this.providerName = providerName;
        return this;
    }

    /**
     * Use a specific provider instance.
     */
    public MmcllClientBuilder provider(MmcllClientProvider provider) {
        this.provider = provider;
        return this;
    }

    /**
     * Directly supply a fully constructed client (skips provider discovery).
     */
    public MmcllClientBuilder client(MmcllClient client) {
        this.overrideClient = client;
        return this;
    }

    public MmcllClientBuilder os(@NonNull OsInfo osInfo) {
        this.config = new MmcllClientConfig(osInfo, config.http(), config.attributes());
        return this;
    }

    public MmcllClientBuilder http(@NonNull HttpOptions httpOptions) {
        this.config = new MmcllClientConfig(config.os(), httpOptions, config.attributes());
        return this;
    }

    public MmcllClientBuilder attributes(@NonNull Map<String, Object> attrs) {
        for (var e : attrs.entrySet()) {
            attribute(e.getKey(), e.getValue());
        }
        return this;
    }

    public MmcllClientBuilder attribute(@NonNull String key, Object value) {
        this.config = config.withAttribute(key, value);
        return this;
    }

    public MmcllClientBuilder accounts(AccountService accountService) {
        this.accountService = accountService;
        return this;
    }

    public MmcllClientBuilder downloads(DownloadService downloadService) {
        this.downloadService = downloadService;
        return this;
    }

    public MmcllClientBuilder launcher(LauncherService launcherService) {
        this.launcherService = launcherService;
        return this;
    }

    public MmcllClientConfig config() {
        return config;
    }

    public MmcllClient build() {
        if (overrideClient != null) {
            return overrideClient;
        }

        // If user provided all services, no need for providers.
        if (accountService != null && downloadService != null && launcherService != null) {
            return new BuiltMmcllClient(accountService, downloadService, launcherService, null);
        }

        MmcllClient base = resolveProvider().create(config);
        AccountService acc = accountService != null ? accountService : base.accounts();
        DownloadService dl = downloadService != null ? downloadService : base.downloads();
        LauncherService la = launcherService != null ? launcherService : base.launcher();

        return new BuiltMmcllClient(acc, dl, la, base);
    }

    private MmcllClientProvider resolveProvider() {
        if (provider != null) {
            return provider;
        }

        List<MmcllClientProvider> providers = new ArrayList<>();
        for (MmcllClientProvider p : ServiceLoader.load(MmcllClientProvider.class)) {
            if (p != null && p.supports(config)) {
                providers.add(p);
            }
        }

        if (providers.isEmpty()) {
            throw new MmcllException(new GenericError(
                    1000,
                    "No MmcllClientProvider found on classpath. Add a provider module (e.g. mmcll-default) " +
                            "or supply AccountService/DownloadService/LauncherService manually."
            ));
        }

        if (providerName != null && !providerName.isBlank()) {
            for (MmcllClientProvider p : providers) {
                if (p.name() != null && p.name().equalsIgnoreCase(providerName)) {
                    return p;
                }
            }
            throw new MmcllException(new GenericError(
                    1001,
                    "MmcllClientProvider with name '" + providerName + "' not found. Available providers: " +
                            providers.stream().map(MmcllClientProvider::name).toList()
            ));
        }

        return providers.stream()
                .max(Comparator.comparingInt(MmcllClientProvider::priority)
                        .thenComparing(p -> p.name() == null ? "" : p.name()))
                .orElseThrow();
    }

    private static final class BuiltMmcllClient implements MmcllClient {
        private final AccountService accounts;
        private final DownloadService downloads;
        private final LauncherService launcher;
        private final MmcllClient delegate;

        private BuiltMmcllClient(
                @NonNull AccountService accounts,
                @NonNull DownloadService downloads,
                @NonNull LauncherService launcher,
                MmcllClient delegate
        ) {
            this.accounts = accounts;
            this.downloads = downloads;
            this.launcher = launcher;
            this.delegate = delegate;
        }

        @Override
        public AccountService accounts() {
            return accounts;
        }

        @Override
        public DownloadService downloads() {
            return downloads;
        }

        @Override
        public LauncherService launcher() {
            return launcher;
        }

        @Override
        public void close() {
            if (delegate != null) {
                try {
                    delegate.close();
                } catch (Exception _) {
                }
            }
        }

    }

}
