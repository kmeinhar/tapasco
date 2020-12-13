namespace eval jtag_over_axi {

    proc connect_jtag {inst {args {}}} {
            set inst [get_bd_cells $inst]
            set name [get_property NAME $inst]
            set ninst [get_bd_cells $inst/internal_$name]

            # Get the number of the target IP
            set kind [scan [regsub {.*target_ip_.*([0-9][0-9][0-9])} $name {\1}] %d]

            # Set the JTAG ID that can be read as IDCODE
            set_property -dict [list CONFIG.jtag_id $kind] $ninst

            foreach jtag_pin [get_bd_intf_pins -of_objects $ninst \
                    -filter "VLNV == xilinx.com:interface:jtag_rtl:2.0"] {
                puts "JTAG pin found = $jtag_pin for IP $ninst"

                # Create JTAG interface ports for every IP with JTAG port
                set jtag_in [create_bd_intf_pin -mode Slave \
                    -vlnv xilinx.com:interface:jtag_rtl:2.0 [format "S_JTAG_%02d" $kind]]

                # Connect JTAG module to JTAG port
                connect_bd_intf_net $jtag_in $jtag_pin
            }

            return [list $inst $args]
    }
}

if {[tapasco::is_feature_enabled "PlatformJtagDebug"]} {
    tapasco::register_plugin "arch::jtag_over_axi::connect_jtag" "post-pe-create"
}
