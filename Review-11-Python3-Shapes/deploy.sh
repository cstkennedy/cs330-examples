#! /bin/bash

workingDirectory="`pwd | sed 's/.*\///g'`"
zipFile="`echo $workingDirectory | sed 's/ /\-/g'`".zip

# echo "$workingDirectory"
# echo $zipFile

if [ -f $zipFile ]
then
    rm $zipFile
fi 

find . -name \*.pyc -exec rm "{}" \;

echo "Creating $zipFile"
zip -r -9 "$zipFile" * -x */makefile -x deploy.sh > /dev/null
