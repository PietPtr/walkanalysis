set -xe

killall ArdourGUI || true
cargo xtask bundle wa_plugin --release
cp ../target/bundled/wa_plugin.vst3/  ~/.vst3 -r 
ardour ~/Music/Ardour/PluginTest/PluginTest.ardour &
read -p "kill ardour?"
killall ArdourGUI
ardour ~/Music/Ardour/PluginTest/PluginTest.ardour &
read -p "kill ardour?"
killall ArdourGUI
