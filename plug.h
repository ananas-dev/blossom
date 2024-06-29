#ifndef PLUG_H
#define PLUG_H

typedef struct PlugState PlugState;

typedef PlugState* (*plug_state_init_handle)();
typedef void (*plug_state_free_handle)(PlugState*);
typedef void (*plug_init_handle)(PlugState*);
typedef void (*plug_update_handle)(PlugState*);
typedef void (*plug_free_handle)(PlugState*);

#endif // PLUG_H
