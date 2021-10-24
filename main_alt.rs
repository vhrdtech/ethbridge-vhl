use canbus_vhl::{Frame, FrameId};


/root {
	/// All available CAN Bus interfaces.
	/bus_list([bus_config::Token; max 16]) {
		access: ro
	}

	/bus_api(fn(bus: bus_config::Token) -> bus_api) {}
	
}

/bus_api {
	/// Get interfaces state (unconfigured / configured).
	/state(bus_config::State) {}

	/// Setup bus mode, speed, hardware filters and start bus communication.
	/configure(fn(bus: bus_config::Unconfigured) -> bus_config::Configured) {}

	/// Get interface status, packet counters, byte counters, error counters.
	/status(fn(freq: time::Hertz) -> stream<bus_status::Status>) {
		/reset_counters() {}
	}
}