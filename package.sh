if [ -z "$1" ]
then
  echo "No argument supplied. Valied arguments:"
  echo "--linux"
  echo "--windows"
  exit
fi

if [ $1 == "--linux" ];
then
  mkdir release
  rm -rf release/linux
  mkdir release/linux
  cp -r assets/ release/linux/
  mkdir release/linux/deps
  cp -r deps/linux/* release/linux/deps/ 
  cp target/release/pacrb release/linux/
  tar -czvf linux_release.tar.gz release/linux/
fi

if [ $1 == "--windows" ];
then
  mkdir release
  rm -rf release/windows
  mkdir release/windows
  cp -r assets/ release/windows/
  cp -r deps/windows/* release/windows/ 
  cp target/release/pacrb release/windows/
  tar -czvf windows_release.tar.gz release/windows/
fi
