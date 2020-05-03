.PHONY: build-release install package-arch

build-release:
	cargo build --release

install: build-release
	# this process allows existing processes to continue while installing
	# a new version for all subsequent callers
	cp target/release/cinderella /opt/cinderella/.cinderella
	# use -f on deletion to avoid errors if file does not exist
	rm -f /opt/cinderella/cinderella
	mv /opt/cinderella/.cinderella /opt/cinderella/cinderella

package-arch:
	# Unfortunately, PKGBUILD has to be in the same directory as the build
	cp packages/arch/PKGBUILD .
	# Ensure that makepkg does not overwrite src/ folder
	# -e: Do not try to extract any source, because we use the source in
	# current directory
	SRCDEST=src_makepkg makepkg -e
	rm PKGBUILD
	rm -r src_makepkg pkg
