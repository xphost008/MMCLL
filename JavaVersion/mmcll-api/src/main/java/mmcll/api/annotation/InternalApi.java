package mmcll.api.annotation;

import java.lang.annotation.*;

/**
 * Marks an API as internal to MMCLL implementation.
 * <p>
 * Consumers should not rely on internal APIs; they may change at any time.
 */
@Documented
@Target({ElementType.TYPE, ElementType.METHOD, ElementType.CONSTRUCTOR, ElementType.FIELD})
@Retention(RetentionPolicy.CLASS)
public @interface InternalApi {
}
