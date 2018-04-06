#! /bin/bash

workingDirectory="`pwd | sed 's/.*\///g'`"
zipFile="`echo $workingDirectory | sed 's/ /\-/g'`".zip

# echo "$workingDirectory"
# echo $zipFile

if [ -f $zipFile ]
then
    rm $zipFile
fi 

for exampleDir in *
do
    if [ -d "$exampleDir" ]
    then
        pushd "$exampleDir" > /dev/null

        echo "$exampleDir -> Cleaning *.o, *.class, and binary files"
        make clean &> /dev/null
        # echo "$exampleDir -> Building Doxygen Documentation"
        # make docs &> /dev/null

        echo
        popd > /dev/null
    fi
done

echo "Creating $zipFile"
zip -r -9 "$zipFile" * -x */makefile -x deploy.sh > /dev/null
