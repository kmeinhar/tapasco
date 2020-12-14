# Definitional proc to organize widgets for parameters.
proc init_gui { IPINST } {
  ipgui::add_param $IPINST -name "Component_Name"
  #Adding Page
  set Page_0 [ipgui::add_page $IPINST -name "Page 0"]
  set jtag_device_type [ipgui::add_param $IPINST -name "jtag_device_type" -parent ${Page_0} -widget comboBox]
  set_property tooltip {Specifies the type of the connected JTAG splitter} ${jtag_device_type}
  ipgui::add_static_text $IPINST -name "JTAG Master" -parent ${Page_0} -text {Number of JTAG Master attached to the Jtag Splitter. Only relevant for JTAG Chain device type}
  set jtag_master [ipgui::add_param $IPINST -name "jtag_master" -parent ${Page_0}]
  set_property tooltip {Number of JTAG Master attached to the JTAG Splitter} ${jtag_master}
  ipgui::add_param $IPINST -name "rtck_type" -parent ${Page_0}


}

proc update_PARAM_VALUE.jtag_device_type { PARAM_VALUE.jtag_device_type } {
	# Procedure called to update jtag_device_type when any of the dependent parameters in the arguments change
}

proc validate_PARAM_VALUE.jtag_device_type { PARAM_VALUE.jtag_device_type } {
	# Procedure called to validate jtag_device_type
	return true
}

proc update_PARAM_VALUE.jtag_master { PARAM_VALUE.jtag_master } {
	# Procedure called to update jtag_master when any of the dependent parameters in the arguments change
}

proc validate_PARAM_VALUE.jtag_master { PARAM_VALUE.jtag_master } {
	# Procedure called to validate jtag_master
	return true
}

proc update_PARAM_VALUE.rtck_type { PARAM_VALUE.rtck_type } {
	# Procedure called to update rtck_type when any of the dependent parameters in the arguments change
}

proc validate_PARAM_VALUE.rtck_type { PARAM_VALUE.rtck_type } {
	# Procedure called to validate rtck_type
	return true
}


proc update_MODELPARAM_VALUE.jtag_device_type { MODELPARAM_VALUE.jtag_device_type PARAM_VALUE.jtag_device_type } {
	# Procedure called to set VHDL generic/Verilog parameter value(s) based on TCL parameter value
	set_property value [get_property value ${PARAM_VALUE.jtag_device_type}] ${MODELPARAM_VALUE.jtag_device_type}
}

proc update_MODELPARAM_VALUE.jtag_master { MODELPARAM_VALUE.jtag_master PARAM_VALUE.jtag_master } {
	# Procedure called to set VHDL generic/Verilog parameter value(s) based on TCL parameter value
	set_property value [get_property value ${PARAM_VALUE.jtag_master}] ${MODELPARAM_VALUE.jtag_master}
}

proc update_MODELPARAM_VALUE.rtck_type { MODELPARAM_VALUE.rtck_type PARAM_VALUE.rtck_type } {
	# Procedure called to set VHDL generic/Verilog parameter value(s) based on TCL parameter value
	set_property value [get_property value ${PARAM_VALUE.rtck_type}] ${MODELPARAM_VALUE.rtck_type}
}

