namespace eval jtag_over_axi {
    proc connect_jtag {inst {args {}}} {
        set inst [get_bd_cells $inst]
        set name [get_property NAME $inst]

        set bd_inst [current_bd_instance .]
        save_bd_design
        set group [get_bd_cell $name]
        #move_bd_cells $group $inst
        set ninst [get_bd_cells $group/internal_$name]
        current_bd_instance $group



            #set inst [get_bd_cells $inst]
            #set name [get_property NAME $inst]
            #set ninst [get_bd_cells $inst/internal_$name]

        # Get the number of the target IP
        set kind [scan [regsub {.*target_ip_.*([0-9][0-9][0-9])} $name {\1}] %d]

        # Set the JTAG ID that can be read as IDCODE (Only for JTAG slave standalone)
        set_property -dict [list CONFIG.jtag_id $kind] $ninst

        set jtag_pin [get_bd_intf_pins -of_objects $ninst \
            -filter "VLNV == xilinx.com:interface:jtag_rtl:2.0"]

        puts "JTAG pin found = $jtag_pin for IP $ninst"

        set axi_to_jtag_converter [tapasco::ip::create_axi_to_jtag "axi_to_jtag"]
        # Get Converter Module interface
        set convert_interface [get_bd_intf_pins -of_objects $axi_to_jtag_converter \
            -filter "VLNV == xilinx.com:interface:jtag_rtl:2.0"]

        # Create AXI slave port
        set axi_port [create_bd_intf_pin -vlnv \
            [tapasco::ip::get_vlnv "aximm_intf"] -mode Slave "S_JTAG_DEBUG"]

        # Connect AXI to converter module
        connect_bd_intf_net $axi_port [get_bd_intf_pins -of_objects $axi_to_jtag_converter \
            -filter "VLNV == [tapasco::ip::get_vlnv "aximm_intf"] && MODE == Slave"]

        save_bd_design
        connect_bd_intf_net $convert_interface $jtag_pin


        # Connect Dmi module to Dmi port
        #connect_bd_intf_net $dmi_in $dmi_pin

        connect_bd_net [get_bd_pins $inst/aclk] [get_bd_pins $axi_to_jtag_converter/ACLK]
        connect_bd_net [get_bd_pins $inst/aclk] [get_bd_pins $axi_to_jtag_converter/ACLK_clk]
        connect_bd_net [get_bd_pins $inst/aresetn] [get_bd_pins $axi_to_jtag_converter/ARESETN]

        current_bd_instance $bd_inst

        return [list $inst $args]
    }
}

if {[tapasco::is_feature_enabled "JtagDebug"]} {
    tapasco::register_plugin "arch::jtag_over_axi::connect_jtag" "post-pe-create"
}
