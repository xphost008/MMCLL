package mmcll.api.annotation;

import java.lang.annotation.*;

/**
 * Describes an MMCLL service/provider implementation.
 * <p>
 * This annotation is informational by default. Implementations are discovered via {@link java.util.ServiceLoader}.
 */
@Documented
@Target(ElementType.TYPE)
@Retention(RetentionPolicy.CLASS)
public @interface MmcllProvider {
    /**
     * Human friendly provider name.
     */
    String name();

    /**
     * Provider priority. Higher priority is preferred when multiple providers are present.
     */
    int priority() default 0;

    /**
     * Optional feature tags, e.g. "range-download", "microsoft-devicecode".
     */
    String[] features() default {};
}
