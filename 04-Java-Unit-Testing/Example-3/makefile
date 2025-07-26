all: deploy

deploy: clean
	echo "Gradle Based Example"

src:
	make -C source

docs:
	doxygen documentation.config

clean:
	./gradlew --no-daemon clean cleanEclipse
	rm -rf .gradle


	
