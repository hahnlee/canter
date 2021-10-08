/* ----------------------------------------------------------------------------
 *   MobileDevice.h - interface to MobileDevice.framework 
 *
 * Adapted from http://theiphonewiki.com/wiki/index.php?title=MobileDevice_Library
 * Adapted from https://github.com/imkira/mobiledevice/blob/master/mobiledevice.h
 * ------------------------------------------------------------------------- */
#ifndef MOBILEDEVICE_H
#define MOBILEDEVICE_H

#include <mach/error.h>
#include <CoreFoundation/CoreFoundation.h>

#define ADNCI_MSG_CONNECTED 1
#define ADNCI_MSG_DISCONNECTED 2
#define ADNCI_MSG_UNSUBSCRIBED 3

struct am_device
{
    unsigned char unknown0[16];
    unsigned int device_id;
    unsigned int product_id;
    char *serial;
    unsigned int unknown1;
    unsigned int unknown2;
    unsigned int lockdown_conn;
    unsigned char unknown3[8];
    unsigned int unknown4;
    unsigned char unknown5[24];
};

struct am_device_notification_callback_info
{
    struct am_device *dev;
    unsigned int msg;
    struct am_device_notification *subscription;
};

typedef void (*am_device_notification_callback)(struct am_device_notification_callback_info *, int cookie);

struct am_device_notification
{
    unsigned int unknown0;
    unsigned int unknown1;
    unsigned int unknown2;
    am_device_notification_callback callback;
    unsigned int cookie;
};

mach_error_t AMDeviceNotificationSubscribe(am_device_notification_callback callback, unsigned int unused0, unsigned int unused1, void *dn_unknown3, struct am_device_notification **notification);

CFStringRef AMDeviceCopyDeviceIdentifier(struct am_device *device);

mach_error_t AMDeviceConnect(struct am_device *device);

#endif
