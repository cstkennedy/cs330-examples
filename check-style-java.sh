CHECKSTYLE_JAR=/home/tkennedy/Courses/bin/checkstyle-8.8-all.jar


for i in 04 07 09 10 11; do
    java -jar ${CHECKSTYLE_JAR} -c checkstyle.xml Review-$i* 2>&1 | sort > build/$i-style-check-java.txt
done
