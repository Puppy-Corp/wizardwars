#include <android/log.h>
#include <jni.h>
#include <string>
#include <iostream>

#define LOG_TAG "WizardWarsNative"
#define LOGI(...) __android_log_print(ANDROID_LOG_INFO, LOG_TAG, __VA_ARGS__)
#define LOGD(...) __android_log_print(ANDROID_LOG_DEBUG, LOG_TAG, __VA_ARGS__)
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, LOG_TAG, __VA_ARGS__)

extern "C" void android_main(struct android_app* app) {
    LOGI("HELLO from android");
}