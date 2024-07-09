#define CIMGUI_DEFINE_ENUMS_AND_STRUCTS
#define IMGUI_IMPL_OPENGL_LOADER_GL3W
#define CIMGUI_USE_GLFW
#define CIMGUI_USE_OPENGL3
#include "cimgui/cimgui.h"
#include "cimgui/generator/output/cimgui_impl.h"
#include <dlfcn.h>
#include <GLFW/glfw3.h>
#include <stdio.h>
#ifdef _MSC_VER
#include <windows.h>
#endif
#include <GL/gl.h>
#include "plug.h"

#ifdef IMGUI_HAS_IMSTR
#define igBegin igBegin_Str
#define igSliderFloat igSliderFloat_Str
#define igCheckbox igCheckbox_Str
#define igColorEdit3 igColorEdit3_Str
#define igButton igButton_Str
#endif

GLFWwindow *window;

int main(int argc, char *argv[]) {
    if (!glfwInit()) {
        return 1;
    }

    // Decide GL+GLSL versions
    glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GLFW_TRUE);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 2);
    glfwWindowHint(GLFW_PLATFORM, GLFW_PLATFORM_X11);

#if __APPLE__
    // GL 3.2 Core + GLSL 150
    const char *glsl_version = "#version 150";
#else
    // GL 3.2 + GLSL 130
    const char *glsl_version = "#version 130";
#endif

    // just an extra window hint for resize
    glfwWindowHint(GLFW_RESIZABLE, GLFW_TRUE);

    window = glfwCreateWindow(1024, 768, "Blossom", NULL, NULL);

    if (!window) {
        printf("Failed to create window! Terminating!\n");
        glfwTerminate();
        return 1;
    }

    glfwMakeContextCurrent(window);

    // enable vsync
    glfwSwapInterval(1);

    // check opengl version sdl uses
    printf("opengl version: %s\n", (char *)glGetString(GL_VERSION));

    // setup imgui
    igCreateContext(NULL);

    // set docking
    ImGuiIO *io = igGetIO();
    io->ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;   // Enable Keyboard Controls
    //ioptr->ConfigFlags |= ImGuiConfigFlags_NavEnableGamepad;  // Enable Gamepad Controls
    io->ConfigFlags |= ImGuiConfigFlags_DockingEnable;       // Enable Docking

    ImGui_ImplGlfw_InitForOpenGL(window, true);
    ImGui_ImplOpenGL3_Init(glsl_version);

    igStyleColorsDark(NULL);

    ImVec4* colors = igGetStyle()->Colors;
    colors[ImGuiCol_Text]                   = (ImVec4) { 0.93f, 0.93f, 0.93f, 1.00f };
    colors[ImGuiCol_WindowBg]               = (ImVec4) { 0.13f, 0.16f, 0.19f, 1.00f };
    colors[ImGuiCol_Border]                 = (ImVec4) { 0.22f, 0.24f, 0.27f, 1.00f };
    colors[ImGuiCol_FrameBg]                = (ImVec4) { 0.22f, 0.24f, 0.27f, 1.00f };
    colors[ImGuiCol_FrameBgHovered]         = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_FrameBgActive]          = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_TitleBg]                = (ImVec4) { 0.13f, 0.16f, 0.19f, 1.00f };
    colors[ImGuiCol_TitleBgActive]          = (ImVec4) { 0.22f, 0.24f, 0.27f, 1.00f };
    colors[ImGuiCol_MenuBarBg]              = (ImVec4) { 0.13f, 0.16f, 0.19f, 1.00f };
    colors[ImGuiCol_CheckMark]              = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_SliderGrab]             = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_SliderGrabActive]       = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_Button]                 = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_ButtonHovered]          = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_ButtonActive]           = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_Header]                 = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_HeaderHovered]          = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_HeaderActive]           = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_ResizeGrip]             = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_ResizeGripHovered]      = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_ResizeGripActive]       = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_Tab]                    = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_TabHovered]             = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_TabActive]              = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };
    colors[ImGuiCol_TabUnfocused]           = (ImVec4) { 0.13f, 0.16f, 0.19f, 1.00f };
    colors[ImGuiCol_TabUnfocusedActive]     = (ImVec4) { 0.22f, 0.24f, 0.27f, 1.00f };
    colors[ImGuiCol_NavHighlight]           = (ImVec4) { 0.00f, 0.68f, 0.71f, 1.00f };

    ImGuiStyle *style = igGetStyle();
    style->WindowBorderSize = 0.0f;
    style->FrameRounding = 4.0f;
    style->GrabRounding = style->FrameRounding;

    ImFontAtlas_AddFontFromFileTTF(io->Fonts, "fonts/Inter-Regular.ttf", 20, NULL, NULL);

    void *libplug = dlopen("target/release/libplug.so", RTLD_NOW);

    plug_state_init_handle plug_state_init = dlsym(libplug, "plug_state_init");
    plug_state_free_handle plug_state_free = dlsym(libplug, "plug_state_free");
    plug_init_handle plug_init = dlsym(libplug, "plug_init");
    plug_update_handle plug_update = dlsym(libplug, "plug_update");
    plug_free_handle plug_free = dlsym(libplug, "plug_free");

    PlugState *state = plug_state_init();

    plug_init(state);

    bool has_reloaded = false;

    // main event loop
    bool quit = false;
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();

        if (glfwGetKey(window, GLFW_KEY_R) == GLFW_PRESS &&
            (glfwGetKey(window, GLFW_KEY_LEFT_CONTROL) == GLFW_PRESS || glfwGetKey(window, GLFW_KEY_RIGHT_CONTROL) == GLFW_PRESS)) {
            // Ctrl+R is pressed
            if (!has_reloaded) {
                plug_free(state);
                dlclose(libplug);

                libplug = dlopen("target/release/libplug.so", RTLD_NOW);

                plug_state_init = dlsym(libplug, "plug_state_init");
                plug_state_free = dlsym(libplug, "plug_state_free");
                plug_init = dlsym(libplug, "plug_init");
                plug_update = dlsym(libplug, "plug_update");
                plug_free = dlsym(libplug, "plug_free");

                if (glfwGetKey(window, GLFW_KEY_LEFT_SHIFT) || glfwGetKey(window, GLFW_KEY_RIGHT_SHIFT)) {
                    plug_state_free(state);
                    state = plug_state_init();
                }

                plug_init(state);

                has_reloaded = true;
            }
        } else {
            has_reloaded = false;
        }

        // start imgui frame
        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplGlfw_NewFrame();
        igNewFrame();

        plug_update(state);

        // render
        igRender();
        glfwMakeContextCurrent(window);
        glViewport(0, 0, (int)io->DisplaySize.x, (int)io->DisplaySize.y);
        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);
        ImGui_ImplOpenGL3_RenderDrawData(igGetDrawData());
        glfwSwapBuffers(window);
    }

    // clean up
    ImGui_ImplOpenGL3_Shutdown();
    ImGui_ImplGlfw_Shutdown();
    igDestroyContext(NULL);

    glfwDestroyWindow(window);
    glfwTerminate();

    plug_free(state);
    plug_state_free(state);
    dlclose(libplug);

    return 0;
}
