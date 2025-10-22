use mlua::*;

pub fn load() {
    println!("Loading Configuration");

    let file_cont = std::fs::read_to_string("./config.lua").expect("Bad File");

    let lua = mlua::Lua::new();
    lua.load_std_libs(mlua::StdLib::OS).expect("Could not load os stdlib");
    let module = lua.load(file_cont.as_str());

    let lf = lua.create_function(|lua: &Lua, _: ()| -> mlua::Result<()> {
        for _ in 0..5 {
            println!("Hi there(from host)");
        }
        Ok(())
    }).expect("Could not create lua host func");
    lua.globals().set("hostfunc", lf).expect("Could not make host func available to lua code");


    module.exec().expect("Error in lua code");

    // Get the var
    let var: u32 = lua.globals().get("xyz").expect("Variable does not exist");
    assert_eq!(var, 345);

    println!("Finished Loading Configuration");
}
