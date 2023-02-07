all: dark.min.css light.min.css

.PRECIOUS: dark.css light.css

%.min.css: %.css
	css-minifier -i $< -o $@

ALL_SASS := $(shell find . -name "*.scss" -o -name "*.sass")

%.css: %.sass $(ALL_SASS)
	grass $< $@

clean:
	rm dark.css dark.min.css light.css light.min.css
