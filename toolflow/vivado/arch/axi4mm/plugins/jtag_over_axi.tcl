namespace eval jtag_over_axi {
    variable module_name "axi_to_jtag_0"

    proc connect_jtag_over_axi {inst {args {}}} {
        variable module_name

        set inst [get_bd_cells $inst]
        set name [get_property NAME $inst]
        set ninst [get_bd_cells $inst/internal_$name]

        # Create AXI to JTAG converter
        set axi_to_jtag_converter [tapasco::ip::create_axi_to_jtag $module_name]
        set convert_interface [get_bd_intf_pins -of_objects $axi_to_jtag_converter \
            -filter "VLNV == xilinx.com:interface:jtag_rtl:2.0"]

        # Connect JTAG modules reset pin
        connect_bd_net [get_bd_pins /arch/design_peripheral_aresetn] \
            [get_bd_pins $axi_to_jtag_converter/ARESETN]

        foreach jtag_pin [get_bd_intf_pins -of_objects $ninst \
                -filter "VLNV == xilinx.com:interface:jtag_rtl:2.0"] {
            puts "JTAG pin found = $jtag_pin"

            # Connect JTAG module to JTAG port
            connect_bd_intf_net $convert_interface $jtag_pin

            # Connect the jtag reset pin of the core
            connect_bd_net [get_bd_pins $axi_to_jtag_converter/jtag_TRST] [get_bd_pins $ninst/JTAG_RST]

            puts "Successfully added AXI to JTAG module"

            # append the new module to list of axi slaves
            set inst [list $inst $axi_to_jtag_converter]
        }
        return [list $inst $args]
    }

    # Helper function thats adds an ILA to JTAG
    proc add_jtag_ila {} {
        variable module_name

        set_property HDL_ATTRIBUTE.DEBUG true [get_bd_intf_nets {$module_name}]

        create_bd_cell -type ip -vlnv xilinx.com:ip:system_ila:1.1 system_ila_0

        set_property -dict [list CONFIG.C_BRAM_CNT {6} CONFIG.C_SLOT_0_INTF_TYPE {xilinx.com:interface:jtag_rtl:2.0}] [get_bd_cells system_ila_0]
        connect_bd_intf_net [get_bd_intf_pins system_ila_0/SLOT_0_JTAG] -boundary_type upper [get_bd_intf_pins $module_name/jtag]
        connect_bd_net [get_bd_pins system_ila_0/clk] [get_bd_pins design_clk]
    }

    # Add the AXI to JTAG module to the address map
    proc add_address_map {map offset} {
        variable module_name

        set intf [get_bd_intf_pins -of_object [get_bd_cells /arch/$module_name] \
            -filter {VLNV == xilinx.com:interface:aximm_rtl:1.0}]

        set offset 0x43C00000
        set range [get_property range [get_bd_addr_segs /arch/$module_name/s_AXI_JTAG/reg0]]

        puts "Adding JTAG Module at address $offset with range $range"

        dict set map $intf "interface $intf \
            [format "offset 0x%08x range 0x%08x" $offset $range] kind invisible"

        incr offset $range

        return $map
    }
}

tapasco::register_plugin "arch::jtag_over_axi::connect_jtag_over_axi" "post-pe-create"
tapasco::register_plugin "arch::jtag_over_axi::add_address_map" "additional-address-map"
