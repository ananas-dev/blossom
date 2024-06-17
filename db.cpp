#include "db.h"

#include <sqlite3.h>

namespace Db {

int worker(const char *path) {
    sqlite3 *db;

    if (sqlite3_open(path, &db) != SQLITE_OK) {
        return -1;
    }

    sqlite3_stmt *add_player_stmt = nullptr;
    sqlite3_stmt *remove_player_stmt = nullptr;

    int cmd = CMD_ADD_PLAYER;

    switch (cmd) {
        case CMD_ADD_PLAYER: {
            if (add_player_stmt == nullptr) {
                sqlite3_prepare_v2(db, "INSERT INTO players(name, federation, sex, title, fide_rating, fide_id) VALUES(?1, ?2, ?3, ?4, ?5, ?6)", -1, &add_player_stmt, nullptr);
            }

            sqlite3_bind_text16(add_player_stmt, 1, "", -1, nullptr);
            sqlite3_bind_int(add_player_stmt, 2, 0);
            sqlite3_bind_int(add_player_stmt, 3, 0);
            sqlite3_bind_int(add_player_stmt, 4, 0);
            sqlite3_bind_int(add_player_stmt, 5, 0);
            sqlite3_bind_int(add_player_stmt, 6, 0);

            sqlite3_reset(add_player_stmt);
        } break;
        case 2: {
        }
    }

    if (sqlite3_close(db) != SQLITE_OK) {
        // Figure out something
    }

    return 0;
}

}
