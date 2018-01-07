# For more information about using CMake with Android Studio, read the
# documentation: https://d.android.com/studio/projects/add-native-code.html

# Sets the minimum version of CMake required to build the native library.

cmake_minimum_required(VERSION 3.4.1)

add_definitions(-DGL_GLEXT_PROTOTYPES)

include_directories(src/SDL/include)
include_directories(src/SDL/include/configs/default)

# Creates and names a library, sets it as either STATIC
# or SHARED, and provides the relative paths to its source code.
# You can define multiple libraries, and CMake builds them for you.
# Gradle automatically packages shared libraries with your APK.

add_library( # Sets the name of the library.
             SDL2

             # Sets the library as a shared library.
             SHARED

             # Provides a relative path to your source file(s).
             #src/main/cpp/native-lib.cpp 
        #src/SDL/android.c
        src/SDL/src/SDL.c
        src/SDL/src/SDL_assert.c
        src/SDL/src/SDL_dataqueue.c
        src/SDL/src/SDL_error.c
        src/SDL/src/SDL_hints.c
        src/SDL/src/SDL_log.c
        src/SDL/src/atomic/SDL_atomic.c
        src/SDL/src/atomic/SDL_spinlock.c
        src/SDL/src/audio/SDL_audio.c
        src/SDL/src/audio/SDL_audiocvt.c
        src/SDL/src/audio/SDL_audiotypecvt.c
        src/SDL/src/audio/SDL_mixer.c
        src/SDL/src/audio/SDL_wave.c
        src/SDL/src/cpuinfo/SDL_cpuinfo.c
        src/SDL/src/dynapi/SDL_dynapi.c
        src/SDL/src/events/SDL_clipboardevents.c
        src/SDL/src/events/SDL_dropevents.c
        src/SDL/src/events/SDL_events.c
        src/SDL/src/events/SDL_gesture.c
        src/SDL/src/events/SDL_keyboard.c
        src/SDL/src/events/SDL_mouse.c
        src/SDL/src/events/SDL_quit.c
        src/SDL/src/events/SDL_touch.c
        src/SDL/src/events/SDL_windowevents.c
        src/SDL/src/file/SDL_rwops.c
        src/SDL/src/haptic/SDL_haptic.c
        src/SDL/src/joystick/SDL_gamecontroller.c
        src/SDL/src/joystick/SDL_joystick.c
        src/SDL/src/libm/e_atan2.c
        src/SDL/src/libm/e_log.c
        src/SDL/src/libm/e_pow.c
        src/SDL/src/libm/e_rem_pio2.c
        src/SDL/src/libm/e_sqrt.c
        src/SDL/src/libm/k_cos.c
        src/SDL/src/libm/k_rem_pio2.c
        src/SDL/src/libm/k_sin.c
        src/SDL/src/libm/k_tan.c
        src/SDL/src/libm/s_atan.c
        src/SDL/src/libm/s_copysign.c
        src/SDL/src/libm/s_cos.c
        src/SDL/src/libm/s_tan.c
        src/SDL/src/libm/s_fabs.c
        src/SDL/src/libm/s_floor.c
        src/SDL/src/libm/s_scalbn.c
        src/SDL/src/libm/s_sin.c
        #src/SDL/src/main/android/SDL_android_main.c
        src/SDL/src/power/SDL_power.c
        src/SDL/src/render/SDL_render.c
        src/SDL/src/render/SDL_yuv_mmx.c
        src/SDL/src/render/SDL_yuv_sw.c
        src/SDL/src/stdlib/SDL_getenv.c
        src/SDL/src/stdlib/SDL_iconv.c
        src/SDL/src/stdlib/SDL_malloc.c
        src/SDL/src/stdlib/SDL_qsort.c
        src/SDL/src/stdlib/SDL_stdlib.c
        src/SDL/src/stdlib/SDL_string.c
        src/SDL/src/thread/SDL_thread.c
        src/SDL/src/timer/SDL_timer.c
        src/SDL/src/video/SDL_RLEaccel.c
        src/SDL/src/video/SDL_blit.c
        src/SDL/src/video/SDL_blit_0.c
        src/SDL/src/video/SDL_blit_1.c
        src/SDL/src/video/SDL_blit_A.c
        src/SDL/src/video/SDL_blit_N.c
        src/SDL/src/video/SDL_blit_auto.c
        src/SDL/src/video/SDL_blit_copy.c
        src/SDL/src/video/SDL_blit_slow.c
        src/SDL/src/video/SDL_bmp.c
        src/SDL/src/video/SDL_clipboard.c
        src/SDL/src/video/SDL_fillrect.c
        src/SDL/src/video/SDL_pixels.c
        src/SDL/src/video/SDL_rect.c
        src/SDL/src/video/SDL_shape.c
        src/SDL/src/video/SDL_stretch.c
        src/SDL/src/video/SDL_surface.c
        src/SDL/src/video/SDL_video.c
        src/SDL/src/audio/disk/SDL_diskaudio.c
        src/SDL/src/loadso/dlopen/SDL_sysloadso.c
        src/SDL/src/render/opengl/SDL_render_gl.c
        src/SDL/src/render/opengl/SDL_shaders_gl.c
        src/SDL/src/render/software/SDL_blendfillrect.c
        src/SDL/src/render/software/SDL_blendline.c
        src/SDL/src/render/software/SDL_blendpoint.c
        src/SDL/src/render/software/SDL_drawline.c
        src/SDL/src/render/software/SDL_drawpoint.c
        src/SDL/src/render/software/SDL_render_sw.c
        src/SDL/src/render/software/SDL_rotate.c
        src/SDL/src/video/dummy/SDL_nullevents.c
        src/SDL/src/video/dummy/SDL_nullframebuffer.c
        src/SDL/src/video/dummy/SDL_nullvideo.c
        src/SDL/src/audio/dummy/SDL_dummyaudio.c	

            src/SDL/src/thread/pthread/SDL_syscond.c
            src/SDL/src/thread/pthread/SDL_sysmutex.c
            src/SDL/src/thread/pthread/SDL_syssem.c
            src/SDL/src/thread/pthread/SDL_systhread.c
            src/SDL/src/thread/pthread/SDL_systls.c
            src/SDL/src/timer/unix/SDL_systimer.c

            src/SDL/src/video/SDL_egl.c
            src/SDL/src/render/opengles/SDL_render_gles.c
            src/SDL/src/render/opengles2/SDL_render_gles2.c
            src/SDL/src/render/opengles2/SDL_shaders_gles2.c

            src/SDL/src/audio/android/SDL_androidaudio.c
            src/SDL/src/core/android/SDL_android.c
            src/SDL/src/joystick/android/SDL_sysjoystick.c
            src/SDL/src/filesystem/android/SDL_sysfilesystem.c
            src/SDL/src/power/android/SDL_syspower.c
            src/SDL/src/video/android/SDL_androidclipboard.c
            src/SDL/src/video/android/SDL_androidevents.c
            src/SDL/src/video/android/SDL_androidgl.c
            src/SDL/src/video/android/SDL_androidkeyboard.c
            src/SDL/src/video/android/SDL_androidtouch.c
            src/SDL/src/video/android/SDL_androidvideo.c
            src/SDL/src/video/android/SDL_androidwindow.c
            src/SDL/src/video/android/SDL_androidmessagebox.c
            src/SDL/src/video/android/SDL_androidmouse.c
            src/SDL/src/haptic/dummy/SDL_syshaptic.c

            src/SDL/include/SDL_config_android.h
             )

# Searches for a specified prebuilt library and stores the path as a
# variable. Because CMake includes system libraries in the search path by
# default, you only need to specify the name of the public NDK library
# you want to add. CMake verifies that the library exists before
# completing its build.

find_library( # Sets the name of the path variable.
              log-lib

              # Specifies the name of the NDK library that
              # you want CMake to locate.
              log )

find_library(GLESv1_CM-lib GLESv1_CM)
find_library(EGL-lib EGL)
find_library(GLESv2-lib GLESv2)
find_library(android-lib android)
find_library(dl-lib dl)

# Specifies libraries CMake should link to your target library. You
# can link multiple libraries, such as libraries you define in this
# build script, prebuilt third-party libraries, or system libraries.

target_link_libraries( # Specifies the target library.
                       SDL2

                       # Links the target library to the log library
                       # included in the NDK.
                       ${log-lib} 
                       ${GLESv1_CM-lib}
                       ${EGL-lib}
                       ${GLESv2-lib}
                       ${android-lib}
                       ${dl-lib}
                       )
