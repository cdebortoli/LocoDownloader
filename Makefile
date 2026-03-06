VERSION := 1.0.0
DIST    := dist

.PHONY: package clean

package:
	cargo build --release
	mkdir -p $(DIST)
	cp target/release/loco install/loco
	tar czf $(DIST)/loco-$(VERSION)-macos-arm.tar.gz -C install loco install.sh -C .. skills
	rm install/loco
	@echo "Created $(DIST)/loco-$(VERSION)-macos-arm.tar.gz"

clean:
	rm -rf $(DIST) install/loco
