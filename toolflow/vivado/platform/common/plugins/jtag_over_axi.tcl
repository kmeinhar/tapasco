
if {[tapasco::is_feature_enabled "JtagDebug"]} {
proc create_custom_subsystem_jtag_debug {{args {}}} {
    puts "  creating debug subsystem"

    # Search for all JTAG slave interfaces
    set jtag_slave_intf {}
    set i 0
    foreach p [get_bd_intf_pins -quiet -of_objects [get_bd_cells /arch] \
        -filter "MODE == Slave && VLNV == xilinx.com:interface:jtag_rtl:2.0"] {

        # Create JTAG and JTAG reset ports
        lappend jtag_slave_intf [create_bd_intf_pin -mode Master \
            -vlnv xilinx.com:interface:jtag_rtl:2.0 [format "M_JTAG_%02d" $i]]
        incr i
    }

    puts "  found [llength $jtag_slave_intf] jtag interfaces"

    # Create AXI to JTAG converter
    set axi_to_jtag_converter [tapasco::ip::create_axi_to_jtag "axi_to_jtag"]
    # Get Converter Module interface
    set convert_interface [get_bd_intf_pins -of_objects $axi_to_jtag_converter \
        -filter "VLNV == xilinx.com:interface:jtag_rtl:2.0"]

    # Create AXI slave port
    set axi_port [create_bd_intf_pin -vlnv \
        [tapasco::ip::get_vlnv "aximm_intf"] -mode Slave "S_DEBUG"]

    # Connect AXI to converter module
    connect_bd_intf_net $axi_port [get_bd_intf_pins -of_objects $axi_to_jtag_converter \
        -filter "VLNV == [tapasco::ip::get_vlnv "aximm_intf"] && MODE == Slave"]

    if {[llength $jtag_slave_intf] > 16} {
        error "Currently only 16 JTAG interfaces supported!"
    } elseif {[llength $jtag_slave_intf] > 1} {
        set SWITCH "jtag_switch"
        set CHAIN  "jtag_chain"

        set device_type [tapasco::get_feature_option "JtagDebug" "device_type" $SWITCH]

        if {$device_type == $SWITCH} {
            # Create jtag switch to split one jtag interface into multiple
            set jtag_splitter [tapasco::ip::create_jtag_switch "jtag_switch"]
        } elseif {$device_type == $CHAIN} {
            set jtag_splitter [tapasco::ip::create_jtag_chain "jtag_chain"]
            # Configure Converter to use JTAG chain
            set_property -dict [list CONFIG.jtag_device_type {0x2}] $axi_to_jtag_converter
            set_property CONFIG.jtag_master [llength $jtag_slave_intf] $axi_to_jtag_converter
        } else {
            error "Only possible device types are $SWITCH and $CHAIN"
        }

        # Get JTAG switch slave interface
        set switch_slave [get_bd_intf_pins -of_objects $jtag_splitter \
            -filter "VLNV == xilinx.com:interface:jtag_rtl:2.0 && MODE == Slave"]

        connect_bd_intf_net $convert_interface $switch_slave

        # Set the number of output ports to the number of slaves
        set_property CONFIG.jtag_master [llength $jtag_slave_intf]  $jtag_splitter

        # Connect all JTAG slaves
        foreach jtag_master [get_bd_intf_pins -of_objects $jtag_splitter \
            -filter "VLNV == xilinx.com:interface:jtag_rtl:2.0 && MODE == Master"] {
            set kind [scan [regsub {.*([0-9][0-9])} $jtag_master {\1}] %d]
            connect_bd_intf_net $jtag_master [lindex $jtag_slave_intf $kind]
        }

        # Connect slave select
        set converter_ss [get_bd_pins -of_objects $axi_to_jtag_converter \
            -filter "NAME == slave_select"]
        set switch_ss [get_bd_pins -of_objects $jtag_splitter \
            -filter "NAME == slave_select"]
        connect_bd_net $converter_ss $switch_ss

        # Connect clk and reset to converter module
        connect_bd_net [tapasco::subsystem::get_port "host" "clk"] \
            [get_bd_pins -of_objects $jtag_splitter -filter {TYPE == clk && DIR == I}]
        connect_bd_net [tapasco::subsystem::get_port "host" "rst" "peripheral" "resetn"] \
            [get_bd_pins -of_objects $jtag_splitter -filter {TYPE == rst && DIR == I}]
    } else {
        # Only one jtag slave present
        connect_bd_intf_net $convert_interface [lindex $jtag_slave_intf 0]
    }

    # Connect clk and reset to converter module
    connect_bd_net [tapasco::subsystem::get_port "host" "clk"] \
        [get_bd_pins -of_objects $axi_to_jtag_converter -filter {TYPE == clk && DIR == I}]
    connect_bd_net [tapasco::subsystem::get_port "host" "rst" "peripheral" "resetn"] \
        [get_bd_pins -of_objects $axi_to_jtag_converter -filter {TYPE == rst && DIR == I}]
 
        return {}
}
}


