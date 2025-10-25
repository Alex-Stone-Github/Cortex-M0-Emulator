use crate::adr::AddressSpace;
use crate::core::{AByte, AWord};
use crate::memory::{AddressDeMultiplexer, BufferMemory, FunctionalAddressSpace};
use crate::fstools::read_file_buffer;

pub fn load<'a>() -> AddressDeMultiplexer<'a> {
    // Load the Config
    let config_file = std::fs::read_to_string("./config.lua").expect("Bad File");
    let lua = mlua::Lua::new();
    lua.load_std_libs(mlua::StdLib::ALL_SAFE).expect("Failed to load lua stdlib");
    let module = lua.load(config_file.as_str());

    // Run the Config
    module.exec().expect("Error in lua code");

    // Examine Results
    let use_config: bool = lua.globals().get("use_config").expect("No Config");
    let address_specs: mlua::Table = lua.globals().get("addresses")
        .expect("No Memory Map");
    assert_eq!(use_config, true, "No Usable Configuration");

    // Parsing memory
    let mut addresses = AddressDeMultiplexer::full();
    address_specs.for_each(|_: String, props: mlua::Table| -> mlua::Result<()> {
        // _ represents a label
        let origin: u32 = props.get("origin").expect("Expected Origin");
        let rtype: String = props.get("type").expect("Expected Type");

        let region: Box<dyn AddressSpace> = match rtype.as_str() {
            "file" => {
                let filepath: String = props.get("path")
                    .expect("Need filepath for type = file");
                let binary = read_file_buffer(&filepath)
                    .expect("Invalid Filepath");

                Box::new(BufferMemory {
                    origin,
                    buffer: binary
                })
            },
            "ram" => {
                let len: u32 = props.get("len").expect("Expected Length");
                let buffer = (0..len).map(|_| 0).collect::<Box<[u8]>>();

                Box::new(BufferMemory {
                    origin,
                    buffer,
                })
            },
            "func" => {
                let length: u32 = props.get("len").expect("Expected Length");
                let readb_fl: mlua::Function = props.get("readb")
                    .expect("Expected readb function");
                let writeb_fl: mlua::Function = props.get("writeb")
                    .expect("Expected writeb function");

                let readb_f = Box::new(move |adr: AWord| -> AByte {
                    readb_fl.call(adr).expect("Invalid Function Return")
                });
                let writeb_f = Box::new(move |adr: AWord, x: AByte| {
                    writeb_fl.call((adr, x)).expect("Invalid Function Return")
                });

                Box::new(FunctionalAddressSpace {
                    origin,
                    length,
                    readb_f,
                    writeb_f
                })
            },
            _ => panic!("Invalid Region Type")
        };
        addresses.add_region(region);
        Ok(())
    }).expect("Invalid Config Format");


    // Return
    std::mem::forget(lua);
    addresses
}
