mkdir -p SiteBuilder.app/Contents/{MacOS,Resources}
echo Copying plist
cp assets/Info.plist SiteBuilder.app/Contents/Info.plist
echo copying icon into .app
cp assets/SiteBuilder.icns SiteBuilder.app/Contents/Resources/SiteBuilder.icns
echo copying bin to .app
cp target/release/site-builder SiteBuilder.app/Contents/MacOS/SiteBuilder
echo zipping up .app
tar -czvf SiteBuilder.tar.gz SiteBuilder.app