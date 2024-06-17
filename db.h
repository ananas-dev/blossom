#pragma once

namespace Db {

enum CommandType {
    CMD_ADD_PLAYER
};

struct AddPlayer {
    int federation;
    int sex;
    int title;
    int fide_elo;
    int file_id;
    char name[]; // pending struct
};

struct Command {
    CommandType type;
    void *data;
};

int worker(const char *path);

}