all: src docs

deploy: clean
	echo "Gradle Based Example"
	./gradlew deploy

src:
	make -C source

docs:
	# doxygen documentation.config
	echo "Gradle Based Example"
	./gradlew --no-daemon doxygen javadoc
	rm -rf .gradle


clean:
	./gradlew --no-daemon clean cleanEclipse
	rm -rf .gradle



