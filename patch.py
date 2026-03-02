import sys
content = open("build/imports/ariel-os/src/ariel-os-rt/build.rs").read()
content = content.replace("""    } else if context("nrf54l15") {
        (256, 1536)

        let ram = 256;
        let flash = 1024;
        if cfg!(feature = "nrf91-modem") {
            (ram - NRF91_MODEM_IPC_KB, flash)
        } else {
            (ram, flash)
        }""", """    } else if context("nrf54l15") {
        (256, 1536)""")
open("build/imports/ariel-os/src/ariel-os-rt/build.rs", "w").write(content)
