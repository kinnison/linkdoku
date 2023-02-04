all: dark.min.css light.min.css

.PRECIOUS: dark.css light.css

%.min.css: %.css
	css-minifier -i $< -o $@

%.css: %.sass mixins.sass linkdoku-bits.sass
	grass $< $@

clean:
	rm dark.css dark.min.css light.css light.min.css
