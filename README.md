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

3.1 Run with arguments. To run with arguments type `cargo run -- ` followed by the arguments that you would like to run. For example, to see a list of available options, type `cargo run -- --help`

## Getting Up and Running with Virtual Can development
Inorder for this program to work, you need to be connected to a canbus. The most common one for testing is vcan0. The following instructions assumes that you are operating in the virtual env created by the [firmware vagrant repo](https://github.com/waterloop/firmware-vagrant).

1. To start the vcan0 on your network, run: `./scripts/start-vcan0.sh`
2. You can install ifconfig by following [this](https://www.how2shout.com/linux/install-ifconfigon-debian-11-or-10-if-command-not-found/) tutorial.
3. Check that it worked, run: `ifconfig`. If you see a network called vcan0, then it worked, if you don't then you will need to debug at this point.
4. Install can utils `sudo apt update && sudo apt install can-utils`
5. Run the virtual can device simulator
6. check that data is being sent on the canbus (you may need to open another terminal): `candump vcan0`



3.1 Run with arguments. To run with arguments type `cargo run -- ` followed by the arguments that you would like to run. For example, to see a list of available options, type `cargo run -- --help`

## More info on Running the CLI
```sh
$ cargo run -- --help
USAGE:
    cargo run -- [OPTIONS]

OPTIONS:
        --can-interface <CAN_INTERFACE>    [default: vcan0]
        --config-folder <CONFIG_FOLDER>    [default: ./configs]
    -h, --help                             Print help information
    -V, --version                          Print version information
```

### Choosing the config-folder
The default config folder points to the folder which stores the configs created by the web UI. These configs are not checked into the git repository. For access to the ones checked into git, use the `--config-folder ./example-configs`.

### Running in Vagrant
For running the CAN-Device-Sim in our [firmware vagrant](https://github.com/waterloop/firmware-vagrant) repo, you will probably want to run on the virtual can interface or `vcan0`. To do this, you can simply run the `scripts/start-vcan0.sh` script and it will setup the vcan interface for you. You can check that it worked by running `ifconfig | grep vcan0 -A 5`. You should get a result that matches the following:

```sh
vcan0: flags=193<UP,RUNNING,NOARP>  mtu 72
        unspec 00-00-00-00-00-00-00-00-00-00-00-00-00-00-00-00  txqueuelen 1000  (UNSPEC)
        RX packets 0  bytes 0 (0.0 B)
        RX errors 0  dropped 0  overruns 0  frame 0
        TX packets 0  bytes 0 (0.0 B)
        TX errors 0  dropped 0 overruns 0  carrier 0  collisions 0
```

If you do not see this and the command returns nothing, then there is something wrong. If you're comfortable, we suggest that you try to debug it yourself, but if you have issues and are not sure where to begin, reach out to a lead and they will be able to help you out. For those new to command line linux and would like to know what that command actually does, checkout [#Helpful Commands]

Since `vcan0` is the default can interface, you should not need to use the `--can-interface` option when running in vagrant.

#### Common Command for running in vagrant
`cargo -- --config-folder ./example-configs` to run with one of the example configs on the virtual can network.

### Running on a Raspberry Pi
This is assuming the raspberry pi is already connected to a CAN shield and configured for using the CAN interface. If its not, then you should check with your lead.
To run the can-device-sim on the actual can network, you will need to add the `--can-interface can0` option when running the simulator.

#### Common Command for running on the pi
`cargo -- --can-interface can0 --config-folder ./example-configs` to run with one of the example configs on the physical can network.

### Current example configs
#### G5 compatible
The following Config files are compatible with the G5 CAN Communication Protocol
- example-configs/HIL-Testing No-Faults All-devices.json:

#### G6 Compatible
There are currently no config files which are compatible with the G6 CAN Communication Protocol.

## Helpful Commands
### `ifconfig | grep vcan0 -A 5`
ifconfig is a useful tool for debugging network related issues. It provides basic information about each of the networks interfaces which are open on your current device. The `|` character is called a "pipe". It's a special command line character which takes the output of the program on the left, and provides it as input or "pipes it" into the program on the right. In this case, it takes the full report of all network interfaces and pipes it into the `grep` program. Grep stands for Global Regular Expression Print. It is a very useful tool for parsing the outputs of commands when you are looking for something specific. The general way to use grep is to take a program with a large console output, pipe it into grep, and then as the first argument to grep, you provide a regular expression that it will try to match. grep will then print out all of the lines which match the regular expression provided. In this case, the regular expression is the string vcan0. For more info on regular expressions, check out the [wiki](https://en.wikipedia.org/wiki/Regular_expression) page on them. Lastly, the argument `-A 5` prints the 5 lines following each match of the regular expression. This is why you get the extra lines about the Rx(received) and Tx(transmitted) packets and errors. For context, there are also arguments which allow you to print n lines before using `-B <number of lines>` and you can print the "context" of the line which is n lines before and n lines after using the `-C <number of lines>` argument.
