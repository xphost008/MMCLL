package mmcll.api.annotation;

import java.lang.annotation.*;

/**
 * Marks a type or member as part of the stable public API.
 * <p>
 * Public API aims to be backwards compatible within a major version.
 */
@Documented
@Target({ElementType.TYPE, ElementType.METHOD, ElementType.CONSTRUCTOR, ElementType.FIELD})
@Retention(RetentionPolicy.CLASS)
public @interface PublicApi {
}
