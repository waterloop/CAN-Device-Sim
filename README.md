# Virtual CAN Device
VCD is a tool for developing devices on a CAN bus. It can be configured to transmit can frames with a specific ID and message on a periodic basis.
It can also be configured to respond to specfic messages. These responses can be set to wait a minimum amount of time before sending their response
as well.

## QuickStart
1.  Run the `scripts/setup.sh` script. This will add any folders that are required for storing things like
configuration files which we otherwise do not want in the git repo.

2.  Get some configuration files. You can find these in the test plans for the device that you are testing or you can create some using
the `web` package. To run the web package, run `scripts/web.sh`. Instructions should appear in the console telling you what address to go to in your browser.
Once in the web UI, you should be able to create config files which will be saved to the configs folder. If you are using a config file from a test plan, simply save it to the configs folder and continue to the next step.


