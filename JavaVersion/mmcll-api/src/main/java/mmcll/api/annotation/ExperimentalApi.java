package mmcll.api.annotation;

import java.lang.annotation.*;

/**
 * Marks an API as experimental.
 * <p>
 * Experimental API may change in minor or patch releases.
 */
@Documented
@Target({ElementType.TYPE, ElementType.METHOD, ElementType.CONSTRUCTOR, ElementType.FIELD})
@Retention(RetentionPolicy.CLASS)
public @interface ExperimentalApi {
    /**
     * Optional note (e.g., migration hint).
     */
    String value() default "";
}
