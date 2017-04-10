all: src docs

deploy: clean
	echo "Gradle Based Example"

src:
	make -C source

docs:
	doxygen documentation.config

clean:
	./gradlew --no-daemon clean
	rm -rf .gradle


	