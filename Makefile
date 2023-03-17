INSTALL = install
INSTALL_PROGRAM = ${INSTALL} -D -m 0755
INSTALL_DATA = ${INSTALL} -D -m 0644

prefix = /usr
exec_prefix = $(prefix)
bindir = $(exec_prefix)/bin
datarootdir = $(prefix)/share

app = boiling_frog
qualified_app = "com.robwilliamson.$(app)"

app_destination = "$(DESTDIR)$(bindir)/$(app)"
desktop_destination = "$(DESTDIR)$(datarootdir)/applications/$(qualified_app).desktop"
icon_destination = "$(DESTDIR)$(datarootdir)/icons/$(qualified_app).png"

all: build

clean:
	cargo clean

build:
	@echo "extra args: [ $(ARGS) ]"
	@echo "version: $(VERSION)"
	cargo build
	cargo build --release

install:
	@echo "DESTDIR = $(DESTDIR)"
	@echo "prefix = $(prefix)"
	@echo "exec prefix = $(exec_prefix)"
	$(INSTALL_PROGRAM) "./target/release/$(app)" "$(app_destination)"
	$(INSTALL_DATA) "./boiling_frog/data/$(qualified_app).desktop" "$(desktop_destination)"
	$(INSTALL_DATA) "./boiling_frog/data/$(qualified_app).png" "$(icon_destination)"

uninstall:
	rm -f "$(app_destination)" || true
	rm -f "$(desktop_destination)" || true
	rm -r "$(icon_destination)" || true

reinstall: uninstall install
