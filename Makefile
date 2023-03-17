INSTALL = install
INSTALL_PROGRAM = ${INSTALL} -D -m 0755
INSTALL_DATA = ${INSTALL} -D -m 0644

prefix = /usr
exec_prefix = $(prefix)
bindir = $(exec_prefix)/bin
datarootdir = $(prefix)/share

app = boiling_frog
qualified_app = "com.robwilliamson.$(app)"
data_folder = "./boiling_frog/data"
icon_name = "$(qualified_app).png"
icon_source = "$(data_folder)/$(icon_name)"
scaled_icons = "./scaled_icons"
icon_scales = 128x128 16x16 192x192 24x24 256x256 32x32 36x36 48x48 512x512 64x64 72x72 96x96

app_destination = "$(DESTDIR)$(bindir)/$(app)"
desktop_destination = "$(DESTDIR)$(datarootdir)/applications/$(qualified_app).desktop"
icon_destination = "$(DESTDIR)$(datarootdir)/icons/hicolor"

all: build

clean:
	cargo clean

build:
	@echo "extra args: [ $(ARGS) ]"
	@echo "version: $(VERSION)"
	cargo build
	cargo build --release

install: do_install refresh
do_install:
	@echo "DESTDIR = $(DESTDIR)"
	@echo "prefix = $(prefix)"
	@echo "exec prefix = $(exec_prefix)"
	$(INSTALL_PROGRAM) "./target/release/$(app)" "$(app_destination)"
	$(INSTALL_DATA) "$(data_folder)/$(qualified_app).desktop" "$(desktop_destination)"
	for i in $(icon_scales); do $(INSTALL_DATA) "$(scaled_icons)/$(qualified_app)_$${i}.png" "$(icon_destination)/$${i}/apps/$(icon_name)" ; done
	$(INSTALL_DATA) "./boiling_frog/data/$(qualified_app).png" "$(icon_destination)"

uninstall: do_uninstall refresh
do_uninstall:
	rm -f "$(app_destination)" || true
	rm -f "$(desktop_destination)" || true
	for i in $(icon_scales); do rm "$(icon_destination)/$${i}/apps/$(icon_name)" ||: ; done

reinstall: do_uninstall install

refresh:
	gtk-update-icon-cache
	update-desktop-database

#Regenerate the icons. Requires imagemagick to be installed.
icons:
	@mkdir -p "$(scaled_icons)"
	for i in $(icon_scales); do convert "$(icon_source)" -resize $$i "$(scaled_icons)/$(qualified_app)_$${i}.png"; done
