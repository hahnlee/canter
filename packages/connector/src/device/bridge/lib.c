#include <stddef.h>
#include <stdio.h>
#include "includes/MobileDevice.h"

static struct am_device_notification *device_notification = NULL;

const char * getUDID(struct am_device device)
{
  CFStringRef nsUDID = AMDeviceCopyDeviceIdentifier(&device);
  const char* udid = CFStringGetCStringPtr(nsUDID, kCFStringEncodingUTF8);
  CFRelease(nsUDID);
  return udid;
}

extern void AMDeviceNotificationSubscribeBridge(am_device_notification_callback callback, void * manager, double timeout)
{
  AMDeviceNotificationSubscribe(callback, 0, 0, manager, &device_notification);
  CFRunLoopRunInMode(kCFRunLoopDefaultMode, timeout, FALSE);
}
