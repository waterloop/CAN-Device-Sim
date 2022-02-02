# Virtual CAN Device
VCD is a tool for developing devices on a CAN bus. It can be configured to transmit can frames with a specific ID and message on a periodic basis.
It can also be configured to respond to specfic messages. These responses can be set to wait a minimum amount of time before sending their response
as well.

## NOTE
You will need cargo installed on your device. Checkout the [Offical Rust Website](https://www.rust-lang.org/tools/install) for tips on installing.

## QuickStart
1.  Run the `scripts/setup.sh` script. This will add any folders that are required for storing things like
configuration files which we otherwise do not want in the git repo.

2.  Get some configuration files. You can find these in the test plans for the device that you are testing or you can create some using
the `web` package. To run the web package, run `scripts/web.sh`. Instructions should appear in the console telling you what address to go to in your browser.
Once in the web UI, you should be able to create config files which will be saved to the configs folder. If you are using a config file from a test plan, simply save it to the configs folder and continue to the next step.

3. Run the CLI. `cargo run` will launch the cli. If this is your first time running, you will need to be connected to the internet for cargo to download and build dependencies.
The Cli should prompt you to choose a config from a list of configs that you have in your configs folder. After choosing the config, open another termial and run `candump vcan0`. If using a real
can network, aptly replace vcan0 with can0. In either case, you should see the appropriate messages being sent from any producers being sent. To test responders, send a can message to their Hook
id and watch for the response id to appear.
