package mmcll.api.account;

import mmcll.api.annotation.PublicApi;

/**
 * Account type used for launch parameter userType and auth strategy.
 */
@PublicApi
public enum AccountType {
    OFFLINE,
    MICROSOFT,
    THIRD_PARTY,
    YGGDRASIL
}
