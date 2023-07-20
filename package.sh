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
  rm -rf release/pacrb_l64
  mkdir release/pacrb_l64
  cp -r assets/ release/pacrb_l64/
  mkdir release/pacrb_l64/deps
  cp -r deps/linux/* release/pacrb_l64/deps/
  cp -r files release/pacrb_l64/
  cp target/release/pacrb release/pacrb_l64/
  cd release
  tar -czvf pacrb_l64_release.tar.gz pacrb_l64/
  cd ..
  mv release/pacrb_l64_release.tar.gz .
fi

if [ $1 == "--windows" ];
then
  mkdir release
  rm -rf release/pacrb_w64
  mkdir release/pacrb_w64
  cp -r assets/ release/pacrb_w64/
  cp -r deps/windows/* release/pacrb_w64/
  cp -r files release/pacrb_w64/
  cp target/release/pacrb release/pacrb_w64/
  cd release
  tar -czvf windows_release.tar.gz pacrb_w64/
  cd ..
  mv release/windows_release.tar.gz .
fi
