package mmcll.api.error;

import mmcll.api.annotation.PublicApi;

/**
 * MMCLL runtime exception wrapping a structured {@link MmcllError}.
 */
@PublicApi
public final class MmcllException extends RuntimeException {
    private final MmcllError error;

    public MmcllException(MmcllError error) {
        super(error == null ? null : error.message(), error == null ? null : error.cause());
        if (error == null) {
            throw new IllegalArgumentException("error must not be null");
        }
        this.error = error;
    }

    public MmcllError error() {
        return error;
    }

    public int code() {
        return error.code();
    }

    @Override
    public String toString() {
        return "MmcllException{code=%d, message='%s'}".formatted(error.code(), error.message());
    }

}
