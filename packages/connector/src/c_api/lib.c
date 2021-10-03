#include <stddef.h>
#include <stdio.h>
#include "includes/MobileDevice.h"

static struct am_device_notification *device_notification = NULL;

static void handleDeviceNotification(struct am_device_notification_callback_info *info, int cookie) {
  if (info->msg == ADNCI_MSG_CONNECTED) {
    CFStringRef udid = AMDeviceCopyDeviceIdentifier(info->dev);
    printf("UDID: %s\n", CFStringGetCStringPtr(udid, kCFStringEncodingUTF8));
  }
}

extern void subscribeDeviceNotification() {
  AMDeviceNotificationSubscribe(&handleDeviceNotification, 0, 0, 0, &device_notification);
}
