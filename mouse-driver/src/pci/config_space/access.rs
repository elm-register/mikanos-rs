use crate::pci::config_space::access::config_address_register::ConfigAddrRegister;
use crate::pci::config_space::access::intel_x86_io::{fetch_config_data, write_config_addr};
use crate::pci::config_space::common_header::common_header_holdable::{
    exists_device, CommonHeaderHoldable,
};
use crate::pci::config_space::device::general_device::GeneralDevice;
use crate::pci::config_space::device::multiple_function_device::MultipleFunctionDevice;
use crate::pci::config_space::device::pci_bridge_device::PciBrideDevice;
use crate::pci::config_space::device::PciDevice;

pub mod config_address_register;
pub mod intel_x86_io;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ConfigurationSpace {
    bus: u8,
    device_slot: u8,
    function: u8,
}

impl ConfigurationSpace {
    pub fn try_new(bus: u8, device_slot: u8, function: u8) -> Option<Self> {
        let me = ConfigurationSpace::new(bus, device_slot, function);
        if exists_device(me.fetch_data_offset_at(0)) {
            return Some(me);
        } else {
            None
        }
    }

    pub fn cast_device(self) -> Option<PciDevice> {
        let header_type = self.header_type();

        match header_type & 0x80 {
            0 => select_pci_device(self, header_type),
            _ => Some(PciDevice::MultipleFunction(MultipleFunctionDevice::new(
                self,
            ))),
        }
    }
    pub fn bus(&self) -> u8 {
        self.bus
    }

    pub fn device_slot(&self) -> u8 {
        self.device_slot
    }

    pub fn function(&self) -> u8 {
        self.function
    }

    pub(crate) fn fetch_data_offset_at(&self, offset: u8) -> u32 {
        write_config_addr(self.config_addr_at(offset));
        fetch_config_data()
    }

    fn new(bus: u8, device_slot: u8, function: u8) -> Self {
        Self {
            bus,
            device_slot,
            function,
        }
    }

    fn config_addr_at(&self, offset: u8) -> ConfigAddrRegister {
        ConfigAddrRegister::new(offset, self.function, self.device_slot, self.bus)
    }
}

impl CommonHeaderHoldable for ConfigurationSpace {
    fn config_space(&self) -> &ConfigurationSpace {
        self
    }
}

fn select_pci_device(config_space: ConfigurationSpace, header_type: u8) -> Option<PciDevice> {
    return if (header_type & 0b11) == 0x1 {
        Some(PciDevice::Bridge(PciBrideDevice::new(config_space)))
    } else {
        Some(PciDevice::General(GeneralDevice::new(config_space)))
    };
}

#[cfg(test)]
mod tests {
    use crate::pci::config_space::access::ConfigurationSpace;

    #[test]
    fn it_new_first_offset() {
        let p = ConfigurationSpace::new(1, 2, 3).config_addr_at(0);
        let inner = *p;
        assert_eq!(p.enabled(), true);
        assert_eq!((inner >> 31), 1);

        assert_eq!(p.bus(), 1);
        assert_eq!(((inner >> 16) & 0xFF), 1);

        assert_eq!(p.device_slot(), 2);
        assert_eq!(((inner >> 11) & 0b11111), 2);

        assert_eq!(p.function(), 3);
        assert_eq!(((inner >> 8) & 0b111), 3);

        assert_eq!(p.register_offset(), 0);
        assert_eq!((inner & 0xFC), 0);
    }
}