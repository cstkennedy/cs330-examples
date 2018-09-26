#! /bin/bash

baseDirectory="./"
zipDirectory="build/"


# echo "$zipDirectory"

if [ ! -d $zipDirectory ]
then
    mkdir -p $zipDirectory
fi


for reviewDir in Review-*
do
    if [ -f "${reviewDir}/deploy.sh" ]
    then
        pushd $reviewDir
        chmod u+x deploy.sh
        ./deploy.sh
        popd
    fi
done

for reviewDir in Review-*
do
    if [ -f "$reviewDir/${reviewDir}.zip" ]
    then
        mv "${reviewDir}/${reviewDir}.zip" build/
    fi
done
