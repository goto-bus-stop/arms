LUA = lua5.3
LUAC = luac5.3

SOURCE_DIR = src
LIB_DIR = lib
SOURCE_FILES = $(wildcard $(SOURCE_DIR)/*.moon)
DEP_FILES = json.lua
LIB_FILES = $(patsubst $(SOURCE_DIR)/%.moon,$(LIB_DIR)/%.lua,$(SOURCE_FILES)) \
	$(DEP_FILES:%.lua=lib/%.lua)

OUTPUT = arms.out

all: $(OUTPUT)

clean:
	rm -rf $(LIB_DIR)

$(LIB_DIR):
	mkdir $(LIB_DIR)

# Compile and wrap Arms source

$(LIB_DIR)/%.lua: $(SOURCE_DIR)/%.moon $(LIB_DIR)
	moonc -p "$<" | $(LUA) ./wrap.lua "arms.$(@F:%.lua=%)" > "$@"

# Wrap dependencies

$(LIB_DIR)/json.lua: deps/json.lua
	$(LUA) ./wrap.lua json < "$<" > "$@"

# Bundle compiled files

$(OUTPUT): $(LIB_FILES)
	$(LUAC) -o "$(OUTPUT)" $(LIB_FILES)
