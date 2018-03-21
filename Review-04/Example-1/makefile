all: src docs

deploy:
	zip -r deploy.zip source documentation* input tests

src:
	make -C source

docs:
	doxygen documentation.config

clean:
	make -C source clean


	
