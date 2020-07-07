namespace eval jtag_over_axi {

    proc connect_jtag {inst {args {}}} {
        set inst [get_bd_cells $inst]
        set name [get_property NAME $inst]
        set ninst [get_bd_cells $inst/internal_$name]

        # Add AXI to JTAG conversion module
        set jtag_in [create_bd_intf_pin -mode Slave -vlnv xilinx.com:interface:jtag_rtl:2.0 S_JTAG]
        set jtag_in_rst [create_bd_pin -dir I JTAG_RST]

        foreach jtag_pin [get_bd_intf_pins -of_objects $ninst \
                -filter "VLNV == xilinx.com:interface:jtag_rtl:2.0"] {
            puts "JTAG pin found = $jtag_pin"

            # TODO JTAG splitter for the case that more that one JTAG slave is present

            # Connect JTAG module to JTAG port
            connect_bd_intf_net $jtag_in $jtag_pin
        }
        foreach jtag_rst [get_bd_pins -of_objects $ninst \
                -filter "NAME == JTAG_RST"] {
            puts "JTAG reset pin found = $jtag_rst"

            # Connect JTAG reset
            connect_bd_net $jtag_in_rst $jtag_rst
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
}

tapasco::register_plugin "arch::jtag_over_axi::connect_jtag" "post-pe-create"
