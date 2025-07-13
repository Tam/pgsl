# pgsl
**P**ost**g**res **S**chema **L**anguage: a superscript language of PostgreSQL

Removes a lot of the guff in postgres files, making schemas easier to write and 
understand at a glance.

Will come with its own migrator so all you have to do is write the schema. The 
migrator will check the structure of the database and only migrate any changes.
It will also warn about destructive actions (like dropping columns or enums).

### Testing

```shell
cargo run -- sample.pgl --debug
```

### Sample

```text
# interfaces/archive.pgl

interface table archive:
  columns:
    is_archived boolean default false
      Whether or not this item is archived (soft-deleted)
      @omit create
    archived_at timestamptz
      When this item was archived
      @omit create,update
    restored_at timestamptz
      When this item was restored
      @omit create,update

trigger before insert or update on archive:
  begin:
    if old.is_archived = false and new.is_archived = true then
      new.archived_at = now();
    elsif old.is_archived = true and new.is_archived = false then
      new.restored_at = now();
    end if;

    return new;
  end plpgsql volatile

# character.pgl

require:
  interfaces/archive
  interfaces/timestamp
  core
  assets
  game

schema character:
  grant usage to anonymous, member

table public.character:
  extends:
    archive
    timestamp
  columns:
    id          uuid primary key default public.uuid_generate_v1mc()
    owner_id    uuid references public.user(id) on delete cascade
                @omit
                @description Hello world!
    game_type   text references game.type not null
    name        varchar(256) not null
    token_id    uuid references public.asset(id) on delete set null

function character.example_func:
  accept:
    id uuid
    some_value text
  declare:
    another_value boolean
  begin:
    select * from test;
  end sql stable
```

### Notes-to-self on migrations
- There should be a clear order of operations:
  1. Creating
     1. Types
     2. Tables
     3. Columns
     4. Indexes / Foreign Keys
     5. Views
     6. Functions
     7. RLS
     8. Triggers
  2. Updating
     1. Column definitions
     2. Views
     3. Functions
     4. RLS
     5. Triggers
  3. Deleting
     - The same items as creating, but in reverse order
- Some operations above can happen out of order if they are needed for others 
  to complete. For example, we might need to add a new column before creating a
  table if that table references that new column (although this could probably 
  be handled in the foreign keys step?)

### Notes-to-self on Custom Language Features
There are some features in this language that differ from Postgres' standard.
The main one being interfaces. Interfaces are similar to built-in postgres 
inheritance, but instead of the inherited data being stored in a separate table,
we add the columns, triggers, etc. to the table being extended.

### TODO
- We need a way of running additional custom migration logic for, example, 
  moving data from an old column to a new column. Perhaps something like:
  ```diff
  interface table archive:
    columns:
  -    is_archived boolean default false
  -      Whether or not this item is archived (soft-deleted)
  -      @omit create
  +    is_public boolean default true
  +      comment:
  +        Whether this item is public
  +        @behaviour -insert -update
  +      migrate:
  +        update [table] set is_public = not is_archived
      archived_at timestamptz
        When this item was archived
        @omit create,update
      restored_at timestamptz
        When this item was restored
        @omit create,updat
  ```
  In the example above we're changing an interface, so we use the `[table]`
  placeholder that should be automatically replaced with the actual table name.
  `migrate:` should support anything available in the standard `do $$` including
  declaring variables. Here we're just using simple SQL.
