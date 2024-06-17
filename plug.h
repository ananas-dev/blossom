#pragma once

#include <vector>
#include <string>
#include <sqlite3.h>
#include <thread>

enum Title: int {
    TITLE_NONE,
    TITLE_GM,
    TITLE_IM,
    TITLE_FM,
    TITLE_CM,
};

enum Sex: int {
    SEX_NONE,
    SEX_M,
    SEX_F,
};

enum Federation: int {
    FED_NONE,
    FED_BE,
    FED_FR,
    FED_US,
    // todo
};

struct Action {
    void *data;
    void *pointer;
    void *size;
};

struct Player {
    char *name;
    Federation federation;
    Sex sex;
    Title title;
    int fide_rating;
    int fide_id;
};

struct PlugState {
    int counter;
    bool show_add_player_window;
    int num_players;
    sqlite3 *db_handle;
    std::vector<Player> players;
};

typedef void (*plug_handle)(PlugState*);