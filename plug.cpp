#include "plug.h"
#include <cassert>
#include <cstddef>
#include <stdio.h>
#include "cimgui/imgui/imgui.h"
#include "sfd.h"

extern "C" {

void plug_init(PlugState *state) {
    printf("[PLUGIN] Init\n");
}

void plug_update(PlugState *state) {
    ImGui::DockSpaceOverViewport();

    ImGui::BeginMainMenuBar();

    if (ImGui::BeginMenu("Tournament")) {
        if (ImGui::MenuItem("Create")) {
            state->show_create_tournament_window = true;
        }
        ImGui::EndMenu();
    };

    if (ImGui::BeginMenu("Theme")) {
        if (ImGui::MenuItem("Light")) {
            ImGui::StyleColorsLight();
        }

        if (ImGui::MenuItem("Dark")) {
            ImGui::StyleColorsDark();
        };

        if (ImGui::MenuItem("Classic")) {
            ImGui::StyleColorsClassic();
        };

        ImGui::EndMenu();
    };

    ImGui::EndMainMenuBar();

    {
        ImGui::Begin("Control", NULL, ImGuiWindowFlags_NoCollapse);                          // Create a window called "Hello, world!" and append into it.

        if (ImGui::Button("Add player")) {
            char *name = (char*)malloc(sizeof(char) * 32);
            name[0] = '\0';
            state->players.push_back(Player{name, FED_NONE, SEX_NONE, TITLE_NONE, 0, 9999});
        }

        ImGui::SameLine();

        if (ImGui::Button("Remove player")) {
            free(state->players[state->players.size() - 1].name);
            state->players.pop_back();
        }

        if (ImGui::Button("Button"))                            // Buttons return true when clicked (most widgets return true when edited/activated)
            state->counter++;

        ImGui::SameLine();
        ImGui::Text("counter = %d", state->counter);

        ImGui::End();
    }

    if (state->show_add_player_window) {
        ImGui::End();
    }

    ImGui::ShowDemoWindow();

    // Player page

    {
        ImGui::Begin("Players");
        ImGuiTableFlags table_flags = ImGuiTableFlags_SizingStretchSame
                                    | ImGuiTableFlags_BordersH
                                    | ImGuiTableFlags_BordersV;

        if (ImGui::BeginTable("tournament_players", 7, table_flags)) {
            ImGui::TableSetupColumn("ID");
            ImGui::TableSetupColumn("Name");
            ImGui::TableSetupColumn("Fed");
            ImGui::TableSetupColumn("Sex");
            ImGui::TableSetupColumn("Title");
            ImGui::TableSetupColumn("ID FIDE");
            ImGui::TableSetupColumn("Rtg FIDE");
            ImGui::TableHeadersRow();

            for (size_t row = 0; row < state->players.size(); row++) {
                Player &player = state->players[row];
                ImGui::TableNextRow();

                ImGui::TableSetColumnIndex(0);
                ImGui::Text("%ld", row);

                ImGui::PushStyleColor(ImGuiCol_FrameBg, ImVec4(0.0f, 0.0f, 0.0f, 0.0f));
                ImGui::TableSetColumnIndex(1);
                ImGui::PushItemWidth(-1);
                ImGui::PushID(11 * row + 1);
                ImGui::InputText("##name", player.name, 32);
                ImGui::PopID();
                ImGui::PopItemWidth();
                ImGui::PopStyleColor();

                ImGui::TableSetColumnIndex(2);
                ImGui::PushItemWidth(-1);
                ImGui::PushID(11 * row + 4);
                ImGui::Combo("##fed", (int*)&player.federation, "-\0BE\0FR\0US\0");
                ImGui::PopID();
                ImGui::PopItemWidth();

                ImGui::TableSetColumnIndex(3);
                ImGui::PushItemWidth(-1);
                ImGui::PushID(11 * row + 3);
                ImGui::Combo("##sex", (int*)&player.sex, "-\0M\0F\0");
                ImGui::PopID();
                ImGui::PopItemWidth();

                ImGui::TableSetColumnIndex(4);
                ImGui::PushItemWidth(-1);
                ImGui::PushID(11 * row + 4);
                ImGui::Combo("##title", (int*)&player.title, "-\0GM\0IM\0FM\0CM\0");
                ImGui::PopID();
                ImGui::PopItemWidth();

                ImGui::PushStyleColor(ImGuiCol_FrameBg, ImVec4(0.0f, 0.0f, 0.0f, 0.0f));
                ImGui::TableSetColumnIndex(5);
                ImGui::PushItemWidth(-1);
                ImGui::PushID(11 * row + 6);
                ImGui::InputInt("##fide_id", &player.fide_id, 0);
                ImGui::PopID();
                ImGui::PopItemWidth();

                ImGui::TableSetColumnIndex(6);
                ImGui::PushItemWidth(-1);
                ImGui::PushID(11 * row + 6);
                ImGui::InputInt("##fide_rating", &player.fide_rating, 0);
                ImGui::PopID();
                ImGui::PopItemWidth();
                ImGui::PopStyleColor();
            }
            ImGui::EndTable();
        }

        ImGui::End();

    }

    if (state->show_create_tournament_window) {
        ImVec2 center = ImGui::GetMainViewport()->GetCenter();
        ImGui::SetNextWindowPos(center, ImGuiCond_Appearing, ImVec2(0.5f, 0.5f));

        ImGui::PushStyleVar(ImGuiStyleVar_WindowBorderSize, 1.0f);
        if (ImGui::Begin("Create tournament", nullptr, ImGuiWindowFlags_NoDocking | ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_AlwaysAutoResize)) {
            static Tournament tournament;

            if (tournament.name == nullptr) {
                tournament.name = (char*)malloc(30 * sizeof(char));
                assert(tournament.name != NULL);
                tournament.name[0] = '\0';
            }

            if (tournament.federation == nullptr) {
                tournament.federation = (char*)malloc(4 * sizeof(char));
                assert(tournament.federation != NULL);
                tournament.federation[0] = '\0';
            }

            ImGui::InputText("Name", tournament.name, 30);
            ImGui::InputText("Federation", tournament.federation, 4, ImGuiInputTextFlags_CharsUppercase);

            if (ImGui::Button("Open File")) {
                sfd_Options opt = {
                  .title        = "Open Image File",
                  .filter_name  = "Image File",
                  .filter       = "*.png|*.jpg",
                };

                const char *filename = sfd_open_dialog(&opt);

                if (filename) {
                  printf("Got file: '%s'\n", filename);
                } else {
                  printf("Open canceled\n");
                }

                printf("%s\n", sfd_get_error());
            }

            ImGui::Spacing();

            if (ImGui::Button("Create tournament")) {
                state->show_create_tournament_window = false;
            }
            ImGui::End();
        }
        ImGui::PopStyleVar();
    }

}

void plug_free(PlugState *state) {
    printf("[PLUGIN] Free\n");
}

}
