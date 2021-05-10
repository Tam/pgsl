# pregres
A superscript language of PostgreSQL

Will come with its own migrator so all you have to do is write the schema. The 
migrator will check the structure of the database and only migrate any changes.
It will also warn about destructive actions (like dropping columns or enums).

```text
# interfaces/archive.pgl

interface archive:
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
  begin
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
  ~archive
  ~timestamp
  id          uuid primary key default public.uuid_generate_v1mc()
  owner_id    uuid references public.user(id) on delete cascade
              @omit
              @description Hello world!
  game_type   text references game.type not null
  name        varchar(256) not null
  token_id    uuid references public.asset(id) on delete set null

function character.example_func:
  args
    id uuid
    some_value text
  declare
    another_value boolean
  begin
    select * from test;
  end sql stable
```
