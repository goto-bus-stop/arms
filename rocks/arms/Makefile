SOURCE_DIR = src
LIB_DIR = lib
SOURCE_FILES = $(wildcard $(SOURCE_DIR)/*.moon)
LIB_FILES = $(patsubst $(SOURCE_DIR)/%.moon,$(LIB_DIR)/%.lua,$(SOURCE_FILES))
OUTPUT = arms.out

$(LIB_DIR):
	mkdir $(LIB_DIR)

$(LIB_DIR)/%.lua: $(SOURCE_DIR)/%.moon $(LIB_DIR)
	moonc -p $< > $@

$(OUTPUT): $(LIB_FILES)
	luac -o "$(OUTPUT)" $(LIB_FILES)

compile: $(OUTPUT)