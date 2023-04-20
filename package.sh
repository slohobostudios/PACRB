if [ $1 == "--linux" ];
then
  mkdir release
  rm -rf release/linux
  mkdir release/linux
  cp -r assets/ release/linux/
  cp -r deps release/linux/ 
  cp target/release/pacrb release/linux/
  zip -r linux_release.zip release/linux
fi

if [ $1 == "--windows" ];
then
echo "Incomplete";
fi
