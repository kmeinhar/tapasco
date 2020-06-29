namespace eval jtag_over_axi {
    proc connect_jtag_over_axi {inst {args {}}} {

        set inst [get_bd_cells $inst]
        set name [get_property NAME $inst]
        set ninst [get_bd_cells $inst/internal_$name]

        foreach jtag_pin [get_bd_intf_pins -of_objects $ninst \
                -filter "VLNV == xilinx.com:interface:jtag_rtl:2.0"] {
            puts "JTAG pin found = $jtag_pin"

            set axi_to_jtag_converter [tapasco::ip::create_axi_to_jtag axi_to_jtag_target_ip_00_000]
            set convert_interface [get_bd_intf_pins -of_objects $axi_to_jtag_converter \
                -filter "VLNV == xilinx.com:interface:jtag_rtl:2.0"]
            connect_bd_intf_net $convert_interface $jtag_pin

            # Connect JTAG modules reset pin
            connect_bd_net [get_bd_pins /arch/design_peripheral_aresetn] \
                [get_bd_pins $axi_to_jtag_converter/ARESETN]

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
        set_property HDL_ATTRIBUTE.DEBUG true [get_bd_intf_nets {axi_to_jtag_target_ip_00_000_jtag}]

        create_bd_cell -type ip -vlnv xilinx.com:ip:system_ila:1.1 system_ila_0

        set_property -dict [list CONFIG.C_BRAM_CNT {6} CONFIG.C_SLOT_0_INTF_TYPE {xilinx.com:interface:jtag_rtl:2.0}] [get_bd_cells system_ila_0]
        connect_bd_intf_net [get_bd_intf_pins system_ila_0/SLOT_0_JTAG] -boundary_type upper [get_bd_intf_pins axi_to_jtag_target_ip_00_000/jtag]
        connect_bd_net [get_bd_pins system_ila_0/clk] [get_bd_pins design_clk]
    }

    # Add the AXI to JTAG module to the address map
    proc add_address_map {map} {
        set intf [get_bd_intf_pins -of_object [get_bd_cells /arch/axi_to_jtag_target_ip_00_000] \
            -filter {VLNV == xilinx.com:interface:aximm_rtl:1.0}]

        set offset 0x43C00000
        set range [get_property range [get_bd_addr_segs /arch/axi_to_jtag_target_ip_00_000/s_AXI_JTAG/reg0]]

        dict set map $intf "interface $intf \
            [format "offset 0x%08x range 0x%08x" $offset $range] kind memory"

        return $map
    }
}

tapasco::register_plugin "arch::jtag_over_axi::connect_jtag_over_axi" "post-pe-create"
tapasco::register_plugin "arch::jtag_over_axi::add_address_map" "additional-address-map"
