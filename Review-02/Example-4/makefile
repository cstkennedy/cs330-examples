all: src docs

deploy: docs clean
	zip -r deploy.zip source documentation* input tests

src:
	make -C source

docs:
	doxygen documentation.config

clean:
	make -C source clean


	