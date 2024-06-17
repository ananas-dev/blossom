# Each tournament has a SQLite database

```c++
struct Player {
    char *name;
    Federation federation;
    Sex sex;
    Title title;
    int fide_rating;
    int fide_id;
};
```

```sql
CREATE TABLE players (
    id PRIMARY KEY,
    federation INT,
    sex INT,
    title INT,
    fide_rating INT,
    fide_id INT
);
```

when update