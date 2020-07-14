# Definitional proc to organize widgets for parameters.
proc init_gui { IPINST } {
  ipgui::add_param $IPINST -name "Component_Name"
  #Adding Page
  set Page_0 [ipgui::add_page $IPINST -name "Page 0"]
  ipgui::add_param $IPINST -name "jtag_master" -parent ${Page_0}


}

proc update_PARAM_VALUE.jtag_master { PARAM_VALUE.jtag_master } {
	# Procedure called to update jtag_master when any of the dependent parameters in the arguments change
}

proc validate_PARAM_VALUE.jtag_master { PARAM_VALUE.jtag_master } {
	# Procedure called to validate jtag_master
	return true
}


proc update_MODELPARAM_VALUE.jtag_master { MODELPARAM_VALUE.jtag_master PARAM_VALUE.jtag_master } {
	# Procedure called to set VHDL generic/Verilog parameter value(s) based on TCL parameter value
	set_property value [get_property value ${PARAM_VALUE.jtag_master}] ${MODELPARAM_VALUE.jtag_master}
}

