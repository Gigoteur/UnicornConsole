buildtype = release

PROJECT = px8
TARGET = asmjs-unknown-emscripten

DOCS_DIR = docs
DOCS_PORT = 8080

JS_FILE = $(PROJECT).js

CARGO_OUTDIR = target/$(TARGET)/$(buildtype)

CARGO_OPTION = --target $(TARGET) --verbose
EMCC_OPTION = -s USE_SDL=2

ifeq ($(buildtype),release)
CARGO_OPTION += --release
EMCC_OPTION += -O3

else ifeq ($(buildtype),debug)
CARGO_OPTION +=
EMCC_OPTION += -g4
DOCS_FILES = $(DOCS_DIR)/$(JS_FILE)

else
$(error "unknown buildtype")
endif

all: $(DOCS_DIR)/$(JS_FILE)
.PHONY: all

clean:
	cargo clean
	$(RM) $(DOCS_DIR)/*.js $(DOCS_DIR)/*.js.mem
.PHONY: clean

serve: all
	ruby -run -e httpd $(DOCS_DIR) -p $(DOCS_PORT)

FORCE:
.PHONY: FORCE

$(CARGO_OUTDIR)/$(JS_FILE): FORCE
	$(RM) $(DOCS_DIR)/*.js $(DOCS_DIR)/*.js.mem
	EMMAKEN_CFLAGS="$(EMCC_OPTION)" cargo build $(CARGO_OPTION)

$(DOCS_DIR)/$(JS_FILE): $(CARGO_OUTDIR)/$(JS_FILE) FORCE
	find $(CARGO_OUTDIR) \( -name '*.js' -or -name '*.js.mem' \) -exec cp {} $(DOCS_DIR) \;