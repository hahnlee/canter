#include <stddef.h>
#include <stdio.h>
#include <vector>
#include "includes/MobileDevice.h"

static struct am_device_notification *device_notification = NULL;

extern "C" const char * getUDID(am_device device)
{
  CFStringRef nsUDID = AMDeviceCopyDeviceIdentifier(&device);
  const char* udid = CFStringGetCStringPtr(nsUDID, kCFStringEncodingUTF8);
  CFRelease(nsUDID);
  return udid;
}

extern "C" void AMDeviceNotificationSubscribeBridge(am_device_notification_callback callback)
{
  AMDeviceNotificationSubscribe(callback, 0, 0, 0, &device_notification);
}
