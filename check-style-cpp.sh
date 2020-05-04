for i in 01 02 03 04 05 06 07; do
    cpplint --recursive Review-$i* 2>&1 > /dev/null | sort > build/$i-style-check-c++.txt
    cppcheck --enable=all Review-$i* 2>&1 > /dev/null | sort > build/$i-style-cppcheck-c++.txt
done
